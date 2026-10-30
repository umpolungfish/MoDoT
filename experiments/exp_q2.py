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
