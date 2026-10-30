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
