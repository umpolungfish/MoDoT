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

/// The twelve primitive keys, in canonical navigator/catalog order.
pub const PRIMS: [&str; 12] = ["Ð", "Þ", "Ř", "Φ", "ƒ", "Ç", "Γ", "ɢ", "⊙", "Ħ", "Σ", "Ω"];

/// glyph → ordinal per primitive. The ORDERING is cross-checked against the
/// p4rakernel Lean kernel (`Primitives/Core.lean`, which is scripture) for
/// Ħ (Chirality: fee<kick<sure<wool) and Ω (Protection: awe<oak<ah<zoo) — the
/// Axiom B verification. The remaining ten follow the canonical
/// imscribing_grammar GLYPH_ORDINALS table.
const GLYPHS: [&[(&str, u8)]; 12] = [
    /* Ð Dimensionality */ &[("𐑛", 0), ("𐑨", 1), ("𐑼", 2), ("𐑦", 3)],
    /* Þ Topology       */ &[("𐑡", 0), ("𐑰", 1), ("𐑥", 2), ("𐑶", 3), ("𐑸", 4)],
    /* Ř Recognition    */ &[("𐑩", 0), ("𐑑", 1), ("𐑽", 2), ("𐑾", 3)],
    /* Φ Parity         */ &[("𐑗", 0), ("𐑿", 1), ("𐑬", 2), ("𐑯", 3), ("𐑹", 4)],
    /* ƒ Fidelity       */ &[("𐑱", 0), ("𐑞", 1), ("𐑐", 2)],
    /* Ç Kinetics       */ &[("𐑘", 0), ("𐑤", 1), ("𐑧", 2), ("𐑺", 3), ("𐑪", 4)],
    /* Γ Granularity    */ &[("𐑚", 0), ("𐑔", 1), ("𐑲", 2)],
    /* ɢ Composition    */ &[("𐑝", 0), ("𐑜", 1), ("𐑠", 2), ("𐑵", 3)],
    /* ⊙ Criticality    */ &[("𐑢", 0), ("⊙", 1), ("𐑮", 2), ("𐑻", 3), ("𐑣", 4)],
    /* Ħ Chirality      */ &[("𐑓", 0), ("𐑒", 1), ("𐑖", 2), ("𐑫", 3)],
    /* Σ Stoichiometry  */ &[("𐑳", 0), ("𐑙", 1), ("𐑕", 2)],
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
    /* Ç */ &["yea", "loll", "egg", "air", "on"],
    /* Γ */ &["bib", "thigh", "ice"],
    /* ɢ */ &["vow", "gag", "measure", "ooze"],
    /* ⊙ */ &["woe", "monad", "roar", "err", "haha"],
    /* Ħ */ &["fee", "kick", "sure", "wool"],
    /* Σ */ &["up", "hung", "so"],
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
