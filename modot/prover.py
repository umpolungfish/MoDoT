#!/usr/bin/env python3
"""
modot/prover.py -- the closed-loop Lean prover

The compile-repair loop from the lean_prover_loop ob3ect, with recursive
decomposition. The agent's proof is gated through the Lean kernel, the one perfect
False-gate a proof has:

  AFWD   generate a tactic block
  IFIX   lake build  (the kernel check, via the lib's own build path)
  EVALT  green, no error, no sorry           -> TANCH (kernel certification), done
  EVALF  error / unsolved goal               -> the frontier, read the goal state
  AREV   error back-propagation              -> feed the goal state to the next attempt

When a goal will not close flat within budget, it is FSPLIT into self-contained
helper lemmas; each is proved through this same loop (recursively, so a helper may
itself split), and the proven leaves are FFUSEd into one file where the main
theorem closes against them. Recursion is depth-bounded: not-reached-within-budget
is a navigation frontier, never a verdict of unprovability.

The attempt is written into a registered scratch module in the p4ramill lib and
built with `lake build`, so Mathlib and Imscribing resolve exactly as they do for
every other module.

Run:  python3 -m modot.prover
"""
from __future__ import annotations

import hashlib
import os
import re
import subprocess
from dataclasses import dataclass, field
from pathlib import Path
from typing import Callable, Dict, List, Optional, Tuple

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
_IMPORT_RE = re.compile(r"^\s*import\s", re.M)
_DECL_RE = re.compile(r"^\s*(?:theorem|lemma)\s", re.M)
_DECL_HEAD_RE = re.compile(r"^\s*(?:lemma|theorem)\s+\S+\s*")
_TOP_DECL_RE = re.compile(r"(?m)^(?=(?:noncomputable\s+)?(?:lemma|theorem|def)\b)")
_DECL_NAME_RE = re.compile(r"^(?:noncomputable\s+)?(?:lemma|theorem|def)\s+([^\s(:{]+)")


def _statement_key(sig: str) -> str:
    """Name-independent key for a lemma: binders + proposition, ws-collapsed.

    Two helpers with the same statement (whatever their names) share a key, so a
    lemma proved once under one parent is reused instead of re-derived under another.
    """
    body = (sig or "").split(":=", 1)[0]
    body = _DECL_HEAD_RE.sub("", body, count=1)
    return re.sub(r"\s+", " ", body).strip()


def _memo_name(key: str) -> str:
    """Stable, statement-derived Lean identifier — identical statements → same name."""
    return "memo_" + hashlib.sha256(key.encode("utf-8")).hexdigest()[:10]


def _rename_decl(sig: str, new_name: str) -> str:
    """Rewrite `lemma/theorem <name>` -> `lemma <new_name>`, keeping binders+prop."""
    return _DECL_HEAD_RE.sub(f"lemma {new_name} ", sig, count=1)


def _dedup_decls(blocks: List[str]) -> str:
    """Join proven blocks, keeping one declaration per name.

    Memoized helpers carry statement-derived names, so the same lemma embedded in
    two sibling blocks collapses to a single declaration rather than tripping a
    duplicate-name error when the blocks are fused into one header.
    """
    text = "\n\n".join(b for b in blocks if b and b.strip())
    seen: Dict[str, str] = {}
    order: List[str] = []
    for part in _TOP_DECL_RE.split(text):
        p = part.strip()
        if not p:
            continue
        m = _DECL_NAME_RE.match(p)
        key = m.group(1) if m else p        # non-decl preamble (open/…) keyed by text
        if key not in seen:
            seen[key] = p
            order.append(key)
    return "\n\n".join(seen[k] for k in order)


def _strip_fences(text: str) -> str:
    m = _FENCE_RE.search(text or "")
    return (m.group(1) if m else (text or "")).strip()


def _strip_imports(source: str) -> str:
    return "\n".join(l for l in source.splitlines()
                     if not _IMPORT_RE.match(l)).strip()


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
    depth: int = 0
    helpers: List[str] = field(default_factory=list)


class LeanProver:
    """Generate -> kernel-check -> repair, with recursive FSPLIT decomposition."""

    def __init__(self, llm: Optional[LLMInterface] = None,
                 flat_budget: int = 4, assemble_budget: int = 3, max_depth: int = 2,
                 top_flat_budget: Optional[int] = None):
        self.llm = llm or LLMInterface()
        self.flat_budget = flat_budget
        self.assemble_budget = assemble_budget
        self.max_depth = max_depth
        # the depth-0 flat attempt may be tightened so hard goals reach FSPLIT while
        # the simpler helpers still get the full budget
        self.top_flat_budget = top_flat_budget or flat_budget
        # statement-key -> proven declaration (or None if that lemma failed);
        # shared across the whole recursion, reset per top-level prove()
        self._memo: Dict[str, Optional[str]] = {}

    def available(self) -> bool:
        return bool(getattr(self.llm, "api_key", None))

    # -- public: recursive prove -------------------------------------------------

    def prove(self, goal: str, imports: str = "import Mathlib",
              depth: int = 0, verbose: bool = True) -> ProofResult:
        if depth == 0:
            self._memo = {}      # fresh memo per top-level goal
        try:
            return self._prove(goal, imports, depth, verbose)
        finally:
            if depth == 0:
                # keep the scratch module green so `lake build Imscribing` stays clean
                SCRATCH_FILE.write_text(PLACEHOLDER)

    def _prove(self, goal, imports, depth, verbose) -> ProofResult:
        pad = "  " * depth
        if verbose:
            print(f"{pad}PROVE(d{depth}): {goal.splitlines()[0][:80]}")

        # 1. flat compile-repair
        flat = self._flat(goal, imports, depth, verbose)
        if flat.closed or depth >= self.max_depth:
            return flat

        # 2. FSPLIT: decompose into self-contained helper lemmas. Prune helpers
        #    that merely restate the parent goal (circular) or repeat a sibling,
        #    and give each a statement-derived name so identical lemmas coincide.
        goal_key = _statement_key(goal)
        lemmas: List[Tuple[str, str]] = []   # (statement key, renamed signature)
        seen_keys = set()
        for lem in self._decompose(goal, flat.last_output, depth):
            key = _statement_key(lem)
            if not key or key == goal_key:
                if verbose:
                    print(f"{pad}  pruned circular helper (restates the goal)")
                continue
            if key in seen_keys:
                continue
            seen_keys.add(key)
            lemmas.append((key, _rename_decl(lem, _memo_name(key))))
        if not lemmas:
            return flat
        if verbose:
            print(f"{pad}FSPLIT(d{depth}) -> {len(lemmas)} helper(s)")

        # 3. prove each helper recursively, memoized by statement: a lemma proved
        #    once anywhere in the tree is reused, not re-derived; a known-failed
        #    lemma is not re-attempted.
        proven_decls: List[str] = []
        for key, sig in lemmas:
            if key in self._memo:
                cached = self._memo[key]
                if cached is not None:
                    proven_decls.append(cached)
                    if verbose:
                        print(f"{pad}  memo hit -> reuse {_memo_name(key)}")
                continue
            sub = self._prove(sig, imports, depth + 1, verbose)
            if sub.closed:
                decl = _strip_imports(sub.source)
                self._memo[key] = decl
                proven_decls.append(decl)
            else:
                self._memo[key] = None
        if not proven_decls:
            return flat

        # 4. FFUSE: assemble helpers, close the main theorem against them
        return self._assemble(goal, proven_decls, imports, depth, verbose)

    # -- flat loop ---------------------------------------------------------------

    def _flat(self, goal, imports, depth, verbose) -> ProofResult:
        budget = self.top_flat_budget if depth == 0 else self.flat_budget

        def prompt(prev, errors):
            return _gen_prompt(goal, imports, prev, errors)
        closed, source, iters, out = self._loop(
            prompt, wrap=lambda body: body,
            budget=budget, depth=depth, verbose=verbose, tag="flat")
        return ProofResult(closed, source, iters, out, depth)

    # -- assemble loop -----------------------------------------------------------

    def _assemble(self, goal, proven_decls, imports, depth, verbose) -> ProofResult:
        header = imports + "\n\n" + _dedup_decls(proven_decls)

        def prompt(prev, errors):
            return _assemble_prompt(goal, header, prev, errors)

        def wrap(body):
            return header + "\n\n" + _strip_imports(body)

        closed, source, iters, out = self._loop(
            prompt, wrap, budget=self.assemble_budget,
            depth=depth, verbose=verbose, tag="fuse")
        return ProofResult(closed, source, iters, out, depth, helpers=proven_decls)

    # -- shared kernel-gated repair loop ----------------------------------------

    def _loop(self, make_prompt: Callable[[str, str], str],
              wrap: Callable[[str], str], budget: int, depth: int,
              verbose: bool, tag: str):
        pad = "  " * depth
        prev, errors, last_source, last_out = "", "", "", ""
        for i in range(1, budget + 1):
            raw, _ = self.llm.infer(
                [{"role": "system", "content": PROVER_SYS},
                 {"role": "user", "content": make_prompt(prev, errors)}],
                max_tokens=4096, temperature=0.0,
            )
            body = _strip_fences(raw)
            source = wrap(body)
            green, out = compile_lean(source)
            last_source, last_out = source, out
            if verbose:
                mark = "GREEN" if green else "frontier"
                print(f"{pad}  [{tag} {i}] {mark} ({len(source)} chars)")
                if not green:
                    tip = _clean(out, 6).replace("\n", "\n" + pad + "        ")
                    print(pad + "        " + tip)
            if green:
                return True, source, i, out
            prev, errors = body, _clean(out)
        return False, last_source, budget, errors

    # -- decomposition -----------------------------------------------------------

    def _decompose(self, goal, frontier, depth) -> List[str]:
        msg = _decompose_prompt(goal, frontier, depth)
        raw, _ = self.llm.infer(
            [{"role": "system", "content": _DECOMPOSE_SYS},
             {"role": "user", "content": msg}],
            max_tokens=1024, temperature=0.0,
        )
        return _parse_lemmas(_strip_fences(raw))


# -- prompts ------------------------------------------------------------------

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


def _assemble_prompt(goal: str, header: str, prev: str, errors: str) -> str:
    p = (
        "The following helper lemmas are ALREADY PROVED and in scope above your "
        "output. Do NOT restate or reprove them, do NOT write any import line. Write "
        "ONLY the main theorem, using these helpers, with a COMPLETE proof and zero "
        "sorry.\n\n"
        f"--- ALREADY-PROVED CONTEXT (in scope) ---\n{header}\n\n"
        f"MAIN GOAL (state as a theorem and prove, using the helpers):\n{goal}\n"
    )
    if prev:
        p += (
            "\nYour previous main theorem did NOT compile. Repair it (helpers are "
            "correct; the gap is in your proof). Output ONLY the corrected main "
            "theorem.\n\n"
            f"--- PREVIOUS ATTEMPT ---\n{prev}\n\n"
            f"--- COMPILER OUTPUT ---\n{errors}\n"
        )
    return p


_DECOMPOSE_SYS = (
    "You decompose a hard Lean 4 goal into helper lemmas. You output ONLY Lean lemma "
    "SIGNATURES, one per line, no proofs, no imports, no prose, no markdown."
)


def _decompose_prompt(goal: str, frontier: str, depth: int) -> str:
    return (
        "This goal did not close within budget. Decompose it into 1 to 3 "
        "self-contained helper lemmas that together make the main proof routine. "
        "Each helper MUST be a standalone Lean 4 lemma: fully universally quantified, "
        "with NO free variable from the main goal's local context, provable on its "
        f"own. Name them aux_d{depth}_1, aux_d{depth}_2, ... Output ONLY the "
        "signatures, one per line, each of the form:\n"
        "  lemma <name> <binders> : <statement>\n"
        "No proofs (no `:=`), no `import`, nothing else.\n\n"
        f"MAIN GOAL:\n{goal}\n\n"
        f"LAST COMPILER FRONTIER:\n{frontier}\n"
    )


def _parse_lemmas(text: str) -> List[str]:
    out = []
    for line in (text or "").splitlines():
        line = line.strip()
        if line.startswith(("lemma ", "theorem ")):
            # keep the statement only, drop any accidental proof
            line = line.split(":=")[0].strip()
            out.append(line)
    return out[:3]


# -- demo ---------------------------------------------------------------------

_DEMO_GOAL = (
    "theorem nicomachus (n : ℕ) : "
    "(∑ i ∈ Finset.range (n + 1), i) ^ 2 = ∑ i ∈ Finset.range (n + 1), i ^ 3"
)


def _demo() -> None:
    p = LeanProver(flat_budget=4, top_flat_budget=2, assemble_budget=3, max_depth=2)
    if not p.available():
        print("no OPENROUTER_API_KEY; prover needs an LLM")
        return
    print(f"prover <- lake build {SCRATCH_MODULE} @ {P4RAMILL}")
    print(f"GOAL: {_DEMO_GOAL}\n")
    r = p.prove(_DEMO_GOAL)
    print()
    print("=" * 62)
    print(f"CLOSED: {r.closed}  |  depth: {r.depth}  |  helpers: {len(r.helpers)}")
    if r.closed:
        print("--- machine-checked proof ---")
        print(r.source)
    else:
        print("--- not yet closed; last frontier ---")
        print(r.last_output)
    print("=" * 62)


if __name__ == "__main__":
    _demo()
