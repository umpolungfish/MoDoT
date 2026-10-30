

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
