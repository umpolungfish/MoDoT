#!/usr/bin/env python3
"""CORRECTED bifurcation experiments — uses proper autopoietic feedback loop.
The loop: program → static_analysis → tuple → build_via_substrate → new_program.
"""
from __future__ import annotations
import sys, json, math, random, time
from collections import Counter
from kernel_port import *
from exp_corrected import correct_cycle, tuple_from_snapshot, static_snap, compute_vp, compute_tier

# ═══════════════════════════════════════════════════════════════════
# Q1: Critical substrate weight w_c — CORRECTED
# ═══════════════════════════════════════════════════════════════════

def q1_sweep_corrected(trials=3, cycles=200) -> dict:
    """Sweep w=0..10 using proper program→program feedback."""
    results = {}
    for w in range(0, 11):
        w_results = []
        for trial in range(trials):
            # Start from canonical XII (Truth Machine)
            prog = list(CANONICALS[11]) if 11 in CANONICALS else [Token.VINIT, Token.IMSCRIB]
            trace_all = []
            bl_tot = 0; gd_tot = 0
            for cyc in range(cycles):
                prog, vt, bl, gd, snap = correct_cycle(prog, w, 12)
                trace_all.extend(vt)
                bl_tot += bl; gd_tot += gd
                # Every 20 cycles, print progress
            vp = compute_vp(trace_all[-16:]) if len(trace_all) >= 16 else 0
            snap_final = static_snap(prog)
            tier = compute_tier(snap_final, bl_tot, gd_tot, vp)
            w_results.append({'trial': trial, 'tier': tier, 'vp': vp,
                              'b_live': bl_tot, 'gate_disc': gd_tot,
                              'final_len': len(prog)})
        tc = Counter(r['tier'] for r in w_results)
        results[w] = {'tier_counts': dict(tc), 'frac_oinf': tc.get(3,0)/trials,
                       'mean_vp': sum(r['vp'] for r in w_results)/trials}
    return results

def find_wc(results: dict, threshold=0.5) -> int:
    for w in sorted(results.keys()):
        if results[w]['frac_oinf'] >= threshold:
            return w
    return -1

# ═══════════════════════════════════════════════════════════════════
# Q2: Universality class — CORRECTED
# ═══════════════════════════════════════════════════════════════════

def q2_universality_corrected(w_values=[1,2,3], cycles=500) -> dict:
    results = {}
    for w in w_values:
        prog = list(CANONICALS.get(11, [Token.VINIT]))
        b_clusters = []
        avalanche_sizes = []
        b_active = 0
        prev_b_count = 0
        for cyc in range(cycles):
            prog, vt, bl, gd, snap = correct_cycle(prog, w, 12)
            # B-state: count B in value trace
            b_in_trace = sum(1 for v in vt if v == B4.B)
            if b_in_trace > 0:
                b_active += 1
                b_clusters.append(b_in_trace)
            else:
                if b_active > 0:
                    avalanche_sizes.append(b_active)
                b_active = 0
        if b_active > 0:
            avalanche_sizes.append(b_active)

        # Fit power law
        ctr = Counter(b_clusters)
        sizes = sorted(ctr.keys())
        tau_est, r2 = 0.0, 0.0
        if len(sizes) >= 3:
            log_s = [math.log(s) for s in sizes if ctr[s] > 0]
            log_p = [math.log(ctr[s]) for s in sizes if ctr[s] > 0]
            n = len(log_s)
            sx = sum(log_s); sy = sum(log_p)
            sxx = sum(x*x for x in log_s); sxy = sum(x*y for x,y in zip(log_s, log_p))
            denom = n*sxx - sx*sx
            if denom != 0:
                slope = (n*sxy - sx*sy) / denom
                y_mean = sy/n
                ss_tot = sum((y-y_mean)**2 for y in log_p)
                ss_res = sum((log_p[i] - (slope*log_s[i] + (sy - slope*sx)/n))**2 for i in range(n))
                r2 = 1 - ss_res/ss_tot if ss_tot != 0 else 0
                tau_est = -slope

        results[w] = {'tau_est': tau_est, 'r2': r2,
                       'n_clusters': len(b_clusters),
                       'n_avalanches': len(avalanche_sizes),
                       'mean_avalanche': sum(avalanche_sizes)/len(avalanche_sizes) if avalanche_sizes else 0}
    return results

def classify_universality(results: dict) -> str:
    best, best_dist = None, 999
    for w, r in results.items():
        tau = r['tau_est']
        for name, target in [('Directed Percolation', 1.1), ('Ising', 1.05), ('Mean-field', 1.5)]:
            d = abs(tau - target)
            if d < best_dist:
                best_dist = d; best = (w, name, tau, target)
    if best:
        return f"Closest: {best[1]} (τ_target={best[3]:.2f}, τ_obs={best[2]:.3f}, Δ={best_dist:.3f}) at w={best[0]}"
    return "No match"

# ═══════════════════════════════════════════════════════════════════
# Q3: Finite-size scaling — CORRECTED
# ═══════════════════════════════════════════════════════════════════

def q3_finite_size_corrected(lengths=None, trials=2, cycles=100) -> dict:
    if lengths is None:
        lengths = [4, 6, 8, 10, 12, 14, 16, 18, 22, 26, 32, 40, 50, 62]
    wc_by_len = {}
    for L in lengths:
        best_wc = -1
        for w in range(0, 11):
            oinf_count = 0
            for trial in range(trials):
                prog = list(CANONICALS.get(11, [Token.VINIT]))
                trace_all = []
                for cyc in range(cycles):
                    prog, vt, bl, gd, snap = correct_cycle(prog, w, L)
                    trace_all.extend(vt)
                vp = compute_vp(trace_all[-16:]) if len(trace_all) >= 16 else 0
                if vp >= 3:
                    oinf_count += 1
            if oinf_count / trials >= 0.5:
                best_wc = w
                break
        wc_by_len[L] = best_wc

    finite = [(L, wc) for L, wc in wc_by_len.items() if wc > 0]
    nu, wc_inf = 0.0, 0
    if len(finite) >= 4:
        wc_inf = min(wc for _, wc in finite)
        pts = [(math.log(L), math.log(wc - wc_inf + 0.01)) for L, wc in finite if wc > wc_inf]
        if len(pts) >= 3:
            n = len(pts)
            sx = sum(p[0] for p in pts); sy = sum(p[1] for p in pts)
            sxx = sum(p[0]*p[0] for p in pts); sxy = sum(p[0]*p[1] for p in pts)
            denom = n*sxx - sx*sx
            slope = (n*sxy - sx*sy) / denom if denom != 0 else 0
            nu = -1/slope if slope != 0 else 0

    return {'wc_by_len': wc_by_len, 'nu_est': nu, 'wc_inf_est': wc_inf}
