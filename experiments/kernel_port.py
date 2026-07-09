#!/usr/bin/env python3
"""Port of mOMonadOS kernel autopoietic loop for bifurcation analysis."""
from __future__ import annotations
from enum import IntEnum
from typing import List, Optional, Tuple
import math, sys

# ─── Belnap FOUR ────────────────────────────────────────────────
class B4(IntEnum):
    N = 0  # None
    T = 1  # True
    F = 2  # False
    B = 3  # Both

def b4_join(a: B4, b: B4) -> B4:
    return B4(a | b)

def b4_meet(a: B4, b: B4) -> B4:
    return B4(a & b)

def b4_from_u8(x: int) -> B4:
    return B4(x & 3)

def b4_score(v: B4) -> int:
    if v == B4.N: return 0
    if v == B4.T or v == B4.F: return 1
    return 2  # B

# ─── Token and Program ──────────────────────────────────────────
class Token(IntEnum):
    VINIT = 0; TANCH = 1; AFWD = 2; AREV = 3
    CLINK = 4; IMSCRIB = 5; FSPLIT = 6; FFUSE = 7
    EVALT = 8; EVALF = 9; ENGAGR = 10; IFIX = 11

ALL_TOKENS = list(Token)
TOKEN_NAMES = ["VINIT","TANCH","AFWD","AREV","CLINK","IMSCRIB",
               "FSPLIT","FFUSE","EVALT","EVALF","ENGAGR","IFIX"]

# ─── IgTuple ────────────────────────────────────────────────────
class IgPrim:
    def __init__(self, ordinal: float, variant: str = ""):
        self.ordinal = ordinal
        self.variant = variant

class IgTuple:
    """mOMonadOS kernel tuple: <D=odot; T=odot; R=lr; P=pm_sym; F=hbar; K=slow; G=aleph; C=seq; Phi=c; H=inf; S=het; Omega=Z>"""
    def __init__(self):
        self.d   = IgPrim(3.0)  # odot (D: 0=wedge,1=triangle,2=infty,3=odot)
        self.t   = IgPrim(4.0)  # odot (T: 0=net,1=in,2=bowtie,3=box,4=odot)
        self.r   = IgPrim(3.0)  # lr
        self.p   = IgPrim(4.0)  # pm_sym
        self.f   = IgPrim(2.0)  # hbar
        self.k   = IgPrim(2.0)  # slow
        self.g   = IgPrim(2.0)  # aleph
        self.c   = IgPrim(3.0)  # seq
        self.phi = IgPrim(1.0)  # c (critical)
        self.h   = IgPrim(3.0)  # inf
        self.s   = IgPrim(2.0)  # het
        self.omega = IgPrim(2.0)  # Z

    def fields(self) -> List[IgPrim]:
        return [self.d, self.t, self.r, self.p, self.f, self.k,
                self.g, self.c, self.phi, self.h, self.s, self.omega]

    def ordinal_list(self) -> List[int]:
        return [max(0, round(f.ordinal)) for f in self.fields()]

# ─── FAMILY_TOKEN_AFFINITY ──────────────────────────────────────
FAMILY_TOKEN_AFFINITY = [
    #VINIT TANCH AFWD AREV CLINK IMSCRIB FSPLIT FFUSE EVALT EVALF ENGAGR IFIX
    [  2,    0,    1,   1,   0,    2,      0,     0,    0,    0,    0,     1 ], # D
    [  0,    0,    0,   0,   1,    1,      2,     2,    0,    0,    1,     0 ], # T
    [  0,    0,    2,   1,   2,    0,      0,     1,    0,    0,    0,     0 ], # R
    [  0,    0,    0,   0,   0,    0,      2,     2,    1,    1,    1,     0 ], # P
    [  0,    0,    1,   1,   0,    1,      0,     0,    2,    2,    0,     0 ], # F
    [  0,    1,    2,   2,   2,    0,      0,     0,    0,    0,    0,     2 ], # K
    [  0,    0,    0,   0,   1,    2,      1,     0,    0,    0,    0,     0 ], # G
    [  0,    0,    1,   0,   1,    0,      2,     2,    0,    0,    2,     0 ], # C
    [  0,    0,    0,   0,   0,    0,      0,     0,    2,    2,    2,     0 ], # Phi
    [  0,    0,    2,   0,   2,    2,      0,     0,    0,    0,    0,     2 ], # H
    [  0,    0,    1,   0,   0,    1,      1,     1,    0,    0,    0,     0 ], # S
    [  1,    0,    1,   0,   0,    2,      0,     0,    1,    1,    2,     0 ], # Omega
]

TOKEN_REG_AFFINITY = [
    # R0  R1  R2  R3
    [  2,  0,  0,  1 ], # VINIT
    [  0,  0,  2,  0 ], # TANCH
    [  1,  2,  2,  0 ], # AFWD
    [  0,  1,  1,  2 ], # AREV
    [  2,  1,  0,  0 ], # CLINK
    [  0,  0,  0,  3 ], # IMSCRIB
    [  0,  2,  0,  0 ], # FSPLIT
    [  0,  2,  0,  1 ], # FFUSE
    [  1,  0,  2,  0 ], # EVALT
    [  1,  0,  2,  0 ], # EVALF
    [  2,  0,  0,  2 ], # ENGAGR
    [  0,  0,  3,  1 ], # IFIX
]

# ─── Canonical programs ─────────────────────────────────────────
CANONICALS = {
    0:  [Token.VINIT, Token.TANCH, Token.VINIT, Token.TANCH, Token.VINIT, Token.TANCH],  # I: Void Genesis
    1:  [Token.VINIT, Token.ENGAGR],  # simplified
    3:  [Token.VINIT, Token.IMSCRIB, Token.AFWD, Token.CLINK,
         Token.FSPLIT, Token.EVALT, Token.FFUSE, Token.AREV],  # IV: Dual Bootstrap
    6:  [Token.VINIT, Token.IMSCRIB, Token.ENGAGR, Token.FSPLIT,
         Token.EVALT, Token.FFUSE, Token.AFWD, Token.CLINK,
         Token.ENGAGR, Token.EVALF],  # VII: Parakernel
    11: [Token.VINIT, Token.IMSCRIB, Token.ENGAGR, Token.FSPLIT,
         Token.EVALT, Token.EVALF, Token.FFUSE, Token.AFWD,
         Token.CLINK, Token.ENGAGR, Token.FSPLIT, Token.IFIX],  # XII: Truth Machine
}

# ─── Functions ported from Rust ─────────────────────────────────

def aggregate_votes(tuple_: IgTuple) -> List[int]:
    s = [0] * 12
    for fam, prim in enumerate(tuple_.fields()):
        ord_ = max(0, round(prim.ordinal))
        if ord_ <= 0:
            continue
        row = FAMILY_TOKEN_AFFINITY[fam]
        for tok in range(12):
            s[tok] += row[tok] * ord_
    return s

def tuple_to_b4(a: IgPrim, b: IgPrim) -> B4:
    combined = (round(a.ordinal) + round(b.ordinal)) & 3
    return B4(combined)

class MiniKernel:
    def __init__(self, tuple_: IgTuple):
        self.stack: List[B4] = []
        self.r = [
            tuple_to_b4(tuple_.d, tuple_.phi),
            tuple_to_b4(tuple_.t, tuple_.omega),
            tuple_to_b4(tuple_.k, tuple_.f),
            tuple_to_b4(tuple_.h, tuple_.p),
        ]

    def push(self, v: B4):
        if len(self.stack) < 64:
            self.stack.append(v)

    def pop(self) -> B4:
        return self.stack.pop() if self.stack else B4.N

    def peek(self) -> B4:
        return self.stack[-1] if self.stack else B4.N

    def step(self, tok: Token):
        N, T, F, B = B4.N, B4.T, B4.F, B4.B
        if tok == Token.VINIT:
            self.push(N)
        elif tok == Token.TANCH:
            self.pop()
        elif tok == Token.AFWD:
            self.r[0] = b4_from_u8((self.r[0] + 1) & 3)
        elif tok == Token.AREV:
            self.r[0] = b4_from_u8((self.r[0] - 1 + 4) & 3)
        elif tok == Token.CLINK:
            self.r[3] = b4_meet(self.r[1], self.r[2])
        elif tok == Token.IMSCRIB:
            self.r[3] = b4_join(self.r[3], self.peek())
        elif tok == Token.FSPLIT:
            self.push(self.peek())
        elif tok == Token.FFUSE:
            a = self.pop()
            b_val = self.pop()
            self.push(b4_join(a, b_val))
        elif tok == Token.EVALT:
            v = self.pop()
            self.push(T if v == T else N)
        elif tok == Token.EVALF:
            v = self.pop()
            self.push(F if v == F else N)
        elif tok == Token.ENGAGR:
            self.push(B)
            self.r[1] = b4_join(self.r[1], B)
        elif tok == Token.IFIX:
            v = self.pop()
            self.r[2] = b4_join(self.r[2], v)

    def run(self, prog: List[Token]):
        for tok in prog:
            self.step(tok)

    def register_scores(self) -> List[int]:
        rv = [b4_score(self.r[i]) for i in range(4)]
        s = [0] * 12
        for tok in range(12):
            for reg in range(4):
                s[tok] += TOKEN_REG_AFFINITY[tok][reg] * rv[reg]
        return s

def substrate_votes(tuple_: IgTuple, tier: int) -> List[int]:
    idx_map = {3: 11, 2: 6, 1: 3, 0: 0}
    idx = idx_map.get(tier, 0)
    mk = MiniKernel(tuple_)
    if idx in CANONICALS:
        mk.run(CANONICALS[idx])
    return mk.register_scores()

STACK_DELTA = {
    Token.VINIT: 1, Token.ENGAGR: 1, Token.FSPLIT: 1,
    Token.TANCH: -1, Token.IFIX: -1, Token.FFUSE: -1,
}

def build_program_from_scores(scores: List[int], length: int, self_ref: bool) -> List[Token]:
    length = max(4, min(62, length))
    # sort tokens by score descending
    order = sorted(range(12), key=lambda i: scores[i], reverse=True)
    preferred = [ALL_TOKENS[o] for o in order]

    prog: List[Token] = []
    est_depth = 1
    open_forks = 0

    i = 0
    while i < length:
        remaining = length - i
        is_first = (i == 0)
        is_last = (remaining == 1)

        if is_first and self_ref:
            prog.append(Token.IMSCRIB)
            i += 1
            continue

        if is_last and self_ref and open_forks == 0:
            prog.append(Token.IMSCRIB)
            i += 1
            continue

        if open_forks > 0 and remaining <= open_forks:
            prog.append(Token.FFUSE)
            open_forks -= 1
            est_depth -= 1
            i += 1
            continue

        chosen = Token.AFWD
        found = False
        for tok_cand in preferred:
            sd = STACK_DELTA.get(tok_cand, 0)
            depth_after = est_depth + sd
            if depth_after < 0:
                continue
            if tok_cand == Token.FFUSE and open_forks == 0:
                continue
            if tok_cand == Token.FSPLIT and remaining <= open_forks + 2:
                continue
            if tok_cand == Token.TANCH and (not is_last or self_ref or open_forks > 0):
                continue
            chosen = tok_cand
            found = True
            break

        if not found:
            chosen = Token.IMSCRIB

        prog.append(chosen)
        if chosen == Token.FSPLIT:
            open_forks += 1
        if chosen == Token.FFUSE:
            open_forks -= 1
        est_depth += STACK_DELTA.get(chosen, 0)
        i += 1

    while open_forks > 0 and len(prog) < 64:
        prog.append(Token.FFUSE)
        open_forks -= 1

    return prog
