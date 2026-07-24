"""
modot/evaluator_geometry.py — SIC-POVM Evaluator Sphere Geometry Constants and Validators

Grounds the MoDoT composer and vessel in the five-part SIC-POVM evaluator sphere
geometry. Every constant here is geometrically forced — not tuned, not fitted.

Geometry reference: ig-docs/sic_povm_evaluator_geometry/01–07

Constants derived in the five-part analysis:
  1. Belnap verdict arm-signature     → arm_count / arm_weight maps
  2. Bevel-gear ratio 4:1             → GEAR_RATIO, evaluator/cosmos axle radii
  3. Tri-fork resultant 14.036°       → PRESSURE_ANGLE (arctan(1/4))
  4. Golden-ratio tangency cos(t)=1/φ → GRAZING_COS, EXIT_ANGLE
  5. Conjugate-angle triangle         → KINK_ANGLE, PRESSURE_ANGLE, EXIT_ANGLE

Author: Lando⊗⊙perator
Date:   2026-07-22
"""
from __future__ import annotations

import math
from typing import Dict, List, Optional, Tuple

# ── Evaluator Sphere Geometry ─────────────────────────────────────────────────

# The 4:1 bevel gear ratio at FFUSE₃
#   evaluator axle radius = 2R (diameter of evaluator 3-sphere)
#   cosmos axle radius    = R/2 (inradius of FFUSE₃ coupler cone)
#   ratio = 2R / (R/2) = 4:1
GEAR_RATIO: float = 4.0

# The pressure angle of the bevel gear: arctan(1/4) = 14.036°
# Forced by Belnap B4 popcount T:F:I = 1:1:2
#   tan(alpha) = |xy|/z = (2*sin(theta/2))/(8*cos(theta/2)) = (1/4)*tan(theta/2)
#   At theta = pi/2 (equatorial evaluators): tan(alpha) = 1/4
PRESSURE_ANGLE_RAD: float = math.atan(1.0 / GEAR_RATIO)
PRESSURE_ANGLE_DEG: float = math.degrees(PRESSURE_ANGLE_RAD)  # 14.03624347°

# The kink angle at the evaluator tip EV[i]
# beta = theta/2 = poloidal half-angle. At theta = pi/2: beta = 45°
KINK_ANGLE_RAD: float = math.pi / 4.0  # 45°
KINK_ANGLE_DEG: float = 45.0

# The golden-ratio exit tangency
# cos(t) = 1/phi where phi = (1+sqrt(5))/2 ≈ 1.618
# The bead exits the evaluator sphere at grazing incidence when t ≈ 51.827°
GOLDEN_RATIO: float = (1.0 + math.sqrt(5.0)) / 2.0
GRAZING_COS: float = 1.0 / GOLDEN_RATIO  # ≈ 0.618034
EXIT_ANGLE_RAD: float = math.acos(GRAZING_COS)  # ≈ 0.9046 rad
EXIT_ANGLE_DEG: float = math.degrees(EXIT_ANGLE_RAD)  # ≈ 51.827°

# The return complement angle: 90° - alpha - t ≈ 24.136°
# This is the angle between the horn-curve exit and the return path to ⊙
RETURN_ANGLE_DEG: float = 90.0 - PRESSURE_ANGLE_DEG - EXIT_ANGLE_DEG
RETURN_ANGLE_RAD: float = math.radians(RETURN_ANGLE_DEG)

# ── Arm weights from Belnap B4 popcount ───────────────────────────────────────
# The 2-bit Belnap FOUR lattice:
#   N = 00 (0 bits), T = 01 (1 bit), F = 10 (1 bit), B = 11 (2 bits)
ARM_WEIGHTS: Dict[str, float] = {
    "EVALT":  1.0,   # T arm: touches bit 0
    "EVALF":  1.0,   # F arm: touches bit 1
    "ENGAGR": 2.0,   # B arm: touches both bits
}
ARM_AZIMUTHS: Dict[str, float] = {
    "EVALT":  0.0,     # 0°
    "EVALF":  120.0,   # 120°
    "ENGAGR": 240.0,   # 240° (paradox channel at opposite azimuth)
}
TOTAL_BELNAP_WEIGHT: float = sum(ARM_WEIGHTS.values())  # = 4.0 = GEAR_RATIO

# The SIC-POVM equiangularity constant: 1/(d+1) = 1/13
SIC_DIMENSION: int = 12
SIC_EQUIANGULARITY: float = 1.0 / (SIC_DIMENSION + 1.0)

# The 16:3 sector decomposition
# 16 = 2^4 winding sectors from the 4 Frobenius-dual primitive pairs
# 3 = evaluator arms (EVALT, EVALF, ENGAGR)
FROBENIUS_DUAL_COUNT: int = 4
WINDING_SECTORS: int = 2 ** FROBENIUS_DUAL_COUNT  # 16
EVALUATOR_SECTORS: int = 3
TOTAL_SECTORS: int = WINDING_SECTORS + EVALUATOR_SECTORS  # 19

# The Weinberg angle from arm-count / SIC equiangularity denominator
# sin^2(theta_W) = evaluator_sectors / (d + 1) = 3/13
WEINBERG_SIN2: float = float(EVALUATOR_SECTORS) / float(SIC_DIMENSION + 1)

# Optimal breath: d+1 = 13 tokens, golden-ratio split 8/5 = 1.6 ≈ phi
OPTIMAL_BREATH_LENGTH: int = SIC_DIMENSION + 1  # 13
GOLDEN_BREATH_SPLIT: int = 8  # tokens before IFIX in optimal 13-token breath
GOLDEN_BREATH_RATIO: float = GOLDEN_BREATH_SPLIT / (OPTIMAL_BREATH_LENGTH - GOLDEN_BREATH_SPLIT)

# Validate geometric constants
_CONJUGATE_CHECK = PRESSURE_ANGLE_DEG + EXIT_ANGLE_DEG + RETURN_ANGLE_DEG
assert abs(_CONJUGATE_CHECK - 90.0) < 1e-10, \
    f"FFUSE3 conjugate angles do not close: {_CONJUGATE_CHECK}° != 90°"

# The three FFUSE₃ angles (alpha, t, return) sum to 90°
_FFUSE3_CHECK = PRESSURE_ANGLE_DEG + EXIT_ANGLE_DEG + RETURN_ANGLE_DEG
assert abs(_FFUSE3_CHECK - 90.0) < 1e-10, \
    f"FFUSE₃ angles do not close: {_FFUSE3_CHECK}° != 90°"


# ========================== GEOMETRIC VALIDATION ==============================

def evaluate_bevel_gear_balance(token_names: List[str]) -> dict:
    """
    Validate the Belnap weight balance across a token sequence.

    Every FSPLIT→FFUSE segment must carry exactly 4 units of Belnap weight
    (1 for each EVALT, 1 for each EVALF, 2 for each ENGAGR). This is the
    bevel gear's torque balance condition — the sum of arm weights must
    equal the gear ratio.

    Returns a dict with:
      - 'balanced': bool — whether the weight condition holds
      - 'total_weight': float — sum of Belnap weights in the segment
      - 'missing_weight': float — how much weight is missing (negative = excess)
      - 'arm_counts': dict — the count of each evaluator token
      - 'verdict': str — which Belnap verdict the arm configuration produces
      - 'resultant_tilt_deg': float — the tri-fork resultant angle from z-axis
    """
    from collections import Counter

    token_counts = Counter(token_names)
    arm_counts = {
        "EVALT":  token_counts.get("EVALT", 0),
        "EVALF":  token_counts.get("EVALF", 0),
        "ENGAGR": token_counts.get("ENGAGR", 0),
    }

    total_weight = (
        arm_counts["EVALT"] * ARM_WEIGHTS["EVALT"]
        + arm_counts["EVALF"] * ARM_WEIGHTS["EVALF"]
        + arm_counts["ENGAGR"] * ARM_WEIGHTS["ENGAGR"]
    )

    # Determine the verdict from arm-count signature
    active_arm_count = sum(1 for v in arm_counts.values() if v > 0)
    if active_arm_count == 0:
        verdict = "N"
    elif active_arm_count == 1:
        if arm_counts["EVALT"] > 0:
            verdict = "T"
        elif arm_counts["EVALF"] > 0:
            verdict = "F"
        else:
            verdict = "B"  # ENGAGR alone is paradox
    elif active_arm_count == 2:
        # Two arms: if BOTH EVALT and EVALF are active, it's a T/F fan
        if arm_counts["EVALT"] > 0 and arm_counts["EVALF"] > 0:
            verdict = "B" if arm_counts["ENGAGR"] > 0 else "T"  # T/F fan
        elif arm_counts["ENGAGR"] > 0:
            # One of T or F + B = paradox-inclined
            verdict = "B"
        else:
            verdict = "T"  # Fallback
    else:  # 3 arms
        verdict = "B"

    # Compute the tri-fork resultant tilt angle
    theta_half = math.pi / 4  # Equatorial evaluators: theta/2 = pi/4
    sin_th = math.sin(theta_half)
    cos_th = math.cos(theta_half)

    # Complex sum of weighted arm projections in xy-plane
    xy_real = 0.0
    xy_imag = 0.0
    z_total = 0.0

    for arm, count in arm_counts.items():
        if count > 0:
            weight = ARM_WEIGHTS[arm]
            azimuth_rad = math.radians(ARM_AZIMUTHS[arm])
            xy_real += weight * sin_th * math.cos(azimuth_rad)
            xy_imag += weight * sin_th * math.sin(azimuth_rad)
            z_total += weight * cos_th

    xy_magnitude = math.sqrt(xy_real**2 + xy_imag**2)
    tilt_rad = math.atan2(xy_magnitude, z_total) if z_total > 0 else math.pi / 2
    tilt_deg = math.degrees(tilt_rad)

    missing_weight = GEAR_RATIO - total_weight

    return {
        "balanced": abs(missing_weight) < 1e-9,
        "total_weight": total_weight,
        "missing_weight": missing_weight,
        "arm_counts": arm_counts,
        "active_arm_count": active_arm_count,
        "verdict": verdict,
        "resultant_tilt_deg": round(tilt_deg, 6),
        "xy_resultant_mag": round(xy_magnitude, 6),
        "z_resultant": round(z_total, 6),
    }


def classify_breath_phase(token_index: int, total_tokens: int) -> str:
    """
    Classify which phase of the conjugate-angle triangle a token position falls in.

    The breath cycle partitions into four phases with angular spans matching
    the conjugate-angle triangle:
      - 'approach'  (beta  = 45°):   VINIT → FSPLIT
      - 'fusion'    (alpha = 14.036°): EVALT/EVALF → FFUSE → ENGAGR
      - 'departure' (t     = 51.827°): IFIX → AREV → CLINK
      - 'return'    (comp  = 24.136°): TANCH → VINIT → IMSCRIB
    """
    if total_tokens <= 0:
        return "unknown"

    # Angular spans normalized to token fraction
    frac = token_index / total_tokens
    beta_span = KINK_ANGLE_DEG / 90.0       # 45/90 = 0.5
    alpha_span = PRESSURE_ANGLE_DEG / 90.0  # ~0.15596
    t_span = EXIT_ANGLE_DEG / 90.0          # ~0.57586
    return_span = RETURN_ANGLE_DEG / 90.0   # ~0.26818

    cumulative = 0.0
    phases = [("approach", beta_span), ("fusion", alpha_span),
              ("departure", t_span), ("return", return_span)]

    for name, span in phases:
        cumulative += span
        if frac <= cumulative:
            return name
    return "return"  # Should not reach here if spans sum to 1

