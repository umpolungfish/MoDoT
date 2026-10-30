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
