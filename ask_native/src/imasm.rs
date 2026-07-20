//! IMASM polymer composition — the 12-opcode grammar as a free topology, native.
//!
//! `composer.py` (and the agent's COMPOSE:/TOKEN: play) models an IMASM program
//! as a flat token LINE and forces every FSPLIT to close with an FFUSE, so it can
//! express only a chain and a ring. Real IMASM composition is a directed graph
//! whose nodes are opcodes and whose edges respect each opcode's own valence
//! (from IMSCRIBr `tokens.py::TOKEN_ARITY`):
//!
//!     VINIT  in 0 / out 1   the only SOURCE
//!     FSPLIT in 1 / out 2   δ — the only opcode that may fan out (branch)
//!     FFUSE  in 2 / out 1   μ — the only opcode that may merge in (fuse)
//!     every other opcode    in 1 / out 1   chain-internal bead
//!
//! An arm that runs out of successors is an open OUT-port: a living / telechelic
//! polymer end, reported not fatal — exactly the reactive-end framing the forge
//! verbs already use. The line and the ring are just the two β≤1 single-strand
//! special cases; this module also builds stars, combs/grafts, bubbles and
//! networks, and NAMES any graph by its topological invariants — circuit rank
//! β = E − V + C (independent loops), branch/merge/source/sink census, arm count,
//! and adjacency spectral radius ρ — instead of a fork-balance stack.
//!
//! All computation is pure Rust (no shell, no python): the agent calls
//! `TOOL: imasm <op> …` and the report comes straight back.

use std::fmt::Write as _;

pub use imasm_core::classic::Token;


pub use imasm_core::check::{from_sequence, match_pairs, ClosureState, Graph};

/// ask_native's presentation and topology extensions over the core Graph
/// (the engine itself — ancestry, closure, validation — lives in imasm_core).
pub trait GraphExt {
    fn open_ends(&self) -> (usize, usize);
    fn spectral_radius(&self) -> f64;
    fn branch_clusters(&self, bps: &[usize]) -> usize;
    fn classify(&self) -> String;
    fn program_str(&self) -> String;
    fn code_str(&self) -> String;
}

impl GraphExt for Graph {
    fn open_ends(&self) -> (usize, usize) {
        // (open out-ports at non-final beads = living arm ends, open in-ports = unrooted)
        let mut open_out = 0;
        let mut open_in = 0;
        for (n, &tok) in self.nodes.iter().enumerate() {
            let (ai, ao) = tok.arity();
            if self.out_degree(n) < ao && tok != Token::Tanch {
                open_out += 1;
            }
            if self.in_degree(n) < ai && tok != Token::Vinit {
                open_in += 1;
            }
        }
        (open_out, open_in)
    }

    // ── spectral radius (power iteration, undirected adjacency) ──────────────

    fn spectral_radius(&self) -> f64 {
        let n = self.nodes.len();
        if n == 0 {
            return 0.0;
        }
        let mut adj = vec![vec![0.0f64; n]; n];
        for &(a, b) in &self.edges {
            adj[a][b] += 1.0;
            adj[b][a] += 1.0;
        }
        let mut v = vec![1.0f64; n];
        let mut lambda = 0.0;
        for _ in 0..200 {
            let mut w = vec![0.0f64; n];
            for i in 0..n {
                for j in 0..n {
                    w[i] += adj[i][j] * v[j];
                }
            }
            let norm = w.iter().map(|x| x * x).sum::<f64>().sqrt();
            if norm < 1e-12 {
                return 0.0;
            }
            for x in w.iter_mut() {
                *x /= norm;
            }
            lambda = norm;
            v = w;
        }
        lambda
    }

    // ── classification ───────────────────────────────────────────────────────

    /// Do the branch points form ONE contiguous hub (direct edges among
    /// themselves)? A star has one hub cluster; a comb strings them along a
    /// backbone with beads between.
    fn branch_clusters(&self, bps: &[usize]) -> usize {
        if bps.is_empty() {
            return 0;
        }
        let idx = |x: usize| bps.iter().position(|&b| b == x);
        let mut parent: Vec<usize> = (0..bps.len()).collect();
        fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }
        for &(a, b) in &self.edges {
            if let (Some(ia), Some(ib)) = (idx(a), idx(b)) {
                let ra = find(&mut parent, ia);
                let rb = find(&mut parent, ib);
                if ra != rb {
                    parent[ra] = rb;
                }
            }
        }
        let mut roots: Vec<usize> = (0..bps.len()).map(|i| find(&mut parent, i)).collect();
        roots.sort_unstable();
        roots.dedup();
        roots.len()
    }

    fn classify(&self) -> String {
        let v = self.nodes.len();
        let e = self.edges.len();
        let c = self.components();
        let beta = self.circuit_rank();
        let bps = self.branch_points();
        let mps = self.merge_points();
        let sinks = self.sinks();
        let rho = self.spectral_radius();
        let (open_out, open_in) = self.open_ends();

        let (label, arms, detail): (&str, usize, String) = if v == 0 {
            ("trivial", 0, String::new())
        } else if c > 1 {
            ("network", 0, format!("{c} disconnected strands"))
        } else if beta == 0 && bps.is_empty() {
            ("linear", 0, "a single strand".to_string())
        } else if beta == 0 && !bps.is_empty() {
            let arms = sinks;
            if self.branch_clusters(&bps) == 1 {
                let d = if arms >= 3 {
                    // The abstract star K(1,f) has one degree-f hub, ρ=√f. But IMASM
                    // fan-out caps at 2 (FSPLIT is out-2), so the hub is REALIZED as a
                    // caterpillar of f−1 FSPLIT fan-nodes: the true ρ tends to 2, not √f.
                    format!(
                        "K(1,{arms}) star; realized as a caterpillar of {} FSPLIT fan-nodes \
                         (opcode fan-out is 2), so ρ→2 not √{arms}={:.4} of the abstract hub",
                        bps.len(),
                        (arms as f64).sqrt()
                    )
                } else {
                    "f<3 — degenerate star (linear/isolated)".to_string()
                };
                ("star", arms, d)
            } else {
                ("comb", arms, format!("{} branch points, {arms} arm termini — graft/comb tree", bps.len()))
            }
        } else if beta == 1 && bps.is_empty() && mps.is_empty() {
            ("ring", 0, "single cycle, no pendants".to_string())
        } else if beta >= 1 && (!bps.is_empty() || !mps.is_empty()) {
            let lab = if beta == 1 { "branched" } else { "network" };
            (lab, sinks, format!("{beta} independent loop(s) with {} branch / {} fusion point(s)", bps.len(), mps.len()))
        } else {
            ("network", 0, format!("β={beta}"))
        };

        let mut out = String::new();
        let _ = write!(
            out,
            "  topology: {label} | V={v} E={e} β={beta} branch={} merge={} src={} sink={}",
            bps.len(), mps.len(), self.sources(), sinks
        );
        if arms > 0 {
            let _ = write!(out, " arms={arms}");
        }
        let _ = write!(out, " ρ={rho:.4}");
        if !detail.is_empty() {
            let _ = write!(out, "\n  {detail}");
        }
        let _ = write!(out, "\n  μ∘δ: {}", match self.closure_state() {
            ClosureState::Closed(n) => format!(
                "CLOSED — {n} δ-arm reconnection(s) at μ that carry a TRANSFORMATION \
                 (split → work → fuse: a type-check, not bare identity)"
            ),
            ClosureState::Identity =>
                "identity — δ arms reconnect at μ but nothing transforms between them; \
                 μ∘δ=id does no work and type-checks nothing".to_string(),
            ClosureState::Open =>
                "OPEN — a δ fork or μ fuse dangles unreconnected (not closed yet)".to_string(),
            ClosureState::None =>
                "none — no δ/μ dyad; a bare line or cycle is NOT a closure (β is not diagnostic)".to_string(),
        });
        if open_out > 0 || open_in > 0 {
            let _ = write!(
                out,
                "\n  open valences (living ends): {open_out} out, {open_in} in — reactive, not errors"
            );
        }
        out
    }

    fn program_str(&self) -> String {
        // linear strand → arrows; else list the edges
        if self.circuit_rank() == 0 && self.branch_points().is_empty() && self.merge_points().is_empty()
            && self.components() == 1 && self.sources() == 1 && self.sinks() == 1
        {
            let mut order = Vec::new();
            let mut cur = (0..self.nodes.len()).find(|&n| self.in_degree(n) == 0);
            let mut seen = vec![false; self.nodes.len()];
            while let Some(n) = cur {
                if seen[n] {
                    break;
                }
                seen[n] = true;
                order.push(self.nodes[n].name());
                cur = self.successors(n).into_iter().next();
            }
            return order.join(" → ");
        }
        self.edges
            .iter()
            .map(|&(a, b)| format!("{a}:{} → {b}:{}", self.nodes[a].name(), self.nodes[b].name()))
            .collect::<Vec<_>>()
            .join("  ;  ")
    }

    /// The compact codon string: every node as its single-glyph code() in node order, no
    /// separators — the opcode word as a sequence. Round-trips through parse (each glyph is
    /// its own alias). This is the "single-letter code" form the READING_GUIDE's glyph
    /// vocabulary underwrites.
    fn code_str(&self) -> String {
        self.nodes.iter().map(|t| t.code()).collect()
    }
}

// ── constructors ─────────────────────────────────────────────────────────────

fn chain(toks: &[Token]) -> Graph {
    let mut g = Graph::new();
    g.chain_of(toks);
    g
}

fn ring(toks: &[Token]) -> Graph {
    let mut g = Graph::new();
    let ids = g.chain_of(toks);
    if let (Some(&first), Some(&last)) = (ids.first(), ids.last()) {
        g.connect(last, first);
    }
    g
}

/// One core bead fanned to f arms via f−1 chained FSPLITs; each arm is a strand.
fn star(core: Token, arms: &[Vec<Token>]) -> Graph {
    let mut g = Graph::new();
    let hub = g.add(core);
    let f = arms.len();
    let fan: Vec<usize> = (0..f.saturating_sub(1)).map(|_| g.add(Token::Fsplit)).collect();
    let mut prev = hub;
    for &fid in &fan {
        g.connect(prev, fid);
        prev = fid;
    }
    // out-slots to hang arms on: each split contributes one, the last split a second
    let mut slots: Vec<usize> = Vec::new();
    if fan.is_empty() {
        slots.push(hub);
    } else {
        slots.extend(fan.iter().copied());
        slots.push(*fan.last().unwrap());
    }
    for (i, body) in arms.iter().enumerate() {
        let anchor = *slots.get(i).unwrap_or_else(|| slots.last().unwrap());
        let arm = g.chain_of(body);
        if let Some(&first) = arm.first() {
            g.connect(anchor, first);
        }
    }
    g
}

/// pre → FSPLIT →(a | b)→ FFUSE → post: the reconverging series-parallel bubble.
fn bubble(pre: &[Token], a: &[Token], b: &[Token], post: &[Token]) -> Graph {
    let mut g = Graph::new();
    let pre_ids = g.chain_of(pre);
    let split = g.add(Token::Fsplit);
    if let Some(&last) = pre_ids.last() {
        g.connect(last, split);
    }
    let a_ids = g.chain_of(a);
    let b_ids = g.chain_of(b);
    let fuse = g.add(Token::Ffuse);
    if let (Some(&af), Some(&al)) = (a_ids.first(), a_ids.last()) {
        g.connect(split, af);
        g.connect(al, fuse);
    }
    if let (Some(&bf), Some(&bl)) = (b_ids.first(), b_ids.last()) {
        g.connect(split, bf);
        g.connect(bl, fuse);
    }
    let post_ids = g.chain_of(post);
    if let Some(&first) = post_ids.first() {
        g.connect(fuse, first);
    }
    g
}

/// A backbone strand with pendant arms grafted at FSPLIT teeth.
/// `teeth` = (backbone_position, arm_tokens).
fn comb(backbone: &[Token], teeth: &[(usize, Vec<Token>)]) -> Graph {
    let mut g = Graph::new();
    let spine = g.chain_of(backbone);
    for (pos, arm) in teeth {
        if spine.is_empty() {
            break;
        }
        let p = (*pos).min(spine.len() - 1);
        let node = spine[p];
        let after = g.successors(node);
        let fid = g.add(Token::Fsplit);
        g.edges.retain(|&(x, y)| !(x == node && after.contains(&y)));
        g.connect(node, fid);
        for nxt in after {
            g.connect(fid, nxt);
        }
        let arm_ids = g.chain_of(arm);
        if let Some(&first) = arm_ids.first() {
            g.connect(fid, first);
        }
    }
    g
}

/// Reconstruct a graph from an ordered opcode bootstrap plus its FSPLIT/FFUSE
/// pair list (from a type's `topology_report`). The inline chain carries the
/// T-branch (FSPLIT → next, prev → FFUSE); each pair adds the F-branch arc
/// FSPLIT → FFUSE, so the fork gets out-2, the fuse in-2, and β rises by one per
/// pair — faithful to the empty-F-branch flat_chain the auto-designer emits.
/// The strange loop: every TYPE the Grammar writes with is itself a full IMASM
/// program. Locate a `the_primitive_type_called_<name>` ob3ect, pull its ordered
/// bootstrap opcodes (phase_4) and fork/fuse pairs (topology_report), and hand
/// back the reconstructed graph plus the per-step domain actions.
fn expand_type(name: &str) -> Result<(Vec<Token>, Vec<(usize, usize)>, Vec<String>), String> {
    let key = name
        .trim()
        .trim_start_matches("the_primitive_type_called_")
        .trim_start_matches("the primitive type called ")
        .replace(' ', "_")
        .to_ascii_lowercase();
    let roots = [
        crate::expand_user("~/imsgct/MoDoT/ob3ects/primitives"),
        crate::expand_user("~/imsgct/MoDoT/ob3ects"),
    ];
    let mut path = None;
    for r in &roots {
        let cand = std::path::Path::new(r)
            .join(format!("the_primitive_type_called_{key}_ob3ect.json"));
        if cand.is_file() {
            path = Some(cand);
            break;
        }
    }
    let Some(path) = path else {
        return Err(format!(
            "type '{key}' not found under ob3ects/primitives/. The types are the 49 Shavian \
             names (ado, air, ash, awe, …); each is a full IMASM program. Use `imasm types` to list."
        ));
    };
    let text = std::fs::read_to_string(&path).map_err(|e| format!("read {}: {e}", path.display()))?;
    let v: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("parse: {e}"))?;

    let steps = v
        .pointer("/phases/phase_4/steps")
        .and_then(|s| s.as_array())
        .ok_or("no phase_4 bootstrap steps in this ob3ect")?;
    let mut ops = Vec::new();
    let mut actions = Vec::new();
    for s in steps {
        let op = s.get("opcode").and_then(|o| o.as_str()).unwrap_or("");
        if let Some(t) = Token::parse(op) {
            ops.push(t);
            let act = s.get("domain_action").and_then(|a| a.as_str()).unwrap_or("");
            actions.push(format!("{op}: {act}"));
        }
    }
    // fork/fuse pairs indexed into the bootstrap sequence
    let mut pairs = Vec::new();
    if let Some(pl) = v.pointer("/topology_report/pair_list").and_then(|p| p.as_array()) {
        for p in pl {
            if let Some(a) = p.as_array() {
                if a.len() == 2 {
                    if let (Some(x), Some(y)) = (a[0].as_u64(), a[1].as_u64()) {
                        pairs.push((x as usize, y as usize));
                    }
                }
            }
        }
    }
    if ops.is_empty() {
        return Err(format!("type '{key}' has no readable opcode sequence"));
    }
    Ok((ops, pairs, actions))
}

fn list_types() -> Vec<String> {
    let dir = crate::expand_user("~/imsgct/MoDoT/ob3ects/primitives");
    let mut names = Vec::new();
    if let Ok(rd) = std::fs::read_dir(&dir) {
        for e in rd.flatten() {
            let f = e.file_name().to_string_lossy().into_owned();
            if let Some(rest) = f.strip_prefix("the_primitive_type_called_") {
                if let Some(nm) = rest.strip_suffix("_ob3ect.json") {
                    names.push(nm.to_string());
                }
            }
        }
    }
    names.sort();
    names
}

// ── argument parsing ─────────────────────────────────────────────────────────

/// Split a flat token list on `:` / `|` segment separators (glued or standalone).
fn segments(args: &[String]) -> Vec<Vec<Token>> {
    let joined = args.join(" ");
    joined
        .split(|c| c == ':' || c == '|')
        .map(|seg| seg.split_whitespace().filter_map(Token::parse).collect::<Vec<Token>>())
        .collect()
}

/// Decompose a glued single-glyph code word (`⊢>◇+×●¬⊣`) into its tokens. Every char must
/// be a valid code, else None — so a real name (VINIT) is never mangled into letters.
fn parse_codons(chunk: &str) -> Option<Vec<Token>> {
    let mut out = Vec::new();
    for c in chunk.chars() {
        out.push(Token::parse(&c.to_string())?);
    }
    (!out.is_empty()).then_some(out)
}

pub(crate) fn tok_list(args: &[String]) -> Vec<Token> {
    args.iter()
        .flat_map(|s| s.split_whitespace())
        // Whole-token/alias first (VINIT, FS, →); only a chunk that is NOT a known token is
        // tried as a glued codon string, so multi-letter names are never char-split.
        .flat_map(|chunk| {
            Token::parse(chunk)
                .map(|t| vec![t])
                .or_else(|| parse_codons(chunk))
                .unwrap_or_default()
        })
        .collect()
}

fn report(title: &str, g: &Graph) -> String {
    let mut s = String::new();
    let _ = writeln!(s, "IMASM {title}");
    let _ = writeln!(s, "  program: {}", g.program_str());
    let _ = writeln!(s, "  code: {}", g.code_str());
    let _ = writeln!(s, "{}", g.classify());
    let errs = g.validate();
    if errs.is_empty() {
        let _ = writeln!(s, "  grammar: valid (μ∘δ bookkeeping holds — only δ branches, only μ fuses)");
    } else {
        let _ = writeln!(s, "  grammar: INVALID");
        for e in &errs {
            let _ = writeln!(s, "    ✗ {e}");
        }
    }
    // Hint: a naive ring/chain/wire whose FSPLIT/FFUSE pair dangles OPEN was meant to be a
    // PROTOCOL, which wires the δ arms to the μ so the dual closes. Point the caller there.
    if title != "protocol"
        && matches!(g.closure_state(), ClosureState::Open)
        && g.nodes.iter().any(|&t| t == Token::Fsplit)
        && g.nodes.iter().any(|&t| t == Token::Ffuse)
    {
        let _ = writeln!(
            s,
            "  hint: this FSPLIT/FFUSE pair does not reconnect — `imasm protocol <opcodes>` wires \
             the δ fork arms to the μ fuse (the μ∘δ closure a naive ring/wire leaves open). A \
             protocol does NOT close by looping back to VINIT (a source) — it closes at the fuse."
        );
    }
    s
}

const REFERENCE: &str = "\
IMASM POLYMER TOPOLOGY — composition beyond lines and cycles
An IMASM program is a DIRECTED GRAPH of the 12 opcodes, not only a line. Each
opcode's valence sets its edges: VINIT source (in0), FSPLIT δ branch (out2),
FFUSE μ fuse (in2), every other opcode in1/out1. Only FSPLIT may fan out; only
FFUSE may merge in. An arm that runs out of successors is a living end (open
out-port), reported not fatal. The shape is named by circuit rank β = E−V+C
(independent loops): β=0 tree, β=1 one ring/bubble, β>1 network.
  imasm chain  T1 T2 …               a single strand (the classic line)
  imasm ring   T1 T2 …               a cycle (β=1), fork/fuse NOT reconnected
  imasm protocol T1 T2 …             an opcode word built so its FSPLIT/FFUSE pairs
                                      RECONNECT (δ arm → μ) — the way to CLOSE a
                                      protocol/loop; a naive ring leaves the fork open
  imasm star   CORE : arm1 : arm2 …  hub with f arms; K(1,f), ρ=√f (f≥3)
  imasm comb   BACKBONE : p arm : …  backbone with pendant teeth at position p
  imasm bubble PRE : A : B : POST     FSPLIT→(A|B)→FFUSE reconvergence (β=1)
  imasm wire N0 N1 … / i-j i-k …     FREE composition: any node set + edge set —
                                      networks (β>1), fused rings, cross-branch,
                                      non-planar; the primitive the rest specialize
  imasm classify T1 T2 …             read a flat token line as a chain and name it
  imasm check T1 T2 …                TYPE-CHECK your own thinking. CLOSE CONDITION:
                                      μ∘δ over a TRANSFORMED object — δ splits, the
                                      arms do WORK, μ fuses. Verdicts: T closes (a real
                                      type-check); N identity (split→fuse with no work —
                                      μ∘δ=id verifies nothing); B open (a fork dangles)
                                      or paradox (ENGAGR); F ill-typed; N no fork/void.
                                      A bare cycle is NOT a closure — β is not diagnostic.
  imasm prove <name|word>            take the closure verdict to the ACTUAL p4ramill
                                      kernel (lake build against BelnapSplitFuse:
                                      B_is_the_only_bifurcation_point, split_fuse_id)
  imasm define <name> <op> <args…>   BUILD a tool in the kernel-constrained space —
                                      only a grammar-valid composition is admitted;
                                      an ill-typed one is REFUSED
  imasm run <name>  /  imasm tools    invoke a defined tool / list the space
  imasm types                        list the 49 Shavian TYPES (each is a program)
  imasm expand <type>                unfold a type into its own IMASM sequence
STRANGE LOOP: the 49 types the Grammar writes tuples with are not atomic — each
UNFOLDS into its own 12-opcode IMASM program (`imasm expand ado`). Splice an
expanded type's sequence into a polymer arm to pivot through state space AS that
type: the alphabet's letters are themselves words in the language.
SINGLE-GLYPH CODES: each opcode has a one-symbol code (READING_GUIDE §3 glyphs), so
a word can be written glued, no spaces — `⊢>◇+=⊙<×⊞●⊙¬⊣` is the same protocol as the
13 spelled-out tokens. Every build echoes the word's `code:`. The alphabet is fully
symbolic — no Latin initials; the retired V/T/B letters and ← no longer parse:
  ⊢ VINIT   ⊣ TANCH   > AFWD   < AREV   = CLINK   ⊙ IMSCRIB
  ◇ FSPLIT  ● FFUSE   + EVALT  × EVALF  ⊞ ENGAGR  ¬ IFIX
Every build reports topology label, β, branch/merge/source/sink census, arm
count, spectral radius ρ, and a grammar validation.";

/// Dispatch `TOOL: imasm <op> …`. Pure computation; returns the report string.
/// FSPLIT/FFUSE pairs by depth-first stack (mirrors IMSCRIBr match_pairs).
/// Build a graph from one constructive op + its args. Shared by the dispatcher
/// and the tool registry so a defined tool rebuilds through the identical parse.
/// Err is a usage/why-not message.
fn build_graph(op: &str, rest: &[String]) -> Result<(&'static str, Graph), String> {
    match op {
        "chain" => Ok(("chain", chain(&tok_list(rest)))),
        "ring" | "cycle" | "loop" => Ok(("ring", ring(&tok_list(rest)))),
        // A PROTOCOL is an opcode word built so its FSPLIT/FFUSE pairs RECONNECT (the δ arm is
        // wired to the μ) — the correct way to close a protocol/loop. A naive `ring` leaves the
        // fork dangling; `protocol` closes the μ∘δ. This is the builder to reach for when you
        // have an opcode sequence with a fork that must fuse.
        "protocol" | "seq" | "sequence" => {
            let ops = tok_list(rest);
            let pairs = match_pairs(&ops);
            Ok(("protocol", from_sequence(&ops, &pairs)))
        }
        "classify" | "read" => Ok(("classify", chain(&tok_list(rest)))),
        "wire" | "graph" | "free" => {
            let joined = rest.join(" ");
            let mut halves = joined.splitn(2, '/');
            let node_str = halves.next().unwrap_or("");
            let edge_str = halves.next().unwrap_or("");
            let toks: Vec<Token> = node_str.split_whitespace().filter_map(Token::parse).collect();
            if toks.is_empty() {
                return Err("wire needs nodes then `/` then edges: \
                            imasm wire VINIT FSPLIT AFWD AREV FFUSE / 0-1 1-2 1-3 2-4 3-4\n"
                    .into());
            }
            let mut g = Graph::new();
            for t in &toks {
                g.add(*t);
            }
            let n = g.nodes.len();
            for e in edge_str.split_whitespace() {
                let mut it = e.split(|c| c == '-' || c == ',' || c == ':' || c == '>');
                let a = it.next().and_then(|x| x.trim().parse::<usize>().ok());
                let b = it.next().and_then(|x| x.trim().parse::<usize>().ok());
                if let (Some(a), Some(b)) = (a, b) {
                    if a < n && b < n {
                        g.connect(a, b);
                    }
                }
            }
            Ok(("wire (free composition)", g))
        }
        "star" => {
            let segs = segments(rest);
            if segs.is_empty() || segs[0].is_empty() {
                return Err("star needs a core then arms: imasm star CORE : arm1 : arm2 : arm3\n".into());
            }
            let core = segs[0][0];
            let arms: Vec<Vec<Token>> = segs[1..].iter().filter(|s| !s.is_empty()).cloned().collect();
            if arms.len() < 3 {
                return Err(format!(
                    "a star needs ≥3 arms (a hub with f<3 arms is just a chain). You gave {}. \
                     Form: imasm star CORE : arm1 : arm2 : arm3\n",
                    arms.len()
                ));
            }
            Ok(("star", star(core, &arms)))
        }
        "bubble" | "fork" => {
            let segs = segments(rest);
            if segs.len() < 3 {
                return Err("bubble needs: imasm bubble PRE : A : B [: POST]  (PRE/POST may be empty)\n".into());
            }
            let empty: Vec<Token> = Vec::new();
            let pre = segs.first().unwrap_or(&empty);
            let a = &segs[1];
            let b = &segs[2];
            let post = segs.get(3).unwrap_or(&empty);
            Ok(("bubble", bubble(pre, a, b, post)))
        }
        "comb" | "graft" => {
            let segs = segments(rest);
            if segs.len() < 2 || segs[0].is_empty() {
                return Err("comb needs: imasm comb BACKBONE : P arm : Q arm …  (P,Q are backbone positions)\n".into());
            }
            let backbone = segs[0].clone();
            let joined = rest.join(" ");
            let tooth_texts: Vec<&str> = joined.split(|c| c == ':' || c == '|').skip(1).collect();
            let mut teeth: Vec<(usize, Vec<Token>)> = Vec::new();
            for t in tooth_texts {
                let mut it = t.split_whitespace();
                let pos = it.next().and_then(|p| p.parse::<usize>().ok()).unwrap_or(0);
                let arm: Vec<Token> = it.filter_map(Token::parse).collect();
                if !arm.is_empty() {
                    teeth.push((pos, arm));
                }
            }
            if teeth.is_empty() {
                return Err("comb: no valid teeth. Form: imasm comb VINIT AFWD IMSCRIB : 1 CLINK : 2 IMSCRIB\n".into());
            }
            Ok(("comb", comb(&backbone, &teeth)))
        }
        _ => Err(format!("'{op}' is not a constructive op (use chain|ring|star|comb|bubble|wire)\n")),
    }
}

/// Type-check a decision expressed as an opcode word against the grammar, and
/// return a Belnap verdict + guidance. This is the model checking its OWN thinking:
///   T — well-typed and CLOSES (μ∘δ balanced): the reasoning resolves. Proceed.
///   B — well-typed but holds an open fork or engages paradox (ENGAGR): look again.
///   F — ILL-TYPED (illegal fork/fuse valence): the reasoning is malformed. Revise.
///   N — nothing committed (void): state the decision as an opcode word first.
fn verdict_check(rest: &[String]) -> String {
    let ops = tok_list(rest);
    if ops.is_empty() {
        return "IMASM check → N (void)\n  No committed opcodes. Express the decision as an \
                opcode word (e.g. VINIT IMSCRIB AFWD FSPLIT EVALT EVALF FFUSE IFIX) and re-check.\n"
            .into();
    }
    let pairs = match_pairs(&ops);
    let g = from_sequence(&ops, &pairs);
    let errs = g.validate();
    let has_engagr = ops.iter().any(|&t| t == Token::Engagr);
    let seq: Vec<&str> = ops.iter().map(|t| t.name()).collect();

    // The close condition is μ∘δ over a TRANSFORMED object (split → work → fuse),
    // not a bare cycle and not identity. closure_state encodes exactly that.
    let (verdict, gloss): (&str, String) = if !errs.is_empty() {
        (
            "F (ill-typed)",
            format!("the reasoning violates the grammar — revise:\n    ✗ {}", errs.join("\n    ✗ ")),
        )
    } else {
        match g.closure_state() {
            ClosureState::Closed(_) if has_engagr => (
                "B (paradox held)",
                "closes over a transformation, but ENGAGR engages a paradox — genuinely both \
                 (Belnap B). Sound to hold, but do not read it as a clean T; look again before \
                 an irreversible IFIX."
                    .to_string(),
            ),
            ClosureState::Closed(n) => (
                "T (closes)",
                format!(
                    "μ∘δ closes over {n} transformed reconnection(s): the decision split \
                     alternatives, did work on the arms, and the fuse resolved it. Proceed."
                ),
            ),
            ClosureState::Identity => (
                "N (identity)",
                "the arms split and refuse but nothing transforms between them — μ∘δ=id does no \
                 work and verifies nothing. Put a transformation on the arms (weigh the \
                 alternatives: EVALT/EVALF, AFWD/AREV, CLINK) before this counts as a check."
                    .to_string(),
            ),
            ClosureState::Open => (
                "B (open)",
                "well-typed but a δ fork or μ fuse dangles unreconnected: the decision opens \
                 alternatives it never rejoins. Fuse them (FFUSE) or commit one arm (IFIX)."
                    .to_string(),
            ),
            ClosureState::None => (
                "N (no fork)",
                "no δ/μ dyad — the reasoning never split alternatives, so there is no μ∘δ to \
                 close (a bare line or cycle is not a closure). For a major decision, weigh \
                 alternatives: FSPLIT → work → FFUSE."
                    .to_string(),
            ),
        }
    };

    format!(
        "IMASM check → {verdict}\n  word: {}\n  {}\n{}",
        seq.join(" "),
        gloss,
        g.classify().replace("  topology:", "  shape:") + "\n"
    )
}

// ── kernel-constrained tool registry ─────────────────────────────────────────
// The model may BUILD its own tools here, but the kernel constrains the space:
// a tool is a named IMASM program, and only a grammar-VALID composition may be
// defined. An ill-typed build is refused, so every tool in the space type-checks.

fn tools_path() -> std::path::PathBuf {
    std::path::PathBuf::from(crate::expand_user("~/imsgct/MoDoT/ob3ects/imasm_tools.json"))
}

/// Persist the registry ATOMICALLY: write a temp file, then rename. A plain
/// overwrite let a parallel reader catch the file mid-write, parse-fail, fall
/// back to {}, and persist the emptiness — 274 agent-built tools were wiped
/// that way (recovered by replaying the tool_calls log). Rename is atomic on
/// the same filesystem, so a reader now sees the old registry or the new one,
/// never a torn one.
fn save_registry(reg: &serde_json::Value) -> std::io::Result<()> {
    let path = tools_path();
    let tmp = path.with_extension("json.tmp");
    std::fs::write(&tmp, serde_json::to_string_pretty(reg).unwrap_or_default())?;
    std::fs::rename(&tmp, &path)
}

fn load_registry() -> serde_json::Value {
    let raw = std::fs::read_to_string(tools_path()).ok();
    if let Some(s) = &raw {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(s) {
            if v.is_object() {
                return v;
            }
        }
        // The file EXISTS but does not parse: that is corruption, not absence.
        // Preserve the evidence before anyone writes over it — silently
        // returning {} here is what turned one torn read into a full wipe.
        if !s.trim().is_empty() {
            let _ = std::fs::copy(tools_path(), tools_path().with_extension("json.corrupt"));
            eprintln!(
                "[imasm] registry exists but did not parse — preserved a copy at \
                 imasm_tools.json.corrupt before continuing empty"
            );
        }
    }
    serde_json::json!({})
}

fn define_tool(rest: &[String]) -> String {
    if rest.len() < 2 {
        return "define needs: imasm define <name> <op> <args…>  \
                (e.g. imasm define breath ring IMSCRIB AFWD AREV)\n"
            .into();
    }
    let name = rest[0].clone();
    let op = rest[1].to_ascii_lowercase();
    let toolargs = &rest[2..];
    let (title, g) = match build_graph(&op, toolargs) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let errs = g.validate();
    if !errs.is_empty() {
        // kernel constraint: an ill-typed composition may NOT enter the tool space
        return format!(
            "REFUSED — the kernel does not admit an ill-typed tool. '{name}' violates the grammar:\n    ✗ {}\n\
             Fix the composition so only FSPLIT branches and only FFUSE fuses, then define again.\n",
            errs.join("\n    ✗ ")
        );
    }
    // An empty program is not a tool. A malformed spec ("ring wire") built the empty
    // graph, passed validate() vacuously, and was admitted — a throne with nobody on it,
    // burning a dyad every time it is proved or run. Give the opcodes and define again:
    // e.g. imasm define <name> ring IMSCRIB AFWD AREV, or wire N0 N1 … / i-j pairs.
    if g.nodes.is_empty() {
        return format!(
            "REFUSED — '{name}' builds an EMPTY program (V=0): '{}' produced no opcode nodes, \
             so there is nothing to run and nothing the kernel could ever verify.\n  \
             Give the opcodes and define again: imasm define {name} ring IMSCRIB AFWD AREV\n",
            rest[1..].join(" ")
        );
    }
    let spec = rest[1..].join(" ");
    let mut reg = load_registry();
    reg.as_object_mut().unwrap().insert(
        name.clone(),
        serde_json::json!({
            "spec": spec,
            "title": title,
            "topology": g.classify(),
        }),
    );
    match save_registry(&reg) {
        Ok(_) => format!(
            "IMASM define → tool '{name}' admitted (grammar-valid, kernel-constrained).\n{}\n  \
             run it: imasm run {name}   |   list: imasm tools\n",
            report(title, &g).trim_end()
        ),
        Err(e) => format!("define: could not persist the tool: {e}\n"),
    }
}

/// Top-level verbs that are native code, not IMASM programs. `imasm run` must
/// point at them rather than deny them.
const NATIVE_VERBS: [&str; 14] = [
    "annihilate", "recalibrate", "homolyze", "polymerize", "distill", "close",
    "arrange", "anneal", "compare", "imscribe", "cycle", "material", "set", "complement",
];

fn run_tool(rest: &[String]) -> String {
    let Some(name) = rest.first() else {
        return "run needs a tool name: imasm run <name>  (`imasm tools` lists them)\n".into();
    };
    let reg = load_registry();
    let Some(rec) = reg.get(name.as_str()) else {
        // A native verb is not an IMASM program, but asking for it here is a
        // registry mix-up, not an absence. Say where it lives; reporting it as
        // missing has cost a real result to a false retraction.
        if NATIVE_VERBS.contains(&name.as_str()) {
            return format!(
                "run: '{name}' is a native verb, not a defined IMASM program. \
                 It is live — call it directly as `{name} <args>` (top-level tool), \
                 not through `imasm run`. `imasm run` invokes only programs made \
                 with `imasm define`.\n"
            );
        }
        let known: Vec<String> = reg.as_object().map(|o| o.keys().cloned().collect()).unwrap_or_default();
        return format!(
            "run: no tool named '{name}'. Defined tools: {}\n",
            if known.is_empty() { "(none yet — define one with `imasm define`)".into() } else { known.join(" ") }
        );
    };
    let spec = rec.get("spec").and_then(|s| s.as_str()).unwrap_or("");
    let parts: Vec<String> = spec.split_whitespace().map(|s| s.to_string()).collect();
    if parts.is_empty() {
        return format!("run: tool '{name}' has an empty spec\n");
    }
    match build_graph(&parts[0].to_ascii_lowercase(), &parts[1..]) {
        Ok((title, g)) => format!("IMASM run '{name}' (spec: {spec})\n{}", report(title, &g)),
        Err(e) => format!("run: tool '{name}' failed to rebuild: {e}"),
    }
}

/// Resolve `rest` to a graph: a defined tool name, else a raw opcode word.
/// The twelve axes in canonical tuple order — the order an entry's types compose in.
pub(crate) const TUPLE_ORDER: [&str; 12] = ["Ð","Þ","Ř","Φ","ƒ","Ç","Γ","ɢ","⊙","Ħ","Σ","Ω"];

/// A catalog entry, expanded into the IMASM program it IS.
///
/// The strange loop makes this exact: the 49 Shavian types the Grammar writes tuples with are
/// themselves full IMASM programs (`ob3ects/primitives/`), so a 12-tuple is twelve programs
/// composed in canonical order. Nothing is invented here — the entry's own glyphs name the
/// types, and each type unfolds to the opcodes it already carries.
fn expand_entry(name: &str) -> Result<(String, Vec<Token>, Vec<(usize, usize)>), String> {
    let path = crate::resolve_catalog_path().ok_or("no catalog on this host")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("read catalog: {e}"))?;
    let v: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("parse catalog: {e}"))?;
    let arr = v.as_array().ok_or("catalog is not a list")?;
    let want = name.trim().to_ascii_lowercase();
    let entry = arr
        .iter()
        .find(|e| e.get("name").and_then(|n| n.as_str()).map(|n| n.to_ascii_lowercase()) == Some(want.clone()))
        .ok_or_else(|| format!("'{name}' is not a catalog entry"))?;

    let mut ops: Vec<Token> = Vec::new();
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    let mut used: Vec<String> = Vec::new();
    for axis in TUPLE_ORDER {
        let Some(glyph) = entry.get(axis).and_then(|g| g.as_str()) else { continue };
        let Some(tname) = type_name_for_glyph(glyph) else { continue };
        let (t_ops, t_pairs, _) = expand_type(tname)?;
        let base = ops.len();
        for (a, b) in t_pairs { pairs.push((base + a, base + b)); }
        ops.extend(t_ops);
        used.push(format!("{axis}{glyph}={tname}"));
    }
    if ops.is_empty() {
        return Err(format!("'{name}' expanded to no opcodes — its glyphs are outside the 49"));
    }
    Ok((format!("entry '{name}' ({})", used.join(" ")), ops, pairs))
}

/// The catalog, loaded once: the whole entry list as parsed JSON.
pub(crate) fn catalog_entries() -> Result<Vec<serde_json::Value>, String> {
    let path = crate::resolve_catalog_path().ok_or("no catalog on this host")?;
    let text = std::fs::read_to_string(&path).map_err(|e| format!("read catalog: {e}"))?;
    let v: serde_json::Value = serde_json::from_str(&text).map_err(|e| format!("parse catalog: {e}"))?;
    v.as_array().cloned().ok_or_else(|| "catalog is not a list".to_string())
}

/// The twelve type-glyphs of a named entry, in canonical tuple order.
pub(crate) fn entry_tuple(name: &str) -> Result<Vec<String>, String> {
    let want = name.trim().to_ascii_lowercase();
    let arr = catalog_entries()?;
    let entry = arr
        .iter()
        .find(|e| e.get("name").and_then(|n| n.as_str()).map(|n| n.to_ascii_lowercase()) == Some(want.clone()))
        .ok_or_else(|| format!("'{name}' is not a catalog entry"))?;
    let mut out = Vec::new();
    for axis in TUPLE_ORDER {
        let g = entry.get(axis).and_then(|g| g.as_str())
            .ok_or_else(|| format!("'{name}' has no {axis} axis"))?;
        out.push(g.to_string());
    }
    Ok(out)
}

/// The glyph word a TUPLE is: each type-glyph unfolded into its own opcode
/// sequence, concatenated in canonical order. The same construction
/// `entry_glyph_word` performs, lifted off the catalog so an INTERMEDIATE
/// tuple — a position no entry occupies yet — has a word too.
pub(crate) fn tuple_glyph_word(tuple: &[String]) -> Result<String, String> {
    let mut ops: Vec<Token> = Vec::new();
    for g in tuple {
        let tname = type_name_for_glyph(g).ok_or_else(|| format!("glyph {g} is outside the 49"))?;
        let (t_ops, _p, _a) = expand_type(tname)?;
        ops.extend(t_ops);
    }
    Ok(ops.iter().map(|t| t.code()).collect())
}

/// The glyph word a catalog entry IS: its twelve type-glyphs unfolded into their
/// own opcode sequences, in canonical tuple order. Deterministic — the entry
/// already carries the types, so no model is asked what an object's word is.
pub fn entry_glyph_word(name: &str) -> Result<String, String> {
    let (_label, ops, _pairs) = expand_entry(name)?;
    Ok(ops.iter().map(|t| t.code()).collect())
}

/// `imasm cycle` — run primitives → imasm → primitives → imasm and measure it.
///
/// For each catalog entry: read its tuple (primitives), write the word the
/// twelve types compose to (imasm), read the word back into types (primitives),
/// write THAT tuple's word again (imasm). The cycle closes when the second word
/// equals the first and the recovered tuple equals the original. Reported
/// whole, because the interesting number is not "does it work" but WHERE the
/// alphabet stops being invertible.
fn cycle_verb(rest: &[String]) -> String {
    // `tuple=⟨…⟩` runs the cycle on ONE tuple, catalog entry or not: the way to
    // put an object through its own return leg.
    if let Some(t) = rest.iter().find_map(|a| a.strip_prefix("tuple=")) {
        return cycle_one(t.trim_matches(|c| c == '⟨' || c == '⟩'));
    }
    let limit: usize = rest
        .iter()
        .find_map(|a| a.strip_prefix("n=").and_then(|v| v.parse().ok()))
        .unwrap_or(usize::MAX);
    let arr = match catalog_entries() {
        Ok(a) => a,
        Err(e) => return format!("imasm cycle: {e}\n"),
    };
    let mut out = String::new();
    let _ = writeln!(
        out,
        "IMASM cycle — primitives → imasm → primitives → imasm, over the live catalog.\n\
         δ writes each type as its own program and concatenates; μ PARSES the word back into the \
         types that could have written it (type programs are not self-delimiting, so the word is \
         parsed, never cut on the boundary pair)."
    );
    let (mut seen, mut exact, mut ambiguous, mut failed) = (0usize, 0usize, 0usize, 0usize);
    let mut amb_axes: std::collections::BTreeMap<String, usize> = Default::default();
    let mut first_fail: Option<String> = None;
    for e in arr.iter().take(limit) {
        let Some(name) = e.get("name").and_then(|n| n.as_str()) else { continue };
        let mut tuple: Vec<String> = Vec::new();
        let mut complete = true;
        for ax in TUPLE_ORDER {
            match e.get(ax).and_then(|g| g.as_str()) {
                Some(g) => tuple.push(g.to_string()),
                None => complete = false,
            }
        }
        if !complete {
            continue;
        }
        seen += 1;
        let word = match tuple_glyph_word(&tuple) {
            Ok(w) => w,
            Err(err) => {
                failed += 1;
                first_fail.get_or_insert(format!("{name}: forward leg — {err}"));
                continue;
            }
        };
        let read = match tuple_from_word(&word) {
            Ok(r) => r,
            Err(err) => {
                failed += 1;
                first_fail.get_or_insert(format!("{name}: return leg — {err}"));
                continue;
            }
        };
        // The original must be among the pre-images at every axis, or the cycle
        // is not merely ambiguous, it is WRONG.
        let mut contains_original = true;
        let mut multi = Vec::new();
        for (i, hits) in read.iter().enumerate() {
            if !hits.contains(&tuple[i]) {
                contains_original = false;
            }
            if hits.len() > 1 {
                multi.push(TUPLE_ORDER[i]);
            }
        }
        if !contains_original {
            failed += 1;
            first_fail.get_or_insert(format!("{name}: return leg lost the original type"));
            continue;
        }
        // Second forward leg: the recovered tuple (taking the original where the
        // read is ambiguous) must write the same word back.
        let rewritten = tuple_glyph_word(&tuple).unwrap_or_default();
        if rewritten != word {
            failed += 1;
            first_fail.get_or_insert(format!("{name}: second forward leg differs"));
            continue;
        }
        if multi.is_empty() {
            exact += 1;
        } else {
            ambiguous += 1;
            for ax in multi {
                *amb_axes.entry(ax.to_string()).or_default() += 1;
            }
        }
    }
    let _ = writeln!(
        out,
        "\n{seen} entr(ies) walked · {exact} closed EXACTLY (the word names one tuple and it is \
         the original) · {ambiguous} closed to within an ambiguous axis · {failed} broke"
    );
    if !amb_axes.is_empty() {
        let _ = writeln!(
            out,
            "ambiguity is confined to: {}",
            amb_axes
                .iter()
                .map(|(a, n)| format!("axis {a} in {n} entr(ies)"))
                .collect::<Vec<_>>()
                .join(", ")
        );
        let _ = writeln!(
            out,
            "The cause is in the ALPHABET, not the reading: the 49 types emit 47 distinct \
             programs. 'loll' and 'yew' collide but sit on different axes, so position separates \
             them; 'ear' and 'tot' collide AND share axis Ř, so a Ř segment carrying that program \
             has two pre-images and no amount of reading can choose between them."
        );
    }
    if let Some(f) = first_fail {
        let _ = writeln!(out, "first break: {f}");
    }
    out
}

/// One tuple through the cycle, reported axis by axis.
fn cycle_one(tuple_glyphs: &str) -> String {
    let tuple: Vec<String> = tuple_glyphs.chars().map(|c| c.to_string()).collect();
    if tuple.len() != 12 {
        return format!("imasm cycle: a tuple is twelve glyphs; this is {}\n", tuple.len());
    }
    let mut out = String::new();
    let word = match tuple_glyph_word(&tuple) {
        Ok(w) => w,
        Err(e) => return format!("imasm cycle: forward leg — {e}\n"),
    };
    let read = match tuple_from_word(&word) {
        Ok(r) => r,
        Err(e) => return format!("imasm cycle: return leg — {e}\n"),
    };
    let _ = writeln!(out, "IMASM cycle on one tuple — written, then read back.\n");
    let (mut exact, mut amb) = (0usize, 0usize);
    for (i, hits) in read.iter().enumerate() {
        let ax = TUPLE_ORDER[i];
        let orig = type_name_for_glyph(&tuple[i]).unwrap_or("?");
        let names: Vec<&str> = hits.iter().map(|g| type_name_for_glyph(g).unwrap_or("?")).collect();
        let kept = hits.contains(&tuple[i]);
        if hits.len() == 1 && kept {
            exact += 1;
            let _ = writeln!(out, "  {ax}  {orig}  → recovered exactly");
        } else if kept {
            amb += 1;
            let _ = writeln!(out, "  {ax}  {orig}  → ambiguous: {}", names.join(" or "));
        } else {
            let _ = writeln!(out, "  {ax}  {orig}  → LOST: read as {}", names.join(" or "));
        }
    }
    let _ = writeln!(
        out,
        "\n{exact} of 12 axes recovered exactly, {amb} ambiguous. The word is {} opcodes long.",
        word.chars().count()
    );
    out
}

/// THE RETURN LEG: an IMASM word read back into the twelve primitive types it
/// was written from. The forward leg (type → program, tuple → word) was always
/// here; this is the μ to that δ, and running both closes the cycle
/// primitives → imasm → primitives → imasm.
///
/// The read is exact wherever it can be: a word is twelve ⊢…⊣ segments, each
/// segment is some type's own program, and the axis a segment sits at restricts
/// which of the 49 types could have written it. The cycle is NOT total, and the
/// obstruction is precise: the 49 types emit only 47 distinct programs. `loll`
/// and `yew` collide but occupy different axes (Ç and Φ), so position separates
/// them; `ear` and `tot` collide AND share axis Ř, so a Ř segment reading as
/// that program has two pre-images and the return leg reports BOTH rather than
/// choosing. That ambiguity is a fact about the type alphabet, not a defect
/// here, and it is why the cycle closes to within one axis and not exactly.
pub(crate) fn tuple_from_word(word: &str) -> Result<Vec<Vec<String>>, String> {
    // Type programs are NOT self-delimiting: `out` carries TANCH in its middle
    // and does not end on ⊣, so the boundary pair cannot cut a concatenated
    // word into its twelve types. The word must be PARSED instead — axis by
    // axis, each axis consuming some admissible type's program as a prefix —
    // and every parse that consumes the whole word is a genuine pre-image.
    let w: Vec<char> = word.chars().collect();
    let mut table: Vec<(&str, &str, Vec<char>)> = Vec::new(); // (glyph, name, program)
    for (g, tname) in GLYPH_TYPE_NAME {
        if let Ok((ops, _p, _a)) = expand_type(tname) {
            let prog: Vec<char> = ops.iter().flat_map(|t| t.code().chars()).collect();
            table.push((g, tname, prog));
        }
    }
    let mut parses: Vec<Vec<String>> = Vec::new();
    let mut stack: Vec<String> = Vec::new();
    parse_axis(&w, 0, 0, &table, &mut stack, &mut parses);
    if parses.is_empty() {
        return Err("no reading of this word as twelve type programs".into());
    }
    // Collapse the parses into per-axis pre-image sets: the axis is determined
    // wherever every parse agrees, ambiguous where they do not.
    let mut out: Vec<Vec<String>> = vec![Vec::new(); 12];
    for parse in &parses {
        for (i, g) in parse.iter().enumerate() {
            if !out[i].contains(g) {
                out[i].push(g.clone());
            }
        }
    }
    Ok(out)
}

/// Depth-first parse: consume one admissible type program per axis, in order.
fn parse_axis(
    w: &[char],
    pos: usize,
    axis: usize,
    table: &[(&str, &str, Vec<char>)],
    stack: &mut Vec<String>,
    parses: &mut Vec<Vec<String>>,
) {
    if axis == 12 {
        if pos == w.len() {
            parses.push(stack.clone());
        }
        return;
    }
    // Guard against a blow-up on a pathological word; the honest reading of a
    // catalog tuple never needs more than a handful of parses.
    if parses.len() >= 64 {
        return;
    }
    let ax = TUPLE_ORDER[axis];
    for (g, _name, prog) in table {
        if !axis_admits(ax, g) {
            continue;
        }
        if pos + prog.len() > w.len() || &w[pos..pos + prog.len()] != prog.as_slice() {
            continue;
        }
        stack.push(g.to_string());
        parse_axis(w, pos + prog.len(), axis + 1, table, stack, parses);
        stack.pop();
    }
}

/// Whether an axis is ever written with a given type-glyph, read off the live
/// catalog. This is what makes position load-bearing in the return leg: the
/// alphabet is 49 types, but each axis draws from its own subset.
fn axis_admits(axis: &str, glyph: &str) -> bool {
    use std::collections::HashSet;
    use std::sync::OnceLock;
    static SEEN: OnceLock<HashSet<String>> = OnceLock::new();
    let seen = SEEN.get_or_init(|| {
        let mut set = HashSet::new();
        if let Ok(arr) = catalog_entries() {
            for e in &arr {
                for ax in TUPLE_ORDER {
                    if let Some(g) = e.get(ax).and_then(|g| g.as_str()) {
                        set.insert(format!("{ax}{g}"));
                    }
                }
            }
        }
        set
    });
    // An empty catalog must not silently reject everything.
    seen.is_empty() || seen.contains(&format!("{axis}{glyph}"))
}

fn resolve_graph(rest: &[String]) -> Result<(String, Graph), String> {
    if let Some(first) = rest.first() {
        let reg = load_registry();
        if let Some(rec) = reg.get(first.as_str()) {
            let spec = rec.get("spec").and_then(|s| s.as_str()).unwrap_or("");
            let parts: Vec<String> = spec.split_whitespace().map(|s| s.to_string()).collect();
            if let Some((_t, g)) = parts.first().and_then(|op| build_graph(&op.to_ascii_lowercase(), &parts[1..]).ok()) {
                return Ok((format!("tool '{first}' ({spec})"), g));
            }
        }
    }
    let ops = tok_list(rest);
    if ops.is_empty() {
        // Not a tool, not an opcode word — is it a catalog entry? Then it IS a program.
        if let Some(first) = rest.first() {
            match expand_entry(first) {
                Ok((label, e_ops, e_pairs)) => return Ok((label, from_sequence(&e_ops, &e_pairs))),
                Err(e) => {
                    return Err(format!(
                        "prove needs a tool name, an opcode word, or a catalog entry: \
                         imasm prove <name|entry|T1 T2 …>\n  ({e})\n"
                    ))
                }
            }
        }
        return Err("prove needs a tool name, an opcode word, or a catalog entry: imasm prove <name|entry|T1 T2 …>\n".into());
    }
    let pairs = match_pairs(&ops);
    Ok(("word".into(), from_sequence(&ops, &pairs)))
}

/// Authoritative check: take the pre-filter's closure verdict to the ACTUAL
/// p4ramill kernel. A genuine transform-closure corresponds to B bifurcating into
/// distinct (T,F) arms and fusing back (B_is_the_only_bifurcation_point +
/// split_fuse_id); an identity closure corresponds to a diagonal copy (s,s). We
/// generate the matching theorem and `lake build` it against the real library.
fn prove_tool(rest: &[String]) -> String {
    let (label, g) = match resolve_graph(rest) {
        Ok(v) => v,
        Err(e) => return e,
    };
    let state = g.closure_state();
    let (kind, source) = match state {
        ClosureState::Closed(_) => (
            "genuine transform-closure (split bifurcates, then fuses back)",
            "import Imscribing.IGFunctor\n\
             import Imscribing.Paraconsistent.BelnapSplitFuse\n\
             open Belnap\n\
             -- the closure carries a transformation: B splits to DISTINCT arms (T,F)\n\
             -- and fuses back (μ∘δ=id). Not a diagonal identity copy.\n\
             theorem imasm_genuine_closure :\n\
             \x20   (fsplit Belnap.B).1 ≠ (fsplit Belnap.B).2 ∧ ffuse (fsplit Belnap.B) = Belnap.B := by\n\
             \x20 exact ⟨by decide, split_fuse_id _⟩\n"
                .to_string(),
        ),
        ClosureState::Identity => (
            "identity closure (diagonal copy — no transformation)",
            "import Imscribing.IGFunctor\n\
             import Imscribing.Paraconsistent.BelnapSplitFuse\n\
             open Belnap\n\
             -- the closure is a diagonal copy: fsplit of a non-B state gives (s,s),\n\
             -- identical arms. μ∘δ=id does no work — the kernel confirms it type-checks nothing.\n\
             theorem imasm_identity_closure :\n\
             \x20   (fsplit Belnap.T).1 = (fsplit Belnap.T).2 ∧ ffuse (fsplit Belnap.T) = Belnap.T := by\n\
             \x20 exact ⟨rfl, split_fuse_id _⟩\n"
                .to_string(),
        ),
        ClosureState::Open => {
            return format!(
                "IMASM prove — {label}: pre-filter says OPEN (a δ or μ dangles). There is no μ∘δ \
                 closure to take to the kernel yet. Reconnect the fork at a fuse first.\n"
            )
        }
        ClosureState::None => {
            return format!(
                "IMASM prove — {label}: pre-filter says NONE (no δ/μ dyad; a bare line/cycle is not \
                 a closure). Nothing for the kernel to verify.\n"
            )
        }
    };
    let (green, log) = crate::prover::compile_lean(&source, "A");
    let tail: String = log.lines().rev().take(4).collect::<Vec<_>>().into_iter().rev().collect::<Vec<_>>().join("\n    ");
    // A registry-resolved prove speaks about the spec REGISTERED under this name — never
    // about a later define of the same name that was refused. Seen live: a refused
    // re-define followed by `prove <name>` returned the OLD program's green and the agent
    // read it as verification of the refused word. Say which program the verdict binds.
    let spec_note = if label.starts_with("tool '") {
        "\n  (this verdict binds the REGISTERED spec named above — a define of this name \
         that was REFUSED is not part of it)"
    } else {
        ""
    };
    if matches!(state, ClosureState::Identity) {
        // The Identity theorem proves the diagonal does NO work. That is a true kernel
        // fact, but it is NOT an affirmation of the program — printing it as "✓ green"
        // let trivial ◇● dyads be harvested as verifications (seen live, repeatedly).
        // The program's own verdict is N: nothing was type-checked by its closure.
        return format!(
            "IMASM prove — {label}\n  pre-filter: {kind}\n  KERNEL VERDICT for THIS program: N \
             (void) — μ∘δ=id does no work here; the arms carry no transform, so the closure \
             type-checks NOTHING. (The kernel {} the diagonal identity theorem, which states \
             exactly that.){spec_note}\n  Put a transforming opcode between the fork and the \
             fuse and prove again.\n",
            if green { "confirms" } else { "did not even confirm" }
        );
    }
    if green {
        format!(
            "IMASM prove — {label}\n  pre-filter: {kind}\n  KERNEL VERDICT: ✓ green — p4ramill \
             confirms the closure class against BelnapSplitFuse (B_is_the_only_bifurcation_point, \
             split_fuse_id).{spec_note}\n"
        )
    } else {
        format!(
            "IMASM prove — {label}\n  pre-filter: {kind}\n  KERNEL VERDICT: ✗ the lake build did not \
             go green.\n    {tail}{spec_note}\n"
        )
    }
}

// ── operational semantics: values through the gates ─────────────────────────
// Until now the engine judged SHAPE only; `imasm eval` shuttles VALUES. One
// evaluator over the SIXTEEN_3 carrier (P({T,F,t,f}), Shramko/Dunn/Takenaka);
// FOUR is its classical slice {T,F} — B={T,F}, N={} — so `eval` (FOUR render)
// and `eval16` are the same machine with two readouts. Gate table:
//   VINIT emits the seed · FSPLIT δ fans truth-part/falsity-part onto its arms
//   EVALT/EVALF pass-gates (truth/falsity projection) · FFUSE μ joins (union)
//   AREV the involution T↔F,t↔f · IFIX latches · TANCH reads out
//   AFWD/CLINK/IMSCRIB/ENGAGR carry. Every gate is ⊆-monotone, so the Kleene
// iteration from all-N edge values reaches its fixpoint in ≤4·E rounds even on
// cyclic graphs — a loop settles, it cannot oscillate.

type Val = crate::imasm16_3::Reg16_3;

fn gate_out(tok: Token, x: Val, seed: Val, slot: usize) -> Val {
    match tok {
        Token::Vinit => seed,
        Token::Fsplit => {
            if slot == 0 { x.truth_part() } else { x.falsity_part() }
        }
        Token::Evalt => x.truth_part(),
        Token::Evalf => x.falsity_part(),
        Token::Arev => x.invol(),
        // FFUSE's join already happened at input aggregation; carriers carry.
        _ => x,
    }
}

/// The flow core shared by eval and chaos: Kleene-iterate values over the
/// graph, return (per-node inputs, per-edge values). Monotone gates, so this
/// settles in ≤4·E rounds regardless of cycles.
fn flow_values(g: &Graph, seed: Val) -> (Vec<Val>, Vec<Val>) {
    let mut edge_val: Vec<Val> = vec![Val::default(); g.edges.len()];
    let mut node_in: Vec<Val> = vec![Val::default(); g.nodes.len()];
    let cap = 4 * g.edges.len().max(1) + 4;
    for _ in 0..cap {
        let mut changed = false;
        for i in 0..g.nodes.len() {
            let mut inp = Val::default();
            for (eidx, &(_a, b)) in g.edges.iter().enumerate() {
                if b == i {
                    inp = inp.union(edge_val[eidx]);
                }
            }
            node_in[i] = inp;
            let mut slot = 0usize;
            for (eidx, &(a, _b)) in g.edges.iter().enumerate() {
                if a == i {
                    let v = gate_out(g.nodes[i], inp, seed, slot);
                    if edge_val[eidx] != v {
                        edge_val[eidx] = v;
                        changed = true;
                    }
                    slot += 1;
                }
            }
        }
        if !changed {
            break;
        }
    }
    (node_in, edge_val)
}

/// Dyad flow verdicts: for each δ/μ pair, did the fuse recover the fork's feed?
fn dyad_signature(g: &Graph, node_in: &[Val]) -> (usize, usize) {
    let (pairs, _fully) = g.frobenius_closures();
    let total = pairs.len();
    let id = pairs.iter().filter(|&&(f, j)| node_in[j] == node_in[f]).count();
    (id, total)
}

fn eval_tool(rest: &[String], sixteen: bool) -> String {
    // optional trailing seed=<name> (FOUR names N/T/F/B or 16_3 names N/A/TFtf combos)
    let mut args: Vec<String> = rest.to_vec();
    let mut seed = if sixteen {
        Val::from_name("A").unwrap()
    } else {
        Val::from_name("TF").unwrap() // B in the classical slice
    };
    if let Some(pos) = args.iter().position(|a| a.starts_with("seed=")) {
        let sname = args.remove(pos);
        let sname = &sname["seed=".len()..];
        let parsed = match sname {
            "B" => Val::from_name("TF"),
            other => Val::from_name(other),
        };
        match parsed {
            Some(v) => seed = v,
            None => return format!("eval: unknown seed '{sname}' — use N/T/F/B or a 16_3 name (A, Tf, TFtf …)\n"),
        }
    }
    let (label, g) = match resolve_graph(&args) {
        Ok(v) => v,
        Err(e) => return e,
    };
    if g.nodes.is_empty() {
        return "eval: empty program — nothing to flow.\n".into();
    }
    let render = |v: Val| if sixteen { v.name() } else { v.four_name() };

    let (node_in, edge_val) = flow_values(&g, seed);

    let mut out = format!(
        "IMASM eval — {label}  [carrier: {}; seed {}]\n",
        if sixteen { "SIXTEEN_3 = P({T,F,t,f})" } else { "FOUR (the classical slice of SIXTEEN_3)" },
        render(seed)
    );
    for i in 0..g.nodes.len() {
        let tok = g.nodes[i];
        let outs: Vec<String> = g
            .edges
            .iter()
            .enumerate()
            .filter(|(_, &(a, _))| a == i)
            .map(|(eidx, _)| render(edge_val[eidx]))
            .collect();
        let shown = if outs.is_empty() {
            render(gate_out(tok, node_in[i], seed, 0))
        } else {
            outs.join(",")
        };
        out.push_str(&format!(
            "  {:>3} {} {:<8} in {:>4} → out {}\n",
            i,
            tok.code(),
            tok.name(),
            render(node_in[i]),
            shown
        ));
    }
    // readout: sinks (no outgoing edges)
    let sinks: Vec<String> = (0..g.nodes.len())
        .filter(|&i| g.out_degree(i) == 0)
        .map(|i| format!("{}({})", g.nodes[i].name(), render(gate_out(g.nodes[i], node_in[i], seed, 0))))
        .collect();
    if !sinks.is_empty() {
        out.push_str(&format!("  readout at sink(s): {}\n", sinks.join("  ")));
    }
    // operational μ∘δ: for each δ/μ pair, does the fuse RECOVER the fork's input?
    let (pairs, _fully) = g.frobenius_closures();
    for (f, j) in pairs {
        let recovered = node_in[j];
        let fed = node_in[f];
        out.push_str(&format!(
            "  μ∘δ on values: FSPLIT@{f} fed {} → FFUSE@{j} recovered {} — {}\n",
            render(fed),
            render(recovered),
            if recovered == fed { "id (lossless split-fuse)" } else { "NOT id (value lost or gained on the arms)" }
        ));
    }
    out
}

// ── the composition law: programs interact end-to-valence ───────────────────
// Two programs compose ONLY by binding a living out-end of A to a living in-end
// of B. Composition CONSUMES valences and may never mint one; the composite must
// re-satisfy the grammar. A program with no living ends is finished — a loop —
// and does not compose: building is the act of consuming reactive ends.
/// The bind core: merge B into A end-to-valence (A's free out-ends to B's free
/// in-ends, node order, deterministic). Err carries the refusal reason. Used by
/// compose (one binding, registered) and chaos (the whole state space).
fn merge_bind(ga: &Graph, gb: &Graph) -> Result<(Graph, Vec<(usize, usize)>), String> {
    let outs_a: Vec<usize> = (0..ga.nodes.len())
        .flat_map(|i| {
            let free = ga.nodes[i].arity().1.saturating_sub(ga.out_degree(i));
            std::iter::repeat(i).take(free)
        })
        .collect();
    let ins_b: Vec<usize> = (0..gb.nodes.len())
        .flat_map(|i| {
            let free = gb.nodes[i].arity().0.saturating_sub(gb.in_degree(i));
            std::iter::repeat(i).take(free)
        })
        .collect();
    if outs_a.is_empty() || ins_b.is_empty() {
        return Err(format!(
            "{} offers no living end ({} free out, {} free in)",
            if outs_a.is_empty() { "A" } else { "B" },
            outs_a.len(),
            ins_b.len()
        ));
    }
    let k = outs_a.len().min(ins_b.len());
    let off = ga.nodes.len();
    let mut g = Graph::new();
    for &t in &ga.nodes {
        g.add(t);
    }
    for &t in &gb.nodes {
        g.add(t);
    }
    for &(a, b) in &ga.edges {
        g.connect(a, b);
    }
    for &(a, b) in &gb.edges {
        g.connect(a + off, b + off);
    }
    let bindings: Vec<(usize, usize)> = outs_a[..k]
        .iter()
        .zip(ins_b[..k].iter())
        .map(|(&a, &b)| (a, b + off))
        .collect();
    for &(a, b) in &bindings {
        g.connect(a, b);
    }
    let errs = g.validate();
    if !errs.is_empty() {
        return Err(format!("ill-typed composite: {}", errs.join("; ")));
    }
    Ok((g, bindings))
}

// ── ChaosComposer: the possibility state space of program interaction ───────
// Feed a SET of programs; a set has no order and no chosen binding, so the
// composer walks every ordering (left-fold end-to-valence binding) and speaks
// the whole space: which arrangements are admitted, which are refused and why,
// and what each admitted arrangement IS under all three verdicts (grammar by
// construction, closure state, flow). Arrangements collapse into outcome
// classes — distinct (topology, closure, flow) results — because the space of
// possibilities is smaller than the space of orderings, and THAT collapse is
// the measurement.
fn chaos_tool(rest: &[String]) -> String {
    if rest.len() < 2 {
        return "chaos needs ≥2 programs: imasm chaos <A> <B> [C…]  \
                (walks every ordering, folds each by end-to-valence binding, and reports \
                the possibility state space: admitted arrangements, refusals, outcome classes)\n"
            .into();
    }
    if rest.len() > 6 {
        return format!(
            "chaos caps at 6 programs ({} given): 7! = 5040 orderings stops being a state \
             space and starts being noise. Compose a sub-space first, then chaos the composites.\n",
            rest.len()
        );
    }
    // resolve every program once
    let mut graphs: Vec<(String, Graph)> = Vec::new();
    for name in rest {
        match resolve_graph(&[name.clone()]) {
            Ok((_, g)) => graphs.push((name.clone(), g)),
            Err(e) => return format!("chaos: '{name}' did not resolve — {e}"),
        }
    }
    // permutations by Heap's algorithm over indices
    let n = graphs.len();
    let mut idx: Vec<usize> = (0..n).collect();
    let mut perms: Vec<Vec<usize>> = Vec::new();
    fn heap(k: usize, a: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if k == 1 {
            out.push(a.clone());
            return;
        }
        for i in 0..k {
            heap(k - 1, a, out);
            if k % 2 == 0 {
                a.swap(i, k - 1);
            } else {
                a.swap(0, k - 1);
            }
        }
    }
    heap(n, &mut idx, &mut perms);

    let seed = Val::from_name("TF").unwrap();
    // outcome class key → (representative ordering, count, closure kind, flow line)
    use std::collections::BTreeMap;
    let mut classes: BTreeMap<String, (String, usize)> = BTreeMap::new();
    let mut refusals: BTreeMap<String, usize> = BTreeMap::new();
    let mut admitted = 0usize;
    for p in &perms {
        let order: Vec<&str> = p.iter().map(|&i| graphs[i].0.as_str()).collect();
        let mut acc = graphs[p[0]].1.clone();
        let mut failed: Option<String> = None;
        for &i in &p[1..] {
            match merge_bind(&acc, &graphs[i].1) {
                Ok((g, _)) => acc = g,
                Err(e) => {
                    failed = Some(e);
                    break;
                }
            }
        }
        match failed {
            Some(e) => {
                *refusals.entry(e).or_insert(0) += 1;
            }
            None => {
                admitted += 1;
                let closure = match acc.closure_state() {
                    ClosureState::Closed(k) => format!("CLOSED({k} worked dyad(s))"),
                    ClosureState::Identity => "IDENTITY (dyad does no work)".into(),
                    ClosureState::Open => "OPEN (a δ or μ dangles)".into(),
                    ClosureState::None => "NONE (no dyad)".into(),
                };
                let (node_in, _) = flow_values(&acc, seed);
                let (id, total) = dyad_signature(&acc, &node_in);
                let sinks: Vec<String> = (0..acc.nodes.len())
                    .filter(|&i| acc.out_degree(i) == 0)
                    .map(|i| gate_out(acc.nodes[i], node_in[i], seed, 0).four_name())
                    .collect();
                let living: usize = (0..acc.nodes.len())
                    .map(|i| {
                        let (ain, aout): (usize, usize) = acc.nodes[i].arity();
                        ain.saturating_sub(acc.in_degree(i)) + aout.saturating_sub(acc.out_degree(i))
                    })
                    .sum();
                let topo_line = acc
                    .classify()
                    .lines()
                    .next()
                    .unwrap_or("")
                    .trim()
                    .trim_start_matches("topology:")
                    .trim()
                    .to_string();
                let key = format!(
                    "{topo_line} | {closure} | flow {id}/{total} id | readout [{}] | {} living end(s)",
                    sinks.join(","),
                    living
                );
                let entry = classes.entry(key).or_insert((order.join(" ∘ "), 0));
                entry.1 += 1;
            }
        }
    }

    let mut out = format!(
        "IMASM chaos — possibility state space of {{{}}}\n  orderings walked: {}  ·  admitted: {}  ·  refused: {}\n  \
         outcome classes: {} (the space of possibilities, after the orderings collapse)\n",
        rest.join(", "),
        perms.len(),
        admitted,
        perms.len() - admitted,
        classes.len()
    );
    for (key, (repr, count)) in &classes {
        out.push_str(&format!("  ◆ {count} ordering(s) → {key}\n      e.g. {repr}\n"));
    }
    for (why, count) in &refusals {
        out.push_str(&format!("  ✗ {count} ordering(s) refused — {why}\n"));
    }
    out.push_str(
        "  mint one: imasm compose <name> <A> <B> (then compose the result onward), \
         or wire the exact arrangement.\n",
    );
    out
}

fn compose_tool(rest: &[String]) -> String {
    if rest.len() < 3 {
        return "compose needs: imasm compose <new_name> <tool_A> <tool_B>  \
                (binds A's living out-ends to B's living in-ends, in order)\n"
            .into();
    }
    let new_name = rest[0].clone();
    let (label_a, ga) = match resolve_graph(&rest[1..2].to_vec()) {
        Ok(v) => v,
        Err(e) => return format!("compose: A did not resolve — {e}"),
    };
    let (label_b, gb) = match resolve_graph(&rest[2..3].to_vec()) {
        Ok(v) => v,
        Err(e) => return format!("compose: B did not resolve — {e}"),
    };
    let (g, bindings) = match merge_bind(&ga, &gb) {
        Ok(v) => v,
        Err(e) => {
            return format!(
                "compose REFUSED — {e}. Composition consumes living ends and never mints one; \
                 a program with no reactive ends is a finished loop, it does not compose, it is \
                 DONE. ({label_a} ∘ {label_b})\n"
            )
        }
    };
    let k = bindings.len();
    // persist as a wire spec so it rebuilds through the identical parse
    let toks: Vec<&str> = g.nodes.iter().map(|t| t.name()).collect();
    let edges: Vec<String> = g.edges.iter().map(|&(a, b)| format!("{a}-{b}")).collect();
    let spec = format!("wire {} / {}", toks.join(" "), edges.join(" "));
    let mut reg = load_registry();
    reg.as_object_mut().unwrap().insert(
        new_name.clone(),
        serde_json::json!({
            "spec": spec,
            "title": format!("compose {} ∘ {}", rest[1], rest[2]),
            "topology": g.classify(),
        }),
    );
    if let Err(e) = save_registry(&reg) {
        return format!("compose: could not persist '{new_name}': {e}\n");
    }
    let bind_str: Vec<String> = bindings
        .iter()
        .map(|&(a, b)| format!("{}@{a} → {}@{b}", g.nodes[a].name(), g.nodes[b].name()))
        .collect();
    format!(
        "IMASM compose → '{new_name}' admitted: {} ∘ {} bound at {} end(s).\n  \
         bindings: {}\n  law: {} valence(s) consumed, 0 minted.\n{}\n  \
         flow it: imasm eval {new_name}   |   verify: imasm prove {new_name}\n",
        rest[1],
        rest[2],
        k,
        bind_str.join("  ·  "),
        2 * k,
        report("compose", &g).trim_end()
    )
}

// ── manifest export: the registry with its graphs, for the composer surface ──
// The visual composer (imasm_composer.html) must never re-derive the grammar in
// a second language; it renders THIS manifest, and every judgment stays here.
fn export_tools() -> String {
    let reg = load_registry();
    let Some(obj) = reg.as_object() else {
        return "export: registry unreadable\n".into();
    };
    let mut items: Vec<serde_json::Value> = Vec::new();
    for (name, rec) in obj {
        let spec = rec.get("spec").and_then(|s| s.as_str()).unwrap_or("");
        let parts: Vec<String> = spec.split_whitespace().map(|s| s.to_string()).collect();
        let Some((_t, g)) = parts
            .first()
            .and_then(|op| build_graph(&op.to_ascii_lowercase(), &parts[1..]).ok())
        else {
            continue;
        };
        let free_in: usize = (0..g.nodes.len())
            .map(|i| g.nodes[i].arity().0.saturating_sub(g.in_degree(i)))
            .sum();
        let free_out: usize = (0..g.nodes.len())
            .map(|i| g.nodes[i].arity().1.saturating_sub(g.out_degree(i)))
            .sum();
        let closure = match g.closure_state() {
            ClosureState::Closed(k) => format!("CLOSED({k})"),
            ClosureState::Identity => "IDENTITY".into(),
            ClosureState::Open => "OPEN".into(),
            ClosureState::None => "NONE".into(),
        };
        items.push(serde_json::json!({
            "name": name,
            "spec": spec,
            "nodes": g.nodes.iter().map(|t| t.name()).collect::<Vec<_>>(),
            "codes": g.nodes.iter().map(|t| t.code()).collect::<Vec<_>>(),
            "edges": g.edges,
            "free_in": free_in,
            "free_out": free_out,
            "closure": closure,
        }));
    }
    let manifest = serde_json::json!({
        "generated_by": "imasm export",
        "opcodes": Token::parse_all_names(),
        "tools": items,
    });
    let path = crate::expand_user("~/imsgct/MoDoT/ob3ects/imasm_manifest.json");
    match std::fs::write(&path, serde_json::to_string_pretty(&manifest).unwrap_or_default()) {
        Ok(_) => format!(
            "IMASM export → {} tool(s) manifested with graphs, ports, and closure states.\n  \
             The composer surface reads this file; the kernel stays the only judge.\n{}",
            manifest["tools"].as_array().map(|a| a.len()).unwrap_or(0),
            embed_manifest_in_surface(&manifest),
        ),
        Err(e) => format!("export: could not write manifest: {e}\n"),
    }
}

/// Splice the manifest INTO the composer surface, between the page's
/// `<script type="application/json" id="manifest">` markers, so `file://`
/// opens work with no server and no picker: the page carries its own data.
/// `</` is escaped to `<\/` so the JSON can never close its own script tag.
fn embed_manifest_in_surface(manifest: &serde_json::Value) -> String {
    let page = crate::expand_user("~/imsgct/MoDoT/imasm_composer.html");
    let Ok(html) = std::fs::read_to_string(&page) else {
        return "  (composer surface not found; manifest not embedded)\n".into();
    };
    const OPEN: &str = r#"<script type="application/json" id="manifest">"#;
    const CLOSE: &str = "</script>";
    let Some(a) = html.find(OPEN) else {
        return "  (composer surface has no inline-manifest block; manifest not embedded)\n".into();
    };
    let body_start = a + OPEN.len();
    let Some(rel) = html[body_start..].find(CLOSE) else {
        return "  (composer surface inline-manifest block unterminated; manifest not embedded)\n".into();
    };
    let json = serde_json::to_string(manifest).unwrap_or_default().replace("</", r"<\/");
    let new_html = format!("{}{}{}", &html[..body_start], json, &html[body_start + rel..]);
    match std::fs::write(&page, new_html) {
        Ok(_) => "  Manifest also embedded into imasm_composer.html — open it directly (file://), \
                  no server needed.\n"
            .into(),
        Err(e) => format!("  (could not embed manifest into the surface: {e})\n"),
    }
}

fn list_tools() -> String {
    let reg = load_registry();
    let obj = reg.as_object();
    if obj.map(|o| o.is_empty()).unwrap_or(true) {
        return "IMASM tools: none defined yet. Build one (kernel-constrained): \
                imasm define <name> <op> <args…>\n"
            .into();
    }
    let mut s = String::from("IMASM tools (kernel-constrained space — every one type-checks):\n");
    for (name, rec) in obj.unwrap() {
        let spec = rec.get("spec").and_then(|v| v.as_str()).unwrap_or("");
        let topo = rec
            .get("topology")
            .and_then(|v| v.as_str())
            .and_then(|t| t.lines().next())
            .unwrap_or("")
            .trim();
        let _ = writeln!(s, "  {name}: {spec}\n    {topo}");
    }
    s.push_str("Run any: imasm run <name>\n");
    s
}

/// ROTAT — the first op-opcode: an operator ON a word, not a token IN it. It rotates
/// the whole word cyclically by k (default 1): the ring automorphism, the Weyl-Heisenberg
/// shift X on ℤ/dℤ. Appending its name as a token does nothing — it is not a node. Every
/// spectral invariant is ROTAT-invariant, and that invariance IS the signal it is a symmetry
/// (not that it is inert): on one ring nothing measurable moves; between two rings being bound
/// it sets their relative phase. A balanced tiling of a period-n cycle is unique up to ROTAT.
fn rotat_op(rest: &[String]) -> String {
    let (k, word_args): (i64, &[String]) = match rest.first() {
        Some(first) if first.trim().parse::<i64>().is_ok() => {
            (first.trim().parse::<i64>().unwrap(), &rest[1..])
        }
        _ => (1, rest),
    };
    let toks = tok_list(word_args);
    if toks.is_empty() {
        return "rotat needs a word: imasm rotat [k] <word>. ROTAT is an OP-OPCODE — an operator on \
                the whole word, not one of the 12 node-opcodes; appending its name as a token does \
                nothing. It rotates the word by k (default 1): the ring's cyclic shift.\n"
            .into();
    }
    let n = toks.len() as i64;
    let shift = (((k % n) + n) % n) as usize;
    let mut rotated: Vec<Token> = Vec::with_capacity(toks.len());
    rotated.extend_from_slice(&toks[shift..]);
    rotated.extend_from_slice(&toks[..shift]);

    let g0 = ring(&toks);
    let g1 = ring(&rotated);
    let (r0, r1) = (g0.spectral_radius(), g1.spectral_radius());
    let invariant = (r0 - r1).abs() < 1e-9;

    let mut s = String::new();
    let _ = writeln!(s, "IMASM rotat (op-opcode ROTAT — the cyclic shift of a ring, k={k})");
    let _ = writeln!(s, "  in : {}", g0.code_str());
    let _ = writeln!(s, "  out: {}", g1.code_str());
    let _ = writeln!(
        s,
        "  ρ: {r0:.4} → {r1:.4}  ({})",
        if invariant {
            "INVARIANT — ROTAT is a symmetry of the ring, not a transformation of it"
        } else {
            "changed"
        }
    );
    let _ = writeln!(
        s,
        "  ROTAT is an operator ON the word, not a node IN it (an op-opcode). It is the \
         Weyl-Heisenberg shift X on ℤ/dℤ: on one ring every spectral invariant is preserved, and \
         between two rings being bound it sets their relative phase (the degree of freedom that \
         seats a junction two same-handed rings cannot close alone). A balanced tiling is unique \
         up to ROTAT."
    );
    s
}

pub fn run(args: &[String]) -> String {
    if args.is_empty() {
        return format!("{REFERENCE}\n");
    }
    let op = args[0].to_ascii_lowercase();
    let rest = &args[1..];
    match op.as_str() {
        "ref" | "reference" | "help" | "rules" => format!("{REFERENCE}\n"),
        "rotat" | "rotate" | "shift" => rotat_op(rest),
        "arev" | "hop" | "door" => crate::arev::run(rest),
        "check" | "typecheck" => verdict_check(rest),
        "define" | "forge_tool" => define_tool(rest),
        "run" | "invoke" => run_tool(rest),
        "prove" | "kernel" => prove_tool(rest),
        "eval" | "flow" => eval_tool(rest, false),
        "eval16" | "flow16" => eval_tool(rest, true),
        // The trilattice sibling, CLI-reachable: `--imasm 16_3 check ⊢∈…∋⊣`.
        "16_3" | "tri" | "imasm16_3" => crate::imasm16_3::run(rest),
        "learn" | "study" => crate::learn::run(rest),
        "path" | "promote" => crate::learn::path(rest),
        "cycle" => cycle_verb(rest),
        "compose" | "bind" => compose_tool(rest),
        "chaos" | "space" => chaos_tool(rest),
        "export" | "manifest" => export_tools(),
        "tools" => list_tools(),
        "chain" | "ring" | "cycle" | "loop" | "protocol" | "seq" | "sequence" | "classify"
        | "read" | "wire" | "graph" | "free" | "star" | "bubble" | "fork" | "comb" | "graft" => {
            match build_graph(&op, rest) {
                Ok((title, g)) => report(title, &g),
                Err(e) => e,
            }
        }
        "types" | "list" => {
            let ts = list_types();
            format!(
                "IMASM types ({} — the Shavian names the Grammar writes tuples with; each is a \
                 full IMASM program):\n  {}\nExpand any one: imasm expand <name>\n",
                ts.len(),
                ts.join(" ")
            )
        }
        "expand" | "unfold" => {
            let Some(name) = rest.first() else {
                return "expand needs a type name: imasm expand <name> (e.g. imasm expand ado). \
                        `imasm types` lists them.\n"
                    .into();
            };
            match expand_type(name) {
                Err(e) => format!("{e}\n"),
                Ok((ops, pairs, actions)) => {
                    let g = from_sequence(&ops, &pairs);
                    let seq: Vec<&str> = ops.iter().map(|t| t.name()).collect();
                    let mut s = String::new();
                    let _ = writeln!(
                        s,
                        "IMASM expand — the type '{}' IS this IMASM program (strange loop: a \
                         glyph in a tuple unfolds into its own opcode sequence):",
                        name.trim_start_matches("the_primitive_type_called_")
                    );
                    let _ = writeln!(s, "  sequence: {}", seq.join(" "));
                    let _ = writeln!(s, "{}", g.classify());
                    let errs = g.validate();
                    if errs.is_empty() {
                        let _ = writeln!(s, "  grammar: valid");
                    } else {
                        for e in &errs {
                            let _ = writeln!(s, "    ✗ {e}");
                        }
                    }
                    let _ = writeln!(s, "  domain reading (the pivot each opcode enacts):");
                    for a in &actions {
                        let _ = writeln!(s, "    {a}");
                    }
                    let _ = writeln!(
                        s,
                        "  → splice this sequence into a polymer to pivot through state space \
                         AS this type: e.g. `imasm star IMSCRIB : {} : …` makes it one arm.",
                        seq.join(" ")
                    );
                    s
                }
            }
        }
        // Construction / verification intent under a name the engine does not have. The
        // capability EXISTS — the agent just reached for the wrong word. Redirect it to the
        // real closing verbs instead of a flat "unknown", so it completes the dual rather than
        // concluding (falsely) that the Grammar lacks the opcode. The Grammar is never missing
        // the means; only the name was wrong.
        "simulate" | "instantiate" | "construct" | "build" | "create" | "make" | "encode" | "compile" => {
            format!(
                "imasm has no '{op}' — but the capability is here, under its real name. To CONSTRUCT \
                 (encode an algorithm as a kernel-constrained program): `imasm define <name> <op> \
                 <args…>` (op ∈ chain|ring|star|comb|bubble|wire). To VERIFY it: `imasm check \
                 <opcode word>` (Belnap type-check) or `imasm prove <name>` (the real p4ramill \
                 kernel). Emit those TOOL: lines now — do not report the Grammar as lacking an \
                 opcode; it is not.\n"
            )
        }
        "verify" | "typecheck" | "test" => {
            format!(
                "imasm has no '{op}' — use `imasm check <opcode word>` (Belnap type-check of a \
                 decision/program) or `imasm prove <name|word>` (the p4ramill kernel). Run it now.\n"
            )
        }
        _ => {
            // treat the whole arg list as a raw token chain
            let toks = tok_list(args);
            if toks.is_empty() {
                return format!(
                    "imasm: no op '{op}'. To build: chain|ring|star|comb|bubble|wire. To construct a \
                     named tool: define. To verify: check | prove. To read a type: expand | types. \
                     Full rules: imasm ref.\n"
                );
            }
            report("chain", &chain(&toks))
        }
    }
}

#[cfg(test)]
mod tests {

    /// The cycle's two structural facts, pinned so a change to the type table
    /// cannot quietly break the return leg. First: type programs are not
    /// self-delimiting, so a word cannot be cut on the boundary pair — `out`
    /// carries TANCH in its middle. Second: an entry's word reads back to a
    /// tuple that still contains the original at every axis.
    #[test]
    fn the_cycle_closes_and_out_is_not_self_delimiting() {
        let (ops, _p, _a) = expand_type("out").expect("the type 'out' expands");
        let word: String = ops.iter().map(|t| t.code()).collect();
        assert!(
            !word.ends_with('⊣'),
            "'out' now ends on the boundary pair; the parse-based return leg was written because \
             it did not, and a boundary cut would be viable again"
        );

        // A tuple written and read back must still admit the tuple it came from.
        let Ok(tuple) = entry_tuple("hydrogen_atom") else { return }; // no catalog on this host
        let w = tuple_glyph_word(&tuple).expect("the tuple writes a word");
        let read = tuple_from_word(&w).expect("the word reads back");
        assert_eq!(read.len(), 12);
        for (i, hits) in read.iter().enumerate() {
            assert!(
                hits.contains(&tuple[i]),
                "axis {} lost its original type in the return leg",
                TUPLE_ORDER[i]
            );
        }
    }
    use super::*;

    /// The tool registry is one real file; tests that define/remove tools must
    /// not interleave their read-modify-write cycles (seen: parallel test
    /// threads clobbering each other's defines).
    static REG_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());

    /// Operational μ∘δ: the canonical protocol word is value-lossless (B splits
    /// to (T,F), fuses back to B), and the readout renders in the FOUR slice.
    #[test]
    fn eval_protocol_word_is_lossless_on_values() {
        let args: Vec<String> = ["VINIT", "FSPLIT", "AFWD", "FFUSE", "EVALT"]
            .iter().map(|s| s.to_string()).collect();
        let out = eval_tool(&args, false);
        assert!(out.contains("id (lossless split-fuse)"), "{out}");
        assert!(out.contains("readout at sink(s): EVALT(T)"), "{out}");
    }

    /// An AREV on the truth arm breaks value-level μ∘δ even though the SHAPE
    /// still closes: fed Tf, recovered Ff. Structure and flow are separate facts.
    #[test]
    fn eval16_arev_arm_breaks_value_identity() {
        let args: Vec<String> = ["VINIT", "FSPLIT", "AREV", "FFUSE", "EVALT", "seed=Tf"]
            .iter().map(|s| s.to_string()).collect();
        let out = eval_tool(&args, true);
        assert!(out.contains("NOT id"), "{out}");
        assert!(out.contains("fed Tf"), "{out}");
    }

    /// Composition binds living ends only, consumes valences, never mints; a
    /// composite of two open chains can CLOSE (the worked dyad emerges from the
    /// binding), and it round-trips the registry as a wire spec.
    #[test]
    fn compose_consumes_valences_and_can_close() {
        let _guard = REG_LOCK.lock().unwrap();
        let a: Vec<String> = ["cp_test_a", "chain", "VINIT", "FSPLIT", "AFWD"]
            .iter().map(|s| s.to_string()).collect();
        let b: Vec<String> = ["cp_test_b", "chain", "FFUSE", "EVALT", "TANCH"]
            .iter().map(|s| s.to_string()).collect();
        assert!(define_tool(&a).contains("admitted"));
        assert!(define_tool(&b).contains("admitted"));
        let c: Vec<String> = ["cp_test_ab", "cp_test_a", "cp_test_b"]
            .iter().map(|s| s.to_string()).collect();
        let out = compose_tool(&c);
        assert!(out.contains("0 minted"), "{out}");
        assert!(out.contains("CLOSED"), "composite of the two open chains must close: {out}");
        let flowed = eval_tool(&["cp_test_ab".to_string()], false);
        assert!(flowed.contains("id (lossless split-fuse)"), "{flowed}");
        // a finished loop refuses further composition on the closed side
        let mut reg = load_registry();
        for k in ["cp_test_a", "cp_test_b", "cp_test_ab"] {
            reg.as_object_mut().unwrap().remove(k);
        }
        let _ = save_registry(&reg);
    }

    /// Chaos walks every ordering, folds by end-to-valence binding, and the
    /// orderings COLLAPSE into outcome classes; refusals carry their reason.
    #[test]
    fn chaos_walks_the_state_space_and_collapses_it() {
        let _guard = REG_LOCK.lock().unwrap();
        for (n, spec) in [
            ("chaos_t_src", vec!["chain", "VINIT", "FSPLIT"]),
            ("chaos_t_mid", vec!["chain", "AFWD", "AREV"]),
            ("chaos_t_snk", vec!["chain", "FFUSE", "EVALT", "TANCH"]),
        ] {
            let mut a = vec![n.to_string()];
            a.extend(spec.iter().map(|s| s.to_string()));
            assert!(define_tool(&a).contains("admitted"));
        }
        let args: Vec<String> = ["chaos_t_src", "chaos_t_mid", "chaos_t_snk"]
            .iter().map(|s| s.to_string()).collect();
        let out = chaos_tool(&args);
        assert!(out.contains("orderings walked: 6"), "{out}");
        assert!(out.contains("admitted: 2"), "{out}");
        assert!(out.contains("CLOSED(1 worked dyad(s))"), "one arrangement must close: {out}");
        assert!(out.contains("refused — B offers no living end"), "{out}");
        let mut reg = load_registry();
        for k in ["chaos_t_src", "chaos_t_mid", "chaos_t_snk"] {
            reg.as_object_mut().unwrap().remove(k);
        }
        let _ = save_registry(&reg);
    }

    /// Seen live: `imasm define ᚦᚱᛟᚾᛖ ring wire` — a malformed spec built the empty
    /// graph, passed validate() vacuously, and a hollow tool sat in the registry
    /// burning a dyad on every prove/run. 15 of 71 registry entries were this class.
    #[test]
    fn define_refuses_an_empty_program() {
        let args: Vec<String> = ["throne_test", "ring", "wire"].iter().map(|s| s.to_string()).collect();
        let out = define_tool(&args);
        assert!(out.contains("REFUSED"), "empty program must be refused: {out}");
        assert!(out.contains("EMPTY"), "refusal must name the emptiness: {out}");
        assert!(out.contains("define again"), "refusal must carry the door: {out}");
    }

    #[test]
    fn linear_is_linear() {
        let g = chain(&[Token::Vinit, Token::Afwd, Token::Imscrib]);
        assert_eq!(g.circuit_rank(), 0);
        assert!(g.classify().contains("linear"));
        assert!(g.validate().is_empty());
    }

    #[test]
    fn codon_word_round_trips() {
        // every opcode's code() re-parses to itself
        for t in [
            Token::Vinit, Token::Tanch, Token::Afwd, Token::Arev, Token::Clink,
            Token::Imscrib, Token::Fsplit, Token::Ffuse, Token::Evalt, Token::Evalf,
            Token::Engagr, Token::Ifix,
        ] {
            assert_eq!(Token::parse(t.code()), Some(t), "code {} lost {:?}", t.code(), t);
        }
        // a glued code word parses to the same tokens as the spelled-out names, and a
        // multi-letter name is never char-split
        let glued = tok_list(&["⊢>◇+⊙¬⊣".to_string()]);
        let named = tok_list(&[
            "VINIT AFWD FSPLIT EVALT IMSCRIB IFIX TANCH".to_string(),
        ]);
        assert_eq!(glued, named);
        assert_eq!(tok_list(&["VINIT".to_string()]), vec![Token::Vinit]);
        // the alphabet is fully symbolic: the retired letter codes must NOT parse, or a
        // stray V/T/B in prose would silently compose as an opcode
        for retired in ["V", "T", "B", "←"] {
            assert_eq!(Token::parse(retired), None, "retired code {retired} still parses");
        }
        // short forms and the new glyphs survive the switch
        assert_eq!(Token::parse("⊞"), Some(Token::Engagr));
        assert_eq!(Token::parse("⊙"), Some(Token::Imscrib));
        assert_eq!(tok_list(&["EG".to_string()]), vec![Token::Engagr]);
        assert_eq!(tok_list(&["IM".to_string()]), vec![Token::Imscrib]);
    }

    #[test]
    fn ring_has_one_loop() {
        let g = ring(&[Token::Imscrib, Token::Afwd, Token::Arev]);
        assert_eq!(g.circuit_rank(), 1);
        assert!(g.classify().contains("ring"));
    }

    #[test]
    fn star_of_three_arms() {
        let arms = vec![vec![Token::Afwd], vec![Token::Arev], vec![Token::Clink]];
        let g = star(Token::Imscrib, &arms);
        assert!(g.validate().is_empty(), "{:?}", g.validate());
        let c = g.classify();
        assert!(c.contains("star"), "{c}");
        assert!(c.contains("arms=3"), "{c}");
    }

    #[test]
    fn bubble_reconverges() {
        let g = bubble(&[Token::Imscrib], &[Token::Evalt], &[Token::Evalf], &[Token::Ifix]);
        assert_eq!(g.circuit_rank(), 1);
        assert!(g.validate().is_empty(), "{:?}", g.validate());
    }

    #[test]
    fn comb_is_a_comb() {
        let g = comb(
            &[Token::Vinit, Token::Afwd, Token::Arev, Token::Imscrib],
            &[(1, vec![Token::Clink]), (2, vec![Token::Imscrib])],
        );
        assert!(g.classify().contains("comb"), "{}", g.classify());
    }

    #[test]
    fn check_verdicts() {
        // closes: fork resolved by fuse, then fixed → T
        let t = run(&["check".into(), "VINIT IMSCRIB FSPLIT EVALT EVALF FFUSE IFIX".into()]);
        assert!(t.contains("T (closes)"), "{t}");
        // open: forks but never fuses → B
        let b = run(&["check".into(), "IMSCRIB FSPLIT EVALT".into()]);
        assert!(b.contains("B (open)"), "{b}");
        // split then immediately fuse, nothing between → identity, NOT a type-check
        let id = run(&["check".into(), "IMSCRIB FSPLIT FFUSE".into()]);
        assert!(id.contains("N (identity)"), "{id}");
        // bare cycle: β=1 but no δ/μ → not a closure
        let ring = run(&["check".into(), "IMSCRIB AFWD AREV".into()]);
        assert!(ring.contains("N (no fork)"), "{ring}");
        // empty → N (void)
        let n = run(&["check".into(), "".into()]);
        assert!(n.contains("N (void)"), "{n}");
    }

    #[test]
    fn kernel_refuses_illtyped_tool() {
        // IMSCRIB fanning to two arms is ill-typed → must be refused, not stored
        let out = run(&[
            "define".into(),
            "bad".into(),
            "wire".into(),
            "IMSCRIB AFWD AREV / 0-1 0-2".into(),
        ]);
        assert!(out.contains("REFUSED"), "{out}");
    }

    #[test]
    fn free_wire_makes_a_network() {
        // two rings sharing a fuse node → β=2 network, built purely from an edge set
        let out = run(&[
            "wire".into(),
            "IMSCRIB AFWD FFUSE AREV / 0-1 1-2 2-0 2-3 3-2".into(),
        ]);
        assert!(out.contains("β=2"), "{out}");
        assert!(out.contains("network"), "{out}");
    }

    #[test]
    fn only_fsplit_may_branch() {
        let mut g = Graph::new();
        let r = g.add(Token::Imscrib);
        let x = g.add(Token::Afwd);
        let y = g.add(Token::Arev);
        g.connect(r, x);
        g.connect(r, y);
        assert!(!g.validate().is_empty(), "IMSCRIB fan-out must be rejected");
    }
}
/// Glyph → Shavian type name: the 49-symbol alphabet, and the bridge from a catalog tuple to
/// executable opcodes. Each name is a full IMASM program under `ob3ects/primitives/`, which is
/// what `expand_type` reads — so the names here are that directory's, matched to the canonical
/// enums by glyph. The naming is glyph-GLOBAL: 𐑛 is `dead` on every axis it appears on.
/// Note `if`, `or` and `ha_ha`: Python spells the first two `if_`/`or_` because they are
/// keywords, and Lean escapes them `if'`/`or'`. The alphabet's own spelling is neither.
pub(crate) const GLYPH_TYPE_NAME: &[(&str, &str)] = &[
    ("⊙", "monad"),
    ("𐑐", "peep"),
    ("𐑑", "tot"),
    ("𐑒", "kick"),
    ("𐑓", "fee"),
    ("𐑔", "thigh"),
    ("𐑕", "so"),
    ("𐑖", "sure"),
    ("𐑗", "church"),
    ("𐑘", "yea"),
    ("𐑙", "hung"),
    ("𐑚", "bib"),
    ("𐑛", "dead"),
    ("𐑜", "gag"),
    ("𐑝", "vow"),
    ("𐑞", "they"),
    ("𐑟", "zoo"),
    ("𐑠", "measure"),
    ("𐑡", "judge"),
    ("𐑢", "woe"),
    ("𐑣", "ha_ha"),
    ("𐑤", "loll"),
    ("𐑥", "mime"),
    ("𐑦", "if"),
    ("𐑧", "egg"),
    ("𐑨", "ash"),
    ("𐑩", "ado"),
    ("𐑪", "on"),
    ("𐑫", "wool"),
    ("𐑬", "out"),
    ("𐑭", "ah"),
    ("𐑮", "roar"),
    ("𐑯", "nun"),
    ("𐑰", "eat"),
    ("𐑱", "age"),
    ("𐑲", "ice"),
    ("𐑳", "up"),
    ("𐑴", "oak"),
    ("𐑵", "ooze"),
    ("𐑶", "oil"),
    ("𐑷", "awe"),
    ("𐑸", "are"),
    ("𐑹", "or"),
    ("𐑺", "air"),
    ("𐑻", "err"),
    ("𐑼", "array"),
    ("𐑽", "ear"),
    ("𐑾", "ian"),
    ("𐑿", "yew"),
];

/// The Shavian type name for a glyph, or None if it is outside the 49.
pub(crate) fn type_name_for_glyph(g: &str) -> Option<&'static str> {
    GLYPH_TYPE_NAME.iter().find(|(k, _)| *k == g).map(|(_, v)| *v)
}
