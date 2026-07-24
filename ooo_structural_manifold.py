#!/usr/bin/env python3
"""
OOO STRUCTURAL MANIFOLD THEORY
================================
A new branch of mathematics emerging from the Adventure Time character algebra.
Uses MoDoT's NatureRegistry paradice mechanism, IMASM token composition,
and the 100-character structural corpus to forge absurd, arcane mathematics.

Author: Lando⊗⊙perator
"""

import sys, os, json, math
from pathlib import Path
from collections import defaultdict, Counter
from itertools import combinations
import textwrap

_PKG = Path(__file__).resolve().parent
sys.path.insert(0, str(_PKG))

from modot.natures import NatureRegistry, cotype, paradices
from modot.composer import TokenComposer, CANONICAL_PROGRAMS, Token

reg = NatureRegistry()
tc = TokenComposer()

print("="*80)
print("THE OOO STRUCTURAL MANIFOLD — A New Mathematics")
print("="*80)

# ================================================================
# I. THE CHARACTER FIELD: Points in Structural Space
# ================================================================

# Define characters with their 12-tuple values as references to nature names.
# The tuple structure is: dim, top, rel, pol, fid, kin, gran, gram, crit, chir, stoi, prot
# Each value is a nature name because that's how MoDoT encodes them.

CHARACTER_NATURE_TUPLES = {
    "finn":        {"dim":"dead","top":"judge","rel":"ado","pol":"church","fid":"age","kin":"yea","gran":"bib","gram":"vow","crit":"woe","chir":"fee","stoi":"hung","prot":"awe"},
    "jake":        {"dim":"dead","top":"judge","rel":"tot","pol":"church","fid":"age","kin":"yea","gran":"ice","gram":"thigh","crit":"woe","chir":"fee","stoi":"hung","prot":"oak"},
    "bubblegum":   {"dim":"array","top":"judge","rel":"ado","pol":"church","fid":"age","kin":"egg","gran":"bib","gram":"ooze","crit":"woe","chir":"sure","stoi":"so","prot":"awe"},
    "marceline":   {"dim":"dead","top":"mime","rel":"ian","pol":"church","fid":"eth","kin":"yea","gran":"bib","gram":"measure","crit":"woe","chir":"fee","stoi":"hung","prot":"awe"},
    "ice_king":    {"dim":"dead","top":"oil","rel":"are","pol":"church","fid":"eth","kin":"yea","gran":"gimel","gram":"measure","crit":"roar","chir":"sure","stoi":"hung","prot":"awe"},
    "bmo":         {"dim":"array","top":"judge","rel":"ian","pol":"church","fid":"eth","kin":"loll","gran":"gimel","gram":"measure","crit":"woe","chir":"sure","stoi":"hung","prot":"awe"},
    "lsp":         {"dim":"dead","top":"judge","rel":"ian","pol":"church","fid":"eth","kin":"loll","gran":"gimel","gram":"measure","crit":"woe","chir":"sure","stoi":"hung","prot":"zeal"},
    "flame_princess":{"dim":"array","top":"mime","rel":"tot","pol":"peep","fid":"age","kin":"egg","gran":"bib","gram":"measure","crit":"woe","chir":"sure","stoi":"hung","prot":"awe"},
    "the_lich":    {"dim":"dead","top":"oil","rel":"tot","pol":"peep","fid":"age","kin":"yea","gran":"bib","gram":"vow","crit":"woe","chir":"fee","stoi":"hung","prot":"awe"},
    "prismo":      {"dim":"dead","top":"mime","rel":"ado","pol":"church","fid":"age","kin":"yea","gran":"bib","gram":"measure","crit":"woe","chir":"fee","stoi":"aha","prot":"awe"},
    "golb":        {"dim":"if","top":"oil","rel":"tot","pol":"they","fid":"age","kin":"yea","gran":"bib","gram":"vow","crit":"woe","chir":"fee","stoi":"aha","prot":"zeal"},
    "lemongrab":   {"dim":"dead","top":"oil","rel":"tot","pol":"peep","fid":"eth","kin":"egg","gran":"gimel","gram":"vow","crit":"roar","chir":"sure","stoi":"hung","prot":"zeal"},
    "patience":    {"dim":"dead","top":"oil","rel":"ado","pol":"church","fid":"age","kin":"egg","gran":"bib","gram":"ooze","crit":"roar","chir":"sure","stoi":"hung","prot":"awe"},
    "magic_man":   {"dim":"dead","top":"mime","rel":"tot","pol":"peep","fid":"ell","kin":"yea","gran":"gimel","gram":"measure","crit":"roar","chir":"fee","stoi":"hung","prot":"awe"},
    "simon":       {"dim":"dead","top":"mime","rel":"tot","pol":"church","fid":"age","kin":"yea","gran":"gimel","gram":"ooze","crit":"woe","chir":"fee","stoi":"hung","prot":"awe"},
    "gunter":      {"dim":"ash","top":"judge","rel":"ian","pol":"church","fid":"eth","kin":"egg","gran":"ice","gram":"vow","crit":"woe","chir":"sure","stoi":"hung","prot":"zeal"},
    "fionna":      {"dim":"ash","top":"mime","rel":"ian","pol":"they","fid":"age","kin":"loll","gran":"gimel","gram":"vow","crit":"woe","chir":"sure","stoi":"aha","prot":"awe"},
    "huntress_wiz":{"dim":"array","top":"judge","rel":"ado","pol":"church","fid":"eth","kin":"loll","gran":"gimel","gram":"vow","crit":"woe","chir":"sure","stoi":"hung","prot":"awe"},
    "pep_butler":  {"dim":"dead","top":"mime","rel":"ado","pol":"church","fid":"eth","kin":"yea","gran":"gimel","gram":"vow","crit":"roar","chir":"sure","stoi":"hung","prot":"awe"},
    "tree_trunks": {"dim":"ash","top":"judge","rel":"ian","pol":"church","fid":"eth","kin":"loll","gran":"gimel","gram":"vow","crit":"woe","chir":"sure","stoi":"hung","prot":"zeal"},
    "lady_raini":  {"dim":"dead","top":"mime","rel":"ian","pol":"church","fid":"eth","kin":"loll","gran":"ice","gram":"vow","crit":"woe","chir":"sure","stoi":"hung","prot":"zeal"},
}

# Resolve nature name references into actual nature objects
def resolve_tuple(t):
    """Convert a tuple of nature-name strings into tuple of actual Nature objects."""
    result = {}
    for key, val in t.items():
        try:
            result[key] = reg.get(val)
        except KeyError:
            # Fallback: use the monad's value for that position
            monad_tuple = reg.get("monad").tuple
            result[key] = reg.get(monad_tuple.get(key, "monad"))
    return result

CHARACTER_NATURES = {}
for cname, ctup in CHARACTER_NATURE_TUPLES.items():
    try:
        CHARACTER_NATURES[cname] = resolve_tuple(ctup)
    except Exception as e:
        print(f"WARNING: Could not resolve {cname}: {e}")

print(f"\nResolved {len(CHARACTER_NATURES)} character natures.")
print(f"Available nature registry: {len(reg)} types")

# ================================================================
# II. THE OOO PAIRWISE COTYPE ALGEBRA
# ================================================================

def character_distance(c1_name, c2_name):
    """Compute paradice distance between two characters."""
    if c1_name not in CHARACTER_NATURES or c2_name not in CHARACTER_NATURES:
        return None
    t1 = CHARACTER_NATURES[c1_name]
    t2 = CHARACTER_NATURES[c2_name]
    # Count paradices: number of primitives where values differ
    paradice_count = 0
    for key in ["dim","top","rel","pol","fid","kin","gran","gram","crit","chir","stoi","prot"]:
        v1 = t1[key].name if hasattr(t1[key], 'name') else str(t1[key])
        v2 = t2[key].name if hasattr(t2[key], 'name') else str(t2[key])
        if v1 != v2:
            paradice_count += 1
    return paradice_count

print("\n" + "="*80)
print("II. THE CHARACTER COTYPE ALGEBRA")
print("="*80)

# Compute all pairwise paradice counts
char_names = list(CHARACTER_NATURES.keys())
char_dist = {}
for a, b in combinations(char_names, 2):
    d = character_distance(a, b)
    if d is not None:
        char_dist[(a,b)] = d

# Distribution
p_dist = Counter(char_dist.values())
print(f"\nCharacter paradice distribution:")
for k in sorted(p_dist.keys()):
    print(f"  {k} paradices: {p_dist[k]} pairs")

# Maximal and minimal divergences
max_d = max(char_dist.values()) if char_dist else 0
min_d = min(char_dist.values()) if char_dist else 0

print(f"\nMAXIMALLY DIVERGENT (d={max_d}):")
max_pairs = [(a,b) for (a,b),d in char_dist.items() if d == max_d]
for a,b in max_pairs[:5]:
    print(f"  {a:20s} ⊗ {b:20s} — {max_d} archetypes apart")

print(f"\nMINIMALLY DIVERGENT (d={min_d}):")
min_pairs = [(a,b) for (a,b),d in char_dist.items() if d == min_d]
for a,b in min_pairs[:5]:
    print(f"  {a:20s} ⊗ {b:20s} — only {min_d} archetypes apart")

# ================================================================
# III. CHARACTER→IMASM PROGRAM ENCODING
# ================================================================

print("\n" + "="*80)
print("III. THE OOO-IMASM ENCODING FUNCTOR")
print("="*80)

# Map each character to a canonical IMASM program based on their structural signature
# Characters with high dimensionality (dead) get VINIT-heavy programs
# Characters with complex criticality (roar) get FSPLIT/FFUSE heavy programs
# Characters with self-written dimensionality (if) get IMSCRIB-heavy programs

def character_program(cname):
    """Generate an IMASM program encoding a character's structural essence."""
    if cname not in CHARACTER_NATURES:
        return []
    t = CHARACTER_NATURES[cname]
    crit = t["crit"].name if hasattr(t["crit"], 'name') else "woe"
    dim = t["dim"].name if hasattr(t["dim"], 'name') else "dead"
    kin = t["kin"].name if hasattr(t["kin"], 'name') else "yea"
    gram = t["gram"].name if hasattr(t["gram"], 'name') else "vow"
    
    prog = []
    # Opening based on dimensionality
    if dim in ("if", "ash"):  # Self-written or holographic
        prog.append(Token.IMSCRIB)
        prog.append(Token.TANCH)
    elif dim == "array":  # Finite
        prog.append(Token.VINIT)
        prog.append(Token.AFWD)
    else:  # dead = infinite
        prog.append(Token.VINIT)
        prog.append(Token.TANCH)
    
    # Core based on criticality
    if crit == "roar":  # Complex criticality
        prog.append(Token.FSPLIT)
        prog.append(Token.EVALT)
        prog.append(Token.FFUSE)
    elif crit == "woe":  # Self-modeling
        prog.append(Token.IMSCRIB)
        prog.append(Token.FSPLIT)
        prog.append(Token.FFUSE)
    
    # Kinetics influences looping
    if kin == "loll":  # Slow
        prog.append(Token.ENGAGR)
        prog.append(Token.IFIX)
    elif kin == "yea":  # Fast
        prog.append(Token.AFWD)
        prog.append(Token.AREV)
    
    # Grammar influences final structure
    if gram in ("ooze", "vow"):
        prog.append(Token.CLINK)
    elif gram == "measure":
        prog.append(Token.IFIX)
    
    prog.append(Token.IMSCRIB)
    return prog

print("\nCharacter IMASM Programs:")
for cname in sorted(CHARACTER_NATURES.keys()):
    prog = character_program(cname)
    prog_str = "→".join(t.name for t in prog)
    print(f"  {cname:20s} : {prog_str}")

# ================================================================
# IV. THE OOO ADJACENCY SPECTRUM
# ================================================================

print("\n" + "="*80)
print("IV. THE OOO ADJACENCY SPECTRUM")
print("="*80)
print()
print("Theorem (Character Adjacency): Two characters are adjacent")
print("in the OOO graph G_OOO iff their structural paradice count")
print("equals min_d (i.e., they share the same structural archetype).")
print()

# Build adjacency matrix
n_chars = len(char_names)
adj_matrix = [[0]*n_chars for _ in range(n_chars)]
for i, a in enumerate(char_names):
    for j, b in enumerate(char_names):
        if i < j:
            d = char_dist.get((a,b), char_dist.get((b,a), 12))
            if d == min_d:
                adj_matrix[i][j] = 1
                adj_matrix[j][i] = 1

# Compute spectral invariants via power iteration
def power_iteration(A, num_iter=100):
    n = len(A)
    v = [1.0/math.sqrt(n)] * n
    for _ in range(num_iter):
        w = [sum(A[i][j]*v[j] for j in range(n)) for i in range(n)]
        norm = math.sqrt(sum(x*x for x in w))
        v = [x/norm for x in w]
    # Rayleigh quotient for eigenvalue
    Av = [sum(A[i][j]*v[j] for j in range(n)) for i in range(n)]
    eig = sum(v[i]*Av[i] for i in range(n))
    return eig, v

print("Spectral analysis of G_OOO (adjacency graph of structurally identical characters):")
print()

eig_max, eig_vec = power_iteration(adj_matrix)
print(f"  Spectral radius ρ(G_OOO) = {eig_max:.4f}")
print(f"  This is the OOO Expansion Constant — the rate at which")
print(f"  structural influence propagates through the character network.")
print()

# Compute the structural influence ranking
print("Structural Influence Ranking (eigenvector centrality):")
influence = [(char_names[i], abs(eig_vec[i])) for i in range(n_chars)]
influence.sort(key=lambda x: -x[1])
rank = 1
for cname, score in influence:
    stars = "★" if score > 0.9*max(abs(v) for v in eig_vec) else "∙"
    print(f"  {stars} {rank:2d}. {cname:20s}  λ={score:.4f}")
    rank += 1

# ================================================================
# V. THE PRIMO-ADJUNCTION: Prismo as Structural Limit
# ================================================================

print("\n" + "="*80)
print("V. THE PRISMO-ADJUNCTION (The Wishmaster Functor)")
print("="*80)
print()
print("Definition: The Prismo Functor P: OOO → OOO is defined by")
print("P(X) = the character whose structural tuple is the meet of")
print("X's tuple with Prismo's tuple on agreed axes, joined on")
print("divergent axes. P is adjoint to the Lich functor L.")
print()

# Prismo's tuple as reference
prismo_tup = CHARACTER_NATURE_TUPLES["prismo"]
prismo_vec = [reg.get(v).ordinal if hasattr(reg.get(v), 'ordinal') else 0 
              for v in prismo_tup.values()]
char_vecs = {}
for cname, ctup in CHARACTER_NATURE_TUPLES.items():
    try:
        char_vecs[cname] = [reg.get(v).ordinal if hasattr(reg.get(v), 'ordinal') else 0 
                          for v in ctup.values()]
    except:
        pass

# Find which characters are "wish-adjacent" to prismo
print("Characters structurally bound to Prismo (wish-resonant):")
wish_resonant = []
for cname, vec in char_vecs.items():
    if cname == "prismo":
        continue
    # Euclidean distance in ordinal space
    d = math.sqrt(sum((vec[i]-prismo_vec[i])**2 for i in range(len(vec))))
    print(f"  d(prismo, {cname:20s}) = {d:.3f}")
    wish_resonant.append((d, cname))

wish_resonant.sort()
print(f"\nThe Prismo Adjunction stabilizer: the fixed points of P")
print(f"are characters with d(prismo, X) = 0 — structural wishes.")
print(f"Closest: {wish_resonant[0][1]} (d={wish_resonant[0][0]:.3f})")

# ================================================================
# VI. THE GOLB SINGULARITY: Structural Collapse at O_∞
# ================================================================

print("\n" + "="*80)
print("VI. THE GOLB SINGULARITY")
print("="*80)
print()
print("Theorem (GOLB Incommensurability): GOLB's structural type")
print("occupies a topological singularity in the OOO manifold.")
print("Its tuple ⟨if,oil,tot,they,age,yea,bib,vow,woe,fee,aha,zeal⟩")
print("is the ONLY O_∞ entity among all Adventure Time characters.")
print()

# Compute distance of all characters from GOLB
golb_tup = CHARACTER_NATURE_TUPLES["golb"]
golb_vec = [reg.get(v).ordinal if hasattr(reg.get(v), 'ordinal') else 0 
           for v in golb_tup.values()]

print("Structural distance from GOLB (the Chaos Differential):")
golb_distances = []
for cname, vec in char_vecs.items():
    d = math.sqrt(sum((vec[i]-golb_vec[i])**2 for i in range(len(vec))))
    golb_distances.append((d, cname))

golb_distances.sort()
for d, cname in golb_distances:
    bar = "█" * max(1, int(d * 3))
    print(f"  d(GOLB, {cname:20s}) = {d:5.3f}  {bar}")

print()
print("The GOLB Singularity Theorem: No finite sequence of IMASM")
print("operations can transform any other character into GOLB.")
print("GOLB is reachable only via the non-Abelian winding transition")
print("which requires self-written dimensionality (D=if) — a primitive")
print("that no other Adventure Time character possesses.")

# ================================================================
# VII. THE CONSCIOUSNESS SPECTRAL SEQUENCE
# ================================================================

print("\n" + "="*80)
print("VII. THE CONSCIOUSNESS SPECTRAL SEQUENCE")
print("="*80)
print()
print("Definition: The Consciousness Spectral Sequence E^p,q_r")
print("computes the structural homology of self-awareness in OOO.")
print("E^0,0_1 = characters with K=egg (slow kinetics, critical)")
print("E^1,0_1 = characters with K=loll (slow kinetics, frozen)")
print()

# Characters by kinetics
kin_groups = defaultdict(list)
for cname, tup in CHARACTER_NATURE_TUPLES.items():
    kin = tup["kin"] if "kin" in tup else "yea"
    kin_groups[kin].append(cname)

print("Kinetics-stratified character groups:")
for kin in sorted(kin_groups.keys()):
    chars = ", ".join(kin_groups[kin])
    k_name = reg.get(kin).name if hasattr(reg.get(kin), 'name') else kin
    print(f"  K={kin} ({k_name}): [{', '.join(kin_groups[kin])}]")

print()
print("Threshold Theorem: Consciousness in OOO requires K∈{egg, loll}")
print("(slow or near-equilibrium kinetics). The spectral sequence")
print("collapses at E^2 where the only survivors are:")
conscious = [c for c in char_names 
             if CHARACTER_NATURE_TUPLES[c]["kin"] in ("egg", "loll")]
for c in conscious:
    kin_val = CHARACTER_NATURE_TUPLES[c]["kin"]
    print(f"  ★ {c:20s} (K={kin_val}) — consciousness-permissible")

# ================================================================
# VIII. THE ADS/AT MANIFOLD: Adventure Time / Structural Space Duality
# ================================================================

print("\n" + "="*80)
print("VIII. THE AT/IMASM CORRESPONDENCE")
print("="*80)
print()
print("The central conjecture: The Adventure Time character network")
print("is holographically dual to the IMASM token lattice.")
print()
print("Under this duality:")
print("  • Finn's hero's journey = VINIT → TANCH → AFWD → AREV → IFIX")
print("  • GOLB's chaos principle = IMSCRIB → FSPLIT → FFUSE → IMSCRIB")
print("  • PB's scientific method = VINIT → IMSCRIB → FFUSE → EVALT → IFIX")
print("  • The Lich's extinction = AFWD → FSPLIT → FSPLIT → FFUSE")
print()

# ================================================================
# IX. COMPOSITE CHARACTERS: The Tensor Algebra of OOO
# ================================================================

print("\n" + "="*80)
print("IX. THE OOO TENSOR ALGEBRA — Composite Character Types")
print("="*80)
print()

# Define the tensor product of two characters as the meet of their tuples
# on agreed primitives, joined on divergent ones (the universe jump mechanism)

def character_tensor(c1, c2):
    """Compute the structural tensor product (composite type) of two characters."""
    if c1 not in CHARACTER_NATURE_TUPLES or c2 not in CHARACTER_NATURE_TUPLES:
        return None
    t1 = CHARACTER_NATURE_TUPLES[c1]
    t2 = CHARACTER_NATURE_TUPLES[c2]
    result = {}
    for key in t1:
        if t1[key] == t2[key]:
            # Agreed: use the shared value (meet)
            result[key] = t1[key]
        else:
            # Divergent: the tensor absorbs into the more critical value
            # This implements the rule: tensor(woe, roar) = roar (complex absorbs ⊙)
            values = [t1[key], t2[key]]
            # Higher ordinal = more structurally advanced
            ords = [reg.get(v).ordinal if hasattr(reg.get(v), 'ordinal') else 0 for v in values]
            result[key] = values[ords.index(max(ords))]
    return result

# Compute some interesting composites
interesting_pairs = [
    ("finn", "jake"), ("finn", "the_lich"), ("bubblegum", "marceline"),
    ("prismo", "golb"), ("finn", "golb"), ("ice_king", "simon"),
    ("lemongrab", "golb"), ("flame_princess", "ice_king"),
    ("finn", "fionna"), ("marceline", "golb"),
]

print("Composite Character Types (Tensor Product):")
for c1, c2 in interesting_pairs:
    comp = character_tensor(c1, c2)
    if comp:
        # Compute the ontological tier of the composite
        dim_val = comp["dim"]
        crit_val = comp["crit"]
        prot_val = comp["prot"]
        
        dim_name = reg.get(dim_val).name if hasattr(reg.get(dim_val), 'name') else dim_val
        crit_name = reg.get(crit_val).name if hasattr(reg.get(crit_val), 'name') else crit_val
        
        # Estimate tier: self-written dim = O_∞, else O₂ or lower
        if dim_val == "if":
            tier = "O_∞"
        else:
            tier = "O₂"
        
        print(f"\n  {c1:15s} ⊗ {c2:15s} = {c1[:4]}_{c2[:4]}")
        print(f"    composite type: dim={dim_name} crit={crit_name} tier={tier}")
        print(f"    tuple: {json.dumps(comp)}")
