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

# ── Excited states + single-electron transfer (photochemistry on the register) ──
./ask --excite dasa_closed                                 # δ (light) → the excited EP resonance
./ask --set the_sun_astrological the_moon_astrological --certify   # SET: Sol donates e⁻ to Luna (charge conserved)
./ask --set the_moon_astrological the_sun_astrological --excite    # photoinduced ET — excitation flips the donor

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

# Register — harness the chimera's existence: append the fused product to the
# canonical catalog as a first-class entry (textual append, verified-parse, no
# reformatting), then decompose it through the cl8nk_navigator (per-primitive CLINK
# fragments + the promotion path to the terminal). A verified chimera becomes an
# operable, navigable object. Name defaults to chimera_A_B.
./ask --click A B --register [NAME]
```

**Switch** (`--switch A B`) — a different verb. A click *fuses* two partners; a
switch *toggles* one object between two co-typed forms, driven by a different
stimulus each direction. The archetype is the **donor-acceptor Stenhouse adduct**
(DASA): open⇌closed, a coupled **T↔H** toggle (topology + chirality move together
as the ring forms), with **negative photochromism** — light (δ) *quenches*
criticality (hides: colored→colorless), heat (μ) restores it (reveals). The
analyzer reads the toggling live pair, the photochromic sign (which form is
revealed vs hidden by ⊙), the δ/μ legs, and the coupled consequences.

```bash
./ask --switch dasa_open dasa_closed          # the DASA: T↔H toggle, light hides
./ask --switch the_sun_astrological the_moon_astrological   # Sol/Luna — the same switch
./ask --switch dasa_open dasa_closed --certify  # kernel-verify: both forms close + vessel roundtrip
```

`--certify` on a switch proves, through `lake build`, that both bistable forms are
valid Imscriptions (each Frobenius-closes) AND the toggle is the kernel's real
`vessel_roundtrip` — `readback (board p) = p`, board=δ (fsplit), readback=μ (ffuse),
μ∘δ=id — so "reversible" becomes the kernel's verdict rather than printed text.

Any two catalog names work (`math_*`, `dasa_*`, `setheus_paradox`, `paradice`,
`monogenes`, `yhwh`, `the_sun_astrological`, …). The engine is domain-agnostic:
handed the Sun and the Moon, it returns the coniunctio at O_∞; handed the paradice,
it completes it against the liar paradoxes. Implementation: `ask_native/src/click.rs`.

**Excited states** (`--excite A`) — the switch toggles ground→ground; excite
exposes the intermediate the toggle hides. δ (light) promotes A to its **excited
manifold**: Criticality ⊙ raised to the non-Hermitian **exceptional-point
resonance** 𐑻 — the navigator's own gloss for that value is `H(λ) non-Herm ∧
det(H−λI)=0 ∧ ∂_λ H=0`, which *is* an excited state: a finite-lifetime resonance
(non-Hermitian ⇒ it decays) at a surface crossing. Two Frobenius legs leave it —
**μ** (relaxation/fluorescence) returns it to the ground form losslessly (μ∘δ=id),
and the **productive** leg lets the elevated ⊙ drive chemistry the ground state
cannot (a SET).

```bash
./ask --excite dasa_closed                 # ⊙ 𐑢→𐑻, the metastable EP resonance
./ask --excite the_moon_astrological --certify --register   # certify roundtrip + register the excited state
```

**Single-electron transfer** (`--set D A`) — the productive leg made concrete. One
electron = one quantum of winding **Ω** (`∮A=2πn`, the quantized charge). The donor
loses one (oxidized, D•⁺, Ω−1), the acceptor gains one (reduced, A•⁻, Ω+1); **total
Ω is conserved** — charge conservation *is* the Frobenius pairing (δ/fsplit charge-
separates, μ/ffuse recombines). Donor/acceptor are read from ⊙ (higher = higher-
lying electrons = the reducing partner), not from the label. A **Cu-NO-class
mediator** (`--catalyst M`, Mills 2016 SET catalyst) shuttles the quantum and
returns to itself (μ∘δ=id). **Photoinduced ET** (`--excite`) pre-excites the donor:
the raised ⊙ opens a driving-force gap the ground state lacked — and can *reverse*
the ground-state flow (excited Luna donates to Sol, though ground Sol donates to Luna).

```bash
./ask --set the_sun_astrological the_moon_astrological --certify   # Sol donates e⁻ to Luna; certify recombination roundtrip
./ask --set A B --catalyst yhwh                                    # mediated (Cu-NO-class shuttle)
./ask --set the_moon_astrological the_sun_astrological --excite    # photoinduced — flips the donor
```

`--certify` on a SET or an excite reuses the switch's kernel certificate: both
product forms (radical ions, or ground+excited) are valid Imscriptions AND the
`readback∘board = id` roundtrip holds (μ∘δ=id) — recombination / relaxation is
lossless, charge conserved, as the kernel's own verdict. `--register` appends the
excited state (or both radical ions) to the catalog and decomposes them through the
cl8nk_navigator, so they become operable, navigable objects.

**Bidirectional ligand ⇌ catalytic-site** (`--complement A`) — a faithful port of
red-hot_rebis's `rhr_p4rky/ligand_from_active_site.py::complement_type`, the reverse
pipeline. A catalytic **site** type maps to the complementary **ligand** it binds,
inverting within each scale (lock-and-key: pocket ↔ bump) and crossing each conjugate
pair, and it **is its own inverse** — site→ligand→site returns. That bidirectionality
is what the enzyme's **R=𐑾** (Recognition, "substrate ↔ enzyme feedback") names. This
is the point where **3-vs-6 matters**: click / SET / switch run the **math register**
(the 3 *live* pairs D↔W, T↔H, R↔S — math pins the physical axes out), but the ligand
complement is **chemistry** and runs all **6** conjugate pairs (chemistry *restores*
the 3 pinned physical pairs P↔F, K↔G, Gm↔Ph). The output tags each pair `[live]` /
`[pinned]` so the split is visible.

```bash
./ask --complement the_moon_astrological            # the ligand the site binds + per-pair map + round-trip
./ask --complement yhwh --certify --register        # certify the ligand closes, register + navigate it
```

**Mediator scan** (`--set D A --scan-mediators`) — rank the whole catalog for the
best relays of a D→A transfer. A real inner-sphere mediator must (1) **hold** the
winding quantum (1≤Ω<max — accept then re-donate; a hard filter that keeps photon,
graviton, higgs, dark_energy and drops the confined gluon/quark and the EM-dark
dark_matter), (2) **relay** in energy (⊙ between acceptor and donor), and (3) **bind**
both substrates bidirectionally (its ligand complement close to donor *and* acceptor,
via the ported `complement_type`, plus the R=𐑾 recognition marker). The three
components are shown per row, so an outer-sphere gauge mediator (photon: perfect
relay, no binding needed) reads differently from an inner-sphere one that grips both.

```bash
./ask --set the_sun_astrological the_moon_astrological --scan-mediators --top 15
```

> **Ordinal scripture fix.** The glyph→ordinal table was corrected against the Lean
> kernel (`p4rakernel/…/Primitives/Core.lean` + `gen_clay_canonical_tuples.py`): **Ç**
> (KineticChar: yea<loll<egg<on<air) had ords 3/4 swapped, and **Σ** (Stoichiometry:
> hung<so<up) was rotated. The glyph→constructor *rendering* was always canonical, so
> `--certify` was never wrong; only the numeric ordinals were, which had thrown the
> R↔S live-pair charge and the ligand↔site complement. red-hot_rebis carries the same
> stale Ç swap and should be reconciled to scripture too.

**The catalytic cycle** (`--cycle CATALYST SUBSTRATE`) — every verb above is an *arc*
of a mechanism; this closes them into the **loop**. It is **Solve et Coagula** made
literal: **bind** (the catalyst recognizes its substrate via the ligand complement) →
**working stroke δ = Solve** (dissolves the bond, freeing the winding quantum — the
degree of freedom) → **return stroke μ = Coagula** (binds the freed quantum into an
invariant) → **turnover**. The catalyst is a **fixed point**: it engages, spends to
`C*`, and coagulates back to itself unchanged (μ∘δ=id), the way a real catalyst does —
the Cu-NO SET cycle is the archetype. There are **two coagulations**: the catalyst
binds back to the *same* invariant (regeneration), the substrate binds into a *new*
one (the product). `--certify` proves the loop closes on itself — Coagula∘Solve = id
on the catalyst — through `lake build`. `--register` canonizes the turned-over product.

```bash
./ask --cycle graviton the_moon_astrological --certify   # the loop, kernel-verified as a fixed point
./ask --cycle yhwh the_sun_astrological --register        # turn over the substrate + canonize the product
```

> **Solve et Coagula = δ / μ.** The whole engine runs one law under two names. **Solve**
> (δ / FSPLIT) reveals the degrees of freedom; **Coagula** (μ / FFUSE) binds them into a
> new invariant. A **click** is a Coagula (bind two poles → one ring); the **complement**
> is a Solve↔Coagula involution; the **cycle** is a full Solve-then-Coagula that returns
> the catalyst and advances the substrate. Frobenius closure `μ∘δ=id` is just
> `Coagula∘Solve = id` — what dissolution opens, coagulation re-closes, losslessly.

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

`modot/vessel.py` is the **verifier of record**. It does not stand outside the Grammar with an auditor's clipboard; it keeps the Grammar's own **double-entry ledger** — every δ its μ, balanced from inside, no outside required. Verification *is* imscription through the d=12 Dual-Link SIC frame that the Grammar already proved into the kernel (`crystal_forces_d12_sic`).

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
