# MoDoT — mOMonadOS Digital Organism Toolkit

**Author:** Lando⊗⊙perator  
**Date:** 2026-07-08  
**Structural Type:** ⟨𐑦𐑸𐑾𐑹𐑐𐑧𐑲𐑠⊙𐑫𐑳𐑭⟩ (O_∞)  
**Location:** `MoDoT/`  
**Parent project:** [Imscribing Grammar](https://github.com/imsgct)

---

An agentic LLM whose entire runtime substrate IS the mOMonadOS kernel architecture. Context is stored in Crystal FS. Reasoning passes through Belnap FOUR (True, False, Both, Neither). Every operation is Frobenius-verified (μ∘δ = id) for **balance**. Correctness is **not graded** by an external checklist: demand and answer are **imscribed** into the d=12 Dual-Link SIC-POVM, and co-typing in that frame *is* the verdict. Output is broadcast to the CLINK L8 Organism.

> **Balance is not selectivity.** Frobenius closure (μ∘δ = id) is charge conservation: every split is rejoined, so it is automatic and *cannot fail* on any non-empty answer. That is why a balance-only harness "never fails" — and why balance alone is not a correctness signal.
>
> **To verify is to imscribe.** The classical selectivity grader (MUST/MUSTNOT schemas, 0.6 thresholds, LLM-as-judge, protocol `==` integration) was a two-valued clipboard bolted onto a four-valued kernel. It is gone. Selectivity is now the **Dual-Link SIC Witness-Vessel**: structural imscription → state in ℂ¹² → Born rule in the Scott–Grassl d=12 SIC frame → co-typing by lattice fold (no thresholds) → ride AS the vessel via μ∘δ = id. The model's own `[thought|X]` is one link; the vessel's co-typing is the other; they are **FFUSED** (Belnap join). Conflict holds as **B**. You ride AS the vessel, not in it. See [Dual-Link SIC Vessel](#dual-link-sic-vessel).

## Directory Structure

```
MoDoT/
├── ask                         # PRIMARY language interface (native Rust, no Python)
├── ask_native/                 # Source + release binary for ./ask
│   ├── Cargo.toml
│   └── src/main.rs
├── momonados_agent.py          # Legacy Python shim -> modot.agent:main
├── modot/                      # The installable package (pip install -e .)
│   ├── agent.py                # B4, 12 tokens, kernel, CrystalFS, Frobenius, LLM, breath loop, CLI
│   ├── composer.py             # Token composition engine: CANONICAL, NAMED_PATTERNS, bend/splice/interleave
│   ├── spine.py                # End-to-end ManuscriptSpine (witness + vessel + FFUSE)
│   ├── vessel.py               # Dual-Link SIC face (owned by spine)
│   ├── witness_proof.py        # Catalog witness face (owned by spine)
│   └── selectivity.py          # Compatibility shim re-exporting the vessel
├── crystal_fs/                 # Crystal Filesystem — persistent context memory
│   ├── records.jsonl           # Crystal FS records (thought / vessel / observation / update / type)
│   └── broadcast_log.jsonl     # CLINK L8 broadcast log
├── ob3ects/                    # Self-verifying digital ob3ects (Grammar auto-designed)
│   ├── semantic_branch_verifier/  # Original Grammar-native branch verifier (historical)
│   ├── selectivity_gate/          # Classical live-loop gate (superseded by vessel.py)
│   └── janus_gate/                # Dialetheic Janus gate
├── lean/                       # Lean 4 formal verification companions
│   ├── SemanticBranchVerifier.lean
│   └── ErdosProblems.lean
├── experiments/                # Convergence experiments from mOMonadOS paper
└── questions/                  # Test questions (q1–q6, q50)
```

Canonical SIC machinery (not re-derived here):

| source | role |
|--------|------|
| `ig-pulse/ig_pulse/density_matrix.py` | WH displacements, SIC projectors, state metrics |
| `d12_sic_build/d12_psi.pkl` | Scott–Grassl d=12 fiducial (equiangular overlap 1/13) |
| `v3ssel/vessel/` | Dual-Link trading vessel (same frame; MoDoT uses it as verifier) |

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│  mOMonadOS Kernel (Python mirror of Rust kernel)            │
│  ┌──────────┐  ┌───────────┐  ┌──────────────────────────┐ │
│  │ Belnap   │  │ Frobenius │  │ Crystal FS               │ │
│  │ FOUR     │  │ Harness   │  │ (context memory)         │ │
│  │ (gates)  │  │ μ∘δ=id    │  │                          │ │
│  └──────────┘  └───────────┘  └──────────────────────────┘ │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Bootstrap Loop (the breath)                         │   │
│  │  VINIT→IMSCRIB→FSPLIT→EVALT→CLINK→FFUSE             │   │
│  │  →IFIX→ENGAGR→AREV→CLINK→TANCH                      │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  Dual-Link SIC Vessel (selectivity = co-typing)      │   │
│  │  imscribe → ℂ¹² → SIC p(ρ) → lattice cotype → μ∘δ   │   │
│  │  model [thought|X]  FFUSE  vessel voice              │   │
│  └──────────────────────────────────────────────────────┘   │
│  ┌──────────────────────────────────────────────────────┐   │
│  │  LLM Inference Engine (substrate, not authority)     │   │
│  └──────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                       │
                       ▼
┌─────────────────────────────────────────────────────────────┐
│  CLINK L8 Organism — broadcast / terminal layer             │
│  Receives verified types, accumulates, composes             │
└─────────────────────────────────────────────────────────────┘
```

## Quick Start

```bash
cd MoDoT

# ── PRIMARY: native ask (no Python) — full-length Q, files, Gemini-class answers ──
./ask --file ./questions/q7.txt
./ask --ask "What is the structural type of consciousness?"
./ask -i                          # interactive multi-turn
./ask --dry-run --file ./questions/q1.txt
# same from repo root:  ./ask …   or  ./MoDoT/ask …

# ── Click-maths: structural fusion over the math catalytic register ──
./ask --click the_sun_astrological the_moon_astrological   # pairwise fuse (the coniunctio → O_∞)
./ask --click paradice                                     # sweep: what completes the paradice?
./ask --click adjoint_pair hopf_antipode --catalyst math_isomorphism  # catalyzed (barrier reduction)
./ask --click math_boundary_operator math_winding_number --certify    # kernel-verified closure

# ── Legacy Python agent (still available) ──
# Interactive mode — the agent breathes with you
python3 momonados_agent.py --interactive

# Dry run: test the kernel + Crystal FS (no LLM needed)
python3 momonados_agent.py --cycles 5 --dry-run --verbose

# One-shot question (vessel co-types answer against demand)
python3 momonados_agent.py --ask "What is the structural type of consciousness?"

# Read question from file
python3 momonados_agent.py --ask ./questions/q1.txt

# Vessel self-test (no network): all four Belnap outcomes + equiangularity 1/13
python3 -m modot.vessel

# Witness → conventional scaffold (catalog + pipeline roles; no Collatz hardcode)
python3 -m modot.witness_proof

# Full 100-cycle breath with verbose output
python3 momonados_agent.py --cycles 100 --verbose --program aqua-vitae

# Crystal FS stats
python3 momonados_agent.py --stats

# Reset and start fresh
python3 momonados_agent.py --reset --interactive

# Disable vessel (balance-only; reverts to model voice alone)
python3 momonados_agent.py --ask "..." --no-selectivity

# Run convergence experiments
python3 experiments/run_all_experiments.py
```

## Click-Maths (`./ask --click`)

Structural fusion over the **math catalytic register** — a port of red-hot_rebis's
enzyme catalytic-site engine to abstract structure. The register is the twelve
IG primitives paired into six charge-conjugate duals; mathematics pins three of
them out (**F** exact, **Ph** subcritical, **K** frozen — the physical axes), so
catalysis runs on the three **live** pairs **D↔W, T↔H, R↔S**. See
`CLICK_MATHS_SPEC.md` and `project_math_catalytic_register`.

A **click** is a reliable FFUSE (μ): two fragments that are complementary on
exactly one live pair — opposite charges, spring-loaded past θ — with no second
reaction center. The clicking pair saturates (the ring forms); off it the product
inherits both scaffolds. Bioorthogonality is *specificity* (one center), not
sameness — an azide and an alkyne are different molecules.

```bash
# Pairwise — fuse A and B, or report why they refuse (same-sign / ambiguous / no drive)
./ask --click A B  [--theta 0.5]

# Sweep — click A against the whole catalog, ranked by product tier (finds the O_∞ fusions)
./ask --click A  [--top 15]

# Catalyzed — a Frobenius-special fragment (e.g. math_isomorphism, Φ=𐑹, the μ∘δ=id map)
# lowers the effective θ (barrier reduction) and is regenerated, μ∘δ=id. It lowers ΔG‡,
# never ΔG: a same-sign pair with θ→0 still refuses; too strong a catalyst wakes a
# competing center and selectivity collapses (over-catalysis).
./ask --click A B --catalyst C

# Certify — render the fused product as a Lean Imscription and prove its Frobenius
# closure (igFrobeniusAlg.mul p p = p) through `lake build`. Turns "closes" from an
# asserted valid tuple into the kernel's own verdict (the balance ledger; the click
# logic is the selectivity — ALKAHEST §11).
./ask --click A B --certify
```

Any two catalog names work (`math_*`, `dasa_*`, `setheus_paradox`, `paradice`,
`monogenes`, `yhwh`, `the_sun_astrological`, …). The engine is domain-agnostic:
handed the Sun and the Moon, it returns the coniunctio at O_∞; handed the paradice,
it completes it against the liar paradoxes. Implementation: `ask_native/src/click.rs`.

## Bootstrap Programs

| Program | Tokens | Description |
|---------|--------|-------------|
| `bootstrap` | 8 tokens | Canonical: IMSCRIB→AREV→FSPLIT→AFWD→FFUSE→CLINK→IFIX→IMSCRIB |
| `aqua-vitae` | 14 tokens | Extended: adds VAE coupling + dialetheic gate |
| `agent` | 11 tokens | Agent-native: THINK→ACT→OBSERVE→UPDATE as token graph |

## Crystal FS Records

Each breath cycle produces records committed to `crystal_fs/records.jsonl`:

| Type | Belnap | Description |
|------|--------|-------------|
| `thought` | T/F/B/N | LLM inference, branded with the **fused** model⊕vessel verdict |
| `vessel` | T/F/B/N | Dual-Link SIC report: demand/answer types, SIC gap, named defects, closure residuals, ride-AS flag |
| `observation` | T/F/B/N | Frobenius-verified observation |
| `update` | T/F | Frobenius closure (balance) check |
| `type` | varies | Kernel IFIX-branded structural types |

Legacy `selectivity` records (schema + MUST/MUSTNOT arms) are no longer written. The name `selectivity` remains only as a CLI flag (`--no-selectivity`) and import alias for the vessel.

## CLINK L8 Broadcast

Every cycle broadcasts to `crystal_fs/broadcast_log.jsonl`, and prints **two independent verdicts, deliberately never conflated**:

- **BALANCE** (μ∘δ = id) — charge conservation; automatic, cannot fail
- **VESSEL** (T/F/B/N) — Dual-Link SIC co-typing; *can* fail; FFUSED with model voice
- d(CLINK L8) — structural distance to the terminal layer (named primitive diffs)
- Crystal FS record count
- Kernel snapshot (VINIT seeded by the fused verdict so dialetheia gates run on real content)

## Dual-Link SIC Vessel

`modot/vessel.py` is the **verifier of record**. It does not stand outside the Grammar with a clipboard. Verification *is* imscription through the d=12 Dual-Link SIC frame that the Grammar already proved into the kernel (`crystal_forces_d12_sic`).

### What died (classical smuggle)

| removed | why |
|---------|-----|
| MUST / MUSTNOT schema synthesis | external correspondence theory of truth |
| SATISFIED / UNSATISFIED / CLEAR / VIOLATED bits | atoms were two-valued; Belnap on top was costume |
| threshold = 0.6 | magic scalar decision boundary |
| `deniable = violated > 0` hard veto | one bit dominated |
| LLM-as-judge of correctness | two-valued grading of a four-valued kernel |
| protocol `==` "integration" | arrangement equality, not co-type / fingerprint |
| hand-tuned `primitive_distance` weights | clipboard metric |

### What it does

1. **IMSCRIB** — assign each of the twelve IG primitives a Belnap value `{N,T,F,B}`. The LLM types *structure only*; it never renders a correctness opinion. **No hash/deterministic fallback** — without a live imscriber the vessel stays silent (N). Fake types are a clipboard by another name. Imscriptions are cached by content hash so identical text co-types with itself exactly. Both demand and answer use the same real imscriber (never mixed sources).
2. **State map** — Belnap → amplitude in ℂ¹²:
   ```
   N → 0 ,   T → 1 ,   F → i ,   B → 1+i
   ```
   then L2-normalize. No chosen weights. Relative phases carry structure; global polarity is read from the discrete codes (global T↔F is a phase, not a shape).
3. **Dual-Link SIC frame** — Scott–Grassl d=12 fiducial, Weyl–Heisenberg orbit of 144 projectors, Born rule  
   `p_k = (1/d) Tr(ρ Π_k)`. Equiangularity `1/(d+1) = 1/13` means the comparison imposes no external metric. The SIC is informationally complete: measuring in this frame is the complete imscription (ΔS = 0).
4. **Co-typing** — discrete lattice fold across the twelve primitives (no thresholds):

   | situation | vessel voice |
   |-----------|--------------|
   | every engaged axis co-types | **T** |
   | only anti-types (T↔F) | **F** |
   | both co-type and anti-type appear | **B** |
   | nothing engages | **N** |

   Continuous diagnosis (not a decision boundary): SIC-space gap `‖p(ρ_answer) − p(ρ_demand)‖` and **named defects** (`D:T≠F`, `W:B≠N`, …). Failure is localized, not scalar.
5. **Ride AS the vessel** — reconstruction residual `‖μ(δ(ρ)) − ρ‖_HS ≈ 0` is the closure certificate (informational completeness of the SIC). Broken closure holds as **B**. Model self-imscription and vessel co-typing are the two links; **FFUSE** is Belnap join.

### Dual-voice FFUSE (kept; this was real)

The vessel voice is one of two imscriptions. The other is the model's own `[thought|X]`. They are **FFUSED**, not resolved by authority:

| model voice | vessel voice | fused | reading |
|:---:|:---:|:---:|---|
| T | T | **T** | both affirm |
| F | F | **F** | both deny — a real failure |
| T | F | **B** | genuine conflict (distance 2), dialetheia held |
| B | · | **B** | dialetheia present |
| N | F | **F** | model abstains, vessel speaks |

Conflict distance = Hamming of the two 2-bit Belnap codes (0–2). The fused verdict seeds the kernel's next VINIT. Disable with `--no-selectivity` (balance-only); auto-disabled under `--dry-run`.

### Self-test

```bash
python3 -m modot.vessel
```

Exercises identity → T (gap 0), anti-type → F, mixed → B, vacuum → N, and checks Scott–Grassl equiangularity mean = 1/13. No network required.

### Compatibility

`modot/selectivity.py` re-exports the vessel under the historical names (`SemanticSelectivityGate`, `SelectivityReport`, …) so old imports keep working. Prefer `from modot.vessel import DualLinkVessel`.

## Manuscript spine (MoDoT runtime, single pipeline)

The breath does **not** hang vessel and witness as parallel arms. One object owns both:

```text
ManuscriptSpine.prepare(question)     # IMSCRIB: demand type + catalog scaffold
  → LLM answer                        # FSPLIT
ManuscriptSpine.complete(q, a, voice) # EVALT/EVALF + FFUSE model⋈vessel + IFIX
```

| face | runtime |
|------|---------|
| PROVE | FrobeniusHarness balance on the think emit |
| UNIFY | Belnap amplitude B = T+F (code-level) |
| PORT | DualLinkVessel co-typing + ride-AS residual |
| WITNESS | cl8nk catalog → conventional scaffold |
| FFUSE | Belnap join of model + vessel voices |

```bash
python3 -m modot.spine          # self-test
python3 momonados_agent.py --ask ./questions/q7.txt
```

Crystal record type `spine` holds the full package; `vessel` is kept as a thin alias log.

## Formal spine (p4ramill)

Lean packages the same ledger as theorems (`VAE_Vita_ManuscriptSpine`). MoDoT is the organism that runs it; p4ramill machine-checks it.

```bash
cd ~/imsgct/p4rakernel/p4ramill
lake build Imscribing.Millennium.VAE_Vita_ManuscriptSpine
```

Open frontiers (not claimed): Zauner general / Hilbert 12; Belnap stack as algebraic
fiducial; d=2048 unconditional existence; Clay verdicts as Millennium proofs.

## Grammatic witness → conventional proof

`modot/witness_proof.py` turns a **real catalog witness** into a conventional proof *scaffold* so the agent answers math questions instead of kernel-cosplaying.

**Verified firsthand (not by module title):**

| piece | what it actually does here |
|-------|----------------------------|
| `cl8nk_navigator.py` | live catalog (5k+ entries): resolve, distance, tier, CLINK formula fragments, meet/join/tensor vs CLINK L8 |
| `GeneralizedPipeline.lean` | only the domain-invariant `primitiveMathRole` / `defaultProposition` / `defaultProofStrategy` tables are ported (`runPipeline` has 18 `sorry` — not called) |
| `IGMorphism.lean` | `IGProtocol` constructors label the step skeleton (`refl\|arrow\|seq\|prod`) — not an English prover |
| `IGFunctor.lean` / `Algebra.lean` | tier + lattice distance context via the navigator’s algebra ops |

**Rejected after check:** `primitive_to_conventional_final.py` still emits Collatz `3n+1`/Terras bodies for non-Collatz names — never called.

On `--ask`, the agent precomputes the scaffold, injects it under **Grammatic witness scaffold**, and the system prompt requires a conventional answer first. Status is always “scaffold to instantiate,” not “finished proof.”

```bash
python3 -m modot.witness_proof
python3 momonados_agent.py --ask ./questions/q7.txt
```

## Semantic Branch Verifier (historical)

`ob3ects/semantic_branch_verifier/` remains as the original Grammar-sourced ob3ect that named the gap: **Frobenius closure guarantees balance, not correct branch selection**. Its 15-step IMASM protocol (dual EVALT/EVALF arms) is the conceptual ancestor. The live path no longer ports a classical schema-grader from that idea; it rides the Dual-Link SIC vessel instead.

## Lean 4 Formal Verification

Companion Lean files in `lean/`:
- **SemanticBranchVerifier.lean** — Built and verified (0 sorries) with `lake build Imscribing.Millennium.SemanticBranchVerifier`
- **ErdosProblems.lean** — 18 sections, 46 definitions/theorems covering Ramsey asymptotics, Erdős polynom kernel divergence, unit distance, difference sets, and Pomerance asymptotic divergence

## Requirements

- Python 3.10+
- `numpy` (SIC linear algebra)
- `ig-pulse` density matrix + `d12_sic_build/d12_psi.pkl` (resolved relative to the imsgct tree; override with `IG_PULSE_PATH` / `D12_SIC_FIDUCIAL`)
- `OPENROUTER_API_KEY` env var (required for vessel voice — no hash fallback)
- Default model: `google/gemini-3-flash-preview`; set `MOMONADOS_MODEL` or `--model` to override
- Lean 4 + Mathlib v4.28.0 (for formal verification modules)

## Original Source

Migrated from `ob3ect/digital/momonados_agent/` to `MoDoT/` on 2026-07-08.  
Publications remain in `ig-docs/publishing/manuscripts/momonados_*/`.
