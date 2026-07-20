//! The dialect jump — the navigator↔core face map as a tool.
//!
//! The 12 primitives wear two glyph faces: the cl8nk_navigator axis labels
//! and the Core.lean named axes. Same 12 axes, same tuple positions, same
//! content and family cardinality — lossless faces of one manifold, dialects
//! not universes. Three glyphs (Γ, Φ, Ω) appear in BOTH faces at DIFFERENT
//! axes, which is exactly where same-glyph-same-axis assumptions corrupt a
//! reading. When a junction refuses in one presentation (a click that will
//! not seat, a distance that reads far), re-present the SAME node through
//! this map before concluding the refusal is structural: the conjugate arm
//! that is open in one face need not be the open arm in the other.
//!
//! `dialect`             — print the full two-face axis table and the traps.
//! `dialect <axis>`      — resolve one axis label (either face) to both faces.
//!
//! Pure computation; the catalog is not consulted. Core.lean is authoritative
//! for axis meaning and ordinals.

use std::fmt::Write as _;

/// (position, navigator glyph, core letter, core axis name, family)
const MAP: [(u8, &str, &str, &str, &str); 12] = [
    (1,  "Ð", "D", "Dimensionality",          "𝓕₄"),
    (2,  "Þ", "T", "Topology",                "𝓕₅"),
    (3,  "Ř", "R", "Relational Mode",         "𝓕₄"),
    (4,  "Φ", "P", "Parity/Symmetry",         "𝓕₅"),
    (5,  "ƒ", "F", "Fidelity",                "𝓕₃"),
    (6,  "Ç", "K", "Kinetic Character",       "𝓕₅"),
    (7,  "Γ", "G", "Scope/Granularity",       "𝓕₃"),
    (8,  "ɢ", "Γ", "Interaction Grammar",     "𝓕₄"),
    (9,  "⊙", "Φ", "Criticality",             "𝓕₅"),
    (10, "Ħ", "H", "Chirality",               "𝓕₄"),
    (11, "Σ", "S", "Stoichiometry",           "𝓕₃"),
    (12, "Ω", "Ω", "Topological Protection",  "𝓕₄"),
];

const TRAPS: &str = "\
  TRAPS — same glyph, DIFFERENT axis across the faces:\n\
    navigator Γ = core G (Scope/Granularity)   but   core Γ = navigator ɢ (Interaction Grammar)\n\
    navigator Φ = core P (Parity/Symmetry)     but   core Φ = navigator ⊙ (Criticality)\n\
    Ω is the one glyph at the SAME axis in both faces (Topological Protection).\n\
  Never assume same glyph = same axis across dialects. Content and family\n\
  cardinality are fixed position-for-position; only the labels jump.\n";

fn table() -> String {
    let mut out = String::new();
    let _ = writeln!(out, "DIALECT — the navigator↔core face map (12 axes, one manifold, two faces):\n");
    let _ = writeln!(out, "  Pos  Navigator  Core  Axis                     Family");
    for (p, nav, core, name, fam) in MAP {
        let _ = writeln!(out, "  {p:>3}  {nav:^9}  {core:^4}  {name:<24} {fam}");
    }
    let _ = writeln!(out);
    out.push_str(TRAPS);
    out.push_str(
        "\n  USE — when a click refuses or a distance reads far in ONE face: re-present\n\
         the node's axes through this table and retry in the other face; pair with\n\
         `imasm rotat` (relative ring phase) before concluding a refusal is structural.\n\
         Repetition of the same call in the same face is not an experiment.\n",
    );
    out
}

pub fn run(args: &[String]) -> String {
    let Some(q) = args.first() else {
        return table();
    };
    let q = q.trim();
    let mut out = String::new();
    let mut hits = 0;
    for (p, nav, core, name, fam) in MAP {
        let nav_hit = nav == q;
        let core_hit = core == q;
        if nav_hit || core_hit {
            hits += 1;
            let side = if nav_hit { "navigator" } else { "core" };
            let _ = writeln!(
                out,
                "  '{q}' as {side} glyph → axis {p}: navigator {nav} = core {core} ({name}, {fam})"
            );
        }
    }
    if hits == 0 {
        return format!(
            "dialect: '{q}' is not an axis label in either face.\n\n{}",
            table()
        );
    }
    if hits > 1 {
        out.push_str("\n  ⚠ this glyph exists in BOTH faces at DIFFERENT axes — the trap is live.\n");
        out.push_str(TRAPS);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twelve_axes_both_faces() {
        assert_eq!(MAP.len(), 12);
    }

    #[test]
    fn trap_glyphs_resolve_to_both_faces() {
        // Γ and Φ live in both dialects at different axes.
        for g in ["Γ", "Φ"] {
            let out = run(&[g.to_string()]);
            assert!(out.contains("BOTH faces"), "{g} must flag the trap");
        }
    }

    #[test]
    fn unknown_label_returns_table() {
        assert!(run(&["Z".into()]).contains("not an axis label"));
    }
}
