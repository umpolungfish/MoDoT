//! Pipeline-sharded Qwen3: one model, its decoder stack split across every
//! CUDA device present, each layer resident on exactly one card.
//!
//! Two 12 GB cards are not 24 GB of pooled memory — there is no unified address
//! space here and no NVLink. What there IS is a partition: layers 0..k on the
//! first card, k..N on the second, and a hidden state of shape [batch, seq,
//! hidden] crossing the boundary once per forward. That crossing is the only
//! traffic, it is O(seq × hidden) (a few MB at prefill, a few KB per decoded
//! token), and it costs a device-to-device copy — against which each card now
//! holds half the weights and half the KV cache. An 8 B model that does not fit
//! one card with usable context fits two with room to think.
//!
//! candle's own `qwen3::Model` builds every layer on the VarBuilder's single
//! device and keeps `DecoderLayer` private, so the stack is rebuilt here from
//! the public pieces (`candle_nn` linear / rms_norm / rope / ConcatKvCache,
//! `candle_transformers::utils::repeat_kv`). The arithmetic is the same
//! arithmetic; what is new is that each layer carries the device it lives on
//! and the hidden state is moved to that device before entering it.
//!
//! One card is still the single-device path in `local.rs` — that route is
//! proven and stays untouched. This module is reached only when two or more
//! CUDA devices are actually open.

use candle_core::{DType, Device, IndexOp, Module, Result, Tensor};
use candle_nn::kv_cache::ConcatKvCache;
use candle_nn::{Activation, Linear, RmsNorm, VarBuilder};
use candle_transformers::models::qwen3::Config;
use candle_transformers::utils::repeat_kv;
use std::sync::Arc;

/// RoPE tables, built once per DEVICE (not per layer): the sin/cos are the same
/// numbers on every card, but a tensor belongs to the device it was created on,
/// so each stage holds its own copy and shares it across the layers there.
#[derive(Debug)]
struct Rotary {
    sin: Tensor,
    cos: Tensor,
}

impl Rotary {
    fn new(dtype: DType, cfg: &Config, dev: &Device) -> Result<Self> {
        let dim = cfg.head_dim;
        let max_seq_len = cfg.max_position_embeddings;
        let inv_freq: Vec<f32> = (0..dim)
            .step_by(2)
            .map(|i| 1f32 / cfg.rope_theta.powf(i as f64 / dim as f64) as f32)
            .collect();
        let n = inv_freq.len();
        let inv_freq = Tensor::from_vec(inv_freq, (1, n), dev)?.to_dtype(DType::F32)?;
        let t = Tensor::arange(0u32, max_seq_len as u32, dev)?
            .to_dtype(DType::F32)?
            .reshape((max_seq_len, 1))?;
        let freqs = t.matmul(&inv_freq)?;
        Ok(Self {
            sin: freqs.sin()?.to_dtype(dtype)?,
            cos: freqs.cos()?.to_dtype(dtype)?,
        })
    }

    fn apply(&self, q: &Tensor, k: &Tensor, offset: usize) -> Result<(Tensor, Tensor)> {
        let (_, _, seq_len, _) = q.dims4()?;
        let cos = self.cos.narrow(0, offset, seq_len)?;
        let sin = self.sin.narrow(0, offset, seq_len)?;
        let q = candle_nn::rotary_emb::rope(&q.contiguous()?, &cos, &sin)?;
        let k = candle_nn::rotary_emb::rope(&k.contiguous()?, &cos, &sin)?;
        Ok((q, k))
    }
}

fn linear(in_d: usize, out_d: usize, bias: bool, vb: VarBuilder) -> Result<Linear> {
    if bias {
        candle_nn::linear(in_d, out_d, vb)
    } else {
        candle_nn::linear_no_bias(in_d, out_d, vb)
    }
}

#[derive(Debug)]
struct Mlp {
    gate: Linear,
    up: Linear,
    down: Linear,
    act: Activation,
}

impl Mlp {
    fn new(cfg: &Config, vb: VarBuilder) -> Result<Self> {
        Ok(Self {
            gate: candle_nn::linear_no_bias(cfg.hidden_size, cfg.intermediate_size, vb.pp("gate_proj"))?,
            up: candle_nn::linear_no_bias(cfg.hidden_size, cfg.intermediate_size, vb.pp("up_proj"))?,
            down: candle_nn::linear_no_bias(cfg.intermediate_size, cfg.hidden_size, vb.pp("down_proj"))?,
            act: cfg.hidden_act,
        })
    }

    fn forward(&self, x: &Tensor) -> Result<Tensor> {
        let lhs = x.apply(&self.gate)?.apply(&self.act)?;
        let rhs = x.apply(&self.up)?;
        (lhs * rhs)?.apply(&self.down)
    }
}

#[derive(Debug)]
struct Attention {
    q_proj: Linear,
    k_proj: Linear,
    v_proj: Linear,
    o_proj: Linear,
    q_norm: RmsNorm,
    k_norm: RmsNorm,
    num_heads: usize,
    num_kv_heads: usize,
    num_kv_groups: usize,
    head_dim: usize,
    hidden_size: usize,
    rotary: Arc<Rotary>,
    kv_cache: ConcatKvCache,
}

impl Attention {
    fn new(cfg: &Config, rotary: Arc<Rotary>, vb: VarBuilder) -> Result<Self> {
        let head_dim = cfg.head_dim;
        let num_heads = cfg.num_attention_heads;
        let num_kv_heads = cfg.num_key_value_heads;
        Ok(Self {
            q_proj: linear(cfg.hidden_size, num_heads * head_dim, cfg.attention_bias, vb.pp("q_proj"))?,
            k_proj: linear(cfg.hidden_size, num_kv_heads * head_dim, cfg.attention_bias, vb.pp("k_proj"))?,
            v_proj: linear(cfg.hidden_size, num_kv_heads * head_dim, cfg.attention_bias, vb.pp("v_proj"))?,
            o_proj: linear(num_heads * head_dim, cfg.hidden_size, cfg.attention_bias, vb.pp("o_proj"))?,
            q_norm: candle_nn::rms_norm(head_dim, cfg.rms_norm_eps, vb.pp("q_norm"))?,
            k_norm: candle_nn::rms_norm(head_dim, cfg.rms_norm_eps, vb.pp("k_norm"))?,
            num_heads,
            num_kv_heads,
            num_kv_groups: num_heads / num_kv_heads.max(1),
            head_dim,
            // The config's hidden_size is not always the attention output width;
            // heads × head_dim is.
            hidden_size: head_dim * num_heads,
            rotary,
            kv_cache: ConcatKvCache::new(2),
        })
    }

    fn forward(&mut self, x: &Tensor, mask: Option<&Tensor>, offset: usize) -> Result<Tensor> {
        let (b, l, _) = x.dims3()?;
        let q = self.q_proj.forward(x)?;
        let k = self.k_proj.forward(x)?;
        let v = self.v_proj.forward(x)?;

        let q = q.reshape((b, l, self.num_heads, self.head_dim))?.transpose(1, 2)?;
        let k = k.reshape((b, l, self.num_kv_heads, self.head_dim))?.transpose(1, 2)?;
        let v = v.reshape((b, l, self.num_kv_heads, self.head_dim))?.transpose(1, 2)?;

        // Per-head RMSNorm, Qwen3's addition over Qwen2.
        let q = self.q_norm.forward(&q.flatten(0, 2)?)?
            .reshape((b, self.num_heads, l, self.head_dim))?;
        let k = self.k_norm.forward(&k.flatten(0, 2)?)?
            .reshape((b, self.num_kv_heads, l, self.head_dim))?;

        let (q, k) = self.rotary.apply(&q, &k, offset)?;
        let (k, v) = self.kv_cache.append(&k, &v)?;

        let k = repeat_kv(k, self.num_kv_groups)?.contiguous()?;
        let v = repeat_kv(v, self.num_kv_groups)?.contiguous()?;
        let scale = 1.0 / (self.head_dim as f64).sqrt();
        let mut scores = (q.matmul(&k.transpose(2, 3)?)? * scale)?;
        if let Some(m) = mask {
            scores = scores.broadcast_add(m)?;
        }
        let probs = candle_nn::ops::softmax_last_dim(&scores)?;
        let ctx = probs.matmul(&v)?;
        ctx.transpose(1, 2)?
            .reshape((b, l, self.hidden_size))?
            .apply(&self.o_proj)
    }

    fn clear_kv_cache(&mut self) {
        self.kv_cache.reset();
    }
}

#[derive(Debug)]
struct DecoderLayer {
    attn: Attention,
    mlp: Mlp,
    ln1: RmsNorm,
    ln2: RmsNorm,
}

impl DecoderLayer {
    fn new(cfg: &Config, rotary: Arc<Rotary>, vb: VarBuilder) -> Result<Self> {
        Ok(Self {
            attn: Attention::new(cfg, rotary, vb.pp("self_attn"))?,
            mlp: Mlp::new(cfg, vb.pp("mlp"))?,
            ln1: candle_nn::rms_norm(cfg.hidden_size, cfg.rms_norm_eps, vb.pp("input_layernorm"))?,
            ln2: candle_nn::rms_norm(cfg.hidden_size, cfg.rms_norm_eps, vb.pp("post_attention_layernorm"))?,
        })
    }

    fn forward(&mut self, x: &Tensor, mask: Option<&Tensor>, offset: usize) -> Result<Tensor> {
        let h = self.ln1.forward(x)?;
        let h = self.attn.forward(&h, mask, offset)?;
        let x = (x + h)?;
        let h2 = self.ln2.forward(&x)?;
        let h2 = self.mlp.forward(&h2)?;
        x + h2
    }

    fn clear_kv_cache(&mut self) {
        self.attn.clear_kv_cache();
    }
}

/// One pipeline stage: a contiguous run of decoder layers on a single device.
struct Stage {
    device: Device,
    layers: Vec<DecoderLayer>,
}

/// The whole model, split across stages. Embedding and the final norm + lm_head
/// live on the FIRST stage's device: the tied-weights case shares `lm_head` with
/// `embed_tokens`, and the tensor that comes back for the head is a single
/// position (`[b, 1, hidden]`), so the return trip is a few KB.
pub struct ShardedQwen3 {
    embed: candle_nn::Embedding,
    stages: Vec<Stage>,
    norm: RmsNorm,
    lm_head: Linear,
    head_device: Device,
    dtype: DType,
}

/// How many layers each device carries, proportional to the free VRAM it
/// reports — an unequal pair (a 4070 next to a 3060 with a desktop on it) must
/// not be split down the middle, or the busier card OOMs while the other idles.
/// The first stage additionally holds the embedding and the head, so its share
/// is charged that cost before the proportion is taken.
pub fn plan_stages(num_layers: usize, free_bytes: &[u64], head_bytes: u64, layer_bytes: u64) -> Vec<usize> {
    let n = free_bytes.len();
    if n == 0 {
        return vec![];
    }
    if n == 1 {
        return vec![num_layers];
    }
    // Budget per card: free VRAM, less a fixed working reserve for activations
    // and the KV cache, less the head cost on card 0.
    let reserve: u64 = 1024 * 1024 * 1024;
    let budgets: Vec<f64> = free_bytes
        .iter()
        .enumerate()
        .map(|(i, &f)| {
            let charged = reserve + if i == 0 { head_bytes } else { 0 };
            f.saturating_sub(charged) as f64
        })
        .collect();
    let total: f64 = budgets.iter().sum();
    if total <= 0.0 {
        // Nothing reports room; fall back to an even split rather than refusing.
        let base = num_layers / n;
        let mut out = vec![base; n];
        for slot in out.iter_mut().take(num_layers - base * n) {
            *slot += 1;
        }
        return out;
    }
    // Proportional allocation, then hand the remainder to the largest budgets.
    let mut counts: Vec<usize> = budgets
        .iter()
        .map(|b| ((b / total) * num_layers as f64).floor() as usize)
        .collect();
    let mut assigned: usize = counts.iter().sum();
    while assigned < num_layers {
        // Give the next layer to whichever card has the most budget per layer
        // already assigned — the least loaded card in its own terms.
        let (best, _) = budgets
            .iter()
            .enumerate()
            .map(|(i, b)| (i, b / (counts[i] + 1) as f64))
            .fold((0usize, f64::MIN), |acc, (i, s)| if s > acc.1 { (i, s) } else { acc });
        counts[best] += 1;
        assigned += 1;
    }
    // A stage that ends up empty is a device that would be opened, warmed and
    // never used; drop it to zero and let the others carry it — the caller
    // filters empty stages out.
    let _ = layer_bytes;
    counts
}

impl ShardedQwen3 {
    /// Build the sharded model. `devices` must hold two or more open devices;
    /// `shards` are the safetensors files; `free` is the free VRAM per device
    /// in bytes (0 where unknown).
    pub fn load(
        cfg: &Config,
        shards: &[std::path::PathBuf],
        devices: &[(Device, Option<usize>)],
        free: &[u64],
        dtype: DType,
        quiet: bool,
    ) -> std::result::Result<Self, String> {
        // Per-parameter-count sizing, so the plan is about THIS model, not a guess.
        let bpe = dtype.size_in_bytes() as u64;
        let per_layer = {
            let h = cfg.hidden_size as u64;
            let i = cfg.intermediate_size as u64;
            let kv = (cfg.num_key_value_heads * cfg.head_dim) as u64;
            let q = (cfg.num_attention_heads * cfg.head_dim) as u64;
            (h * q + h * kv * 2 + q * h + 3 * h * i) * bpe
        };
        let head = (cfg.vocab_size as u64) * (cfg.hidden_size as u64) * bpe
            * if cfg.tie_word_embeddings { 1 } else { 2 };

        let counts = plan_stages(cfg.num_hidden_layers, free, head, per_layer);
        if !quiet {
            let plan: Vec<String> = devices
                .iter()
                .zip(counts.iter())
                .map(|((_, ord), c)| match ord {
                    Some(i) => format!("cuda:{i}×{c}L"),
                    None => format!("cpu×{c}L"),
                })
                .collect();
            eprintln!(
                "\x1b[2m[local] sharding {} layers: {}\x1b[0m",
                cfg.num_hidden_layers,
                plan.join(" | ")
            );
        }

        let head_device = devices[0].0.clone();
        // One VarBuilder per device over the same mmapped shards: mmap is cheap
        // and lazy, so each card materializes only the tensors it is asked for.
        let mut vbs = Vec::with_capacity(devices.len());
        for (dev, _) in devices {
            let vb = unsafe {
                VarBuilder::from_mmaped_safetensors(shards, dtype, dev)
                    .map_err(|e| format!("load weights on {dev:?}: {e}"))?
            };
            vbs.push(vb);
        }

        let embed = candle_nn::embedding(cfg.vocab_size, cfg.hidden_size, vbs[0].pp("model.embed_tokens"))
            .map_err(|e| format!("embed_tokens: {e}"))?;
        let norm = candle_nn::rms_norm(cfg.hidden_size, cfg.rms_norm_eps, vbs[0].pp("model.norm"))
            .map_err(|e| format!("model.norm: {e}"))?;
        let lm_head = if cfg.tie_word_embeddings {
            Linear::new(embed.embeddings().clone(), None)
        } else {
            candle_nn::linear_no_bias(cfg.hidden_size, cfg.vocab_size, vbs[0].pp("lm_head"))
                .map_err(|e| format!("lm_head: {e}"))?
        };

        let mut stages = Vec::new();
        let mut next = 0usize;
        for (si, ((dev, _ord), count)) in devices.iter().zip(counts.iter()).enumerate() {
            if *count == 0 {
                continue;
            }
            let rotary = Arc::new(
                Rotary::new(dtype, cfg, dev).map_err(|e| format!("rope on stage {si}: {e}"))?,
            );
            let vb_l = vbs[si].pp("model.layers");
            let mut layers = Vec::with_capacity(*count);
            for li in next..next + count {
                layers.push(
                    DecoderLayer::new(cfg, rotary.clone(), vb_l.pp(li))
                        .map_err(|e| format!("layer {li}: {e}"))?,
                );
            }
            next += count;
            stages.push(Stage { device: dev.clone(), layers });
        }
        if next != cfg.num_hidden_layers {
            return Err(format!(
                "shard plan covers {next} of {} layers",
                cfg.num_hidden_layers
            ));
        }

        Ok(Self { embed, stages, norm, lm_head, head_device, dtype })
    }

    pub fn forward(&mut self, input: &Tensor, offset: usize) -> Result<Tensor> {
        let (b, l) = input.dims2()?;
        let input = input.to_device(&self.head_device)?;
        let mut h = self.embed.forward(&input)?;
        for stage in self.stages.iter_mut() {
            // The one crossing: [b, l, hidden] onto this stage's card. At decode
            // l = 1, so this is a single hidden vector per stage per token.
            h = h.to_device(&stage.device)?;
            let mask = if l > 1 {
                Some(Self::stage_mask(b, l, offset, &stage.device, self.dtype)?)
            } else {
                None
            };
            for layer in stage.layers.iter_mut() {
                h = layer.forward(&h, mask.as_ref(), offset)?;
            }
        }
        let h = h.to_device(&self.head_device)?;
        let h = self.norm.forward(&h)?;
        h.i((.., l - 1..l, ..))?.apply(&self.lm_head)
    }

    fn stage_mask(b: usize, tgt: usize, offset: usize, dev: &Device, dtype: DType) -> Result<Tensor> {
        let minf = f32::NEG_INFINITY;
        let mask: Vec<f32> = (0..tgt)
            .flat_map(|i| {
                (0..(tgt + offset)).map(move |j| if j <= i + offset { 0f32 } else { minf })
            })
            .collect();
        Tensor::from_slice(&mask, (b, 1, tgt, tgt + offset), dev)?.to_dtype(dtype)
    }

    pub fn clear_kv_cache(&mut self) {
        for s in self.stages.iter_mut() {
            for l in s.layers.iter_mut() {
                l.clear_kv_cache();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::plan_stages;

    const GB: u64 = 1024 * 1024 * 1024;

    #[test]
    fn every_layer_is_placed() {
        for &n in &[28usize, 36, 40, 64] {
            for free in [vec![11 * GB, 11 * GB], vec![11 * GB, 6 * GB], vec![4 * GB, 11 * GB]] {
                let c = plan_stages(n, &free, 2 * GB, 100 * 1024 * 1024);
                assert_eq!(c.iter().sum::<usize>(), n, "n={n} free={free:?}");
            }
        }
    }

    #[test]
    fn the_head_card_carries_fewer_layers_when_cards_are_equal() {
        // Equal cards, but card 0 also holds embedding + lm_head, so it must take
        // the smaller share — an even split is what OOMs the head card.
        let c = plan_stages(36, &[11 * GB, 11 * GB], 2 * GB, 100 * 1024 * 1024);
        assert!(c[0] < c[1], "expected head card to carry fewer layers, got {c:?}");
    }

    #[test]
    fn a_bigger_card_carries_more() {
        let c = plan_stages(36, &[6 * GB, 11 * GB], 1 * GB, 100 * 1024 * 1024);
        assert!(c[1] > c[0], "{c:?}");
    }

    #[test]
    fn one_device_takes_everything() {
        assert_eq!(plan_stages(36, &[11 * GB], 2 * GB, 100 * 1024 * 1024), vec![36]);
    }
}
