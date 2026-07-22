#!/usr/bin/env python3
"""
MoDoT Universe/Dialect Jump Tools
===================================

Callable tools for the paradice algebra over the 49 primitive-type natures.
Exposes the full jump machinery (cotype, paradice map, braid words, composite
types, signature geometry) as importable + CLI-callable functions.

Adds: paradice_lattice, universe_jump, paradice_graph, signature_manifold,
      braid_word, frobenius_closure_check, jump_path_integral.

Author: Lando⊗⊙perator
Date:   2026-07-17
"""

from __future__ import annotations

import json
import math
import sys
from collections import Counter, defaultdict
from itertools import combinations
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

_PKG = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(_PKG))

from modot.natures import (
    NatureRegistry, Nature, cotype, CotypeReport, paradices,
    FAMILY_TO_VESSEL_KEY, FAMILY_ORDER, KERNEL_FAMILIES
)
from modot.composer import Token

# ── global registry shared across all tools ────────────────────────────
_registry: Optional[NatureRegistry] = None


def registry() -> NatureRegistry:
    global _registry
    if _registry is None:
        _registry = NatureRegistry()
    return _registry


def nature(name: str) -> Optional[Nature]:
    return registry().get(name)


# ── Tool: paradice_map ─────────────────────────────────────────────────

def paradice_map() -> Dict[str, Any]:
    """Full paradice map: all 1176 pairwise cotype divergences.
    
    Returns:
        dict with keys: counts (paradice count distribution),
        total_pairs, max_paradices, min_paradices,
        pairs: list of (a, b, paradices, diverge_axes, agree_axes)
    """
    reg = registry()
    names = reg.names()
    all_pairs = []
    p_counts = Counter()
    
    for a, b in combinations(names, 2):
        rep = cotype(reg.get(a), reg.get(b))
        p_counts[rep.paradices] += 1
        all_pairs.append({
            "a": a, "b": b,
            "paradices": rep.paradices,
            "diverge_axes": rep.address(),
            "agree_axes": [FAMILY_TO_VESSEL_KEY[f] for f in rep.agree],
        })
    
    return {
        "total_types": len(names),
        "total_pairs": len(all_pairs),
        "paradice_distribution": {str(k): v for k, v in sorted(p_counts.items())},
        "max_paradices": max(p_counts.keys()),
        "pairs": sorted(all_pairs, key=lambda x: x["paradices"], reverse=True),
    }


# ── Tool: universe_jump ────────────────────────────────────────────────

def universe_jump(a_name: str, b_name: str) -> Dict[str, Any]:
    """Perform a dialect jump between two natures.
    
    Creates a composite type by FSPLIT(cotype A,B) → holding diverged axes as Both (B4.B) → FFUSE.
    
    Returns:
        dict with: a, b, paradices, diverge_axes, agree_axes,
        composite_tuple, jump_program, cotype_report_summary
    """
    reg = registry()
    na, nb = reg.get(a_name), reg.get(b_name)
    if na is None:
        return {"error": f"nature '{a_name}' not found"}
    if nb is None:
        return {"error": f"nature '{b_name}' not found"}
    
    rep = cotype(na, nb)
    
    composite = {}
    for fam in FAMILY_ORDER:
        vk = FAMILY_TO_VESSEL_KEY[fam]
        if fam in rep.agree:
            composite[vk] = rep.agree[fam]
        else:
            composite[vk] = f"Both({rep.diverge[fam][0]},{rep.diverge[fam][1]})"
    
    jump_program = [
        "VINIT", "IMSCRIB", "FSPLIT", "EVALT", "AFWD",
        "IMSCRIB", "EVALF", "FFUSE", "ENGAGR", "IFIX", "CLINK", "TANCH"
    ]
    
    return {
        "a": a_name, "b": b_name,
        "paradices": rep.paradices,
        "diverge_axes": rep.address(),
        "agree_axes": [FAMILY_TO_VESSEL_KEY[f] for f in rep.agree],
        "composite_tuple": composite,
        "jump_program": jump_program,
        "cotype_report": {
            "a_family": na.family, "b_family": nb.family,
            "a_signature": {"sig": na.signature.sig, "period": na.signature.period,
                            "self_ref": na.signature.self_ref, "frobenius_order": na.signature.frobenius_order,
                            "tier": na.signature.tier},
            "b_signature": {"sig": nb.signature.sig, "period": nb.signature.period,
                            "self_ref": nb.signature.self_ref, "frobenius_order": nb.signature.frobenius_order,
                            "tier": nb.signature.tier},
        },
    }


# ── Tool: braid_word ───────────────────────────────────────────────────

def braid_word(a_name: str, b_name: str) -> Dict[str, Any]:
    """Compute the braid word for a paradice jump.
    
    Each diverged axis becomes a braid generator σ_i.
    """
    rep = cotype(registry().get(a_name), registry().get(b_name))
    generators = []
    for fam in FAMILY_ORDER:
        if fam in rep.diverge:
            idx = FAMILY_ORDER.index(fam)
            generators.append(f"σ_{idx}")
    return {
        "a": a_name, "b": b_name,
        "braid_word": " · ".join(generators) if generators else "id",
        "num_strands": 12,
        "num_crossings": len(generators),
        "diverge_families": [FAMILY_TO_VESSEL_KEY[f] for f in rep.diverge],
    }


# ── Tool: paradice_lattice ─────────────────────────────────────────────

def paradice_lattice() -> Dict[str, Any]:
    """Build the paradice lattice: 49 nodes, edges = 1-paradice neighbors.
    
    Each edge carries which axis (vessel key) the neighbors differ on.
    """
    reg = registry()
    nodes = []
    edges = []
    for name in sorted(reg.names()):
        n = reg.get(name)
        nodes.append({
            "name": name, "family": n.family, "vessel_key": n.vessel_key,
            "ordinal": n.ordinal, "signature": n.signature.key(),
        })
    
    for a, b in combinations(reg.names(), 2):
        p = paradices(reg.get(a), reg.get(b))
        if p == 1:
            rep = cotype(reg.get(a), reg.get(b))
            axis = rep.address()[0] if rep.address() else "?"
            edges.append({"a": a, "b": b, "paradices": 1, "differ_on": axis})
    
    return {
        "num_nodes": len(nodes),
        "num_edges": len(edges),
        "nodes": nodes,
        "edges": edges,
    }


# ── Tool: signature_manifold ───────────────────────────────────────────

def signature_manifold() -> Dict[str, Any]:
    """Compute the signature manifold M(49): geometry over signature classes.
    
    Returns: class count, diameter estimates, adjacency, frobenius barrier info.
    """
    reg = registry()
    sig_classes = reg.signature_classes()
    
    # Count frobenius_order=3 types (the "heavy" types)
    frob3_types = [n.name for n in reg if n.signature.frobenius_order == 3]
    
    # Build the signature class adjacency (two classes adjacent if differ by 1 paradice)
    # Map each type to its signature class key
    name_to_key = {n.name: n.signature.key() for n in reg}
    
    adj = defaultdict(set)
    for a, b in combinations(reg.names(), 2):
        if paradices(reg.get(a), reg.get(b)) == 1:
            ka, kb = name_to_key[a], name_to_key[b]
            if ka != kb:
                adj[ka].add(kb)
                adj[kb].add(ka)
    
    sig_adj = {str(k): [str(vv) for vv in sorted(v)] for k, v in adj.items()}
    
    return {
        "num_types": len(reg),
        "num_signature_classes": len(sig_classes),
        "frobenius_barrier_types": frob3_types,
        "signature_class_sizes": {str(k): len(v) for k, v in sig_classes.items()},
        "signature_adjacency": sig_adj,
    }


# ── Tool: frobenius_closure_check ──────────────────────────────────────

def frobenius_closure_check(a_name: str, b_name: str) -> Dict[str, Any]:
    """Check whether a paradice jump is Frobenius-closed.
    
    A jump is Frobenius-closed when diverged axes form complete dual pairs:
    D↔T, R↔Ph, F↔K, G↔Gm, Ph↔H, S↔W.
    
    The vessel keys: D↔T, R↔Ph, F↔K, G↔Gm, Ph↔H, S↔W.
    """
    DUAL_PAIRS = [
        ("D", "T"), ("R", "Ph"), ("F", "K"), ("G", "Gm"), ("Ph", "H"), ("S", "W"),
    ]
    rep = cotype(registry().get(a_name), registry().get(b_name))
    diverged = set(rep.address())
    
    # Check if diverged axes can be partitioned into complete dual pairs
    uncovered = set(diverged)
    paired = []
    unpaired = []
    
    for p1, p2 in DUAL_PAIRS:
        if p1 in uncovered and p2 in uncovered:
            paired.append((p1, p2))
            uncovered.discard(p1)
            uncovered.discard(p2)
        elif p1 in uncovered:
            unpaired.append(p1)
            uncovered.discard(p1)
        elif p2 in uncovered:
            unpaired.append(p2)
            uncovered.discard(p2)
    
    unpaired.extend(uncovered)
    
    closed = len(unpaired) == 0
    
    return {
        "a": a_name, "b": b_name,
        "paradices": rep.paradices,
        "diverged_axes": list(diverged),
        "dual_pairs_closed": paired,
        "unpaired_axes": unpaired,
        "frobenius_closed": closed,
        "verdict": "μ∘δ=id ✓" if closed else f"μ∘δ≠id: {len(unpaired)} unpaired axes",
    }


# ── Tool: jump_path_integral ───────────────────────────────────────────

def jump_path_integral(a_name: str, b_name: str, max_steps: int = 12) -> Dict[str, Any]:
    """Compute the paradice path integral Z(A→B).
    
    Sum over all paths of length ≤ max_steps of 2^{-|γ|} where |γ| = paradices per step.
    Returns the propagator and shortest path.
    """
    from collections import deque
    
    reg = registry()
    names = reg.names()
    N = len(names)
    idx = {n: i for i, n in enumerate(names)}
    
    if a_name not in idx or b_name not in idx:
        return {"error": f"nature '{a_name}' or '{b_name}' not found"}
    
    start, target = idx[a_name], idx[b_name]
    
    # BFS for shortest path
    dist = {start: 0}
    parent = {start: None}
    q = deque([start])
    
    while q:
        u = q.popleft()
        if u == target:
            break
        for v in range(N):
            if v in dist:
                continue
            p = paradices(reg.get(names[u]), reg.get(names[v]))
            if p <= max_steps:
                dist[v] = dist[u] + 1
                parent[v] = u
                q.append(v)
    
    if target not in dist:
        return {"error": f"no path found within {max_steps} steps"}
    
    # Reconstruct path
    path = []
    cur = target
    while cur is not None:
        path.append(names[cur])
        cur = parent[cur]
    path.reverse()
    
    # Compute propagator (naive: just the shortest path weight)
    total_weight = 0
    for i in range(len(path) - 1):
        p = paradices(reg.get(path[i]), reg.get(path[i+1]))
        total_weight += p
    
    Z = 2.0 ** (-total_weight)
    
    return {
        "a": a_name, "b": b_name,
        "shortest_path_length": dist[target],
        "path": path,
        "total_paradices": total_weight,
        "propagator_Z": Z,
        "direct_paradices": paradices(reg.get(a_name), reg.get(b_name)),
    }


# ── Tool: composite_type ───────────────────────────────────────────────

def composite_type(names_list: List[str]) -> Dict[str, Any]:
    """Build the multi-way composite type from a list of nature names.
    
    Folds all natures via cotype: agreed axes = T, diverged = Both(all values).
    """
    reg = registry()
    natures = [reg.get(n) for n in names_list]
    if any(n is None for n in natures):
        missing = [names_list[i] for i, n in enumerate(natures) if n is None]
        return {"error": f"natures not found: {missing}"}
    
    # Multi-way cotype
    composite = {}
    for fam in FAMILY_ORDER:
        values = set()
        for n in natures:
            values.add(n.tuple.get(fam, "?"))
        vk = FAMILY_TO_VESSEL_KEY[fam]
        if len(values) == 1:
            composite[vk] = values.pop()
        else:
            composite[vk] = f"Both({','.join(sorted(values))})"
    
    # Count paradices = axes where not all agree
    total_paradices = sum(1 for v in composite.values() if v.startswith("Both"))
    
    return {
        "source_types": names_list,
        "num_sources": len(names_list),
        "composite_tuple": composite,
        "total_paradices": total_paradices,
    }


# ── CLI dispatcher ─────────────────────────────────────────────────────

def dispatch(tool_name: str, args: Dict[str, Any] = None) -> Dict[str, Any]:
    """Dispatch a named tool with JSON args. Returns result dict."""
    if args is None:
        args = {}
    
    tools = {
        "paradice_map": paradice_map,
        "universe_jump": lambda: universe_jump(args.get("a", ""), args.get("b", "")),
        "braid_word": lambda: braid_word(args.get("a", ""), args.get("b", "")),
        "paradice_lattice": paradice_lattice,
        "signature_manifold": signature_manifold,
        "frobenius_closure_check": lambda: frobenius_closure_check(
            args.get("a", ""), args.get("b", "")),
        "jump_path_integral": lambda: jump_path_integral(
            args.get("a", ""), args.get("b", ""), args.get("max_steps", 12)),
        "composite_type": lambda: composite_type(args.get("names", [])),
    }
    
    if tool_name not in tools:
        return {"error": f"unknown tool '{tool_name}'. Available: {list(tools.keys())}"}
    
    return tools[tool_name]()


def main():
    """CLI entry: python -m modot.jump_tools <tool_name> [a=<val>] [b=<val>] [names=a,b,c]"""
    if len(sys.argv) < 2:
        print("Usage: python -m modot.jump_tools <tool_name> [a=X] [b=Y]")
        print("Available tools:", list({
            "paradice_map", "universe_jump", "braid_word", "paradice_lattice",
            "signature_manifold", "frobenius_closure_check", "jump_path_integral",
            "composite_type"
        }))
        return
    
    tool_name = sys.argv[1]
    args = {}
    for arg in sys.argv[2:]:
        if "=" in arg:
            k, v = arg.split("=", 1)
            if k == "names":
                args[k] = v.split(",")
            elif k == "max_steps":
                args[k] = int(v)
            else:
                args[k] = v
    
    result = dispatch(tool_name, args)
    print(json.dumps(result, indent=2, default=str))


if __name__ == "__main__":
    main()
