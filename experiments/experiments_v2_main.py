
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
