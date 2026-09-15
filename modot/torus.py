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
            "VINIT": "⊢", "TANCH": "⊣", "AFWD": "≻", "AREV": "≺",
            "CLINK": "⋈", "IMSCRIB": "⊙", "FSPLIT": "∈", "FFUSE": "∋",
            "EVALT": "⊤", "EVALF": "⊥", "EVALI": "⊞", "ENGAGR": "⊞", "IFIX": "⊡",
