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


@dataclass
class Nature:
    """The elaborated nature of one primitive type."""

    name: str                              # e.g. "egg"
    domain_type: str                       # "computational" | "topological"
    surface_tokens: List[str]              # the type's physical handles
    tuple: Dict[str, str]                  # scaffold field -> value name (12 families)
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
    print(f"natures: {len(reg)} loaded, closed algebra, "
          f"{chk['distinct_values_used']} distinct values in use")
    print(f"egg vessel_tuple: {egg.vessel_tuple()}")


if __name__ == "__main__":
    _selftest()
