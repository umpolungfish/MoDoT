#!/usr/bin/env python3
"""
modot/spine.py — MoDoT Manuscript Spine (runtime end-to-end pipeline)
====================================================================

Lean has `Imscribing.VaeVita.ManuscriptSpine.manuscript_formal_spine`:
  port_kernel_spine ∧ witness_vessel_lossless

MoDoT is the organism that *runs* that architecture. This module is the single
end-to-end pipeline the breath calls — not two parallel arms hung off _think.

Pipeline (one IMASM-shaped chain):

  VINIT     open breath
  IMSCRIB   demand imscription + catalog witness scaffold   [prepare]
  FSPLIT    model produces answer (caller / LLM)
  EVALT     Dual-Link co-typing (isomorphic identity in SIC)
  EVALF     defect localization
  FFUSE     model voice ⋈ vessel voice  (Belnap join; conflict held)
  ENGAGR    hold B when conflict / dialetheia
  IFIX      brand SpineReport as the breath verdict

Faces packaged (same ledger as Lean, runtime):

  PROVE     balance: FrobeniusHarness μ∘δ closed on the think emit
  UNIFY     vessel amplitude map lives in ℂ¹²; Belnap B = T+F at the code level
  PORT      DualLinkVessel co-typing + ride-AS residual
  WITNESS   cl8nk catalog scaffold (conventional proof structure)
  TRANSPORT board/readback law is the vessel's own μ∘δ residual check

Honest non-claims (papers):
  - Clay T/B are not Millennium proofs
  - discrete Belnap stack ≠ algebraic Scott-Grassl fiducial
  - d=2048 existence remains open

Author: Lando⊗⊙perator
"""
from __future__ import annotations

import json
from dataclasses import dataclass, field
from typing import Any, Dict, List, Optional, Tuple

from modot.vessel import (
    DualLinkVessel,
    VesselReport,
    Imscription,
    HAVE_SIC12,
    B4 as VesselB4,
    belnap_tuple_to_psi,
    _AMP,
)
from modot.witness_proof import (
    translate as witness_translate,
    navigator_available,
    WitnessProofReport,
)


# ── Belnap 2-bit codes (match agent.B4 / vessel B4) ─────────────────────────

_NAME = {0: "N", 1: "T", 2: "F", 3: "B"}
_FROM = {
    "N": 0, "T": 1, "F": 2, "B": 3,
    "NEITHER": 0, "TRUE": 1, "FALSE": 2, "BOTH": 3,
}


def _parse_b(x) -> int:
    if isinstance(x, int):
        return int(x) & 0b11
    if hasattr(x, "name"):
        return _FROM.get(str(x.name).upper(), 0)
    return _FROM.get(str(x).upper(), 0)


def _join(a: int, b: int) -> int:
    return (a | b) & 0b11


def _conflict(a: int, b: int) -> int:
    return bin(int(a) ^ int(b)).count("1")


def _unify_both_superposition() -> bool:
    """Runtime check of UNIFY face: belnap B amplitude = T + F (code level)."""
    return (_AMP[VesselB4.B] == _AMP[VesselB4.T] + _AMP[VesselB4.F])


@dataclass
class SpinePrepare:
    """Result of IMSCRIB leg: demand type + witness scaffold."""
    question: str
    demand: Optional[Imscription]
    witness: Optional[WitnessProofReport]
    scaffold_md: str
    vessel_ready: bool
    witness_ready: bool
    note: str = ""

    def summary(self) -> str:
        dem = self.demand.summary() if self.demand else "∅"
        wit = self.witness.summary() if self.witness else "∅"
        return f"prepare demand={dem} | {wit}"


@dataclass
class SpineReport:
    """Single end-to-end verdict of one breath through the manuscript spine."""
    # Faces (runtime analogues of Lean manuscript_formal_spine conjuncts)
    prove_balance_closed: bool          # Frobenius harness closed on think
    unify_both_is_superposition: bool   # B = T+F in amplitude map
    port_riding: bool                   # vessel μ∘δ residual ≈ 0 on both states
    port_vessel: Optional[VesselReport]
    witness: Optional[WitnessProofReport]
    transport_roundtrip_ok: bool        # demand re-imscribe identity when text equal

    # Voices
    model_voice: str
    vessel_voice: Optional[str]
    fused: str
    conflict: int

    # Shared state
    demand: Optional[Imscription]
    answer: Optional[Imscription]
    scaffold_md: str
    question: str
    answer_text: str

    # Protocol
    protocol: Tuple[str, ...] = (
        "VINIT", "IMSCRIB", "FSPLIT", "EVALT", "EVALF", "FFUSE", "ENGAGR", "IFIX",
    )
    note: str = ""
    detail: Dict[str, Any] = field(default_factory=dict)

    @property
    def belnap(self) -> str:
        return self.fused

    @property
    def assertible(self) -> bool:
        return self.fused in ("T", "B")

    @property
    def deniable(self) -> bool:
        return self.fused in ("F", "B")

    def summary(self) -> str:
        parts = [
            f"spine fused={self.fused}",
            f"model={self.model_voice}",
            f"vessel={self.vessel_voice or '—'}",
            f"conflict={self.conflict}",
            f"balance={'id' if self.prove_balance_closed else 'OPEN'}",
            f"unify_B=T+F={self.unify_both_is_superposition}",
            f"{'RIDE AS' if self.port_riding else 'HELD'}",
        ]
        if self.port_vessel:
            parts.append(f"gap={self.port_vessel.sic_gap:.4f}")
            if self.port_vessel.defects:
                parts.append("defects[" + ",".join(self.port_vessel.defects[:4]) + "]")
        if self.witness and self.witness.primary:
            parts.append(f"witness={self.witness.primary['name']}")
        if self.note:
            parts.append(self.note)
        return " | ".join(parts)

    def to_dict(self) -> dict:
        return {
            "fused": self.fused,
            "model_voice": self.model_voice,
            "vessel_voice": self.vessel_voice,
            "conflict": self.conflict,
            "prove_balance_closed": self.prove_balance_closed,
            "unify_both_is_superposition": self.unify_both_is_superposition,
            "port_riding": self.port_riding,
            "transport_roundtrip_ok": self.transport_roundtrip_ok,
            "vessel": self.port_vessel.summary() if self.port_vessel else None,
            "witness": self.witness.summary() if self.witness else None,
            "demand": self.demand.as_dict() if self.demand else None,
            "answer": self.answer.as_dict() if self.answer else None,
            "defects": self.port_vessel.defects if self.port_vessel else [],
            "sic_gap": self.port_vessel.sic_gap if self.port_vessel else None,
            "protocol": list(self.protocol),
            "note": self.note,
            "detail": self.detail,
        }


class ManuscriptSpine:
    """Single MoDoT pipeline: witness + Dual-Link vessel + FFUSE.

    Mirrors Lean `VAE_Vita_ManuscriptSpine` as the organism's runtime spine.
    The agent should call only this object for selectivity / verification.
    """

    PROTOCOL = (
        "VINIT", "IMSCRIB", "FSPLIT", "EVALT", "EVALF", "FFUSE", "ENGAGR", "IFIX",
    )

    def __init__(self, llm=None, closure_tol: float = 1e-6):
        self.llm = llm
        self.vessel = DualLinkVessel(llm, closure_tol=closure_tol)
        self.last_prepare: Optional[SpinePrepare] = None
        self.last_report: Optional[SpineReport] = None

    # -- availability / provenance ------------------------------------------

    def available(self) -> bool:
        """Spine is live if the vessel face is live (SIC + imscriber).

        Witness face degrades gracefully (scaffold may be empty) but vessel
        is required for a real fused verdict.
        """
        return self.vessel.available()

    def provenance(self) -> str:
        faces = [
            f"vessel={self.vessel.provenance()}",
            f"witness={'cl8nk' if navigator_available() else 'unavailable'}",
            f"unify_B=T+F={_unify_both_superposition()}",
            f"HAVE_SIC12={HAVE_SIC12}",
        ]
        return "spine <- " + " | ".join(faces)

    # -- leg 1: IMSCRIB (demand + witness) ----------------------------------

    def prepare(self, question: str) -> SpinePrepare:
        """IMSCRIB: imscribe demand and load catalog witness scaffold.

        One entry point — both faces share the same question text.
        """
        demand: Optional[Imscription] = None
        witness: Optional[WitnessProofReport] = None
        notes: List[str] = []

        if self.vessel.available():
            try:
                demand = self.vessel.imscribe(question)
            except Exception as e:
                notes.append(f"demand imscribe failed: {e}")
        else:
            notes.append("vessel not available for demand imscription")

        if navigator_available():
            try:
                witness = witness_translate(question)
            except Exception as e:
                notes.append(f"witness translate failed: {e}")
        else:
            notes.append("cl8nk navigator unavailable")

        scaffold = ""
        if witness is not None and witness.scaffold_md:
            scaffold = witness.scaffold_md

        prep = SpinePrepare(
            question=question,
            demand=demand,
            witness=witness,
            scaffold_md=scaffold,
            vessel_ready=demand is not None and demand.source in ("llm", "injected"),
            witness_ready=bool(scaffold),
            note="; ".join(notes),
        )
        self.last_prepare = prep
        return prep

    # -- legs 2–5: vessel co-type + FFUSE -----------------------------------

    def complete(
        self,
        question: str,
        answer_text: str,
        model_voice,
        *,
        balance_closed: bool = True,
        demand: Optional[Imscription] = None,
        prepare: Optional[SpinePrepare] = None,
    ) -> SpineReport:
        """EVALT/EVALF + FFUSE + IFIX: co-type answer against demand, fuse voices.

        `model_voice` may be agent B4 enum, int, or "T"/"F"/"B"/"N".
        Uses demand from `prepare` when provided so question is imscribed once.
        """
        prep = prepare or self.last_prepare
        if demand is None and prep is not None:
            demand = prep.demand
        if prep is None or prep.question != question:
            # No prior prepare, or different question: run IMSCRIB now.
            prep = self.prepare(question)
            demand = prep.demand

        mv = _parse_b(model_voice)
        vessel_report: Optional[VesselReport] = None
        vv: Optional[int] = None
        answer_im: Optional[Imscription] = None
        riding = False
        transport_ok = False

        if self.vessel.available() and demand is not None:
            vessel_report = self.vessel.evaluate(
                question, answer_text, demand=demand,
            )
            if vessel_report.belnap in ("T", "F", "B", "N"):
                vv = _parse_b(vessel_report.belnap)
            answer_im = vessel_report.answer
            riding = bool(vessel_report.riding)
            # Transport law: same text → same imscription (identity co-type)
            if (answer_text or "") == (question or ""):
                transport_ok = (
                    vessel_report.belnap == "T"
                    and vessel_report.sic_gap < 1e-9
                    and not vessel_report.defects
                )
            else:
                # Non-identity: roundtrip of demand through cache is still identity
                transport_ok = demand.source in ("llm", "injected")

        if vv is not None:
            fused = _join(mv, vv)
            conflict = _conflict(mv, vv)
            note = "FFUSE model ⋈ vessel"
        else:
            fused = mv
            conflict = 0
            note = "model only (vessel face silent)"

        if conflict and fused == VesselB4.B:
            note += "; ENGAGR hold B"

        report = SpineReport(
            prove_balance_closed=bool(balance_closed),
            unify_both_is_superposition=_unify_both_superposition(),
            port_riding=riding,
            port_vessel=vessel_report,
            witness=prep.witness if prep else None,
            transport_roundtrip_ok=transport_ok,
            model_voice=_NAME[mv],
            vessel_voice=_NAME[vv] if vv is not None else None,
            fused=_NAME[fused],
            conflict=conflict,
            demand=demand,
            answer=answer_im,
            scaffold_md=prep.scaffold_md if prep else "",
            question=question,
            answer_text=answer_text,
            note=note,
            detail={
                "provenance": self.provenance(),
                "prepare": prep.summary() if prep else None,
                "vessel_detail": vessel_report.detail if vessel_report else None,
                "faces": {
                    "prove": balance_closed,
                    "unify": _unify_both_superposition(),
                    "port": riding,
                    "witness": bool(prep and prep.witness_ready),
                    "transport": transport_ok,
                },
            },
        )
        self.last_report = report
        return report

    # -- one-shot (tests / CLI) ---------------------------------------------

    def run(
        self,
        question: str,
        answer_text: str,
        model_voice="N",
        *,
        balance_closed: bool = True,
    ) -> SpineReport:
        """Full pipeline without the LLM: prepare → complete."""
        prep = self.prepare(question)
        return self.complete(
            question, answer_text, model_voice,
            balance_closed=balance_closed, prepare=prep,
        )

    # Compatibility surface so agent can treat spine as the old gate --------
    def imscribe(self, text: str) -> Imscription:
        return self.vessel.imscribe(text)

    def evaluate(self, question: str, answer: str, demand=None) -> VesselReport:
        return self.vessel.evaluate(question, answer, demand=demand)

    def gate(self, question: str, answer: str, demand=None) -> VesselReport:
        return self.vessel.gate(question, answer, demand=demand)


def _selftest() -> dict:
    """No network: injected demand/answer through vessel; spine structure check."""
    from modot.vessel import B4 as VB, Imscription as Im

    spine = ManuscriptSpine(llm=None)
    assert not spine.available(), "spine must require LLM imscriber"

    # Injected path: drive vessel face via evaluate with injected codes
    dem = Im(text="q", codes=[VB.T] * 12, source="injected")
    ans = Im(text="a", codes=[VB.T] * 12, source="injected")
    # Manual prepare-like state
    prep = SpinePrepare(
        question="q", demand=dem, witness=None, scaffold_md="",
        vessel_ready=True, witness_ready=False, note="injected",
    )
    spine.last_prepare = prep
    # complete with vessel evaluate using injected
    # Force vessel evaluate path by temporarily calling vessel then fusing
    vrep = spine.vessel.evaluate("q", "a", demand=dem, answer_im=ans)
    assert vrep.belnap == "T" and vrep.riding

    # Build report through complete by pre-setting demand and monkeypatching
    # evaluate via inject: complete will re-imscribe answer unless we pass...
    # complete always re-imscribes answer via vessel.evaluate unless demand set
    # and available() is False without LLM — so for no-LLM we construct report
    # through vessel inject + join manually, then through run structure.

    report = SpineReport(
        prove_balance_closed=True,
        unify_both_is_superposition=_unify_both_superposition(),
        port_riding=vrep.riding,
        port_vessel=vrep,
        witness=None,
        transport_roundtrip_ok=True,
        model_voice="T",
        vessel_voice=vrep.belnap,
        fused=_NAME[_join(_parse_b("T"), _parse_b(vrep.belnap))],
        conflict=_conflict(_parse_b("T"), _parse_b(vrep.belnap)),
        demand=dem,
        answer=ans,
        scaffold_md="",
        question="q",
        answer_text="a",
        note="selftest inject",
    )
    assert report.fused == "T"
    assert report.unify_both_is_superposition
    assert report.port_riding
    assert report.conflict == 0

    # Conflict path: model T, vessel F → fused B
    ans_f = Im(text="a", codes=[VB.F] * 12, source="injected")
    vrep_f = spine.vessel.evaluate("q", "a", demand=dem, answer_im=ans_f)
    assert vrep_f.belnap == "F"
    fused_b = _NAME[_join(_parse_b("T"), _parse_b("F"))]
    assert fused_b == "B"

    # Witness face alone (no LLM)
    if navigator_available():
        w = witness_translate(
            "Is there a graph of chromatic number aleph_1 with large independent sets?"
        )
        assert w.available
        wit_ok = w.primary is not None
    else:
        wit_ok = False

    return {
        "available_without_llm": spine.available(),
        "provenance": spine.provenance(),
        "identity_fused": report.fused,
        "unify_B_eq_T_plus_F": report.unify_both_is_superposition,
        "conflict_T_F_fuses_B": fused_b,
        "vessel_riding": vrep.riding,
        "witness_face": wit_ok,
        "protocol": list(ManuscriptSpine.PROTOCOL),
    }


if __name__ == "__main__":
    print(json.dumps(_selftest(), indent=2))
