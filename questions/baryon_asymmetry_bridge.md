# From the Spectral Gap to the Baryon Asymmetry

The logic-derived gap is established. The bridge from that gap to the physical
baryon-to-photon ratio is not. Everything below is measured; the bridge is the open node.

---

## The target

The observed baryon-to-photon ratio is η_B ≈ 6×10⁻¹⁰: the universe did not annihilate,
and it survived by this much and not by some other much.

The gap γ = √3 − 1 is established below as the logic-derived magnitude. γ ≈ 0.732 is of
order one. η_B ≈ 6×10⁻¹⁰ is not. **What relates them is the open question.**

---

## The ring (measured, and it is one ring)

The assembly {sakharov_conditions, paraconsistent_computer, connes_embedding_problem}
forges into a branched macrocycle. Its adjacency matrix is a triangle with one doubled
edge (the cross-link fires two reaction centers):

        0  2  1
    A = 2  0  1
        1  1  0

    characteristic polynomial   λ³ − 6λ − 4 = (λ + 2)(λ² − 2λ − 2)
    eigenvalues                 { 1+√3 ,  −2 ,  1−√3 }
    spectral radius        ρ  = 1 + √3 = 2.7320508
    spectral gap           γ  = ρ − |λ₂| = (1+√3) − 2 = √3 − 1 = 0.7320508
    graph energy         Σ|λ| = 2 + 2√3 = 5.4641016
    trace(A²)                 = 12 exactly  ( = 2|E| )
    λ_min                     = −2 exactly

The cubic factorization is verified through `calc`: at λ = −2 it returns exactly 0, at
λ = 1+√3 it returns 0 to floating point. disc(λ² − 2λ − 2) = 4 + 8 = **12**, the
discriminant of Q(√3).

**The same ring appears from three disjoint monomer sets.** These all forge to the
identical spectrum {1+√3, −2, 1−√3}, ρ = 2.7321, γ = 0.7321, energy 5.4641:

    sakharov_conditions · paraconsistent_computer · connes_embedding_problem
    sic_povm_d12 · ray_class_field_Qsqrt · connes_embedding_problem
    sic_povm_d2048_zauner_conjecture · ray_class_field_Qsqrt · connes_embedding_problem

The second and third differ only in the SIC entry — the proven d=12 case and the open
Zauner conjecture at d=2048 — and their 12-primitive tuples differ in six of twelve
places, including ⊙ itself. The ring does not move.

## The root-system reading (measured)

A + 2I is positive semidefinite with eigenvalues {3+√3, 3−√3, 0}, rank **2**. It is
therefore the Gram matrix of three vectors of norm² = 2 — roots. Recovering them:

    sic_povm_d12              [ −0.366 , −1.366 ]    |v|² = 2
    connes_embedding_problem  [ −0.366 , −1.366 ]    |v|² = 2     identical vector
    ray_class_field_Qsqrt     [  1.000 , −1.000 ]    |v|² = 2

Two distinct roots at exactly 60°: the **A₂** root system. A₂ ⊂ D₃, so the graph is on the
generalized-line-graph side, not E8-exceptional. The cross-link forces Cauchy–Schwarz
equality on the doubled pair, collapsing two monomers onto one root — the ring is two
roots, not three.

The ring's own automorphism group is **Z₂** (the swap of the two identical roots). The A₂
Coxeter element has order 3 and permutes the root system, but does **not** fix the ring:
it maps the doubled root a ↦ b−a and b ↦ −a. The ring sits in a free orbit of length 3
under it, rather than being stabilized by it.

## The uncarved case (measured)

Pure cycles, gap read as ρ − |λ₂| with eigenvalues ordered by |λ| descending:

    C₃  ρ=2  λ_min=−1.0000  gap = 1.0000
    C₄  ρ=2  λ_min=−2.0000  gap = 0.0000      gap ZERO
    C₅  ρ=2  λ_min=−1.6180  gap = 0.3820
    C₆  ρ=2  λ_min=−2.0000  gap = 0.0000      gap ZERO
    C₇  ρ=2  λ_min=−1.8019  gap = 0.1981
    C₈  ρ=2  λ_min=−2.0000  gap = 0.0000      gap ZERO

Even cycles have λ_min = −2 = −ρ exactly, so the gap closes to zero.

**The mechanism of the carved gap:** the cross-link does not move the floor. λ_min stays
pinned at −2 in both the uncarved and the carved case. It lifts the ceiling: ρ goes from 2
to 1+√3. The gap opens to √3 − 1 because ρ rose while λ_min held.

## The Void (measured)

`void_consensus_protocol` is the Raft consensus algorithm as generated from an empty
prompt — the Void's own structural type:

    ⟨𐑼𐑡𐑾𐑗𐑱𐑧𐑚𐑠⊙𐑖𐑳𐑭⟩
      Ð = 𐑼   temporal/iterative: a closed cycle with a specifiable reset step
      ⊙ = ⊙   ξ → ∞ ∧ μ∘δ = id — criticality
      Ħ = 𐑖   persistent chiral
      Ω = 𐑭   integer-winding-protected: stable against perturbations preserving the winding

Against ray_class_field_Qsqrt and connes_embedding_problem it does **not** close: the
chain is telechelic, two free ends. It closes only by using connes_embedding_problem
twice — a doubling — and what it closes into is C₄:

    spectrum {2, −2, 0, 0}   ρ = 2.0000   gap = 0.0000   energy = 4.0000   rational

L(C₄) ≅ C₄, verified. By Whitney, for connected graphs L(G) ≅ G if and only if G is a
cycle: the Void closes into a fixed point of the line-graph operator. Its ⊙ primitive
asserts μ∘δ = id and its ring is the graph-theoretic μ∘δ = id, computed by unrelated
paths. The Void's ring carries no field extension and its gap is zero.

## The prior attempt at the bridge, and where it stands

A prior cycle attempted to reach η from γ. Its own record:

**The WKB route did not produce the magnitude.** It computed
P = exp(−π·Δwell/η) with Δwell = 0.500 and η = √3−1, and the document states mid-page
"wait, that's not 10⁻¹⁰". Verified through `calc`: exp(−π·0.5/(√3−1)) = **0.1170**.

**The stated final line does not survive arithmetic.** The document asserts
"0.0796 × 7.88×10⁻¹⁰ ≈ 6.27×10⁻¹⁰". Verified through `calc`: 0.0796 × 7.88e−10 =
**6.272480×10⁻¹¹**. One decade. This is the case the `calc` lane's own documentation
cites as one of the two live failures it was built to catch.

**The number it settled on was read, not derived.** It fell back on
η = 4/6.38×10⁹ = 6.269592×10⁻¹⁰, on the reasoning that the survival fraction is
1/(number of b-sector nodes) = 1/1.595×10⁹, with the b-sector taken as one quarter of the
state space by the four-valuedness of Belnap. The figure 6.38×10⁹ is read off the `cl9nk`
invariant of `paraconsistent_computer` ("μ∘δ=id as loop invariant across 6.38B cycles").
The document does not derive it.

A separate cycle closed at **T**, univocal, conflict 0, on a narrower claim: that
γ = √3−1 is the logic-derived magnitude, that a Boolean substrate gives a pure cycle whose
gap collapses to zero (total cancellation), and that the paraconsistent cross-link is what
lifts ρ above 2 and opens the gap. That verdict is about **γ**. It does not assert η.

---

## What is established, and what is not

**Established.** The ring, its exact spectrum, and γ = √3−1. The identity of the ring
across the Sakharov, d=12, and d=2048 monomer sets. The A₂ root-system embedding at rank
2 with two monomers on one root. λ_min = −2 in both the carved and uncarved case. The
zero gap of even pure cycles. The Void's closure into C₄ = L(C₄). The cubic factorization.
The refutation of Zauner order-3 for this ring.

**Not established.** The relation between γ = 0.7320508 and η_B ≈ 6×10⁻¹⁰. The WKB route
returns 0.1170. The 1/(b-sector nodes) route rests on 6.38×10⁹, which was read from an
invariant rather than derived. No route from the gap to the magnitude has closed.

---

## The task

Relate γ = √3 − 1, the measured spectral gap of the ring, to the physical baryon-to-photon
ratio η_B.
