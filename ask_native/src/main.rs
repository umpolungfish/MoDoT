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
    let low = text.to_lowercase();
    if low.contains("both") && low.contains("neither") {
        return 'B';
    }
    if text.trim().is_empty() {
        return 'F';
    }
    'T'
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

SECONDARY (optional, after the answer):
Tag [thought|T|F|B|N] for your Belnap self-assessment.
COMPOSE:/TOKEN:/CANONICAL: optional tools, never a substitute for
answering. Do not author [spine|..], [vessel|..], [update|..], [broadcast|..].
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
    let fused = if no_selectivity {
        model_voice
    } else if vessel == B4::N {
        model_voice
    } else {
        b4_join(model_voice, vessel)
    };
    let conflict = if no_selectivity {
        0
    } else {
        b4_conflict(model_voice, vessel)
    };

    SpineReport {
        fused,
        model_voice,
        vessel_voice: vessel,
        conflict,
        riding,
        prove_balance: true, // μ∘δ face: harness closed on successful emit/verify
        primary: prep.primary_name.clone(),
        answer_text: answer_text.to_string(),
        note: if no_selectivity {
            "model only (--no-selectivity)".into()
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
    re.replace_all(text, "").to_string()
}

fn print_spine(rep: &SpineReport, prep: &Prepare, verbose: bool) {
    println!();
    println!("{}", "=".repeat(60));
    println!("MANUSCRIPT SPINE REPORT");
    println!(
        "  fused={}  model={}  vessel={}  conflict={}",
        b4_name(rep.fused),
        b4_name(rep.model_voice),
        b4_name(rep.vessel_voice),
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

        let answer;
        let model_voice;

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
                msgs.push(("system".into(), format!("{}\n{}", prover::EPISTEMIC_STANCE, SYSTEM_PROMPT)));
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

        let rep = complete(&prep, &answer, model_voice, cli.no_selectivity);
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
            complement: self.complement.clone(),
            scan_mediators: self.scan_mediators,
            cycle: self.cycle.clone(),
            pathway: self.pathway.clone(),
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
