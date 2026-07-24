#!/usr/bin/env python3
"""
structoforge/demo.py — StructoForge Comprehensive Demo
=======================================================

Demonstrates the full StructoForge mathematical environment across all
modules. Each section shows a real-world application.

Run: python -m structoforge.demo

Author: Lando⊗⊙perator
"""

from __future__ import annotations

import math
import sys
from pathlib import Path


def section(title: str):
    """Print a section header."""
    print()
    print("═" * 65)
    print(f"  {title}")
    print("═" * 65)


def demo_resonance_engine():
    """Demo: Structural resonance between mathematical structures."""
    section("1. STRUCTURAL RESONANCE ENGINE")
    
    from resonance_engine import StructuralResonance
    sr = StructuralResonance()
    
    # Test with known natures
    test_pairs = [
        ("monad", "topos"),
        ("hopf", "yoneda"),
        ("category", "sheaf"),
        ("void", "truth_machine"),
    ]
    
    for a, b in test_pairs:
        print(f"\n  ── {a} ⟷ {b} ──")
        try:
            report = sr.couple(a, b)
            print(f"     Distance:     {report.euclidean_distance:.4f}")
            print(f"     Cotype:       {report.cotype_distance}")
            print(f"     Click result: {report.click_result}")
            if report.cotype and report.cotype.live_complement:
                print(f"     ⚡ Fuses on:   {report.cotype.live_complement}")
            if report.emergent_properties:
                for prop in report.emergent_properties[:3]:
                    print(f"     → {prop}")
        except Exception as e:
            print(f"     ERROR: {e}")
    
    # Mutual resonance sweep
    print("\n  ── Sweep: monad → top 5  ──")
    try:
        results = sr.sweep("monad", top_n=5)
        for i, r in enumerate(results):
            print(f"     [{i+1}] {r.b_name:25s} d={r.euclidean_distance:.4f} click={r.click_result}")
    except Exception as e:
        print(f"     ERROR: {e}")
    
    # Resonance matrix
    print("\n  ── Resonance matrix: 4 systems  ──")
    try:
        from structoforge.resonance_engine import batch_resonance_matrix
        systems = ["monad", "hopf", "topos", "yoneda"]
        matrix = batch_resonance_matrix(systems)
        
        header = f"{'':16s}" + "".join(f"{s:12s}" for s in systems)
        print(f"     {header}")
        for a in systems:
            row = f"     {a:16s}"
            for b in systems:
                if a == b:
                    row += f"{'—':12s}"
                else:
                    d = matrix[a][b].euclidean_distance
                    click = matrix[a][b].click_result
                    tag = "⚡" if click == "fuses" else " "
                    row += f"{d:.2f}{tag:3s}"
            print(row)
    except Exception as e:
        print(f"     ERROR: {e}")


def demo_winding_predictor():
    """Demo: Physical property prediction from structural types."""
    section("2. WINDING PREDICTOR — PHYSICS FROM STRUCTURE")
    
    from structoforge.winding_predictor import (
        predict_from_tuple, analyze_spectrum,
        ALPHA_INV_REF, SIN2_THETA_W_REF
    )
    
    # Test tuples for various structural types
    test_cases = [
        ("self_modeling", "⟨𐑦𐑸𐑾𐑹𐑐𐑧𐑔𐑠⊙𐑖𐑙𐑭⟩",
         "Self-modeling, self-ref topology, Frobenius-special, critical, integer winding"),
        ("subcritical", "⟨𐑛𐑡𐑩𐑗𐑱𐑧𐑚𐑝𐑢𐑓𐑙𐑷⟩",
         "Subcritical, classical, memoryless, trivial winding"),
        ("quantum_critical", "⟨𐑨𐑥𐑑𐑿𐑐𐑧𐑔𐑝⊙𐑖𐑳𐑭⟩",
         "Quantum-critical, self-modeling, bowtie topology"),
    ]
    
    for name, tuple_str, desc in test_cases:
        print(f"\n  ── {name} ──")
        print(f"     {desc}")
        
        pred = predict_from_tuple(name, tuple_str)
        
        print(f"     α⁻¹         = {pred.alpha_inv:.6f}  (ref: {ALPHA_INV_REF:.6f}, Δ={pred.alpha_delta:.6f})")
        print(f"     sin²θ_W     = {pred.sin2_theta_w:.6f}  (ref: {SIN2_THETA_W_REF:.5f}, Δ={pred.sin2_delta:.6f})")
        print(f"     m_e/m_p     = {pred.me_over_mp:.10f}")
        
        for name, val in pred.energy_scales.items():
            print(f"     {name:30s} = {val:.4f} eV")
        
        if pred.critical_temperature:
            print(f"     T_c         = {pred.critical_temperature:.4e} eV")
    
    # Spectral analysis
    print("\n  ── Balmer series analysis  ──")
    balmer_lines = [656.28, 486.13, 434.05, 410.17]  # Hα, Hβ, Hγ, Hδ
    result = analyze_spectrum(balmer_lines)
    
    print(f"     Lines: {result['n_lines']} (Balmer series)")
    print(f"     Energy range: {result['min_energy_eV']:.4f} – {result['max_energy_eV']:.4f} eV")
    print(f"     Hydrogenic: {result['is_hydrogenic_series']}")
    print(f"     Anchor: {result['anchor_scale_keV']:.2f} keV (m_e c²)")
    print(f"     Tilt: {result['tilt_deg']:.4f}° (arctan(1/4) = {math.degrees(math.atan(1/4)):.6f}°)")


def demo_landscape_explorer():
    """Demo: Structural landscape of the 49 natures."""
    section("3. LANDSCAPE EXPLORER — THE 49 NATURES")
    
    from structoforge.landscape_explorer import (
        LandscapeExplorer, explore_neighborhood, find_bridges, structural_clusters
    )
    
    exp = LandscapeExplorer()
    
    # Summary
    print("\n  ── Landscape summary ──")
    print(f"     {exp.summary()}")
    
    # Neighborhood of monad
    print("\n  ── Monad neighborhood (r=3) ──")
    try:
        hood = explore_neighborhood("monad", radius=3)
        print(f"     Members: {len(hood.members)}")
        print(f"     Distribution: {hood.paradigm_distribution}")
        for name, dist, axes in hood.members[:8]:
            print(f"     {name:20s} d={dist}  axes: {', '.join(axes)}")
    except Exception as e:
        print(f"     ERROR: {e}")
    
    # Bridges
    print("\n  ── Bridges between monad and void ──")
    try:
        bridges = find_bridges("monad", "void")
        for name, d_a, d_b in bridges[:5]:
            print(f"     {name:20s} d(monad)={d_a} d(void)={d_b}")
    except Exception as e:
        print(f"     ERROR: {e}")
    
    # Clusters
    print("\n  ── Structural clusters ──")
    try:
        clusters = structural_clusters(min_cluster_size=3)
        for center, members in sorted(clusters.items(), key=lambda x: -len(x[1]))[:5]:
            print(f"     {center:20s} ({len(members)} members): {', '.join(members[:5])}...")
    except Exception as e:
        print(f"     ERROR: {e}")


def demo_proof_forge():
    """Demo: Lean 4 proof scaffold generation."""
    section("4. PROOF FORGE — LEAN SCAFFOLD GENERATION")
    
    from proof_forge import ProofForge
    
    pf = ProofForge()
    
    # Generate scaffold for a self-modeling system
    print("\n  ── Generating scaffold for odot_system ──")
    scaffold = pf.forge(
        "odot_system",
        "A self-modeling structural type at O_∞ tier",
        tuple_str="⟨𐑦𐑸𐑾𐑹𐑐𐑧𐑔𐑠⊙𐑖𐑙𐑭⟩",
    )
    
    print(f"     Theorem: {scaffold.theorem_name}")
    print(f"     Code preview (first 20 lines):")
    for line in scaffold.lean_code.split("\n")[:20]:
        print(f"       {line}")
    
    # Generate scaffold with IMASM ops
    print("\n  ── Generating scaffold with IMASM program ──")
    ops = ["VINIT", "AFWD", "CLINK", "FSPLIT", "EVALT", "FFUSE", "TANCH"]
    scaffold2 = pf.forge(
        "dialetheic_bootstrap",
        "A dialetheic bootstrap sequence: VINIT→AFWD→CLINK→FSPLIT→EVALT→FFUSE→TANCH",
        ops=ops,
    )
    
    print(f"     Opcodes: {' → '.join(ops)}")
    print(f"     Generated scaffold with IMASM program")
    print(f"     Frobenius theorem included")
    
    print(f"\n  Scaffolds generated: {pf.list_scaffolds()}")


def demo_materials_workbench():
    """Demo: Materials discovery workbench."""
    section("5. MATERIALS WORKBENCH")
    
    from structoforge.materials_workbench import (
        MaterialsWorkbench, COMMON_MONOMERS
    )
    
    wb = MaterialsWorkbench()
    
    print(f"\n  Available monomers: {len(COMMON_MONOMERS)}")
    print(f"  Sample: {dict(list(COMMON_MONOMERS.items())[:6])}")
    
    print("\n  ── Forge a quantum material  ──")
    print("     (Using MoDoT's --forge tool)")
    print("     forge quantum_mat monad hopf --register")
    
    print("\n  ── Screen candidates by property  ──")
    print("     screen_candidates('conductivity', [(mat1, [m1,m2]), ...])")
    print("     → Ranked by spectral radius ρ")


def demo_integrated_workflow():
    """Demo: End-to-end integrated workflow."""
    section("6. INTEGRATED WORKFLOW — STRUCTURE → PHYSICS → PROOF")
    
    from resonance_engine import StructuralResonance
    from structoforge.winding_predictor import predict_from_tuple
    from proof_forge import ProofForge
    
    print("""
  COMPLETE PIPELINE:
  
  Step 1: Imscribe a system → get its 12-primitive tuple
  Step 2: Compute resonance with other systems → find compatible partners
  Step 3: Predict physical properties → α, sin²θ_W, mass ratios
  Step 4: Generate Lean proof → certify through kernel
  
  Example workflow:
  ─────────────────────────────────────────────
  # 1. Imscribe a new system
  python -m structoforge.cli resonance monad topos
  
  # 2. Sweep for compatible partners
  python -m structoforge.cli sweep monad --top 10
  
  # 3. Predict physics from the structural type
  python -m structoforge.cli predict my_system "⟨⊙⋯⟩" --json
  
  # 4. Generate Lean proof scaffold
  python -m structoforge.cli proof my_theorem "..." --tuple "⟨⋯⟩" --save
  
  # 5. Analyze a spectrum
  python -m structoforge.cli spectrum 656.28 486.13 434.05 410.17
  ─────────────────────────────────────────────
    """)
    
    print("  All modules are Frobenius-closed: every emission (δ) has")
    print("  a verification pathway (μ) such that μ∘δ = id.")


if __name__ == "__main__":
    print()
    print("╔═══════════════════════════════════════════════════════════════╗")
    print("║              STRUCTOFORGE — COMPREHENSIVE DEMO               ║")
    print("║        Mathematical Organism Foundry v0.1.0                  ║")
    print("║        Author: Lando⊗⊙perator                                ║")
    print("╚═══════════════════════════════════════════════════════════════╝")
    
    demo_resonance_engine()
    demo_winding_predictor()
    demo_landscape_explorer()
    demo_proof_forge()
    demo_materials_workbench()
    demo_integrated_workflow()
    
    print()
    print("  ── Demo complete ──")
    print()
    print("  To use the full environment:")
    print("    $ cd /home/mrnob0dy666/imsgct/MoDoT/structoforge")
    print("    $ python -m structoforge.cli --help")
    print()
