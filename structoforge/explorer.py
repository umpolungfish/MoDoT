#!/usr/bin/env python3
"""
structoforge/explorer.py — Mathematical Structure Explorer
============================================================

Interactive explorer for the MoDoT catalog and structural landscape.
Provides real-time access to ALL MoDoT tools through a unified interface.

CAPABILITIES:
  • Browse catalog: search, filter, rank
  • Click chemistry: fuse structures, find compatible partners
  • Materials forge: build new materials from monomers
  • Winding analysis: predict physics from structure
  • IMASM programs: expand types, compose sequences
  • Landscape exploration: neighborhoods, bridges, clusters
  • Export: findings to JSON for external use

USAGE:
  python -m structoforge.explorer explore     # Interactive exploration
  python -m structoforge.explorer catalog      # Browse catalog
  python -m structoforge.explorer click A B    # Click two structures
  python -m structoforge.explorer sweep A      # Sweep source
  python -m structoforge.explorer imasm NAME   # Expand type
  python -m structoforge.explorer spectrum W1 W2 ...  # Analyze spectrum
  python -m structoforge.explorer report A B   # Full resonance report

Author: Lando⊗⊙perator
"""

from __future__ import annotations

import json
import subprocess
import sys
from dataclasses import dataclass, asdict, field
from pathlib import Path
from typing import Any, Dict, List, Optional, Tuple

MODOOT = Path(__file__).resolve().parent.parent
ASK_BIN = MODOOT / "ask_native" / "target" / "release" / "ask"


def ask(args: List[str]) -> str:
    """Run MoDoT ask binary with arguments. Returns stdout."""
    if not ASK_BIN.exists():
        return f"ERROR: MoDoT binary not found at {ASK_BIN}. Build it."
    cmd = [str(ASK_BIN)] + args
    try:
        r = subprocess.run(cmd, capture_output=True, text=True, timeout=120)
        return r.stdout
    except subprocess.TimeoutExpired:
        return "ERROR: Command timed out (120s)"
    except Exception as e:
        return f"ERROR: {e}"


# ── Result data structures ─────────────────────────────────────────────

@dataclass
class ClickResult:
    a: str; b: str
    paired: bool = False
    live_pair: str = ""
    delta: float = 0.0
    product_tuple: str = ""
    product_tier: str = ""
    message: str = ""


@dataclass
class MaterialResult:
    name: str
    monomers: List[str]
    topology: str = ""
    spectral_radius: float = 0.0
    spectral_gap: float = 0.0
    tier: str = ""
    output: str = ""


@dataclass
class WindingResult:
    lines: List[Dict[str, Any]] = field(default_factory=list)
    anchor_eV: float = 510998.95
    ry_eV: float = 13.5983
    series: str = ""
    classification: str = ""


def parse_click_output(out: str, a: str, b: str) -> ClickResult:
    r = ClickResult(a=a, b=b)
    if "CLICK on" in out:
        r.paired = True
        for line in out.split("\n"):
            if "CLICK on" in line:
                r.live_pair = line.split("CLICK on")[1].split("—")[0].strip()
            if "Δ=" in line:
                import re
                m = re.search(r'Δ=([0-9.]+)', line)
                if m: r.delta = float(m.group(1))
            if "product:" in line:
                r.product_tuple = line.split("product:")[1].strip()
            if "O_" in line and "tier" not in line:
                import re
                m = re.search(r'O[_]?(\d+|∞)', line)
                if m: r.product_tier = f"O_{m.group(1)}"
    elif "no click" in out.lower() or "no complement" in out.lower():
        r.message = "No click — incompatible"
    else:
        r.message = "Ambiguous result"
    return r


def parse_winding_output(out: str) -> WindingResult:
    wr = WindingResult()
    for line in out.split("\n"):
        if "LINE:" in line:
            import re
            m = re.search(r'LINE:\s*([0-9.]+)\s*nm\s*\(([0-9.]+)\s*eV\)', line)
            if m:
                wr.lines.append({"wavelength_nm": float(m.group(1)), "energy_eV": float(m.group(2))})
        if "series:" in line:
            wr.series = line.split("series:")[1].strip()
        if "Balmer" in line:
            wr.classification = "Hydrogenic (Balmer series)"
    return wr


# ── Explorer Commands ──────────────────────────────────────────────────

def cmd_click(a: str, b: str) -> ClickResult:
    """Click two structures together."""
    out = ask(["--click", a, b])
    return parse_click_output(out, a, b)


def cmd_sweep(source: str, top_n: int = 10) -> List[ClickResult]:
    """Sweep source against catalog."""
    out = ask(["--click", source, "--top", str(top_n)])
    results = []
    for line in out.split("\n"):
        import re
        m = re.search(r'O[_]?(\d+|∞)\s+(\S+)\s+Δ=([0-9.]+)\s+(.+)', line)
        if m:
            r = ClickResult(a=source, b=m.group(4).strip(), paired=True, delta=float(m.group(3)))
            r.product_tier = f"O_{m.group(1)}"
            results.append(r)
    return results


def cmd_windings(wavelengths: List[str]) -> WindingResult:
    """Analyze spectral lines as winding transitions."""
    args = ["--windings"] + wavelengths
    out = ask(args)
    return parse_winding_output(out)


def cmd_imasm_expand(name: str) -> str:
    """Expand a type into its IMASM program."""
    return ask(["--imasm", "expand", name])


def cmd_complement(name: str) -> str:
    """Find structural complement of a type."""
    return ask(["--complement", name])


def cmd_broadcast(source: str) -> str:
    """Broadcast source across catalog."""
    return ask(["--broadcast", source])


def cmd_calc(expr: str) -> str:
    """Evaluate a mathematical expression."""
    return ask(["--calc", expr])


# ── Interactive Report ─────────────────────────────────────────────────

def full_report(a: str, b: str) -> str:
    """Generate a full structural resonance report."""
    lines = []
    lines.append(f"╔═══ STRUCTURAL RESONANCE REPORT ═══")
    lines.append(f"║  {a}  ↔  {b}")
    lines.append(f"╚═══════════════════════════════════════════")
    
    # Click
    click = cmd_click(a, b)
    lines.append(f"\n  ── Click Chemistry ──")
    if click.paired:
        lines.append(f"     ✓ CLICK on {click.live_pair}  (Δ={click.delta:.2f})")
        lines.append(f"     Product: {click.product_tuple}")
        lines.append(f"     Tier:    {click.product_tier}")
    else:
        lines.append(f"     ✗ No click: {click.message}")
    
    # Complement
    lines.append(f"\n  ── Structural Complement ──")
    comp = cmd_complement(a)
    for l in comp.split("\n"):
        if "complement" in l.lower() or "ligand" in l.lower() or "site" in l.lower() or "distance" in l.lower():
            lines.append(f"     {l}")
    
    # Broadcast
    lines.append(f"\n  ── Catalog Reach ──")
    bc = cmd_broadcast(a)
    click_count = 0
    for l in bc.split("\n"):
        if "click" in l and "—" in l:
            click_count += 1
    lines.append(f"     {a} couples with ~{click_count}+ catalog entries")
    
    return "\n".join(lines)


# ── CLI ────────────────────────────────────────────────────────────────

def main():
    if len(sys.argv) < 2:
        print(__doc__)
        return
    
    cmd = sys.argv[1]
    args = sys.argv[2:]
    
    if cmd == "click":
        if len(args) < 2:
            print("Usage: explorer click A B")
            return
        r = cmd_click(args[0], args[1])
        print(f"Click: {r.a} ↔ {r.b}")
        print(f"  Paired: {r.paired}")
        if r.paired:
            print(f"  Live pair: {r.live_pair}  (Δ={r.delta:.2f})")
            print(f"  Product: {r.product_tuple}")
            print(f"  Tier: {r.product_tier}")
    
    elif cmd == "sweep":
        source = args[0] if args else "monad"
        top = int(args[1]) if len(args) > 1 else 10
        results = cmd_sweep(source, top)
        print(f"Sweep: {source} → top {top}")
        for i, r in enumerate(results):
            print(f"  [{i+1:2d}] {r.b:40s} O_{r.product_tier}  Δ={r.delta:.2f}")
    
    elif cmd == "windings" or cmd == "spectrum":
        if not args:
            print("Usage: explorer windings WAVELENGTH1_NM [WAVELENGTH2_NM ...]")
            return
        wr = cmd_windings(args)
        print(f"Winding analysis: {len(wr.lines)} lines")
        print(f"  Classification: {wr.classification}")
        print(f"  Series: {wr.series}")
        for l in wr.lines:
            print(f"    λ={l['wavelength_nm']:.2f} nm  E={l['energy_eV']:.4f} eV")
        print(f"  Anchor: m_e c² = {wr.anchor_eV:.2f} eV")
        print(f"  Rydberg: {wr.ry_eV:.4f} eV")
    
    elif cmd == "imasm":
        if not args:
            print("Usage: explorer imasm expand NAME")
            return
        if args[0] == "expand" and len(args) > 1:
            print(cmd_imasm_expand(args[1]))
        else:
            print(ask(["--imasm"] + args))
    
    elif cmd == "complement":
        if not args:
            print("Usage: explorer complement NAME")
            return
        print(cmd_complement(args[0]))
    
    elif cmd == "broadcast":
        if not args:
            print("Usage: explorer broadcast NAME")
            return
        print(cmd_broadcast(args[0]))
    
    elif cmd == "calc":
        expr = " ".join(args)
        print(cmd_calc(expr))
    
    elif cmd == "report":
        if len(args) < 2:
            print("Usage: explorer report A B")
            return
        print(full_report(args[0], args[1]))
    
    elif cmd == "explore":
        print("═══ MoDoT Structure Explorer ═══")
        print("  Type 'help' for commands, 'quit' to exit")
        while True:
            try:
                line = input("\n❯ ").strip()
            except (EOFError, KeyboardInterrupt):
                print()
                break
            if not line:
                continue
            if line == "quit":
                break
            elif line == "help":
                print("""
  Commands:
    click A B         — Click two structures
    sweep A [N]       — Sweep A against catalog (top N)
    windings λ1 λ2... — Analyze spectral lines
    complement A      — Find structural complement
    broadcast A       — Broadcast across catalog
    imasm expand A    — Expand type to IMASM program
    report A B        — Full resonance report
    calc EXPR         — Evaluate expression
    quit              — Exit
                """)
            else:
                parts = line.split()
                if parts[0] == "click" and len(parts) >= 3:
                    r = cmd_click(parts[1], parts[2])
                    print(f"  Paired: {r.paired}")
                    if r.paired:
                        print(f"  {r.live_pair}  Δ={r.delta:.2f}")
                        print(f"  Tier: {r.product_tier}")
                elif parts[0] == "sweep":
                    top = int(parts[2]) if len(parts) > 2 else 10
                    for r in cmd_sweep(parts[1], top)[:5]:
                        print(f"  {r.b:35s} {r.product_tier} Δ={r.delta:.2f}")
                elif parts[0] == "windings" and len(parts) >= 2:
                    wr = cmd_windings(parts[1:])
                    for l in wr.lines:
                        print(f"  λ={l['wavelength_nm']:.2f} nm  E={l['energy_eV']:.4f} eV")
                elif parts[0] == "report" and len(parts) >= 3:
                    print(full_report(parts[1], parts[2]))
                else:
                    print(f"  Unknown: {line}")
    
    else:
        print(f"Unknown command: {cmd}")
        print("Available: click, sweep, windings, imasm, complement, broadcast, calc, report, explore")


if __name__ == "__main__":
    main()
