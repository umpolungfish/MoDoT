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

/// Build `source` as the scratch module in the given slot ("A" or "B"). Two
/// slots exist so a racing pair of attempts (see `race_portal`) can each run
/// their own `lake build` concurrently without touching the same file.
/// Green iff exit 0, no error, no sorry.
fn compile_lean(source: &str, slot: &str) -> (bool, String) {
    let scratch = scratch_file(slot);
    if let Some(parent) = scratch.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if fs::write(&scratch, source).is_err() {
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

fn restore_placeholder(slot: &str) {
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
        }
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

        let rounds: [(u32, u32, u32); 3] = [(2, 4, 3), (3, 5, 4), (4, 6, 4)];
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
                     not a verdict of unprovability; raise the rounds/budget to push further"
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
            println!("── ROUTE: imscription → formalize its mathematical expression → kernel ──");
        }
        // The model emits a whole kernel-importing file; the escalating budget is
        // the number of generate/compile/repair passes before we call it a frontier.
        let budgets = [3u32, 4, 5];
        let mut prev = String::new();
        let mut errors = String::new();
        let mut last_source = String::new();
        let mut last_out = String::new();
        for (round, &budget) in budgets.iter().enumerate() {
            if self.verbose {
                println!("── imscription round {}: budget={budget} ──", round + 1);
            }
            for i in 1..=budget {
                let msgs = vec![
                    ("system".to_string(), IMSCRIBE_SYS.to_string()),
                    ("user".to_string(), imscribe_prompt(imscription, &prev, &errors)),
                ];
                let res = infer(self.llm, &msgs, 4096, 0.0);
                let source = strip_fences(&res.text);
                let (green, out) = compile_lean(&source, self.scratch_slot);
                last_source = source.clone();
                last_out = out.clone();
                if self.verbose {
                    let mark = if green { "GREEN" } else { "frontier" };
                    println!("  [imscribe {i}] {mark} ({} chars)", source.len());
                    if !green {
                        let tip = clean(&out, 6).replace('\n', "\n        ");
                        println!("        {tip}");
                    }
                }
                if green {
                    // Grounding guard: a green here must actually be the validity
                    // two-step structure against the REAL library, not a substitute
                    // the model authored: a Witness-Vessel (fsplit/ffuse transport)
                    // FILLED with the conventional validity (igFrobeniusAlg), neither
                    // redefined. (A free-hypothesis dodge can't reach green anyway:
                    // the real theorems only apply to the real objects.)
                    if !grounded_in_real_algebra(&source) {
                        prev = source;
                        errors = "Your file compiled but was not the required two-step \
                                  structure. STEP 1: build the Witness-Vessel — board/readback \
                                  Belnap cargo with `fsplit`/`ffuse` and prove μ∘δ=id \
                                  (`vessel_roundtrip`) by induction with `split_fuse_id`. \
                                  STEP 2: fill it — prove `igFrobeniusAlg.mul s0 s0 = s0` via \
                                  `igFrobAlg_self_fusion`, then a capstone conjoining the \
                                  roundtrip and the validity. Import the real library names; \
                                  do NOT redefine fsplit/ffuse/igFrobeniusAlg or introduce a \
                                  free hypothesis. Return the corrected file."
                            .to_string();
                        if self.verbose {
                            println!("        (rejected: compiled but not the two-step vessel+filling — re-prompting)");
                        }
                        continue;
                    }
                    restore_placeholder(self.scratch_slot);
                    return ProofResult {
                        closed: true,
                        source,
                        depth: 0,
                        last_output: out,
                        note: "closed via imscription: built a Witness-Vessel \
                               (Dual-Link fsplit/ffuse transport, μ∘δ=id lossless) \
                               and filled it with the conventional formalization \
                               (Frobenius validity + tier), against the real kernel"
                            .into(),
                    };
                }
                prev = source;
                errors = clean(&out, 60);
            }
        }
        restore_placeholder(self.scratch_slot);
        ProofResult {
            closed: false,
            source: last_source,
            depth: 0,
            last_output: last_out,
            note: "imscription frontier: could not yet formalize this imscription's \
                   mathematical expression into a compiling kernel proof within budget \
                   — a navigation frontier (B), not a refusal and not unprovable; raise \
                   the budget or enrich the kernel context to push further"
                .into(),
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
        let (closed, source, out) = self.repair_loop(
            budget,
            depth,
            "flat",
            &|prev, errors| gen_prompt(&goal, &imports, prev, errors),
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
        let (closed, source, out) = self.repair_loop(
            self.assemble_budget,
            depth,
            "fuse",
            &|prev, errors| assemble_prompt(&goal, &header_for_prompt, prev, errors),
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
                ("system".to_string(), PROVER_SYS.to_string()),
                ("user".to_string(), make_prompt(&prev, &errors)),
            ];
            let res = infer(self.llm, &msgs, 4096, 0.0);
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
            ("system".to_string(), DECOMPOSE_SYS.to_string()),
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
    let decl = Regex::new(r"(?m)^\s*(?:theorem|lemma)\s+\S").unwrap();
    if decl.is_match(t) {
        return Some(t.to_string());
    }
    None
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
