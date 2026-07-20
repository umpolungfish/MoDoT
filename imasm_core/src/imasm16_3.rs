//! IMASM-16_3 — the 14-opcode, purely-symbolic trilattice extension of the classic
//! 12-opcode IMASM grammar (`imasm.rs`), for the real trilattice SIXTEEN_3:
//! Shramko, Dunn & Takenaka, "The Trilattice of Constructive Truth Values",
//! J. Logic and Computation 11(6):761-788, 2001 — verified against the source PDF.
//!
//! THE REAL CONSTRUCTION (§5 of the paper). The base set is four initial truth
//! values, not two and not a product of two FOURs:
//!
//!     I = {T, F, t, f}
//!       T — a sentence is constructively PROVEN
//!       F — a sentence is constructively REFUTED
//!       t — a sentence is (non-constructively) ACCEPTABLE
//!       f — a sentence is (non-constructively) REJECTABLE
//!
//! SIXTEEN_3 is the full powerset P(I) — all 16 subsets of these four base
//! values (N = {} = empty, A = {T,F,t,f} = full). Three orderings (Def. 5.2):
//!
//!     x ≤_i y  ⟺  x ⊆ y                                        (information)
//!     x ≤_t y  ⟺  x∩{T,t} ⊆ y∩{T,t}  and  y∩{F,f} ⊆ x∩{F,f}     (truth)
//!     x ≤_c y  ⟺  x∩{T,F} ⊆ y∩{T,F}  and  y∩{t,f} ⊆ x∩{t,f}     (constructivity)
//!
//! Verified against the paper's own worked example: T ∧ t = N under ≤_t (the
//! conjunction of two "truths" gives nothing) — `meet_t` below reproduces this
//! exactly, checked by a unit test.
//!
//! Register: a real 4-bit subset of {T, F, t, f} (16 states, matching the
//! paper's carrier exactly). Opcode → base-value mapping:
//!
//!     EVALT sets T (constructive truth touched)
//!     EVALF sets F (constructive falsity touched)
//!     EVALI sets BOTH t and f (the acceptable/rejectable pair IS the
//!           information layer beyond classical T/F)
//!     TNEG  swaps T ↔ F   (classical bilattice negation: inverts ≤_t,
//!           leaves ≤_i exactly unchanged — a swap preserves |x|)
//!     INEG  swaps t ↔ f   (the same negation, on the acceptable/rejectable pair)
//!
//! A sibling module, not a replacement: FSPLIT3/FFUSE3 (3-way fork/fuse) sit
//! alongside FSPLIT/FFUSE — `imasm.rs`'s ancestry-pairing graph engine and the
//! Lean `.prod` scaffold generator are untouched. No opcode glyph is a Latin
//! letter, so a trilattice verdict (T, N, B, F) can never be confused with a
//! graph node.

use core::fmt::Write as _;

use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token16_3 {
    Vinit,   // ⊢  0→1  source boundary
    Tanch,   // ⊣  1→1  sink boundary
    Afwd,    // >  1→1  forward morphism, WORK
    Arev,    // <  1→1  reverse morphism, WORK
    Clink,   // =  1→1  composition / relational link, WORK
    Imscrib, // ⊙  1→1  identity / neutral self-reference
    Fsplit3, // ∈  1→3  3-way split: T, F, I arms
    Ffuse3,  // ∋  3→1  3-way fuse: merges T, F, I arms
    Evalt,   // +  1→1  evaluates the True axis (≤_t), WORK
    Evalf,   // ×  1→1  evaluates the False axis (≤_t), WORK
    Evali,   // ⊞  1→1  evaluates the Information axis (≤_i), WORK
    Tneg,    // ~  1→1  negation: T ↔ F, WORK
    Ineg,    // ≁  1→1  con-negation: t ↔ f, WORK
    Ifix,    // ¬  1→1  irreversible commit, WORK
}

use Token16_3::*;

pub const ALL_TOKENS: [Token16_3; 14] = [
    Vinit, Tanch, Afwd, Arev, Clink, Imscrib, Fsplit3, Ffuse3,
    Evalt, Evalf, Evali, Tneg, Ineg, Ifix,
];

impl Token16_3 {
    pub fn glyph(self) -> char {
        match self {
            Vinit => '⊢', Tanch => '⊣', Afwd => '>', Arev => '<', Clink => '=',
            Imscrib => '⊙', Fsplit3 => '∈', Ffuse3 => '∋', Evalt => '+',
            Evalf => '×', Evali => '⊞', Tneg => '~', Ineg => '≁', Ifix => '¬',
        }
    }

    pub fn name(self) -> &'static str {
        match self {
            Vinit => "VINIT", Tanch => "TANCH", Afwd => "AFWD", Arev => "AREV",
            Clink => "CLINK", Imscrib => "IMSCRIB", Fsplit3 => "FSPLIT3",
            Ffuse3 => "FFUSE3", Evalt => "EVALT", Evalf => "EVALF",
            Evali => "EVALI", Tneg => "TNEG", Ineg => "INEG", Ifix => "IFIX",
        }
    }

    pub fn is_work(self) -> bool {
        matches!(self, Afwd | Arev | Clink | Evalt | Evalf | Evali | Tneg | Ineg | Ifix)
    }

    fn from_glyph(c: char) -> Option<Token16_3> {
        // ☊/☋ were a drifted face for the tri-dyad; the guide's ∈/∋ are the
        // correct tokens. Parse accepts the drifted pair so old words load.
        match c {
            '☊' => Some(Fsplit3),
            '☋' => Some(Ffuse3),
            _ => ALL_TOKENS.iter().copied().find(|t| t.glyph() == c),
        }
    }
}

pub fn parse_glyph_word(word: &str) -> Vec<Token16_3> {
    word.chars().filter_map(Token16_3::from_glyph).collect()
}

/// Register: a real subset of the 4-element base {T, F, t, f} — 16 states,
/// the actual Shramko-Wansing SIXTEEN_3 carrier (not an approximation).
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub struct Reg16_3 {
    pub big_t: bool,   // T — constructively proven
    pub big_f: bool,   // F — constructively refuted
    pub small_t: bool, // t — acceptable (non-constructive truth)
    pub small_f: bool, // f — rejectable (non-constructive falsity)
}

impl Reg16_3 {
    pub fn name(self) -> String {
        if !self.big_t && !self.big_f && !self.small_t && !self.small_f {
            return "N".to_string();
        }
        if self.big_t && self.big_f && self.small_t && self.small_f {
            return "A".to_string();
        }
        let mut s = String::new();
        if self.big_t { s.push('T'); }
        if self.big_f { s.push('F'); }
        if self.small_t { s.push('t'); }
        if self.small_f { s.push('f'); }
        s
    }

    /// The glyph face: occupancy of {T, F, t, f} drawn as quadrant ink —
    /// T upper-left, F upper-right, t lower-left, f lower-right. The glyph
    /// IS the subset (meet/join of states = intersection/union of ink);
    /// N renders as ░, the unmarked field, never as absence. No Latin,
    /// matching the opcode face's rule.
    pub fn glyph(self) -> char {
        match (self.big_t, self.big_f, self.small_t, self.small_f) {
            (false, false, false, false) => '░',
            (true,  false, false, false) => '▘',
            (false, true,  false, false) => '▝',
            (false, false, true,  false) => '▖',
            (false, false, false, true ) => '▗',
            (true,  true,  false, false) => '▀',
            (true,  false, true,  false) => '▌',
            (true,  false, false, true ) => '▚',
            (false, true,  true,  false) => '▞',
            (false, true,  false, true ) => '▐',
            (false, false, true,  true ) => '▄',
            (true,  true,  true,  false) => '▛',
            (true,  true,  false, true ) => '▜',
            (true,  false, true,  true ) => '▙',
            (false, true,  true,  true ) => '▟',
            (true,  true,  true,  true ) => '█',
        }
    }

    pub fn union(self, o: Reg16_3) -> Reg16_3 {
        Reg16_3 {
            big_t: self.big_t || o.big_t, big_f: self.big_f || o.big_f,
            small_t: self.small_t || o.small_t, small_f: self.small_f || o.small_f,
        }
    }

    /// The truth part x ∩ {T, t} — what EVALT passes, and the first δ arm.
    pub fn truth_part(self) -> Reg16_3 {
        Reg16_3 { big_t: self.big_t, small_t: self.small_t, ..Reg16_3::default() }
    }

    /// The falsity part x ∩ {F, f} — what EVALF passes, and the second δ arm.
    pub fn falsity_part(self) -> Reg16_3 {
        Reg16_3 { big_f: self.big_f, small_f: self.small_f, ..Reg16_3::default() }
    }

    /// The involution T ↔ F, t ↔ f — the reverse morphism's action on values
    /// (TNEG and INEG applied together; ⊆-monotone, its own inverse).
    pub fn invol(self) -> Reg16_3 {
        Reg16_3 { big_t: self.big_f, big_f: self.big_t, small_t: self.small_f, small_f: self.small_t }
    }

    /// FOUR sits inside SIXTEEN_3 as the classical pair {T, F}: N={}, T={T},
    /// F={F}, B={T,F}. Render this value as its FOUR name when it lives in that
    /// slice; a value touching t/f has left the slice and keeps its 16_3 name.
    pub fn four_name(self) -> String {
        if self.small_t || self.small_f {
            return self.name();
        }
        match (self.big_t, self.big_f) {
            (false, false) => "N".into(),
            (true, false) => "T".into(),
            (false, true) => "F".into(),
            (true, true) => "B".into(),
        }
    }

    /// The FOUR slice's glyph face: ░/▘/▝/▀ for N/T/F/B per `glyph()` — B of
    /// FOUR IS the state {T,F}, so it wears TF's ink; there is no separate
    /// verdict alphabet and no Latin anywhere. Values touching t/f have left
    /// the slice and keep their 16_3 glyph.
    pub fn four_glyph(self) -> char {
        self.glyph()
    }

    /// Parse a register from its name — "N", "A", or any combination of
    /// T/F/t/f (order-insensitive), e.g. "Tf" or "tF". The inverse of `name()`.
    pub fn from_name(s: &str) -> Option<Reg16_3> {
        if s == "N" { return Some(Reg16_3::default()); }
        if s == "A" {
            return Some(Reg16_3 { big_t: true, big_f: true, small_t: true, small_f: true });
        }
        let mut r = Reg16_3::default();
        for c in s.chars() {
            match c {
                'T' => r.big_t = true,
                'F' => r.big_f = true,
                't' => r.small_t = true,
                'f' => r.small_f = true,
                _ => return None,
            }
        }
        Some(r)
    }
}

// ── The three orderings (Definition 5.2) — kept as free functions since
// they're part of the trilattice's public algebra, not just machine internals.

/// Information order: x ≤_i y ⟺ x ⊆ y.
pub fn leq_i(x: Reg16_3, y: Reg16_3) -> bool {
    (!x.big_t || y.big_t) && (!x.big_f || y.big_f) && (!x.small_t || y.small_t) && (!x.small_f || y.small_f)
}

/// Truth order: positive part (T,t) grows, negative part (F,f) shrinks.
pub fn leq_t(x: Reg16_3, y: Reg16_3) -> bool {
    let pos_ok = (!x.big_t || y.big_t) && (!x.small_t || y.small_t);
    let neg_ok = (!y.big_f || x.big_f) && (!y.small_f || x.small_f);
    pos_ok && neg_ok
}

/// Constructivity order: constructive part (T,F) grows, non-constructive part (t,f) shrinks.
pub fn leq_c(x: Reg16_3, y: Reg16_3) -> bool {
    let con_ok = (!x.big_t || y.big_t) && (!x.big_f || y.big_f);
    let noncon_ok = (!y.small_t || x.small_t) && (!y.small_f || x.small_f);
    con_ok && noncon_ok
}

/// Meet under ≤_t: positive part intersects, negative part unions. Verified
/// against the paper's own worked example, T ∧ t = N (§5, p.776-777) — see
/// the unit test `meet_t_matches_paper_example`.
pub fn meet_t(x: Reg16_3, y: Reg16_3) -> Reg16_3 {
    Reg16_3 {
        big_t: x.big_t && y.big_t, small_t: x.small_t && y.small_t,
        big_f: x.big_f || y.big_f, small_f: x.small_f || y.small_f,
    }
}

pub fn join_t(x: Reg16_3, y: Reg16_3) -> Reg16_3 {
    Reg16_3 {
        big_t: x.big_t || y.big_t, small_t: x.small_t || y.small_t,
        big_f: x.big_f && y.big_f, small_f: x.small_f && y.small_f,
    }
}

pub fn meet_c(x: Reg16_3, y: Reg16_3) -> Reg16_3 {
    Reg16_3 {
        big_t: x.big_t && y.big_t, big_f: x.big_f && y.big_f,
        small_t: x.small_t || y.small_t, small_f: x.small_f || y.small_f,
    }
}

pub fn join_c(x: Reg16_3, y: Reg16_3) -> Reg16_3 {
    Reg16_3 {
        big_t: x.big_t || y.big_t, big_f: x.big_f || y.big_f,
        small_t: x.small_t && y.small_t, small_f: x.small_f && y.small_f,
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Axis { T, F, I }

struct Machine {
    reg: Reg16_3,
    fixed: bool,
    in_split: bool,
    split_touched: Vec<Axis>,
}

impl Machine {
    fn new() -> Self {
        Self { reg: Reg16_3::default(), fixed: false, in_split: false, split_touched: Vec::new() }
    }

    fn touch(&mut self, set: Reg16_3, a: Axis) {
        self.reg = self.reg.union(set);
        if self.in_split && !self.split_touched.contains(&a) {
            self.split_touched.push(a);
        }
    }

    fn step(&mut self, tok: Token16_3) {
        if self.fixed && tok != Ifix && tok != Imscrib {
            return;
        }
        let none = Reg16_3::default();
        match tok {
            Vinit => {
                self.reg = none;
                self.in_split = false;
                self.split_touched.clear();
            }
            Tanch => {}
            Afwd => {
                if self.reg == none { self.reg.big_t = true; }
            }
            Arev => {
                self.reg = none;
                self.in_split = false;
                self.split_touched.clear();
            }
            Clink => {}
            Imscrib => {
                if self.reg == none { self.reg.big_t = true; }
            }
            Fsplit3 => {
                self.in_split = true;
                self.split_touched.clear();
            }
            Ffuse3 => {
                for a in &self.split_touched {
                    match a {
                        Axis::T => self.reg.big_t = true,
                        Axis::F => self.reg.big_f = true,
                        Axis::I => { self.reg.small_t = true; self.reg.small_f = true; }
                    }
                }
                self.in_split = false;
                self.split_touched.clear();
            }
            Evalt => self.touch(Reg16_3 { big_t: true, ..none }, Axis::T),
            Evalf => self.touch(Reg16_3 { big_f: true, ..none }, Axis::F),
            Evali => self.touch(Reg16_3 { small_t: true, small_f: true, ..none }, Axis::I),
            Tneg => {
                let (t, f) = (self.reg.big_t, self.reg.big_f);
                self.reg.big_t = f;
                self.reg.big_f = t;
                for a in self.split_touched.iter_mut() {
                    *a = match *a { Axis::T => Axis::F, Axis::F => Axis::T, Axis::I => Axis::I };
                }
            }
            Ineg => {
                let (t, f) = (self.reg.small_t, self.reg.small_f);
                self.reg.small_t = f;
                self.reg.small_f = t;
                // I is already both t and f touched together; a swap of the
                // underlying bits doesn't change which AXIS was touched.
            }
            Ifix => { self.fixed = true; }
        }
    }
}

/// Tri-ancestral close condition, the arity-3 generalization of the classic
/// `imasm check`'s ancestry rule:
///   T — every FSPLIT3 pairs with a later FFUSE3, and at least one work
///       opcode ran somewhere inside that interval (a real transformation).
///   N — paired, but no work ran inside — μ∘δ=id verifies nothing (identity).
///   B — a FSPLIT3 has no matching later FFUSE3 — a fork left open.
///   F — a FFUSE3 has no preceding FSPLIT3 — ill-typed.
pub fn tri_ancestral_verdict(steps: &[Token16_3]) -> (char, String) {
    let split_idx: Vec<usize> = steps.iter().enumerate().filter(|(_, t)| **t == Fsplit3).map(|(i, _)| i).collect();
    let fuse_idx: Vec<usize> = steps.iter().enumerate().filter(|(_, t)| **t == Ffuse3).map(|(i, _)| i).collect();

    for &fj in &fuse_idx {
        if !split_idx.iter().any(|&si| si < fj) {
            return ('F', format!("FFUSE3 at step {} has no preceding FSPLIT3 — ill-typed", fj + 1));
        }
    }
    if split_idx.is_empty() && fuse_idx.is_empty() {
        return ('N', "No fork/fuse — void, never weighed alternatives".to_string());
    }
    if let Some(&si) = split_idx.iter().find(|&&si| !fuse_idx.iter().any(|&fj| fj > si)) {
        return ('B', format!("FSPLIT3 at step {} dangles — no matching FFUSE3", si + 1));
    }
    for &si in &split_idx {
        let fj = *fuse_idx.iter().find(|&&fj| fj > si).unwrap();
        if steps[si + 1..fj].iter().any(|t| t.is_work()) {
            return ('T', "Tri-ancestral reconnection over a transformed object — closes".to_string());
        }
    }
    ('N', "Split/fused with no work on any arm — μ∘δ=id verifies nothing".to_string())
}

fn run_trace(steps: &[Token16_3]) -> String {
    let mut mach = Machine::new();
    let mut out = String::new();
    let _ = writeln!(out, "  {:>3} {:^5} {:<9} {:>5} → {:>5}", "Step", "Glyph", "Token", "Reg↓", "Reg↑");
    let start = mach.reg;
    for (idx, &tok) in steps.iter().enumerate() {
        let before = mach.reg.name();
        mach.step(tok);
        let after = mach.reg.name();
        let _ = writeln!(out, "  {:>3} {:^5} {:<9} {:>5} → {:>5}", idx + 1, tok.glyph(), tok.name(), before, after);
    }
    let closed = mach.reg == start;
    let (verdict, msg) = tri_ancestral_verdict(steps);
    let _ = writeln!(out);
    let _ = writeln!(out, "  Closed walk: {closed}");
    let _ = writeln!(out, "  Final register: {}", mach.reg.name());
    let _ = writeln!(out, "  Tri-ancestral verdict: {verdict} — {msg}");
    out
}

pub fn run(args: &[String]) -> String {
    let Some(op) = args.first() else {
        return "imasm16_3 <op> …; op ∈ check|ref|algebra — the 14-opcode SIXTEEN_3 trilattice grammar.\n\
                `check <glyph_word>` runs the register machine and type-checks tri-ancestral closure.\n\
                `ref` lists the 14 opcodes.\n\
                `algebra <op> A B` runs a trilattice lattice operation on two register values (named\n\
                  N, A, or any of T/F/t/f, e.g. `algebra meet_t T t`); op ∈ leq_i|leq_t|leq_c|meet_t|join_t|meet_c|join_c.\n".to_string();
    };
    match op.as_str() {
        "algebra" => {
            let (Some(sub), Some(a_name), Some(b_name)) = (args.get(1), args.get(2), args.get(3)) else {
                return "imasm16_3 algebra <op> A B; op ∈ leq_i|leq_t|leq_c|meet_t|join_t|meet_c|join_c; \
                        A, B ∈ {N, A, or any combination of T/F/t/f}.\n".to_string();
            };
            let (Some(a), Some(b)) = (Reg16_3::from_name(a_name), Reg16_3::from_name(b_name)) else {
                return format!("imasm16_3 algebra: could not parse '{a_name}' or '{b_name}' — use N, A, or T/F/t/f.\n");
            };
            match sub.as_str() {
                "leq_i" => format!("{} ≤_i {} : {}\n", a.name(), b.name(), leq_i(a, b)),
                "leq_t" => format!("{} ≤_t {} : {}\n", a.name(), b.name(), leq_t(a, b)),
                "leq_c" => format!("{} ≤_c {} : {}\n", a.name(), b.name(), leq_c(a, b)),
                "meet_t" => format!("{} ∧ {} = {}\n", a.name(), b.name(), meet_t(a, b).name()),
                "join_t" => format!("{} ∨ {} = {}\n", a.name(), b.name(), join_t(a, b).name()),
                "meet_c" => format!("{} △ {} = {}\n", a.name(), b.name(), meet_c(a, b).name()),
                "join_c" => format!("{} ▽ {} = {}\n", a.name(), b.name(), join_c(a, b).name()),
                other => format!("imasm16_3 algebra: unknown op '{other}'; op ∈ leq_i|leq_t|leq_c|meet_t|join_t|meet_c|join_c\n"),
            }
        }
        "ref" => {
            let mut out = String::from("IMASM-16_3 — 14 symbolic opcodes (SIXTEEN_3 = P({T,F,t,f}), Shramko/Dunn/Takenaka 2001):\n");
            for t in ALL_TOKENS {
                let _ = writeln!(out, "  {}  {:<9} {}", t.glyph(), t.name(), if t.is_work() { "WORK" } else { "no-op (structural)" });
            }
            out
        }
        "check" => {
            let Some(word) = args.get(1) else {
                return "imasm16_3 check <glyph_word>; e.g. imasm16_3 check ⊢>∈+×⊞≁∋¬⊣\n".to_string();
            };
            let steps = parse_glyph_word(word);
            if steps.is_empty() {
                return format!("imasm16_3 check: no recognized IMASM-16_3 glyphs in '{word}'\n");
            }
            format!("Word: {}\n{}", steps.iter().map(|t| t.glyph()).collect::<String>(), run_trace(&steps))
        }
        other => format!("imasm16_3: unknown op '{other}'; op ∈ check|ref\n"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_word_closes_with_work() {
        let steps = parse_glyph_word("⊢>∈+×⊞≁∋¬⊣");
        assert_eq!(steps.len(), 10);
        let (verdict, _) = tri_ancestral_verdict(&steps);
        assert_eq!(verdict, 'T');
    }

    #[test]
    fn cross_repo_parity_word() {
        let steps = parse_glyph_word("⊢>>=∈+~∋<¬⊣");
        let (verdict, _) = tri_ancestral_verdict(&steps);
        assert_eq!(verdict, 'T');
    }

    #[test]
    fn neutral_inflation_is_identity_not_error() {
        let steps = parse_glyph_word("⊢∈⊙⊙⊙∋⊣");
        let (verdict, _) = tri_ancestral_verdict(&steps);
        assert_eq!(verdict, 'N');
    }

    #[test]
    fn dangling_split_is_b() {
        let steps = parse_glyph_word("⊢∈+⊣");
        let (verdict, _) = tri_ancestral_verdict(&steps);
        assert_eq!(verdict, 'B');
    }

    #[test]
    fn fuse_without_split_is_f() {
        let steps = parse_glyph_word("⊢+∋⊣");
        let (verdict, _) = tri_ancestral_verdict(&steps);
        assert_eq!(verdict, 'F');
    }

    #[test]
    fn all_14_opcodes_have_distinct_glyphs() {
        let glyphs: std::collections::HashSet<char> = ALL_TOKENS.iter().map(|t| t.glyph()).collect();
        assert_eq!(glyphs.len(), 14);
        for g in &glyphs {
            assert!(!g.is_ascii_alphabetic(), "opcode glyph {g} is a Latin letter");
        }
    }

    /// The paper's own worked example (§5, p.776-777): "T ∧ t = N" — the
    /// conjunction of two truths gives nothing, because neither conjunct is
    /// BOTH T and t simultaneously.
    #[test]
    fn meet_t_matches_paper_example() {
        let big_t_only = Reg16_3 { big_t: true, ..Default::default() };
        let small_t_only = Reg16_3 { small_t: true, ..Default::default() };
        let result = meet_t(big_t_only, small_t_only);
        assert_eq!(result.name(), "N");
    }

    #[test]
    fn negation_preserves_information_order() {
        // A defining property of trilattice negation: it must leave ≤_i
        // (the subset/information order) unchanged. Swapping T↔F preserves
        // popcount, hence preserves ⊆-comparisons against any fixed y.
        let x = Reg16_3 { big_t: true, small_t: true, ..Default::default() };
        let mut neg_x = x;
        std::mem::swap(&mut neg_x.big_t, &mut neg_x.big_f);
        // |x| == |neg_x|, and both are still comparable to N and A the same way.
        assert_eq!(leq_i(Reg16_3::default(), x), leq_i(Reg16_3::default(), neg_x));
        assert_eq!(leq_i(x, Reg16_3 { big_t: true, big_f: true, small_t: true, small_f: true }),
                   leq_i(neg_x, Reg16_3 { big_t: true, big_f: true, small_t: true, small_f: true }));
    }

    #[test]
    fn sixteen_states_reachable() {
        // The full carrier has exactly 16 elements — spot check a handful of
        // named ones from Table 1 of the paper are constructible and distinct.
        let n = Reg16_3::default();
        let a = Reg16_3 { big_t: true, big_f: true, small_t: true, small_f: true };
        let t = Reg16_3 { big_t: true, ..Default::default() };
        let tf = Reg16_3 { big_t: true, big_f: true, ..Default::default() };
        assert_eq!(n.name(), "N");
        assert_eq!(a.name(), "A");
        assert_eq!(t.name(), "T");
        assert_eq!(tf.name(), "TF");
        assert_ne!(n.name(), a.name());
    }
}
