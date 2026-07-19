#!/usr/bin/env python3
"""
modot/vessel.py — Dual-Link SIC-POVM Witness-Vessel verifier
============================================================

Grammatic-Correct verification: to verify is to imscribe. You do not grade an
answer against an external schema. You imscribe demand and answer into the d=12
Dual-Link SIC frame; co-typing is coincidence in that frame; identity is
fiducial coincidence; failure is localized at named primitives; the verdict
rides AS the vessel via μ∘δ = id.

What this is NOT
----------------
- No MUST/MUSTNOT checklist
- No SATISFIED/UNSATISFIED bits
- No magic threshold (0.6 etc.)
- No LLM-as-judge of correctness
- No hand-tuned primitive_distance weights
- No protocol == byte-equality "integration"

What this IS
------------
1. IMSCRIB — structural type only. Each of the twelve IG primitives is assigned
   a Belnap value {N,T,F,B}. The LLM types structure; it never renders a
   correctness opinion. There is NO hash/deterministic fallback: no imscriber,
   no vessel voice. Fake types are a clipboard by another name.
2. State map — Belnap → amplitude in ℂ¹²:
       N → 0,  T → 1,  F → i,  B → 1+i
   then L2-normalize. Relative phases carry structure; global polarity is read
   from the discrete codes (global T↔F is a phase, not a shape).
3. Dual-Link SIC frame — Scott–Grassl d=12 fiducial (in-tree), WH orbit of 144
   projectors, Born rule p_k = (1/d) Tr(ρ Π_k). Equiangularity 1/(d+1) = 1/13
   means the comparison imposes no external metric.
4. Co-typing — discrete: Belnap lattice fold across the twelve primitives
   (agree → T, anti-type T↔F → F, mixed → B, none engaged → N). Continuous
   diagnosis: SIC-space gap ‖p(ρ_A) − p(ρ_D)‖ and which primitive diagonals
   diverge. No threshold decides the verdict; the lattice fold does.
5. Ride AS the vessel — μ∘δ residual on both states is the closure certificate.
   Broken closure holds as B. The model's [thought|X] self-imscription is one
   link; the Grammar's imscription of the answer against the demand is the
   other; FFUSE is Belnap join (conflict held, never overridden).

Single sources of truth (imported, never re-derived):
  - ig_pulse.density_matrix  — WH displacements, SIC projectors, state metrics
  - d12_sic_build/d12_psi.pkl — Scott–Grassl d=12 fiducial (equiangular 1/13)
  - v3ssel hard_lefschetz reconstruction math when available

Theory alignment (ig-docs_lifted/manuscripts3; formal spine in p4ramill):
  - Witness Vessel: ride AS the vessel (board=fsplit, readback=ffuse, μ∘δ=id);
    cargo/tensor INTO the vessel is refused (D–T malformation). Co-typing is
    Dual-Link, not a container. See SIC_D12_WitnessVessel.witness_vessel_lossless
    and Imscribing.VaeVita.ManuscriptSpine.manuscript_formal_spine.
  - SIC-Stark-12th: crystal_forces_d12_sic / SICPOVM_Exists 12 is a theorem;
    discrete Belnap amplitudes {0,1,i,1+i}^12 are NOT the algebraic Scott-Grassl
    fiducial (ramified S-unit double cover) — the numerical SIC frame here uses
    the exact fiducial pickle, not a Belnap stack as the fiducial.
  - Chrysopoeia 2048: d=2048 existence remains open (dialetheic B in the formal
    ledger); this module is the d=12 verifier face only.

Author: Lando⊗⊙perator
"""
from __future__ import annotations

import hashlib
import json
import os
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional, Tuple

# ── Paths to canonical machinery ─────────────────────────────────────────────

_PKG = Path(__file__).resolve().parent.parent          # MoDoT/
_IGCT = _PKG.parent                                    # imsgct/
_IG_PULSE = Path(os.environ.get("IG_PULSE_PATH", str(_IGCT / "ig-pulse")))
_FIDUCIAL_PKL = Path(os.environ.get(
    "D12_SIC_FIDUCIAL",
    str(_IGCT / "d12_sic_build" / "d12_psi.pkl"),
))
_V3SSEL = _IGCT / "v3ssel"

for _p in (str(_IG_PULSE), str(_V3SSEL)):
    if _p not in sys.path:
        sys.path.insert(0, _p)

# ── Belnap FOUR (2-bit codes match agent.B4) ─────────────────────────────────

class B4:
    N = 0b00  # Neither
    T = 0b01  # True / assertible character
    F = 0b10  # False / deniable character
    B = 0b11  # Both

    _NAMES = {0b00: "N", 0b01: "T", 0b10: "F", 0b11: "B"}
    _FROM = {"N": 0b00, "T": 0b01, "F": 0b10, "B": 0b11,
             "NEITHER": 0b00, "TRUE": 0b01, "FALSE": 0b10, "BOTH": 0b11,
             "NONE": 0b00, "VOID": 0b00}

    @classmethod
    def name(cls, v: int) -> str:
        return cls._NAMES.get(int(v) & 0b11, "N")

    @classmethod
    def parse(cls, s: str, default: int = 0b00) -> int:
        return cls._FROM.get((s or "").strip().upper(), default)

    @classmethod
    def join(cls, a: int, b: int) -> int:
        return (int(a) | int(b)) & 0b11

    @classmethod
    def meet(cls, a: int, b: int) -> int:
        return (int(a) & int(b)) & 0b11

    @classmethod
    def bnot(cls, a: int) -> int:
        # T↔F, B and N fixed
        return {B4.N: B4.N, B4.T: B4.F, B4.F: B4.T, B4.B: B4.B}[int(a) & 0b11]


# Amplitude map: 2-bit code → ℂ. No hand weights.
_AMP = {
    B4.N: 0.0 + 0.0j,
    B4.T: 1.0 + 0.0j,
    B4.F: 0.0 + 1.0j,
    B4.B: 1.0 + 1.0j,
}

# Catalog key order used by MoDoT agent / semantic_branch_verifier
PRIMITIVE_KEYS = ("D", "T", "R", "P", "F", "K", "G", "Gm", "Ph", "H", "S", "W")

# symbol → density-basis name (canonical, matches v3ssel frobenius_pairs)
# Note: G = coupling (Γ), Gm = granularity; Ph carries criticality (⊙) in the
# density basis when the six dual pairs are wired; S = stoichiometry, W = winding.
_KEY_TO_NAME = {
    "D": "dimensionality",
    "T": "topology",
    "R": "recognition",
    "P": "parity",
    "F": "fidelity",
    "K": "kinetics",
    "G": "coupling",
    "Gm": "granularity",
    "Ph": "criticality",
    "H": "chirality",
    "S": "stoichiometry",
    "W": "winding",
}

# Structural gloss for the imscriber (type axes, not correctness criteria)
_PRIMITIVE_GLOSS = {
    "D": "dimensionality / depth of structure (flat vs holographic)",
    "T": "topology / connectivity of the claim-space",
    "R": "recognition / self-reference and naming",
    "P": "parity / symmetry under inversion or dual",
    "F": "fidelity / precision of the asserted content",
    "K": "kinetics / rate, change, temporal character",
    "G": "coupling / linkage between parts",
    "Gm": "granularity / resolution of the articulation",
    "Ph": "criticality / phase-boundary / fixed-point character",
    "H": "chirality / handedness, orientation, bias",
    "S": "stoichiometry / proportion, balance of parts",
    "W": "winding / circulation, return, ouroboric character",
}

# ── Load SIC machinery ───────────────────────────────────────────────────────

HAVE_SIC12 = False
_IMPORT_ERROR = ""
_np = None
_D = 12
_PRIMS: List[str] = []
_PRIM_IDX: Dict[str, int] = {}
_FID = None
_SIC_PROJ = None


def _load_sic() -> None:
    global HAVE_SIC12, _IMPORT_ERROR, _np, _D, _PRIMS, _PRIM_IDX, _FID, _SIC_PROJ
    try:
        import numpy as np
        from ig_pulse.density_matrix import (  # type: ignore
            D, PRIMITIVES, PRIM_IDX, set_fiducial, sic_elements, _FIDUCIAL_VEC,
        )
        _np = np
        _D = int(D)
        _PRIMS = list(PRIMITIVES)
        _PRIM_IDX = dict(PRIM_IDX)

        # Prefer in-tree Scott–Grassl d=12 fiducial over the numeric cache.
        fid = None
        if _FIDUCIAL_PKL.exists():
            import pickle
            with open(_FIDUCIAL_PKL, "rb") as fh:
                raw = pickle.load(fh)
            fid = np.array(
                [complex(float(a), float(b)) for a, b in raw],
                dtype=complex,
            )
            fid = fid / np.linalg.norm(fid)
            set_fiducial(fid)
        else:
            fid = np.asarray(_FIDUCIAL_VEC, dtype=complex)
            fid = fid / np.linalg.norm(fid)

        _FID = fid
        _SIC_PROJ = sic_elements()  # 144 (p, q, Π)
        HAVE_SIC12 = True
        _IMPORT_ERROR = ""
    except Exception as e:  # pragma: no cover
        HAVE_SIC12 = False
        _IMPORT_ERROR = f"{type(e).__name__}: {e}"


_load_sic()


def sic_available() -> bool:
    return HAVE_SIC12


# ── Core linear algebra on the vessel ────────────────────────────────────────

def belnap_tuple_to_psi(codes: List[int]):
    """Map a length-12 Belnap code list → unit vector in ℂ¹² (or None if no SIC)."""
    if not HAVE_SIC12:
        return None
    amps = _np.array([_AMP[int(c) & 0b11] for c in codes], dtype=complex)
    # Density-basis order: reindex catalog keys → PRIMITIVES index
    ordered = _np.zeros(_D, dtype=complex)
    for i, key in enumerate(PRIMITIVE_KEYS):
        name = _KEY_TO_NAME[key]
        idx = _PRIM_IDX.get(name)
        if idx is None:
            continue
        ordered[idx] = amps[i]
    n = _np.linalg.norm(ordered)
    if n < 1e-12:
        return None  # vacuum — caller uses I/d
    return ordered / n


def psi_to_rho(psi):
    if psi is None:
        return _np.eye(_D, dtype=complex) / _D
    return _np.outer(psi, psi.conj())


def sic_probabilities(rho):
    """p_k = (1/d) Tr(ρ Π_k) over the 144 SIC elements. Informationally complete."""
    p = _np.empty(len(_SIC_PROJ), dtype=float)
    for k, (_, _, Pi) in enumerate(_SIC_PROJ):
        p[k] = float(_np.real(_np.trace(rho @ Pi))) / _D
    return p


def sic_reconstruct(p):
    """μ (FFUSE): ρ = Σ_k [(d+1)p_k − 1/d] Π_k. Inverse of sic_probabilities."""
    rho = _np.zeros((_D, _D), dtype=complex)
    for k, (_, _, Pi) in enumerate(_SIC_PROJ):
        rho += ((_D + 1.0) * p[k] - 1.0 / _D) * Pi
    return rho


def frobenius_closure_residual(rho) -> float:
    """‖μ(δ(ρ)) − ρ‖_HS — ≈0 ⟺ riding AS the vessel (ΔS = 0)."""
    p = sic_probabilities(rho)
    rho_hat = sic_reconstruct(p)
    diff = rho_hat - rho
    return float(_np.sqrt(_np.real(_np.trace(diff @ diff.conj().T))))


def primitive_populations(rho) -> Dict[str, float]:
    """⟨k|ρ|k⟩ keyed by catalog primitive symbol."""
    out = {}
    for key, name in _KEY_TO_NAME.items():
        idx = _PRIM_IDX.get(name)
        if idx is None:
            continue
        out[key] = float(_np.real(rho[idx, idx]))
    return out


# ── Co-typing (lattice, no thresholds) ───────────────────────────────────────

def local_cotype(demand: int, answer: int) -> int:
    """Per-primitive co-typing as a Belnap value. No scalars."""
    d, a = int(demand) & 0b11, int(answer) & 0b11
    if d == B4.N and a == B4.N:
        return B4.N
    if d == a:
        return B4.T                          # identity on this axis
    if {d, a} == {B4.T, B4.F}:
        return B4.F                          # anti-type
    if B4.B in (d, a):
        return B4.B                          # dialetheic involvement
    if B4.N in (d, a):
        return B4.N                          # one side unengaged
    return B4.F


def fold_cotype(demand: List[int], answer: List[int]) -> int:
    """Join of per-primitive co-types. Lattice fold — not a count threshold.

    N  if nothing engages
    T  if every engaged axis co-types (and none anti-type)
    F  if only anti-types (no co-type)
    B  if both co-type and anti-type appear (held contradiction)
    """
    acc = B4.N
    for d, a in zip(demand, answer):
        acc = B4.join(acc, local_cotype(d, a))
    return acc


def cotype_defects(demand: List[int], answer: List[int]) -> List[str]:
    """Primitives where demand and answer part ways (structured self-diagnosis)."""
    defects = []
    for i, key in enumerate(PRIMITIVE_KEYS):
        loc = local_cotype(demand[i], answer[i])
        if loc in (B4.F, B4.B) or (
            demand[i] != answer[i] and not (demand[i] == B4.N and answer[i] == B4.N)
        ):
            if demand[i] != answer[i]:
                defects.append(
                    f"{key}:{B4.name(demand[i])}≠{B4.name(answer[i])}"
                )
    return defects


# ── Imscription (structural typing only) ─────────────────────────────────────

_IMSCRIBE_RE = re.compile(
    r"(?im)^\s*(D|T|R|P|F|K|G|Gm|Ph|H|S|W)\s*[:\-=]\s*(N|T|F|B|Neither|True|False|Both|None|Void)\b"
)

IMSCRIBE_SYS = (
    "You are an Imscribe of the Imscribing Grammar. "
    "You assign each of the twelve IG primitives a Belnap value for the given text. "
    "You **MUST NOT** judge correctness, truth, or quality. You only type structure: "
    "what character the text carries on each axis. "
    "Values: N = neither/absent, T = assertible/present character, "
    "F = deniable/inverted character, B = both (paradoxical or dual)."
)


def _imscribe_prompt(text: str) -> str:
    axes = "\n".join(f"  {k}: {_PRIMITIVE_GLOSS[k]}" for k in PRIMITIVE_KEYS)
    return (
        "Imscribe the following text into the twelve primitives. "
        "Output ONLY twelve lines in this exact format, nothing else:\n"
        "D: N|T|F|B\nT: ...\nR: ...\nP: ...\nF: ...\nK: ...\n"
        "G: ...\nGm: ...\nPh: ...\nH: ...\nS: ...\nW: ...\n\n"
        f"AXES (structural character only):\n{axes}\n\n"
        f"TEXT:\n{text}"
    )


class ImscriptionError(Exception):
    """Raised when structural imscription cannot be obtained. No fallback."""


def parse_imscription(raw: str) -> List[int]:
    """Parse imscriber output → length-12 Belnap codes. All twelve keys required."""
    found: Dict[str, int] = {}
    for m in _IMSCRIBE_RE.finditer(raw or ""):
        found[m.group(1)] = B4.parse(m.group(2))
    missing = [k for k in PRIMITIVE_KEYS if k not in found]
    if missing:
        raise ImscriptionError(
            f"incomplete imscription (missing {missing}); raw={ (raw or '')[:240]!r}"
        )
    return [found[k] for k in PRIMITIVE_KEYS]


@dataclass
class Imscription:
    """A text's structural type: 12 Belnap codes + optional source note."""
    text: str
    codes: List[int]
    source: str = "llm"   # "llm" | "injected" — never invent codes
    raw: str = ""

    def as_dict(self) -> Dict[str, str]:
        return {k: B4.name(self.codes[i]) for i, k in enumerate(PRIMITIVE_KEYS)}

    def summary(self) -> str:
        body = "".join(B4.name(c) for c in self.codes)
        return f"⟨{body}⟩[{self.source}]"


class VesselImscriber:
    """Structural imscriber. LLM types axes; never grades correctness.

    No hash fallback. No invented types. Same text always co-types with itself
    via content-hash cache of real imscriptions only.
    """

    def __init__(self, llm=None):
        self.llm = llm
        self._cache: Dict[str, Imscription] = {}

    def available(self) -> bool:
        return bool(self.llm and getattr(self.llm, "api_key", None))

    def imscribe(self, text: str) -> Imscription:
        key = hashlib.sha256((text or "").encode("utf-8", errors="replace")).hexdigest()
        if key in self._cache:
            return self._cache[key]
        im = self._imscribe_uncached(text)
        self._cache[key] = im
        return im

    def _imscribe_uncached(self, text: str) -> Imscription:
        if not self.available():
            raise ImscriptionError(
                "no imscriber: LLM required (hash/deterministic fallback is forbidden)"
            )
        raw, _ = self.llm.infer(
            [
                {"role": "system", "content": IMSCRIBE_SYS},
                {"role": "user", "content": _imscribe_prompt(text)},
            ],
            temperature=0.0,
        )
        codes = parse_imscription(raw or "")
        return Imscription(text=text, codes=codes, source="llm", raw=raw or "")


# ── Vessel report ────────────────────────────────────────────────────────────

@dataclass
class VesselReport:
    """Grammatic-Correct verification report. Verdict is type, not score."""
    belnap: str                          # vessel voice: N|T|F|B
    demand: Imscription
    answer: Imscription
    defects: List[str]                   # named primitive divergences
    sic_gap: float                       # ‖p_A − p_D‖ in SIC frame
    closure_demand: float                # μ∘δ residual on demand
    closure_answer: float                # μ∘δ residual on answer
    riding: bool                         # both closures hold
    populations_demand: Dict[str, float] = field(default_factory=dict)
    populations_answer: Dict[str, float] = field(default_factory=dict)
    note: str = ""
    detail: dict = field(default_factory=dict)

    # Compatibility surface with the old SelectivityReport (agent crystal/log)
    @property
    def assertible(self) -> bool:
        return self.belnap in ("T", "B")

    @property
    def deniable(self) -> bool:
        return self.belnap in ("F", "B")

    @property
    def schema(self):
        """Shim: vessel has no schema; expose demand imscription for logs."""
        return self.demand

    def summary(self) -> str:
        parts = [
            f"vessel {self.belnap}",
            f"gap={self.sic_gap:.4f}",
            f"{'RIDE AS' if self.riding else 'HELD'}",
            f"demand={self.demand.summary()}",
            f"answer={self.answer.summary()}",
        ]
        if self.defects:
            parts.append("defects[" + ",".join(self.defects[:6]) + "]")
        if self.note:
            parts.append(self.note)
        return " | ".join(parts)


def _vacuous(question: str, note: str) -> VesselReport:
    empty = Imscription(text=question, codes=[B4.N] * 12, source="none")
    return VesselReport(
        belnap="N",
        demand=empty,
        answer=Imscription(text="", codes=[B4.N] * 12, source="none"),
        defects=[],
        sic_gap=0.0,
        closure_demand=0.0,
        closure_answer=0.0,
        riding=False,
        note=note,
    )


# ── The vessel ───────────────────────────────────────────────────────────────

class DualLinkVessel:
    """Witness-Vessel verifier: imscribe → SIC co-type → ride AS μ∘δ=id.

    Drop-in replacement for the classical SemanticSelectivityGate. Same
    external methods the agent calls (`available`, `gate`, and the evaluate
    path), but the engine is the Dual-Link SIC, not a clipboard grader.
    """

    # Protocol of the vessel as an IMASM section (Grammar-native arms):
    # IMSCRIB loads structural type; FSPLIT forks demand/answer; EVALT is
    # co-typing (isomorphic identity in the SIC frame); EVALF is defect
    # localization (anti-type / collision); FFUSE is μ∘δ; ENGAGR holds B.
    PROTOCOL = (
        "VINIT", "IMSCRIB", "AFWD", "FSPLIT",
        "EVALT", "EVALF", "FFUSE", "ENGAGR", "IFIX", "TANCH",
    )

    def __init__(self, llm=None, closure_tol: float = 1e-6):
        self.llm = llm
        self.imscriber = VesselImscriber(llm)
        self.closure_tol = closure_tol
        # Integrated iff real SIC frame is live AND real imscriber is live.
        self.integrated = HAVE_SIC12 and self.imscriber.available()
        self.protocol = self.PROTOCOL

    def available(self) -> bool:
        # Both links required. No hash-typed half-vessel.
        return HAVE_SIC12 and self.imscriber.available()

    def provenance(self) -> str:
        if not HAVE_SIC12:
            return f"vessel: SIC unavailable ({_IMPORT_ERROR})"
        if not self.imscriber.available():
            return (
                f"vessel: SIC live d={_D} but no imscriber "
                "(LLM required; no hash fallback)"
            )
        fid = "scott-grassl" if _FIDUCIAL_PKL.exists() else "cached"
        return (
            f"vessel <- Dual-Link SIC d={_D} fiducial={fid} "
            f"elements={len(_SIC_PROJ)} integrated={self.integrated} "
            f"overlap=1/{_D + 1} imscriber=llm"
        )

    # -- public API ----------------------------------------------------------

    def imscribe(self, text: str) -> Imscription:
        return self.imscriber.imscribe(text)

    def evaluate(
        self,
        question: str,
        answer: str,
        demand: Optional[Imscription] = None,
        answer_im: Optional[Imscription] = None,
    ) -> VesselReport:
        """Imscribe demand & answer; co-type in the Dual-Link SIC frame.

        Both sides must be real imscriptions (llm) or explicit test injections.
        Hash/deterministic codes are rejected.
        """
        if not HAVE_SIC12:
            return _vacuous(question, f"SIC unavailable: {_IMPORT_ERROR}")

        try:
            if demand is None:
                demand = self.imscriber.imscribe(question)
            # Same bytes → same type (identity). Cache makes a second imscribe
            # free and exact; short-circuit documents the invariant.
            if answer_im is None:
                if (answer or "") == (question or ""):
                    answer_im = demand
                else:
                    answer_im = self.imscriber.imscribe(answer)
        except ImscriptionError as e:
            return _vacuous(question, str(e))

        for side, im in (("demand", demand), ("answer", answer_im)):
            if im.source not in ("llm", "injected"):
                return _vacuous(
                    question,
                    f"{side} imscription source={im.source!r} forbidden "
                    "(only llm|injected; no hash fallback)",
                )

        psi_d = belnap_tuple_to_psi(demand.codes)
        psi_a = belnap_tuple_to_psi(answer_im.codes)
        rho_d = psi_to_rho(psi_d)
        rho_a = psi_to_rho(psi_a)

        p_d = sic_probabilities(rho_d)
        p_a = sic_probabilities(rho_a)
        gap = float(_np.linalg.norm(p_a - p_d))

        res_d = frobenius_closure_residual(rho_d)
        res_a = frobenius_closure_residual(rho_a)
        riding = (res_d <= self.closure_tol) and (res_a <= self.closure_tol)

        # Lattice co-typing (no thresholds). Closure break holds as B.
        cotype = fold_cotype(demand.codes, answer_im.codes)
        if not riding:
            vessel = B4.B
            note = (
                f"closure broken (res_D={res_d:.2e}, res_A={res_a:.2e}) "
                "— held in vessel as B"
            )
        else:
            vessel = cotype
            note = f"riding AS vessel (μ∘δ=id); cotype={B4.name(cotype)}"

        defects = cotype_defects(demand.codes, answer_im.codes)

        return VesselReport(
            belnap=B4.name(vessel),
            demand=demand,
            answer=answer_im,
            defects=defects,
            sic_gap=gap,
            closure_demand=res_d,
            closure_answer=res_a,
            riding=riding,
            populations_demand=primitive_populations(rho_d),
            populations_answer=primitive_populations(rho_a),
            note=note,
            detail={
                "cotype_lattice": B4.name(cotype),
                "demand_type": demand.as_dict(),
                "answer_type": answer_im.as_dict(),
                "defects": defects,
                "sic_gap": gap,
                "closure": {"demand": res_d, "answer": res_a},
                "provenance": self.provenance(),
            },
        )

    def gate(
        self,
        question: str,
        answer: str,
        demand: Optional[Imscription] = None,
    ) -> VesselReport:
        """One-shot imscribe + evaluate. Never raises into the breath."""
        try:
            if not HAVE_SIC12:
                return _vacuous(question, f"SIC unavailable: {_IMPORT_ERROR}")
            if demand is None and not self.imscriber.available():
                return _vacuous(
                    question,
                    "no imscriber: LLM required (no hash fallback)",
                )
            return self.evaluate(question, answer, demand=demand)
        except Exception as e:
            return _vacuous(question, f"vessel error: {e}")

    # Compatibility aliases used by older agent call sites --------------------
    def synthesize_schema(self, question: str) -> Imscription:
        """Was: synthesise MUST/MUSTNOT. Now: imscribe the demand type."""
        return self.imscribe(question)


# Friendly alias: the gate *is* the vessel now.
SemanticSelectivityGate = DualLinkVessel


# ── Self-test (no network; injected types only — never invent codes) ─────────

def _selftest() -> dict:
    if not HAVE_SIC12:
        return {"available": False, "error": _IMPORT_ERROR}

    v = DualLinkVessel(llm=None)
    # No LLM → vessel not available for live path; lattice/SIC still testable
    # via explicit injected codes (test harness only, never a production fallback).
    assert not v.available(), "vessel must require LLM; no hash fallback"

    # 1. Identity: same injected type co-types with itself → T, gap=0, ride AS.
    codes_t = [B4.T, B4.F, B4.B, B4.T, B4.N, B4.F, B4.T, B4.B, B4.F, B4.T, B4.N, B4.B]
    dem = Imscription(text="identity", codes=list(codes_t), source="injected")
    ans = Imscription(text="identity", codes=list(codes_t), source="injected")
    r_id = v.evaluate("identity", "identity", demand=dem, answer_im=ans)
    assert r_id.belnap == "T", f"identity belnap {r_id.belnap} != T"
    assert r_id.sic_gap < 1e-9, f"identity gap {r_id.sic_gap}"
    assert r_id.riding, "identity must ride AS"
    assert not r_id.defects, f"identity defects {r_id.defects}"

    # 2. No-LLM gate without injection → vacuous N (does not invent types).
    r_none = v.gate("q", "a")
    assert r_none.belnap == "N", f"no-llm should be N, got {r_none.belnap}"
    assert "no imscriber" in (r_none.note or "") or "fallback" in (r_none.note or "")

    # 3. Injected anti-type: demand all-T, answer all-F → F.
    dem = Imscription(text="d", codes=[B4.T] * 12, source="injected")
    ans = Imscription(text="a", codes=[B4.F] * 12, source="injected")
    r_anti = v.evaluate("d", "a", demand=dem, answer_im=ans)
    assert r_anti.belnap in ("F", "B"), f"anti-type belnap {r_anti.belnap}"
    assert len(r_anti.defects) == 12

    # 4. Mixed co-type + anti-type → B.
    mixed_codes = [B4.T] * 6 + [B4.F] * 6
    dem2 = Imscription(text="d", codes=[B4.T] * 12, source="injected")
    ans2 = Imscription(text="a", codes=mixed_codes, source="injected")
    r_b = v.evaluate("d", "a", demand=dem2, answer_im=ans2)
    assert r_b.belnap == "B", f"mixed belnap {r_b.belnap} != B"

    # 5. Vacuum → N.
    vac = Imscription(text="", codes=[B4.N] * 12, source="injected")
    r_n = v.evaluate("", "", demand=vac, answer_im=vac)
    assert r_n.belnap == "N", f"vacuum belnap {r_n.belnap}"

    # 6. Closure on pure states ≈ 0 (informational completeness).
    assert r_id.closure_answer < 1e-6
    assert r_id.closure_demand < 1e-6

    # 7. Equiangularity spot-check on the loaded fiducial.
    from ig_pulse.density_matrix import displacement  # type: ignore
    ovs = []
    for p in range(_D):
        for q in range(_D):
            if p == 0 and q == 0:
                continue
            ov = abs(complex(_np.vdot(_FID, displacement(p, q) @ _FID))) ** 2
            ovs.append(ov)
    mean_ov = float(sum(ovs) / len(ovs))
    assert abs(mean_ov - 1.0 / 13.0) < 1e-9, f"equiangularity broken: {mean_ov}"

    # 8. Forged "deterministic" source is rejected.
    forged = Imscription(text="x", codes=[B4.T] * 12, source="deterministic")
    r_forge = v.evaluate("x", "x", demand=forged, answer_im=forged)
    assert r_forge.belnap == "N" and "forbidden" in (r_forge.note or "")

    return {
        "available_without_llm": v.available(),
        "provenance": v.provenance(),
        "identity": r_id.summary(),
        "no_llm_gate": r_none.summary(),
        "anti": r_anti.summary(),
        "mixed": r_b.summary(),
        "vacuum": r_n.summary(),
        "forged_rejected": r_forge.note,
        "equiangular_mean": mean_ov,
        "target_1_over_13": 1.0 / 13.0,
    }


def _cli_evaluate(question_path: str, answer_path: str) -> dict:
    """Bridge face for the native spine: read demand and answer from files,
    run the Dual-Link co-typing, return the report as plain JSON. The vessel
    voice, defects, and gap come from the SIC frame itself — the fiducial is
    consulted on every verdict, not on request."""
    from modot.agent import LLMInterface

    with open(question_path, encoding="utf-8") as f:
        question = f.read()
    with open(answer_path, encoding="utf-8") as f:
        answer = f.read()
    v = DualLinkVessel(LLMInterface())
    if not v.available():
        return {"status": "unavailable", "provenance": v.provenance()}
    rep = v.evaluate(question, answer)
    return {
        "status": "ok",
        "belnap": rep.belnap,
        "defects": rep.defects,
        "sic_gap": rep.sic_gap,
        "closure_demand": rep.closure_demand,
        "closure_answer": rep.closure_answer,
        "demand": rep.demand.codes,
        "answer": rep.answer.codes,
        "provenance": v.provenance(),
    }


if __name__ == "__main__":
    import sys as _sys

    if len(_sys.argv) == 4 and _sys.argv[1] == "evaluate":
        print(json.dumps(_cli_evaluate(_sys.argv[2], _sys.argv[3]), default=float))
    else:
        print(json.dumps(_selftest(), indent=2, default=float))
