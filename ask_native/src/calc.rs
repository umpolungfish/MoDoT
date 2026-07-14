//! Exact numeric evaluation — the calculator lane, native.
//!
//! Arithmetic done in the head is the single cheapest way for a synthesis to go wrong
//! while sounding right: a slipped exponent reads exactly like a correct one. Two live
//! failures motivated this module, both of which a calculator kills instantly:
//!
//!   0.0796 * 7.88e-10   claimed 6.27e-10, actually 6.27e-11   (one decade, invisible)
//!   -(3/2 + 1/2)        claimed -3/2,     actually -2          (sign/precedence slip)
//!
//! So every number the agent SPEAKS should come through here. Pure Rust: tokenizer,
//! recursive-descent parser, f64 evaluator. No shell, no python.
//!
//! Grammar:
//!   expr   := term (('+' | '-') term)*
//!   term   := unary (('*' | '/' | '%') unary)*
//!   unary  := ('-' | '+') unary | power
//!   power  := atom (('^' | '**') unary)?          right-assoc; ^ binds tighter than unary -
//!   atom   := number | ident '(' args ')' | ident | '(' expr ')'

#[derive(Clone, Debug, PartialEq)]
enum Tok {
    Num(f64),
    Ident(String),
    Op(char),
    Pow, // ^ or **
    LP,
    RP,
    Comma,
}

fn lex(s: &str) -> Result<Vec<Tok>, String> {
    let mut out = Vec::new();
    let cs: Vec<char> = s.chars().collect();
    let mut i = 0;
    while i < cs.len() {
        let c = cs[i];
        if c.is_whitespace() || c == '_' || c == ',' && false {
            i += 1;
            continue;
        }
        if c == ',' {
            out.push(Tok::Comma);
            i += 1;
            continue;
        }
        if c.is_ascii_digit() || c == '.' {
            let start = i;
            while i < cs.len() && (cs[i].is_ascii_digit() || cs[i] == '.') {
                i += 1;
            }
            // exponent: 1e-10, 2.5E+3
            if i < cs.len() && (cs[i] == 'e' || cs[i] == 'E') {
                let save = i;
                let mut j = i + 1;
                if j < cs.len() && (cs[j] == '+' || cs[j] == '-') {
                    j += 1;
                }
                if j < cs.len() && cs[j].is_ascii_digit() {
                    while j < cs.len() && cs[j].is_ascii_digit() {
                        j += 1;
                    }
                    i = j;
                } else {
                    i = save;
                }
            }
            let t: String = cs[start..i].iter().collect();
            out.push(Tok::Num(t.parse::<f64>().map_err(|_| format!("bad number: {t}"))?));
            continue;
        }
        if c.is_alphabetic() {
            let start = i;
            while i < cs.len() && (cs[i].is_alphanumeric() || cs[i] == '_') {
                i += 1;
            }
            out.push(Tok::Ident(cs[start..i].iter().collect::<String>().to_lowercase()));
            continue;
        }
        match c {
            '(' => {
                out.push(Tok::LP);
                i += 1;
            }
            ')' => {
                out.push(Tok::RP);
                i += 1;
            }
            '*' => {
                if i + 1 < cs.len() && cs[i + 1] == '*' {
                    out.push(Tok::Pow);
                    i += 2;
                } else {
                    out.push(Tok::Op('*'));
                    i += 1;
                }
            }
            '^' => {
                out.push(Tok::Pow);
                i += 1;
            }
            '+' | '-' | '/' | '%' => {
                out.push(Tok::Op(c));
                i += 1;
            }
            // common unicode the model will emit
            '×' => {
                out.push(Tok::Op('*'));
                i += 1;
            }
            '÷' => {
                out.push(Tok::Op('/'));
                i += 1;
            }
            '−' => {
                out.push(Tok::Op('-'));
                i += 1;
            }
            'π' => {
                out.push(Tok::Ident("pi".into()));
                i += 1;
            }
            '√' => {
                out.push(Tok::Ident("sqrt".into()));
                i += 1;
            }
            _ => return Err(format!("unexpected character: {c}")),
        }
    }
    Ok(out)
}

struct P {
    t: Vec<Tok>,
    i: usize,
}

impl P {
    fn peek(&self) -> Option<&Tok> {
        self.t.get(self.i)
    }
    fn next(&mut self) -> Option<Tok> {
        let x = self.t.get(self.i).cloned();
        self.i += 1;
        x
    }

    fn expr(&mut self) -> Result<f64, String> {
        let mut v = self.term()?;
        while let Some(Tok::Op(c)) = self.peek() {
            let c = *c;
            if c != '+' && c != '-' {
                break;
            }
            self.i += 1;
            let r = self.term()?;
            v = if c == '+' { v + r } else { v - r };
        }
        Ok(v)
    }

    fn term(&mut self) -> Result<f64, String> {
        let mut v = self.unary()?;
        while let Some(Tok::Op(c)) = self.peek() {
            let c = *c;
            if c != '*' && c != '/' && c != '%' {
                break;
            }
            self.i += 1;
            let r = self.unary()?;
            match c {
                '*' => v *= r,
                '/' => {
                    if r == 0.0 {
                        return Err("division by zero".into());
                    }
                    v /= r;
                }
                _ => {
                    if r == 0.0 {
                        return Err("modulo by zero".into());
                    }
                    v %= r;
                }
            }
        }
        Ok(v)
    }

    /// `-` binds LOOSER than `^`, per convention: -2^2 = -(2^2) = -4, not (-2)^2 = 4.
    fn unary(&mut self) -> Result<f64, String> {
        if let Some(Tok::Op(c)) = self.peek() {
            let c = *c;
            if c == '-' {
                self.i += 1;
                return Ok(-self.unary()?);
            }
            if c == '+' {
                self.i += 1;
                return self.unary();
            }
        }
        self.power()
    }

    /// Right-associative, and the exponent may itself be unary: 2^3^2 = 512, 2^-1 = 0.5.
    fn power(&mut self) -> Result<f64, String> {
        let b = self.atom()?;
        if let Some(Tok::Pow) = self.peek() {
            self.i += 1;
            let e = self.unary()?;
            return Ok(b.powf(e));
        }
        Ok(b)
    }

    fn atom(&mut self) -> Result<f64, String> {
        match self.next() {
            Some(Tok::Num(n)) => Ok(n),
            Some(Tok::LP) => {
                let v = self.expr()?;
                match self.next() {
                    Some(Tok::RP) => Ok(v),
                    _ => Err("missing )".into()),
                }
            }
            Some(Tok::Ident(name)) => {
                // constant?
                if let Some(Tok::LP) = self.peek() {
                    self.i += 1;
                    let mut args = vec![self.expr()?];
                    while let Some(Tok::Comma) = self.peek() {
                        self.i += 1;
                        args.push(self.expr()?);
                    }
                    match self.next() {
                        Some(Tok::RP) => {}
                        _ => return Err(format!("missing ) after {name}(")),
                    }
                    return call(&name, &args);
                }
                konst(&name)
            }
            other => Err(format!("unexpected token: {other:?}")),
        }
    }
}

fn konst(name: &str) -> Result<f64, String> {
    Ok(match name {
        "pi" => std::f64::consts::PI,
        "tau" => std::f64::consts::TAU,
        "e" => std::f64::consts::E,
        "phi" => (1.0 + 5f64.sqrt()) / 2.0,
        "inf" => f64::INFINITY,
        _ => return Err(format!("unknown name: {name}")),
    })
}

fn call(name: &str, a: &[f64]) -> Result<f64, String> {
    let one = |f: fn(f64) -> f64| -> Result<f64, String> {
        if a.len() != 1 {
            return Err(format!("{name} takes 1 argument, got {}", a.len()));
        }
        Ok(f(a[0]))
    };
    Ok(match name {
        "sqrt" => one(f64::sqrt)?,
        "cbrt" => one(f64::cbrt)?,
        "ln" => one(f64::ln)?,
        "log" | "log10" => one(f64::log10)?,
        "log2" => one(f64::log2)?,
        "exp" => one(f64::exp)?,
        "abs" => one(f64::abs)?,
        "floor" => one(f64::floor)?,
        "ceil" => one(f64::ceil)?,
        "round" => one(f64::round)?,
        "sin" => one(f64::sin)?,
        "cos" => one(f64::cos)?,
        "tan" => one(f64::tan)?,
        "asin" => one(f64::asin)?,
        "acos" => one(f64::acos)?,
        "atan" => one(f64::atan)?,
        "sinh" => one(f64::sinh)?,
        "cosh" => one(f64::cosh)?,
        "tanh" => one(f64::tanh)?,
        "logb" => {
            if a.len() != 2 {
                return Err("logb takes 2 arguments: logb(x, base)".into());
            }
            a[0].log(a[1])
        }
        "pow" => {
            if a.len() != 2 {
                return Err("pow takes 2 arguments".into());
            }
            a[0].powf(a[1])
        }
        "min" => a.iter().cloned().fold(f64::INFINITY, f64::min),
        "max" => a.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        _ => return Err(format!("unknown function: {name}")),
    })
}

/// Evaluate one expression. Public so other lanes can bind a number to tool-truth.
pub fn eval(src: &str) -> Result<f64, String> {
    let toks = lex(src)?;
    if toks.is_empty() {
        return Err("empty expression".into());
    }
    let mut p = P { t: toks, i: 0 };
    let v = p.expr()?;
    if p.i != p.t.len() {
        return Err(format!("trailing input after position {}", p.i));
    }
    Ok(v)
}

fn render(v: f64) -> String {
    if v.is_nan() {
        return "  = NaN (undefined)".into();
    }
    if v.is_infinite() {
        return format!("  = {}inf", if v < 0.0 { "-" } else { "" });
    }
    let mag = v.abs();
    // Scientific is the form that exposes a slipped decade, so it leads and is always shown.
    let mut out = format!("  = {v:.6e}");
    // A fixed-point form is only shown where it is FAITHFUL. Rendering 6.27248e-11 as
    // "0.000000000063" would round away the significant digits inside a tool whose whole
    // purpose is that a decade cannot hide — worse than printing nothing.
    if v == 0.0 || (1e-4..1e15).contains(&mag) {
        let mut plain = format!("  = {v:.12}");
        if plain.contains('.') {
            while plain.ends_with('0') {
                plain.pop();
            }
            if plain.ends_with('.') {
                plain.pop();
            }
        }
        out.push('\n');
        out.push_str(&plain);
    }
    if v.fract() == 0.0 && mag < 1e15 {
        out.push_str("\n  (exact integer)");
    }
    out
}

/// `TOOL: calc <expression>` — the arithmetic lane.
pub fn run(args: &[String]) -> String {
    let src = args.join(" ");
    let src = src.trim();
    if src.is_empty() {
        return "calc: give an expression, e.g. `calc 0.0796 * 7.88e-10`\n  \
                ops: + - * / % ^ (**)  ·  parens  ·  1e-10 scientific\n  \
                fns: sqrt cbrt ln log10 log2 exp abs floor ceil round sin cos tan \
                asin acos atan sinh cosh tanh logb(x,b) pow(x,y) min max\n  \
                consts: pi tau e phi"
            .to_string();
    }
    match eval(src) {
        Ok(v) => format!("calc: {src}\n{}", render(v)),
        Err(e) => format!("calc: {src}\n  ERROR: {e}\n  (nothing is asserted — fix the expression and re-run)"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ok(s: &str) -> f64 {
        eval(s).unwrap_or_else(|e| panic!("{s} -> {e}"))
    }

    #[test]
    fn the_two_live_failures() {
        // a slipped decade: this is 6.27e-11, NOT 6.27e-10
        let v = ok("0.0796 * 7.88e-10");
        assert!((v - 6.27248e-11).abs() < 1e-16, "got {v:e}");
        assert!((v - 6.27e-10).abs() > 1e-11, "must not equal the claimed value");
        // precedence/sign: -(d/2 + 1/2) at d=3 is -2, not -1.5
        assert_eq!(ok("-(3/2 + 1/2)"), -2.0);
    }

    #[test]
    fn precedence_and_assoc() {
        assert_eq!(ok("2 + 3 * 4"), 14.0);
        assert_eq!(ok("(2 + 3) * 4"), 20.0);
        assert_eq!(ok("2 ^ 3 ^ 2"), 512.0); // right-assoc
        assert_eq!(ok("-2 ^ 2"), -4.0); // unary binds looser than ^
        assert_eq!(ok("10 % 3"), 1.0);
    }

    #[test]
    fn scientific_and_unicode() {
        assert_eq!(ok("1e-10"), 1e-10);
        assert_eq!(ok("2.5E+3"), 2500.0);
        assert_eq!(ok("6 × 7"), 42.0);
        assert_eq!(ok("10 − 1"), 9.0);
    }

    #[test]
    fn functions_and_consts() {
        assert!((ok("sqrt(3) - 1") - 0.7320508075688772).abs() < 1e-12);
        assert!((ok("ln(10)") - 2.302585092994046).abs() < 1e-12);
        assert!((ok("exp(-pi * 0.5 / (sqrt(3)-1))") - 0.116981).abs() < 1e-5);
        assert_eq!(ok("logb(8, 2)"), 3.0);
        assert_eq!(ok("max(1, 7, 3)"), 7.0);
    }

    #[test]
    fn errors_do_not_assert() {
        assert!(eval("1/0").is_err());
        assert!(eval("2 +").is_err());
        assert!(eval("frobnicate(2)").is_err());
        assert!(eval("(1 + 2").is_err());
    }
}
