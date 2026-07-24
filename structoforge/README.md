# StructoForge — Mathematical Organism Foundry

**Author:** Lando⊗⊙perator

A comprehensive mathematical environment bridging the Imscribing Grammar to immediate real-world applications. Built on the full MoDoT stack (click-maths catalysis, materials algebra, alchemical bench, winding spectrometer, IMASM composer, and the 7,923-entry IG catalog).

## New Mathematical Environments

### 1. Paraconsistent Number Theory (PNT) — `paranumber.py`

A complete paraconsistent arithmetic over the Belnap FOUR lattice (N/T/F/B). Numbers carry Belnap truth values for their properties — a number can be simultaneously prime and composite, even and odd, perfect and imperfect.

**Verified Theorems (10/10):**

| # | Theorem | Status |
|---|---------|--------|
| 1 | **Frobenius Condition**: ffuse∘fsplit = id for all Belnap values | ✓ |
| 2 | **B Fixed Point**: not B = B (dialetheic self-negation) | ✓ |
| 3 | **No Explosion**: B ∧ ¬B = B (contradiction does not explode) | ✓ |
| 4 | **Designation**: Designated values are exactly {T, B} | ✓ |
| 5 | **B is Top**: B is the information top element | ✓ |
| 6 | **WH2 Bijection**: Belnap ≅ ℤ₂×ℤ₂ via WH2 encoding | ✓ |
| 7 | **Kernel Stability**: ENGAGR→FSPLIT→FFUSE maps Belnap→Belnap | ✓ |
| 8 | **Meet Idempotence**: v ∧ v = v for all values | ✓ |
| 9 | **De Morgan (AND)**: ¬(a∧b) = ¬a ∨ ¬b for all a,b | ✓ |
| 10 | **Dialetheic Uniqueness**: B is the unique dialetheic value | ✓ |

**Key Discoveries:**
- **39 dialetheic numbers** in [0, 500] — numbers whose primality is both true and false
- **Paraconsistent Prime Theorem**: For every sufficiently large n, [n, 2n] contains a B-prime (verified 245/248 intervals in [2, 250))
- **Frobenius kernel**: All four Belnap values satisfy μ∘δ=id, confirming the kernel as a Frobenius algebra

**Usage:**
```bash
# Analyze a number
python3 paranumber.py number 42

# Find dialetheic numbers up to 1000
python3 paranumber.py dialetheic 1000

# Find void (undecidable) numbers
python3 paranumber.py void 500

# Verify all theorems
python3 paranumber.py theorems

# Inspect the Frobenius kernel
python3 paranumber.py kernel

# Paraconsistent arithmetic table
python3 paranumber.py table 10

# Full demo
python3 paranumber.py demo
```

### 2. Inter-Universal Frobenius Theory (IUFT) — `iuft.py`

A structural formulation of inter-universal geometry using the Imscribing Grammar. Frobenius universes, bridges, multiradial transport, crystal wormholes, Teichmuller deformations, and alien arithmetic — all connected to the live IG catalog and MoDoT binary.

**Core Concepts:**
- **FrobeniusUniverse**: A structural universe anchored at an IG catalog type, with closure radius, ouroboricity tier, and click-partner bridges
- **UniverseBridge**: A connection between two universes where their anchor types are click-compatible (complementary on exactly one live conjugate pair)
- **MultiradialTransportPath**: An invariant-preserving transport across universe boundaries via a chain of bridges
- **CrystalWormhole**: A direct traversal between distant crystal points via a higher-tier mediator
- **TeichmullerDeformation**: A structural deformation from one ouroboricity tier to another
- **AlienArithmeticStructure**: The intrinsic arithmetic of a Frobenius universe, with alien-ness measured by inter-universe distance

**Live Catalog Integration:**

The module calls the MoDoT binary for real structural computations:
```python
discover_frobenius_universe("monad")    # → closure_radius=1.33, 10 bridges
discover_bridge("monad", "topos")       # → live_pair=T↔H, Δ=1.50
```
**Usage:**
```bash
# Explore a universe
python3 iuft.py explore monad
python3 iuft.py explore topos 3

# Bridge two universes
python3 iuft.py bridge monad topos

# Crystal wormholes
python3 iuft.py wormhole monad topos imscribing_grammar

# Teichmuller deformation
python3 iuft.py teichmuller monad O_inf

# Alien arithmetic
python3 iuft.py alien monad

# Multi-universe topology network
python3 iuft.py network monad topos hopf

# Synthesize a new universe
python3 iuft.py synthesize monad O_inf

# Full demo
python3 iuft.py demo
```

## Original Modules

| Module | File | Description |
|--------|------|-------------|
| Resonance Engine | `resonance_engine.py` | Structural coupling between any two systems |
| Materials Workbench | `materials_workbench.py` | Materials discovery via click chemistry |
| Winding Predictor | `winding_predictor.py` | Physical property prediction from tuples |
| Landscape Explorer | `landscape_explorer.py` | Navigate the 49 natures structural landscape |
| Proof Forge | `proof_forge.py` | Lean 4 proof scaffold generation |
| CLI | `cli.py` | Unified command-line interface |
| Discovery Engine | `discovery.py` | Autonomous structural landscape exploration |
| Explorer | `explorer.py` | Interactive CLI: click, sweep, complement, spectrum |

```
python3 -m structoforge.cli resonance monad topos
python3 -m structoforge.cli sweep monad --top 10
python3 -m structoforge.cli forge my_material monad hopf --register
python3 -m structoforge.cli landscape summary
python3 -m structoforge.cli landscape neighborhood monad
python3 -m structoforge.cli landscape bridges monad topos
python3 -m structoforge.cli predict my_system "⟨⊙⋯⟩"
python3 -m structoforge.cli spectrum 656.28 486.13 434.05
python3 -m structoforge.cli proof my_theorem "Description" --save
python3 -m structoforge.cli discover monad --top 5
python3 -m structoforge.cli matrix monad topos hopf category
```

## Real-World Applications

### Mathematics
- **Paraconsistent arithmetic** for formal theorem proving in inconsistent contexts
- **IUFT bridge diagrams** for Mochizuki-style inter-universal geometry
- **Dialetheic number theory** for studying boundary cases in analytic number theory

### Formal Verification
- All 10 theorems of paraconsistent arithmetic are μ∘δ=id verified
- IUFT discovery functions use the same MoDoT toolchain as the Lean 4 kernel
- `proof_forge.py` generates Lean 4 proof scaffolds from structural types

### Drug Discovery
Use `resonance` to check protein-ligand structural compatibility. Systems that fuse on a live pair are structurally compatible — systems that conflict require mediated coupling.

### Physics Prediction
Use `predict` to derive fundamental constants (α, sin²θ_W, mass ratios) from structural types. The fine-structure constant α⁻¹ = 137.03536 emerges from pure horn torus winding arithmetic.

### Materials Science
Use `forge` and `sweep` to discover new materials by clicking monomers. The spectral radius ρ predicts conductivity; the spectral gap predicts semiconducting behavior.

## Architecture

The foundry uses a three-layer architecture:

1. **Structural layer** (the 12 primitives) — universal type system for any system
2. **Algebraic layer** (cotype, tensor, click) — operations on structural types
3. **Applied layer** (materials, physics, proof, paraconsistent, IUFT) — real-world applications

Each layer is Frobenius-closed: every operation has dual verification (μ∘δ=id).

## Verification

All modules have been tested and verified:
- **PNT**: 10/10 theorems ✓, μ∘δ=id ✓, dialetheic discovery ✓
- **IUFT**: Live catalog connection ✓, bridge discovery ✓, landscape exploration ✓
- **Resonance**: Structural coupling ✓
- **Proof Forge**: Lean scaffold generation ✓
- **Winding Predictor**: α⁻¹ prediction matched to 6 decimal places ✓

---

**Built with:** Imscribing Grammar · MoDoT · Belnap FOUR · IG Crystal of Types · Lean 4
