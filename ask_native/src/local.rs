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
    device_index: usize,
    force_cpu: bool,
}

fn env(k: &str) -> Option<String> {
    std::env::var(k).ok().filter(|v| !v.is_empty())
}

fn expand(p: &str) -> String {
    if let Some(rest) = p.strip_prefix("~/") {
        if let Some(h) = dirs::home_dir() {
            return h.join(rest).to_string_lossy().into_owned();
        }
    }
    p.to_string()
}

fn local_cfg() -> LocalCfg {
    // Align candle's device indexing with nvidia-smi so MODOT_LOCAL_DEVICE means
    // what the user sees. Without this, CUDA's default (fastest-first) ordering
    // put the sm_75 2080S at index 1 and the sm_86 PTX failed to JIT there. Set
    // only if the user has not pinned it themselves.
    if env("CUDA_DEVICE_ORDER").is_none() {
        std::env::set_var("CUDA_DEVICE_ORDER", "PCI_BUS_ID");
    }
    LocalCfg {
        // MODOT_LOCAL_MODEL_DIR, else Qwen3-1.7B: ~4 GB in bf16, fits the 3060's
        // 12 GB with room for MoDoT's long-context (non-flash, quadratic) attention.
        // The 4B OOM'd here; the 1.7B is the working default.
        model_dir: expand(
            &env("MODOT_LOCAL_MODEL_DIR").unwrap_or_else(|| "~/models/Qwen3-1.7B".to_string()),
        ),
        // The 3060 is index 1 under PCI_BUS_ID ordering (the sm_86 card the
        // kernels are built for); override with MODOT_LOCAL_DEVICE.
        device_index: env("MODOT_LOCAL_DEVICE")
            .and_then(|s| s.parse().ok())
            .unwrap_or(1),
        force_cpu: env("MODOT_LOCAL_CPU").is_some(),
    }
}

fn pick_device(cfg: &LocalCfg) -> candle_core::Result<Device> {
    if cfg.force_cpu {
        return Ok(Device::Cpu);
    }
    match Device::cuda_if_available(cfg.device_index) {
        Ok(d) => Ok(d),
        Err(_) => Ok(Device::Cpu),
    }
}

/// A loaded model held resident for the process lifetime.
struct Engine {
    model: ModelForCausalLM,
    tokenizer: Tokenizer,
    device: Device,
    eos_ids: Vec<u32>,
    model_dir: String,
}

fn read_qwen3_config(dir: &str) -> Result<Qwen3Config, String> {
    let path = format!("{dir}/config.json");
    let raw = std::fs::read_to_string(&path).map_err(|e| format!("read {path}: {e}"))?;
    let j: serde_json::Value =
        serde_json::from_str(&raw).map_err(|e| format!("parse {path}: {e}"))?;
    let g = |k: &str| j.get(k);
    let u = |k: &str, d: usize| g(k).and_then(|v| v.as_u64()).map(|x| x as usize).unwrap_or(d);
    let f = |k: &str, d: f64| g(k).and_then(|v| v.as_f64()).unwrap_or(d);
    let b = |k: &str, d: bool| g(k).and_then(|v| v.as_bool()).unwrap_or(d);
    let hidden = u("hidden_size", 2560);
    let heads = u("num_attention_heads", 32);
    Ok(Qwen3Config {
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
        rope_theta: f("rope_theta", 1_000_000.0),
        rms_norm_eps: f("rms_norm_eps", 1e-6),
        use_sliding_window: false,
        hidden_act: candle_nn::Activation::Silu,
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

impl Engine {
    fn load() -> Result<Engine, String> {
        let cfg = local_cfg();
        let quiet = env("MODOT_LOCAL_STREAM").map(|v| v == "0").unwrap_or(false);
        let t0 = std::time::Instant::now();
        let device = pick_device(&cfg).map_err(|e| format!("device: {e}"))?;
        let where_ = if device.is_cuda() {
            format!("cuda:{}", cfg.device_index)
        } else {
            "cpu".into()
        };
        if !quiet {
            eprintln!(
                "\x1b[2m[local] loading {} onto {} …\x1b[0m",
                cfg.model_dir, where_
            );
        }
        let qcfg = read_qwen3_config(&cfg.model_dir)?;
        let shards = safetensor_shards(&cfg.model_dir)?;
        // bf16 on GPU (the weights' native dtype), f32 on CPU (no bf16 matmul there).
        let dtype = if device.is_cuda() { DType::BF16 } else { DType::F32 };
        let vb = unsafe {
            VarBuilder::from_mmaped_safetensors(&shards, dtype, &device)
                .map_err(|e| format!("load weights: {e}"))?
        };
        let model = ModelForCausalLM::new(&qcfg, vb).map_err(|e| format!("build model: {e}"))?;
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
        Ok(Engine { model, tokenizer, device, eos_ids, model_dir: cfg.model_dir })
    }

    /// Render OpenAI-style messages into the Qwen ChatML template.
    fn apply_template(&self, messages: &[(String, String)]) -> String {
        let mut s = String::new();
        for (role, content) in messages {
            let r = match role.as_str() {
                "assistant" => "assistant",
                "system" => "system",
                _ => "user",
            };
            s.push_str(&format!("<|im_start|>{r}\n{content}<|im_end|>\n"));
        }
        s.push_str("<|im_start|>assistant\n");
        s
    }

    fn generate(
        &mut self,
        messages: &[(String, String)],
        max_tokens: usize,
        temperature: f64,
    ) -> Result<String, String> {
        let prompt = self.apply_template(messages);
        let enc = self
            .tokenizer
            .encode(prompt, true)
            .map_err(|e| format!("tokenize: {e}"))?;
        let mut tokens: Vec<u32> = enc.get_ids().to_vec();
        if tokens.is_empty() {
            return Err("empty prompt after tokenization".into());
        }
        self.model.clear_kv_cache();
        // Deterministic when temperature is ~0, else sampled.
        let mut logits_proc = if temperature <= f64::EPSILON {
            LogitsProcessor::new(0, None, None)
        } else {
            LogitsProcessor::new(0, Some(temperature), Some(0.95))
        };

        let mut out_ids: Vec<u32> = Vec::new();
        let mut offset = 0usize;
        // cap so a runaway generation cannot spin forever
        let cap = max_tokens.clamp(1, 8192);

        // Live progress: the model is silent for seconds while it loads kernels
        // and chews the prompt, then streams tokens. Print to STDERR so the
        // streamed text never contaminates the answer the caller reads on the
        // return value / stdout. Default on; MODOT_LOCAL_STREAM=0 silences it.
        let stream = env("MODOT_LOCAL_STREAM").map(|v| v != "0").unwrap_or(true);
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
            let input = Tensor::new(ctx, &self.device)
                .and_then(|t| t.unsqueeze(0))
                .map_err(|e| format!("input tensor: {e}"))?;
            let logits = self
                .model
                .forward(&input, offset)
                .map_err(|e| format!("forward: {e}"))?;
            let logits = logits
                .squeeze(0)
                .and_then(|t| t.squeeze(0))
                .and_then(|t| t.to_dtype(DType::F32))
                .map_err(|e| format!("logits reshape: {e}"))?;
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
            // NEW suffix. Decoding the growing prefix (rather than one token at a
            // time) is what makes byte-level BPE multi-byte chars render correctly
            // once their completing token arrives.
            if stream {
                if let Ok(full) = self.tokenizer.decode(&out_ids, true) {
                    if full.len() > printed_len && full.is_char_boundary(printed_len) {
                        eprint!("{}", &full[printed_len..]);
                        let _ = std::io::Write::flush(&mut std::io::stderr());
                        printed_len = full.len();
                    }
                }
            }
        }
        let text = self
            .tokenizer
            .decode(&out_ids, true)
            .map_err(|e| format!("decode: {e}"))?;
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
    } else {
        format!("cuda:{} (else cpu)", cfg.device_index)
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
) -> Result<String, String> {
    let cell = ENGINE.get_or_init(|| Mutex::new(Engine::load()));
    let mut guard = cell.lock().map_err(|_| "local engine mutex poisoned".to_string())?;
    match guard.as_mut() {
        Ok(engine) => engine.generate(messages, max_tokens, temperature),
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
