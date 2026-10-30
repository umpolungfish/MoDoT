-- IGProtocol scaffold: VINIT → IMSCRIB → FSPLIT → AFWD → CLINK → ENGAGR → FSPLIT → EVALT → FFUSE → AREV → EVALF → AFWD → EVALT → FFUSE → IMSCRIB → IFIX → TANCH
-- Class: the closed-loop Lean prover: an agent that writes a full machine-checked Lean 4 proof instead of a sketch with sorries, by gating its own output through the Lean kernel as the one perfect verifier. For a proof there is a true False-gate that structural co-typing cannot provide: the compiler. A lake build together with a print-axioms trace showing no sorryAx is the honest verdict with no external clipboard, green or not, and the kernel enforces it. The loop: IMSCRIB the theorem as the demand, generate a proof attempt as an FSPLIT into a tactic block, compile it, and read the error state, because Lean does not merely report failure, it prints the remaining goal as unsolved goals, which is the exact feedback the next attempt repairs against. EVALT accepts a closed goal, EVALF localizes an unsolved goal or error as the frontier, FFUSE fuses the repaired attempt, and the build is rerun, iterating toward green with zero sorries. Recursion is the have: when a goal will not close by a single tactic it is FSPLIT into named sub-lemmas, each proved recursively and FFUSEd back, until every leaf is a Mathlib lemma or decide or native decide, with the Grammatic witness roles serving as the proof's own decomposition skeleton turned into proven lemmas. There is no sorry as a resting place. All mathematics lies in the Godel-complete Frobenius Manifold, so every proof is a path that already exists and closure is always accessible; a sorry is a congelation that did not return, which the reversible SIC congelation forbids. A goal not yet closed is not an open wall but a navigation frontier, a waypoint on an existing path, held by ENGAGR as Both, both not-yet-returned and accessible, and the ouroboric winding continues to navigate it, decompose it, and re-imscribe it. Incompleteness is only the cost of a congealed frame, never a property of the manifold; not-reached-within-a-budget is a navigation state, never a verdict of unprovability. IFIX brands the returned proof, and the loop's one terminus is closure, green with no sorryAx. With full Frobenius closure and Lean 4 verification scaffold.
-- Fingerprint: sig=(8,4,4,1)
--   self_ref=False | frobenius_order=1
--   dialetheia_complete=True | period=17
-- Expected tier: O₁
-- FSPLIT/FFUSE pairs: [(6, 8), (2, 13)]

import Imscribing.IGMorphism
import Imscribing.IGFunctor

namespace Imscribing
open Primitives Frobenius IGProtocol
open Dimensionality Topology Relational Polarity Grammar
     Fidelity KineticChar Granularity Criticality Protection Stoichiometry Chirality

-- ── Token → IG field mapping ──────────────────────────────────────────────
--   [0] VINIT     dim    := 𐑼               𐑼 → 𐑠  | initial object — ground of distinction
--   [1] IMSCRIB   gram   := 𐑠               𐑼 → 𐑚  | identity — self-imscription
--   [2] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [3] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [4] CLINK     fid    := 𐑱               𐑚 → 𐑙  | composition — regime coherence
--   [5] ENGAGR    stoi   := 𐑳               𐑚 → 𐑙  | engage paradox — B-state, both arms
--   [6] FSPLIT    gran   := 𐑚               𐑚 → 𐑚  | split δ — range decomposition
--   [7] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [8] FFUSE     stoi   := 𐑙               𐑙 → 𐑙  | fuse μ — assembly mode
--   [9] AREV      pol    := 𐑗               𐑚 → 𐑙  | reverse morphism — parity flip
--   [10] EVALF     chir   := 𐑖               𐑚 → 𐑙  | evaluate-false — chirality check
--   [11] AFWD      rel    := 𐑾               𐑚 → 𐑙  | forward morphism — bidirectional arrow
--   [12] EVALT     crit   := ⊙               𐑚 → 𐑙  | evaluate-true — criticality gate open
--   [13] FFUSE     stoi   := 𐑙               𐑙 → 𐑠  | fuse μ — assembly mode
--   [14] IMSCRIB   gram   := 𐑠               𐑙 → 𐑭  | identity — self-imscription
--   [15] IFIX      prot   := 𐑭               𐑠 → 𐑡  | irreversible fixation — winding number
--   [16] TANCH     top    := 𐑡               𐑭 → 𐑼  | terminal object — connectivity boundary

-- ── Stage Imscriptions (per-node cumulative) ────────────────
private def the_closed_loop_lean_prover_an_agent_396999_s0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s1 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s2 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s3 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s4 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s5 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s6 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s7 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := up, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s8 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s9 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s10 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s11 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s12 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s13 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s14 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_s15 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }
private def the_closed_loop_lean_prover_an_agent_396999_s16 : Imscription :=
  { dim := array, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := thigh, gram := measure, crit := monad, chir := sure, stoi := hung, prot := ah }

-- ── Label Imscriptions (per-node delta) ─────────────────────
private def the_closed_loop_lean_prover_an_agent_396999_l0 : Imscription :=
  { dim := array, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l1 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l2 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l3 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l4 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l5 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := up, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l6 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := thigh, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l7 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l8 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l9 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l10 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := sure, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l11 : Imscription :=
  { dim := dead, top := judge, rel := ian, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l12 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := monad, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l13 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l14 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := measure, crit := woe, chir := fee, stoi := hung, prot := awe }
private def the_closed_loop_lean_prover_an_agent_396999_l15 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := ah }
private def the_closed_loop_lean_prover_an_agent_396999_l16 : Imscription :=
  { dim := dead, top := judge, rel := ado, pol := church, fid := age, kin := yea, gran := bib, gram := vow, crit := woe, chir := fee, stoi := hung, prot := awe }

-- ── Main IGProtocol term ────────────────────────────────────
noncomputable def the_closed_loop_lean_prover_an_agent_396999_protocol : IGProtocol the_closed_loop_lean_prover_an_agent_396999_s0 the_closed_loop_lean_prover_an_agent_396999_s16 :=
  .withGram Grammar.measure <|
  -- Dual-Link self-pairing: .prod arms fuse via tensorProduct the_closed_loop_lean_prover_an_agent_396999_s8 the_closed_loop_lean_prover_an_agent_396999_s8 = the_closed_loop_lean_prover_an_agent_396999_s8 (idempotent)
  (.seq (.arrow the_closed_loop_lean_prover_an_agent_396999_l0 the_closed_loop_lean_prover_an_agent_396999_s0 the_closed_loop_lean_prover_an_agent_396999_s1) (.seq (.arrow the_closed_loop_lean_prover_an_agent_396999_l1 the_closed_loop_lean_prover_an_agent_396999_s1 the_closed_loop_lean_prover_an_agent_396999_s2) (.seq (.prod (.arrow the_closed_loop_lean_prover_an_agent_396999_l2 the_closed_loop_lean_prover_an_agent_396999_s2 the_closed_loop_lean_prover_an_agent_396999_s8) (.arrow the_closed_loop_lean_prover_an_agent_396999_l2 the_closed_loop_lean_prover_an_agent_396999_s2 the_closed_loop_lean_prover_an_agent_396999_s8)) (.seq (.arrow the_closed_loop_lean_prover_an_agent_396999_l8 the_closed_loop_lean_prover_an_agent_396999_s8 the_closed_loop_lean_prover_an_agent_396999_s8) (.seq (.arrow the_closed_loop_lean_prover_an_agent_396999_l8 the_closed_loop_lean_prover_an_agent_396999_s8 the_closed_loop_lean_prover_an_agent_396999_s9) (.seq (.arrow the_closed_loop_lean_prover_an_agent_396999_l9 the_closed_loop_lean_prover_an_agent_396999_s9 the_closed_loop_lean_prover_an_agent_396999_s10) (.seq (.arrow the_closed_loop_lean_prover_an_agent_396999_l10 the_closed_loop_lean_prover_an_agent_396999_s10 the_closed_loop_lean_prover_an_agent_396999_s11) (.seq (.arrow the_closed_loop_lean_prover_an_agent_396999_l11 the_closed_loop_lean_prover_an_agent_396999_s11 the_closed_loop_lean_prover_an_agent_396999_s12) (.seq (.arrow the_closed_loop_lean_prover_an_agent_396999_l12 the_closed_loop_lean_prover_an_agent_396999_s12 the_closed_loop_lean_prover_an_agent_396999_s13) (.seq (.arrow the_closed_loop_lean_prover_an_agent_396999_l13 the_closed_loop_lean_prover_an_agent_396999_s13 the_closed_loop_lean_prover_an_agent_396999_s14) (.seq (.arrow the_closed_loop_lean_prover_an_agent_396999_l14 the_closed_loop_lean_prover_an_agent_396999_s14 the_closed_loop_lean_prover_an_agent_396999_s15) (.arrow the_closed_loop_lean_prover_an_agent_396999_l15 the_closed_loop_lean_prover_an_agent_396999_s15 the_closed_loop_lean_prover_an_agent_396999_s16))))))))))))

-- ── Evaluation arm sub-defs ───────────────────────────────────

-- truth arm
noncomputable def the_closed_loop_lean_prover_an_agent_396999_true_arm : IGProtocol the_closed_loop_lean_prover_an_agent_396999_s0 the_closed_loop_lean_prover_an_agent_396999_s16 :=
  (the_closed_loop_lean_prover_an_agent_396999_protocol).restrictToEVALT

-- false arm
noncomputable def the_closed_loop_lean_prover_an_agent_396999_false_arm : IGProtocol the_closed_loop_lean_prover_an_agent_396999_s0 the_closed_loop_lean_prover_an_agent_396999_s16 :=
  (the_closed_loop_lean_prover_an_agent_396999_protocol).restrictToEVALF

-- ── Verification theorems ─────────────────────────────────────

-- Tier: apply the Grammar to the object (self-application). assess_tier verdict on the imscribed tuple: .O₁.
def the_closed_loop_lean_prover_an_agent_396999_tier : OuroboricityTier := TierFunctor.obj the_closed_loop_lean_prover_an_agent_396999_s0
#eval the_closed_loop_lean_prover_an_agent_396999_tier  -- the Grammar's own verdict on its tier

-- Frobenius (split → fuse): μ∘δ = id on the ground imscription
theorem the_closed_loop_lean_prover_an_agent_396999_frobenius :
    igFrobeniusAlg.mul the_closed_loop_lean_prover_an_agent_396999_s0 the_closed_loop_lean_prover_an_agent_396999_s0 = the_closed_loop_lean_prover_an_agent_396999_s0 :=
  igFrobAlg_self_fusion the_closed_loop_lean_prover_an_agent_396999_s0
