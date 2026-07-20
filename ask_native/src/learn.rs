//! `imasm learn` — the excription/imscription learning loop.
//!
//! One model takes a word, iterates nearby words, excribes each into an object
//! (a standalone domain rendering that never names an opcode), imscribes the
//! OBJECT back into a word, checks the recovered word, measures the residual
//! between the two words, and updates its knowledge. This is verification as
//! imscription run on the model itself: δ excribes the word into a genuinely
//! different form, μ recovers it, and μ∘δ = id exactly when the residual is
//! zero. The residual is measured on every reading; where it is not zero, the
//! confusion is recorded, distilled into a lesson, and the lesson rides the
//! imscriber's next prompt. The walk moves to the highest-residual neighbor,
//! so the loop spends its rounds where its knowledge is thinnest.

use crate::imasm::{from_sequence, match_pairs, ClosureState, Token};
use std::fmt::Write as _;

const ALL_TOKENS: [Token; 12] = [
    Token::Vinit,
    Token::Tanch,
    Token::Afwd,
    Token::Arev,
    Token::Clink,
    Token::Imscrib,
    Token::Fsplit,
    Token::Ffuse,
    Token::Evalt,
    Token::Evalf,
    Token::Engagr,
    Token::Ifix,
];

fn word_str(ops: &[Token]) -> String {
    ops.iter().map(|t| t.code()).collect()
}

/// The alphabet the model reads and writes with. Shared by both arms so the
/// excriber and the imscriber speak from the same table; only the imscriber
/// additionally carries the lessons.
fn alphabet_table() -> &'static str {
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

// ── the verdict letter (same reading as `imasm check`) ───────────────────────

fn verdict_letter(ops: &[Token]) -> char {
    if ops.is_empty() {
        return 'N';
    }
    let pairs = match_pairs(ops);
    let g = from_sequence(ops, &pairs);
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

// ── the residual: token Levenshtein with an alignment backtrace ──────────────

/// Edit distance between two opcode words, plus the aligned substitutions
/// (sent token → recovered token) along one optimal alignment. The distance is
/// the residual of μ∘δ; the substitutions are the confusions worth learning.
fn residual(a: &[Token], b: &[Token]) -> (usize, Vec<(Token, Token)>) {
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

/// All words one edit away: substitute, insert, or delete a single opcode.
/// The ⊢…⊣ boundary pair is held fixed — the neighborhood explores the body,
/// not the frame — and only grammar-valid words (no F under `validate`) are
/// admitted as candidates.
fn neighbors(ops: &[Token]) -> Vec<Vec<Token>> {
    let mut out: Vec<Vec<Token>> = Vec::new();
    let lo = usize::from(ops.first() == Some(&Token::Vinit));
    let hi = ops.len() - usize::from(ops.last() == Some(&Token::Tanch));
    for i in lo..hi {
        // substitution
        for &t in &ALL_TOKENS {
            if t != ops[i] && t != Token::Vinit && t != Token::Tanch {
                let mut w = ops.to_vec();
                w[i] = t;
                out.push(w);
            }
        }
        // deletion (keep at least one body opcode)
        if hi - lo > 1 {
            let mut w = ops.to_vec();
            w.remove(i);
            out.push(w);
        }
    }
    // insertion at every body position
    for i in lo..=hi {
        for &t in &ALL_TOKENS {
            if t != Token::Vinit && t != Token::Tanch {
                let mut w = ops.to_vec();
                w.insert(i, t);
                out.push(w);
            }
        }
    }
    out.sort_by_key(|w| word_str(w));
    out.dedup_by_key(|w| word_str(w));
    out.retain(|w| {
        let pairs = match_pairs(w);
        from_sequence(w, &pairs).validate().is_empty()
    });
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
    serde_json::json!({ "rounds": 0, "visited": {}, "confusions": {} })
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

fn excribe(llm: &crate::Llm, ops: &[Token]) -> Result<String, String> {
    let names: Vec<&str> = ops.iter().map(|t| t.name()).collect();
    let system = format!(
        "You are the excriber. You take an IMASM program and excribe it into an OBJECT: a \
         concrete process in one real domain of your choosing (a synthesis, a court case, a \
         repair, a metabolic pathway, anything real), written as a numbered list of steps, one \
         step per opcode, in order. The object must stand entirely alone: NEVER mention IMASM, \
         opcode names, glyphs, forks/fuses by those words, or this instruction. Each step is one \
         plain sentence enacting that opcode's meaning in the domain. Number every step. No \
         preamble, no commentary after.\n\n{}",
        alphabet_table()
    );
    let user = format!("Excribe this {}-step program:\n{}", ops.len(), names.join(" "));
    let res = crate::infer(
        llm,
        &[("system".into(), system), ("user".into(), user)],
        8192,
        0.9,
    );
    match res.err {
        Some(e) => Err(e),
        None => Ok(res.text),
    }
}

fn imscribe(llm: &crate::Llm, object: &str, lessons_text: &str) -> Result<Vec<Token>, String> {
    let system = format!(
        "You are the imscriber. You are given only an OBJECT: a numbered list of steps from some \
         domain. Recover the IMASM word that imscribes it: exactly one opcode per numbered step, \
         in order. Answer with the glyph word ONLY (e.g. ⊢◇>+●¬⊣) — no names, no spaces, no \
         commentary.\n\n{}\n\n{}",
        alphabet_table(),
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
    // Take the last nonempty line that parses to opcodes — models sometimes
    // restate the object first even when told not to.
    let mut best: Vec<Token> = Vec::new();
    for line in res.text.lines().rev() {
        let toks = crate::imasm::tok_list(&[line.trim().to_string()]);
        if !toks.is_empty() {
            best = toks;
            break;
        }
    }
    if best.is_empty() {
        Err(format!("imscriber returned no parseable word: {}", res.text.trim()))
    } else {
        Ok(best)
    }
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
    let seed = crate::imasm::tok_list(&word_args);
    if seed.is_empty() {
        return "imasm learn needs a seed word: imasm learn <word> [rounds=N] [breadth=K]\n\
                e.g. imasm learn ⊢◇>+●¬⊣ rounds=3 breadth=4\n\
                The loop excribes nearby words into objects, imscribes the objects back, \
                measures the residual, and updates ob3ects/imasm_knowledge.json.\n"
            .into();
    }

    let llm = crate::make_llm(None, None, false);
    let mut knowledge = load_knowledge();
    let mut out = String::new();
    let _ = writeln!(
        out,
        "IMASM learn — μ∘δ on the model itself: excribe → imscribe → check → residual → update.\n\
         seed {}  ·  rounds {rounds} × breadth {breadth}  ·  provider {}",
        word_str(&seed),
        llm.model,
    );

    let mut center = seed;
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
        let mut hood = neighbors(&center);
        // Unvisited first; within each class keep the sorted (deterministic) order.
        hood.sort_by_key(|w| visited_words.contains(&word_str(w)));
        hood.truncate(breadth);
        if hood.is_empty() {
            let _ = writeln!(out, "round {round}: the neighborhood is empty — the walk halts.");
            break;
        }
        let _ = writeln!(out, "\nround {round} — center {}", word_str(&center));

        let mut frontier: Option<(usize, Vec<Token>)> = None;
        for cand in hood {
            let cw = word_str(&cand);
            let cand_v = verdict_letter(&cand);
            let (rec_res, obj_len) = match excribe(&llm, &cand) {
                Err(e) => {
                    let _ = writeln!(out, "  {cw}  → excribe failed: {e}");
                    continue;
                }
                Ok(object) => (imscribe(&llm, &object, &lessons_text), object.len()),
            };
            let recovered = match rec_res {
                Err(e) => {
                    let _ = writeln!(out, "  {cw}  → imscribe failed: {e}");
                    continue;
                }
                Ok(r) => r,
            };
            let rw = word_str(&recovered);
            let rec_v = verdict_letter(&recovered);
            let (dist, subs) = residual(&cand, &recovered);
            tested += 1;
            residual_sum += dist;
            if dist == 0 {
                perfect += 1;
            }
            let _ = writeln!(
                out,
                "  {cw} [{cand_v}]  →(excribe {obj_len}ch)→(imscribe)→  {rw} [{rec_v}]  \
                 residual {dist}{}{}",
                if dist == 0 { " — μ∘δ = id" } else { "" },
                if cand_v != rec_v { "  · verdict drifted" } else { "" },
            );

            // knowledge update
            knowledge["visited"][&cw] = serde_json::json!({
                "recovered": rw, "residual": dist,
                "verdict": cand_v.to_string(), "recovered_verdict": rec_v.to_string(),
            });
            for (sent, got) in subs {
                let key = format!("{}→{}", sent.code(), got.code());
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

    let _ = writeln!(
        out,
        "\n{} word(s) round-tripped · {} perfect (μ∘δ = id) · mean residual {:.2}\n\
         knowledge → {}",
        tested,
        perfect,
        if tested > 0 { residual_sum as f64 / tested as f64 } else { 0.0 },
        knowledge_path().display(),
    );
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn w(s: &str) -> Vec<Token> {
        crate::imasm::tok_list(&[s.to_string()])
    }

    #[test]
    fn residual_zero_iff_identical() {
        let a = w("⊢◇>+●¬⊣");
        assert_eq!(residual(&a, &a).0, 0);
        let b = w("⊢◇>×●¬⊣");
        let (d, subs) = residual(&a, &b);
        assert_eq!(d, 1);
        assert_eq!(subs, vec![(Token::Evalt, Token::Evalf)]);
    }

    #[test]
    fn neighbors_hold_the_boundary_and_the_grammar() {
        let a = w("⊢◇>+●¬⊣");
        let hood = neighbors(&a);
        assert!(!hood.is_empty());
        for n in &hood {
            assert_eq!(n.first(), Some(&Token::Vinit));
            assert_eq!(n.last(), Some(&Token::Tanch));
            let pairs = match_pairs(n);
            assert!(from_sequence(n, &pairs).validate().is_empty());
        }
    }

    #[test]
    fn verdicts_match_the_check_reading() {
        assert_eq!(verdict_letter(&w("⊢◇>+●⊣")), 'T');
        assert_eq!(verdict_letter(&w("⊢◇⊙●⊣")), 'N');
        assert_eq!(verdict_letter(&w("⊢◇⊞>●⊣")), 'B');
    }
}
