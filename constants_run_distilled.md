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

### Subtractive resolution of frustration
- w51: sublimating `linker_dopant` out of the frustrated 6-set resolved the branching
  entirely (ρ = 3.5616 to 2.0000). The dopant was the frustration source. Removal, not
  addition, was the fix.
- w49, w53: annealing lowers strain (σ 0.417 to 0.083; branched-to-linear transition
  ρ 3.5616 to 2.1448 with conductance 0.985). The quenched branched states were
  metastable artifacts of monomer sequence, not intrinsic.

### Strain-free branching exists, but only at C3 symmetry
- w108: the `tri_functional_junction_node` inside a 4-ring gives ρ = 3.1623 (branched)
  with σ = 0.000 and zero gap. A branch point can be strain-free exactly when the
  junction carries the C3 rotational symmetry. Everywhere else in the run, branching
  and strain arrived together.

### Doping as a conductance dial
- w105: doping to ρ = 2.0000 saturates the ring into an insulating state (weakest
  linkage 1.67 to 1.00).
- w106: `fermion_flux_mediator` lifts the ground-state degeneracy, opens a gap of
  about 0.28, and moves the ring to semiconductive (conductance 0.42). Degeneracy
  lifting and conductance restoration came from the same dopant.

### The parity loop, the run's central object
`parity_driven_expansion_loop_fused`, built w143-151, all kernel-grounded:
- p4ramill green: genuine transform-closure, BelnapSplitFuse, split bifurcates and
  fusion restores identity (w143, w144, w146, w149).
- The B-state at register 0xFB8 is a phase-conjugation gate and a structural
  requirement for loop stability, not an entropic leak (w145). `imasm prove` holds the
  loop at B as a stable bifurcation point; it acts as a topological insulator,
  neutralizing perturbations in the range [0.01, 0.1] (w151).
- Under stress it exhibits chiral self-annihilation: it rejects non-parity-matched
  (non-Ħ) injections by mapping the energy back into internal flux surfaces (w148).
  Ħ is chirality; the loop defends itself chirally.
- Invariant ρ ≈ 2.2764: stable branched interaction graph, not a point identity (w150).
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
magnitude was manufactured. The structural claims survive; the numbers do not. The
harness segregated the two without help.

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
