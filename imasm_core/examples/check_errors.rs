//! Read one glyph word per line on stdin, print the word, check::word_verdict's
//! letter and ClosureState, and every Graph::validate() error found along the
//! way -- nothing else in this repo surfaces those errors by themselves,
//! word_verdict only returns the letter.
use std::io::{self, BufRead};

use imasm_core::check::{from_sequence, match_pairs, word_verdict};
use imasm_core::classic::Token;

fn main() {
    for line in io::stdin().lock().lines() {
        let w = line.unwrap();
        let w = w.trim();
        if w.is_empty() {
            continue;
        }
        let tokens: Option<Vec<Token>> = w
            .chars()
            .map(|c| {
                let mut buf = [0u8; 4];
                Token::parse(c.encode_utf8(&mut buf))
            })
            .collect();
        let tokens = match tokens {
            Some(t) => t,
            None => {
                println!("{w}\tPARSE FAILED");
                continue;
            }
        };
        let (letter, state) = word_verdict(&tokens);
        let pairs = match_pairs(&tokens);
        let g = from_sequence(&tokens, &pairs);
        let errs = g.validate();
        println!("{w}\t{letter}\t{state:?}\tpairs={pairs:?}");
        for e in &errs {
            println!("  {e}");
        }
    }
}
