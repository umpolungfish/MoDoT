//! In-process local inference: safetensors → logits inside THIS binary.
//!
//! No server, no port, no second process. The weights (`~/.modelz/<MODEL>/`,
//! HF Qwen3 safetensors) are mmapped straight into the `ask` process via candle
//! and run on the GPU. This is the broke-mode / offline path: the agent thinks
//! locally, no cloud, no credits.
//!
//! Gated behind the `local` cargo feature so the default build stays lean. The
//! model loads once per process and stays warm across every cycle of a run
//! (a run makes many infer() calls; reloading 8 GB each time would be absurd),
//! held in a process-global mutex. Across separate `ask` invocations it reloads
//! — that is the only cost of not running a resident server, and it is the
//! right trade for a closed local loop.

use std::sync::{Mutex, OnceLock};

use candle_core::{DType, Device, Tensor};
use candle_nn::VarBuilder;
use candle_transformers::generation::LogitsProcessor;
use candle_transformers::models::qwen3::{Config as Qwen3Config, ModelForCausalLM};
use tokenizers::Tokenizer;

/// Where the weights live and how to run them, all from the environment so the
/// same binary points at whatever local model is present.
struct LocalCfg {
    model_dir: String,
    /// CUDA ordinals to use, in stage order. Empty means "every card present".
    devices: Vec<usize>,
    /// The user named the cards. A named pair is a decision to split; an
    /// unnamed pair is only a fact about the box.
    devices_pinned: bool,
    force_cpu: bool,
}

fn env(k: &str) -> Option<String> {
    std::env::var(k).ok().filter(|v| !v.is_empty())
}

/// Read the canonical `IG_*` key, falling back to the legacy `MODOT_*` spelling
/// so an old shell keeps working. IG_ is the one name across every repo here.
fn env_ig(canonical: &str, legacy: &[&str]) -> Option<String> {
    if let Some(v) = env(canonical) {
        return Some(v);
    }
    for k in legacy {
        if let Some(v) = env(k) {
            return Some(v);
        }
    }
    None
}

fn expand(p: &str) -> String {
    if let Some(rest) = p.strip_prefix("~/") {
        if let Some(h) = dirs::home_dir() {
            return h.join(rest).to_string_lossy().into_owned();
        }
    }
    p.to_string()
}

/// Parse `IG_DEVICES`: a comma list of CUDA ordinals ("0,1"), a single ordinal
/// ("1"), "cpu" to force CPU, or "auto"/unset to take every card present.
fn parse_devices(spec: &str) -> (Vec<usize>, bool) {
    let s = spec.trim().to_ascii_lowercase();
    if s == "cpu" {
        return (vec![], true);
    }
    if s.is_empty() || s == "auto" || s == "all" {
        return (vec![], false);
    }
    let mut out = Vec::new();
    for part in s.split(',') {
        if let Ok(i) = part.trim().parse::<usize>() {
            if !out.contains(&i) {
                out.push(i);
            }
        }
    }
    (out, false)
}

fn local_cfg() -> LocalCfg {
    // Align candle's device indexing with nvidia-smi so IG_DEVICES means what the
    // user sees. Without this, CUDA's default (fastest-first) ordering reshuffles
    // the ordinals with the shell's env. Set only if the user has not pinned it.
    if env("CUDA_DEVICE_ORDER").is_none() {
        std::env::set_var("CUDA_DEVICE_ORDER", "PCI_BUS_ID");
    }
    let (devices, spec_cpu) = match env_ig("IG_DEVICES", &["MODOT_LOCAL_DEVICE"]) {
        Some(spec) => parse_devices(&spec),
        None => (vec![], false),
    };
    LocalCfg {
        // IG_LOCAL_MODEL_DIR, else Qwen3-1.7B: ~4 GB in bf16, which fits either
        // card alone. Bigger models are what the multi-card split is for — a 4B
        // or 8B splits across both with room for context neither card has on its own.
        model_dir: expand(
            &env_ig("IG_LOCAL_MODEL_DIR", &["MODOT_LOCAL_MODEL_DIR"])
                .unwrap_or_else(|| "~/models/Qwen3-1.7B".to_string()),
        ),
        devices_pinned: devices.len() > 1,
        devices,
        force_cpu: spec_cpu || env_ig("IG_LOCAL_CPU", &["MODOT_LOCAL_CPU"]).is_some(),
    }
}

/// Free VRAM in bytes for one CUDA ordinal, straight from nvidia-smi (the same
/// PCI ordinal — CUDA_DEVICE_ORDER is pinned). 0 when unreadable.
fn free_vram(idx: usize) -> u64 {
    std::process::Command::new("nvidia-smi")
        .args([
            "--query-gpu=memory.free",
            "--format=csv,noheader,nounits",
            "-i",
            &idx.to_string(),
        ])
        .output()
        .ok()
        .filter(|o| o.status.success())
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|s| s.trim().parse::<u64>().ok())
        .map(|mib| mib * 1024 * 1024)
        .unwrap_or(0)
}

/// Open every device the run should use, in stage order. Two cards of the same
/// generation both open here and the model is SPLIT across them; one card (or a
/// pinned single ordinal) keeps the proven single-device path.
///
/// Probing is by open-and-keep rather than by count: an ordinal that will not
/// open is simply not in the list, so a card claimed by another process or
/// absent from the driver never becomes a stage that fails mid-load.
fn open_devices(cfg: &LocalCfg) -> Vec<(Device, Option<usize>)> {
    if cfg.force_cpu {
        return vec![(Device::Cpu, None)];
    }
    let wanted: Vec<usize> = if cfg.devices.is_empty() {
        (0..8).collect()
    } else {
        cfg.devices.clone()
    };
    let mut open: Vec<(Device, Option<usize>)> = Vec::new();
    for idx in wanted {
        if let Ok(d) = Device::new_cuda(idx) {
            open.push((d, Some(idx)));
        }
    }
    if open.is_empty() {
        eprintln!("\x1b[2m[local] no CUDA device reachable — CPU (slow); check LD_LIBRARY_PATH / driver\x1b[0m");
        return vec![(Device::Cpu, None)];
    }
    open
}

/// Size the prompt cap from the cards themselves, not a guess.
///
/// With the weights resident, what is left must hold two things: the KV cache,
/// which is exact from config.json (2 × layers × kv_heads × head_dim × bf16 per
/// token), and the transient attention scores of one prefill block, which are
/// `heads × chunk × seq` twice over (the scores and their softmax). So the
/// budget equation is linear in the sequence length,
///
///   avail ≥ seq × (kv_per_tok + 2 × 2 × heads × chunk × dtype_bytes)
///
/// and the cap is what that solves to. Chunked prefill is what makes this
/// linear rather than quadratic; flash-attn removes the second term entirely.
///
/// Across a SPLIT model the KV cache splits with the layers, so the budget is
/// the sum of what every stage has free — which is the point of the split: two
/// cards carry a context neither could hold alone. IG_LOCAL_CTX overrides.
fn compute_ctx_cap(qcfg: &Qwen3Config, cuda_idx: &[usize], chunk: usize, quiet: bool) -> usize {
    if let Some(v) = env_ig("IG_LOCAL_CTX", &["MODOT_LOCAL_CTX"]).and_then(|s| s.parse().ok()) {
        return v;
    }
    if cuda_idx.is_empty() {
        return 9000; // CPU: RAM is not the constraint, patience is
    }
    let kv_per_tok = 2 * qcfg.num_hidden_layers * qcfg.num_key_value_heads * qcfg.head_dim * 2;
    // Flash attention never materializes the score matrix, so only the KV term
    // remains; without it, one block's scores and softmax are live at once.
    let score_per_tok = if cfg!(feature = "flash-attn") {
        0
    } else {
        2 * 2 * qcfg.num_attention_heads * chunk
    };
    let free_bytes: u64 = cuda_idx.iter().map(|i| free_vram(*i)).sum();
    let total_mib = (free_bytes / (1024 * 1024)) as usize;
    // One headroom charge per card: each stage runs its own activations and its
    // own allocator fragmentation.
    let headroom_mib = 1024 * cuda_idx.len();
    let cap = if total_mib == 0 {
        9000 // nvidia-smi unreachable: the old conservative constant
    } else {
        let avail = total_mib.saturating_sub(headroom_mib) * 1024 * 1024;
        let gen_reserve = 2048; // decode extends the KV cache past the prompt
        (avail / (kv_per_tok + score_per_tok).max(1))
            .saturating_sub(gen_reserve)
            .clamp(2048, qcfg.max_position_embeddings.min(32000))
    };
    if !quiet {
        eprintln!(
            "\x1b[2m[local] ctx cap {cap} tok ({} free MiB over {} card(s), {:.2} MiB/tok KV, prefill chunk {chunk})\x1b[0m",
            total_mib,
            cuda_idx.len(),
            kv_per_tok as f64 / (1024.0 * 1024.0)
        );
    }
    cap
}

/// The resident stack: one card runs candle's own Qwen3 (the proven path),
/// two or more run the pipeline-sharded stack from `shard.rs`. Both answer the
/// same two calls, so the generate loop below never branches on which it holds.
enum Backend {
    Single(ModelForCausalLM),
    Split(crate::shard::ShardedQwen3),
}

impl Backend {
    fn forward(&mut self, input: &Tensor, offset: usize) -> candle_core::Result<Tensor> {
        match self {
            Backend::Single(m) => m.forward(input, offset),
            Backend::Split(m) => m.forward(input, offset),
        }
    }

    fn clear_kv_cache(&mut self) {
        match self {
            Backend::Single(m) => m.clear_kv_cache(),
            Backend::Split(m) => m.clear_kv_cache(),
        }
    }
}

/// A loaded model held resident for the process lifetime.
struct Engine {
    model: Backend,
    tokenizer: Tokenizer,
    device: Device,
    eos_ids: Vec<u32>,
    model_dir: String,
    /// The model's OWN Jinja chat template, COMPILED ONCE. The ob3ect decomposition
    /// of this template grounds it as an O₀, ΔS≈0, deterministic ("frozen kinetics")
    /// object — a constant. So it is parsed a single time at load and rendered from
    /// the cached environment every call, rather than re-parsed per generation. None
    /// if the model ships no template (then a ChatML fallback is used).
    template_env: Option<minijinja::Environment<'static>>,
    /// Prompt-length cap in tokens, sized once at load from the free VRAM left
    /// after the weights and this model's exact per-token KV cost.
    ctx_cap: usize,
    /// How many prompt tokens enter the model at once during prefill. This, not
    /// the prompt length, is what sets the peak attention-score allocation.
    prefill_chunk: usize,
}

/// What the checkpoint says it is, before any weight is touched.
///
/// Qwen3 keeps its hyperparameters at the top level of config.json. Qwen3.5 is a
/// `ForConditionalGeneration` that nests them under `text_config`, prefixes every
/// text tensor with `model.language_model.`, and — the part that matters — is a
/// HYBRID: `layer_types` marks three layers in four as `linear_attention`, a
/// gated-DeltaNet recurrence with its own conv1d and state, and only every fourth
/// as `full_attention`. Reading the nested config is a rename; running the
/// recurrence is a different model.
struct ModelShape {
    cfg: Qwen3Config,
    /// Prefix on the decoder tensors: "model." for Qwen3,
    /// "model.language_model." for the 3.5 layout.
    prefix: String,
}

fn read_model_shape(dir: &str) -> Result<ModelShape, String> {
    let path = format!("{dir}/config.json");
    let raw = std::fs::read_to_string(&path).map_err(|e| format!("read {path}: {e}"))?;
    let root: serde_json::Value =
        serde_json::from_str(&raw).map_err(|e| format!("parse {path}: {e}"))?;

    // The text hyperparameters, wherever this checkpoint keeps them.
    let (j, prefix) = match root.get("text_config") {
        Some(t) => (t.clone(), "model.language_model.".to_string()),
        None => (root.clone(), "model.".to_string()),
    };

    // Refuse a hybrid rather than load it into a full-attention stack. A stack
    // that reads a DeltaNet layer's weights as attention does not fail — it
    // produces numbers, and they are noise. Say what is missing instead.
    if let Some(types) = j.get("layer_types").and_then(|v| v.as_array()) {
        let linear = types
            .iter()
            .filter(|t| t.as_str() == Some("linear_attention"))
            .count();
        if linear > 0 {
            return Err(format!(
                "{} of {} layers are linear_attention (gated DeltaNet: conv1d + recurrent state), \
                 which this in-process stack does not implement — it runs full-attention Qwen3 \
                 layers only. Serve this model with vLLM or SGLang and point IG_LOCAL_BASE_URL \
                 at it, or use a Qwen3 checkpoint here.",
                linear,
                types.len()
            ));
        }
    }
    let model_type = j.get("model_type").and_then(|v| v.as_str()).unwrap_or("");
    if j.get("linear_attn_config").is_some() || model_type.contains("moe") {
        return Err(format!(
            "model_type '{model_type}' is not a dense full-attention Qwen3 stack; \
             serve it with vLLM or SGLang instead of loading it in-process"
        ));
    }

    let g = |k: &str| j.get(k);
    let u = |k: &str, d: usize| g(k).and_then(|v| v.as_u64()).map(|x| x as usize).unwrap_or(d);
    let b = |k: &str, d: bool| g(k).and_then(|v| v.as_bool()).unwrap_or(d);
    // rope_theta moved into rope_parameters in the 3.5 layout; read either.
    let rope_theta = j
        .get("rope_parameters")
        .and_then(|r| r.get("rope_theta"))
        .or_else(|| g("rope_theta"))
        .and_then(|v| v.as_f64())
        .unwrap_or(1_000_000.0);
    let rms_norm_eps = g("rms_norm_eps").and_then(|v| v.as_f64()).unwrap_or(1e-6);
    let hidden = u("hidden_size", 2560);
    let heads = u("num_attention_heads", 32);
    // A partial rotary factor rotates only part of each head. Nothing here
    // implements that, and applying full RoPE to a partially-rotary model is
    // silent corruption, so it is a refusal rather than a warning.
    if let Some(f) = j
        .get("rope_parameters")
        .and_then(|r| r.get("partial_rotary_factor"))
        .and_then(|v| v.as_f64())
    {
        if (f - 1.0).abs() > 1e-9 {
            return Err(format!(
                "partial_rotary_factor {f} rotates only part of each head; this stack applies \
                 full RoPE, which would corrupt every position. Serve this model instead."
            ));
        }
    }
    Ok(ModelShape {
        cfg: Qwen3Config {
            vocab_size: u("vocab_size", 151936),
            hidden_size: hidden,
            intermediate_size: u("intermediate_size", 9728),
            num_hidden_layers: u("num_hidden_layers", 36),
            num_attention_heads: heads,
            // Qwen3 carries head_dim explicitly; fall back to hidden/heads.
            head_dim: u("head_dim", hidden / heads.max(1)),
            attention_bias: b("attention_bias", false),
            num_key_value_heads: u("num_key_value_heads", 8),
            max_position_embeddings: u("max_position_embeddings", 40960),
            sliding_window: None,
            max_window_layers: u("max_window_layers", 0),
            tie_word_embeddings: b("tie_word_embeddings", true),
            rope_theta,
            rms_norm_eps,
            use_sliding_window: false,
            hidden_act: candle_nn::Activation::Silu,
        },
        prefix,
    })
}

fn safetensor_shards(dir: &str) -> Result<Vec<std::path::PathBuf>, String> {
    let idx = format!("{dir}/model.safetensors.index.json");
    if let Ok(raw) = std::fs::read_to_string(&idx) {
        let j: serde_json::Value =
            serde_json::from_str(&raw).map_err(|e| format!("parse {idx}: {e}"))?;
        let mut set = std::collections::BTreeSet::new();
        if let Some(map) = j.get("weight_map").and_then(|m| m.as_object()) {
            for v in map.values() {
                if let Some(f) = v.as_str() {
                    set.insert(f.to_string());
                }
            }
        }
        if !set.is_empty() {
            return Ok(set.into_iter().map(|f| format!("{dir}/{f}").into()).collect());
        }
    }
    // single-file model
    let single = format!("{dir}/model.safetensors");
    if std::path::Path::new(&single).exists() {
        return Ok(vec![single.into()]);
    }
    Err(format!("no safetensors found in {dir}"))
}

/// The sampling a model's own card asks for. Qwen3 and Qwen3.5 differ, and the
/// difference is not cosmetic: 3.5 thinks at temperature 1.0 and wants a presence
/// penalty where 3 wanted none. Keyed off the checkpoint directory because that
/// is what the user actually chose; an unrecognized name gets Qwen3's numbers,
/// which are the conservative pair.
struct SamplingCard {
    think: (f64, f64),
    instruct: (f64, f64),
    repeat_penalty: f32,
    presence_penalty: f32,
    /// The output length the card asks for on an ordinary query.
    max_output: usize,
}

impl SamplingCard {
    fn for_model(dir: &str) -> Self {
        let name = dir.to_ascii_lowercase();
        // Qwen3.5: thinking T=1.0/top-p 0.95, instruct T=0.7/top-p 0.8, presence
        // penalty 1.5, repetition penalty 1.0 (i.e. off — the presence penalty
        // does that work instead).
        if name.contains("qwen3.5") || name.contains("qwen3_5") || name.contains("qwen35") {
            return Self {
                think: (1.0, 0.95),
                instruct: (0.7, 0.8),
                repeat_penalty: 1.0,
                presence_penalty: 1.5,
                max_output: 32768,
            };
        }
        Self {
            think: (0.6, 0.95),
            instruct: (0.7, 0.8),
            repeat_penalty: 1.15,
            presence_penalty: 0.0,
            max_output: 8192,
        }
    }

    fn mode(&self, think: bool) -> (f64, f64) {
        if think {
            self.think
        } else {
            self.instruct
        }
    }
}

/// Subtract `penalty` from the logit of every token already emitted — once each,
/// however many times it has appeared. That "once each" is the whole difference
/// from a repetition penalty: presence discourages returning to a token at all,
/// repetition discourages returning to it AGAIN, and a model card asking for one
/// is not asking for the other.
fn apply_presence_penalty(logits: &Tensor, penalty: f32, seen: &[u32]) -> candle_core::Result<Tensor> {
    let device = logits.device().clone();
    let mut v = logits.to_dtype(DType::F32)?.to_vec1::<f32>()?;
    let mut done = std::collections::HashSet::new();
    for &t in seen {
        if done.insert(t) {
            if let Some(slot) = v.get_mut(t as usize) {
                *slot -= penalty;
            }
        }
    }
    let n = v.len();
    Tensor::from_vec(v, n, &device)
}

impl Engine {
    fn load() -> Result<Engine, String> {
        let cfg = local_cfg();
        let quiet = env_ig("IG_LOCAL_STREAM", &["MODOT_LOCAL_STREAM"])
            .map(|v| v == "0")
            .unwrap_or(false);
        let t0 = std::time::Instant::now();
        let devices = open_devices(&cfg);
        let ordinals: Vec<usize> = devices.iter().filter_map(|(_, o)| *o).collect();
        let shape = read_model_shape(&cfg.model_dir)?;
        let qcfg = shape.cfg.clone();
        let shards = safetensor_shards(&cfg.model_dir)?;
        // A bitsandbytes-NF4 checkpoint is unpacked to dense weights on ONE device
        // (the dequantizer materializes the whole tensor set at once), so it takes
        // the single-device path whatever the card count.
        let is_bnb = crate::bnb::is_bnb(&cfg.model_dir);
        // Split only when splitting BUYS something. The partition costs a
        // device-to-device copy per forward — measured at roughly a third of the
        // decode rate — and buys the VRAM of a second card. A model that fits one
        // card with room for its context should stay on one card; a model that
        // does not fit should split rather than fail. So: an explicitly named
        // pair (IG_DEVICES=0,1) splits because the user asked, and an
        // auto-detected pair splits only when the weights plus a working reserve
        // will not sit on the roomiest card alone.
        let weights_bytes: u64 = shards
            .iter()
            .filter_map(|p| std::fs::metadata(p).ok())
            .map(|m| m.len())
            .sum();
        // IG_LOCAL_FORCE_SPLIT runs the sharded stack even on ONE card. It buys
        // nothing at runtime; it is how the sharded decoder is CHECKED against
        // candle's own — same card, same kernels, so identical greedy output is
        // identical arithmetic rather than a coincidence of two architectures.
        let force_split = env("IG_LOCAL_FORCE_SPLIT").is_some();
        let split = !is_bnb
            && (force_split && !devices[0].0.is_cpu()
                || devices.len() > 1 && (cfg.devices_pinned || {
                let reserve = 3 * 1024 * 1024 * 1024u64; // KV cache + prefill blocks
                let roomiest = ordinals.iter().map(|i| free_vram(*i)).max().unwrap_or(0);
                let fits_one = roomiest > 0 && roomiest >= weights_bytes + reserve;
                if fits_one && !quiet {
                    eprintln!(
                        "\x1b[2m[local] {:.1} GB of weights fit one card — no split (name both in IG_DEVICES to force it)\x1b[0m",
                        weights_bytes as f64 / (1024.0 * 1024.0 * 1024.0)
                    );
                }
                !fits_one
            }));

        // Not splitting, but several cards are open: take the roomiest, not the
        // first. The first is only an ordinal; the roomiest is where the model
        // will actually have space to think.
        let devices = if split || devices.len() < 2 {
            devices
        } else {
            let mut best = devices;
            best.sort_by_key(|(_, o)| std::cmp::Reverse(o.map(free_vram).unwrap_or(0)));
            best
        };
        let head_device = devices[0].0.clone();
        // bf16 on GPU (the weights' native dtype), f32 on CPU (no bf16 matmul there).
        let dtype = if head_device.is_cuda() { DType::BF16 } else { DType::F32 };
        // The ctx cap is sized from the cards the model actually lands on.
        let ordinals: Vec<usize> = if split {
            ordinals
        } else {
            devices[0].1.into_iter().collect()
        };

        let where_ = if split {
            let names: Vec<String> = ordinals.iter().map(|i| format!("cuda:{i}")).collect();
            names.join(" + ")
        } else {
            match devices[0].1 {
                Some(idx) => format!("cuda:{idx}"),
                None => "cpu".into(),
            }
        };
        if !quiet {
            eprintln!(
                "\x1b[2m[local] loading {} onto {} …\x1b[0m",
                cfg.model_dir, where_
            );
        }
        if is_bnb && devices.len() > 1 && !quiet {
            eprintln!("\x1b[2m[local] bitsandbytes checkpoint — dequantized onto one card, no split\x1b[0m");
        }

        let model = if split {
            let free: Vec<u64> = ordinals.iter().map(|i| free_vram(*i)).collect();
            let m = crate::shard::ShardedQwen3::load(&qcfg, &shards, &devices, &free, dtype, &shape.prefix, quiet)?;
            Backend::Split(m)
        } else {
            // candle's own Qwen3 hardcodes the "model." tensor names, so a
            // wrapped checkpoint can only take the sharded path, which is told
            // its prefix.
            if shape.prefix != "model." {
                return Err(format!(
                    "this checkpoint keeps its text stack under '{}', which candle's Qwen3 loader                      cannot address; name two cards in IG_DEVICES (or set IG_LOCAL_FORCE_SPLIT=1)                      to load it through the sharded stack",
                    shape.prefix
                ));
            }
            let device = head_device.clone();
            let vb = if is_bnb {
                let tensors = crate::bnb::load_dequantized(&shards, dtype, &device, quiet)?;
                VarBuilder::from_tensors(tensors, dtype, &device)
            } else {
                unsafe {
                    VarBuilder::from_mmaped_safetensors(&shards, dtype, &device)
                        .map_err(|e| format!("load weights: {e}"))?
                }
            };
            Backend::Single(
                ModelForCausalLM::new(&qcfg, vb).map_err(|e| format!("build model: {e}"))?,
            )
        };
        if !quiet {
            eprintln!(
                "\x1b[2m[local] model resident ({:.1}s)\x1b[0m",
                t0.elapsed().as_secs_f64()
            );
        }
        let tok_path = format!("{}/tokenizer.json", cfg.model_dir);
        let tokenizer =
            Tokenizer::from_file(&tok_path).map_err(|e| format!("tokenizer {tok_path}: {e}"))?;
        // Qwen3 EOS + the chat turn terminator <|im_end|>.
        let mut eos_ids = vec![151643u32, 151645u32];
        if let Some(id) = tokenizer.token_to_id("<|im_end|>") {
            eos_ids.push(id);
        }
        eos_ids.sort_unstable();
        eos_ids.dedup();
        // Load the model's own Jinja chat template so the prompt is built exactly
        // the way the model was trained (special tokens, the enable_thinking hard
        // switch, glyphs preserved) rather than by a hand-rolled ChatML guess.
        let chat_template = std::fs::read_to_string(format!("{}/tokenizer_config.json", cfg.model_dir))
            .ok()
            .and_then(|s| serde_json::from_str::<serde_json::Value>(&s).ok())
            .and_then(|j| j.get("chat_template").and_then(|t| t.as_str()).map(String::from));
        let template_env = match chat_template {
            Some(tmpl) => {
                let mut jenv = minijinja::Environment::new();
                // Qwen's template calls Python str methods (.split/.startswith/.strip …);
                // pycompat supplies them.
                jenv.set_unknown_method_callback(minijinja_contrib::pycompat::unknown_method_callback);
                jenv.add_template_owned("chat".to_string(), tmpl)
                    .map_err(|e| format!("chat template parse: {e}"))?;
                Some(jenv)
            }
            None => None,
        };
        let prefill_chunk = env_ig("IG_LOCAL_PREFILL_CHUNK", &[])
            .and_then(|s| s.parse::<usize>().ok())
            .filter(|v| *v > 0)
            .unwrap_or(512);
        let ctx_cap = compute_ctx_cap(&qcfg, &ordinals, prefill_chunk, quiet);
        Ok(Engine {
            model,
            tokenizer,
            device: head_device,
            eos_ids,
            model_dir: cfg.model_dir,
            template_env,
            ctx_cap,
            prefill_chunk,
        })
    }

    /// Build the prompt from the model's OWN chat template. `think=false` sets
    /// Jinja `enable_thinking=false`, Qwen3's hard switch: the template emits an
    /// empty `<think></think>` and the model skips reasoning. Rendering the real
    /// template (not a hand-rolled stub) is what preserves the special tokens and
    /// the rare glyphs (> < Σ …) that a naive ChatML string mangled.
    fn apply_template(&self, messages: &[(String, String)], think: bool) -> Result<String, String> {
        let Some(jenv) = self.template_env.as_ref() else {
            // Fallback: minimal ChatML, only if the model ships no template.
            let mut s = String::new();
            for (role, content) in messages {
                let r = if role == "assistant" { "assistant" } else if role == "system" { "system" } else { "user" };
                s.push_str(&format!("<|im_start|>{r}\n{content}<|im_end|>\n"));
            }
            s.push_str("<|im_start|>assistant\n");
            if !think {
                s.push_str("<think>\n\n</think>\n\n");
            }
            return Ok(s);
        };
        let t = jenv.get_template("chat").map_err(|e| format!("chat template: {e}"))?;
        let msgs: Vec<minijinja::Value> = messages
            .iter()
            .map(|(role, content)| minijinja::context! { role => role, content => content })
            .collect();
        t.render(minijinja::context! {
            messages => msgs,
            add_generation_prompt => true,
            enable_thinking => think,
        })
        .map_err(|e| format!("chat template render: {e}"))
    }

    /// Feed `ids` through the model at `offset`, in CHUNKS, and return the
    /// logits for the last position.
    ///
    /// The prefill is where a long prompt dies: without flash-attn the attention
    /// scores are a [heads × q_len × kv_len] tensor, so a one-shot prefill of an
    /// 8 k prompt allocates it squared — several GB, twice (scores and softmax) —
    /// and the card OOMs before a single token is decoded. Feeding the prompt in
    /// blocks makes that q_len the BLOCK length instead of the whole prompt: the
    /// KV cache still grows to the full prompt (it must), but the transient drops
    /// from O(seq²) to O(chunk × seq). The arithmetic is identical — attention at
    /// position i sees exactly the same keys either way — which is why this is a
    /// memory fix and not an approximation.
    fn forward_chunked(&mut self, ids: &[u32], mut offset: usize) -> Result<Tensor, String> {
        let chunk = self.prefill_chunk.max(1);
        let mut last: Option<Tensor> = None;
        for block in ids.chunks(chunk) {
            let input = Tensor::new(block, &self.device)
                .and_then(|t| t.unsqueeze(0))
                .map_err(|e| format!("input tensor: {e}"))?;
            last = Some(
                self.model
                    .forward(&input, offset)
                    .map_err(|e| format!("forward: {e}"))?,
            );
            offset += block.len();
        }
        last.ok_or_else(|| "empty forward".to_string())
    }

    fn generate(
        &mut self,
        messages: &[(String, String)],
        max_tokens: usize,
        temperature: f64,
        think: bool,
    ) -> Result<String, String> {
        let prompt = self.apply_template(messages, think)?;
        // The template already inserts every special token as text; add_special_tokens
        // = false avoids a double BOS while the tokenizer still matches <|im_start|> &c.
        let enc = self
            .tokenizer
            .encode(prompt, false)
            .map_err(|e| format!("tokenize: {e}"))?;
        let mut tokens: Vec<u32> = enc.get_ids().to_vec();

        // Prefill OOM guard. Without flash-attn, prefill allocates a
        // [heads × seq × seq] f32 scores tensor, so long prompts blow the GPU
        // (~7 GB at 10k tokens on top of the weights). A jam grows its prompt
        // every round as tool results accumulate, so it WILL cross the line and
        // kill the series with CUDA_ERROR_OUT_OF_MEMORY. Cap the prefill length:
        // keep the HEAD (system prompt / framing) and the TAIL (current question
        // and most recent results), drop the middle, and say so. Configurable via
        // IG_LOCAL_CTX. With flash-attn the prefill is O(seq) not O(seq²) and
        // the constraint becomes the KV cache, so the cap was sized once at load
        // (compute_ctx_cap) from the free VRAM left after the weights — a 4B on
        // a 12 GB card gets ~10-14k, not a hopeful 32k that OOMs mid-series. The
        // agentic loop needs the WHOLE prompt (dropping the middle drops the tool
        // results the model must react to), so flash-attn is what makes local jam
        // actually work, not just survive.
        let ctx_cap = self.ctx_cap;
        if tokens.len() > ctx_cap {
            // Keep the head (framing) but bias hard toward the tail: in an agentic
            // loop the recent tool results are what the next step must see.
            let head = ctx_cap / 4;
            let tail = ctx_cap - head;
            let dropped = tokens.len() - ctx_cap;
            let mut kept = Vec::with_capacity(ctx_cap);
            kept.extend_from_slice(&tokens[..head]);
            kept.extend_from_slice(&tokens[tokens.len() - tail..]);
            eprintln!(
                "\x1b[2m[local] prompt {} tok > ctx cap {} — kept head {}+tail {}, dropped {} in the middle (raise IG_LOCAL_CTX, add a card to IG_DEVICES, or build with flash-attn)\x1b[0m",
                tokens.len(), ctx_cap, head, tail, dropped
            );
            tokens = kept;
        }
        if tokens.is_empty() {
            return Err("empty prompt after tokenization".into());
        }
        self.model.clear_kv_cache();
        // Sampling per the model's own card. Qwen3: thinking T=0.6/top-p 0.95,
        // non-thinking T=0.7/top-p 0.8. Qwen3.5 raises the thinking temperature to
        // 1.0 and asks for a PRESENCE penalty of 1.5 in both modes. Both keep
        // top-k=20. Greedy / plain temp sampling is what caused the endless
        // repetition seen in jam. A caller temperature ~0 still means greedy.
        let card = SamplingCard::for_model(&self.model_dir);
        let (temp, top_p) = card.mode(think);
        let temp = if temperature > f64::EPSILON { temperature } else { temp };
        let mut logits_proc = if temperature <= f64::EPSILON {
            LogitsProcessor::from_sampling(0, candle_transformers::generation::Sampling::ArgMax)
        } else {
            LogitsProcessor::from_sampling(
                0,
                candle_transformers::generation::Sampling::TopKThenTopP { k: 20, p: top_p, temperature: temp },
            )
        };
        // Two different penalties, because the cards ask for different things.
        // A REPEAT penalty divides the logit of a recently seen token — Qwen3's
        // route out of the "records the results, records the results" loop. A
        // PRESENCE penalty subtracts a constant from every token already emitted,
        // once, however often it appeared; that is what Qwen3.5 asks for, and it
        // is additive rather than multiplicative, so it cannot be spelled as a
        // repeat penalty. Both are honoured, each where its card calls for it.
        let repeat_penalty: f32 = env_ig("IG_LOCAL_REPEAT_PENALTY", &["MODOT_LOCAL_REPEAT_PENALTY"])
            .and_then(|s| s.parse().ok())
            .unwrap_or(card.repeat_penalty);
        let presence_penalty: f32 = env_ig("IG_LOCAL_PRESENCE_PENALTY", &[])
            .and_then(|s| s.parse().ok())
            .unwrap_or(card.presence_penalty);
        let repeat_window: usize = 128;

        let mut out_ids: Vec<u32> = Vec::new();
        let mut offset = 0usize;
        // Cap so a runaway generation cannot spin forever, at the ceiling the
        // model's card asks for: Qwen3.5 wants room for 32768 tokens on an
        // ordinary query because its reasoning runs long, where 8192 was enough
        // for Qwen3. The KV cost of that headroom is already reserved in the
        // context cap.
        let cap = max_tokens.clamp(1, card.max_output);

        // Live progress: the model is silent for seconds while it loads kernels
        // and chews the prompt, then streams tokens. Print to STDERR so the
        // streamed text never contaminates the answer the caller reads on the
        // return value / stdout. Default on; IG_LOCAL_STREAM=0 silences it.
        let stream = env_ig("IG_LOCAL_STREAM", &["MODOT_LOCAL_STREAM"]).map(|v| v != "0").unwrap_or(true);
        let mut printed_len = 0usize; // chars already streamed (incremental decode)
        let t_start = std::time::Instant::now();
        let mut first_token_at: Option<std::time::Duration> = None;
        if stream {
            eprint!(
                "\x1b[2m[local · {} prompt tok · thinking…]\x1b[0m ",
                tokens.len()
            );
            let _ = std::io::Write::flush(&mut std::io::stderr());
        }

        for _ in 0..cap {
            let ctx = if offset == 0 { &tokens[..] } else { &tokens[tokens.len() - 1..] };
            let logits = self.forward_chunked(ctx, offset)?;
            let logits = logits
                .squeeze(0)
                .and_then(|t| t.squeeze(0))
                .and_then(|t| t.to_dtype(DType::F32))
                .map_err(|e| format!("logits reshape: {e}"))?;
            // Penalize tokens seen in the recent window before sampling.
            let logits = if repeat_penalty != 1.0 && !out_ids.is_empty() {
                let start = out_ids.len().saturating_sub(repeat_window);
                candle_transformers::utils::apply_repeat_penalty(&logits, repeat_penalty, &out_ids[start..])
                    .map_err(|e| format!("repeat penalty: {e}"))?
            } else {
                logits
            };
            let logits = if presence_penalty != 0.0 && !out_ids.is_empty() {
                apply_presence_penalty(&logits, presence_penalty, &out_ids)
                    .map_err(|e| format!("presence penalty: {e}"))?
            } else {
                logits
            };
            let next = logits_proc.sample(&logits).map_err(|e| format!("sample: {e}"))?;
            offset = tokens.len();
            if self.eos_ids.contains(&next) {
                break;
            }
            tokens.push(next);
            out_ids.push(next);
            if first_token_at.is_none() {
                first_token_at = Some(t_start.elapsed());
            }
            // Incremental decode: re-decode the whole output and stream only the
            // NEW suffix. A rare glyph (> < Σ …) spans several byte-level BPE
            // tokens, so a decode taken before the last byte-token arrives ends in
            // the U+FFFD replacement char. HOLD that incomplete trailing char
            // (don't print up to it) until the completing token lands and it
            // decodes to the real glyph; otherwise the stream shows � for symbols
            // the final decode gets right.
            if stream {
                if let Ok(full) = self.tokenizer.decode(&out_ids, true) {
                    // safe boundary = end of text, unless it ends mid-multi-byte
                    // char (a trailing replacement char): then stop before it.
                    let safe = if full.ends_with('\u{FFFD}') {
                        full.rfind('\u{FFFD}').unwrap_or(full.len())
                    } else {
                        full.len()
                    };
                    // LINE-buffered: emit only up to the last completed newline,
                    // holding the partial line — whole lines land at once instead
                    // of a letter-by-letter trickle. The held tail flushes after
                    // the loop.
                    let line_end = full[..safe].rfind('\n').map(|i| i + 1).unwrap_or(0);
                    if line_end > printed_len && full.is_char_boundary(printed_len) {
                        eprint!("{}", &full[printed_len..line_end]);
                        let _ = std::io::Write::flush(&mut std::io::stderr());
                        printed_len = line_end;
                    }
                }
            }
        }
        // Flush the held partial final line (generation rarely ends on a newline).
        if stream {
            if let Ok(full) = self.tokenizer.decode(&out_ids, true) {
                if full.len() > printed_len && full.is_char_boundary(printed_len) {
                    eprint!("{}", &full[printed_len..]);
                    let _ = std::io::Write::flush(&mut std::io::stderr());
                }
            }
        }
        let text = self
            .tokenizer
            .decode(&out_ids, true)
            .map_err(|e| format!("decode: {e}"))?;
        // The <think> block is the model's private register: it hedges, second-guesses,
        // and narrates TOOL lines it is not calling. It stays visible in the stream
        // (stderr) but is CUT from the returned text — otherwise the reasoning enters
        // the transcript as the operator's voice, the harness parses its narrated
        // TOOL lines as real calls, and its wobble reaches the vessel. Multi-turn
        // history stays think-free too (Qwen's own template strips prior think blocks).
        let text = match text.rsplit_once("</think>") {
            Some((_, after)) => after.trim_start().to_string(),
            None => text,
        };
        if stream {
            let secs = t_start.elapsed().as_secs_f64().max(1e-6);
            let ttft = first_token_at.map(|d| d.as_secs_f64()).unwrap_or(secs);
            eprintln!(
                "\n\x1b[2m[local · {} tok · {:.1} tok/s · first token {:.1}s]\x1b[0m",
                out_ids.len(),
                out_ids.len() as f64 / secs,
                ttft
            );
            let _ = std::io::Write::flush(&mut std::io::stderr());
        }
        Ok(text)
    }
}

static ENGINE: OnceLock<Mutex<Result<Engine, String>>> = OnceLock::new();

/// One-line description of the resident engine, for the startup banner.
pub fn describe() -> String {
    let cfg = local_cfg();
    let dev = if cfg.force_cpu {
        "cpu".to_string()
    } else if cfg.devices.is_empty() {
        "every cuda card present (else cpu)".to_string()
    } else {
        let names: Vec<String> = cfg.devices.iter().map(|i| format!("cuda:{i}")).collect();
        format!("{} (else cpu)", names.join(" + "))
    };
    format!("local candle · {} · {}", cfg.model_dir, dev)
}

/// Generate a completion fully in-process. Loads the model on first call and
/// keeps it resident. Returns Ok(text) or Err(reason); the caller maps Err into
/// the same InferResult error path the HTTP providers use.
pub fn generate(
    messages: &[(String, String)],
    max_tokens: usize,
    temperature: f64,
    think: bool,
) -> Result<String, String> {
    let cell = ENGINE.get_or_init(|| Mutex::new(Engine::load()));
    let mut guard = cell.lock().map_err(|_| "local engine mutex poisoned".to_string())?;
    match guard.as_mut() {
        Ok(engine) => engine.generate(messages, max_tokens, temperature, think),
        Err(e) => Err(format!("local model failed to load: {e}")),
    }
}

/// Exposed so a `--warm` / status path can confirm the model directory resolves
/// without paying for a full generation.
pub fn model_dir() -> String {
    local_cfg().model_dir
}

/// Silence dead-code warnings for helpers reserved for the status path.
#[allow(dead_code)]
fn _keep(e: &Engine) -> &str {
    &e.model_dir
}

#[cfg(test)]
mod template_tests {
    /// Render a model's own chat template through the SAME minijinja setup the
    /// engine uses, so what a template needs is measured rather than assumed.
    fn render_msgs(tmpl: &str, msgs: Vec<minijinja::Value>, think: bool) -> Result<String, String> {
        let mut jenv = minijinja::Environment::new();
        jenv.set_unknown_method_callback(minijinja_contrib::pycompat::unknown_method_callback);
        jenv.add_template_owned("chat".to_string(), tmpl.to_string())
            .map_err(|e| format!("parse: {e:#}"))?;
        let t = jenv.get_template("chat").map_err(|e| format!("get: {e:#}"))?;
        t.render(minijinja::context! {
            messages => msgs,
            add_generation_prompt => true,
            enable_thinking => think,
        })
        .map_err(|e| format!("render: {e:#}"))
    }

    fn render(tmpl: &str) -> Result<String, String> {
        let mut jenv = minijinja::Environment::new();
        jenv.set_unknown_method_callback(minijinja_contrib::pycompat::unknown_method_callback);
        jenv.add_template_owned("chat".to_string(), tmpl.to_string())
            .map_err(|e| format!("parse: {e:#}"))?;
        let t = jenv.get_template("chat").map_err(|e| format!("get: {e:#}"))?;
        let msgs = vec![
            minijinja::context! { role => "system", content => "You are a golem." },
            minijinja::context! { role => "user", content => "Name the four values." },
        ];
        t.render(minijinja::context! {
            messages => msgs,
            add_generation_prompt => true,
            enable_thinking => false,
        })
        .map_err(|e| format!("render: {e:#}"))
    }

    /// The agent's real shape: a multi-turn history where an earlier assistant
    /// turn carries a think block. Qwen3.5's template splits on `</think>`,
    /// walks the messages in reverse to find the last user query, and drops the
    /// reasoning from every turn before it — the branches a single-turn render
    /// never touches.
    #[test]
    fn a_hybrid_checkpoint_is_refused_by_name_not_by_a_wrong_answer() {
        let dir = dirs::home_dir().unwrap().join(".modelz/3p54b");
        if !dir.join("config.json").exists() {
            eprintln!("no qwen3.5 checkpoint present; skipping");
            return;
        }
        let err = super::read_model_shape(dir.to_str().unwrap())
            .err()
            .expect("a hybrid must not load into a full-attention stack");
        eprintln!("REFUSED: {err}");
        assert!(err.contains("linear_attention"), "the refusal must name what is missing: {err}");
    }

    #[test]
    fn the_sampling_card_follows_the_model() {
        let q3 = super::SamplingCard::for_model("/home/x/models/Qwen3-1.7B");
        assert_eq!(q3.mode(true), (0.6, 0.95));
        assert_eq!(q3.presence_penalty, 0.0);
        let q35 = super::SamplingCard::for_model("/home/x/models/Qwen3.5-27B");
        assert_eq!(q35.mode(true), (1.0, 0.95));
        assert_eq!(q35.presence_penalty, 1.5);
        assert_eq!(q35.repeat_penalty, 1.0);
        assert_eq!(q35.max_output, 32768);
        assert_eq!(q3.max_output, 8192);
    }

    #[test]
    fn qwen35_template_survives_a_multi_turn_history() {
        let path = dirs::home_dir().unwrap().join(".modelz/3p54b/chat_template.jinja");
        if !path.exists() {
            eprintln!("no qwen3.5 template present; skipping");
            return;
        }
        let tmpl = std::fs::read_to_string(&path).unwrap();
        let msgs = vec![
            minijinja::context! { role => "system", content => "You are a golem." },
            minijinja::context! { role => "user", content => "First question." },
            minijinja::context! { role => "assistant", content => "<think>\nweighing it\n</think>\n\nFirst answer." },
            minijinja::context! { role => "user", content => "Second question." },
        ];
        let out = match render_msgs(&tmpl, msgs, true) {
            Ok(o) => o,
            Err(e) => panic!("multi-turn render failed: {e}"),
        };
        eprintln!("MULTI-TURN:\n{out}");
        // The prior turn's reasoning is dropped; the live turn opens a think block.
        assert!(!out.contains("weighing it"), "prior reasoning leaked into the prompt");
        assert!(out.ends_with("<think>\n"), "expected an open think block, got:\n{out}");
    }

    #[test]
    fn qwen35_template_through_our_engine() {
        let path = dirs::home_dir().unwrap().join(".modelz/3p54b/chat_template.jinja");
        if !path.exists() {
            eprintln!("no qwen3.5 template present; skipping");
            return;
        }
        let tmpl = std::fs::read_to_string(&path).unwrap();
        match render(&tmpl) {
            Ok(out) => eprintln!("RENDERED OK:\n{out}"),
            Err(e) => panic!("qwen3.5 template does not render here: {e}"),
        }
    }
}
