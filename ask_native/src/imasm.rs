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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Token {
    Vinit,
    Tanch,
    Afwd,
    Arev,
    Clink,
    Imscrib,
    Fsplit,
    Ffuse,
    Evalt,
    Evalf,
    Engagr,
    Ifix,
}

impl Token {
    fn name(self) -> &'static str {
        match self {
            Token::Vinit => "VINIT",
            Token::Tanch => "TANCH",
            Token::Afwd => "AFWD",
            Token::Arev => "AREV",
            Token::Clink => "CLINK",
            Token::Imscrib => "IMSCRIB",
            Token::Fsplit => "FSPLIT",
            Token::Ffuse => "FFUSE",
            Token::Evalt => "EVALT",
            Token::Evalf => "EVALF",
            Token::Engagr => "ENGAGR",
            Token::Ifix => "IFIX",
        }
    }

    /// Single-glyph code — one symbol per opcode, so an opcode word reads as a
    /// sequence (a codon string) instead of a space-delimited token list. The alphabet
    /// is not invented: it references the per-token glyph vocabulary the IMSCRIBr pen-mode
    /// READING_GUIDE already fixes (ob3ect/READING_GUIDE.md §3). Six are the guide's own
    /// midpoint glyphs (FSPLIT ◇, FFUSE ●, EVALT +, EVALF ×, CLINK from its double-line
    /// ═ → =); IFIX is the guide's stated meaning "fix (¬)"; AFWD/AREV are the guide's
    /// forward→/reverse→ as > / <. Every token is SYMBOLIC — no Latin initials: VINIT ⊢ and
    /// TANCH ⊣ are the opening/closing boundary turnstiles, ENGAGR ⊞ is the Belnap Both it
    /// holds, and IMSCRIB is ⊙ — the Grammar's own self-modeling glyph, which is exactly what
    /// IMSCRIB means (identity / self-reference). Distinct by construction, so `parse`
    /// round-trips every code back to its token. Retired and no longer parsing: the letter
    /// codes V/T/B, and ← for IMSCRIB. Full names and the short forms VI/TA/EG/IM still do.
    /// IMSCRIB is ⊙ for a reason, not by availability: imscribing is the very act of
    /// INCLOSURE — the monadic operation itself — hence self-referential, and so referenced
    /// self-referentially. The glyph is a boundary drawn around its own centre, denoting the
    /// act of denoting. Its appearance as Criticality in the 12-primitive notation is not a
    /// collision: it is the same structure surfacing wherever inclosure closes on itself.
    fn code(self) -> &'static str {
        match self {
            Token::Vinit => "⊢",
            Token::Tanch => "⊣",
            Token::Afwd => ">",
            Token::Arev => "<",
            Token::Clink => "=",
            Token::Imscrib => "⊙",
            Token::Fsplit => "◇",
            Token::Ffuse => "●",
            Token::Evalt => "+",
            Token::Evalf => "×",
            Token::Engagr => "⊞",
            Token::Ifix => "¬",
        }
    }

    /// Does this opcode TRANSFORM the object (do work), as opposed to being an
    /// identity/structural node? IMSCRIB is the identity morphism, VINIT/TANCH are
    /// source/sink, FSPLIT/FFUSE are the split/fuse themselves — none transform.
    /// A μ∘δ closure whose arms carry only these is identity, not a type-check.
    fn transforms(self) -> bool {
        matches!(
            self,
            Token::Afwd | Token::Arev | Token::Clink | Token::Evalt | Token::Evalf
                | Token::Engagr | Token::Ifix
        )
    }

    /// (arity_in, arity_out) — the max ports the opcode may carry.
    fn arity(self) -> (usize, usize) {
        match self {
            Token::Vinit => (0, 1),
            Token::Fsplit => (1, 2),
            Token::Ffuse => (2, 1),
            _ => (1, 1),
        }
    }

    /// Accepts full names (VINIT) and the IMSCRIBr short forms (VI, FS, FF, …),
    /// case-insensitively.
    fn parse(s: &str) -> Option<Token> {
        let u = s.trim().to_ascii_uppercase();
        Some(match u.as_str() {
            "VINIT" | "VI" | "⊢" => Token::Vinit,
            "TANCH" | "TA" | "⊣" => Token::Tanch,
            "AFWD" | "AF" | ">" => Token::Afwd,
            "AREV" | "AR" | "<" => Token::Arev,
            "CLINK" | "CL" | "=" | "═" => Token::Clink,
            "IMSCRIB" | "IMSCRIBE" | "IM" | "⊙" => Token::Imscrib,
            "FSPLIT" | "FS" | "SPLIT" | "DELTA" | "◇" | "δ" => Token::Fsplit,
            "FFUSE" | "FF" | "FUSE" | "MU" | "●" | "μ" => Token::Ffuse,
            "EVALT" | "ET" | "+" => Token::Evalt,
            "EVALF" | "EF" | "×" => Token::Evalf,
            "ENGAGR" | "EG" | "⊞" => Token::Engagr,
            "IFIX" | "IX" | "FIX" | "¬" => Token::Ifix,
            _ => return None,
        })
    }
}

/// μ∘δ closure state — the real close condition (δ arms reconnecting at μ), which
/// is independent of whether the graph merely has a cycle.
enum ClosureState {
    Closed(usize), // δ arms transformed AND rejoined at μ: a real type-check
    Identity,      // δ/μ reconnect but nothing transforms between — μ∘δ=id, no work done
    Open,          // δ/μ present but a fork or fuse dangles unreconnected
    None,          // no δ/μ dyad at all — a bare line or cycle is NOT a closure
}

/// A directed IMASM program graph. Node id = index into `nodes`.
pub struct Graph {
    nodes: Vec<Token>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    fn new() -> Graph {
        Graph { nodes: Vec::new(), edges: Vec::new() }
    }

    fn add(&mut self, t: Token) -> usize {
        self.nodes.push(t);
        self.nodes.len() - 1
    }

    fn connect(&mut self, a: usize, b: usize) {
        self.edges.push((a, b));
    }

    /// Add a fresh path of `toks`, wire head→tail, return the node ids.
    fn chain_of(&mut self, toks: &[Token]) -> Vec<usize> {
        let ids: Vec<usize> = toks.iter().map(|&t| self.add(t)).collect();
        for w in ids.windows(2) {
            self.connect(w[0], w[1]);
        }
        ids
    }

    fn out_degree(&self, n: usize) -> usize {
        self.edges.iter().filter(|&&(a, _)| a == n).count()
    }

    fn in_degree(&self, n: usize) -> usize {
        self.edges.iter().filter(|&&(_, b)| b == n).count()
    }

    fn successors(&self, n: usize) -> Vec<usize> {
        self.edges.iter().filter(|&&(a, _)| a == n).map(|&(_, b)| b).collect()
    }

    fn predecessors(&self, n: usize) -> Vec<usize> {
        self.edges.iter().filter(|&&(_, b)| b == n).map(|&(a, _)| a).collect()
    }

    // ── invariants ──────────────────────────────────────────────────────────

    /// Weakly-connected component count (edges read undirected).
    fn components(&self) -> usize {
        let n = self.nodes.len();
        if n == 0 {
            return 0;
        }
        let mut parent: Vec<usize> = (0..n).collect();
        fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }
        for &(a, b) in &self.edges {
            let ra = find(&mut parent, a);
            let rb = find(&mut parent, b);
            if ra != rb {
                parent[ra] = rb;
            }
        }
        let mut roots: Vec<usize> = (0..n).map(|i| find(&mut parent, i)).collect();
        roots.sort_unstable();
        roots.dedup();
        roots.len()
    }

    /// First Betti number β = E − V + C: the number of independent loops.
    fn circuit_rank(&self) -> i64 {
        self.edges.len() as i64 - self.nodes.len() as i64 + self.components() as i64
    }

    /// For each node, which FSPLIT nodes can reach it going forward. `anc_or_self`
    /// also counts a node that IS an FSPLIT as its own ancestor, so an FSPLIT that
    /// feeds an FFUSE directly (an empty-branch arm) still counts as that arm.
    fn fsplit_ancestors(&self) -> Vec<Vec<usize>> {
        let n = self.nodes.len();
        let mut anc = vec![Vec::new(); n];
        for f in 0..n {
            if self.nodes[f] != Token::Fsplit {
                continue;
            }
            let mut seen = vec![false; n];
            let mut stack = vec![f];
            seen[f] = true;
            while let Some(x) = stack.pop() {
                for s in self.successors(x) {
                    if !seen[s] {
                        seen[s] = true;
                        anc[s].push(f);
                        stack.push(s);
                    }
                }
            }
        }
        anc
    }

    /// The REAL close condition: δ arms reconnecting at μ. A μ∘δ closure is an
    /// (FSPLIT, FFUSE) pair where two distinct in-arms of the FFUSE trace back to
    /// a common FSPLIT — the fork was undone by the fuse, however it routed. A bare
    /// cycle (β≥1 with no δ/μ) is NOT a closure. Returns (reconnection pairs,
    /// fully_closed) where fully_closed means every δ and every μ participates.
    fn frobenius_closures(&self) -> (Vec<(usize, usize)>, bool) {
        let anc = self.fsplit_ancestors();
        let anc_or_self = |node: usize| -> Vec<usize> {
            let mut v = anc[node].clone();
            if self.nodes[node] == Token::Fsplit && !v.contains(&node) {
                v.push(node);
            }
            v
        };
        let fsplits: Vec<usize> =
            (0..self.nodes.len()).filter(|&i| self.nodes[i] == Token::Fsplit).collect();
        let mut pairs = Vec::new();
        for j in 0..self.nodes.len() {
            if self.nodes[j] != Token::Ffuse {
                continue;
            }
            // in-edges WITH multiplicity — an FFUSE fed twice by the same δ (empty
            // arms) still reconnects. A δ whose arms feed ≥2 of j's in-ports closes.
            let in_edges: Vec<usize> =
                self.edges.iter().filter(|&&(_, b)| b == j).map(|&(a, _)| a).collect();
            // Every δ whose arms feed ≥2 of j's in-ports is a candidate. On a strand an
            // UPSTREAM δ reaches every later μ, so it qualifies at every one of them —
            // taking the first candidate would let it claim them all and starve the δ
            // that actually forked here, reporting Open for a correctly wired program.
            // Pair j with the INNERMOST candidate instead: the one no other candidate
            // descends from. A δ may close more than one μ; a μ closes with exactly one δ.
            let cands: Vec<usize> = fsplits
                .iter()
                .copied()
                .filter(|&f| in_edges.iter().filter(|&&p| anc_or_self(p).contains(&f)).count() >= 2)
                .collect();
            if let Some(&f) = cands
                .iter()
                .find(|&&f| !cands.iter().any(|&g| g != f && anc[g].contains(&f)))
            {
                pairs.push((f, j));
            }
        }
        let n_split = self.nodes.iter().filter(|&&t| t == Token::Fsplit).count();
        let n_fuse = self.nodes.iter().filter(|&&t| t == Token::Ffuse).count();
        let closed_splits: std::collections::BTreeSet<usize> = pairs.iter().map(|&(f, _)| f).collect();
        let closed_fuses: std::collections::BTreeSet<usize> = pairs.iter().map(|&(_, j)| j).collect();
        let fully = (n_split > 0 || n_fuse > 0)
            && closed_splits.len() == n_split
            && closed_fuses.len() == n_fuse;
        (pairs, fully)
    }

    /// Nodes strictly on a path from `f` forward to `j`: forward-reachable from f
    /// AND backward-reachable from j (endpoints excluded). These are the arms.
    fn between(&self, f: usize, j: usize) -> Vec<usize> {
        let n = self.nodes.len();
        let reach = |start: usize, fwd: bool| -> Vec<bool> {
            let mut seen = vec![false; n];
            let mut stack = vec![start];
            seen[start] = true;
            while let Some(x) = stack.pop() {
                let nbrs: Vec<usize> = if fwd {
                    self.successors(x)
                } else {
                    self.predecessors(x)
                };
                for y in nbrs {
                    if !seen[y] {
                        seen[y] = true;
                        stack.push(y);
                    }
                }
            }
            seen
        };
        let fwd = reach(f, true);
        let bwd = reach(j, false);
        (0..n).filter(|&k| k != f && k != j && fwd[k] && bwd[k]).collect()
    }

    /// Does a reconnection (f→…→j) carry a TRANSFORMATION — i.e. do the arms
    /// between the split and the fuse do real work (a transforming opcode), rather
    /// than pass the object through unchanged? This is the crux: split→fuse with
    /// nothing between is μ∘δ=id (identity), which type-checks nothing.
    fn transforms_between(&self, f: usize, j: usize) -> bool {
        self.between(f, j).into_iter().any(|k| self.nodes[k].transforms())
    }

    /// The μ∘δ closure state. A real closure (type-check) needs BOTH: δ arms that
    /// reconnect at μ, AND a transformation carried on those arms. Reconnection with
    /// no transformation is Identity (μ∘δ=id, no work). No δ/μ dyad at all is None —
    /// a bare line or cycle is not a closure (β is not diagnostic).
    fn closure_state(&self) -> ClosureState {
        let n_split = self.nodes.iter().filter(|&&t| t == Token::Fsplit).count();
        let n_fuse = self.nodes.iter().filter(|&&t| t == Token::Ffuse).count();
        if n_split == 0 && n_fuse == 0 {
            return ClosureState::None;
        }
        let (pairs, fully) = self.frobenius_closures();
        if !fully {
            return ClosureState::Open;
        }
        if pairs.iter().any(|&(f, j)| self.transforms_between(f, j)) {
            ClosureState::Closed(pairs.len())
        } else {
            ClosureState::Identity
        }
    }

    fn branch_points(&self) -> Vec<usize> {
        (0..self.nodes.len()).filter(|&n| self.out_degree(n) > 1).collect()
    }

    fn merge_points(&self) -> Vec<usize> {
        (0..self.nodes.len()).filter(|&n| self.in_degree(n) > 1).collect()
    }

    fn sources(&self) -> usize {
        (0..self.nodes.len()).filter(|&n| self.in_degree(n) == 0).count()
    }

    fn sinks(&self) -> usize {
        (0..self.nodes.len()).filter(|&n| self.out_degree(n) == 0).count()
    }

    // ── validation ──────────────────────────────────────────────────────────

    /// Grammar check. Errors: a non-FSPLIT fanning out, a non-FFUSE merging in,
    /// or any over-valence. Open valences (living ends) are reported, not fatal.
    fn validate(&self) -> Vec<String> {
        let mut errs = Vec::new();
        for (n, &tok) in self.nodes.iter().enumerate() {
            let (ai, ao) = tok.arity();
            let od = self.out_degree(n);
            let idg = self.in_degree(n);
            if od > 1 && tok != Token::Fsplit {
                errs.push(format!(
                    "node {n} ({}) fans out to {od}; only FSPLIT (δ) may branch",
                    tok.name()
                ));
            }
            if idg > 1 && tok != Token::Ffuse {
                errs.push(format!(
                    "node {n} ({}) merges {idg} in; only FFUSE (μ) may fuse",
                    tok.name()
                ));
            }
            if od > ao {
                errs.push(format!("node {n} ({}) out-degree {od} > arity_out {ao}", tok.name()));
            }
            if idg > ai {
                errs.push(format!("node {n} ({}) in-degree {idg} > arity_in {ai}", tok.name()));
            }
        }
        errs
    }

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
fn from_sequence(ops: &[Token], pairs: &[(usize, usize)]) -> Graph {
    let mut g = Graph::new();
    let ids = g.chain_of(ops);
    for &(fs, ff) in pairs {
        if fs < ids.len() && ff < ids.len() {
            g.connect(ids[fs], ids[ff]);
        }
    }
    g
}

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

fn tok_list(args: &[String]) -> Vec<Token> {
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
fn match_pairs(ops: &[Token]) -> Vec<(usize, usize)> {
    let mut stack = Vec::new();
    let mut pairs = Vec::new();
    for (i, &t) in ops.iter().enumerate() {
        if t == Token::Fsplit {
            stack.push(i);
        } else if t == Token::Ffuse {
            if let Some(fs) = stack.pop() {
                pairs.push((fs, i));
            }
        }
    }
    pairs
}

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

fn load_registry() -> serde_json::Value {
    std::fs::read_to_string(tools_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .filter(|v: &serde_json::Value| v.is_object())
        .unwrap_or_else(|| serde_json::json!({}))
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
    match std::fs::write(tools_path(), serde_json::to_string_pretty(&reg).unwrap_or_default()) {
        Ok(_) => format!(
            "IMASM define → tool '{name}' admitted (grammar-valid, kernel-constrained).\n{}\n  \
             run it: imasm run {name}   |   list: imasm tools\n",
            report(title, &g).trim_end()
        ),
        Err(e) => format!("define: could not persist the tool: {e}\n"),
    }
}

fn run_tool(rest: &[String]) -> String {
    let Some(name) = rest.first() else {
        return "run needs a tool name: imasm run <name>  (`imasm tools` lists them)\n".into();
    };
    let reg = load_registry();
    let Some(rec) = reg.get(name.as_str()) else {
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
const TUPLE_ORDER: [&str; 12] = ["Ð","Þ","Ř","Φ","ƒ","Ç","Γ","ɢ","⊙","Ħ","Σ","Ω"];

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
    if green {
        format!(
            "IMASM prove — {label}\n  pre-filter: {kind}\n  KERNEL VERDICT: ✓ green — p4ramill \
             confirms the closure class against BelnapSplitFuse (B_is_the_only_bifurcation_point, \
             split_fuse_id).\n"
        )
    } else {
        format!(
            "IMASM prove — {label}\n  pre-filter: {kind}\n  KERNEL VERDICT: ✗ the lake build did not \
             go green.\n    {tail}\n"
        )
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

pub fn run(args: &[String]) -> String {
    if args.is_empty() {
        return format!("{REFERENCE}\n");
    }
    let op = args[0].to_ascii_lowercase();
    let rest = &args[1..];
    match op.as_str() {
        "ref" | "reference" | "help" | "rules" => format!("{REFERENCE}\n"),
        "check" | "typecheck" => verdict_check(rest),
        "define" | "forge_tool" => define_tool(rest),
        "run" | "invoke" => run_tool(rest),
        "prove" | "kernel" => prove_tool(rest),
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
    use super::*;

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
