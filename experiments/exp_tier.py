# ─── Tier computation (ports kernel.rs) ──────────────────────────

def compute_vp(trace: list) -> int:
    """compute_value_period — matches kernel.rs exactly."""
    n = len(trace)
    for p in range(1, n + 1):
        ok = True
        for i in range(n - p):
            if trace[(n - 1 - i) % n] != trace[(n - 1 - i - p) % n]:
                ok = False; break
        if ok:
            return p
    return 0

def static_snap(prog: list) -> dict:
    n = len(prog)
    d = len(set(prog))
    sr = n > 0 and prog[0] == prog[-1]
    fs = Token.FSPLIT in prog; ff = Token.FFUSE in prog
    if not fs and not ff: fo = 0
    elif fs and not ff: fo = 1
    elif not fs and ff: fo = 2
    else:
        fi = prog.index(Token.FSPLIT); fj = prog.index(Token.FFUSE)
        fo = 1 if fi < fj else 2

    et=Token.EVALT in prog; ef=Token.EVALF in prog; eg=Token.ENGAGR in prog
    dc = False
    if et and ef and eg:
        ok = True
        for i, t in enumerate(prog):
            if t == Token.ENGAGR:
                fg = False
                for off in range(1, n):
                    j = (i + off) % n
                    if prog[j] == Token.ENGAGR: break
                    if prog[j] in (Token.EVALT, Token.EVALF): fg = True; break
                if not fg: ok = False; break
        dc = ok
    return {'fro': fo, 'div': d, 'sr': sr, 'dc': dc}

def compute_tier(snap: dict, bl: int, gd: int, vp: int) -> int:
    ed = snap['dc'] or bl > 0
    fo = snap['fro']; sr = snap['sr']
    if ed and sr and fo > 0:
        if vp >= 3 and (bl > 0 or vp >= 3): return 3
        if gd > 0: return 2
        return 1
    if sr and fo > 0 and vp >= 3: return 3
    if fo > 0 or snap['dc']: return 1
    return 0
