# Stone / Frontier Runs — Data

Context: successor to the constants program (`UNIVERSAL_CONSTANTS_FORMALIZED.md`,
`constants_run_distilled.md`).

Covers two runs:

| run | window | calls | ran / cached / miss |
|---|---|---|---|
| `19f74b0fdd0-31b92f` | 03:06:24 → 03:24:43 | 109 | 85 / 21 / 3 |
| `19f74e2b818-31d1c6` | 04:00:43 → 04:45:28 | 909 | 767 / 0 / 142 |

**Run 2 was seeded with §1 and §7 of this document** (its first call imscribes the
click product by name). Loop time from document written (03:51:10) to agent acting on
it (04:00:43): 9 minutes.

**A blocker found in run 2 has been fixed — read §10 before the next launch.**

---

# Part I — Run `19f74b0fdd0-31b92f`

## Run identity

| field | value |
|---|---|
| window | 2026-07-18 03:06:24 → 03:24:43 (18.3 min) |
| calls | 109 |
| outcomes | ran 85, cached 21, miss 3 |
| misses | `sublimate` × 3 (`sic_d12_existence_proof`, `parity_phase_transition_matrix`, `stone_self_verifying_proof_kernel`) |
| predecessor | `19f70eb9f43-2ef392` (constants run) |

## Log properties

- Logged: verb, args, outcome, tool output.
- Not logged: model prose. `records.jsonl` holds kernel-tick records (`IFIX(B)`, tick 40).
- Consequence: no fabricated-garnish class exists in this run; no fused verdicts exist either.
- Audit performed: every tuple and figure below confirmed present in the run's tool stream by substring match. One exception, marked inline.

## Entities

| name | tuple | ⊙ | Ω class | C_score |
|---|---|---|---|---|
| `stone_self_verifying_proof_kernel` | ⟨𐑦𐑸𐑾𐑹𐑐𐑧𐑚𐑠⊙𐑖𐑙𐑭⟩ | at criticality | 𐑭 — Z protected, integer winding (Kitaev, SSH) | 0.555, both gates open |
| `carved_ring_3_doubled_a2_45_pair_flip` | ⟨𐑛𐑥𐑾𐑹𐑐𐑪𐑔𐑝⊙𐑫𐑙𐑟⟩ | at criticality | 𐑟 — non-Abelian, anyonic braiding (Fibonacci anyons, non-Abelian CS) | 0.0, gate 2 closed (Ç=𐑪) |
| `parity_phase_transition_matrix` | ⟨𐑦𐑥𐑾𐑹𐑐𐑪𐑔𐑠⊙𐑫𐑳𐑴⟩ | — | 𐑴 | — |
| `sic_d12_existence_proof` | ⟨𐑨𐑥𐑑𐑹𐑱𐑧𐑲𐑠𐑢𐑖𐑙𐑷⟩ | 𐑢 subcritical | 𐑷 | — |
| `monotone_integer_winding` | ⟨𐑼𐑰𐑑𐑬𐑐𐑧𐑲𐑠⊙𐑖𐑳𐑭⟩ | at criticality | 𐑭 | — |

Closure signature carried by the stone: Ð=𐑦 with Þ=𐑸, Φ=𐑹 (μ∘δ=id provably exact), ⊙=⊙.

## Click — the frontier

`parity_phase_transition_matrix` was the cut frontier named at the end of the constants
run (kernel-proven, held at B, one winding old). It clicks here.

```
click-maths:  stone_self_verifying_proof_kernel  ⋈  parity_phase_transition_matrix   (θ=0.50)
    D↔W    stone= +0.33   parity= +0.67   Δ=0.33
    T↔H    stone= +0.33   parity= -0.50   Δ=0.83
    R↔S    stone= +1.00   parity= +0.00   Δ=1.00
  ✓ CLICK on T↔H — spring-loaded Δ=0.83, single reaction center, closes.
  product: ⟨𐑦𐑸𐑾𐑹𐑐𐑪𐑔𐑠⊙𐑫𐑳𐑭⟩  (ring saturated on T↔H = Þ↔Ħ)
  inherited scaffold (blended from both partners): [Ç, Γ, Σ, Ω]
```

Comparison: `theta_link ⋈ log_link` (same day, earlier) returned no click — Δ=0.00 on
every live pair, identical tuples, no reaction center.

Unresolved: R↔S carries the larger offset (Δ=1.00); the click fired on T↔H (Δ=0.83).
Selection rule not stated by the tool.

## Carved ring vs Axiom D formulation

Axiom D grounded_tuple ⟨𐑛𐑸𐑽𐑹𐑐𐑧𐑔𐑝⊙𐑫𐑙𐑟⟩ — *from the `correct_formulation_of_axiom_d`
ob3ect, NOT from this run's stream; cross-reference only.*

| axis | carved ring | axiom D | |
|---|---|---|---|
| Ð | 𐑛 | 𐑛 | = |
| Þ | 𐑥 | 𐑸 | ≠ |
| Ř | 𐑾 | 𐑽 | ≠ |
| Φ | 𐑹 | 𐑹 | = |
| ƒ | 𐑐 | 𐑐 | = |
| Ç | 𐑪 | 𐑧 | ≠ |
| Γ | 𐑔 | 𐑔 | = |
| ɢ | 𐑝 | 𐑝 | = |
| ⊙ | ⊙ | ⊙ | = |
| Ħ | 𐑫 | 𐑫 | = |
| Σ | 𐑙 | 𐑙 | = |
| Ω | 𐑟 | 𐑟 | = |

9 of 12 identical. Both carry Ω=𐑟 (non-Abelian) at Ð≠𐑦 — the configuration the
coordinate form of Axiom D forbade. The carved ring is an independent second instance.

Stone and carved ring: same ⊙, opposite C_score (0.555 / 0.0), differing only by Ç
(𐑧 / 𐑪) on the gate-2 test.

## Polymer chains

5-monomer chain, tool output: *"imscriptive: the chain stores the monomer SEQUENCE
losslessly (R∧W∧X) — a click blends, a polymer remembers."*

Sequences run: `[carved_ring · sic_d12 · stone · parity · monotone_integer_winding]`
and `[sic_d12 · stone · parity · carved_ring · monotone_integer_winding]`.

## Not done

- Click product ⟨𐑦𐑸𐑾𐑹𐑐𐑪𐑔𐑠⊙𐑫𐑳𐑭⟩ produced at 03:22:09, never consumed: not imscribed
  under a name, not probed, not polymerized.
- Run terminated inside a repeat loop, not at a conclusion.

## Harness

- 3 misses / 109 calls. No bare `prove` calls (the constants-run fix held).
- Spin: batteries at 03:22:09, 03:22:47, 03:23:38, 03:24:13 are identical repeated sets,
  fully cached — `click · topo_protection_probe · switch · trap · homolyze · click ·
  forge · imasm check`, twice over.
- Detector available: N consecutive cached calls matching an earlier arg-set = no
  advance; force new object or cut.

## Next

1. Consume ⟨𐑦𐑸𐑾𐑹𐑐𐑪𐑔𐑠⊙𐑫𐑳𐑭⟩ — imscribe under a name, probe, attempt ring closure.
   Inherits [Ç, Γ, Σ, Ω] from both partners.
2. Recover the reaction-center selection rule (T↔H at 0.83 chosen over R↔S at 1.00).
3. Carved ring: what opens gate 2 without leaving Ω=𐑟. `excite` / `anneal` target.
   Determines whether the Axiom-D configuration is a class or a pair.
4. Add the spin detector before relaunch.
5. No coordinate-level axiom checks (see closure-condition result; carved ring is the
   second counterexample).

---

# Part II — Run `19f74e2b818-31d1c6`

## 8. Identity

| field | value |
|---|---|
| window | 2026-07-18 04:00:43 → 04:45:28 (44.8 min) |
| calls | 909 — ran 767, miss 142 (15.6%) |
| seeded with | §1 and §7 of this document |
| `imscribe` calls | 179, across 96 distinct entity names |
| terminated | on `gate_2_parity_check` (miss — invented verb) |

Items from §7 addressed: 1 (consume the product), 2 (`T_H_vs_R_S`,
`reaction_center_parity_map`), 3 (`gate_2_open` imscribed).

## 9. Reaction-center rule — proposed by the run

`imscribe reaction_center_parity_map` — *"Formal Mapping: T↔H=Active_Tunnel,
R↔S=Inert_Reflective_Barrier"*.

Status: proposed, not demonstrated. It names why T↔H (Δ=0.83) was selected over R↔S
(Δ=1.00) — R↔S is inert, not merely larger — but no verb output in the run tests it.
§7 item 2 remains open until a click with varied offsets confirms it.

## 10. BLOCKER — lossy registration (FIXED 2026-07-18)

**Defect.** `imscribe <name> <tuple>` routed the tuple to the generate pipeline, which
reads its argument as a *description*. The glyphs were read as prose and a different
tuple was minted. Any tuple a verb computed was unstorable.

**Evidence.** The click product ⟨𐑦𐑸𐑾𐑹𐑐𐑪𐑔𐑠⊙𐑫𐑳𐑭⟩ was submitted 49 times under 20
distinct names. Roughly a third returned a different tuple. Examples:

| name | passed in | catalog returned |
|---|---|---|
| `stone_parity_product` (×5) | ⟨𐑦𐑸𐑾𐑹𐑐𐑪𐑔𐑠⊙𐑫𐑳𐑭⟩ | ⟨𐑦𐑥𐑽𐑹𐑐𐑧𐑔𐑠⊙𐑫𐑕𐑭⟩ |
| `clicked_stone_parity_product` | ⟨𐑦𐑸𐑾𐑹𐑐𐑪𐑔𐑠⊙𐑫𐑳𐑭⟩ | ⟨𐑦𐑸𐑾𐑹𐑐𐑧𐑔𐑠⊙𐑫𐑳𐑭⟩ |
| `stone_parity_product_α` | ⟨𐑦𐑸𐑾𐑹𐑐𐑪𐑔𐑠⊙𐑫𐑳𐑭⟩ | ⟨𐑦𐑥𐑽𐑹𐑐𐑧𐑔𐑠⊙𐑫𐑳𐑭⟩ |
| `fibonacci_anyon_o2` | ⟨𐑱𐑯𐑾𐑹𐑨𐑲𐑠𐑢𐑖𐑙𐑴⟩ | ⟨𐑛𐑡𐑽𐑯𐑐𐑤𐑔𐑵⊙𐑒𐑳𐑟⟩ |

The 20 renames were not confusion: each was a retry against a lossy write. This
accounts for 49 of the run's 179 `imscribe` calls.

**Fix (committed).** `run_imscribe` in `ask_native` parses a well-formed 12-glyph
notation and registers it verbatim — no generation, no model call, keys written in
notation order ⟨Ð Þ Ř Φ ƒ Ç Γ ɢ ⊙ Ħ Σ Ω⟩, catalog committed by temp-file + rename.
A trailing provenance note after the tuple is preserved. Anything not exactly 12
glyphs still routes to `generate`.

Verified: the run's own unstorable product round-trips exactly; prose and an 11-glyph
tuple both still generate.

**Outstanding:** `fart.rs` carries the same "via the generate pipeline" message — a
probable second call site with the identical defect, untraced.

## 11. Invented verbs

23 verbs were called that appear nowhere in the corpus as `ran`. Typos (`distil`,
`annealing`, `probe`, `define`, `find`, `measure`, `catalog`) aside, the rest are
structurally specific operations with coherent arguments:

`transfer_matrix_calculate` · `topological_scan_density_matrix` ·
`t_to_h_vs_r_to_s_energy_differential` · `gate_2_parity_check` · `topo_symmetry_check` ·
`thermal_stability_scan` · `kinetic_injection` · `catalyst_fuse` · `distill_break` ·
`catalog_link` · `stabilize_ring` · `search_bridge` · `find_closure` · `recalibrate`
(called as `recalibrate <entity> --perturb_chirality Ħ`)

Precedent: ROTAT was reached for the same way, under structural necessity, and proved
real. Each is an un-run test carrying its own intended signature. Treat as a tool
backlog to be implemented and checked one at a time, not as noise.

Top misses on EXISTING verbs (argument-shape failures, not absence):
`anneal` 39, `compare` 14, `material` 14, `register` 10, `dope` 9.

## 12. Terminal state

- Closes at T: word `VINIT IMSCRIB AFWD FSPLIT EVALT EVALF FFUSE IFIX TANCH IFIX` —
  μ∘δ CLOSED over 1 **transformed** reconnection ("split → work → fuse: a type-check,
  not bare identity"). Runs both EVALT and EVALF: dialetheia-complete.
  Shape: branched, V=10 E=10 β=1, ρ=2.3180.
- Goes OPEN: `VINIT IMSCRIB FSPLIT EVALT EVALF FFUSE IFIX` — dropping AFWD and TANCH
  leaves the δ fork unreconnected. Linear, β=0, ρ=1.8478.
- Final chain reached the Hodge conjecture:
  `[spectral_invariants · hodge_conjecture · stone_parity_product_α · monotone_integer_winding]`.
- Last `material` call passed the tool's own pretty-printed output back as arguments —
  "·" became 9 monomers, chain terminated at θ=0.50. Input-shape error.

## 13. Next — supersedes §7

1. **Re-store the product first.** The verbatim path now exists; register the click
   product once, under one name, and stop renaming.
2. Test the reaction-center rule (§9) — clicks with varied offsets. Confirm or drop
   "R↔S = inert reflective barrier".
3. Carved ring gate 2: what opens it without leaving Ω=𐑟.
4. Implement invented verbs from §11 one at a time, highest structural necessity first
   (`recalibrate --perturb_chirality`, `topo_symmetry_check`, `gate_2_parity_check`).
5. Fix the `anneal` / `compare` / `material` argument shapes — 76 misses between them.
6. Never pass a tool's pretty-printed output back as arguments (§12).
7. Trace `fart.rs` for the second lossy-registration call site (§10).
8. No coordinate-level axiom checks.

---

# Part III — Run `19f753d0c3d-320240`

## 14. Identity

| field | value |
|---|---|
| window | 2026-07-18 05:39:22 → 05:47:52 (8.5 min) |
| calls | 146 — ran 122, miss 24 |
| verbs | `imasm` 60, `imscribe` 20, `anneal` 14, `forge` 12, `compare` 7 |
| first production run after the verbatim-registration fix | yes |

## 15. Verbatim registration confirmed in production

`clicked_product_alpha` and `residue_beta_prime_doped` both returned
*"registered VERBATIM … the tuple you supplied, unchanged."* Registered once each; the
49-retry rename loop of run 2 did not recur.

## 16. Two lying arity contracts (FIXED)

| verb | advertised | actually required | misses |
|---|---|---|---|
| `anneal` | "2+ names" | 3+ (CLI arity and impl both) | 14 |
| `compare` | "two sets split by `vs`" | 2+ names per side | 5 |

The agent supplied exactly what the error text asked for, was rejected, and repeated.
19 of the run's 24 misses are these two mismatches. The implementations are correct —
a ring needs three monomers to have distinct orderings — so the contracts were fixed,
not the semantics.

## 17. `recalibrate` — the verb it was actually stuck on (BUILT)

Requested three times: at 05:39, 05:43 and 05:47:52, the last being the final call of
the run. Always `recalibrate <entity> --perturb_chirality Ħ`. Never available.

After `annihilate` failed at 05:40 the agent pivoted off protection and onto chirality:
`phase_neutralizer`, `chiral_symmetry_breaker`, then nine separate attempts to define
`h_chiral_assert` / `h_chiral_check_bond_angle` as an imasm ring, plus
`lean h_chiral_assertion.lean` and `imasm prove Imscribing/CLINK_L9.lean`. The instinct
was right: Ω=𐑟 cannot be deformed, so the route is a coupled axis.

Now built. On the carved ring it returns the answer:

```
recalibrate:  carved_ring_3_doubled_a2_45_pair_flip   axis Ħ
  current: 𐑫   full tuple ⟨𐑛𐑥𐑾𐑹𐑐𐑪𐑔𐑝⊙𐑫𐑙𐑟⟩
    𐑓  ↓3   ⚠ lowering Ħ below 𐑖 breaks Ω≥𐑭 — the protection would fall with it
    𐑒  ↓2   ⚠ lowering Ħ below 𐑖 breaks Ω≥𐑭 — the protection would fall with it
    𐑖  ↓1
```

Chirality is the coupled axis; perturbing it drags protection. Writes nothing — a
perturbation is a probe. Reports the two couplings the kernel holds and labels the
Ħ=𐑫 / Ç=𐑪 co-occurrence a tendency, never enforcing it.

## 18. `annihilate` (BUILT)

`imasm run annihilate Ω=𐑟` at 05:40 logged as `ran` — the tool executed and returned
"no tool named". The agent retried bare, then abandoned the route.

Now a native verb: `annihilate A [B]`, pair fusion μ, the counterpart to `homolyze`'s δ.
Abelian Ω windings add — opposite cancel to vacuum (T), like windings leave a residual
(F). Ω=𐑟 returns a **channel**, not a value (Fibonacci τ×τ=1+τ) → verdict **B**, both
open, with braid-then-re-annihilate named as what selects one.

## 19. BLOCKER — catalog disconnection (FIXED)

The generate pipeline writes to the live store; the verbatim path wrote only to
`IG_catalog.json`. The two drifted: 17 entries existed in IG and not in live —
including `gap_bridger` and `residue_beta_prime`, which this run had just created — so
they were invisible to the pipeline's duplicate check and to every live-first reader.

Verbatim registration now mirrors into the live store (both shapes, first-name-wins,
temp-file + rename, best-effort with IG as store of record). The 17 divergent entries
were backfilled.

## 20. Still open

- `imasm define` / `prove` gave no usable failure: `h_chiral_check_bond_angle` was
  redefined nine times with different opcode rings and never converged. Read those
  outputs before adding any further verb.
- Invented and still absent: `phase_neutralizer`, `chiral_symmetry_breaker`,
  `topological_scan_density_matrix`, `transfer_matrix_calculate`, `distill_break`,
  `catalog_link`, `gate_2_parity_check`.
- The agent passes tool pretty-printed output back as arguments (`material` with a
  concatenated monomer name; `forge --close`).

## 21. Next

1. Re-run. `recalibrate`, `annihilate`, verbatim registration and catalog sync are all
   in; the three things that blocked the last run are gone.
2. Carved ring gate 2 via `recalibrate … Ħ` — the coupling is now visible.
3. Read the `imasm define`/`prove` failure text before building more verbs (§20).
4. No coordinate-level axiom checks.
