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
        Face::Classic => &['≻', '≺', '=', '⊙', '◇', '●', '+', '×', '⊞', '¬'],
        Face::Tri => &['≻', '≺', '=', '⊙', '∈', '∋', '+', '×', '⊞', '~', '≁', '¬'],
    }
}

/// The verdict of a word that may be SEVERAL programs. A composite word has one
/// source and one sink per program, so judging it as a single word answers F to
/// every catalog entry ever written. Judge each program and speak the weakest
/// verdict its programs reach, which is the honest reading of a conjunction.
fn composite_verdict(face: Face, w: &[char]) -> char {
    let segs = segments(w);
    if segs.len() <= 1 {
        return verdict_letter(face, w);
    }
    segs.iter()
        .map(|seg| verdict_letter(face, seg))
        .min_by_key(|v| verdict_rank(*v))
        .unwrap_or('N')
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
             ⊤ EVALT   affirm the true axis : YES : Criticality (the true-gate at the tipping point)\n\
             ⊥ EVALF   refute the false axis : YES : Chirality (the handedness check)\n\
             ⊞ EVALI   evaluate the information axis (t and f) : YES : Criticality (the information gate)\n\
             ⊡ IFIX    irreversible commit : YES : Topological Protection (the winding that cannot be undone)\n\
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
         fork/fuse/morphism: the guess must stand entirely in its own domain.\n\n\
         AND NEVER answer with the Grammar's own vocabulary. The name of a TYPE (monad, ear, \
         egg, ice, out, vow, …), the name of a PRIMITIVE AXIS (topology, chirality, \
         criticality, …) and the name of an OPCODE (CLINK, IMSCRIB, …) are all refused: the \
         word was WRITTEN from those, so handing one back says only that the program is the \
         program. Name a thing in the world that has this shape.\n\n{}\n\n{}\n\n{}\n\n{example}{forbidden}",
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
        if name_tokens(&guess) == 0 {
            continue;
        }
        if parroted(&guess, taken) {
            eprintln!("[learn]   guess parroted ({guess}) → retrying hotter…");
            continue;
        }
        if names_the_grammar(&guess) {
            eprintln!("[learn]   guess names the grammar, not an object ({guess}) → retrying…");
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
/// The twelve axes, named, in canonical tuple order.
const AXIS_NAMES: [(&str, &str); 12] = [
    ("⊢", "Dimensionality"), ("⊣", "Topology"), ("≻", "Relational Mode"),
