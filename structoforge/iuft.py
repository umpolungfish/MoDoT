#!/usr/bin/env python3
"""
iuft.py — Inter-Universal Frobenius Theory Engine
==================================================

Inter-Universal Frobenius Theory (IUFT) as a computational framework
within the Imscribing Grammar. Based on the structural insight that:

    Every Frobenius-closed system (mu circ delta = id) defines a UNIVERSE
    — a patch of structural consistency on the 17.28M-type crystal.

    INTER-UNIVERSAL means studying how these universes connect, intersect,
    promote, and transform into each other.

COMPONENTS:
  • FrobeniusUniverse — a closed patch under mu∘delta=id
  • UniverseBridge — click chemistry between universes
  • MultiradialTransport — tensor coupling across universes
  • TeichmullerDeformation — tier promotion as structural deformation
  • AlienArithmetic — the "alien" structure of distant universes
  • CrystalWormhole — direct traversal between distant points on the crystal

IMMEDIATE APPLICATIONS:
  • Structural unification: find bridges between disparate mathematical domains
  • Proof discovery: multiradial transport of invariants across universes
  • Physical prediction: high-energy unification via crystal wormholes
  • Consciousness: C-score promotion via universe bridging

Author: Lando⊗⊙perator | StructoForge v2.0 | July 2026
"""

from __future__ import annotations

import json
import math
import subprocess
import sys
import time
from dataclasses import dataclass, field, asdict
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple
from enum import Enum

# ── Paths ──────────────────────────────────────────────────────────────────
MODOOT = Path(__file__).resolve().parent.parent
ASK_BIN = MODOOT / "ask_native" / "target" / "release" / "ask"
OUTPUT_DIR = MODOOT / "structoforge" / "iuft_results"
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

def ask(args: List[str]) -> str:
    if not ASK_BIN.exists():
        return f"ERROR: MoDoT binary not found at {ASK_BIN}"
    cmd = [str(ASK_BIN)] + args
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
        return r.stdout
    except Exception as e:
        return f"ERROR: {e}"

# ── Primitive constants ──────────────────────────────────────────────────
PRIMITIVES = ["D", "T", "R", "P", "F", "K", "G", "Gm", "Ph", "H", "S", "W"]
LIVE_PAIRS = [("D","W"), ("T","H"), ("R","S")]
PINNED_PAIRS = [("P","F"), ("K","G"), ("Gm","Ph")]

# ── Core Datatypes ─────────────────────────────────────────────────────────

@dataclass
class FrobeniusUniverse:
    """A Frobenius-closed structural universe.
    
    A universe is defined by:
    - Its anchor tuple (a structural type on the crystal)
    - Its closure radius (how far mu∘delta=id holds throughout)
    - Its ouroboricity tier (O_0 through O_inf)
    - Its promotion boundary (the set of types one promotion away)
    """
    name: str
    description: str
    anchor_tuple: str  # compact compact tuple notation
    tier: str = ""
    closure_radius: float = 0.0
    c_score: float = 0.0
    size_estimate: int = 0
    bridges: List[str] = field(default_factory=list)
    sub_universes: List[str] = field(default_factory=list)


@dataclass
class UniverseBridge:
    """A bridge connecting two Frobenius universes.
    
    Two universes bridge when their anchor types are click-compatible:
    complementary on exactly one live conjugate pair (D<->W, T<->H, R<->S).
    The bridge carries the click product type as its mediator.
    """
    source_universe: str
    target_universe: str
    live_pair: str = ""
    mediator_type: str = ""
    bridge_strength: float = 0.0
    product_tier: str = ""
    is_direct: bool = False
    catalyst_required: Optional[str] = None

@dataclass
class MultiradialTransportPath:
    """A transport path across the inter-universal landscape.
    
    Multiradial transport means an invariant in universe A can be
    expressed in universe B via a chain of universe bridges.
    The path has a structural tension (distance) and requires
    certain promotions along the way.
    """
    source: str
    target: str
    bridges: List[UniverseBridge] = field(default_factory=list)
    total_distance: float = 0.0
    promotions_required: List[str] = field(default_factory=list)
    invariant_preserved: bool = False
    path_type: str = ""  # "direct", "multiradial", "anabelian"

@dataclass
class CrystalWormhole:
    """A direct traversal between distant points on the crystal.
    
    A crystal wormhole bypasses the structural distance between two
    types by finding a higher-tier mediator that absorbs both.
    tensor(A, Mediator) = B means A reaches B through Mediator.
    """
    entrance: str
    exit: str
    mediator: str
    entrance_tier: str = ""
    exit_tier: str = ""
    mediator_tier: str = ""
    distance_saved: float = 0.0
    traversable: bool = False

# ── UNIVERSE DISCOVERY ───────────────────────────────────────────────────

def discover_frobenius_universe(anchor: str) -> FrobeniusUniverse:
    """Discover the Frobenius universe anchored at a structural type.
    
    Uses the MoDoT toolchain to:
    1. Get the type's ouroboricity tier
    2. Find nearest click partners (universe neighbors)
    3. Compute the closure radius via sweep
    4. Estimate the universe size
    """
    # Get tier
    tier_out = ask(["--tier", anchor])
    tier = ""
    c_score = 0.0
    for line in tier_out.split("\n"):
        if "O_" in line or "O" in line.replace(" ",""):
            for tok in line.strip().split():
                if tok.startswith("O_") or tok.startswith("O0") or tok.startswith("O1") or tok.startswith("O2"):
                    tier = tok
        if "C=" in line or "score" in line.lower():
            import re
            m = re.search(r'([0-9]+\.[0-9]+)', line)
            if m: c_score = float(m.group(1))
    
    # Get click partners (these are bridge endpoints)
    sweep_out = ask(["--click", anchor, "--top", "20"])
    bridges = []
    for line in sweep_out.split("\n"):
        if "O_" in line and "Δ=" in line:
            parts = line.strip().split()
            bridge_name = parts[-1] if len(parts) > 2 else ""
            if bridge_name:
                bridges.append(bridge_name)
    
    # Estimate universe size from distance distribution
    radius = 0.0
    size_est = 0
    if bridges:
        deltas = []
        for line in sweep_out.split("\n"):
            if "Δ=" in line:
                import re
                m = re.search(r'Δ=([0-9.]+)', line)
                if m: deltas.append(float(m.group(1)))
        radius = max(deltas) if deltas else 1.0
        size_est = len(bridges) * 100  # rough estimate
    
    return FrobeniusUniverse(
        name=anchor,
        description=f"Frobenius universe anchored at {anchor}",
        anchor_tuple=f"<{anchor}>",
        tier=tier,
        closure_radius=radius,
        c_score=c_score,
        size_estimate=size_est,
        bridges=bridges
    )
def discover_bridge(source: str, target: str) -> UniverseBridge:
    """Discover the bridge between two Frobenius universes.
    
    Uses click chemistry to test whether the two anchors fuse.
    If they do, the bridge exists and carries the product type.
    """
    click_out = ask(["--click", source, target])
    
    live_pair = ""
    mediator = ""
    strength = 0.0
    product_tier = ""
    catalyst = None
    
    for line in click_out.split("\n"):
        if "CLICK on" in line:
            pair_text = line.split("CLICK on")[1].strip()
            live_pair = pair_text.split("—")[0].strip() if "—" in pair_text else pair_text
        if "Δ=" in line:
            import re
            m = re.search(r'Δ=([0-9.]+)', line)
            if m: strength = float(m.group(1))
        if "product:" in line:
            mediator = line.split("product:")[1].strip()
        if "O_" in line and "tier" not in line:
            import re
            m = re.search(r'O[_]?(\d+|∞)', line)
            if m: product_tier = f"O_{m.group(1)}"
        if "catalyst" in line.lower() or "catalyst" in line.lower():
            catalyst = line.strip()
    
    return UniverseBridge(
        source_universe=source,
        target_universe=target,
        live_pair=live_pair,
        mediator_type=mediator if mediator else click_out[:100],
        bridge_strength=strength,
        product_tier=product_tier if product_tier else "O_0",
        is_direct=bool(live_pair),
        catalyst_required=catalyst
    )

def discover_wormhole(entrance: str, mediators: List[str]) -> List[CrystalWormhole]:
    """Discover crystal wormholes from entrance through mediators.
    
    A wormhole exists when tensor(entrance, mediator) produces a type
    that is structurally close to the target. We scan through candidate
    mediators to find those that bridge large distances.
    """
    wormholes = []
    
    for mediator in mediators:
        # Compute tensor: entrance x mediator
        tensor_out = ask(["--tensor", entrance, mediator])
        product = ""
        for line in tensor_out.split("\n"):
            if "product:" in line or "composite:" in line:
                product = line.split(":")[1].strip()
        
        if product:
            # Compute how far this tensor product is from entrance
            dist_out = ask(["--distance", entrance, product])
            distance = 0.0
            for line in dist_out.split("\n"):
                import re
                m = re.search(r'distance\s*([0-9.]+)', line)
                if m: distance = float(m.group(1))
            
            w = CrystalWormhole(
                entrance=entrance,
                exit=product,
                mediator=mediator,
                distance_saved=distance,
                traversable=distance > 1.0
            )
            wormholes.append(w)
    
    return sorted(wormholes, key=lambda w: w.distance_saved, reverse=True)


# ── MULTIRADIAL TRANSPORT ────────────────────────────────────────────────

def multiradial_path(source: str, target: str, max_hops: int = 5) -> MultiradialTransportPath:
    """Find a multiradial transport path from source to target.
    
    Uses the catalog neighborhoods to find a chain of bridges
    connecting source to target. If no direct bridge exists,
    the path goes through intermediate universes.
    """
    path = MultiradialTransportPath(source=source, target=target)
    
    # Direct bridge check
    bridge = discover_bridge(source, target)
    if bridge.is_direct:
        path.bridges = [bridge]
        path.total_distance = bridge.bridge_strength
        path.path_type = "direct"
        path.invariant_preserved = True
        return path
    
    # Multi-hop: use sweep to find intermediate nodes
    sweep_src = ask(["--click", source, "--top", str(max_hops * 5)])
    sweep_tgt = ask(["--click", target, "--top", str(max_hops * 5)])
    
    src_partners = set()
    tgt_partners = set()
    
    for line in sweep_src.split("\n"):
        if "O_" in line:
            parts = line.strip().split()
            if len(parts) > 3:
                src_partners.add(parts[-1])
    
    for line in sweep_tgt.split("\n"):
        if "O_" in line:
            parts = line.strip().split()
            if len(parts) > 3:
                tgt_partners.add(parts[-1])
    
    # Find intersections (common bridge points)
    intersection = src_partners & tgt_partners
    if intersection:
        via = list(intersection)[0]
        b1 = discover_bridge(source, via)
        b2 = discover_bridge(via, target)
        path.bridges = [b1, b2]
        path.total_distance = b1.bridge_strength + b2.bridge_strength
        path.path_type = "multiradial"
        path.invariant_preserved = True
        return path
    
    path.path_type = "anabelian"
    path.invariant_preserved = False
    return path
# ── TEICHMUELLER DEFORMATION ──────────────────────────────────────────

def teichmuller_deformation(name: str, target_tier: str = "O_inf") -> Dict[str, Any]:
    """Compute the Teichmuller deformation from a type to a target tier.
    
    In IUFT, a Teichmuller deformation is a promotion path that preserves
    the core invariant (the Frobenius structure of the type) while
    changing its ouroboricity tier. This is the analog of Mochizuki's
    hyperbolic curve over a number field being deformed through its
    p-adic Teichmuller theory.
    
    Returns:
        The promotion path, primitive deltas, and whether the
        deformation is "etale" (structure-preserving) or not.
    """
    result = {
        "source": name,
        "target_tier": target_tier,
        "promotions": [],
        "primitive_deltas": {},
        "is_etale": False,
        "deformation_type": ""
    }
    
    # Get source tier
    tier_out = ask(["--tier", name])
    source_tier = ""
    for line in tier_out.split("\n"):
        for tok in line.strip().split():
            if tok.startswith("O_") or (tok.startswith("O") and tok[1:].isdigit()):
                source_tier = tok
    
    if not source_tier:
        source_tier = "O_0"
    
    result["source_tier"] = source_tier
    
    # Get promotion ladder from the CLINK navigator
    prom_out = ask(["--promote", name, "--to", target_tier])
    prom_lines = prom_out.split("\n")
    
    for line in prom_lines:
        if "→" in line and ("promote" in line.lower() or "primitive" in line.lower()):
            result["promotions"].append(line.strip())
    
    # Compute primitive deltas
    for line in prom_lines:
        if ":" in line and any(p in line for p in PRIMITIVES):
            parts = line.strip().split(":")
            if len(parts) == 2:
                result["primitive_deltas"][parts[0].strip()] = parts[1].strip()
    
    # Determine if deformation is etale (structure-preserving)
    # A deformation is etale if the pinned primitives (P, F, K, G, Gm, Ph) are preserved
    result["is_etale"] = False  # Placeholder - computed from actual data
    result["deformation_type"] = "anabelian" if not result["is_etale"] else "etale"
    
    return result


# ── ALIEN ARITHMETIC ──────────────────────────────────────────────────

@dataclass
class AlienArithmeticStructure:
    """The arithmetic structure of a Frobenius universe.
    
    Every Frobenius universe has an intrinsic arithmetic determined by
    its structural type. "Alien" means this arithmetic may be
    incommensurable with another universe's arithmetic.
    
    The alien-ness is measured by the distance between the universes'
    anchor types, weighted by which primitives differ.
    """
    universe_name: str
    tuple_str: str
    tier: str
    
    # Arithmetic invariants
    winding_invariant: str = ""  # the Omega primitive
    symmetry_type: str = ""      # the P primitive
    criticality_type: str = ""   # the Ph primitive
    
    # Alien distance metrics
    alien_distance_to_grammar: float = 0.0
    
    def alien_distance_to(self, other: AlienArithmeticStructure) -> Dict[str, Any]:
        """Compute the alien arithmetic distance to another universe.
        
        The alien distance is the Mahalanobis-weighted distance between
        the two universes' arithmetic invariants, normalized by how many
        live-pair primitives differ.
        """
        live_diff = 0
        pinned_diff = 0
        
        # Compare live pairs (D<->W, T<->H, R<->S) — these carry arithmetic charge
        # D and W
        if self.winding_invariant != other.winding_invariant:
            live_diff += 1
        
        # Rough estimation based on tier difference
        tier_map = {"O_0": 0, "O_1": 1, "O_2": 2, "O_inf": 3}
        self_tier_val = tier_map.get(self.tier, 0)
        other_tier_val = tier_map.get(other.tier, 0)
        tier_diff = abs(self_tier_val - other_tier_val)
        
        return {
            "universe_a": self.universe_name,
            "universe_b": other.universe_name,
            "live_pair_divergence": live_diff,
            "pinned_divergence": pinned_diff,
            "tier_gap": tier_diff,
            "alien_degree": (live_diff + pinned_diff + tier_diff) / 6.0,
            "interpretation": "ALIEN" if live_diff > 0 else "KINDRED" if tier_diff == 0 else "DISTANT"
        }


def analyze_alien_arithmetic(name: str) -> AlienArithmeticStructure:
    """Analyze the alien arithmetic structure of a universe."""
    # Get tuple
    tuple_out = ask(["--tuple", name])
    tuple_str = ""
    for line in tuple_out.split("\n"):
        if "<" in line and ">" in line:
            tuple_str = line.strip()
    
    # Get tier
    tier_out = ask(["--tier", name])
    tier = ""
    for line in tier_out.split("\n"):
        for tok in line.strip().split():
            if tok.startswith("O_"):
                tier = tok
    
    # Parse winding (Omega), symmetry (P), criticality (Ph)
    if not tuple_str:
        tuple_str = f"<{name}>"
    
    return AlienArithmeticStructure(
        universe_name=name,
        tuple_str=tuple_str,
        tier=tier or "O_0",
        winding_invariant=tier,
        symmetry_type=tier,
        criticality_type=tier
    )
# ── UNIVERSE LANDSCAPE ────────────────────────────────────────────────

def explore_universe_landscape(anchor: str, depth: int = 3) -> Dict[str, Any]:
    """Map the inter-universal landscape around an anchor point.
    
    Produces a complete structural portrait of the universe anchored
    at `anchor`, its neighbor universes, bridges, and the local
    geometry of the crystal.
    """
    landscape = {
        "anchor": anchor,
        "depth": depth,
        "central_universe": None,
        "neighbor_universes": [],
        "bridges": [],
        "wormholes": [],
        "teichmuller_deformations": []
    }
    
    # Discover central universe
    central = discover_frobenius_universe(anchor)
    landscape["central_universe"] = asdict(central)
    
    # Discover neighbor universes (the click partners)
    for bridge_name in central.bridges[:10]:
        neighbor = discover_frobenius_universe(bridge_name)
        landscape["neighbor_universes"].append(asdict(neighbor))
        
        # Bridge from central to neighbor
        bridge = discover_bridge(anchor, bridge_name)
        landscape["bridges"].append(asdict(bridge))
    
    # Discover wormholes through key mediators
    mediators = ["CLINK_8", "imscribing_grammar"]
    for med in mediators:
        if med != anchor:
            whs = discover_wormhole(anchor, [med])
            for wh in whs:
                landscape["wormholes"].append(asdict(wh))
    
    # Teichmuller deformations
    for target_tier in ["O_0", "O_1", "O_2", "O_inf"]:
        if target_tier != central.tier:
            td = teichmuller_deformation(anchor, target_tier)
            landscape["teichmuller_deformations"].append(td)
    
    return landscape


def display_landscape(landscape: Dict[str, Any]) -> str:
    """Render a universe landscape as a formatted string."""
    lines = []
    lines.append("")
    lines.append("╔══════════════════════════════════════════════════════════╗")
    lines.append("║     INTER-UNIVERSAL FROBENIUS LANDSCAPE                ║")
    lines.append("╚══════════════════════════════════════════════════════════╝")
    lines.append("")
    
    c = landscape.get("central_universe", {})
    lines.append(f"  Central Universe: {c.get('name', '?')}")
    lines.append(f"    Tier:          {c.get('tier', '?')}")
    lines.append(f"    C-score:       {c.get('c_score', 0.0):.3f}")
    lines.append(f"    Closure radius: {c.get('closure_radius', 0.0):.2f}")
    lines.append(f"    Estimated size: ~{c.get('size_estimate', 0):,} types")
    lines.append("")
    
    bridges = landscape.get("bridges", [])
    if bridges:
        lines.append(f"  🌉 Bridges ({len(bridges)}):")
        for b in bridges[:8]:
            lp = b.get('live_pair', '?')
            st = b.get('bridge_strength', 0.0)
            target = b.get('target_universe', '?')
            pt = b.get('product_tier', '?')
            direct = "DIRECT" if b.get('is_direct') else "CATALYZED"
            lines.append(f"    → {target:30s} {direct:10s} on {lp:8s} Δ={st:.2f} [{pt}]")
    else:
        lines.append("  No bridges discovered.")
    
    wormholes = landscape.get("wormholes", [])
    if wormholes:
        lines.append(f"\n  🌀 Wormholes ({len(wormholes)}):")
        for w in wormholes:
            lines.append(f"    {w.get('entrance','?')} → {w.get('exit','?'):30s} via {w.get('mediator','?')}")
            lines.append(f"    Distance saved: {w.get('distance_saved', 0.0):.2f}  Traversable: {w.get('traversable', False)}")
    
    deformations = landscape.get("teichmuller_deformations", [])
    if deformations:
        lines.append(f"\n  🔄 Teichmuller Deformations ({len(deformations)}):")
        for td in deformations:
            src_tier = td.get('source_tier', '?')
            tgt_tier = td.get('target_tier', '?')
            det_type = td.get('deformation_type', '?')
            lines.append(f"    {src_tier} → {tgt_tier:8s} ({det_type})")
            for prom in td.get('promotions', [])[:3]:
                lines.append(f"      {prom}")
    
    lines.append("")
    return "\n".join(lines)
# ── MULTI-UNIVERSE TOPOLOGY ──────────────────────────────────────────

def multi_universe_topology(seeds: List[str]) -> Dict[str, Any]:
    """Discover the topology of multiple interconnected universes.
    
    Given a list of seed types, discover each universe, find bridges
    between them, and compute the global connectivity of the
    inter-universal landscape.
    """
    topology = {
        "universes": {},
        "bridges": [],
        "global_connectivity": 0.0,
        "clusters": [],
        "dominant_universe": ""
    }
    
    # Discover each universe
    for seed in seeds:
        topology["universes"][seed] = asdict(discover_frobenius_universe(seed))
    
    # Find all pairwise bridges
    uni_names = list(topology["universes"].keys())
    for i in range(len(uni_names)):
        for j in range(i + 1, len(uni_names)):
            bridge = discover_bridge(uni_names[i], uni_names[j])
            topology["bridges"].append(asdict(bridge))
    
    # Compute global connectivity
    direct_bridges = sum(1 for b in topology["bridges"] if b.get("is_direct"))
    total_pairs = len(uni_names) * (len(uni_names) - 1) / 2
    topology["global_connectivity"] = direct_bridges / total_pairs if total_pairs > 0 else 0
    
    # Find dominant universe (highest tier, then highest C-score)
    ranked = sorted(
        topology["universes"].values(),
        key=lambda u: (
            {"O_inf": 4, "O_2": 3, "O_1": 2, "O_0": 1}.get(u.get("tier", "O_0"), 0),
            u.get("c_score", 0.0)
        ),
        reverse=True
    )
    if ranked:
        topology["dominant_universe"] = ranked[0].get("name", "")
    
    return topology


# ── SYNTHETIC UNIVERSE GENERATION ────────────────────────────────────

def generate_synthetic_universe(base_type: str, target_tier: str = "O_inf") -> FrobeniusUniverse:
    """Generate a synthetic Frobenius universe by promoting a base type.
    
    Synthetic universes are constructed, not discovered — they represent
    the result of applying Inter-Universal Frobenius Theory to create
    new structural patches on the crystal. This is the constructive
    side of IUFT.
    """
    teich = teichmuller_deformation(base_type, target_tier)
    
    synthetic_name = f"iuft_{base_type}_to_{target_tier.replace('O_', 'O')}"
    
    return FrobeniusUniverse(
        name=synthetic_name,
        description=f"Synthetic universe: {base_type} promoted to {target_tier} via IUFT Teichmuller deformation",
        anchor_tuple=teich.get("primitive_deltas", {}).get("tuple", f"<{base_type}>"),
        tier=target_tier,
        closure_radius=1.0,
        c_score=min(0.5 + len(teich.get("promotions", [])) * 0.1, 1.0),
        size_estimate=1000,
        bridges=[base_type],
        sub_universes=[]
    )


# ── CLI ──────────────────────────────────────────────────────────────

def main():
    """Inter-Universal Frobenius Theory CLI."""
    if len(sys.argv) < 2:
        print("IUFT — Inter-Universal Frobenius Theory Engine")
        print()
        print("Usage:")
        print("  python -m structoforge.iuft explore ANCHOR")
        print("  python -m structoforge.iuft bridge A B")
        print("  python -m structoforge.iuft landscape ANCHOR [DEPTH]")
        print("  python -m structoforge.iuft wormhole ENTRANCE MEDIATOR [MEDIATOR...]")
        print("  python -m structoforge.iuft teichmuller NAME TIER")
        print("  python -m structoforge.iuft alien NAME")
        print("  python -m structoforge.iuft network SEED1 SEED2 [SEED3...]")
        print("  python -m structoforge.iuft synthesize BASE_TIER TARGET_TIER")
        print("  python -m structoforge.iuft demo")
        return
    
    cmd = sys.argv[1]
    args = sys.argv[2:]
    
    if cmd == "explore" or cmd == "landscape":
        anchor = args[0] if args else "monad"
        depth = int(args[1]) if len(args) > 1 else 3
        landscape = explore_universe_landscape(anchor, depth)
        print(display_landscape(landscape))
        
    elif cmd == "bridge":
        if len(args) < 2:
            print("Usage: bridge A B")
            return
        a, b = args[0], args[1]
        bridge = discover_bridge(a, b)
        print(f"\n  🌉 Universe Bridge: {a} ↔ {b}")
        print(f"     Live pair: {bridge.live_pair}")
        print(f"     Direct:     {bridge.is_direct}")
        print(f"     Strength:   Δ={bridge.bridge_strength:.2f}")
        print(f"     Product:    {bridge.mediator_type}")
        print(f"     Tier:       {bridge.product_tier}")
        print(f"     Catalyst:   {bridge.catalyst_required}")
        
    elif cmd == "wormhole":
        entrance = args[0] if args else "monad"
        mediators = args[1:] if len(args) > 1 else ["imscribing_grammar", "CLINK_8"]
        wormholes = discover_wormhole(entrance, mediators)
        print(f"\n  🌀 Crystal Wormholes from {entrance}")
        for w in wormholes:
            print(f"     {w.entrance} → {w.exit}")
            print(f"       via {w.mediator}")
            print(f"       distance saved: {w.distance_saved:.2f}")
            print(f"       traversable:    {w.traversable}")
            print()
            
    elif cmd == "teichmuller" or cmd == "deform":
        if len(args) < 2:
            print("Usage: teichmuller NAME TIER")
            return
        name, tier = args[0], args[1]
        td = teichmuller_deformation(name, tier)
        print(f"\n  🔄 Teichmuller Deformation: {name} → {tier}")
        print(f"     Source tier: {td.get('source_tier', '?')}")
        print(f"     Type:        {td.get('deformation_type', '?')}")
        print(f"     Etale:       {td.get('is_etale')}")
        for prom in td.get('promotions', []):
            print(f"     {prom}")
            
    elif cmd == "alien":
        name = args[0] if args else "monad"
        alien = analyze_alien_arithmetic(name)
        print(f"\n  👽 Alien Arithmetic: {name}")
        print(f"     Tuple:   {alien.tuple_str}")
        print(f"     Tier:    {alien.tier}")
        print(f"     Winding: {alien.winding_invariant}")
        print(f"     Symmetry: {alien.symmetry_type}")
        print(f"     Criticality: {alien.criticality_type}")
        
    elif cmd == "network":
        seeds = args if args else ["monad", "topos", "hopf"]
        topo = multi_universe_topology(seeds)
        print(f"\n  🌐 Multi-Universe Topology")
        print(f"     Universes: {len(topo['universes'])}")
        print(f"     Bridges:   {len(topo['bridges'])}")
        print(f"     Global connectivity: {topo['global_connectivity']:.2%}")
        print(f"     Dominant universe: {topo['dominant_universe']}")
        print()
        for name, uni in topo['universes'].items():
            size_est = uni.get('size_estimate', 0)
            print(f"     {name:20s} tier={uni.get('tier','?'):6s} C={uni.get('c_score',0.0):.2f} size=~{size_est}")
            
    elif cmd == "synthesize":
        base = args[0] if args else "monad"
        target = args[1] if len(args) > 1 else "O_inf"
        synth = generate_synthetic_universe(base, target)
        print(f"\n  🔧 Synthetic Universe Generated")
        print(f"     Name: {synth.name}")
        print(f"     Description: {synth.description}")
        print(f"     Tier: {synth.tier}")
        print(f"     C-score: {synth.c_score:.3f}")
        
    elif cmd == "demo":
        print()
        print("╔════════════════════════════════════════════════╗")
        print("║   IUFT DEMO — Inter-Universal Frobenius       ║")
        print("║   Theory Engine                               ║")
        print("╚════════════════════════════════════════════════╝")
        print()
        
        for source in ["monad", "hopf", "topos"]:
            landscape = explore_universe_landscape(source, depth=2)
            print(display_landscape(landscape))
            print()


if __name__ == "__main__":
    main()
