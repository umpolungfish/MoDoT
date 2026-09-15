//! The dialect jump — the navigator↔core face map as a tool.
//!
//! The 12 primitives wear two glyph faces: the cl8nk_navigator axis labels
//! and the Core.lean named axes. Same 12 axes, same tuple positions, same
//! content and family cardinality — lossless faces of one manifold, dialects
//! not universes. Three glyphs (∈, <, ⊡) appear in BOTH faces at DIFFERENT
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
    (1,  "⊢", "D", "Dimensionality",          "𝓕₄"),
    (2,  "⊣", "T", "Topology",                "𝓕₅"),
    (3,  "≻", "R", "Relational Mode",         "𝓕₄"),
    (4,  "≺", "P", "Parity/Symmetry",         "𝓕₅"),
    (5,  "⋈", "F", "Fidelity",                "𝓕₃"),
    (6,  "⊤", "K", "Kinetic Character",       "𝓕₅"),
    (7,  "∈", "G", "Scope/Granularity",       "𝓕₃"),
    (8,  "∋", "∈", "Interaction Grammar",     "𝓕₄"),
    (9,  "⊙", "≺", "Criticality",             "𝓕₅"),
    (10, "⊥", "H", "Chirality",               "𝓕₄"),
    (11, "⊞", "S", "Stoichiometry",           "𝓕₃"),
    (12, "⊡", "⊡", "Topological Protection",  "𝓕₄"),
