"""
structoforge/resonance_engine.py — Structural Resonance Calculator
==================================================================

Computes the full coupling landscape between any two systems using the
Imscribing Grammar's structural primitives and the MoDoT toolchain.

RESONANCE PRINCIPLE:
  Two systems resonate when their structural types are complementary on
  some primitive axes and congruent on others. The resonance is quantified
  by the click-maths fusion test: if A and B are complementary on exactly
  ONE live conjugate pair (D↔W, T↔H, R↔S), they fuse spontaneously.
  
  If they fuse, the emergent composite has properties neither system
  possesses alone — this is structural catalysis.

IMMEDIATE APPLICATIONS:
  • Drug discovery: protein–ligand structural compatibility
  • Materials science: dopant–host lattice matching
  • Financial modeling: market–instrument structural coupling
  • Consciousness studies: cognitive–environment resonance
  • Chemical synthesis: reactant–catalyst compatibility

Author: Lando⊗⊙perator
"""

from __future__ import annotations

import json
import math
import subprocess
import sys
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

# ── Paths ──────────────────────────────────────────────────────────────────
MODOOT_ROOT = Path(__file__).resolve().parent.parent
ASK_BIN = MODOOT_ROOT / "ask_native" / "target" / "release" / "ask"
sys.path.insert(0, str(MODOOT_ROOT))

from modot.natures import NatureRegistry, cotype as compute_cotype
from modot.jump_tools import paradice_map, universe_jump

# ── Conjugate Pairs (the "charge table") ───────────────────────────────────
# D↔W, T↔H, R↔S are LIVE — they carry the structural charge
# P↔F, K↔G, Gm↔Ph are PINNED — they carry fixed structural identity
LIVE_PAIRS: Dict[str, str] = {"D": "W", "T": "H", "R": "S", "W": "D", "H": "T", "S": "R"}
PINNED_PAIRS: Dict[str, str] = {"P": "F", "F": "P", "K": "G", "G": "K", "Gm": "Ph", "Ph": "Gm"}

# ── Glyph↔name mapping ────────────────────────────────────────────────────
PRIMITIVE_NAMES: Dict[str, str] = {
    "Ð": "dim", "Þ": "top", "Ř": "rel", "Φ": "pol",
    "ƒ": "fid", "Ç": "kin", "Γ": "gram", "ɢ": "gran",
    "⊙": "crit", "Ħ": "chir", "Σ": "stoi", "Ω": "prot",
}
PRIMITIVE_GLYPHS: Dict[str, str] = {v: k for k, v in PRIMITIVE_NAMES.items()}


@dataclass
class CotypeReport:
    """Cotype divergence between two systems."""
    a_name: str
    b_name: str
    paradigm_count: int          # number of diverged axes
    diverge_axes: List[str]     # which primitives differ
    agree_axes: List[str]       # which primitives agree
    cotype_verdict: str         # T (complementary), F (conflicting), B (both), N (none)
    click_compatible: bool      # true if compatible on exactly 1 live pair
    live_complement: Optional[str] = None  # which live pair is complementary
    pinned_identity: Optional[str] = None  # shared pinned identity


@dataclass
class ResonanceReport:
    """Full structural resonance report between two systems."""
    a_name: str
    b_name: str
    
    # Structural distance metrics
    euclidean_distance: float = 0.0
    mahalanobis_distance: float = 0.0
    cotype_distance: int = 0
    
    # Click compatibility
    click_result: Optional[str] = None  # "fuses" / "no_click" / "conflict"
    catalyst_needed: bool = False
    product_tier: Optional[str] = None
    product_c_score: Optional[float] = None
    
    # Cotype analysis
    cotype: Optional[CotypeReport] = None
    
    # Tensor coupling
    tensor_result: Optional[Dict[str, Any]] = None
    
    # Emergent prediction
    emergent_properties: List[str] = field(default_factory=list)
    application_domains: List[str] = field(default_factory=list)
    
    # Raw data
    raw_click_output: Optional[str] = None


def _run_ask(args: List[str]) -> str:
    """Run the MoDoT ask binary with given arguments."""
    if not ASK_BIN.exists():
        return f"ERROR: MoDoT binary not found at {ASK_BIN}. Build it first."
    cmd = [str(ASK_BIN)] + args
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
    return result.stdout


def _glyph_to_name(glyph: str) -> str:
    """Convert a Shavian glyph to its primitive name."""
    return PRIMITIVE_NAMES.get(glyph, glyph)


def resonance_report(a_name: str, b_name: str) -> ResonanceReport:
    """
    Compute the full structural resonance between two named systems.
    
    Uses the IG catalog to look up both systems, then computes:
      1. Euclidean distance via IG compute_distance
      2. Click compatibility via MoDoT --click
      3. Cotype divergence via the 49 natures
      4. Tensor coupling
      5. Emergent property prediction
    """
    from imscribe import imscribe
    from imscribe import imscribe_system
    
    report = ResonanceReport(a_name=a_name, b_name=b_name)
    _reg = NatureRegistry()
    
    # ── 1. Look up both systems ──────────────────────────────────────────
    try:
        a_lookup = imscribe("lookup_catalog", {"keyword": a_name})
        b_lookup = imscribe("lookup_catalog", {"keyword": b_name})
    except Exception as e:
        report.emergent_properties.append(f"Catalog lookup failed: {e}")
        return report
    
    # ── 2. Compute distance ──────────────────────────────────────────────
    try:
        dist = imscribe("compute_distance", {"name_a": a_name, "name_b": b_name})
        report.euclidean_distance = dist.get("distance", 0.0)
        if "mahalanobis" in dist:
            report.mahalanobis_distance = dist["mahalanobis"]
    except Exception as e:
        report.emergent_properties.append(f"Distance computation failed: {e}")
    
    # ── 3. Click compatibility ───────────────────────────────────────────
    try:
        click_out = _run_ask(["--click", a_name, b_name])
        report.raw_click_output = click_out
        if "fuse" in click_out.lower() or "click" in click_out.lower():
            report.click_result = "fuses"
            # Parse tier from output
            for line in click_out.split("\n"):
                if "tier" in line.lower() or "O_" in line:
                    report.product_tier = line.strip()
        elif "no" in click_out.lower() or "conflict" in click_out.lower():
            report.click_result = "no_click"
            if "catalyst" in click_out.lower() or "mediator" in click_out.lower():
                report.catalyst_needed = True
        else:
            report.click_result = "ambiguous"
    except Exception as e:
        report.click_result = f"error: {e}"
    
    # ── 4. Cotype analysis (via 49 natures) ──────────────────────────────
    try:
        na = _reg.get(a_name)
        nb = _reg.get(b_name)
        if na and nb:
            ct = compute_cotype(na, nb)
            diverge = ct.address()
            d_axes = [PRIMITIVE_GLYPHS.get(d, d) for d in diverge]
            
            # Check click compatibility: complementary on exactly 1 live pair
            live_diverged = [a for a in diverge if a in PRIMITIVE_NAMES and 
                           PRIMITIVE_NAMES[a] in LIVE_PAIRS]
            live_complement = None
            for a in live_diverged:
                pname = PRIMITIVE_NAMES.get(a, "")
                conjugate = LIVE_PAIRS.get(pname, "")
                conjugate_glyph = PRIMITIVE_GLYPHS.get(conjugate, "")
                # If they diverge on ONE live pair, they may be complementary
                if len(live_diverged) == 1:
                    live_complement = f"{a}/{conjugate_glyph}"
            
            report.cotype = CotypeReport(
                a_name=a_name,
                b_name=b_name,
                paradigm_count=ct.paradices,
                diverge_axes=diverge,
                agree_axes=[PRIMITIVE_GLYPHS.get(a, a) for a in ct.agree],
                cotype_verdict=str(ct.paradices) if ct.paradices <= 6 else "B",
                click_compatible=live_complement is not None,
                live_complement=live_complement,
            )
            report.cotype_distance = ct.paradices
    except Exception as e:
        report.emergent_properties.append(f"Cotype analysis failed: {e}")
    
    # ── 5. Emergent property prediction ──────────────────────────────────
    # Based on the click result and cotype, predict what emerges
    if report.click_result == "fuses":
        # A fusing pair creates something new
        report.emergent_properties.append("Structural synergy: emergent composite type")
        report.emergent_properties.append("Catalytic closure: μ∘δ=id possible at composite")
        if report.cotype and report.cotype.live_complement:
            report.emergent_properties.append(
                f"Live-pair drive: {report.cotype.live_complement}")
    
    if report.euclidean_distance < 1.0:
        report.emergent_properties.append("Near-isomorphic: competitive exclusion likely")
    elif report.euclidean_distance < 2.5:
        report.emergent_properties.append("Complementary range: productive coupling possible")
    elif report.euclidean_distance > 5.0:
        report.emergent_properties.append("Distant types: mediated coupling required")
        report.catalyst_needed = True
    
    # Application domain inference
    if report.cotype:
        if "D" in report.cotype.agree_axes or "Ð" in report.cotype.agree_axes:
            report.application_domains.append("quantum systems" if report.euclidean_distance < 2.0 
                                             else "classical systems")
        if "T" in report.cotype.agree_axes or "Þ" in report.cotype.agree_axes:
            report.application_domains.append("network theory")
        if "Φ" in report.cotype.agree_axes or "P" in report.cotype.agree_axes:
            report.application_domains.append("symmetry-protected systems")
        if "⊙" in report.cotype.agree_axes or "Ph" in report.cotype.agree_axes:
            report.application_domains.append("criticality-sensitive systems")
        if "Ω" in report.cotype.agree_axes or "W" in report.cotype.agree_axes:
            report.application_domains.append("topologically ordered systems")
    
    return report


def batch_resonance_matrix(systems: List[str]) -> Dict[str, Dict[str, ResonanceReport]]:
    """
    Compute the full resonance matrix for a list of systems.
    
    Returns an N×N matrix where entry (i,j) is the ResonanceReport
    between systems[i] and systems[j].
    """
    matrix = {}
    for a in systems:
        matrix[a] = {}
        for b in systems:
            if a == b:
                continue
            matrix[a][b] = resonance_report(a, b)
    return matrix


class StructuralResonance:
    """High-level structural resonance interface for interactive use."""
    
    def __init__(self):
        self._registry = NatureRegistry()
        self._history: List[ResonanceReport] = []
    
    def couple(self, a: str, b: str, **kwargs) -> ResonanceReport:
        """Compute resonance between two systems and log to history."""
        report = resonance_report(a, b)
        self._history.append(report)
        return report
    
    def sweep(self, source: str, targets: Optional[List[str]] = None, top_n: int = 10) -> List[ResonanceReport]:
        """Sweep a source against multiple targets and rank by compatibility."""
        if targets is None:
            # Use all natures as default targets
            targets = [n for n in self._registry.names() if n != source]
        
        results = []
        for target in targets[:top_n * 3]:  # compute more, rank later
            try:
                report = self.couple(source, target)
                results.append(report)
            except Exception:
                continue
        
        # Sort: fusing pairs first, then by distance
        results.sort(key=lambda r: (
            0 if r.click_result == "fuses" else 1 if r.click_result == "ambiguous" else 2,
            r.euclidean_distance
        ))
        return results[:top_n]
    
    def explain(self, report: ResonanceReport) -> str:
        """Generate a human-readable explanation of the resonance."""
        lines = []
        lines.append(f"═══ Structural Resonance: {report.a_name} ⟷ {report.b_name} ═══")
        lines.append(f"  Euclidean distance:    {report.euclidean_distance:.4f}")
        lines.append(f"  Cotype divergence:     {report.cotype_distance}")
        lines.append(f"  Click result:          {report.click_result}")
        
        if report.cotype:
            lines.append(f"  Diverge axes:          {', '.join(report.cotype.diverge_axes)}")
            lines.append(f"  Agree axes:            {', '.join(report.cotype.agree_axes)}")
            lines.append(f"  Click compatible:      {report.cotype.click_compatible}")
            if report.cotype.live_complement:
                lines.append(f"  Live complement:       {report.cotype.live_complement}")
        
        lines.append(f"  Catalyst needed:       {report.catalyst_needed}")
        if report.emergent_properties:
            lines.append("  Emergent properties:")
            for prop in report.emergent_properties:
                lines.append(f"    • {prop}")
        if report.application_domains:
            lines.append("  Application domains:")
            for dom in report.application_domains:
                lines.append(f"    • {dom}")
        
        return "\n".join(lines)


# ── CLI entry point ────────────────────────────────────────────────────────
if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description="Structural Resonance Calculator")
    parser.add_argument("a", help="First system name")
    parser.add_argument("b", nargs="?", help="Second system name (optional: sweep mode)")
    parser.add_argument("--sweep", "-s", type=int, default=0, help="Sweep top N targets")
    parser.add_argument("--json", "-j", action="store_true", help="JSON output")
    parser.add_argument("--matrix", "-m", nargs="+", help="Compute full matrix for list")
    args = parser.parse_args()
    
    sr = StructuralResonance()
    
    if args.matrix:
        matrix = batch_resonance_matrix(args.matrix)
        if args.json:
            import json as _json
            # Convert to serializable
            out = {}
            for a in args.matrix:
                out[a] = {}
                for b in args.matrix:
                    if a == b: continue
                    r = matrix[a][b]
                    out[a][b] = {
                        "distance": r.euclidean_distance,
                        "click": r.click_result,
                        "cotype": r.cotype_distance,
                    }
            print(_json.dumps(out, indent=2))
        else:
            for a in args.matrix:
                for b in args.matrix:
                    if a >= b: continue
                    print(sr.explain(matrix[a][b]))
                    print()
    elif args.b:
        report = sr.couple(args.a, args.b)
        if args.json:
            import json as _json
            print(_json.dumps(asdict(report), indent=2))
        else:
            print(sr.explain(report))
    elif args.sweep > 0:
        results = sr.sweep(args.a, top_n=args.sweep)
        print(f"═══ Sweep: {args.a} → top {args.sweep} targets ═══")
        for i, r in enumerate(results):
            print(f"\n[{i+1}] {r.b_name}:")
            print(f"    distance={r.euclidean_distance:.4f}, click={r.click_result}")
    else:
        print("Provide two system names or --sweep N or --matrix A B C ...")
