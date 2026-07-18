#!/usr/bin/env python3
"""
IMASM ground-truth corpus generator.

The kernel is a DECIDABLE oracle, not a heuristic judge: every word gets a
computed verdict with a structural reason. That makes labels free, unlimited,
and — uniquely — FOUR-VALUED.

    T  closes: mu-after-delta over a transformed reconnection
    F  does not close: a fork or fuse dangles
    B  both: genuinely contradictory, dialetheia-complete
    N  void: nothing asserted

The B/N split is the asset. Every other ground-truth source gives T/F or a
scalar. Nothing else can hand you "genuinely both" as a label distinct from
"nothing known", which is exactly the distinction models cannot currently make
and have no data for.

Representation note (checked, not assumed): the IMASM opcode alphabet is all
BMP math symbols that tokenizers already carry as single tokens. The Shavian
primitive glyphs are astral-plane, 4 bytes each, and BPE fragments them. So the
corpus emits OPCODE words, never Shavian tuples.

Usage:
    python3 gen_imasm_corpus.py --n 2000 --max-depth 10 --out corpus.jsonl
    python3 gen_imasm_corpus.py --n 500 --rotations   # ROTAT-invariance pairs
"""
from __future__ import annotations

import argparse
import json
import random
import re
import subprocess
import sys
from pathlib import Path

ASK = Path(__file__).resolve().parent.parent / "ask_native" / "target" / "release" / "ask"

# The twelve node-opcodes. Only FSPLIT branches, only FFUSE fuses — the grammar's
# own constraint, so a sampler that respects it produces mostly-valid words and a
# sampler that violates it produces the negative class. Both are wanted.
OPCODES = [
    "VINIT", "IMSCRIB", "AREV", "AFWD", "FSPLIT", "FFUSE",
    "CLINK", "IFIX", "TANCH", "EVALT", "EVALF", "ENGAGR",
]

VERDICT_RE = re.compile(r"IMASM check\s*→\s*([TFBN])")
# `check` prints "shape:", `ring` prints "topology:" — accept either.
SHAPE_RE = re.compile(r"(?:shape|topology):\s*(\w+)\s*\|\s*V=(\d+)\s*E=(\d+)\s*β=(\d+)")
RHO_RE = re.compile(r"ρ=([0-9.]+)")


def check(word: list[str]) -> dict | None:
    """Label one word with the kernel. Returns None if the kernel gave no verdict."""
    try:
        out = subprocess.run(
            [str(ASK), "--imasm", "check", *word],
            capture_output=True, text=True, timeout=30,
        ).stdout
    except (subprocess.TimeoutExpired, OSError):
        return None
    m = VERDICT_RE.search(out)
    if not m:
        return None
    rec: dict = {"word": " ".join(word), "verdict": m.group(1), "n_ops": len(word)}
    if (s := SHAPE_RE.search(out)):
        rec["shape"] = s.group(1)
        rec["V"], rec["E"], rec["beta"] = int(s.group(2)), int(s.group(3)), int(s.group(4))
    if (r := RHO_RE.search(out)):
        rec["rho"] = float(r.group(1))
    # The reason line is the localized signal: it names WHERE closure failed.
    for line in out.splitlines():
        if "μ∘δ" in line or "mu" in line.lower():
            rec["reason"] = line.strip()
            break
    rec["raw"] = out.strip()
    return rec


def sample_word(rng: random.Random, max_depth: int, well_formed_bias: float) -> list[str]:
    """
    Sample a word. With probability `well_formed_bias` build a fork/fuse-balanced
    word (likely to close); otherwise sample freely (likely not to). Both classes
    are wanted — a corpus of only-valid words teaches nothing about failure.
    """
    n = rng.randint(3, max_depth)
    if rng.random() < well_formed_bias:
        word = ["VINIT", "IMSCRIB"]
        forks = 0
        while len(word) < n:
            if forks and rng.random() < 0.4:
                word.append("FFUSE")
                forks -= 1
            elif rng.random() < 0.3:
                word.append("FSPLIT")
                forks += 1
            else:
                word.append(rng.choice(["AFWD", "AREV", "CLINK", "EVALT", "EVALF", "IFIX"]))
        word += ["FFUSE"] * forks
        word.append("TANCH")
        return word
    return [rng.choice(OPCODES) for _ in range(n)]


def ring_signature(word: list[str]) -> dict | None:
    """Signature of the word read as a RING — the object ROTAT actually acts on."""
    try:
        out = subprocess.run(
            [str(ASK), "--imasm", "ring", *word],
            capture_output=True, text=True, timeout=30,
        ).stdout
    except (subprocess.TimeoutExpired, OSError):
        return None
    m = SHAPE_RE.search(out)
    r = RHO_RE.search(out)
    if not m:
        return None
    return {
        "word": " ".join(word),
        "n_ops": len(word),
        "kind": "ring",
        "sig": {"V": int(m.group(2)), "E": int(m.group(3)),
                "beta": int(m.group(4)), "rho": float(r.group(1)) if r else None},
        "raw": out.strip(),
    }


def rotate(word: list[str], k: int) -> list[str]:
    k %= len(word)
    return word[k:] + word[:k]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--n", type=int, default=500)
    ap.add_argument("--max-depth", type=int, default=10)
    ap.add_argument("--bias", type=float, default=0.5, help="fraction sampled well-formed")
    ap.add_argument("--seed", type=int, default=0)
    ap.add_argument("--out", default="imasm_corpus.jsonl")
    ap.add_argument("--rotations", action="store_true",
                    help="also emit every ROTAT rotation of each word (invariance set)")
    args = ap.parse_args()

    if not ASK.is_file():
        print(f"kernel binary not found: {ASK}", file=sys.stderr)
        return 2

    rng = random.Random(args.seed)
    counts: dict[str, int] = {}
    inv_ok = inv_broken = 0
    seen: set[str] = set()

    with open(args.out, "w") as fh:
        for i in range(args.n):
            word = sample_word(rng, args.max_depth, args.bias)
            key = " ".join(word)
            if key in seen:
                continue
            seen.add(key)
            rec = check(word)
            if rec is None:
                continue
            rec["rotation_of"] = None
            fh.write(json.dumps(rec, ensure_ascii=False) + "\n")
            base_sig = ring_signature(word) if args.rotations else None
            counts[rec["verdict"]] = counts.get(rec["verdict"], 0) + 1

            if args.rotations:
                # ROTAT is an automorphism ON A RING, and what it preserves is the
                # SIGNATURE (V, E, β, ρ) — not the closure verdict of a linear word.
                #
                # Verified against the kernel: rotating the linear word
                # `VINIT FSPLIT AFWD FFUSE TANCH` gives T → F → F, correctly, because
                # a fuse landing before any fork is genuinely ill-typed. Rotating the
                # ring `IMSCRIB AFWD AREV` gives V=3 E=3 β=1 ρ=2.0000 every time.
                #
                # Consequence for the corpus: rotation is NOT free augmentation for
                # closure labels (the label legitimately changes), but IS free
                # augmentation for signature labels. Emit the ring orbit.
                for k in range(1, len(word)):
                    rrec = ring_signature(rotate(word, k))
                    if rrec is None:
                        continue
                    rrec["rotation_of"] = key
                    rrec["invariant"] = (base_sig is not None
                                         and rrec.get("sig") == base_sig.get("sig"))
                    if rrec["invariant"]:
                        inv_ok += 1
                    else:
                        inv_broken += 1
                    fh.write(json.dumps(rrec, ensure_ascii=False) + "\n")

            if (i + 1) % 50 == 0:
                print(f"  {i+1}/{args.n}  labels so far: {counts}", file=sys.stderr)

    print(f"\nwrote {args.out}")
    print(f"verdict distribution: {counts}")
    total = sum(counts.values()) or 1
    for v, c in sorted(counts.items()):
        print(f"   {v}: {c:5d}  ({100*c/total:.1f}%)")
    if args.rotations:
        tot = inv_ok + inv_broken or 1
        print(f"\nROTAT ring-signature invariance: {inv_ok}/{inv_ok+inv_broken} rotations preserved the signature")
        if inv_broken:
            print(f"   {inv_broken} did NOT — investigate before trusting rotation as augmentation.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
