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
        'VINIT': '⊢', 'TANCH': '⊣', 'AFWD': '≻', 'AREV': '≺', 'CLINK': '⋈',
        'IMSCRIB': '⊙', 'FSPLIT': '∈', 'FFUSE': '∋', 'EVALT': '⊤', 'EVALF': '⊥',
        'ENGAGR': '⊞', 'IFIX': '⊡'
