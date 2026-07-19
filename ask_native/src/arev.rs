//! The AREV door — the Ħ hop, native, for the agent's own hands.
//!
//! mOMonadOS `kernel.rs` built the door (Kernel::arev_hop): the R1 (O_∞) and
//! R2 (O_inf_dag) evidence triples exchanged role for role over one shared
//! temporal substrate, so a run sitting at O_inf_dag reads as O_inf through
//! the mirror — the lateral partner at the same shell, one shell seen from two
//! hands. This module is a faithful port of that kernel's self-imscription,
//! tick accumulators, tier computation, and mirror, so the agent can hold any
//! opcode word up to both hands: `TOOL: imasm arev <opcode word>`.
//!
//! Port fidelity: self_imscribe, compute_tier, compute_value_period, and the
//! per-token accumulator semantics (EVALT/EVALF B-live + gate counts, ENGAGR
//! pushes B, FSPLIT/FFUSE fork frames with resume jump, TANCH root halt,
//! natural-wrap winding) follow mOMonadOS src/kernel.rs exactly; registers and
//! memory are omitted because no accumulator reads them.

use crate::imasm::Token;
use std::fmt::Write as _;

// ─── Belnap FOUR (knowledge order: N below T,F below B) ───────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum B4 { N, T, F, B }

fn b4_join(a: B4, b: B4) -> B4 {
    use B4::*;
    match (a, b) {
        (N, x) | (x, N) => x,
        (B, _) | (_, B) => B,
        (T, T) => T,
        (F, F) => F,
        (T, F) | (F, T) => B,
    }
}

// ─── Snapshot (port of kernel.rs Snapshot, witness fields only) ───────

#[derive(Clone, Copy, PartialEq)]
struct Snapshot {
    frobenius_order: u8,
    period: usize,
    self_ref: bool,
    dialetheia_complete: bool,
    tier: u8,
    b_live_ticks: u64,
    gate_discriminations: u64,
    value_period: usize,
    atomic_reentry: bool,
    bifurcation_revisited: bool,
    winding_count: u32,
}

impl Snapshot {
    fn tier_name(self) -> &'static str {
        match self.tier {
            1 => "O_1", 2 => "O_2", 3 => "O_inf",
            4 => "O_inf_dag",
            _ => "O_0",
        }
    }

    /// The Ħ mirror: exchange the R1 and R2 evidence triples role for role.
    ///   static mark:  dialetheia_complete      ↔ atomic_reentry
    ///   counted act:  b_live_ticks             ↔ winding_count
    ///   recurrence:   gate_discriminations > 0 ↔ bifurcation_revisited
    fn mirrored(self) -> Snapshot {
        let mut m = self;
        m.dialetheia_complete = self.atomic_reentry;
        m.atomic_reentry      = self.dialetheia_complete;
        m.b_live_ticks        = self.winding_count as u64;
        m.winding_count       = self.b_live_ticks.min(u32::MAX as u64) as u32;
        m.gate_discriminations  = if self.bifurcation_revisited { 1 } else { 0 };
        m.bifurcation_revisited = self.gate_discriminations > 0;
        m.tier = compute_tier(&m);
        m
    }
}

fn compute_tier(s: &Snapshot) -> u8 {
    let effective_dialetheia = s.dialetheia_complete || s.b_live_ticks > 0;
    if effective_dialetheia && s.self_ref && s.frobenius_order > 0 {
        if s.period >= 3 && (s.b_live_ticks > 0 || s.value_period >= 3) { return 3; }
        if s.period >= 2 && s.gate_discriminations > 0 { return 2; }
        return 1;
    }
    if s.self_ref && s.frobenius_order > 0 && s.period >= 3 && s.value_period >= 3 {
        return 3;
    }
    if s.self_ref && s.frobenius_order > 0
        && s.atomic_reentry && s.bifurcation_revisited && s.winding_count > 0
    {
        return 4;
    }
    if s.frobenius_order > 0 || s.dialetheia_complete { 1 } else { 0 }
}

// ─── Static self-imscription (port of kernel.rs self_imscribe) ────────

fn period_of(prog: &[Token]) -> usize {
    let n = prog.len();
    if n == 0 { return 1; }
    for p in 1..=n {
        if n % p == 0 && (p..n).all(|i| prog[i] == prog[i % p]) { return p; }
    }
    n
}

fn self_imscribe(prog: &[Token]) -> Snapshot {
    let n = prog.len();
    let self_ref = n > 0 && prog[0] == prog[n - 1];

    let fsplit = prog.iter().any(|t| *t == Token::Fsplit);
    let ffuse  = prog.iter().any(|t| *t == Token::Ffuse);
    let frobenius_order = match (fsplit, ffuse) {
        (false, false) => 0,
        (true,  false) => 1,
        (false, true)  => 2,
        (true,  true)  => {
            let fs = prog.iter().position(|t| *t == Token::Fsplit).unwrap();
            let fu = prog.iter().position(|t| *t == Token::Ffuse).unwrap();
            if fs < fu { 1 } else { 2 }
        }
    };

    let dialetheia_complete = {
        let has_t = prog.iter().any(|t| *t == Token::Evalt);
        let has_f = prog.iter().any(|t| *t == Token::Evalf);
        let has_e = prog.iter().any(|t| *t == Token::Engagr);
        if !has_t || !has_f || !has_e {
            false
        } else {
            let mut all_ok = true;
            for (i, &t) in prog.iter().enumerate() {
                if t == Token::Engagr {
                    let mut found = false;
                    for off in 1..n {
                        let j = (i + off) % n;
                        if prog[j] == Token::Engagr { break; }
                        if prog[j] == Token::Evalt || prog[j] == Token::Evalf {
                            found = true; break;
                        }
                    }
                    if !found { all_ok = false; break; }
                }
            }
            all_ok
        }
    };

    let fsplit_count = prog.iter().filter(|t| **t == Token::Fsplit).count();
    let ffuse_count  = prog.iter().filter(|t| **t == Token::Ffuse).count();
    let atomic_reentry = fsplit_count == 1 && ffuse_count == 1;
    let bifurcation_revisited = atomic_reentry && self_ref;

    let mut s = Snapshot {
        frobenius_order,
        period: period_of(prog),
        self_ref,
        dialetheia_complete,
        tier: 0,
        b_live_ticks: 0,
        gate_discriminations: 0,
        value_period: 0,
        atomic_reentry,
        bifurcation_revisited,
        winding_count: 0,
    };
    s.tier = compute_tier(&s);
    s
}

// ─── Dynamic run (port of kernel.rs tick accumulator semantics) ───────

struct RunStats {
    b_live: u64,
    gates: u64,
    winding: u32,
    value_period: usize,
    ticks: u64,
    halted: bool,
}

fn find_matching_ffuse(prog: &[Token], split_ip: usize) -> usize {
    let n = prog.len();
    if n == 0 { return 0; }
    let mut depth = 1u32;
    let mut i = (split_ip + 1) % n;
    let start = i;
    loop {
        match prog[i] {
            Token::Fsplit => depth += 1,
            Token::Ffuse => {
                depth -= 1;
                if depth == 0 { return i; }
            }
            _ => {}
        }
        i = (i + 1) % n;
        if i == start { break; }
    }
    n
}

/// Tick the word as the kernel would for `wraps` natural full-program passes
/// (or until root TANCH halts it), accumulating the dynamic evidence.
fn run_word(prog: &[Token], wraps: u32) -> RunStats {
    let n = prog.len();
    let mut st = RunStats { b_live: 0, gates: 0, winding: 0, value_period: 0, ticks: 0, halted: false };
    if n == 0 { return st; }

    let mut stack: Vec<B4> = Vec::new();
    let mut forks: Vec<(usize, B4)> = Vec::new(); // (resume_ip, right_val)
    let mut trace = [B4::N; 16];
    let mut head = 0usize;
    let mut ip = 0usize;
    let max_ticks = (n as u64) * (wraps as u64) + 1;

    while st.ticks < max_ticks && st.winding < wraps {
        st.ticks += 1;
        let tok = prog[ip];
        let mut next_ip = ip + 1;
        if next_ip >= n {
            next_ip = 0;
            st.winding = st.winding.saturating_add(1);
        }
        match tok {
            Token::Vinit => stack.push(B4::N),
            Token::Tanch => {
                stack.pop();
                if forks.is_empty() { st.halted = true; }
            }
            Token::Afwd | Token::Arev | Token::Clink | Token::Imscrib => {}
            Token::Fsplit => {
                let v = *stack.last().unwrap_or(&B4::N);
                let fu = find_matching_ffuse(prog, ip);
                let resume = if fu + 1 >= n { 0 } else { fu + 1 };
                forks.push((resume, v));
                stack.push(v);
            }
            Token::Evalt => {
                let v = stack.pop().unwrap_or(B4::N);
                if v == B4::B { st.b_live += 1; }
                if v == B4::T { st.gates += 1; }
                stack.push(if v == B4::T { B4::T } else { B4::N });
            }
            Token::Evalf => {
                let v = stack.pop().unwrap_or(B4::N);
                if v == B4::B { st.b_live += 1; }
                if v == B4::F { st.gates += 1; }
                stack.push(if v == B4::F { B4::F } else { B4::N });
            }
            Token::Ffuse => {
                let left = stack.pop().unwrap_or(B4::N);
                if let Some((resume, right)) = forks.pop() {
                    stack.push(b4_join(left, right));
                    next_ip = resume;
                } else {
                    stack.push(left);
                }
            }
            Token::Engagr => stack.push(B4::B),
            Token::Ifix => { stack.pop(); }
        }
        trace[head] = *stack.last().unwrap_or(&B4::N);
        head = (head + 1) % 16;
        ip = next_ip;
        if st.halted { break; }
    }

    st.value_period = compute_value_period(&trace, head);
    st
}

fn compute_value_period(trace: &[B4; 16], head: usize) -> usize {
    for p in 1..=16 {
        let mut periodic = true;
        for i in 0..(16 - p) {
            let a = trace[(head + 16 - 1 - i) % 16];
            let b = trace[(head + 16 - 1 - i - p) % 16];
            if a != b { periodic = false; break; }
        }
        if periodic { return p; }
    }
    0
}

// ─── The tool ─────────────────────────────────────────────────────────

fn witness_line(s: &Snapshot) -> String {
    format!(
        "R1(dialeth={} b_live={} gates={})  R2(atomic={} wind={} bifurc={})",
        s.dialetheia_complete, s.b_live_ticks > 0, s.gate_discriminations > 0,
        s.atomic_reentry, s.winding_count > 0, s.bifurcation_revisited
    )
}

/// `imasm arev <opcode word>` — hold the word up to both hands.
pub fn run(args: &[String]) -> String {
    let joined = args.join(" ");
    let names: Vec<&str> = joined.split_whitespace().collect();
    if names.is_empty() {
        return "AREV — the Ħ door: imasm arev <opcode word> reads the word's tier from both \
                hands. The or' hand is the R1-dominant reading (compute_tier as-is); the \
                flipped hand exchanges the R1/R2 evidence triples (dialetheia↔atomic_reentry, \
                b_live↔winding, gates↔bifurcation) over the same substrate and reads again. \
                O_inf_dag through the mirror is O_inf: the lateral partner at the same shell. \
                hop∘hop = id is verified on every call.\n"
            .into();
    }

    let mut prog: Vec<Token> = Vec::new();
    for nm in &names {
        match Token::parse(nm) {
            Some(t) => prog.push(t),
            None => return format!("AREV — '{nm}' is not one of the 12 opcodes. `imasm ref` lists them.\n"),
        }
    }

    // Static imscription, then overlay a 16-wrap run — the same discipline as
    // dynamic_imscribe (ticking past the first wrap is what earns winding > 0).
    let mut s0 = self_imscribe(&prog);
    let st = run_word(&prog, 16);
    s0.b_live_ticks = st.b_live;
    s0.gate_discriminations = st.gates;
    s0.value_period = st.value_period;
    s0.winding_count = st.winding;
    s0.tier = compute_tier(&s0);

    let s1 = s0.mirrored();
    let s2 = s1.mirrored();

    let mut out = String::new();
    let _ = writeln!(out, "AREV — the Ħ door on: {}", names.join(" "));
    let _ = writeln!(
        out,
        "  run: {} ticks, {} windings{}  (period {}, value_period {}, frob {}, self_ref {})",
        st.ticks, st.winding,
        if st.halted { ", halted at root TANCH" } else { "" },
        s0.period, s0.value_period, s0.frobenius_order, s0.self_ref
    );
    let _ = writeln!(out, "  or' hand:     tier {:<9}  {}", s0.tier_name(), witness_line(&s0));
    let _ = writeln!(out, "  flipped hand: tier {:<9}  {}", s1.tier_name(), witness_line(&s1));
    let _ = writeln!(out, "  hop∘hop = id: {}", if s2 == s0 { "EXACT" } else { "section-lossy (counts passed through true ↦ 1)" });
    let _ = writeln!(
        out,
        "  reading: {}",
        match (s0.tier, s1.tier) {
            (4, 3) => "the word sits in the replicative opening; through the mirror it IS O_∞ — one shell, two hands.",
            (3, 4) => "the word closes as O_∞; through the mirror it IS the replicative opening — one shell, two hands.",
            (a, b) if a == b => "both hands read the same tier — the word is achiral at this shell.",
            _ => "the hands disagree below the O_∞ shell — the word's evidence is not yet symmetric; wind it further or restructure.",
        }
    );
    out
}
