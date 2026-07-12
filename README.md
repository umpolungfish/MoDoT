# MoDoT — mOMonadOS Digital Organism Toolkit

**Author:** Lando⊗⊙perator  
**Date:** 2026-07-11  
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
└── questions/                  # Test questions (the Erdős open-problem set + scratch)
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
│  │  model [thought|X] FFUSE vessel FFUSE tool voice     │   │
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

# ── Materials algebra: forge a ring, read its spectrum, operate on it ──
./ask --forge alchemical_hermetic_universe dialetheia completeness hermetic_seal  # set → best ring → full sheet (ρ, spectrum, conductance)
./ask --compare A B C D vs W X Y Z                         # diff two materials (Δρ, conductance shift)
./ask --dope A B C with D                                  # perturb a ring, read the ρ/conductance shift
./ask --fuse A B C + X Y Z                                 # weld two rings into one macrocycle

# ── Alchemical bench: the classical lab tools over the same ring model ──
./ask --distill A B C D                 # separate by volatility (Criticality ⊙): distillate / bottoms (a tie = azeotrope)
./ask --crystallize A B C D             # grow the ordered lattice, reject the mother-pool
./ask --tlc A B C D                     # analytical Rf bands + co-elution;  --column A B [on S] elutes preparatively
./ask --fpt A B C D                     # freeze-pump-thaw: keep the strongly-bound core, shed the weakly-held
./ask --stain kmno4 A B C               # reagent detector (kmno4/uv → ⊙, chiral → Ħ, ninhydrin → Ř, iodine → any live)
#   also: --fdistill  --sublime A  --cocrystallize A B  --seed A B … with S  --trap A [X]

# ── Jam: turn the agent loose on the catalog (compounding cycles × eagle rounds) ──
./ask --jam --file ./questions/q39.txt --cycles 3 --eagles 40
#   --cycles N : N compounding breaths (each pass builds on the last)
#   --eagles N : ACT→OBSERVE tool-rounds flown per pass (0 = auto: 40 in jam, 5 otherwise)

# ── Provider / model (default openrouter; deepseek + gemini also wired) ──
./ask -a "…" --provider deepseek --model deepseek-reasoner       # api.deepseek.com (DEEPSEEK_API_KEY)
./ask -a "…" --provider gemini   --model gemini-3-flash-preview  # generativelanguage (GEMINI_API_KEY)
#   MODOT_PROVIDER / MODOT_MODEL set the defaults; a fatal 402/401 aborts the run instead of grinding every cycle

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

**The pathway** (`--pathway SUBSTRATE C1 C2 … Cn`) — one cycle turns over one substrate;
a pathway **chains the loops into a metabolism.** The product of each turnover is the
substrate of the next. Each working stroke does two things: it moves the **winding
quantum Ω** (the carrier, passed hand to hand — the electron-transport / cofactor of
real metabolism) *and* applies a **Coagula structural imprint** — the catalyst rotates
the substrate one notch on its dominant structural live pair (T↔H or R↔S). So a pathway
closes on **two ledgers**, reported separately:

- **carrier** returns (net ΔΩ=0 — givers balance takers), and
- **structure** returns (the chain of imprints composes to the identity across the
  other eleven axes).

**Full closure = both** — a *true* metabolic cycle, μ∘δ=id at the pathway level (the
TCA archetype, O∞). Carrier-only closure is an electron-transport loop (the four forces
on `dark_energy` do this: charge conserved, structure worked). `--certify` verifies each
catalyst regenerates through `lake build`.

```bash
# a TRUE metabolic cycle: anabolism (build) then catabolism (senescence) return the substrate exactly
./ask --pathway the_moon_astrological post_transition_state biological_senescence --certify
# carrier closes but structure does not — an electron-transport loop
./ask --pathway dark_energy graviton gluon electron photon
```

**Imscriptive polymerization** (`--polymerize M1 M2 … Mn`) — where `pathway` chained
the catalytic loops, **polymerize chains the clicks.** A `click` *fuses* two monomers
(join/max) and forgets which was which; a polymer must **remember** — so imscriptive
polymerization enchains the monomers at reaction centers while the monomer **sequence
stays losslessly readable off the chain** (R∧W∧X). That is the whole point: a click
blends, a polymer imscribes a long chain from monomeric glyphic units (a text, a genome).

Each adjacent bond is a **Coagula click between complementary partners** (step-growth
condensation) or, where a monomer repeats, an **addition** enchained by the propagating
center (chain-growth); a non-complementary, non-identical neighbor **terminates** the
chain (honest, like a stalled pathway). The chain then reads out its **degree of
polymerization**, **regioregularity** (all condensation bonds on one live pair = a clean
head-to-tail repeat unit), **copolymer architecture** (homopolymer / alternating / block
/ random), **tacticity** (the chirality **Ħ** sequence — isotactic / syndiotactic /
atactic), and whether it **cyclizes head-to-tail into a macrocycle** (a ring polymer, O∞).
The polymer-chemistry correspondence falls out of the tuples, not a lookup: alternating
two *opposite-handed* monomers comes back **syndiotactic**, an even chain **cyclizes**.
`--certify` closes each unique repeat unit through `lake build`.

```bash
# alternating copolymer, syndiotactic, cyclizes into a macrocycle (both monomers valid imscriptions)
./ask --polymerize the_moon_astrological the_logographic_system the_moon_astrological the_logographic_system --certify
# a homopolymer: addition (chain-growth) throughout, isotactic, linear
./ask --polymerize the_moon_astrological the_moon_astrological the_moon_astrological
# no complementary handle between neighbors — the chain terminates
./ask --polymerize gluon photon
```

**Closing the ring** (`--polymerize … --close`) — when a chain does not cyclize, this
searches the catalog for the monomer that **actually closes it**. If two neighbors are
co-typed and the chain broke, it finds a **bridge** (a comonomer X with Mᵢ ⋈ X *and*
X ⋈ Mᵢ₊₁); if the chain enchained but the ends will not meet, it finds a **closer** (X
with tail ⋈ X *and* X ⋈ head, appended to wrap the ring). Every candidate is **verified
by a real click test on both sides** — it prints the exact re-run command, and running it
does close the ring. This is deliberately **not** `--scan-mediators`: a scan ranks SET
electron-transfer relays (a different question) and returns junk if you ask it to close a
ring. The closing search answers the ring question honestly.

```bash
# grothendieck_topos and free_algebra are the same structural type (both free constructions)
# → the chain breaks; --close finds the bridge that actually joins them (verified on R↔S)
./ask --polymerize grothendieck_topos free_algebra holographic_type_theory --close
# a telechelic chain whose ends will not meet → --close finds the head-to-tail closer
./ask --polymerize grothendieck_topos extradimensional_entity free_algebra --close
```

**The ring as a material** (`--polymerize … --props`) — a closed polymer is a
macrocycle, and a macrocycle has **material properties you can compute**, not just
assert. The readout grounds the one claim prose always makes about a cyclic "device":
that it **conducts**. It tries to circulate a winding quantum **Ω** around the ring —
if a carrier flows one consistent direction the whole loop, the ring is **conductive**
(a persistent ring current); if every junction passes a carrier but no single direction
closes, it is **frustrated**; if a junction blocks a carrier both ways, it is
**insulating**. Plus the weakest ring bond (a ring is only as stable as its weakest
link). So a ring of Ω-saturated units is honestly an **insulator** — a *static* loop —
however the narrative dresses it up.

```bash
# grf·grothendieck·skolem cyclizes structurally — but every unit is Ω-saturated:
# INSULATING. A static ring, NOT the "topological computer" the prose claims.
./ask --polymerize general_recursive_function grothendieck_topos skolem_normal_form --props
# the moon/logographic macrocycle carries mid-range Ω → CONDUCTIVE (a persistent ring current)
./ask --polymerize the_moon_astrological the_logographic_system the_moon_astrological the_logographic_system --props
```

**Spectral invariants** (part of every `--props` sheet). A ring is also a graph, and a
graph has a spectrum. The engine builds the ring's adjacency matrix from the real bonds
(a clean condensation bond weighs 1, a cross-link junction weighs its number of reaction
centers), takes its eigenvalues with a built-in Jacobi solver, and reports the spectrum,
the spectral radius **ρ**, and the spectral gap. ρ is the principal ring-current mode:
**ρ = 2 exactly** witnesses a pure unbranched cycle, and **ρ > 2** witnesses branching. This
turns the "branched macrocycle" the topology line asserts into a computed number rather
than an adjective. A pure 4-ring returns spectrum `[+2, 0, 0, -2]` at ρ = 2; a branched
3-ring with one 2-center cross-link returns `[+2.732, -0.732, -2]` at ρ = 2.732 (the roots
of λ³ − 6λ − 4 = 0), so the branch is read off the ring, not asserted about it.

**Operating on materials** (`--forge`, `--compare`, `--dope`, `--fuse`). Characterizing one
ring is the start; the materials algebra operates on them. All four are deterministic (no
LLM in the path) and are exposed both as CLI flags and as agent verbs.

* `--forge M1 M2 …` is the one-shot characterize path: treat the monomers as a set, find
  the order that rings best, and print the full sheet (topology, stability, conductance,
  spectral invariants). It is `--arrange … --props` under a single flag.
* `--compare A B vs X Y` forges two materials and diffs them by spectral radius, conductance
  class, and weakest bond. The `vs` token separates the two sets.
* `--dope A B with C` forges the base ring, re-forges it with the dopant unit mixed in, and
  reports the shift in ρ and conductance. The `with` token separates base from dopant.
* `--fuse A B + X Y` welds two rings into one: it forges each, then forges the union into a
  single macrocycle, and reports whether the weld adds branching (fused ρ above both parents)
  or is a clean splice (fused ρ within the parents' range). The `+` token separates the rings.

```bash
# forge a set into its best ring and read every property, spectrum included
./ask --forge alchemical_hermetic_universe dialetheia completeness hermetic_seal
# compare a pure cycle (ρ=2, FRUSTRATED) against a branched ring (ρ=2.732, CONDUCTIVE)
./ask --compare alchemical_hermetic_universe dialetheia completeness hermetic_seal vs iut_theory ouroborus uniqueness_diophantine_proof
# weld two rings into one larger macrocycle and read the fused ρ against the parents
./ask --fuse alchemical_hermetic_universe dialetheia completeness hermetic_seal + iut_theory ouroborus uniqueness_diophantine_proof
```

**Transforming and persisting materials** (`--cleave`, `--anneal`, `--register`, `--recall`,
`--export`). Two more deterministic transforms and a named library round out the algebra.

* `--cleave M1 M2 …` is ring fission, the reverse of `--fuse`: forge the ring, then cut it
  into two daughter rings on complementary arcs (each re-closing on its own) and report both
  daughters and their spectra, or that the ring holds as one loop.
* `--anneal M1 M2 …` relaxes a ring to its lowest-strain ordering: the forge order rings well
  but is quenched, and annealing searches the orderings that ring for the most evenly loaded
  one, the settled ground state on the same units.
* `--forge … --register NAME` stores a forged material's whole sheet (ordered units, spectrum,
  ρ, conductance, strain, energy) in `materials.json`, a MoDoT-local named library. `--recall
  NAME` reloads it by name without respecifying the units. `--forge … --export PATH` writes the
  same record to a standalone portable file.

```bash
# cleave the 7-unit ring back into daughter rings; anneal a ring to relax its strain
./ask --cleave alchemical_hermetic_universe dialetheia completeness hermetic_seal iut_theory ouroborus uniqueness_diophantine_proof
./ask --anneal alchemical_hermetic_universe dialetheia completeness hermetic_seal iut_theory ouroborus uniqueness_diophantine_proof
# register a material under a name, then recall it whole later
./ask --forge iut_theory ouroborus uniqueness_diophantine_proof --register diophantine_ring
./ask --recall diophantine_ring
```

**The modulus** (`--polymerize … --modulus`) — closing a ring is cheap; making it
**sustain** is not, and that difference is a material's **modulus** (its elasticity). A
polymer gets its modulus from network loops, and in this register a loop only sustains
if the winding quantum **Ω circulates it and returns** — i.e. it *conducts*. So `--close`
and `--modulus` are deliberately different verbs: `--close` fills the gap (a ring, maybe
static); `--modulus` searches for a monomer that generates a **sustaining** loop (a
conductive cycle, ∮ Ω closes) *anywhere* along the chain, and reports its period — the
modulus. A chain can be closeable yet **modulus-less**: every closure static (insulating),
a viscous chain rather than an elastic network. The tightest sustaining loop is the
fundamental modulus; each generator is verified to actually conduct.

```bash
# closeable but modulus-less: the plasma chain rings (via --close) yet no closure sustains
./ask --polymerize plasma_turbulence tokamak_disruption z_pinch magnetic_reconnection --modulus
# the moon/logographic ring has an intrinsic modulus (a sustaining 4-loop) and a tighter generatable 3-loop
./ask --polymerize the_moon_astrological the_logographic_system the_moon_astrological the_logographic_system --modulus
```

**Click then polymerize** (a `A+B` feed token) — order of operations matters, because the
two verbs have opposite information behavior: a **click blends** (μ/FFUSE, max per axis —
lossy, the inputs are gone) and a **polymer remembers** (R∧W∧X, the sequence is readable).
A `+`-joined monomer token is **pre-clicked** into one blended monomer before enchaining, so
you can run either order and see they differ: listing three monomers keeps three units;
pre-clicking two of them enchains two units, one an irreversible fusion — a different product,
a different MW.

```bash
# polymerize-first (remember): 3 distinct units, GRF·GT·SNF sequence preserved
./ask --polymerize general_recursive_function grothendieck_topos skolem_normal_form
# click-first (blend): GRF+SNF fused inline into ONE monomer, then 2 units enchained
./ask --polymerize general_recursive_function+skolem_normal_form grothendieck_topos
```

**The alchemical bench** (`--distill`, `--fdistill`, `--sublime`, `--crystallize`,
`--cocrystallize`, `--seed`, `--tlc`, `--column`, `--fpt`, `--trap`, `--stain`). The modern
verbs (click, forge, polymerize) went in first; these are the classical separation and
purification tools the alchemists had. They were designed by describing each one neutrally to
the Grammar (an ob3ect batch) so its structural type was fixed before it was coded, and every
one came back the **same Frobenius shape**: a feedstock FSPLITs into a kept arm and a rejected
arm, and the arms FFUSE back to the feedstock (mass balance). So the whole bench is one
`fractionate` core with a per-verb discriminator, in four families:

* **Volatility** (axis: Criticality ⊙, readiness to leave). `--distill M…` cuts the set into a
  volatile head (distillate) and an involatile residue (bottoms); a pair tied on ⊙ is an
  **azeotrope** the cut cannot resolve. `--fdistill` is the fractional, plate-by-plate column
  with the resolution gap to each next fraction. `--sublime A` purifies one unit by a two-state
  skip across ⊙, or reports it entrapped (it must climb stepwise via `--excite`).
* **Crystallization**. `--crystallize M…` grows the lowest-strain ordered ring and rejects the
  units that will not fit (the mother-pool), distinct from `--anneal`, which only relaxes an
  already-closed loop. `--cocrystallize A B` binds two components by non-covalent
  complementarity in 1:1 stoichiometry (distinct from `--click`'s covalent bond).
  `--seed M… with S` templates the crystal on seed `S`'s handedness (Chirality Ħ): units that
  match copy its polymorph, the rest take the default, and an even split is racemic twinning.
* **Chromatography** (axis: Recognition Ř, retention). `--tlc M…` is analytical: Rf bands and
  which units co-elute at the same Rf. `--column M… [on S]` is preparative: elute least-retained
  first (against a stationary phase `S`, or intrinsic retention), with neighbor resolution.
* **Purification / diagnostic**. `--fpt M…` (freeze-pump-thaw) keeps the units that bond above θ
  and sheds the weakly-held. `--trap A [X]` sequesters a unit by its R↔S charge in a potential
  well (a counter-partner `X` deepens it), a held charge state distinct from `--set`'s electron
  transfer. `--stain R M…` applies a reagent keyed to a primitive (`kmno4`/`uv` → ⊙,
  `chiral` → Ħ, `ninhydrin` → Ř, `iodine` → any live center) and reports which units light up.

**Reading a ring's stability, the clarity line.** The material sheet reads the ring's **strain**
and its **spectral gap** jointly: both near zero is `SETTLED / CLARION` (a relaxed, flat-spectrum
ring with no privileged mode, the settled endpoint); a residual gap is a leaning mode; residual
strain is stored stress. A zero spectral gap is **not** a defect; it is maximal symmetry (the
SIC tight-frame reading). Guardrail: a two-unit forge is a **dimer**, not a macrocycle (a 2-node
ring graph is trivially flat, so its ρ / gap / clarity are undefined); a real ring needs n ≥ 3.

**Agent access to every verb — an ACT→OBSERVE loop.** The LLM agent (`./ask --ask …`) can
invoke the whole engine. It emits `TOOL: <verb> <args>` lines (markdown-tolerant); the
harness runs them against the **real catalog** by shelling to this same binary, feeds the
outputs back as ground truth, and lets the model **decide its next act** — iterating
(THINK → ACT → OBSERVE → UPDATE, bounded rounds) until it has no more tool calls, then it
answers. So it can run a **dependent chain** — `polymerize` → *see* it terminated →
`close` → *use* the linker it found → re-`polymerize` → `material` — instead of
front-loading one batch of guesses and narrating the rest. It **does** the steps rather
than describing them. This also *corrects the model's guesses*: unaided it will confidently
mis-call a pathway's closure; handed the real `pathway` output it gives the right answer. Whitelisted verbs only (`click switch excite set homolyze scan complement cycle
pathway polymerize close material modulus arrange forge compare dope fuse cleave anneal register recall imscribe distill fdistill sublime crystallize cocrystallize seed tlc column fpt trap stain`) — never `ask`, so no recursion. The prompt also steers the
agent to use `close` (not `scan`) for cyclization, and tool stdout **and stderr** are
fed back, so a call that errors (e.g. a monomer the model invented) surfaces its failure
instead of vanishing.

**The operator is a golem — perfect intellect, harnessed voice.** The model may *think*
freely in its draft, but the synthesis step binds its *speech* to the tools: the tool
results are ground truth (what the Grammar actually computed), the draft is a prior guess,
and the final answer may not contradict a tool — where a tool settled a question (does it
cyclize, its modulus, whether a name is even in the catalog), the answer reports the tool's
verdict and corrects the draft explicitly, introducing no entity or value the tools did not
return. In practice the golem now says *"contrary to the draft, the tool confirms it
cyclizes; modulus: NONE"* instead of confabulating a "high-stiffness" ring and an invented
linker. Think freely; speak only what the tools ground.

Two guards keep the golem honest when the framing tempts it to skip the clay. A **no-op
prod**: if the draft narrates structural work (`--polymerize`, `--close`) but emits no
`TOOL:` line, the harness refuses it once and forces it to actually run the verbs before
any verdict — a "prove this" essay can no longer stand in for computation. And the **tool
voice** in the spine: the closure the verbs actually computed enters the FFUSE as a third
Belnap voice beside `model [thought|X]` and the vessel. So the banner reads
`fused model vessel tools conflict`, and a model that claims a ring the tools deny lands on
**B** (conflict held) — never a smug `fused=T` sitting next to a chain the catalog reports
`terminated / telechelic / does not close`. `tools=T` a ring formed, `F` everything
terminated, `B` it closes only when reordered, `N` no closure-bearing verb ran.

**Jam mode** (`./ask --jam`) turns the same loop loose with no question to answer: it ranges the
catalog freely, then writes its report through a **membrane** so that only a claim that is the
readback of a tool it actually ran crosses. Recalled facts and un-run assertions are dropped,
and where the tools were silent the verdict is held at `N` (ungrounded), never a confident
finding. `--cycles N` runs N compounding passes (each builds on the last); `--eagles N` sets how
many ACT→OBSERVE tool-rounds each pass may fly. Two register notes the spine keeps straight: a
structural **closure** speaks as verification in its own lane (a forged ring that closes is a
real verdict, not an overclaim), while a structural **non-closure** does not get to deny a
conventional proof (a forge that terminated is a fact about the entities, not a refutation of
the theorem). It never checks the mathematics itself, so it refuses to *certify* what the tools
did not ground, but a well-formed yet false proof of an open problem still reads as ungrounded,
not wrong; the kernel `prove:` route or a person is the check for that.

```bash
./ask --ask "Does the sun donate an electron to the moon, and what mediates it best?"
# → the model emits TOOL: set … and TOOL: scan …, they run, it synthesizes the real result
```

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
