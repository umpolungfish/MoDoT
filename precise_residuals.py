#!/usr/bin/env python3
"""Precision residual analysis for all kernel-derived constants."""
import math
import sys

d = 12
tilt = math.atan(1/4)
phi = (1 + math.sqrt(5))/2
pi = math.pi

# Kernel fine-structure constant
alpha_inv_k = d*d - 7 + tilt/(4*math.sqrt(3))
alpha_k = 1/alpha_inv_k
alpha_meas = 1/137.035999084

print("=" * 72)
print("PRECISION RESIDUAL ANALYSIS — KERNEL CONSTANTS")
print("=" * 72)

# 1. α⁻¹ residual
print("\n## 1. α⁻¹ FINE STRUCTURE CONSTANT")
res_alpha = alpha_inv_k - 137.035999084
print(f"  Kernel:         {alpha_inv_k:.10f}")
print(f"  Measured:       137.035999084")
print(f"  Residual:        {res_alpha:.6e} ({res_alpha/137.035999084*1e6:.3f} ppm)")

# α² × d = 0.0006390222
g1 = alpha_k**2 * d
print(f"\n  α² × d = {g1:.10f}")
print(f"  Diff from residual: {abs(g1 - abs(res_alpha)):.3e}")

# α² × d × (1 + α/π) = better match
g2 = alpha_k**2 * d * (1 + alpha_k/pi)
print(f"  α² × d × (1+α/π) = {g2:.10f}")
print(f"  Diff from residual: {abs(g2 - abs(res_alpha)):.3e}")

# α² × d × φ/(φ+1) where φ = phi
g3 = alpha_k**2 * d * phi/(phi+1)
print(f"  α² × d × φ/(φ+1) = {g3:.10f}")

# Best: α² × d × (1 + α/(2π)) 
g4 = alpha_k**2 * d * (1 + alpha_k/(2*pi))
print(f"  α² × d × (1+α/(2π)) = {g4:.10f}")
print(f"  Diff from residual: {abs(g4 - abs(res_alpha)):.3e}")
