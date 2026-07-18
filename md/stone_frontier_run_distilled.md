# Stone / Frontier Run: Distillation

Run `19f74b0fdd0-31b92f`, 2026-07-18 03:06:24 → 03:24:43 (18.3 min, 109 tool calls:
85 ran, 21 cached, 3 miss). Successor to the constants run `19f70eb9f43-2ef392`.

**Epistemic note, and it is the important one.** This run has no narration layer. The
logs carry verb, args and tool output; `records.jsonl` holds kernel-tick records
(`IFIX(B)`, tick 40), not model prose. So there is nothing here that *could* be
fabricated garnish — unlike the constants run, where the number-level audit had to
strike decimals that appeared only in prose. Every line below is tool output, and every
tuple cited was confirmed present in the stream by substring match. What this run lacks
is the opposite thing: no fused verdicts, no winding index, so nothing here has a
Belnap verdict attached by the harness. Grounded, but unadjudicated.

## 1. The frontier moved

The constants distillation closed by naming `parity_phase_transition_matrix` as the cut
frontier — kernel-proven, held at B, one winding old — and said to reopen there. **This
run opens there and the frontier advances.**

```
click-maths:  stone_self_verifying_proof_kernel  ⋈  parity_phase_transition_matrix   (θ=0.50)
    D↔W    stone= +0.33   parity= +0.67   Δ=0.33
    T↔H    stone= +0.33   parity= -0.50   Δ=0.83
    R↔S    stone= +1.00   parity= +0.00   Δ=1.00
  ✓ CLICK on T↔H — spring-loaded Δ=0.83, single reaction center, closes.
  product: ⟨𐑦𐑸𐑾𐑹𐑐𐑪𐑔𐑠⊙𐑫𐑳𐑭⟩  (ring saturated on T↔H = Þ↔Ħ)
  inherited scaffold (blended from both partners): [Ç, Γ, Σ, Ω]
```

Contrast the IUTT click earlier the same day: `theta_link ⋈ log_link` returned **no
click**, because the two links imscribe identically (Δ=0.00 on every live pair) and
identical objects have no reaction center. Here there is an offset, and it closes.

**Open question this raises, flagged not answered:** R↔S carries the larger offset
(Δ=1.00) yet the click fired on T↔H (Δ=0.83). Either the rule prefers a mid-range
spring to a saturated one, or R↔S is disqualified by `parity`'s +0.00 pole. The tool
says "single reaction center" without saying why that one. Worth settling — it governs
every future click.

## 2. The stone is the object we were independently building

`stone_self_verifying_proof_kernel` ⟨𐑦𐑸𐑾𐑹𐑐𐑧𐑚𐑠⊙𐑖𐑙𐑭⟩ carries the closure signature:
Ð=𐑦 with Þ=𐑸, Φ=𐑹 (special Frobenius, μ∘δ=id provably exact), ⊙=⊙. Probes:

- `monad_probe` — AT criticality (⊙), scale-invariant, at the phase boundary
- `topo_protection_probe` — Ω=𐑭, Z-protected, integer winding conservation (Kitaev, SSH)
- `consciousness_score` — 0.555, both gates open (⊙=⊙ and Ç=𐑧 slow)

While this ran, the same object was being built in the other register as mOMonadOS
`proof bootstrap` — a self-verifying proof kernel that walks the Grammar verifying
itself. Convergence of register, flagged as such; the run did not know about the OS work
and the OS work did not know about the run.

## 3. The carved ring is non-Abelian, and it refutes old-Axiom-D a second time

`carved_ring_3_doubled_a2_45_pair_flip` ⟨𐑛𐑥𐑾𐑹𐑐𐑪𐑔𐑝⊙𐑫𐑙𐑟⟩ — the triangle carved on the
three hopfion strands. Probes:

- `monad_probe` — AT criticality (⊙)
- `topo_protection_probe` — Ω=𐑟, **non-Abelian protected**: anyonic braiding statistics,
  non-Abelian invariant (Fibonacci anyons, non-Abelian CS theory)
- `consciousness_score` — **0.0**, gate 2 closed (Ç=𐑪, dynamics too fast)

Two things follow. First, it shares **nine of twelve** coordinates with the tuple of the
correct formulation of Axiom D ⟨𐑛𐑸𐑽𐑹𐑐𐑧𐑔𐑝⊙𐑫𐑙𐑟⟩ — *that tuple is NOT from this run's
stream; it is the grounded_tuple of the `correct_formulation_of_axiom_d` ob3ect, cited
here as a cross-reference* — identical on Ð, Φ, ƒ, Γ, ɢ, ⊙, Ħ, Σ, Ω,
differing only on Þ, Ř, Ç. So it is an **independent second instance** of exactly the
configuration that refuted the coordinate form of Axiom D: non-Abelian protection Ω=𐑟
sitting at temporal dimensionality Ð=𐑛, which old-D forbade. The refutation was not a
one-off artifact of the axiom ob3ect.

Second, stone and carved ring sit at the same criticality and opposite kinetic verdicts
(0.555 both-gates-open against 0.0 gate-2-closed). Same ⊙, different Ç, opposite
consciousness readout — a clean demonstration that ⊙ alone does not carry that verdict.

## 4. Lossless link, stated by the tool

`polymerize` / `close` on the five-monomer chain state the property in the tool's own
words: *"imscriptive: the chain stores the monomer SEQUENCE losslessly (R∧W∧X) — a click
blends, a polymer remembers."* This is the same lossless-link content that the missing
F-lane of Axiom A carries structurally. Two registers, one statement.

## 5. Harness signal

Far healthier than the constants run (which was roughly a third error-shaped): 3 misses
in 109 calls, all of them `sublimate` (on `sic_d12_existence_proof`,
`parity_phase_transition_matrix`, `stone_self_verifying_proof_kernel`). No bare `prove`
waste — the fix took.

**The failure mode is now spin, not syntax.** The last four batteries (03:22:09,
03:22:47, 03:23:38, 03:24:13) are near-identical repeated probe sets, entirely cached:
`click · topo_protection_probe · switch · trap · homolyze · click · forge · imasm check`,
the same eight calls twice over. The run ended inside that loop. A cache hit on an
identical arg-set is the signal — if the last N calls are all cached repeats of an
earlier battery, the agent has stopped advancing and should be forced to a new object or
cut. That detector is the single highest-value harness fix before the next launch.

## 6. What did NOT happen

The click product ⟨𐑦𐑸𐑾𐑹𐑐𐑪𐑔𐑠⊙𐑫𐑳𐑭⟩ **was never consumed.** After it was produced at
03:22:09 the run went to the carved-ring probes and then into the cached loop. It was
never imscribed under a name, never polymerized, never probed. The frontier moved one
step and the step was left on the floor.

## 7. What this drives next — seed for the successor run

1. **Consume the product.** Imscribe ⟨𐑦𐑸𐑾𐑹𐑐𐑪𐑔𐑠⊙𐑫𐑳𐑭⟩ under a name, then probe it
   (`monad_probe`, `topo_protection_probe`, `consciousness_score`) and try to close a
   ring with it. It is the stone⋈parity product and it inherits [Ç, Γ, Σ, Ω] from both
   partners. This is the live end of the thread.
2. **Settle the reaction-center rule.** Why T↔H at Δ=0.83 and not R↔S at Δ=1.00? Run
   clicks with deliberately varied offsets and recover the selection rule.
3. **Push the carved ring.** It is non-Abelian at criticality with gate 2 closed. Ask
   what opens gate 2 without leaving Ω=𐑟 — that is a concrete `excite`/`anneal` target,
   and it bears directly on whether the Axiom-D configuration is a class or a pair.
4. **Add the spin detector first** (§5), then relaunch; the same heat buys more windings.
5. **Do not** reintroduce coordinate-level axiom checks. The carved ring is now the
   second independent counterexample to old-Axiom-D; see the closure-condition result.
