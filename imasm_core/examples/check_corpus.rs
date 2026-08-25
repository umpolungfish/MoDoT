//! Read one glyph word per line on stdin (ASCII > < accepted as AFWD/AREV,
//! matching daydr33m/m2_torus/ob3ect_words.txt's actual spelling), print
//! `word<TAB>verdict` from the real classic-mark kernel (check::word_verdict).
use std::io::{self, BufRead};

use imasm_core::check::word_verdict;
use imasm_core::classic::Token;

fn main() {
    for line in io::stdin().lock().lines() {
        let w = line.unwrap();
        let w = w.trim();
        if w.is_empty() {
            continue;
        }
        let toks: Vec<Token> = w
            .chars()
            .filter_map(|c| match c {
                '>' => Some('≻'),
                '<' => Some('≺'),
                other => Some(other),
            })
            .filter_map(|c| Token::parse(&c.to_string()))
            .collect();
        let (v, _state) = word_verdict(&toks);
        println!("{w}\t{v}");
    }
}
