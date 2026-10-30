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
