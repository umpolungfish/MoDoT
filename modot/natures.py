"""Primitive-type natures: the elaborated character of each of the 49 types.

Each of the 49 primitive types was auto-designed into a full ob3ect whose
`lean_scaffold` carries a completed 12-family Imscription tuple. The values in
that tuple are themselves drawn from the same 49 names: the alphabet is its own
value set, so the natures form a closed (ouroboric) algebra. This module loads
those ob3ects, structures each into a `Nature`, and bridges the 12 families to
the vessel's PRIMITIVE_KEYS so any type can be imscribed to a Belnap/SIC state
by table lookup rather than by asking an LLM.

The phase_1 opcodes of every ob3ect are exactly `modot.composer.Token`, so a
nature also ships as an IMASM word in MoDoT's own token algebra.
"""

from __future__ import annotations

import glob
import json
import os
import re
from dataclasses import dataclass, field
from typing import Dict, List, Optional

from modot.composer import Token

# ── the twelve families, in scaffold field order, mapped to vessel keys ──────
# scaffold field -> vessel PRIMITIVE_KEYS symbol (see modot.vessel._PRIMITIVE_GLOSS)
FAMILY_TO_VESSEL_KEY: Dict[str, str] = {
    "dim": "D",    # dimensionality / depth of structure
    "top": "T",    # topology / connectivity
    "rel": "R",    # recognition / self-reference and naming
    "pol": "P",    # parity / symmetry under inversion
    "fid": "F",    # fidelity / precision of content
    "kin": "K",    # kinetics / rate, change, temporal character
    "gram": "G",   # coupling / linkage between parts (grammatical binding)
    "gran": "Gm",  # granularity / resolution of articulation
    "crit": "Ph",  # criticality / phase-boundary / fixed-point
    "chir": "H",   # chirality / handedness, orientation, bias
    "stoi": "S",   # stoichiometry / proportion, balance of parts
    "prot": "W",   # winding / circulation, return, ouroboric character
}

FAMILY_ORDER = list(FAMILY_TO_VESSEL_KEY.keys())

_DATA_DIR = os.path.join(os.path.dirname(__file__), "..", "ob3ects", "primitives")

_TUPLE_RE = re.compile(r"\{ dim :=.*?\}")
_FIELD_RE = re.compile(r"(\w+) := (\w+)")
_SIG_RE = re.compile(r"sig=\(([^)]*)\)")
_PERIOD_RE = re.compile(r"period=(\d+)")
_SELFREF_RE = re.compile(r"self_ref=(\w+)")
_FROBORD_RE = re.compile(r"frobenius_order=(\d+)")
_PAIRS_RE = re.compile(r"FSPLIT/FFUSE pairs: (\[[^\]]*\])")
_TIER_RE = re.compile(r"Expected tier: (\S+)")


@dataclass
class Signature:
    """The derivation-path fingerprint that actually distinguishes a type.

    The endpoint tuple and the flat opcode list are near-constant across the 49;
    what separates them is the staged path: its signature, period, self-reference,
    Frobenius order, fork/fuse pairing, and the tier it folds to. This is the
    composer's TokenFingerprint, read off the scaffold header.
    """

    sig: tuple = ()
    period: int = 0
    self_ref: bool = False
    frobenius_order: int = 0
    pairs: str = ""
    tier: str = ""

    def key(self) -> tuple:
        return (self.sig, self.period, self.self_ref, self.frobenius_order, self.pairs)


def _signature(lean_scaffold: str) -> Signature:
    def _grp(rx, default=""):
        m = rx.search(lean_scaffold)
        return m.group(1) if m else default
    sig_str = _grp(_SIG_RE)
    sig = tuple(int(x) for x in sig_str.split(",")) if sig_str else ()
    return Signature(
        sig=sig,
        period=int(_grp(_PERIOD_RE, "0")),
        self_ref=_grp(_SELFREF_RE, "False") == "True",
        frobenius_order=int(_grp(_FROBORD_RE, "0")),
        pairs=_grp(_PAIRS_RE),
        tier=_grp(_TIER_RE),
    )


@dataclass
class Nature:
    """The elaborated nature of one primitive type."""

    name: str                              # e.g. "egg"
    domain_type: str                       # "computational" | "topological"
    surface_tokens: List[str]              # the type's physical handles
    tuple: Dict[str, str]                  # scaffold field -> value name (12 families)
    signature: Signature                   # derivation-path fingerprint (distinguishing)
    program: List[Token]                   # phase_1 opcode word (IMASM)
    belnap: Dict[str, str]                 # void / true / false / both descriptions
    frobenius_verdict: str                 # PASS / FAIL from phase_2
    entropy: str = ""                      # phase_6 delta_s_verdict
    boundary: str = ""                     # phase_0 boundary_condition
    _raw: dict = field(default=None, repr=False)

    def vessel_tuple(self) -> Dict[str, str]:
        """The 12-family tuple keyed by the vessel's PRIMITIVE_KEYS symbols."""
        return {FAMILY_TO_VESSEL_KEY[k]: v for k, v in self.tuple.items()}

    def references(self) -> List[str]:
        """The other primitive types this nature is built from (its tuple values)."""
        return sorted(set(self.tuple.values()))


def _name_from(d: dict) -> str:
    return d["name"].replace("the primitive type called ", "").replace(
        "the primitive called ", ""
    )


def _final_tuple(lean_scaffold: str) -> Dict[str, str]:
    tuples = _TUPLE_RE.findall(lean_scaffold)
    if not tuples:
        return {}
    return {k: v for k, v in _FIELD_RE.findall(tuples[-1])}


def _program(d: dict) -> List[Token]:
    ph1 = d.get("phases", {}).get("phase_1", {})
    ops: List[Token] = []
    for op in ph1:
        if hasattr(Token, op):
            ops.append(getattr(Token, op))
    return ops


def _load_one(path: str) -> Nature:
    d = json.load(open(path))
    p0 = d.get("phases", {}).get("phase_0", {})
    p2 = d.get("phases", {}).get("phase_2", {})
    p3 = d.get("phases", {}).get("phase_3", {})
    p6 = d.get("phases", {}).get("phase_6", {})
    return Nature(
        name=_name_from(d),
        domain_type=p0.get("domain_type", ""),
        surface_tokens=list(p0.get("surface_tokens", [])),
        tuple=_final_tuple(d.get("lean_scaffold", "")),
        signature=_signature(d.get("lean_scaffold", "")),
        program=_program(d),
        belnap={
            "void": p3.get("void_description", ""),
            "true": p3.get("true_description", ""),
            "false": p3.get("false_description", ""),
            "both": p3.get("both_description", ""),
        },
        frobenius_verdict=p2.get("frobenius_verdict", ""),
        entropy=p6.get("delta_s_verdict", ""),
        boundary=p0.get("boundary_condition", ""),
        _raw=d,
    )


class NatureRegistry:
    """Loads and indexes the 49 primitive-type natures."""

    def __init__(self, data_dir: str = _DATA_DIR):
        self.data_dir = data_dir
        self._by_name: Dict[str, Nature] = {}
        self._load()

    def _load(self) -> None:
        pattern = os.path.join(self.data_dir, "the_primitive_type_called_*_ob3ect.json")
        for path in sorted(glob.glob(pattern)):
            n = _load_one(path)
            self._by_name[n.name] = n

    def __len__(self) -> int:
        return len(self._by_name)

    def __contains__(self, name: str) -> bool:
        return name in self._by_name

    def __iter__(self):
        return iter(self._by_name.values())

    def get(self, name: str) -> Optional[Nature]:
        return self._by_name.get(name)

    def names(self) -> List[str]:
        return sorted(self._by_name)

    def by_family_value(self, family: str, value: str) -> List[Nature]:
        """Every nature whose `family` slot holds `value`."""
        return [n for n in self if n.tuple.get(family) == value]

    def by_tier(self, tier: str) -> List[Nature]:
        """Every nature that folds to `tier` (e.g. 'O₁')."""
        return [n for n in self if n.signature.tier == tier]

    def signature_classes(self) -> Dict[tuple, List[str]]:
        """Partition the types by derivation-path signature (the real fingerprint)."""
        out: Dict[tuple, List[str]] = {}
        for n in self:
            out.setdefault(n.signature.key(), []).append(n.name)
        return out

    def distinction_layers(self) -> Dict[str, int]:
        """How many classes each descriptive layer resolves the 49 into.

        Value tuple and flat opcode word are near-degenerate; the derivation-path
        signature and the semantic surface are what actually separate the types.
        """
        tuples = {tuple(sorted(n.tuple.items())) for n in self}
        progs = {tuple(t.name for t in n.program) for n in self}
        sigs = {n.signature.key() for n in self}
        surfaces = {tuple(n.surface_tokens) for n in self}
        return {
            "value_tuple": len(tuples),
            "opcode_word": len(progs),
            "path_signature": len(sigs),
            "surface_tokens": len(surfaces),
        }

    def closure_check(self) -> Dict[str, object]:
        """Verify the algebra is closed: every tuple value is a known type."""
        names = set(self._by_name)
        used = set()
        open_refs = set()
        for n in self:
            for v in n.tuple.values():
                used.add(v)
                if v not in names:
                    open_refs.add(v)
        return {
            "types": len(names),
            "distinct_values_used": len(used),
            "open_references": sorted(open_refs),
            "closed": not open_refs,
        }


# ── nature chemistry: co-typing at the IMASM value layer ─────────────────────
# Two natures are compared axis by axis over their twelve family values. This is
# the grounded layer: it invents nothing, it only reads values already fixed by
# the ob3ects. Where two natures name the same value on an axis, that axis
# co-types (settled, T). Where they name different values, both are held at once
# on that axis: a paradice, one quantum of paradox. The vessel's Belnap-code
# layer is deliberately NOT touched here — mapping a value to a B4 code needs a
# principled rule, and injecting a guess would violate the vessel's no-fallback
# contract.


@dataclass
class CotypeReport:
    """The result of co-typing two natures at the value layer."""

    a: str
    b: str
    agree: Dict[str, str]              # family -> shared value (settled axes)
    diverge: Dict[str, tuple]          # family -> (a_value, b_value) held Both
    paradices: int                     # count of divergent axes

    def address(self) -> List[str]:
        """The families at which the two natures part ways (vessel keys)."""
        return [FAMILY_TO_VESSEL_KEY[f] for f in self.diverge]

    def cotypes(self) -> bool:
        """True iff the two natures agree on every axis (zero paradices)."""
        return self.paradices == 0


def cotype(a: Nature, b: Nature) -> CotypeReport:
    """Co-type two natures over the twelve families. Grounded, no invented codes."""
    agree: Dict[str, str] = {}
    diverge: Dict[str, tuple] = {}
    for fam in FAMILY_ORDER:
        av, bv = a.tuple.get(fam), b.tuple.get(fam)
        if av is not None and av == bv:
            agree[fam] = av
        else:
            diverge[fam] = (av, bv)
    return CotypeReport(a=a.name, b=b.name, agree=agree,
                        diverge=diverge, paradices=len(diverge))


def paradices(a: Nature, b: Nature) -> int:
    """How live the contradiction between two natures is, in paradices."""
    return cotype(a, b).paradices


_REGISTRY: Optional[NatureRegistry] = None


def registry() -> NatureRegistry:
    """The process-wide nature registry (lazily loaded)."""
    global _REGISTRY
    if _REGISTRY is None:
        _REGISTRY = NatureRegistry()
    return _REGISTRY


def nature(name: str) -> Optional[Nature]:
    """Look up one primitive type's nature by name."""
    return registry().get(name)


def _selftest() -> None:
    reg = registry()
    assert len(reg) == 49, f"expected 49 natures, got {len(reg)}"
    chk = reg.closure_check()
    assert chk["closed"], f"algebra not closed: {chk['open_references']}"
    egg = reg.get("egg")
    assert egg is not None
    assert len(egg.tuple) == 12, f"egg tuple has {len(egg.tuple)} families"
    assert len(egg.vessel_tuple()) == 12
    assert egg.program and egg.program[0] == Token.VINIT
    assert egg.belnap["both"], "egg missing Both reading"
    # chemistry: a nature co-types with itself at zero paradices
    assert cotype(egg, egg).cotypes(), "egg does not co-type with itself"
    yea = reg.get("yea")
    rep = cotype(egg, yea)
    assert rep.paradices == len(rep.diverge)
    layers = reg.distinction_layers()
    assert layers["path_signature"] > layers["value_tuple"], \
        "path signature should out-resolve the near-degenerate value tuple"
    print(f"natures: {len(reg)} loaded, closed algebra, "
          f"{chk['distinct_values_used']} distinct values in use")
    print(f"distinction layers (classes / 49): {layers}")
    print(f"egg signature: sig={egg.signature.sig} period={egg.signature.period} "
          f"tier={egg.signature.tier} pairs={egg.signature.pairs}")
    print(f"cotype(egg,yea) at value layer: {rep.paradices} paradices "
          f"(degenerate: identical ground tuple)")


if __name__ == "__main__":
    _selftest()
