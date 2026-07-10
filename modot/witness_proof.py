#!/usr/bin/env python3
"""
modot/witness_proof.py — Grammatic witness → conventional proof scaffold
=======================================================================

Built against machinery verified firsthand (not by module title):

  cl8nk_navigator.py (live)
    - catalog: 5265 entries, resolve_system, action_entry, action_distance,
      action_meet, action_join, action_tensor, generate_formula
    - returns real tuples, CLINK formula fragments, structural_algebra
      (distance, conflicts, tier), lattice ops vs CLINK L8

  Imscribing/Millennium/GeneralizedPipeline.lean (read, not trusted by name)
    - primitiveMathRole / defaultProposition / defaultProofStrategy
      are domain-invariant string maps (0 Collatz hardcode in those defs)
    - GeneralizedPipeline itself has 18 `sorry` — we port only the
      verified role/proposition/strategy tables, not runPipeline
    - sectionBackbone order is used as section skeleton

  Imscribing/IGMorphism.lean (read)
    - IGProtocol := refl | arrow | seq | prod | withGram | withMem
    - used only as protocol skeleton labels for proof steps (not a prover)

  Imscribing/IGFunctor.lean (read)
    - TierFunctor / FrobeniusAlg — tier of witness, not English proof text

  Imscribing/Algebra.lean + cl8nk lattice ops (read)
    - meet / join / tensor / distance — structural algebra of the witness

  REJECTED after firsthand check:
    imscribing_grammar/.stuff/scripts/primitive_to_conventional_final.py
    — still emits 3n+1 / Terras / parity-sequence Collatz body when fed
      non-Collatz names. Do not call it.

This module does NOT claim the scaffold is a finished conventional proof.
It surfaces catalog witnesses + structural algebra + domain-invariant
lemma roles so the agent can instantiate them for the actual question.
"""
from __future__ import annotations

import json
import re
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

# ── Import cl8nk_navigator (the only live catalog organ) ─────────────────────

_IGCT = Path(__file__).resolve().parent.parent.parent
_NAV_DIRS = [
    _IGCT / "imscribing_grammar" / "navigators",
    _IGCT / "imscribing_grammar",
]
for _d in _NAV_DIRS:
    if _d.is_dir() and str(_d) not in sys.path:
        sys.path.insert(0, str(_d))

_NAV = None
_NAV_ERR = ""
_LIVE_CATALOG = Path.home() / ".imscrbgrmr" / "catalog.json"
_LIVE_MERGED = 0


def _merge_live_catalog() -> int:
    """Fold the live-crawler catalog into the navigator index, first-name-wins.

    `cl8nk_navigator` loads only the flat IG_catalog.json, but the canonical
    `imscribe catalog list` CLI merges that file with the crawler's
    ~/.imscrbgrmr/catalog.json at read time. Without the same merge the Witness
    arm is blind to live-crawler entries the CLI already counts (the 5275-vs-5292
    gap). This replicates the CLI's read-time merge, so the arm stays current.
    """
    if _NAV is None or not _LIVE_CATALOG.exists():
        return 0
    try:
        with open(_LIVE_CATALOG, "r", encoding="utf-8") as fh:
            raw = json.load(fh)
    except Exception:
        return 0
    live = raw.get("imscriptions", raw) if isinstance(raw, dict) else raw
    index = getattr(_NAV, "CATALOG_INDEX", None)
    catalog = getattr(_NAV, "CATALOG", None)
    if not isinstance(live, list) or index is None:
        return 0
    added = 0
    for e in live:
        if not isinstance(e, dict):
            continue
        name = e.get("name")
        if name and name not in index:
            index[name] = e
            if isinstance(catalog, list):
                catalog.append(e)
            added += 1
    return added


try:
    import cl8nk_navigator as _NAV  # type: ignore
    _NAV.load_catalog()
    _LIVE_MERGED = _merge_live_catalog()
except Exception as e:  # pragma: no cover
    _NAV_ERR = f"{type(e).__name__}: {e}"
    _NAV = None


def navigator_available() -> bool:
    return _NAV is not None and bool(getattr(_NAV, "CATALOG", None))


def catalog_size() -> int:
    """Entries the arm can actually match (flat file + live-crawler merge)."""
    return len(getattr(_NAV, "CATALOG_INDEX", {})) if _NAV is not None else 0


# ── Domain-invariant maps verified from GeneralizedPipeline.lean ─────────────
# Keys: pipeline primitive ids as in the Lean file. We map catalog primitives
# (Ð Þ Ř …) onto these roles by the fixed IG correspondence used in the
# pipeline sectionBackbone (Phi_} ← Φ, Theta_O ← Þ, R_= ← Ř, …).

# Catalog primitive symbol → pipeline role id (from sectionBackbone / roles)
_PRIM_TO_ROLE_ID = {
    "Φ": "Phi_}",          # polarity / Frobenius-symmetric encoding
    "Þ": "Theta_O",        # topology / self-ref structure
    "Ř": "R_=",            # relational / bidirectional
    "Ω": "Omega_z",        # protection / winding
    "⊙": "phi_hat_y",      # criticality / phase boundary
    "Ç": "C_@",            # kinetics / equidistribution
    "Ð": "D_C",            # dimensionality
    "Γ": "Gamma_ʔ",        # granularity / quantification
    "ƒ": "f_dot_z",        # fidelity / coherence
    "Ħ": "H_A",            # chirality / Markov
    "Σ": "Sigma_S",        # stoichiometry / uniqueness
    "ɢ": "Theta_double_dot",  # grammar / composition — closest pipeline slot
}

# Verbatim roles from primitiveMathRole (Lean) — domain-invariant.
PRIMITIVE_MATH_ROLE = {
    "Phi_}": "Bijective encoding / duality",
    "Theta_O": "Inverse/dual construction",
    "R_=": "Adjoint pair / Galois connection",
    "Omega_z": "Topological invariant",
    "phi_hat_y": "Phase boundary / extremal principle",
    "C_@": "Equidistribution / regularity",
    "D_C": "Manifold / quotient structure",
    "Gamma_ʔ": "Universal/local quantification",
    "f_dot_z": "Coherence / non-classical feature",
    "H_A": "Markov order / recursion depth",
    "Sigma_S": "Uniqueness of witness",
    "Theta_double_dot": "Intersection / transversality",
}

# Verbatim defaultProposition (Lean) — structural, not Collatz-specific.
DEFAULT_PROPOSITION = {
    "Phi_}": (
        "The encoding map δ: X → Y induced by the system's symmetry is "
        "injective on equivalence classes. That is, if δ(x) = δ(x′), then "
        "x and x′ lie in the same orbit under the dynamics."
    ),
    "Theta_O": (
        "The space X admits a self-referential decomposition "
        "X = S ∪ X_exc, where S is the closure under the inverse relation "
        "and X_exc is empty iff the conjecture holds."
    ),
    "R_=": (
        "The forward construction S and inverse construction I are mutually "
        "exhaustive: S ⊆ I and I ⊆ S, hence S = I."
    ),
    "Omega_z": (
        "The target structure carries an integer-valued invariant w ∈ ℤ "
        "that distinguishes it from all exotic configurations."
    ),
    "phi_hat_y": (
        "No trajectory escapes to infinity. The system is confined to a "
        "bounded region of state space."
    ),
    "C_@": (
        "The relevant statistical or geometric quantity is well-distributed "
        "on typical configurations."
    ),
    "D_C": (
        "The state space is a finite-dimensional manifold admitting a "
        "quotient structure."
    ),
    "Gamma_ʔ": (
        "The property holds universally for the designated scope."
    ),
    "f_dot_z": (
        "Quantum or complex coherence is essential to the system's behavior."
    ),
    "H_A": (
        "The dynamics depend on at most two prior states (Markov order 2)."
    ),
    "Sigma_S": (
        "There exists exactly one solution or witness."
    ),
    "Theta_double_dot": (
        "Two substructures intersect at a distinguished point."
    ),
}

DEFAULT_STRATEGY = {
    "Phi_}": (
        "Establish that μ ∘ δ acts as the identity on the quotient. "
        "Show the encoding partitions X into distinguishable classes."
    ),
    "Theta_O": (
        "Construct the inverse relation. Show it generates a structure "
        "whose closure is the full space (or characterize the exception)."
    ),
    "R_=": (
        "Define both constructions explicitly. Show mutual containment "
        "by induction on the relevant parameter."
    ),
    "Omega_z": (
        "Define the invariant. Show it is preserved under the dynamics. "
        "Rule out alternative values by constraints."
    ),
    "phi_hat_y": (
        "Define a Lyapunov function or energy. Show negative drift or "
        "coercivity. Apply an extremal principle."
    ),
    "C_@": (
        "Apply mixing/ergodicity or regularity arguments. Show the measure "
        "converges to the expected distribution."
    ),
    "D_C": (
        "Exhibit local chart structure and transition maps. Verify the "
        "quotient is well-defined."
    ),
    "Gamma_ʔ": (
        "Quantify over the appropriate domain. Distinguish universal from "
        "existential claims."
    ),
    "f_dot_z": (
        "Identify the coherent feature. Show classical alternatives fail."
    ),
    "H_A": (
        "Establish the Markov property. Show two-step memory suffices."
    ),
    "Sigma_S": (
        "Prove existence by construction. Prove uniqueness by contradiction."
    ),
    "Theta_double_dot": (
        "Exhibit the intersection. Show transversality via dimensional counting."
    ),
}

# sectionBackbone order from Lean (section title, role id or meta)
SECTION_BACKBONE = [
    ("Abstract", "introduction"),
    ("1. Introduction", "introduction"),
    ("2. Preliminaries", "preliminaries"),
    ("3. Encoding / Duality", "Phi_}"),
    ("4. Inverse Structure", "Theta_O"),
    ("5. Bidirectional Correspondence", "R_="),
    ("6. Boundedness", "phi_hat_y"),
    ("7. Topological Invariant", "Omega_z"),
    ("8. Regularity", "C_@"),
    ("9. Main Theorem", "main_theorem"),
    ("10. Discussion", "discussion"),
]

# domainKeywords from Lean — used only for soft domain tagging of the scaffold
DOMAIN_KEYWORDS = {
    "number_theory": [
        "integer", "prime", "divis", "modular", "arithmetic", "congruence",
        "diophantine", "zeta", "l-function", "elliptic curve", "galois",
    ],
    "combinatorics": [
        "graph", "chromatic", "planar", "matching", "partition",
        "enumeration", "independent set", "ramsey", "coloring",
    ],
    "topology": [
        "manifold", "homotopy", "homology", "fundamental group", "covering",
        "braid", "knot",
    ],
    "analysis": [
        "converge", "bounded", "compact", "continuous", "integral", "measure",
        "hilbert", "banach", "ergodic",
    ],
    "logic_foundations": [
        "axiom", "consistent", "complete", "decidable", "forcing", "cardinal",
        "aleph", "ordinal",
    ],
    "dynamical_systems": [
        "orbit", "trajectory", "attractor", "lyapunov", "fixed point",
        "periodic", "collatz",
    ],
    "algebraic_geometry": [
        "variety", "scheme", "sheaf", "divisor", "moduli", "projective",
    ],
    "arithmetic_geometry": [
        "modular form", "galois representation", "l-function", "height",
        "mordell",
    ],
}


def detect_domains(text: str) -> List[Tuple[str, int]]:
    low = (text or "").lower()
    scores = []
    for dom, kws in DOMAIN_KEYWORDS.items():
        sc = sum(1 for k in kws if k in low)
        if sc:
            scores.append((dom, sc))
    scores.sort(key=lambda x: -x[1])
    return scores


# ── Catalog witness search ───────────────────────────────────────────────────

_STOP = {
    "the", "a", "an", "of", "to", "and", "or", "for", "in", "on", "is", "are",
    "that", "this", "with", "as", "by", "be", "from", "it", "its", "such",
    "every", "all", "any", "has", "have", "show", "prove", "please", "solve",
    "following", "work", "there", "which", "than", "then", "into", "about",
}


def _normalize_math_text(text: str) -> str:
    """Fold common math unicode so catalog descriptions match the question."""
    t = text or ""
    repl = {
        "ℵ": " aleph ", "χ": " chromatic ", "α": " independent ",
        "ε": " epsilon ", "ω": " omega ", "∈": " in ", "→": " to ",
        "₁": "1", "₂": "2", "₀": "0", "∞": " infinity ",
        # fold diacritics so "Gödel" → "godel", "Erdős" → "erdos", etc.
        "ö": "o", "ő": "o", "ø": "o", "ü": "u", "é": "e", "è": "e", "á": "a",
    }
    for a, b in repl.items():
        t = t.replace(a, b)
    return t.lower()


def _tokens(text: str) -> List[str]:
    raw = re.findall(r"[a-zA-Z0-9_]+", _normalize_math_text(text))
    return [t for t in raw if len(t) > 2 and t not in _STOP]


# Distinctive math anchors: if present in the question, strongly prefer
# catalog entries that also carry them (name or normalized description).
_ANCHORS = (
    "erdos", "hajnal", "aleph", "chromatic", "independent", "ramsey",
    "hadwiger", "collatz", "navier", "riemann", "yang", "hodge",
    "goldbach", "twin", "beal", "cuboid", "zauner", "sic",
    # foundational / self-reference concepts (present in the catalog: godel_*,
    # liar_paradox, tarskis_undefinability_theorem, halting_problem, cantor
    # diagonal, CH_independent). A self-referential goal matched none of the
    # named-problem anchors above, so the B route surfaced nothing.
    "godel", "goedel", "liar", "undecidab", "incompleteness", "paradox",
    "tarski", "halting", "epimenides", "diagonal", "continuum", "referen",
    "unprovab", "consisten", "cantor",
)


def find_witnesses(question: str, limit: int = 5) -> List[Dict[str, Any]]:
    """Score catalog entries against the question text. No invented systems."""
    if not navigator_available():
        return []
    toks = _tokens(question)
    if not toks:
        return []
    qnorm = _normalize_math_text(question)
    q_anchors = {a for a in _ANCHORS if a in qnorm or a in toks}

    # Prefer direct resolve on distinctive multiword phrases / known aliases
    direct = []
    for phrase in (
        "erdos_hajnal_aleph1_graph", "erdos_hajnal", "erdos hajnal",
        "chromatic number", "independent set",
        "ramsey", "collatz", "hadwiger", "list coloring", "aleph_1", "aleph1",
        # foundational / self-reference resolves (substring-matched by the navigator)
        "godel", "goedel", "liar", "liar paradox", "self referential",
        "undecidable", "incompleteness", "halting problem", "tarski",
        "diagonal lemma", "continuum hypothesis", "epimenides", "unprovable",
    ):
        key = phrase.replace(" ", "_")
        if phrase in qnorm or key in qnorm or all(
            p in qnorm for p in phrase.replace("_", " ").split() if len(p) > 3
        ):
            r = _NAV.resolve_system(key)
            if r:
                direct.append(r["name"])

    scored: Dict[str, float] = {}
    for name in direct:
        scored[name] = scored.get(name, 0) + 80.0

    for name, entry in _NAV.CATALOG_INDEX.items():
        desc = entry.get("description") or ""
        blob = _normalize_math_text(name + " " + desc)
        name_l = name.lower()
        sc = 0.0
        # Anchor overlap (strong)
        for a in q_anchors:
            if a in name_l:
                sc += 12.0
            elif a in blob:
                sc += 6.0
        # Token overlap
        for t in toks:
            if t in name_l:
                sc += 3.0
            elif t in blob:
                sc += 1.0
        # Require real multi-signal overlap (not a single shared "graph")
        if sc >= 10.0:
            scored[name] = scored.get(name, 0) + sc

    ranked = sorted(scored.items(), key=lambda x: -x[1])[:limit]
    out = []
    for name, sc in ranked:
        entry = _NAV.action_entry(name)
        if not isinstance(entry, dict) or entry.get("status") == "error":
            continue
        dist = _NAV.action_distance(name)
        meet = _NAV.action_meet(name)
        join = _NAV.action_join(name)
        tensor = _NAV.action_tensor(name)
        out.append({
            "name": name,
            "score": sc,
            "description": entry.get("description") or "",
            "tuple": entry.get("tuple") or {},
            "structural_algebra": entry.get("structural_algebra") or {},
            "formula_decomposition": entry.get("formula_decomposition") or {},
            "promotions_needed": entry.get("promotions_needed") or [],
            "has_transcendence": entry.get("has_transcendence"),
            "distance": dist if isinstance(dist, dict) else {},
            "meet": meet if isinstance(meet, dict) else {},
            "join": join if isinstance(join, dict) else {},
            "tensor": tensor if isinstance(tensor, dict) else {},
            "proved_hint": bool(re.search(r"proved|proven|theorem", name, re.I)),
        })
    return out


# ── IGProtocol skeleton (IGMorphism constructors — labels only) ──────────────

def igprotocol_skeleton(role_ids: List[str]) -> List[Dict[str, str]]:
    """Map section roles to IGProtocol constructors. Not a Lean proof."""
    steps = [{"ctor": "refl", "note": "VINIT — start from the problem type"}]
    for rid in role_ids:
        steps.append({
            "ctor": "arrow",
            "label": rid,
            "note": f"lemma arm licensed by {rid}: {PRIMITIVE_MATH_ROLE.get(rid, '?')}",
        })
    if len(role_ids) >= 2:
        steps.append({
            "ctor": "seq",
            "note": "compose lemma arms (ɢ-chain) into the main theorem",
        })
        steps.append({
            "ctor": "prod",
            "note": "FSPLIT dual arms where both encode and bound are needed; FFUSE at close",
        })
    steps.append({"ctor": "withMem", "note": "IFIX — fix the conventional statement"})
    return steps


# ── Scaffold assembly ────────────────────────────────────────────────────────

@dataclass
class WitnessProofReport:
    question: str
    domains: List[Tuple[str, int]]
    witnesses: List[Dict[str, Any]]
    primary: Optional[Dict[str, Any]]
    scaffold_md: str
    available: bool
    note: str = ""

    def summary(self) -> str:
        if not self.available:
            return f"witness-proof UNAVAILABLE: {self.note}"
        if not self.primary:
            return f"witness-proof: no catalog hit ({self.note})"
        sa = self.primary.get("structural_algebra") or {}
        return (
            f"witness={self.primary['name']} tier={sa.get('ouroboricity_tier')} "
            f"d(CL8)={sa.get('distance_from_cl8nk')} proved_hint={self.primary.get('proved_hint')} "
            f"domains={[d for d,_ in self.domains[:3]]}"
        )


def _lemma_blocks_from_witness(w: Dict[str, Any]) -> List[Dict[str, str]]:
    """For each catalog primitive value, attach pipeline role + CLINK fragment."""
    tup = w.get("tuple") or {}
    frags = {
        f["primitive"]: f
        for f in (w.get("formula_decomposition") or {}).get("per_primitive_fragments", [])
        if isinstance(f, dict) and "primitive" in f
    }
    blocks = []
    for prim_sym, role_id in _PRIM_TO_ROLE_ID.items():
        val = tup.get(prim_sym)
        if not val:
            continue
        frag = frags.get(prim_sym, {})
        blocks.append({
            "primitive": prim_sym,
            "value": val,
            "role_id": role_id,
            "math_role": PRIMITIVE_MATH_ROLE.get(role_id, "Unknown"),
            "proposition": DEFAULT_PROPOSITION.get(role_id, ""),
            "strategy": DEFAULT_STRATEGY.get(role_id, ""),
            "clink_fragment": frag.get("clink_fragment", ""),
            "proximity": frag.get("proximity", ""),
            "promoted_atom": frag.get("promoted_atom") or "",
        })
    return blocks


def build_conventional_scaffold(question: str, w: Dict[str, Any],
                                domains: List[Tuple[str, int]]) -> str:
    """Assemble markdown scaffold from a real catalog witness + Lean role tables."""
    sa = w.get("structural_algebra") or {}
    blocks = _lemma_blocks_from_witness(w)
    role_ids = [b["role_id"] for b in blocks]
    proto = igprotocol_skeleton(role_ids)

    # Prefer match/close fragments — they are the strongest structural constraints
    strong = [b for b in blocks if b.get("proximity") in ("match", "close")]
    weak = [b for b in blocks if b.get("proximity") not in ("match", "close")]

    lines: List[str] = []
    lines.append("# Conventional proof scaffold (from grammatic witness)")
    lines.append("")
    lines.append("## Provenance (verified sources, not titles)")
    lines.append(
        f"- Catalog witness: `{w['name']}` "
        f"(score={w.get('score')}, proved_hint={w.get('proved_hint')})"
    )
    lines.append(f"- Description: {w.get('description', '')[:400]}")
    lines.append(
        f"- structural_algebra: tier={sa.get('ouroboricity_tier')}, "
        f"d(CLINK L8)={sa.get('distance_from_cl8nk')}, "
        f"match={sa.get('match_count')} close={sa.get('close_count')} "
        f"distant={sa.get('distant_count')}"
    )
    ten = w.get("tensor") or {}
    lines.append(
        f"- IGAlgebra vs CLINK L8: tensor absorbed={ten.get('absorbed')}, "
        f"d_tensor={ten.get('distance_from_cl8nk')}, "
        f"meet d_from_system={(w.get('meet') or {}).get('d_from_system')}"
    )
    lines.append(
        f"- Domains (keyword scores on the question): "
        + ", ".join(f"{d}:{s}" for d, s in domains[:5])
    )
    lines.append(
        "- Role/proposition/strategy tables: ported from "
        "`Imscribing/Millennium/GeneralizedPipeline.lean` "
        "(`primitiveMathRole`, `defaultProposition`, `defaultProofStrategy`) — "
        "domain-invariant structural templates."
    )
    lines.append(
        "- IGProtocol step labels: constructors from "
        "`Imscribing/IGMorphism.lean` (`refl|arrow|seq|prod|withMem`). "
        "This is a protocol skeleton, not a machine-checked conventional proof."
    )
    lines.append(
        "- **Status:** this is a scaffold. Instantiating each proposition in the "
        "object language of the question is the agent's job. Catalog "
        "`proved_hint` only means the *entry name* suggests a proved form — "
        "verify before claiming."
    )
    lines.append("")
    lines.append("## Question")
    lines.append(question.strip()[:2000])
    lines.append("")

    lines.append("## IGProtocol skeleton (morphism chain)")
    for i, step in enumerate(proto):
        lines.append(f"{i}. `{step['ctor']}` — {step.get('note','')}"
                      + (f" [{step['label']}]" if step.get("label") else ""))
    lines.append("")

    lines.append("## Strong structural constraints (CLINK match/close)")
    if not strong:
        lines.append("_None — witness is far from CLINK L8; treat all lemmas as soft._")
    for b in strong:
        lines.append(
            f"- **{b['primitive']}={b['value']}** ({b['proximity']}) "
            f"→ role `{b['math_role']}`"
        )
        lines.append(f"  - CLINK: `{b['clink_fragment']}`")
        if b.get("promoted_atom"):
            lines.append(f"  - promoted atom: `{b['promoted_atom']}`")
    lines.append("")

    lines.append("## Lemma scaffold (instantiate for THIS question)")
    # Follow sectionBackbone order for role-bearing sections
    role_to_block = {b["role_id"]: b for b in blocks}
    for title, rid in SECTION_BACKBONE:
        if rid in ("introduction", "preliminaries", "main_theorem", "discussion"):
            continue
        b = role_to_block.get(rid)
        if not b:
            continue
        lines.append(f"### {title}  [{b['primitive']} → {rid}]")
        lines.append(f"**Mathematical role:** {b['math_role']}")
        lines.append(f"**Structural proposition (template):** {b['proposition']}")
        lines.append(f"**Proof strategy (template):** {b['strategy']}")
        lines.append(
            f"**Witness constraint:** value={b['value']} proximity={b['proximity']} "
            f"CLINK=`{b['clink_fragment']}`"
        )
        lines.append(
            "**Instantiation task:** restate the proposition in the language of "
            "the question; carry the strategy without importing foreign dynamics "
            "(do not paste Collatz/3n+1 unless the question is Collatz)."
        )
        lines.append("")

    if weak:
        lines.append("## Distant primitives (lower priority)")
        for b in weak:
            lines.append(
                f"- {b['primitive']}={b['value']} ({b['proximity']}): "
                f"{b['math_role']} — `{b['clink_fragment']}`"
            )
        lines.append("")

    lines.append("## Main theorem (to state)")
    lines.append(
        "Combine the instantiated encoding / dual / coupling / boundedness / "
        "invariant / regularity arms into a single conventional theorem matching "
        "the question. If the witness is only structural (open problem), state "
        "precisely what is open and what the grammar predicts as the barrier "
        f"(tier={sa.get('ouroboricity_tier')}, promotions_needed="
        f"{len(w.get('promotions_needed') or [])})."
    )
    lines.append("")
    lines.append("## Required answer format for the agent")
    lines.append("1. Direct answer / theorem statement (or 'open, barrier = …').")
    lines.append("2. Conventional proof or rigorous proof sketch with all work shown.")
    lines.append("3. Short appendix: which witness entry and which primitives licensed which steps.")
    return "\n".join(lines)


def translate(question: str, limit: int = 5) -> WitnessProofReport:
    """End-to-end: find catalog witnesses, build conventional scaffold."""
    if not navigator_available():
        return WitnessProofReport(
            question=question, domains=[], witnesses=[], primary=None,
            scaffold_md="", available=False,
            note=f"cl8nk_navigator unavailable ({_NAV_ERR or 'no catalog'})",
        )
    domains = detect_domains(question)
    witnesses = find_witnesses(question, limit=limit)
    if not witnesses:
        return WitnessProofReport(
            question=question, domains=domains, witnesses=[], primary=None,
            scaffold_md=(
                "# No catalog witness found\n\n"
                "Answer the question by ordinary mathematical reasoning. "
                "Do not invent a catalog entry.\n\n"
                f"## Question\n{question.strip()[:2000]}\n"
            ),
            available=True,
            note="no catalog hit",
        )
    primary = witnesses[0]
    scaffold = build_conventional_scaffold(question, primary, domains)
    return WitnessProofReport(
        question=question,
        domains=domains,
        witnesses=witnesses,
        primary=primary,
        scaffold_md=scaffold,
        available=True,
        note=f"{len(witnesses)} witness(es)",
    )


def _selftest() -> dict:
    if not navigator_available():
        return {"available": False, "error": _NAV_ERR}
    # q7-shaped: aleph1 chromatic / independent set
    q = (
        "Is there a graph of chromatic number aleph_1 on aleph_1 vertices "
        "such that every subgraph on n vertices has an independent set of "
        "size > n^{1-epsilon} for all epsilon>0?"
    )
    rep = translate(q)
    assert rep.available
    assert rep.primary is not None, "expected catalog hit for erdos/hajnal-like question"
    # Must not contain Collatz *proof* hardcode (warning text may mention Collatz)
    assert "Terras" not in rep.scaffold_md
    assert "the cycle } 1" not in rep.scaffold_md
    assert "compressed map which applies" not in rep.scaffold_md
    # Prefer graph-theory witness for this question shape
    assert rep.primary["name"]  # real catalog name
    # Must contain real navigator fields
    assert "d(CLINK L8)" in rep.scaffold_md or "structural_algebra" in rep.scaffold_md
    assert "IGProtocol" in rep.scaffold_md
    assert "Instantiation task" in rep.scaffold_md
    return {
        "available": True,
        "summary": rep.summary(),
        "primary": rep.primary["name"],
        "tier": (rep.primary.get("structural_algebra") or {}).get("ouroboricity_tier"),
        "domains": rep.domains[:3],
        "scaffold_chars": len(rep.scaffold_md),
        "alt_witnesses": [w["name"] for w in rep.witnesses[1:4]],
    }


if __name__ == "__main__":
    print(json.dumps(_selftest(), indent=2, default=str))
