#!/usr/bin/env python3
"""Bifurcation experiments — mOMonadOS autopoietic convergence.
Resolves Q1-Q5. Import kernel_port.py (must be in same directory)."""
from __future__ import annotations
import sys, json, math, random
from collections import Counter, defaultdict
from kernel_port import *

# ─── Dynamic loop: one autopoietic cycle ─────────────────────────

def run_cycle(tuple_: IgTuple, length: int, tier: int, self_ref: bool,
              substrate_weight: int) -> tuple:
    """One cycle: scores → program → MiniKernel execution → value trace.
    Returns (program, value_trace, b_live_count, gate_disc_count, mk_registers)."""
    family_s = aggregate_votes(tuple_)
    sub_s = substrate_votes(tuple_, tier)
    combined = [sub_s[i] * substrate_weight + family_s[i] for i in range(12)]
    prog = build_program_from_scores(combined, length, self_ref)

    mk = MiniKernel(tuple_)
    value_trace = []
    b_live = 0
    gate_disc = 0
    for tok in prog:
        # Track B-live: B on stack when EVALT/EVALF fires
        if tok in (Token.EVALT, Token.EVALF):
            if mk.peek() == B4.B:
                b_live += 1
            if tok == Token.EVALT and mk.peek() == B4.T:
                gate_disc += 1
            elif tok == Token.EVALF and mk.peek() == B4.F:
                gate_disc += 1
        mk.step(tok)
        value_trace.append(mk.peek())

    return prog, value_trace, b_live, gate_disc, [int(r) for r in mk.r]
# ─── Tier computation (ports kernel.rs) ──────────────────────────

def compute_vp(trace: list) -> int:
    """compute_value_period — matches kernel.rs exactly."""
    n = len(trace)
    for p in range(1, n + 1):
        ok = True
        for i in range(n - p):
            if trace[(n - 1 - i) % n] != trace[(n - 1 - i - p) % n]:
                ok = False; break
        if ok:
            return p
    return 0

def static_snap(prog: list) -> dict:
    n = len(prog)
    d = len(set(prog))
    sr = n > 0 and prog[0] == prog[-1]
    fs = Token.FSPLIT in prog; ff = Token.FFUSE in prog
    if not fs and not ff: fo = 0
    elif fs and not ff: fo = 1
    elif not fs and ff: fo = 2
    else:
        fi = prog.index(Token.FSPLIT); fj = prog.index(Token.FFUSE)
        fo = 1 if fi < fj else 2

    et=Token.EVALT in prog; ef=Token.EVALF in prog; eg=Token.ENGAGR in prog
    dc = False
    if et and ef and eg:
        ok = True
        for i, t in enumerate(prog):
            if t == Token.ENGAGR:
                fg = False
                for off in range(1, n):
                    j = (i + off) % n
                    if prog[j] == Token.ENGAGR: break
                    if prog[j] in (Token.EVALT, Token.EVALF): fg = True; break
                if not fg: ok = False; break
        dc = ok
    return {'fro': fo, 'div': d, 'sr': sr, 'dc': dc}

def compute_tier(snap: dict, bl: int, gd: int, vp: int) -> int:
    ed = snap['dc'] or bl > 0
    fo = snap['fro']; sr = snap['sr']
    if ed and sr and fo > 0:
        if vp >= 3 and (bl > 0 or vp >= 3): return 3
        if gd > 0: return 2
        return 1
    if sr and fo > 0 and vp >= 3: return 3
    if fo > 0 or snap['dc']: return 1
    return 0
# ─── Q1: Critical weight sweep ───────────────────────────────────

def perturb_tuple(tuple_: IgTuple, seed: int) -> IgTuple:
    """Clone tuple with ±1 perturbation on one random primitive."""
    rng = random.Random(seed)
    t2 = IgTuple()
    fields_src = tuple_.fields()
    fields_dst = t2.fields()
    perturb_idx = rng.randint(0, 11)
    for i in range(12):
        val = fields_src[i].ordinal
        if i == perturb_idx:
            val += rng.choice([-1, 1])
        fields_dst[i].ordinal = val
    return t2

def q1_sweep(trials=3, cycles=200) -> dict:
    """Sweep w=0..10, measure tier and value_period."""
    base = IgTuple()
    results = {}
    for w in range(0, 11):
        w_results = []
        for trial in range(trials):
            tup = perturb_tuple(base, w * 100 + trial)
            trace_all = []
            bl_tot = 0; gd_tot = 0
            prog = None
            for cyc in range(cycles):
                length = 12  # O_∞ canonical length
                prog, vt, bl, gd, regs = run_cycle(tup, length, 3, True, w)
                trace_all.extend(vt)
                bl_tot += bl; gd_tot += gd
                # feed back: use last register state to perturb tuple slightly
                # This simulates the kernel's self-modifying feedback
                if cyc % 10 == 0:
                    # subtle perturbation from register B-state
                    for ri, rv in enumerate(regs):
                        if rv == 3:  # B state
                            fields = tup.fields()
                            fields[ri].ordinal += 0.1
            # analyze last 16 values
            vp = compute_vp(trace_all[-16:]) if len(trace_all) >= 16 else 0
            snap = static_snap(prog) if prog else {'fro':0,'div':0,'sr':False,'dc':False}
            tier = compute_tier(snap, bl_tot, gd_tot, vp)
            w_results.append({'trial': trial, 'tier': tier, 'vp': vp,
                              'b_live': bl_tot, 'gate_disc': gd_tot})
        tier_counts = Counter(r['tier'] for r in w_results)
        results[w] = {'trials': w_results, 'tier_counts': dict(tier_counts),
                       'frac_oinf': tier_counts.get(3, 0) / trials}
    return results

def find_wc(results: dict, threshold=0.5) -> int:
    for w in sorted(results.keys()):
        if results[w]['frac_oinf'] >= threshold:
            return w
    return -1
# ─── Q2: Universality class ──────────────────────────────────────

def q2_universality(w_values=[2,3,4], cycles=500) -> dict:
    """Analyze B-state cluster sizes and avalanche distributions."""
    base = IgTuple()
    results = {}
    for w in w_values:
        tup = IgTuple()  # fresh canonical tuple
        b_clusters = []  # sizes of contiguous B-states
        avalanche_sizes = []  # how long B perturbations persist
        b_active = 0
        prev_regs = [0,0,0,0]
        for cyc in range(cycles):
            prog, vt, bl, gd, regs = run_cycle(tup, 12, 3, True, w)
            # B-cluster: count registers in B state
            b_count = sum(1 for r in regs if r == 3)
            if b_count > 0:
                b_active += 1
                b_clusters.append(b_count)
            else:
                if b_active > 0:
                    avalanche_sizes.append(b_active)
                b_active = 0
            prev_regs = regs
            # feedback
            for ri, rv in enumerate(regs):
                if rv == 3:
                    fields = tup.fields()
                    fields[ri].ordinal += 0.05
        if b_active > 0:
            avalanche_sizes.append(b_active)

        # Fit power law to cluster size distribution
        from collections import Counter
        ctr = Counter(b_clusters)
        sizes = sorted(ctr.keys())
        if len(sizes) >= 3:
            log_s = [math.log(s) for s in sizes if ctr[s] > 0]
            log_p = [math.log(ctr[s]) for s in sizes if ctr[s] > 0]
            n = len(log_s)
            sx = sum(log_s); sy = sum(log_p)
            sxx = sum(x*x for x in log_s); sxy = sum(x*y for x,y in zip(log_s, log_p))
            slope = (n*sxy - sx*sy) / (n*sxx - sx*sx) if (n*sxx - sx*sx) != 0 else 0
            y_mean = sy/n
            ss_tot = sum((y-y_mean)**2 for y in log_p)
            ss_res = sum((log_p[i] - (slope*log_s[i] + (sy - slope*sx)/n))**2 for i in range(n))
            r2 = 1 - ss_res/ss_tot if ss_tot != 0 else 0
        else:
            slope = 0; r2 = 0

        results[w] = {
            'tau_est': -slope if slope < 0 else slope,
            'r2': r2,
            'n_clusters': len(b_clusters),
            'n_avalanches': len(avalanche_sizes),
            'mean_avalanche': sum(avalanche_sizes)/len(avalanche_sizes) if avalanche_sizes else 0,
            'cluster_sizes': dict(Counter(b_clusters)),
        }
    return results

def classify_universality(results: dict) -> str:
    """Compare tau to known classes: DP~1.1, Ising~1.05, MF~1.5"""
    best = None
    best_dist = 999
    for w, r in results.items():
        tau = r['tau_est']
        for name, target in [('Directed Percolation', 1.1),
                              ('Ising', 1.05),
                              ('Mean-field', 1.5)]:
            d = abs(tau - target)
            if d < best_dist:
                best_dist = d
                best = (w, name, tau, target)
    return f"Closest: {best[1]} (τ_target={best[3]:.2f}, τ_obs={best[2]:.3f}, Δ={best_dist:.3f}) at w={best[0]}"
# ─── Q3: Finite-size scaling ─────────────────────────────────────

def q3_finite_size(lengths=None, trials=2, cycles=100) -> dict:
    """Find w_c(L) for each program length, fit scaling relation."""
    if lengths is None:
        lengths = [4, 6, 8, 10, 12, 14, 16, 18, 22, 26, 32, 40, 50, 62]
    base = IgTuple()
    wc_by_len = {}
    for L in lengths:
        best_wc = -1
        for w in range(0, 11):
            oinf_count = 0
            for trial in range(trials):
                tup = IgTuple()
                # adjust length-appropriate tier
                tier = 3 if L >= 12 else (2 if L >= 10 else (1 if L >= 8 else 0))
                trace_all = []
                for cyc in range(cycles):
                    prog, vt, bl, gd, regs = run_cycle(tup, L, tier, L >= 6, w)
                    trace_all.extend(vt)
                vp = compute_vp(trace_all[-16:]) if len(trace_all) >= 16 else 0
                if vp >= 3:
                    oinf_count += 1
            if oinf_count / trials >= 0.5:
                best_wc = w
                break
        wc_by_len[L] = best_wc

    # Fit w_c(L) = w_c(∞) + A * L^(-1/ν)
    # Linear regression on log(w_c(L) - w_c_inf) vs log(L)
    finite = [(L, wc) for L, wc in wc_by_len.items() if wc > 0]
    if len(finite) >= 4:
        # Assume w_c_inf is the asymptotic value (largest L's w_c)
        wc_inf = min(wc for _, wc in finite)
        pts = [(math.log(L), math.log(wc - wc_inf + 0.01)) for L, wc in finite if wc > wc_inf]
        if len(pts) >= 3:
            n = len(pts)
            sx = sum(p[0] for p in pts); sy = sum(p[1] for p in pts)
            sxx = sum(p[0]*p[0] for p in pts); sxy = sum(p[0]*p[1] for p in pts)
            slope = (n*sxy - sx*sy) / (n*sxx - sx*sx) if (n*sxx - sx*sx) != 0 else 0
            nu = -1/slope if slope != 0 else 0
        else:
            nu = 0; wc_inf = 0
    else:
        nu = 0; wc_inf = 0

    return {'wc_by_len': wc_by_len, 'nu_est': nu, 'wc_inf_est': wc_inf}
# ─── Q4: Lyapunov spectrum ───────────────────────────────────────

def hamming_dist(prog_a: list, prog_b: list) -> float:
    """Normalized Hamming distance between two programs."""
    max_len = max(len(prog_a), len(prog_b))
    if max_len == 0: return 0.0
    diffs = 0
    for i in range(max_len):
        a = prog_a[i] if i < len(prog_a) else None
        b = prog_b[i] if i < len(prog_b) else None
        if a != b: diffs += 1
    return diffs / max_len

def q4_lyapunov(max_w=10, cycles=40) -> dict:
    """Measure Lyapunov exponent via Hamming distance divergence."""
    results = {}
    for w in range(0, max_w + 1):
        t0 = IgTuple()
        t1 = IgTuple()
        t1.fields()[0].ordinal += 1  # perturb D by +1

        distances = []
        prog0 = None; prog1 = None
        for cyc in range(cycles):
            p0, _, _, _, _ = run_cycle(t0, 12, 3, True, w)
            p1, _, _, _, _ = run_cycle(t1, 12, 3, True, w)
            if prog0 is not None:
                distances.append(hamming_dist(prog0, prog1))
            prog0 = p0; prog1 = p1

        # Fit log(d_n) ~ λ*n + c
        if len(distances) >= 5:
            xs = list(range(len(distances)))
            ys = [math.log(d + 0.001) for d in distances]
            n = len(xs)
            sx = sum(xs); sy = sum(ys)
            sxx = sum(x*x for x in xs); sxy = sum(x*y for x,y in zip(xs, ys))
            denom = n*sxx - sx*sx
            lam = (n*sxy - sx*sy) / denom if denom != 0 else 0
        else:
            lam = 0

        results[w] = {'lambda': lam, 'final_dist': distances[-1] if distances else 0}

    # Find λ=0 crossing
    lam_cross = None
    for w in range(1, max_w + 1):
        if results[w-1]['lambda'] <= 0 and results[w]['lambda'] > 0:
            lam_cross = w - 1 + (0 - results[w-1]['lambda']) / (results[w]['lambda'] - results[w-1]['lambda'] + 0.0001)
            break
        elif results[w-1]['lambda'] > 0 and results[w]['lambda'] <= 0:
            lam_cross = w - 1 + (results[w-1]['lambda'] - 0) / (results[w-1]['lambda'] - results[w]['lambda'] + 0.0001)
            break

    return {'by_weight': results, 'lambda_zero_crossing': lam_cross}
# ─── Q5: Algebraic w_c from entropy decomposition ────────────────

def shannon_entropy(scores: list) -> float:
    """Shannon entropy of a score distribution (normalized to probabilities)."""
    total = sum(scores)
    if total == 0: return 0.0
    probs = [s/total for s in scores if s > 0]
    return -sum(p * math.log(p) for p in probs)

def q5_algebraic_wc(max_w=10) -> dict:
    """Derive w_c from entropy balance of substrate vs family channels."""
    base = IgTuple()
    fam_s = aggregate_votes(base)
    H_fam = shannon_entropy(fam_s)

    results = {}
    for w in range(0, max_w + 1):
        sub_s = substrate_votes(base, 3)
        H_sub = shannon_entropy(sub_s)
        combined = [sub_s[i] * w + fam_s[i] for i in range(12)]
        H_comb = shannon_entropy(combined)

        # Critical condition: H_comb should balance between H_fam and w*H_sub
        # The w-scaled substrate entropy: H(w*sub) = H_sub (scaling doesn't change entropy of distribution shape)
        # But the weighting changes which distribution dominates
        # w_c is where the combined entropy is closest to max(H_fam, H_sub)
        target = max(H_fam, H_sub)
        delta = abs(H_comb - target)

        results[w] = {
            'H_fam': H_fam, 'H_sub': H_sub, 'H_comb': H_comb,
            'delta': delta,
            'entropy_ratio': H_fam / H_sub if H_sub > 0 else float('inf'),
        }

    # w_c = argmin delta
    best_w = min(range(max_w + 1), key=lambda w: results[w]['delta'])
    # Also compute from ratio: H_fam/H_sub gives the balancing point
    H_sub_val = results[0]['H_sub']
    ratio = H_fam / H_sub_val if H_sub_val > 0 else 1.0

    return {
        'by_weight': results,
        'wc_entropy_min': best_w,
        'entropy_ratio': ratio,
        'H_fam': H_fam,
        'H_sub': H_sub_val,
        'interpretation': f'w_c ≈ H_fam/H_sub = {ratio:.3f} balances structural continuity (family entropy) against creative divergence (substrate entropy)',
    }


# ═══════════════════════════════════════════════════════════════════
# MAIN: run all experiments
# ═══════════════════════════════════════════════════════════════════

def main():
    import time
    quick = '--quick' in sys.argv
    trials = 2 if quick else 5
    cycles = 50 if quick else 200

    print("=" * 60)
    print("mOMonadOS Autopoietic Convergence — Bifurcation Analysis")
    print("=" * 60)
    t0 = time.time()

    # Q1
    print("\n─── Q1: Critical substrate weight w_c ───")
    q1 = q1_sweep(trials=trials, cycles=cycles)
    wc = find_wc(q1)
    print(f"w_c (50% O_∞ threshold): {wc}")
    for w in sorted(q1.keys()):
        r = q1[w]
        print(f"  w={w}: tier_counts={r['tier_counts']}, O_∞ frac={r['frac_oinf']:.2f}")

    # Q2
    print("\n─── Q2: Universality class ───")
    w_vals = [wc-1, wc, wc+1] if wc > 1 else [1, 2, 3]
    q2 = q2_universality(w_values=w_vals, cycles=cycles)
    for w, r in q2.items():
        print(f"  w={w}: τ={r['tau_est']:.3f}, R²={r['r2']:.3f}, "
              f"clusters={r['n_clusters']}, avalanches={r['n_avalanches']}, "
              f"mean_avalanche={r['mean_avalanche']:.1f}")
    print(f"  {classify_universality(q2)}")

    # Q3
    print("\n─── Q3: Finite-size scaling ───")
    q3 = q3_finite_size(trials=trials, cycles=cycles)
    print(f"  w_c(L): {q3['wc_by_len']}")
    print(f"  ν ≈ {q3['nu_est']:.3f}, w_c(∞) ≈ {q3['wc_inf_est']}")

    # Q4
    print("\n─── Q4: Lyapunov spectrum ───")
    q4 = q4_lyapunov(max_w=10, cycles=30 if quick else 40)
    for w, r in sorted(q4['by_weight'].items()):
        lam = r['lambda']
        marker = " ← CHAOTIC" if lam > 0 else " (stable)" if lam <= 0 else ""
        print(f"  w={w}: λ={lam:.4f}{marker}")
    if q4['lambda_zero_crossing']:
        print(f"  λ=0 crossing at w≈{q4['lambda_zero_crossing']:.3f}")

    # Q5
    print("\n─── Q5: Algebraic w_c from entropy ───")
    q5 = q5_algebraic_wc()
    for w in [0, 1, 2, 3, 4, 5, 7, 10]:
        r = q5['by_weight'][w]
        print(f"  w={w}: H_comb={r['H_comb']:.4f}, Δ={r['delta']:.4f}")
    print(f"  w_c_entropy = {q5['wc_entropy_min']}")
    print(f"  H_fam={q5['H_fam']:.4f}, H_sub={q5['H_sub']:.4f}, ratio={q5['entropy_ratio']:.3f}")
    print(f"  Interpretation: {q5['interpretation']}")

    elapsed = time.time() - t0
    print(f"\n{'='*60}")
    print(f"All experiments completed in {elapsed:.1f}s")
    print(f"{'='*60}")

    # JSON output
    report = {
        'Q1': {'wc': wc, 'sweep': {str(w): q1[w]['tier_counts'] for w in q1}},
        'Q2': {str(w): {k: v for k, v in r.items() if k != 'cluster_sizes'}
               for w, r in q2.items()},
        'Q3': q3,
        'Q4': {'lambda_zero_crossing': q4['lambda_zero_crossing'],
               'lambdas': {str(w): q4['by_weight'][w]['lambda'] for w in q4['by_weight']}},
        'Q5': {'wc_entropy_min': q5['wc_entropy_min'],
               'entropy_ratio': q5['entropy_ratio'],
               'H_fam': q5['H_fam'], 'H_sub': q5['H_sub']},
        'config': {'trials': trials, 'cycles': cycles, 'quick': quick},
    }
    with open('/home/mrnob0dy666/imsgct/ig-docs/momonados_convergence/results.json', 'w') as f:
        json.dump(report, f, indent=2)
    print(f"\nResults written to results.json")

if __name__ == '__main__':
    main()
