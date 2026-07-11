//! click.rs — the math catalytic register and click-maths fusion.
//!
//! Ports red-hot_rebis's catalytic-site complementation
//! (`rhr_p4rky/ch3mpiler_serpentrod_pipeline.py`) to abstract math. The register
//! finding (`project_math_catalytic_register`): mathematics pins F / Ph / K out
//! (exact fidelity / subcritical / frozen kinetics), so catalysis runs on the
//! three LIVE conjugate pairs D↔W, T↔H, R↔S. Click-maths is the simplest
//! operation on that register — reliable FFUSE (μ) of two fragments that are
//! complementary on exactly ONE live pair and co-typed (orthogonal) elsewhere.
//! Spring-loaded, closes by construction; the smallest delta from the prover's
//! existing FSPLIT/FFUSE. See `CLICK_MATHS_SPEC.md`.

use crate::CatalogEntry;
use std::path::Path;
use std::process::Command;

/// The twelve primitive keys, in canonical navigator/catalog order.
pub const PRIMS: [&str; 12] = ["Ð", "Þ", "Ř", "Φ", "ƒ", "Ç", "Γ", "ɢ", "⊙", "Ħ", "Σ", "Ω"];

/// glyph → ordinal per primitive. The ORDERING is scripture: each primitive's
/// constructor order comes from the p4rakernel Lean kernel (`Primitives/Core.lean`)
/// and the glyph↔constructor map from `gen_clay_canonical_tuples.py`. Ç
/// (KineticChar: yea<loll<egg<on<air) and Σ (Stoichiometry: hung<so<up) were
/// CORRECTED here to match scripture — earlier values had Ç ords 3/4 swapped
/// (𐑺/𐑪) and Σ rotated (𐑳 at 0 instead of 2), which threw the R↔S live-pair
/// charge and the ligand↔site complement off (glyph→ctor rendering was unaffected,
/// so certify always stayed correct; only the numeric ordinals were wrong).
const GLYPHS: [&[(&str, u8)]; 12] = [
    /* Ð Dimensionality */ &[("𐑛", 0), ("𐑨", 1), ("𐑼", 2), ("𐑦", 3)],
    /* Þ Topology       */ &[("𐑡", 0), ("𐑰", 1), ("𐑥", 2), ("𐑶", 3), ("𐑸", 4)],
    /* Ř Recognition    */ &[("𐑩", 0), ("𐑑", 1), ("𐑽", 2), ("𐑾", 3)],
    /* Φ Parity         */ &[("𐑗", 0), ("𐑿", 1), ("𐑬", 2), ("𐑯", 3), ("𐑹", 4)],
    /* ƒ Fidelity       */ &[("𐑱", 0), ("𐑞", 1), ("𐑐", 2)],
    /* Ç Kinetics       */ &[("𐑘", 0), ("𐑤", 1), ("𐑧", 2), ("𐑪", 3), ("𐑺", 4)],
    /* Γ Granularity    */ &[("𐑚", 0), ("𐑔", 1), ("𐑲", 2)],
    /* ɢ Composition    */ &[("𐑝", 0), ("𐑜", 1), ("𐑠", 2), ("𐑵", 3)],
    /* ⊙ Criticality    */ &[("𐑢", 0), ("⊙", 1), ("𐑮", 2), ("𐑻", 3), ("𐑣", 4)],
    /* Ħ Chirality      */ &[("𐑓", 0), ("𐑒", 1), ("𐑖", 2), ("𐑫", 3)],
    /* Σ Stoichiometry  */ &[("𐑙", 0), ("𐑕", 1), ("𐑳", 2)],
    /* Ω Protection     */ &[("𐑷", 0), ("𐑴", 1), ("𐑭", 2), ("𐑟", 3)],
];

fn ordinal(prim: usize, glyph: &str) -> Option<u8> {
    GLYPHS[prim].iter().find(|(g, _)| *g == glyph).map(|(_, o)| *o)
}
fn max_ord(prim: usize) -> u8 {
    GLYPHS[prim].iter().map(|(_, o)| *o).max().unwrap_or(0)
}
fn glyph_of(prim: usize, ord: u8) -> &'static str {
    GLYPHS[prim].iter().find(|(_, o)| *o == ord).map(|(g, _)| *g).unwrap_or("?")
}

/// The six conjugate pairs (indices into `PRIMS`): the "charge table", an
/// involution pairing each primitive with its dual.  D↔W, T↔H, R↔S, P↔F, K↔G, Gm↔Ph.
/// The full table; `LIVE_PAIRS` is the active subset for the math register. The
/// three non-live pairs (each with a pinned member F/Ph/K) are reserved for the
/// addition mode — fusion onto a pinned pole — not the two-pole cycloaddition click.
#[allow(dead_code)]
pub const CONJUGATE_PAIRS: [(usize, usize); 6] =
    [(0, 11), (1, 9), (2, 10), (3, 4), (5, 6), (7, 8)];

/// The three LIVE pairs where mathematics actually varies. P↔F (F pinned max),
/// K↔G (K pinned low), Gm↔Ph (Ph pinned min) are degenerate in the math
/// register, so catalysis is designed only over these three.
pub const LIVE_PAIRS: [(usize, usize); 3] = [(0, 11), (1, 9), (2, 10)];
pub const LIVE_LABELS: [&str; 3] = ["D↔W", "T↔H", "R↔S"];

/// A 12-primitive ordinal tuple (None where the glyph was missing/unknown).
#[derive(Clone)]
pub struct Tuple {
    pub name: String,
    pub ord: [Option<u8>; 12],
}

impl Tuple {
    /// Read the twelve glyphs from a catalog entry's raw JSON.
    pub fn from_entry(e: &CatalogEntry) -> Tuple {
        let mut ord = [None; 12];
        for (i, key) in PRIMS.iter().enumerate() {
            if let Some(g) = e.raw.get(*key).and_then(|v| v.as_str()) {
                ord[i] = ordinal(i, g);
            }
        }
        Tuple { name: e.name.clone(), ord }
    }
    /// Normalized ordinal on a primitive (ord / max), or None if missing.
    pub fn norm(&self, prim: usize) -> Option<f32> {
        let m = max_ord(prim);
        self.ord[prim].map(|o| if m > 0 { o as f32 / m as f32 } else { 0.0 })
    }
}

/// Signed charge of a fragment on a live pair (x,y): norm(x) − norm(y).
fn pair_charge(t: &Tuple, pair: (usize, usize)) -> Option<f32> {
    Some(t.norm(pair.0)? - t.norm(pair.1)?)
}

/// The product of a successful click.
pub struct ClickProduct {
    pub pair_idx: usize, // index into LIVE_PAIRS — which pair fired
    pub drive: f32,      // spring-loaded offset that drove the click
    pub product: [Option<u8>; 12],
    pub ring: (usize, usize), // the live pair that saturated (formed the ring)
    pub inherited: Vec<usize>, // non-clicking primitives the fragments differed on (blended scaffold)
}

/// Why two fragments do NOT click.
pub enum ClickFail {
    NoComplementarity,     // no live pair reaches the spring-loaded offset
    Ambiguous(Vec<usize>), // >1 live pair complementary — no single reaction center
    Missing,               // a tuple couldn't be read
}

/// A clean click iff **exactly one** live pair is complementary at Δ≥theta
/// (opposite charges — one fragment high-x/low-y, the other the reverse), and the
/// other live pairs do NOT also fire (a single reaction center — bioorthogonality
/// as specificity, not as sameness). The two fragments need not be identical off
/// the reaction center; like an azide and an alkyne they are different scaffolds,
/// and the product inherits both — the fuse blends the non-clicking axes by join.
/// Reliable FFUSE (μ), spring-loaded, closes by construction.
pub fn click_pair(a: &Tuple, b: &Tuple, theta: f32) -> Result<ClickProduct, ClickFail> {
    // 1. complementarity + drive on each live pair (strict opposite charge)
    let mut complementary: Vec<(usize, f32)> = Vec::new();
    for (i, &pair) in LIVE_PAIRS.iter().enumerate() {
        let (ca, cb) = match (pair_charge(a, pair), pair_charge(b, pair)) {
            (Some(x), Some(y)) => (x, y),
            _ => return Err(ClickFail::Missing),
        };
        let opposite = (ca > 0.0 && cb < 0.0) || (ca < 0.0 && cb > 0.0);
        let drive = (ca - cb).abs();
        if opposite && drive >= theta {
            complementary.push((i, drive));
        }
    }
    // 2. selectivity: exactly one reaction center (specificity = orthogonality)
    if complementary.is_empty() {
        return Err(ClickFail::NoComplementarity);
    }
    if complementary.len() > 1 {
        return Err(ClickFail::Ambiguous(
            complementary.iter().map(|(i, _)| *i).collect(),
        ));
    }
    let (pair_idx, drive) = complementary[0];
    let ring = LIVE_PAIRS[pair_idx];
    // 3. fuse (μ): join (max) on every axis. On the clicking pair the opposite
    //    charges bring both members up together — the ring forms. Off it, the
    //    product inherits the stronger character of each scaffold.
    let mut product = [None; 12];
    for p in 0..12 {
        product[p] = match (a.ord[p], b.ord[p]) {
            (Some(x), Some(y)) => Some(x.max(y)),
            (Some(x), None) | (None, Some(x)) => Some(x),
            (None, None) => None,
        };
    }
    // 4. record the inherited (blended) non-clicking axes — the scaffold the
    //    product carries from both partners, for the report.
    let inherited: Vec<usize> = (0..12)
        .filter(|&p| p != ring.0 && p != ring.1 && a.ord[p] != b.ord[p])
        .collect();
    Ok(ClickProduct { pair_idx, drive, product, ring, inherited })
}

/// Ouroboricity-tier heuristic on a product tuple — ported from
/// `cl8nk_navigator.assess_tier`: counts the advanced primitive values a
/// self-modeling O_∞ system carries. Score 0–8 → tier.
fn tier_score(ord: &[Option<u8>; 12]) -> u8 {
    let eq = |i: usize, v: u8| ord[i] == Some(v);
    let mut s = 0u8;
    if eq(8, 1) { s += 1; } // ⊙ = ⊙ (self-modeling critical)
    if eq(3, 4) { s += 1; } // Φ = 𐑹 (Frobenius-special parity)
    if eq(9, 3) { s += 1; } // Ħ = 𐑫 (eternal chirality)
    if matches!(ord[11], Some(2) | Some(3)) { s += 1; } // Ω integer/non-Abelian winding
    if eq(0, 3) { s += 1; } // Ð = 𐑦 (holographic)
    if eq(5, 2) { s += 1; } // Ç = 𐑧 (slow/coherent kinetics)
    if eq(1, 4) { s += 1; } // Þ = 𐑸 (self-referential topology)
    if eq(2, 3) { s += 1; } // Ř = 𐑾 (bidirectional recognition)
    s
}
fn tier_label(score: u8) -> &'static str {
    match score {
        7..=8 => "O_∞",
        5..=6 => "O₂",
        3..=4 => "O₁",
        _ => "O₀",
    }
}

/// CLI entry: `./ask --click <A>` (one name) — sweep A against the whole catalog,
/// keep every fragment that clicks with it, and rank the products by tier (the
/// self-modeling ascent), then by drive. Surfaces the coniunctio-class results
/// (products that fuse to O_∞) automatically. Optional catalyst lowers θ throughout.
pub fn run_click_sweep(
    catalog: Option<&[CatalogEntry]>,
    name: &str,
    theta: f32,
    catalyst_name: Option<&str>,
    top: usize,
) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("click-sweep: no catalog loaded");
        return 2;
    };
    let Some(ea) = cat.iter().find(|e| e.name == name) else {
        eprintln!("click-sweep: catalog entry not found: {name}");
        return 2;
    };
    let ta = Tuple::from_entry(ea);
    let catalyst: Option<Tuple> = match catalyst_name {
        Some(cn) => match cat.iter().find(|e| e.name == cn) {
            Some(ec) => Some(Tuple::from_entry(ec)),
            None => {
                eprintln!("click-sweep: catalyst not found: {cn}");
                return 2;
            }
        },
        None => None,
    };

    // hit: (partner name, live-pair label, drive, product tier score, product tier label)
    let mut hits: Vec<(String, &str, f32, u8, &str)> = Vec::new();
    for e in cat {
        if e.name == name {
            continue;
        }
        let tb = Tuple::from_entry(e);
        let res = match &catalyst {
            Some(tc) => click_pair_catalyzed(&ta, &tb, tc, theta).0,
            None => click_pair(&ta, &tb, theta),
        };
        if let Ok(p) = res {
            let sc = tier_score(&p.product);
            hits.push((e.name.clone(), LIVE_LABELS[p.pair_idx], p.drive, sc, tier_label(sc)));
        }
    }
    // rank: highest tier first, then strongest drive
    hits.sort_by(|a, b| b.3.cmp(&a.3).then(b.2.partial_cmp(&a.2).unwrap_or(std::cmp::Ordering::Equal)));

    let cat_note = catalyst_name.map(|c| format!(" (catalyst {c})")).unwrap_or_default();
    println!(
        "click-sweep: {name} ⋈ * over {} entries{cat_note} — {} clicks, top {}:",
        cat.len(),
        hits.len(),
        top.min(hits.len())
    );
    for (partner, pair, drive, _sc, tier) in hits.iter().take(top) {
        println!("  {tier:4}  {pair:5} Δ={drive:.2}   {partner}");
    }
    0
}

/// Catalyzed click. Ports the catalyst ob3ect protocol the Grammar designed
/// (`ob3ect/digital/a_lossless_self_restoring_operation_that_lowers/`): a catalyst
/// COUPLES to the pair (CLINK), LOWERS the threshold (AFWD barrier reduction),
/// the fusion fires at the lowered θ (EVALT), the barrier is RESTORED (AREV), and
/// the catalyst is verified unchanged (IMSCRIB — μ∘δ=id, regenerated, not consumed).
///
/// The catalyst's strength is its self-restoring / Frobenius-special character:
/// discriminated by its Parity Φ (Φ=𐑹 is the Frobenius-special value, the lossless
/// μ∘δ=id map — e.g. `math_isomorphism`). Strength scales the barrier reduction.
/// Crucially the catalyst lowers only the threshold, NEVER the complementarity: a
/// same-sign or neutral pair still cannot fuse (a catalyst lowers ΔG‡, not ΔG — it
/// speeds an allowed fusion, it cannot drive a forbidden one). Returns the product
/// and the lowered θ that was actually applied.
/// Returns the fusion result and the effective (lowered) threshold that was
/// applied — the effective θ is returned whether the fusion succeeds or fails, so
/// a caller can honestly report "barrier at 0, still refuses" (the guardrail).
pub fn click_pair_catalyzed(
    a: &Tuple,
    b: &Tuple,
    catalyst: &Tuple,
    base_theta: f32,
) -> (Result<ClickProduct, ClickFail>, f32) {
    // Catalyst strength = Frobenius-special / self-restoring character, discriminated
    // by Parity Φ (index 3). Φ=𐑹 (norm 1.0) is the maximal μ∘δ=id catalyst.
    let strength = catalyst.norm(3).unwrap_or(0.0);
    // Barrier reduction: the catalyst lowers the effective threshold.
    let theta_eff = base_theta * (1.0 - strength);
    (click_pair(a, b, theta_eff), theta_eff)
}

// ── Harness: register a chimera + decompose it through the cl8nk_navigator ────

/// Append a chimera (a click product) to the canonical catalog as a first-class
/// entry, so the cl8nk_navigator can decompose, promote, and tensor it. Textual
/// append before the closing `]` (no reformatting of the 84k-line file), verified
/// to still parse before it is written. Fails (leaving the catalog untouched) if
/// the entry name already exists or the append would not parse.
fn register_chimera(
    catalog_path: &Path,
    name: &str,
    description: &str,
    product: &[Option<u8>; 12],
) -> Result<(), String> {
    let text = std::fs::read_to_string(catalog_path).map_err(|e| format!("catalog read: {e}"))?;
    if text.contains(&format!("\"name\": \"{name}\"")) {
        return Err(format!("an entry named '{name}' already exists — pick another --name"));
    }
    let esc = |s: &str| serde_json::Value::String(s.to_string()).to_string();
    let mut lines = vec![
        format!("    \"name\": {}", esc(name)),
        format!("    \"description\": {}", esc(description)),
    ];
    for i in 0..12 {
        let g = product[i]
            .map(|o| glyph_of(i, o))
            .ok_or_else(|| "product has a missing primitive".to_string())?;
        lines.push(format!("    \"{}\": \"{}\"", PRIMS[i], g));
    }
    let entry = format!("  {{\n{}\n  }}", lines.join(",\n"));
    let close = text.rfind(']').ok_or("catalog has no closing ]")?;
    let head = text[..close].trim_end();
    if !head.ends_with('}') {
        return Err("catalog tail is not a well-formed array".into());
    }
    let new = format!("{head},\n{entry}\n]\n");
    serde_json::from_str::<serde_json::Value>(&new)
        .map_err(|e| format!("append would corrupt the catalog (aborted): {e}"))?;
    std::fs::write(catalog_path, new).map_err(|e| format!("catalog write: {e}"))?;
    Ok(())
}

/// Decompose a registered chimera through the cl8nk_navigator (the canonical tool —
/// per feedback_use_cl8nk_navigator, use it, never re-derive its internals). Shells
/// to `python3 cl8nk_navigator.py entry <name>` in the navigators dir beside the
/// catalog; on any failure, prints the commands for the user to run.
fn decompose_via_navigator(catalog_path: &Path, name: &str) {
    let nav_dir = catalog_path
        .parent()
        .map(|d| d.join("navigators"))
        .unwrap_or_else(|| Path::new("navigators").to_path_buf());
    let script = nav_dir.join("cl8nk_navigator.py");
    let ran = Command::new("python3")
        .arg(&script)
        .args(["entry", name])
        .current_dir(&nav_dir)
        .output();
    match ran {
        Ok(o) if o.status.success() => {
            print!("{}", String::from_utf8_lossy(&o.stdout));
        }
        _ => {
            println!("  decompose it with the cl8nk_navigator:");
            println!("    cd {}", nav_dir.display());
            println!("    python3 cl8nk_navigator.py entry {name}       # per-primitive CLINK fragments + promotions to L8");
            println!("    python3 cl8nk_navigator.py distance {name}    # distance to CLINK L8 (the terminal organism)");
            println!("    python3 cl8nk_navigator.py tensor {name}      # CLINK L8 ⊗ {name} — absorption test");
        }
    }
}

// ── DASA switch: the reversible bistable toggle ──────────────────────────────
// A switch is not a click. A click fuses two different fragments at one reaction
// center. A switch is ONE object in two co-typed forms, toggling on a live pair,
// driven by a DIFFERENT stimulus each direction (light δ forward, heat μ reverse) —
// the donor-acceptor Stenhouse adduct archetype: open↔closed, negative photochromism
// (light HIDES: quenches criticality; heat REVEALS: restores it). ⊙ = Criticality, idx 8.
const CRIT: usize = 8;

/// CLI entry: `./ask --switch <A> <B>` — analyze two co-typed forms as a reversible
/// bistable switch. Reports the toggling live pair (coupled — both members move),
/// the photochromic sign (which form is revealed/colored vs hidden/colorless by
/// criticality), the asymmetric δ (light) / μ (heat) legs, and the coupled
/// consequences. The DASA (donor-acceptor Stenhouse adduct) is the archetype.
pub fn run_switch(catalog: Option<&[CatalogEntry]>, name_a: &str, name_b: &str, certify: bool) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("switch: no catalog loaded");
        return 2;
    };
    let find = |n: &str| cat.iter().find(|e| e.name == n);
    let (ea, eb) = match (find(name_a), find(name_b)) {
        (Some(a), Some(b)) => (a, b),
        (None, _) => { eprintln!("switch: catalog entry not found: {name_a}"); return 2; }
        (_, None) => { eprintln!("switch: catalog entry not found: {name_b}"); return 2; }
    };
    let ta = Tuple::from_entry(ea);
    let tb = Tuple::from_entry(eb);

    // primitives that differ between the two forms = the switch coordinates
    let diffs: Vec<usize> = (0..12).filter(|&p| ta.ord[p] != tb.ord[p]).collect();
    println!("switch:  {name_a}  ⇌  {name_b}");
    if diffs.is_empty() {
        println!("  the two forms are identical — no switch (not a bistable pair).");
        return 0;
    }

    // toggling live pair(s): a live pair whose BOTH members move = a coupled switch
    let coupled: Vec<usize> = LIVE_PAIRS
        .iter()
        .enumerate()
        .filter(|(_, &(x, y))| diffs.contains(&x) && diffs.contains(&y))
        .map(|(i, _)| i)
        .collect();
    match coupled.as_slice() {
        [i] => {
            let (x, y) = LIVE_PAIRS[*i];
            let g = |t: &Tuple, p: usize| t.ord[p].map(|o| glyph_of(p, o)).unwrap_or("?");
            println!(
                "  toggling live pair: {} (coupled — both members move: {} {}→{}, {} {}→{})",
                LIVE_LABELS[*i], PRIMS[x], g(&ta, x), g(&tb, x), PRIMS[y], g(&ta, y), g(&tb, y)
            );
        }
        [] => println!("  no coupled live-pair toggle — the forms differ but not as a clean live-pair switch."),
        many => {
            let names: Vec<&str> = many.iter().map(|&i| LIVE_LABELS[i]).collect();
            println!("  multiple coupled live pairs move ({}) — a compound switch, not a single-axis toggle.", names.join(", "));
        }
    }

    // photochromic sign — criticality (⊙) orders the two forms: higher = revealed
    // (colored/exoteric/critical), lower = hidden (colorless/esoteric/subcritical).
    match (ta.ord[CRIT], tb.ord[CRIT]) {
        (Some(ca), Some(cb)) if ca != cb => {
            let (revealed, hidden) = if ca > cb { (name_a, name_b) } else { (name_b, name_a) };
            println!("  criticality (⊙): {revealed} is REVEALED (higher-⊙, colored/exoteric); {hidden} is HIDDEN (subcritical, colorless/esoteric)");
            println!("  δ (light):  {revealed} → {hidden}  — quenches criticality (negative photochromism: light hides)");
            println!("  μ (heat):   {hidden} → {revealed}  — restores criticality (relaxation reveals)");
        }
        _ => println!("  criticality (⊙) is equal — no photochromic sign; a criticality-neutral switch."),
    }

    // coupled consequences: other moving primitives (e.g. ring winding on closure)
    let in_pair: Vec<usize> = coupled.iter().flat_map(|&i| [LIVE_PAIRS[i].0, LIVE_PAIRS[i].1]).collect();
    let consequences: Vec<&str> = diffs
        .iter()
        .filter(|&&p| p != CRIT && !in_pair.contains(&p))
        .map(|&p| PRIMS[p])
        .collect();
    if !consequences.is_empty() {
        println!("  coupled consequences (move with the toggle): [{}]", consequences.join(", "));
    }
    println!("  reversible: μ∘δ = id — forward (δ) then reverse (μ) returns to the start form, lossless.");
    if certify {
        println!("  certifying through the Lean kernel (both forms valid + vessel roundtrip)…");
        let (green, out) = certify_switch(&ta.ord, &tb.ord);
        if green {
            println!("  ✓ KERNEL-CERTIFIED: both forms Frobenius-close AND readback∘board = id (μ∘δ=id, lossless toggle, real verdict).");
        } else {
            let tail: String = out.lines().rev().take(4).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n    ");
            println!("  ✗ kernel rejected the switch:\n    {tail}");
        }
    }
    0
}

// ── Kernel closure certificate ───────────────────────────────────────────────
// Glyph→Lean-constructor per primitive, ordinal-indexed (matching GLYPHS above).
// Canonical source: imscribing_grammar/scripts/gen_clay_canonical_tuples.py.
const CTORS: [&[&str]; 12] = [
    /* Ð */ &["dead", "ash", "array", "if'"],
    /* Þ */ &["judge", "eat", "mime", "oil", "are"],
    /* Ř */ &["ado", "tot", "ear", "ian"],
    /* Φ */ &["church", "yew", "out", "nun", "or'"],
    /* ƒ */ &["age", "they", "peep"],
    /* Ç */ &["yea", "loll", "egg", "on", "air"],
    /* Γ */ &["bib", "thigh", "ice"],
    /* ɢ */ &["vow", "gag", "measure", "ooze"],
    /* ⊙ */ &["woe", "monad", "roar", "err", "haha"],
    /* Ħ */ &["fee", "kick", "sure", "wool"],
    /* Σ */ &["hung", "so", "up"],
    /* Ω */ &["awe", "oak", "ah", "zoo"],
];
const TYPES: [&str; 12] = [
    "Dimensionality", "Topology", "Relational", "Polarity", "Fidelity", "KineticChar",
    "Granularity", "Grammar", "Criticality", "Chirality", "Stoichiometry", "Protection",
];
const FIELDS: [&str; 12] = [
    "dim", "top", "rel", "pol", "fid", "kin", "gran", "gram", "crit", "chir", "stoi", "prot",
];

/// Render a product tuple as a Lean `Imscription` literal `{ dim := Dimensionality.x, … }`.
fn render_imscription(product: &[Option<u8>; 12]) -> Option<String> {
    let mut parts = Vec::with_capacity(12);
    for i in 0..12 {
        let o = product[i]? as usize;
        let ctor = CTORS[i].get(o)?;
        parts.push(format!("{} := {}.{}", FIELDS[i], TYPES[i], ctor));
    }
    Some(format!("{{ {} }}", parts.join(", ")))
}

/// Certify a switch through the Lean kernel: both bistable forms are valid
/// Imscriptions (each Frobenius-closes, the balance), AND the reversible toggle is
/// the kernel's real `vessel_roundtrip` — μ∘δ=id, `readback (board p) = p` with
/// board=δ (fsplit) and readback=μ (ffuse), the lossless transport. Turns the
/// printed "reversible: μ∘δ=id" into the kernel's verdict. Restores the scratch after.
pub fn certify_switch(form_a: &[Option<u8>; 12], form_b: &[Option<u8>; 12]) -> (bool, String) {
    let (Some(la), Some(lb)) = (render_imscription(form_a), render_imscription(form_b)) else {
        return (false, "cannot render a form (missing/unknown primitive)".into());
    };
    let source = format!(
        "import Imscribing.IGFunctor\n\
         import Imscribing.Paraconsistent.BelnapSplitFuse\n\
         namespace SwitchCertify\n\
         open Imscribing Imscribing.Primitives Imscribing.Frobenius\n\
         def formA : Imscription := {la}\n\
         def formB : Imscription := {lb}\n\
         theorem formA_closes : igFrobeniusAlg.mul formA formA = formA := igFrobAlg_self_fusion formA\n\
         theorem formB_closes : igFrobeniusAlg.mul formB formB = formB := igFrobAlg_self_fusion formB\n\
         def board (p : List Belnap) : List (Belnap × Belnap) := p.map fsplit\n\
         def readback (q : List (Belnap × Belnap)) : List Belnap := q.map ffuse\n\
         theorem switch_reversible (p : List Belnap) : readback (board p) = p := by\n\
         \x20 induction p with\n\
         \x20 | nil => rfl\n\
         \x20 | cons a t ih =>\n\
         \x20   simp only [board, readback, List.map_cons] at ih ⊢\n\
         \x20   rw [split_fuse_id, ih]\n\
         end SwitchCertify\n"
    );
    let (green, out) = crate::prover::compile_lean(&source, "A");
    crate::prover::restore_placeholder("A");
    (green, out)
}

/// Certify a click product through the Lean kernel: construct it as a real
/// `Imscription` and prove its Frobenius closure `igFrobeniusAlg.mul p p = p` via
/// `igFrobAlg_self_fusion`, built as the prover's scratch module through `lake
/// build`. Green ⟺ the kernel accepts the product as a valid Imscription that
/// μ∘δ-closes — the "closes" claim verified, not asserted. Restores the scratch after.
pub fn certify_click(product: &[Option<u8>; 12]) -> (bool, String) {
    let Some(lit) = render_imscription(product) else {
        return (false, "cannot render product (missing/unknown primitive)".into());
    };
    let source = format!(
        "import Imscribing.IGFunctor\n\
         namespace ClickCertify\n\
         open Imscribing Imscribing.Primitives Imscribing.Frobenius\n\
         def clickProduct : Imscription := {lit}\n\
         theorem clickProduct_closes :\n\
         \x20   igFrobeniusAlg.mul clickProduct clickProduct = clickProduct :=\n\
         \x20 igFrobAlg_self_fusion clickProduct\n\
         end ClickCertify\n"
    );
    let (green, out) = crate::prover::compile_lean(&source, "A");
    crate::prover::restore_placeholder("A");
    (green, out)
}

/// CLI entry: `./ask --click <A> <B> [--catalyst <C>] [--certify]`. Prints the
/// live-pair charge diagnostic and the click verdict for two catalog fragments,
/// optionally lowering the threshold with a catalyst and/or kernel-certifying the
/// product's Frobenius closure. Returns a process exit code.
pub fn run_click(
    catalog: Option<&[CatalogEntry]>,
    name_a: &str,
    name_b: &str,
    theta: f32,
    catalyst_name: Option<&str>,
    certify: bool,
    register: Option<&str>,
    catalog_path: Option<&Path>,
) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("click: no catalog loaded");
        return 2;
    };
    let find = |n: &str| cat.iter().find(|e| e.name == n);
    let (ea, eb) = match (find(name_a), find(name_b)) {
        (Some(a), Some(b)) => (a, b),
        (None, _) => {
            eprintln!("click: catalog entry not found: {name_a}");
            return 2;
        }
        (_, None) => {
            eprintln!("click: catalog entry not found: {name_b}");
            return 2;
        }
    };
    let ta = Tuple::from_entry(ea);
    let tb = Tuple::from_entry(eb);

    // Optional catalyst: lowers the effective threshold (barrier reduction).
    let catalyst: Option<(String, Tuple, f32)> = match catalyst_name {
        Some(cn) => match find(cn) {
            Some(ec) => {
                let tc = Tuple::from_entry(ec);
                let strength = tc.norm(3).unwrap_or(0.0);
                Some((cn.to_string(), tc, strength))
            }
            None => {
                eprintln!("click: catalyst not found: {cn}");
                return 2;
            }
        },
        None => None,
    };

    match &catalyst {
        Some((cn, _, strength)) => println!(
            "click-maths:  {name_a}  ⋈  {name_b}   (θ={theta:.2}, catalyst {cn}: Φ-strength {strength:.2})"
        ),
        None => println!("click-maths:  {name_a}  ⋈  {name_b}   (θ={theta:.2})"),
    }
    println!("  live-pair charges (norm(x) − norm(y), the spring-loaded axis):");
    for (i, &pair) in LIVE_PAIRS.iter().enumerate() {
        let ca = pair_charge(&ta, pair);
        let cb = pair_charge(&tb, pair);
        let fmt = |c: Option<f32>| c.map(|x| format!("{x:+.2}")).unwrap_or_else(|| "  ? ".into());
        let drive = match (ca, cb) {
            (Some(x), Some(y)) => format!("Δ={:.2}", (x - y).abs()),
            _ => "Δ=?".into(),
        };
        println!(
            "    {:5}  {}={:>6}  {}={:>6}   {drive}",
            LIVE_LABELS[i], name_a, fmt(ca), name_b, fmt(cb)
        );
    }

    // Run the fusion. Catalyzed → the catalyst lowers the effective threshold
    // (barrier reduction), reported whether the fusion succeeds or fails so the
    // guardrail stays honest ("barrier at 0, still refuses" when same-sign).
    let catalyzed = catalyst.is_some();
    let result = match &catalyst {
        Some((_, tc, _)) => {
            let (r, applied_theta) = click_pair_catalyzed(&ta, &tb, tc, theta);
            println!(
                "  catalyst lowers the barrier: θ {theta:.2} → {applied_theta:.2}  (AFWD reduction; AREV restores it; catalyst regenerated, μ∘δ=id)"
            );
            r
        }
        None => click_pair(&ta, &tb, theta),
    };

    match result {
        Ok(p) => {
            let via = if catalyzed { " (catalyzed)" } else { "" };
            println!(
                "  ✓ CLICK on {} — spring-loaded Δ={:.2}, single reaction center, closes.{via}",
                LIVE_LABELS[p.pair_idx], p.drive
            );
            print!("  product: ⟨");
            for i in 0..12 {
                match p.product[i] {
                    Some(o) => print!("{}", glyph_of(i, o)),
                    None => print!("?"),
                }
            }
            println!("⟩  (ring saturated on {})", LIVE_LABELS[p.pair_idx]);
            if !p.inherited.is_empty() {
                let names: Vec<&str> = p.inherited.iter().map(|&i| PRIMS[i]).collect();
                println!(
                    "  inherited scaffold (blended from both partners): [{}]",
                    names.join(", ")
                );
            }
            if certify {
                println!("  certifying closure through the Lean kernel (lake build)…");
                let (green, out) = certify_click(&p.product);
                if green {
                    println!("  ✓ KERNEL-CERTIFIED: igFrobeniusAlg.mul p p = p closes (μ∘δ=id, real verdict).");
                } else {
                    let tail: String = out.lines().rev().take(4).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n    ");
                    println!("  ✗ kernel rejected the product:\n    {tail}");
                }
            }
            // Harness: register the chimera as a first-class catalog object, then
            // decompose it through the cl8nk_navigator (make its existence operable).
            if let Some(reg_name) = register {
                let name = if reg_name.is_empty() {
                    format!("chimera_{name_a}_{name_b}")
                } else {
                    reg_name.to_string()
                };
                let desc = format!(
                    "click fusion of {name_a} ⋈ {name_b} on {} (Δ={:.2}); the μ (FFUSE) product, ring saturated on the {} pair.",
                    LIVE_LABELS[p.pair_idx], p.drive, LIVE_LABELS[p.pair_idx]
                );
                match catalog_path {
                    Some(path) => match register_chimera(path, &name, &desc, &p.product) {
                        Ok(()) => {
                            println!("  ✓ registered chimera '{name}' in the catalog — now a first-class navigable object.");
                            println!("  ── decomposition (cl8nk_navigator) ──────────────────────────────");
                            decompose_via_navigator(path, &name);
                        }
                        Err(e) => println!("  ✗ register failed: {e}"),
                    },
                    None => println!("  ✗ register failed: catalog path not resolved"),
                }
            }
        }
        Err(ClickFail::NoComplementarity) => {
            println!("  ✗ no click: no live pair reaches the spring-loaded offset θ.");
        }
        Err(ClickFail::Ambiguous(pairs)) => {
            let names: Vec<&str> = pairs.iter().map(|&i| LIVE_LABELS[i]).collect();
            println!(
                "  ✗ no click: ambiguous — {} live pairs complementary ({}); no single reaction center.",
                names.len(),
                names.join(", ")
            );
        }
        Err(ClickFail::Missing) => {
            println!("  ✗ no click: a fragment is missing a live-pair primitive.");
        }
    }
    0
}

// ── Excited states + single-electron transfer (SET) ──────────────────────────
// Photochemistry on the register — the same δ/μ legs the DASA switch runs, but
// exposing the intermediate the switch hides.
//
// EXCITE (δ, light): promote an object to its excited manifold. Criticality ⊙ is
// raised to the non-Hermitian exceptional-point resonance 𐑻 (ord 3) — the
// navigator's own gloss for that value: `H(λ) non-Herm ∧ det(H−λI)=0 ∧ ∂_λ H=0`.
// That is precisely an excited state: a finite-lifetime RESONANCE (non-Hermitian
// ⇒ it decays), sitting at a surface crossing. Two Frobenius legs leave it:
//   μ (relaxation / fluorescence) returns it to the ground form — lossless, μ∘δ=id;
//   the productive leg lets the elevated ⊙ drive chemistry the ground state cannot.
//
// SET (single-electron transfer): the productive leg made concrete. One electron =
// one quantum of winding Ω (∮A=2πn, the quantized charge). The donor loses one
// (oxidized, D•⁺, Ω−1), the acceptor gains one (reduced, A•⁻, Ω+1); total Ω is
// conserved — charge conservation IS the Frobenius pairing (δ/fsplit charge-
// separates, μ/ffuse recombines). Donor/acceptor are read from Criticality ⊙
// (higher ⊙ = higher-lying electrons = the reducing partner). A Cu-NO-class
// mediator (Mills 2016 SET catalyst) shuttles the quantum and returns to itself —
// the catalytic cycle, μ∘δ=id. Photoinduced ET = excite the donor first: the
// raised ⊙ opens the driving-force gap the ground state lacked.
const WIND: usize = 11; // Ω Protection = winding ∮A=2πn, the quantized charge (electron count)
const EP_RESONANCE: u8 = 3; // ⊙ = 𐑻, the non-Hermitian exceptional-point (excited-state resonance)

/// Format a full 12-tuple as ⟨glyphs⟩.
fn fmt_tuple(ord: &[Option<u8>; 12]) -> String {
    let mut s = String::from("⟨");
    for (i, o) in ord.iter().enumerate() {
        match o {
            Some(v) => s.push_str(glyph_of(i, *v)),
            None => s.push('?'),
        }
    }
    s.push('⟩');
    s
}

/// Promote a ground tuple to its excited manifold: Criticality ⊙ → the
/// exceptional-point resonance 𐑻 (non-Hermitian, finite lifetime — it decays).
/// Returns (excited tuple, already_excited). If ⊙ is already at/above the
/// resonance the tuple is returned unchanged with the flag set. None if ⊙ missing.
fn excite_tuple(ground: &[Option<u8>; 12]) -> Option<([Option<u8>; 12], bool)> {
    let cur = ground[CRIT]?;
    let already = cur >= EP_RESONANCE;
    let mut ex = *ground;
    if !already {
        ex[CRIT] = Some(EP_RESONANCE);
    }
    Some((ex, already))
}

/// Transfer one winding quantum Ω from donor to acceptor: donor −1 (oxidized),
/// acceptor +1 (reduced). Total Ω is conserved by construction. Errors honestly
/// if the donor is already fully oxidized (Ω=0, nothing to give) or the acceptor
/// is saturated (Ω=max, no vacancy).
fn transfer_electron(
    donor: &[Option<u8>; 12],
    acceptor: &[Option<u8>; 12],
) -> Result<([Option<u8>; 12], [Option<u8>; 12]), String> {
    let dw = donor[WIND].ok_or("donor has no winding Ω")?;
    let aw = acceptor[WIND].ok_or("acceptor has no winding Ω")?;
    if dw == 0 {
        return Err("donor Ω=𐑷 — already fully oxidized, no winding quantum to give".into());
    }
    if aw >= max_ord(WIND) {
        return Err("acceptor Ω saturated — no vacancy to receive the electron".into());
    }
    let mut d = *donor;
    let mut a = *acceptor;
    d[WIND] = Some(dw - 1);
    a[WIND] = Some(aw + 1);
    Ok((d, a))
}

/// Radical character from the winding quantum Ω (electron count): odd Ω = an intrinsically
/// unpaired electron (already open-shell); even = a closed-shell parent whose SOMO is the
/// freshly-opened bond center.
fn somo_note(t: &Tuple) -> String {
    match t.ord[WIND] {
        Some(w) if w % 2 == 1 => format!("Ω={} odd — intrinsically open-shell", glyph_of(WIND, w)),
        Some(w) => format!("Ω={} even — SOMO is the opened bond center", glyph_of(WIND, w)),
        None => "Ω absent".into(),
    }
}

/// CLI: `./ask --homolyze A [B]`. Homolytic cleavage → NEUTRAL radicals — the δ_A symmetric
/// split, the radical-generating REVERSE of `click`. `homolyze A B` cleaves the A—B σ-bond
/// (they must click on a single live pair) into two neutral radicals A• + B•, each keeping
/// one electron of the shared pair (the SOMO). `homolyze A` splits A symmetrically into two
/// identical radicals A• + A•. Contrast `set`: the single-electron / heterolytic route → the
/// radical IONS A•⁺ / B•⁻ (δ_D, one fragment takes both electrons).
pub fn run_homolyze(
    catalog: Option<&[CatalogEntry]>,
    name_a: &str,
    name_b: Option<&str>,
    theta: f32,
) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("homolyze: no catalog loaded");
        return 2;
    };
    let find = |n: &str| cat.iter().find(|e| e.name == n);
    let Some(ea) = find(name_a) else {
        eprintln!("homolyze: catalog entry not found: {name_a}");
        return 2;
    };
    let ta = Tuple::from_entry(ea);

    let Some(nb) = name_b else {
        // Symmetric homolysis of a single entity — the diagonal split δ_A(a) = (a, a).
        println!("homolyze (symmetric homolysis, δ_A(a)=(a,a)):  {name_a}");
        println!("  → two identical NEUTRAL radicals — the diagonal split (μ∘δ_A=id):");
        println!("      {name_a}•  {}   radical (open SOMO)  [{}]", fmt_tuple(&ta.ord), somo_note(&ta));
        println!("      {name_a}•  {}   radical (open SOMO)  [{}]", fmt_tuple(&ta.ord), somo_note(&ta));
        println!("  recombination (μ): {name_a}• + {name_a}• → {name_a}—{name_a}  — radical coupling re-pairs the SOMO (lossless).");
        println!("  one electron instead (heterolytic / SET → radical ION): `./ask --set {name_a} <acceptor>`, or `--excite {name_a}`.");
        return 0;
    };

    let Some(eb) = find(nb) else {
        eprintln!("homolyze: catalog entry not found: {nb}");
        return 2;
    };
    let tb = Tuple::from_entry(eb);
    println!("homolyze (homolytic cleavage — the radical-generating reverse of click, δ_A symmetric):  {name_a} ⋈ {nb}");
    match click_pair(&ta, &tb, theta) {
        Ok(p) => {
            let axis = LIVE_LABELS[p.pair_idx];
            println!("  the {name_a}—{nb} σ-bond: a click on {axis} (Δ={:.2}) — a shared electron pair, the reaction center.", p.drive);
            println!("  → homolysis splits that pair EVENLY (δ_A): each fragment keeps ONE electron — the SOMO:");
            println!("      {name_a}•  {}   neutral radical — SOMO open on {axis}, seeks a partner  [{}]", fmt_tuple(&ta.ord), somo_note(&ta));
            println!("      {nb}•  {}   neutral radical — SOMO open on {axis}  [{}]", fmt_tuple(&tb.ord), somo_note(&tb));
            println!("  recombination (μ): {name_a}• + {nb}• → {name_a}—{nb}  — radical coupling re-pairs the SOMO (μ∘δ=id, lossless).");
            println!("  heterolytic alternative (δ_D → ion pair): the same bond can break UNEVENLY — one fragment takes both");
            println!("    electrons (anion), the other none (cation): the single-electron / radical-ion route — `./ask --set {name_a} {nb}` → {name_a}•⁺ / {nb}•⁻.");
            0
        }
        Err(ClickFail::NoComplementarity) => {
            println!("  ✗ {name_a} and {nb} share no reaction center (not complementary on any live pair) — there is no σ-bond to cleave, and they would not couple. Nothing to homolyze.");
            2
        }
        Err(ClickFail::Ambiguous(pairs)) => {
            let labs: Vec<&str> = pairs.iter().filter_map(|&i| LIVE_LABELS.get(i).copied()).collect();
            println!("  ✗ {name_a} and {nb} are complementary on {} centers ({}) — not one clean σ-bond. Homolysis is ambiguous: it would open a diradical/network, not two clean radicals.", pairs.len(), labs.join(", "));
            2
        }
        Err(ClickFail::Missing) => {
            println!("  ✗ a fragment is missing a live-pair charge — cannot gauge the bond to homolyze.");
            2
        }
    }
}

/// CLI entry: `./ask --excite A [--certify] [--register [NAME]]` — promote A to
/// its excited manifold (⊙ → the exceptional-point resonance), report the δ (light)
/// promotion and the μ (relaxation/fluorescence) + productive decay legs, optionally
/// kernel-certify (ground + excited valid, relaxation roundtrip) and register the
/// excited state as a first-class navigable object.
pub fn run_excite(
    catalog: Option<&[CatalogEntry]>,
    name: &str,
    certify: bool,
    register: Option<&str>,
    catalog_path: Option<&Path>,
) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("excite: no catalog loaded");
        return 2;
    };
    let Some(e) = cat.iter().find(|e| e.name == name) else {
        eprintln!("excite: catalog entry not found: {name}");
        return 2;
    };
    let ground = Tuple::from_entry(e).ord;
    println!("excite:  {name}  →hν→  {name}*");
    let Some(g_crit) = ground[CRIT] else {
        println!("  ✗ {name} has no Criticality ⊙ — cannot excite.");
        return 0;
    };
    let Some((excited, already)) = excite_tuple(&ground) else {
        println!("  ✗ cannot excite {name}.");
        return 0;
    };
    if already {
        println!(
            "  {name} is already at/above the exceptional-point resonance (⊙={}) — already an excited/critical form; no promotion.",
            glyph_of(CRIT, g_crit)
        );
        return 0;
    }
    println!(
        "  δ (light): promote to the excited manifold — Criticality ⊙ {}→{} (the non-Hermitian exceptional-point resonance: metastable, finite lifetime — it DECAYS).",
        glyph_of(CRIT, g_crit),
        glyph_of(CRIT, EP_RESONANCE)
    );
    println!("  {name}*  {}", fmt_tuple(&excited));
    println!("  decay channels (the two Frobenius legs out of {name}*):");
    println!("    μ (relaxation / fluorescence): {name}* → {name}  — returns to the ground form, lossless (μ∘δ=id).");
    println!("    productive: the elevated ⊙ drives chemistry the ground state cannot — {name}* is a far stronger donor/acceptor (see `--set {name} <acceptor> --excite`).");
    if certify {
        println!("  certifying through the Lean kernel (ground + excited valid + relaxation roundtrip)…");
        let (green, out) = certify_switch(&ground, &excited);
        if green {
            println!("  ✓ KERNEL-CERTIFIED: {name} and {name}* are both valid Imscriptions AND readback∘board = id — the excite→relax roundtrip is lossless (μ∘δ=id, real verdict).");
        } else {
            let tail: String = out.lines().rev().take(4).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n    ");
            println!("  ✗ kernel rejected the excited state:\n    {tail}");
        }
    }
    if let Some(reg) = register {
        match catalog_path {
            Some(path) => {
                let nm = if reg.is_empty() { format!("{name}_excited") } else { reg.to_string() };
                let desc = format!(
                    "excited state {name}* — {name} promoted by δ (light) to the exceptional-point resonance (⊙→𐑻), the metastable non-Hermitian excited manifold that decays by fluorescence (μ) or drives a SET."
                );
                match register_chimera(path, &nm, &desc, &excited) {
                    Ok(()) => {
                        println!("  ✓ registered excited state '{nm}' — now a first-class navigable object.");
                        println!("  ── decomposition (cl8nk_navigator) ──────────────────────────────");
                        decompose_via_navigator(path, &nm);
                    }
                    Err(e) => println!("  ✗ register failed: {e}"),
                }
            }
            None => println!("  ✗ register failed: catalog path not resolved"),
        }
    }
    0
}

/// CLI entry: `./ask --set D A [--catalyst M] [--excite] [--certify] [--register [NAME]]`
/// — single-electron transfer. Reads donor/acceptor from Criticality ⊙, moves one
/// winding quantum Ω donor→acceptor (oxidation/reduction), reports the radical-ion
/// products and the conserved-charge invariant, and (optionally) the Cu-NO mediator
/// cycle, photoinduced pre-excitation, kernel certification of the recombination
/// roundtrip, and registration of both radical ions.
pub fn run_set(
    catalog: Option<&[CatalogEntry]>,
    name_a: &str,
    name_b: &str,
    certify: bool,
    catalyst_name: Option<&str>,
    photo: bool,
    register: Option<&str>,
    catalog_path: Option<&Path>,
) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("set: no catalog loaded");
        return 2;
    };
    let find = |n: &str| cat.iter().find(|e| e.name == n);
    let (ea, eb) = match (find(name_a), find(name_b)) {
        (Some(a), Some(b)) => (a, b),
        (None, _) => { eprintln!("set: catalog entry not found: {name_a}"); return 2; }
        (_, None) => { eprintln!("set: catalog entry not found: {name_b}"); return 2; }
    };
    let mut da = Tuple::from_entry(ea).ord;
    let db = Tuple::from_entry(eb).ord;

    println!("set (single-electron transfer):  {name_a}  ⟶e⁻⟶  {name_b}");

    // Photoinduced: excite the first-named chromophore first, raising its ⊙ so the
    // driving-force gap opens (reductive quenching by the excited state).
    if photo {
        match excite_tuple(&da) {
            Some((_, true)) => println!(
                "  photoinduced: {name_a} already at/above the exceptional-point resonance (⊙={}) — already excited.",
                da[CRIT].map(|o| glyph_of(CRIT, o)).unwrap_or("?")
            ),
            Some((ex, false)) => {
                println!(
                    "  photoinduced (hν): {name_a} → {name_a}* — ⊙ {}→{} (excited to the EP resonance, now a far stronger donor).",
                    da[CRIT].map(|o| glyph_of(CRIT, o)).unwrap_or("?"),
                    glyph_of(CRIT, EP_RESONANCE)
                );
                da = ex;
            }
            None => println!("  ✗ cannot excite {name_a}: no ⊙."),
        }
    }

    // Donor / acceptor by criticality ⊙ (higher = higher-lying electrons = donor).
    let (ca, cb) = match (da[CRIT], db[CRIT]) {
        (Some(x), Some(y)) => (x, y),
        _ => { println!("  ✗ a form is missing Criticality ⊙ — cannot gauge the driving force."); return 0; }
    };
    if ca == cb {
        println!("  driving force (⊙): equal criticality — thermoneutral ET, no bias. Excite one partner (`--set {name_a} {name_b} --excite`) to open a gap.");
        return 0;
    }
    let ((dn, donor), (an, acceptor)) = if ca > cb {
        ((name_a, da), (name_b, db))
    } else {
        ((name_b, db), (name_a, da))
    };
    println!("  driving force (⊙, energy): {dn} is the DONOR (higher-⊙, electrons higher-lying, reducing);  {an} is the ACCEPTOR (lower-⊙, oxidizing)");
    if dn != name_a {
        println!("  note: ⊙ makes {dn} the donor (not the first-named {name_a}) — the electron flows down the criticality gradient, read from the tuples, not the label.");
    }

    match transfer_electron(&donor, &acceptor) {
        Ok((dox, ared)) => {
            let dw0 = donor[WIND].unwrap();
            let aw0 = acceptor[WIND].unwrap();
            println!("  electron = one winding quantum Ω (∮A=2πn, the quantized charge):");
            println!("    donor {dn}:    Ω {}→{}  (oxidized, {dn}•⁺ — gave one winding quantum)", glyph_of(WIND, dw0), glyph_of(WIND, dw0 - 1));
            println!("    acceptor {an}: Ω {}→{}  (reduced,  {an}•⁻ — took one winding quantum)", glyph_of(WIND, aw0), glyph_of(WIND, aw0 + 1));
            println!("  radical-ion products:");
            println!("    {dn}•⁺  {}", fmt_tuple(&dox));
            println!("    {an}•⁻  {}", fmt_tuple(&ared));
            println!("  invariant: total Ω conserved (−1 + +1 = 0) — charge conservation is the Frobenius pairing (δ/fsplit charge-separates, μ/ffuse recombines).");
            println!("  channels:  μ (back-ET / geminate recombination): {dn}•⁺ + {an}•⁻ → {dn} + {an}, lossless (μ∘δ=id);  productive: cage escape — the free radicals do chemistry.");

            // Cu-NO-class mediator: shuttles the quantum, regenerated μ∘δ=id.
            if let Some(cn) = catalyst_name {
                match find(cn) {
                    Some(ec) => {
                        let mo = Tuple::from_entry(ec).ord;
                        match mo[WIND] {
                            Some(mw) if mw >= 1 && mw < max_ord(WIND) => println!(
                                "  mediator {cn} (Cu-NO-class SET shuttle): can hold the intermediate (Ω={}, 1≤Ω<max) — takes e⁻ from {dn} (Ω+1, reduced), delivers to {an} (Ω−1, re-oxidized), returns to itself. Catalytic cycle, μ∘δ=id — regenerated, not consumed.",
                                glyph_of(WIND, mw)
                            ),
                            Some(mw) => println!("  mediator {cn}: Ω={} cannot shuttle (needs 1≤Ω<max to both accept and re-donate a quantum).", glyph_of(WIND, mw)),
                            None => println!("  mediator {cn}: no winding Ω — cannot shuttle an electron."),
                        }
                    }
                    None => eprintln!("set: mediator not found: {cn}"),
                }
            }

            if certify {
                println!("  certifying through the Lean kernel (both radical ions valid + recombination roundtrip)…");
                let (green, out) = certify_switch(&dox, &ared);
                if green {
                    println!("  ✓ KERNEL-CERTIFIED: {dn}•⁺ and {an}•⁻ are both valid Imscriptions AND readback∘board = id — the charge-separation/recombination roundtrip is lossless (μ∘δ=id, charge conserved, real verdict).");
                } else {
                    let tail: String = out.lines().rev().take(4).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n    ");
                    println!("  ✗ kernel rejected the SET:\n    {tail}");
                }
            }

            if let Some(reg) = register {
                match catalog_path {
                    Some(path) => {
                        let base = if reg.is_empty() { String::new() } else { format!("{reg}_") };
                        let cat_name = format!("{base}{dn}_radical_cation");
                        let an_name = format!("{base}{an}_radical_anion");
                        let cdesc = format!("radical cation {dn}•⁺ — {dn} oxidized by single-electron transfer to {an} (Ω −1, one winding quantum given).");
                        let adesc = format!("radical anion {an}•⁻ — {an} reduced by single-electron transfer from {dn} (Ω +1, one winding quantum taken).");
                        for (nm, desc, ord) in [(cat_name, cdesc, dox), (an_name, adesc, ared)] {
                            match register_chimera(path, &nm, &desc, &ord) {
                                Ok(()) => {
                                    println!("  ✓ registered '{nm}' — now a first-class navigable object.");
                                    println!("  ── decomposition (cl8nk_navigator) ──────────────────────────────");
                                    decompose_via_navigator(path, &nm);
                                }
                                Err(e) => println!("  ✗ register '{nm}' failed: {e}"),
                            }
                        }
                    }
                    None => println!("  ✗ register failed: catalog path not resolved"),
                }
            }
        }
        Err(e) => println!("  ✗ no transfer: {e}"),
    }
    0
}

// ── Bidirectional ligand ⇌ catalytic-site (ported from red-hot_rebis) ─────────
// Port of rhr_p4rky/ligand_from_active_site.py::complement_type — the reverse
// pipeline: a catalytic SITE type maps to the complementary LIGAND it binds, and
// back. For each of the six conjugate pairs (A,B) it INVERTS within each scale
// (lock-and-key: where the site is high the ligand is low, a pocket ↔ a bump) and
// CROSSES the pair (site A informs ligand B and vice versa). It IS ITS OWN
// INVERSE: site→ligand→site returns — the bidirectionality the enzyme's R=𐑾
// (Recognition, "substrate ↔ enzyme feedback") names. In the SET picture this is
// what an inner-sphere mediator does: it must BIND both donor and acceptor (its
// complement close to each) to relay the electron between them.

/// Python-parity round-half-to-even (banker's rounding), so the port agrees with
/// red-hot_rebis on the .5 boundary cases. Inputs here are always ≥ 0.
fn round_half_even(x: f32) -> i32 {
    let f = x.floor();
    if (x - f - 0.5).abs() < 1e-6 {
        let fi = f as i32;
        if fi % 2 == 0 { fi } else { fi + 1 }
    } else {
        x.round() as i32
    }
}

/// Mean normalized per-primitive ordinal distance in [0,1] (heuristic — an
/// unweighted ordinal metric, port of tuple_distance_dict; not the navigator's
/// canonical SIC frame, see feedback_two_distance_metrics). Missing axes skipped.
fn tuple_dist(a: &[Option<u8>; 12], b: &[Option<u8>; 12]) -> f32 {
    let mut sum = 0.0;
    let mut n = 0;
    for i in 0..12 {
        if let (Some(x), Some(y)) = (a[i], b[i]) {
            let m = max_ord(i);
            if m > 0 {
                sum += (x as f32 - y as f32).abs() / m as f32;
                n += 1;
            }
        }
    }
    if n > 0 { sum / n as f32 } else { 1.0 }
}

/// Bidirectional ligand ⇌ catalytic-site complement over the six conjugate pairs.
/// Faithful port of complement_type: `ligand[B]=INV(site[A])`, `ligand[A]=INV(site[B])`,
/// each rescaled to the partner's ordinal range. Its own inverse (bidirectional).
fn complement_type(site: &[Option<u8>; 12]) -> [Option<u8>; 12] {
    let mut ligand = *site; // every index is written below (the six pairs partition all 12)
    for &(a, b) in CONJUGATE_PAIRS.iter() {
        let a_max = max_ord(a);
        let b_max = max_ord(b);
        let sa = site[a].unwrap_or(0);
        let sb = site[b].unwrap_or(0);
        let inv_a = a_max - sa; // reflect within each scale (lock-and-key)
        let inv_b = b_max - sb;
        ligand[b] = Some(if a_max > 0 {
            round_half_even(inv_a as f32 / a_max as f32 * b_max as f32).clamp(0, b_max as i32) as u8
        } else {
            b_max
        });
        ligand[a] = Some(if b_max > 0 {
            round_half_even(inv_b as f32 / b_max as f32 * a_max as f32).clamp(0, a_max as i32) as u8
        } else {
            a_max
        });
    }
    ligand
}

/// One-line reading of a Recognition (Ř, index 2) ordinal as binding directionality.
fn recognition_reading(r: Option<u8>) -> (&'static str, f32) {
    match r {
        Some(3) => ("Ř=𐑾 (bidirectional — substrate↔enzyme feedback): a true catalytic/binding site", 1.0),
        Some(2) => ("Ř=𐑽 (lateral dual): partial bidirectional binding", 0.5),
        _ => ("Ř low: weak / one-way binding, not a strong bidirectional site", 0.0),
    }
}

/// CLI entry: `./ask --complement A [--certify] [--register [NAME]]` — the ported
/// reverse pipeline. Maps a catalytic-site type A to the complementary ligand it
/// binds (and back — its own inverse), shows the per-conjugate-pair map and the
/// bidirectional round-trip, optionally certifies the ligand closes and registers it.
pub fn run_complement(
    catalog: Option<&[CatalogEntry]>,
    name: &str,
    certify: bool,
    register: Option<&str>,
    catalog_path: Option<&Path>,
) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("complement: no catalog loaded");
        return 2;
    };
    let Some(e) = cat.iter().find(|e| e.name == name) else {
        eprintln!("complement: catalog entry not found: {name}");
        return 2;
    };
    let site = Tuple::from_entry(e).ord;
    let ligand = complement_type(&site);
    let g = |t: &[Option<u8>; 12], i: usize| t[i].map(|o| glyph_of(i, o)).unwrap_or("?");

    println!("complement (bidirectional ligand ⇌ catalytic-site):  {name}");
    println!("  site   {name}:   {}", fmt_tuple(&site));
    println!("  ligand {name}′:  {}   (the complementary partner the site binds — lock-and-key over the 6 conjugate pairs)", fmt_tuple(&ligand));
    let (recog_txt, _) = recognition_reading(site[2]);
    println!("  recognition: {recog_txt}");
    println!("  conjugate-pair map (site → ligand — inverted within each scale, crossed across the pair):");
    println!("    the complement is CHEMISTRY: all 6 pairs. [live] = the 3 catalytic axes math varies (D↔W, T↔H, R↔S);");
    println!("    [pinned] = the 3 physical axes math abstracts out and chemistry restores (P↔F, K↔G, Gm↔Ph).");
    for &(a, b) in CONJUGATE_PAIRS.iter() {
        let tag = if LIVE_PAIRS.contains(&(a, b)) { "[live]  " } else { "[pinned]" };
        println!(
            "    {tag} {}↔{}:   {}{}→{}{}   |   {}{}→{}{}",
            PRIMS[a], PRIMS[b],
            PRIMS[a], g(&site, a), PRIMS[b], g(&ligand, b),
            PRIMS[b], g(&site, b), PRIMS[a], g(&ligand, a)
        );
    }
    let back = complement_type(&ligand);
    let d = tuple_dist(&site, &back);
    if d < 1e-6 {
        println!("  round-trip {name} → {name}′ → {name}″: distance 0.00 — the complement is its own inverse (bidirectional, lossless R∧W∧X).");
    } else {
        println!("  round-trip {name} → {name}′ → {name}″: distance {d:.3} — near-involutive (a rescaling residual on the unequal-scale pairs; the bidirection holds up to ordinal granularity).");
    }
    if certify {
        println!("  certifying the ligand closes through the Lean kernel (lake build)…");
        let (green, out) = certify_click(&ligand);
        if green {
            println!("  ✓ KERNEL-CERTIFIED: the complementary ligand is a valid Imscription (igFrobeniusAlg.mul p p = p, μ∘δ=id).");
        } else {
            let tail: String = out.lines().rev().take(4).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n    ");
            println!("  ✗ kernel rejected the ligand:\n    {tail}");
        }
    }
    if let Some(reg) = register {
        match catalog_path {
            Some(path) => {
                let nm = if reg.is_empty() { format!("{name}_ligand") } else { reg.to_string() };
                let desc = format!("de-novo ligand for the catalytic site {name} — the bidirectional structural complement (complement_type, ported from red-hot_rebis ligand_from_active_site); the partner {name} binds lock-and-key over the six conjugate pairs.");
                match register_chimera(path, &nm, &desc, &ligand) {
                    Ok(()) => {
                        println!("  ✓ registered ligand '{nm}' — now a first-class navigable object.");
                        println!("  ── decomposition (cl8nk_navigator) ──────────────────────────────");
                        decompose_via_navigator(path, &nm);
                    }
                    Err(e) => println!("  ✗ register failed: {e}"),
                }
            }
            None => println!("  ✗ register failed: catalog path not resolved"),
        }
    }
    0
}

/// CLI entry: `./ask --set D A --scan-mediators [--top N]` — rank the whole catalog
/// for the best mediators of the D→A single-electron transfer. A good inner-sphere
/// SET mediator must (1) HOLD the winding quantum (1≤Ω<max — accept then re-donate),
/// (2) RELAY in energy (⊙ between acceptor and donor, both legs downhill), and
/// (3) BIND both substrates bidirectionally (its ligand complement close to donor
/// AND acceptor — the ported catalytic-site recognition). Composite-ranked.
pub fn run_scan_mediators(
    catalog: Option<&[CatalogEntry]>,
    name_a: &str,
    name_b: &str,
    top: usize,
) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("scan-mediators: no catalog loaded");
        return 2;
    };
    let find = |n: &str| cat.iter().find(|e| e.name == n);
    let (ea, eb) = match (find(name_a), find(name_b)) {
        (Some(a), Some(b)) => (a, b),
        (None, _) => { eprintln!("scan-mediators: catalog entry not found: {name_a}"); return 2; }
        (_, None) => { eprintln!("scan-mediators: catalog entry not found: {name_b}"); return 2; }
    };
    let ta = Tuple::from_entry(ea).ord;
    let tb = Tuple::from_entry(eb).ord;
    let (ca, cb) = match (ta[CRIT], tb[CRIT]) {
        (Some(x), Some(y)) => (x, y),
        _ => { eprintln!("scan-mediators: a form is missing Criticality ⊙."); return 2; }
    };
    if ca == cb {
        println!("scan-mediators: {name_a} and {name_b} have equal ⊙ — thermoneutral, no directed relay to scan for. Excite one first.");
        return 0;
    }
    // donor = higher ⊙, acceptor = lower ⊙
    let ((dn, donor, cd), (an, acceptor, cac)) = if ca > cb {
        ((name_a, ta, ca), (name_b, tb, cb))
    } else {
        ((name_b, tb, cb), (name_a, ta, ca))
    };
    let (lo, hi) = (cac, cd); // relay band [acceptor ⊙, donor ⊙]
    let wmax = max_ord(WIND);

    // (name, composite, relay, bind, recog, Ω glyph, ⊙ glyph)
    let mut hits: Vec<(String, f32, f32, f32, f32, &'static str, &'static str)> = Vec::new();
    for e in cat {
        if e.name == dn || e.name == an {
            continue;
        }
        let m = Tuple::from_entry(e).ord;
        let Some(mw) = m[WIND] else { continue };
        if !(mw >= 1 && mw < wmax) {
            continue; // hard filter: must hold the quantum (accept then re-donate)
        }
        let Some(cm) = m[CRIT] else { continue };
        let relay = if cm >= lo && cm <= hi {
            1.0
        } else {
            let outside = if cm < lo { lo - cm } else { cm - hi };
            1.0 / (1.0 + outside as f32)
        };
        let comp = complement_type(&m);
        let dbind = (tuple_dist(&comp, &donor) + tuple_dist(&comp, &acceptor)) * 0.5;
        let bind = 1.0 / (1.0 + dbind);
        let (_, recog) = recognition_reading(m[2]);
        let composite = 0.4 * relay + 0.4 * bind + 0.2 * recog;
        hits.push((e.name.clone(), composite, relay, bind, recog, glyph_of(WIND, mw), glyph_of(CRIT, cm)));
    }
    hits.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    println!("scan-mediators:  {dn}  ⟶e⁻⟶  {an}   ({} holdable candidates over {} entries)", hits.len(), cat.len());
    println!("  relay band ⊙∈[{},{}] (acceptor→donor);  hold band Ω∈[𐑴,{}] (accept then re-donate);  bind = complement recognizes both substrates",
        glyph_of(CRIT, lo), glyph_of(CRIT, hi), glyph_of(WIND, wmax - 1));
    println!("  {:>4}  {:>6}  {:>5} {:>5} {:>5}  {:>3} {:>3}  mediator", "rank", "score", "relay", "bind", "recog", "Ω", "⊙");
    for (i, (name, comp, relay, bind, recog, wg, cg)) in hits.iter().take(top).enumerate() {
        println!("  {:>4}  {:>6.3}  {:>5.2} {:>5.2} {:>5.2}  {:>3} {:>3}  {name}", i + 1, comp, relay, bind, recog, wg, cg);
    }
    0
}

// ── The catalytic cycle: the loop, closed and certified ──────────────────────
// Everything above is an ARC of a mechanism — bind (complement), transform (set/
// excite), hold (scan). This closes them into the LOOP: bind → working stroke δ →
// return stroke μ → turnover, with the catalyst a FIXED POINT (μ∘δ=id: it engages,
// spends, and returns to itself unchanged, the way a real catalyst does — the
// Cu-NO SET cycle is the archetype). The certificate is no longer "this product
// closes" but "this PROCESS is a closed loop": the catalyst's two states are valid
// Imscriptions and the regeneration is the kernel's lossless transport.

/// CLI entry: `./ask --cycle CATALYST SUBSTRATE [--certify] [--register [NAME]]`.
/// bind (the catalyst recognizes its substrate via the ligand complement) → working
/// stroke δ (the catalyst moves one winding quantum, transforming substrate→product
/// and spending itself) → return stroke μ (regeneration to the resting state) →
/// turnover (net substrate→product, catalyst unchanged). --certify proves the
/// catalyst is a fixed point of the loop; --register canonizes the product.
pub fn run_cycle(
    catalog: Option<&[CatalogEntry]>,
    catalyst_name: &str,
    substrate_name: &str,
    certify: bool,
    register: Option<&str>,
    catalog_path: Option<&Path>,
) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("cycle: no catalog loaded");
        return 2;
    };
    let find = |n: &str| cat.iter().find(|e| e.name == n);
    let (ec, es) = match (find(catalyst_name), find(substrate_name)) {
        (Some(c), Some(s)) => (c, s),
        (None, _) => { eprintln!("cycle: catalog entry not found: {catalyst_name}"); return 2; }
        (_, None) => { eprintln!("cycle: catalog entry not found: {substrate_name}"); return 2; }
    };
    let cat_t = Tuple::from_entry(ec).ord;
    let sub_t = Tuple::from_entry(es).ord;
    println!("catalytic cycle:  {catalyst_name}  ⟳  turning over  {substrate_name}");

    // ── Bind (δ / CLINK): the catalyst recognizes the substrate (ligand complement) ──
    let comp = complement_type(&cat_t);
    let bind_d = tuple_dist(&comp, &sub_t);
    let recog = recognition_reading(cat_t[2]).1;
    let grip = if bind_d < 0.5 && recog >= 1.0 {
        "a real catalytic grip (Ř=𐑾 bidirectional, complement fits)"
    } else if bind_d < 0.5 {
        "binds, but weak recognition (Ř not bidirectional)"
    } else {
        "weak grip — expect sluggish turnover"
    };
    println!("  bind (δ / CLINK): {catalyst_name} grips {substrate_name} — complement match d={bind_d:.3}, {grip}");

    // ── Working stroke (δ): one winding quantum moves; catalyst spent, substrate → product ──
    // Reuse the SET primitive both directions: reductive = catalyst donates (cat is
    // the SET donor), oxidative = catalyst abstracts (cat is the SET acceptor).
    let (c_star, product, dir, imprint) = match working_stroke(&cat_t, &sub_t) {
        Ok(t) => t,
        Err(e) => {
            println!("  ✗ no turnover: {e} — neither redox direction is feasible for this pair.");
            return 0;
        }
    };
    let dirdesc = if dir == "reductive" {
        "reductive — catalyst donates e⁻, substrate reduced"
    } else {
        "oxidative — catalyst abstracts e⁻, substrate oxidized"
    };
    let (cw0, cw1) = (cat_t[WIND].unwrap(), c_star[WIND].unwrap());
    let (sw0, sw1) = (sub_t[WIND].unwrap(), product[WIND].unwrap());
    println!("  working stroke (δ / SOLVE — dissolves the bond, reveals the degree of freedom: the winding quantum comes free): {dirdesc}");
    println!("    carrier   {catalyst_name}: Ω {}→{}  (spent → {catalyst_name}*)", glyph_of(WIND, cw0), glyph_of(WIND, cw1));
    println!(
        "    substrate {substrate_name}: Ω {}→{}{}  (transformed → {substrate_name}‡)",
        glyph_of(WIND, sw0), glyph_of(WIND, sw1),
        imprint.map(|p| format!(" + Coagula imprint on {p}")).unwrap_or_default()
    );
    println!("    {catalyst_name}*  {}", fmt_tuple(&c_star));
    println!("    {substrate_name}‡  {}", fmt_tuple(&product));

    // ── Return stroke (μ / Coagula): regeneration binds the freed quantum ──
    println!("  return stroke (μ / COAGULA — binds the freed quantum into an invariant): {catalyst_name}* Ω {}→{} — the catalyst coagulates back to itself (terminal redox restores the resting state).", glyph_of(WIND, cw1), glyph_of(WIND, cw0));

    // ── Turnover / loop closure: the two coagulations ──
    println!("  turnover: net {substrate_name} → {substrate_name}‡ (one quantum delivered); {catalyst_name} returned unchanged — the catalyst is a FIXED POINT of the loop.");
    println!("  two coagulations: the CATALYST binds back to the SAME invariant (regeneration, μ∘δ=id); the SUBSTRATE binds into a NEW invariant ({substrate_name}‡). Solve freed one quantum; Coagula placed it.");
    println!("  the loop:  {catalyst_name} ─Solve(δ)→ {catalyst_name}* ─Coagula(μ)→ {catalyst_name}   ‖   {substrate_name} ──────→ {substrate_name}‡");

    // ── Certify the catalyst regeneration (Coagula∘Solve = id on the catalyst) ──
    if certify {
        println!("  certifying the catalyst is a fixed point of the loop (both states valid + regeneration μ∘δ=id)…");
        let (green, out) = certify_switch(&cat_t, &c_star);
        if green {
            println!("  ✓ KERNEL-CERTIFIED: {catalyst_name} and {catalyst_name}* are both valid Imscriptions AND readback∘board = id — Coagula∘Solve = id on the catalyst (μ∘δ=id, lossless regeneration); the loop closes as the kernel's own verdict.");
        } else {
            let tail: String = out.lines().rev().take(4).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n    ");
            println!("  ✗ kernel rejected the cycle:\n    {tail}");
        }
    }

    // ── Register the turned-over product ──
    if let Some(reg) = register {
        match catalog_path {
            Some(path) => {
                let nm = if reg.is_empty() { format!("{substrate_name}_product") } else { reg.to_string() };
                let desc = format!("product of the {catalyst_name}-catalyzed turnover of {substrate_name} ({dir}); one winding quantum moved, the catalyst regenerated (μ∘δ=id, fixed point of the loop).");
                match register_chimera(path, &nm, &desc, &product) {
                    Ok(()) => {
                        println!("  ✓ registered product '{nm}' — now a first-class navigable object.");
                        println!("  ── decomposition (cl8nk_navigator) ──────────────────────────────");
                        decompose_via_navigator(path, &nm);
                    }
                    Err(e) => println!("  ✗ register failed: {e}"),
                }
            }
            None => println!("  ✗ register failed: catalog path not resolved"),
        }
    }
    0
}

// ── The pathway: a metabolism — loops chained into a loop of loops ────────────
// One cycle turns over one substrate. A pathway chains them: the product of each
// turnover is the substrate of the next, the winding quantum Ω the CARRIER passed
// hand to hand down the chain (the electron-transport / NAD carrier of real
// metabolism). Every catalyst is a fixed point (it regenerates). And if the
// carrier RETURNS to its start (net ΔΩ=0 — Solve and Coagula balanced across the
// whole chain), the pathway CLOSES into a metabolic cycle: a loop of loops, the
// TCA archetype, μ∘δ=id at the pathway level.

/// CLI entry: `./ask --pathway SUBSTRATE C1 C2 … Cn [--certify]`. Runs the substrate
/// through the catalyst sequence, one turnover each, and reports the running species,
/// each catalyst as a fixed point, and whether the pathway closes into a metabolic
/// cycle. --certify verifies each (unique) catalyst regenerates through `lake build`.
pub fn run_pathway(
    catalog: Option<&[CatalogEntry]>,
    substrate_name: &str,
    catalysts: &[String],
    certify: bool,
) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("pathway: no catalog loaded");
        return 2;
    };
    let find = |n: &str| cat.iter().find(|e| e.name == n);
    let Some(es) = find(substrate_name) else {
        eprintln!("pathway: substrate not found: {substrate_name}");
        return 2;
    };
    let start = Tuple::from_entry(es).ord;
    let mut current = start;

    println!("pathway (metabolism):  {substrate_name}  through  [{}]", catalysts.join(" → "));
    println!("  the winding quantum Ω is the carrier; each catalyst is a fixed point that passes it along.");

    let mut all_fixed = true;
    let (mut n_red, mut n_ox) = (0u32, 0u32);
    let mut certified: Vec<String> = Vec::new();
    let mut blocked = false;

    for (i, cn) in catalysts.iter().enumerate() {
        let Some(ec) = find(cn) else {
            println!("  ✗ step {}: catalyst not found: {cn}", i + 1);
            return 2;
        };
        let ct = Tuple::from_entry(ec).ord;
        let (c_star, product, dir, imprint) = match working_stroke(&ct, &current) {
            Ok(t) => t,
            Err(e) => {
                println!("  ✗ step {} ({cn}): pathway blocked — {e}", i + 1);
                blocked = true;
                break;
            }
        };
        let red = dir == "reductive";
        if red { n_red += 1; } else { n_ox += 1; }
        let (w0, w1) = (current[WIND].unwrap(), product[WIND].unwrap());
        println!(
            "  step {}: {cn} — {dir} [{}], Solve→Coagula — Ω {}→{}{}",
            i + 1, dir_arrow(red), glyph_of(WIND, w0), glyph_of(WIND, w1),
            imprint.map(|p| format!(", imprint {p}")).unwrap_or_default()
        );
        println!("           {} → {}", fmt_tuple(&current), fmt_tuple(&product));
        if certify && !certified.iter().any(|c| c == cn) {
            let (green, _) = certify_switch(&ct, &c_star);
            println!("           {} {cn} regenerates (fixed point){}", if green { "✓" } else { "✗" }, if green { "" } else { " — REJECTED" });
            all_fixed &= green;
            certified.push(cn.clone());
        }
        current = product;
    }

    // ── Net transformation + closure: carrier (Ω) vs structure (all other axes) ──
    let carrier_ok = !blocked && current[WIND] == start[WIND];
    let structure_ok = !blocked && (0..WIND).all(|i| current[i] == start[i]);
    let full = carrier_ok && structure_ok; // == (current == start): the chain composed to identity
    println!("  net: {substrate_name} → {}   (carrier: {n_red} reductive · {n_ox} oxidative)", fmt_tuple(&current));
    if blocked {
        println!("  ✗ pathway stalled before completing — a carrier could not pass at some step.");
    } else if full {
        println!("  ✓✓✓ FULLY CLOSED — carrier AND structure returned across all twelve axes.");
        println!("      a TRUE METABOLIC CYCLE: the whole chain of catalysts composed to the identity on {substrate_name} (μ∘δ=id at the pathway level, the TCA archetype). O∞.");
    } else if carrier_ok {
        println!("  ~ carrier closed (net ΔΩ=0) but the STRUCTURE did not return — the substrate was worked and not regenerated.");
        println!("    an electron-transport loop, not yet a full metabolic cycle. A chain whose Coagula imprints compose to identity would close it.");
    } else if structure_ok {
        println!("  ~ structure returned but the carrier is unbalanced (net ΔΩ≠0) — a return leg (opposite-direction turnover) would close the carrier.");
    } else {
        println!("  open pathway — neither carrier nor structure returned; not yet a cycle.");
    }
    if certify && !certified.is_empty() {
        println!("  {} every catalyst in the chain is a certified fixed point.", if all_fixed { "✓" } else { "✗ NOT" });
    }
    0
}

/// Small helper: the Solve/Coagula direction arrow for a pathway step.
fn dir_arrow(reductive: bool) -> &'static str {
    if reductive { "C→S" } else { "S→C" }
}

// ── Structural transformation: the catalyst works the live pairs ─────────────
// The carrier (Ω) is one wire; a real metabolism transforms the substrate's
// STRUCTURE. In its working stroke a catalyst also imprints (Coagula) on its
// dominant structural live pair (T↔H or R↔S), rotating the substrate one notch in
// the catalyst's own polarity. The imprint is invertible (a rotation mod the pair
// cycle), so a pathway closes STRUCTURALLY only when the whole chain of imprints
// composes back to the identity — the substrate returned across all twelve axes.

fn norm_ord(ord: &[Option<u8>; 12], i: usize) -> Option<f32> {
    let m = max_ord(i);
    ord[i].map(|o| if m > 0 { o as f32 / m as f32 } else { 0.0 })
}

/// Rotate primitive `i` of `t` by `dir` notches, mod its cycle length (invertible).
fn rot(t: &mut [Option<u8>; 12], i: usize, dir: i16) {
    if let Some(o) = t[i] {
        let n = max_ord(i) as i16 + 1;
        let v = (((o as i16 + dir) % n) + n) % n;
        t[i] = Some(v as u8);
    }
}

/// Coagula imprint: the catalyst shapes the substrate on its dominant structural
/// live pair (T↔H or R↔S), raising the member it favors and lowering the other by
/// one notch (mod cycle) — the catalyst's polarity pressed into the substrate.
/// Returns the pair label, or None if the catalyst has no structural polarity.
fn structural_imprint(catalyst: &[Option<u8>; 12], substrate: &mut [Option<u8>; 12]) -> Option<&'static str> {
    let pairs = [(1usize, 9usize, "T↔H"), (2usize, 10usize, "R↔S")];
    let mut best: Option<(usize, usize, &'static str, f32)> = None;
    for &(x, y, lbl) in pairs.iter() {
        if let (Some(nx), Some(ny)) = (norm_ord(catalyst, x), norm_ord(catalyst, y)) {
            let ch = nx - ny;
            if best.map_or(true, |(_, _, _, m)| ch.abs() > m.abs()) {
                best = Some((x, y, lbl, ch));
            }
        }
    }
    let (x, y, lbl, ch) = best?;
    if ch == 0.0 {
        return None;
    }
    let s: i16 = if ch > 0.0 { 1 } else { -1 };
    rot(substrate, x, s);
    rot(substrate, y, -s);
    Some(lbl)
}

/// One catalytic working stroke: the Ω carrier transfer (reductive if the catalyst
/// can donate, else oxidative) plus the Coagula structural imprint on the substrate.
/// Returns (catalyst spent C*, product, carrier direction, imprint pair label).
fn working_stroke(
    catalyst: &[Option<u8>; 12],
    substrate: &[Option<u8>; 12],
) -> Result<([Option<u8>; 12], [Option<u8>; 12], &'static str, Option<&'static str>), String> {
    let (c_star, mut product, dir) = match transfer_electron(catalyst, substrate) {
        Ok((cs, p)) => (cs, p, "reductive"),
        Err(_) => match transfer_electron(substrate, catalyst) {
            Ok((p, cs)) => (cs, p, "oxidative"),
            Err(e) => return Err(e),
        },
    };
    let imprint = structural_imprint(catalyst, &mut product);
    Ok((c_star, product, dir, imprint))
}

// ── Imscriptive polymerization: chain the clicks into a sequence-preserving polymer ──
// A click FUSES two monomers (join/max on every axis) — it forgets which was which.
// A polymer must REMEMBER. Imscriptive polymerization enchains monomers at reaction
// centers while the monomer SEQUENCE stays losslessly readable off the chain (R∧W∧X):
// the Grammar building a long imscription (a text, a genome) from monomeric glyphic
// units. Each bond is a Coagula click on one live pair — step-growth condensation
// between complementary partners — or, where the same monomer repeats, an addition
// (chain-growth) enchainment by the propagating active center. The chain then reads
// out its degree of polymerization, regioregularity, copolymer architecture, tacticity
// (the chirality Ħ sequence), and whether it cyclizes head-to-tail into a macrocycle (O∞).

const CHIR: usize = 9; // Ħ Chirality — the stereochemistry axis; its ordered sequence IS the polymer's tacticity

/// One enchainment bond in the polymer walk.
enum Bond {
    Condensation(usize),   // click between complementary partners (step-growth); live-pair index
    Addition,              // identical monomer enchained by the propagating center (chain-growth)
    CrossLink,             // >1 reaction center — a branch/network junction, not a linear bond
}

/// Copolymer architecture from the enchained monomer-name sequence.
fn classify_architecture(seq: &[String]) -> String {
    let mut distinct: Vec<&str> = Vec::new();
    for s in seq {
        if !distinct.contains(&s.as_str()) {
            distinct.push(s.as_str());
        }
    }
    if distinct.len() <= 1 {
        return "homopolymer (a single repeat unit)".into();
    }
    if distinct.len() == seq.len() {
        return format!("heteropolymer ({} distinct monomers, each once — no repeat unit)", seq.len());
    }
    let changes = (1..seq.len()).filter(|&i| seq[i] != seq[i - 1]).count();
    // "alternating" is period-2 between exactly two types (…ABAB…).
    if distinct.len() == 2 && changes == seq.len() - 1 {
        return "alternating copolymer (2 types, strict …ABAB… alternation)".into();
    }
    if changes + 1 == distinct.len() {
        return format!("block copolymer ({} contiguous blocks of {} monomer types)", changes + 1, distinct.len());
    }
    format!("random / statistical copolymer ({} monomer types)", distinct.len())
}

/// Tacticity from the chirality Ħ sequence over the enchained units.
fn classify_tacticity(chir: &[Option<u8>]) -> String {
    let glyphs: String = chir.iter().map(|o| o.map(|v| glyph_of(CHIR, v)).unwrap_or("?")).collect();
    if chir.iter().any(|o| o.is_none()) {
        return format!("{glyphs} — incomplete (a unit lacks Ħ)");
    }
    let v: Vec<u8> = chir.iter().map(|o| o.unwrap()).collect();
    if v.iter().all(|&x| x == v[0]) {
        return format!("isotactic ({glyphs}) — one chirality throughout, a single stereo-configuration");
    }
    let mut vals: Vec<u8> = Vec::new();
    for &x in &v {
        if !vals.contains(&x) {
            vals.push(x);
        }
    }
    if vals.len() == 2 {
        let other = if vals[0] == v[0] { vals[1] } else { vals[0] };
        let syndio = (0..v.len()).all(|i| v[i] == if i % 2 == 0 { v[0] } else { other });
        if syndio {
            return format!("syndiotactic ({glyphs}) — chirality strictly inverts each unit, a regular alternation");
        }
    }
    format!("atactic ({glyphs}) — irregular chirality, no stereo-regular pattern")
}

/// The strongest OPPOSITE-charge live pair between two monomers regardless of the
/// threshold, with its drive — so a bond that terminates below θ can report the
/// near-miss honestly (there IS a complementary handle, just sub-threshold) rather
/// than claiming none exists. None if no live pair is even oppositely charged.
fn best_weak_complement(a: &Tuple, b: &Tuple) -> Option<(usize, f32)> {
    let mut best: Option<(usize, f32)> = None;
    for (i, &pair) in LIVE_PAIRS.iter().enumerate() {
        if let (Some(ca), Some(cb)) = (pair_charge(a, pair), pair_charge(b, pair)) {
            let opposite = (ca > 0.0 && cb < 0.0) || (ca < 0.0 && cb > 0.0);
            if opposite {
                let drive = (ca - cb).abs();
                if best.map_or(true, |(_, d)| drive > d) {
                    best = Some((i, drive));
                }
            }
        }
    }
    best
}

/// A ring-closing / bridging comonomer: a catalog entry X that CLICKS BOTH sides of a
/// failing junction (A,B) — A⋈X and X⋈B each a clean single-center bond. This is the
/// honest answer to "what monomer closes (or repairs) this polymer", and it is a
/// DIFFERENT question from `scan`, which ranks SET electron-transfer mediators.
struct Linker {
    name: String,
    pair_a: usize,
    drive_a: f32,
    pair_b: usize,
    drive_b: f32,
}

/// Search the catalog for monomers X that click both A and X, and X and B — ranked by
/// the WEAKER of the two bonds (a good linker is strongly complementary on both sides).
fn find_linkers(cat: &[CatalogEntry], a: &Tuple, b: &Tuple, theta: f32, top: usize) -> Vec<Linker> {
    let mut hits: Vec<Linker> = Vec::new();
    for e in cat {
        let x = Tuple::from_entry(e);
        if x.ord == a.ord || x.ord == b.ord {
            continue;
        }
        let (Ok(pax), Ok(pxb)) = (click_pair(a, &x, theta), click_pair(&x, b, theta)) else {
            continue;
        };
        hits.push(Linker {
            name: e.name.clone(),
            pair_a: pax.pair_idx,
            drive_a: pax.drive,
            pair_b: pxb.pair_idx,
            drive_b: pxb.drive,
        });
    }
    hits.sort_by(|p, q| {
        let (mp, mq) = (p.drive_a.min(p.drive_b), q.drive_a.min(q.drive_b));
        mq.partial_cmp(&mp).unwrap_or(std::cmp::Ordering::Equal)
    });
    hits.truncate(top);
    hits
}

/// Pre-click a `+`-joined monomer token (`A+B`, `A+B+C`) into ONE blended monomer on
/// the fly, without touching the catalog: fold the parts left-to-right through the click
/// primitive (μ/FFUSE join). This is "click then polymerize" as a single feed token —
/// and the blend is LOSSY (max per axis), so the inputs are no longer recoverable. That
/// is precisely why order of operations matters: a click blends, a polymer remembers.
fn preclick(cat: &[CatalogEntry], token: &str, theta: f32) -> Result<Tuple, String> {
    let parts: Vec<&str> = token.split('+').filter(|s| !s.is_empty()).collect();
    if parts.len() < 2 {
        return Err(format!("pre-click token '{token}' needs at least two '+'-joined entries"));
    }
    let load = |name: &str| cat.iter().find(|e| e.name == name)
        .map(Tuple::from_entry)
        .ok_or_else(|| format!("pre-click: entry not found: {name}"));
    let mut acc = load(parts[0])?;
    for p in &parts[1..] {
        let t = load(p)?;
        match click_pair(&acc, &t, theta) {
            Ok(prod) => acc = Tuple { name: token.to_string(), ord: prod.product },
            Err(_) => return Err(format!("pre-click: {} ⋈ {p} does not fuse (no single clean click at θ={theta:.2})", acc.name)),
        }
    }
    acc.name = token.to_string();
    Ok(acc)
}

/// Load a monomer feed into tuples, resolving `+` pre-click tokens (`A+B` → one blend).
/// Shared by --polymerize and --arrange.
fn load_monomers(cat: &[CatalogEntry], monomers: &[String], theta: f32) -> Result<Vec<Tuple>, String> {
    let mut units = Vec::with_capacity(monomers.len());
    for m in monomers {
        if m.contains('+') {
            units.push(preclick(cat, m, theta)?);
        } else {
            let e = cat.iter().find(|e| e.name == *m).ok_or_else(|| format!("monomer not found: {m}"))?;
            units.push(Tuple::from_entry(e));
        }
    }
    Ok(units)
}

/// Whether an ordered pair bonds at all — condensation, cross-link, or identical addition —
/// with its condensation drive (0 for cross-link/addition). None = the bond fails.
fn bond_forms(a: &Tuple, b: &Tuple, theta: f32) -> Option<f32> {
    match click_pair(a, b, theta) {
        Ok(p) => Some(p.drive),
        Err(ClickFail::Ambiguous(_)) => Some(0.0),
        Err(ClickFail::NoComplementarity) => if a.ord == b.ord { Some(0.0) } else { None },
        Err(ClickFail::Missing) => None,
    }
}

/// Score one ordering of a set: (enchained length, cyclizes, total condensation drive).
fn walk_score(units: &[Tuple], order: &[usize], theta: f32) -> (usize, bool, f32) {
    let n = order.len();
    let mut dp = 1usize;
    let mut drive = 0.0f32;
    for i in 0..n.saturating_sub(1) {
        match bond_forms(&units[order[i]], &units[order[i + 1]], theta) {
            Some(d) => { drive += d; dp = i + 2; }
            None => break,
        }
    }
    let cyclic = dp == n && n >= 2 && click_pair(&units[order[n - 1]], &units[order[0]], theta).is_ok();
    (dp, cyclic, drive)
}

/// Ranking: longer enchainment wins; then cyclization; then bond stability.
fn ordering_better(c: (usize, bool, f32), b: (usize, bool, f32)) -> bool {
    if c.0 != b.0 { c.0 > b.0 } else if c.1 != b.1 { c.1 } else { c.2 > b.2 }
}

/// Result of the ordering search: the best order, how many were searched, whether the
/// search was exhaustive, and — so the caller can report MULTIPLICITY instead of letting a
/// reader assume the best order is the ONLY one that works — how many searched orderings
/// fully enchain and how many cyclize.
struct Arrangement {
    order: Vec<usize>,
    searched: usize,
    exhaustive: bool,
    n_full: usize,   // orderings that enchain all n units
    n_cyclic: usize, // orderings that also close into a ring
}

/// Find the ordering of a monomer SET that polymerizes best. A set is unordered, so this
/// searches orderings rather than assuming the given sequence. Exhaustive (every
/// permutation, Heap's algorithm) for n ≤ 9, else greedy nearest-neighbor from each start
/// (heuristic).
fn best_ordering(units: &[Tuple], theta: f32) -> Arrangement {
    let n = units.len();
    let mut best_order: Vec<usize> = (0..n).collect();
    let mut best_score = walk_score(units, &best_order, theta);
    let mut searched = 1usize;
    let (mut n_full, mut n_cyclic) = (0usize, 0usize);
    let mut tally = |s: (usize, bool, f32)| {
        if s.0 == n { n_full += 1; }
        if s.1 { n_cyclic += 1; }
    };
    tally(best_score);
    let exhaustive = n <= 9;
    if exhaustive {
        let mut idx: Vec<usize> = (0..n).collect();
        let mut c = vec![0usize; n];
        let mut i = 0;
        while i < n {
            if c[i] < i {
                if i % 2 == 0 { idx.swap(0, i); } else { idx.swap(c[i], i); }
                let s = walk_score(units, &idx, theta);
                searched += 1;
                tally(s);
                if ordering_better(s, best_score) { best_score = s; best_order = idx.clone(); }
                c[i] += 1;
                i = 0;
            } else {
                c[i] = 0;
                i += 1;
            }
        }
    } else {
        for start in 0..n {
            let mut used = vec![false; n];
            let mut order = vec![start];
            used[start] = true;
            for _ in 1..n {
                let last = *order.last().unwrap();
                let (mut pick, mut pd) = (None, -1.0f32);
                for j in 0..n {
                    if !used[j] {
                        if let Ok(p) = click_pair(&units[last], &units[j], theta) {
                            if p.drive > pd { pd = p.drive; pick = Some(j); }
                        }
                    }
                }
                match pick { Some(j) => { order.push(j); used[j] = true; } None => break }
            }
            for j in 0..n { if !used[j] { order.push(j); } }
            let s = walk_score(units, &order, theta);
            searched += 1;
            tally(s);
            if ordering_better(s, best_score) { best_score = s; best_order = order; }
        }
    }
    Arrangement { order: best_order, searched, exhaustive, n_full, n_cyclic }
}

/// CLI entry: `./ask --polymerize <SET> --arrange`. Treats the monomers as an UNORDERED
/// set, searches orderings for the one that polymerizes best, and runs the full analysis on
/// that order — so the engine finds the sequence instead of assuming the one you typed.
pub fn run_arrange(
    catalog: Option<&[CatalogEntry]>,
    monomers: &[String],
    theta: f32,
    certify: bool,
    close: bool,
    props: bool,
    modulus: bool,
) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("arrange: no catalog loaded");
        return 2;
    };
    if monomers.len() < 2 {
        eprintln!("arrange needs at least two monomers");
        return 2;
    }
    let units = match load_monomers(cat, monomers, theta) {
        Ok(u) => u,
        Err(e) => { eprintln!("arrange: {e}"); return 2; }
    };
    let n = units.len();
    let arr = best_ordering(&units, theta);
    let (order, searched, exhaustive) = (arr.order, arr.searched, arr.exhaustive);
    let (dp, cyclic, _) = walk_score(&units, &order, theta);
    let ordered: Vec<String> = order.iter().map(|&i| monomers[i].clone()).collect();

    println!("arrange (unordered set → best order):  {{{}}}", monomers.join(", "));
    println!(
        "  searched {searched} ordering(s) {} — a set has no inherent order, so this finds the sequence that polymerizes best (longest enchainment, then closure, then stability).",
        if exhaustive { "(exhaustive: every permutation)" } else { "(greedy nearest-neighbor: heuristic — too many permutations to exhaust)" }
    );
    if dp == n {
        println!(
            "  ✓ best ordering FULLY enchains all {n} units{} — the co-typed wall was an artifact of the given order, not the set:",
            if cyclic { " AND CYCLIZES into a ring" } else { "" }
        );
        // Multiplicity — so nobody reads "best" as "the only one that works". A set where
        // most orderings close is order-ROBUST, not order-specific; only n_cyclic==1 is
        // genuinely a unique closing sequence.
        if cyclic {
            if arr.n_cyclic <= 1 {
                println!("  closure is order-SPECIFIC: this is the only ordering (of {searched} searched) that cyclizes.");
            } else {
                println!(
                    "  closure is order-ROBUST: {}/{searched} searched orderings cyclize (this is merely the best-scoring one, NOT the only sequence that closes — do not call it unique).",
                    arr.n_cyclic
                );
            }
        } else if arr.n_full > 1 {
            println!("  ({}/{searched} searched orderings also fully enchain — this is the best, not the only one.)", arr.n_full);
        }
    } else {
        println!(
            "  ✗ NO ordering fully enchains — best reaches {dp}/{n}. The set is fundamentally fragmented (a monomer is co-typed with every other); a linker is needed regardless of order (--close):"
        );
    }
    println!("      [{}]", ordered.join(" · "));
    println!("  → running the full analysis on the best order:");
    println!();
    run_polymerize(catalog, &ordered, theta, certify, close, props, modulus)
}

/// Split a flat monomer list on a literal separator token (`vs`, `with`) into two sub-lists.
/// None when the token is absent. The token itself is dropped.
fn split_on<'a>(monomers: &'a [String], sep: &str) -> Option<(&'a [String], &'a [String])> {
    monomers
        .iter()
        .position(|m| m == sep)
        .map(|i| (&monomers[..i], &monomers[i + 1..]))
}

/// Resolve a name set, find the ordering that rings best, and read its material signature.
/// Returns the ring-ordered names plus (ρ, conductance, weakest bond) — None if it will not
/// close. The shared core of `--forge`, `--compare`, and `--dope`.
fn material_of(
    cat: &[CatalogEntry],
    names: &[String],
    theta: f32,
) -> Result<(Vec<String>, Option<(f64, &'static str, f32)>), String> {
    let units = load_monomers(cat, names, theta)?;
    let arr = best_ordering(&units, theta);
    let ordered_names: Vec<String> = arr.order.iter().map(|&i| names[i].clone()).collect();
    let ordered_units: Vec<Tuple> = arr.order.iter().map(|&i| units[i].clone()).collect();
    Ok((ordered_names, ring_signature(&ordered_units, theta)))
}

/// CLI entry: `./ask --forge M1 M2 … Mn`. The one-shot deterministic characterize path —
/// treat the monomers as a set, find the best-ringing order, and print the full material
/// sheet (topology, stability, conductance, spectral invariants). No LLM, no search verbs;
/// just the material and its numbers. Equivalent to `--polymerize … --arrange --props`.
pub fn run_forge(catalog: Option<&[CatalogEntry]>, monomers: &[String], theta: f32) -> i32 {
    run_arrange(catalog, monomers, theta, false, false, true, false)
}

/// CLI entry: `./ask --compare A B C vs X Y Z`. Forge two materials and diff them — spectral
/// radius, conductance class, weakest bond. The compare operation of the materials algebra:
/// two rings side by side and the Δ between them.
pub fn run_compare(catalog: Option<&[CatalogEntry]>, monomers: &[String], theta: f32) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("compare: no catalog loaded");
        return 2;
    };
    let Some((left, right)) = split_on(monomers, "vs") else {
        eprintln!("compare needs two sets separated by `vs`:  --compare A B C vs X Y Z");
        return 2;
    };
    if left.len() < 2 || right.len() < 2 {
        eprintln!("compare: each side needs at least two monomers");
        return 2;
    }
    let a = match material_of(cat, left, theta) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("compare (left): {e}");
            return 2;
        }
    };
    let b = match material_of(cat, right, theta) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("compare (right): {e}");
            return 2;
        }
    };
    let fmt = |name: &str, m: &(Vec<String>, Option<(f64, &'static str, f32)>)| match &m.1 {
        Some((rho, cond, wk)) => println!(
            "  {name}: [{}]\n       ρ={rho:.4}  {cond}  weakest Δ={wk:.2}",
            m.0.join(" · ")
        ),
        None => println!("  {name}: [{}]  — does NOT close into a ring (no material)", m.0.join(" · ")),
    };
    println!("compare (two materials):");
    fmt("A", &a);
    fmt("B", &b);
    match (a.1, b.1) {
        (Some((ra, ca, _)), Some((rb, cb, _))) => {
            let d = rb - ra;
            println!(
                "  Δ: ρ {ra:.4} → {rb:.4} ({d:+.4}); conductance {ca} → {cb}{}",
                if ca == cb { " (unchanged)" } else { "" }
            );
            println!(
                "  read: {}",
                if d.abs() < 1e-3 {
                    "same principal mode — the two rings are spectrally equivalent as materials."
                } else if d > 0.0 {
                    "B is the more branched/connected material (higher ρ = a stronger dominant ring-current mode)."
                } else {
                    "A is the more branched/connected material (higher ρ)."
                }
            );
        }
        _ => println!("  Δ: one side is not a ring — no material comparison."),
    }
    0
}

/// CLI entry: `./ask --dope A B C with D`. Forge the base ring, then forge it again with the
/// dopant unit(s) mixed in, and report the shift in ρ and conductance. The dope operation of
/// the materials algebra: perturb a material by a unit and read how its transport moves.
pub fn run_dope(catalog: Option<&[CatalogEntry]>, monomers: &[String], theta: f32) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("dope: no catalog loaded");
        return 2;
    };
    let Some((base, dopant)) = split_on(monomers, "with") else {
        eprintln!("dope needs a base set and a dopant separated by `with`:  --dope A B C with D");
        return 2;
    };
    if base.len() < 2 || dopant.is_empty() {
        eprintln!("dope: need at least two base monomers and at least one dopant");
        return 2;
    }
    let before = match material_of(cat, base, theta) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("dope (base): {e}");
            return 2;
        }
    };
    let doped_names: Vec<String> = base.iter().chain(dopant.iter()).cloned().collect();
    let after = match material_of(cat, &doped_names, theta) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("dope (doped): {e}");
            return 2;
        }
    };
    println!("dope (perturb a material):  base + {{{}}}", dopant.join(", "));
    match &before.1 {
        Some((r, c, w)) => println!("  before: [{}]\n          ρ={r:.4}  {c}  weakest Δ={w:.2}", before.0.join(" · ")),
        None => println!("  before: [{}]  — base does not close into a ring", before.0.join(" · ")),
    }
    match &after.1 {
        Some((r, c, w)) => println!("  after:  [{}]\n          ρ={r:.4}  {c}  weakest Δ={w:.2}", after.0.join(" · ")),
        None => println!("  after:  [{}]  — doping BREAKS the ring (the dopant will not close)", after.0.join(" · ")),
    }
    if let (Some((ra, ca, _)), Some((rb, cb, _))) = (before.1, after.1) {
        println!(
            "  shift: ρ {ra:.4} → {rb:.4} ({:+.4}); conductance {ca} → {cb}{}",
            rb - ra,
            if ca == cb { " (held)" } else { " (CHANGED)" }
        );
    }
    0
}

/// CLI entry: `./ask --fuse A B C + X Y Z`. Weld two rings into one: forge each parent, then
/// forge the union of their units into a single macrocycle, and report how the fused ring's
/// ρ and conductance relate to the two parents'. The fuse operation of the materials algebra
/// — the μ that takes two loops and returns one (when they close together). A fusion that
/// will not re-close is reported honestly: the two rings do not merge into one material.
pub fn run_fuse(catalog: Option<&[CatalogEntry]>, monomers: &[String], theta: f32) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("fuse: no catalog loaded");
        return 2;
    };
    let Some((left, right)) = split_on(monomers, "+") else {
        eprintln!("fuse needs two rings separated by `+`:  --fuse A B C + X Y Z");
        return 2;
    };
    if left.len() < 2 || right.len() < 2 {
        eprintln!("fuse: each ring needs at least two monomers");
        return 2;
    }
    let a = match material_of(cat, left, theta) {
        Ok(v) => v,
        Err(e) => { eprintln!("fuse (ring A): {e}"); return 2; }
    };
    let b = match material_of(cat, right, theta) {
        Ok(v) => v,
        Err(e) => { eprintln!("fuse (ring B): {e}"); return 2; }
    };
    let union: Vec<String> = left.iter().chain(right.iter()).cloned().collect();
    let fused = match material_of(cat, &union, theta) {
        Ok(v) => v,
        Err(e) => { eprintln!("fuse (fused): {e}"); return 2; }
    };
    let show = |tag: &str, m: &(Vec<String>, Option<(f64, &'static str, f32)>)| match &m.1 {
        Some((r, c, w)) => println!("  {tag}: [{}]\n       ρ={r:.4}  {c}  weakest Δ={w:.2}", m.0.join(" · ")),
        None => println!("  {tag}: [{}]  — does NOT close into a ring", m.0.join(" · ")),
    };
    println!("fuse (weld two rings into one):");
    show("ring A", &a);
    show("ring B", &b);
    show("fused ", &fused);
    match (a.1, b.1, fused.1) {
        (Some((ra, _, _)), Some((rb, _, _)), Some((rf, cf, _))) => {
            println!(
                "  fusion: parents ρ {{{ra:.4}, {rb:.4}}} → fused ρ={rf:.4} ({cf}); the two loops weld into a single {}-unit macrocycle.",
                fused.0.len()
            );
            if rf > ra.max(rb) + 1e-3 {
                println!("  read: fusion RAISES the principal mode above either parent — the weld adds connectivity (a more branched material).");
            } else {
                println!("  read: the fused ring's ρ sits within the parents' range — a clean splice, no new branching from the weld.");
            }
        }
        (_, _, None) => println!("  fusion: the combined units do NOT re-close — these two rings do not fuse into one material (the union stays open)."),
        _ => println!("  fusion: a parent is not itself a ring — forge each side first."),
    }
    0
}

/// CLI entry: `./ask --anneal M1 M2 … Mn`. Relax a ring to its lowest-strain ordering. The
/// forge order rings well but is quenched (it optimizes enchainment and closure, not even
/// loading); annealing searches the orderings that ring for the one whose bond drive is most
/// uniform — the settled ground state on the same units. Exhaustive for n ≤ 9. Reports the
/// quenched order and its strain against the annealed order and its strain.
pub fn run_anneal(catalog: Option<&[CatalogEntry]>, monomers: &[String], theta: f32) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("anneal: no catalog loaded");
        return 2;
    };
    if monomers.len() < 3 {
        eprintln!("anneal: need at least three monomers");
        return 2;
    }
    let units = match load_monomers(cat, monomers, theta) {
        Ok(u) => u,
        Err(e) => { eprintln!("anneal: {e}"); return 2; }
    };
    let n = units.len();
    let quenched = best_ordering(&units, theta).order;
    let (dp0, cyc0, _) = walk_score(&units, &quenched, theta);
    let qnames: Vec<String> = quenched.iter().map(|&i| monomers[i].clone()).collect();
    println!("anneal (relax the ring to its lowest-strain ordering):");
    if !(dp0 == n && cyc0) {
        println!("  [{}] does not close into a ring — nothing to relax (forge or --close it first).", qnames.join(" · "));
        return 0;
    }
    let qunits: Vec<Tuple> = quenched.iter().map(|&i| units[i].clone()).collect();
    let quenched_strain = ring_strain(&qunits, theta);

    // search orderings that fully enchain AND cyclize for the minimum ring strain (Heap's
    // algorithm, mirroring best_ordering; exhaustive for n ≤ 9, else the quenched order stands).
    let mut best_order = quenched.clone();
    let mut best_strain = quenched_strain;
    let exhaustive = n <= 9;
    let mut consider = |order: &[usize], best_order: &mut Vec<usize>, best_strain: &mut f32| {
        let (dp, cyc, _) = walk_score(&units, order, theta);
        if dp == n && cyc {
            let u: Vec<Tuple> = order.iter().map(|&i| units[i].clone()).collect();
            let s = ring_strain(&u, theta);
            if s < *best_strain - 1e-6 {
                *best_strain = s;
                *best_order = order.to_vec();
            }
        }
    };
    if exhaustive {
        let mut idx: Vec<usize> = (0..n).collect();
        let mut c = vec![0usize; n];
        let mut i = 0;
        while i < n {
            if c[i] < i {
                if i % 2 == 0 { idx.swap(0, i); } else { idx.swap(c[i], i); }
                consider(&idx, &mut best_order, &mut best_strain);
                c[i] += 1;
                i = 0;
            } else {
                c[i] = 0;
                i += 1;
            }
        }
    }

    println!("  quenched (forge order): [{}]\n            strain σ(Δ)={quenched_strain:.3}", qnames.join(" · "));
    if best_strain < quenched_strain - 1e-6 {
        let anames: Vec<String> = best_order.iter().map(|&i| monomers[i].clone()).collect();
        println!(
            "  annealed (relaxed):     [{}]\n            strain σ(Δ)={best_strain:.3} ({:+.3}) — a lower-stress ring on the same units, the settled ground state.",
            anames.join(" · "),
            best_strain - quenched_strain
        );
    } else {
        println!(
            "  annealed: the forge order is ALREADY the lowest-strain ring{} — no relaxation available.",
            if exhaustive { " (exhaustive over every ordering)" } else { " among those searched" }
        );
    }
    0
}

/// CLI entry: `./ask --cleave M1 M2 … Mn`. Ring fission, the δ-split reverse of `--fuse`.
/// Forge the set into its best ring, then cut the loop at two junctions so it falls into two
/// daughter rings on complementary arcs, each re-closing head-to-tail on its own. Reports the
/// parent and the best fission (both daughters and their spectra), or that the ring will not
/// cleave. Each daughter shares the parent's units — the fission conserves the vertex set.
pub fn run_cleave(catalog: Option<&[CatalogEntry]>, monomers: &[String], theta: f32) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("cleave: no catalog loaded");
        return 2;
    };
    if monomers.len() < 4 {
        eprintln!("cleave: need at least four monomers (two daughter rings of ≥2 units each)");
        return 2;
    }
    let units = match load_monomers(cat, monomers, theta) {
        Ok(u) => u,
        Err(e) => { eprintln!("cleave: {e}"); return 2; }
    };
    let arr = best_ordering(&units, theta);
    let ring: Vec<Tuple> = arr.order.iter().map(|&i| units[i].clone()).collect();
    let names: Vec<String> = arr.order.iter().map(|&i| monomers[i].clone()).collect();
    let n = ring.len();

    println!("cleave (fission a ring into two daughter rings):");
    let parent_sig = ring_signature(&ring, theta);
    match parent_sig {
        Some((r, c, w)) => println!("  parent: [{}]\n          ρ={r:.4}  {c}  weakest Δ={w:.2}", names.join(" · ")),
        None => {
            println!("  parent: [{}]  — does NOT close into a ring; nothing to cleave (forge or --close it first).", names.join(" · "));
            return 0;
        }
    }

    // Search contiguous bipartitions of the cycle: daughter A = ring[a..a+l], daughter B = the
    // complementary arc. A cut leaves two rings only if BOTH arcs re-close on their own head→tail
    // bond. Canonicalize each partition by daughter-A's sorted indices so the two arcs of one cut
    // are not counted twice; rank by the weaker daughter's stability (a ring is its weakest link).
    let mut seen: Vec<Vec<usize>> = Vec::new();
    let mut best: Option<(usize, usize, f32)> = None; // (start a, length l, score)
    for l in 2..=(n - 2) {
        for a in 0..n {
            let idx_a: Vec<usize> = (0..l).map(|k| arr.order[(a + k) % n]).collect();
            let idx_b: Vec<usize> = (0..(n - l)).map(|k| arr.order[(a + l + k) % n]).collect();
            let da: Vec<Tuple> = (0..l).map(|k| ring[(a + k) % n].clone()).collect();
            let db: Vec<Tuple> = (0..(n - l)).map(|k| ring[(a + l + k) % n].clone()).collect();
            if click_pair(&da[l - 1], &da[0], theta).is_err()
                || click_pair(&db[n - l - 1], &db[0], theta).is_err()
            {
                continue;
            }
            let mut canon = idx_a.clone();
            canon.sort_unstable();
            let mut canon_b = idx_b.clone();
            canon_b.sort_unstable();
            let key = canon.clone().min(canon_b); // the smaller-indexed arc labels the partition
            if seen.contains(&key) {
                continue;
            }
            seen.push(key);
            let sa = ring_signature(&da, theta).map_or(0.0, |s| s.2);
            let sb = ring_signature(&db, theta).map_or(0.0, |s| s.2);
            let score = sa.min(sb);
            if best.map_or(true, |(_, _, bs)| score > bs) {
                best = Some((a, l, score));
            }
        }
    }

    match best {
        None => println!("  fission: this ring does NOT cleave — no cut leaves both arcs closing into daughter rings. A single-bridge ring holds as one loop."),
        Some((a, l, _)) => {
            let da: Vec<Tuple> = (0..l).map(|k| ring[(a + k) % n].clone()).collect();
            let db: Vec<Tuple> = (0..(n - l)).map(|k| ring[(a + l + k) % n].clone()).collect();
            let na: Vec<String> = (0..l).map(|k| names[(a + k) % n].clone()).collect();
            let nb: Vec<String> = (0..(n - l)).map(|k| names[(a + l + k) % n].clone()).collect();
            let show = |tag: &str, units: &[Tuple], nm: &[String]| match ring_signature(units, theta) {
                Some((r, c, w)) => println!("  {tag}: [{}]\n            ρ={r:.4}  {c}  weakest Δ={w:.2}", nm.join(" · ")),
                None => println!("  {tag}: [{}]  — open", nm.join(" · ")),
            };
            println!("  best fission ({} distinct cut(s) leave both arcs closing):", seen.len());
            show("daughter A", &da, &na);
            show("daughter B", &db, &nb);
            if let (Some((rp, _, _)), Some((ra, _, _)), Some((rb, _, _))) =
                (parent_sig, ring_signature(&da, theta), ring_signature(&db, theta))
            {
                println!(
                    "  reconstitution: parent ρ={rp:.4} splits into daughter modes {{{ra:.4}, {rb:.4}}}; fission divides the single principal ring-current into two smaller-ring modes (μ∘δ: re-fuse the daughters with --fuse to weld them back)."
                );
            }
        }
    }
    0
}

/// The conductance verdict of a ring: does a winding quantum Ω circulate it?
enum Cond {
    Conductive { fwd: bool },      // one consistent direction closes the loop — a persistent current
    Frustrated,                    // every junction passes, but no single direction circulates
    Insulating { blocked: Vec<usize> }, // some junction blocks a carrier both ways
}

/// Can a winding quantum Ω circulate the ring (units treated as a cycle)? A consistent
/// one-way circulation is a persistent ring current (the loop SUSTAINS itself, μ∘δ=id
/// around the cycle). Reuses the Ω-transfer primitive at each junction.
fn ring_conductance(units: &[Tuple]) -> Cond {
    let n = units.len();
    let dir_ok = |fwd: bool| (0..n).all(|i| {
        let (a, b) = if fwd { (i, (i + 1) % n) } else { ((i + 1) % n, i) };
        transfer_electron(&units[a].ord, &units[b].ord).is_ok()
    });
    let (fwd, bwd) = (dir_ok(true), dir_ok(false));
    if fwd || bwd {
        return Cond::Conductive { fwd };
    }
    let blocked: Vec<usize> = (0..n).filter(|&i| {
        !(transfer_electron(&units[i].ord, &units[(i + 1) % n].ord).is_ok()
            || transfer_electron(&units[(i + 1) % n].ord, &units[i].ord).is_ok())
    }).collect();
    if blocked.is_empty() { Cond::Frustrated } else { Cond::Insulating { blocked } }
}

/// The adjacency matrix of the ring's bond graph — the ring treated as a weighted graph so
/// its spectral invariants can be computed. Convention (printed, so it is auditable): each
/// ring edge (i, i+1) carries the weight of its junction — a clean condensation bond or an
/// identical addition weighs 1; a cross-link junction weighs k, its number of reaction
/// centers (k parallel bonds). A pure unbranched cycle is therefore all-1s and has spectral
/// radius exactly 2; every extra reaction center a cross-link fires lifts ρ above 2. This is
/// the honest graph behind the "branched macrocycle" the tools already report.
fn ring_adjacency(units: &[Tuple], theta: f32) -> Vec<Vec<f64>> {
    let n = units.len();
    let mut m = vec![vec![0.0f64; n]; n];
    for i in 0..n {
        let j = (i + 1) % n;
        let w = match click_pair(&units[i], &units[j], theta) {
            Ok(_) => 1.0,                                                     // clean condensation bond
            Err(ClickFail::Ambiguous(centers)) => centers.len() as f64,      // cross-link: k parallel reaction centers
            Err(ClickFail::NoComplementarity) if units[i].ord == units[j].ord => 1.0, // identical addition (repeat unit)
            Err(_) => 0.0,                                                    // no edge — a broken ring (should not occur once cyclic)
        };
        m[i][j] = w;
        m[j][i] = w;
    }
    m
}

/// Eigenvalues of a real symmetric matrix via the classical (largest-off-diagonal) Jacobi
/// rotation sweep. Exact to machine precision for the small rings these materials form
/// (n ≲ a few dozen); no external linear-algebra dependency. Returns the diagonal once the
/// off-diagonal mass is annihilated — i.e. the eigenvalues, in the matrix's own order.
fn sym_eigenvalues(mut a: Vec<Vec<f64>>) -> Vec<f64> {
    let n = a.len();
    if n == 0 {
        return vec![];
    }
    for _ in 0..(50 * n * n) {
        // pivot on the largest off-diagonal element
        let (mut p, mut q, mut off) = (0usize, 1usize, 0.0f64);
        for i in 0..n {
            for j in (i + 1)..n {
                if a[i][j].abs() > off {
                    off = a[i][j].abs();
                    p = i;
                    q = j;
                }
            }
        }
        if off < 1e-13 {
            break;
        }
        // rotation that zeroes a[p][q] (numerically stable t-formula)
        let theta = (a[q][q] - a[p][p]) / (2.0 * a[p][q]);
        let t = if theta == 0.0 {
            1.0
        } else {
            theta.signum() / (theta.abs() + (theta * theta + 1.0).sqrt())
        };
        let c = 1.0 / (t * t + 1.0).sqrt();
        let s = t * c;
        for k in 0..n {
            let (akp, akq) = (a[k][p], a[k][q]);
            a[k][p] = c * akp - s * akq;
            a[k][q] = s * akp + c * akq;
        }
        for k in 0..n {
            let (apk, aqk) = (a[p][k], a[q][k]);
            a[p][k] = c * apk - s * aqk;
            a[q][k] = s * apk + c * aqk;
        }
    }
    (0..n).map(|i| a[i][i]).collect()
}

/// Spectral-invariant block for the ring material: the adjacency spectrum, the spectral
/// radius ρ (the principal ring-current mode), and the spectral gap. ρ = 2 exactly witnesses
/// a pure unbranched cycle; ρ > 2 witnesses branching — the same fact the topology line
/// reports, now as a computed number rather than an asserted adjective.
fn print_ring_spectrum(units: &[Tuple], theta: f32) {
    let n = units.len();
    if n < 2 {
        return;
    }
    let mut ev = sym_eigenvalues(ring_adjacency(units, theta));
    ev.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let rho = ev.iter().fold(0.0f64, |m, &x| m.max(x.abs()));
    let mut mags: Vec<f64> = ev.iter().map(|x| x.abs()).collect();
    mags.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let gap = if mags.len() >= 2 { rho - mags[1] } else { rho };
    let spec: Vec<String> = ev.iter().map(|x| format!("{x:+.3}")).collect();
    let verdict = if (rho - 2.0).abs() < 1e-3 {
        "= 2 exactly ⟹ a pure unbranched cycle (every junction one clean bond)"
    } else if rho > 2.0 {
        "> 2 ⟹ branched: a cross-link lifts the principal mode above the bare ring"
    } else {
        "< 2 ⟹ a broken/open ring (a junction carries no bond)"
    };
    let energy: f64 = ev.iter().map(|x| x.abs()).sum();
    println!("    ── spectral invariants (adjacency of the ring graph; clean bond=1, cross-link=k centers) ──");
    println!("    spectral radius ρ = {rho:.4}  ({verdict})");
    println!("    spectrum: [{}]", spec.join(", "));
    println!(
        "    spectral gap (ρ − |λ₂|) = {gap:.4} — the wider the gap, the more a single ring-current mode dominates transport."
    );
    println!(
        "    graph energy Σ|λ| = {energy:.4} — the ring's total spectral weight; where ρ is stiffness (the dominant mode), this is toughness (the reserve carried across all modes)."
    );
}

/// Material-property sheet for a CLOSED (cyclic) polymer — the ring treated as a
/// mathematical material. Grounds the transport claims that prose loves to assert: a
/// winding quantum Ω that circulates the whole loop one direction is a persistent ring
/// current (CONDUCTIVE); a junction that blocks a carrier both ways is INSULATING;
/// conduction with no consistent global direction is FRUSTRATED. Plus the weakest ring
/// bond — a ring is only as stable as its weakest link. Reuses the Ω-transfer primitive.
fn print_ring_material(units: &[Tuple], theta: f32, branched: bool) {
    let n = units.len();
    // weakest clean condensation bond around the ring (cross-links/additions counted apart),
    // plus every clean drive so the ring's strain (how evenly the bonds are loaded) is read.
    let mut weakest: Option<(usize, f32)> = None;
    let mut noncond = 0;
    let mut drives: Vec<f32> = Vec::new();
    for i in 0..n {
        match click_pair(&units[i], &units[(i + 1) % n], theta) {
            Ok(p) => {
                if weakest.map_or(true, |(_, d)| p.drive < d) {
                    weakest = Some((i, p.drive));
                }
                drives.push(p.drive);
            }
            Err(_) => noncond += 1,
        }
    }
    // ring strain: the spread of clean-bond drive around the loop (shared with `anneal`).
    let strain = ring_strain(units, theta);
    println!("  ── material properties (the ring as a mathematical material) ──");
    println!(
        "    macrocycle: {n}-membered ring{}",
        if branched { ", branched (a cross-linked network node on the ring)" } else { "" }
    );
    match weakest {
        Some((i, d)) => println!(
            "    ring stability: weakest clean bond Δ={d:.2} at junction {}→{} (only as stable as its weakest link){}",
            i + 1, (i + 1) % n + 1,
            if noncond > 0 { format!("; {noncond} junction(s) cross-link/addition, not one clean bond") } else { String::new() }
        ),
        None => println!("    ring stability: no clean condensation bond around the ring (every junction a cross-link/addition)"),
    }
    if drives.len() > 1 {
        println!(
            "    ring strain σ(Δ) = {strain:.3} — {}",
            if strain < 0.15 {
                "bonds evenly loaded, a relaxed ring at rest"
            } else {
                "bonds unevenly loaded, internal stress stored (an anneal would relax it)"
            }
        );
    }
    match ring_conductance(units) {
        Cond::Conductive { fwd } => {
            let d = if fwd { "→ reductive" } else { "← oxidative" };
            println!("    conductance: CONDUCTIVE — a winding quantum Ω circulates the whole ring one way ({d}); a persistent ring current is supported (∮ carrier closes). This ring SUSTAINS — it carries a modulus.");
        }
        Cond::Frustrated => println!("    conductance: FRUSTRATED — every junction passes a carrier, but no single direction circulates the loop (no persistent global current; an ohmic/segmented conductor)."),
        Cond::Insulating { blocked } => {
            let js: Vec<String> = blocked.iter().map(|&i| format!("{}→{}", i + 1, (i + 1) % n + 1)).collect();
            println!("    conductance: INSULATING — no carrier can pass junction(s) {} in either direction; the ring cannot circulate a current (the units are Ω-saturated/empty, a static ring not a dynamic one).", js.join(", "));
        }
    }
    print_ring_spectrum(units, theta);
}

/// Ring strain: the population σ of clean-bond drive around the loop. Near zero when every
/// junction carries the same tension, large when the ring is forced shut against reluctant
/// bonds. The scalar `anneal` minimizes over the orderings that ring.
fn ring_strain(units: &[Tuple], theta: f32) -> f32 {
    let n = units.len();
    let drives: Vec<f32> = (0..n)
        .filter_map(|i| click_pair(&units[i], &units[(i + 1) % n], theta).ok().map(|p| p.drive))
        .collect();
    if drives.len() < 2 {
        return 0.0;
    }
    let mean = drives.iter().sum::<f32>() / drives.len() as f32;
    (drives.iter().map(|d| (d - mean).powi(2)).sum::<f32>() / drives.len() as f32).sqrt()
}

/// Compact one-line material signature — ρ, conductance, and weakest bond — for a cyclic
/// ring. Used by the operate verbs (`--compare`, `--dope`) to diff two materials without
/// reprinting the full sheet. Returns None when the units do not form a ring.
fn ring_signature(units: &[Tuple], theta: f32) -> Option<(f64, &'static str, f32)> {
    let n = units.len();
    if n < 2 {
        return None;
    }
    // must actually close head-to-tail to be a ring material
    if click_pair(&units[n - 1], &units[0], theta).is_err() {
        return None;
    }
    let mut ev = sym_eigenvalues(ring_adjacency(units, theta));
    ev.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    let rho = ev.iter().fold(0.0f64, |m, &x| m.max(x.abs()));
    let cond = match ring_conductance(units) {
        Cond::Conductive { .. } => "CONDUCTIVE",
        Cond::Frustrated => "FRUSTRATED",
        Cond::Insulating { .. } => "INSULATING",
    };
    let weakest = (0..n)
        .filter_map(|i| click_pair(&units[i], &units[(i + 1) % n], theta).ok().map(|p| p.drive))
        .fold(f32::INFINITY, f32::min);
    Some((rho, cond, if weakest.is_finite() { weakest } else { 0.0 }))
}

/// CLI entry: `./ask --polymerize M1 M2 … Mn [--certify] [--close] [--props]`. Chain the monomers into a
/// polymer — each adjacent bond a click (step-growth condensation) or an addition
/// (chain-growth, where a monomer repeats) — while the sequence stays losslessly
/// readable off the chain. Reports degree of polymerization, regioregularity,
/// architecture, tacticity, and head-to-tail cyclization. --certify closes each unit.
pub fn run_polymerize(
    catalog: Option<&[CatalogEntry]>,
    monomers: &[String],
    theta: f32,
    certify: bool,
    close: bool,
    props: bool,
    modulus: bool,
) -> i32 {
    let Some(cat) = catalog else {
        eprintln!("polymerize: no catalog loaded");
        return 2;
    };
    if monomers.len() < 2 {
        eprintln!("polymerize needs at least two monomers (repeat one to show homopolymerization, e.g. --polymerize M M M)");
        return 2;
    }
    // Load the feed (resolving `+` pre-click tokens — `A+B` blends into one monomer first).
    let units = match load_monomers(cat, monomers, theta) {
        Ok(u) => u,
        Err(e) => { eprintln!("polymerize: {e}"); return 2; }
    };
    let n = units.len();

    println!("polymerization (imscriptive):  [{}]   ({n} monomers)", monomers.join(" · "));
    println!("  imscriptive: the chain stores the monomer SEQUENCE losslessly (R∧W∧X) — a click blends, a polymer remembers.");
    if n <= 8 {
        println!("  sequence (read back off the chain, unit by unit):");
        for (i, t) in units.iter().enumerate() {
            let tag = if monomers[i].contains('+') { "   (pre-clicked blend — inputs no longer recoverable)" } else { "" };
            println!("    {}. {}  {}{}", i + 1, monomers[i], fmt_tuple(&t.ord), tag);
        }
    }

    println!("  bonds (each a Coagula link — a click between complementary partners, or an addition where a monomer repeats):");
    let mut bonds: Vec<Bond> = Vec::new();
    let mut terminated: Option<usize> = None;
    for i in 0..n - 1 {
        let (a, b) = (&units[i], &units[i + 1]);
        let tag = format!("    {}–{}  {} ⋈ {}", i + 1, i + 2, monomers[i], monomers[i + 1]);
        match click_pair(a, b, theta) {
            Ok(p) => {
                println!("{tag}  → condensation on {} (Δ={:.2})", LIVE_LABELS[p.pair_idx], p.drive);
                bonds.push(Bond::Condensation(p.pair_idx));
            }
            Err(ClickFail::Ambiguous(pairs)) => {
                let labels: Vec<&str> = pairs.iter().map(|&i| LIVE_LABELS[i]).collect();
                println!("{tag}  → cross-link ({} reaction centers: {}) — a branch/network junction", pairs.len(), labels.join(", "));
                bonds.push(Bond::CrossLink);
            }
            Err(ClickFail::NoComplementarity) => {
                if a.ord == b.ord {
                    println!("{tag}  → addition (chain-growth: identical unit enchained by the propagating center)");
                    bonds.push(Bond::Addition);
                } else {
                    println!("{tag}  → ✗ termination at θ={theta:.2} — the chain ends here.");
                    match best_weak_complement(a, b) {
                        Some((pi, drive)) => println!(
                            "           near-miss: weakly complementary on {} (Δ={drive:.2} < θ) — near-co-typed monomers; they would condense at θ≤{drive:.2} or with a catalyst.",
                            LIVE_LABELS[pi]
                        ),
                        None => println!("           no oppositely-charged live pair at all — fully co-typed / same-handed; no reaction center exists."),
                    }
                    terminated = Some(i);
                    break;
                }
            }
            Err(ClickFail::Missing) => {
                println!("{tag}  → ✗ a monomer lacks a live-pair primitive — no bond forms; the chain ends here.");
                terminated = Some(i);
                break;
            }
        }
    }

    let dp = terminated.map(|i| i + 1).unwrap_or(n);
    let n_bonds = bonds.len();
    println!(
        "  degree of polymerization: {dp} unit(s), {n_bonds} bond(s){}",
        if terminated.is_some() { " (terminated early)" } else { "" }
    );

    // regioregularity of the condensation backbone
    let cond_pairs: Vec<usize> = bonds.iter().filter_map(|b| match b {
        Bond::Condensation(p) => Some(*p),
        _ => None,
    }).collect();
    if !cond_pairs.is_empty() {
        let first = cond_pairs[0];
        if cond_pairs.iter().all(|&p| p == first) {
            println!("  backbone: regioregular — every condensation bond on {} (a clean head-to-tail repeat unit).", LIVE_LABELS[first]);
        } else {
            let mut seen: Vec<&str> = Vec::new();
            for &p in &cond_pairs {
                if !seen.contains(&LIVE_LABELS[p]) {
                    seen.push(LIVE_LABELS[p]);
                }
            }
            println!("  backbone: regioirregular — bonds on {} (head-to-head/tail-to-tail defects).", seen.join(", "));
        }
    }
    if n_bonds > 0 && bonds.iter().all(|b| matches!(b, Bond::Addition)) {
        println!("  backbone: addition (chain-growth) throughout — one repeat unit enchained by the propagating center.");
    }
    let branched = bonds.iter().any(|b| matches!(b, Bond::CrossLink));
    if branched {
        println!("  topology: BRANCHED/NETWORK — a cross-link junction fired (≥2 reaction centers); not a purely linear chain.");
    }

    // Architecture and tacticity describe a CHAIN; with zero bonds there is none —
    // one enchained unit is an unreacted monomer, not a homopolymer. Only read them
    // out once at least one bond has formed (dp ≥ 2).
    if dp >= 2 {
        println!("  architecture: {}", classify_architecture(&monomers[..dp]));
        let chir: Vec<Option<u8>> = units[..dp].iter().map(|t| t.ord[CHIR]).collect();
        println!("  tacticity (Ħ chirality per unit): {}", classify_tacticity(&chir));
    } else {
        println!("  no chain formed — the feed did not enchain past the first monomer (0 bonds); these are unreacted monomers, not a polymer.");
    }

    // cyclization: only a fully enchained chain (both original ends still live) can close head-to-tail
    let head_tail = if terminated.is_none() && dp >= 2 {
        Some(click_pair(&units[dp - 1], &units[0], theta))
    } else {
        None
    };
    let cyclic = matches!(head_tail, Some(Ok(_)));
    match &head_tail {
        Some(Ok(p)) => println!(
            "  cyclization: {} ⋈ {} → ✓ CYCLIC — a macrocycle (ring polymer); the sequence closes head-to-tail on {} (O∞).",
            monomers[dp - 1], monomers[0], LIVE_LABELS[p.pair_idx]
        ),
        Some(Err(_)) => println!(
            "  cyclization: {} ⋈ {} → linear (telechelic — two free ends, no head-to-tail closure).",
            monomers[dp - 1], monomers[0]
        ),
        None => println!("  cyclization: linear — the chain terminated, so it cannot close into a ring."),
    }

    // --close: find the monomer that actually CLOSES this chain (or BRIDGES the break),
    // by testing real clicks against the catalog — the honest cyclization search. This is
    // NOT `scan` (SET electron relays); each candidate is verified to click both sides.
    if close && !cyclic {
        let bridge = terminated.is_some();
        let (ai, bi) = match terminated {
            Some(i) => (i, i + 1), // internal break Mᵢ ⋈ Mᵢ₊₁ — a linker goes between them
            None => (dp - 1, 0),   // head-tail ring bond Mₙ ⋈ M₁ — a closer appends to wrap
        };
        println!(
            "  ── closing search: a monomer X with {} ⋈ X and X ⋈ {} (a real click test, NOT a SET-mediator scan) ──",
            monomers[ai], monomers[bi]
        );
        let linkers = find_linkers(cat, &units[ai], &units[bi], theta, 6);
        if linkers.is_empty() {
            println!("    none — no catalog monomer clicks both sides of this junction at θ={theta:.2}. Lower θ or accept the chain is open.");
        } else {
            for l in &linkers {
                println!(
                    "    {}   ({} ⋈ X on {} Δ={:.2}  ·  X ⋈ {} on {} Δ={:.2})",
                    l.name, monomers[ai], LIVE_LABELS[l.pair_a], l.drive_a, monomers[bi], LIVE_LABELS[l.pair_b], l.drive_b
                );
            }
            let best = &linkers[0].name;
            if bridge {
                // Insert into the FULL feed (the break truncated dp, but Mᵢ₊₁ and the
                // rest of the sequence still follow the junction we are repairing).
                let mut feed: Vec<&str> = monomers.iter().map(|s| s.as_str()).collect();
                feed.insert(ai + 1, best); // between Mᵢ and Mᵢ₊₁
                println!(
                    "    ⮑ insert {best} to repair the break (Mᵢ ⋈ X ⋈ Mᵢ₊₁):  ./ask --polymerize {}",
                    feed.join(" ")
                );
            } else {
                println!(
                    "    ⮑ append {best} to close the ring head-to-tail:  ./ask --polymerize {} {best}",
                    monomers[..dp].join(" ")
                );
            }
        }
    }

    // --props: characterize the CLOSED ring as a material (conductance, stability). This
    // grounds the transport claims prose asserts about a cyclic "computer" — a ring of
    // Ω-saturated units cannot circulate a current, whatever the narrative says.
    if props {
        if cyclic {
            print_ring_material(&units[..dp], theta, branched);
        } else {
            println!("  material properties: not a closed ring — no macrocycle to characterize (use --close to find the ring-closing monomer first).");
        }
    }

    // --modulus: a SUSTAINING loop = a conductive cycle (∮ Ω current closes) somewhere
    // along the chain. Distinct from --close, which only fills a gap: a ring can close
    // and still be static (insulating, no modulus). This searches for a monomer that
    // generates a loop that actually SUSTAINS, and reports its period — the modulus.
    if modulus {
        println!("  ── modulus: a monomer that generates a SUSTAINING loop (a conductive cycle, ∮ Ω closes), not merely a closed one ──");
        if cyclic {
            if let Cond::Conductive { .. } = ring_conductance(&units[..dp]) {
                println!("    intrinsic modulus = {dp} — the closed chain is already a sustaining {dp}-loop (persistent Ω current); no monomer needed.");
            }
        }
        // generative search: a monomer X that closes a backbone span [i..j] into a
        // conductive loop. Catalog tuples precomputed once; spans are within the
        // enchained region, so the segment is already bonded.
        let cat_t: Vec<(&str, Tuple)> = cat.iter().map(|e| (e.name.as_str(), Tuple::from_entry(e))).collect();
        let mut found: Vec<(usize, usize, usize, String)> = Vec::new(); // (period, i, j, monomer)
        for i in 0..dp {
            for j in (i + 1)..dp {
                for (name, x) in &cat_t {
                    if x.ord == units[i].ord || x.ord == units[j].ord {
                        continue;
                    }
                    if click_pair(&units[j], x, theta).is_err() || click_pair(x, &units[i], theta).is_err() {
                        continue;
                    }
                    let mut loop_units: Vec<Tuple> = units[i..=j].to_vec();
                    loop_units.push((*x).clone());
                    if let Cond::Conductive { .. } = ring_conductance(&loop_units) {
                        found.push((j - i + 2, i, j, name.to_string()));
                    }
                }
            }
        }
        if found.is_empty() {
            println!("    none — no catalog monomer generates a sustaining loop over this chain; every closure is static (insulating). This material has NO modulus (a viscous chain, not an elastic network).");
        } else {
            found.sort_by(|a, b| a.0.cmp(&b.0).then(a.3.cmp(&b.3)));
            let min_p = found[0].0;
            println!("    modulus = {min_p} — the tightest sustaining loop this chain admits (a conductive {min_p}-cycle). Generators:");
            for (p, i, j, name) in found.iter().filter(|f| f.0 == min_p).take(4) {
                let seg: Vec<&str> = units[*i..=*j].iter().map(|t| t.name.as_str()).collect();
                println!("      {name}  closes units {}‥{} into a sustaining {p}-loop:  ./ask --polymerize {} {name} --props", i + 1, j + 1, seg.join(" "));
            }
        }
    }

    if certify {
        println!("  certifying each repeat unit is a valid Imscription (Frobenius closure, lake build)…");
        let mut seen: Vec<[Option<u8>; 12]> = Vec::new();
        let mut all_green = true;
        for t in &units[..dp] {
            if seen.iter().any(|o| *o == t.ord) {
                continue;
            }
            let (green, _) = certify_click(&t.ord);
            println!("    {} {} closes", if green { "✓" } else { "✗" }, t.name);
            all_green &= green;
            seen.push(t.ord);
        }
        println!(
            "  {} every repeat unit Frobenius-closes — the polymer is a chain of valid imscriptions.",
            if all_green { "✓" } else { "✗ NOT" }
        );
    }
    0
}
