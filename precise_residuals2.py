#!/usr/bin/env python3
"""Part 2: m_p/m_e precise closure check."""
import math

d = 12
tilt = math.atan(1/4)
phi = (1 + math.sqrt(5))/2
pi = math.pi

alpha_inv_k = d*d - 7 + tilt/(4*math.sqrt(3))
alpha_k = 1/alpha_inv_k
alpha_meas = 1/137.035999084

print("=" * 72)
print("## 2. m_p/m_e PROTON-ELECTRON MASS RATIO")
print("=" * 72)

d3 = d**3
d2 = d**2
term_main = d3 + d2 * 3/4  # = 1836 exactly
correction = alpha_meas * d2 / (4 * math.sqrt(3))
mpme_new = term_main + correction
mpme_meas = 1836.15267343

print(f"  d³ + d²·3/4 = {term_main:.10f}")
print(f"  Correction: α·d²/(4√3) = {correction:.10f}")
print(f"  Using kernel α: α_k·d²/(4√3) = {alpha_k*d2/(4*math.sqrt(3)):.10f}")
print(f"  Using meas α:  α_m·d²/(4√3) = {alpha_meas*d2/(4*math.sqrt(3)):.10f}")
print()
print(f"  Computed (kernel α): {term_main + alpha_k*d2/(4*math.sqrt(3)):.10f}")
print(f"  Computed (meas α):   {term_main + alpha_meas*d2/(4*math.sqrt(3)):.10f}")
print(f"  Measured:            {mpme_meas:.10f}")
print()
delta_k = term_main + alpha_k*d2/(4*math.sqrt(3)) - mpme_meas
delta_m = term_main + alpha_meas*d2/(4*math.sqrt(3)) - mpme_meas
print(f"  Delta (kernel α): {delta_k:.6e} ({abs(delta_k)/mpme_meas*1e6:.3f} ppm)")
print(f"  Delta (meas α):   {delta_m:.6e} ({abs(delta_m)/mpme_meas*1e6:.3f} ppm)")

# Check higher-order term
# The remaining delta is ~1.0e-3. Could it be α² × d² / (4√3 × d)?
h1 = alpha_k**2 * d**2 / (4 * math.sqrt(3) * d)
print(f"\n  Higher order: α²·d²/(4√3·d) = {h1:.10f}")
print(f"  With α_k: {term_main + alpha_k*d2/(4*math.sqrt(3)) + h1:.10f}")
print(f"  Delta: {term_main + alpha_k*d2/(4*math.sqrt(3)) + h1 - mpme_meas:.6e}")

# α² × d × 3/13
h2 = alpha_k**2 * d * 3/13
print(f"\n  α² × d × 3/13 = {h2:.10f}")
print(f"  With α_k: {term_main + alpha_k*d2/(4*math.sqrt(3)) + h2:.10f}")
print(f"  Delta: {term_main + alpha_k*d2/(4*math.sqrt(3)) + h2 - mpme_meas:.6e}")

# α² × d / (4√3)
h3 = alpha_k**2 * d / (4*math.sqrt(3))
print(f"\n  α² × d / (4√3) = {h3:.10f}")
print(f"  With α_k: {term_main + alpha_k*d2/(4*math.sqrt(3)) + h3:.10f}")
print(f"  Delta: {term_main + alpha_k*d2/(4*math.sqrt(3)) + h3 - mpme_meas:.6e}")

# The ratio of the remaining residual to α²
delta_remain = abs(delta_k)
print(f"\n  Remaining residual: {delta_remain:.6e}")
print(f"  Remaining/α²: {delta_remain/alpha_k**2:.6f}")
print(f"  Remaining × d: {delta_remain*d:.6f}")
print(f"  Remaining × d²: {delta_remain*d**2:.6f}")

# Check: is the remaining 0.001 = 1/(d-2)?
print(f"\n  1/(d-2) = {1/(d-2):.6f}")
print(f"  1/d = {1/d:.6f}")
print(f"  tilt/d² = {tilt/d**2:.6f}")
