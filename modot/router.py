#!/usr/bin/env python3
"""
modot/router.py — the IMSCRIB type-router in front of the prover
================================================================

Before any proof loop runs, a goal must be sorted to the arm that can actually
settle it. That sorting is not a heuristic switch; it is an imscription. The
router types the goal into the twelve IG primitives (each a Belnap value), folds
that type to a single vessel voice, and dispatches:

  N  →  vacuity          the goal engages no axis; there is nothing to prove.
  T  →  the kernel loop   a definite, assertible proposition. modot.prover.
  F  →  the kernel loop   a definite, deniable proposition. The kernel is the
                          honest False-gate; a stated-false goal simply will not
                          close, which is the correct verdict.
  B  →  the Witness arm   a dialetheia: the imscriber placed B (top of the
                          Belnap lattice) on an axis, so the goal carries both
                          assertible and deniable character at once. A T-xor-F
                          kernel proof is the wrong instrument; the four-valued
                          grammatic Witness (modot.witness_proof) is right.

Why an *explicit* B axis is the dialetheia marker, not a T/F mix
---------------------------------------------------------------
A well-posed theorem routinely imscribes with T on some axes and F on others —
e.g. parity F ("inverted / anti-symmetric character") alongside topology T. That
is a definite proposition with mixed *structural* character, not a contradiction.
The dialetheic signal is the imscriber assigning **B** (both) to an axis: it is
the top of the knowledge lattice placed on that axis, the honest "this axis is
over-determined." So the route is decided by B-presence, with the pure Belnap
join of all twelve codes reported alongside for transparency.

This mirrors the vessel's discipline (modot.vessel): the LLM types structure and
never renders a correctness opinion, and there is NO hash / deterministic
fallback. No imscriber, no route — a fabricated type is a clipboard by another
name. For tests, a type may be injected explicitly (source="injected").

Protocol (IMASM front-door):

  VINIT     receive the goal
  IMSCRIB   type the goal into twelve Belnap axes (vessel imscriber)
  EVALT     fold the axes to one vessel voice (the route)
  FSPLIT    dispatch: N→∅ · T/F→kernel loop · B→Witness scaffold
  TANCH     brand the RouterVerdict

Run:  python3 -m modot.router            # no-network fold selftest
      python3 -m modot.router --demo     # live: type + dispatch real goals
"""
from __future__ import annotations

import json
import sys
from dataclasses import dataclass, field
from typing import List, Optional, Tuple

from modot.vessel import (
    B4,
    Imscription,
    ImscriptionError,
    PRIMITIVE_KEYS,
    VesselImscriber,
)
from modot.prover import LeanProver, ProofResult
from modot.witness_proof import (
    translate as witness_translate,
    navigator_available,
    WitnessProofReport,
)


# ── Fold: twelve Belnap axes → one route ─────────────────────────────────────

def fold_goal_type(codes: List[int]) -> Tuple[str, str, str]:
    """Fold a length-12 Belnap imscription to (route, raw_join, reason).

    route     the arm to dispatch: "N" | "T" | "F" | "B"
    raw_join  pure Belnap join of all twelve codes (⊔ = the lattice fold)
    reason    one-line structural justification

    N  iff nothing engages (all axes N)             → vacuity
    B  iff any axis carries B                         → dialetheia / Witness
    T/F otherwise (definite proposition)             → the kernel loop
        polarity T if assertible axes ≥ deniable axes, else F
    """
    norm = [int(c) & 0b11 for c in codes]
    raw = B4.N
    for c in norm:
        raw = B4.join(raw, c)
    raw_name = B4.name(raw)

    engaged = [c for c in norm if c != B4.N]
    if not engaged:
        return "N", raw_name, "no primitive engaged — structurally vacuous"

    n_b = sum(1 for c in norm if c == B4.B)
    if n_b:
        return "B", raw_name, (
            f"{n_b} axis/axes carry B (both) — dialetheic, needs the Witness"
        )

    n_t = sum(1 for c in norm if c == B4.T)
    n_f = sum(1 for c in norm if c == B4.F)
    pol = "T" if n_t >= n_f else "F"
    return pol, raw_name, (
        f"definite (T:{n_t} F:{n_f}, no B) — one bit, the kernel settles it"
    )


# ── Router verdict ───────────────────────────────────────────────────────────

@dataclass
class RouterVerdict:
    """What the front door decided, and what the chosen arm returned."""
    goal: str
    route: str                              # arm chosen: N | T | F | B
    raw_join: str                           # pure Belnap join of the 12 codes
    reason: str
    imscription: Optional[Imscription]      # the 12-code type (None if untyped)
    available: bool                         # was the imscriber live?
    dispatched: bool                        # was the arm actually run?
    proof: Optional[ProofResult] = None     # T/F arm output
    witness: Optional[WitnessProofReport] = None  # B arm output
    note: str = ""
    detail: dict = field(default_factory=dict)

    # Protocol trace (IMASM front-door)
    protocol: Tuple[str, ...] = ("VINIT", "IMSCRIB", "EVALT", "FSPLIT", "TANCH")

    @property
    def arm(self) -> str:
        return {
            "N": "vacuity",
            "T": "kernel-loop (prover)",
            "F": "kernel-loop (prover)",
            "B": "witness",
        }.get(self.route, "?")

    @property
    def closed(self) -> bool:
        """Kernel closure — meaningful only on the T/F arm."""
        return bool(self.proof and self.proof.closed)

    @property
    def disposition(self) -> str:
        if not self.available:
            return "unavailable"
        if self.route == "N":
            return "vacuous"
        if self.route == "B":
            if not self.dispatched:
                return "routed-witness"
            return "witnessed" if (self.witness and self.witness.primary) else "witness-empty"
        if not self.dispatched:
            return "routed-prover"
        return "proved" if self.closed else "unproved"

    def summary(self) -> str:
        parts = [
            f"route={self.route}→{self.arm}",
            f"raw⊔={self.raw_join}",
            f"disp={self.disposition}",
        ]
        if self.imscription is not None:
            parts.append(f"type={self.imscription.summary()}")
        if self.proof is not None:
            parts.append(
                f"proof[closed={self.proof.closed} depth={self.proof.depth} "
                f"helpers={len(self.proof.helpers)}]"
            )
        if self.witness is not None and self.witness.primary:
            parts.append(f"witness={self.witness.primary['name']}")
        parts.append(self.reason)
        if self.note:
            parts.append(self.note)
        return " | ".join(parts)

    def to_dict(self) -> dict:
        return {
            "goal": self.goal[:200],
            "route": self.route,
            "arm": self.arm,
            "raw_join": self.raw_join,
            "reason": self.reason,
            "disposition": self.disposition,
            "available": self.available,
            "dispatched": self.dispatched,
            "type": self.imscription.as_dict() if self.imscription else None,
            "proof": {
                "closed": self.proof.closed,
                "depth": self.proof.depth,
                "helpers": len(self.proof.helpers),
                "iterations": self.proof.iterations,
            } if self.proof else None,
            "witness": self.witness.summary() if self.witness else None,
            "protocol": list(self.protocol),
            "note": self.note,
        }


# ── The router ───────────────────────────────────────────────────────────────

class TypeRouter:
    """IMSCRIB type-router: imscribe a goal, fold to a route, dispatch the arm.

    The three arms are the codebase's existing organs, unchanged:
      - modot.prover.LeanProver   (T/F — kernel-gated compile-repair loop)
      - modot.witness_proof       (B   — grammatic Witness scaffold)
      - vacuity                   (N   — nothing to prove)

    The router owns only the front door: type + fold + demux.
    """

    def __init__(self, llm=None, prover: Optional[LeanProver] = None):
        self.llm = llm
        # Reuse the vessel's imscriber verbatim: LLM types structure, no fallback.
        self.imscriber = VesselImscriber(llm)
        self._prover = prover  # constructed lazily on first T/F dispatch

    def available(self) -> bool:
        return self.imscriber.available()

    def provenance(self) -> str:
        return (
            f"router <- imscriber={'llm' if self.imscriber.available() else 'none'} "
            f"| prover=modot.prover.LeanProver "
            f"| witness={'cl8nk' if navigator_available() else 'unavailable'}"
        )

    @property
    def prover(self) -> LeanProver:
        if self._prover is None:
            self._prover = LeanProver(self.llm)
        return self._prover

    # -- classify only (cheap: one imscription, no arm) ----------------------

    def classify(
        self,
        goal: str,
        imscription: Optional[Imscription] = None,
    ) -> RouterVerdict:
        """IMSCRIB + fold, no dispatch. Returns the route the goal would take."""
        if imscription is None:
            if not self.imscriber.available():
                return RouterVerdict(
                    goal=goal, route="N", raw_join="N",
                    reason="no imscriber (LLM required; no hash fallback)",
                    imscription=None, available=False, dispatched=False,
                    note="cannot type goal",
                )
            try:
                imscription = self.imscriber.imscribe(goal)
            except ImscriptionError as e:
                return RouterVerdict(
                    goal=goal, route="N", raw_join="N",
                    reason=f"imscription failed: {e}",
                    imscription=None, available=True, dispatched=False,
                    note="cannot type goal",
                )

        if imscription.source not in ("llm", "injected"):
            return RouterVerdict(
                goal=goal, route="N", raw_join="N",
                reason=f"imscription source={imscription.source!r} forbidden "
                       "(only llm|injected)",
                imscription=imscription, available=True, dispatched=False,
                note="cannot type goal",
            )

        route, raw_join, reason = fold_goal_type(imscription.codes)
        return RouterVerdict(
            goal=goal, route=route, raw_join=raw_join, reason=reason,
            imscription=imscription, available=True, dispatched=False,
            detail={"type": imscription.as_dict()},
        )

    # -- classify + dispatch the chosen arm ----------------------------------

    def route(
        self,
        goal: str,
        imscription: Optional[Imscription] = None,
        dispatch: bool = True,
        verbose: bool = True,
    ) -> RouterVerdict:
        """Full front door: type, fold, and (optionally) run the chosen arm."""
        verdict = self.classify(goal, imscription=imscription)
        if verbose:
            print(f"IMSCRIB {verdict.imscription.summary() if verdict.imscription else '∅'} "
                  f"→ route {verdict.route} ({verdict.arm}) : {verdict.reason}")

        if not verdict.available or not dispatch:
            return verdict

        if verdict.route in ("T", "F"):
            verdict.dispatched = True
            if not self.prover.available():
                verdict.note = "T/F route but prover LLM unavailable"
                return verdict
            if verbose:
                print(f"  FSPLIT → kernel loop (modot.prover)")
            verdict.proof = self.prover.prove(goal, verbose=verbose)

        elif verdict.route == "B":
            verdict.dispatched = True
            if not navigator_available():
                verdict.note = "B route but cl8nk navigator unavailable"
                return verdict
            if verbose:
                print(f"  FSPLIT → Witness arm (modot.witness_proof)")
            verdict.witness = witness_translate(goal)

        else:  # N
            verdict.dispatched = True
            if verbose:
                print("  FSPLIT → ∅ (vacuous; nothing to prove)")

        return verdict


# ── No-network selftest (injected types only; never invent codes) ────────────

def _inject(codes: List[int]) -> Imscription:
    return Imscription(text="test", codes=list(codes), source="injected")


def _selftest() -> dict:
    r = TypeRouter(llm=None)
    assert not r.available(), "router must require an LLM imscriber"

    # 1. all-N → vacuity
    v_n = r.classify("", imscription=_inject([B4.N] * 12))
    assert v_n.route == "N", f"all-N route {v_n.route}"
    assert v_n.raw_join == "N"

    # 2. all-T → definite T → kernel loop
    v_t = r.classify("g", imscription=_inject([B4.T] * 12))
    assert v_t.route == "T", f"all-T route {v_t.route}"
    assert v_t.arm.startswith("kernel")

    # 3. all-F → definite F → kernel loop (kernel is the False-gate)
    v_f = r.classify("g", imscription=_inject([B4.F] * 12))
    assert v_f.route == "F", f"all-F route {v_f.route}"
    assert v_f.arm.startswith("kernel")

    # 4. one explicit B axis → dialetheia → Witness (even amid many T)
    codes_b = [B4.T] * 11 + [B4.B]
    v_b = r.classify("g", imscription=_inject(codes_b))
    assert v_b.route == "B", f"one-B route {v_b.route}"
    assert v_b.arm == "witness"

    # 5. mixed T/F, NO explicit B → still definite (kernel loop, NOT witness).
    #    A structural T/F mix is not a contradiction; only an explicit B is.
    mixed = [B4.T] * 6 + [B4.F] * 6
    v_mix = r.classify("g", imscription=_inject(mixed))
    assert v_mix.route in ("T", "F"), f"mixed route {v_mix.route} should be T/F"
    assert v_mix.raw_join == "B", "pure join of T+F is B (reported for transparency)"
    assert v_mix.arm.startswith("kernel"), "mixed T/F must go to the kernel, not witness"

    # 6. polarity: more T than F → T; more F than T → F
    assert r.classify("g", imscription=_inject([B4.T] * 7 + [B4.F] * 5)).route == "T"
    assert r.classify("g", imscription=_inject([B4.F] * 7 + [B4.T] * 5)).route == "F"

    # 7. no-imscriber (no LLM, no injection) → unavailable, does not invent a type
    v_none = r.classify("g")
    assert v_none.route == "N" and not v_none.available
    assert "no imscriber" in v_none.reason

    # 8. forged deterministic source is rejected (no route, no invented type)
    forged = Imscription(text="x", codes=[B4.T] * 12, source="deterministic")
    v_forge = r.classify("x", imscription=forged)
    assert v_forge.route == "N" and "forbidden" in v_forge.reason

    # 9. dispatch=False on a live-ish injected type does not run any arm
    v_nod = r.route("g", imscription=_inject([B4.T] * 12), dispatch=False, verbose=False)
    assert v_nod.route == "T" and not v_nod.dispatched
    assert v_nod.proof is None and v_nod.witness is None

    return {
        "available_without_llm": r.available(),
        "provenance": r.provenance(),
        "all_N": v_n.summary(),
        "all_T": v_t.summary(),
        "all_F": v_f.summary(),
        "one_B": v_b.summary(),
        "mixed_TF": v_mix.summary(),
        "no_imscriber": v_none.summary(),
        "forged_rejected": v_forge.reason,
    }


# ── Live demo: type + dispatch three real goals to three arms ────────────────

_DEMO_GOALS = [
    # definite → kernel loop; a trivial-but-real theorem so the build is quick
    ("theorem router_demo_add : (2 : ℕ) + 2 = 4", "expect T/F → prover"),
    # dialetheia → Witness; a self-referential / undecidable-in-frame statement
    ("This sentence asserts its own unprovability: it is true if and only if no "
     "proof of it exists (a Gödel/liar self-reference).", "expect B → witness"),
    # vacuous → nothing to prove
    ("", "expect N → vacuity"),
]


def _demo() -> None:
    from modot.agent import LLMInterface  # openrouter + gemini default

    r = TypeRouter(llm=LLMInterface())
    print(r.provenance())
    if not r.available():
        print("router imscriber unavailable (need OPENROUTER_API_KEY for the LLM)")
        return
    for goal, expectation in _DEMO_GOALS:
        print("\n" + "=" * 66)
        print(f"GOAL: {goal[:80] or '(empty)'}   [{expectation}]")
        verdict = r.route(goal, dispatch=True, verbose=True)
        print("-" * 66)
        print(verdict.summary())
        if verdict.proof and verdict.proof.closed:
            print("--- machine-checked proof ---")
            print(verdict.proof.source)
    print("=" * 66)


if __name__ == "__main__":
    if "--demo" in sys.argv:
        _demo()
    else:
        print(json.dumps(_selftest(), indent=2, default=str))
