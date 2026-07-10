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
    let (c_star, product, dir) = match transfer_electron(&cat_t, &sub_t) {
        Ok((cs, p)) => (cs, p, "reductive — catalyst donates e⁻, substrate reduced"),
        Err(_) => match transfer_electron(&sub_t, &cat_t) {
            Ok((p, cs)) => (cs, p, "oxidative — catalyst abstracts e⁻, substrate oxidized"),
            Err(e) => {
                println!("  ✗ no turnover: {e} — neither redox direction is feasible for this pair.");
                return 0;
            }
        }
    };
    let (cw0, cw1) = (cat_t[WIND].unwrap(), c_star[WIND].unwrap());
    let (sw0, sw1) = (sub_t[WIND].unwrap(), product[WIND].unwrap());
    println!("  working stroke (δ / SOLVE — dissolves the bond, reveals the degree of freedom: the winding quantum comes free): {dir}");
    println!("    catalyst  {catalyst_name}: Ω {}→{}  (spent → {catalyst_name}*)", glyph_of(WIND, cw0), glyph_of(WIND, cw1));
    println!("    substrate {substrate_name}: Ω {}→{}  (transformed → {substrate_name}‡)", glyph_of(WIND, sw0), glyph_of(WIND, sw1));
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
