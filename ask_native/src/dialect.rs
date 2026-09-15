//! The twelve primitives — position, glyph, axis name, family.
//!
//! One face only. The glyph set is exactly:
//! ⊢ ⊣ ≻ ≺ ⋈ ⊤ ∈ ∋ ⊙ ⊥ ⊞ ⊡ — nothing else parses.
//!
//! `dialect`         — print the twelve-axis table.
//! `dialect <axis>`  — resolve one glyph or axis name to its position.
//!
//! Pure computation; the catalog is not consulted.

use std::fmt::Write as _;

/// (position, glyph, axis name, family)
const MAP: [(u8, &str, &str, &str); 12] = [
    (1,  "⊢", "Dimensionality",          "𝓕₄"),
    (2,  "⊣", "Topology",                "𝓕₅"),
    (3,  "≻", "Relational Mode",         "𝓕₄"),
    (4,  "≺", "Parity/Symmetry",         "𝓕₅"),
    (5,  "⋈", "Fidelity",                "𝓕₃"),
    (6,  "⊤", "Kinetic Character",       "𝓕₅"),
    (7,  "∈", "Scope/Granularity",       "𝓕₃"),
    (8,  "∋", "Interaction Grammar",     "𝓕₄"),
    (9,  "⊙", "Criticality",             "𝓕₅"),
    (10, "⊥", "Chirality",               "𝓕₄"),
    (11, "⊞", "Stoichiometry",           "𝓕₃"),
    (12, "⊡", "Topological Protection",  "𝓕₄"),
];

fn table() -> String {
    let mut out = String::new();
    let _ = writeln!(out, "DIALECT — the twelve primitives (position, glyph, axis, family):\n");
    let _ = writeln!(out, "  Pos  Glyph  Axis                     Family");
    for (pos, glyph, name, fam) in MAP {
        let _ = writeln!(out, "  {pos:>3}  {glyph:^5}  {name:<24} {fam}");
    }
    out
}

pub fn run(args: &[String]) -> String {
    let Some(q) = args.first() else {
        return table();
    };
    let q = q.trim();
    let mut out = String::new();
    let mut hits = 0;
    for (pos, glyph, name, fam) in MAP {
        if glyph == q || name.eq_ignore_ascii_case(q) {
            hits += 1;
            let _ = writeln!(
                out,
                "  '{q}' → axis {pos}: {glyph} ({name}, {fam})"
            );
        }
    }
    if hits == 0 {
        return format!(
            "dialect: '{q}' is not a primitive glyph or axis name.\n\n{}",
            table()
        );
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn twelve_axes() {
        assert_eq!(MAP.len(), 12);
    }

    #[test]
    fn glyph_set_is_exactly_the_twelve() {
        let glyphs: Vec<&str> = MAP.iter().map(|(_, g, _, _)| *g).collect();
        assert_eq!(
            glyphs,
            vec!["⊢", "⊣", "≻", "≺", "⋈", "⊤", "∈", "∋", "⊙", "⊥", "⊞", "⊡"]
        );
    }

    #[test]
    fn glyph_resolves_to_position() {
        assert!(run(&["⊡".into()]).contains("axis 12"));
    }

    #[test]
    fn unknown_label_returns_table() {
        assert!(run(&["Z".into()]).contains("not a primitive"));
    }
}
