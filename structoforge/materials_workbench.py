"""
structoforge/materials_workbench.py — Materials Discovery Workbench
====================================================================

A practical materials discovery platform using MoDoT's click chemistry,
materials algebra, and alchemical bench tools.

CAPABILITIES:
  • Forge materials from monomer sets → spectral invariants (ρ, gap, conductance)
  • Compare materials → structural diffs, compatibility scores
  • Click-sweep monomers against catalog → find fusing partners
  • Dope materials → predict property shifts
  • Crystallize structures → lattice type, stability
  • Screen candidates → rank by application-relevant properties

DIRECT APPLICATIONS:
  • Electronic materials: find high-conductance/low-gap structures
  • Catalytic materials: find high-surface-area, active-site-rich structures
  • Structural materials: find high-modulus, low-strain arrangements
  • Optical materials: find bandgap-engineered semiconductors
  • Quantum materials: find topologically ordered phases

Author: Lando⊗⊙perator
"""

from __future__ import annotations

import json
import math
import re
import subprocess
import sys
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

MODOOT_ROOT = Path(__file__).resolve().parent.parent
ASK_BIN = MODOOT_ROOT / "ask_native" / "target" / "release" / "ask"


@dataclass
class MaterialSheet:
    """Complete material specification from a forge operation."""
    name: str
    monomers: List[str]
    topology: str = ""              # ring / chain / star / comb / bubble
    spectral_radius: float = 0.0    # ρ — largest eigenvalue of adjacency
    spectral_gap: float = 0.0       # HOMO-LUMO gap from spectrum
    conductance_class: str = ""     # metallic / semiconducting / insulating
    stability: float = 0.0          # 0–1 stability score
    strain: float = 0.0             # ring strain
    bond_count: int = 0
    node_count: int = 0
    circuit_rank: int = 0           # β = E − V + C
    
    # Structural type
    tier: str = ""
    c_score: float = 0.0
    
    # Raw output
    forge_output: str = ""
    
    # Catalog links
    analog_names: List[str] = field(default_factory=list)
    
    @property
    def is_metal(self) -> bool:
        return self.conductance_class == "metallic"
    
    @property
    def is_semiconductor(self) -> bool:
        return self.conductance_class == "semiconducting"
    
    @property
    def quality_score(self) -> float:
        """Composite quality metric: higher is better."""
        score = 0.0
        if self.is_metal:
            score += self.spectral_radius * 0.3
        if self.spectral_gap > 0:
            score += min(self.spectral_gap, 2.0) * 0.4
        score += self.stability * 0.3
        return score


def _run_ask(args: List[str]) -> str:
    """Run the MoDoT ask binary."""
    if not ASK_BIN.exists():
        return f"ERROR: binary not found at {ASK_BIN}"
    cmd = [str(ASK_BIN)] + args
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=60)
    return result.stdout


def _parse_forge_output(output: str) -> Dict[str, Any]:
    """Parse structural invariants from forge output."""
    result = {}
    
    # Spectral radius ρ
    m = re.search(r'ρ[=:]\s*([0-9.]+)', output)
    if m: result['spectral_radius'] = float(m.group(1))
    
    # Spectral gap
    m = re.search(r'gap[=:]\s*([0-9.]+)', output)
    if m: result['spectral_gap'] = float(m.group(1))
    
    # Conductance class
    if re.search(r'metallic', output, re.I):
        result['conductance_class'] = "metallic"
    elif re.search(r'semiconduct', output, re.I):
        result['conductance_class'] = "semiconducting"
    elif re.search(r'insulat', output, re.I):
        result['conductance_class'] = "insulating"
    
    # Topology
    if re.search(r'ring', output, re.I):
        result['topology'] = "ring"
    elif re.search(r'chain', output, re.I):
        result['topology'] = "chain"
    elif re.search(r'star', output, re.I):
        result['topology'] = "star"
    elif re.search(r'comb', output, re.I):
        result['topology'] = "comb"
    elif re.search(r'bubble', output, re.I):
        result['topology'] = "bubble"
    
    # Node/bond counts
    m = re.search(r'(\d+)\s*nodes?', output, re.I)
    if m: result['node_count'] = int(m.group(1))
    m = re.search(r'(\d+)\s*bonds?', output, re.I)
    if m: result['bond_count'] = int(m.group(1))
    m = re.search(r'β[=:]\s*(\d+)', output)
    if m: result['circuit_rank'] = int(m.group(1))
    
    # Tier
    m = re.search(r'O[_]?(\d+|∞)', output)
    if m: result['tier'] = f"O_{m.group(1)}"
    
    return result


def forge_material(name: str, *monomers: str, register: bool = False) -> MaterialSheet:
    """
    Forge a material from a set of monomers.
    
    Uses MoDoT's --forge to find the best-ringing order and compute
    spectral invariants.
    
    Args:
        name: Name for the material
        monomers: List of monomer names (from MoDoT catalog)
        register: Whether to register in the material registry
    
    Returns:
        MaterialSheet with all computed properties
    """
    args = ["--forge"] + list(monomers)
    if register:
        args.extend(["--register", name])
    
    output = _run_ask(args)
    parsed = _parse_forge_output(output)
    
    sheet = MaterialSheet(
        name=name,
        monomers=list(monomers),
        forge_output=output,
        **parsed,
    )
    
    return sheet


def compare_materials(a_name: str, b_name: str, 
                      a_monomers: List[str], b_monomers: List[str]) -> Dict[str, Any]:
    """
    Compare two materials and produce a structural diff.
    
    Uses MoDoT's --compare to forge both and compute the differences
    in spectral radius, conductance, stability, and topology.
    """
    args = ["--compare"] + a_monomers + ["vs"] + b_monomers
    output = _run_ask(args)
    
    result = {
        "a": a_name,
        "b": b_name,
        "a_monomers": a_monomers,
        "b_monomers": b_monomers,
        "output": output,
    }
    
    # Parse spectral radius differences
    radii = re.findall(r'ρ[=:]\s*([0-9.]+)', output)
    if len(radii) >= 2:
        result["rho_a"] = float(radii[0])
        result["rho_b"] = float(radii[1])
        result["rho_delta"] = float(radii[0]) - float(radii[1])
    
    # Which is more conductive?
    if re.search(r'metallic.*insulat|insulat.*metallic', output, re.I):
        result["conductivity_winner"] = "first" if "metallic" in output.split("vs")[0] else "second"
    
    return result


def click_sweep(monomer: str, top_n: int = 10) -> List[Dict[str, Any]]:
    """
    Sweep a monomer against the MoDoT catalog to find fusing partners.
    
    Returns ranked list of click-compatible monomers.
    """
    args = ["--click", monomer, "--top", str(top_n)]
    output = _run_ask(args)
    
    results = []
    for line in output.split("\n"):
        if "→" in line or any(name in line.lower() for name in [monomer.lower()]):
            results.append({"raw": line})
    
    return results


def dope_material(base_monomers: List[str], dopant: str) -> Dict[str, Any]:
    """
    Dope a base material with a dopant and measure property shifts.
    
    Uses MoDoT's --dope to forge the base ring, mix in the dopant,
    and report the shift in ρ and conductance.
    """
    args = ["--dope"] + base_monomers + ["with", dopant]
    output = _run_ask(args)
    
    result = {"output": output, "base_monomers": base_monomers, "dopant": dopant}
    
    # Parse ρ before/after
    radii = re.findall(r'ρ[=:]\s*([0-9.]+)', output)
    if len(radii) >= 2:
        result["rho_base"] = float(radii[0])
        result["rho_doped"] = float(radii[1])
        result["rho_shift"] = float(radii[1]) - float(radii[0])
    
    return result


def screen_candidates(target_property: str, 
                      candidates: List[Tuple[str, List[str]]]) -> List[Dict[str, Any]]:
    """
    Screen material candidates by target property.
    
    Forges each candidate and ranks them by the target property.
    
    Args:
        target_property: One of "conductivity", "bandgap", "stability", "quality"
        candidates: List of (name, [monomer1, monomer2, ...])
    
    Returns:
        Ranked list of candidate sheets
    """
    sheets = []
    for name, monomers in candidates:
        sheet = forge_material(name, *monomers)
        sheets.append(sheet)
    
    # Sort by target property
    if target_property == "conductivity":
        sheets.sort(key=lambda s: s.spectral_radius, reverse=True)
    elif target_property == "bandgap":
        sheets.sort(key=lambda s: s.spectral_gap, reverse=True)
    elif target_property == "stability":
        sheets.sort(key=lambda s: s.stability, reverse=True)
    elif target_property == "quality":
        sheets.sort(key=lambda s: s.quality_score, reverse=True)
    
    return [asdict(s) for s in sheets]


def search_crystal(constraints: Dict[str, str]) -> List[Dict[str, Any]]:
    """
    Search the crystal of types for structures matching constraints.
    
    Args:
        constraints: Dict of primitive values, e.g. {"Ph": "⊙", "W": "𐑭"}
    
    Returns:
        List of matching structural types
    """
    try:
        from imscribe import imscribe
        result = imscribe("crystal_navigate", {
            "limit": 20,
            **constraints
        })
        return result.get("results", [])
    except Exception as e:
        return [{"error": str(e)}]


class MaterialsWorkbench:
    """Interactive materials discovery workbench."""
    
    def __init__(self):
        self._registry: Dict[str, MaterialSheet] = {}
    
    def forge(self, name: str, *monomers: str, register: bool = False) -> MaterialSheet:
        """Forge a material and optionally register it."""
        sheet = forge_material(name, *monomers, register=register)
        self._registry[name] = sheet
        return sheet
    
    def compare(self, a: str, b: str) -> Dict[str, Any]:
        """Compare two registered materials."""
        if a not in self._registry:
            return {"error": f"Material '{a}' not registered. Forge it first."}
        if b not in self._registry:
            return {"error": f"Material '{b}' not registered. Forge it first."}
        
        return compare_materials(
            a, b,
            self._registry[a].monomers,
            self._registry[b].monomers,
        )
    
    def list_materials(self) -> List[Dict[str, Any]]:
        """List all registered materials with key properties."""
        return [
            {"name": s.name, "topology": s.topology, 
             "rho": s.spectral_radius, "gap": s.spectral_gap,
             "conductance": s.conductance_class, "quality": round(s.quality_score, 3)}
            for s in self._registry.values()
        ]
    
    def report(self, name: str) -> str:
        """Generate a human-readable material report."""
        if name not in self._registry:
            return f"Material '{name}' not found."
        
        s = self._registry[name]
        lines = [
            f"═══ Material: {s.name} ═══",
            f"  Monomers:      {', '.join(s.monomers)}",
            f"  Topology:      {s.topology}",
            f"  Nodes:         {s.node_count}",
            f"  Bonds:         {s.bond_count}",
            f"  Circuit rank:  {s.circuit_rank}",
            f"  Spectral ρ:    {s.spectral_radius:.4f}",
            f"  Spectral gap:  {s.spectral_gap:.4f}",
            f"  Conductance:   {s.conductance_class}",
            f"  Stability:     {s.stability:.3f}",
            f"  Tier:          {s.tier}",
            f"  C-score:       {s.c_score:.4f}",
            f"  Quality:       {s.quality_score:.3f}",
        ]
        return "\n".join(lines)


# ── Quick material dictionary ──────────────────────────────────────────────
COMMON_MONOMERS = {
    # Electronic materials
    "graphene":        "graphene",
    "silicon":         "silicon",
    "gallium_arsenide": "gallium_arsenide",
    "carbon_nanotube": "carbon_nanotube",
    
    # Catalytic materials
    "platinum":        "platinum",
    "palladium":       "palladium",
    "zeolite":         "zeolite",
    "mof":             "metal_organic_framework",
    
    # Quantum materials
    "topological_insulator": "topological_insulator",
    "superconductor":  "superconductor",
    "magnet":          "magnet",
    
    # Structural materials
    "diamond":         "diamond",
    "kevlar":          "kevlar",
    "carbon_fiber":    "carbon_fiber",
    
    # MoDoT natures (structural primitives)
    "monad":           "monad",
    "void":            "void",
    "hopf":            "hopf_fibration",
    "topos":           "topos",
    "tensor":          "tensor_network",
    "category":        "category",
    "operad":          "operad",
    "yoneda":          "yoneda_lemma",
    "galois":          "galois_theory",
    "sheaf":            "sheaf_theory",
    "presheaf":         "presheaf",
    "linearlogic":      "linear_logic",
    "daggercompact":    "dagger_compact",
    "homotopy":         "homotopy_type_theory",
    "kanextension":     "kan_extension",
}


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description="Materials Discovery Workbench")
    parser.add_argument("action", choices=["forge", "compare", "sweep", "dope", "list", "screen", "crystal"])
    parser.add_argument("args", nargs="*", help="Action-specific arguments")
    parser.add_argument("--register", "-r", action="store_true")
    args = parser.parse_args()
    
    wb = MaterialsWorkbench()
    
    if args.action == "forge":
        if len(args.args) < 2:
            print("Usage: forge NAME MONOMER1 [MONOMER2 ...]")
        else:
            sheet = wb.forge(args.args[0], *args.args[1:], register=args.register)
            print(wb.report(sheet.name))
    
    elif args.action == "list":
        for m in wb.list_materials():
            print(f"  {m['name']:30s} ρ={m['rho']:.4f} gap={m['gap']:.4f} {m['conductance']:15s} Q={m['quality']:.3f}")
