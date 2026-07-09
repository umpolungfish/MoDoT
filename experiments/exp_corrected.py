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
