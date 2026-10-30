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
