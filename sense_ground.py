#!/usr/bin/env python3
"""
sense_ground.py — VAE-Vita as the sense organ's real OBSERVE signal.

The guided imscription pipeline hands back a valid tuple for ANY name — even a
bare, context-free token like a random word — because every primitive gets
assigned SOMETHING. That doesn't mean the assignment means anything: for a
context-poor token there was nothing to reason from, so the tuple is closer to
a coin flip than a reading.

VAE-Vita was trained on all 17,280,000 Crystal-of-Types configurations and
independently recovered the SIC-POVM geometry from structural type alone — it
has learned what a genuinely well-formed point in Crystal space looks like,
with no notion of English words at all. Running a freshly-imscribed tuple
through its encoder/decoder (mu . delta) and reading back the reconstruction
confidence gives a real, non-circular signal for how well that assignment
sits on the learned manifold, independent of the LLM that proposed it.

Usage: python3 sense_ground.py <token>
Prints one JSON object: per-primitive confidence, overall confidence, and
whether the argmax reconstruction matches the assigned value at all.
"""
import json
import re
import sys

sys.path.insert(0, "/home/mrnob0dy666/imsgct/ig-pulse/vae_vita")
from crystal_data import PRIMS, VALUE_COUNTS, VALUES, crystal_address, CrystalDataset  # noqa: E402

MODEL_PATH = "/home/mrnob0dy666/imsgct/ig-pulse/vae_vita/vae_vita_sic.pt"
_model = None
_ds = CrystalDataset()


def load_model():
    global _model
    if _model is not None:
        return _model
    import torch
    from hyperspherical_vae import HypersphericalVAE

    ckpt = torch.load(MODEL_PATH, map_location="cpu")
    model = HypersphericalVAE(d_latent=12, hidden_dim=256, beta=0.2, lambda_sic=50.0)
    model.load_state_dict(ckpt["model_state_dict"])
    model.eval()
    _model = model
    return model


def parse_notation(notation: str) -> dict:
    """`⟨Ð=𐑨; Þ=𐑥; ...⟩` -> {'Ð': '𐑨', 'Þ': '𐑥', ...}"""
    tup = {}
    inner = notation.strip().strip("⟨⟩")
    for part in inner.split(";"):
        part = part.strip()
        if not part or "=" not in part:
            continue
        k, v = part.split("=", 1)
        tup[k.strip()] = v.strip()
    return tup


def ground(token: str, notation: str) -> dict:
    import torch

    tup = parse_notation(notation)
    missing = [p for p in PRIMS if p not in tup]
    if missing:
        return {"token": token, "error": f"tuple missing primitives: {missing}", "grounded": False}

    addr = crystal_address(tup)
    ordinal = _ds.get_ordinal(addr)
    x = torch.from_numpy(ordinal).float().unsqueeze(0)

    model = load_model()
    with torch.no_grad():
        mu, kappa, z, logits = model(x)

    per_prim = {}
    confidences = []
    matches = 0
    for i, (prim, nv, logit) in enumerate(zip(PRIMS, VALUE_COUNTS, logits)):
        probs = torch.softmax(logit[0], dim=-1)
        actual_idx = VALUES[prim].index(tup[prim])
        actual_conf = probs[actual_idx].item()
        pred_idx = probs.argmax().item()
        confidences.append(actual_conf)
        if pred_idx == actual_idx:
            matches += 1
        per_prim[prim] = {
            "assigned": tup[prim],
            "confidence": round(actual_conf, 4),
            "reconstructed_matches_assigned": pred_idx == actual_idx,
        }

    overall = sum(confidences) / len(confidences)
    return {
        "token": token,
        "grounded": True,
        "crystal_address": addr,
        "per_primitive": per_prim,
        "overall_confidence": round(overall, 4),
        "primitives_matched": matches,
        "primitives_total": len(PRIMS),
        "latent_kappa": round(kappa.item(), 3),
        "reading": (
            "well-formed — sits on the learned SIC manifold"
            if overall > 0.15 and matches >= 8
            else "genuinely novel / off-manifold — not an error, a real signal that this point is unlike the trained structure"
        ),
    }


if __name__ == "__main__":
    if len(sys.argv) < 2:
        print(json.dumps({"error": "usage: sense_ground.py <token>"}))
        sys.exit(2)
    token = sys.argv[1]

    import subprocess

    out = subprocess.run(
        ["python3", "-m", "modot.ig_tools", "call", "lookup_catalog", token],
        cwd="/home/mrnob0dy666/imsgct/MoDoT",
        capture_output=True,
        text=True,
    )
    try:
        data = json.loads(out.stdout)
        matches = data.get("matches", [])
        exact = [m for m in matches if m.get("name", "").lower() == token.lower()]
        entry = exact[0] if exact else (matches[0] if matches else None)
    except Exception:
        entry = None

    if entry is None or "notation" not in entry:
        print(json.dumps({"token": token, "error": "not in catalog or no notation", "grounded": False}))
        sys.exit(1)

    result = ground(token, entry["notation"])
    print(json.dumps(result, ensure_ascii=False))
