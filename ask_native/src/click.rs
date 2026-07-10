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

/// CLI entry: `./ask --click <A> <B> [--catalyst <C>]`. Prints the live-pair charge
/// diagnostic and the click verdict for two catalog fragments, optionally lowering
/// the threshold with a catalyst. Returns a process exit code.
pub fn run_click(
    catalog: Option<&[CatalogEntry]>,
    name_a: &str,
    name_b: &str,
    theta: f32,
    catalyst_name: Option<&str>,
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
