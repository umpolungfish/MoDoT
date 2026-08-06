//! bitsandbytes NF4 dequantization — loads a 4-bit `quant_method: bitsandbytes`
//! safetensors checkpoint directly into a candle VarBuilder by unpacking every
//! quantized weight back to a dense tensor. candle's own qwen3 model has no 4-bit
//! matmul, so the weights are materialized (bf16 on GPU, f32 on CPU); a dequantized
//! 8B is ~16 GB, so this path wants a large-RAM CPU or a ≥16 GB GPU.
//!
//! NF4 double-quant layout, per tensor `W`:
//!   W                              U8   packed, two 4-bit codes per byte (hi, lo)
//!   W.quant_map                    F32  [16]   the NF4 code→value codebook
//!   W.absmax                       U8   per 64-elem block, 8-bit-quantized scale
//!   W.nested_quant_map             F32  [256]  codebook for the scales
//!   W.nested_absmax                F32  per 256-block scale-of-scales
//!   W.quant_state.bitsandbytes__nf4 U8  JSON: blocksize, shape, nested_blocksize,
//!                                          nested_offset
//! Dequant: absmax[i] = nested_quant_map[absmax_u8[i]] * nested_absmax[i/nbs] + off;
//!          w[k]      = quant_map[code_k] * absmax[k / blocksize].

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use candle_core::safetensors::MmapedSafetensors;
use candle_core::{DType, Device, Tensor};

/// The five companion suffixes a bnb-NF4 quantized weight carries. They are the
/// quantization state, never fed to the model directly.
const COMPANIONS: &[&str] = &[
    ".absmax",
    ".nested_absmax",
    ".nested_quant_map",
    ".quant_map",
    ".quant_state.bitsandbytes__nf4",
];

/// True when `dir/config.json` declares a bitsandbytes quantization.
pub fn is_bnb(dir: &str) -> bool {
    let Ok(raw) = std::fs::read_to_string(format!("{dir}/config.json")) else {
        return false;
    };
    let Ok(j) = serde_json::from_str::<serde_json::Value>(&raw) else {
        return false;
    };
    j.get("quantization_config")
        .and_then(|q| q.get("quant_method"))
        .and_then(|m| m.as_str())
        .map(|s| s.contains("bitsandbytes"))
        .unwrap_or(false)
}

/// Load every tensor, dequantizing the NF4 ones, into a name→tensor map ready for
/// `VarBuilder::from_tensors`. Plain tensors (embeddings, norms, skip-listed
/// modules kept in F16) pass through cast to `dtype`.
pub fn load_dequantized(
    shards: &[PathBuf],
    dtype: DType,
    device: &Device,
    quiet: bool,
) -> Result<HashMap<String, Tensor>, String> {
    let st = unsafe { MmapedSafetensors::multi(shards) }.map_err(|e| format!("mmap: {e}"))?;
    let names: HashSet<String> = st.tensors().into_iter().map(|(n, _)| n).collect();
    let cpu = Device::Cpu;

    let mut out = HashMap::new();
    let mut n_dq = 0usize;
    for name in &names {
        if COMPANIONS.iter().any(|s| name.ends_with(s)) {
            continue; // quant state, not a model weight
        }
        let t = if names.contains(&format!("{name}.absmax")) {
            n_dq += 1;
            dequant_one(&st, name, &cpu)
                .map_err(|e| format!("dequant {name}: {e}"))?
                .to_dtype(dtype)
                .and_then(|t| t.to_device(device))
                .map_err(|e| format!("{name} cast/move: {e}"))?
        } else {
            st.load(name, device)
                .and_then(|t| t.to_dtype(dtype))
                .map_err(|e| format!("load {name}: {e}"))?
        };
        out.insert(name.clone(), t);
    }
    if !quiet {
        eprintln!(
            "\x1b[2m[local] bitsandbytes NF4: dequantized {n_dq} weights to {dtype:?} \
             ({} tensors total)\x1b[0m",
            out.len()
        );
    }
    Ok(out)
}

fn dequant_one(st: &MmapedSafetensors, name: &str, cpu: &Device) -> candle_core::Result<Tensor> {
    let u8v = |suffix: &str| -> candle_core::Result<Vec<u8>> {
        st.load(&format!("{name}{suffix}"), cpu)?
            .flatten_all()?
            .to_vec1::<u8>()
    };
    let f32v = |suffix: &str| -> candle_core::Result<Vec<f32>> {
        st.load(&format!("{name}{suffix}"), cpu)?
            .flatten_all()?
            .to_vec1::<f32>()
    };

    let packed = st.load(name, cpu)?.flatten_all()?.to_vec1::<u8>()?;
    let absmax_q = u8v(".absmax")?;
    let nested_absmax = f32v(".nested_absmax")?;
    let nested_qmap = f32v(".nested_quant_map")?;
    let qmap = f32v(".quant_map")?;

    // quant_state is a small JSON blob stored as raw bytes.
    let qs_bytes = u8v(".quant_state.bitsandbytes__nf4")?;
    let qs: serde_json::Value = serde_json::from_slice(&qs_bytes)
        .map_err(|e| candle_core::Error::Msg(format!("quant_state json: {e}")))?;
    let blocksize = qs["blocksize"].as_u64().unwrap_or(64) as usize;
    let nested_bs = qs["nested_blocksize"].as_u64().unwrap_or(256) as usize;
    let offset = qs["nested_offset"].as_f64().unwrap_or(0.0) as f32;
    let shape: Vec<usize> = qs["shape"]
        .as_array()
        .ok_or_else(|| candle_core::Error::Msg("quant_state.shape missing".into()))?
        .iter()
        .map(|x| x.as_u64().unwrap_or(0) as usize)
        .collect();
    let numel: usize = shape.iter().product();

    // Double dequant of the per-block scales.
    let mut absmax = vec![0f32; absmax_q.len()];
    for (i, &q) in absmax_q.iter().enumerate() {
        absmax[i] = nested_qmap[q as usize] * nested_absmax[i / nested_bs] + offset;
    }

    // Unpack the NF4 codes (first in the high nibble) and scale by the block absmax.
    let mut data = vec![0f32; numel];
    for (j, &b) in packed.iter().enumerate() {
        let k0 = 2 * j;
        if k0 < numel {
            data[k0] = qmap[(b >> 4) as usize] * absmax[k0 / blocksize];
        }
        let k1 = k0 + 1;
        if k1 < numel {
            data[k1] = qmap[(b & 0x0F) as usize] * absmax[k1 / blocksize];
        }
    }

    Tensor::from_vec(data, shape, cpu)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Checks the NF4 double-dequant against a bitsandbytes reference for
    // model.layers.0.self_attn.q_proj.weight of ~/.modelz/ADULTBB. Skips when the
    // model is absent so a checkout without it still passes.
    #[test]
    fn nf4_matches_bitsandbytes_reference() {
        let shard = format!(
            "{}/.modelz/ADULTBB/model-00001-of-00002.safetensors",
            std::env::var("HOME").unwrap_or_default()
        );
        if !std::path::Path::new(&shard).exists() {
            eprintln!("skip: ADULTBB not present");
            return;
        }
        let st = unsafe { MmapedSafetensors::multi(&[shard]) }.unwrap();
        let w = dequant_one(&st, "model.layers.0.self_attn.q_proj.weight", &Device::Cpu).unwrap();
        assert_eq!(w.dims(), &[4096, 4096]);
        let flat = w.flatten_all().unwrap().to_vec1::<f32>().unwrap();

        // bitsandbytes reference (dequantize_4bit, dtype bf16 → f32).
        let want8 = [
            0.0f32, 0.003891, 0.0, 0.00193, 0.010681, -0.016846, 0.017456, 0.008179,
        ];
        for (i, &w) in want8.iter().enumerate() {
            assert!(
                (flat[i] - w).abs() < 2e-3,
                "elem {i}: got {} want {w}",
                flat[i]
            );
        }
        let n = flat.len() as f32;
        let mean = flat.iter().sum::<f32>() / n;
        let var = flat.iter().map(|x| (x - mean).powi(2)).sum::<f32>() / n;
        let std = var.sqrt();
        let min = flat.iter().cloned().fold(f32::INFINITY, f32::min);
        let max = flat.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        assert!((mean - 0.000022).abs() < 5e-4, "mean {mean}");
        assert!((std - 0.026031).abs() < 5e-4, "std {std}");
        assert!((min - (-0.527344)).abs() < 5e-3, "min {min}");
        assert!((max - 0.550781).abs() < 5e-3, "max {max}");
    }
}
