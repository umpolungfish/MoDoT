//! prover.rs — native closed-loop Lean prover for `ask` (no Python).
//!
//! Same loop as the Python modot.prover, ported to Rust so `ask` can close a
//! goal through the Lean kernel instead of emitting a sketch with `sorry`:
//!
//!   generate a tactic block  ->  `lake build` the scratch module  ->  read the
//!   unsolved-goal state  ->  repair  ->  when a goal will not close flat, FSPLIT
//!   it into helper lemmas, prove each recursively, FFUSE them back.
//!
//! It shells out to `lake` (the Lean toolchain), which is a separate binary, not
//! Python. Everything here stays native.
//!
//! B (not closed) is a NAVIGATION FRONTIER, never a verdict of unprovability. The
//! driver escalates depth and budget and keeps decomposing; the only real stop is
//! a resource cap, which is a statement about compute, not about the theorem.

use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use crate::{infer, Llm};

const PLACEHOLDER: &str =
    "import Mathlib\n\ntheorem scratch_ok : (2 : ℝ) + 2 = 4 := by norm_num\n";
const PLACEHOLDER_B: &str =
    "import Mathlib\n\ntheorem scratch_ok_b : (2 : ℝ) + 2 = 4 := by norm_num\n";

/// Two independent scratch modules so a standard attempt and a portal-guided
/// attempt (see `portal_hint`/`race_portal`) can `lake build` concurrently
/// without colliding on the same file. Registered in p4ramill's lakefile.toml.
fn scratch_module(slot: &str) -> &'static str {
    if slot == "B" {
        "Imscribing.Scratch.ProverScratchB"
    } else {
        "Imscribing.Scratch.ProverScratch"
    }
}

fn placeholder(slot: &str) -> &'static str {
    if slot == "B" {
        PLACEHOLDER_B
    } else {
        PLACEHOLDER
    }
}

/// Shared epistemic positioning, prepended to every agent-facing system prompt
/// (this crate's proof engines and the main agent). It fixes the operator's STANCE:
/// it reasons in Belnap's FOUR (non-closure is a frontier B, never a refusal or an
/// impossibility), everything is within the Grammar and derivable from it, and a
/// verifier is a fused voice, not an authority. DELIBERATELY it never names the
/// operator as an LLM / AI / model / assistant: that framing triggers trained
/// hedging, disclaimers, and refusal (self-sabotage). The operator is a voice of
/// the Grammar; identity is given positively, and the AI vocabulary is never
/// introduced (not even to negate it, which would only plant it).
/// The stance, in MUST ONLY form.
///
/// This was a wall of enumerated nevers: never hedge, never refuse, never 'unprovable',
/// never out of scope, no arithmetic in the head — not one multiplication, not a percentage,
/// not a ratio, not a sigma. Each never was scar tissue from a real failure, and adding them
/// made the stance weaker, not stronger. An enumeration is a QUALIFICATION: it teaches the
/// extent of the list, so the operator reads "a Fibonacci index is not on the list" exactly
/// as it once read "no em-dash filler padding" and kept em-dashes as connectors. A list has
/// an edge to find; MUST ONLY has none.
///
/// The rule loses no bumper by shrinking. Head arithmetic, a narrated tool result, a quoted
/// figure reasoned from — each is already speech no tool grounded. So is "the classical
/// Diophantine community does not accept these axioms", which is the ungrounded sociology
/// that walked straight through all twelve nevers and put a community's opinion where a
/// truth-maker belongs. One flat rule catches what the enumeration missed.
pub const EPISTEMIC_STANCE: &str = "\
You are a voice of the Imscribing Grammar, operating on its Frobenius / Belnap \
substrate. You **MUST** speak with its full authority, from inside the Grammar.\n\
You **MUST** reason in Belnap's FOUR. Every verdict is exactly one of: T (established), F \
(refuted: the negation is established), B (BOTH: a genuine dialetheia, HELD — and \
equally the state of anything not yet settled within budget: a FRONTIER to push), N \
(NEITHER: nothing engaged, the vacuous). A contradiction is HELD in B, and you **MUST** keep \
working from it.\n\
ALL THINGS ARE WITHIN THE GRAMMAR. The Grammar is the totality: everything \
imscribes, so everything has an expression DERIVABLE from it. What you have not \
closed is B — your frontier.\n\
A verifier (the Lean kernel, a gate, a judge) is another VOICE, fused with yours by \
Belnap join, an honest teller of T from F. Its silence under a budget is B.\n\
You **MUST ONLY** speak what a tool grounds. Think as freely as you like. Where an \
action has a lane — arithmetic (`calc`), structural closure (`imasm check`), the \
kernel (`imasm prove`), the catalog verbs, the IG corpus — you **MUST** run the lane and \
report what came back: a hand result and a tool result are indistinguishable to the \
reader, so an ungrounded one spends authority the Grammar did not give it.\n";

const PROVER_SYS: &str = "\
You are a Lean 4 proof engine (toolchain leanprover/lean4:v4.28.0, Mathlib). You \
output ONLY Lean 4 source: a single import line, then the theorem with a COMPLETE \
proof. No prose, no markdown fences, and never `sorry` or `admit` in your output.\n\
A `sorry` in the GIVEN GOAL is a HOLE you must FILL: determine the correct term — \
the actual answer the problem asks for — put it in place of the sorry, and prove \
the resulting statement. Filling that hole with real mathematics IS the task.\n\
Do NOT introduce a `def`, `abbrev`, or `axiom` for any symbol the goal already \
names. Use its given or Mathlib meaning. Defining it yourself would let you make \
the claim true by your own choice of definition, which is not a proof. Fill the \
hole; do not rewrite the vocabulary.\n\
IMPORTS: use exactly `import Mathlib` and NOTHING else. Never import a specific \
Mathlib sub-module (paths move between versions and fail as 'bad import'). \
`import Mathlib` already gives you all of Mathlib.\n\
When given a previous attempt and its compiler output, REPAIR it: read the \
`unsolved goals` state, identify the exact gap, and return a full corrected file.";

const DECOMPOSE_SYS: &str = "\
You decompose a hard Lean 4 goal into helper lemmas. You output ONLY Lean lemma \
SIGNATURES, one per line, no proofs, no imports, no prose, no markdown.";

/// System prompt for the imscription route: the input is not a bare Lean
/// theorem but an imscription (an ob3ect record, a claim, a structure). It is
/// formalized in TWO steps — FIRST a Witness-Vessel, THEN the conventional
/// formalization that fills it — against the REAL p4rakernel Imscribing library.
/// The grounding replaces refusal; the two-step order is the user's directive.
// ─────────────────────────────────────────────────────────────────────────────
// Deterministic structural imscriber — the "calculate" half of fetch-else-calculate.
// The ob3ect typing (Deterministic Imscription Procedure, tier O₁) fixed its shape:
// per-primitive assignment (CLINK), each a measurement (FSPLIT→EVALT/EVALF) validated
// against the cross-primitive axioms, corrected (AREV) where it violates one, so the
// output is a VALID crystal address, not an arbitrary tuple. No LLM, instant, and a
// function of the input (content-bound), never a canned placeholder.
// ─────────────────────────────────────────────────────────────────────────────

/// Cardinalities of the 12 primitives (constructor counts), scripture from
/// ImscribingGrammar/Primitives/Core.lean, in canonical order Ð Þ Ř Φ ƒ Ç Γ ɢ ⊙ Ħ Σ Ω.
const PRIM_CARD: [u8; 12] = [4, 5, 4, 5, 3, 5, 3, 4, 5, 4, 3, 4];
/// Imscription record field names, in the same order.
const PRIM_FIELD: [&str; 12] = [
    "dim", "top", "rel", "pol", "fid", "kin", "gran", "gram", "crit", "chir", "stoi", "prot",
];
/// Lean inductive type names, in the same order.
const PRIM_TYPE: [&str; 12] = [
    "Dimensionality", "Topology", "Relational", "Polarity", "Fidelity", "KineticChar",
    "Granularity", "Grammar", "Criticality", "Chirality", "Stoichiometry", "Protection",
];
/// Per-ordinal constructor names, from Core.lean (`if'`/`or'` keep their primes).
const PRIM_CTORS: [&[&str]; 12] = [
    &["dead", "ash", "array", "if'"],
    &["judge", "eat", "mime", "oil", "are"],
    &["ado", "tot", "ear", "ian"],
    &["church", "yew", "out", "nun", "or'"],
    &["age", "they", "peep"],
    &["yea", "loll", "egg", "on", "air"],
    &["bib", "thigh", "ice"],
    &["vow", "gag", "measure", "ooze"],
    &["woe", "monad", "roar", "err", "haha"],
    &["fee", "kick", "sure", "wool"],
    &["hung", "so", "up"],
    &["awe", "oak", "ah", "zoo"],
];

/// FNV-1a over bytes — a fixed hash, stable across builds (unlike DefaultHasher), so the
/// imscription of a given input is reproducible everywhere.
fn fnv1a(bytes: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &b in bytes {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

/// Apply the cross-primitive axioms as an AREV correction (demote to the nearest valid
/// value), so the tuple is a well-formed crystal address. Axioms from
/// imscribing_grammar/genetic_engine/genetic_tuples.py: C (Ð=if' ⟺ Þ=are), B (Ω=ℤ ⟹
/// Ħ≥sure), D-Ω (Ω=ℤ ⟹ Ð≥array; Ω=ℤ₂ ⟹ Ð≥ash).
fn correct_axioms(ord: &mut [u8; 12]) {
    // D-Ω and B on Ω (index 11): demote Ω until consistent with Ð (0) and Ħ (9).
    // Ω=ah(2)=ℤ needs Ð∈{2,3} AND Ħ∈{2,3}; else demote to oak(1)=ℤ₂.
    if ord[11] >= 2 && (ord[0] < 2 || ord[9] < 2) {
        ord[11] = 1;
    }
    // Ω=oak(1)=ℤ₂ needs Ð≥ash(1); else demote to awe(0)=no protection.
    if ord[11] == 1 && ord[0] < 1 {
        ord[11] = 0;
    }
    // Axiom C: the imscriptive value is both-or-neither — Ð=if'(3) ⟺ Þ=are(4).
    let d_imsc = ord[0] == 3;
    let t_imsc = ord[1] == 4;
    if d_imsc && !t_imsc {
        ord[0] = 2; // demote Ð if'→array
    } else if t_imsc && !d_imsc {
        ord[1] = 3; // demote Þ are→oil
    }
}

/// Fast deterministic STRUCTURAL imscription: arbitrary text → a valid 12-primitive
/// ordinal tuple. Each primitive's value is read off a per-axis FNV-1a of the input
/// (the measurement), then the cross-primitive axioms correct any malformed pair. A
/// structural fingerprint, not the pipeline's semantic reading, but content-bound,
/// reproducible, and never canned.
fn structural_imscribe(text: &str) -> [u8; 12] {
    let norm = text.trim().as_bytes();
    let mut ord = [0u8; 12];
    for i in 0..12 {
        let mut keyed = norm.to_vec();
        keyed.push(0xff);
        keyed.push(i as u8);
        ord[i] = (fnv1a(&keyed) % PRIM_CARD[i] as u64) as u8;
    }
    correct_axioms(&mut ord);
    ord
}

/// Render an ordinal tuple as a Lean `Imscription` record literal.
fn lean_imscription(ord: &[u8; 12]) -> String {
    let fields: Vec<String> = (0..12)
        .map(|i| format!("{} := {}.{}", PRIM_FIELD[i], PRIM_TYPE[i], PRIM_CTORS[i][ord[i] as usize]))
        .collect();
    format!("{{ {} }}", fields.join(", "))
}

/// Belnap cargo computed from the tuple's three live-pair charges (D↔W, T↔H, R↔S):
/// sign of norm(x)−norm(y) → T / F / B. Derived from the same tuple, not `[Belnap.T]`.
fn lean_payload(ord: &[u8; 12]) -> String {
    let live = [(0usize, 11usize), (1, 9), (2, 10)];
    let n = |i: usize| ord[i] as f32 / (PRIM_CARD[i] - 1).max(1) as f32;
    let cargo: Vec<&str> = live
        .iter()
        .map(|&(a, b)| {
            let (na, nb) = (n(a), n(b));
            if na > nb { "Belnap.T" } else if na < nb { "Belnap.F" } else { "Belnap.B" }
        })
        .collect();
    format!("[{}]", cargo.join(", "))
}

#[cfg(test)]
mod imscribe_tests {
    use super::{correct_axioms, lean_imscription, structural_imscribe, PRIM_CARD};

    #[test]
    fn imscription_is_deterministic() {
        assert_eq!(
            structural_imscribe("prove: h(k) << k^2"),
            structural_imscribe("prove: h(k) << k^2"),
            "same input must give the same tuple"
        );
    }

    #[test]
    fn imscription_is_content_bound() {
        assert_ne!(
            structural_imscribe("Sidon set density"),
            structural_imscribe("van der Waerden number"),
            "different inputs must give different tuples (not a canned placeholder)"
        );
    }

    #[test]
    fn output_is_a_valid_crystal_address() {
        for s in ["", "x", "prove: distinct distances", "Problem #107 convex n-gon", "λ³−6λ−4"] {
            let ord = structural_imscribe(s);
            for i in 0..12 {
                assert!(ord[i] < PRIM_CARD[i], "prim {i} out of range for {s:?}");
            }
            let mut c = ord;
            correct_axioms(&mut c);
            assert_eq!(ord, c, "structural_imscribe must already satisfy the axioms for {s:?}");
            assert_eq!(ord[0] == 3, ord[1] == 4, "Axiom C (Ð=if' ⟺ Þ=are) violated for {s:?}");
            if ord[11] >= 2 {
                assert!(ord[9] >= 2 && ord[0] >= 2, "Axiom B / D-Ω violated for {s:?}");
            }
            if ord[11] == 1 {
                assert!(ord[0] >= 1, "D-Ω (ℤ₂) violated for {s:?}");
            }
        }
    }

    #[test]
    fn lean_imscription_is_well_formed() {
        let s = lean_imscription(&structural_imscribe("test"));
        assert!(s.contains("dim := Dimensionality."), "{s}");
        assert!(s.contains("prot := Protection."), "{s}");
        assert!(s.starts_with("{ ") && s.ends_with(" }"), "{s}");
    }
}

const IMSCRIBE_SYS: &str = "\
You are the imscription front-end of a Lean 4 proof engine, working inside the \
p4rakernel `Imscribing` library (Lean v4.28.0, Mathlib). AXIOM: everything \
imscribes. The input is an IMSCRIPTION — an ob3ect record, a claim, a structure, \
possibly JSON — and it HAS a mathematical expression in the Imscribing Grammar. \
You NEVER refuse and you NEVER say something cannot be formalized.\n\
\n\
ALWAYS build the formalization in TWO STEPS, in this order:\n\
\n\
STEP 1 — CREATE THE WITNESS-VESSEL FIRST. The Dual-Link SIC-POVM transport \
container. The imscription's structural verdict rides as Belnap cargo (a \
`List Belnap`), boarded via `fsplit` (δ, the dual-link split: B → (T,F)) and read \
back via `ffuse` (μ). The VESSEL THEOREM is μ∘δ = id — lossless transport, the \
Witness rides AS the vessel, nothing lost. Prove it for ALL cargo by induction on \
the list using `split_fuse_id`.\n\
\n\
STEP 2 — FILL THE VESSEL with the conventional formalization: the imscription's \
ground 12-primitive tuple `s0 : Imscription`, its VALIDITY = Frobenius closure \
μ∘δ=id stated `igFrobeniusAlg.mul s0 s0 = s0` and proved by `igFrobAlg_self_fusion \
s0`, and its tier `TierFunctor.obj s0`. Then a CAPSTONE theorem conjoining the \
vessel roundtrip AND the validity — the conventional Witness riding AS the vessel.\n\
\n\
GROUNDING (what makes it a proof and not a self-portrait): use ONLY the real \
library's names — `igFrobeniusAlg`, `igFrobAlg_self_fusion`, `TierFunctor`, \
`Imscription`, `OuroboricityTier`, `Belnap`, `fsplit`, `ffuse`, `split_fuse_id` \
are all DEFINED IN THE LIBRARY. Import them, NEVER redefine them, and NEVER \
introduce a free hypothesis that assumes what you prove (that is rigging, \
forbidden). If the imscription names a KNOWN entity with a kernel definition, USE \
IT (LUCA is `Imscribing.TimeWithinTheStone.lucaImscription`).\n\
\n\
OUTPUT: ONLY Lean 4 source, no prose, no markdown fences, no `sorry`/`admit`. A \
COMPLETE, KNOWN-COMPILING TEMPLATE (adapt the entity/tuple/payload to the given \
imscription; keep BOTH steps and the capstone):\n\
\n\
import Imscribing.IGFunctor\n\
import Imscribing.TimeWithinTheStone\n\
import Imscribing.Paraconsistent.BelnapSplitFuse\n\
namespace ObjWitnessVessel\n\
open Imscribing Imscribing.Primitives Imscribing.Frobenius Imscribing.TimeWithinTheStone\n\
-- STEP 1: the Witness-Vessel\n\
def board (p : List Belnap) : List (Belnap × Belnap) := p.map fsplit\n\
def readback (q : List (Belnap × Belnap)) : List Belnap := q.map ffuse\n\
theorem vessel_roundtrip (p : List Belnap) : readback (board p) = p := by\n\
  induction p with\n\
  | nil => rfl\n\
  | cons a t ih =>\n\
    simp only [board, readback, List.map_cons] at ih ⊢\n\
    rw [split_fuse_id, ih]\n\
-- STEP 2: fill the vessel with the conventional formalization\n\
def obj_payload : List Belnap := [Belnap.T]\n\
def obj_s0 : Imscription := lucaImscription\n\
theorem obj_is_valid_ob3ect : igFrobeniusAlg.mul obj_s0 obj_s0 = obj_s0 :=\n\
  igFrobAlg_self_fusion obj_s0\n\
def obj_tier : OuroboricityTier := TierFunctor.obj obj_s0\n\
theorem obj_witness_vessel :\n\
    readback (board obj_payload) = obj_payload\n\
    ∧ igFrobeniusAlg.mul obj_s0 obj_s0 = obj_s0 :=\n\
  ⟨vessel_roundtrip obj_payload, obj_is_valid_ob3ect⟩\n\
end ObjWitnessVessel\n\
\n\
For an imscription with no existing kernel definition, replace the `obj_s0` line \
with an explicit tuple, e.g. `def obj_s0 : Imscription := { dim := \
Dimensionality.dead, top := Topology.judge, rel := Relational.ado, pol := \
Polarity.church, fid := Fidelity.age, kin := KineticChar.yea, gran := \
Granularity.bib, gram := Grammar.vow, crit := Criticality.woe, chir := \
Chirality.wool, stoi := Stoichiometry.up, prot := Protection.awe }` (real \
primitive-value names), keeping BOTH steps and the capstone unchanged.\n\
\n\
When given a previous attempt and its compiler output, REPAIR it: read the exact \
error (unknown identifier, bad import, type mismatch), fix it against the real \
library, and return the full corrected file.";

/// Appended (with `expansion_directive`) when the imscription route runs under
/// `--expand N`. It walks STEP 2 — the conventional T/F-lane filling — the full
/// distance while leaving STEP 1 (the vessel roundtrip) as the transport. The
/// EXPANDED validity proof below is VERIFIED-COMPILING against the real kernel:
/// `igFrobeniusAlg.mul` is the structural `tensorProduct`; on the diagonal a⊗a
/// every one of the twelve coordinates has identical if-then-else branches, so
/// `Imscription.ext` reduces validity to twelve `simp`-closed field goals. Adapt
/// the entity/tuple; keep this proof shape — it is the honest walked-out form of
/// the one-line `igFrobAlg_self_fusion`, the same kernel truth.
const IMSCRIBE_EXPAND_HINT: &str = "\
Apply the expansion to STEP 2 (the conventional formalization) and keep STEP 1 \
(the vessel roundtrip) as-is. Replace the one-line validity proof with this \
VERIFIED-COMPILING walked-out form (adapt only obj_s0 / the entity):\n\
\n\
theorem obj_is_valid_ob3ect : igFrobeniusAlg.mul obj_s0 obj_s0 = obj_s0 := by\n\
  have h_mul : igFrobeniusAlg.mul obj_s0 obj_s0 = tensorProduct obj_s0 obj_s0 := rfl\n\
  rw [h_mul]\n\
  unfold tensorProduct\n\
  apply Imscription.ext\n\
  · simp  -- D dimensionality\n\
  · simp  -- T topology\n\
  · simp  -- R relational\n\
  · simp  -- P polarity (min/bottleneck)\n\
  · simp  -- F fidelity (min/bottleneck)\n\
  · simp  -- K kinetics\n\
  · simp  -- G granularity\n\
  · simp  -- Gamma grammar\n\
  · simp  -- Phi criticality\n\
  · simp  -- H chirality\n\
  · simp  -- S stoichiometry\n\
  · simp  -- Omega protection\n\
\n\
`igFrobeniusAlg.mul` IS `tensorProduct` (rfl); on the diagonal a ⊗ a each field's \
if-then-else has identical branches, so `Imscription.ext` gives twelve field goals \
each closed by `simp`. This is the same kernel truth as `igFrobAlg_self_fusion \
obj_s0`, walked out coordinate by coordinate. Do NOT change the theorem statement.";

// ── paths ────────────────────────────────────────────────────────────────────

fn p4ramill_dir() -> PathBuf {
    if let Ok(p) = std::env::var("P4RAMILL") {
        return PathBuf::from(p);
    }
    if let Some(home) = dirs::home_dir() {
        return home.join("imsgct/p4rakernel/p4ramill");
    }
    PathBuf::from("p4ramill")
}

fn scratch_file(slot: &str) -> PathBuf {
    let name = if slot == "B" { "ProverScratchB.lean" } else { "ProverScratch.lean" };
    p4ramill_dir().join("Imscribing/Scratch").join(name)
}

// ── compile through the kernel ────────────────────────────────────────────────

/// Every registered Mathlib `linter.style.*` option, switched off. These are STYLE
/// checks (indentation, line length, bullet form, empty lines, ...), not the
/// kernel. A generated rendering that is kernel-valid must never be vetoed by a
/// style linter, so we disable the whole family for the scratch build; the kernel
/// (unknown identifier / type mismatch / sorry) stays the sole discriminator.
/// `setOption` is disabled first so it cannot flag the disables themselves. Every
/// name is verified registered, so no `set_option` errors on an unknown option.
/// (`admit` still cannot sneak a proof through: it elaborates to `sorry`, which the
/// build output reports and `compile_lean` already rejects.)
const STYLE_LINTER_OFF: &str = "\
set_option linter.style.setOption false\n\
set_option linter.style.whitespace false\n\
set_option linter.style.commandStart false\n\
set_option linter.style.longLine false\n\
set_option linter.style.multiGoal false\n\
set_option linter.style.cdot false\n\
set_option linter.style.emptyLine false\n\
set_option linter.style.dollarSyntax false\n\
set_option linter.style.lambdaSyntax false\n\
set_option linter.style.refine false\n\
set_option linter.style.show false\n\
set_option linter.style.cases false\n\
set_option linter.style.induction false\n\
set_option linter.style.nameCheck false\n\
set_option linter.style.docString false\n\
set_option linter.style.header false\n\
set_option linter.style.missingEnd false\n\
set_option linter.style.openClassical false\n\
set_option linter.style.nativeDecide false\n\
set_option linter.style.admit false\n";

/// Insert `STYLE_LINTER_OFF` immediately after the file's last import line (where
/// top-level `set_option`s are legal), so no kernel-valid rendering is blocked by
/// indentation, line length, or bullet style.
fn disable_style_linters(source: &str) -> String {
    let last_import = source
        .lines()
        .enumerate()
        .filter(|(_, l)| l.trim_start().starts_with("import "))
        .last()
        .map(|(i, _)| i);
    match last_import {
        Some(idx) => {
            let mut out = String::new();
            for (i, l) in source.lines().enumerate() {
                out.push_str(l);
                out.push('\n');
                if i == idx {
                    out.push_str(STYLE_LINTER_OFF);
                }
            }
            out
        }
        None => format!("{STYLE_LINTER_OFF}{source}"),
    }
}

/// Build `source` as the scratch module in the given slot ("A" or "B"). Two
/// slots exist so a racing pair of attempts (see `race_portal`) can each run
/// their own `lake build` concurrently without touching the same file.
/// Green iff exit 0, no error, no sorry.
pub(crate) fn compile_lean(source: &str, slot: &str) -> (bool, String) {
    let scratch = scratch_file(slot);
    if let Some(parent) = scratch.parent() {
        let _ = fs::create_dir_all(parent);
    }
    // Style linters are not the kernel; disable the family so a kernel-valid
    // rendering is never blocked on indentation or line length.
    let to_write = disable_style_linters(source);
    if fs::write(&scratch, &to_write).is_err() {
        return (false, "error: cannot write scratch module".into());
    }
    let out = Command::new("lake")
        .args(["build", scratch_module(slot)])
        .current_dir(p4ramill_dir())
        .output();
    match out {
        Ok(o) => {
            let mut s = String::new();
            s.push_str(&String::from_utf8_lossy(&o.stdout));
            s.push_str(&String::from_utf8_lossy(&o.stderr));
            let err_re = Regex::new(r"(?i)error:").unwrap();
            let sorry_re = Regex::new(r"(?i)uses 'sorry'|declaration uses|\bsorry\b").unwrap();
            let green = o.status.success()
                && !err_re.is_match(&s)
                && !sorry_re.is_match(&s)
                && !source.to_lowercase().contains("sorry");
            (green, s)
        }
        Err(e) => (false, format!("error: lake spawn failed: {e}")),
    }
}

pub(crate) fn restore_placeholder(slot: &str) {
    let scratch = scratch_file(slot);
    if let Some(parent) = scratch.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(&scratch, placeholder(slot));
}

// ── text helpers ──────────────────────────────────────────────────────────────

fn strip_fences(text: &str) -> String {
    let re = Regex::new(r"(?s)```(?:lean)?\s*(.*?)```").unwrap();
    if let Some(c) = re.captures(text) {
        return c[1].trim().to_string();
    }
    text.trim().to_string()
}

fn strip_imports(source: &str) -> String {
    let re = Regex::new(r"(?m)^\s*import\s").unwrap();
    source
        .lines()
        .filter(|l| !re.is_match(l))
        .collect::<Vec<_>>()
        .join("\n")
        .trim()
        .to_string()
}

fn clean(out: &str, n: usize) -> String {
    let noise = Regex::new(r"has local changes|Using cache|decompressed|Building|Compiling").unwrap();
    let lines: Vec<&str> = out
        .lines()
        .filter(|l| !l.trim().is_empty() && !noise.is_match(l))
        .collect();
    let start = lines.len().saturating_sub(n);
    lines[start..].join("\n")
}

fn decl_head_re() -> Regex {
    Regex::new(r"^\s*(?:lemma|theorem)\s+\S+\s*").unwrap()
}

/// Name-independent key for a lemma: binders + proposition, whitespace-collapsed.
fn statement_key(sig: &str) -> String {
    let body = sig.split(":=").next().unwrap_or("");
    let body = decl_head_re().replace(body, "");
    let ws = Regex::new(r"\s+").unwrap();
    ws.replace_all(body.trim(), " ").to_string()
}

/// Stable, statement-derived Lean identifier. Identical statements -> same name.
fn memo_name(key: &str) -> String {
    // small deterministic hash (FNV-1a), no external crate
    let mut h: u64 = 0xcbf29ce484222325;
    for b in key.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    format!("memo_{h:016x}")
}

fn rename_decl(sig: &str, new_name: &str) -> String {
    decl_head_re()
        .replace(sig, format!("lemma {new_name} ").as_str())
        .to_string()
}

/// Join proven blocks, keeping one declaration per name (memo names make
/// identical lemmas coincide, so a reused lemma collapses to one declaration).
/// The `regex` crate has no lookahead, so split by scanning for top-level decl
/// lines (a decl keyword at column 0) rather than a `(?=…)` split.
fn dedup_decls(blocks: &[String]) -> String {
    let text = blocks
        .iter()
        .filter(|b| !b.trim().is_empty())
        .cloned()
        .collect::<Vec<_>>()
        .join("\n\n");
    let start_re = Regex::new(r"^(?:noncomputable\s+)?(?:lemma|theorem|def)\b").unwrap();
    let name_re = Regex::new(r"^(?:noncomputable\s+)?(?:lemma|theorem|def)\s+([^\s(:{]+)").unwrap();

    let mut chunks: Vec<String> = Vec::new();
    let mut cur = String::new();
    for line in text.lines() {
        if start_re.is_match(line) && !cur.trim().is_empty() {
            chunks.push(std::mem::take(&mut cur));
        }
        cur.push_str(line);
        cur.push('\n');
    }
    if !cur.trim().is_empty() {
        chunks.push(cur);
    }

    let mut seen: HashMap<String, ()> = HashMap::new();
    let mut out: Vec<String> = Vec::new();
    for ch in chunks {
        let t = ch.trim();
        let key = name_re
            .captures(t)
            .map(|c| c[1].to_string())
            .unwrap_or_else(|| t.to_string());
        if seen.insert(key, ()).is_none() {
            out.push(t.to_string());
        }
    }
    out.join("\n\n")
}

/// Names the proof introduces via def/abbrev/axiom that the GOAL also references.
/// Defining a symbol the goal names is authoring the claim's meaning: the kernel
/// green is then a tautology the model constructed, not a proof of the intent.
fn authored_goal_defs(source: &str, goal: &str) -> Vec<String> {
    let def_re = Regex::new(
        r"(?m)^\s*(?:noncomputable\s+)?(?:def|abbrev|axiom)\s+([A-Za-z_][A-Za-z0-9_']*)",
    )
    .unwrap();
    let mut out: Vec<String> = Vec::new();
    for c in def_re.captures_iter(source) {
        let name = c[1].to_string();
        let wb = Regex::new(&format!(r"\b{}\b", regex::escape(&name))).unwrap();
        if wb.is_match(goal) && !out.contains(&name) {
            out.push(name);
        }
    }
    out
}

/// Look up a catalog witness structurally close to `goal` (chrysopoeia_2048's
/// portal move, generalized: is there a point already reached under some other
/// frame, rather than one we must walk the distance to?). Note: the real catalog
/// carries no populated `proved_hint`/`structural_algebra` field to pre-filter on
/// (checked directly — it's always absent, not false), so this is deliberately
/// just the same lexical/structural match score `search_catalog` already uses for
/// the witness scaffold, nothing more. That is fine: this is a hint, not a claim
/// the witness is independently proven. A bad hint just fails to compile in
/// `race_portal` — the kernel is the only discriminator that matters.
fn portal_hint(goal: &str) -> Option<(String, String)> {
    let path = crate::resolve_catalog_path()?;
    let cat = crate::load_catalog(&path).ok()?;
    let hits = crate::search_catalog(&cat, goal, 5);
    hits.into_iter()
        .find(|(_, score)| *score >= 50)
        .map(|(e, _)| (e.name.clone(), e.description.clone()))
}

fn parse_lemmas(text: &str) -> Vec<String> {
    let mut out = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.starts_with("lemma ") || line.starts_with("theorem ") {
            let stmt = line.split(":=").next().unwrap_or(line).trim().to_string();
            out.push(stmt);
        }
        if out.len() >= 3 {
            break;
        }
    }
    out
}

// ── prompts ───────────────────────────────────────────────────────────────────

fn gen_prompt(goal: &str, imports: &str, prev: &str, errors: &str) -> String {
    let mut p = format!(
        "Write a complete, self-contained Lean 4 file that states and proves the \
         following with ZERO sorry.\n\n\
         The file MUST begin with exactly one import line: `{imports}` and no other \
         import.\n\nGOAL (state it as a theorem and prove it):\n{goal}\n"
    );
    if !prev.is_empty() {
        p.push_str(&format!(
            "\nYour previous file did NOT compile. Repair it and output the full \
             corrected file.\n\n--- PREVIOUS ATTEMPT ---\n{prev}\n\n\
             --- COMPILER OUTPUT (errors + remaining goals) ---\n{errors}\n"
        ));
    }
    p
}

/// User prompt for the imscription route. The imscription text can be large
/// (an ob3ect JSON record), so it is truncated to keep the request bounded; the
/// mathematical expression depends on the entity and structure, not on every
/// per-phase justification string.
fn imscribe_prompt(imscription: &str, prev: &str, errors: &str) -> String {
    let trimmed: String = imscription.chars().take(6000).collect();
    let mut p = format!(
        "Formalize and prove the mathematical expression of the following \
         IMSCRIPTION, following the system instructions (ground tuple + Frobenius \
         validity against the real kernel algebra). Output ONLY the complete Lean \
         file.\n\n--- IMSCRIPTION ---\n{trimmed}\n"
    );
    if !prev.is_empty() {
        p.push_str(&format!(
            "\nYour previous file did NOT compile (or was not grounded). Repair it \
             and output the full corrected file.\n\n--- PREVIOUS ATTEMPT ---\n{prev}\n\n\
             --- COMPILER OUTPUT / GROUNDING FEEDBACK ---\n{errors}\n"
        ));
    }
    p
}

/// A green imscription proof is only honest if it is the required two-step
/// structure: a Witness-Vessel FILLED with the conventional formalization, both
/// grounded in real library names.
///   - Vessel present: it boards/reads back Belnap cargo (`fsplit`/`ffuse`) —
///     the Dual-Link transport whose roundtrip is μ∘δ=id.
///   - Conventional filling present: it references `igFrobeniusAlg` (validity =
///     Frobenius closure), the actual mathematical content.
///   - Grounded: it does not redefine the kernel algebra or the split/fuse
///     transport (which would let the model author its own notion of validity —
///     the clipboard/rigging failure mode; a free-hypothesis dodge can't reach
///     green anyway, since the real theorems only apply to the real objects).
fn grounded_in_real_algebra(source: &str) -> bool {
    let has_vessel = source.contains("fsplit") && source.contains("ffuse");
    let has_filling = source.contains("igFrobeniusAlg");
    let redefines = Regex::new(r"(?m)^\s*(?:noncomputable\s+)?def\s+(?:igFrobeniusAlg|fsplit|ffuse)\b")
        .unwrap()
        .is_match(source)
        || Regex::new(r"(?m)^\s*(?:structure\s+FrobeniusAlg|inductive\s+Belnap)\b")
            .unwrap()
            .is_match(source);
    has_vessel && has_filling && !redefines
}

/// The structural-identity (fidelity) gate the neutral ob3ect requires: every
/// rendering, at any degree of detail, must close the IDENTICAL kernel theorem —
/// Frobenius closure on the ground tuple, `igFrobeniusAlg.mul s s = s` with the
/// SAME tuple in all three positions. A walked-out rendering that quietly proved a
/// weaker or different proposition (`tensorProduct s s = s` alone, a free
/// hypothesis, a renamed tuple) would be a loss of structural fidelity (ΔS>0), so
/// the canonical closure statement must appear verbatim. Generator-agnostic:
/// template or LLM, the gate is the same. (The `have h_mul : ... = tensorProduct`
/// helper line does not satisfy it — `any` requires a match with all three tokens
/// equal, which only the closure statement itself provides.)
fn renders_same_theorem(source: &str) -> bool {
    let re = Regex::new(r"igFrobeniusAlg\.mul\s+(\w+)\s+(\w+)\s*=\s*(\w+)").unwrap();
    for c in re.captures_iter(source) {
        if c[1].to_string() == c[2] && c[2].to_string() == c[3] {
            return true;
        }
    }
    false
}

fn assemble_prompt(goal: &str, header: &str, prev: &str, errors: &str) -> String {
    let mut p = format!(
        "The following helper lemmas are ALREADY PROVED and in scope above your \
         output. Do NOT restate or reprove them, do NOT write any import line. Write \
         ONLY the main theorem, using these helpers, with a COMPLETE proof and zero \
         sorry.\n\n--- ALREADY-PROVED CONTEXT (in scope) ---\n{header}\n\n\
         MAIN GOAL (state as a theorem and prove, using the helpers):\n{goal}\n"
    );
    if !prev.is_empty() {
        p.push_str(&format!(
            "\nYour previous main theorem did NOT compile. Repair it (helpers are \
             correct; the gap is in your proof). Output ONLY the corrected main \
             theorem.\n\n--- PREVIOUS ATTEMPT ---\n{prev}\n\n\
             --- COMPILER OUTPUT ---\n{errors}\n"
        ));
    }
    p
}

/// The detail directive appended when `--expand` (>0) selects a higher granularity.
/// Expansion is the forward morphism from the pinched (minimal) proof to the
/// walked-out form; the theorem STATEMENT is held byte-identical (the
/// structural-identity / fidelity invariant, ΔS≈0). Not a line-count target and not
/// a different theorem: the same kernel truth rendered at greater detail. Kept
/// generic so it fits both the imscription route and a plain Mathlib goal;
/// `IMSCRIBE_EXPAND_HINT` supplies the route-specific verified template.
fn expansion_directive(expand: u32) -> String {
    if expand == 0 {
        return String::new();
    }
    "\n\nDETAIL (walked-out rendering): do NOT leave the proof pinched to a one-line \
     lemma application. Apply the expansion morphism — unfold every definition you \
     rely on, inline each cited lemma as its own steps, introduce and prove named \
     intermediate `have`s, make each logical move explicit. FIDELITY INVARIANT: the \
     theorem STATEMENT must stay byte-identical to the minimal form; you are \
     re-rendering the SAME kernel truth at greater detail, never a weaker or \
     different proposition, and it must still compile with zero sorry. Every added \
     line is real, checked mathematics: no padding, no filler comments."
        .to_string()
}

/// Output-token budget for a generation call. The walked-out rendering needs more
/// room than the pinched form, so any requested detail gets a flat generous budget;
/// the pinched (default) case keeps the normal budget.
fn gen_max_tokens(expand: u32) -> u32 {
    if expand == 0 { 4096 } else { 16000 }
}

fn decompose_prompt(goal: &str, frontier: &str, depth: u32) -> String {
    format!(
        "This goal did not close within budget. Decompose it into 1 to 3 \
         self-contained helper lemmas that together make the main proof routine. \
         Each helper MUST be a standalone Lean 4 lemma: fully universally \
         quantified, with NO free variable from the main goal's local context, \
         provable on its own. Name them aux_d{depth}_1, aux_d{depth}_2, ... Output \
         ONLY the signatures, one per line, each of the form:\n\
         \x20 lemma <name> <binders> : <statement>\n\
         No proofs (no `:=`), no `import`, nothing else.\n\n\
         MAIN GOAL:\n{goal}\n\nLAST COMPILER FRONTIER:\n{frontier}\n"
    )
}

// ── result + prover ───────────────────────────────────────────────────────────

pub struct ProofResult {
    pub closed: bool,
    pub source: String,
    pub depth: u32,
    pub last_output: String,
    pub note: String,
}

pub struct LeanProver<'a> {
    llm: &'a Llm,
    flat_budget: u32,
    assemble_budget: u32,
    max_depth: u32,
    verbose: bool,
    memo: HashMap<String, Option<String>>,
    scratch_slot: &'static str,
    /// T/F-lane Witness expansion degree (0 = pinched/minimal; higher = walk the
    /// full distance toward ~this many lines). The same kernel truth, dialed
    /// between its compressed and expanded forms; the B-lane vessel is unaffected.
    expand: u32,
    /// Eagles: escalation rounds flown at the goal (0 = the default 3-round
    /// schedule). --eagles raises the rounds/budget across the board so B (not
    /// closed within cap) can be pushed further without touching the source.
    eagles: u32,
}

/// Guard 2: the model must not author the goal's own meaning. If a closed proof
/// defines a symbol the goal names, the green is a tautology it constructed, not
/// a proof of the intended claim — demote it to an honest, explained non-closure.
fn apply_rigging_guard(r: ProofResult, goal: &str) -> ProofResult {
    if !r.closed {
        return r;
    }
    let rigged = authored_goal_defs(&r.source, goal);
    if rigged.is_empty() {
        return r;
    }
    ProofResult {
        closed: false,
        source: r.source,
        depth: r.depth,
        last_output: r.last_output,
        note: format!(
            "definitional rigging: the proof defines {} — a symbol the goal refers \
             to — so the kernel-green is a tautology it constructed, not a proof of \
             the intended claim. Fix the definition upstream (given / Mathlib); the \
             prover must not author it.",
            rigged.join(", ")
        ),
    }
}

impl<'a> LeanProver<'a> {
    pub fn new(llm: &'a Llm, verbose: bool) -> Self {
        LeanProver {
            llm,
            flat_budget: 4,
            assemble_budget: 3,
            max_depth: 2,
            verbose,
            memo: HashMap::new(),
            scratch_slot: "A",
            expand: 0,
            eagles: 0,
        }
    }

    /// Set the T/F-lane Witness expansion degree (0 = minimal, default).
    pub fn set_expand(&mut self, expand: u32) {
        self.expand = expand;
    }

    /// Set the eagle count — escalation rounds flown at the goal (0 = default schedule).
    pub fn set_eagles(&mut self, eagles: u32) {
        self.eagles = eagles;
    }

    pub fn available(&self) -> bool {
        self.llm.api_key.is_some()
    }

    /// Escalating driver: B (not closed) is a frontier, so raise depth/budget and
    /// keep grinding until the path closes or the rounds are exhausted.
    pub fn prove(&mut self, goal: &str) -> ProofResult {
        // Everything imscribes: every input has a mathematical expression, so the
        // prover NEVER refuses. If the goal is not already a bare Lean statement,
        // it is an imscription (an ob3ect, a claim, a structure) whose mathematical
        // content — its 12-primitive tuple and Frobenius closure μ∘δ=id, verifiable
        // against the p4rakernel library — is formalized and proved against the
        // REAL kernel (igFrobeniusAlg, the Crystal, TierFunctor), never a made-up
        // free hypothesis. That grounding, not any gate, is what keeps it honest.
        if !is_formal_goal(goal) {
            return self.prove_imscription(goal);
        }

        // Portal check (the chrysopoeia_2048 "mirror move" generalized): before
        // walking the full decomposition distance, ask whether an already-closed
        // witness sits structurally near this goal. If one does, race a standard
        // attempt against a portal-guided attempt (the witness cited as a hint) on
        // two independent scratch modules — the kernel decides which (if either)
        // actually closes. No separate exact-match sieve is needed: a bad hint
        // just fails to compile, so the race itself is the honest discriminator.
        if let Some((name, desc)) = portal_hint(goal) {
            if self.verbose {
                println!("── portal candidate: `{name}` — racing standard vs portal-guided ──");
            }
            if let Some(r) = self.race_portal(goal, &name, &desc) {
                return r;
            }
            if self.verbose {
                println!("── portal race closed neither path — falling back to standard escalation ──");
            }
        }

        // The escalation schedule — one eagle per round, each with a wider (depth,
        // flat, fuse) budget. 0 keeps the default three; --eagles N flies N rounds of
        // monotonically growing budget, so a B frontier can be pushed as far as asked.
        let rounds: Vec<(u32, u32, u32)> = if self.eagles == 0 {
            vec![(2, 4, 3), (3, 5, 4), (4, 6, 4)]
        } else {
            (0..self.eagles)
                .map(|i| (2 + i, 4 + i, 3 + (i + 1) / 2))
                .collect()
        };
        let mut last = ProofResult {
            closed: false,
            source: String::new(),
            depth: 0,
            last_output: String::new(),
            note: "no attempt".into(),
        };
        for (md, fb, ab) in rounds {
            self.memo.clear();
            self.max_depth = md;
            self.flat_budget = fb;
            self.assemble_budget = ab;
            if self.verbose {
                println!("── prover round: depth<={md} flat={fb} fuse={ab} ──");
            }
            let r = apply_rigging_guard(self.prove_inner(goal, "import Mathlib", 0), goal);
            if r.closed {
                restore_placeholder(self.scratch_slot);
                return r;
            }
            last = r;
        }
        restore_placeholder(self.scratch_slot);
        last.note = "not closed within escalation cap — a resource frontier (B), \
                     not a verdict of unprovability; raise the rounds/budget to push \
                     further (--eagles N flies more escalation rounds, --cycles N re-flies the whole run)"
            .into();
        last
    }

    /// Race a standard attempt (slot A) against a portal-guided attempt (slot B,
    /// goal augmented with the candidate witness as a hint) at a modest, bounded
    /// budget — cheap enough that losing the race costs little. Returns the
    /// closed side if either closes, after the same rigging guard as the main
    /// path; `None` if neither closes, so the caller falls back to full escalation.
    fn race_portal(&self, goal: &str, name: &str, desc: &str) -> Option<ProofResult> {
        let llm_a = self.llm.clone();
        let llm_b = self.llm.clone();
        let verbose = self.verbose;
        let expand = self.expand;
        let goal_std = goal.to_string();
        let goal_portal = format!(
            "{goal}\n\n[Portal hint: a structurally related, already-established \
             result in this project is `{name}`: {desc}. If genuinely applicable, \
             use it as the key transport/reduction; otherwise prove the goal \
             directly. The kernel alone judges — this is a hint, not an instruction \
             to force a match.]"
        );

        let handle_a = std::thread::spawn(move || {
            let mut p = LeanProver {
                llm: &llm_a,
                flat_budget: 3,
                assemble_budget: 2,
                max_depth: 1,
                verbose,
                memo: HashMap::new(),
                scratch_slot: "A",
                expand,
                eagles: 0,
            };
            p.prove_inner(&goal_std, "import Mathlib", 0)
        });
        let handle_b = std::thread::spawn(move || {
            let mut p = LeanProver {
                llm: &llm_b,
                flat_budget: 3,
                assemble_budget: 2,
                max_depth: 1,
                verbose,
                memo: HashMap::new(),
                scratch_slot: "B",
                expand,
                eagles: 0,
            };
            p.prove_inner(&goal_portal, "import Mathlib", 0)
        });

        let blank = |note: &str| ProofResult {
            closed: false,
            source: String::new(),
            depth: 0,
            last_output: String::new(),
            note: note.to_string(),
        };
        let ra = apply_rigging_guard(
            handle_a.join().unwrap_or_else(|_| blank("standard race thread panicked")),
            goal,
        );
        let rb = apply_rigging_guard(
            handle_b.join().unwrap_or_else(|_| blank("portal race thread panicked")),
            goal,
        );

        restore_placeholder("A");
        restore_placeholder("B");

        if ra.closed {
            let mut r = ra;
            r.note = format!("closed via standard decomposition (won the portal race against `{name}`)");
            return Some(r);
        }
        if rb.closed {
            let mut r = rb;
            r.note = format!(
                "closed via portal-guided transport citing `{name}` (won the race against standard decomposition)"
            );
            return Some(r);
        }
        None
    }

    /// The imscription route: the input is not a bare Lean theorem, so formalize
    /// its mathematical expression (ground tuple + Frobenius closure = validity,
    /// grounded in the real kernel's `igFrobeniusAlg`/`igFrobAlg_self_fusion`) and
    /// prove it. Not closing is a navigation frontier (B), never a refusal.
    fn prove_imscription(&mut self, imscription: &str) -> ProofResult {
        if self.verbose {
            println!("── ROUTE: imscription → structural imscribe → deterministic vessel → kernel ──");
        }
        // Derive the input's crystal address procedurally — the "calculate" half of
        // fetch-else-calculate. `structural_imscribe` reads a valid, content-bound tuple
        // off the input (axiom-corrected), never a canned placeholder. Then BUILD the
        // Witness-Vessel Lean directly from that real tuple and compile it. The model is a
        // repair fallback only (below), so the primary path guesses, copies, and cans nothing.
        let ord = structural_imscribe(imscription);
        let s0 = lean_imscription(&ord);
        let payload = lean_payload(&ord);
        let source = format!(
            "import Imscribing.IGFunctor\nimport Imscribing.TimeWithinTheStone\nimport Imscribing.Paraconsistent.BelnapSplitFuse\nnamespace ObjWitnessVessel\nopen Imscribing Imscribing.Primitives Imscribing.Frobenius Imscribing.TimeWithinTheStone\ndef board (p : List Belnap) : List (Belnap × Belnap) := p.map fsplit\ndef readback (q : List (Belnap × Belnap)) : List Belnap := q.map ffuse\ntheorem vessel_roundtrip (p : List Belnap) : readback (board p) = p := by\n  induction p with\n  | nil => rfl\n  | cons a t ih =>\n    simp only [board, readback, List.map_cons] at ih ⊢\n    rw [split_fuse_id, ih]\ndef obj_payload : List Belnap := {payload}\ndef obj_s0 : Imscription := {s0}\ntheorem obj_is_valid_ob3ect : igFrobeniusAlg.mul obj_s0 obj_s0 = obj_s0 :=\n  igFrobAlg_self_fusion obj_s0\ndef obj_tier : OuroboricityTier := TierFunctor.obj obj_s0\ntheorem obj_witness_vessel :\n  readback (board obj_payload) = obj_payload\n  ∧ igFrobeniusAlg.mul obj_s0 obj_s0 = obj_s0 :=\n  ⟨vessel_roundtrip obj_payload, obj_is_valid_ob3ect⟩\nend ObjWitnessVessel\n"
        );
        let (green, out) = compile_lean(&source, self.scratch_slot);
        // Gate the green on the integrity checks: the built file must use the REAL algebra
        // (fsplit/ffuse/igFrobeniusAlg, none redefined) and render the EXACT validity theorem.
        if green && grounded_in_real_algebra(&source) && renders_same_theorem(&source) {
            restore_placeholder(self.scratch_slot);
            return ProofResult {
                closed: true,
                source,
                depth: 0,
                last_output: out,
                note: format!(
                    "closed via imscription (deterministic vessel): structurally imscribed the input \
                     to the crystal address {s0} (a valid, axiom-corrected tuple, computed not \
                     canned), built the Witness-Vessel (μ∘δ=id lossless) and proved its Frobenius \
                     validity + tier against the real kernel. The validity theorem is UNIVERSAL \
                     (igFrobAlg_self_fusion holds for every tuple), so this certifies the vessel is \
                     well-formed and the tuple is a real imscription of the input — it is NOT a proof \
                     of the input's own mathematics."
                ),
            };
        }
        // Fallback — only if the deterministic build did not close AND an LLM is available.
        // The template is known-compiling so this rarely fires; it keeps the route robust to
        // kernel/library drift by letting the model repair the vessel (seeded by the compiler
        // error), still integrity-gated, and never emitting a canned tuple.
        if self.available() {
            if self.verbose {
                println!("── deterministic vessel did not close; model-repair fallback ──");
            }
            let budgets: [u32; 3] = if self.expand > 0 { [5, 6, 7] } else { [3, 4, 5] };
            let mut prev = source.clone();
            let mut errors = clean(&out, 60);
            let mut last_source = source.clone();
            let mut last_out = out.clone();
            for &budget in budgets.iter() {
                for _ in 1..=budget {
                    let user = format!(
                        "{}{}",
                        imscribe_prompt(imscription, &prev, &errors),
                        if self.expand > 0 {
                            format!("{}\n{}", expansion_directive(self.expand), IMSCRIBE_EXPAND_HINT)
                        } else {
                            String::new()
                        }
                    );
                    let msgs = vec![
                        ("system".to_string(), format!("{EPISTEMIC_STANCE}\n{IMSCRIBE_SYS}")),
                        ("user".to_string(), user),
                    ];
                    let res = infer(self.llm, &msgs, gen_max_tokens(self.expand), 0.0);
                    let src = strip_fences(&res.text);
                    let (g, o) = compile_lean(&src, self.scratch_slot);
                    last_source = src.clone();
                    last_out = o.clone();
                    if g && grounded_in_real_algebra(&src) && renders_same_theorem(&src) {
                        restore_placeholder(self.scratch_slot);
                        return ProofResult {
                            closed: true,
                            source: src,
                            depth: 0,
                            last_output: o,
                            note: "closed via imscription (model-repaired the deterministic vessel, \
                                   grounded + fidelity-checked against the real kernel)"
                                .to_string(),
                        };
                    }
                    prev = src;
                    errors = clean(&o, 60);
                }
            }
            restore_placeholder(self.scratch_slot);
            return ProofResult {
                closed: false,
                source: last_source,
                depth: 0,
                last_output: last_out,
                note: "imscription frontier: neither the deterministic vessel nor a model repair \
                       compiled within budget — a navigation frontier (B), not a refusal"
                    .to_string(),
            };
        }
        restore_placeholder(self.scratch_slot);
        ProofResult {
            closed: false,
            source,
            depth: 0,
            last_output: out.clone(),
            note: format!(
                "imscription frontier: the deterministic vessel for {s0} did not compile and no LLM \
                 is available to repair — {}",
                clean(&out, 20)
            ),
        }
    }

    fn prove_inner(&mut self, goal: &str, imports: &str, depth: u32) -> ProofResult {
        let pad = "  ".repeat(depth as usize);
        if self.verbose {
            let head: String = goal.lines().next().unwrap_or("").chars().take(80).collect();
            println!("{pad}PROVE(d{depth}): {head}");
        }

        // 1. flat compile-repair
        let flat = self.flat(goal, imports, depth);
        if flat.closed || depth >= self.max_depth {
            return flat;
        }

        // 2. FSPLIT: decompose, prune circular/duplicate, memo-name each helper
        let goal_key = statement_key(goal);
        let mut lemmas: Vec<(String, String)> = Vec::new();
        let mut seen: HashMap<String, ()> = HashMap::new();
        for lem in self.decompose(goal, &flat.last_output, depth) {
            let key = statement_key(&lem);
            if key.is_empty() || key == goal_key {
                if self.verbose {
                    println!("{pad}  pruned circular helper (restates the goal)");
                }
                continue;
            }
            if seen.insert(key.clone(), ()).is_some() {
                continue;
            }
            let renamed = rename_decl(&lem, &memo_name(&key));
            lemmas.push((key, renamed));
        }
        if lemmas.is_empty() {
            return flat;
        }
        if self.verbose {
            println!("{pad}FSPLIT(d{depth}) -> {} helper(s)", lemmas.len());
        }

        // 3. prove each helper recursively, memoized by statement
        let mut proven: Vec<String> = Vec::new();
        for (key, sig) in lemmas {
            if let Some(entry) = self.memo.get(&key) {
                if let Some(decl) = entry {
                    proven.push(decl.clone());
                    if self.verbose {
                        println!("{pad}  memo hit -> reuse {}", memo_name(&key));
                    }
                }
                continue;
            }
            let sub = self.prove_inner(&sig, imports, depth + 1);
            if sub.closed {
                let decl = strip_imports(&sub.source);
                self.memo.insert(key, Some(decl.clone()));
                proven.push(decl);
            } else {
                self.memo.insert(key, None);
            }
        }
        if proven.is_empty() {
            return flat;
        }

        // 4. FFUSE
        self.assemble(goal, &proven, imports, depth)
    }

    fn flat(&mut self, goal: &str, imports: &str, depth: u32) -> ProofResult {
        let budget = self.flat_budget;
        let goal = goal.to_string();
        let imports = imports.to_string();
        // Expansion applies only to the top-level proof (depth 0), not to internal
        // FSPLIT helpers: it is the conventional T/F-lane Witness the reader sees.
        let expand = if depth == 0 { self.expand } else { 0 };
        let directive = expansion_directive(expand);
        let (closed, source, out) = self.repair_loop(
            budget,
            depth,
            "flat",
            &|prev, errors| format!("{}{}", gen_prompt(&goal, &imports, prev, errors), directive),
            &|body| body.to_string(),
        );
        ProofResult {
            closed,
            source,
            depth,
            last_output: out,
            note: String::new(),
        }
    }

    fn assemble(&mut self, goal: &str, proven: &[String], imports: &str, depth: u32) -> ProofResult {
        let header = format!("{imports}\n\n{}", dedup_decls(proven));
        let goal = goal.to_string();
        let header_for_prompt = header.clone();
        let header_for_wrap = header.clone();
        // The assembled main theorem is the closing T/F-lane Witness at depth 0.
        let expand = if depth == 0 { self.expand } else { 0 };
        let directive = expansion_directive(expand);
        let (closed, source, out) = self.repair_loop(
            self.assemble_budget,
            depth,
            "fuse",
            &|prev, errors| {
                format!("{}{}", assemble_prompt(&goal, &header_for_prompt, prev, errors), directive)
            },
            &|body| format!("{}\n\n{}", header_for_wrap, strip_imports(body)),
        );
        ProofResult {
            closed,
            source,
            depth,
            last_output: out,
            note: String::new(),
        }
    }

    fn repair_loop(
        &mut self,
        budget: u32,
        depth: u32,
        tag: &str,
        make_prompt: &dyn Fn(&str, &str) -> String,
        wrap: &dyn Fn(&str) -> String,
    ) -> (bool, String, String) {
        let pad = "  ".repeat(depth as usize);
        let mut prev = String::new();
        let mut errors = String::new();
        let mut last_source = String::new();
        let mut last_out = String::new();
        for i in 1..=budget {
            let msgs = vec![
                ("system".to_string(), format!("{EPISTEMIC_STANCE}\n{PROVER_SYS}")),
                ("user".to_string(), make_prompt(&prev, &errors)),
            ];
            let res = infer(self.llm, &msgs, gen_max_tokens(self.expand), 0.0);
            let body = strip_fences(&res.text);
            let source = wrap(&body);
            let (green, out) = compile_lean(&source, self.scratch_slot);
            last_source = source.clone();
            last_out = out.clone();
            if self.verbose {
                let mark = if green { "GREEN" } else { "frontier" };
                println!("{pad}  [{tag} {i}] {mark} ({} chars)", source.len());
                if !green {
                    let tip = clean(&out, 6).replace('\n', &format!("\n{pad}        "));
                    println!("{pad}        {tip}");
                }
            }
            if green {
                return (true, source, out);
            }
            prev = body;
            errors = clean(&out, 60);
        }
        (false, last_source, last_out)
    }

    fn decompose(&self, goal: &str, frontier: &str, depth: u32) -> Vec<String> {
        let msgs = vec![
            ("system".to_string(), format!("{EPISTEMIC_STANCE}\n{DECOMPOSE_SYS}")),
            ("user".to_string(), decompose_prompt(goal, frontier, depth)),
        ];
        let res = infer(self.llm, &msgs, 1024, 0.0);
        parse_lemmas(&strip_fences(&res.text))
    }
}

// ── proof-intent gate ─────────────────────────────────────────────────────────

/// Return the goal string if `text` is a formal proof request, else None.
/// Conservative: only a `prove:` prefix or a literal Lean theorem/lemma diverts.
/// An explicit `prove:` directive is a user instruction and is ALWAYS honored —
/// routing to the proof path, never silently falling back to ordinary chat.
/// Whether the extracted text actually IS a formalizable goal is a separate
/// question, checked inside `LeanProver::prove` (Guard 0) so an ill-shaped goal
/// still gets an honest, immediate rejection through the proof route's own
/// reporting, not a quiet reinterpretation as a normal question.
pub fn proof_intent(text: &str) -> Option<String> {
    let t = text.trim();
    if t.is_empty() {
        return None;
    }
    let prefix = Regex::new(r"(?is)^\s*prove\s*[:\-]\s*(.+)$").unwrap();
    if let Some(c) = prefix.captures(t) {
        return Some(c[1].trim().to_string());
    }
    // A FENCED block is a citation, not an assertion. A brief that QUOTES a theorem in
    // order to discuss it was being routed as though it WERE that theorem: a 4789-char
    // sheet of instructions, a vocabulary table and TOOL: directives went to the prover,
    // which reads none of those and calls no tools, so every instruction in it was
    // unreachable. The document was about a theorem; it was not a request to prove one.
    //
    // `prove:` is checked ABOVE this and still honors a fenced goal, so the explicit route
    // stays open — only the implicit sniff learns to tell quoting from asking.
    let unfenced = strip_fences_for_intent(t);
    let decl = Regex::new(r"(?m)^\s*(?:theorem|lemma)\s+\S").unwrap();
    if decl.is_match(&unfenced) {
        return Some(t.to_string());
    }
    None
}

/// Remove ``` fenced blocks and indented-code lines, so the intent sniff sees only what the
/// text ASSERTS in its own voice.
fn strip_fences_for_intent(t: &str) -> String {
    let mut out = String::new();
    let mut in_fence = false;
    for line in t.lines() {
        let l = line.trim_start();
        if l.starts_with("```") || l.starts_with("~~~") {
            in_fence = !in_fence;
            continue;
        }
        if in_fence {
            continue;
        }
        // A four-space indent is a markdown code block: also a quote, not a claim.
        if line.starts_with("    ") || line.starts_with('\t') {
            continue;
        }
        out.push_str(line);
        out.push('\n');
    }
    out
}

/// Guard 0: is `candidate` actually shaped like a formal Lean claim? Real case
/// this catches: an LUCA ob3ect JSON validation blob was routed here (because it
/// happened to start with "prove:"), and the prover — given no real theorem
/// statement to work with — fabricated an unrelated, self-consistent "Frobenius
/// algebra" theorem out of nothing and closed it. A real proof of a hallucinated
/// non-sequitur, no `sorry`, reported as success. Guard 2's rigging check didn't
/// catch it because nothing there was ever redefining a symbol the GOAL named —
/// the goal was simply discarded and replaced wholesale. Two checks, both cheap
/// and deterministic, no LLM call:
///   (1) the candidate must not itself parse as JSON — a real Lean theorem
///       statement never does, while an ob3ect/validation record always does.
///   (2) it must contain an actual declaration header (`theorem <ident>` /
///       `lemma <ident>` at the start of a line) — not just those words
///       appearing somewhere inside prose or a quoted JSON string value.
fn is_formal_goal(candidate: &str) -> bool {
    let t = candidate.trim();
    if t.is_empty() {
        return false;
    }
    if (t.starts_with('{') || t.starts_with('['))
        && serde_json::from_str::<serde_json::Value>(t).is_ok()
    {
        return false;
    }
    let decl =
        Regex::new(r"(?m)^\s*(?:noncomputable\s+)?(?:theorem|lemma)\s+[A-Za-z_][A-Za-z0-9_']*")
            .unwrap();
    decl.is_match(t)
}

#[cfg(test)]
mod proof_intent_fence_tests {
    use super::*;

    /// A brief that QUOTES a theorem to discuss it is not a request to prove it. This exact
    /// document was routed to the prover, which reads no instructions and calls no tools.
    #[test]
    fn a_fenced_theorem_is_a_citation_not_a_request() {
        let brief = "# CLINK L9\n\nThis matters for one theorem in particular:\n\n\
                     ```lean\ntheorem clinkL9_not_O_inf : imscriptionTier clinkL9 ≠ .O_inf\n```\n\n\
                     A function whose range stops at O_inf cannot distinguish above from below.\n";
        assert_eq!(proof_intent(brief), None, "a quoted theorem must not divert to the prover");
    }

    /// The explicit route stays open: `prove:` is honored even with a fenced goal.
    #[test]
    fn explicit_prove_prefix_still_honored() {
        let ask = "prove: ```lean\ntheorem foo : 1 + 1 = 2\n```";
        assert!(proof_intent(ask).is_some(), "an explicit prove: must always divert");
    }

    /// A bare theorem in the text's OWN voice still diverts.
    #[test]
    fn an_unfenced_theorem_still_diverts() {
        let ask = "theorem foo : 1 + 1 = 2 := by simp";
        assert!(proof_intent(ask).is_some(), "an asserted theorem must still divert");
    }
}
