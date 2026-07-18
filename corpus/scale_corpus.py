#!/usr/bin/env python3
"""
Scaled IMASM corpus: both legs, joined.

  δ  sample opcode words
  μ₁ label each with the ask kernel   -> verdict T/F/B/N + shape + reason
  μ₂ address each in the crystal      -> twelve indices + crystal address

μ₂ runs inside mOMonadOS, so it needs QEMU. One boot handles the whole batch:
the words are written as `crystal indices …` lines, piped in, and the INDICES
replies are matched back by order. That is the difference between minutes and
hours — never boot per word.

Output: one JSONL row per word carrying BOTH the operational verdict and the
structural type. That pairing is the training signal: the map from executable
sequence to crystal address, with a decidable four-valued label attached.

Usage:
    python3 scale_corpus.py --n 2000 --max-depth 12 --out corpus_2k.jsonl
"""
from __future__ import annotations

import argparse
import json
import random
import re
import subprocess
import sys
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
ASK = ROOT / "ask_native" / "target" / "release" / "ask"
OS_ELF = ROOT.parent / "mOMonadOS" / "target" / "x86_64-unknown-none" / "release" / "momonados"

OPCODES = ["VINIT", "IMSCRIB", "AREV", "AFWD", "FSPLIT", "FFUSE",
           "CLINK", "IFIX", "TANCH", "EVALT", "EVALF", "ENGAGR"]
AXES = ["Ð", "Þ", "Ř", "Φ", "ƒ", "Ç", "Γ", "ɢ", "⊙", "Ħ", "Σ", "Ω"]

VERDICT_RE = re.compile(r"IMASM check\s*→\s*([TFBN])")
SHAPE_RE = re.compile(r"(?:shape|topology):\s*(\w+)\s*\|\s*V=(\d+)\s*E=(\d+)\s*β=(\d+)")
RHO_RE = re.compile(r"ρ=([0-9.]+)")
IDX_RE = re.compile(r"INDICES ([0-9,]+) ADDR (\d+)")


def sample_word(rng: random.Random, max_depth: int, bias: float) -> list[str]:
    n = rng.randint(3, max_depth)
    if rng.random() < bias:
        word, forks = ["VINIT", "IMSCRIB"], 0
        while len(word) < n:
            if forks and rng.random() < 0.4:
                word.append("FFUSE"); forks -= 1
            elif rng.random() < 0.3:
                word.append("FSPLIT"); forks += 1
            else:
                word.append(rng.choice(["AFWD", "AREV", "CLINK", "EVALT", "EVALF", "IFIX"]))
        word += ["FFUSE"] * forks
        word.append("TANCH")
        return word
    return [rng.choice(OPCODES) for _ in range(n)]


def label_batch(words: list[list[str]]) -> list[dict | None]:
    """μ₁ — the ask kernel, one subprocess per word (native, no VM)."""
    out: list[dict | None] = []
    for w in words:
        try:
            txt = subprocess.run([str(ASK), "--imasm", "check", *w],
                                 capture_output=True, text=True, timeout=30).stdout
        except (subprocess.TimeoutExpired, OSError):
            out.append(None); continue
        m = VERDICT_RE.search(txt)
        if not m:
            out.append(None); continue
        rec: dict = {"verdict": m.group(1)}
        if (s := SHAPE_RE.search(txt)):
            rec |= {"shape": s.group(1), "V": int(s.group(2)),
                    "E": int(s.group(3)), "beta": int(s.group(4))}
        if (r := RHO_RE.search(txt)):
            rec["rho"] = float(r.group(1))
        for line in txt.splitlines():
            if "μ∘δ" in line:
                rec["reason"] = line.strip(); break
        out.append(rec)
    return out


def index_batch(words: list[list[str]], timeout: int = 900) -> list[dict | None]:
    """
    μ₂ — the crystal address, for the WHOLE batch in one QEMU boot.

    Replies are matched back by ORDER, so a word that produces no INDICES line
    would shift every later row. Guard against that: the kernel emits exactly one
    INDICES per accepted command, and unknown opcodes print a different line, so
    a length mismatch is detected and reported rather than silently misaligned.
    """
    if not OS_ELF.is_file():
        return [None] * len(words)
    script = "\ncrystal\n" + "".join(f"crystal indices {' '.join(w)}\n" for w in words) + "quit\n"
    try:
        proc = subprocess.run(
            ["qemu-system-x86_64", "-kernel", str(OS_ELF), "-m", "256M",
             "-display", "none", "-no-reboot",
             "-device", "isa-debug-exit,iobase=0xf4,iosize=4", "-serial", "stdio"],
            input=script, capture_output=True, text=True, timeout=timeout,
        )
    except (subprocess.TimeoutExpired, OSError) as e:
        print(f"  [qemu] batch failed: {e}", file=sys.stderr)
        return [None] * len(words)

    hits = IDX_RE.findall(proc.stdout)
    if len(hits) != len(words):
        print(f"  [qemu] WARNING {len(hits)} replies for {len(words)} words — "
              f"dropping this batch rather than misaligning rows", file=sys.stderr)
        return [None] * len(words)
    return [{"indices": [int(x) for x in idx.split(",")], "address": int(addr)}
            for idx, addr in hits]


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--n", type=int, default=1000)
    ap.add_argument("--max-depth", type=int, default=12)
    ap.add_argument("--bias", type=float, default=0.5)
    ap.add_argument("--seed", type=int, default=0)
    ap.add_argument("--batch", type=int, default=250, help="words per QEMU boot")
    ap.add_argument("--out", default="corpus.jsonl")
    args = ap.parse_args()

    if not ASK.is_file():
        print(f"ask binary missing: {ASK}", file=sys.stderr); return 2

    rng = random.Random(args.seed)
    seen: set[str] = set()
    words: list[list[str]] = []
    while len(words) < args.n:
        w = sample_word(rng, args.max_depth, args.bias)
        k = " ".join(w)
        if k not in seen:
            seen.add(k); words.append(w)

    t0 = time.time()
    counts: dict[str, int] = {}
    typed = both = 0
    with open(args.out, "w") as fh:
        for b in range(0, len(words), args.batch):
            chunk = words[b:b + args.batch]
            labels = label_batch(chunk)
            idxs = index_batch(chunk)
            for w, lab, idx in zip(chunk, labels, idxs):
                if lab is None:
                    continue
                row = {"word": " ".join(w), "n_ops": len(w)} | lab
                if idx is not None:
                    row |= idx
                    row["tuple"] = dict(zip(AXES, idx["indices"]))
                    typed += 1
                    both += 1
                fh.write(json.dumps(row, ensure_ascii=False) + "\n")
                counts[lab["verdict"]] = counts.get(lab["verdict"], 0) + 1
            done = min(b + args.batch, len(words))
            print(f"  {done}/{len(words)}  {time.time()-t0:.0f}s  labels={counts} typed={typed}",
                  file=sys.stderr)

    total = sum(counts.values()) or 1
    print(f"\nwrote {args.out}  ({time.time()-t0:.0f}s)")
    print(f"labelled: {total}   with crystal type: {both}")
    for v in "TFBN":
        c = counts.get(v, 0)
        print(f"   {v}: {c:6d}  ({100*c/total:5.1f}%)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
