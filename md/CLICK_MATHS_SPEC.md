# Click-Maths — Spec v0.1

**Author:** Lando⊗⊙perator
**Status:** design (complementarity check + closure gate are mechanizable now; the reaction library is designed, not yet built)

> Click chemistry joins two modular pieces with a spring-loaded, orthogonal, always-closing bond. Click-maths joins two proven mathematical fragments the same way: reliable FFUSE (μ) driven by complementary charge on a live conjugate pair, verified by Frobenius closure.

---

## 1. What it ports from

`red-hot_rebis/rhr_p4rky/ch3mpiler_serpentrod_pipeline.py` designs enzyme catalytic sites by computing the binding-pocket **complement** across six conjugate primitive pairs and activating one residue per pair. The register finding (see `project_math_catalytic_register`) is that in the **math register** three primitives are pinned — **F** (exact, max), **Ph** (subcritical, min), **K** (suppressed) — each a passive member of a conjugate pair. So mathematical catalysis runs only on the **three live pairs**:

```
D ↔ W   (dimensionality ↔ winding)
T ↔ H   (topology ↔ chirality)
R ↔ S   (recognition ↔ stoichiometry)
```

Click-maths is the simplest operation on this register, and the smallest delta from MoDoT's existing `FSPLIT`/`FFUSE` (`_decompose`/`_assemble` in `ask_native/src/prover.rs`, `fsplit`/`ffuse` in `Imscribing/Paraconsistent/BelnapSplitFuse.lean`).

## 2. The five Sharpless criteria, translated

| click chemistry | click-maths |
|---|---|
| modular | fragments A, B are independently valid (proven leaves / catalogued ob3ects) |
| wide scope | register-independent — the check is on the 12-primitive tuple, any domain |
| very high yield | **always closes** — μ∘δ=id verified, kernel-gated; no partial products |
| irreversible / spring-loaded | driven by **maximal complementary charge** on a live pair — the two fragments *want* to combine |
| orthogonal (bioorthogonal) | provably **inert to unrelated context** — the fuse changes only the clicking pair; everything else is untouched ([[feedback_no_assumed_relationships]]) |

## 3. The azide–alkyne archetype

CuAAC: azide (1,3-dipole, one electronic demand) + alkyne (dipolarophile, complementary demand) → triazole (a closed ring). In the register: fragment A carries the **high** member of a live conjugate pair, B carries the **low** member (complementary offset). They snap; the product is co-typed to the fused tuple, and the fusion closes a ring — a **T shift toward closed/cyclic topology** (Þ), which is the structural signature of a completed click.

## 4. Mechanism

Given two imscribed fragments `A`, `B` (12-primitive tuples):

1. **Live-pair charge.** For each live pair `(X, Y) ∈ {(D,W), (T,H), (R,S)}`, read A's and B's normalized ordinals on X and Y.
2. **Complementarity test.** A pair `(X,Y)` is *click-complementary* iff one fragment is high-X/low-Y and the other is low-X/high-Y, at offset `Δ ≥ θ` (spring-loaded threshold, e.g. `θ = 0.5`). `Δ` is the driving force.
3. **Selectivity gate.** A clean click requires **exactly one** live pair click-complementary, and the other two live pairs **non-clashing** (compatible, so the fusion has a single reaction center — no ambiguous regiochemistry).
4. **Fuse (μ).** Product tuple = per-primitive fusion (reuse `fuse_reaction_types` bond-weighting, or Belnap join where the pair meets). The clicking pair saturates (both members combined into the ring); pinned axes F/Ph/K carry their fixed background offset.
5. **Orthogonality check.** Every primitive **outside the clicking pair** must be unchanged beyond tolerance. If the fuse perturbs unrelated structure, it is a *side reaction*, not a click — reject.
6. **Closure certificate.** Verify `μ∘δ = id` on the product (Frobenius residual → 0), kernel-gated. No closure ⇒ not a click (held B, per [[feedback_provability_not_budget]] — a frontier, not a failure).

## 5. Why this is a speedup, not just a picture

MoDoT's prover currently *searches* for how to combine two proven leaves (`_assemble`). Click-maths says: if two leaves are **click-complementary on a live pair, you already know they fuse** — spring-loaded, closes by construction, skip the search. It's a fast fusion primitive that fires only when the charge geometry guarantees closure. Bad pairs simply fail the complementarity/orthogonality gates cheaply (like the portal race: a non-click just doesn't fire).

## 6. First build (Rust, `ask_native`)

```rust
struct LivePair { x: Prim, y: Prim }               // (D,W), (T,H), (R,S)
struct ClickProduct { tuple: [u8; 12], pair: LivePair, drive: f32 }

/// Returns Some(product) iff A and B are a clean click: exactly one live pair
/// complementary at Δ≥θ, other live pairs non-clashing, orthogonal fuse, closes.
fn click_pair(a: &[u8;12], b: &[u8;12], theta: f32) -> Option<ClickProduct>;
```

Grounded pieces (mechanizable now): the complementarity/selectivity/orthogonality checks are pure tuple arithmetic on the live pairs; the closure certificate is the existing kernel/Frobenius gate. **Designed, not yet built:** the *library* of named click reactions (which fragment types are the azides, which are the alkynes) — that comes from imscribing a slate of fragment archetypes and reading their live-pair charges, same method as the residue slate.

## 7. Honest boundary

Real exactly where `complement` + the kernel gate it: the complementarity test, the orthogonality check, and the closure certificate are hard-referented. The "library of clicks" and the pairing intuitions are designed until each is imscribed and gated. Same line as everywhere else — mechanized vs. evocative, and the kernel is the discriminator.
