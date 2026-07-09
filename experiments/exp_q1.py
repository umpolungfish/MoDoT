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
