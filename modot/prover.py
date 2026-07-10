#!/usr/bin/env python3
"""
modot/prover.py -- the closed-loop Lean prover (T/F arm)

The conventional compile-repair loop from the lean_prover_loop ob3ect. The agent's
proof is gated through the Lean kernel, the one perfect False-gate a proof has:

  AFWD   generate a tactic block
  IFIX   lake build  (the kernel check, via the lib's own build path)
  EVALT  green, no error, no sorry           -> TANCH (kernel certification), done
  EVALF  error / unsolved goal               -> the frontier, read the goal state
  AREV   error back-propagation              -> feed the goal state to the next attempt
         loop until closure

The attempt is written into a registered scratch module in the p4ramill lib and
built with `lake build`, so Mathlib and Imscribing resolve exactly as they do for
every other module. (A loose `lake env lean` file does not build the dependency
oleans; `lake build` does.)

This is the T/F arm. The IMSCRIB type-router (N -> vacuity, T/F -> here, B -> Witness)
and the Witness arm are separate: a conventional proof carries T xor F, one bit; a
B-typed goal needs the four-valued Witness, not this loop.

Run:  python3 -m modot.prover
"""
from __future__ import annotations

import os
import re
import subprocess
from dataclasses import dataclass
from pathlib import Path
from typing import Optional, Tuple

from modot.agent import LLMInterface  # openrouter + gemini default

# The lake project to build against (Mathlib + Imscribing already cached/built).
P4RAMILL = Path(os.environ.get(
    "P4RAMILL", str(Path.home() / "imsgct" / "p4rakernel" / "p4ramill")))

# A scratch module registered in the lib's globs. The attempt is written here and
# built. At rest it must hold a valid proof so `lake build Imscribing` stays green.
SCRATCH_MODULE = "Imscribing.Scratch.ProverScratch"
SCRATCH_FILE = P4RAMILL / "Imscribing" / "Scratch" / "ProverScratch.lean"
PLACEHOLDER = "import Mathlib\n\ntheorem scratch_ok : (2 : ℝ) + 2 = 4 := by norm_num\n"

PROVER_SYS = (
    "You are a Lean 4 proof engine (toolchain leanprover/lean4:v4.28.0, Mathlib). "
    "You output ONLY Lean 4 source: a single import line, then the theorem with a "
    "COMPLETE proof. No prose, no markdown fences, and never `sorry` or `admit`.\n"
    "IMPORTS: use exactly `import Mathlib` and NOTHING else. Never import a specific "
    "Mathlib sub-module (e.g. Mathlib.Algebra.BigOperators.Basic) -- those paths move "
    "between versions and will fail as 'bad import'. `import Mathlib` already gives "
    "you all of Mathlib.\n"
    "When given a previous attempt and its compiler output, REPAIR it: read the "
    "`unsolved goals` state, identify the exact gap, and return a full corrected file "
    "that compiles clean."
)

_ERR_RE = re.compile(r"error:", re.I)
_SORRY_RE = re.compile(r"uses 'sorry'|declaration uses|\bsorry\b", re.I)
_FENCE_RE = re.compile(r"```(?:lean)?\s*(.*?)```", re.S)
_NOISE_RE = re.compile(r"has local changes|Using cache|decompressed|Building|Compiling")


def _strip_fences(text: str) -> str:
    m = _FENCE_RE.search(text or "")
    return (m.group(1) if m else (text or "")).strip()


def _gen_prompt(goal: str, imports: str, prev: str, errors: str) -> str:
    p = (
        "Write a complete, self-contained Lean 4 file that states and proves the "
        "following with ZERO sorry.\n\n"
        f"The file MUST begin with exactly one import line: `{imports}` -- and no "
        "other import.\n\n"
        f"GOAL (state it as a theorem and prove it):\n{goal}\n"
    )
    if prev:
        p += (
            "\nYour previous file did NOT compile. Repair it and output the full "
            "corrected file.\n\n"
            f"--- PREVIOUS ATTEMPT ---\n{prev}\n\n"
            f"--- COMPILER OUTPUT (errors + remaining goals) ---\n{errors}\n"
        )
    return p


def _clean(out: str, n: int = 60) -> str:
    """Drop lake's benign chatter, keep the lean diagnostics."""
    lines = [l for l in (out or "").splitlines()
             if l.strip() and not _NOISE_RE.search(l)]
    return "\n".join(lines[-n:])


def compile_lean(source: str, timeout: int = 600) -> Tuple[bool, str]:
    """Build `source` as the scratch module. Green iff exit 0, no error, no sorry."""
    SCRATCH_FILE.parent.mkdir(parents=True, exist_ok=True)
    SCRATCH_FILE.write_text(source)
    try:
        proc = subprocess.run(
            ["lake", "build", SCRATCH_MODULE],
            cwd=str(P4RAMILL), capture_output=True, text=True, timeout=timeout,
        )
        out = (proc.stdout or "") + (proc.stderr or "")
        green = (
            proc.returncode == 0
            and not _ERR_RE.search(out)
            and not _SORRY_RE.search(out)
            and "sorry" not in source.lower()
        )
        return green, out
    except subprocess.TimeoutExpired:
        return False, "error: lake build timed out"


@dataclass
class ProofResult:
    closed: bool
    source: str
    iterations: int
    last_output: str = ""


class LeanProver:
    """The T/F arm: generate -> kernel-check -> repair, until green."""

    def __init__(self, llm: Optional[LLMInterface] = None, budget: int = 6):
        self.llm = llm or LLMInterface()
        self.budget = budget

    def available(self) -> bool:
        return bool(getattr(self.llm, "api_key", None))

    def prove(self, goal: str, imports: str = "import Mathlib",
              verbose: bool = True) -> ProofResult:
        prev, errors, result = "", "", None
        try:
            for i in range(1, self.budget + 1):
                raw, _ = self.llm.infer(
                    [{"role": "system", "content": PROVER_SYS},
                     {"role": "user",
                      "content": _gen_prompt(goal, imports, prev, errors)}],
                    max_tokens=4096, temperature=0.0,
                )
                source = _strip_fences(raw)
                green, out = compile_lean(source)
                if verbose:
                    print(f"  [iter {i}] {'GREEN' if green else 'frontier'} "
                          f"({len(source)} chars)")
                    if not green:
                        tail = _clean(out, 8).replace("\n", "\n      ")
                        print("      " + tail)
                if green:
                    result = ProofResult(True, source, i, out)
                    return result
                prev, errors = source, _clean(out)
            result = ProofResult(False, prev, self.budget, errors)
            return result
        finally:
            # keep the scratch module green so `lake build Imscribing` stays clean;
            # the winning proof, if any, is returned in result.source
            SCRATCH_FILE.write_text(PLACEHOLDER)


# -- demo: close a real theorem end to end through the kernel ------------------

_DEMO_GOAL = (
    "theorem gauss_sum (n : ℕ) : "
    "2 * (∑ i ∈ Finset.range (n + 1), i) = n * (n + 1)"
)


def _demo() -> None:
    p = LeanProver(budget=6)
    if not p.available():
        print("no OPENROUTER_API_KEY; prover needs an LLM")
        return
    print(f"prover <- lake build {SCRATCH_MODULE} @ {P4RAMILL}")
    print(f"GOAL: {_DEMO_GOAL}\n")
    r = p.prove(_DEMO_GOAL)
    print()
    print("=" * 60)
    print(f"CLOSED: {r.closed}  |  iterations: {r.iterations}")
    if r.closed:
        print("--- machine-checked proof ---")
        print(r.source)
    else:
        print("--- not yet closed; last frontier ---")
        print(r.last_output)
    print("=" * 60)


if __name__ == "__main__":
    _demo()
