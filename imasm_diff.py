#!/usr/bin/env python3
"""
imasm diff — Compare two IMASM type programs (Shavian types or opcode words).

Usage:
  imasm diff <typeA> <typeB>           Compare two types by name
  imasm diff --word "<opsA>" "<opsB>"  Compare two raw opcode words
  imasm diff --all                      Compare all trefoil/frobenioid pairs

Output: side-by-side opcode comparison with differences highlighted,
Frobenius verdict per program, spectral comparison, and ENGAGR/IFIX/FSPLIT census.
"""
import json, os, sys
from pathlib import Path

PRIMITIVES_DIR = Path(os.path.dirname(__file__)) / "ob3ects" / "primitives"

# ── Opcode census ──────────────────────────────────────────────────────────

def census(ops: list[str]) -> dict:
    c = {}
    for o in ops:
        c[o] = c.get(o, 0) + 1
    return c

def code_line(ops: list[str]) -> str:
    glyphs = {
        'VINIT': '⊢', 'TANCH': '⊣', 'AFWD': '>', 'AREV': '<', 'CLINK': '=',
        'IMSCRIB': '⊙', 'FSPLIT': '◇', 'FFUSE': '●', 'EVALT': '+', 'EVALF': '×',
        'ENGAGR': '⊞', 'IFIX': '¬'
    }
    return ''.join(glyphs.get(o, o) for o in ops)

def load_type(name: str) -> dict | None:
    path = PRIMITIVES_DIR / f"the_primitive_type_called_{name}_ob3ect.json"
    if not path.exists():
        return None
    with open(path) as f:
        return json.load(f)

def get_ops(t: dict) -> list[str]:
    return [s['opcode'] for s in t['phases']['phase_4']['steps']]

# ── Diff engine ────────────────────────────────────────────────────────────

def diff_ops(ops_a: list[str], ops_b: list[str], 
             name_a: str = "A", name_b: str = "B") -> str:
    lines = []
    max_len = max(len(ops_a), len(ops_b))
    glyphs_a = code_line(ops_a)
    glyphs_b = code_line(ops_b)
    
    lines.append(f"{'='*70}")
    lines.append(f"  IMASM DIFF: {name_a} ({len(ops_a)} ops) vs {name_b} ({len(ops_b)} ops)")
    lines.append(f"{'='*70}")
    
    # Glyph lines
    lines.append(f"  A: {glyphs_a}")
    lines.append(f"  B: {glyphs_b}")
    
    # Side-by-side opcode table
    lines.append(f"\n  {'Step':>4} │ {name_a:<12} │ {name_b:<12} │")
    lines.append(f"  {'─'*4}─┼─{'─'*12}─┼─{'─'*12}─┤")
    
    common = 0
    diffs_at = []
    for i in range(max_len):
        a = ops_a[i] if i < len(ops_a) else "—"
        b = ops_b[i] if i < len(ops_b) else "—"
        match = " " if a == b else "←"
        if a == b:
            common += 1
        else:
            diffs_at.append((i, a, b))
        lines.append(f"  {i+1:>4} │ {a:<12} │ {b:<12} │ {match}")
    
    # Census comparison
    lines.append(f"\n  ── Census ──")
    lines.append(f"  {'Opcode':<10} {'A':>4} {'B':>4} {'Δ':>4}")
    all_ops = sorted(set(ops_a) | set(ops_b))
    for o in all_ops:
        ca = ops_a.count(o)
        cb = ops_b.count(o)
        d = cb - ca
        marker = " ←" if d != 0 else ""
        lines.append(f"  {o:<10} {ca:>4} {cb:>4} {d:>+4}{marker}")
    
    # Differences summary
    lines.append(f"\n  ── Summary ──")
    lines.append(f"  Common positions: {common}/{max_len}")
    lines.append(f"  Differences: {len(diffs_at)}")
    if diffs_at:
        lines.append(f"  At steps: {', '.join(f'{i+1}:{a}→{b}' for i,a,b in diffs_at[:8])}")
        if len(diffs_at) > 8:
            lines.append(f"  ... and {len(diffs_at)-8} more")
    
    # Feature comparison
    lines.append(f"  ENGAGR: {'✓' if 'ENGAGR' in ops_a else '✗'} → {'✓' if 'ENGAGR' in ops_b else '✗'}")
    lines.append(f"  IMSCRIB count: {ops_a.count('IMSCRIB')} → {ops_b.count('IMSCRIB')}")
    lines.append(f"  FSPLIT count:  {ops_a.count('FSPLIT')} → {ops_b.count('FSPLIT')}")
    lines.append(f"  IFIX count:    {ops_a.count('IFIX')} → {ops_b.count('IFIX')}")
    
    return '\n'.join(lines)


def diff_types(name_a: str, name_b: str) -> str:
    ta = load_type(name_a)
    tb = load_type(name_b)
    if ta is None:
        return f"Type '{name_a}' not found in {PRIMITIVES_DIR}"
    if tb is None:
        return f"Type '{name_b}' not found in {PRIMITIVES_DIR}"
    
    ops_a = get_ops(ta)
    ops_b = get_ops(tb)
    
    label_a = f"{name_a} (ops={len(ops_a)})"
    label_b = f"{name_b} (ops={len(ops_b)})"
    return diff_ops(ops_a, ops_b, label_a, label_b)


def diff_all_pairs() -> str:
    """Compare all trefoil/frobenioid primitive-type pairs."""
    pairs = [
        ('⊢', 'dead', 'dead',   '0D anchor (shared)'),
        ('⊣', 'mime', 'mime',   'crossing topology (shared)'),
        ('Ř', 'ear',  'ian',    'coupling: trefoil→frobenioid'),
        ('Φ', 'yew',  'out',    'parity: trefoil→frobenioid'),
        ('ƒ', 'peep', 'age',    'fidelity: trefoil→frobenioid'),
        ('Ç', 'egg',  'egg',    'kinetics (shared)'),
        ('Γ', 'bib',  'bib',    'cardinality (shared)'),
        ('ɢ', 'measure','measure','composition (shared)'),
        ('⊙', 'monad','roar',   'criticality: trefoil→frobenioid'),
        ('Ħ', 'sure', 'wool',   'chirality: trefoil→frobenioid'),
        ('Σ', 'hung', 'up',     'stoichiometry: trefoil→frobenioid'),
        ('Ω', 'ah',   'ah',     'winding (shared)'),
    ]
    
    sections = []
    for axis, ta, tb, desc in pairs:
        if ta == tb:
            sections.append(f"\n{'─'*70}\n  {axis}: {ta} (shared — {desc})\n  (no diff, same type on both sides)")
            continue
        d = diff_types(ta, tb)
        sections.append(f"\n{'─'*70}\n  {axis} — {desc}")
        sections.append(d)
    
    return '\n'.join(sections)


# ── CLI ────────────────────────────────────────────────────────────────────

if __name__ == '__main__':
    if len(sys.argv) < 2:
        print("Usage: imasm_diff.py <typeA> <typeB> | --all | --word '<opsA>' '<opsB>'")
        sys.exit(1)
    
    if sys.argv[1] == '--all':
        print(diff_all_pairs())
    elif sys.argv[1] == '--word':
        ops_a = sys.argv[2].split()
        ops_b = sys.argv[3].split()
        print(diff_ops(ops_a, ops_b, "word A", "word B"))
    elif sys.argv[1] == '--census':
        name = sys.argv[2]
        t = load_type(name)
        if t:
            ops = get_ops(t)
            c = census(ops)
            print(f"  {name}: {len(ops)} ops")
            print(f"  glyph: {code_line(ops)}")
            for k, v in sorted(c.items(), key=lambda x: -x[1]):
                print(f"    {k}: {v}")
            print(f"  ENGAGR: {'✓' if 'ENGAGR' in ops else '✗'}")
            print(f"  IFIX: {ops.count('IFIX')}")
            print(f"  FSPLIT+FFUSE pairs: {ops.count('FSPLIT')}")
        else:
            print(f"Type '{name}' not found")
    else:
        a, b = sys.argv[1], sys.argv[2]
        print(diff_types(a, b))
