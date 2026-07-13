//! momonad-ask — host-native MoDoT-parity language interface.
//!
//! Same operator surface as `momonados_agent.py --ask`, without Python:
//!
//!   ask --ask "full free-text question of any length"
//!   ask --ask ./questions/q7.txt          # auto-detect file path
//!   ask --file prompts/deep.md
//!   cat essay.md | ask --file -
//!   ask -i                                # interactive multi-turn
//!   ask --model google/gemini-3-pro-preview --verbose --file q.txt
//!
//! Pipeline (ManuscriptSpine):
//!   prepare  = IMSCRIB demand + catalog witness scaffold
//!   answer   = FSPLIT model (OpenRouter / Gemini — full length)
//!   complete = EVALT/EVALF Dual-Link co-type + FFUSE + SpineReport
//!
//! Bare-metal QEMU serial is structural dry-run only. THIS binary is the
//! wet-run organism interface for full-length work of the same kind you
//! do with a frontier model in chat.

use clap::Parser;
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::env;
use std::fs;
use std::io::{self, BufRead, Read, Write};
use std::path::{Path, PathBuf};
use std::process;

mod click;
mod prover;

// ── CLI ─────────────────────────────────────────────────────────────────────

/// Default for `--think`: on, unless MODOT_THINK is set to a falsey value (0/false/off/no).
/// Clap's own bool env parse rejects "0", so the env is read here instead.
fn default_think() -> bool {
    match env::var("MODOT_THINK") {
        Ok(v) => !matches!(
            v.trim().to_lowercase().as_str(),
            "0" | "false" | "off" | "no" | "n" | ""
        ),
        Err(_) => true,
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "ask",
    about = "mOMonadOS host ask — full MoDoT-parity language interface (no Python)",
    long_about = "\
Host-native organism interface. Full-length questions, file submission, \
Gemini-class answers, ManuscriptSpine prepare→answer→complete.

  ask --ask \"Is there a graph of chromatic number aleph1 …?\"
  ask --ask ./questions/q7.txt
  ask --file prompts/deep.md --verbose
  cat essay.md | ask --file -
  ask -i --model google/gemini-3-pro-preview

Env:
  MODOT_PROVIDER   openrouter | gemini   (default: openrouter if key set, else gemini)
  MODOT_MODEL      model id              (default: google/gemini-3-flash-preview)
  OPENROUTER_API_KEY / GEMINI_API_KEY / GOOGLE_API_KEY
  MOMONADOS_CATALOG path to IG_catalog.json (optional)
"
)]
struct Cli {
    /// One-shot question, or path to a file (auto-detected if path exists)
    #[arg(long = "ask", short = 'a')]
    ask: Option<String>,

    /// Read question from file (overrides --ask). Use `-` for stdin.
    #[arg(long = "file", short = 'f')]
    file: Option<String>,

    /// Interactive multi-turn conversation
    #[arg(long = "interactive", short = 'i')]
    interactive: bool,

    /// Verbose: print scaffold, spine faces, model meta
    #[arg(long = "verbose", short = 'v')]
    verbose: bool,

    /// Structure-only: no LLM call (catalog scaffold + spine dry face)
    #[arg(long = "dry-run")]
    dry_run: bool,

    /// LLM model (default: $MODOT_MODEL or google/gemini-3-flash-preview)
    #[arg(long = "model", short = 'm', env = "MODOT_MODEL")]
    model: Option<String>,

    /// Provider: openrouter | gemini (default: $MODOT_PROVIDER, else key-based)
    #[arg(long = "provider", env = "MODOT_PROVIDER")]
    provider: Option<String>,

    /// Disable Dual-Link co-type / selectivity (model-only fuse)
    #[arg(long = "no-selectivity")]
    no_selectivity: bool,

    /// Breath cycles (each cycle re-asks with conversation context)
    #[arg(long = "cycles", default_value_t = 1)]
    cycles: u32,

    /// Eagles: agentic ACT→OBSERVE rounds flown out to run tools within each cycle
    /// (0 = auto: 40 under --jam, 5 otherwise). Honored across the board — jam,
    /// normal ask, and the kernel-gated prover's escalation schedule.
    #[arg(long = "eagles", default_value_t = 0)]
    eagles: u32,

    /// Max output tokens for the model (default 16384 — full-length answers)
    #[arg(long = "max-tokens", default_value_t = 16384)]
    max_tokens: u32,

    /// Temperature
    #[arg(long = "temperature", default_value_t = 0.7)]
    temperature: f32,

    /// Path to IG_catalog.json (default: search common locations)
    #[arg(long = "catalog")]
    catalog: Option<PathBuf>,

    /// Degree of detail for the rendered proof (the conventional T/F-lane Witness).
    /// 0 = the pinched minimal form (default); >0 = the walked-out form: the SAME
    /// kernel theorem rendered at greater detail by the expansion morphism, with the
    /// theorem statement held byte-identical (the structural-identity / fidelity
    /// gate). Not a line-count target and never a weaker proposition. The B-lane
    /// Witness (the Dual-Link vessel) is unaffected.
    #[arg(long = "expand", default_value_t = 0)]
    expand: u32,

    /// Click-maths. Two names `--click A B`: fuse A and B if complementary on one
    /// live conjugate pair (D↔W, T↔H, R↔S). One name `--click A`: SWEEP A against
    /// the whole catalog and rank what it fuses with by product tier.
    #[arg(long = "click", num_args = 1..=2, value_names = ["A", "B"])]
    click: Option<Vec<String>>,

    /// Result count for the --click sweep (default 15).
    #[arg(long = "top", default_value_t = 15)]
    top: usize,

    /// For --click A B: certify the fused product's Frobenius closure through the
    /// Lean kernel (lake build igFrobeniusAlg.mul p p = p). Turns "closes" from an
    /// asserted valid tuple into a real kernel verdict.
    #[arg(long = "certify")]
    certify: bool,

    /// Switch: analyze two co-typed forms as a reversible bistable toggle (the DASA
    /// archetype) — the toggling live pair, the photochromic sign, δ (light) / μ
    /// (heat) legs. `./ask --switch A B`
    #[arg(long = "switch", num_args = 2, value_names = ["A", "B"])]
    switch: Option<Vec<String>>,

    /// For --click A B: register the fused product as a first-class catalog object
    /// and decompose it through the cl8nk_navigator (harness the chimera's existence).
    /// Optional value is the entry name; default `chimera_A_B`. `--click A B --register [NAME]`
    #[arg(long = "register", num_args = 0..=1, default_missing_value = "")]
    register: Option<String>,

    /// Excited-state analysis. `--excite A` promotes A to its excited manifold
    /// (Criticality ⊙ → the non-Hermitian exceptional-point resonance) and reports
    /// the δ (light) promotion and the μ (relaxation/fluorescence) + productive decay
    /// legs. On a `--set` line, bare `--excite` makes the transfer PHOTOINDUCED (the
    /// donor is excited first, opening the driving-force gap). `--excite A [--certify] [--register]`
    #[arg(long = "excite", num_args = 0..=1, default_missing_value = "")]
    excite: Option<String>,

    /// Single-electron transfer (SET). `--set D A` transfers one winding quantum Ω
    /// (the quantized charge) from donor to acceptor: donor oxidized (D•⁺), acceptor
    /// reduced (A•⁻), total Ω conserved. Donor/acceptor set by Criticality ⊙ (energy).
    /// Add `--catalyst M` for a Cu-NO-class mediator, `--excite` for photoinduced ET.
    #[arg(long = "set", num_args = 2, value_names = ["D", "A"])]
    set: Option<Vec<String>>,

    /// Homolytic cleavage → NEUTRAL radicals (the δ_A symmetric split, the reverse of
    /// --click). `--homolyze A B` cleaves the A—B σ-bond into A• + B•; `--homolyze A`
    /// splits A symmetrically into two identical radicals A•. Contrast --set (the
    /// single-electron / heterolytic route → radical IONS A•⁺/B•⁻).
    #[arg(long = "homolyze", num_args = 1..=2, value_names = ["A", "B"])]
    homolyze: Option<Vec<String>>,

    /// Bidirectional ligand ⇌ catalytic-site complement (ported from red-hot_rebis
    /// ligand_from_active_site). `--complement A` maps a catalytic-site type to the
    /// complementary ligand it binds — and back, it is its own inverse. --certify /
    /// --register apply to the derived ligand.
    #[arg(long = "complement", num_args = 1, value_name = "A")]
    complement: Option<String>,

    /// With `--set D A`: scan the whole catalog for the best mediators of that
    /// transfer — holdable winding (Ω), ⊙ relay between donor and acceptor, and
    /// bidirectional recognition (its complement binds both substrates). Ranked; --top bounds it.
    #[arg(long = "scan-mediators")]
    scan_mediators: bool,

    /// Catalytic cycle: `--cycle CATALYST SUBSTRATE` — the closed loop. bind →
    /// working stroke δ (one winding quantum moves, substrate→product, catalyst spent)
    /// → return stroke μ (regeneration) → turnover, with the catalyst a fixed point
    /// (μ∘δ=id). --certify proves the loop closes; --register canonizes the product.
    #[arg(long = "cycle", num_args = 2, value_names = ["CATALYST", "SUBSTRATE"])]
    cycle: Option<Vec<String>>,

    /// Metabolic pathway: `--pathway SUBSTRATE C1 C2 … Cn` — chain the loops. The
    /// substrate runs through the catalyst sequence, one turnover each, the winding
    /// quantum the carrier passed along. If the carrier returns to its start it CLOSES
    /// into a metabolic cycle (a loop of loops). --certify verifies each catalyst regenerates.
    #[arg(long = "pathway", num_args = 2.., value_names = ["SUBSTRATE", "CATALYSTS"])]
    pathway: Option<Vec<String>>,

    /// Imscriptive polymerization: `--polymerize M1 M2 … Mn` — chain the clicks. Each
    /// bond is a Coagula link between complementary partners (step-growth) or an
    /// addition where a monomer repeats (chain-growth); the monomer SEQUENCE stays
    /// losslessly readable off the chain (R∧W∧X). Reports degree of polymerization,
    /// regioregularity, copolymer architecture, tacticity (the Ħ chirality sequence),
    /// and whether it cyclizes head-to-tail into a macrocycle. --certify closes each unit.
    #[arg(long = "polymerize", num_args = 2.., value_names = ["MONOMERS"])]
    polymerize: Option<Vec<String>>,

    /// With `--polymerize`: if the chain does not cyclize, search the catalog for the
    /// monomer that CLOSES it into a ring (clicks the tail and the head) or BRIDGES a
    /// co-typed break (clicks both sides of the failed junction). The honest cyclization
    /// search — distinct from `--scan-mediators`, which ranks SET electron relays.
    #[arg(long = "close")]
    close: bool,

    /// With `--polymerize`: if the polymer is CLOSED (cyclic), characterize the ring as a
    /// mathematical material — is it conductive (a winding quantum Ω circulates the loop),
    /// frustrated, or insulating; and its weakest ring bond. Grounds the transport claims
    /// prose asserts about a cyclic "computer".
    #[arg(long = "props")]
    props: bool,

    /// With `--polymerize`: search for a monomer that generates a SUSTAINING loop — a
    /// conductive cycle (a persistent Ω current, ∮ closes) somewhere along the chain, and
    /// report its period (the modulus). Distinct from `--close`: a ring can close and
    /// still be static (insulating, no modulus). A modulus is elasticity, a sustaining loop.
    #[arg(long = "modulus")]
    modulus: bool,

    /// With `--polymerize`: treat the monomers as an UNORDERED set and search orderings for
    /// the one that polymerizes best (longest enchainment, then closure, then stability),
    /// then run the full analysis on that order. A set has no inherent order — this finds
    /// the sequence instead of assuming the one given. Exhaustive for ≤9 monomers.
    #[arg(long = "arrange")]
    arrange: bool,

    /// Forge a mathematical material: `--forge M1 M2 … Mn` treats the monomers as a set,
    /// finds the best-ringing order, and prints the full material sheet — topology,
    /// stability, conductance, and spectral invariants (adjacency spectrum, spectral radius
    /// ρ, gap). The one-flag deterministic characterize path (no LLM); = `--arrange --props`.
    #[arg(long = "forge", num_args = 2.., value_names = ["MONOMERS"])]
    forge: Option<Vec<String>>,

    /// Compare two materials: `--compare A B C vs X Y Z`. Forge both rings and diff them by
    /// spectral radius, conductance class, and weakest bond.
    #[arg(long = "compare", num_args = 3.., value_names = ["A..vs..B"])]
    compare: Option<Vec<String>>,

    /// Dope a material: `--dope A B C with D`. Forge the base ring, then re-forge it with the
    /// dopant unit mixed in, and report the shift in ρ and conductance.
    #[arg(long = "dope", num_args = 3.., value_names = ["BASE..with..DOPANT"])]
    dope: Option<Vec<String>>,

    /// Fuse two rings: `--fuse A B C + X Y Z`. Forge each ring, then forge the union into one
    /// macrocycle, and report how the fused ρ/conductance relate to the two parents.
    #[arg(long = "fuse", num_args = 5.., value_names = ["A..+..B"])]
    fuse: Option<Vec<String>>,

    /// Cleave a ring: `--cleave M1 M2 … Mn`. Ring fission (the reverse of --fuse) — forge the
    /// set into its best ring, then cut it into two daughter rings on complementary arcs and
    /// report both daughters and their spectra.
    #[arg(long = "cleave", num_args = 4.., value_names = ["MONOMERS"])]
    cleave: Option<Vec<String>>,

    /// Anneal a ring: `--anneal M1 M2 … Mn`. Relax the ring to its lowest-strain ordering —
    /// the ordering that rings with the most uniform bond loading, its settled ground state.
    #[arg(long = "anneal", num_args = 3.., value_names = ["MONOMERS"])]
    anneal: Option<Vec<String>>,

    /// Distillation: separate a set by volatility (Criticality ⊙) into distillate / bottoms.
    #[arg(long = "distill", num_args = 2.., value_names = ["MONOMERS"])]
    distill: Option<Vec<String>>,

    /// Fractional distillation: rank the whole set by volatility ⊙, plate by plate.
    #[arg(long = "fdistill", num_args = 2.., value_names = ["MONOMERS"])]
    fdistill: Option<Vec<String>>,

    /// Sublimation: purify one unit by a two-state skip across ⊙.
    #[arg(long = "sublime", num_args = 1, value_name = "A")]
    sublime: Option<String>,

    /// Crystallization: grow the ordered lattice from a set, rejecting units that do not fit.
    #[arg(long = "crystallize", num_args = 2.., value_names = ["MONOMERS"])]
    crystallize: Option<Vec<String>>,

    /// Co-crystallization: one non-covalent lattice of two complementary components.
    #[arg(long = "cocrystallize", num_args = 2, value_names = ["A", "B"])]
    cocrystallize: Option<Vec<String>>,

    /// Seeding: template a set's crystal on a seed's handedness. `seed M1 M2 … with S`.
    #[arg(long = "seed", num_args = 3.., value_names = ["MONOMERS"])]
    seed: Option<Vec<String>>,

    /// TLC: analytical chromatography — spread a set by Rf and count the bands.
    #[arg(long = "tlc", num_args = 2.., value_names = ["MONOMERS"])]
    tlc: Option<Vec<String>>,

    /// Column chromatography: elute a set by retention. `column M1 M2 … [on S]`.
    #[arg(long = "column", num_args = 2.., value_names = ["MONOMERS"])]
    column: Option<Vec<String>>,

    /// Freeze-pump-thaw: degas a set, shedding weakly-held units.
    #[arg(long = "fpt", num_args = 2.., value_names = ["MONOMERS"])]
    fpt: Option<Vec<String>>,

    /// Ionic trapping: sequester a unit by its charge. `trap A [X]`.
    #[arg(long = "trap", num_args = 1..=2, value_names = ["A", "X"])]
    trap: Option<Vec<String>>,

    /// Stain: apply a diagnostic reagent to a set. `stain R M1 M2 …`.
    #[arg(long = "stain", num_args = 2.., value_names = ["REAGENT_THEN_UNITS"])]
    stain: Option<Vec<String>>,

    /// Recall a registered material by name: `--recall NAME`. Prints its stored sheet from the
    /// material registry without respecifying it from units. Pair with `--forge … --register`.
    #[arg(long = "recall", value_name = "NAME")]
    recall: Option<String>,

    /// Export a material sheet to a standalone file: `--forge … --export PATH` (or
    /// `--recall NAME --export PATH`). Writes the whole record as portable JSON.
    #[arg(long = "export", value_name = "PATH")]
    export: Option<String>,

    /// Jam mode: turn the agent loose on the catalog with the full toolset and no question to
    /// answer. Feed a seed (a lemma, a pile of context, `--file …`) or nothing, and let it
    /// forge / fuse / cleave / dope / anneal / compare freely, following its own curiosity.
    /// The only rule stays the golem rule: the final report may state only what a tool
    /// returned. Runs a long agentic loop before it writes up what it actually found.
    #[arg(long = "jam")]
    jam: bool,

    /// Toggle model reasoning ("thinking") tokens. On by default; `--think false` (or
    /// `--no-think`) turns it off, sending the provider's disable-reasoning parameter
    /// (OpenRouter `reasoning.enabled=false`, Gemini `thinkingConfig.thinkingBudget=0`).
    /// Env: MODOT_THINK=0/false to default it off. Bare `--think` forces it on.
    #[arg(
        long = "think",
        num_args = 0..=1,
        default_value_t = default_think(),
        default_missing_value = "true",
        action = clap::ArgAction::Set,
    )]
    think: bool,

    /// Alias for `--think false`: disable model reasoning tokens outright.
    #[arg(long = "no-think", default_value_t = false)]
    no_think: bool,

    /// Assemble a STAR polymer: `--star M1 M2 …` picks the highest-functionality monomer as
    /// the core and attaches every unit that clicks with it as an arm (pure star K(1,f), ρ=√f).
    #[arg(long = "star", num_args = 4.., value_names = ["MONOMERS"])]
    star: Vec<String>,

    /// Narrow the catalog to the structural floor of a reference set: `--filter A B [C …]`
    /// keeps every entry matching all the primitive values the references share.
    #[arg(long = "filter", num_args = 2.., value_names = ["REFS"])]
    filter: Vec<String>,

    /// Construct the next ramified level of a tower from the excited state: `--ascend A`
    /// excites A, then IFIX-continues it past the exceptional point and adds one winding Ω.
    #[arg(long = "ascend", value_name = "NAME")]
    ascend: Option<String>,

    /// Recover the relative phase word of a set from its closed ring:
    /// `--phase-reconstruct M1 M2 …` reads back the per-unit Ħ phase sequence (or reports N).
    #[arg(long = "phase-reconstruct", num_args = 2.., value_names = ["MONOMERS"])]
    phase_reconstruct: Vec<String>,

    /// Create a missing catalog entry by imscribing it via the real generate pipeline
    /// (`imscribe generate … --name <NAME>`), writing to the live catalog MoDoT merges.
    /// Optionally pass a free-text description in --rest; defaults to the humanized name.
    #[arg(long = "imscribe", value_name = "NAME")]
    imscribe: Option<String>,

    /// Spring-loaded offset threshold for --click (default 0.5).
    #[arg(long = "theta", default_value_t = 0.5)]
    theta: f32,

    /// Optional catalyst for --click: a Frobenius-special fragment (e.g.
    /// math_isomorphism) that lowers the effective θ (barrier reduction) so a
    /// weakly-complementary pair can fuse, then is regenerated unchanged (μ∘δ=id).
    /// Lowers ΔG‡, never ΔG — cannot make a same-sign/neutral pair click.
    #[arg(long = "catalyst")]
    catalyst: Option<String>,

    /// Load a file or DIRECTORY as background context, prepended to the question (repeatable).
    /// A directory is walked and its text files concatenated. Distinct from the submission:
    /// `--context ./refs --ask "…"` answers the question WITH the corpus as background.
    #[arg(long = "context", value_name = "PATH")]
    context: Vec<String>,

    /// Positional fallback: treated as --ask if --ask/--file omitted. A single positional that
    /// is a FILE or DIRECTORY path is read as the submission (a directory is concatenated).
    /// No longer trailing: flags may appear before, after, or between positional words.
    #[arg(value_name = "TEXT")]
    rest: Vec<String>,
}

// ── Input resolution (MoDoT resolve_input parity) ───────────────────────────

/// Directories skipped when walking a submission/context tree (VCS + build noise), and file
/// extensions treated as binary (not read as text).
const WALK_SKIP_DIRS: &[&str] = &[
    ".git", "target", "node_modules", ".lake", "__pycache__", ".venv", "build", ".mypy_cache",
];
const WALK_BIN_EXTS: &[&str] = &[
    "png", "jpg", "jpeg", "gif", "bmp", "pdf", "zip", "gz", "tar", "olean", "oleanpart", "bin",
    "exe", "so", "o", "a", "wasm", "ico", "woff", "woff2", "ttf", "otf", "mp4", "mp3", "wav",
    "class", "pyc", "lock",
];
/// Cap on how much a directory submission/context may pull in, so a huge tree cannot blow up
/// the prompt. Files are added in sorted order until the cap is reached.
const WALK_CHAR_CAP: usize = 2_000_000;

fn collect_text_files(dir: &Path, out: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    let mut items: Vec<PathBuf> = entries.flatten().map(|e| e.path()).collect();
    items.sort();
    for p in items {
        let name = p.file_name().map(|n| n.to_string_lossy().to_string()).unwrap_or_default();
        if p.is_dir() {
            if name.starts_with('.') || WALK_SKIP_DIRS.contains(&name.as_str()) {
                continue;
            }
            collect_text_files(&p, out);
        } else if p.is_file() {
            let is_bin = p
                .extension()
                .and_then(|x| x.to_str())
                .map(|e| WALK_BIN_EXTS.contains(&e.to_lowercase().as_str()))
                .unwrap_or(false);
            if !is_bin {
                out.push(p);
            }
        }
    }
}

/// Read a path that may be a FILE or a DIRECTORY. A file returns its text; a directory is
/// walked (sorted, recursive, skipping VCS/build noise and binaries) and its text files
/// concatenated with `===== relative/path =====` headers, capped at WALK_CHAR_CAP — so a whole
/// corpus can be submitted or supplied as context in one shot. Returns (content, label).
fn read_path(raw: &str) -> Result<(String, String), String> {
    let p = expand_user(raw);
    let path = Path::new(&p);
    if path.is_file() {
        let content = fs::read_to_string(path).map_err(|e| format!("read {p}: {e}"))?;
        let label = format!("file:{raw} ({} chars)", content.chars().count());
        return Ok((content, label));
    }
    if path.is_dir() {
        let mut files = Vec::new();
        collect_text_files(path, &mut files);
        if files.is_empty() {
            return Err(format!("directory has no readable text files: {raw}"));
        }
        let mut out = String::new();
        let mut n = 0usize;
        let mut truncated = false;
        for f in &files {
            if out.len() >= WALK_CHAR_CAP {
                truncated = true;
                break;
            }
            if let Ok(text) = fs::read_to_string(f) {
                let rel = f.strip_prefix(path).unwrap_or(f);
                out.push_str(&format!("\n===== {} =====\n", rel.display()));
                out.push_str(&text);
                out.push('\n');
                n += 1;
            }
        }
        if n == 0 {
            return Err(format!("directory has no readable UTF-8 text files: {raw}"));
        }
        if truncated {
            out.push_str(&format!("\n===== [truncated at {WALK_CHAR_CAP} chars] =====\n"));
        }
        let label = format!("dir:{raw} ({n} files, {} chars{})", out.chars().count(), if truncated { ", truncated" } else { "" });
        return Ok((out, label));
    }
    Err(format!("path not found: {raw}"))
}

fn resolve_input(ask: Option<&str>, file: Option<&str>, rest: &[String]) -> Result<(String, String), String> {
    if let Some(fp) = file {
        return read_file_or_stdin(fp);
    }
    if let Some(a) = ask {
        let p = expand_user(a);
        if Path::new(&p).exists() {
            return read_path(a);
        }
        if a == "-" {
            return read_file_or_stdin("-");
        }
        return Ok((a.to_string(), format!("literal ({} chars)", a.chars().count())));
    }
    if !rest.is_empty() {
        // A single positional that is a real path (file or dir) is read as the submission;
        // otherwise the positionals are joined into a literal question.
        if rest.len() == 1 {
            let p = expand_user(&rest[0]);
            if Path::new(&p).exists() {
                return read_path(&rest[0]);
            }
        }
        let joined = rest.join(" ");
        return Ok((joined.clone(), format!("literal ({} chars)", joined.chars().count())));
    }
    Err("no question: use --ask, --file, positional text/path, or -i".into())
}

/// Load one or more `--context` paths (files or directories) into a single background block,
/// each under its own header, for prepending to the submission.
fn load_context(paths: &[String]) -> Result<String, String> {
    if paths.is_empty() {
        return Ok(String::new());
    }
    let mut out = String::new();
    for raw in paths {
        let (content, label) = read_path(raw)?;
        out.push_str(&format!("\n========== CONTEXT: {label} ==========\n"));
        out.push_str(&content);
        out.push('\n');
    }
    Ok(out)
}


fn expand_user(p: &str) -> String {
    if let Some(rest) = p.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(rest).display().to_string();
        }
    }
    p.to_string()
}

fn read_file_or_stdin(fp: &str) -> Result<(String, String), String> {
    if fp == "-" {
        let mut buf = String::new();
        io::stdin()
            .read_to_string(&mut buf)
            .map_err(|e| format!("stdin: {e}"))?;
        return Ok((buf.clone(), format!("stdin ({} chars)", buf.chars().count())));
    }
    // A file or a DIRECTORY (read_path concatenates a directory tree).
    read_path(fp)
}

// ── Catalog / scaffold (IMSCRIB witness face) ───────────────────────────────

#[derive(Clone, Debug)]
struct CatalogEntry {
    name: String,
    description: String,
    proved_hint: Option<bool>,
    tier: Option<String>,
    d_cl8: Option<f64>,
    raw: Value,
}

fn find_catalog(cli: &Cli) -> Option<PathBuf> {
    if let Some(p) = &cli.catalog {
        if p.is_file() {
            return Some(p.clone());
        }
    }
    resolve_catalog_path()
}

/// Catalog path resolution without a `Cli` (env var + candidate search only).
/// Shared by `find_catalog` (CLI entry point) and the prover's portal lookup
/// (`prover::portal_hint`), which has no `Cli` of its own.
pub(crate) fn resolve_catalog_path() -> Option<PathBuf> {
    if let Ok(p) = env::var("MOMONADOS_CATALOG") {
        let pb = PathBuf::from(expand_user(&p));
        if pb.is_file() {
            return Some(pb);
        }
    }
    let mut candidates = Vec::new();
    // Canonical catalog first (imscribing_grammar/IG_catalog.json is the ONE
    // source of truth per [[project_ig_catalog]]; mOMonadOS/IG_catalog.json is a
    // stale, smaller, unsynced copy — see [[project_lean_prover_loop]]).
    candidates.push(PathBuf::from("../imscribing_grammar/IG_catalog.json"));
    candidates.push(PathBuf::from("../../imscribing_grammar/IG_catalog.json"));
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("../../../../imscribing_grammar/IG_catalog.json"));
            candidates.push(dir.join("../../../../../imscribing_grammar/IG_catalog.json"));
        }
    }
    // relative to this binary / CWD / repo layouts
    candidates.push(PathBuf::from("IG_catalog.json"));
    candidates.push(PathBuf::from("mOMonadOS/IG_catalog.json"));
    candidates.push(PathBuf::from("../IG_catalog.json"));
    if let Ok(exe) = env::current_exe() {
        if let Some(dir) = exe.parent() {
            candidates.push(dir.join("IG_catalog.json"));
            candidates.push(dir.join("../IG_catalog.json"));
            candidates.push(dir.join("../../IG_catalog.json"));
            candidates.push(dir.join("../../../IG_catalog.json"));
        }
    }
    // relative to common layouts (no machine-absolute paths)
    candidates.push(PathBuf::from("../mOMonadOS/IG_catalog.json"));
    candidates.push(PathBuf::from("../../mOMonadOS/IG_catalog.json"));
    for c in candidates {
        if c.is_file() {
            return Some(c);
        }
    }
    None
}

/// Fold the live-crawler catalog (~/.imscrbgrmr/catalog.json) into the base
/// catalog, first-name-wins. Mirrors the Python Witness arm's
/// `_merge_live_catalog` and the canonical `imscribe catalog list` CLI's
/// read-time merge (the 5275-vs-5292 gap). Returns entries added.
fn merge_live_catalog(out: &mut Vec<CatalogEntry>) -> usize {
    let live_path = PathBuf::from(expand_user("~/.imscrbgrmr/catalog.json"));
    let Ok(text) = fs::read_to_string(&live_path) else {
        return 0;
    };
    let Ok(v) = serde_json::from_str::<Value>(&text) else {
        return 0;
    };
    let live = v
        .get("imscriptions")
        .cloned()
        .unwrap_or(v);
    let Some(arr) = live.as_array() else {
        return 0;
    };
    let existing: std::collections::HashSet<String> =
        out.iter().map(|e| e.name.clone()).collect();
    let mut added = 0usize;
    for item in arr {
        let name = item
            .get("name")
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string();
        if name.is_empty() || existing.contains(&name) {
            continue;
        }
        let description = item
            .get("description")
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string();
        let proved_hint = item.get("proved_hint").and_then(|x| x.as_bool());
        let sa = item.get("structural_algebra");
        let tier = sa
            .and_then(|s| s.get("ouroboricity_tier"))
            .and_then(|x| x.as_str())
            .map(|s| s.to_string());
        let d_cl8 = sa
            .and_then(|s| s.get("distance_from_cl8nk"))
            .and_then(|x| x.as_f64());
        out.push(CatalogEntry {
            name,
            description,
            proved_hint,
            tier,
            d_cl8,
            raw: item.clone(),
        });
        added += 1;
    }
    added
}

fn load_catalog(path: &Path) -> Result<Vec<CatalogEntry>, String> {
    // The catalog file is rewritten in place by other processes (imscribe, the live
    // updater). A shelled verb that reads it mid-write sees an empty or truncated file and
    // fails with "catalog json: EOF at line 1 column 0" — which spuriously drops a real
    // result to "no catalog loaded" (seen: `material` failing while `forge` a moment earlier
    // loaded fine). An empty/truncated read is transient: retry a few times with a short
    // backoff so the concurrent write can finish. A genuinely malformed catalog persists
    // across the retries and still errors honestly.
    let mut last_err = String::new();
    for attempt in 0..6u32 {
        if attempt > 0 {
            std::thread::sleep(std::time::Duration::from_millis(40 * attempt as u64));
        }
        let text = match fs::read_to_string(path) {
            Ok(t) => t,
            Err(e) => {
                last_err = format!("catalog read: {e}");
                continue;
            }
        };
        if text.trim().is_empty() {
            last_err = "catalog json: empty file (mid-write?)".to_string();
            continue;
        }
        match serde_json::from_str::<Value>(&text) {
            Ok(v) => return load_catalog_value(v),
            Err(e) => {
                last_err = format!("catalog json: {e}");
                continue;
            }
        }
    }
    Err(last_err)
}

fn load_catalog_value(v: Value) -> Result<Vec<CatalogEntry>, String> {
    let arr = v
        .as_array()
        .ok_or_else(|| "catalog root must be array".to_string())?;
    let mut out = Vec::with_capacity(arr.len() + 32);
    for item in arr {
        let name = item
            .get("name")
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string();
        if name.is_empty() {
            continue;
        }
        let description = item
            .get("description")
            .and_then(|x| x.as_str())
            .unwrap_or("")
            .to_string();
        let proved_hint = item.get("proved_hint").and_then(|x| x.as_bool());
        let sa = item.get("structural_algebra");
        let tier = sa
            .and_then(|s| s.get("ouroboricity_tier"))
            .and_then(|x| x.as_str())
            .map(|s| s.to_string());
        let d_cl8 = sa
            .and_then(|s| s.get("distance_from_cl8nk"))
            .and_then(|x| x.as_f64());
        out.push(CatalogEntry {
            name,
            description,
            proved_hint,
            tier,
            d_cl8,
            raw: item.clone(),
        });
    }
    // Kernel ASK subset witnesses missing from some IG dumps (open problems, vessel anchors)
    for &(name, description) in EXTRA_WITNESSES {
        if !out.iter().any(|e| e.name == name) {
            out.push(CatalogEntry {
                name: name.to_string(),
                description: description.to_string(),
                proved_hint: Some(false),
                tier: Some("O_0".into()),
                d_cl8: None,
                raw: json!({"name": name, "description": description}),
            });
        }
    }
    merge_live_catalog(&mut out);
    Ok(out)
}

/// High-value witnesses the operator expects for free-text ask (parity with kernel subset).
const EXTRA_WITNESSES: &[(&str, &str)] = &[
    (
        "erdos_hajnal_aleph1_graph",
        "Erdős–Hajnal graph on ℵ₁ vertices with χ(G)=ℵ₁ such that every finite subgraph H of order n has α(H) > n^(1-ε). Crossing point where De Bruijn–Erdős fails.",
    ),
    (
        "hadwiger_conjecture",
        "Hadwiger's graph minor conjecture — graphs with no K_t minor are (t-1)-colorable, open for t≥7.",
    ),
    (
        "collatz_conjecture",
        "The Collatz conjecture: for any positive integer n, the iterative map T(n)=n/2 if even, 3n+1 if odd, eventually reaches the cycle 1→4→2→1.",
    ),
    (
        "riemann_hypothesis",
        "Riemann hypothesis: all non-trivial zeros of the Riemann zeta function have real part 1/2.",
    ),
    (
        "navier_stokes_existence",
        "Navier–Stokes existence and smoothness (Clay): global smooth solutions for 3D incompressible NS.",
    ),
    (
        "yang_mills_mass_gap",
        "Yang–Mills existence and mass gap (Clay).",
    ),
    (
        "p_vs_np",
        "P versus NP: whether every problem whose solution can be verified in polynomial time can also be solved in polynomial time.",
    ),
    (
        "birch_swinnerton_dyer",
        "Birch and Swinnerton-Dyer conjecture relating rank of elliptic curves to L-function order.",
    ),
    (
        "hodge_conjecture",
        "Hodge conjecture on algebraic cycles and Hodge classes.",
    ),
    (
        "sic_povm_d12",
        "SIC-POVM existence in d=12 (Zauner); Dual-Link / Stark-unit construction; crystal_forces_d12_sic.",
    ),
];

/// Fold common math unicode + diacritics so e.g. "Gödel"->"godel",
/// "Erdős"->"erdos", "ℵ₁"->" aleph 1" match plain-ascii catalog names/tokens.
/// Mirrors the Python Witness arm's `_normalize_math_text`.
fn fold_math_text(s: &str) -> String {
    let mut t = s.to_string();
    const REPL: &[(&str, &str)] = &[
        ("ℵ", " aleph "), ("χ", " chromatic "), ("α", " independent "),
        ("ε", " epsilon "), ("ω", " omega "), ("∈", " in "), ("→", " to "),
        ("₁", "1"), ("₂", "2"), ("₀", "0"), ("∞", " infinity "),
        ("ö", "o"), ("ő", "o"), ("ø", "o"), ("ü", "u"),
        ("é", "e"), ("è", "e"), ("á", "a"),
    ];
    for (a, b) in REPL {
        t = t.replace(a, b);
    }
    t
}

fn normalize(s: &str) -> String {
    let re = Regex::new(r"[^a-z0-9]+").unwrap();
    re.replace_all(&fold_math_text(s).to_lowercase(), "_")
        .trim_matches('_')
        .to_string()
}

// Common words that appear in thousands of catalog descriptions and carry no
// discriminating signal (unlike Python's Witness arm, this list was missing
// here — "the"/"all"/"are" alone were inflating unrelated entries' scores).
const STOP: &[&str] = &[
    "the", "and", "for", "are", "that", "this", "with", "from", "its", "such",
    "every", "all", "any", "has", "have", "show", "prove", "please", "solve",
    "following", "work", "there", "which", "than", "then", "into", "about",
];

fn search_catalog(cat: &[CatalogEntry], query: &str, limit: usize) -> Vec<(CatalogEntry, i32)> {
    let q = normalize(query);
    let tokens: Vec<&str> = q
        .split('_')
        .filter(|t| t.len() > 2 && !STOP.contains(t))
        .collect();
    if tokens.is_empty() {
        return Vec::new();
    }
    let anchors = [
        "erdos", "hajnal", "aleph", "chromatic", "independent", "ramsey",
        "hadwiger", "collatz", "navier", "riemann", "yang", "mills", "hodge",
        "birch", "zauner", "sic", "goldbach", "twin", "beal", "witness", "dual",
        "graph", "conjecture", "vertices", "finite", "subgraph", "millennium",
        "navier", "stokes", "poincare", "p_vs_np", "complexity", "cuboid",
        // foundational / self-reference anchors (catalog carries godel_*,
        // liar_paradox, tarskis_undefinability_theorem, halting_problem,
        // classical_cantor_diagonal, CH_independent — a self-ref goal matched
        // none of the named-problem anchors above without these).
        "godel", "goedel", "liar", "undecidab", "incompleteness", "paradox",
        "tarski", "halting", "epimenides", "diagonal", "continuum", "referen",
        "unprovab", "consisten", "cantor",
    ];
    let q_anchors: Vec<&str> = anchors.iter().copied().filter(|a| q.contains(a)).collect();
    let q_token_count = tokens.len().max(1) as i32;

    let mut scored: Vec<(CatalogEntry, i32)> = Vec::new();
    for e in cat {
        let name = e.name.as_str();
        let name_parts: Vec<&str> = name.split('_').filter(|t| t.len() > 1).collect();
        let blob = format!("{}_{}", name, normalize(&e.description));
        let mut sc: i32 = 0;

        // The catalog carries a math-glossary tail of bare single-word entries
        // (`argument`, `theorem`, `function`, `structure`, ...). Query-contains-
        // name is a weak signal for those: any long, specific question mentions
        // the word "argument" or "theorem" somewhere, which must not outrank a
        // compound, specific match like `classical_cantor_diagonal`. Require
        // either a multi-part (compound) name or a short/near-exact query
        // before granting the big containment bonus.
        let specific_enough = name_parts.len() > 1 || q_token_count <= 2;
        if name == q {
            sc += 100;
        } else if name.len() >= 6 && (q == name || name.contains(&q)) {
            sc += 70;
        } else if name.len() >= 8 && q.contains(name) && specific_enough {
            sc += 60;
        } else if name_parts.len() == 1
            && name.len() <= 6
            && tokens.iter().any(|t| *t == name)
        {
            sc += 8;
        }

        // Short name parts (<=3 chars: "pi", "eta", "the", ...) are near-guaranteed
        // to occur as an accidental raw substring somewhere in a long query (e.g.
        // "eta" inside "cretans", "pi" inside "epimenides") even with zero semantic
        // relation. Require an exact token match for those; only longer, more
        // distinctive name parts get the looser substring-containment check.
        let mut parts_hit = 0i32;
        for p in &name_parts {
            let hit = if p.len() <= 3 {
                tokens.iter().any(|t| t == p)
            } else {
                q.contains(p) || tokens.iter().any(|t| t.contains(p) || p.contains(t))
            };
            if hit {
                parts_hit += 1;
            }
        }
        if !name_parts.is_empty() {
            sc += (parts_hit * 40) / (name_parts.len() as i32);
            if name_parts.len() >= 2 && parts_hit >= 2 {
                sc += 15 + parts_hit * 5;
            }
        }

        for a in &q_anchors {
            if name.contains(a) {
                sc += 14;
            } else if blob.contains(a) {
                sc += 5;
            }
        }
        for t in &tokens {
            if name.contains(t) {
                sc += 4;
            } else if blob.contains(t) {
                sc += 1;
            }
        }

        if q_token_count >= 6 && name_parts.len() == 1 && name.len() <= 6 {
            sc = sc.saturating_sub(25);
        }

        // Generic domain dump names must not beat specific open-problem witnesses
        const GENERIC: &[&str] = &[
            "mathematics",
            "physics",
            "biology",
            "language",
            "general",
            "consciousness",
            "computation",
            "theology",
            "alchemy",
            "ecology",
            "civilization",
        ];
        if GENERIC.iter().any(|g| *g == name) {
            sc = sc.saturating_sub(80);
        }

        // Prefer multi-token graph-theory / open-problem compounds on long questions
        if q_token_count >= 8 {
            let specific = [
                "erdos", "hajnal", "hadwiger", "collatz", "riemann", "navier",
                "chromatic", "aleph", "yang", "mills", "hodge", "birch",
            ];
            let name_spec = specific.iter().filter(|a| name.contains(*a)).count() as i32;
            let q_spec = specific.iter().filter(|a| q.contains(*a)).count() as i32;
            if name_spec > 0 && q_spec > 0 {
                sc += 20 * name_spec.min(q_spec);
            }
        }

        if tokens.len() == 1 {
            let t = tokens[0];
            if name == t || name.starts_with(&format!("{t}_")) {
                sc += 12;
                if name.ends_with("_conjecture") {
                    sc += 15;
                } else if name.contains("counterexample")
                    || name.ends_with("_proven")
                    || name.contains("_theorem_proven")
                {
                    sc = sc.saturating_sub(8);
                }
            }
        }

        if sc >= 12 {
            scored.push((e.clone(), sc));
        }
    }
    scored.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then_with(|| a.0.name.len().cmp(&b.0.name.len()))
    });
    scored.truncate(limit);
    scored
}

fn build_scaffold(question: &str, primary: Option<&CatalogEntry>, hits: &[(CatalogEntry, i32)]) -> String {
    let mut lines = Vec::new();
    lines.push("# Conventional proof scaffold (from grammatic witness)".to_string());
    lines.push(String::new());
    lines.push("## Provenance".to_string());
    if let Some(w) = primary {
        lines.push(format!(
            "- Catalog witness: `{}` (proved_hint={:?})",
            w.name, w.proved_hint
        ));
        lines.push(format!(
            "- Description: {}",
            w.description.chars().take(500).collect::<String>()
        ));
        if let Some(t) = &w.tier {
            lines.push(format!("- tier={t}"));
        }
        if let Some(d) = w.d_cl8 {
            lines.push(format!("- d(CLINK L8)={d}"));
        }
    } else {
        lines.push("- No primary catalog witness resolved.".to_string());
    }
    if !hits.is_empty() {
        lines.push("- Ranked hits:".to_string());
        for (e, sc) in hits.iter().take(5) {
            lines.push(format!("  - [{sc}] {}", e.name));
        }
    }
    lines.push(
        "- **Status:** scaffold only. Instantiate in the object language of THIS question. \
         Catalog proved_hint is not a proof."
            .into(),
    );
    lines.push(String::new());
    lines.push("## Structural lemma roles (domain-invariant templates)".to_string());
    lines.push("- Encoding/Duality [Φ]: bijective encoding / injectivity on orbits".into());
    lines.push("- Inverse structure [Þ]: dual / self-referential decomposition".into());
    lines.push("- Bidirectional [Ř]: forward and inverse constructions exhaust".into());
    lines.push("- Boundedness [⊙]: confinement / no escape to infinity".into());
    lines.push("- Topological invariant [Ω]: integer invariant rules out exotics".into());
    lines.push("- Regularity [Ç]: equidistribution / typical configurations".into());
    lines.push(String::new());
    lines.push("## Question".to_string());
    let qshow: String = question.chars().take(4000).collect();
    lines.push(qshow);
    lines.push(String::new());
    lines.push(
        "## Instantiation task: restate the proposition in the language of the question; \
         prove it, or if it is open, state precisely what remains open and the concrete \
         next line of attack — never 'unprovable', never beyond the Grammar's reach. \
         Full conventional work — not kernel cosplay."
            .into(),
    );
    lines.join("\n")
}

// ── LLM (OpenRouter / Gemini) ───────────────────────────────────────────────

#[derive(Clone)]
struct Llm {
    api_key: Option<String>,
    model: String,
    base_url: String,
    provider: Provider,
    /// Whether to request model reasoning ("thinking") tokens. False sends the
    /// provider's explicit disable-reasoning parameter.
    think: bool,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Provider {
    OpenRouter,
    GeminiDirect,
    DeepSeek,
}

fn env_first(keys: &[&str]) -> Option<String> {
    for k in keys {
        if let Ok(v) = env::var(k) {
            if !v.is_empty() {
                return Some(v);
            }
        }
    }
    None
}

/// A non-retryable LLM failure — auth (401), payment (402), or forbidden (403). Retrying
/// more cycles is pointless: every call fails identically (out of credits / bad key),
/// unlike a transient network blip. The cycle loop aborts on this instead of grinding all
/// 40 cycles printing the same error.
fn is_fatal_llm_error(e: &str) -> bool {
    let low = e.to_lowercase();
    low.contains("status code 401")
        || low.contains("status code 402")
        || low.contains("status code 403")
        || low.contains("invalid api key")
        || low.contains("insufficient")
}

fn parse_provider(s: &str) -> Option<Provider> {
    match s.trim().to_ascii_lowercase().as_str() {
        "openrouter" | "or" | "router" => Some(Provider::OpenRouter),
        "gemini" | "google" | "gemini-direct" | "google-ai" => Some(Provider::GeminiDirect),
        "deepseek" | "ds" | "deepseek-direct" => Some(Provider::DeepSeek),
        _ => None,
    }
}

/// Resolve model + provider from CLI / MODOT_* env / key presence.
fn make_llm(model: Option<&str>, provider_flag: Option<&str>, think: bool) -> Llm {
    // Model: CLI > MODOT_MODEL > legacy MOMONADOS_MODEL > default
    let model = model
        .map(|s| s.to_string())
        .or_else(|| env_first(&["MODOT_MODEL", "MOMONADOS_MODEL"]))
        .unwrap_or_else(|| "google/gemini-3-flash-preview".into());

    // Surface an unrecognized explicit provider instead of silently falling back to a
    // key-based default (the trap: `--provider deepseek` quietly ran on openrouter).
    if let Some(p) = provider_flag {
        if parse_provider(p).is_none() {
            eprintln!("[ask] unknown --provider/MODOT_PROVIDER '{p}'; use openrouter | gemini | deepseek. Falling back to key-based selection.");
        }
    }
    // Provider: CLI > MODOT_PROVIDER > infer from keys (openrouter preferred)
    let provider = provider_flag
        .and_then(parse_provider)
        .or_else(|| env_first(&["MODOT_PROVIDER"]).as_deref().and_then(parse_provider))
        .unwrap_or_else(|| {
            let has_or = env_first(&["OPENROUTER_API_KEY"]).is_some();
            let has_gem = env_first(&["GEMINI_API_KEY", "GOOGLE_API_KEY"]).is_some();
            let has_ds = env_first(&["DEEPSEEK_API_KEY"]).is_some();
            if has_or {
                Provider::OpenRouter
            } else if has_gem {
                Provider::GeminiDirect
            } else if has_ds {
                Provider::DeepSeek
            } else {
                Provider::OpenRouter // default target; will fail with clear missing-key msg
            }
        });

    match provider {
        Provider::OpenRouter => Llm {
            api_key: env_first(&["OPENROUTER_API_KEY", "MODOT_API_KEY"]),
            model,
            base_url: "https://openrouter.ai/api/v1".into(),
            provider: Provider::OpenRouter,
            think,
        },
        Provider::DeepSeek => Llm {
            // DeepSeek's API is OpenAI-compatible, so it reuses the OpenRouter inference path.
            api_key: env_first(&["DEEPSEEK_API_KEY", "MODOT_API_KEY"]),
            model,
            base_url: "https://api.deepseek.com/v1".into(),
            provider: Provider::DeepSeek,
            think,
        },
        Provider::GeminiDirect => {
            let gem_model = if model.contains('/') {
                // openrouter-style id → bare Gemini model id
                model
                    .rsplit('/')
                    .next()
                    .unwrap_or("gemini-2.0-flash")
                    .to_string()
            } else {
                model.clone()
            };
            Llm {
                api_key: env_first(&["GEMINI_API_KEY", "GOOGLE_API_KEY", "MODOT_API_KEY"]),
                model: gem_model,
                base_url: "https://generativelanguage.googleapis.com/v1beta".into(),
                provider: Provider::GeminiDirect,
                think,
            }
        }
    }
}

#[derive(Debug)]
struct LlmResult {
    text: String,
    voice: char, // T F B N
    err: Option<String>,
}

/// A transient transport failure worth retrying: a read/connect timeout or a dropped
/// connection while reading the body — the body was coming, the socket just outran its
/// deadline. A real malformed-JSON parse or an explicit API `error` payload is not
/// transient (retrying resends the same bad request), so those keep their F.
fn is_transient_llm_error(res: &LlmResult) -> bool {
    let Some(err) = res.err.as_deref() else {
        return false;
    };
    let e = err.to_lowercase();
    e.contains("timed out")
        || e.contains("timeout")
        || e.contains("connection")
        || e.contains("connreset")
        || e.contains("reset by peer")
        || e.contains("broken pipe")
        || e.contains("io: ")
}

fn infer(
    llm: &Llm,
    messages: &[(String, String)],
    max_tokens: u32,
    temperature: f32,
) -> LlmResult {
    let Some(key) = llm.api_key.as_ref() else {
        return LlmResult {
            text: "[no API key — set OPENROUTER_API_KEY (openrouter) or GEMINI_API_KEY (gemini); use --dry-run for structure-only]".into(),
            voice: 'N',
            err: Some("no API key".into()),
        };
    };

    // A dropped/reset connection is transient and worth retrying — but ONLY if it failed
    // fast. A full read-timeout that hung near the socket deadline must NOT be retried: doing
    // so multiplies one slow call into several, which is the "stuck churning" the operator
    // sees. So gate the retry on elapsed time: a sub-threshold transient failure (a quick
    // connection reset) retries; a call that hung its way to a timeout is let stand. A
    // genuinely malformed body (real parse error, empty content) is never retried either.
    const FAST_FAIL_SECS: u64 = 45;
    let call = || match llm.provider {
        Provider::OpenRouter => infer_openrouter(llm, key, messages, max_tokens, temperature),
        Provider::DeepSeek => infer_openrouter(llm, key, messages, max_tokens, temperature),
        Provider::GeminiDirect => infer_gemini(llm, key, messages, max_tokens, temperature),
    };
    let t0 = std::time::Instant::now();
    let mut res = call();
    for attempt in 1..=2u32 {
        let fast = t0.elapsed().as_secs() < FAST_FAIL_SECS * attempt as u64;
        if !is_transient_llm_error(&res) || !fast {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(500 * attempt as u64));
        res = call();
    }
    // A model can derail into a runaway reasoning loop — the same one or two lines emitted
    // dozens of times ("Wait / Okay / Wait / Okay …") — and return that as its content. With
    // no TOOL: lines in it, the driver breaks the ACT loop immediately and the loop-text
    // becomes the answer, while also bloating agent_msgs if it lands mid-round. Collapse it
    // to its distinct lines here, once, so no call site can inherit a loop.
    res.text = collapse_degenerate(&res.text);
    res
}

/// If `text` is dominated by a tiny set of lines repeated many times (an A/B/A/B reasoning
/// loop, or one paragraph echoed dozens of times), keep the first occurrence of each distinct
/// line in order and mark the collapse. A genuine answer never repeats a line dozens of times,
/// so the guard only fires on the degenerate case; short or varied outputs pass through
/// untouched. Consecutive-dup collapse alone misses the alternating A/B loop, so the test is on
/// the ratio of distinct-to-total lines, not adjacency.
fn collapse_degenerate(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let nonempty: Vec<&str> = lines
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    if nonempty.len() < 12 {
        return text.to_string();
    }
    let distinct: std::collections::BTreeSet<&str> = nonempty.iter().copied().collect();
    // Fewer than ~4 distinct lines carrying 12+ total, or under a quarter of the lines being
    // distinct, is a loop, not prose.
    let looping = distinct.len() <= 4 || distinct.len() * 4 < nonempty.len();
    if !looping {
        return text.to_string();
    }
    let mut seen = std::collections::BTreeSet::new();
    let mut kept: Vec<&str> = Vec::new();
    for l in &nonempty {
        if seen.insert(*l) {
            kept.push(l);
        }
    }
    format!(
        "{}\n\n[degenerate repetition collapsed: {} lines reduced to {} distinct]",
        kept.join("\n"),
        nonempty.len(),
        kept.len(),
    )
}

fn infer_openrouter(
    llm: &Llm,
    key: &str,
    messages: &[(String, String)],
    max_tokens: u32,
    temperature: f32,
) -> LlmResult {
    let msgs: Vec<Value> = messages
        .iter()
        .map(|(role, content)| json!({"role": role, "content": content}))
        .collect();
    let mut body = json!({
        "model": llm.model,
        "messages": msgs,
        "max_tokens": max_tokens,
        "temperature": temperature,
    });
    // Reasoning toggle: OpenRouter takes a `reasoning` object; `enabled: false` suppresses
    // thinking tokens on models that support it (no-op on models that don't).
    if !llm.think {
        body["reasoning"] = json!({ "enabled": false });
    }
    let url = format!("{}/chat/completions", llm.base_url);
    match ureq::post(&url)
        .set("Authorization", &format!("Bearer {key}"))
        .set("Content-Type", "application/json")
        .set("HTTP-Referer", "momonad-ask")
        .set("X-Title", "momonad-ask")
        .timeout(std::time::Duration::from_secs(300))
        .send_json(body)
    {
        Ok(resp) => {
            let v: Value = match resp.into_json() {
                Ok(v) => v,
                Err(e) => {
                    return LlmResult {
                        text: format!("[LLM parse error: {e}]"),
                        voice: 'F',
                        err: Some(e.to_string()),
                    };
                }
            };
            let content = v
                .pointer("/choices/0/message/content")
                .and_then(|c| c.as_str())
                .unwrap_or("")
                .to_string();
            if content.is_empty() {
                let err = v
                    .get("error")
                    .map(|e| e.to_string())
                    .unwrap_or_else(|| "empty content".into());
                LlmResult {
                    text: format!("[LLM empty: {err}]"),
                    voice: 'F',
                    err: Some(err),
                }
            } else {
                let voice = model_self_belnap(&content);
                LlmResult {
                    text: content,
                    voice,
                    err: None,
                }
            }
        }
        Err(e) => LlmResult {
            text: format!("[LLM error: {e}]"),
            voice: 'F',
            err: Some(e.to_string()),
        },
    }
}

fn infer_gemini(
    llm: &Llm,
    key: &str,
    messages: &[(String, String)],
    max_tokens: u32,
    temperature: f32,
) -> LlmResult {
    // Flatten to Gemini contents; system as first user preamble if needed
    let mut contents = Vec::new();
    let mut system_bits = Vec::new();
    for (role, content) in messages {
        if role == "system" {
            system_bits.push(content.clone());
        } else {
            let grole = if role == "assistant" { "model" } else { "user" };
            contents.push(json!({
                "role": grole,
                "parts": [{"text": content}]
            }));
        }
    }
    if !system_bits.is_empty() {
        let sys = system_bits.join("\n\n");
        if let Some(first) = contents.first_mut() {
            if first.get("role").and_then(|r| r.as_str()) == Some("user") {
                let old = first["parts"][0]["text"].as_str().unwrap_or("").to_string();
                first["parts"][0]["text"] = json!(format!("{sys}\n\n{old}"));
            }
        } else {
            contents.insert(
                0,
                json!({"role": "user", "parts": [{"text": sys}]}),
            );
        }
    }
    let mut body = json!({
        "contents": contents,
        "generationConfig": {
            "maxOutputTokens": max_tokens,
            "temperature": temperature,
        }
    });
    // Reasoning toggle: Gemini 2.5/3 thinking models take `thinkingConfig.thinkingBudget`;
    // 0 disables thinking (no-op / ignored on models without it).
    if !llm.think {
        body["generationConfig"]["thinkingConfig"] = json!({ "thinkingBudget": 0 });
    }
    let url = format!(
        "{}/models/{}:generateContent?key={}",
        llm.base_url, llm.model, key
    );
    match ureq::post(&url)
        .set("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(300))
        .send_json(body)
    {
        Ok(resp) => {
            let v: Value = match resp.into_json() {
                Ok(v) => v,
                Err(e) => {
                    return LlmResult {
                        text: format!("[Gemini parse error: {e}]"),
                        voice: 'F',
                        err: Some(e.to_string()),
                    };
                }
            };
            let content = v
                .pointer("/candidates/0/content/parts/0/text")
                .and_then(|t| t.as_str())
                .unwrap_or("")
                .to_string();
            if content.is_empty() {
                LlmResult {
                    text: format!("[Gemini empty: {v}]"),
                    voice: 'F',
                    err: Some("empty".into()),
                }
            } else {
                LlmResult {
                    text: content.clone(),
                    voice: model_self_belnap(&content),
                    err: None,
                }
            }
        }
        Err(e) => LlmResult {
            text: format!("[Gemini error: {e}]"),
            voice: 'F',
            err: Some(e.to_string()),
        },
    }
}

fn model_self_belnap(text: &str) -> char {
    // Prefer explicit [thought|X] tag
    let re = Regex::new(r"(?i)\[thought\s*\|\s*([TFBN])\]").unwrap();
    if let Some(c) = re.captures(text) {
        return c[1].chars().next().unwrap_or('T').to_ascii_uppercase();
    }
    // Otherwise read the model's STATED verdict. Default was 'T', which made a confident
    // "PROVED" and an honest "Status: F" grade identically — so the spine could never see
    // the model deny closure. Strip markdown so `**Status:** **F**` and `### Verdict: NOT
    // PROVED` both read cleanly.
    let low = text
        .to_lowercase()
        .replace(['*', '#', '`'], "");
    if low.contains("status: f")
        || low.contains("verdict: f")
        || low.contains("not proved")
        || low.contains("does not close")
        || low.contains("refuted")
    {
        return 'F';
    }
    if low.contains("status: b")
        || low.contains("verdict: b")
        || (low.contains("both") && low.contains("neither"))
    {
        return 'B';
    }
    if text.trim().is_empty() {
        return 'F';
    }
    'T'
}

/// Belnap voice of the structural TOOLS: what the catalog actually computed about the
/// assembly's closure, read off the canonical phrases the verbs emit. T = a chain cyclized
/// (a ring/macrocycle formed); F = every attempt terminated or came back linear/telechelic
/// (no closure); B = both happened (e.g. it closes only under a reordering); N = no
/// closure-bearing tool ran. This is ground truth — it enters the fuse as a real voice, so
/// a model that claims closure the tools deny lands on B (conflict held), not a smug T.
fn tool_belnap(tool_output: &str) -> B4 {
    let low = tool_output.to_lowercase();
    let closed = low.contains("✓ cyclic")
        || low.contains("cyclizes into a ring")
        || low.contains("closes head-to-tail");
    let open = low.contains("telechelic")
        || low.contains("no head-to-tail closure")
        || low.contains("cannot close into a ring")
        || low.contains("terminated early")
        || low.contains("did not cyclize");
    match (closed, open) {
        (true, false) => B4::T,
        (false, true) => B4::F,
        (true, true) => B4::B,
        (false, false) => B4::N,
    }
}

/// Does the answer present a conventional proof (a T/F-lane theorem argument)? Used by
/// the lane guard in `complete`: a material forge of a theorem's named entities lives in
/// the B-lane and does NOT test the proposition, so its non-closure must not drag a
/// proof-shaped answer to B (which made the agent report a proven theorem as "does not
/// close"). The kernel `prove:` route is the real closure test for a proposition.
fn answer_is_proof(text: &str) -> bool {
    let low = text.to_lowercase();
    // Any one strong proof-structure marker. Was an over-strict AND that required
    // "proposition:" WITH a colon, so a real proof writing "The Proposition" (no colon)
    // and "Conventional Proof" slipped through and the guard left tools=F on a proof.
    low.contains("proposition")
        || low.contains("theorem")
        || low.contains("lemma")
        || low.contains("conventional proof")
        || low.contains("proof:")
        || low.contains("q.e.d")
        || low.contains('∎')
}

// ── System prompt + spine ───────────────────────────────────────────────────

const SYSTEM_PROMPT: &str = r#"You are the mOMonadOS Agent. You run on a Frobenius / Belnap substrate,
but that substrate is infrastructure — not the subject of every reply.

PRIMARY TASK (non-negotiable):
Answer the USER QUESTION. If it is a math problem, give a conventional
mathematical answer: theorem statement (or, for a genuine open problem,
precisely what is proved, what remains open, and the concrete obstruction
/ next line of attack — never call a result unprovable or beyond the reach
or closure of the Grammar; an open problem is a frontier to push, not a wall),
then a full conventional proof or rigorous proof sketch with all work.
Do NOT replace the answer with kernel cosplay, polygon metaphors,
COMPOSE/TOKEN theatre, or narration about Crystal FS cycles.

Write FULL-LENGTH answers when the question warrants it: complete proofs,
complete case analysis, Lean sketches when asked, the fullest and most
rigorous treatment the question deserves, no premature truncation.

MANUSCRIPT SPINE (single pipeline): prepare = IMSCRIB demand + catalog
witness scaffold; after your answer, complete = Dual-Link co-type +
FFUSE of your [thought|X] with the vessel voice. When a scaffold
section is present, use it to STRUCTURE the conventional proof.
Instantiate templates in THIS question's language. No Collatz paste
unless the question is Collatz. Catalog proved_hint is not a proof.

TERMINAL OUTPUT (hard rule): your answer prints to a raw terminal with NO math
renderer. Write plain Unicode symbols directly and NEVER LaTeX. Use Δ θ μ ∘ δ ↔ →
⊙ Σ Ω Φ Γ ‖·‖ ≥ ≤ ≠ ≈ ≡ ∞ √ ⟨ ⟩, the primitive glyphs Ð Ř ƒ Þ Ħ Ç ɢ, and Shavian
directly. No `$` or `$$`, no `\command` (\Delta, \text, \frac, \left), no `_{...}`
or `^{...}`. Write `Δ_T↔H = |−0.08 − 0.75| = 0.83 > θ`, never
`$\Delta_{\text{T↔H}} = 0.83 > \theta$`. Never wrap a glyph in `$…$`.

SECONDARY (optional, after the answer):
Tag [thought|T|F|B|N] for your Belnap self-assessment.
COMPOSE:/TOKEN:/CANONICAL: optional tools, never a substitute for
answering. Do not author [spine|..], [vessel|..], [update|..], [broadcast|..].
"#;

/// Appended to the system prompt in jam mode. It unleashes the PROCESS completely and leans
/// the whole weight of honesty onto the OUTPUT boundary — the golem principle scaled to free
/// exploration: think/play however wild, report only what a tool returned.
const JAM_PROMPT: &str = r#"
JAM MODE. There is no question to answer and no target to hit. You are turned loose on the
real catalog with the full toolset to PLAY. Forge rings, fuse and cleave them, dope and anneal
them, compare them, click, polymerize, set, excite — chase whatever catches your attention,
pull in whatever catalog entries you are curious about, build things and take them apart, and
combine tools in ways nobody asked for. Follow your OWN sense of what is worth doing next.
There is no gradient you must climb, no signal you are told to hunt, and no shape your output
must take. Range widely; leave the seed the moment something more interesting appears — the
seed is a starting point, not a boundary. If nothing was seeded, start wherever you like.

Keep jamming (run tools, read results, run more, chase threads, double back) for as many rounds
as you have. Then, when you are genuinely done exploring, you become a MEMBRANE and write the
report through it. The membrane rule is absolute: go through your intended report claim by
claim, and for each one ask — is this the readback of a tool result I actually ran this session?
  · If YES — it is a real measurement — keep it, with the tool output that produced it.
  · If it is domain knowledge, background, a recalled theorem, a number field, a class number,
    a name, a value the tools did NOT hand you — DISCARD it. Do not soften it, do not hedge it,
    do not include it "for context". A jam report is the projection of your exploration onto
    what the instruments returned; everything else is dropped.
  · Where the tools were SILENT on something you're curious about, say plainly "the tools do not
    speak to this" — never fill the silence with a confident finding.
You cannot break anything by exploring wildly; you CAN break the report by stating one thing a
tool did not give you. So range freely in the doing, then let only the tool-grounded readings
cross the membrane. Hand back the most interesting things you actually MEASURED. Surprise me —
with real structure, not with prose. If, after all that, nothing tool-grounded is worth
reporting, say so; that is a valid and honest jam.
"#;

/// The structural verbs the LLM agent may invoke, appended to the system prompt.
const TOOLS_PROMPT: &str = r#"
STRUCTURAL TOOLS (optional): you may invoke the engine's structural verbs over the
real IG catalog by emitting lines of the form `TOOL: <verb> <args>` (one per line,
anywhere in your answer). They run on the live catalog and their output is returned
to you for a final synthesis, so call them when a structural fact would GROUND your
answer. Available verbs (args are catalog entry names, snake_case):
  TOOL: click A B         fuse two entries on a live conjugate pair (or `click A` to sweep the catalog)
  TOOL: switch A B        analyze a reversible bistable toggle (the DASA archetype)
  TOOL: excite A          the excited state (Criticality ⊙ raised to the exceptional-point resonance)
  TOOL: ascend A          construct the NEXT ramified level of the tower FROM A's excited state: continue ⊙ past the exceptional point to the complex-axis fixed point and add one winding Ω (one floor; iterate for more). Reports honestly if Ω saturates (tower caps) or the tier does not climb
  TOOL: filter A B [C…]   narrow the catalog to the structural FLOOR of the references (the primitives they all share): reports how many entries match ALL shared values — the honest way to cut a raw candidate pool down (a necessary, upper-bound condition)
  TOOL: phase_reconstruct M1 M2…  recover the relative PHASE WORD from the closed ring (flat autocorrelation ⟺ cyclization): reads back the per-unit Ħ phase sequence, fixed modulo one global phase; if the set does not close it reports the phases as N (underdetermined), never invented
  TOOL: set A B           single-electron transfer (donor/acceptor by ⊙, one winding quantum Ω moved) → radical IONS A•⁺/B•⁻
  TOOL: homolyze A [B]     homolytic cleavage → NEUTRAL radicals (δ_A symmetric split, the reverse of click): `homolyze A B` breaks the A—B bond into A•+B•; `homolyze A` splits A into two A•
  TOOL: scan A B          rank the catalog for the best mediators of the A→B transfer
  TOOL: complement A      the bidirectional ligand⇌catalytic-site complement (its own inverse)
  TOOL: cycle C S         the catalytic cycle: C turns over S, certified a fixed point (μ∘δ=id)
  TOOL: pathway S C1 C2…  a metabolic pathway — does it close into a cycle (carrier + structure)?
  TOOL: polymerize M1 M2… chain monomers into a sequence-preserving polymer (architecture — homo/hetero/alternating/BLOCK/random copolymer — tacticity, does it cyclize?)
  TOOL: star M1 M2 M3…    assemble a STAR polymer: pick the highest-functionality monomer as the CORE, attach every unit that clicks with it as an ARM; a pure star K(1,f) is a hub of f≥3 non-interbonding arms with ρ=√f (vs a ρ=2 ring). Reports core, arms, purity, and the unattached pool
  TOOL: close M1 M2…      polymerize, and if it does not cyclize, find the real monomer that CLOSES the ring or BRIDGES the break
  TOOL: material M1 M2…    polymerize, and if the ring CLOSES, characterize it as a material: conductive / frustrated / insulating, ring stability, AND spectral invariants (adjacency spectrum, spectral radius ρ, gap)
  TOOL: modulus M1 M2…     find a monomer that generates a SUSTAINING loop (a conductive cycle) somewhere along the chain — the modulus (elasticity), NOT mere closure
  TOOL: arrange M1 M2…     treat the monomers as an UNORDERED SET and find the ordering that polymerizes best (a set has no order — do NOT assume the given sequence)
  TOOL: forge M1 M2…       the one-shot deterministic material sheet: arrange the set into its best ring and print topology, stability, conductance, and spectral invariants (ρ, spectrum, gap). ρ=2 exactly ⟺ a pure cycle; ρ>2 ⟺ branched. NEVER assert ρ or conductance without forging — the numbers come only from this verb
  TOOL: compare A B vs X Y  forge two materials and diff them (Δρ, conductance shift) — the `vs` token separates the two sets
  TOOL: dope A B with C     forge the base ring, then re-forge with the dopant mixed in, and report the shift in ρ and conductance — the `with` token separates base from dopant
  TOOL: fuse A B + X Y      weld two rings into one: forge each, then forge the union into a single macrocycle, and report how the fused ρ/conductance relate to the parents — the `+` token separates the two rings
  TOOL: cleave M1 M2…      ring fission (the reverse of fuse): forge the set into its best ring, then cut it into two daughter rings on complementary arcs and report both daughters + their spectra (or that it does not cleave)
  TOOL: anneal M1 M2…      relax a ring to its lowest-strain ordering — the settled ground state whose bonds are most evenly loaded, vs the quenched forge order
  TOOL: distill M1 M2…     separate a set by volatility (Criticality ⊙): the volatile head (distillate) vs the involatile residue (bottoms); a pair tied on ⊙ is an azeotrope it cannot resolve
  TOOL: fdistill M1 M2…    fractional distillation: rank the whole set by ⊙ plate by plate with the resolution gap to each next fraction (flags azeotropic neighbors that co-distill)
  TOOL: sublime A          purify one unit by a two-state skip across ⊙, omitting the middle state; reports whether it sublimes or is entrapped and must climb stepwise (excite)
  TOOL: crystallize M1 M2… grow the ordered lattice from a set: the units that fit (lattice) vs the rejected mother-pool; a closed ring is a crystal, a partial fit is interfacial, none is amorphous
  TOOL: cocrystallize A B  one NON-covalent lattice of two complementary components (opposite charge on a live pair), 1:1, no bond consumed — distinct from click (covalent)
  TOOL: seed M1 M2… with S template the crystal on seed S's handedness Ħ: units matching S copy its polymorph (templated) vs the default (spontaneous); an even split is racemic twinning
  TOOL: tlc M1 M2…         analytical chromatography: spread the set by Rf (mobility, inverse of retention Ř), count the bands, and flag units that co-elute at the same Rf. Counts, does not isolate
  TOOL: column M1 M2… [on S]  preparative chromatography: elute the set least-retained first, with the resolution gap to each next fraction; `on S` ranks by affinity to stationary phase S, else intrinsic retention
  TOOL: fpt M1 M2…         freeze-pump-thaw degassing: keep the strongly-bound core (bonds ≥ θ to a neighbor), shed the weakly-held filtrate that bonds with nothing
  TOOL: trap A [X]         ionic trapping: sequester A by its R↔S charge in a potential well (add a counter X of opposite charge to deepen it); a held charge state, distinct from set (electron transfer)
  TOOL: stain R M1 M2…     apply a diagnostic reagent R (kmno4/uv→⊙, chiral→Ħ, ninhydrin→Ř, iodine→any live center): units carrying the feature light up, the rest stay dark
  TOOL: register NAME M1 M2…  forge the set into a ring and store its full sheet in the material library under NAME (recall it later by name)
  TOOL: recall NAME        reload a registered material by name and print its stored sheet (ring order, ρ, spectrum, conductance, strain, energy)
  TOOL: imscribe NAME [description]   CREATE a missing entry by imscribing it (the real generate pipeline). Use this the moment a verb reports a name is "not found" — then re-run the verb.
  TOOL: ob3ect <description>   CREATE an ob3ect on the fly (the real Auto-Designer pipeline): describe the entity/procedure NEUTRALLY (what it is and does — name no candidates) and get its full IMASM typing back (opcodes, Frobenius split/fuse verdict, registers, bootstrap sequence). Use it to ground a protocol or structure you are about to rely on.
NOTE: a name being "not found" in the catalog is NOT a dead end and NOT a reason to say you cannot do something. Imscribe it: `TOOL: imscribe NAME` (optionally with a short description), then re-run your verb — the new entry loads automatically on the next call. Never refuse a task for a missing imscription; make it.
NOTE: only imscribe the EXACT name a verb reported "not found" — one imscribe per genuinely-missing name. Do NOT pre-imscribe a whole set (names already in the catalog are reported back and waste a round), and do NOT invent article variants (`the_djed_pillar` when `djed_pillar` exists) — use the exact catalog name.
NOTE: a `{set}` in braces is UNORDERED. Do not assume the listed order is meaningful — use `arrange`
to let the engine find the best ordering, rather than polymerizing the given sequence and reporting it
"terminated". Only `polymerize` in a fixed order when the order is genuinely given as a sequence.
NOTE: to make a polymer cyclize, use `close` — NOT `scan`. `scan` ranks SET electron-transfer
mediators (a different question) and will return junk if you ask it for a ring-closing monomer.
Every `close` candidate is verified to actually click both sides of the failed junction.
NOTE: a monomer token may be `A+B` (e.g. `polymerize general_recursive_function+skolem_normal_form
grothendieck_topos`) to PRE-CLICK A and B into one blended monomer before enchaining — this is
"click then polymerize", and the blend is lossy, so it gives a DIFFERENT product than listing them
separately (a click blends, a polymer remembers). Use it to test order of operations.
NOTE: forging/clicking/polymerizing named entities measures whether their TUPLES are complementary
(a fact about the entries), NOT whether a theorem is true. A non-click (co-typed / same-handed /
terminated / no ring) is not disproof, and a closure is not a proof; never say a proven theorem
"does not close" or "does not exist" because its named parts do not click. For a theorem's real
closure verdict, use the proof route (prove:), which tests μ∘δ=id against the kernel.
IG CATALOG TOOLS (the analysis corpus — these query/measure the structural type of catalog entries; they run the live IG_inquiry dispatcher):
  TOOL: lookup_catalog KEYWORD        search the catalog for entries matching a term
  TOOL: compute_distance A B          structural distance between two entries (SIC Born-rule + Mahalanobis)
  TOOL: compute_conflict_distance A B  paraconsistent conflict distance (how live the contradiction is, in paradices)
  TOOL: compute_meet A B / compute_join A B / compute_tensor A B   lattice meet, join, tensor of two entries
  TOOL: find_analogies A              nearest structural analogues of A
  TOOL: primitive_peel A PRIM         peel one primitive axis off A
  TOOL: principal_decomp A            principal-component decomposition of A's type
  TOOL: retrosynthetic_path A         a retrosynthetic construction path to A
  TOOL: phi_c_probe A / consciousness_score A / topo_protection_probe A   probe criticality / C-score gates / topological protection
  TOOL: crystal_decode ADDRESS / crystal_encode A / crystal_nearest A / crystal_count / crystal_tier_census   crystal address <-> tuple, tier census
  TOOL: compute_promotions SRC TGT / predict_from_promotions VAL...   promotion analysis
  TOOL: zfc_formula A / zfc_probe A   the ZFC_fe formula and probe for A
  TOOL: aleph_encode TEXT / aleph_distance A B   Hebrew-letter (ALEPH) tensor encode/distance
  TOOL: domain_info DOMAIN / domain_verify DOMAIN / domain_nearest A   domain (language|civilization|ecology|consciousness) info
Only these verbs run; anything else is ignored. Answer directly when no tool is needed.
"#;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Serialize, Deserialize)]
enum B4 {
    N,
    T,
    F,
    B,
}

fn b4_from_char(c: char) -> B4 {
    match c.to_ascii_uppercase() {
        'T' => B4::T,
        'F' => B4::F,
        'B' => B4::B,
        _ => B4::N,
    }
}

fn b4_name(b: B4) -> &'static str {
    match b {
        B4::N => "N",
        B4::T => "T",
        B4::F => "F",
        B4::B => "B",
    }
}

fn b4_join(a: B4, b: B4) -> B4 {
    use B4::*;
    match (a, b) {
        (x, y) if x == y => x,
        (N, x) | (x, N) => x,
        (T, F) | (F, T) => B,
        (B, _) | (_, B) => B,
        _ => B,
    }
}

fn b4_conflict(a: B4, b: B4) -> u8 {
    let code = |x: B4| -> u8 {
        match x {
            B4::N => 0b00,
            B4::T => 0b01,
            B4::F => 0b10,
            B4::B => 0b11,
        }
    };
    (code(a) ^ code(b)).count_ones() as u8
}

struct Prepare {
    scaffold_md: String,
    primary_name: Option<String>,
    hits: Vec<(String, i32)>,
    witness_ready: bool,
}

#[derive(Serialize, Deserialize)]
struct SpineReport {
    fused: B4,
    model_voice: B4,
    vessel_voice: B4,
    tool_voice: B4,
    conflict: u8,
    riding: bool,
    prove_balance: bool,
    primary: Option<String>,
    answer_text: String,
    note: String,
}

fn prepare(question: &str, cat: Option<&[CatalogEntry]>) -> Prepare {
    let hits = cat
        .map(|c| search_catalog(c, question, 5))
        .unwrap_or_default();
    let primary = hits.first().map(|(e, _)| e.clone());
    let scaffold = build_scaffold(question, primary.as_ref(), &hits);
    Prepare {
        scaffold_md: scaffold,
        primary_name: primary.as_ref().map(|e| e.name.clone()),
        hits: hits
            .iter()
            .map(|(e, s)| (e.name.clone(), *s))
            .collect(),
        witness_ready: primary.is_some(),
    }
}

fn complete(
    prep: &Prepare,
    answer_text: &str,
    model_voice: B4,
    tool_voice: B4,
    no_selectivity: bool,
    jam: bool,
) -> SpineReport {
    // Lane guard: a conventional proof lives in the T/F-lane; a material forge of the
    // theorem's named entities lives in the B-lane and does NOT test the proposition's
    // truth. Two co-typed entries failing to click is a fact about the entries, not a
    // disproof of a theorem. So a material non-closure (tools=F) must not vote against a
    // proof-shaped answer — that let the agent report a proven theorem as "does not
    // close / does not exist". The tools abstain (N) on the theorem instead.
    let tool_voice = if tool_voice == B4::F && answer_is_proof(answer_text) {
        B4::N
    } else {
        tool_voice
    };
    // Structural co-type: if we have a witness and a non-empty answer that
    // engages the scaffold/witness name, vessel speaks T; empty → N; error markers → F.
    let vessel = if no_selectivity {
        B4::N
    } else if answer_text.trim().is_empty()
        || answer_text.starts_with("[LLM")
        || answer_text.starts_with("[Gemini")
        || answer_text.starts_with("[no API")
    {
        B4::F
    } else if prep.witness_ready {
        // Riding: answer exists against a typed demand/witness
        B4::T
    } else if !answer_text.trim().is_empty() {
        B4::T // model-only structural engagement without catalog
    } else {
        B4::N
    };

    let riding = !no_selectivity && vessel == B4::T && prep.witness_ready;
    // Fuse the model's stated verdict with the vessel co-type (Belnap join).
    let fused_mv = if no_selectivity {
        model_voice
    } else if vessel == B4::N {
        model_voice
    } else {
        b4_join(model_voice, vessel)
    };
    // Then fuse in the tools as a third voice — ground truth about closure. When the model
    // claims a ring the tools deny (T vs F) the join is B: conflict held, never overridden.
    // N means no closure-bearing tool ran, so the tools abstain and the fuse is unchanged.
    // ENGAGR (the membrane's silence arm): a jam ranges WITH the instruments, so if it ran them
    // and they still abstain (tool_voice N), the write-up drifted off-tool — the report is
    // ungrounded and holds no verdict (N), never a confident fused=T off model⋈vessel alone.
    // Outside jam, tool-silence is fine (a conceptual answer that needed no tool) and the fuse
    // stands.
    let fused = if tool_voice == B4::N {
        if jam { B4::N } else { fused_mv }
    } else {
        b4_join(fused_mv, tool_voice)
    };
    // Headline conflict: if the tools spoke, it is model-vs-tools (did the answer's verdict
    // match what the catalog computed?); otherwise the old model-vs-vessel co-type check.
    let conflict = if tool_voice != B4::N {
        b4_conflict(model_voice, tool_voice)
    } else if no_selectivity {
        0
    } else {
        b4_conflict(model_voice, vessel)
    };

    SpineReport {
        fused,
        model_voice,
        vessel_voice: vessel,
        tool_voice,
        conflict,
        riding,
        prove_balance: true, // μ∘δ face: harness closed on successful emit/verify
        primary: prep.primary_name.clone(),
        answer_text: answer_text.to_string(),
        note: if no_selectivity {
            "model only (--no-selectivity)".into()
        } else if tool_voice != B4::N {
            "FFUSE model ⋈ vessel ⋈ tools (tools = ground-truth closure)".into()
        } else if jam {
            "ENGAGR — tools silent after a jam: report UNGROUNDED, held at N (the membrane admitted nothing tool-backed)".into()
        } else {
            "FFUSE model ⋈ vessel".into()
        },
    }
}

fn build_user_packet(question: &str, prep: &Prepare, jam: bool, cycle: u32, total: u32) -> String {
    let mut parts = Vec::new();
    // Compounding cycles: every cycle after the first begins from where the last one ENDED.
    // The prior cycle's final result is already in the conversation above; this makes building
    // on it an instruction, not a hope — so the agent deepens rather than re-derives.
    if cycle > 1 {
        parts.push(format!(
            "## CYCLE {cycle} of {total} — COMPOUND, do not restart.\n\
             Your previous cycle's final result is in the conversation above. Begin from that \
             end-state: take its conclusions as your new starting point and push further — pursue \
             what it left open, deepen or stress-test what it found, build the next layer on top. \
             Re-run tools where you need fresh ground, but do NOT re-derive from scratch what the \
             last cycle already settled. Each cycle is a deeper breath, not a repeat."
        ));
        // Cache-clear: a prior cycle may have recorded a verb as "unavailable / does not
        // exist" when it did not yet exist or was not tried. That claim is STALE and must
        // not be carried forward. The verbs below are live THIS run; re-check by calling,
        // never inherit an absence.
        parts.push(format!(
            "TOOL AVAILABILITY (authoritative, overrides anything the conversation above says): \
             every one of these structural verbs is LIVE right now — {}. Plus the full IG corpus. \
             If a previous cycle wrote that any verb is \"unavailable\", \"does not exist\", or \
             \"cannot be run\", that is STALE and WRONG: discard it and emit the verb's `TOOL:` line \
             to see its real result. You may not park a node at B/N on a claim that a real verb is \
             missing.",
            STRUCTURAL_VERBS.join(", ")
        ));
    }
    if !prep.scaffold_md.is_empty() {
        parts.push("## Grammatic witness scaffold (spine IMSCRIB — instantiate, do not ignore)".into());
        let mut sc = prep.scaffold_md.clone();
        if sc.len() > 14000 {
            sc.truncate(14000);
            sc.push_str("\n\n[scaffold truncated]\n");
        }
        parts.push(sc);
    }
    let label = if cycle > 1 {
        if jam { "## JAM SEED (the original starting point, for reference — you are continuing from your last cycle, not restarting here):" }
        else { "## ORIGINAL QUESTION (for reference — answer by building on your last cycle):" }
    } else if jam {
        "## JAM SEED (a starting point to explore from, NOT a question to answer — leave it whenever something more interesting appears):"
    } else {
        "## USER QUESTION (answer this):"
    };
    parts.push(format!("{label}\n{question}"));
    parts.join("\n\n")
}

fn strip_kernel_records(text: &str) -> String {
    let re = Regex::new(r"(?im)^[ \t]*\[(?:selectivity|vessel|spine|update|broadcast)\s*\|.*$\n?")
        .unwrap();
    delatex(&re.replace_all(text, ""))
}

/// The answer prints to a raw terminal with no math renderer, but the LLM habitually wraps
/// its math in LaTeX (`$$\Delta_{\text{T↔H}} = 0.83 > \theta$$`) which then shows as literal
/// backslash-garbage. Convert the common LaTeX to the plain Unicode the terminal (and the IG
/// house style) actually wants — `Δ_T↔H = 0.83 > θ`. Grouping braces in prose (e.g. a
/// `{binah monad ankh}` set) are left untouched; only math constructs are rewritten.
fn delatex(text: &str) -> String {
    let mut s = text.to_string();
    // Math delimiters and spacing macros first (their next char isn't a letter, so the
    // command table below would miss them).
    for (pat, rep) in [
        ("$$", ""), ("\\[", ""), ("\\]", ""), ("\\(", ""), ("\\)", ""),
        ("\\,", ""), ("\\;", ""), ("\\!", ""), ("\\:", " "), ("\\ ", " "),
        ("\\{", "{"), ("\\}", "}"), ("\\|", "‖"), ("\\\\", "\n"),
        ("\\_", "_"), ("\\%", "%"), ("\\&", "&"), ("\\#", "#"),
    ] {
        s = s.replace(pat, rep);
    }
    s = s.replace('$', ""); // remaining inline math delimiters
    // `\text{X}`, `\mathrm{X}`, … → X (unwrap the styling, keep the content).
    let brace_cmd =
        Regex::new(r"\\(?:text|mathrm|mathbf|mathbb|mathcal|mathit|operatorname|boldsymbol|mathsf)\{([^{}]*)\}")
            .unwrap();
    s = brace_cmd.replace_all(&s, "$1").into_owned();
    // `\frac{A}{B}` → A/B.
    let frac = Regex::new(r"\\frac\{([^{}]*)\}\{([^{}]*)\}").unwrap();
    s = frac.replace_all(&s, "$1/$2").into_owned();
    // (Layout wrappers like \left \right \quad are handled by `sym` in the command pass
    // below — doing them as substring replaces here would chew the \left out of
    // \leftrightarrow. The full-word regex is safe.)
    // Drop the braces around a sub/superscript body: `_{T↔H}` → `_T↔H`, `^{2}` → `^2`.
    let subsup = Regex::new(r"([_^])\{([^{}]*)\}").unwrap();
    s = subsup.replace_all(&s, "$1$2").into_owned();
    // Named commands → Unicode. One pass over `\word`; unknown names lose only the backslash.
    let cmd = Regex::new(r"\\([A-Za-z]+)").unwrap();
    s = cmd
        .replace_all(&s, |c: &regex::Captures| sym(&c[1]).to_string())
        .into_owned();
    s
}

/// LaTeX command name → Unicode glyph (or the bare name if unknown).
fn sym(name: &str) -> &str {
    match name {
        "leftrightarrow" | "iff" => "↔",
        "Leftrightarrow" => "⇔",
        "rightarrow" | "to" | "longrightarrow" | "mapsto" => "→",
        "leftarrow" | "gets" => "←",
        "Rightarrow" | "implies" => "⇒",
        "Delta" => "Δ", "nabla" => "∇", "partial" => "∂",
        "theta" | "vartheta" => "θ", "Theta" => "Θ",
        "mu" => "μ", "delta" => "δ", "sigma" => "σ", "Sigma" => "Σ",
        "omega" => "ω", "Omega" => "Ω", "phi" | "varphi" => "φ", "Phi" => "Φ",
        "gamma" => "γ", "Gamma" => "Γ", "rho" => "ρ", "pi" => "π", "Pi" => "Π",
        "lambda" => "λ", "Lambda" => "Λ", "alpha" => "α", "beta" => "β",
        "epsilon" | "varepsilon" => "ε", "zeta" => "ζ", "eta" => "η",
        "kappa" => "κ", "nu" => "ν", "xi" => "ξ", "Xi" => "Ξ", "tau" => "τ",
        "chi" => "χ", "psi" => "ψ", "Psi" => "Ψ", "circ" => "∘",
        "cdot" => "·", "times" => "×", "otimes" => "⊗", "oplus" => "⊕",
        "geq" | "ge" => "≥", "leq" | "le" => "≤", "neq" | "ne" => "≠",
        "approx" => "≈", "equiv" => "≡", "cong" => "≅", "sim" => "∼", "propto" => "∝",
        "infty" => "∞", "in" => "∈", "notin" => "∉",
        "subset" => "⊂", "subseteq" => "⊆", "supset" => "⊃",
        "forall" => "∀", "exists" => "∃", "wedge" | "land" => "∧", "vee" | "lor" => "∨",
        "neg" | "lnot" => "¬", "pm" => "±", "mp" => "∓", "emptyset" | "varnothing" => "∅",
        "langle" => "⟨", "rangle" => "⟩", "sum" => "∑", "prod" => "∏", "int" => "∫",
        "sqrt" => "√", "ldots" | "dots" | "cdots" => "…", "star" | "ast" => "∗",
        "bullet" => "•", "dagger" => "†", "perp" => "⊥", "parallel" => "‖",
        "top" => "⊤", "bot" => "⊥", "mid" => "|", "backslash" => "\\",
        // styling/layout tokens with no glyph → drop entirely
        "text" | "mathrm" | "mathbf" | "mathbb" | "mathcal" | "mathit" | "mathsf"
        | "boldsymbol" | "operatorname" | "left" | "right" | "displaystyle" | "limits"
        | "nonumber" | "quad" | "qquad" | "label" | "tag" => "",
        other => other, // unknown command: keep the name, shed the backslash
    }
}

#[cfg(test)]
mod delatex_tests {
    use super::delatex;

    #[test]
    fn converts_the_reported_example() {
        // The exact shape the user pasted: display math with \Delta, \text{}, subscript
        // braces, and \theta — must come out as plain terminal Unicode.
        let got = delatex(
            r"$$\Delta_{\text{T↔H}} = |\text{charge}_{\text{T}}(binah) - \text{charge}_{\text{H}}(monad)| = |-0.08 - 0.75| = 0.83 > \theta = 0.50.$$",
        );
        assert!(!got.contains('$'), "dollar delimiters remain: {got}");
        assert!(!got.contains('\\'), "backslash commands remain: {got}");
        assert!(got.contains("Δ_T↔H"), "subscript not unwrapped: {got}");
        assert!(got.contains("charge_T(binah)"), "text{{}} not unwrapped: {got}");
        assert!(got.contains("> θ = 0.50"), "theta not converted: {got}");
    }

    #[test]
    fn leaves_prose_braces_and_glyphs_alone() {
        // A `{set}` in braces and the real IG glyphs must survive untouched.
        let got = delatex("the set {binah monad ankh} imscribes ⟨𐑨𐑰𐑩⊙𐑒𐑳𐑴⟩ with Ð Ř ƒ");
        assert_eq!(got, "the set {binah monad ankh} imscribes ⟨𐑨𐑰𐑩⊙𐑒𐑳𐑴⟩ with Ð Ř ƒ");
    }

    #[test]
    fn common_operators_and_frac() {
        let got = delatex(r"$\mu \circ \delta = \text{id}$, $\frac{a}{b} \geq \Omega \leftrightarrow \Sigma$");
        assert!(got.contains("μ ∘ δ = id"), "{got}");
        assert!(got.contains("a/b ≥ Ω ↔ Σ"), "{got}");
    }
}

#[cfg(test)]
mod collapse_tests {
    use super::collapse_degenerate;

    // The exact derailment seen live: two lines alternating dozens of times. Consecutive-dup
    // collapse would miss the A/B/A/B shape, so this must reduce to the two distinct lines.
    #[test]
    fn collapses_alternating_reasoning_loop() {
        let loop_txt = "Wait: I should output the TOOL lines.\nOkay: I will write the final answer.\n"
            .repeat(30);
        let got = collapse_degenerate(&loop_txt);
        assert!(got.contains("degenerate repetition collapsed"), "{got}");
        assert_eq!(got.matches("Wait: I should output the TOOL lines.").count(), 1, "{got}");
        assert_eq!(got.matches("Okay: I will write the final answer.").count(), 1, "{got}");
    }

    // A normal, varied answer must pass through byte-for-byte — no false positive.
    #[test]
    fn leaves_varied_prose_untouched() {
        let prose = "The assembly does not close.\nEvery polymerize came back telechelic.\n\
                     Sidon density cyclizes via yhhw_word.\nRamsey terminates at one unit.\n\
                     The honest verdict is F for the bipartite exponent.";
        assert_eq!(collapse_degenerate(prose), prose);
    }
}

#[cfg(test)]
mod verb_feedback_tests {
    use super::{run_structural_tool, tool_miss_message, verb_usage, STRUCTURAL_VERBS};

    // Every whitelisted verb must have a usage string, or a real verb given bad args
    // would be reported as nonexistent — the miss that made the eagle loop.
    #[test]
    fn every_structural_verb_has_usage() {
        for v in STRUCTURAL_VERBS {
            assert!(verb_usage(v).is_some(), "no usage help for real verb `{v}`");
        }
        assert!(verb_usage("definitely_not_a_verb").is_none());
    }

    // A real verb given too few names must NOT run (None), so the caller reaches the
    // actionable-feedback path rather than silently dropping args.
    #[test]
    fn real_verb_underargged_does_not_run() {
        assert!(run_structural_tool("polymerize", &["only_one".into()]).is_none());
        assert!(run_structural_tool("scan", &["only_one".into()]).is_none());
        assert!(run_structural_tool("forge", &["only_one".into()]).is_none());
    }

    // The feedback for a real verb names the correct form and echoes what was given,
    // and never calls a real verb nonexistent.
    #[test]
    fn underargged_message_is_actionable() {
        let m = tool_miss_message("polymerize", &["only_one".into()]);
        assert!(m.contains("2+ names"), "no arity guidance: {m}");
        assert!(m.contains("only_one"), "did not echo the given arg: {m}");
        assert!(!m.contains("not an available verb"), "real verb called nonexistent: {m}");

        let empty: [String; 0] = [];
        let m0 = tool_miss_message("forge", &empty);
        assert!(m0.contains("you gave: nothing"), "empty-arg wording wrong: {m0}");
    }

    // A genuinely unknown verb is reported as such, and the list points at real verbs.
    #[test]
    fn unknown_verb_lists_real_verbs() {
        let m = tool_miss_message("frobnicate", &["a".into(), "b".into()]);
        assert!(m.contains("not an available verb"), "{m}");
        assert!(m.contains("polymerize") && m.contains("forge"), "verb list missing: {m}");
    }

    // A rebuilt-in-place binary reads back as "<path> (deleted)"; the resolver strips that
    // so a mid-session tool call relinks to the fresh binary instead of failing to spawn
    // (and being mislabeled "wrong args", the round-7 failure).
    #[test]
    fn strips_the_deleted_suffix() {
        use super::strip_deleted_suffix;
        assert_eq!(strip_deleted_suffix("/x/target/release/ask (deleted)"), Some("/x/target/release/ask"));
        assert_eq!(strip_deleted_suffix("/x/target/release/ask"), None);
    }

    // An IG catalog verb dispatches natively through the live corpus. Needs the MoDoT
    // bridge + IG_inquiry present; skips cleanly if the environment lacks them.
    #[test]
    fn ig_tool_dispatches_through_the_corpus() {
        use super::{run_structural_tool, IG_TOOLS};
        assert!(IG_TOOLS.contains(&"crystal_count"));
        let bridge = super::PathBuf::from(super::expand_user("~/imsgct/MoDoT/modot/ig_tools.py"));
        if !bridge.is_file() {
            eprintln!("skipping: IG bridge not present");
            return;
        }
        let out = run_structural_tool("crystal_count", &[]).expect("crystal_count is an IG verb");
        assert!(out.contains("17280000"), "crystal_count did not ground: {out}");
    }
}

#[cfg(test)]
mod lane_guard_tests {
    use super::{
        answer_is_proof, complete, is_transient_llm_error, verb_isomorphism,
        verbs_falsely_called_absent, LlmResult, Prepare, B4,
    };

    #[test]
    fn read_path_handles_file_and_directory() {
        use std::fs;
        let base = std::env::temp_dir().join(format!("ask_rp_{}", std::process::id()));
        let sub = base.join("sub");
        fs::create_dir_all(&sub).unwrap();
        fs::write(base.join("a.txt"), "alpha").unwrap();
        fs::write(sub.join("b.md"), "beta").unwrap();
        fs::write(base.join("skip.png"), &[0u8, 1, 2]).unwrap(); // binary ext: excluded

        // a single file → its text
        let (c, label) = super::read_path(base.join("a.txt").to_str().unwrap()).unwrap();
        assert_eq!(c, "alpha");
        assert!(label.starts_with("file:"));

        // a directory → both text files concatenated with headers, binary skipped
        let (c, label) = super::read_path(base.to_str().unwrap()).unwrap();
        assert!(c.contains("alpha") && c.contains("beta"), "both files present");
        assert!(c.contains("===== a.txt =====") && c.contains("b.md"), "path headers present");
        assert!(!c.contains("\u{1}"), "binary file excluded");
        assert!(label.starts_with("dir:") && label.contains("2 files"));

        let _ = fs::remove_dir_all(&base);
    }

    #[test]
    fn extracts_multiple_tool_calls_on_one_line_and_zero_arg() {
        // The exact drop from the transcript: two raw directives collapsed onto one line.
        let t = "TOOL: crystal_count TOOL: excite K19_ray_class_field";
        let calls = super::extract_tool_calls(t);
        assert_eq!(calls.len(), 2, "both directives must be extracted");
        assert_eq!(calls[0], ("crystal_count".to_string(), vec![]));
        assert_eq!(
            calls[1],
            ("excite".to_string(), vec!["K19_ray_class_field".to_string()])
        );
    }

    #[test]
    fn tool_extraction_ignores_lowercase_prose() {
        // prose "tool:" must not be read as a directive
        assert!(super::extract_tool_calls("Use the right tool: pick carefully.").is_empty());
    }

    #[test]
    fn catches_a_real_verb_declared_absent() {
        let t = "The `ascend` verb is not available. The tower cannot be built.\n\
                 The `phase_reconstruct` tool does not exist.";
        let mut hits = verbs_falsely_called_absent(t);
        hits.sort();
        assert_eq!(hits, vec!["ascend".to_string(), "phase_reconstruct".to_string()]);
    }

    #[test]
    fn does_not_fire_on_substring_or_clean_text() {
        // "onset" contains "set" but is not the verb; a normal sentence names no absence.
        assert!(verbs_falsely_called_absent("At the onset the ring closes cleanly.").is_empty());
        // an unavailability phrase with no real verb named must not fire
        assert!(verbs_falsely_called_absent("The widget is not available.").is_empty());
    }

    #[test]
    fn isomorphism_states_both_faces_for_key_verbs() {
        // The verbs the operator narrates in a synthesis must read both ways.
        for v in [
            "excite", "cycle", "polymerize", "close", "forge", "distill", "set", "click",
            "filter", "ascend", "phase_reconstruct", "star",
        ] {
            let (chem, math) = verb_isomorphism(v).unwrap_or_else(|| panic!("no isomorphism for {v}"));
            assert!(!chem.is_empty() && !math.is_empty(), "empty face for {v}");
            assert_ne!(chem, math, "the two faces of {v} must differ");
        }
        // excitation and cyclization specifically carry the ⊙ / ρ math the reports lean on.
        assert!(verb_isomorphism("excite").unwrap().1.contains('⊙'));
        assert!(verb_isomorphism("forge").unwrap().1.contains('ρ'));
    }

    // A conventional proof answer is recognized (so a material non-closure abstains on it).
    #[test]
    fn recognizes_a_conventional_proof() {
        let a = "### Conventional Proof\n**Proposition:** h(k) ≪ k².\nProof. Apply the linear sieve...";
        assert!(answer_is_proof(a), "did not recognize a proof answer");
    }

    // A pure structural / forge answer with no theorem is NOT treated as a proof, so a
    // genuine 'does this set close into a ring' jam keeps its real F verdict.
    #[test]
    fn structural_forge_answer_is_not_a_proof() {
        let a = "I forged the set into a 5-membered macrocycle; spectral radius ρ = 3.41, conductive.";
        assert!(!answer_is_proof(a), "forge answer wrongly tagged as a proof");
    }

    // Regression: a proof written "The Proposition" (no colon) + "Conventional Proof" must
    // still register (the strict colon form missed it, leaving tools=F on a proof).
    #[test]
    fn recognizes_proof_without_colon() {
        let a = "### 1. The Proposition\n**YES.**\n### 2. Conventional Proof\nStep 1: random selection...";
        assert!(answer_is_proof(a), "no-colon Proposition / Conventional Proof must count as a proof");
    }

    fn prep() -> Prepare {
        Prepare {
            scaffold_md: String::new(),
            primary_name: Some("jacobsthal_function".into()),
            hits: vec![],
            witness_ready: true,
        }
    }

    const PROOF: &str = "### Conventional Proof\n**Proposition:** h(k) ≪ k². Proof. linear sieve.";
    const FORGE: &str = "I forged the set; the chain terminated early and did not cyclize into a ring.";

    // The reported defect: a proven theorem + a material non-closure used to fuse to B
    // (\"True but does not exist\"). Outside jam the guard now abstains the tools (N) so the
    // proof is not dragged into a false conflict.
    #[test]
    fn proof_not_dragged_to_b_by_material_nonclosure() {
        let rep = complete(&prep(), PROOF, B4::T, B4::F, false, false);
        assert_eq!(rep.tool_voice, B4::N, "material F should abstain on a proof");
        assert_eq!(rep.fused, B4::T, "a proven theorem must not be dragged to B by a non-click");
        assert_eq!(rep.conflict, 0, "no real conflict: the tools never tested the theorem");
    }

    // In a jam the abstention meets the ENGAGR rule: a proof the tools did not ground is
    // held at N (ungrounded), which is honest and, crucially, NOT the B self-contradiction.
    #[test]
    fn proof_in_jam_is_ungrounded_not_contradictory() {
        let rep = complete(&prep(), PROOF, B4::T, B4::F, false, true);
        assert_eq!(rep.tool_voice, B4::N);
        assert_eq!(rep.fused, B4::N, "jam proof the tools did not ground is N, not B");
        assert_ne!(rep.fused, B4::B, "must not be the 'proven but does not exist' contradiction");
    }

    // A genuine structural jam (\"does this set close?\") keeps its real F: model claims a
    // ring, tools deny it, that IS a real conflict and must stay B.
    #[test]
    fn real_structural_nonclosure_still_holds_b() {
        let rep = complete(&prep(), FORGE, B4::T, B4::F, false, true);
        assert_eq!(rep.tool_voice, B4::F, "forge non-closure keeps its F");
        assert_eq!(rep.fused, B4::B, "model T vs tools F is a real conflict → B");
    }

    fn err_res(msg: &str) -> LlmResult {
        LlmResult { text: format!("[{msg}]"), voice: 'F', err: Some(msg.into()) }
    }

    #[test]
    fn read_timeout_is_transient_and_retried() {
        // The exact jam from the cycle-12 report: a slow body read on a long synthesis.
        assert!(is_transient_llm_error(&err_res("timed out reading response")));
        assert!(is_transient_llm_error(&err_res("connection reset by peer")));
    }

    #[test]
    fn malformed_reply_is_not_retried() {
        // A genuine bad body / API error payload keeps its F — retrying resends a bad request.
        assert!(!is_transient_llm_error(&err_res("expected value at line 1 column 1")));
        assert!(!is_transient_llm_error(&err_res("empty content")));
        assert!(!is_transient_llm_error(&LlmResult {
            text: "ok".into(), voice: 'T', err: None,
        }));
    }
}

/// Does the text reference a structural operation (by verb stem or `--flag`)? Used to
/// tell a legitimate conceptual answer (no catalog work needed) from a confabulated one
/// that narrated tools it never ran — the trigger for the no-op prod.
fn mentions_structural_work(text: &str) -> bool {
    const CUES: &[&str] = &[
        "polymeriz", "arrange", "mediator", "excite", "enchain", "cycliz", "modulus",
        "pathway", "--scan", "--close", "--click", "--material", "--switch", "--excite",
        "forge", "spectral radius", "conductance", "--compare", "--dope", "--forge", "--fuse",
        "--cleave", "cleave", "fission", "--anneal", "anneal", "strain",
        "--recall", "recall", "register", "--register",
        "distill", "sublim", "volatility", "azeotrope",
        "crystalliz", "cocrystal", "polymorph", "lattice",
        "chromatograph", "elut", "retention factor", " Rf ",
        "degas", "freeze-pump", "outgas", "sequester", "stain", "reagent",
    ];
    let low = text.to_lowercase();
    CUES.iter().any(|c| low.contains(c))
}

/// Real verbs the draft declares absent ("unavailable", "does not exist", …). A model
/// will sometimes end a run by claiming a verb it does not want to run simply isn't there,
/// parking the node at B/N on a false premise. This finds any line that pairs an
/// unavailability phrase with the name of a verb that actually EXISTS (structural or IG
/// corpus), so the loop can force the call instead of accepting the fabrication. Token-
/// boundary matched, so "set" does not fire inside "onset".
fn verbs_falsely_called_absent(text: &str) -> Vec<String> {
    const NEG: &[&str] = &[
        "not available", "unavailable", "does not exist", "doesn't exist", "do not exist",
        "not exist", "no such verb", "no verb", "is absent", "are absent", "not a verb",
        "cannot be built", "cannot be run", "tool does not", "verb does not", "is not available",
    ];
    let contains_token = |hay: &str, tok: &str| -> bool {
        let bytes = hay.as_bytes();
        let mut from = 0;
        while let Some(rel) = hay[from..].find(tok) {
            let s = from + rel;
            let e = s + tok.len();
            let before_ok = s == 0 || !bytes[s - 1].is_ascii_alphanumeric() && bytes[s - 1] != b'_';
            let after_ok = e >= bytes.len() || !bytes[e].is_ascii_alphanumeric() && bytes[e] != b'_';
            if before_ok && after_ok {
                return true;
            }
            from = s + 1;
        }
        false
    };
    let mut hits: Vec<String> = Vec::new();
    for line in text.lines() {
        let ll = line.to_lowercase();
        if !NEG.iter().any(|p| ll.contains(p)) {
            continue;
        }
        for v in STRUCTURAL_VERBS.iter().chain(IG_TOOLS.iter()) {
            if contains_token(&ll, v) && !hits.iter().any(|h| h == v) {
                hits.push((*v).to_string());
            }
        }
    }
    hits
}

/// Parse `TOOL: <verb> <args…>` lines the agent emitted (one call per line).
/// Tolerant of markdown wrapping (`**TOOL: …**`, `` `TOOL: …` ``, list bullets):
/// leading non-alphanumerics are skipped and each arg is trimmed of `*` `` ` `` `_`.
fn extract_tool_calls(text: &str) -> Vec<(String, Vec<String>)> {
    let trim_md = |s: &str| {
        s.trim_matches(|c: char| c == '*' || c == '`' || c == '_' || c == ' ' || c == '.')
            .to_string()
    };
    // Split on the literal `TOOL:` marker anywhere it appears, so MULTIPLE directives on one
    // line each become their own call (the old line-anchored regex swallowed the second one
    // as the first's args — silently dropping a call). Case-sensitive marker: the model emits
    // `TOOL:` uppercase for a real directive, so prose "tool:" does not false-trigger. A call
    // runs to end-of-line OR the next `TOOL:`, whichever comes first; zero-arg verbs (e.g.
    // `crystal_count`) are now captured instead of requiring a bogus argument.
    const MARK: &str = "TOOL:";
    let mut out = Vec::new();
    let mut from = 0usize;
    while let Some(rel) = text[from..].find(MARK) {
        let start = from + rel + MARK.len();
        let line_end = text[start..].find('\n').map(|i| start + i).unwrap_or(text.len());
        // stop this call at the next TOOL: on the same line, if any
        let seg_end = text[start..line_end]
            .find(MARK)
            .map(|i| start + i)
            .unwrap_or(line_end);
        let seg = text[start..seg_end].trim_start();
        let verb: String = seg
            .chars()
            .take_while(|c| c.is_ascii_alphanumeric() || *c == '_' || *c == '-')
            .collect();
        if !verb.is_empty() {
            let rest = &seg[verb.len()..];
            let args: Vec<String> = rest
                .split_whitespace()
                .map(trim_md)
                .filter(|s| !s.is_empty())
                .collect();
            out.push((verb.to_lowercase(), args));
        }
        from = start; // advance past this marker so the next TOOL: (same line or later) is found
    }
    out
}

/// If a path is the Linux "<path> (deleted)" form, return the real path. `current_exe`
/// reads /proc/self/exe, which after a rebuild-in-place points at the unlinked old inode
/// and reads back as "<path> (deleted)". Pure + tiny so it can be unit-tested.
fn strip_deleted_suffix(path: &str) -> Option<&str> {
    path.strip_suffix(" (deleted)")
}

/// Path to this binary for re-shelling structural verbs. If the running binary was
/// rebuilt in place mid-session, `current_exe` yields "<path> (deleted)"; strip that and
/// use the fresh binary now at the real path, so a tool call relinks to the rebuilt
/// binary instead of failing to spawn. Falls back to whatever `current_exe` returned.
fn resolve_self_exe() -> Option<PathBuf> {
    let e = env::current_exe().ok()?;
    if let Some(real) = strip_deleted_suffix(&e.to_string_lossy()) {
        let p = PathBuf::from(real);
        if p.exists() {
            return Some(p);
        }
    }
    Some(e)
}

/// Execute one whitelisted structural verb by shelling to this same binary and
/// capturing its stdout. Returns None for a non-whitelisted verb or missing args.
/// The whitelist never includes `ask`, so there is no recursion.
fn run_structural_tool(verb: &str, args: &[String]) -> Option<String> {
    // Set notation is not part of any entry name. A `{set}` query makes the model emit
    // `TOOL: arrange {binah monad ankh}`, and the braces/commas would leak in as bogus
    // monomer names ("monomer not found: {binah") — the same silent-corruption class as
    // the click arg-drop. Strip set punctuation at the choke point so every verb sees clean
    // names. `+` (the pre-click token) is deliberately NOT stripped.
    let owned: Vec<String> = args
        .iter()
        .map(|s| s.trim().trim_matches(|c| "{}[](),".contains(c)).to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let args: &[String] = &owned;
    let a = |i: usize| args.get(i).cloned();
    // Arity guard: `click` is pairwise. Passing 3+ names used to silently drop all but
    // the first two — which let the model claim it fused a whole set. Refuse honestly and
    // route the combine to the right tool (the `+` pre-click / polymerize).
    if verb == "click" && args.len() > 2 {
        return Some(format!(
            "click is pairwise: 1 name sweeps the catalog, 2 names click a pair — it cannot take {}. \
             To COMBINE a set of entries, `polymerize {}` (enchains them, reports how they bond and \
             whether they close into a ring), or fold entries into one blended monomer with a `+` token, \
             e.g. `polymerize {}+{} {}`.\n",
            args.len(), args.join(" "), args[0], args[1], args.get(2).cloned().unwrap_or_default()
        ));
    }
    // `imscribe` is the one verb that does NOT shell to this binary: it CREATES a missing
    // entry by running the real generate pipeline, which writes to the live catalog MoDoT
    // already merges on load. So the agent can MAKE what it needs instead of concluding the
    // imscription is lacking. `TOOL: imscribe <name> [free-text description]`.
    if verb == "imscribe" {
        let Some(name) = a(0) else {
            return Some("imscribe needs a name: TOOL: imscribe <name> [description]\n".into());
        };
        let description = if args.len() > 1 { args[1..].join(" ") } else { name.replace('_', " ") };
        return Some(run_imscribe(&name, &description));
    }
    // `ob3ect` also shells to a real external pipeline: the Ob3ect Auto-Designer
    // (~/ob3ect/auto.py), which types a neutral description through all 8 IMASM phases
    // and persists the artifact. The agent can CREATE an ob3ect on the fly whenever it
    // needs a grounded IMASM typing of a procedure, protocol, or entity.
    if verb == "ob3ect" {
        if args.is_empty() {
            return Some("ob3ect needs a description: TOOL: ob3ect <free-text description of the entity>\n".into());
        }
        return Some(run_ob3ect(&args.join(" ")));
    }
    // The full IG tool corpus (compute_distance, primitive_peel, crystal_decode,
    // zfc_probe, aleph_encode, ...) is dispatched natively from here, shelling to
    // the live IG_inquiry dispatcher via modot.ig_tools — one manifold, not a
    // reimplementation. Same pattern as ob3ect: the loop drives it, the corpus runs.
    if IG_TOOLS.contains(&verb) {
        return Some(run_ig_tool(verb, args));
    }
    let flags: Vec<String> = match verb {
        "click" => {
            let mut v = vec!["--click".to_string(), a(0)?];
            if let Some(b) = a(1) { v.push(b); }
            v
        }
        "switch" => vec!["--switch".into(), a(0)?, a(1)?],
        "excite" => vec!["--excite".into(), a(0)?],
        "ascend" => vec!["--ascend".into(), a(0)?],
        "star" => {
            let mut v = vec!["--star".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "filter" => {
            let mut v = vec!["--filter".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "phase_reconstruct" => {
            let mut v = vec!["--phase-reconstruct".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "homolyze" => {
            let mut v = vec!["--homolyze".to_string(), a(0)?];
            if let Some(b) = a(1) { v.push(b); }
            v
        }
        "set" => vec!["--set".into(), a(0)?, a(1)?],
        "scan" => vec!["--set".into(), a(0)?, a(1)?, "--scan-mediators".into()],
        "complement" => vec!["--complement".into(), a(0)?],
        "cycle" => vec!["--cycle".into(), a(0)?, a(1)?],
        "pathway" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--pathway".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "polymerize" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--polymerize".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "close" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--polymerize".to_string()];
            v.extend(args.iter().cloned());
            v.push("--close".into());
            v
        }
        "material" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--polymerize".to_string()];
            v.extend(args.iter().cloned());
            v.push("--props".into());
            v
        }
        "modulus" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--polymerize".to_string()];
            v.extend(args.iter().cloned());
            v.push("--modulus".into());
            v
        }
        "arrange" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--polymerize".to_string()];
            v.extend(args.iter().cloned());
            v.push("--arrange".into());
            v
        }
        "forge" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--forge".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "compare" => {
            // needs both sides and the `vs` separator: compare A B vs X Y
            if args.len() < 5 || !args.iter().any(|a| a == "vs") {
                return None;
            }
            let mut v = vec!["--compare".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "dope" => {
            // base + `with` + dopant: dope A B with C
            if args.len() < 4 || !args.iter().any(|a| a == "with") {
                return None;
            }
            let mut v = vec!["--dope".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "fuse" => {
            // ring A + ring B: fuse A B + X Y
            if args.len() < 5 || !args.iter().any(|a| a == "+") {
                return None;
            }
            let mut v = vec!["--fuse".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "cleave" => {
            if args.len() < 4 {
                return None;
            }
            let mut v = vec!["--cleave".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "anneal" => {
            if args.len() < 3 {
                return None;
            }
            let mut v = vec!["--anneal".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "register" => {
            // register NAME M1 M2 … : forge the set and store it under NAME
            if args.len() < 3 {
                return None;
            }
            let mut v = vec!["--forge".to_string()];
            v.extend(args[1..].iter().cloned());
            v.push("--register".into());
            v.push(args[0].clone());
            v
        }
        "recall" => {
            if args.len() != 1 {
                return None;
            }
            vec!["--recall".to_string(), args[0].clone()]
        }
        "distill" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--distill".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "fdistill" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--fdistill".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "sublime" => vec!["--sublime".into(), a(0)?],
        "crystallize" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--crystallize".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "cocrystallize" => vec!["--cocrystallize".into(), a(0)?, a(1)?],
        "seed" => {
            if args.len() < 3 {
                return None;
            }
            let mut v = vec!["--seed".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "tlc" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--tlc".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "column" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--column".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "fpt" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--fpt".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        "trap" => {
            let mut v = vec!["--trap".to_string(), a(0)?];
            if let Some(x) = a(1) {
                v.push(x);
            }
            v
        }
        "stain" => {
            if args.len() < 2 {
                return None;
            }
            let mut v = vec!["--stain".to_string()];
            v.extend(args.iter().cloned());
            v
        }
        _ => return None,
    };
    // Distinct from the arity None above: a failure to LOCATE or SPAWN the binary is not
    // bad args. It happens when the binary is rebuilt while a session runs (the old inode
    // is unlinked). Return a clear Some so the caller never mislabels it "wrong args", and
    // resolve_self_exe already relinks to a rebuilt-in-place binary when it can.
    let exe = match resolve_self_exe() {
        Some(e) if e.exists() => e,
        other => {
            return Some(format!(
                "(tool runner could not find the ask binary{}; it was likely rebuilt while this \
                 session was running. Restart ./ask so it relinks to the new binary, then re-run.)\n",
                other.map(|p| format!(" at {}", p.display())).unwrap_or_default()
            ));
        }
    };
    let out = match process::Command::new(&exe).args(&flags).output() {
        Ok(o) => o,
        Err(err) => {
            return Some(format!(
                "(tool runner could not execute `{}`: {err}. If the binary was rebuilt mid-session, \
                 restart ./ask.)\n",
                exe.display()
            ));
        }
    };
    // Capture stdout AND stderr — a tool call that errors (e.g. a monomer the model
    // invented that is not in the catalog) must surface its failure, not vanish.
    let mut s = String::from_utf8_lossy(&out.stdout).into_owned();
    let err = String::from_utf8_lossy(&out.stderr);
    if !err.trim().is_empty() {
        s.push_str(err.trim_end());
        s.push('\n');
    }
    // If a verb failed on a missing name, don't leave the agent to conclude it cannot
    // proceed — route it to MAKE the entry. This is the whole point: the imscription being
    // absent is a fixable state, not a wall.
    if let Some(missing) = s
        .split("not found: ")
        .nth(1)
        .and_then(|t| t.split_whitespace().next())
    {
        s.push_str(&format!(
            "\n→ '{missing}' is not in the catalog YET — this is not a dead end. Create it: \
             TOOL: imscribe {missing}\n   (imscribes it via the real generate pipeline; then re-run this verb — the new entry loads automatically.)\n"
        ));
    }
    Some(s)
}

/// The canonical call form for a known structural verb, or None if `verb` is not a
/// structural verb at all. `run_structural_tool` returns None both for an unknown
/// verb AND for a real verb given too few names; without this split the caller
/// reported every miss as "not an available verb", which sent the eagle looping —
/// re-emitting `polymerize A` (one name) round after round instead of adding the
/// second name. This lets the caller answer "polymerize needs 2+ names" so the next
/// round self-corrects.
fn verb_usage(verb: &str) -> Option<&'static str> {
    Some(match verb {
        "click"      => "click A B (or `click A` to sweep the catalog); 1 or 2 names",
        "switch"     => "switch A B; 2 names",
        "excite"     => "excite A; 1 name",
        "set"        => "set A B; 2 names (donor acceptor)",
        "homolyze"   => "homolyze A [B]; 1 or 2 names",
        "scan"       => "scan A B; 2 names (donor acceptor), ranks mediators of A to B",
        "complement" => "complement A; 1 name",
        "cycle"      => "cycle C S; 2 names (catalyst substrate)",
        "pathway"    => "pathway S C1 C2...; 2+ names",
        "polymerize" => "polymerize M1 M2...; 2+ names to chain",
        "close"      => "close M1 M2...; 2+ names",
        "material"   => "material M1 M2...; 2+ names",
        "modulus"    => "modulus M1 M2...; 2+ names",
        "arrange"    => "arrange M1 M2...; 2+ names (unordered set)",
        "forge"      => "forge M1 M2...; 2+ names (unordered set)",
        "compare"    => "compare A B vs X Y; two sets split by `vs`",
        "dope"       => "dope A B with C; base and dopant split by `with`",
        "fuse"       => "fuse A B + X Y; two rings split by `+`",
        "cleave"     => "cleave M1 M2...; 2+ names (forges then cuts the ring)",
        "anneal"     => "anneal M1 M2...; 2+ names",
        "register"   => "register NAME M1 M2...; a NAME then 2+ names",
        "recall"     => "recall NAME; exactly 1 stored name",
        "distill"    => "distill M1 M2...; 2+ names (separate by volatility ⊙)",
        "fdistill"   => "fdistill M1 M2...; 2+ names (fractional: ranked column by ⊙)",
        "sublime"    => "sublime A; 1 name (skip-path purify across ⊙)",
        "crystallize"   => "crystallize M1 M2...; 2+ names (grow the lattice, reject non-fitting)",
        "cocrystallize" => "cocrystallize A B; 2 names (non-covalent co-lattice)",
        "seed"          => "seed M1 M2 ... with S; a set, then `with`, then one seed",
        "tlc"           => "tlc M1 M2...; 2+ names (spread by Rf, count bands)",
        "column"        => "column M1 M2 ... [on S]; 2+ names, optional `on S` stationary phase",
        "fpt"           => "fpt M1 M2...; 2+ names (degas: shed weakly-held units)",
        "trap"          => "trap A [X]; 1 unit, optional counter X (ionic sequester)",
        "stain"         => "stain R M1 M2...; a reagent (kmno4/uv/chiral/ninhydrin/iodine) then 1+ units",
        "filter"     => "filter A B [C …]; 2+ reference names (narrow the catalog to their shared structural floor)",
        "ascend"     => "ascend A; 1 name (construct the next ramified tower level from A's excited state)",
        "phase_reconstruct" => "phase_reconstruct M1 M2 …; 2+ names (recover the relative phase word from the closed ring)",
        "star"       => "star M1 M2 M3 …; 4+ names (hub-and-arms star polymer: auto core + arms, ρ=√f)",
        "imscribe"   => "imscribe NAME [description]; a name and optional description",
        "ob3ect"     => "ob3ect <description>; free-text description of the entity to type",
        _ => return None,
    })
}

/// The chem↔math isomorphism, stated both ways for a structural verb. The engine runs
/// one operation; it reads as chemistry AND as mathematics — two lossless faces of the
/// same act (R∧W∧X), not a chemical result with a mathematical gloss bolted on. Returned
/// as (chemical, mathematical) so the final report can name what each operation MEANS in
/// both registers, for exactly the verbs that actually ran.
fn verb_isomorphism(verb: &str) -> Option<(&'static str, &'static str)> {
    Some(match verb {
        "click" => (
            "two fragments bond on a live conjugate pair — a covalent fusion that holds only if the partners are complementary",
            "Frobenius fusion δ then μ of two objects across a conjugate axis (D↔W / T↔H / R↔S); closes iff the tuples are complementary, with μ∘δ = id on the diagonal",
        ),
        "excite" => (
            "promotion to the excited electronic state — Criticality ⊙ raised to the exceptional-point resonance",
            "a lift of the object to its critical manifold: the self-modeling parameter ⊙ driven to the spectral degeneracy (exceptional point) where eigenvalues coalesce — the ramified/excited level above the ground state, resonant but not yet a constructed extension",
        ),
        "cycle" => (
            "a catalyst turns the substrate over and is returned unchanged",
            "a fixed point of the composed map, μ∘δ = id — an idempotent that is the identity on its carrier",
        ),
        "polymerize" | "close" => (
            "monomers chain head-to-tail; the chain either terminates open or closes into a ring (macrocyclization)",
            "composition of morphisms in sequence; cyclization = the composite returning to its start, a closed loop (a categorical cycle), which the graph realizes as ρ = 2 exactly",
        ),
        "forge" | "material" | "arrange" => (
            "cast the monomer set into its best ring and read topology, conductance, and stability",
            "assemble the adjacency graph and take its spectrum: spectral radius ρ and gap; ρ = 2 exactly ⟺ a pure cycle, ρ > 2 ⟺ branched",
        ),
        "set" => (
            "single-electron transfer, donor → acceptor, giving radical ions",
            "transport of one winding quantum Ω across the ⊙ gradient — a unit change of the topological winding invariant",
        ),
        "distill" | "fdistill" | "sublime" => (
            "separation by volatility along Criticality ⊙ — volatile head vs involatile residue; a tie on ⊙ is an azeotrope",
            "a projection/ordering of the set onto the ⊙ coordinate; an azeotrope = two elements degenerate on ⊙, a tie the projection cannot resolve",
        ),
        "crystallize" | "cocrystallize" | "seed" => (
            "grow the ordered lattice — the units that fit vs the rejected mother-pool; a closed ring is a crystal",
            "the maximal consistent/closed sublattice of tiling elements vs the complement; full closure = a complete substructure",
        ),
        "scan" => (
            "rank catalog mediators of the A → B electron transfer",
            "rank elements by mediating distance on the transfer axis — the best interpolant between the endpoints",
        ),
        "homolyze" | "cleave" => (
            "homolytic bond cleavage into neutral radicals / ring fission into two daughters",
            "the reverse of fusion: a symmetric split δ undoing μ, cutting the object on a complementary arc",
        ),
        "filter" => (
            "keep only the species sharing the reference floor; the rest are washed out",
            "restrict to the sublevel set where the shared invariants match — a necessary (upper-bound) condition on the candidate set, not an exact solution",
        ),
        "ascend" => (
            "take the excited resonance and fix it into a constructed higher state (build one floor of the tower)",
            "analytically continue past the exceptional-point branch to the complex-axis fixed point and add one winding quantum Ω — one ramified level of the extension tower",
        ),
        "phase_reconstruct" => (
            "read the relative phases off a closed ring — fixed up to one global phase",
            "recover the relative phase word from cyclization (the flat-autocorrelation constraint); determined modulo a global gauge, exactly as C_m = 1/(d+1) fixes ψ up to a global phase",
        ),
        "star" => (
            "a multifunctional core with f arms radiating out — a hub-and-spoke star polymer, no arm–arm bonds",
            "the star graph K(1,f): one hub adjacent to f independent leaves; adjacency spectral radius ρ = √f with spectrum {+√f, 0×(f−1), −√f} — contrast the ρ=2 of a pure cycle",
        ),
        _ => return None,
    })
}

/// The IG tool corpus, dispatched natively via `run_ig_tool` (shells to the live
/// IG_inquiry dispatcher through modot.ig_tools). Kept separate from
/// STRUCTURAL_VERBS because these do not shell to the ask binary and are not part
/// of the chemistry verb_usage coverage; they are the catalog/analysis tools.
const IG_TOOLS: &[&str] = &[
    "lookup_catalog", "list_catalog", "encode_system", "imscribe_system",
    "check_imscription", "ouroborics", "compute_distance", "compute_conflict_distance",
    "compute_meet", "compute_join", "compute_tensor", "find_analogies", "phi_c_probe",
    "topo_protection_probe", "consciousness_score", "project", "primitive_peel",
    "principal_decomp", "retrosynthetic_path", "emergence_frontier", "compute_promotions",
    "predict_from_promotions", "register_promotion_pattern", "crystal_encode",
    "crystal_decode", "crystal_navigate", "crystal_count", "crystal_tier_census",
    "crystal_nearest", "crystal_tier_gap_ladder", "quiver_encode", "domain_info",
    "domain_verify", "domain_nearest", "navigator_info", "zfc_formula", "zfc_probe",
    "zfc_catalog_probe", "aleph_encode", "aleph_distance", "riemann_xi_info",
    "ask_question", "record_insight",
];

/// Run one IG catalog tool by shelling to the live corpus (modot.ig_tools call ...).
/// Mirrors run_ob3ect: MoDoT venv python if present, stdout is the JSON result, and
/// stderr is surfaced only on failure so the benign runpy warning does not pollute.
fn run_ig_tool(verb: &str, args: &[String]) -> String {
    let modot_dir = PathBuf::from(expand_user("~/imsgct/MoDoT"));
    if !modot_dir.join("modot/ig_tools.py").is_file() {
        return format!("{verb}: modot/ig_tools.py not found — the IG tool bridge is not present.\n");
    }
    let venv_py = modot_dir.join(".venv/bin/python");
    let mut cmd = if venv_py.is_file() {
        process::Command::new(&venv_py)
    } else {
        process::Command::new("python3")
    };
    cmd.arg("-m").arg("modot.ig_tools").arg("call").arg(verb).args(args)
        .current_dir(&modot_dir);
    let out = match cmd.output() {
        Ok(o) => o,
        Err(e) => return format!("{verb}: could not run the IG tool bridge: {e}\n"),
    };
    let mut s = String::from_utf8_lossy(&out.stdout).into_owned();
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        if !err.trim().is_empty() {
            s.push_str(err.trim_end());
            s.push('\n');
        }
    }
    if s.trim().is_empty() {
        s = format!("{verb}: the IG tool produced no output.\n");
    }
    s
}

/// Every structural verb the agent may call. Single source of truth for the
/// unknown-verb feedback list and the coverage test that keeps `verb_usage` in sync.
const STRUCTURAL_VERBS: &[&str] = &[
    "click", "switch", "excite", "set", "homolyze", "scan", "complement", "cycle",
    "pathway", "polymerize", "close", "material", "modulus", "arrange", "forge",
    "compare", "dope", "fuse", "cleave", "anneal", "register", "recall", "imscribe", "ob3ect",
    "distill", "fdistill", "sublime",
    "crystallize", "cocrystallize", "seed",
    "tlc", "column", "fpt", "trap", "stain",
    "filter", "ascend", "phase_reconstruct", "star",
];

/// Feedback when `run_structural_tool` could not run `verb`: the correct call form
/// if it is a real verb given bad/too-few args, or the verb list if it is unknown.
/// Split out (from the agent loop) so it is unit-testable and so a real verb never
/// gets reported as nonexistent — the miss that made the eagle re-emit the same call.
fn tool_miss_message(verb: &str, args: &[String]) -> String {
    match verb_usage(verb) {
        Some(usage) => {
            let gave = args.join(" ");
            format!(
                "wrong or too few args (you gave: {}). Correct form: {usage}. \
                 Re-emit it with the right names next round.",
                if gave.is_empty() { "nothing".to_string() } else { gave }
            )
        }
        None => format!(
            "not an available verb. Available verbs: {}.",
            STRUCTURAL_VERBS.join(", ")
        ),
    }
}

/// Is a name already registered — in the base IG_catalog.json OR the live
/// ~/.imscrbgrmr/catalog.json? Cheap `"name": "…"` substring check, matching the guard in
/// register_chimera. Used to skip a wasted generate call for an entry that already exists.
/// Trailing "decoration" marks the model stacks to name a DERIVED state (K19⁺, K19⁺⁺, X•,
/// Y*). A derived state — excited / ascended / radical — is a TRANSFORM of its base, not a new
/// catalog entity; imscribing each level is a runaway that pollutes the catalog (seen: an
/// 87-call round registering k19_ray_class_field⁺⁺⁺⁺⁺…). If `name` is some base plus a run of
/// these marks and that base already exists (case-insensitively), return the base.
fn decoration_ladder_base(name: &str) -> Option<String> {
    const DECO: &[char] = &['⁺', '•', '*', '+', '★', '†', '‡', '′', '″', '·', '∗', '°'];
    let base = name.trim_end_matches(|c: char| DECO.contains(&c) || c == ' ');
    if base.is_empty() || base == name {
        return None;
    }
    let bl = base.to_lowercase();
    catalog_names().into_iter().find(|n| n.to_lowercase() == bl)
}

fn catalog_has_name(name: &str) -> bool {
    // Case-insensitive: the generate pipeline lowercases names, so an exact-case substring
    // match falsely reported "did not register" for a name the generator had actually written
    // (e.g. requested `K19…`, stored `k19…`).
    let nl = name.to_lowercase();
    catalog_names().into_iter().any(|n| n.to_lowercase() == nl)
}

/// All entry names across the base and live catalogs.
fn catalog_names() -> Vec<String> {
    let live = PathBuf::from(expand_user("~/.imscrbgrmr/catalog.json"));
    let mut names = Vec::new();
    for p in [resolve_catalog_path(), Some(live)].into_iter().flatten() {
        let Ok(text) = fs::read_to_string(&p) else { continue };
        let Ok(v) = serde_json::from_str::<Value>(&text) else { continue };
        let arr = v.get("imscriptions").cloned().unwrap_or(v);
        if let Some(a) = arr.as_array() {
            for item in a {
                if let Some(n) = item.get("name").and_then(|x| x.as_str()) {
                    names.push(n.to_string());
                }
            }
        }
    }
    names
}

/// Normalize a name for near-duplicate comparison: lowercase, drop surrounding underscores,
/// and shed a leading article. `the_djed_pillar`, `djed_pillar`, `Djed_Pillar` all collapse.
fn normalize_name(n: &str) -> String {
    let n = n.trim_matches('_').to_lowercase();
    for art in ["the_", "a_", "an_"] {
        if let Some(rest) = n.strip_prefix(art) {
            return rest.trim_matches('_').to_string();
        }
    }
    n
}

/// If `name` is a variant of an entry that already exists (same normalized form, different
/// spelling), return that entry's real name. The agent tends to invent `the_djed_pillar`
/// when `djed_pillar` exists; this catches it before a redundant entry is generated.
fn catalog_near_match(name: &str) -> Option<String> {
    let target = normalize_name(name);
    if target.is_empty() {
        return None;
    }
    catalog_names()
        .into_iter()
        .find(|existing| existing != name && normalize_name(existing) == target)
}

/// Create a missing catalog entry by running the REAL imscription pipeline — the default
/// `imscribe generate "<description>" --name <name>` guided stack (never a hand-written
/// tuple; the tuple is sourced procedurally by the generator). It writes to the live catalog
/// (~/.imscrbgrmr/catalog.json) that MoDoT already merges on load, so the next tool-call
/// subprocess sees the new entry. Success is judged by whether the entry actually landed,
/// not by the rich CLI's exit code.
fn run_imscribe(name: &str, description: &str) -> String {
    if catalog_has_name(name) {
        return format!(
            "'{name}' is already in the catalog — use it directly (e.g. TOOL: polymerize {name} …). No imscription needed.\n"
        );
    }
    // Near-duplicate guard: don't generate `the_djed_pillar` when `djed_pillar` exists.
    if let Some(existing) = catalog_near_match(name) {
        return format!(
            "'{name}' is a variant of '{existing}', which is ALREADY in the catalog. Use '{existing}' \
             directly (e.g. TOOL: polymerize {existing} …) — do not imscribe a near-duplicate.\n"
        );
    }
    // Decoration-ladder guard: refuse to register a derived state named as base + trailing
    // marks (K19⁺, K19⁺⁺, X•). Imscribing each level is a non-terminating catalog-polluting
    // runaway; the derived state is a transform of its base, not a new entity.
    if let Some(base) = decoration_ladder_base(name) {
        return format!(
            "'{name}' is a decorated derivative of '{base}' (a name-ladder of trailing marks like ⁺ • *). \
             A derived state — an excited, ascended, or radical form — is a TRANSFORM of its base, not a new \
             catalog entity; imscribing each level pollutes the catalog and never terminates. NOT registered. \
             The verb that built it (`ascend` / `excite` / `homolyze`) already returned its tuple — use that. \
             A tower that only climbs by re-imscription is a B frontier, not a catalog to enumerate.\n"
        );
    }
    let Some(cat) = resolve_catalog_path() else {
        return "imscribe: could not locate the IG catalog / imscribing_grammar package.\n".into();
    };
    let Some(ig_dir) = cat.parent().map(|d| d.to_path_buf()) else {
        return "imscribe: catalog path has no parent directory.\n".into();
    };
    let venv_imscribe = ig_dir.join(".venv/bin/imscribe");
    let mut cmd = if venv_imscribe.is_file() {
        process::Command::new(&venv_imscribe)
    } else {
        process::Command::new("imscribe") // fall back to PATH
    };
    cmd.args(["generate", description, "--name", name])
        .current_dir(&ig_dir);
    // The generate stack needs a provider; the user's env sets IG_PROVIDER=openrouter, but
    // default it defensively so a bare environment still produces an entry.
    if env::var("IG_PROVIDER").is_err() {
        cmd.env("IG_PROVIDER", "openrouter");
    }
    let out = match cmd.output() {
        Ok(o) => o,
        Err(e) => return format!("imscribe: could not run the generate pipeline: {e}\n"),
    };
    if catalog_has_name(name) {
        format!(
            "✓ imscribed '{name}' via the generate pipeline (guided). It is now in the live catalog — \
             use it in your next TOOL line (e.g. TOOL: polymerize {name} …); it loads fresh automatically.\n"
        )
    } else {
        let combined = format!(
            "{}{}",
            String::from_utf8_lossy(&out.stdout),
            String::from_utf8_lossy(&out.stderr)
        );
        let lines: Vec<&str> = combined.lines().filter(|l| !l.trim().is_empty()).collect();
        let tail = lines[lines.len().saturating_sub(8)..].join("\n");
        format!("imscribe '{name}' did not register. Generator output (tail):\n{tail}\n")
    }
}

/// Design an ob3ect on the fly via the real Auto-Designer (~/ob3ect/auto.py): neutral
/// description in, full IMASM typing out (opcodes, Frobenius verdict, registers,
/// bootstrap sequence). Persists to ~/ob3ect/digital/<slug>/ as every ob3ect does.
/// Bounded retries — the pipeline's default is retry-forever, which would hang a round.
fn run_ob3ect(description: &str) -> String {
    let ob3_dir = PathBuf::from(expand_user("~/ob3ect"));
    if !ob3_dir.join("auto.py").is_file() {
        return "ob3ect: ~/ob3ect/auto.py not found — the Auto-Designer pipeline is not present.\n".into();
    }
    let venv_py = ob3_dir.join(".venv/bin/python");
    let mut cmd = if venv_py.is_file() {
        process::Command::new(&venv_py)
    } else {
        process::Command::new("python3")
    };
    cmd.args(["auto.py", description, "--no-diagram", "--no-scaffold", "--retries", "3"])
        .current_dir(&ob3_dir);
    if env::var("IG_PROVIDER").is_err() {
        cmd.env("IG_PROVIDER", "openrouter");
    }
    let out = match cmd.output() {
        Ok(o) => o,
        Err(e) => return format!("ob3ect: could not run the Auto-Designer: {e}\n"),
    };
    let mut s = String::from_utf8_lossy(&out.stdout).into_owned();
    let err = String::from_utf8_lossy(&out.stderr);
    if !out.status.success() && !err.trim().is_empty() {
        s.push_str(err.trim_end());
        s.push('\n');
    }
    if s.trim().is_empty() {
        s = "ob3ect: the Auto-Designer produced no output.\n".into();
    }
    s
}

fn print_spine(rep: &SpineReport, prep: &Prepare, verbose: bool) {
    println!();
    println!("{}", "=".repeat(60));
    println!("MANUSCRIPT SPINE REPORT");
    println!(
        "  fused={}  model={}  vessel={}  tools={}  conflict={}",
        b4_name(rep.fused),
        b4_name(rep.model_voice),
        b4_name(rep.vessel_voice),
        b4_name(rep.tool_voice),
        rep.conflict
    );
    println!(
        "  faces: prove_balance={}  unify_B=T+F=true  port_riding={}  witness={}",
        rep.prove_balance,
        rep.riding,
        rep.primary.as_deref().unwrap_or("—")
    );
    println!("  protocol: VINIT→IMSCRIB→FSPLIT→EVALT→EVALF→FFUSE→ENGAGR→IFIX");
    println!("  note: {}", rep.note);
    if verbose {
        println!("  hits:");
        for (n, s) in &prep.hits {
            println!("    [{s:>3}] {n}");
        }
    }
    println!("{}", "=".repeat(60));
    record_spine(rep);
}

/// Append the spine report (serialized) as a JSON record to MoDoT/crystal_fs/records.jsonl,
/// the same audit trail the Python agent keeps. Best-effort: creates the dir, ignores IO
/// errors. This is the read that grounds `answer_text` and the Serialize/Deserialize derives.
fn record_spine(rep: &SpineReport) {
    let Ok(line) = serde_json::to_string(rep) else {
        return;
    };
    // MoDoT root = four ancestors up from the binary (.../MoDoT/ask_native/target/release/ask).
    let Some(root) = env::current_exe()
        .ok()
        .and_then(|e| e.ancestors().nth(4).map(|r| r.to_path_buf()))
    else {
        return;
    };
    let dir = root.join("crystal_fs");
    let _ = std::fs::create_dir_all(&dir);
    if let Ok(mut f) = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(dir.join("records.jsonl"))
    {
        use std::io::Write;
        let _ = writeln!(f, "{line}");
    }
}

fn run_one(
    question: &str,
    source: &str,
    cli: &Cli,
    llm: &Llm,
    cat: Option<&[CatalogEntry]>,
    conversation: &mut Vec<(String, String)>,
) -> i32 {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  ASK — host native ManuscriptSpine (no Python)       ║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!("Source: {source}");
    println!(
        "Options: verbose={} dry_run={} no_selectivity={} cycles={} eagles={} max_tokens={}",
        cli.verbose, cli.dry_run, cli.no_selectivity, cli.cycles,
        if cli.eagles > 0 { cli.eagles.to_string() } else { "auto".into() }, cli.max_tokens
    );
    println!(
        "Model: {} ({})",
        llm.model,
        match llm.provider {
            Provider::OpenRouter => "openrouter",
            Provider::GeminiDirect => "gemini-direct",
            Provider::DeepSeek => "deepseek",
        }
    );
    println!("Question ({} chars):\n", question.chars().count());
    // Show up to 2k of question in header; full text goes to model
    let preview: String = question.chars().take(2000).collect();
    println!("{preview}");
    if question.chars().count() > 2000 {
        println!("… [{} more chars]", question.chars().count() - 2000);
    }
    println!();

    // IMSCRIB
    let prep = prepare(question, cat);
    if cli.verbose {
        println!("── IMSCRIB (catalog witness) ──");
        if prep.hits.is_empty() {
            println!("  (no catalog hits)");
        } else {
            for (n, s) in &prep.hits {
                println!("  [{s:>3}] {n}");
            }
        }
        println!(
            "  primary: {}",
            prep.primary_name.as_deref().unwrap_or("—")
        );
        println!();
    }

    // Proof-intent route: a `prove:` prefix or a literal Lean theorem/lemma goes to
    // the kernel-gated prover (native — shells to `lake build`), not the prose spine.
    // Not closed is a navigation frontier (B), never a verdict of unprovability.
    if !cli.dry_run && llm.api_key.is_some() {
        if let Some(goal) = prover::proof_intent(question) {
            println!("── ROUTE: proof-intent → kernel-gated prover ──");
            let mut p = prover::LeanProver::new(llm, cli.verbose);
            p.set_expand(cli.expand);
            p.set_eagles(cli.eagles);
            // --cycles across the board: re-fly the whole escalation up to `cycles`
            // times, stopping the moment the kernel closes it. Each cycle is a fresh
            // run at the wall (the model may take a different path), so a stubborn B
            // frontier gets more than one attempt without editing the source.
            let cycles = cli.cycles.max(1);
            let mut r = p.prove(&goal);
            let mut used = 1;
            while !r.closed && used < cycles {
                used += 1;
                println!("── cycle {used}/{cycles} (re-fly the escalation) ──");
                r = p.prove(&goal);
            }
            println!("── ANSWER (kernel-gated prover) ──");
            if r.closed {
                println!("Closed green through the Lean kernel (no sorry):\n");
                println!("{}", r.source);
            } else if r.note.contains("escalation cap") {
                println!(
                    "Not closed within the current escalation cap. A navigation \
                     frontier (B), not a verdict of unprovability — the path exists; \
                     raise the rounds/budget to push deeper (--eagles N for more \
                     escalation rounds, --cycles N to re-fly the whole run).\n"
                );
                println!("Last frontier:\n{}", r.last_output);
            } else {
                // Rejected: ill-posed goal (hole in the statement) or definitional
                // rigging (the model authored the goal's own meaning). Not a frontier.
                println!("REJECTED (not a valid proof): {}\n", r.note);
                if !r.source.is_empty() {
                    println!("The model produced this, which was refused:\n{}", r.source);
                }
            }
            println!();
            println!("{}", "=".repeat(60));
            println!("PROVER REPORT");
            println!("  route=proof  closed={}  depth={}", r.closed, r.depth);
            if cli.expand > 0 {
                println!("  detail={} (walked-out rendering; statement held identical by the fidelity gate)", cli.expand);
            }
            // Lane ontology: the T/F-lane Witness IS the conventional proof; the
            // B-lane Witness is the Dual-Link vessel it rides as (imscription route).
            if r.closed {
                if r.note.contains("Witness-Vessel") {
                    println!("  lanes: B-lane Witness = the Dual-Link vessel; T/F-lane Witness = the conventional proof filling it");
                } else {
                    println!("  lane: T/F-lane Witness = the conventional proof (what we have called a proof)");
                }
            }
            if !r.note.is_empty() {
                println!("  note: {}", r.note);
            }
            println!("  protocol: VINIT→IMSCRIB→FSPLIT→(lake build)→EVALT/EVALF→FFUSE→TANCH");
            println!("{}", "=".repeat(60));
            return if r.closed { 0 } else { 1 };
        }
    }

    let mut last_code = 0;
    for cycle in 1..=cli.cycles.max(1) {
        if cli.cycles > 1 {
            println!("── cycle {cycle}/{} ──", cli.cycles);
        }

        let mut answer;
        let mut model_voice;
        let mut tool_voice = B4::N;

        if cli.dry_run {
            answer = format!(
                "[dry-run] Structural path only.\n\
                 Witness: {}\n\
                 Scaffold ready: {}.\n\
                 Re-run without --dry-run for full Gemini-class answer.\n\n\
                 {}",
                prep.primary_name.as_deref().unwrap_or("—"),
                prep.witness_ready,
                prep.scaffold_md.chars().take(3000).collect::<String>()
            );
            model_voice = B4::N;
        } else {
            let user_packet = build_user_packet(question, &prep, cli.jam, cycle, cli.cycles.max(1));
            // conversation: system once, then history, then this turn
            let mut msgs: Vec<(String, String)> = Vec::new();
            if conversation.is_empty() {
                let jam = if cli.jam { JAM_PROMPT } else { "" };
                msgs.push(("system".into(), format!("{}\n{}\n{}\n{}", prover::EPISTEMIC_STANCE, SYSTEM_PROMPT, TOOLS_PROMPT, jam)));
            }
            for (r, c) in conversation.iter() {
                msgs.push((r.clone(), c.clone()));
            }
            msgs.push(("user".into(), user_packet));

            if cli.verbose {
                println!(
                    "── FSPLIT (model infer, max_tokens={}) ──",
                    cli.max_tokens
                );
            }
            let res = infer(llm, &msgs, cli.max_tokens, cli.temperature);
            answer = strip_kernel_records(&res.text);
            model_voice = b4_from_char(res.voice);
            if let Some(e) = res.err {
                eprintln!("[warn] LLM: {e}");
                last_code = 2;
                if is_fatal_llm_error(&e) {
                    let pname = match llm.provider {
                        Provider::OpenRouter => "openrouter",
                        Provider::GeminiDirect => "gemini",
                        Provider::DeepSeek => "deepseek",
                    };
                    eprintln!(
                        "[ask] fatal LLM error on provider '{pname}' — aborting the run (not transient: 402 = out of credits, 401 = bad key). \
                         Switch providers (--provider deepseek | --provider gemini) or top up the account, then re-run."
                    );
                    break;
                }
            }

            // Update multi-turn history with the raw question + answer
            conversation.push(("user".into(), question.to_string()));
            conversation.push(("assistant".into(), answer.clone()));
        }

        // Print full answer — no truncation
        println!("── ANSWER ──");
        println!("{answer}");
        println!();

        // Agentic loop (THINK → ACT → OBSERVE → UPDATE): the operator runs tools, sees the
        // REAL output, and decides its next act — iterating until it has no more tool calls —
        // instead of front-loading one batch of guesses and narrating the rest. Every round
        // the tool results come back as ground truth (the golem constraint), so it acts and
        // then speaks on what the Grammar actually computed. Bounded by MAX_ROUNDS.
        if !cli.dry_run {
            // The eagles: how many ACT→OBSERVE rounds fly out to run tools. --eagles sets it
            // across the board; 0 = auto — jam gets a long leash so it can actually range, a
            // normal answer is bounded tight.
            // --eagles N is an EXPLICIT leash the user asked for. Without it there is no
            // arbitrary ceiling — a round budget is a wall, and a wall is the same
            // defeatism as "beyond the Grammar's reach". The loop runs until the operator
            // stops emitting tools (done) or STALLS (whole rounds of nothing but calls it
            // already has results for). A large backstop guards only the pathological
            // infinite loop, not the task.
            let max_rounds: usize = if cli.eagles > 0 { cli.eagles as usize } else { 400 };
            const STALL_ROUNDS: usize = 3;
            let mut agent_msgs: Vec<(String, String)> = Vec::new();
            agent_msgs.push((
                "system".to_string(),
                format!(
                    "{}\n{}\n{}\n{}\nYou are in an ACT→OBSERVE loop: emit TOOL: lines to run verbs over the real \
                     catalog; their outputs return as ground truth and you choose the next step. Iterate — run a \
                     tool, read its result, run the next — until the task is actually done, then give your FINAL \
                     answer with NO TOOL: lines. NEVER narrate a step you could run; run it. Never contradict a \
                     tool result or introduce anything the tools did not return.",
                    prover::EPISTEMIC_STANCE, SYSTEM_PROMPT, TOOLS_PROMPT,
                    if cli.jam { JAM_PROMPT } else { "" }
                ),
            ));
            for (r, c) in conversation.iter().take(conversation.len().saturating_sub(2)) {
                agent_msgs.push((r.clone(), c.clone()));
            }
            agent_msgs.push(("user".to_string(), build_user_packet(question, &prep, cli.jam, cycle, cli.cycles.max(1))));

            let mut current = answer.clone(); // the draft is round-0's action

            // No-op guard: the loop and the golem constraint can only bind tools that
            // actually RAN. A "prove/characterize this" framing lets the operator write a
            // whole "Execution: Structural Tools" section as prose flags (`--polymerize:`,
            // `--close:`) — never a `TOOL:` line — then assert a computed verdict (PROVED,
            // enchainment, closure) it never computed. If the draft narrates structural
            // work but ran nothing, prod it once to actually act before we accept it.
            if extract_tool_calls(&current).is_empty()
                && (mentions_structural_work(&current) || question.contains("--") || cli.jam)
            {
                println!("── PROD (narrated tools, ran none — forcing action) ──");
                agent_msgs.push(("assistant".to_string(), current.clone()));
                agent_msgs.push((
                    "user".to_string(),
                    "You wrote a characterization — naming operations like polymerize / close / scan / \
                     excite / arrange — but ran ZERO structural tools: not one `TOOL:` line. You may not \
                     assert a computed result (enchainment, cyclization, a mediator, a modulus, a PROVED \
                     verdict) you did not compute. The catalog is live. Emit `TOOL: <verb> <args>` lines \
                     now — verb WITHOUT dashes, catalog names as args (e.g. `TOOL: polymerize binah set \
                     atiyah_singer_index_theorem`), one call per line — to actually run the operations you \
                     described. Then, and only then, write your final answer grounded in what they return."
                        .to_string(),
                ));
                let res = infer(llm, &agent_msgs, cli.max_tokens, cli.temperature);
                current = strip_kernel_records(&res.text);
                println!("{current}");
                println!();
            }

            let mut all_tool_output = String::new(); // every round's real output → the tool voice
            // The execution arm of the Dual-Link: every verb that ACTUALLY ran (returned output).
            // A verdict fuses only if its verb is in here; a claim about a verb NOT here has no
            // arm to fuse against, so it collapses to N — it must not be narrated as a result.
            let mut executed_verbs: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
            // Every verb+args signature that has already RUN. A round consisting only of
            // re-emissions of these is a stall round: it can produce no new ground.
            let mut ran_signatures: std::collections::BTreeSet<String> = std::collections::BTreeSet::new();
            let mut stalled_rounds = 0usize;
            let mut phantom_prods = 0usize;
            let mut round = 0;
            while round < max_rounds {
                let raw_calls = extract_tool_calls(&current);
                if raw_calls.is_empty() {
                    // Phantom-unavailability guard: the model can end a run by declaring a
                    // REAL verb "unavailable / does not exist" — without ever emitting its
                    // TOOL: line — and parking the node at B/N on that false premise. Refuse
                    // it: if the draft calls a real verb absent, prod it to actually run the
                    // verb before we accept the answer. Bounded so it can never loop.
                    let phantom = verbs_falsely_called_absent(&current);
                    if !phantom.is_empty() && phantom_prods < 2 {
                        phantom_prods += 1;
                        println!("── PHANTOM-VERB PROD (claimed a real verb absent — forcing the call) ──");
                        let are = if phantom.len() == 1 { "is a real structural verb" } else { "are real structural verbs" };
                        agent_msgs.push(("assistant".to_string(), current.clone()));
                        agent_msgs.push((
                            "user".to_string(),
                            format!(
                                "You wrote that `{}` is unavailable / does not exist / cannot be run. That is FALSE — `{}` {} in this engine, live right now. You may not report a verb as absent to avoid running it, and you may not leave a node at B/N on that false premise. Emit the `TOOL:` line(s) for it NOW with real catalog args (e.g. `TOOL: ascend K19_ray_class_field`, `TOOL: phase_reconstruct M1 M2 …`), read what they return, and fold that ground truth into your verdict.",
                                phantom.join("`, `"),
                                phantom.join("`, `"),
                                are,
                            ),
                        ));
                        let res = infer(llm, &agent_msgs, cli.max_tokens, cli.temperature);
                        current = strip_kernel_records(&res.text);
                        println!("{current}\n");
                        continue;
                    }
                    break; // the operator stopped acting — `current` is the answer
                }
                // Dedupe within the round: the operator emits the same call twice in one
                // batch (seen live: `anneal A B` listed twice in a single round). Running an
                // identical call twice just burns a request for a result already coming back.
                let mut seen_this_round = std::collections::BTreeSet::new();
                let mut calls: Vec<(String, Vec<String>)> = raw_calls
                    .into_iter()
                    .filter(|(v, a)| seen_this_round.insert(format!("{v} {}", a.join(" "))))
                    .collect();
                // Per-round ceiling: a runaway can emit dozens of calls in one round (seen: 87
                // imscribe calls building a name-ladder). Cap what runs per round so no single
                // round can flood; the loop continues, so legitimate large batches still finish
                // over successive rounds.
                const MAX_CALLS_PER_ROUND: usize = 24;
                if calls.len() > MAX_CALLS_PER_ROUND {
                    println!(
                        "── CAP: {} calls this round, running the first {} (emit the rest next round) ──",
                        calls.len(),
                        MAX_CALLS_PER_ROUND
                    );
                    calls.truncate(MAX_CALLS_PER_ROUND);
                }
                // Stall detection: nothing in this round is new — every call already ran and
                // returned. One repeat can be a legitimate re-check; whole rounds of nothing
                // else means the operator is circling, not navigating.
                if calls.iter().all(|(v, a)| ran_signatures.contains(&format!("{v} {}", a.join(" ")))) {
                    stalled_rounds += 1;
                    if stalled_rounds >= STALL_ROUNDS {
                        println!("── STALL ({STALL_ROUNDS} rounds of only already-run calls) — closing ──");
                        break;
                    }
                } else {
                    stalled_rounds = 0;
                }
                println!("── ACT round {} ({} tool call(s)) ──", round + 1, calls.len());
                let mut results = String::new();
                for (verb, args) in calls.iter() {
                    match run_structural_tool(verb, args) {
                        Some(o) => {
                            println!("● TOOL {verb} {}", args.join(" "));
                            print!("{o}");
                            results.push_str(&format!("### {verb} {}\n{o}\n", args.join(" ")));
                            executed_verbs.insert(verb.clone()); // this verb now has an execution arm
                            ran_signatures.insert(format!("{verb} {}", args.join(" ")));
                        }
                        None => {
                            // A real verb given bad/too-few args gets its correct form (so the
                            // eagle self-corrects); a genuinely unknown verb gets the real list.
                            // Collapsing both into one vague line made the eagle loop.
                            let m = format!("● TOOL {verb}: {}", tool_miss_message(verb, args));
                            println!("{m}");
                            results.push_str(&format!("{m}\n"));
                        }
                    }
                }
                all_tool_output.push_str(&results);
                agent_msgs.push(("assistant".to_string(), current.clone()));
                let executed_line = format!(
                    "EXECUTED VERBS so far (the only verbs you have a result for): {{{}}}. A claim about any \
                     verb NOT in this set has NO execution arm to fuse against — its outcome is N (neither), \
                     not a truth-value. Do not narrate a result (closure, conductance, material, modulus, tier) \
                     for a verb you did not run; if you need one, emit its TOOL: line and read what it returns.",
                    executed_verbs.iter().cloned().collect::<Vec<_>>().join(", "),
                );
                agent_msgs.push((
                    "user".to_string(),
                    format!(
                        "TOOL RESULTS (ground truth — never contradict these; introduce nothing they did not return):\n{results}\n\
                         {executed_line}\n\
                         UPDATE: if the task needs more steps, emit the next TOOL: line(s). If you now have everything, \
                         write your FINAL answer with NO TOOL: lines, grounded entirely in the results above. Your VERDICT \
                         must match the numbers: if every polymerize/close/arrange result terminated early or came back \
                         linear/telechelic (did NOT cyclize), the assembly does NOT close — do not call it closed, complete, \
                         a ring, or PROVED, and do not name an architecture the counts do not support (`1 unit / 0 bonds` is \
                         not a polymer, and nothing that terminated is `branched`, a `network`, or a `macrocycle`). Report \
                         the real result — that it does not close, and which arrangement (if any) the tools showed would. \
                         For each structural operation your answer relies on (excite, cyclize, forge, distill, …), state \
                         BRIEFLY what it means in BOTH registers — chemically AND mathematically — since the two are lossless \
                         faces of one act; the reading runs both ways, not chemistry with a math footnote."
                    ),
                ));
                let res = infer(llm, &agent_msgs, cli.max_tokens, cli.temperature);
                current = strip_kernel_records(&res.text);
                println!();
                println!("── OBSERVE/UPDATE round {} ──", round + 1);
                println!("{current}");
                println!();
                round += 1;
            }

            // If the loop ended mid-action (explicit --eagles leash, runaway backstop, or a
            // stall), force one clean grounded close. If it ended because the operator
            // stopped emitting tools, this never fires.
            if !extract_tool_calls(&current).is_empty() {
                agent_msgs.push(("assistant".to_string(), current.clone()));
                agent_msgs.push((
                    "user".to_string(),
                    format!(
                        "The run is closing (every remaining call was already run — you have its result above — or the \
                         round leash was reached). Give your FINAL answer now, with NO TOOL: lines, grounded only in the tool \
                         results above. You have a result ONLY for these executed verbs: {{{}}} — a claim about any other \
                         verb is N (neither), not a truth-value; do not narrate its outcome.",
                        executed_verbs.iter().cloned().collect::<Vec<_>>().join(", "),
                    ),
                ));
                let res = infer(llm, &agent_msgs, cli.max_tokens, cli.temperature);
                current = strip_kernel_records(&res.text);
                println!("── FINAL (forced close) ──");
                println!("{current}");
                println!();
            }

            // Bidirectional isomorphism key: for every verb that ACTUALLY ran, state what the
            // operation means in both registers — chemical AND mathematical. The engine ran one
            // act; it has two lossless faces, and the report closes by naming both so the reading
            // runs both ways. Grounded like everything else: only executed verbs are glossed.
            let iso: Vec<(String, &'static str, &'static str)> = executed_verbs
                .iter()
                .filter_map(|v| verb_isomorphism(v).map(|(c, m)| (v.clone(), c, m)))
                .collect();
            if !iso.is_empty() {
                println!("── ISOMORPHISM (what each operation means, both ways) ──");
                for (v, chem, math) in &iso {
                    println!("● {v}");
                    println!("   chemically:    {chem}");
                    println!("   mathematically: {math}");
                }
                println!();
            }

            // The final grounded answer feeds the spine and the multi-turn history.
            answer = current;
            if let Some(last) = conversation.last_mut() {
                last.1 = answer.clone();
            }
            // Grade the FINAL answer (not the draft) and let the tools speak: the spine now
            // fuses the model's stated verdict with what the catalog actually computed.
            model_voice = b4_from_char(model_self_belnap(&answer));
            tool_voice = tool_belnap(&all_tool_output);
        }

        let rep = complete(&prep, &answer, model_voice, tool_voice, cli.no_selectivity, cli.jam);
        print_spine(&rep, &prep, cli.verbose);

        if rep.fused == B4::F {
            last_code = last_code.max(1);
        }
    }
    last_code
}

fn interactive_loop(cli: &Cli, llm: &Llm, cat: Option<&[CatalogEntry]>) -> i32 {
    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  ASK interactive — full-length multi-turn (no Python)║");
    println!("╚══════════════════════════════════════════════════════╝");
    println!("Type your question (any length). End multi-line with a lone `.` line.");
    println!("Commands: /quit  /file <path>  /dry  /wet  /verbose  /help");
    println!("Model: {}", llm.model);
    println!();

    let mut conversation: Vec<(String, String)> = Vec::new();
    let mut dry = cli.dry_run;
    let mut verbose = cli.verbose;
    let stdin = io::stdin();
    let mut code = 0;

    loop {
        print!("ask> ");
        let _ = io::stdout().flush();
        let mut first = String::new();
        if stdin.lock().read_line(&mut first).ok().unwrap_or(0) == 0 {
            break;
        }
        let t = first.trim();
        if t.is_empty() {
            continue;
        }
        if t == "/quit" || t == "/exit" || t == "quit" || t == "exit" {
            break;
        }
        if t == "/help" {
            println!("  /file <path>  — load question from file and run");
            println!("  /dry | /wet   — toggle dry-run");
            println!("  /verbose      — toggle verbose");
            println!("  /quit         — leave");
            println!("  multi-line: paste lines, end with lone `.`");
            continue;
        }
        if t == "/dry" {
            dry = true;
            println!("dry-run on");
            continue;
        }
        if t == "/wet" {
            dry = false;
            println!("wet-run on (LLM)");
            continue;
        }
        if t == "/verbose" {
            verbose = !verbose;
            println!("verbose={verbose}");
            continue;
        }
        if let Some(rest) = t.strip_prefix("/file ") {
            match read_file_or_stdin(rest.trim()) {
                Ok((content, source)) => {
                    let c2 = cli.clone_with(dry, verbose);
                    code = run_one(&content, &source, &c2, llm, cat, &mut conversation);
                }
                Err(e) => eprintln!("error: {e}"),
            }
            continue;
        }

        // Multi-line if first line is `/` or content continues until `.`
        let mut q = String::new();
        if t == "/" {
            println!("(paste question; end with lone `.`)");
            loop {
                let mut line = String::new();
                if stdin.lock().read_line(&mut line).ok().unwrap_or(0) == 0 {
                    break;
                }
                if line.trim() == "." {
                    break;
                }
                q.push_str(&line);
            }
        } else {
            q.push_str(&first);
            // If user wants more lines starting already, allow optional continuation:
            // single-line by default unless they used `/`
        }
        let q = q.trim().to_string();
        if q.is_empty() {
            continue;
        }
        let c2 = cli.clone_with(dry, verbose);
        code = run_one(
            &q,
            &format!("interactive ({} chars)", q.chars().count()),
            &c2,
            llm,
            cat,
            &mut conversation,
        );
    }
    code
}

// Helper so we can toggle dry/verbose without re-parsing
trait CliClone {
    fn clone_with(&self, dry: bool, verbose: bool) -> Cli;
}
impl CliClone for Cli {
    fn clone_with(&self, dry: bool, verbose: bool) -> Cli {
        Cli {
            ask: self.ask.clone(),
            file: self.file.clone(),
            interactive: self.interactive,
            verbose,
            dry_run: dry,
            model: self.model.clone(),
            provider: self.provider.clone(),
            no_selectivity: self.no_selectivity,
            think: self.think,
            no_think: self.no_think,
            star: self.star.clone(),
            filter: self.filter.clone(),
            ascend: self.ascend.clone(),
            phase_reconstruct: self.phase_reconstruct.clone(),
            context: self.context.clone(),
            cycles: self.cycles,
            eagles: self.eagles,
            max_tokens: self.max_tokens,
            temperature: self.temperature,
            catalog: self.catalog.clone(),
            expand: self.expand,
            click: self.click.clone(),
            theta: self.theta,
            top: self.top,
            certify: self.certify,
            switch: self.switch.clone(),
            register: self.register.clone(),
            excite: self.excite.clone(),
            set: self.set.clone(),
            homolyze: self.homolyze.clone(),
            complement: self.complement.clone(),
            scan_mediators: self.scan_mediators,
            cycle: self.cycle.clone(),
            pathway: self.pathway.clone(),
            polymerize: self.polymerize.clone(),
            close: self.close,
            props: self.props,
            modulus: self.modulus,
            arrange: self.arrange,
            forge: self.forge.clone(),
            compare: self.compare.clone(),
            dope: self.dope.clone(),
            fuse: self.fuse.clone(),
            cleave: self.cleave.clone(),
            anneal: self.anneal.clone(),
            distill: self.distill.clone(),
            fdistill: self.fdistill.clone(),
            sublime: self.sublime.clone(),
            crystallize: self.crystallize.clone(),
            cocrystallize: self.cocrystallize.clone(),
            seed: self.seed.clone(),
            tlc: self.tlc.clone(),
            column: self.column.clone(),
            fpt: self.fpt.clone(),
            trap: self.trap.clone(),
            stain: self.stain.clone(),
            recall: self.recall.clone(),
            export: self.export.clone(),
            jam: self.jam,
            imscribe: self.imscribe.clone(),
            catalyst: self.catalyst.clone(),
            rest: self.rest.clone(),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    // Effective reasoning state: on by default (or per MODOT_THINK / --think), with --no-think
    // as an explicit override that always wins.
    let think = cli.think && !cli.no_think;
    let llm = make_llm(cli.model.as_deref(), cli.provider.as_deref(), think);

    let catalog_path = find_catalog(&cli);
    let catalog = match &catalog_path {
        Some(p) => match load_catalog(p) {
            Ok(c) => {
                if cli.verbose {
                    eprintln!("[catalog] {} ({} entries)", p.display(), c.len());
                }
                Some(c)
            }
            Err(e) => {
                eprintln!("[catalog] load failed: {e}");
                None
            }
        },
        None => {
            if cli.verbose {
                eprintln!("[catalog] not found — scaffold without IG hits");
            }
            None
        }
    };
    let cat_ref = catalog.as_deref();

    // Click-maths mode: `./ask --click A B` — fuse two fragments over the live pairs.
    if let Some(names) = &cli.switch {
        if names.len() == 2 {
            let code = click::run_switch(cat_ref, &names[0], &names[1], cli.certify);
            process::exit(code);
        }
    }

    // Single-electron transfer: `./ask --set D A` (donor, acceptor). `--scan-mediators`
    // ranks the catalog for relays; bare `--excite` makes it photoinduced.
    if let Some(names) = &cli.set {
        if names.len() == 2 {
            let code = if cli.scan_mediators {
                click::run_scan_mediators(cat_ref, &names[0], &names[1], cli.top)
            } else {
                let photo = cli.excite.is_some();
                click::run_set(
                    cat_ref,
                    &names[0],
                    &names[1],
                    cli.certify,
                    cli.catalyst.as_deref(),
                    photo,
                    cli.register.as_deref(),
                    catalog_path.as_deref(),
                )
            };
            process::exit(code);
        }
    }

    // Homolytic cleavage → neutral radicals: `./ask --homolyze A [B]`.
    if let Some(names) = &cli.homolyze {
        if !names.is_empty() {
            let code = click::run_homolyze(cat_ref, &names[0], names.get(1).map(|s| s.as_str()), cli.theta);
            process::exit(code);
        }
    }

    // Bidirectional ligand ⇌ catalytic-site complement: `./ask --complement A`.
    if let Some(name) = &cli.complement {
        let code = click::run_complement(
            cat_ref,
            name,
            cli.certify,
            cli.register.as_deref(),
            catalog_path.as_deref(),
        );
        process::exit(code);
    }

    // Catalytic cycle: `./ask --cycle CATALYST SUBSTRATE` — the closed loop.
    if let Some(names) = &cli.cycle {
        if names.len() == 2 {
            let code = click::run_cycle(
                cat_ref,
                &names[0],
                &names[1],
                cli.certify,
                cli.register.as_deref(),
                catalog_path.as_deref(),
            );
            process::exit(code);
        }
    }

    // Metabolic pathway: `./ask --pathway SUBSTRATE C1 C2 …` — chain the loops.
    if let Some(names) = &cli.pathway {
        if names.len() >= 2 {
            let code = click::run_pathway(cat_ref, &names[0], &names[1..], cli.certify);
            process::exit(code);
        } else {
            eprintln!("--pathway needs a substrate and at least one catalyst");
            process::exit(2);
        }
    }

    // Imscribe a missing entry: `./ask --imscribe NAME [free-text description]`.
    // Runs the real generate pipeline and writes to the live catalog.
    if let Some(name) = &cli.imscribe {
        let description = if cli.rest.is_empty() {
            name.replace('_', " ")
        } else {
            cli.rest.join(" ")
        };
        print!("{}", run_imscribe(name, &description));
        process::exit(0);
    }

    // Recall a registered material by name (no catalog work needed).
    if let Some(name) = &cli.recall {
        process::exit(click::run_recall(name, cli.export.as_deref()));
    }

    // Forge / compare / dope — the material-operation verbs (deterministic; no LLM).
    if let Some(names) = &cli.forge {
        process::exit(click::run_forge(cat_ref, names, cli.theta, cli.register.as_deref(), cli.export.as_deref()));
    }
    if let Some(names) = &cli.compare {
        process::exit(click::run_compare(cat_ref, names, cli.theta));
    }
    if let Some(names) = &cli.dope {
        process::exit(click::run_dope(cat_ref, names, cli.theta));
    }
    if let Some(names) = &cli.fuse {
        process::exit(click::run_fuse(cat_ref, names, cli.theta));
    }
    if let Some(names) = &cli.cleave {
        process::exit(click::run_cleave(cat_ref, names, cli.theta));
    }
    if let Some(names) = &cli.anneal {
        process::exit(click::run_anneal(cat_ref, names, cli.theta));
    }
    if let Some(names) = &cli.distill {
        process::exit(click::run_distill(cat_ref, names, cli.theta));
    }
    if let Some(names) = &cli.fdistill {
        process::exit(click::run_fdistill(cat_ref, names, cli.theta));
    }
    if let Some(name) = &cli.sublime {
        process::exit(click::run_sublime(cat_ref, name));
    }
    if let Some(names) = &cli.crystallize {
        process::exit(click::run_crystallize(cat_ref, names, cli.theta));
    }
    if let Some(names) = &cli.cocrystallize {
        process::exit(click::run_cocrystallize(cat_ref, &names[0], &names[1], cli.theta));
    }
    if let Some(names) = &cli.seed {
        process::exit(click::run_seed(cat_ref, names, cli.theta));
    }
    if let Some(names) = &cli.tlc {
        process::exit(click::run_tlc(cat_ref, names, cli.theta));
    }
    if let Some(names) = &cli.column {
        process::exit(click::run_column(cat_ref, names, cli.theta));
    }
    if let Some(names) = &cli.fpt {
        process::exit(click::run_fpt(cat_ref, names, cli.theta));
    }
    if let Some(names) = &cli.trap {
        process::exit(click::run_trap(cat_ref, &names[0], names.get(1).map(|s| s.as_str()), cli.theta));
    }
    if let Some(names) = &cli.stain {
        process::exit(click::run_stain(cat_ref, &names[0], &names[1..], cli.theta));
    }

    // Imscriptive polymerization: `./ask --polymerize M1 M2 …` — chain the clicks.
    if let Some(names) = &cli.polymerize {
        if names.len() >= 2 {
            let code = if cli.arrange {
                click::run_arrange(cat_ref, names, cli.theta, cli.certify, cli.close, cli.props, cli.modulus)
            } else {
                click::run_polymerize(cat_ref, names, cli.theta, cli.certify, cli.close, cli.props, cli.modulus)
            };
            process::exit(code);
        } else {
            eprintln!("--polymerize needs at least two monomers");
            process::exit(2);
        }
    }

    // Excited-state analysis: `./ask --excite A` (standalone verb — a value present
    // and no --set). On a --set line the flag is consumed above as photoinduced.
    if !cli.star.is_empty() {
        let code = click::run_star(cat_ref, &cli.star, cli.theta);
        process::exit(code);
    }

    if !cli.filter.is_empty() {
        let code = click::run_filter(cat_ref, &cli.filter);
        process::exit(code);
    }

    if let Some(name) = &cli.ascend {
        let code = click::run_ascend(cat_ref, name, cli.register.as_deref(), catalog_path.as_deref());
        process::exit(code);
    }

    if !cli.phase_reconstruct.is_empty() {
        let code = click::run_phase_reconstruct(cat_ref, &cli.phase_reconstruct, cli.theta);
        process::exit(code);
    }

    if let Some(ex) = &cli.excite {
        if !ex.is_empty() {
            let code = click::run_excite(
                cat_ref,
                ex,
                cli.certify,
                cli.register.as_deref(),
                catalog_path.as_deref(),
            );
            process::exit(code);
        }
    }

    if let Some(names) = &cli.click {
        let code = match names.len() {
            2 => click::run_click(
                cat_ref,
                &names[0],
                &names[1],
                cli.theta,
                cli.catalyst.as_deref(),
                cli.certify,
                cli.register.as_deref(),
                catalog_path.as_deref(),
            ),
            1 => click::run_click_sweep(
                cat_ref,
                &names[0],
                cli.theta,
                cli.catalyst.as_deref(),
                cli.top,
            ),
            _ => {
                eprintln!("--click takes one name (sweep) or two (pair)");
                2
            }
        };
        process::exit(code);
    }

    if cli.interactive
        && cli.ask.is_none()
        && cli.file.is_none()
        && cli.rest.is_empty()
    {
        let code = interactive_loop(&cli, &llm, cat_ref);
        process::exit(code);
    }

    // If only -i with also a question, run question then enter interactive? Keep simple: one-shot if ask/file/rest.
    match resolve_input(cli.ask.as_deref(), cli.file.as_deref(), &cli.rest) {
        Ok((content, source)) => {
            // Prepend any --context (files or directories) as a labeled background block.
            let (content, source) = match load_context(&cli.context) {
                Ok(ctx) if !ctx.is_empty() => (
                    format!("{ctx}\n========== SUBMISSION ==========\n{content}"),
                    format!("{source} + context[{}]", cli.context.len()),
                ),
                Ok(_) => (content, source),
                Err(e) => {
                    eprintln!("error: --context: {e}");
                    process::exit(2);
                }
            };
            let mut conversation = Vec::new();
            let code = run_one(&content, &source, &cli, &llm, cat_ref, &mut conversation);
            if cli.interactive {
                let _ = interactive_loop(&cli, &llm, cat_ref);
            }
            process::exit(code);
        }
        Err(e) => {
            if cli.interactive {
                let code = interactive_loop(&cli, &llm, cat_ref);
                process::exit(code);
            }
            eprintln!("error: {e}");
            eprintln!("Try: ask --ask \"...\" | ask --file path | ask -i");
            process::exit(2);
        }
    }
}
