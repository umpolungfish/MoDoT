//! `imasm learn` — the excription/imscription learning loop.
//!
//! One model takes a word, iterates nearby words, excribes each into an object
//! (a GUESS: the model names one concrete thing whose structure IS the word),
//! imscribes the OBJECT back into a word, checks the recovered word, measures
//! the residual between the two words, and updates its knowledge. This is
//! verification as imscription run on the model itself: δ excribes the word
//! into a genuinely different form, μ recovers it, and μ∘δ = id exactly when
//! the residual is zero. The guess is the fixed point the two words pivot on;
//! it is kept in the record. Where the residual is not zero, the confusion is
//! recorded, distilled into a lesson, and the lesson rides the imscriber's
//! next prompt. The walk moves to the highest-residual neighbor, so the loop
//! spends its rounds where its knowledge is thinnest.
//!
//! Both faces run here: a word carrying tri tokens (∈ ∋ ~ ≁) is read by the
//! SIXTEEN_3 trilattice grammar, any other by the classic 12-opcode grammar —
//! one loop, one knowledge file, the face chosen by the word itself.

use crate::imasm::{from_sequence, match_pairs, ClosureState, Token};
use imasm_core::imasm16_3::{parse_glyph_word, tri_ancestral_verdict};
use std::fmt::Write as _;
use std::sync::OnceLock;

// ── the two faces ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq)]
enum Face {
    Classic,
    Tri,
}

/// The face is chosen by the word: any tri-only glyph selects the trilattice.
fn face_of(raw: &str) -> Face {
    if raw.chars().any(|c| matches!(c, '∈' | '∋' | '~' | '≁' | '☊' | '☋')) {
        Face::Tri
    } else {
        Face::Classic
    }
}

/// Body alphabet of a face: every glyph except the ⊢/⊣ boundary pair.
fn body_alphabet(face: Face) -> &'static [char] {
    match face {
        Face::Classic => &['>', '<', '=', '⊙', '◇', '●', '+', '×', '⊞', '¬'],
        Face::Tri => &['>', '<', '=', '⊙', '∈', '∋', '+', '×', '⊞', '~', '≁', '¬'],
    }
}

/// Rank of a verdict for ordering a walk: F < N < B < T.
fn verdict_rank(v: char) -> u8 {
    match v {
        'T' => 3,
        'B' => 2,
        'N' => 1,
        _ => 0,
    }
}

/// The search mode. `Fixed` walks inside one face; `Cross` walks the union
/// alphabet between endpoints in DIFFERENT faces. The two faces share ten
/// glyphs and differ only in the dyad (◇/● binary δ/μ against ∈/∋ ternary
/// δ₃/μ₃), so a cross-face walk is well posed: each waypoint carries one dyad
/// and is judged by that dyad's grammar, and the crossing is the substitution
/// ◇→∈ (or ●→∋) that changes the branch arity. A word carrying BOTH dyads
/// belongs to neither grammar and is refused as a waypoint.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    Fixed(Face),
    Cross,
}

/// The face a word belongs to by its own content; None when it mixes dyads.
fn word_face(w: &[char]) -> Option<Face> {
    let tri = w.iter().any(|c| matches!(c, '∈' | '∋' | '~' | '≁' | '☊' | '☋'));
    let classic = w.iter().any(|c| matches!(c, '◇' | '●'));
    match (tri, classic) {
        (true, true) => None,
        (true, false) => Some(Face::Tri),
        _ => Some(Face::Classic),
    }
}

/// The alphabet a mode may edit with: one face's, or the union across the crossing.
fn mode_alphabet(mode: Mode) -> Vec<char> {
    match mode {
        Mode::Fixed(f) => body_alphabet(f).to_vec(),
        Mode::Cross => {
            let mut v = body_alphabet(Face::Classic).to_vec();
            for &c in body_alphabet(Face::Tri) {
                if !v.contains(&c) {
                    v.push(c);
                }
            }
            v
        }
    }
}

/// Validity of a waypoint under a mode: in `Cross`, the word names its own face.
fn mode_valid(mode: Mode, w: &[char]) -> bool {
    match mode {
        Mode::Fixed(f) => word_valid(f, w),
        Mode::Cross => word_face(w).is_some_and(|f| word_valid(f, w)),
    }
}

/// Verdict of a waypoint under a mode; a dyad-mixing word is ill-typed.
fn mode_verdict(mode: Mode, w: &[char]) -> char {
    match mode {
        Mode::Fixed(f) => verdict_letter(f, w),
        Mode::Cross => word_face(w).map_or('F', |f| verdict_letter(f, w)),
    }
}

/// Parse a raw word in its face; None when nothing parses.
fn parse_word(raw: &str, face: Face) -> Option<Vec<char>> {
    let w: Vec<char> = match face {
        Face::Classic => crate::imasm::tok_list(&[raw.to_string()])
            .iter()
            .map(|t| t.code().chars().next().unwrap_or('⊙'))
            .collect(),
        Face::Tri => parse_glyph_word(raw).iter().map(|t| t.glyph()).collect(),
    };
    if w.is_empty() {
        None
    } else {
        Some(w)
    }
}

fn word_str(w: &[char]) -> String {
    w.iter().collect()
}

fn classic_tokens(w: &[char]) -> Vec<Token> {
    w.iter().filter_map(|c| Token::parse(&c.to_string())).collect()
}

/// Opcode name of one glyph in its face, for the excriber's prompt.
fn glyph_name(face: Face, c: char) -> &'static str {
    match face {
        Face::Classic => Token::parse(&c.to_string()).map(|t| t.name()).unwrap_or("?"),
        Face::Tri => parse_glyph_word(&c.to_string())
            .first()
            .map(|t| t.name())
            .unwrap_or("?"),
    }
}

/// The alphabet the model reads and writes with. Shared by both arms so the
/// excriber and the imscriber speak from the same table; only the imscriber
/// additionally carries the lessons.
fn alphabet_table(face: Face) -> &'static str {
    match face {
        Face::Classic => {
            "IMASM opcode alphabet (glyph : name : meaning : TRANSFORM? : the primitive axis it rides):\n\
             ⊢ VINIT   begin / source boundary : no  : Dimensionality (the ground where distinctions begin)\n\
             ⊣ TANCH   terminal anchor / close : no  : Topology (the connectivity boundary, the close)\n\
             > AFWD    forward morphism (real work) : YES : Relational (the forward arrow between parts)\n\
             < AREV    reverse / undoing : YES : Polarity (the parity flip, the reversal)\n\
             = CLINK   compose / link into one : YES : Fidelity (composition holding coherence)\n\
             ⊙ IMSCRIB identity / self-reference : no  : Interaction Grammar (self-imscription, the rules of combination)\n\
             ◇ FSPLIT  fork into two (δ) : no  : Granularity (dividing into finer grain)\n\
             ● FFUSE   fuse two into one (μ) : no  : Stoichiometry (the proportion of the assembly)\n\
             + EVALT   affirm the true arm : YES : Criticality (the true-gate at the tipping point)\n\
             × EVALF   refute the false arm : YES : Chirality (the handedness check)\n\
             ⊞ ENGAGR  hold both arms (paradox) : YES : Stoichiometry (both arms held in proportion)\n\
             ¬ IFIX    irreversible commit : YES : Topological Protection (the winding that cannot be undone)\n\
             (Kinetic Character, the twelfth axis, is carried by the register's motion, not one opcode.)"
        }
        Face::Tri => {
            "IMASM SIXTEEN_3 opcode alphabet (glyph : name : meaning : TRANSFORM? : the primitive axis it rides):\n\
             ⊢ VINIT   begin / source boundary : no  : Dimensionality (the ground where distinctions begin)\n\
             ⊣ TANCH   terminal anchor / close : no  : Topology (the connectivity boundary, the close)\n\
             > AFWD    forward morphism (real work) : YES : Relational (the forward arrow between parts)\n\
             < AREV    reverse / undoing : YES : Polarity (the parity flip, the reversal)\n\
             = CLINK   compose / link into one : YES : Fidelity (composition holding coherence)\n\
             ⊙ IMSCRIB identity / self-reference : no  : Interaction Grammar (self-imscription, the rules of combination)\n\
             ∈ FSPLIT3 three-way split: true/false/information : no  : Granularity (three-way division of the grain)\n\
             ∋ FFUSE3  three-way fuse: the arms rejoin : no  : Stoichiometry (the three-arm assembly)\n\
             + EVALT   affirm the true axis : YES : Criticality (the true-gate at the tipping point)\n\
             × EVALF   refute the false axis : YES : Chirality (the handedness check)\n\
             ⊞ EVALI   evaluate the information axis (t and f) : YES : Criticality (the information gate)\n\
             ~ TNEG    swaps T and F : YES : Polarity (swaps the parity poles)\n\
             ≁ INEG    swaps t and f : YES : Chirality (swaps the handedness poles)\n\
             ¬ IFIX    irreversible commit : YES : Topological Protection (the winding that cannot be undone)\n\
             (Kinetic Character, the twelfth axis, is carried by the register's motion, not one opcode.)"
        }
    }
}

/// The twelve primitive axes, tangible handles for what an IMASM program's
/// shape MEANS. IMASM names the characteristics abstractly (fork, fuse, work,
/// hold); the primitives give them grounded names an object can be judged
/// against. Canonical axis names from the Core.lean/navigator face map.
fn primitives_reference() -> &'static str {
    "The twelve primitive axes (tangible handles on what a program's shape means):\n\
     1  Dimensionality        : how many independent directions it extends in; its degrees of freedom\n\
     2  Topology              : how it is connected; its boundary, its holes, whether it closes\n\
     3  Relational Mode       : how its parts point at and depend on each other\n\
     4  Parity / Symmetry     : its symmetry, and what a reflection or reversal does to it\n\
     5  Fidelity              : how faithfully it holds coherence across change\n\
     6  Kinetic Character     : how it moves and at what rate; its dynamics\n\
     7  Scope / Granularity   : how finely it is divided; coarse grain versus fine\n\
     8  Interaction Grammar   : the rules by which its parts combine; its syntax\n\
     9  Criticality           : how close it sits to a tipping point or resonance\n\
     10 Chirality             : its handedness; whether it differs from its mirror image\n\
     11 Stoichiometry         : in what proportions its parts combine; the count of the assembly\n\
     12 Topological Protection: how robust its structure is; the winding that cannot be undone"
}

/// The primitive types, each itself an IMASM word: the strange loop where the
/// types the Grammar writes tuples with are themselves programs. Loaded once
/// from ob3ect/digital (the_primitive_type_called_*), each rendered as
/// name : glyph-word, so the agent has concrete exemplars of how the
/// characteristics compose into real programs. Cached; empty if the directory
/// is absent.
fn primitive_codes() -> &'static str {
    static CODES: OnceLock<String> = OnceLock::new();
    CODES
        .get_or_init(|| {
            let dir = crate::expand_user("~/imsgct/ob3ect/digital");
            let Ok(entries) = std::fs::read_dir(&dir) else {
                return String::new();
            };
            let mut names: Vec<String> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.file_name().to_string_lossy().into_owned())
                .filter(|n| n.starts_with("the_primitive_type_called_"))
                .collect();
            names.sort();
            let mut lines = Vec::new();
            for n in names {
                let short = n.trim_start_matches("the_primitive_type_called_").to_string();
                let path = format!("{dir}/{n}/{n}_ob3ect.json");
                let Ok(txt) = std::fs::read_to_string(&path) else { continue };
                let Ok(v) = serde_json::from_str::<serde_json::Value>(&txt) else { continue };
                let scaffold = v.get("lean_scaffold").and_then(|s| s.as_str()).unwrap_or("");
                let Some(seq) = scaffold
                    .lines()
                    .find_map(|l| l.split_once("scaffold:").map(|(_, r)| r.trim()))
                else {
                    continue;
                };
                // The scaffold line is "VINIT → IMSCRIB → …"; map names to glyphs.
                let glyphs: String = seq
                    .split(|c: char| c == '\u{2192}' || c.is_whitespace())
                    .filter(|t| !t.is_empty())
                    .filter_map(|t| Token::parse(t.trim()).map(|tk| tk.code()))
                    .collect();
                if !glyphs.is_empty() {
                    lines.push(format!("  {short}: {glyphs}"));
                }
            }
            if lines.is_empty() {
                String::new()
            } else {
                format!(
                    "The primitive types, each itself an IMASM word (its own structure):\n{}",
                    lines.join("\n")
                )
            }
        })
        .as_str()
}

// ── the verdict letter (same reading as `imasm check` / `imasm16_3 check`) ───

fn verdict_letter(face: Face, w: &[char]) -> char {
    match face {
        Face::Classic => {
            let ops = classic_tokens(w);
            if ops.is_empty() {
                return 'N';
            }
            let pairs = match_pairs(&ops);
            let g = from_sequence(&ops, &pairs);
            if !g.validate().is_empty() {
                return 'F';
            }
            let has_engagr = ops.iter().any(|&t| t == Token::Engagr);
            match g.closure_state() {
                ClosureState::Closed(_) if has_engagr => 'B',
                ClosureState::Closed(_) => 'T',
                ClosureState::Identity => 'N',
                ClosureState::Open => 'B',
                ClosureState::None => 'N',
            }
        }
        Face::Tri => tri_ancestral_verdict(&parse_glyph_word(&word_str(w))).0,
    }
}

/// Grammar admissibility of a candidate in its face (no F verdict).
/// Split a word into its ⊢…⊣ programs. A single-program word yields one
/// segment; a catalog entry's word yields twelve.
fn segments(w: &[char]) -> Vec<Vec<char>> {
    let mut out = Vec::new();
    let mut cur: Vec<char> = Vec::new();
    for &c in w {
        cur.push(c);
        if c == '⊣' {
            out.push(std::mem::take(&mut cur));
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

fn word_valid(face: Face, w: &[char]) -> bool {
    // A COMPOSITE word (an entry's twelve type-programs concatenated) is valid
    // exactly when each of its programs is. Judging it as one word asks the
    // grammar a question it should refuse: it has twelve sources and twelve
    // sinks, so every neighbour of a catalog word read this way is ill-typed
    // and the neighbourhood comes back empty.
    let segs = segments(w);
    if segs.len() > 1 {
        return segs.iter().all(|seg| word_valid_single(face, seg));
    }
    word_valid_single(face, w)
}

fn word_valid_single(face: Face, w: &[char]) -> bool {
    match face {
        Face::Classic => {
            let ops = classic_tokens(w);
            !ops.is_empty() && {
                let pairs = match_pairs(&ops);
                from_sequence(&ops, &pairs).validate().is_empty()
            }
        }
        Face::Tri => {
            !w.is_empty() && tri_ancestral_verdict(&parse_glyph_word(&word_str(w))).0 != 'F'
        }
    }
}

// ── the residual: glyph Levenshtein with an alignment backtrace ──────────────

/// Edit distance between two words, plus the aligned substitutions (sent glyph
/// → recovered glyph) along one optimal alignment. The distance is the residual
/// of μ∘δ; the substitutions are the confusions worth learning.
fn residual(a: &[char], b: &[char]) -> (usize, Vec<(char, char)>) {
    let (n, m) = (a.len(), b.len());
    let mut d = vec![vec![0usize; m + 1]; n + 1];
    for (i, row) in d.iter_mut().enumerate() {
        row[0] = i;
    }
    for j in 0..=m {
        d[0][j] = j;
    }
    for i in 1..=n {
        for j in 1..=m {
            let sub = d[i - 1][j - 1] + usize::from(a[i - 1] != b[j - 1]);
            d[i][j] = sub.min(d[i - 1][j] + 1).min(d[i][j - 1] + 1);
        }
    }
    let mut subs = Vec::new();
    let (mut i, mut j) = (n, m);
    while i > 0 && j > 0 {
        let sub = d[i - 1][j - 1] + usize::from(a[i - 1] != b[j - 1]);
        if d[i][j] == sub {
            if a[i - 1] != b[j - 1] {
                subs.push((a[i - 1], b[j - 1]));
            }
            i -= 1;
            j -= 1;
        } else if d[i][j] == d[i - 1][j] + 1 {
            i -= 1;
        } else {
            j -= 1;
        }
    }
    (d[n][m], subs)
}

// ── the neighborhood ─────────────────────────────────────────────────────────

/// All words one edit away: substitute, insert, or delete a single opcode from
/// the face's body alphabet. The ⊢…⊣ boundary pair is held fixed — the
/// neighborhood explores the body, not the frame — and only grammar-valid
/// words are admitted as candidates.
fn neighbors(face: Face, w: &[char]) -> Vec<Vec<char>> {
    neighbors_mode(Mode::Fixed(face), w)
}

fn neighbors_mode(mode: Mode, w: &[char]) -> Vec<Vec<char>> {
    let alphabet = mode_alphabet(mode);
    let mut out: Vec<Vec<char>> = Vec::new();
    let lo = usize::from(w.first() == Some(&'⊢'));
    let hi = w.len() - usize::from(w.last() == Some(&'⊣'));
    for i in lo..hi {
        // substitution
        for &c in alphabet.iter() {
            if c != w[i] {
                let mut v = w.to_vec();
                v[i] = c;
                out.push(v);
            }
        }
        // deletion (keep at least one body opcode)
        if hi - lo > 1 {
            let mut v = w.to_vec();
            v.remove(i);
            out.push(v);
        }
    }
    // insertion at every body position
    for i in lo..=hi {
        for &c in alphabet.iter() {
            let mut v = w.to_vec();
            v.insert(i, c);
            out.push(v);
        }
    }
    out.sort_by_key(|v| word_str(v));
    out.dedup_by_key(|v| word_str(v));
    out.retain(|v| mode_valid(mode, v));
    out
}

// ── the knowledge ────────────────────────────────────────────────────────────

fn knowledge_path() -> std::path::PathBuf {
    std::path::PathBuf::from(crate::expand_user("~/imsgct/MoDoT/ob3ects/imasm_knowledge.json"))
}

fn load_knowledge() -> serde_json::Value {
    match std::fs::read_to_string(knowledge_path()) {
        Ok(s) => serde_json::from_str(&s).unwrap_or_else(|_| empty_knowledge()),
        Err(_) => empty_knowledge(),
    }
}

fn empty_knowledge() -> serde_json::Value {
    serde_json::json!({ "rounds": 0, "visited": {}, "confusions": {}, "history": [] })
}

/// Persist atomically, same discipline as the tool registry: temp then rename,
/// so a reader sees the old knowledge or the new, never a torn one.
fn save_knowledge(k: &serde_json::Value) -> std::io::Result<()> {
    let path = knowledge_path();
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, serde_json::to_string_pretty(k).unwrap_or_default())?;
    std::fs::rename(&tmp, &path)
}

/// Distill the confusion counts into lessons for the imscriber. The knowledge
/// update is not a log: it is the part of the record that changes the next
/// reading, spoken into the prompt where the confusion actually happens.
fn lessons(k: &serde_json::Value) -> String {
    let Some(conf) = k.get("confusions").and_then(|c| c.as_object()) else {
        return String::new();
    };
    let mut ranked: Vec<(&String, u64)> =
        conf.iter().filter_map(|(pair, n)| n.as_u64().map(|n| (pair, n))).collect();
    ranked.sort_by(|a, b| b.1.cmp(&a.1));
    let mut s = String::new();
    for (pair, n) in ranked.iter().take(6) {
        if let Some((sent, got)) = pair.split_once('→') {
            let _ = writeln!(
                s,
                "- You have previously imscribed {sent} as {got} ({n}×). They are different \
                 opcodes; re-read the alphabet before writing either."
            );
        }
    }
    if s.is_empty() {
        s
    } else {
        format!("Lessons from your previous readings (your own recorded confusions):\n{s}")
    }
}

/// The relations the loop has learned: prior (object → word) pairs, retrieved
/// nearest to the object now being imscribed. Each visited entry records that
/// some word was excribed INTO its guess, so (guess → word) is a ground-truth
/// relation between an object and the word that produced it. Feeding the
/// nearest such relations to the imscriber is the learning channel: it
/// generalizes the object→word direction from confirmed examples, closest
/// first. Nearness is word-set overlap between the objects; ties break toward
/// the lower-residual (better-confirmed) relation.
fn nearest_relations(k: &serde_json::Value, object: &str, want: usize) -> String {
    let norm = |t: &str| -> std::collections::BTreeSet<String> {
        t.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.len() > 3)
            .map(|w| w.to_string())
            .collect()
    };
    let target = norm(object);
    if target.is_empty() {
        return String::new();
    }
    let Some(visited) = k.get("visited").and_then(|v| v.as_object()) else {
        return String::new();
    };
    // (overlap, -residual, word, guess) for every prior relation that shares a
    // word with the target and is not the target object itself.
    let mut scored: Vec<(usize, i64, String, String)> = Vec::new();
    for (word, rec) in visited {
        let guess = rec.get("guess").and_then(|g| g.as_str()).unwrap_or("");
        if guess.is_empty() || guess == object {
            continue;
        }
        let overlap = target.intersection(&norm(guess)).count();
        if overlap == 0 {
            continue;
        }
        let resid = rec.get("residual").and_then(|r| r.as_i64()).unwrap_or(99);
        scored.push((overlap, -resid, word.clone(), guess.to_string()));
    }
    if scored.is_empty() {
        return String::new();
    }
    scored.sort_by(|a, b| b.0.cmp(&a.0).then(b.1.cmp(&a.1)));
    scored.truncate(want);
    let mut s = String::from(
        "Relations you have already learned (objects like these imscribed to these words — \
         let them guide this one):\n",
    );
    for (_, _, word, guess) in &scored {
        let _ = writeln!(s, "- \"{guess}\"  ⟶  {word}");
    }
    s
}

// ── the two arms ─────────────────────────────────────────────────────────────

/// The example excriptions are spent: a small model reuses them verbatim no
/// matter what the prompt says, so the guard is mechanical, not rhetorical.
const SPENT_GUESSES: &[&str] = &[
    "a jury trial ending in a sealed verdict",
    "a molecule at a reaction fork taking the lower-barrier path to a crystallized product",
    "a bird choosing between two branches, landing on one, folding its wings",
    "a triage nurse routing patients to treat, discharge, and observe, then closing the shift",
    "white light split by a prism, one band absorbed, one reflected, recombined into a beam",
    "a three-way estate dispute settled, one claim upheld, one dismissed, the deed recorded",
];

/// Word-set overlap: a guess that is mostly the same words as a spent or taken
/// guess is a parrot, whatever its exact phrasing.
fn parroted(guess: &str, taken: &[String]) -> bool {
    let norm = |t: &str| -> std::collections::BTreeSet<String> {
        t.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.len() > 3)
            .map(|w| w.to_string())
            .collect()
    };
    let g = norm(guess);
    if g.is_empty() {
        return true;
    }
    // Two thresholds. The spent examples are radioactive: moderate overlap
    // (3/5 of the smaller word set) already reads as the example resurfacing.
    // Taken guesses only refuse a NEAR-DUPLICATE (7/10): distinct objects in
    // one domain legitimately share vocabulary ("route", "planner"), and the
    // strict threshold was refusing genuinely fresh guesses until candidates
    // starved.
    let overlap = |prior: &str, num: usize, den: usize| {
        let p = norm(prior);
        let inter = g.intersection(&p).count();
        inter * den >= g.len().min(p.len()) * num
    };
    SPENT_GUESSES.iter().any(|s| overlap(s, 3, 5))
        || taken.iter().rev().take(60).any(|t| overlap(t, 7, 10))
}

fn excribe(
    llm: &crate::Llm,
    face: Face,
    w: &[char],
    taken: &[String],
) -> Result<String, String> {
    let names: Vec<&str> = w.iter().map(|&c| glyph_name(face, c)).collect();
    // No example guesses in the prompt: a small model parrots any example as
    // its answer for every word (the court-appeal collapse, seen live). The
    // already-taken guesses ride along as FORBIDDEN instead, so distinct words
    // are forced to earn distinct names.
    let forbidden = if taken.is_empty() {
        String::new()
    } else {
        format!(
            "\n\nGuesses already taken by OTHER programs — this program is different, so its \
             object must be different. FORBIDDEN:\n{}",
            taken.iter().rev().take(12).map(|g| format!("- {g}")).collect::<Vec<_>>().join("\n")
        )
    };
    // ONE example word, THREE wildly different excriptions of it: the example
    // teaches the fan-out (one structure, any domain) instead of one answer to
    // parrot, and its three guesses are named as spent.
    let example = match face {
        Face::Classic => {
            "Example: the program VINIT FSPLIT AFWD EVALT FFUSE IFIX TANCH (begin, fork two \
             alternatives, work forward, affirm the true one, fuse back, commit, close) has been \
             excribed as all three of:\n\
             - a jury trial ending in a sealed verdict\n\
             - a molecule at a reaction fork taking the lower-barrier path to a crystallized product\n\
             - a bird choosing between two branches, landing on one, folding its wings\n\
             Three domains, one structure. Your guess must fit its OWN program the same way, in \
             yet another domain; those three are spent."
        }
        Face::Tri => {
            "Example: the program VINIT FSPLIT3 AFWD EVALT EVALF FFUSE3 IFIX TANCH (begin, split \
             three ways, work forward, affirm one arm, refute another, rejoin all three, commit, \
             close) has been excribed as all three of:\n\
             - a triage nurse routing patients to treat, discharge, and observe, then closing the shift\n\
             - white light split by a prism, one band absorbed, one reflected, recombined into a beam\n\
             - a three-way estate dispute settled, one claim upheld, one dismissed, the deed recorded\n\
             Three domains, one structure. Your guess must fit its OWN program the same way, in \
             yet another domain; those three are spent."
        }
    };
    // The primitive codes ground the reading in real exemplars; keep them out
    // of the retry prompt so the hotter attempt has more room to diverge.
    let codes = if taken.is_empty() { primitive_codes() } else { "" };
    let system = format!(
        "You are the excriber. You are given an IMASM program: an ordered sequence of opcodes. \
         GUESS the object it imscribes: identify ONE concrete thing or process in a real domain \
         whose structure matches this program step for step. Any real domain: biology, \
         chemistry, law, music, cooking, machinery, astronomy, ritual, sport. Use the primitive \
         axes below as tangible handles: read what the program's shape says about each axis \
         (how it divides, whether it closes, its handedness, its proportion) and name the real \
         object that has those same characteristics. Answer with the identification ALONE: one \
         line, a name or short noun phrase. NEVER use opcode names, glyphs, or words like \
         fork/fuse/morphism: the guess must stand entirely in its own domain.\n\n{}\n\n{}\n\n{}\n\n{example}{forbidden}",
        alphabet_table(face),
        primitives_reference(),
        codes,
    );
    // The domain is ASSIGNED, rotated by the word itself: a small model given
    // a free choice collapses onto the example, so the guess space is
    // partitioned mechanically before it answers. The retry moves domain.
    const DOMAINS: &[&str] = &[
        "organic chemistry", "ornithology", "cooking", "mountaineering", "plumbing",
        "astronomy", "beekeeping", "sailing", "blacksmithing", "immunology",
        "orchestral music", "gardening", "railway operations", "weaving", "volcanology",
        "chess", "archaeology", "fermentation", "cartography", "falconry",
    ];
    let word_hash: usize = w.iter().map(|&c| c as usize).sum::<usize>() + w.len() * 31;
    // Two attempts: the retry runs hotter and in the next domain over, and a
    // guess that parrots a spent example or an already-taken guess is refused
    // mechanically.
    for (attempt, temp) in [0.9f32, 1.3].into_iter().enumerate() {
        let domain = DOMAINS[(word_hash + attempt) % DOMAINS.len()];
        let user = format!(
            "Guess the object of this {}-step program. The object MUST come from the domain of \
             {domain}:\n{}",
            w.len(),
            names.join(" ")
        );
        let res = crate::infer(
            llm,
            &[("system".into(), system.clone()), ("user".into(), user)],
            8192,
            temp,
        );
        if let Some(e) = res.err {
            return Err(e);
        }
        // The guess is the last nonempty line, blinded: any leaked opcode
        // vocabulary is excised so the imscriber works from the domain alone.
        let guess = res
            .text
            .lines()
            .rev()
            .map(str::trim)
            .find(|l| !l.is_empty())
            .map(blind)
            .unwrap_or_default();
        if guess.split_whitespace().count() < 2 {
            continue;
        }
        if parroted(&guess, taken) {
            eprintln!("[learn]   guess parroted ({guess}) → retrying hotter…");
            continue;
        }
        if transcribes_the_word(&guess) {
            eprintln!("[learn]   guess transcribes the word ({guess}) → retrying…");
            continue;
        }
        return Ok(guess);
    }
    Err("excription parroted or empty on both attempts".into())
}

/// The anchored excription. Same alphabet, same blind, but the user turn
/// carries WHERE the word sits: between two named objects, at a stated step,
/// with the one primitive axis that just re-typed. The domain is NOT assigned
/// here — the endpoints already fix the domain, and assigning one on top would
/// fight the anchor (asking for the object between hydrogen_atom and
/// neutron_star, but in falconry).
fn excribe_anchored(
    llm: &crate::Llm,
    face: Face,
    w: &[char],
    taken: &mut Vec<String>,
    anchor: &str,
) -> Result<String, String> {
    let names: Vec<&str> = w.iter().map(|&c| glyph_name(face, c)).collect();
    let system = format!(
        "You are the excriber. You are given an IMASM program and told WHERE it sits: on a path \
         between two named objects. Name the object that occupies that position. Answer with the \
         identification ALONE: one line, a name or short noun phrase. NEVER use opcode names, \
         glyphs, or words like fork/fuse/morphism — the answer must stand entirely in its own \
         domain.\n\n{}\n\n{}",
        alphabet_table(face),
        primitives_reference(),
    );
    for (attempt, temp) in [0.7f32, 1.1].into_iter().enumerate() {
        let user = format!(
            "{anchor}\n\nThe program at this position, {} steps:\n{}{}",
            w.len(),
            names.join(" "),
            if attempt == 0 { "" } else { "\n\n(Answer again, differently; the first answer was rejected.)" },
        );
        let res = crate::infer(
            llm,
            &[("system".into(), system.clone()), ("user".into(), user)],
            8192,
            temp,
        );
        if let Some(e) = res.err {
            return Err(e);
        }
        let guess = res
            .text
            .lines()
            .rev()
            .map(str::trim)
            .find(|l| !l.is_empty())
            .map(blind)
            .unwrap_or_default();
        if guess.split_whitespace().count() < 2 {
            continue;
        }
        if parroted(&guess, taken) {
            eprintln!("[churn]   guess parroted ({guess}) → retrying…");
            continue;
        }
        if transcribes_the_word(&guess) {
            eprintln!("[churn]   guess transcribes the word ({guess}) → retrying…");
            continue;
        }
        return Ok(guess);
    }
    Err("anchored excription parroted or empty on both attempts".into())
}

/// Enforce the blind: strip every opcode name and glyph (both faces) from the
/// guess before the imscriber sees it. A model that answers "VINIT begins…"
/// has smuggled the answer into the detour; μ recovering the word from such an
/// object tests reading-back, not imscription.
fn blind(object: &str) -> String {
    const NAMES: &[&str] = &[
        "VINIT", "TANCH", "AFWD", "AREV", "CLINK", "IMSCRIB", "FSPLIT3", "FFUSE3", "FSPLIT",
        "FFUSE", "EVALT", "EVALF", "EVALI", "ENGAGR", "TNEG", "INEG", "IFIX",
    ];
    let mut s = object.to_string();
    for c in "⊢⊣><=⊙◇●∈∋+×⊞~≁¬☊☋".chars() {
        s = s.replace(c, "");
    }
    s.split_whitespace()
        .filter(|word| {
            let bare: String = word.chars().filter(|c| c.is_ascii_alphanumeric()).collect();
            !NAMES.iter().any(|n| n.eq_ignore_ascii_case(&bare))
        })
        .collect::<Vec<_>>()
        .join(" ")
}

/// A guess that TRANSCRIBES the word instead of naming an object. Stripping the
/// opcode names was never enough: the run shows guesses like "sailing route
/// planning with three wind directions affirmed, reversed, linked, and closed"
/// and "heat, affirm true state, reverse, link into one, split three ways",
/// which carry the sequence in plain English and leave nothing for the
/// imscriber to read but the answer. Such a guess makes the round trip vacuous
/// in the direction that looks like success, so it is refused at the source
/// rather than blinded: blinding it would leave a mutilated phrase, and the
/// honest move is to ask again.
fn transcribes_the_word(guess: &str) -> bool {
    // The plain-English gloss each opcode travels under, taken from the table
    // the excriber is given. A guess is a transcription when it strings several
    // of these together: one is a domain word ("closed", "linked"), three or
    // more in one line is the sequence wearing a costume.
    const GLOSS: &[&str] = &[
        "affirm", "affirmed", "affirms", "refute", "refuted", "refutes",
        "reverse", "reversed", "reverses", "undo", "undone",
        "link into one", "linked", "links", "compose",
        "fork", "forked", "forks", "split", "splits", "split three ways",
        "fuse", "fused", "fuses", "rejoin", "rejoined",
        "commit", "committed", "commits", "irreversibly",
        "true arm", "false arm", "both arms", "hold paradox",
        "begin", "close", "closed", "closes", "self-reference", "identity",
    ];
    let low = guess.to_ascii_lowercase();
    let hits = GLOSS.iter().filter(|g| low.contains(*g)).count();
    hits >= 3
}

fn imscribe(
    llm: &crate::Llm,
    face: Face,
    object: &str,
    lessons_text: &str,
    relations_text: &str,
) -> Result<Vec<char>, String> {
    let example = if face == Face::Tri { "⊢∈>+×∋¬⊣" } else { "⊢◇>+●¬⊣" };
    let system = format!(
        "You are the imscriber. You are given the name of an OBJECT: one concrete thing or \
         process from a real domain. Imscribe it: write the IMASM word whose opcode sequence IS \
         the object's structure, from its beginning boundary to its close. Answer with the glyph \
         word ONLY (e.g. {example}) — no names, no spaces, no commentary.\n\n{}\n\n{}\n\n{}",
        alphabet_table(face),
        relations_text,
        lessons_text
    );
    let res = crate::infer(
        llm,
        &[("system".into(), system), ("user".into(), object.to_string())],
        2048,
        0.1,
    );
    if let Some(e) = res.err {
        return Err(e);
    }
    // Take the last nonempty line that parses in the face — models sometimes
    // restate the object first even when told not to.
    for line in res.text.lines().rev() {
        if let Some(w) = parse_word(line.trim(), face) {
            return Ok(w);
        }
    }
    Err(format!("imscriber returned no parseable word: {}", res.text.trim()))
}

// ── the promotion path between two programs ──────────────────────────────────
//
// Two tuples that differ have a promotion path: a walk that changes one
// primitive at a time, every intermediate well-formed (`recalibrate` walks one
// axis through its values). Two IMASM programs are no different. A promotion
// path from word A to word B is a sequence of single-opcode edits — substitute,
// insert, delete one glyph — where EVERY waypoint is itself a grammar-valid
// program. The learn loop's residual is the RAW edit distance between two words;
// the promotion path is the stronger object, the shortest walk through valid
// programs only, and its verdict may climb along the way (N → B → T) exactly as
// a tuple's verdict does under promotion.

/// Describe the single edit that turns `from` into `to` (they differ by one).
// ── the churn: one word's round trip ─────────────────────────────────────────

/// A provider error trimmed to its first line. The raw bodies are multi-line
/// quota prose that buries the report it is printed inside.
fn one_line(e: &str) -> String {
    let first = e.lines().next().unwrap_or("").trim().to_string();
    if first.len() > 160 { format!("{}…", &first[..first.char_indices().nth(160).map_or(first.len(), |(i, _)| i)]) } else { first }
}

/// What one word's round trip produced.
struct Churn {
    object: String,
    recovered: String,
    dist: usize,
    verdict: char,
    rec_verdict: char,
}

/// Excribe a word into an object, imscribe the object back into a word, measure
/// the residual, and fold the result into the knowledge. This is the single
/// winding of the loop, factored out so the walk (`learn`) and the path
/// (`path … churn`) drive the SAME round trip rather than two drifting copies.
/// The word names its own face, so a churn along a face-crossing path reads each
/// waypoint in the grammar that waypoint actually belongs to.
fn churn_word(
    llm: &crate::Llm,
    knowledge: &mut serde_json::Value,
    w: &[char],
    taken: &mut Vec<String>,
) -> Result<Churn, String> {
    churn_word_anchored(llm, knowledge, w, taken, None)
}

/// The round trip, optionally ANCHORED. The anchor is what two named endpoints
/// buy: an unanchored excriber invents any object whose structure fits the word,
/// while an anchored one must name the object that sits at a stated position
/// between two given ones. The word is the same; the space of admissible answers
/// is enormously smaller, and a wrong guess is then a real error rather than a
/// different valid reading.
fn churn_word_anchored(
    llm: &crate::Llm,
    knowledge: &mut serde_json::Value,
    w: &[char],
    taken: &mut Vec<String>,
    anchor: Option<&str>,
) -> Result<Churn, String> {
    let face = word_face(w).ok_or_else(|| "word mixes both dyads; it is in neither grammar".to_string())?;
    let cw = word_str(w);
    eprintln!("[churn] {cw} → excribing…");
    let object = match anchor {
        None => excribe(llm, face, w, taken)?,
        Some(a) => excribe_anchored(llm, face, w, taken, a)?,
    };
    eprintln!("[churn]   guess: {object} → imscribing…");
    let relations_text = nearest_relations(knowledge, &object, 5);
    let lessons_text = lessons(knowledge);
    taken.push(object.clone());
    let recovered = imscribe(llm, face, &object, &lessons_text, &relations_text)?;
    let rw = word_str(&recovered);
    let (dist, subs) = residual(w, &recovered);
    eprintln!("[churn]   recovered {rw} · residual {dist}");

    let mut residuals: Vec<i64> = knowledge["visited"][&cw]["residuals"]
        .as_array()
        .map(|a| a.iter().filter_map(|v| v.as_i64()).collect())
        .unwrap_or_default();
    residuals.push(dist as i64);
    let (verdict, rec_verdict) = (verdict_letter(face, w), verdict_letter(face, &recovered));
    knowledge["visited"][&cw] = serde_json::json!({
        "guess": object, "recovered": rw, "residual": dist,
        "residuals": residuals,
        "verdict": verdict.to_string(), "recovered_verdict": rec_verdict.to_string(),
    });
    for (sent, got) in subs {
        let key = format!("{sent}→{got}");
        let n = knowledge["confusions"][&key].as_u64().unwrap_or(0);
        knowledge["confusions"][&key] = serde_json::json!(n + 1);
    }
    Ok(Churn { object, recovered: rw, dist, verdict, rec_verdict })
}

fn describe_edit(from: &[char], to: &[char]) -> String {
    use std::cmp::Ordering;
    match from.len().cmp(&to.len()) {
        Ordering::Equal => {
            for (i, (a, b)) in from.iter().zip(to).enumerate() {
                if a != b {
                    return format!("substitute {a}→{b} at {}", i + 1);
                }
            }
            "identity".into()
        }
        Ordering::Greater => {
            for i in 0..to.len() {
                if from[i] != to[i] {
                    return format!("delete {} at {}", from[i], i + 1);
                }
            }
            format!("delete {} at {}", from[from.len() - 1], from.len())
        }
        Ordering::Less => {
            for i in 0..from.len() {
                if from[i] != to[i] {
                    return format!("insert {} at {}", to[i], i + 1);
                }
            }
            format!("insert {} at {}", to[to.len() - 1], to.len())
        }
    }
}

/// A* over valid programs: nodes are grammar-valid words, moves are the
/// face's valid single-opcode edits (`neighbors`), the heuristic is the raw
/// edit distance to the target (admissible — one edit closes at most one unit
/// of distance). Returns the waypoint sequence A..=B, or None within budget.
fn promotion_path_mode(mode: Mode, a: &[char], b: &[char], budget: usize) -> Option<Vec<Vec<char>>> {
    use std::cmp::Reverse;
    use std::collections::{BinaryHeap, HashMap};
    if !mode_valid(mode, a) || !mode_valid(mode, b) {
        return None;
    }
    let start = word_str(a);
    let goal = word_str(b);
    if start == goal {
        return Some(vec![a.to_vec()]);
    }
    let mut came_from: HashMap<String, Vec<char>> = HashMap::new();
    let mut g_score: HashMap<String, usize> = HashMap::new();
    let mut open: BinaryHeap<Reverse<(usize, usize, String)>> = BinaryHeap::new();
    g_score.insert(start.clone(), 0);
    let h0 = residual(a, b).0;
    open.push(Reverse((h0, 0, start.clone())));
    let mut expansions = 0usize;
    while let Some(Reverse((_f, g, cur_s))) = open.pop() {
        if cur_s == goal {
            // reconstruct
            let mut path = vec![b.to_vec()];
            let mut key = goal.clone();
            while let Some(prev) = came_from.get(&key) {
                path.push(prev.clone());
                key = word_str(prev);
                if key == start {
                    break;
                }
            }
            path.reverse();
            return Some(path);
        }
        if g > *g_score.get(&cur_s).unwrap_or(&usize::MAX) {
            continue;
        }
        expansions += 1;
        if expansions > budget {
            return None;
        }
        let cur: Vec<char> = cur_s.chars().collect();
        for nb in neighbors_mode(mode, &cur) {
            let ns = word_str(&nb);
            let tentative = g + 1;
            if tentative < *g_score.get(&ns).unwrap_or(&usize::MAX) {
                came_from.insert(ns.clone(), cur.clone());
                g_score.insert(ns.clone(), tentative);
                let h = residual(&nb, b).0;
                open.push(Reverse((tentative + h, tentative, ns)));
            }
        }
    }
    None
}

// ── the tuple path: objects walk in TUPLE space, not glyph space ─────────────

/// The promotion path between two OBJECTS.
///
/// An entry's word is twelve type-programs concatenated, so single-glyph edits
/// between two entries are neither tractable nor meaningful: they would cut
/// across the type boundaries the word is made of. The object-level space is
/// the TUPLE — twelve primitive axes, each carrying one of the 49 types — and
/// the step is one AXIS changing type. Every waypoint is therefore a genuine
/// object position, its word is the concatenation of its own twelve types, and
/// the number of steps is exactly the number of axes on which the endpoints
/// disagree. That is what makes the interior guessable: a waypoint is not an
/// arbitrary string, it is "hydrogen_atom, but with THIS axis reading as the
/// star's type", and the catalog may already name the thing sitting there.
fn tuple_path(a: &str, b: &str, churn: bool) -> String {
    let mut out = String::new();
    let (ta, tb) = match (crate::imasm::entry_tuple(a), crate::imasm::entry_tuple(b)) {
        (Ok(x), Ok(y)) => (x, y),
        (Err(e), _) | (_, Err(e)) => return format!("imasm path: {e}\n"),
    };
    let axes = crate::imasm::TUPLE_ORDER;
    let differing: Vec<usize> = (0..12).filter(|&i| ta[i] != tb[i]).collect();
    let _ = writeln!(
        out,
        "IMASM promotion path in TUPLE space — one primitive axis re-types per step.\n\
         from '{a}' → to '{b}'  ·  {} of 12 axes differ  ·  shared: {}",
        differing.len(),
        (0..12)
            .filter(|&i| ta[i] == tb[i])
            .map(|i| format!("{}{}", axes[i], ta[i]))
            .collect::<Vec<_>>()
            .join(" "),
    );
    if differing.is_empty() {
        let _ = writeln!(out, "\nThe two entries carry the SAME tuple: they occupy one position, and there is no path to walk.");
        return out;
    }

    // Every entry in the catalog, keyed by tuple, so a waypoint that some
    // object ALREADY occupies is named rather than guessed. This is ground
    // truth the loop gets for free: where the catalog names the position, the
    // excriber's guess can be scored against a real name.
    let occupants: std::collections::HashMap<String, String> = crate::imasm::catalog_entries()
        .map(|arr| {
            arr.iter()
                .filter_map(|e| {
                    let name = e.get("name")?.as_str()?.to_string();
                    let mut key = String::new();
                    for ax in axes {
                        key.push_str(e.get(ax)?.as_str()?);
                    }
                    Some((key, name))
                })
                .collect()
        })
        .unwrap_or_default();
    let key_of = |t: &[String]| t.concat();

    // Walk the differing axes in canonical tuple order: the order the Grammar
    // itself writes a tuple in, so the walk re-types the object from its ground
    // (Dimensionality) outward rather than in an order chosen for convenience.
    let mut cur = ta.clone();
    let mut waypoints: Vec<(Vec<String>, Option<usize>)> = vec![(cur.clone(), None)];
    for &i in &differing {
        cur[i] = tb[i].clone();
        waypoints.push((cur.clone(), Some(i)));
    }

    for (step, (t, changed)) in waypoints.iter().enumerate() {
        let word = crate::imasm::tuple_glyph_word(t).unwrap_or_default();
        let w: Vec<char> = word.chars().collect();
        let v = if w.is_empty() { '?' } else { verdict_letter(Face::Classic, &w) };
        let here = occupants.get(&key_of(t)).cloned();
        let label = match changed {
            None => "(start)".to_string(),
            Some(i) => format!(
                "← {}: {} → {} ({})",
                axes[*i],
                ta[*i],
                tb[*i],
                crate::imasm::type_name_for_glyph(&tb[*i]).unwrap_or("?"),
            ),
        };
        let occupied = match &here {
            Some(n) => format!("   ⇐ the catalog already names this position: '{n}'"),
            None => String::new(),
        };
        let _ = writeln!(out, "  step {step}  [{v}]  {label}{occupied}");
    }

    if churn {
        let llm = crate::make_llm(None, None, false);
        let mut knowledge = load_knowledge();
        let mut taken: Vec<String> = Vec::new();
        let _ = writeln!(
            out,
            "\nchurning {} interior waypoint(s) — the endpoints are KNOWN, so each guess is an \
             interpolation between two named objects, not free association · provider {:?}",
            waypoints.len().saturating_sub(2),
            llm.provider,
        );
        let n = waypoints.len();
        let (mut scored, mut hit) = (0usize, 0usize);
        for (step, (t, changed)) in waypoints.iter().enumerate() {
            if step == 0 || step == n - 1 {
                continue; // the endpoints are given, not guessed
            }
            let Some(i) = changed else { continue };
            let word = match crate::imasm::tuple_glyph_word(t) {
                Ok(w) => w,
                Err(e) => {
                    let _ = writeln!(out, "  step {step}  → no word: {e}");
                    continue;
                }
            };
            let w: Vec<char> = word.chars().collect();
            let truth = occupants.get(&key_of(t)).cloned();
            let anchor = format!(
                "This word is step {step} of {} on a path from the object '{a}' to the object \
                 '{b}'. The two are the same in {} of their twelve primitive axes; this step is \
                 the one where the axis {} re-types from {} ({}) to {} ({}). Name the object that \
                 sits HERE: it must be recognisably between '{a}' and '{b}', and it must differ \
                 from '{a}' in exactly the way that axis names.",
                n - 1,
                12 - differing.len(),
                axes[*i],
                ta[*i],
                crate::imasm::type_name_for_glyph(&ta[*i]).unwrap_or("?"),
                tb[*i],
                crate::imasm::type_name_for_glyph(&tb[*i]).unwrap_or("?"),
            );
            match churn_word_anchored(&llm, &mut knowledge, &w, &mut taken, Some(&anchor)) {
                Err(e) => {
                    let _ = writeln!(out, "  step {step}  → churn failed: {}", one_line(&e));
                }
                Ok(c) => {
                    let verdict = match &truth {
                        None => String::new(),
                        Some(name) => {
                            scored += 1;
                            let g = c.object.to_ascii_lowercase();
                            let nm = name.to_ascii_lowercase();
                            let overlap = nm
                                .split('_')
                                .any(|part| part.len() > 3 && g.contains(part));
                            if overlap {
                                hit += 1;
                                format!("   ✓ the catalog names this position '{name}' — the guess touches it")
                            } else {
                                format!("   ✗ the catalog names this position '{name}'")
                            }
                        }
                    };
                    let _ = writeln!(
                        out,
                        "  step {step}  guess: {}  · residual {} on the round trip{}",
                        c.object.replace('\n', " "),
                        c.dist,
                        verdict,
                    );
                }
            }
        }
        if let Err(e) = save_knowledge(&knowledge) {
            let _ = writeln!(out, "  [knowledge did not persist: {e}]");
        }
        if scored > 0 {
            let _ = writeln!(
                out,
                "\n{hit} of {scored} guessed waypoint(s) touched the name the catalog already \
                 carries at that position. Those positions are the scored ones; the rest are \
                 unoccupied, and the guess there is a PROPOSAL for an object that would sit \
                 between the endpoints."
            );
        }
    }
    out
}

pub fn path(rest: &[String]) -> String {
    // Drop only genuine parameter tokens (`rounds=…`). A bare `=` is CLINK, a
    // real opcode: filtering on "contains '='" ate any word carrying it.
    let is_param = |s: &str| {
        ["rounds=", "breadth=", "fixed=", "budget="].iter().any(|p| s.starts_with(p))
            || s == "churn"
    };
    // `churn` after the two words runs the loop's round trip on every waypoint.
    let churn = rest.iter().any(|s| s == "churn");
    let words: Vec<&String> = rest.iter().filter(|s| !is_param(s)).collect();
    if words.len() < 2 {
        return "imasm path needs two words: imasm path '<A>' '<B>'\n\
                e.g. ./ask --imasm path '⊢◇>+●¬⊣' '⊢◇>×●¬⊣'\n\
                It finds the promotion path: the shortest walk of single-opcode edits from A \
                to B in which every intermediate is itself a valid program, the program-space \
                analogue of a tuple's promotion path.\n"
            .into();
    }
    // An endpoint may be given as a glyph word OR as a CATALOG ENTRY NAME. An
    // entry already carries its twelve type-glyphs, and each type is itself an
    // IMASM program, so the entry's word is deterministic: no model is asked
    // what an object's word is. Naming the endpoints as objects is what makes
    // the interior tractable — every waypoint sits a known number of edits from
    // two NAMED things, so excribing it is interpolation, not free association.
    let resolve = |raw: &str| -> (String, Option<String>) {
        let looks_glyphic = !raw.chars().any(|c| c.is_alphanumeric());
        if looks_glyphic {
            (raw.to_string(), None)
        } else {
            match crate::imasm::entry_glyph_word(raw) {
                Ok(w) => (w, Some(raw.to_string())),
                Err(_) => (raw.to_string(), None),
            }
        }
    };
    let (word_a, obj_a) = resolve(words[0]);
    let (word_b, obj_b) = resolve(words[1]);
    // Two OBJECTS walk in tuple space, where a step is one primitive axis
    // re-typing; only words walk in glyph space.
    if let (Some(a), Some(b)) = (&obj_a, &obj_b) {
        return tuple_path(a, b, churn);
    }
    if obj_a.is_some() != obj_b.is_some() {
        return format!(
            "imasm path: '{}' is a catalog entry and '{}' is not. Give two entries (they walk in \
             tuple space, one primitive axis per step) or two glyph words (they walk in glyph \
             space, one opcode per step). The two spaces do not mix: an entry's word is twelve \
             type-programs concatenated, and a single-glyph edit inside it cuts across the types \
             the word is made of.\n",
            words[0], words[1],
        );
    }
    let (raw_a, raw_b) = (&word_a, &word_b);
    // Each endpoint names its OWN face. Reading both in one face would silently
    // drop the other dyad (the tri parser eats ◇ and ●) and report a word nobody
    // asked about; when the faces differ, the walk CROSSES instead. The faces
    // share ten glyphs and differ only in the dyad, so the crossing is a single
    // substitution ◇→∈ / ●→∋ changing the branch arity, and every waypoint is a
    // valid program in exactly one face.
    let (face_a, face_b) = (face_of(raw_a), face_of(raw_b));
    let mode = if face_a == face_b { Mode::Fixed(face_a) } else { Mode::Cross };
    let (Some(a), Some(b)) = (parse_word(raw_a, face_a), parse_word(raw_b, face_b)) else {
        return "imasm path: one of the words did not parse in its face.\n".into();
    };
    let face_name = |f: Face| if f == Face::Tri { "SIXTEEN_3" } else { "classic" };
    let mut out = String::new();
    let _ = writeln!(
        out,
        "IMASM promotion path — single-opcode edits, every waypoint a valid program.\n\
         from {}  →  to {}  ·  face {}  ·  raw edit distance {}",
        word_str(&a),
        word_str(&b),
        if mode == Mode::Cross {
            format!("{} → {} (crossing)", face_name(face_a), face_name(face_b))
        } else {
            face_name(face_a).to_string()
        },
        residual(&a, &b).0,
    );
    match promotion_path_mode(mode, &a, &b, 200_000) {
        None => {
            let _ = writeln!(
                out,
                "\nNo promotion path found within budget (one endpoint ill-typed, or the valid \
                 region between them is larger than the search bound)."
            );
        }
        Some(p) => {
            let verdicts: String = p.iter().map(|w| mode_verdict(mode, w)).collect();
            let _ = writeln!(
                out,
                "\npromotion length {} step(s); verdict walk {}",
                p.len() - 1,
                verdicts
            );
            for (i, w) in p.iter().enumerate() {
                let v = mode_verdict(mode, w);
                if i == 0 {
                    let _ = writeln!(out, "  {}  [{v}]  (start)", word_str(w));
                } else {
                    let edit = describe_edit(&p[i - 1], w);
                    // Name the step where the grammar itself changes: the dyad
                    // swap is the only edit that moves a word between faces.
                    let crossed = word_face(&p[i - 1]) != word_face(w);
                    let mark = if crossed {
                        format!("   ⇐ FACE CROSSING → {}", face_name(word_face(w).unwrap_or(Face::Classic)))
                    } else {
                        String::new()
                    };
                    let _ = writeln!(out, "  {}  [{v}]  ← {edit}{mark}", word_str(w));
                }
            }
            // A walk that changes verdict is not automatically a promotion:
            // read the DIRECTION off the endpoints (F < N < B < T), and say
            // plainly when the walk descends or merely wanders.
            let first = verdicts.chars().next().unwrap();
            let last = verdicts.chars().last().unwrap();
            let varied = verdicts.chars().any(|c| c != first);
            let _ = match (varied, verdict_rank(last).cmp(&verdict_rank(first))) {
                (false, _) => writeln!(
                    out,
                    "\nThe path stays within one verdict class; no promotion, a lateral walk."
                ),
                (true, std::cmp::Ordering::Greater) => writeln!(
                    out,
                    "\nThe verdict climbs {first} → {last}: a genuine promotion."
                ),
                (true, std::cmp::Ordering::Less) => writeln!(
                    out,
                    "\nThe verdict DESCENDS {first} → {last}: a demotion, not a promotion. \
                     Read it backwards for the climb."
                ),
                (true, std::cmp::Ordering::Equal) => writeln!(
                    out,
                    "\nThe verdict wanders and returns to {last}: the endpoints share a class, \
                     but the interior leaves it."
                ),
            };

            // `churn`: run the loop's round trip on every waypoint. The path
            // supplies a route the walk could not find on its own — the walk
            // moves to its worst neighbor and can circle one region, while the
            // path is a straight line between two named programs, crossing
            // faces if it must. Churning it reads the residual as a PROFILE
            // along the route: where the model's reading of the grammar is
            // thin is a place on the path, not a scattered set of words.
            if churn {
                let llm = crate::make_llm(None, None, false);
                let mut knowledge = load_knowledge();
                let mut taken: Vec<String> = knowledge
                    .get("visited")
                    .and_then(|v| v.as_object())
                    .map(|m| {
                        m.values()
                            .filter_map(|r| r["guess"].as_str().map(str::to_string))
                            .collect()
                    })
                    .unwrap_or_default();
                let _ = writeln!(
                    out,
                    "\nchurning {} waypoint(s) — excribe → imscribe → residual · provider {:?}",
                    p.len(),
                    llm.provider,
                );
                let mut profile: Vec<(String, Option<usize>)> = Vec::new();
                for w in &p {
                    let cw = word_str(w);
                    match churn_word(&llm, &mut knowledge, w, &mut taken) {
                        Err(e) => {
                            let _ = writeln!(out, "  {cw}  → churn failed: {}", one_line(&e));
                            profile.push((cw, None));
                        }
                        Ok(c) => {
                            let _ = writeln!(
                                out,
                                "  {cw} [{}]  →  {} [{}]  residual {}{}\n      guess: {}",
                                c.verdict,
                                c.recovered,
                                c.rec_verdict,
                                c.dist,
                                if c.dist == 0 { " — μ∘δ = id" } else { "" },
                                c.object.replace('\n', " "),
                            );
                            profile.push((cw, Some(c.dist)));
                        }
                    }
                }
                if let Err(e) = save_knowledge(&knowledge) {
                    let _ = writeln!(out, "  [knowledge did not persist: {e}]");
                }
                let read: Vec<usize> = profile.iter().filter_map(|(_, d)| *d).collect();
                if read.is_empty() {
                    let _ = writeln!(out, "\nno waypoint completed the round trip.");
                } else {
                    let bar: String = profile
                        .iter()
                        .map(|(_, d)| match d {
                            None => '?',
                            Some(0) => '.',
                            Some(n) if *n < 10 => char::from_digit(*n as u32, 10).unwrap_or('#'),
                            Some(_) => '#',
                        })
                        .collect();
                    let mean = read.iter().sum::<usize>() as f64 / read.len() as f64;
                    let worst = profile
                        .iter()
                        .filter_map(|(w, d)| d.map(|d| (d, w)))
                        .max_by_key(|(d, _)| *d);
                    let _ = writeln!(
                        out,
                        "\nresidual profile along the path: {bar}   (. = μ∘δ = id, digit = residual, \
                         # = ten or more, ? = no round trip)\n{} of {} waypoints round-tripped · \
                         mean residual {mean:.2}",
                        read.len(),
                        p.len(),
                    );
                    if let Some((d, w)) = worst {
                        let _ = writeln!(
                            out,
                            "thinnest reading at {w} (residual {d}) — the place on this route \
                             where the model least recovers what it was given."
                        );
                    }
                }
            }
        }
    }
    out
}

// ── the loop ─────────────────────────────────────────────────────────────────

pub fn run(rest: &[String]) -> String {
    let mut rounds: usize = 3;
    let mut breadth: usize = 4;
    let mut word_args: Vec<String> = Vec::new();
    for a in rest {
        if let Some(v) = a.strip_prefix("rounds=") {
            rounds = v.parse().unwrap_or(rounds);
        } else if let Some(v) = a.strip_prefix("breadth=") {
            breadth = v.parse().unwrap_or(breadth);
        } else {
            word_args.push(a.clone());
        }
    }
    let mut raw = word_args.join(" ");
    // A catalog NAME is a legitimate seed: the entry's twelve types compose to a
    // word deterministically (`imasm words` writes the whole book). Starting
    // from a name is what makes the loop scorable — the catalog already says
    // what the object IS, so the excriber's guess has a truth to be judged
    // against instead of only a residual to be measured.
    let mut truth: Option<String> = None;
    if raw.chars().any(|c| c.is_alphanumeric()) {
        if let Some(w) = crate::imasm::word_for_entry(raw.trim()) {
            truth = Some(raw.trim().to_string());
            raw = w;
        }
    }
    let face = face_of(&raw);
    let Some(seed) = parse_word(&raw, face) else {
        return "imasm learn needs a seed word: imasm learn '<word>' [rounds=N] [breadth=K]\n\
                e.g. ./ask --imasm learn '⊢◇>+●¬⊣' rounds=3 breadth=4\n\
                Both faces parse: a word carrying ∈ ∋ ~ ≁ runs on the SIXTEEN_3 trilattice \
                grammar, any other on the classic 12-opcode grammar.\n\
                QUOTE the glyph word at a shell: > and < are redirections unquoted, and the \
                shell will eat the word's tail (and write the report to a file named +●¬⊣).\n\
                The loop excribes nearby words into guessed objects, imscribes the guesses \
                back, measures the residual, and updates ob3ects/imasm_knowledge.json.\n"
            .into();
    };

    let llm = crate::make_llm(None, None, false);
    let mut knowledge = load_knowledge();
    let mut out = String::new();
    let _ = writeln!(
        out,
        "IMASM learn — μ∘δ on the model itself: excribe → imscribe → check → residual → update.{}\n\
         seed {}  ·  face {}  ·  rounds {rounds} × breadth {breadth}  ·  provider {:?}",
        match &truth {
            Some(t) => format!("\n  seeded from the catalog entry '{t}': the object is KNOWN, so a guess can be scored and not only measured."),
            None => String::new(),
        },
        word_str(&seed),
        if face == Face::Tri { "SIXTEEN_3" } else { "classic" },
        llm.provider,
    );

    let mut center = seed;
    // Every guess already in the knowledge plus this run's own: fed back to the
    // excriber as forbidden, so the object space cannot collapse to one name.
    let mut taken_guesses: Vec<String> = knowledge
        .get("visited")
        .and_then(|v| v.as_object())
        .map(|m| {
            m.values()
                .filter_map(|r| r.get("guess").and_then(|g| g.as_str()))
                .map(|g| g.to_string())
                .collect()
        })
        .unwrap_or_default();
    let mut tested = 0usize;
    let mut perfect = 0usize;
    let mut residual_sum = 0usize;

    for round in 1..=rounds {
        // The lessons are refreshed inside each round trip, so a confusion
        // recorded on one word rides the very next word's prompt.
        let visited_words: Vec<String> = knowledge
            .get("visited")
            .and_then(|v| v.as_object())
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default();
        let mut hood = neighbors(face, &center);
        // Unvisited first; within each class keep the sorted (deterministic) order.
        hood.sort_by_key(|w| visited_words.contains(&word_str(w)));
        hood.truncate(breadth);
        if hood.is_empty() {
            let _ = writeln!(out, "round {round}: the neighborhood is empty — the walk halts.");
            break;
        }
        let _ = writeln!(out, "\nround {round} — center {}", word_str(&center));

        let mut frontier: Option<(usize, Vec<char>)> = None;
        for cand in hood {
            let cw = word_str(&cand);
            // The loop is minutes-long over a network provider; speak each arm as it
            // happens, or the operator reads a hung process where a winding is running.
            let dist = match churn_word(&llm, &mut knowledge, &cand, &mut taken_guesses) {
                Err(e) => {
                    let _ = writeln!(out, "  {cw}  → round trip failed: {}", one_line(&e));
                    continue;
                }
                Ok(c) => {
                    tested += 1;
                    residual_sum += c.dist;
                    if c.dist == 0 {
                        perfect += 1;
                    }
                    let _ = writeln!(
                        out,
                        "  {cw} [{}]  →(excribe)→(imscribe)→  {} [{}]  residual {}{}{}\n      guess: {}",
                        c.verdict,
                        c.recovered,
                        c.rec_verdict,
                        c.dist,
                        if c.dist == 0 { " — μ∘δ = id" } else { "" },
                        if c.verdict != c.rec_verdict { "  · verdict drifted" } else { "" },
                        c.object.replace('\n', " "),
                    );
                    c.dist
                }
            };
            if frontier.as_ref().map(|(d, _)| dist > *d).unwrap_or(true) {
                frontier = Some((dist, cand));
            }
        }
        knowledge["rounds"] = serde_json::json!(knowledge["rounds"].as_u64().unwrap_or(0) + 1);
        if let Err(e) = save_knowledge(&knowledge) {
            let _ = writeln!(out, "  [knowledge did not persist: {e}]");
        }
        match frontier {
            // Walk to where the reading failed hardest: the frontier of ignorance.
            Some((_, next)) => center = next,
            None => {
                let _ = writeln!(out, "  no candidate completed the loop this round.");
                break;
            }
        }
    }

    // The accuracy trajectory: each run appends its mean residual, so the
    // knowledge file carries the curve the loop is supposed to bend downward.
    let mean = if tested > 0 { residual_sum as f64 / tested as f64 } else { 0.0 };
    if tested > 0 {
        if !knowledge["history"].is_array() {
            knowledge["history"] = serde_json::json!([]);
        }
        if let Some(h) = knowledge["history"].as_array_mut() {
            h.push(serde_json::json!({
                "model": llm.model, "tested": tested, "perfect": perfect,
                "mean_residual": (mean * 100.0).round() / 100.0,
            }));
        }
        let _ = save_knowledge(&knowledge);
        let trend: Vec<String> = knowledge["history"]
            .as_array()
            .map(|h| {
                h.iter()
                    .filter_map(|r| r.get("mean_residual").and_then(|m| m.as_f64()))
                    .map(|m| format!("{m:.2}"))
                    .collect()
            })
            .unwrap_or_default();
        let _ = writeln!(out, "\naccuracy trajectory (mean residual per run): {}", trend.join(" → "));
    }

    // The learning signal, immune to territory drift: among words visited more
    // than once, compare each word's FIRST residual to its LAST. This measures
    // the model against itself across runs, so it isolates genuine improvement
    // from the walk simply landing on easier words.
    if let Some(visited) = knowledge["visited"].as_object() {
        let mut firsts = Vec::new();
        let mut lasts = Vec::new();
        let (mut better, mut worse, mut same) = (0i32, 0i32, 0i32);
        for rec in visited.values() {
            let rs: Vec<i64> = rec["residuals"]
                .as_array()
                .map(|a| a.iter().filter_map(|v| v.as_i64()).collect())
                .unwrap_or_default();
            if rs.len() < 2 {
                continue;
            }
            let (f, l) = (rs[0], *rs.last().unwrap());
            firsts.push(f);
            lasts.push(l);
            match l.cmp(&f) {
                std::cmp::Ordering::Less => better += 1,
                std::cmp::Ordering::Greater => worse += 1,
                std::cmp::Ordering::Equal => same += 1,
            }
        }
        if !firsts.is_empty() {
            let mean = |v: &[i64]| v.iter().sum::<i64>() as f64 / v.len() as f64;
            let (mf, ml) = (mean(&firsts), mean(&lasts));
            let _ = writeln!(
                out,
                "learning signal ({} word(s) seen more than once): mean residual {:.2} → {:.2} \
                 (Δ {:+.2}); {better} improved, {worse} worsened, {same} unchanged",
                firsts.len(),
                mf,
                ml,
                ml - mf,
            );
        } else {
            let _ = writeln!(
                out,
                "learning signal: no word revisited yet — run again over overlapping territory \
                 to compare a word to its earlier self."
            );
        }
    }
    let _ = writeln!(
        out,
        "{} word(s) round-tripped · {} perfect (μ∘δ = id) · mean residual {mean:.2}\n\
         knowledge → {}",
        tested,
        perfect,
        knowledge_path().display(),
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(s: &str) -> Vec<char> {
        parse_word(s, face_of(s)).unwrap()
    }

    #[test]
    fn residual_zero_iff_identical() {
        let a = w("⊢◇>+●¬⊣");
        assert_eq!(residual(&a, &a).0, 0);
        let b = w("⊢◇>×●¬⊣");
        let (d, subs) = residual(&a, &b);
        assert_eq!(d, 1);
        assert_eq!(subs, vec![('+', '×')]);
    }

    #[test]
    fn neighbors_hold_the_boundary_and_the_grammar() {
        let a = w("⊢◇>+●¬⊣");
        let hood = neighbors(Face::Classic, &a);
        assert!(!hood.is_empty());
        for n in &hood {
            assert_eq!(n.first(), Some(&'⊢'));
            assert_eq!(n.last(), Some(&'⊣'));
            assert!(word_valid(Face::Classic, n));
        }
    }

    #[test]
    fn tri_words_choose_the_trilattice_face() {
        let raw = "⊢∈>+<×∋=⊙⊞¬⊣";
        assert!(face_of(raw) == Face::Tri);
        let seed = w(raw);
        assert_eq!(seed.len(), raw.chars().count());
        assert_eq!(verdict_letter(Face::Tri, &seed), 'T');
        let hood = neighbors(Face::Tri, &seed);
        assert!(!hood.is_empty());
        for n in &hood {
            assert!(word_valid(Face::Tri, n));
        }
    }

    #[test]
    fn blinding_strips_leaked_opcode_vocabulary() {
        let leaky = "VINIT begins the ⊙ bird taking flight from EVALT branch";
        let b = blind(leaky);
        assert!(!b.contains("VINIT") && !b.contains("EVALT") && !b.contains('⊙'), "{b}");
        assert!(b.contains("bird taking flight"));
    }

    #[test]
    fn promotion_path_single_substitution() {
        let a = w("⊢◇>+●¬⊣");
        let b = w("⊢◇>×●¬⊣");
        let p = promotion_path_mode(Mode::Fixed(Face::Classic), &a, &b, 50_000).expect("path exists");
        assert_eq!(p.len(), 2, "one edit apart");
        assert_eq!(p.first(), Some(&a));
        assert_eq!(p.last(), Some(&b));
        assert!(describe_edit(&p[0], &p[1]).contains("substitute"));
    }

    #[test]
    fn promotion_path_waypoints_all_valid() {
        let a = w("⊢◇>+●¬⊣");
        let b = w("⊢◇>+⊞●¬⊣");
        let p = promotion_path_mode(Mode::Fixed(Face::Classic), &a, &b, 50_000).expect("path exists");
        for w in &p {
            assert!(word_valid(Face::Classic, w), "waypoint {} invalid", word_str(w));
        }
    }

    #[test]
    fn verdicts_match_the_check_reading() {
        assert_eq!(verdict_letter(Face::Classic, &w("⊢◇>+●⊣")), 'T');
        assert_eq!(verdict_letter(Face::Classic, &w("⊢◇⊙●⊣")), 'N');
        assert_eq!(verdict_letter(Face::Classic, &w("⊢◇⊞>●⊣")), 'B');
    }
}
