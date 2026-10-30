#!/usr/bin/env python3
"""Kernel-derived constants — comprehensive computation."""
import math

d = 12
tilt = math.atan(1/4)
phi = (1 + math.sqrt(5))/2
pi = math.pi
e = math.e
g = 0.5772156649015329
G = 0.9159655941772190
z3 = 1.202056903159594
alpha_inv = d*d - 7 + tilt/(4*math.sqrt(3))
alpha = 1/alpha_inv

print('='*80)
print('KERNEL-DERIVED CONSTANTS — FULL TABLE')
print(f'd={d}, tilt=arctan(1/4)={math.degrees(tilt):.4f}°, R=r=2, SIXTEEN_3')
print('='*80)
print()

# --- DIGAMMA ---
print('=== DIGAMMA psi(z) ===')
print(f'  psi(1) = -gamma = {-g:.12f}')
psi_14 = -g - pi/2 - 3*math.log(2)
print(f'  psi(1/4) = -gamma - pi/2 - 3*ln(2) = {psi_14:.12f}')
psi_13 = -g - pi/(2*math.sqrt(3)) - 1.5*math.log(3)
print(f'  psi(1/3) = -gamma - pi/(2*sqrt(3)) - (3/2)*ln(3) = {psi_13:.12f}')

def psi_series(z, N=100000):
    s = -g
    for k in range(N):
        s += 1/(k+1) - 1/(k+z)
    return s

psi_313 = psi_series(3/13)
print(f'  psi(3/13) = {psi_313:.12f} (series, N=100k)')
psi_12 = sum(1/k for k in range(1,12)) - g
print(f'  psi(12) = H_11 - gamma = {psi_12:.12f}')
print(f'  psi(13) = H_12 - gamma = {sum(1/k for k in range(1,13)) - g:.12f}')
print()

# --- e ---
print('=== e: INFINITE-WINDING LIMIT ===')
print(f'  e = lim(w->inf) (1 + 1/w)^w')
for w, name in [(d,'d=12'),(16,'16'),(d*d,'d^2=144'),(d**3,'d^3=1728'),(d**4,'d^4=20736')]:
    approx = (1+1/w)**w
    print(f'    e({name}) = {approx:.10f}  (delta={abs(approx-e):.3e})')
print()

# --- phi ---
print('=== phi: LUCAS NUMBER ===')
L12 = round(phi**12 + (-phi)**(-12))
print(f'  L_12 = phi^12 + phi^(-12) = {L12}')
print(f'  phi^12 = {phi**12:.10f}')
print(f'  phi^(-12) = {phi**(-12):.15f}')
print(f'  So: phi^d = L_d - phi^(-d) = 322 - {phi**(-12):.10f}')
print()

# --- CKM-like sector overlaps ---
print('=== CKM-LIKE WINDING OVERLAPS ===')
print(f'  SIXTEEN_3: 16 sectors, 3 evaluators')
print(f'  Overlap of evaluator sectors under shift k:')
for k in range(17):
    base = set(range(3))
    shifted = set((i+k)%16 for i in range(3))
    ov = len(base & shifted)/3
    if ov > 0:
        angle = math.acos(math.sqrt(max(0,min(1,ov))))
        print(f'    k={k:2d}: overlap={ov:.4f}, theta={math.degrees(angle):.4f}deg')
print()

# --- Cosmological constant ---
print('=== COSMOLOGICAL CONSTANT ===')
alpha_G_val = d**(-36) / (3/13)
Lambda_obs = 1e-122
print(f'  Lambda (Planck units, obs) ~ 10^(-122)')
print(f'  alpha_G = d^(-36)/(3/13) = {alpha_G_val:.4e}')
print(f'  d^(-122) = {d**(-122):.4e}')
print(f'  d^(-112) = {d**(-112):.4e}')
print(f'  alpha_G^3 * (3/13) = {alpha_G_val**3 * (3/13):.4e}')
print(f'  alpha_G^3.4 = {alpha_G_val**3.4:.4e}')
print()

# --- Neutrino mass scale ---
print('=== NEUTRINO MASS SCALE ===')
nu_ratio = 0.1/511000
print(f'  m_nu/m_e (obs) ~ {nu_ratio:.2e}')
print(f'  d^(-7) = {d**(-7):.2e}')
print(f'  d^(-6) * alpha = {d**(-6)*alpha:.2e}')
print(f'  alpha_G^(1/6) * d^2 = {alpha_G_val**(1/6)*d*d:.4e}')
print()

# --- Muon mass ---
print('=== MUON MASS ===')
mmu_obs = 206.768283
print(f'  m_mu/m_e (obs) = {mmu_obs:.4f}')
print(f'  d^2 * sqrt(2) = {d*d*math.sqrt(2):.4f}')
print(f'  d^2 * sqrt(2 + 1/d) = {d*d*math.sqrt(2+1/d):.4f}')
print(f'  d^2 * sqrt(phi) = {d*d*math.sqrt(phi):.4f}')
print(f'  d^2 * (1 + 3/13 + tilt/4) = {d*d*(1+3/13+tilt/4):.4f}')
mmu_corr = mmu_obs/(d*d)
print(f'  correction factor = {mmu_corr:.6f}')
print(f'  Correction ~ 1 + tilt*? : sqrt(1 + tilt*?) -- {math.sqrt(1 + tilt):.6f}')
print()

# --- Tau mass ---
print('=== TAU MASS ===')
mtau_obs = 3477.23
print(f'  m_tau/m_e (obs) = {mtau_obs:.2f}')
print(f'  d^3 * (1 + 3/13 + 2*alpha/pi) = {d**3*(1+3/13+2*alpha/pi):.2f}')
print(f'  d^4 / (12 + 3/13) = {d**4/(12+3/13):.2f}')
print(f'  d^3 * phi / 4 = {d**3*phi/4:.2f}')
print()

# --- W/Z/Higgs masses ---
print('=== W/Z/HIGGS MASSES ===')
mW_obs = 80379  # m_W/m_e
mZ_obs = 91188
mH_obs = 244960
print(f'  m_W/m_e (obs) = {mW_obs}')
print(f'  d^4 / 12 * alpha/pi = {d**4/12*alpha/pi:.1f}')
print(f'  d^4 / (cos(theta_W)) where cos^2 = 1-3/13 = 10/13')
cos2W = 1 - 3/13
print(f'  d^4 / (12*sqrt(10/13)) = {d**4/(12*math.sqrt(cos2W)):.1f}')
print(f'  m_Z/m_e (obs) = {mZ_obs}')
print(f'  m_W / cos(theta_W) = m_W / sqrt(10/13) = {mW_obs/math.sqrt(cos2W):.1f}')
print(f'  m_H/m_e (obs) = {mH_obs}')
print(f'  d^4 / (6 * phi) = {d**4/(6*phi):.1f}')
print()

# --- Winding sectors as angles ---
print('=== WINDING SECTORS ===')
print(f'  Each of 16 sectors: 360/16 = 22.5 deg')
print(f'  3 evaluator sectors: 67.5 deg')
print(f'  13 non-evaluator sectors: 292.5 deg')
print(f'  Tilt angle: {math.degrees(tilt):.4f} deg')
print(f'  Tilt / sector width = {math.degrees(tilt)/22.5:.6f}')
print()

# --- FINAL TABLE ---
print('='*80)
print('COMPLETED CONSTANT TABLE')
print('='*80)
print()
print(f'{"#":>3s}  {"Constant":<20s}  {"Winding Expression":<40s}  {"Derived":>14s}  {"Measured":>14s}  {"Delta":>10s}')
print('-'*110)
rows = [
    ('α⁻¹', 'd²−7+tilt/(4√3)', 137.0353596243, 137.035999084, 4.7e-6),
    ('sin²θ_W', '3/13', 0.2307692308, 0.23122, 2.0e-3),
    ('m_p/m_e', 'd³(1+tilt/4)', 1833.83, 1836.15, 1.3e-3),
    ('m_π⁰/m_e', 'd²·11/6', 264.00, 264.14, 5.3e-4),
    ('α_s(m_Z)', 'tilt/2', 0.12249, 0.1179, 3.9e-2),
    ('α_G', 'd⁻³⁶/(3/13)', 6.11e-39, 5.91e-39, 3.4e-2),
    ('Vessel/Contents', '16π²/(4π/3)', 37.69911184, '12π', 0),
    ('A/V (horn torus)', '1 (self-dual)', 1.0, 1.0, 0),
    ('e', 'lim(1+1/w)^w', 2.7182818285, 2.7182818285, 0),
    ('π', 'vessel/contents/12', 3.1415926536, 3.1415926536, 0),
    ('φ^d', 'L_d−φ^(−d)', 321.99689438, '322−1/φ^12', 0),
    ('G·d', 'Catalan·d', 10.99158713, 11, 7.6e-4),
]
for i, (name, expr, derived, measured, delta) in enumerate(rows):
    if isinstance(derived, float) and abs(derived) < 0.01:
        ds = f'{derived:14.4e}'
    elif isinstance(derived, float):
        ds = f'{derived:14.6f}'
    else:
        ds = f'{str(derived):>14s}'
    ms = f'{str(measured):>14s}'
    if isinstance(delta, float) and delta < 0.01 and delta > 0:
        dl = f'{delta:10.1e}'
    else:
        dl = f'{str(delta):>10s}'
    print(f'{i:3d}  {name:<20s}  {expr:<40s}  {ds}  {ms}  {dl}')

print()
print('=== OPEN FRONTIERS ===')
print('  m_μ/m_e: d² × correction factor ~ 206.77')
print('    Best guess: d² × sqrt(2 + 1/(d*phi))')
print('  m_τ/m_e: d⁴-scale particle, needs clean winding expression')
print('  CKM matrix: sector overlap integrals under winding shifts')
print('  Neutrino masses: d^(-7) scale, needs refinement')
print('  Λ (cosmological): α_G^3 scale, exact exponent TBD')
print('  m_W, m_Z, m_H: d⁴-scale with electroweak corrections')
