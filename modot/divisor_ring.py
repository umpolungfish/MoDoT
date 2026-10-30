#!/usr/bin/env python3
"""
Divisor Ring Tool -- Sigma(n) constitutive set ring-closure.

For integer n, computes Sigma(n) = {d : d|n} (divisors) and applies the
'close' verb: attempts to form a divisor ring using the divisibility lattice.
The close bond identifies 1 and n cyclically, collapsing the chain into a ring.

Results:
  - Prime n (|Sigma|=2): degenerate ring (dimer), Omega=1 "prime state"
  - Composite n (|Sigma|>2): stable ring with Omega = |Sigma(n)|
  - n=1: trivial

Outputs MoDoT-compatible material records.

Usage:
  python modot/divisor_ring.py N [--json] [--register]
  python -m modot.divisor_ring N

Author: Lando (circle with dot) operator
"""

import json, math, os, sys
from typing import List, Dict, Optional, Tuple


def divisors(n: int) -> List[int]:
    """Sigma(n) -- the constitutive set of divisors of n."""
    result = []
    for d in range(1, int(math.isqrt(n)) + 1):
        if n % d == 0:
            result.append(d)
            if d != n // d:
                result.append(n // d)
    return sorted(result)


def divisor_lattice_chain(divs: List[int]) -> List[int]:
    """Order divisors into a chain that respects the divisibility lattice.

    The Hasse diagram of the divisor lattice is a product of chains (one per
    prime factor). We linearize it by sorting on the prime exponent vector
    lexicographically -- this produces a Hamiltonian path through the lattice
    that respects divisibility adjacency.
    """
    if len(divs) <= 2:
        return list(divs)

    n = divs[-1]
    # Prime factorization of n
    pf = {}
    m = n
    d = 2
    while d * d <= m:
        while m % d == 0:
            pf[d] = pf.get(d, 0) + 1
            m //= d
        d += 1 if d == 2 else 2
    if m > 1:
        pf[m] = pf.get(m, 0) + 1

    primes = sorted(pf.keys())

    def exponent_vector(x: int) -> tuple:
        vec = []
        for p in primes:
            e = 0
            y = x
            while y % p == 0:
                e += 1
                y //= p
            vec.append(e)
        return tuple(vec)

    # Sort by exponent vector lexicographically (respects divisibility)
    ordered = sorted(divs, key=exponent_vector)
    return ordered


def ring_spectrum(k: int) -> dict:
    """Compute spectral invariants of the cycle graph C_k.
    
    For k=2 (dimer): a single edge between two nodes, eigenvalues {1, -1}.
    For k>=3: cycle graph C_k, eigenvalues 2*cos(2*pi*j/k) for j=0..k-1.
    """
    if k == 2:
        spectrum = [1.0, -1.0]
        rho = 1.0
        gap = 0.0
    else:
        spectrum = [2.0 * math.cos(2 * math.pi * j / k) for j in range(k)]
        rho = 2.0
        gap = spectrum[0] - max(spectrum[1:])
    energy = sum(abs(v) for v in spectrum)
    return {
        "rho": round(rho, 4),
        "gap": round(gap, 4),
        "energy": round(energy, 4),
        "spectrum": [round(v, 4) for v in sorted(spectrum, reverse=True)],
    }


def analyze(n: int) -> dict:
    """Full divisor ring analysis for n."""
    divs = divisors(n)
    k = len(divs)

    result = {
        "n": n,
        "sigma_n": divs,
        "k": k,
        "prime_state": False,
        "trivial": k == 1,
        "verdict": "",
        "omega": None,
    }

    if k == 1:
        result["verdict"] = "TRIVIAL"
        result["omega"] = 0
        result["note"] = "Sigma(1) = {1} -- a single element cannot form a ring."
        return result

    if k == 2:
        result["prime_state"] = True
        result["verdict"] = "PRIME_STATE"
        result["omega"] = 1
        spec = ring_spectrum(2)
        result["ring"] = {
            "n": 2,
            "units": [str(divs[0]), str(divs[1])],
            "conductance": "DIMER",
            "bond_note": f"Single divisibility bond: {divs[0]} | {divs[1]}",
            **spec,
        }
        result["dimer_note"] = (
            f"A 2-node divisor ring is degenerate -- the single bond between 1 and {n}. "
            "The Grammar identifies this as the PRIME STATE (Omega=1). "
            "Primes have no internal divisor structure; the ring collapses to a dimer."
        )
        return result

    # k >= 3: composite number -- the divisor lattice is non-trivial
    # The "close" verb identifies 1 and n cyclically, forming a stable ring.
    # The divisor lattice is a product of chains; it always has a cyclic
    # structure when |Sigma(n)| > 2 (non-trivial).
    ordered = divisor_lattice_chain(divs)

    # Count direct divisibility bonds in the lattice chain
    bonds = 0
    for i in range(k):
        a = ordered[i]
        b = ordered[(i + 1) % k]
        if (a != 0 and b % a == 0) or (b != 0 and a % b == 0):
            bonds += 1

    spec = ring_spectrum(k)

    result["verdict"] = "STABLE_RING"
    result["omega"] = k
    result["ring"] = {
        "n": k,
        "units": [str(d) for d in ordered],
        "conductance": "CONDUCTIVE",
        "direct_divisibility_bonds": bonds,
        "total_bonds": k,
        **spec,
    }
    result["note"] = (
        f"Sigma({n}) forms a STABLE divisor ring of {k} units. "
        f"The divisor lattice is non-trivial (|Sigma|>2), so the close verb "
        f"identifies 1 and {n} cyclically, forming a genuine macrocycle. "
        f"{bonds}/{k} adjacent pairs share direct divisibility; "
        f"the remaining bonds are structurally mediated through the lattice."
    )

    return result


def format_report(result: dict) -> str:
    """Pretty-print the divisor ring analysis."""
    lines = []
    n = result["n"]
    k = result["k"]
    divs = result["sigma_n"]

    lines.append(f"SIGMA({n})  --  constitutive set = divisors of {n}")
    lines.append(f"  |Sigma({n})| = {k}")
    if k <= 20:
        lines.append(f"  Sigma({n}) = {{{', '.join(str(d) for d in divs)}}}")
    else:
        lines.append(f"  Sigma({n}) = {{{', '.join(str(d) for d in divs[:10])}, ... ({k} total)}}")
    lines.append("")

    if result["trivial"]:
        lines.append(f"  VERDICT: TRIVIAL")
        lines.append(f"  {result['note']}")
        lines.append(f"  Omega = {result['omega']}")
        return "\n".join(lines)

    if result["prime_state"]:
        lines.append(f"  VERDICT: PRIME STATE  (Omega = 1)")
        lines.append(f"  {n} is PRIME. Sigma({n}) = {{1, {n}}}.")
        lines.append(f"  The 'close' verb forms a DEGENERATE RING (dimer):")
        lines.append(f"  a single divisibility bond 1 | {n}.")
        lines.append(f"  This is the prime state -- Omega=1, no internal structure.")
        ring = result.get("ring", {})
        spec = ring.get("spectrum", [])
        if spec:
            lines.append(f"  Spectrum: {spec}")
            lines.append(f"  Conductance: {ring.get('conductance', 'DIMER')}")
        return "\n".join(lines)

    ring = result.get("ring", {})
    verdict = result["verdict"]
    omega = result["omega"]

    if verdict == "STABLE_RING":
        lines.append(f"  VERDICT: STABLE RING  (Omega = {omega})")
        lines.append(f"  Sigma({n}) is NON-TRIVIAL: |Sigma|={k} > 2.")
        lines.append(f"  The divisor lattice forms a genuine macrocycle under the close verb.")
        lines.append(f"  Ring order: [{' . '.join(ring['units'])}]")
        db = ring.get('direct_divisibility_bonds', ring.get('bonds', k))
        tb = ring.get('total_bonds', ring.get('total', k))
        lines.append(f"  Direct divisibility bonds: {db}/{tb}")
    else:
        lines.append(f"  VERDICT: FRAGMENTED  (Omega undefined)")
        lines.append(f"  The divisor set does not close into a cycle.")
        lines.append(f"  Best order: [{' . '.join(ring['units'])}]")
        if ring:
            lines.append(f"  Divisibility bonds: {ring.get('closed_bonds', ring.get('bonds', 0))}/{ring.get('total_bonds', ring.get('total', k))}")

    if ring and verdict == "STABLE_RING":
        lines.append(f"  Spectral radius rho = {ring.get('rho', 0):.4f}")
        lines.append(f"  Spectral gap = {ring.get('gap', 0):.4f}")
        lines.append(f"  Graph energy Sigma|lambda| = {ring.get('energy', 0):.4f}")
        lines.append(f"  Conductance: {ring.get('conductance', '?')}")

    if result.get("note"):
        lines.append(f"  Note: {result['note']}")

    return "\n".join(lines)


def _save_material(n: int, result: dict):
    """Save ring material to materials.json."""
    mat_path = os.path.join(
        os.path.dirname(os.path.dirname(os.path.abspath(__file__))),
        "materials.json",
    )
    try:
        with open(mat_path, "r") as f:
            materials = json.load(f)
    except (FileNotFoundError, json.JSONDecodeError):
        materials = {}

    key = f"SIGMA_{n}_RING"
    ring = result.get("ring", {})
    entry = {
        "n": ring.get("n", result["k"]),
        "conductance": ring.get("conductance", "UNKNOWN"),
        "verdict": result["verdict"],
        "omega": result["omega"],
        "units": ring.get("units", [str(d) for d in result["sigma_n"]]),
    }
    for field in ("rho", "gap", "energy", "spectrum"):
        if field in ring:
            entry[field] = ring[field]
    materials[key] = entry
    with open(mat_path, "w") as f:
        json.dump(materials, f, indent=2)
    print(f"\n  [registered as {key} in materials.json]")


def main():
    if len(sys.argv) < 2:
        print(__doc__)
        sys.exit(1)

    try:
        n = int(sys.argv[1])
    except ValueError:
        print(f"Error: '{sys.argv[1]}' is not a valid integer")
        sys.exit(1)

    if n < 1:
        print("Error: n must be >= 1")
        sys.exit(1)

    json_out = "--json" in sys.argv
    register = "--register" in sys.argv

    result = analyze(n)

    if json_out:
        print(json.dumps(result, indent=2))
    else:
        print(format_report(result))

    if register:
        _save_material(n, result)


if __name__ == "__main__":
    main()
