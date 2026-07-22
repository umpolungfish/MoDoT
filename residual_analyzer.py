#!/usr/bin/env python3
"""Analyze residuals of kernel-derived constants and compute CLINK L8 promotion corrections."""
import math

d = 12
tilt = math.atan(1/4)
phi = (1 + math.sqrt(5))/2
pi = math.pi
g = 0.5772156649015329

alpha_inv_k = d*d - 7 + tilt/(4*math.sqrt(3))
alpha_k = 1/alpha_inv_k

print("="*70)
print("RESIDUAL ANALYSIS: HORN TORUS → CLINK L8 CORRECTIONS")
print("="*70)

print(f"\n--- α⁻¹ (Fine Structure) ---")
print(f"Kernel:        {alpha_inv_k:.10f}")
print(f"Measured:      137.035999084")
resid = alpha_inv_k - 137.035999084
print(f"Residual:      {resid:.6e}")
print(f"Residual (ppm): {resid/137.035999084*1e6:.3f} ppm")

# α² × d
corr1 = alpha_k**2 * d
print(f"\nα² × d = {corr1:.10f}")
print(f"Match: {abs(resid - corr1)/abs(resid)*100:.3f}%")

# α² × (d+1)
corr2 = alpha_k**2 * (d+1)
print(f"α² × (d+1) = {corr2:.10f}")
print(f"Match: {abs(resid - corr2)/abs(resid)*100:.3f}%")

# check: residual * 4*sqrt(3)
print(f"\nResidual × 4√3 = {resid * 4*math.sqrt(3):.10f}")
print(f"tilt = {tilt:.10f}")
ratio = resid * 4*math.sqrt(3) / tilt
print(f"Ratio resid×4√3 / tilt = {ratio:.6f}")

# The correction as α² × d × (1 + δ)
# residual needs 0.0006394597
# α² × d = 0.0006390228
# α² × d × (1 + 1/(d+1)) = ?
corr3 = alpha_k**2 * d * (1 + 1/(d+1))
print(f"\nα² × d × (1+1/(d+1)) = {corr3:.10f}")
print(f"Match: {abs(resid - corr3)/abs(resid)*100:.3f}%")

# α² × d × (1 + 1/d)
corr4 = alpha_k**2 * d * (1 + 1/d)
print(f"α² × d × (1+1/d) = {corr4:.10f}")
print(f"Match: {abs(resid - corr4)/abs(resid)*100:.3f}%")

# residual / (α² × d) = ?
ratio2 = resid / (alpha_k**2 * d)
print(f"\nResidual/(α²×d) = {ratio2:.10f}")
print(f"That's 1 + {ratio2-1:.10f}")
# ratio2 - 1 ≈ 0.000685 — very small
# 0.000685 × d ≈ 0.00822
# 0.000685 × (d+1) ≈ 0.00891
# Hmm. 0.000685 ≈ 1 / 1460 ≈ ? 
# 1 / (12×pi) = 0.0265 — no
# 1 / (4×d) = 0.0208 — no

# Let's try: the correction = α² × d × (1 + α/π)
corr5 = alpha_k**2 * d * (1 + alpha_k/pi)
print(f"\nα² × d × (1+α/π) = {corr5:.10f}")
print(f"Match: {abs(resid - corr5)/abs(resid)*100:.3f}%")

# α² × d × (1 + tilt/d)
corr6 = alpha_k**2 * d * (1 + tilt/d)
print(f"α² × d × (1+tilt/d) = {corr6:.10f}")
print(f"Match: {abs(resid - corr6)/abs(resid)*100:.3f}%")

print(f"\n\n--- m_p/m_e (Proton/Electron Mass Ratio) ---")
d3_d2_3_4 = d**3 + d**2 * 3/4
print(f"Kernel (d³ + d²·3/4): {d3_d2_3_4:.6f}")
print(f"Measured:              1836.15267343")
resid_p = d3_d2_3_4 - 1836.15267343
print(f"Residual:              {resid_p:.6f}")
print(f"Residual (ppm):        {abs(resid_p)/1836.15267343*1e6:.3f} ppm")

# Try: add α × d² / (4√3)
corr_p1 = alpha_k * d**2 / (4*math.sqrt(3))
result1 = d3_d2_3_4 + corr_p1
print(f"\nd³ + d²·3/4 + α·d²/(4√3) = {result1:.6f}")
print(f"Difference from meas: {result1 - 1836.15267343:.6f}")

# Try: add α × d² / (d-3) = α × d² / 9
corr_p2 = alpha_k * d**2 / 9
result2 = d3_d2_3_4 + corr_p2
print(f"\nd³ + d²·3/4 + α·d²/9 = {result2:.6f}")
print(f"Difference from meas: {result2 - 1836.15267343:.6f}")

# Try: add α⁻¹ / (d² × something)
# The residual is 0.15267343
# α⁻¹ × ? = 0.15267343
# ? = 0.15267343 / 137.036 = 0.001114
# Hmm, not clean

# Try: residual ≈ 1/(4√3) × α × d²
# α × d² / (4√3) = 144/(137.036 × 6.9282) = 144/949.6 = 0.1517
# residual = 0.1527
# That's within 0.001!
ratio_p = resid_p / (alpha_k * d**2 / (4*math.sqrt(3)))
print(f"\nResidual / (α·d²/(4√3)) = {ratio_p:.6f}")
# If this were exactly 1, then m_p/m_e = d³ + d²·3/4 + α·d²/(4√3)
# Let me compute that precisely:
mpme_new = d**3 + d**2*3/4 + alpha_k * d**2 / (4*math.sqrt(3))
print(f"\nComputed m_p/m_e = {mpme_new:.10f}")
print(f"Measured         = 1836.15267343")
print(f"Delta            = {mpme_new - 1836.15267343:.6e}")
print(f"Delta (ppm)      = {(mpme_new - 1836.15267343)/1836.15267343*1e6:.3f} ppm")

# Now check: 3/4 = 9/12 = (d-3)/d
# Does this suggest 3/4 is really A₂_roots / d?
# A₂ has 3 roots. 3/d = 3/12 = 1/4... but we have 3/4
# Or: 3/4 = 9/12 = (d-3)/d?

# Let me recompute m_p/m_e exactly
# d³ + d² × (d-3)/d = d³ + d(d-3) = d³ + d² - 3d
# = 1728 + 144 - 36 = 1836
print(f"\nd³ + d(d-3) = {d**3 + d*(d-3)}")
print(f"d³ + d²/4 = {d**3 + d**2/4}")

# The residual is 0.1527
# Check: α_G × d^something?
alpha_G = d**(-36) / (3/13)
print(f"\nα_G = {alpha_G:.4e}")
print(f"α_G × d = {alpha_G * d:.4e}")
print(f"α_G × d^37 = {alpha_G * d**37:.4e}")

# Residual as a fraction of α × d³ ?
frac = 0.15267343 / (alpha_k * d**3)
print(f"\nresidual / (α × d³) = {frac:.6f}")
# 0.15267343 / (1728/137.036) = 0.15267343 / 12.614 = 0.0121
# Hmm

print(f"\n\n--- α_s (Strong Coupling at M_Z) ---")
alpha_s_k = tilt/2
alpha_s_meas = 0.1179
print(f"Kernel (tilt/2): {alpha_s_k:.6f}")
print(f"Measured:         {alpha_s_meas:.6f}")
resid_s = alpha_s_k - alpha_s_meas
print(f"Residual:         {resid_s:.6f}")

# The document says α_s/α = 16 = (2R/LR)²
# α_s = 16α = 0.116758
# Measured = 0.1180 ± 0.0009
print(f"\nα_s from ratio (16α): {16*alpha_k:.6f}")
print(f"Measured:               {alpha_s_meas:.6f}")
print(f"Delta from α ratio:     {16*alpha_k - alpha_s_meas:.6f}")

print(f"\n\n--- sin²θ_W (Weinberg Angle) ---")
sw_k = 3/13
sw_meas = 0.23122
print(f"Kernel (3/13): {sw_k:.6f}")
print(f"Measured:       {sw_meas:.6f}")
resid_w = sw_k - sw_meas
print(f"Residual:       {resid_w:.6f}")

# The running from kernel scale to Z-pole
# The SM running of sin²θ_W from 0.5 MeV to 91 GeV is about +0.007
print(f"\nSM running from kernel scale to M_Z: ~ +0.007")
print(f"Prediction with running: 3/13 + 0.007 = {3/13 + 0.007:.6f}")
print(f"Measured: {sw_meas}")

print(f"\n\n--- Catalans constant × d ---")
G = 0.9159655941772190
print(f"G × d = {G*d:.6f}")
print(f"Target = 11")
print(f"Delta = {G*d - 11:.6f}")
# G × d ≈ 10.9916, target is 11
# Difference: 0.0084
# 11 - G×d = 0.008413
print(f"11 - G×d = {11 - G*d:.6f}")
# As fraction of G×d: 0.0084/10.9916 = 0.000765
# Hmm, not clean
# But G × d = 10.9916, and 11 - 10.9916 = 0.0084
# α² = 5.325e-5, and 0.0084/α² ≈ 157.7 — not clean
