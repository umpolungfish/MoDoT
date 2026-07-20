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
            "IMASM opcode alphabet (glyph · name · meaning · does it TRANSFORM?):\n\
             ⊢ VINIT   begin / source boundary — no\n\
             ⊣ TANCH   terminal anchor / close boundary — no\n\
             > AFWD    forward morphism (a step of real forward work) — YES\n\
             < AREV    reverse morphism (an inversion / undoing) — YES\n\
             = CLINK   compose / link two things into one — YES\n\
             ⊙ IMSCRIB identity / self-reference (no work) — no\n\
             ◇ FSPLIT  fork into two alternatives (δ) — no\n\
             ● FFUSE   fuse two arms back into one (μ) — no\n\
             + EVALT   evaluate / affirm the true arm — YES\n\
             × EVALF   evaluate / refute the false arm — YES\n\
             ⊞ ENGAGR  hold both arms at once (a lived paradox) — YES\n\
             ¬ IFIX    irreversible commit / fixation — YES"
        }
        Face::Tri => {
            "IMASM SIXTEEN_3 opcode alphabet (glyph · name · meaning · does it TRANSFORM?):\n\
             ⊢ VINIT   begin / source boundary — no\n\
             ⊣ TANCH   terminal anchor / close boundary — no\n\
             > AFWD    forward morphism (a step of real forward work) — YES\n\
             < AREV    reverse morphism (an inversion / undoing) — YES\n\
             = CLINK   compose / link two things into one — YES\n\
             ⊙ IMSCRIB identity / self-reference (no work) — no\n\
             ∈ FSPLIT3 three-way split: true / false / information arms — no\n\
             ∋ FFUSE3  three-way fuse: the three arms rejoin — no\n\
             + EVALT   evaluate / affirm the true axis — YES\n\
             × EVALF   evaluate / refute the false axis — YES\n\
             ⊞ EVALI   evaluate the information axis (both t and f) — YES\n\
             ~ TNEG    negation: swaps T and F — YES\n\
             ≁ INEG    con-negation: swaps t and f — YES\n\
             ¬ IFIX    irreversible commit / fixation — YES"
        }
    }
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
fn word_valid(face: Face, w: &[char]) -> bool {
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
    let mut out: Vec<Vec<char>> = Vec::new();
    let lo = usize::from(w.first() == Some(&'⊢'));
    let hi = w.len() - usize::from(w.last() == Some(&'⊣'));
    for i in lo..hi {
        // substitution
        for &c in body_alphabet(face) {
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
        for &c in body_alphabet(face) {
            let mut v = w.to_vec();
            v.insert(i, c);
            out.push(v);
        }
    }
    out.sort_by_key(|v| word_str(v));
    out.dedup_by_key(|v| word_str(v));
    out.retain(|v| word_valid(face, v));
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
    SPENT_GUESSES
        .iter()
        .map(|s| s.to_string())
        .chain(taken.iter().cloned())
        .any(|prior| {
            let p = norm(&prior);
            let inter = g.intersection(&p).count();
            inter * 5 >= g.len().min(p.len()) * 3
        })
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
    let system = format!(
        "You are the excriber. You are given an IMASM program: an ordered sequence of opcodes. \
         GUESS the object it imscribes: identify ONE concrete thing or process in a real domain \
         whose structure matches this program step for step. Any real domain: biology, \
         chemistry, law, music, cooking, machinery, astronomy, ritual, sport. Answer with the \
         identification ALONE: one line, a name or short noun phrase. NEVER use opcode names, \
         glyphs, or words like fork/fuse/morphism — the guess must stand entirely in its own \
         domain.\n\n{}\n\n{example}{forbidden}",
        alphabet_table(face)
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
        return Ok(guess);
    }
    Err("excription parroted or empty on both attempts".into())
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

fn imscribe(
    llm: &crate::Llm,
    face: Face,
    object: &str,
    lessons_text: &str,
) -> Result<Vec<char>, String> {
    let example = if face == Face::Tri { "⊢∈>+×∋¬⊣" } else { "⊢◇>+●¬⊣" };
    let system = format!(
        "You are the imscriber. You are given the name of an OBJECT: one concrete thing or \
         process from a real domain. Imscribe it: write the IMASM word whose opcode sequence IS \
         the object's structure, from its beginning boundary to its close. Answer with the glyph \
         word ONLY (e.g. {example}) — no names, no spaces, no commentary.\n\n{}\n\n{}",
        alphabet_table(face),
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
    let raw = word_args.join(" ");
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
        "IMASM learn — μ∘δ on the model itself: excribe → imscribe → check → residual → update.\n\
         seed {}  ·  face {}  ·  rounds {rounds} × breadth {breadth}  ·  provider {:?}",
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
        let lessons_text = lessons(&knowledge);
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
            let cand_v = verdict_letter(face, &cand);
            // The loop is minutes-long over a network provider; speak each arm as it
            // happens, or the operator reads a hung process where a winding is running.
            eprintln!("[learn] round {round} · {cw} → excribing…");
            let (rec_res, object) = match excribe(&llm, face, &cand, &taken_guesses) {
                Err(e) => {
                    let _ = writeln!(out, "  {cw}  → excribe failed: {e}");
                    continue;
                }
                Ok(object) => {
                    eprintln!("[learn]   guess: {object} → imscribing…");
                    taken_guesses.push(object.clone());
                    (imscribe(&llm, face, &object, &lessons_text), object)
                }
            };
            let recovered = match rec_res {
                Err(e) => {
                    let _ = writeln!(out, "  {cw}  → imscribe failed: {e}");
                    continue;
                }
                Ok(r) => r,
            };
            let rw = word_str(&recovered);
            let rec_v = verdict_letter(face, &recovered);
            let (dist, subs) = residual(&cand, &recovered);
            eprintln!("[learn]   recovered {rw} · residual {dist}");
            tested += 1;
            residual_sum += dist;
            if dist == 0 {
                perfect += 1;
            }
            let _ = writeln!(
                out,
                "  {cw} [{cand_v}]  →(excribe)→(imscribe)→  {rw} [{rec_v}]  \
                 residual {dist}{}{}\n      guess: {}",
                if dist == 0 { " — μ∘δ = id" } else { "" },
                if cand_v != rec_v { "  · verdict drifted" } else { "" },
                object.replace('\n', " "),
            );

            // The guess IS the record's centre: the word went out as this
            // object and came home as the recovered word. Keep all three.
            knowledge["visited"][&cw] = serde_json::json!({
                "guess": object, "recovered": rw, "residual": dist,
                "verdict": cand_v.to_string(), "recovered_verdict": rec_v.to_string(),
            });
            for (sent, got) in subs {
                let key = format!("{sent}→{got}");
                let n = knowledge["confusions"][&key].as_u64().unwrap_or(0);
                knowledge["confusions"][&key] = serde_json::json!(n + 1);
            }
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
    fn verdicts_match_the_check_reading() {
        assert_eq!(verdict_letter(Face::Classic, &w("⊢◇>+●⊣")), 'T');
        assert_eq!(verdict_letter(Face::Classic, &w("⊢◇⊙●⊣")), 'N');
        assert_eq!(verdict_letter(Face::Classic, &w("⊢◇⊞>●⊣")), 'B');
    }
}
