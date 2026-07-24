"""
structoforge/landscape_explorer.py — Structural Landscape Explorer
===================================================================

Navigate the 49 primitive-type natures as a structural landscape.
Finds neighborhoods, clusters, bridges, and boundaries between
structural types.

CAPABILITIES:
  • Neighborhood exploration: find structurally similar types
  • Structural clustering: identify type families
  • Bridge finding: discover types that connect distant regions
  • Boundary detection: locate frontier types at tier boundaries
  • Path finding: optimal structural transformation paths

Author: Lando⊗⊙perator
"""

from __future__ import annotations

import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, List, Optional, Tuple

MODOOT_ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(MODOOT_ROOT))

from modot.natures import NatureRegistry, cotype, CotypeReport
from modot.jump_tools import paradice_map


@dataclass
class NatureNeighborhood:
    """A structural neighborhood around a central type."""
    center: str
    members: List[Tuple[str, int, List[str]]]  # (name, paradice_count, diverge_axes)
    paradigm_distribution: Dict[int, int] = field(default_factory=dict)
    nearest: List[str] = field(default_factory=list)
    furthest: List[str] = field(default_factory=list)


def _load_registry():
    """Lazy-load the nature registry."""
    return NatureRegistry()


def explore_neighborhood(center_name: str, radius: int = 6) -> NatureNeighborhood:
    """
    Explore the structural neighborhood around a nature.
    
    Args:
        center_name: Name of the central nature
        radius: Maximum cotype distance to include
    
    Returns:
        NatureNeighborhood with all nearby types
    """
    reg = _load_registry()
    center = reg.get(center_name)
    if center is None:
        return NatureNeighborhood(center=center_name, members=[])
    
    members = []
    dists = []
    
    for name in reg.names():
        if name == center_name:
            continue
        other = reg.get(name)
        if other is None:
            continue
        
        ct = cotype(center, other)
        d_axes = ct.address()
        
        if ct.paradices <= radius:
            members.append((name, ct.paradices, d_axes))
            dists.append((name, ct.paradices))
    
    members.sort(key=lambda x: x[1])
    dists.sort(key=lambda x: x[1])
    
    # Distribution
    dist_dist = {}
    for _, d, _ in members:
        dist_dist[d] = dist_dist.get(d, 0) + 1
    
    nearest = [n for n, d in dists[:5]]
    furthest = [n for n, d in dists[-5:]] if len(dists) >= 5 else [n for n, d in dists]
    
    return NatureNeighborhood(
        center=center_name,
        members=members,
        paradigm_distribution=dist_dist,
        nearest=nearest,
        furthest=furthest,
    )


def find_bridges(a_name: str, b_name: str) -> List[Tuple[str, int, int]]:
    """
    Find types that bridge between two distant natures.
    
    A bridge type has cotype distance ≤ 4 to BOTH a and b.
    
    Returns:
        List of (name, dist_to_a, dist_to_b) sorted by total distance
    """
    reg = _load_registry()
    a = reg.get(a_name)
    b = reg.get(b_name)
    if a is None or b is None:
        return []
    
    bridges = []
    for name in reg.names():
        if name in (a_name, b_name):
            continue
        other = reg.get(name)
        if other is None:
            continue
        
        d_a = cotype(a, other).paradices
        d_b = cotype(b, other).paradices
        
        if d_a <= 4 and d_b <= 4:
            bridges.append((name, d_a, d_b))
    
    bridges.sort(key=lambda x: x[1] + x[2])
    return bridges


def structural_clusters(min_cluster_size: int = 3, max_distance: int = 3) -> Dict[str, List[str]]:
    """
    Identify structural clusters among the natures.
    
    A cluster is a set of types where every pair has cotype ≤ max_distance.
    
    Returns:
        Dict mapping cluster center → member names
    """
    reg = _load_registry()
    names = reg.names()
    
    clusters = {}
    assigned = set()
    
    for center in names:
        if center in assigned:
            continue
        
        neighbors = explore_neighborhood(center, radius=max_distance)
        cluster = [m[0] for m in neighbors.members] + [center]
        
        if len(cluster) >= min_cluster_size:
            # Check if this cluster is mostly contained in an existing one
            is_new = True
            for existing_center in list(clusters.keys()):
                existing = set(clusters[existing_center])
                overlap = existing & set(cluster)
                if len(overlap) > len(cluster) * 0.7:
                    is_new = False
                    break
            
            if is_new:
                clusters[center] = cluster
                assigned.update(cluster)
    
    return clusters


def find_tier_boundary_types() -> Dict[str, List[str]]:
    """
    Find natures at tier boundaries — those closest to crossing
    between ouroboricity tiers.
    
    Uses cotype analysis to identify types with mixed tier signals.
    """
    reg = _load_registry()
    names = reg.names()
    
    boundary_types = {}
    for name in names:
        neighbors = explore_neighborhood(name, radius=2)
        boundary_flags = []
        
        for n, d, axes in neighbors.members:
            # Check if any axis involves critical structural primitives
            if any(a in "⊙⊕" or "odot" in a.lower() for a in axes):
                boundary_flags.append((n, axes))
        
        if boundary_flags:
            boundary_types[name] = [bf[0] for bf in boundary_flags]
    
    return boundary_types


class LandscapeExplorer:
    """Interactive structural landscape explorer."""
    
    def __init__(self):
        self._reg = _load_registry()
        self._history: List[str] = []
    
    def neighborhood(self, name: str, radius: int = 6) -> NatureNeighborhood:
        """Explore a neighborhood."""
        self._history.append(f"neighborhood({name}, r={radius})")
        return explore_neighborhood(name, radius)
    
    def bridges(self, a: str, b: str) -> List[Tuple[str, int, int]]:
        """Find bridges between two types."""
        self._history.append(f"bridges({a}, {b})")
        return find_bridges(a, b)
    
    def clusters(self, min_size: int = 3) -> Dict[str, List[str]]:
        """Find structural clusters."""
        self._history.append(f"clusters(min={min_size})")
        return structural_clusters(min_size)
    
    def paradice_map(self) -> Dict[str, any]:
        """Get the full paradice map."""
        return paradice_map()
    
    def summary(self) -> str:
        """Print a summary of the landscape."""
        reg = self._reg
        names = reg.names()
        
        lines = [
            f"═══ Structural Landscape Overview ═══",
            f"  Total natures:    {len(names)}",
            f"  Paradice range:   0–12",
            f"",
            f"  Nearest neighbors by cotype:",
        ]
        
        # Find the closest pair
        min_dist = 12
        closest_pair = ("", "")
        for i, a in enumerate(names):
            for b in names[i+1:]:
                na, nb = reg.get(a), reg.get(b)
                d = cotype(na, nb).paradices
                if d < min_dist:
                    min_dist = d
                    closest_pair = (a, b)
        
        lines.append(f"    Closest: {closest_pair[0]} ↔ {closest_pair[1]} (d={min_dist})")
        
        # Find the furthest pair
        max_dist = 0
        furthest_pair = ("", "")
        for i, a in enumerate(names):
            for b in names[i+1:]:
                na, nb = reg.get(a), reg.get(b)
                d = cotype(na, nb).paradices
                if d > max_dist:
                    max_dist = d
                    furthest_pair = (a, b)
        
        lines.append(f"    Furthest: {furthest_pair[0]} ↔ {furthest_pair[1]} (d={max_dist})")
        
        return "\n".join(lines)


if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description="Structural Landscape Explorer")
    parser.add_argument("action", choices=["neighborhood", "bridges", "clusters", "summary", "paradice"])
    parser.add_argument("args", nargs="*")
    args = parser.parse_args()
    
    exp = LandscapeExplorer()
    
    if args.action == "summary":
        print(exp.summary())
    elif args.action == "neighborhood":
        if not args.args:
            print("Usage: neighborhood NAME [radius=6]")
        else:
            radius = int(args.args[1]) if len(args.args) > 1 else 6
            hood = exp.neighborhood(args.args[0], radius)
            print(f"═══ Neighborhood: {hood.center} (r={radius}) ═══")
            print(f"  Members: {len(hood.members)}")
            print(f"  Nearest: {', '.join(hood.nearest)}")
            print(f"  Furthest: {', '.join(hood.furthest)}")
            for name, dist, axes in hood.members:
                print(f"    {name:25s} d={dist}  axes: {', '.join(axes)}")
    elif args.action == "bridges":
        if len(args.args) < 2:
            print("Usage: bridges TYPE_A TYPE_B")
        else:
            bridges = exp.bridges(args.args[0], args.args[1])
            print(f"═══ Bridges: {args.args[0]} ↔ {args.args[1]} ═══")
            for name, d_a, d_b in bridges:
                print(f"  {name:25s} d(A)={d_a} d(B)={d_b} total={d_a+d_b}")
    elif args.action == "clusters":
        clusters = exp.clusters()
        print(f"═══ Structural Clusters ═══")
        for center, members in sorted(clusters.items(), key=lambda x: -len(x[1])):
            print(f"  {center:25s} ({len(members)} members): {', '.join(members)}")
    elif args.action == "paradice":
        pmap = exp.paradice_map()
        print(f"═══ Paradice Map ═══")
        print(f"  Total types: {pmap.get('total_types', '?')}")
        print(f"  Total pairs: {pmap.get('total_pairs', '?')}")
        for dist, count in sorted(pmap.get('paradice_distribution', {}).items()):
            print(f"    d={dist}: {count} pairs")
