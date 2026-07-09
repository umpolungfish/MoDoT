# ─── CORRECTED AUTOPOIETIC LOOP ──────────────────────────────────
# The actual kernel feedback: program → static_analysis → IgTuple →
# build_via_substrate → new_program. The MiniKernel within
# build_via_substrate is separate from the kernel executing the program.

def tuple_from_snapshot(snap: dict) -> IgTuple:
    """Build IgTuple from static snapshot fields (matches IgTuple::from_snapshot)."""
    t = IgTuple()
    # Map snapshot fields to primitive ordinals
    # D: token_diversity → ordinal (0-3 scaled from 1-12)
    t.d.ordinal = min(3, snap['div'] / 4.0)
    # T: self_ref → pushes topology toward odot
    t.t.ordinal = 4.0 if snap['sr'] else 2.0
    # R: frobenius_order → coupling
    t.r.ordinal = snap['fro'] * 1.0  # 0,1,2→0,1,2
    # P: dialetheia_complete → parity
    t.p.ordinal = 4.0 if snap['dc'] else 2.0
    # Remaining fields keep defaults (O_∞ tuple)
    return t

def correct_cycle(prog: List[Token], substrate_weight: int,
                  length: int = 12) -> Tuple[List[Token], List[B4], int, int, dict]:
    """One correct autopoietic cycle.
    Returns (new_program, value_trace, b_live, gate_disc, snapshot)."""
    snap = static_snap(prog)
    tup = tuple_from_snapshot(snap)
    # tier for substrate_votes: compute from current snapshot
    tier = 3  # O_∞ tuple → tier 3

    family_s = aggregate_votes(tup)
    sub_s = substrate_votes(tup, tier)
    combined = [sub_s[i] * substrate_weight + family_s[i] for i in range(12)]
    new_prog = build_program_from_scores(combined, length, snap['sr'])

    # Run MiniKernel on new_prog to get value trace
    mk = MiniKernel(tup)
    value_trace = []
    b_live = 0
    gate_disc = 0
    for tok in new_prog:
        if tok in (Token.EVALT, Token.EVALF):
            if mk.peek() == B4.B:
                b_live += 1
            if tok == Token.EVALT and mk.peek() == B4.T:
                gate_disc += 1
            elif tok == Token.EVALF and mk.peek() == B4.F:
                gate_disc += 1
        mk.step(tok)
        value_trace.append(mk.peek())

    return new_prog, value_trace, b_live, gate_disc, snap
#!/usr/bin/env python3
"""CORRECTED bifurcation experiments — uses proper autopoietic feedback loop.
The loop: program → static_analysis → tuple → build_via_substrate → new_program.
"""
import sys, json, math, random, time
from collections import Counter

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

# ═══════════════════════════════════════════════════════════════════
# Q4: Lyapunov spectrum — CORRECTED
# ═══════════════════════════════════════════════════════════════════

def hamming_dist(prog_a: list, prog_b: list) -> float:
    max_len = max(len(prog_a), len(prog_b))
    if max_len == 0: return 0.0
    diffs = sum(1 for i in range(max_len)
                if (prog_a[i] if i < len(prog_a) else None) !=
                   (prog_b[i] if i < len(prog_b) else None))
    return diffs / max_len

def q4_lyapunov_corrected(max_w=10, cycles=40) -> dict:
    results = {}
    for w in range(0, max_w + 1):
        # Two nearby starting programs
        p0 = list(CANONICALS.get(11, [Token.VINIT]))
        p1 = list(CANONICALS.get(11, [Token.VINIT]))
        if len(p1) > 0:
            # Perturb one token
            p1[0] = Token.ENGAGR if p1[0] != Token.ENGAGR else Token.VINIT

        distances = []
        prev_p0, prev_p1 = None, None
        for cyc in range(cycles):
            p0, _, _, _, _ = correct_cycle(p0, w, 12)
            p1, _, _, _, _ = correct_cycle(p1, w, 12)
            if prev_p0 is not None:
                distances.append(hamming_dist(prev_p0, prev_p1))
            prev_p0, prev_p1 = list(p0), list(p1)

        lam = 0.0
        if len(distances) >= 5:
            xs = list(range(len(distances)))
            ys = [math.log(d + 0.001) for d in distances]
            n = len(xs)
            sx = sum(xs); sy = sum(ys)
            sxx = sum(x*x for x in xs); sxy = sum(x*y for x,y in zip(xs, ys))
            denom = n*sxx - sx*sx
            lam = (n*sxy - sx*sy) / denom if denom != 0 else 0

        results[w] = {'lambda': lam, 'final_dist': distances[-1] if distances else 0}

    lam_cross = None
    for w in range(1, max_w + 1):
        prev_lam = results[w-1]['lambda']
        curr_lam = results[w]['lambda']
        if (prev_lam <= 0 and curr_lam > 0) or (prev_lam > 0 and curr_lam <= 0):
            denom = curr_lam - prev_lam
            lam_cross = w - 1 + (0 - prev_lam) / denom if abs(denom) > 0.0001 else float(w)
            break

    return {'by_weight': results, 'lambda_zero_crossing': lam_cross}


# ═══════════════════════════════════════════════════════════════════
# Q5: Algebraic w_c from entropy — CORRECTED
# ═══════════════════════════════════════════════════════════════════

def shannon_entropy(scores: list) -> float:
    total = sum(scores)
    if total == 0: return 0.0
    probs = [s/total for s in scores if s > 0]
    return -sum(p * math.log(p) for p in probs)

def q5_algebraic_wc_corrected(max_w=10) -> dict:
    # Build tuple from a canonical program snapshot
    prog = list(CANONICALS.get(11, [Token.VINIT]))
    snap = static_snap(prog)
    tup = tuple_from_snapshot(snap)

    fam_s = aggregate_votes(tup)
    H_fam = shannon_entropy(fam_s)
    sub_s = substrate_votes(tup, 3)
    H_sub = shannon_entropy(sub_s)

    results = {}
    for w in range(0, max_w + 1):
        combined = [sub_s[i] * w + fam_s[i] for i in range(12)]
        H_comb = shannon_entropy(combined)
        target = max(H_fam, H_sub)
        delta = abs(H_comb - target)
        results[w] = {'H_comb': H_comb, 'delta': delta}

    best_w = min(range(max_w + 1), key=lambda w: results[w]['delta'])
    ratio = H_fam / H_sub if H_sub > 0 else 1.0

    return {'wc_entropy_min': best_w, 'entropy_ratio': ratio,
            'H_fam': H_fam, 'H_sub': H_sub, 'by_weight': results,
            'interpretation': f'w_c ≈ H_fam/H_sub = {ratio:.3f}'}


# ═══════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════

def main():
    quick = '--quick' in sys.argv
    trials = 2 if quick else 5
    cycles = 50 if quick else 200

    print("=" * 60)
    print("mOMonadOS Autopoietic Convergence — CORRECTED Loop")
    print(f"trials={trials}, cycles={cycles}")
    print("=" * 60)
    t0 = time.time()

    # Q1
    print("\n─── Q1: Critical substrate weight w_c ───")
    q1 = q1_sweep_corrected(trials=trials, cycles=cycles)
    wc = find_wc(q1)
    print(f"w_c (50% O_∞): {wc}")
    for w in sorted(q1.keys()):
        r = q1[w]
        print(f"  w={w}: tiers={r['tier_counts']}, O_∞={r['frac_oinf']:.2f}, vp={r['mean_vp']:.1f}")

    # Q2
    print("\n─── Q2: Universality class ───")
    w_vals = [max(1, wc-1), wc, wc+1] if wc >= 1 else [1, 2, 3]
    if wc < 0: w_vals = [1, 2, 3]
    q2 = q2_universality_corrected(w_values=w_vals, cycles=cycles)
    for w, r in q2.items():
        print(f"  w={w}: τ={r['tau_est']:.3f} R²={r['r2']:.3f} "
              f"clusters={r['n_clusters']} avalanches={r['n_avalanches']} "
              f"mean_av={r['mean_avalanche']:.1f}")
    print(f"  {classify_universality(q2)}")

    # Q3
    print("\n─── Q3: Finite-size scaling ───")
    q3 = q3_finite_size_corrected(trials=trials, cycles=cycles)
    print(f"  w_c(L): {q3['wc_by_len']}")
    print(f"  ν ≈ {q3['nu_est']:.3f}, w_c(∞) ≈ {q3['wc_inf_est']}")

    # Q4
    print("\n─── Q4: Lyapunov spectrum ───")
    q4 = q4_lyapunov_corrected(max_w=10, cycles=min(30, cycles))
    for w in sorted(q4['by_weight'].keys()):
        r = q4['by_weight'][w]
        marker = "CHAOTIC" if r['lambda'] > 0 else "stable"
        print(f"  w={w}: λ={r['lambda']:.4f} ({marker}) d_final={r['final_dist']:.3f}")
    if q4['lambda_zero_crossing'] is not None:
        print(f"  λ=0 crossing at w≈{q4['lambda_zero_crossing']:.3f}")

    # Q5
    print("\n─── Q5: Algebraic w_c from entropy ───")
    q5 = q5_algebraic_wc_corrected()
    print(f"  H_fam={q5['H_fam']:.4f}, H_sub={q5['H_sub']:.4f}")
    print(f"  Entropy ratio H_fam/H_sub = {q5['entropy_ratio']:.3f}")
    for w in [0, 1, 2, 3, 4, 5, 7, 10]:
        r = q5['by_weight'][w]
        print(f"  w={w}: H_comb={r['H_comb']:.4f}, Δ={r['delta']:.4f}")
    print(f"  w_c_entropy = {q5['wc_entropy_min']}")
    print(f"  {q5['interpretation']}")

    elapsed = time.time() - t0
    print(f"\n{'='*60}")
    print(f"Completed in {elapsed:.1f}s")

    # JSON
    report = {
        'Q1': {'wc': wc, 'sweep': {str(w): q1[w]['tier_counts'] for w in q1}},
        'Q2': {str(w): {k: v for k, v in r.items()} for w, r in q2.items()},
        'Q3': q3,
        'Q4': {'lambda_zero_crossing': q4['lambda_zero_crossing'],
               'lambdas': {str(w): q4['by_weight'][w]['lambda'] for w in q4['by_weight']}},
        'Q5': {'wc_entropy_min': q5['wc_entropy_min'],
               'entropy_ratio': q5['entropy_ratio']},
        'config': {'trials': trials, 'cycles': cycles, 'quick': quick},
    }
    with open('/home/mrnob0dy666/imsgct/ig-docs/momonados_convergence/results_v2.json', 'w') as f:
        json.dump(report, f, indent=2)

if __name__ == '__main__':
    main()
# Auto-generated corrected experiment
from __future__ import annotations
import sys, json, math, random, time
from collections import Counter
from kernel_port import *
# ─── CORRECTED AUTOPOIETIC LOOP ──────────────────────────────────
# The actual kernel feedback: program → static_analysis → IgTuple →
# build_via_substrate → new_program. The MiniKernel within
# build_via_substrate is separate from the kernel executing the program.

def tuple_from_snapshot(snap: dict) -> IgTuple:
    """Build IgTuple from static snapshot fields (matches IgTuple::from_snapshot)."""
    t = IgTuple()
    # Map snapshot fields to primitive ordinals
    # D: token_diversity → ordinal (0-3 scaled from 1-12)
    t.d.ordinal = min(3, snap['div'] / 4.0)
    # T: self_ref → pushes topology toward odot
    t.t.ordinal = 4.0 if snap['sr'] else 2.0
    # R: frobenius_order → coupling
    t.r.ordinal = snap['fro'] * 1.0  # 0,1,2→0,1,2
    # P: dialetheia_complete → parity
    t.p.ordinal = 4.0 if snap['dc'] else 2.0
    # Remaining fields keep defaults (O_∞ tuple)
    return t

def correct_cycle(prog: List[Token], substrate_weight: int,
                  length: int = 12) -> Tuple[List[Token], List[B4], int, int, dict]:
    """One correct autopoietic cycle.
    Returns (new_program, value_trace, b_live, gate_disc, snapshot)."""
    snap = static_snap(prog)
    tup = tuple_from_snapshot(snap)
    # tier for substrate_votes: compute from current snapshot
    tier = 3  # O_∞ tuple → tier 3

    family_s = aggregate_votes(tup)
    sub_s = substrate_votes(tup, tier)
    combined = [sub_s[i] * substrate_weight + family_s[i] for i in range(12)]
    new_prog = build_program_from_scores(combined, length, snap['sr'])

    # Run MiniKernel on new_prog to get value trace
    mk = MiniKernel(tup)
    value_trace = []
    b_live = 0
    gate_disc = 0
    for tok in new_prog:
        if tok in (Token.EVALT, Token.EVALF):
            if mk.peek() == B4.B:
                b_live += 1
            if tok == Token.EVALT and mk.peek() == B4.T:
                gate_disc += 1
            elif tok == Token.EVALF and mk.peek() == B4.F:
                gate_disc += 1
        mk.step(tok)
        value_trace.append(mk.peek())

    return new_prog, value_trace, b_live, gate_disc, snap
#!/usr/bin/env python3
"""CORRECTED bifurcation experiments — uses proper autopoietic feedback loop.
The loop: program → static_analysis → tuple → build_via_substrate → new_program.
"""
import sys, json, math, random, time
from collections import Counter

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

# ═══════════════════════════════════════════════════════════════════
# Q4: Lyapunov spectrum — CORRECTED
# ═══════════════════════════════════════════════════════════════════

def hamming_dist(prog_a: list, prog_b: list) -> float:
    max_len = max(len(prog_a), len(prog_b))
    if max_len == 0: return 0.0
    diffs = sum(1 for i in range(max_len)
                if (prog_a[i] if i < len(prog_a) else None) !=
                   (prog_b[i] if i < len(prog_b) else None))
    return diffs / max_len

def q4_lyapunov_corrected(max_w=10, cycles=40) -> dict:
    results = {}
    for w in range(0, max_w + 1):
        # Two nearby starting programs
        p0 = list(CANONICALS.get(11, [Token.VINIT]))
        p1 = list(CANONICALS.get(11, [Token.VINIT]))
        if len(p1) > 0:
            # Perturb one token
            p1[0] = Token.ENGAGR if p1[0] != Token.ENGAGR else Token.VINIT

        distances = []
        prev_p0, prev_p1 = None, None
        for cyc in range(cycles):
            p0, _, _, _, _ = correct_cycle(p0, w, 12)
            p1, _, _, _, _ = correct_cycle(p1, w, 12)
            if prev_p0 is not None:
                distances.append(hamming_dist(prev_p0, prev_p1))
            prev_p0, prev_p1 = list(p0), list(p1)

        lam = 0.0
        if len(distances) >= 5:
            xs = list(range(len(distances)))
            ys = [math.log(d + 0.001) for d in distances]
            n = len(xs)
            sx = sum(xs); sy = sum(ys)
            sxx = sum(x*x for x in xs); sxy = sum(x*y for x,y in zip(xs, ys))
            denom = n*sxx - sx*sx
            lam = (n*sxy - sx*sy) / denom if denom != 0 else 0

        results[w] = {'lambda': lam, 'final_dist': distances[-1] if distances else 0}

    lam_cross = None
    for w in range(1, max_w + 1):
        prev_lam = results[w-1]['lambda']
        curr_lam = results[w]['lambda']
        if (prev_lam <= 0 and curr_lam > 0) or (prev_lam > 0 and curr_lam <= 0):
            denom = curr_lam - prev_lam
            lam_cross = w - 1 + (0 - prev_lam) / denom if abs(denom) > 0.0001 else float(w)
            break

    return {'by_weight': results, 'lambda_zero_crossing': lam_cross}


# ═══════════════════════════════════════════════════════════════════
# Q5: Algebraic w_c from entropy — CORRECTED
# ═══════════════════════════════════════════════════════════════════

def shannon_entropy(scores: list) -> float:
    total = sum(scores)
    if total == 0: return 0.0
    probs = [s/total for s in scores if s > 0]
    return -sum(p * math.log(p) for p in probs)

def q5_algebraic_wc_corrected(max_w=10) -> dict:
    # Build tuple from a canonical program snapshot
    prog = list(CANONICALS.get(11, [Token.VINIT]))
    snap = static_snap(prog)
    tup = tuple_from_snapshot(snap)

    fam_s = aggregate_votes(tup)
    H_fam = shannon_entropy(fam_s)
    sub_s = substrate_votes(tup, 3)
    H_sub = shannon_entropy(sub_s)

    results = {}
    for w in range(0, max_w + 1):
        combined = [sub_s[i] * w + fam_s[i] for i in range(12)]
        H_comb = shannon_entropy(combined)
        target = max(H_fam, H_sub)
        delta = abs(H_comb - target)
        results[w] = {'H_comb': H_comb, 'delta': delta}

    best_w = min(range(max_w + 1), key=lambda w: results[w]['delta'])
    ratio = H_fam / H_sub if H_sub > 0 else 1.0

    return {'wc_entropy_min': best_w, 'entropy_ratio': ratio,
            'H_fam': H_fam, 'H_sub': H_sub, 'by_weight': results,
            'interpretation': f'w_c ≈ H_fam/H_sub = {ratio:.3f}'}


# ═══════════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════════

def main():
    quick = '--quick' in sys.argv
    trials = 2 if quick else 5
    cycles = 50 if quick else 200

    print("=" * 60)
    print("mOMonadOS Autopoietic Convergence — CORRECTED Loop")
    print(f"trials={trials}, cycles={cycles}")
    print("=" * 60)
    t0 = time.time()

    # Q1
    print("\n─── Q1: Critical substrate weight w_c ───")
    q1 = q1_sweep_corrected(trials=trials, cycles=cycles)
    wc = find_wc(q1)
    print(f"w_c (50% O_∞): {wc}")
    for w in sorted(q1.keys()):
        r = q1[w]
        print(f"  w={w}: tiers={r['tier_counts']}, O_∞={r['frac_oinf']:.2f}, vp={r['mean_vp']:.1f}")

    # Q2
    print("\n─── Q2: Universality class ───")
    w_vals = [max(1, wc-1), wc, wc+1] if wc >= 1 else [1, 2, 3]
    if wc < 0: w_vals = [1, 2, 3]
    q2 = q2_universality_corrected(w_values=w_vals, cycles=cycles)
    for w, r in q2.items():
        print(f"  w={w}: τ={r['tau_est']:.3f} R²={r['r2']:.3f} "
              f"clusters={r['n_clusters']} avalanches={r['n_avalanches']} "
              f"mean_av={r['mean_avalanche']:.1f}")
    print(f"  {classify_universality(q2)}")

    # Q3
    print("\n─── Q3: Finite-size scaling ───")
    q3 = q3_finite_size_corrected(trials=trials, cycles=cycles)
    print(f"  w_c(L): {q3['wc_by_len']}")
    print(f"  ν ≈ {q3['nu_est']:.3f}, w_c(∞) ≈ {q3['wc_inf_est']}")

    # Q4
    print("\n─── Q4: Lyapunov spectrum ───")
    q4 = q4_lyapunov_corrected(max_w=10, cycles=min(30, cycles))
    for w in sorted(q4['by_weight'].keys()):
        r = q4['by_weight'][w]
        marker = "CHAOTIC" if r['lambda'] > 0 else "stable"
        print(f"  w={w}: λ={r['lambda']:.4f} ({marker}) d_final={r['final_dist']:.3f}")
    if q4['lambda_zero_crossing'] is not None:
        print(f"  λ=0 crossing at w≈{q4['lambda_zero_crossing']:.3f}")

    # Q5
    print("\n─── Q5: Algebraic w_c from entropy ───")
    q5 = q5_algebraic_wc_corrected()
    print(f"  H_fam={q5['H_fam']:.4f}, H_sub={q5['H_sub']:.4f}")
    print(f"  Entropy ratio H_fam/H_sub = {q5['entropy_ratio']:.3f}")
    for w in [0, 1, 2, 3, 4, 5, 7, 10]:
        r = q5['by_weight'][w]
        print(f"  w={w}: H_comb={r['H_comb']:.4f}, Δ={r['delta']:.4f}")
    print(f"  w_c_entropy = {q5['wc_entropy_min']}")
    print(f"  {q5['interpretation']}")

    elapsed = time.time() - t0
    print(f"\n{'='*60}")
    print(f"Completed in {elapsed:.1f}s")

    # JSON
    report = {
        'Q1': {'wc': wc, 'sweep': {str(w): q1[w]['tier_counts'] for w in q1}},
        'Q2': {str(w): {k: v for k, v in r.items()} for w, r in q2.items()},
        'Q3': q3,
        'Q4': {'lambda_zero_crossing': q4['lambda_zero_crossing'],
               'lambdas': {str(w): q4['by_weight'][w]['lambda'] for w in q4['by_weight']}},
        'Q5': {'wc_entropy_min': q5['wc_entropy_min'],
               'entropy_ratio': q5['entropy_ratio']},
        'config': {'trials': trials, 'cycles': cycles, 'quick': quick},
    }
    with open('/home/mrnob0dy666/imsgct/ig-docs/momonados_convergence/results_v2.json', 'w') as f:
        json.dump(report, f, indent=2)

if __name__ == '__main__':
    main()
