//! The classic 12-opcode Token — moved here from ask_native/src/imasm.rs so
//! every consumer derives its faces from ONE definition.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token {
    Vinit,
    Tanch,
    Afwd,
    Arev,
    Clink,
    Imscrib,
    Fsplit,
    Ffuse,
    Evalt,
    Evalf,
    Engagr,
    Ifix,
}

impl Token {
    pub fn name(self) -> &'static str {
        match self {
            Token::Vinit => "VINIT",
            Token::Tanch => "TANCH",
            Token::Afwd => "AFWD",
            Token::Arev => "AREV",
            Token::Clink => "CLINK",
            Token::Imscrib => "IMSCRIB",
            Token::Fsplit => "FSPLIT",
            Token::Ffuse => "FFUSE",
            Token::Evalt => "EVALT",
            Token::Evalf => "EVALF",
            Token::Engagr => "ENGAGR",
            Token::Ifix => "IFIX",
        }
    }

    /// Single-glyph code — one symbol per opcode, so an opcode word reads as a
    /// sequence (a codon string) instead of a space-delimited token list. The alphabet
    /// is not invented: it references the per-token glyph vocabulary the IMSCRIBr pen-mode
    /// READING_GUIDE already fixes (ob3ect/READING_GUIDE.md §3). Six are the guide's own
    /// midpoint glyphs (FSPLIT ◇, FFUSE ●, EVALT +, EVALF ×, CLINK from its double-line
    /// ═ → =); IFIX is the guide's stated meaning "fix (¬)"; AFWD/AREV are the guide's
    /// forward→/reverse→ as > / <. Every token is SYMBOLIC — no Latin initials: VINIT ⊢ and
    /// TANCH ⊣ are the opening/closing boundary turnstiles, ENGAGR ⊞ is the Belnap Both it
    /// holds, and IMSCRIB is ⊙ — the Grammar's own self-modeling glyph, which is exactly what
    /// IMSCRIB means (identity / self-reference). Distinct by construction, so `parse`
    /// round-trips every code back to its token. Retired and no longer parsing: the letter
    /// codes V/T/B, and ← for IMSCRIB. Full names and the short forms VI/TA/EG/IM still do.
    /// IMSCRIB is ⊙ for a reason, not by availability: imscribing is the very act of
    /// INCLOSURE — the monadic operation itself — hence self-referential, and so referenced
    /// self-referentially. The glyph is a boundary drawn around its own centre, denoting the
    /// act of denoting. Its appearance as Criticality in the 12-primitive notation is not a
    /// collision: it is the same structure surfacing wherever inclosure closes on itself.
    pub fn code(self) -> &'static str {
        match self {
            Token::Vinit => "⊢",
            Token::Tanch => "⊣",
            Token::Afwd => ">",
            Token::Arev => "<",
            Token::Clink => "=",
            Token::Imscrib => "⊙",
            Token::Fsplit => "◇",
            Token::Ffuse => "●",
            Token::Evalt => "+",
            Token::Evalf => "×",
            Token::Engagr => "⊞",
            Token::Ifix => "¬",
        }
    }

    /// Does this opcode TRANSFORM the object (do work), as opposed to being an
    /// identity/structural node? IMSCRIB is the identity morphism, VINIT/TANCH are
    /// source/sink, FSPLIT/FFUSE are the split/fuse themselves — none transform.
    /// A μ∘δ closure whose arms carry only these is identity, not a type-check.
    pub fn transforms(self) -> bool {
        matches!(
            self,
            Token::Afwd | Token::Arev | Token::Clink | Token::Evalt | Token::Evalf
                | Token::Engagr | Token::Ifix
        )
    }

    /// (arity_in, arity_out) — the max ports the opcode may carry.
    pub fn arity(self) -> (usize, usize) {
        match self {
            Token::Vinit => (0, 1),
            Token::Fsplit => (1, 2),
            Token::Ffuse => (2, 1),
            _ => (1, 1),
        }
    }

    /// The full opcode table as JSON, for the export manifest: name, glyph,
    /// and arity, so the composer surface renders ports without re-deriving them.
    pub fn parse_all_names() -> serde_json::Value {
        let all = [
            Token::Vinit, Token::Tanch, Token::Afwd, Token::Arev, Token::Clink,
            Token::Imscrib, Token::Fsplit, Token::Ffuse, Token::Evalt,
            Token::Evalf, Token::Engagr, Token::Ifix,
        ];
        serde_json::Value::Array(
            all.iter()
                .map(|t| {
                    let (ain, aout) = t.arity();
                    serde_json::json!({
                        "name": t.name(), "code": t.code(),
                        "arity_in": ain, "arity_out": aout,
                    })
                })
                .collect(),
        )
    }

    /// Accepts full names (VINIT) and the IMSCRIBr short forms (VI, FS, FF, …),
    /// case-insensitively.
    pub fn parse(s: &str) -> Option<Token> {
        let u = s.trim().to_ascii_uppercase();
        Some(match u.as_str() {
            "VINIT" | "VI" | "⊢" => Token::Vinit,
            "TANCH" | "TA" | "⊣" => Token::Tanch,
            "AFWD" | "AF" | ">" => Token::Afwd,
            "AREV" | "AR" | "<" => Token::Arev,
            "CLINK" | "CL" | "=" | "═" => Token::Clink,
            "IMSCRIB" | "IMSCRIBE" | "IM" | "⊙" => Token::Imscrib,
            "FSPLIT" | "FS" | "SPLIT" | "DELTA" | "◇" | "δ" => Token::Fsplit,
            "FFUSE" | "FF" | "FUSE" | "MU" | "●" | "μ" => Token::Ffuse,
            "EVALT" | "ET" | "+" => Token::Evalt,
            "EVALF" | "EF" | "×" => Token::Evalf,
            "ENGAGR" | "EG" | "⊞" => Token::Engagr,
            "IFIX" | "IX" | "FIX" | "¬" => Token::Ifix,
            _ => return None,
        })
    }
}
