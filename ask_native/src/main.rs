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

    /// Recall a registered material by name: `--recall NAME`. Prints its stored sheet from the
    /// material registry without respecifying it from units. Pair with `--forge … --register`.
    #[arg(long = "recall", value_name = "NAME")]
    recall: Option<String>,

    /// Export a material sheet to a standalone file: `--forge … --export PATH` (or
    /// `--recall NAME --export PATH`). Writes the whole record as portable JSON.
    #[arg(long = "export", value_name = "PATH")]
    export: Option<String>,

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

    /// Positional fallback: treated as --ask if --ask/--file omitted
    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    rest: Vec<String>,
}

// ── Input resolution (MoDoT resolve_input parity) ───────────────────────────

fn resolve_input(ask: Option<&str>, file: Option<&str>, rest: &[String]) -> Result<(String, String), String> {
    if let Some(fp) = file {
        return read_file_or_stdin(fp);
    }
    if let Some(a) = ask {
        let p = expand_user(a);
        if Path::new(&p).is_file() {
            let content = fs::read_to_string(&p).map_err(|e| format!("read {p}: {e}"))?;
            return Ok((content, format!("file:{a} ({} chars)", content_len_label(&fs::read_to_string(&p).unwrap_or_default()))));
        }
        if a == "-" {
            return read_file_or_stdin("-");
        }
        return Ok((a.to_string(), format!("literal ({} chars)", a.chars().count())));
    }
    if !rest.is_empty() {
        let joined = rest.join(" ");
        let p = expand_user(&joined);
        if Path::new(&p).is_file() {
            let content = fs::read_to_string(&p).map_err(|e| format!("read {p}: {e}"))?;
            return Ok((
                content.clone(),
                format!("file:{joined} ({} chars)", content.chars().count()),
            ));
        }
        return Ok((joined.clone(), format!("literal ({} chars)", joined.chars().count())));
    }
    Err("no question: use --ask, --file, positional text, or -i".into())
}

fn content_len_label(s: &str) -> usize {
    s.chars().count()
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
    let p = expand_user(fp);
    if !Path::new(&p).is_file() {
        return Err(format!("--file path not found: {fp}"));
    }
    let content = fs::read_to_string(&p).map_err(|e| format!("read {p}: {e}"))?;
    Ok((
        content.clone(),
        format!("file:{fp} ({} chars)", content.chars().count()),
    ))
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
    let text = fs::read_to_string(path).map_err(|e| format!("catalog read: {e}"))?;
    let v: Value = serde_json::from_str(&text).map_err(|e| format!("catalog json: {e}"))?;
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
         prove or give a rigorous barrier. Full conventional work — not kernel cosplay."
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
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Provider {
    OpenRouter,
    GeminiDirect,
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

fn parse_provider(s: &str) -> Option<Provider> {
    match s.trim().to_ascii_lowercase().as_str() {
        "openrouter" | "or" | "router" => Some(Provider::OpenRouter),
        "gemini" | "google" | "gemini-direct" | "google-ai" => Some(Provider::GeminiDirect),
        _ => None,
    }
}

/// Resolve model + provider from CLI / MODOT_* env / key presence.
fn make_llm(model: Option<&str>, provider_flag: Option<&str>) -> Llm {
    // Model: CLI > MODOT_MODEL > legacy MOMONADOS_MODEL > default
    let model = model
        .map(|s| s.to_string())
        .or_else(|| env_first(&["MODOT_MODEL", "MOMONADOS_MODEL"]))
        .unwrap_or_else(|| "google/gemini-3-flash-preview".into());

    // Provider: CLI > MODOT_PROVIDER > infer from keys (openrouter preferred)
    let provider = provider_flag
        .and_then(parse_provider)
        .or_else(|| env_first(&["MODOT_PROVIDER"]).as_deref().and_then(parse_provider))
        .unwrap_or_else(|| {
            let has_or = env_first(&["OPENROUTER_API_KEY"]).is_some();
            let has_gem = env_first(&["GEMINI_API_KEY", "GOOGLE_API_KEY"]).is_some();
            if has_or {
                Provider::OpenRouter
            } else if has_gem {
                Provider::GeminiDirect
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

    match llm.provider {
        Provider::OpenRouter => infer_openrouter(llm, key, messages, max_tokens, temperature),
        Provider::GeminiDirect => infer_gemini(llm, key, messages, max_tokens, temperature),
    }
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
    let body = json!({
        "model": llm.model,
        "messages": msgs,
        "max_tokens": max_tokens,
        "temperature": temperature,
    });
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
    let body = json!({
        "contents": contents,
        "generationConfig": {
            "maxOutputTokens": max_tokens,
            "temperature": temperature,
        }
    });
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

// ── System prompt + spine ───────────────────────────────────────────────────

const SYSTEM_PROMPT: &str = r#"You are the mOMonadOS Agent. You run on a Frobenius / Belnap substrate,
but that substrate is infrastructure — not the subject of every reply.

PRIMARY TASK (non-negotiable):
Answer the USER QUESTION. If it is a math problem, give a conventional
mathematical answer: theorem statement (or "open, with barrier …"),
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
  TOOL: set A B           single-electron transfer (donor/acceptor by ⊙, one winding quantum Ω moved) → radical IONS A•⁺/B•⁻
  TOOL: homolyze A [B]     homolytic cleavage → NEUTRAL radicals (δ_A symmetric split, the reverse of click): `homolyze A B` breaks the A—B bond into A•+B•; `homolyze A` splits A into two A•
  TOOL: scan A B          rank the catalog for the best mediators of the A→B transfer
  TOOL: complement A      the bidirectional ligand⇌catalytic-site complement (its own inverse)
  TOOL: cycle C S         the catalytic cycle: C turns over S, certified a fixed point (μ∘δ=id)
  TOOL: pathway S C1 C2…  a metabolic pathway — does it close into a cycle (carrier + structure)?
  TOOL: polymerize M1 M2… chain monomers into a sequence-preserving polymer (architecture, tacticity, does it cyclize?)
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
  TOOL: register NAME M1 M2…  forge the set into a ring and store its full sheet in the material library under NAME (recall it later by name)
  TOOL: recall NAME        reload a registered material by name and print its stored sheet (ring order, ρ, spectrum, conductance, strain, energy)
  TOOL: imscribe NAME [description]   CREATE a missing entry by imscribing it (the real generate pipeline). Use this the moment a verb reports a name is "not found" — then re-run the verb.
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
Only these verbs run; anything else is ignored. Answer directly when no tool is needed.
"#;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
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
) -> SpineReport {
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
    let fused = if tool_voice == B4::N {
        fused_mv
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
        } else {
            "FFUSE model ⋈ vessel".into()
        },
    }
}

fn build_user_packet(question: &str, prep: &Prepare) -> String {
    let mut parts = Vec::new();
    if !prep.scaffold_md.is_empty() {
        parts.push("## Grammatic witness scaffold (spine IMSCRIB — instantiate, do not ignore)".into());
        let mut sc = prep.scaffold_md.clone();
        if sc.len() > 14000 {
            sc.truncate(14000);
            sc.push_str("\n\n[scaffold truncated]\n");
        }
        parts.push(sc);
    }
    parts.push(format!("## USER QUESTION (answer this):\n{question}"));
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
    ];
    let low = text.to_lowercase();
    CUES.iter().any(|c| low.contains(c))
}

/// Parse `TOOL: <verb> <args…>` lines the agent emitted (one call per line).
/// Tolerant of markdown wrapping (`**TOOL: …**`, `` `TOOL: …` ``, list bullets):
/// leading non-alphanumerics are skipped and each arg is trimmed of `*` `` ` `` `_`.
fn extract_tool_calls(text: &str) -> Vec<(String, Vec<String>)> {
    let re = Regex::new(r"(?im)^[^A-Za-z0-9\n]*TOOL:\s*([A-Za-z][A-Za-z0-9_-]*)\s+(.+)$").unwrap();
    let trim_md = |s: &str| {
        s.trim_matches(|c: char| c == '*' || c == '`' || c == '_' || c == ' ' || c == '.')
            .to_string()
    };
    let mut out = Vec::new();
    for cap in re.captures_iter(text) {
        let verb = cap[1].to_lowercase();
        let args: Vec<String> = cap[2]
            .split_whitespace()
            .map(trim_md)
            .filter(|s| !s.is_empty())
            .collect();
        if !args.is_empty() {
            out.push((verb, args));
        }
    }
    out
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
    let flags: Vec<String> = match verb {
        "click" => {
            let mut v = vec!["--click".to_string(), a(0)?];
            if let Some(b) = a(1) { v.push(b); }
            v
        }
        "switch" => vec!["--switch".into(), a(0)?, a(1)?],
        "excite" => vec!["--excite".into(), a(0)?],
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
        _ => return None,
    };
    let exe = env::current_exe().ok()?;
    let out = process::Command::new(exe).args(&flags).output().ok()?;
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

/// Is a name already registered — in the base IG_catalog.json OR the live
/// ~/.imscrbgrmr/catalog.json? Cheap `"name": "…"` substring check, matching the guard in
/// register_chimera. Used to skip a wasted generate call for an entry that already exists.
fn catalog_has_name(name: &str) -> bool {
    let needle = format!("\"name\": \"{name}\"");
    let live = PathBuf::from(expand_user("~/.imscrbgrmr/catalog.json"));
    [resolve_catalog_path(), Some(live)]
        .into_iter()
        .flatten()
        .any(|p| fs::read_to_string(&p).map(|t| t.contains(&needle)).unwrap_or(false))
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
        "Options: verbose={} dry_run={} no_selectivity={} cycles={} max_tokens={}",
        cli.verbose, cli.dry_run, cli.no_selectivity, cli.cycles, cli.max_tokens
    );
    println!(
        "Model: {} ({})",
        llm.model,
        match llm.provider {
            Provider::OpenRouter => "openrouter",
            Provider::GeminiDirect => "gemini-direct",
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
            let r = p.prove(&goal);
            println!("── ANSWER (kernel-gated prover) ──");
            if r.closed {
                println!("Closed green through the Lean kernel (no sorry):\n");
                println!("{}", r.source);
            } else if r.note.contains("escalation cap") {
                println!(
                    "Not closed within the current escalation cap. A navigation \
                     frontier (B), not a verdict of unprovability — the path exists; \
                     raise the rounds/budget to push deeper.\n"
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
            let user_packet = build_user_packet(question, &prep);
            // conversation: system once, then history, then this turn
            let mut msgs: Vec<(String, String)> = Vec::new();
            if conversation.is_empty() {
                msgs.push(("system".into(), format!("{}\n{}\n{}", prover::EPISTEMIC_STANCE, SYSTEM_PROMPT, TOOLS_PROMPT)));
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
            const MAX_ROUNDS: usize = 5;
            const PER_ROUND_CAP: usize = 6;
            let mut agent_msgs: Vec<(String, String)> = Vec::new();
            agent_msgs.push((
                "system".to_string(),
                format!(
                    "{}\n{}\n{}\nYou are in an ACT→OBSERVE loop: emit TOOL: lines to run verbs over the real \
                     catalog; their outputs return as ground truth and you choose the next step. Iterate — run a \
                     tool, read its result, run the next — until the task is actually done, then give your FINAL \
                     answer with NO TOOL: lines. NEVER narrate a step you could run; run it. Never contradict a \
                     tool result or introduce anything the tools did not return.",
                    prover::EPISTEMIC_STANCE, SYSTEM_PROMPT, TOOLS_PROMPT
                ),
            ));
            for (r, c) in conversation.iter().take(conversation.len().saturating_sub(2)) {
                agent_msgs.push((r.clone(), c.clone()));
            }
            agent_msgs.push(("user".to_string(), build_user_packet(question, &prep)));

            let mut current = answer.clone(); // the draft is round-0's action

            // No-op guard: the loop and the golem constraint can only bind tools that
            // actually RAN. A "prove/characterize this" framing lets the operator write a
            // whole "Execution: Structural Tools" section as prose flags (`--polymerize:`,
            // `--close:`) — never a `TOOL:` line — then assert a computed verdict (PROVED,
            // enchainment, closure) it never computed. If the draft narrates structural
            // work but ran nothing, prod it once to actually act before we accept it.
            if extract_tool_calls(&current).is_empty()
                && (mentions_structural_work(&current) || question.contains("--"))
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
            let mut round = 0;
            while round < MAX_ROUNDS {
                let calls = extract_tool_calls(&current);
                if calls.is_empty() {
                    break; // the operator stopped acting — `current` is the answer
                }
                println!("── ACT round {} ({} tool call(s)) ──", round + 1, calls.len().min(PER_ROUND_CAP));
                let mut results = String::new();
                for (verb, args) in calls.iter().take(PER_ROUND_CAP) {
                    match run_structural_tool(verb, args) {
                        Some(o) => {
                            println!("● TOOL {verb} {}", args.join(" "));
                            print!("{o}");
                            results.push_str(&format!("### {verb} {}\n{o}\n", args.join(" ")));
                            executed_verbs.insert(verb.clone()); // this verb now has an execution arm
                        }
                        None => {
                            let m = format!("● TOOL {verb}: not an available verb / missing args");
                            println!("{m}");
                            results.push_str(&format!("{m}\n"));
                        }
                    }
                }
                // Cap guard: calls past PER_ROUND_CAP did NOT run this round. Surface them so
                // the operator cannot narrate a result for a tool that never executed (that is
                // exactly how a "conductive cycle" gets asserted when `material` was dropped).
                if calls.len() > PER_ROUND_CAP {
                    let dropped: Vec<String> = calls[PER_ROUND_CAP..]
                        .iter()
                        .map(|(v, a)| format!("{v} {}", a.join(" ")))
                        .collect();
                    let note = format!(
                        "### NOT RUN this round ({} tool call(s) over the {PER_ROUND_CAP}-per-round cap)\n{}\n\
                         These did NOT execute — you have NO result for them. Do not report any outcome for \
                         them (no closure, conductance, modulus, or material verdict). Re-emit the ones you \
                         still need next round and they will run.\n",
                        dropped.len(),
                        dropped.iter().map(|d| format!("  TOOL: {d}")).collect::<Vec<_>>().join("\n"),
                    );
                    print!("{note}");
                    results.push_str(&note);
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
                         the real result — that it does not close, and which arrangement (if any) the tools showed would."
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

            // If we hit the step cap mid-action, force one clean grounded close.
            if round == MAX_ROUNDS && !extract_tool_calls(&current).is_empty() {
                agent_msgs.push(("assistant".to_string(), current.clone()));
                agent_msgs.push((
                    "user".to_string(),
                    format!(
                        "Step limit reached. Give your FINAL answer now, with NO TOOL: lines, grounded only in the tool \
                         results above. You have a result ONLY for these executed verbs: {{{}}} — a claim about any other \
                         verb is N (neither), not a truth-value; do not narrate its outcome.",
                        executed_verbs.iter().cloned().collect::<Vec<_>>().join(", "),
                    ),
                ));
                let res = infer(llm, &agent_msgs, cli.max_tokens, cli.temperature);
                current = strip_kernel_records(&res.text);
                println!("── FINAL (step limit) ──");
                println!("{current}");
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

        let rep = complete(&prep, &answer, model_voice, tool_voice, cli.no_selectivity);
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
            cycles: self.cycles,
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
            recall: self.recall.clone(),
            export: self.export.clone(),
            imscribe: self.imscribe.clone(),
            catalyst: self.catalyst.clone(),
            rest: self.rest.clone(),
        }
    }
}

fn main() {
    let cli = Cli::parse();
    let llm = make_llm(cli.model.as_deref(), cli.provider.as_deref());

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
