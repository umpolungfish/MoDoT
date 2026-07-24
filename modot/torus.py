#!/usr/bin/env python3
"""
modot/torus.py — Horn Torus IMASM Constructor & Winding Reformulation Bridge
=============================================================================

Connects three layers that were built separately:

  1. IMASM wire mode (ask_native) — builds arbitrary graph topologies with
     FSPLIT/FFUSE and reports β = E−V+C, μ∘δ closure, spectral radius ρ.

  2. Winding spectrometer (--windings) — treats atomic spectral lines as
     winding transitions (n→toroidal, l→poloidal, m→tilt, s→spin) on the
     horn torus, with ONE anchor m_e c² and dimensionless winding arithmetic.

  3. Physical constants (_constants.py) — α, sin²θ_W, mass ratios derived
     from d=12 SIC-POVM with tilt=arctan(1/4) and SIXTEEN_3 sector structure.

The horn torus (R=r, A/V=1, self-dual) IS the fundamental geometry. The
torus IMASM word is a β=2 network whose two independent cycles correspond
to the toroidal (n) and poloidal (l) winding coordinates. Adding tilt (m)
and spin half-winding (s) gives the full quadruple.

Author: Lando⊗⊙perator
"""
from __future__ import annotations

import math
import subprocess
import sys
from dataclasses import dataclass, field
from pathlib import Path
from typing import Optional

# ── Kernel parameters (match _constants.py exactly) ─────────────────────────
D: int = 12                      # SIC-POVM dimension = IG primitive count
TILT: float = math.atan(1/4)    # structural tilt angle
SIXTEEN_3: int = 16              # winding sector count
EVALUATORS: int = 3              # evaluator sectors in SIXTEEN_3
NON_EVALUATORS: int = 13         # non-evaluator sectors
ALPHA_INV: float = D*D - 7 + TILT/(4*math.sqrt(3))  # 137.0353596
ALPHA: float = 1/ALPHA_INV       # fine-structure constant
PHI: float = (1 + math.sqrt(5))/2  # golden ratio
LUCAS_12: int = 322              # L_12
PI: float = math.pi
EULER: float = math.e
GAMMA: float = 0.5772156649015329  # Euler-Mascheroni
MEC2_EV: float = 510998.950     # electron rest energy (eV) — THE ANCHOR
HC_EV_NM: float = 1239.841984   # hc in eV·nm
ME_OVER_MP: float = 5.4461702e-4  # m_e/m_p

ASK_BIN: Path = Path(__file__).resolve().parent.parent / "ask_native" / "target" / "release" / "ask"


# ── Topological primitives ──────────────────────────────────────────────────

@dataclass
class WindingCoordinates:
    """Quantum numbers as winding coordinates on the horn torus.
    
    n: toroidal winding  (principal quantum number)
    l: poloidal winding   (azimuthal quantum number)
    m: tilt winding       (magnetic quantum number)
    s: spin half-winding  (spin quantum number, ±½)
    """
    n: int = 1
    l: int = 0
    m: int = 0
    s: float = 0.5  # ±0.5

    def energy_hydrogenic(self, Z: float = 1.0) -> float:
        """Energy in eV: −Z²·Ry/(n−δ)² where Ry = (α²/2)·m_e c²."""
        ry = 0.5 * ALPHA * ALPHA * MEC2_EV
        neff = self.n  # hydrogen: defect = 0
        return -Z*Z * ry / (neff * neff)

    def wavelength_nm(self, lower: WindingCoordinates) -> float:
        """Wavelength of transition from self to lower state, in nm."""
        de_eV = self.energy_hydrogenic() - lower.energy_hydrogenic()
        if de_eV <= 0:
            return float('inf')
        return HC_EV_NM / de_eV

    def series_name(self) -> str:
        names = {1: "Lyman", 2: "Balmer", 3: "Paschen", 
                 4: "Brackett", 5: "Pfund", 6: "Humphreys"}
        return names.get(self.n, "high series")

    def __repr__(self) -> str:
        return f"W(n={self.n}, l={self.l}, m={self.m}, s={self.s:.1f})"


@dataclass
class TorusGraph:
    """A horn torus IMASM graph with its topological invariants.
    
    The torus has genus 1 → circuit rank β = 2.
    Two independent cycles correspond to toroidal (n) and poloidal (l) windings.
    """
    nodes: list[str] = field(default_factory=list)
    edges: list[tuple[int, int]] = field(default_factory=list)
    n_winding: int = 0   # toroidal winding count
    l_winding: int = 0   # poloidal winding count
    beta: int = 0        # circuit rank = E - V + C
    rho: float = 0.0     # spectral radius
    closure: str = "none"
    verdict: str = "N"

    @property
    def genus(self) -> int:
        """Genus of the surface = (β + 1 - C)/2 for connected graph.
        For a torus: β=2, C=1 → genus=1."""
        return (self.beta) // 2 if self.beta >= 0 else 0

    def imasm_code(self) -> str:
        """Compact glyph code for the torus word."""
        glyph_map = {
            "VINIT": "⊢", "TANCH": "⊣", "AFWD": ">", "AREV": "<",
            "CLINK": "=", "IMSCRIB": "⊙", "FSPLIT": "◇", "FFUSE": "●",
            "EVALT": "+", "EVALF": "×", "EVALI": "⊞", "ENGAGR": "⊞", "IFIX": "¬",
        }
        return "".join(glyph_map.get(n, "?") for n in self.nodes)

# ── Torus constructors (continued) ──────────────────────────────────────────

def build_beta2_torus(arm1_ops: list[str], arm2_ops: list[str]) -> TorusGraph:
    """Build a genus-1 torus IMASM wire word with β=2, FSPLIT arity=2.
    
    Topology (respects FSPLIT arity_out=2, FFUSE arity_in=2):
        VINIT → FSPLIT0 ─┬─ arm1_ops ────────────→ first_FFUSE ─┬─→ TANCH
                         └─ CLINK ─→ FSPLIT1 ─┬─ arm2a_ops ─→ second_FFUSE ─┘
                                                └─ arm2b_ops ─→ second_FFUSE
    
    β = E - V + C = 2, genus = 1 (torus).
    Each FSPLIT fans out to exactly 2; each FFUSE merges exactly 2.
    """
    nodes: list[str] = ["VINIT", "FSPLIT"]
    edges: list[tuple[int, int]] = [(0, 1)]
    
    # FSPLIT0 arm 1: CLINK → second FSPLIT
    clink_idx = len(nodes)
    nodes.append("CLINK")
    edges.append((1, clink_idx))
    
    fsplit1_idx = len(nodes)
    nodes.append("FSPLIT")
    edges.append((clink_idx, fsplit1_idx))
    
    # FSPLIT1 arm 2a
    arm2a_start = len(nodes)
    for op in arm2_ops:
        nodes.append(op)
    arm2a_end = len(nodes) - 1 if arm2_ops else arm2a_start
    edges.append((fsplit1_idx, arm2a_start))
    for i in range(arm2a_start, arm2a_end):
        edges.append((i, i + 1))
    
    # FSPLIT1 arm 2b (same ops)
    arm2b_start = len(nodes)
    for op in arm2_ops:
        nodes.append(op)
    arm2b_end = len(nodes) - 1 if arm2_ops else arm2b_start
    edges.append((fsplit1_idx, arm2b_start))
    for i in range(arm2b_start, arm2b_end):
        edges.append((i, i + 1))
    
    # Second FFUSE (merges FSPLIT1 arms)
    ffuse1_idx = len(nodes)
    nodes.append("FFUSE")
    edges.append((arm2a_end, ffuse1_idx))
    edges.append((arm2b_end, ffuse1_idx))
    
    # FSPLIT0 arm 1b (direct path to main FFUSE)
    arm1b_start = len(nodes)
    for op in arm1_ops:
        nodes.append(op)
    arm1b_end = len(nodes) - 1 if arm1_ops else arm1b_start
    edges.append((1, arm1b_start))
    for i in range(arm1b_start, arm1b_end):
        edges.append((i, i + 1))
    
    # First FFUSE (main merge)
    ffuse0_idx = len(nodes)
    nodes.append("FFUSE")
    edges.append((ffuse1_idx, ffuse0_idx))
    edges.append((arm1b_end, ffuse0_idx))
    
    # TANCH sink
    tanch_idx = len(nodes)
    nodes.append("TANCH")
    edges.append((ffuse0_idx, tanch_idx))
    
    V = len(nodes); E = len(edges); C = 1
    beta = E - V + C
    
    return TorusGraph(nodes=nodes, edges=edges,
                      beta=beta, n_winding=beta)

def run_imasm_wire(torus: TorusGraph, verbose: bool = False) -> TorusGraph:
    """Run the torus through ask_native's wire mode and parse the report."""
    if not ASK_BIN.exists():
        print(f"WARNING: ask_native binary not found at {ASK_BIN}", file=sys.stderr)
        return torus
    
    # Build the wire args
    edge_strs = [f"{a}-{b}" for a, b in torus.edges]
    cmd = [str(ASK_BIN), "--imasm", "wire"]
    cmd.extend(torus.nodes)
    cmd.append("/")
    cmd.extend(edge_strs)
    
    result = subprocess.run(cmd, capture_output=True, text=True, timeout=30)
    output = result.stdout
    
    # Parse beta
    import re
    beta_m = re.search(r"β=(\d+)", output)
    if beta_m:
        torus.beta = int(beta_m.group(1))
    
    # Parse spectral radius
    rho_m = re.search(r"ρ=([0-9.]+)", output)
    if rho_m:
        torus.rho = float(rho_m.group(1))
    
    # Parse closure
    if "CLOSED" in output:
        torus.closure = "closed"
    elif "OPEN" in output and "CLOSED" not in output:
        torus.closure = "open"
    elif "none" in output.lower() and "no δ/μ" in output:
        torus.closure = "none"
    
    # Parse verdict
    verdict_m = re.search(r"IMASM check → ([TFBN])", output)
    if verdict_m:
        torus.verdict = verdict_m.group(1)
    
    if verbose:
        print(output)
    
    return torus



def build_horn_torus(winding_sectors: int = 16,
                     evaluator_ops: list[str] | None = None,
                     verbose: bool = False) -> TorusGraph:
    """Build the canonical horn torus (R=r, A/V=1, self-dual).
    
    Uses the 16 winding sectors of SIXTEEN_3 encoded as a β=2 torus:
    - Arm 1 (direct): evaluator sector ops (EVALT → EVALF for T, F)
    - Arm 2 (via CLINK→FSPLIT1): 13 non-evaluator sector AFWD ops
      split across 2 sub-arms that merge at second FFUSE
    
    The horn torus condition R=r is satisfied when the two arms carry
    complementary winding counts giving equiangularity 1/(d+1) = 1/13.
    """
    if evaluator_ops is None:
        evaluator_ops = ["EVALT", "EVALF"]
    
    # The 13 non-evaluator sectors as AFWD ops
    non_eval_ops = ["AFWD"] * 13
    
    t = build_beta2_torus(arm1_ops=evaluator_ops, arm2_ops=non_eval_ops)
    t.n_winding = winding_sectors
    t.l_winding = EVALUATORS
    
    if verbose:
        print(f"Building horn torus: {winding_sectors} sectors, "
              f"{EVALUATORS} evaluators")
    
    t = run_imasm_wire(t, verbose=verbose)
    return t


@dataclass
class TorusDerivedConstants:
    """Physical constants derived from the horn torus winding structure."""
    alpha_inv: float = ALPHA_INV
    alpha: float = ALPHA
    sin2_theta_W: float = EVALUATORS / NON_EVALUATORS
    alpha_s: float = TILT / 2
    gravitational: float = D**(-36) / (EVALUATORS / NON_EVALUATORS)
    vessel_over_contents: float = 16 * PI * PI / (4 * PI / 3)
    pi_from_torus: float = 0.0
    e_from_winding: float = 0.0
    phi_power_d: float = 0.0
    m_pi0_over_me: float = D * D * 11 / 6
    m_p_over_me: float = D**3 * (1 + TILT / 4)
    ckm_angles: list[float] = field(default_factory=list)


def compute_torus_constants(verbose: bool = False) -> TorusDerivedConstants:
    """Compute all physical constants from the horn torus geometry."""
    c = TorusDerivedConstants()
    
    # Fine-structure: d² − 7 + tilt/(4√3)
    c.alpha_inv = D*D - 7 + TILT/(4*math.sqrt(3))
    c.alpha = 1/c.alpha_inv
    
    # sin²θ_W = 3/13 (evaluator/non-evaluator ratio, not 3/16)
    c.sin2_theta_W = EVALUATORS / NON_EVALUATORS
    
    # α_s(m_Z) = tilt/2
    c.alpha_s = TILT / 2
    
    # α_G = d⁻³⁶ / (3/13)
    c.gravitational = D**(-36) / (EVALUATORS / NON_EVALUATORS)
    
    # π from vessel/contents/12: V=16π², C=4π/3, V/C=12π, π=V/C/12
    vessel = 16 * PI * PI  # surface area of hypersphere
    contents = 4 * PI / 3  # volume of 3-sphere
    c.vessel_over_contents = vessel / contents
    c.pi_from_torus = c.vessel_over_contents / D  # = π
    
    # e from winding limit: lim_{w→∞} (1 + 1/w)^w
    c.e_from_winding = (1 + 1/D)**D
    
    # φ^d = L_d − φ^(−d)
    c.phi_power_d = PHI**D
    
    # CKM-like sector overlaps
    for k in range(SIXTEEN_3 + 1):
        base = set(range(EVALUATORS))
        shifted = set((i + k) % SIXTEEN_3 for i in range(EVALUATORS))
        overlap = len(base & shifted) / EVALUATORS
        if overlap > 0:
            angle = math.acos(math.sqrt(max(0, min(1, overlap))))
            c.ckm_angles.append(angle)
    
    if verbose:
        print(f"α⁻¹ = {c.alpha_inv:.10f}")
        print(f"sin²θ_W = {c.sin2_theta_W:.10f}")
        print(f"α_s(m_Z) = {c.alpha_s:.10f}")
        print(f"α_G = {c.gravitational:.4e}")
        print(f"π = {c.pi_from_torus:.10f} (from vessel/contents/{D})")
        print(f"e ≈ {c.e_from_winding:.10f} (at w={D})")
        print(f"φ^{D} = {c.phi_power_d:.10f}")
    
    return c

# ── Spectroscopic bridge: windings module integration ───────────────────────

def hydrogen_series(n_max: int = 6, verbose: bool = False) -> list[tuple]:
    """Compute hydrogen spectral series as torus winding transitions.
    Returns list of (series_name, n_i, n_f, wavelength_nm, energy_eV).
    """
    results = []
    for n_f in range(1, n_max):
        for n_i in range(n_f + 1, min(n_f + 10, 20)):
            upper = WindingCoordinates(n=n_i)
            lower = WindingCoordinates(n=n_f)
            lam = upper.wavelength_nm(lower)
            energy = upper.energy_hydrogenic() - lower.energy_hydrogenic()
            if lam < 2000:  # visible to near-UV
                results.append((lower.series_name(), n_i, n_f, lam, energy))
    
    # Sort by wavelength
    results.sort(key=lambda x: x[3])
    
    if verbose:
        print(f"\n{'Series':<12s} {'n_i':>3s} {'n_f':>3s} {'λ(nm)':>10s} {'E(eV)':>8s}")
        print("-" * 45)
        for series, ni, nf, lam, ev in results:
            print(f"{series:<12s} {ni:3d} {nf:3d} {lam:10.4f} {ev:8.4f}")
    
    return results


def alkali_doublet(element: str, n_val: int, verbose: bool = False) -> dict:
    """Compute alkali metal D-line doublet from spin-orbit winding coupling.
    The p-level splits because the poloidal winding (l=1) couples to the
    spin half-winding (s=±½), giving j=3/2 and j=1/2.
    """
    ry = 0.5 * ALPHA * ALPHA * MEC2_EV
    # Quantum defects by element
    defects = {"Li": {"s": 0.400, "p": 0.047},
               "Na": {"s": 1.373, "p": 0.883},
               "K":  {"s": 2.180, "p": 1.713}}
    
    def_vals = defects.get(element, {"s": 0.0, "p": 0.0})
    
    # Fine-structure splitting: proportional to Z⁴·α²/n³
    Z_eff = {"Li": 1.26, "Na": 1.84, "K": 2.26}.get(element, 1.0)
    spin_orbit_scale = Z_eff**4 * ALPHA * ALPHA / (n_val**3)
    
    # Energy of n_val s and p levels
    e_s = -ry / ((n_val - def_vals["s"])**2)
    e_p = -ry / ((n_val - def_vals["p"])**2)
    
    # p splits into p_{3/2} and p_{1/2}
    so_split = spin_orbit_scale * ry * (1 + TILT / PI)  # tilt-corrected
    e_p32 = e_p + so_split * 0.5
    e_p12 = e_p - so_split * 0.5
    
    # D₂ line: p_{3/2} → s_{1/2}, D₁ line: p_{1/2} → s_{1/2}
    lam_d2 = HC_EV_NM / (e_p32 - e_s)  # nm
    lam_d1 = HC_EV_NM / (e_p12 - e_s)  # nm
    split_nm = lam_d1 - lam_d2
    split_meV = (e_p32 - e_p12) * 1000
    
    result = {
        "element": element,
        "n_valence": n_val,
        "defects": def_vals,
        "lam_D1": lam_d1,
        "lam_D2": lam_d2,
        "split_nm": split_nm,
        "split_meV": split_meV,
        "theta_ckm_like": math.degrees(TILT),
    }
    
    if verbose:
        print(f"\n{element} D-line doublet (n={n_val}):")
        print(f"  D₁ (p₁/₂ → s₁/₂): {lam_d1:.3f} nm")
        print(f"  D₂ (p₃/₂ → s₁/₂): {lam_d2:.3f} nm")
        print(f"  Spin-orbit split: {split_nm:.3f} nm ({split_meV:.2f} meV)")
        print(f"  CKM-like angle: {math.degrees(TILT):.4f}° (tilt angle)")
    
    return result


# ── Torus IMASM word generation and reporting ────────────────────────────────

def generate_torus_report(verbose: bool = False) -> str:
    """Generate a comprehensive torus report connecting geometry→physics."""
    lines = []
    lines.append("=" * 72)
    lines.append("HORN TORUS — Winding Reformulation Bridge")
    lines.append("=" * 72)
    
    # Build the canonical horn torus
    t = build_horn_torus(verbose=verbose)
    
    lines.append(f"\nTorus IMASM word: {t.imasm_code()}")
    lines.append(f"  Nodes: {len(t.nodes)}, Edges: {len(t.edges)}")
    lines.append(f"  Circuit rank β: {t.beta} (genus={t.genus})")
    lines.append(f"  Spectral radius ρ: {t.rho:.4f}")
    lines.append(f"  μ∘δ closure: {t.closure}")
    lines.append(f"  Verdict: {t.verdict}")
    
    # Physical constants
    c = compute_torus_constants(verbose=verbose)
    lines.append(f"\nConstants from torus geometry:")
    lines.append(f"  α⁻¹ = {c.alpha_inv:.10f}  (delta vs CODATA: "
                 f"{abs(c.alpha_inv - 137.035999084):.2e})")
    lines.append(f"  sin²θ_W = {c.sin2_theta_W:.6f}  (3/13)")
    lines.append(f"  α_s(m_Z) = {c.alpha_s:.6f}  (tilt/2)")
    lines.append(f"  m_π⁰/m_e = {c.m_pi0_over_me:.2f}")
    lines.append(f"  π = {c.pi_from_torus:.10f} (vessel/contents/{D})")
    
    # CKM-like mixing angles
    if c.ckm_angles:
        lines.append(f"\nCKM-like sector overlaps ({SIXTEEN_3} sectors):")
        for k, angle in enumerate(c.ckm_angles[:6]):
            deg = math.degrees(angle)
            lines.append(f"  shift k={k+1}: θ={deg:.4f}°")
    
    lines.append(f"\nHorn torus condition: R=r (self-dual), A/V=1")
    lines.append(f"  vessel/contents = 12π → π from winding structure")
    
    return "\n".join(lines)


# ── Main ─────────────────────────────────────────────────────────────────────

if __name__ == "__main__":
    import argparse
    parser = argparse.ArgumentParser(description="Horn Torus IMASM Constructor")
    parser.add_argument("--verbose", "-v", action="store_true",
                       help="Print detailed output")
    parser.add_argument("--hydrogen", action="store_true",
                       help="Print hydrogen spectral series")
    parser.add_argument("--alkali", type=str, default=None,
                       help="Print alkali D-line (Na, Li, K)")
    parser.add_argument("--build-only", action="store_true",
                       help="Only build the torus, skip ask_native")
    
    args = parser.parse_args()
    
    if args.hydrogen:
        hydrogen_series(verbose=True)
    elif args.alkali:
        alkali_doublet(args.alkali, {"Li": 2, "Na": 3, "K": 4}.get(args.alkali, 3),
                       verbose=True)
    else:
        report = generate_torus_report(verbose=args.verbose or True)
        print(report)
