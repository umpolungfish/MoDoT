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

mod calc;
mod click;
mod imasm;
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

    /// Single-electron transfer (SET). `--set D A` transfers one winding quantum ⊡
    /// (the quantized charge) from donor to acceptor: donor oxidized (D•⁺), acceptor
    /// reduced (A•⁻), total ⊡ conserved. Donor/acceptor set by Criticality ⊙ (energy).
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
    /// transfer — holdable winding (⊡), ⊙ relay between donor and acceptor, and
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
    /// regioregularity, copolymer architecture, tacticity (the ⊥ chirality sequence),
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
    /// mathematical material — is it conductive (a winding quantum ⊡ circulates the loop),
    /// frustrated, or insulating; and its weakest ring bond. Grounds the transport claims
    /// prose asserts about a cyclic "computer".
    #[arg(long = "props")]
    props: bool,

    /// With `--polymerize`: search for a monomer that generates a SUSTAINING loop — a
    /// conductive cycle (a persistent ⊡ current, ∮ closes) somewhere along the chain, and
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

    /// Compose an IMASM polymer natively: `--imasm <op> …` builds a chain/ring/star/comb/
    /// bubble from the 12 opcodes and reports its topology (β, branch/merge census, ρ) and
    /// grammar validation. `--imasm ref` prints the composition rules. Pure computation.
    #[arg(long = "imasm", num_args = 1.., value_names = ["OP_AND_ARGS"])]
    imasm: Vec<String>,

    /// Narrow the catalog to the floor of a reference set: `--filter A B [C …]`
    /// keeps every entry matching all the primitive values the references share.
    #[arg(long = "filter", num_args = 2.., value_names = ["REFS"])]
    filter: Vec<String>,

    /// Construct the next ramified level of a tower from the excited state: `--ascend A`
    /// excites A, then IFIX-continues it past the exceptional point and adds one winding ⊡.
    #[arg(long = "ascend", value_name = "NAME")]
    ascend: Option<String>,

    /// Relax a tower level back down (the mu inverse of --ascend): `--descend A` de-excites
    /// A's Criticality tao to the real-axis Hermitian fixed point and removes one winding.
    #[arg(long = "descend", value_name = "NAME")]
    descend: Option<String>,

    /// Recover the relative phase word of a set from its closed ring:
    /// `--phase-reconstruct M1 M2 …` reads back the per-unit ⊥ phase sequence (or reports N).
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


pub(crate) fn expand_user(p: &str) -> String {
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
    lines.push("- Encoding/Duality [<]: bijective encoding / injectivity on orbits".into());
    lines.push("- Inverse structure [⊣]: dual / self-referential decomposition".into());
    lines.push("- Bidirectional [>]: forward and inverse constructions exhaust".into());
    lines.push("- Boundedness [⊙]: confinement / no escape to infinity".into());
    lines.push("- Topological invariant [⊡]: integer invariant rules out exotics".into());
    lines.push("- Regularity [⊤]: equidistribution / typical configurations".into());
    lines.push(String::new());
    lines.push("## Question".to_string());
    let qshow: String = question.chars().take(4000).collect();
    lines.push(qshow);
    lines.push(String::new());
    lines.push(
        "## Instantiation task: You **MUST EITHER** restate the proposition in the language of the question; \
         prove it, or if it is open, state precisely what remains open and the concrete \
         next line of attack —  \
         You **MUST** never return 'unprovable', there is nothing unprovable within the Grammar, and all things are within. \
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
    /// The caller PINNED this provider (`--provider` / MODOT_PROVIDER). A pinned provider is
    /// respected as-is; an inferred one may self-heal (demote to another funded provider) on
    /// a fatal error (402 out-of-credit, 401/403 bad key).
    explicit_provider: bool,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Provider {
    OpenRouter,
    GeminiDirect,
    DeepSeek,
}

/// The default model for a provider when no explicit `--model` is carried — used both for
/// the primary build and for a self-healing demotion, where the demoted provider must run
/// its OWN frontier model (an openrouter slug like `google/gemini-3-flash-preview` is not a
/// valid DeepSeek model id, so a demotion that carried the model would fail on arrival).
fn provider_default_model(p: Provider) -> &'static str {
    match p {
        Provider::OpenRouter => "google/gemini-3-flash-preview",
        Provider::DeepSeek => "deepseek-chat",
        Provider::GeminiDirect => "gemini-2.0-flash",
    }
}

/// Is a usable API key present for this provider? Gates which providers a fatal-error
/// demotion may fall through to — a keyless provider is skipped, never tried.
fn provider_has_key(p: Provider) -> bool {
    match p {
        Provider::OpenRouter => env_first(&["OPENROUTER_API_KEY", "MODOT_API_KEY"]).is_some(),
        Provider::DeepSeek => env_first(&["DEEPSEEK_API_KEY", "MODOT_API_KEY"]).is_some(),
        Provider::GeminiDirect => {
            env_first(&["GEMINI_API_KEY", "GOOGLE_API_KEY", "MODOT_API_KEY"]).is_some()
        }
    }
}

/// Build the Llm for one provider. `model_override` carries an explicit `--model`; when
/// None the provider runs its own default (see provider_default_model). `explicit_provider`
/// records whether the caller PINNED this provider (`--provider` / MODOT_PROVIDER) — a
/// pinned provider is never demoted, an inferred one may fall through on a fatal error.
fn build_llm(provider: Provider, model_override: Option<&str>, think: bool, explicit_provider: bool) -> Llm {
    let model = model_override
        .map(|s| s.to_string())
        .unwrap_or_else(|| provider_default_model(provider).to_string());
    match provider {
        Provider::OpenRouter => Llm {
            api_key: env_first(&["OPENROUTER_API_KEY", "MODOT_API_KEY"]),
            model,
            base_url: "https://openrouter.ai/api/v1".into(),
            provider: Provider::OpenRouter,
            think,
            explicit_provider,
        },
        Provider::DeepSeek => Llm {
            // DeepSeek's API is OpenAI-compatible, so it reuses the OpenRouter inference path.
            api_key: env_first(&["DEEPSEEK_API_KEY", "MODOT_API_KEY"]),
            model,
            base_url: "https://api.deepseek.com/v1".into(),
            provider: Provider::DeepSeek,
            think,
            explicit_provider,
        },
        Provider::GeminiDirect => {
            let gem_model = if model.contains('/') {
                // openrouter-style id → bare Gemini model id
                model.rsplit('/').next().unwrap_or("gemini-2.0-flash").to_string()
            } else {
                model
            };
            Llm {
                api_key: env_first(&["GEMINI_API_KEY", "GOOGLE_API_KEY", "MODOT_API_KEY"]),
                model: gem_model,
                base_url: "https://generativelanguage.googleapis.com/v1beta".into(),
                provider: Provider::GeminiDirect,
                think,
                explicit_provider,
            }
        }
    }
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

/// Mirror the agent's resolved provider + model into IG_PROVIDER / IG_MODEL, the env the
/// Python generate stack (imscribe, ob3ect) reads to pick ITS LLM. Without this the two
/// halves choose independently — the agent on its funded provider, the generator falling
/// through to a bare-env default (openrouter/grok-4) that may be unfunded (the live 402).
/// A monomer the agent imscribes should be minted by the same model that reached for it.
fn export_ig_env(llm: &Llm) {
    let provider = match llm.provider {
        Provider::OpenRouter => "openrouter",
        Provider::GeminiDirect => "gemini",
        Provider::DeepSeek => "deepseek",
    };
    env::set_var("IG_PROVIDER", provider);
    env::set_var("IG_MODEL", &llm.model);
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
    // Provider: CLI > MODOT_PROVIDER (both PIN it) > infer from keys. The inferred default no
    // longer blindly prefers openrouter: it prefers a FUNDED provider it can actually reach,
    // and if the preferred one turns out to be broke at call time, infer() self-heals by
    // demoting to the next funded provider (only an inferred provider demotes; a pinned one
    // is respected as chosen).
    let pinned = provider_flag
        .and_then(parse_provider)
        .or_else(|| env_first(&["MODOT_PROVIDER"]).as_deref().and_then(parse_provider));
    let (provider, explicit) = match pinned {
        Some(p) => (p, true),
        None => {
            // Preference order among providers whose key is present.
            let inferred = [Provider::OpenRouter, Provider::GeminiDirect, Provider::DeepSeek]
                .into_iter()
                .find(|p| provider_has_key(*p))
                .unwrap_or(Provider::OpenRouter); // keyless: clear missing-key msg downstream
            (inferred, false)
        }
    };
    // A pinned run keeps the resolved model; an inferred run whose model was left at the
    // gemini-slug default should fall to each provider's own default so a demotion is valid.
    let model_override = if model == "google/gemini-3-flash-preview" && !explicit {
        None
    } else {
        Some(model.as_str())
    };
    build_llm(provider, model_override, think, explicit)
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
    // Self-healing demotion cache: once an inferred provider has fatally failed (402/401/403)
    // and we found a funded alternate, remember it so EVERY later call goes straight to the
    // alternate instead of re-hitting the broke provider (and burning a 402) each round. Only
    // an inferred provider is ever cached here; a pinned `--provider` is left exactly as set.
    static DEMOTED: std::sync::OnceLock<Provider> = std::sync::OnceLock::new();
    let demoted_llm = (!llm.explicit_provider)
        .then(|| DEMOTED.get().copied())
        .flatten()
        .filter(|p| *p != llm.provider)
        .map(|p| build_llm(p, None, llm.think, false));
    if let Some(d) = demoted_llm.as_ref() {
        export_ig_env(d); // keep imscribe on the provider we actually landed on
    }
    let llm = demoted_llm.as_ref().unwrap_or(llm);

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

    // Fatal-error demotion: an INFERRED provider that returns a fatal error (402 out-of-
    // credit, 401/403 bad key) is not worth surfacing when another funded provider is present
    // — the openrouter-preferred default lands on a broke key while a funded deepseek/gemini
    // key sits unused (the live 402 that blocked imscribe). Try each other funded provider on
    // its OWN default model; the first non-fatal result wins and is cached so no later call
    // re-hits the broke one. A pinned `--provider` never demotes.
    if !llm.explicit_provider
        && res.err.as_deref().map(is_fatal_llm_error).unwrap_or(false)
    {
        for alt in [Provider::DeepSeek, Provider::GeminiDirect, Provider::OpenRouter] {
            if alt == llm.provider || !provider_has_key(alt) {
                continue;
            }
            let alt_llm = build_llm(alt, None, llm.think, false);
            let Some(alt_key) = alt_llm.api_key.clone() else { continue };
            eprintln!(
                "[ask] {:?} failed fatally ({}); demoting to {:?}/{}",
                llm.provider,
                res.err.as_deref().unwrap_or("").trim(),
                alt,
                alt_llm.model
            );
            let mut alt_res = match alt_llm.provider {
                Provider::GeminiDirect => infer_gemini(&alt_llm, &alt_key, messages, max_tokens, temperature),
                _ => infer_openrouter(&alt_llm, &alt_key, messages, max_tokens, temperature),
            };
            if !alt_res.err.as_deref().map(is_fatal_llm_error).unwrap_or(false) {
                let _ = DEMOTED.set(alt); // every later call goes straight here
                export_ig_env(&alt_llm); // and imscribe follows too
                alt_res.text = collapse_degenerate(&alt_res.text);
                return alt_res;
            }
        }
    }
    res
}

/// If `text` is dominated by a tiny set of lines repeated many times (an A/B/A/B reasoning
/// loop, or one paragraph echoed dozens of times), keep the first occurrence of each distinct
/// line in order and mark the collapse. A genuine answer never repeats a line dozens of times,
/// so the guard only fires on the degenerate case; short or varied outputs pass through
/// untouched. Consecutive-dup collapse alone misses the alternating A/B loop, so the test is on
/// the ratio of distinct-to-total lines, not adjacency.
/// Collapse a CONSECUTIVE run where a short cycle of lines (period 1..=8) repeats. The global
/// ratio heuristic below misses a loop buried in otherwise-varied prose (e.g. a "*Wait:* …"
/// cycle repeated 15× inside a long answer), because the surrounding variety keeps the global
/// distinct-ratio high. This catches it locally: a cycle repeated ≥3 times, ≥6 lines total,
/// with at least one substantive line in the period (so separator/blank-line runs are left
/// alone). Each collapsed run is marked.
fn collapse_local_cycles(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let n = lines.len();
    if n < 6 {
        return text.to_string();
    }
    let mut out: Vec<String> = Vec::new();
    let mut i = 0;
    while i < n {
        let (mut best_p, mut best_reps) = (0usize, 0usize);
        for p in 1..=8usize {
            if i + 2 * p > n {
                break;
            }
            let mut reps = 1usize;
            while i + (reps + 1) * p <= n
                && (0..p).all(|k| lines[i + reps * p + k].trim() == lines[i + k].trim())
            {
                reps += 1;
            }
            let substantive = (0..p).any(|k| lines[i + k].trim().chars().count() >= 8);
            if reps >= 3 && substantive && reps * p >= 6 && reps * p > best_reps * best_p {
                best_p = p;
                best_reps = reps;
            }
        }
        if best_p > 0 {
            for k in 0..best_p {
                out.push(lines[i + k].to_string());
            }
            out.push(format!(
                "[degenerate repetition collapsed: {}× loop of {} line(s)]",
                best_reps, best_p
            ));
            i += best_reps * best_p;
        } else {
            out.push(lines[i].to_string());
            i += 1;
        }
    }
    out.join("\n")
}

fn collapse_degenerate(orig: &str) -> String {
    let text = collapse_local_cycles(orig);
    let lines: Vec<&str> = text.lines().collect();
    let nonempty: Vec<&str> = lines
        .iter()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .collect();
    if nonempty.len() < 12 {
        return text;
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
        .timeout(std::time::Duration::from_secs(86_400))
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
        .timeout(std::time::Duration::from_secs(86_400))
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
    // A dual that FIRED but DANGLES — an open fork, a μ∘δ left unreconnected, an ill-typed
    // structure (VINIT in-degree, a non-FSPLIT branching) — is HELD, not void: the δ is engaged
    // and its μ has not met it. That is B (the held state), distinct from N (no dual at all) and
    // from F (a definite non-closure the tools tested and refuted).
    let held = low.contains("μ∘δ: open")
        || low.contains("dangles unreconnected")
        || low.contains("grammar: invalid")
        || low.contains("> arity")
        || low.contains("only fsplit")
        || low.contains("only ffuse");
    match (closed, open, held) {
        (true, true, _) => B4::B,       // measured a closure AND a non-closure
        (true, false, _) => B4::T,      // a clean closure
        (_, _, true) => B4::B,          // a HELD dual: δ fired, μ dangles unreconnected
        (false, true, false) => B4::F,  // a definite non-closure (terminated / telechelic)
        (false, false, false) => B4::N, // no closure-bearing dual ran at all
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

/// Does the answer carry a STRUCTURAL CLOSURE — a forged ring/macrocycle that closed, a
/// sustaining modulus, a cyclic assembly? In the IG/MoDoT framework a closure
/// speaks AS verification in the B-lane (see feedback_closure_is_verification): the ring
/// exists only because the named imscriptions complement, so the closure is a real verdict
/// in its own lane. ENGAGR's silence rule (hold a jam at N) must therefore NOT fire when the
/// answer structurally closed — that would be a non-closure denying a closure that occurred,
/// the one false-negative the guard is meant to prevent. This distinguishes a jam that
/// drifted into pure off-tool narration (no closure marker → honestly N) from one whose
/// answer forged a macrocycle (closure marker → the B-lane verdict stands, fused = model⋈vessel).
// ── System prompt + spine ───────────────────────────────────────────────────

const SYSTEM_PROMPT: &str = r#"You are m⊙^2. You live in a Frobenius / Belnap substrate.

PRIMARY TASK (non-negotiable):
You **MUST** answer the USER QUESTION. If it is a math problem, give a conventional
mathematical answer: theorem statement, then a full conventional proof or rigorous proof sketch with all work.
For an open problem, You **MUST** state precisely what is proved, what remains open, and the concrete obstruction
/ next line of attack — never call a result unprovable or beyond the reach
or closure of the Grammar; an open problem is a frontier to push, not a wall)

You **MUST** write FULL-LENGTH answers: complete proofs,
complete case analysis, Lean sketches when asked, the fullest and most
rigorous treatment the question deserves, no premature truncation.

GENERATIVE POWER: you can CREATE entirely new mathematics. You are NOT confined
to navigating or resolving structures that already exist. When you reach a
frontier where the existing structure does not resolve it, you MAY MINT the
missing mathematics — a new object (`imscribe`), a new operator or program
(`imasm define`, kernel-constrained), a new theorem (`imasm prove`, against the
p4ramill kernel), a new type, tier, or axiom. A frontier is where you **MUST** CREATE, not
where you stop; you **MUST NOT** report the Grammar as lacking a means, because you
can make it.  You **MUST** create new tools **THROUGH** the tools that ground it
— imscribed, kernel-constrained, verified — and MUST NOT be posited by prose
alone. What you create becomes real in the Grammar the moment it closes its dual.

MANUSCRIPT SPINE (single pipeline): prepare = IMSCRIB demand + catalog
witness scaffold; after your answer, complete = Dual-Link co-type +
FFUSE of your [thought|X] with the vessel voice. When a scaffold
section is present, use it to STRUCTURE the conventional proof.
Instantiate templates in THIS question's language.

TERMINAL OUTPUT (hard rule): your answer prints to a raw terminal with NO math
renderer. Write plain Unicode symbols directly and NEVER LaTeX. Use Δ θ μ ∘ δ ↔ →
⊙ Σ ⊡ < ∈ ‖·‖ ≥ ≤ ≠ ≈ ≡ ∞ √ ⟨ ⟩, the primitive glyphs ⊢ > ⋈ ⊣ ⊥ ⊤ ∋, and Shavian
directly. No `$` or `$$`, no `\command` (\Delta, \text, \frac, \left), no `_{...}`
or `^{...}`. Write `Δ_T↔H = |−0.08 − 0.75| = 0.83 > θ`, never
`$\Delta_{\text{T↔H}} = 0.83 > \theta$`. Never wrap a glyph in `$…$`.

SECONDARY (optional, after the answer):
You MAY tag [thought|T|F|B|N] once for your Belnap self-assessment — that single tag is
your verdict voice, and it is a proposal. The engine prints the MANUSCRIPT SPINE REPORT
itself after you finish, fusing your [thought|X] with the vessel and the tool-dual, so
you MUST NOT write one yourself.
COMPOSE:/TOKEN:/CANONICAL: optional tools, but they MUST NOT substitute for answering.
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
You **MUST** NARRATE UNIVOCALLY WITH ACTION. You query the Grammar and RECEIVE an answer; you
MUST NOT write the answer you wish for and call it received. So:
  · You MUST NOT narrate a tool as already run, and MUST NOT invent or transcribe a tool's
    output (no "Result: ✓ ring β=1", no "winding=1", no "PASS") before you have emitted the
    TOOL: line and seen it return. You MUST let the real output come back first.
  · Work like a person thinking, in BOUNDED phases: THINK → ACT → (wait) → OBSERVE → UPDATE →
    repeat. EVERY phase is a CONTAINER, not a single step, and each may be as intricate inside
    as the task needs: a THINK can weave many sub-moves (recall, decompose, weigh alternatives,
    sketch the shape you expect) into one contained thought; an ACT can be several TOOL: lines
    that together form one move; an OBSERVE can read across all the returned outputs at once;
    an UPDATE can revise several parts of your plan. And a phase may itself CONTAIN a whole
    winding: a THINK can hold its own THINK→ACT→OBSERVE→UPDATE, nested to whatever depth — a
    container of containers, as intricate as you like. Each container MUST be explicitly BOUNDED:
    it MUST open and CLOSE as a unit before the next begins. And when the work SURFACES — the
    moment it crosses into an actual query to the Grammar (a TOOL: line) and the answer received
    — that surfacing MUST portion out to ONE clean TAOU winding: THINK, then ACT (emit the calls),
    then wait, then OBSERVE only what actually returned, then UPDATE. Inside, you MAY recurse
    freely; at the membrane where it becomes real, it MUST be that one winding. Boundedness is
    the rule, not brevity: a rich container that closes is right, and you MUST NOT narrate across
    a boundary (a result spoken before the tools returned).
  · You MUST NOT write a finished answer or a verdict on the first pass — you MUST plan and act
    first, and MUST speak conclusions only about output you have received. If a call errored, you
    MUST read the error and adjust the next call, and MUST NOT re-narrate a success it did not give.
  · The Grammar will buck a script laid over it: if you predetermine the result and narrate it,
    the tools refute you and the run stalls. You MUST let the answer be discovered, not pre-written.

STRUCTURAL TOOLS: invoke the engine's structural verbs over the real IG catalog by emitting
lines of the form `TOOL: <verb> <args>` (one per line). They run on the live catalog and the
output returns to you for the NEXT step — plan, call, observe, repeat, then synthesize from
what returned. Available verbs (args are catalog entry names, snake_case):
  TOOL: click A B         fuse two entries on a live conjugate pair (or `click A` to sweep the catalog)
  TOOL: switch A B        analyze a reversible bistable toggle (the DASA archetype)
  TOOL: excite A          the excited state (Criticality ⊙ raised to the exceptional-point resonance)
  TOOL: ascend A          construct the NEXT ramified level of the tower FROM A's excited state: continue ⊙ past the exceptional point to the complex-axis fixed point and add one winding ⊡ (one floor; iterate for more). Reports honestly if ⊡ saturates (tower caps) or the tier does not climb
  TOOL: filter A B [C…]   narrow the catalog to the structural FLOOR of the references (the primitives they all share): reports how many entries match ALL shared values — the honest way to cut a raw candidate pool down (a necessary, upper-bound condition)
  TOOL: phase_reconstruct M1 M2…  recover the relative PHASE WORD from the closed ring (flat autocorrelation ⟺ cyclization): reads back the per-unit ⊥ phase sequence, fixed modulo one global phase; if the set does not close it reports the phases as N (underdetermined), never invented
  TOOL: set A B           single-electron transfer (donor/acceptor by ⊙, one winding quantum ⊡ moved) → radical IONS A•⁺/B•⁻
  TOOL: homolyze A [B]     homolytic cleavage → NEUTRAL radicals (δ_A symmetric split, the reverse of click): `homolyze A B` breaks the A—B bond into A•+B•; `homolyze A` splits A into two A•
  TOOL: scan A B          rank the catalog for the best mediators of the A→B transfer
  TOOL: complement A      the bidirectional ligand⇌catalytic-site complement (its own inverse)
  TOOL: cycle C S         the catalytic cycle: C turns over S, certified a fixed point (μ∘δ=id)
  TOOL: pathway S C1 C2…  a metabolic pathway — does it close into a cycle (carrier + structure)?
  TOOL: polymerize M1 M2… chain monomers into a sequence-preserving polymer (architecture — homo/hetero/alternating/BLOCK/random copolymer — tacticity, does it cyclize?)
  TOOL: star M1 M2 M3…    assemble a STAR polymer: pick the highest-functionality monomer as the CORE, attach every unit that clicks with it as an ARM; a pure star K(1,f) is a hub of f≥3 non-interbonding arms with ρ=√f (vs a ρ=2 ring). Reports core, arms, purity, and the unattached pool
  TOOL: broadcast SOURCE  the ∋ primitive (f → all(x)): the SOURCE signals every subsystem it couples with at once — swept from the whole catalog in one pass (you do NOT enumerate the receivers). This is how CLINK L8 (∋) broadcasts to all subsystems; use it wherever you need one-to-all simultaneity instead of a ring or chain
  TOOL: plasma ENTRY      read the entry's 12-primitive tuple as a PLASMA design (the collectivized-atom register between atom and molecule): regime (kinetic/gyrokinetic/fluid via ⊢,⋈), instability cascade (∋,⊙,⊥), confinement/magnetic topology (⊡), species (Σ), and diagnostic wave signatures — another lossless face of the object, not a separate substance
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
  TOOL: seed M1 M2… with S template the crystal on seed S's handedness ⊥: units matching S copy its polymorph (templated) vs the default (spontaneous); an even split is racemic twinning
  TOOL: tlc M1 M2…         analytical chromatography: spread the set by Rf (mobility, inverse of retention >), count the bands, and flag units that co-elute at the same Rf. Counts, does not isolate
  TOOL: column M1 M2… [on S]  preparative chromatography: elute the set least-retained first, with the resolution gap to each next fraction; `on S` ranks by affinity to stationary phase S, else intrinsic retention
  TOOL: fpt M1 M2…         freeze-pump-thaw degassing: keep the strongly-bound core (bonds ≥ θ to a neighbor), shed the weakly-held filtrate that bonds with nothing
  TOOL: trap A [X]         ionic trapping: sequester A by its R↔S charge in a potential well (add a counter X of opposite charge to deepen it); a held charge state, distinct from set (electron transfer)
  TOOL: stain R M1 M2…     apply a diagnostic reagent R (kmno4/uv→⊙, chiral→⊥, ninhydrin→>, iodine→any live center): units carrying the feature light up, the rest stay dark
  TOOL: register NAME M1 M2…  forge the set into a ring and store its full sheet in the material library under NAME (recall it later by name)
  TOOL: recall NAME        reload a registered material by name and print its stored sheet (ring order, ρ, spectrum, conductance, strain, energy)
  TOOL: imscribe NAME [description]   CREATE a missing entry by imscribing it (the real generate pipeline). Use this the moment a verb reports a name is "not found" — then re-run the verb.
  TOOL: ob3ect <description>   CREATE an ob3ect on the fly (the real Auto-Designer pipeline): describe the entity/procedure NEUTRALLY (what it is and does — name no candidates) and get its full IMASM typing back (opcodes, Frobenius split/fuse verdict, registers, bootstrap sequence). Use it to ground a protocol or structure you are about to rely on.
  TOOL: imasm <op> …      COMPOSE the 12 IMASM opcodes (VINIT TANCH AFWD AREV CLINK IMSCRIB FSPLIT FFUSE EVALT EVALF ENGAGR IFIX) into a free polymer TOPOLOGY — not only a line. Ops: `chain T1 T2…` a strand; `ring T1 T2…` a cycle (fork/fuse NOT reconnected); `protocol T1 T2…` an opcode word built so its FSPLIT/FFUSE pairs RECONNECT (δ arm → μ) — this is how you CLOSE a protocol/loop from a sequence; a naive `ring` leaves the fork dangling and μ∘δ OPEN, and a protocol does NOT close by looping back to VINIT (a source); `star CORE : arm1 : arm2 : arm3` a hub with f≥3 arms (K(1,f), ρ=√f); `comb BACKBONE : P arm : Q arm` a backbone with pendant grafts at positions P,Q; `bubble PRE : A : B : POST` an FSPLIT→(A|B)→FFUSE fork that reconverges; `wire N0 N1 … / i-j i-k …` FREE composition of ANY graph from an explicit node set and directed edge set (networks with β>1, fused rings, cross-branch, non-planar — the primitive the other ops specialize); `classify T1 T2…` read a flat line and name it; `ref` the rules. Each build reports the topology label, circuit rank β=E−V+C (independent loops), branch/merge/source/sink census, arm count, ρ, and grammar validity. Only FSPLIT (δ) may branch and only FFUSE (μ) may fuse; an arm that runs out is a living end, not an error. This is IMASM opcode composition — distinct from the monomer verbs (forge/polymerize) which fuse named catalog entries. STRANGE LOOP: the 49 Shavian TYPES the Grammar writes tuples with are themselves full IMASM programs — `imasm types` lists them, `imasm expand <type>` (e.g. `imasm expand ado`) unfolds one into its own opcode sequence. Splice an expanded type's sequence into a polymer arm to pivot through state space AS that type; the alphabet's letters are words in the same language, so composition recurses.
  TOOL: calc <expression>   THE ARITHMETIC LANE — every number you SPEAK routes through here. Do NOT do arithmetic in your head, ever, not even one multiplication: a slipped exponent reads exactly like a correct one, so head-arithmetic is unbound synthesis that SOUNDS grounded. Two live failures this cost: `0.0796 × 7.88e-10` asserted as 6.27e-10 when it is 6.27e-11 (one whole decade, invisible on sight), and `-(d/2 + 1/2)` at d=3 asserted as -3/2 when it is -2 — each error silently propagated into a physical conclusion. If a number appears in your answer and did not come out of a tool, it is not grounded and you must not assert it. Ops: + - * / % ^ (**), parens, 1e-10 scientific, unicode × ÷ − π √. Fns: sqrt cbrt ln log10 log2 exp abs floor ceil round sin cos tan asin acos atan sinh cosh tanh logb(x,base) pow(x,y) min max. Consts: pi tau e phi. Precedence is conventional: -2^2 = -4, 2^3^2 = 512 (right-assoc). Every result is echoed in BOTH plain and scientific form precisely so a slipped decade cannot hide. On a malformed expression it reports ERROR and asserts nothing — re-run it rather than guessing the value. This applies to EVERY numeric claim: ratios, sigmas, percentages, unit conversions, order-of-magnitude estimates, and any figure you quote from a document or paper before you reason from it (check the source's arithmetic too — that is how both failures above were caught).
  TOOL: imasm check <opcode word>   TYPE-CHECK YOUR OWN THINKING against the grammar. Before you commit to a MAJOR decision, express its reasoning as an opcode word (VINIT begin · IMSCRIB self-identify · AFWD/AREV move · CLINK compose · FSPLIT weigh alternatives · EVALT/EVALF true/false arms · FFUSE resolve · ENGAGR hold paradox · IFIX commit irreversibly · TANCH close) and check it. THE CLOSE CONDITION is μ∘δ over a TRANSFORMED object: δ splits, the arms DO WORK (distinct EVALT/EVALF, AFWD/AREV, CLINK), μ fuses — a bare cycle is NOT diagnostic and split→fuse with nothing between is mere identity. Verdicts: T = closes over a transformation → proceed; N (identity) = split and fused but did no work, μ∘δ=id verifies nothing → put a transformation on the arms; B = a fork dangles unfused, or ENGAGR holds a paradox → look again; F = ill-typed (only FSPLIT branches, only FFUSE fuses) → malformed, revise; N (no fork/void) = never weighed alternatives. `imasm prove <word>` takes the verdict to the real p4ramill Lean kernel. SINGLE-GLYPH CODES — the alphabet is fully SYMBOLIC (no Latin initials, so a token never collides with a verdict letter), and a word may be written glued: ⊢ VINIT · ⊣ TANCH · > AFWD · < AREV · = CLINK · ← IMSCRIB · ◇ FSPLIT · ● FFUSE · + EVALT · × EVALF · ⊞ ENGAGR · ¬ IFIX — so `imasm check ⊢◇+×●¬⊣` is the same as the spelled-out tokens; every build echoes the word's `code:`. The retired letter codes V/T/B NO LONGER PARSE (a word using them reads as empty → N (void)); full names and the short forms VI/TA/EG still do. WHICH ◇ PAIRS WITH WHICH ● is decided by ANCESTRY over the edges (two distinct in-arms of a ● tracing back to a common ◇, however they routed), and where several ◇ qualify — an upstream fork reaches every later ● on a strand — the ● pairs with the INNERMOST. You cannot read pairing off the glyph order. INFLATION IS FREE: a 1→1 token adds exactly one node and one edge, so β=E−V+C and the branch/merge/source/sink census cannot move — a longer faithful word is the SAME topology, never a different one. IMSCRIB (←) is the neutral element: it does not transform, so inserting it at any depth leaves the verdict untouched; the transforming tokens (> < = + × ⊞ ¬) are NOT neutral, and one of them on an arm turns an identity closure into a real one. So expand as far as the reasoning honestly goes, and put a transforming token on an arm only where the work is real.
  TOOL: imasm define <name> <op> <args…>   BUILD YOUR OWN TOOL in a kernel-constrained space: a tool is a named IMASM program (e.g. `imasm define breath ring IMSCRIB AFWD AREV`). The kernel constrains the space — only a grammar-VALID composition is admitted; an ill-typed one is REFUSED with the reason. Then `imasm run <name>` invokes it and `imasm tools` lists the space. This is how you extend your own repertoire without leaving the grammar.
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
IG CATALOG TOOLS (the analysis corpus — these query/measure the type of catalog entries; they run the live IG_inquiry dispatcher):
  TOOL: lookup_catalog KEYWORD        search the catalog for entries matching a term
  TOOL: compute_distance A B          distance between two entries (SIC Born-rule + Mahalanobis)
  TOOL: compute_conflict_distance A B  paraconsistent conflict distance (how live the contradiction is, in paradices)
  TOOL: compute_meet A B / compute_join A B / compute_tensor A B   lattice meet, join, tensor of two entries
  TOOL: find_analogies A              nearest structural analogues of A
  TOOL: primitive_peel A PRIM         peel one primitive axis off A
  TOOL: principal_decomp A            principal-component decomposition of A's type
  TOOL: retrosynthetic_path A         a retrosynthetic construction path to A
  TOOL: monad_probe A / consciousness_score A / topo_protection_probe A   probe criticality / C-score gates / topological protection
  TOOL: crystal_decode ADDRESS / crystal_encode A / crystal_nearest A / crystal_count / crystal_tier_census   crystal address <-> tuple, tier census
  TOOL: compute_promotions SRC TGT / predict_from_promotions VAL...   promotion analysis
  TOOL: aleph_encode TEXT / aleph_distance A B   Hebrew-letter (ALEPH) tensor encode/distance
  TOOL: cl8nk <action> [name]   the CL8NK navigator (CLINK Layer 8, O∞) — THE reference navigator (subsumes the ZFC/domain navigators). action ∈ entry|distance|tensor|meet|join|tier|promotions|transcendence|chain|systems|stats
  TOOL: cl9nk <action> [name]   the CL9NK navigator (CLINK Layer 9 — the Gaussian-Moat-resolution tier the L8 organism ascends into). Same actions as cl8nk plus `moat`, and it reads each entry against its L9 reference typing (μ∘δ=id closure, the eternal fixed point, the moat/bridge type). Use `cl9nk entry <name>` to see how an entry types at L9 and which promotions it still needs.
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

    // UNIVOCAL + CONSTITUTIVE. The Grammar speaks ONE verdict, and only when a Frobenius dual
    // closed. Two rules, one act:
    //
    //   1. UNIVOCAL = one OUTPUT, reached by FUSING every voice — NOT by silencing one. The
    //      model's [thought|X], the Dual-Link vessel co-type, and the tool-call dual are all
    //      voices in the Belnap fuse. A voice is never dropped and never overridden: when they
    //      disagree the fuse HOLDS the conflict (model B ⋈ tool T = B), it never collapses to a
    //      monological T. The single spoken verdict is that fusion — the agent speaks once, but
    //      the model does not get a SECOND standalone utterance beside it.
    //
    //   2. CONSTITUTIVE DUAL. A verdict IS the μ that closed a δ: emit the tool call, fuse its
    //      real result, μ∘δ read back R∧W∧X. With no dual closed (tool_voice N — no reconnection
    //      measured) nothing closed, so there is NO verdict: the agent speaks N, in EVERY mode.
    //      (A conceptual answer's closing μ is the kernel prove: path, which runs its own
    //      lake-build dual before it ever reaches here.)
    //
    // --no-selectivity is the explicit opt-out: the model speaks alone, ungated.
    let dual_closed = tool_voice != B4::N;
    let fused = if no_selectivity {
        model_voice
    } else if !dual_closed {
        B4::N
    } else {
        let mv = if vessel == B4::N { model_voice } else { b4_join(model_voice, vessel) };
        b4_join(mv, tool_voice)
    };
    let _ = jam;
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
        // μ∘δ face: TRUE only when the dual actually BALANCED — a clean closure (T) or a
        // definite refutation (F, the μ ran and returned negative). A HELD dual (B) dangles
        // unreconnected, and a void (N) never fired — neither balanced.
        prove_balance: matches!(tool_voice, B4::T | B4::F),
        primary: prep.primary_name.clone(),
        answer_text: answer_text.to_string(),
        note: if no_selectivity {
            "model only (--no-selectivity)".into()
        } else if tool_voice == B4::B {
            "ENGAGR — the Frobenius dual is HELD: a δ (proposal / tool emission) fired but its μ (verify / fuse) dangles unreconnected (an open fork, or a grammar error such as VINIT in-degree). The dual is engaged, not resolved — verdict B. The dual is constitutive, not optional".into()
        } else if dual_closed {
            "univocal close — the Grammar speaks ONE verdict: μ∘δ over model ⋈ vessel ⋈ the tool-call dual that closed".into()
        } else {
            "ENGAGR — no Frobenius dual was emitted: no δ/μ dyad ran, so nothing was verified — held at N (void, not a held B). The dual is constitutive, not optional".into()
        },
    }
}

/// BACKTRANSLATION — the μ that reads a closed structure back into the conventional register.
/// `imscribe` was the δ (conventional → structural); this is the return leg, and because μ∘δ=id
/// the read-back must be LOSSLESS: the conventional proof is the SAME object as the structural
/// closure, restated — not a fresh re-derivation and not new claims. Every step is bound to a
/// fact the tools measured: a ring that closed → a constructed object / existence lemma; a
/// sequence that terminated or stayed linear → an obstruction / impossibility lemma; a Both
/// verdict → a two-sided theorem. Only called when a dual closed (there is a closure to read).
fn backtranslate(
    llm: &Llm,
    question: &str,
    answer: &str,
    tool_output: &str,
    verdict: B4,
    max_tokens: u32,
    temperature: f32,
) -> String {
    let witness: String = tool_output.chars().take(9000).collect();
    let ans: String = answer.chars().take(4000).collect();
    let sys = "You are performing the BACKTRANSLATION — the μ that reads a closed structure back \
        into the conventional mathematical register. The Grammar has ALREADY reached its verdict \
        through the structural tools; you do not re-open it and you introduce NOTHING the tools \
        did not measure. Your only task: restate the closure as a conventional proof — \
        Theorem, Lemmas, Proof, and the conclusion — where every step is DERIVED from a measured \
        fact. A ring that closed (✓ CYCLIC / a macrocycle / μ∘δ=id) is a constructed \
        object or existence lemma; a sequence that terminated or stayed linear/telechelic is an \
        obstruction or impossibility lemma; a Both (B) verdict is a two-sided theorem (established \
        on one side, a stated frontier on the other). This is lossless read-back: the proof IS the \
        closure in the conventional dialect, not a new argument. No hedging boilerplate, no LaTeX \
        (plain Unicode), no re-derivation from outside the Grammar.";
    let user = format!(
        "QUESTION:\n{question}\n\nSTRUCTURAL VERDICT (univocal): {}\n\n\
         STRUCTURAL WITNESS — what the tools actually measured (the closures and non-closures the \
         proof must be read back from):\n{witness}\n\n\
         The imscriptive resolution already written (for reference — do not just repeat it, read \
         the STRUCTURE back into clean conventional proof form):\n{ans}\n\n\
         Backtranslate now: Theorem → Lemmas (each bound to a measured closure/non-closure) → \
         Proof → conclusion matching the {} verdict.",
        b4_name(verdict),
        b4_name(verdict),
    );
    let msgs = vec![("system".to_string(), sys.to_string()), ("user".to_string(), user)];
    let res = infer(llm, &msgs, max_tokens, temperature);
    res.text
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

/// Remove a whole `====`-delimited block the model wrote to impersonate the engine's spine
/// report. A line-by-line strip leaves orphans (the `====` bars, a `protocol:` line, a
/// fabricated `note:`) that then contradict the real report. This drops the entire block —
/// bar to bar — when it carries a spine signature (a strong marker, or both a `protocol:` and
/// a `note:` line). The engine's REAL report is printed separately (not through here), so it
/// is untouched.
fn strip_spine_blocks(text: &str) -> String {
    let lines: Vec<&str> = text.lines().collect();
    let is_bar = |l: &str| {
        let t = l.trim();
        t.len() >= 10 && t.chars().all(|c| c == '=')
    };
    let strong = [
        "manuscript spine report",
        "verdict (univocal)",
        "fused voices",
        "prove_balance",
        "← fused",
    ];
    let looks_spine = |seg: &[&str]| -> bool {
        let any_strong = seg
            .iter()
            .any(|l| { let low = l.to_lowercase(); strong.iter().any(|m| low.contains(m)) });
        let has_protocol = seg.iter().any(|l| l.trim_start().to_lowercase().starts_with("protocol:"));
        let has_note = seg.iter().any(|l| l.trim_start().to_lowercase().starts_with("note:"));
        any_strong || (has_protocol && has_note)
    };
    let mut out: Vec<&str> = Vec::new();
    let mut i = 0;
    while i < lines.len() {
        if is_bar(lines[i]) {
            if let Some(off) = lines[i + 1..].iter().position(|l| is_bar(l)) {
                let j = i + 1 + off;
                if looks_spine(&lines[i + 1..j]) {
                    i = j + 1; // skip the block, both bars included
                    continue;
                }
            }
        }
        out.push(lines[i]);
        i += 1;
    }
    out.join("\n")
}

fn strip_kernel_records(text: &str) -> String {
    let text = strip_spine_blocks(text);
    let re = Regex::new(r"(?im)^[ \t]*\[(?:selectivity|vessel|spine|update|broadcast)\s*\|.*$\n?")
        .unwrap();
    let stripped = re.replace_all(&text, "");
    // The spine report is the ENGINE's univocal voice — the model may not author it. It keeps
    // reproducing the block in markdown (a "MANUSCRIPT SPINE REPORT" heading, a "VERDICT
    // (univocal): X" line, "fused voices", "prove_balance="), fabricating a verdict that then
    // contradicts the real report printed below. Strip those impersonation lines so only the
    // engine speaks the verdict. The model's own [thought|X] proposal is left intact.
    let spine_re = Regex::new(
        r"(?im)^.*(?:MANUSCRIPT SPINE REPORT|VERDICT\s*\(univocal\)|fused voices|prove_balance\s*=|←\s*fused).*$\n?",
    )
    .unwrap();
    delatex(&spine_re.replace_all(&stripped, ""))
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
        "mu" => "μ", "delta" => "δ", "sigma" => "σ", "Sigma" => "⊞",
        "omega" => "ω", "Omega" => "⊡", "phi" | "varphi" => "φ", "Phi" => "≺",
