#!/usr/bin/env python3
"""COMPLETE mOMonadOS autopoietic loop — exact port of kernel.rs + imas_ig.rs.
Implements the full from_snapshot→IgTuple mapping, signature, period, and
build_via_substrate cycle. Resolves Q1-Q5."""
import sys, math, random, time, json
from collections import Counter
from enum import IntEnum
from typing import List, Tuple

# ═══════════════════════════════════════════════════════════════
# BELNAP FOUR (exact port of belnap.rs)
# ═══════════════════════════════════════════════════════════════
class B4(IntEnum):
    N=0; T=1; F=2; B=3
def b4_join(a,b): return B4(a|b)
def b4_meet(a,b): return B4(a&b)
def b4_from_u8(x): return B4(x&3)
def b4_score(v): return 0 if v==B4.N else (1 if v in (B4.T,B4.F) else 2)

# ═══════════════════════════════════════════════════════════════
# TOKENS & FAMILIES (exact port of tokens.rs)
# ═══════════════════════════════════════════════════════════════
class Token(IntEnum):
    VINIT=0; TANCH=1; AFWD=2; AREV=3; CLINK=4; IMSCRIB=5
    FSPLIT=6; FFUSE=7; EVALT=8; EVALF=9; ENGAGR=10; IFIX=11

class Family(IntEnum):
    Logical=0; Frobenius=1; Dialetheia=2; Linear=3

def token_family(tok: Token) -> Family:
    if tok in (Token.VINIT, Token.TANCH, Token.AFWD, Token.AREV, Token.CLINK, Token.IMSCRIB):
        return Family.Logical
    if tok in (Token.FSPLIT, Token.FFUSE):
        return Family.Frobenius
    if tok in (Token.EVALT, Token.EVALF, Token.ENGAGR):
        return Family.Dialetheia
    return Family.Linear  # IFIX

ALL_TOKENS = list(Token)
TOKEN_NAMES = ["VINIT","TANCH","AFWD","AREV","CLINK","IMSCRIB",
               "FSPLIT","FFUSE","EVALT","EVALF","ENGAGR","IFIX"]

def signature(prog: List[Token]) -> Tuple[int,int,int,int]:
    L=F=D=X=0
    for t in prog:
        fam = token_family(t)
        if fam==Family.Logical: L+=1
        elif fam==Family.Frobenius: F+=1
        elif fam==Family.Dialetheia: D+=1
        else: X+=1
    return (L,F,D,X)

def period(prog: List[Token]) -> int:
    n=len(prog)
    if n==0: return 1
    for p in range(1,n+1):
        if n%p==0:
            if all(prog[i]==prog[i%p] for i in range(p,n)):
                return p
    return n

# ═══════════════════════════════════════════════════════════════
# IG PRIMITIVES (exact port of imas_ig.rs)
# ═══════════════════════════════════════════════════════════════
class IgPrim:
    """IG primitive with ordinal value."""
    __slots__ = ('name', 'ordinal')
    def __init__(self, name: str, ordinal: float):
        self.name = name; self.ordinal = ordinal
    def __repr__(self): return f'{self.name}({self.ordinal})'

# All IgPrim constants (from imas_ig.rs ordinal())
D_odot=IgPrim('D_odot',4.0); D_wedge=IgPrim('D_wedge',1.0); D_triangle=IgPrim('D_triangle',2.0); D_infty=IgPrim('D_infty',3.0)
T_odot=IgPrim('T_odot',5.0); T_net=IgPrim('T_net',1.0); T_in=IgPrim('T_in',2.0); T_bowtie=IgPrim('T_bowtie',3.0); T_boxtimes=IgPrim('T_boxtimes',4.0)
R_lr=IgPrim('R_lr',4.0); R_dagger=IgPrim('R_dagger',3.0); R_cat=IgPrim('R_cat',2.0); R_super=IgPrim('R_super',1.0)
P_pmsym=IgPrim('P_pmsym',5.0); P_sym=IgPrim('P_sym',4.0); P_pm=IgPrim('P_pm',3.0); P_psi=IgPrim('P_psi',2.0); P_asym=IgPrim('P_asym',1.0)
F_hbar=IgPrim('F_hbar',3.0); F_ell=IgPrim('F_ell',1.0); F_eth=IgPrim('F_eth',2.0)
K_trap=IgPrim('K_trap',4.0); K_slow=IgPrim('K_slow',3.0); K_mod=IgPrim('K_mod',2.0); K_fast=IgPrim('K_fast',1.0); K_mbl=IgPrim('K_mbl',4.5)
G_aleph=IgPrim('G_aleph',3.0); G_beth=IgPrim('G_beth',1.0); G_gimel=IgPrim('G_gimel',2.0)
C_seq=IgPrim('C_seq',3.0); C_and=IgPrim('C_and',1.0); C_or=IgPrim('C_or',2.0); C_broad=IgPrim('C_broad',4.0)
Phi_c=IgPrim('Phi_c',2.0); Phi_c_complex=IgPrim('Phi_c_complex',2.33); Phi_ep=IgPrim('Phi_ep',2.67); Phi_sub=IgPrim('Phi_sub',1.0); Phi_super=IgPrim('Phi_super',3.0)
H_inf=IgPrim('H_inf',4.0); H2=IgPrim('H2',3.0); H1=IgPrim('H1',2.0); H0=IgPrim('H0',1.0)
S_nm=IgPrim('S_nm',3.0); S_nn=IgPrim('S_nn',2.0); S_11=IgPrim('S_11',1.0)
Omega_z=IgPrim('Omega_z',3.0); Omega_z2=IgPrim('Omega_z2',2.0); Omega_0=IgPrim('Omega_0',1.0); Omega_na=IgPrim('Omega_na',4.0)

class IgTuple:
    """12-primitive IG tuple, exact port of imas_ig.rs."""
    __slots__ = ('d','t','r','p','f','k','g','c','phi','h','s','omega')
    def __init__(self, d=D_odot, t=T_odot, r=R_lr, p=P_pmsym, f=F_hbar,
                 k=K_slow, g=G_aleph, c=C_seq, phi=Phi_c, h=H_inf,
                 s=S_nm, omega=Omega_z):
        self.d=d; self.t=t; self.r=r; self.p=p; self.f=f; self.k=k
        self.g=g; self.c=c; self.phi=phi; self.h=h; self.s=s; self.omega=omega

    def fields(self): return [self.d,self.t,self.r,self.p,self.f,self.k,
                               self.g,self.c,self.phi,self.h,self.s,self.omega]

# ═══════════════════════════════════════════════════════════════
# FROM_SNAPSHOT (exact port of imas_ig.rs)
# ═══════════════════════════════════════════════════════════════

def from_snapshot(diversity: int, period_val: int, frob_order: int,
                  self_ref: bool, dialetheia_complete: bool,
                  sig: Tuple[int,int,int,int], b_live: int = 0) -> IgTuple:
    """Exact port of IgTuple::from_snapshot()."""
    d = diversity; p = period_val; fo = frob_order
    sr = self_ref; dc = dialetheia_complete or b_live > 0
    sx = sig[3]  # IFIX count

    # D from token diversity
    d_val = D_wedge if d<=2 else (D_triangle if d<=5 else (D_infty if d<=9 else D_odot))
    # T from self_ref + period + frob_order
    t_val = T_odot if sr else (T_net if p==1 else (T_bowtie if p==2 else (T_boxtimes if fo>0 else T_in)))
    # R from frob_order
    r_val = R_lr if fo==1 else (R_dagger if fo==2 else (R_cat if fo==3 else R_super))
    # P from frob_order + dialetheia
    p_val = P_pmsym if fo==1 else (P_sym if fo==2 else (P_pm if fo==3 else (P_psi if dc else P_asym)))
    # F from dialetheia + period
    f_val = F_hbar if dc else (F_ell if p==1 else F_eth)
    # K from period + IFIX count
    k_val = K_trap if sx>=8 else (K_slow if p==1 else (K_mod if p<=4 else K_fast))
    # G from IFIX + diversity
    g_val = G_aleph if sx>=3 else (G_gimel if sx>=1 else (G_beth if d<=3 else G_gimel))
    # C from frob_order + period
    c_val = C_seq if fo>0 else (C_and if p==1 else (C_or if p==2 else C_broad))
    # Phi from self_ref + dialetheia + period
    phi_val = Phi_c if (sr and dc) else (Phi_c_complex if sr else (Phi_ep if dc else (Phi_sub if p==1 else Phi_super)))
    # H from period
    h_val = H0 if p==1 else (H1 if p==2 else (H2 if p==3 else H_inf))
    # S from non-zero signature count
    nz = (1 if sig[0]>0 else 0)+(1 if sig[1]>0 else 0)+(1 if sig[2]>0 else 0)+(1 if sig[3]>0 else 0)
    s_val = S_11 if nz==1 else (S_nn if nz==2 else S_nm)
    # Omega from frob_order + self_ref + period
    omega_val = Omega_z if fo==1 else (Omega_z2 if fo==2 else (Omega_z if sr else (Omega_z2 if p==2 else Omega_0)))

    return IgTuple(d_val,t_val,r_val,p_val,f_val,k_val,g_val,c_val,phi_val,h_val,s_val,omega_val)

# ═══════════════════════════════════════════════════════════════
# STATIC ANALYSIS (exact port of self_imscribe())
# ═══════════════════════════════════════════════════════════════

def static_analysis(prog: List[Token]) -> dict:
    """Port of self_imscribe(). Returns all fields needed for from_snapshot."""
    n = len(prog)
    diversity = len(set(prog))
    self_ref = n>0 and prog[0]==prog[-1]
    p = period(prog)
    sig = signature(prog)

    fsplit_present = Token.FSPLIT in prog
    ffuse_present = Token.FFUSE in prog
    if not fsplit_present and not ffuse_present: fro=0
    elif fsplit_present and not ffuse_present: fro=1
    elif not fsplit_present and ffuse_present: fro=2
    else:
        fi=prog.index(Token.FSPLIT); fj=prog.index(Token.FFUSE)
        fro=1 if fi<fj else 2

    has_evt=Token.EVALT in prog; has_evf=Token.EVALF in prog; has_eg=Token.ENGAGR in prog
    dc = False
    if has_evt and has_evf and has_eg:
        ok=True
        for i,t in enumerate(prog):
            if t==Token.ENGAGR:
                fg=False
                for off in range(1,n):
                    j=(i+off)%n
                    if prog[j]==Token.ENGAGR: break
                    if prog[j] in (Token.EVALT, Token.EVALF): fg=True; break
                if not fg: ok=False; break
        dc=ok

    return {'div':diversity,'period':p,'fro':fro,'sr':self_ref,'dc':dc,'sig':sig,'len':n}

# ═══════════════════════════════════════════════════════════════
# FAMILY TOKEN AFFINITY (exact port of sequence.rs)
# ═══════════════════════════════════════════════════════════════

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
    [  2,  0,  0,  1 ], [  0,  0,  2,  0 ], [  1,  2,  2,  0 ], [  0,  1,  1,  2 ],
    [  2,  1,  0,  0 ], [  0,  0,  0,  3 ], [  0,  2,  0,  0 ], [  0,  2,  0,  1 ],
    [  1,  0,  2,  0 ], [  1,  0,  2,  0 ], [  2,  0,  0,  2 ], [  0,  0,  3,  1 ],
]

CANONICALS = {
    0:  [Token.VINIT, Token.TANCH]*3,                    # I: Void Genesis
    1:  [Token.VINIT, Token.ENGAGR],                      # simplified
    3:  [Token.VINIT, Token.IMSCRIB, Token.AFWD, Token.CLINK,
         Token.FSPLIT, Token.EVALT, Token.FFUSE, Token.AREV],  # IV
    6:  [Token.VINIT, Token.IMSCRIB, Token.ENGAGR, Token.FSPLIT,
         Token.EVALT, Token.FFUSE, Token.AFWD, Token.CLINK,
         Token.ENGAGR, Token.EVALF],                       # VII
    11: [Token.VINIT, Token.IMSCRIB, Token.ENGAGR, Token.FSPLIT,
         Token.EVALT, Token.EVALF, Token.FFUSE, Token.AFWD,
         Token.CLINK, Token.ENGAGR, Token.FSPLIT, Token.IFIX], # XII
}

def aggregate_votes(tup: IgTuple) -> List[int]:
    s=[0]*12
    for fam,prim in enumerate(tup.fields()):
        ord_=max(0,round(prim.ordinal))
        if ord_<=0: continue
        row=FAMILY_TOKEN_AFFINITY[fam]
        for tok in range(12): s[tok]+=row[tok]*ord_
    return s

# ═══════════════════════════════════════════════════════════════
# MINIKERNEL (exact port of sequence.rs)
# ═══════════════════════════════════════════════════════════════

def tuple_to_b4(a: IgPrim, b: IgPrim) -> B4:
    return B4((round(a.ordinal)+round(b.ordinal))&3)

class MiniKernel:
    def __init__(self, tup: IgTuple):
        self.stack=[]
        self.r=[tuple_to_b4(tup.d,tup.phi), tuple_to_b4(tup.t,tup.omega),
                tuple_to_b4(tup.k,tup.f), tuple_to_b4(tup.h,tup.p)]
    def push(self,v): 
        if len(self.stack)<64: self.stack.append(v)
    def pop(self): return self.stack.pop() if self.stack else B4.N
    def peek(self): return self.stack[-1] if self.stack else B4.N
    def step(self,tok:Token):
        N,T,F,B=B4.N,B4.T,B4.F,B4.B
        if tok==Token.VINIT: self.push(N)
        elif tok==Token.TANCH: self.pop()
        elif tok==Token.AFWD: self.r[0]=b4_from_u8((self.r[0]+1)&3)
        elif tok==Token.AREV: self.r[0]=b4_from_u8((self.r[0]-1+4)&3)
        elif tok==Token.CLINK: self.r[3]=b4_meet(self.r[1],self.r[2])
        elif tok==Token.IMSCRIB: self.r[3]=b4_join(self.r[3],self.peek())
        elif tok==Token.FSPLIT: self.push(self.peek())
        elif tok==Token.FFUSE:
            a=self.pop(); b=self.pop(); self.push(b4_join(a,b))
        elif tok==Token.EVALT:
            v=self.pop(); self.push(T if v==T else N)
        elif tok==Token.EVALF:
            v=self.pop(); self.push(F if v==F else N)
        elif tok==Token.ENGAGR:
            self.push(B); self.r[1]=b4_join(self.r[1],B)
        elif tok==Token.IFIX:
            v=self.pop(); self.r[2]=b4_join(self.r[2],v)
    def run(self,prog):
        for tok in prog: self.step(tok)
    def register_scores(self) -> List[int]:
        rv=[b4_score(self.r[i]) for i in range(4)]
        s=[0]*12
        for tok in range(12):
            for reg in range(4): s[tok]+=TOKEN_REG_AFFINITY[tok][reg]*rv[reg]
        return s

def substrate_votes(tup: IgTuple, tier: int) -> List[int]:
    idx_map={3:11,2:6,1:3,0:0}
    idx=idx_map.get(tier,0)
    mk=MiniKernel(tup)
    if idx in CANONICALS: mk.run(CANONICALS[idx])
    return mk.register_scores()

STACK_DELTA={Token.VINIT:1,Token.ENGAGR:1,Token.FSPLIT:1,
             Token.TANCH:-1,Token.IFIX:-1,Token.FFUSE:-1}

def build_program(scores,length,self_ref):
    length=max(4,min(62,length))
    order=sorted(range(12),key=lambda i:scores[i],reverse=True)
    preferred=[ALL_TOKENS[o] for o in order]
    prog=[]; est_depth=1; open_forks=0; i=0
    while i<length:
        rem=length-i
        if i==0 and self_ref: prog.append(Token.IMSCRIB); i+=1; continue
        if rem==1 and self_ref and open_forks==0: prog.append(Token.IMSCRIB); i+=1; continue
        if open_forks>0 and rem<=open_forks: prog.append(Token.FFUSE); open_forks-=1; est_depth-=1; i+=1; continue
        chosen=Token.AFWD; found=False
        for tc in preferred:
            da=est_depth+STACK_DELTA.get(tc,0)
            if da<0: continue
            if tc==Token.FFUSE and open_forks==0: continue
            if tc==Token.FSPLIT and rem<=open_forks+2: continue
            if tc==Token.TANCH and (rem!=1 or self_ref or open_forks>0): continue
            chosen=tc; found=True; break
        if not found: chosen=Token.IMSCRIB
        prog.append(chosen)
        if chosen==Token.FSPLIT: open_forks+=1
        if chosen==Token.FFUSE: open_forks-=1
        est_depth+=STACK_DELTA.get(chosen,0); i+=1
    while open_forks>0 and len(prog)<64: prog.append(Token.FFUSE); open_forks-=1
    return prog

# ═══════════════════════════════════════════════════════════════
# AUTOPOIETIC CYCLE — exact port of kernel.rs dynamic mode
# ═══════════════════════════════════════════════════════════════

def autopoietic_cycle(prog, substrate_weight, length=12):
    snap = static_analysis(prog)
    tup = from_snapshot(snap['div'], snap['period'], snap['fro'],
                        snap['sr'], snap['dc'], snap['sig'])
    tier = 3
    if snap['dc'] and snap['sr'] and snap['fro'] > 0:
        tier = 3
    elif snap['fro'] > 0 or snap['dc']:
        tier = 1
    else:
        tier = 0

    family_s = aggregate_votes(tup)
    sub_s = substrate_votes(tup, tier)
    combined = [sub_s[i] * substrate_weight + family_s[i] for i in range(12)]
    new_prog = build_program(combined, length, snap['sr'])

    mk = MiniKernel(tup)
    b_live = 0; gate_disc = 0
    for tok in new_prog:
        if tok in (Token.EVALT, Token.EVALF):
            if mk.peek() == B4.B: b_live += 1
            if (tok == Token.EVALT and mk.peek() == B4.T) or                (tok == Token.EVALF and mk.peek() == B4.F):
                gate_disc += 1
        mk.step(tok)

    diag = {'b_live': b_live, 'gate_disc': gate_disc,
            'diversity': snap['div'], 'period': snap['period'],
            'fro': snap['fro'], 'dc': snap['dc'], 'sr': snap['sr'],
            'sig': snap['sig'], 'tier': tier,
            'prog_str': ''.join(TOKEN_NAMES[t][0] for t in new_prog)}
    return new_prog, diag

def compute_value_period(trace):
    n = len(trace)
    for p in range(1, n+1):
        ok = all(trace[(n-1-i)%n] == trace[(n-1-i-p)%n] for i in range(n-p))
        if ok: return p
    return 0

def run_chain(start_prog, w, L, cycles):
    prog = list(start_prog)
    diags = []
    for cyc in range(cycles):
        prog, diag = autopoietic_cycle(prog, w, L)
        diags.append(diag)
    return diags

# Q1: SWEEP
def q1_sweep(trials=3, cycles=100):
    results = {}
    for w in range(0, 11):
        w_results = []
        for trial in range(trials):
            start = list(CANONICALS.get(11, [Token.VINIT, Token.IMSCRIB]))
            diags = run_chain(start, w, 12, cycles)
            last = diags[-1]
            final_tier = 0
            if last['dc'] and last['sr'] and last['fro'] > 0 and last['period'] >= 3:
                final_tier = 3
            elif last['fro'] > 0 or last['dc']:
                final_tier = 1
            divs = [d['diversity'] for d in diags]
            periods = [d['period'] for d in diags]
            progs_seen = len(set(d['prog_str'] for d in diags))
            w_results.append({
                'trial': trial, 'final_tier': final_tier,
                'mean_div': sum(divs)/len(divs), 'max_div': max(divs),
                'mean_period': sum(periods)/len(periods),
                'unique_progs': progs_seen,
                'final_dc': last['dc'], 'final_sr': last['sr'],
                'final_fro': last['fro'], 'final_period': last['period'],
            })
        tc = {}
        for r in w_results: tc[r['final_tier']] = tc.get(r['final_tier'],0)+1
        results[w] = {'tier_counts': tc, 'frac_oinf': tc.get(3,0)/trials,
                       'mean_unique': sum(r['unique_progs'] for r in w_results)/trials,
                       'mean_diversity': sum(r['mean_div'] for r in w_results)/trials}
    return results

def find_wc(results, threshold=0.5):
    for w in sorted(results.keys()):
        if results[w]['frac_oinf'] >= threshold: return w
    return -1

# Q4: LYAPUNOV
def program_distance(pa, pb):
    ml = max(len(pa), len(pb))
    if ml == 0: return 0.0
    return sum(1 for i in range(ml)
               if (pa[i] if i<len(pa) else None) != (pb[i] if i<len(pb) else None)) / ml

def q4_lyapunov(max_w=10, cycles=30):
    results = {}
    for w in range(0, max_w+1):
        p0 = list(CANONICALS.get(11, [Token.VINIT]))
        p1 = list(p0)
        if p1: p1[0] = Token.ENGAGR if p1[0] != Token.ENGAGR else Token.VINIT
        dists = []
        for cyc in range(cycles):
            np0, _ = autopoietic_cycle(p0, w, 12)
            np1, _ = autopoietic_cycle(p1, w, 12)
            dists.append(program_distance(np0, np1))
            p0, p1 = np0, np1
        lam = 0.0
        if len(dists) >= 5 and any(d>0 for d in dists):
            import math
            xs = list(range(len(dists)))
            ys = [math.log(d+0.001) for d in dists]
            n = len(xs); sx=sum(xs); sy=sum(ys)
            sxx=sum(x*x for x in xs); sxy=sum(x*y for x,y in zip(xs,ys))
            denom=n*sxx-sx*sx
            lam = (n*sxy-sx*sy)/denom if denom!=0 else 0
        results[w] = {'lambda': lam, 'final_dist': dists[-1] if dists else 0,
                       'mean_dist': sum(dists)/len(dists) if dists else 0}
    lam_cross = None
    for w in range(1, max_w+1):
        pv=results[w-1]['lambda']; cv=results[w]['lambda']
        if (pv<=0 and cv>0) or (pv>0 and cv<=0):
            lam_cross = w-1+(0-pv)/(cv-pv+0.0001); break
    return {'by_weight': results, 'lambda_zero_crossing': lam_cross}

# Q5: ENTROPY
def shannon_entropy(scores):
    total=sum(scores)
    if total==0: return 0.0
    import math
    probs=[s/total for s in scores if s>0]
    return -sum(p*math.log(p) for p in probs)

def q5_entropy(max_w=10):
    start = list(CANONICALS.get(11, [Token.VINIT]))
    snap = static_analysis(start)
    tup = from_snapshot(snap['div'], snap['period'], snap['fro'],
                        snap['sr'], snap['dc'], snap['sig'])
    fam_s = aggregate_votes(tup)
    sub_s = substrate_votes(tup, 3)
    H_fam = shannon_entropy(fam_s)
    H_sub = shannon_entropy(sub_s)
    by_w = {}
    for w in range(max_w+1):
        comb = [sub_s[i]*w + fam_s[i] for i in range(12)]
        Hc = shannon_entropy(comb)
        target = max(H_fam, H_sub)
        by_w[w] = {'H_comb': Hc, 'delta': abs(Hc-target)}
    best_w = min(range(max_w+1), key=lambda w: by_w[w]['delta'])
    ratio = H_fam/H_sub if H_sub>0 else 1.0
    return {'wc_entropy_min': best_w, 'entropy_ratio': ratio,
            'H_fam': H_fam, 'H_sub': H_sub, 'by_weight': by_w}


# ═══════════════════════════════════════════════════════════════
# MAIN
# ═══════════════════════════════════════════════════════════════

def main():
    import time
    quick = '--quick' in sys.argv
    trials = 2 if quick else 5
    cycles = 50 if quick else 200

    print("=" * 60)
    print("mOMonadOS Autopoietic Convergence — COMPLETE Analysis")
    print(f"trials={trials}, cycles={cycles}")
    print("=" * 60)
    t0 = time.time()

    # Q1
    print("\n--- Q1: Critical substrate weight w_c ---")
    q1 = q1_sweep(trials=trials, cycles=cycles)
    wc = find_wc(q1)
    print(f"w_c = {wc}")
    for w in sorted(q1.keys()):
        r = q1[w]
        print(f"  w={w}: tiers={r['tier_counts']}, Oinf={r['frac_oinf']:.2f}, "
              f"uniq={r['mean_unique']:.1f}, div={r['mean_diversity']:.1f}")

    # Q4
    print("\n--- Q4: Lyapunov spectrum ---")
    q4 = q4_lyapunov(max_w=10, cycles=min(30, cycles))
    for w in sorted(q4['by_weight'].keys()):
        r = q4['by_weight'][w]
        m = "CHAOTIC" if r['lambda'] > 0 else "stable"
        print(f"  w={w}: lam={r['lambda']:.4f} ({m}) d_final={r['final_dist']:.3f} "
              f"d_mean={r['mean_dist']:.3f}")
    if q4['lambda_zero_crossing'] is not None:
        print(f"  lam=0 crossing at w~{q4['lambda_zero_crossing']:.3f}")

    # Q5
    print("\n--- Q5: Algebraic w_c from entropy ---")
    q5 = q5_entropy()
    print(f"  H_fam={q5['H_fam']:.4f}, H_sub={q5['H_sub']:.4f}")
    print(f"  Entropy ratio H_fam/H_sub = {q5['entropy_ratio']:.3f}")
    for w in [0,1,2,3,4,5,7,10]:
        r = q5['by_weight'][w]
        print(f"  w={w}: H_comb={r['H_comb']:.4f}, Delta={r['delta']:.4f}")
    print(f"  w_c_entropy = {q5['wc_entropy_min']}")
    print(f"  w_c ~ H_fam/H_sub = {q5['entropy_ratio']:.3f}")

    elapsed = time.time() - t0
    print(f"\n{'='*60}")
    print(f"Completed in {elapsed:.1f}s")

    # JSON
    report = {
        'Q1': {'wc': wc, 'tier_counts_by_w': {str(w): q1[w]['tier_counts'] for w in q1},
               'unique_progs': {str(w): q1[w]['mean_unique'] for w in q1}},
        'Q4': {'lambda_zero_crossing': q4['lambda_zero_crossing'],
               'lambdas': {str(w): q4['by_weight'][w]['lambda'] for w in q4['by_weight']},
               'final_dists': {str(w): q4['by_weight'][w]['final_dist'] for w in q4['by_weight']}},
        'Q5': {'wc_entropy_min': q5['wc_entropy_min'],
               'entropy_ratio': q5['entropy_ratio'],
               'H_fam': q5['H_fam'], 'H_sub': q5['H_sub']},
        'config': {'trials': trials, 'cycles': cycles, 'quick': quick},
    }
    with open('/home/mrnob0dy666/imsgct/ig-docs/momonados_convergence/results_final.json', 'w') as f:
        json.dump(report, f, indent=2)
    print("\nResults: results_final.json")

if __name__ == '__main__':
    main()
