#!/usr/bin/env python3
"""
structoforge/cli.py — Structural Resonance Foundry CLI
========================================================

Unified command-line interface for the entire StructoForge mathematical
environment. Provides access to all tools: resonance engine, materials
workbench, winding predictor, landscape explorer, and proof forge.

USAGE:
  python -m structoforge.cli resonance A B
  python -m structoforge.cli sweep SOURCE [--top N]
  python -m structoforge.cli forge NAME MONOMER1 [MONOMER2...]
  python -m structoforge.cli landscape summary
  python -m structoforge.cli landscape neighborhood NAME [radius]
  python -m structoforge.cli predict NAME TUPLE
  python -m structoforge.cli proof NAME DESCRIPTION [--tuple TUPLE]
  python -m structoforge.cli spectrum WAVELENGTH_NM [WAVELENGTH_NM...]
  python -m structoforge.cli matrix A B C...

Author: Lando⊗⊙perator
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import List, Optional


def main():
    parser = argparse.ArgumentParser(
        description="StructoForge — Structural Resonance Foundry",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python -m structoforge.cli resonance monad topos
  python -m structoforge.cli sweep monad --top 5
  python -m structoforge.cli forge my_material graphene silicon --register
  python -m structoforge.cli landscape summary
  python -m structoforge.cli landscape neighborhood monad
  python -m structoforge.cli predict my_system "⟨𐑛𐑸𐑾⊙⋯⟩"
  python -m structoforge.cli proof my_theorem "Description" --tuple "⟨⋯⟩"
  python -m structoforge.cli spectrum 656.28 486.13 434.05 410.17
  python -m structoforge.cli matrix monad topos hopf
        """
    )
    
    subparsers = parser.add_subparsers(dest="command", help="Available commands")
    
    # ── resonance ──────────────────────────────────────────────────────
    res_parser = subparsers.add_parser("resonance", help="Compute structural resonance between two systems")
    res_parser.add_argument("a", help="First system name")
    res_parser.add_argument("b", help="Second system name")
    res_parser.add_argument("--json", "-j", action="store_true", help="JSON output")
    
    # ── sweep ──────────────────────────────────────────────────────────
    sweep_parser = subparsers.add_parser("sweep", help="Sweep a source against catalog")
    sweep_parser.add_argument("source", help="Source system name")
    sweep_parser.add_argument("--top", "-n", type=int, default=10, help="Number of results")
    sweep_parser.add_argument("--json", "-j", action="store_true", help="JSON output")
    
    # ── matrix ─────────────────────────────────────────────────────────
    matrix_parser = subparsers.add_parser("matrix", help="Compute full resonance matrix")
    matrix_parser.add_argument("systems", nargs="+", help="System names")
    matrix_parser.add_argument("--json", "-j", action="store_true", help="JSON output")
    
    # ── forge ──────────────────────────────────────────────────────────
    forge_parser = subparsers.add_parser("forge", help="Forge a material")
    forge_parser.add_argument("name", help="Material name")
    forge_parser.add_argument("monomers", nargs="+", help="Monomer names")
    forge_parser.add_argument("--register", "-r", action="store_true", help="Register in catalog")
    forge_parser.add_argument("--json", "-j", action="store_true", help="JSON output")
    
    # ── landscape ──────────────────────────────────────────────────────
    land_parser = subparsers.add_parser("landscape", help="Explore structural landscape")
    land_sub = land_parser.add_subparsers(dest="landscape_action", help="Landscape action")
    
    land_summary = land_sub.add_parser("summary", help="Landscape overview")
    land_neighbor = land_sub.add_parser("neighborhood", help="Explore a neighborhood")
    land_neighbor.add_argument("name", help="Center nature name")
    land_neighbor.add_argument("radius", nargs="?", type=int, default=6, help="Cotype radius")
    land_bridges = land_sub.add_parser("bridges", help="Find bridges between types")
    land_bridges.add_argument("a", help="First type")
    land_bridges.add_argument("b", help="Second type")
    land_clusters = land_sub.add_parser("clusters", help="Find structural clusters")
    land_clusters.add_argument("--min-size", type=int, default=3)
    
    # ── predict ────────────────────────────────────────────────────────
    pred_parser = subparsers.add_parser("predict", help="Predict physical properties from tuple")
    pred_parser.add_argument("name", help="Name for prediction")
    pred_parser.add_argument("tuple", help="12-primitive tuple string")
    pred_parser.add_argument("--winding", type=int, help="Winding number override")
    pred_parser.add_argument("--json", "-j", action="store_true", help="JSON output")
    
    # ── spectrum ───────────────────────────────────────────────────────
    spec_parser = subparsers.add_parser("spectrum", help="Analyze spectral lines")
    spec_parser.add_argument("wavelengths", nargs="+", type=float, help="Wavelengths in nm")
    spec_parser.add_argument("--json", "-j", action="store_true", help="JSON output")
    
    # ── proof ──────────────────────────────────────────────────────────
    proof_parser = subparsers.add_parser("proof", help="Generate Lean proof scaffold")
    proof_parser.add_argument("name", help="Theorem/definition name")
    proof_parser.add_argument("description", help="Human-readable description")
    proof_parser.add_argument("--tuple", "-t", help="12-primitive tuple")
    proof_parser.add_argument("--save", "-s", action="store_true", help="Save to Lean file")
    proof_parser.add_argument("--ops", "-o", nargs="+", help="IMASM opcode sequence")
    
    # ── discover ───────────────────────────────────────────────────────
    discover_parser = subparsers.add_parser("discover", help="Discover new structures by clicking source with catalog")
    discover_parser.add_argument("source", help="Source system or monomer")
    discover_parser.add_argument("--targets", nargs="*", help="Specific targets (default: all catalog)")
    discover_parser.add_argument("--top", type=int, default=5, help="Top results")
    
    args = parser.parse_args()
    
    if args.command is None:
        parser.print_help()
        return 1
    
    # ── Route commands ─────────────────────────────────────────────────
    
    if args.command == "resonance":
        from structoforge.resonance_engine import StructuralResonance
        sr = StructuralResonance()
        report = sr.couple(args.a, args.b)
        if args.json:
            from dataclasses import asdict
            print(json.dumps(asdict(report), indent=2, default=str))
        else:
            print(sr.explain(report))
    
    elif args.command == "sweep":
        from structoforge.resonance_engine import StructuralResonance
        sr = StructuralResonance()
        results = sr.sweep(args.source, top_n=args.top)
        if args.json:
            from dataclasses import asdict
            print(json.dumps([asdict(r) for r in results], indent=2, default=str))
        else:
            print(f"═══ Sweep: {args.source} → top {args.top} ═══")
            for i, r in enumerate(results):
                print(f"\n[{i+1:2d}] {r.b_name:30s}  d={r.euclidean_distance:.4f}  click={r.click_result}")
                if r.cotype and r.cotype.live_complement:
                    print(f"     Live complement: {r.cotype.live_complement}")
    
    elif args.command == "matrix":
        from structoforge.resonance_engine import batch_resonance_matrix
        matrix = batch_resonance_matrix(args.systems)
        if args.json:
            # Serialize
            out = {}
            for a in args.systems:
                out[a] = {}
                for b in args.systems:
                    if a == b: continue
                    r = matrix[a][b]
                    out[a][b] = {
                        "distance": r.euclidean_distance,
                        "click": r.click_result,
                        "cotype": r.cotype_distance,
                    }
            print(json.dumps(out, indent=2))
        else:
            header = f"{'':20s}" + "".join(f"{s:12s}" for s in args.systems)
            print(header)
            for a in args.systems:
                row = f"{a:20s}"
                for b in args.systems:
                    if a == b:
                        row += f"{'—':12s}"
                    else:
                        r = matrix[a][b]
                        cell = f"{r.euclidean_distance:.2f}"
                        if r.click_result == "fuses":
                            cell += "⚡"
                        row += f"{cell:12s}"
                print(row)
    
    elif args.command == "forge":
        from structoforge.materials_workbench import MaterialsWorkbench
        wb = MaterialsWorkbench()
        sheet = wb.forge(args.name, *args.monomers, register=args.register)
        if args.json:
            from dataclasses import asdict
            print(json.dumps(asdict(sheet), indent=2, default=str))
        else:
            print(wb.report(sheet.name))
    
    elif args.command == "landscape":
        from structoforge.landscape_explorer import LandscapeExplorer
        exp = LandscapeExplorer()
        
        if args.landscape_action == "summary":
            print(exp.summary())
        elif args.landscape_action == "neighborhood":
            hood = exp.neighborhood(args.name, args.radius)
            print(f"═══ Neighborhood: {hood.center} (r={args.radius}) ═══")
            print(f"  Members: {len(hood.members)}")
            print(f"  Nearest: {', '.join(hood.nearest)}")
            print(f"  Paradigm distribution: {hood.paradigm_distribution}")
            print()
            for name, dist, axes in hood.members[:15]:
                print(f"  {name:25s} d={dist}  {', '.join(axes)}")
            if len(hood.members) > 15:
                print(f"  ... and {len(hood.members) - 15} more")
        elif args.landscape_action == "bridges":
            bridges = exp.bridges(args.a, args.b)
            print(f"═══ Bridges: {args.a} ↔ {args.b} ═══")
            for name, d_a, d_b in bridges:
                print(f"  {name:25s} d(A)={d_a} d(B)={d_b} total={d_a+d_b}")
        elif args.landscape_action == "clusters":
            clusters = exp.clusters(min_size=args.min_size)
            print(f"═══ Structural Clusters ═══")
            for center, members in sorted(clusters.items(), key=lambda x: -len(x[1])):
                print(f"  {center:25s} ({len(members)} members)")
        else:
            print("Landscape action required: summary, neighborhood, bridges, or clusters")
    
    elif args.command == "predict":
        from structoforge.winding_predictor import predict_from_tuple
        pred = predict_from_tuple(args.name, args.tuple, args.winding)
        if args.json:
            from dataclasses import asdict
            print(json.dumps(asdict(pred), indent=2, default=str))
        else:
            print(f"═══ Physical Prediction: {pred.name} ═══")
            print(f"  α⁻¹ = {pred.alpha_inv:.6f}  (ref: 137.036, Δ={pred.alpha_delta:.6f})")
            print(f"  sin²θ_W = {pred.sin2_theta_w:.6f}  (ref: 0.23122, Δ={pred.sin2_delta:.6f})")
            print(f"  m_e/m_p = {pred.me_over_mp:.10f}")
            for name, val in pred.mass_ratios.items():
                print(f"  {name} = {val:.6f}")
            for name, val in pred.energy_scales.items():
                print(f"  {name} = {val:.6f}")
            if pred.critical_temperature:
                print(f"  T_c = {pred.critical_temperature:.4e} eV")
                print(f"  Critical exponent = {pred.critical_exponent}")
            if pred.spectral_lines:
                print(f"  Spectral lines:")
                for name, energy in pred.spectral_lines:
                    print(f"    {name:30s} {energy:.4f} eV / {1240/energy:.2f} nm")
    
    elif args.command == "spectrum":
        from structoforge.winding_predictor import analyze_spectrum
        result = analyze_spectrum(args.wavelengths)
        if args.json:
            print(json.dumps(result, indent=2, default=str))
        else:
            print("═══ Spectral Analysis ═══")
            print(f"  Lines: {result['n_lines']}")
            print(f"  Energy range: {result['min_energy_eV']:.4f} – {result['max_energy_eV']:.4f} eV")
            print(f"  Hydrogenic series: {result['is_hydrogenic_series']}")
            print(f"  Anchor scale: {result['anchor_scale_keV']:.2f} keV (m_e c²)")
            print(f"  Tilt angle: {result['tilt_deg']:.4f}°")
            if result['estimated_rydberg_eV']:
                print(f"  Estimated Rydberg: {result['estimated_rydberg_eV']:.4f} eV")
    
    elif args.command == "proof":
        from structoforge.proof_forge import ProofForge
        pf = ProofForge()
        scaffold = pf.forge(args.name, args.description, args.tuple, args.ops)
        print(f"═══ Proof Scaffold: {scaffold.name} ═══")
        print(scaffold.lean_code)
        if args.save:
            path = scaffold.save()
            print(f"\nSaved to: {path}")
    
    elif args.command == "discover":
        from structoforge.resonance_engine import StructuralResonance
        from structoforge.landscape_explorer import NatureRegistry
        
        sr = StructuralResonance()
        reg = NatureRegistry()
        
        targets = args.targets or [n for n in reg.names() if n != args.source]
        
        print(f"═══ Discover: clicking {args.source} against {len(targets)} targets ═══")
        results = sr.sweep(args.source, targets=targets, top_n=args.top)
        
        print(f"\nTop {args.top} compatible structures:")
        fusing = [r for r in results if r.click_result == "fuses"]
        if fusing:
            print(f"\n  ⚡ FUSING PAIRS:")
            for r in fusing:
                print(f"    {r.a_name} ⟷ {r.b_name}  d={r.euclidean_distance:.4f}")
                if r.cotype and r.cotype.live_complement:
                    print(f"      Live complement: {r.cotype.live_complement}")
        
        print(f"\n  Closest by distance:")
        for r in results[:5]:
            print(f"    {r.b_name:30s} d={r.euclidean_distance:.4f}  {r.click_result}")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())
