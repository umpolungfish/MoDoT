//! Terminal styling for the host tools.
//!
//! Two rules, and the second is the one that matters more than the colours.
//!
//! 1. Colour is decided ONCE, from whether stdout is a terminal, with `NO_COLOR`
//!    honoured and `CLICOLOR_FORCE` able to override in the other direction.
//! 2. When it is off, every helper returns the bare text. Not a dimmer escape —
//!    nothing. `ask` output is read by pipes, by `tee`, and by the agent itself
//!    reading a tool's result, and an escape sequence in any of those is noise
//!    that a later reader has to strip. Styling exists for the human at the
//!    terminal and must vanish for everyone else.

use std::io::IsTerminal;
use std::sync::OnceLock;

fn on() -> bool {
    static ON: OnceLock<bool> = OnceLock::new();
    *ON.get_or_init(|| {
        if std::env::var_os("CLICOLOR_FORCE").is_some_and(|v| v != "0") {
            return true;
        }
        if std::env::var_os("NO_COLOR").is_some() {
            return false;
        }
        std::io::stdout().is_terminal()
    })
}

fn wrap(code: &str, s: &str) -> String {
    if on() { format!("\x1b[{code}m{s}\x1b[0m") } else { s.to_string() }
}

pub fn bold(s: &str) -> String { wrap("1", s) }
pub fn dim(s: &str) -> String { wrap("2", s) }
pub fn cyan(s: &str) -> String { wrap("36", s) }
pub fn green(s: &str) -> String { wrap("32", s) }
pub fn yellow(s: &str) -> String { wrap("33", s) }
pub fn red(s: &str) -> String { wrap("31", s) }
pub fn magenta(s: &str) -> String { wrap("35", s) }
/// One escape, not two. `bold(&cyan(x))` nests and emits a double reset, which
/// is harmless to a terminal and ugly in a captured log.
pub fn title(s: &str) -> String { wrap("1;36", s) }

/// Whether STDERR should carry colour — a different question from stdout, since
/// diagnostics and results are redirected separately.
pub fn err_on() -> bool {
    use std::io::IsTerminal;
    if std::env::var_os("CLICOLOR_FORCE").is_some_and(|v| v != "0") { return true; }
    if std::env::var_os("NO_COLOR").is_some() { return false; }
    std::io::stderr().is_terminal()
}

/// Dim, for stderr diagnostics. Returns bare text when stderr is not a terminal,
/// which is what `2>&1 | tee` and every captured run want.
pub fn dim_err(s: &str) -> String {
    if err_on() { format!("\x1b[2m{s}\x1b[0m") } else { s.to_string() }
}

/// A verb's banner: a rule, the name, and what it was given. This is the
/// separation the output was missing — several tools print thirty lines with no
/// mark of where one ends and the next begins, which is unreadable the moment two
/// are run in sequence.
pub fn header(verb: &str, subject: &str) -> String {
    let bar = "─".repeat(72);
    if subject.is_empty() {
        format!("\n{}\n{}\n{}", dim(&bar), title(verb), dim(&bar))
    } else {
        format!(
            "\n{}\n{}  {}\n{}",
            dim(&bar),
            title(verb),
            dim(subject),
            dim(&bar)
        )
    }
}

/// A section break inside one verb's output.
pub fn rule(label: &str) -> String {
    if label.is_empty() {
        dim(&"─".repeat(72))
    } else {
        let pad = 72usize.saturating_sub(label.chars().count() + 4);
        dim(&format!("── {} {}", label, "─".repeat(pad)))
    }
}

/// Paint the words that carry a verdict, so the eye finds the answer without
/// reading the paragraph. Anything not named here is left exactly as it was.
pub fn verdict(s: &str) -> String {
    match s.trim() {
        "T" | "CYCLIC" | "SETTLED" | "CONDUCTIVE" | "PASS" | "full" | "✓" => green(s),
        "B" | "BALANCED" | "FRUSTRATED" | "SHAKY" | "◐" | "⚠" => yellow(s),
        "F" | "FAIL" | "INSULATING" | "RETAINED" | "✗" => red(s),
        "N" | "VOID" | "linear" | "—" => dim(s),
        _ => s.to_string(),
    }
}
