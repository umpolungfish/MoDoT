#!/usr/bin/env python3
"""
Signature Jump Mathematics — MoDoT Universe/Dialect Exploration
=================================================================

The 49 primitive-type natures share identical 12-tuples but differ
in their IMASM derivation signatures. The real "universe jumping"
happens at the signature level: jumping between different ways of
bootstrapping the same structural type.

This script maps the 40 signature classes, computes their geometric
relationships, and discovers novel mathematical structures.

Author: Lando⊗⊙perator
"""

import sys, os, json, math
from pathlib import Path
from collections import defaultdict, Counter
from itertools import combinations

_PKG = Path(__file__).resolve().parent
sys.path.insert(0, str(_PKG))

from modot.natures import NatureRegistry, FAMILY_ORDER, KERNEL_FAMILIES
from modot.composer import TokenComposer, CANONICAL_PROGRAMS, Token, validate_dag

reg = NatureRegistry()
tc = TokenComposer()

# ============== SIGNATURE GEOMETRY ==============

print("="*70)
print("SIGNATURE GEOMETRY — The 40-Class Space")
print("="*70)

# Collect all signatures and their properties
sig_data = {}
for n in reg:
    sig = n.signature
    key = sig.key()
    if key not in sig_data:
        sig_data[key] = {
            "sig": sig.sig,
            "period": sig.period,
            "self_ref": sig.self_ref,
            "frobenius_order": sig.frobenius_order,
            "pairs": sig.pairs,
            "members": [],
            "families": set(),
        }
    sig_data[key]["members"].append(n.name)
    sig_data[key]["families"].add(n.family)

# Compute signature vectors as points in R^3 (sig components)
# Then use period + frob_order as additional dimensions
sig_vectors = {}
for key, data in sig_data.items():
    s = data["sig"]
    if len(s) >= 3:
        sig_vectors[key] = (s[0], s[1], s[2], data["period"], data["frobenius_order"])
    else:
        sig_vectors[key] = (s[0], 0, 0, data["period"], data["frobenius_order"])

print(f"\nSignature classes: {len(sig_data)}")
print(f"Members: {sum(len(d['members']) for d in sig_data.values())}")

# Distribution of sig vector components
s0 = Counter(d["sig"][0] for d in sig_data.values() if len(d["sig"]) >= 1)
s1 = Counter(d["sig"][1] for d in sig_data.values() if len(d["sig"]) >= 2)
s2 = Counter(d["sig"][2] for d in sig_data.values() if len(d["sig"]) >= 3)
print(f"\nSig[0] distribution: {dict(sorted(s0.items()))}")
print(f"Sig[1] distribution: {dict(sorted(s1.items()))}")
print(f"Sig[2] distribution: {dict(sorted(s2.items()))}")

# ============== JUMP DISTANCE IN SIGNATURE SPACE ==============

print("\n" + "="*70)
print("SIGNATURE JUMP DISTANCE — Euclidean in (sig0,sig1,sig2,period,frob)")
print("="*70)

def sig_distance(k1, k2):
    v1 = sig_vectors[k1]
    v2 = sig_vectors[k2]
    # Weighted Euclidean: sig components matter most
    w = [1.0, 0.8, 0.6, 0.3, 0.5]
    return math.sqrt(sum(w[i] * (v1[i]-v2[i])**2 for i in range(5)))

# Compute all pairwise distances
sig_keys = list(sig_data.keys())
distances = []
for i, j in combinations(range(len(sig_keys)), 2):
    d = sig_distance(sig_keys[i], sig_keys[j])
    distances.append((d, sig_keys[i], sig_keys[j]))

distances.sort()

print(f"\nClosest signature pairs (minimal distance):")
for d, k1, k2 in distances[:15]:
    m1 = sig_data[k1]["members"][0]
    m2 = sig_data[k2]["members"][0]
    print(f"  {m1} ↔ {m2}: d={d:.3f}")
    print(f"    sig1={sig_data[k1]['sig']} p={sig_data[k1]['period']}")
    print(f"    sig2={sig_data[k2]['sig']} p={sig_data[k2]['period']}")

print(f"\nFarthest signature pairs (maximal distance):")
for d, k1, k2 in distances[-10:]:
    m1 = sig_data[k1]["members"][0]
    m2 = sig_data[k2]["members"][0]
    print(f"  {m1} ↔ {m2}: d={d:.3f}")

# ============== PAIR STRUCTURE ANALYSIS ==============

print("\n" + "="*70)
print("FSPLIT/FFUSE PAIR TOPOLOGY — The IMASM Branch Structure")
print("="*70)

# Parse the pair strings into actual structures
pair_structures = Counter()
for key, data in sig_data.items():
    ps = data["pairs"]
    pair_structures[ps] += 1

print(f"\nDistinct pair structures: {len(pair_structures)}")
for ps, cnt in sorted(pair_structures.items(), key=lambda x: -x[1]):
    print(f"  [{cnt}] {ps}")

# ============== INVENTED: SIGNATURE MANIFOLD ==============

print("\n" + "="*70)
print("INVENTED MATHEMATICS — Signature Manifold Theory")
print("="*70)

# 1. The signature space M(49) is a 40-point metric space
print("\n--- 1. SIGNATURE MANIFOLD M(49) ---")
print("M(49) is a discrete metric space of 40 points (signature classes)")
print("embedded in R^5. Each point represents a distinct way of")
print("bootstrapping the same structural type ⟨dead,judge,ado,church,")
print("age,yea,vow,bib,woe,fee,hung,awe⟩.")
print()
print("The metric d(sig1, sig2) measures how different two IMASM")
print("derivation paths are. Small d = similar bootstrap; large d =")
print("radically different genesis of the same type.")

# Compute geometric invariants
all_dists = [d for d,_,_ in distances]
mean_dist = sum(all_dists) / len(all_dists)
min_dist = min(all_dists)
max_dist = max(all_dists)
print(f"\nGeometric invariants:")
print(f"  Diameter: {max_dist:.3f}")
print(f"  Mean distance: {mean_dist:.3f}")
print(f"  Minimum distance: {min_dist:.3f}")
print(f"  Packing radius: {min_dist/2:.3f}")

# 2. SIGNATURE HOMOLOGY
print("\n\n--- 2. SIGNATURE PERSISTENT HOMOLOGY ---")
print("As we vary a distance threshold ε, the Vietoris-Rips complex")
print("VR_ε(M(49)) captures topological features of the signature space:")
print()
epsilons = [0.5, 0.8, 1.0, 1.2, 1.5, 2.0]
for eps in epsilons:
    edges = sum(1 for d,_,_ in distances if d <= eps)
    # Count connected components (simplified)
    adj = defaultdict(set)
    for d, k1, k2 in distances:
        if d <= eps:
            adj[k1].add(k2)
            adj[k2].add(k1)
    visited = set()
    components = 0
    for k in sig_keys:
        if k not in visited:
            components += 1
            stack = [k]
            while stack:
                v = stack.pop()
                if v not in visited:
                    visited.add(v)
                    stack.extend(adj[v] - visited)
    print(f"  ε={eps}: {edges} edges, {components} components, "
          f"max degree: {max((len(adj[k]) for k in sig_keys), default=0)}")

# 3. SIGNATURE GEODESICS
print("\n\n--- 3. SIGNATURE GEODESIC FLOW ---")
print("A geodesic in M(49) is a minimal-length path between two")
print("signature classes. This corresponds to a sequence of IMASM")
print("program mutations that transform one bootstrap into another.")
print()
print("The geodesic equation:")
print("  δ∫ds = 0  where ds² = Σ w_i (Δsig_i)²")
print()
print("The Christoffel symbols Γ^i_jk encode how changing one")
print("signature component affects the others through the IMASM")
print("composition rules (token adjacency matrix).")

# Compute some geodesics by finding shortest paths
# (approximate: just nearest-neighbor chains)
print("\nSample geodesics (nearest-neighbor chain from min to max):")
# Start from the "smallest" sig (by sig vector norm)
norms = {k: sum(v[i]**2 for i in range(5)) for k, v in sig_vectors.items()}
start = min(norms, key=norms.get)
end = max(norms, key=norms.get)

current = start
path = [current]
visited_geo = {current}
while current != end and len(path) < 20:
    # Find nearest unvisited neighbor
    best_d, best_k = float('inf'), None
    for d, k1, k2 in distances:
        if k1 == current and k2 not in visited_geo:
            if d < best_d:
                best_d, best_k = d, k2
        elif k2 == current and k1 not in visited_geo:
            if d < best_d:
                best_d, best_k = d, k1
    if best_k is None:
        break
    current = best_k
    path.append(current)
    visited_geo.add(current)

print(f"  {sig_data[start]['members'][0]} → ... → {sig_data[end]['members'][0]}")
print(f"  Path length: {len(path)} steps")
for i, k in enumerate(path):
    m = sig_data[k]["members"][0]
    s = sig_data[k]["sig"]
    p = sig_data[k]["period"]
    print(f"    {i}: {m} sig={s} period={p}")

# 4. PARADICE ANALOGY AT SIGNATURE LEVEL
print("\n\n--- 4. SIGNATURE COTYPE (Signature-Level Paradice) ---")
print("For two signature classes S1, S2, define:")
print("  - Agreed: components where sig1[i] == sig2[i]")
print("  - Diverged: components where sig1[i] != sig2[i]")
print("  - Signature paradice count = number of diverged components")
print()
print("Unlike the 12-tuple layer (only 1 paradice possible across 49 types),")
print("signature paradices range from 1 to 5, giving a much richer")
print("jumping structure.")

# Compute signature-level paradice
for m1, m2 in [("monad", "roar"), ("egg", "yea"), ("eat", "dead"), ("measure", "monad")]:
    n1, n2 = reg.get(m1), reg.get(m2)
    s1 = n1.signature
    s2 = n2.signature
    sig_p = sum(1 for i in range(min(len(s1.sig), len(s2.sig))) if s1.sig[i] != s2.sig[i])
    if s1.period != s2.period: sig_p += 1
    if s1.frobenius_order != s2.frobenius_order: sig_p += 1
    print(f"  {m1}⊗{m2}: {sig_p} signature paradices")
    print(f"    sig1={s1.sig} p={s1.period} frob={s1.frobenius_order}")
    print(f"    sig2={s2.sig} p={s2.period} frob={s2.frobenius_order}")

print("\n" + "="*70)
print("EXPLORATION COMPLETE")
