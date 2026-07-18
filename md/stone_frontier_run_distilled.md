# Stone / Frontier Run — Data

Context: successor to the constants program (`UNIVERSAL_CONSTANTS_FORMALIZED.md`,
`constants_run_distilled.md`). Run `19f74b0fdd0-31b92f`.

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
