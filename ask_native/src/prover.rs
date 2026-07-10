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

const SCRATCH_MODULE: &str = "Imscribing.Scratch.ProverScratch";
const PLACEHOLDER: &str =
    "import Mathlib\n\ntheorem scratch_ok : (2 : ℝ) + 2 = 4 := by norm_num\n";

const PROVER_SYS: &str = "\
You are a Lean 4 proof engine (toolchain leanprover/lean4:v4.28.0, Mathlib). You \
output ONLY Lean 4 source: a single import line, then the theorem with a COMPLETE \
proof. No prose, no markdown fences, and never `sorry` or `admit`.\n\
IMPORTS: use exactly `import Mathlib` and NOTHING else. Never import a specific \
Mathlib sub-module (paths move between versions and fail as 'bad import'). \
`import Mathlib` already gives you all of Mathlib.\n\
When given a previous attempt and its compiler output, REPAIR it: read the \
`unsolved goals` state, identify the exact gap, and return a full corrected file.";

const DECOMPOSE_SYS: &str = "\
You decompose a hard Lean 4 goal into helper lemmas. You output ONLY Lean lemma \
SIGNATURES, one per line, no proofs, no imports, no prose, no markdown.";

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

fn scratch_file() -> PathBuf {
    p4ramill_dir().join("Imscribing/Scratch/ProverScratch.lean")
}

// ── compile through the kernel ────────────────────────────────────────────────

/// Build `source` as the scratch module. Green iff exit 0, no error, no sorry.
fn compile_lean(source: &str) -> (bool, String) {
    let scratch = scratch_file();
    if let Some(parent) = scratch.parent() {
        let _ = fs::create_dir_all(parent);
    }
    if fs::write(&scratch, source).is_err() {
        return (false, "error: cannot write scratch module".into());
    }
    let out = Command::new("lake")
        .args(["build", SCRATCH_MODULE])
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

fn restore_placeholder() {
    let scratch = scratch_file();
    if let Some(parent) = scratch.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let _ = fs::write(&scratch, PLACEHOLDER);
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
        }
    }

    pub fn available(&self) -> bool {
        self.llm.api_key.is_some()
    }

    /// Escalating driver: B (not closed) is a frontier, so raise depth/budget and
    /// keep grinding until the path closes or the rounds are exhausted.
    pub fn prove(&mut self, goal: &str) -> ProofResult {
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
            let r = self.prove_inner(goal, "import Mathlib", 0);
            if r.closed {
                restore_placeholder();
                return r;
            }
            last = r;
        }
        restore_placeholder();
        last.note = "not closed within escalation cap — a resource frontier (B), \
                     not a verdict of unprovability; raise the rounds/budget to push further"
            .into();
        last
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
            let (green, out) = compile_lean(&source);
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
