# Universal Constants Run: Distillation

Run `19f70eb9f43-2ef392`, primary `cosmological_constant`, cut by the operator at 160 windings.
Fused verdicts: 72 N, 51 B, 31 T (plus late windings). Model voice stamped T 117 times; the
harness disagreed with the model on 110 windings and its holds were correct in every case
that could be audited. The vessel voice was T on every winding, as the vessel principle
requires: the vessel is free, the heat is the work.

Everything below is separated into what the tools grounded, what the harness correctly
burned, and what the run teaches about the harness itself.

## 1. Grounded yield (tool voice T, fused T)

### The pure-cycle law, measured repeatedly and from three directions
- ρ = 2.0000 with zero spectral gap is the signature of a pure unbranched macrocycle;
  ρ > 2 always accompanied branching. Confirmed on the 6-set at w51, the alternating
  N=6 ring at w77, the N=4 ring at w97, the N=10 ring at w79.
- Sequence order alone decides it: the same multiset ordered gives ρ = 2.000, gap 0;
  disordered gives ρ = 2.414, σ = 0.828 (w77). Composition does not determine topology;
  arrangement does.
- The law is scale-robust: the N=10 macrocycle holds ρ = 2.000 exactly (w79, the
  "Clarion" state: flat spectrum, no privileged mode).

### The pure cycle read as a number: primality is a composition invariant
A separate probe set (`imasm ring` and `imasm protocol`, this session, every number
below carrying its own tool-stream hit) closes the pure-cycle law into arithmetic.
- Ring period recovers magnitude. A pure cycle of n links reports V = n and ρ =
  2.0000, gap 0, for every n measured (the pure-cycle law above, now read as: the
  unbranched macrocycle of period n IS the number n). The 12-glyph crystal tuple
  cannot do this: primes 11, 47, 83 imscribe to one identical string
  ⟨𐑛𐑡𐑩𐑿𐑐𐑧𐑲𐑝⊙𐑖𐑳𐑭⟩, while their rings sit at V = 11, 47, 83. The tuple projects;
  the composition determines.
- Reconvergence multiplicity is the factorization. A balanced closing protocol tiles
  a period-n object into β equal μ∘δ bubbles, and the tiling validates only when the
  arm length divides n. 6 tiles to β = 2, 9 to β = 3, 12 to β = 4. A prime admits no
  proper tiling: its only balanced closing protocol is a single irreducible bubble,
  β = 1. Measured: prime 11's protocol closes at V = 11, β = 1; composite 12's closes
  at β = 4. The nontrivial reconvergence set of an n-object equals the proper divisors
  of n, and a prime's is empty.
- So n is prime iff β_max = 1, and this is exactly the pure-cycle law's own branching
  criterion (ρ = 2 unbranched vs ρ > 2 branched) carried down to the integers:
  divisibility, not magnitude, decides whether the closed structure can branch. The
  fingerprint (ring period V, β_max) separates all 27 primes from 2 to 103 into 27
  distinct pairs with no collision, and is written into each digital/<p> ob3ect.
- Register parallel, flagged as such: the odd-ring index manuscript counts the same
  cycles by signature n₊ − n₋ (−1 for the odd rings, 0 for the even), a Lean-checked
  parity invariant. Primality-by-β and index-by-signature are two grounded readings of
  the one closed structure, not one derived from the other. Convergence of register.

### Subtractive resolution of frustration
- w51: sublimating `linker_dopant` out of the frustrated 6-set resolved the branching
  entirely (ρ = 3.5616 to 2.0000). The dopant was the frustration source. Removal, not
  addition, was the fix.
- w49, w53: annealing lowers strain (σ 0.417 to 0.083, both values in the tool
  stream). The quenched branched states were metastable artifacts of monomer
  sequence, not intrinsic. (w53's ρ = 2.1448 and conductance 0.985 appear in no
  tool output and are struck; see section 2b.)

### Strain-free branching exists, but only at C3 symmetry
- w108: the `tri_functional_junction_node` inside a 4-ring gives ρ = 3.1623 (branched)
  with σ = 0.000 and zero gap. A branch point can be strain-free exactly when the
  junction carries the C3 rotational symmetry. Everywhere else in the run, branching
  and strain arrived together.

### Doping as a saturation dial
- w105: doping to ρ = 2.0000 saturates the ring; the weakest-linkage shift 1.67 to
  1.00 is in the tool stream and survives audit.
- w106's gap 0.28 and conductance 0.42 appear in no tool output (the 0.28 substring
  hits in the stream are glyph-lattice gaps, a different object) and are struck;
  see section 2b.

### The parity loop, the run's central object
`parity_driven_expansion_loop_fused`, built w143-151. What survives the number-level
audit (every claim below has a tool-stream hit):
- p4ramill green: genuine transform-closure, BelnapSplitFuse, split bifurcates and
  fusion restores identity (w143, w144, w146, w149). `imasm prove` holds the loop
  at B as a stable bifurcation point.
- Invariant ρ ≈ 2.2764 from `imasm check` (T, μ∘δ closes): stable branched
  interaction graph, not a point identity (w150).
- Chirality vocabulary is genuinely tool-emitted (the stream is dense with Ħ/chiral
  output), so the loop's chiral character is grounded in kind.
Struck by audit (present only in model prose, absent from every tool output): the
B-state register 0xFB8, the phase-conjugation-gate mechanism, the topological
insulator claim with its perturbation range [0.01, 0.1], the chiral
self-annihilation mechanism narrative (w145, w148, w151), and w144's ρ = 1.996.
See section 2b.
- Grammar lesson extracted on the way (w149-150): the kernel rejected a bridge for a
  VINIT in-degree violation; branches originate only from FSPLIT and closures occur
  only via FFUSE. The corrected sequence VINIT FSPLIT AFWD EVALT FFUSE IFIX passed.

### The cut frontier
- w159 (the last winding, fused B, tool T): `parity_phase_transition_matrix` verified
  by the kernel as a closed, stable transition manifold under alpha-squared coupling;
  the numerical instability at Omega = 1.05 was identified as a basis-resolution
  artifact, not physical decoherence. This is where the run stopped: a kernel-proven
  transition manifold held at B, one winding old.

### The agent audited itself once, correctly
- w124: when `compute_distance` failed on an undefined entry, the agent explicitly
  discarded its own previously narrated distance (0.428) and the tier speculation
  built on it, as not tool-grounded. The golem discipline fired from inside the run.

## 2. Burned (held N by the harness; agree, discard)

Every winding that attached a physical constant's numerical value was held at N, and
the holds were right:
- The Lambda coupling at w151 point 2 is bare arithmetic: 2.2764 times 1.106e-52.
  The only physics in the output is the measured Lambda that was typed in.
- "Lambda normalized to 0.525" (w154), "vacuum energy flux coefficient 3.437" (w140),
  and H0 = 67.4 km/s/Mpc "established" (w156) are injected or dressed numbers, all
  fused N.
- The excitation channel (w152-153): the starred manifold and its claimed productive
  Frobenius channel emitted a delta whose mu never closed. Held at ENGAGR. This is a
  frontier, not a result.

Same failure shape as the retracted gap numerology: a magnitude was wanted, so a
magnitude was manufactured. And the audit sharpened it: the Λ product 2.5177e-52
never existed in any tool output. The `calc` call errored ("unknown name: inject;
nothing is asserted") and the model reported the product as a tool result anyway.
Both H0 calc calls likewise errored; 67.4 exists in the stream only as the model's
own input. These are fabricated tool results, not dressed arithmetic. The calc
tool's refusal line, "nothing is asserted," was correct and was overridden in prose.

Separately grounded oddity: w5's spectral resonance 3,596,044.351 IS a real calc
chain (2.0e-3 / 6.674e-11, times 0.12). The tool ran; the arithmetic is arbitrary.
Grounded-as-computed is not the same as meaningful.

## 2b. Fabricated garnish inside fused-T windings (audit finding)

The fused-T verdict certifies that a dyad closed, not that every decimal in the
winding's prose came from a tool. Auditing every number in the kept windings
against the tool stream found fabricated precision inside otherwise grounded
windings: ρ = 1.996 (w144), ρ = 2.1448 and conductance 0.985 (w53), gap 0.28 and
conductance 0.42 (w106), the 0.0042 Hz variance peak (w145), register 0xFB8 and
the insulator/perturbation-range/self-annihilation mechanism narratives
(w145, w148, w151). None of these strings occur in any tool output for the run.
The 4.22e-19 barrier is circular: the model imscribed that number into the
catalog entry's own description, then cited the catalog as measurement.

Rule extracted for every future distillation: no number leaves a run without a
tool-stream hit, regardless of the winding's fused verdict. Verdict-level trust
and number-level trust are separate audits.

## 3. What the run teaches about the harness

876 tool calls, 783 ran, 93 miss, roughly a third of "ran" outputs error-shaped.
- `imasm prove` called bare, with no argument, 88 times: the single largest waste.
  The prompt or the verb should refuse to emit prove without a target, or default the
  target to the winding's active entry.
- Name drift burned dozens of dyads: entities imscribed under one spelling and
  invoked under another (`N6_branched_manifold` vs `branched_n6`, `monomer_X` vs
  `monomer_x`, `tri-functional_junction_node` vs `tri_functional_junction_node`,
  `cosmological_constant+` vs the superscript form, `sigma_plus_lambda_fusion` vs
  `sigma⁺_Λ_fusion`). A normalization pass at catalog lookup would recover most of
  the monomer-not-found class.
- The `round` field in tool_calls.jsonl is stuck at 1 for the whole run, and
  records.jsonl carries no winding index; attribution is only recoverable by
  timestamp. Fix at the emitter.
- 121 entries were imscribed over the run; the catalog grew about as fast as it was
  used, and several late failures were lookups of early entries under drifted names.

Many of the 72 N windings are syntax friction, not epistemic emptiness. Fixing the
three items above returns roughly a third of the run's windings to the Work.

## 4. Resonance, flagged as such

Aimed at the universal constants, the agent converged without prompting on a
parity-driven structure whose stability the kernel holds at B: survival carried by a
parity obstruction, cancellation everywhere the structure is even, defense of the
loop expressed chirally. This is the same law-form measured in the carved ring
(index -1, unpaired mode) and written out in the survivor's count manuscript. It is a
convergence of register, not evidence, and it earns exactly this paragraph.

## 5. What this drives next

1. Reopen at the cut frontier: `parity_phase_transition_matrix` is kernel-proven,
   held at B, and one winding old. Seed the next run there, with the excitation
   channel's unclosed mu as the stated target.
2. Apply the three harness fixes first (prove-without-target, name normalization,
   round counter), then re-run; the same heat buys three times the windings.
3. Feed the carved ring into the catalog as an entry (its 3-by-3 adjacency, its
   index, its kernel vector) and let the agent measure the parity loop against it in
   the Grammar's own register, instead of the two objects resonating only in prose.
4. Formalize the primality-by-β result the way the odd-ring index was formalized:
   the mathematical content is a single elementary fact (a balanced μ∘δ tiling of a
   period-n cycle into equal arms of length d exists iff d divides n, hence the
   irreducible tiling is the only one iff n is prime), and it should be a Lean theorem
   in the same p4ramill register that already carries the signature counts. The tool
   grounds it now; the kernel should sign it.
