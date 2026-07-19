# IMASM Quick Reference

A self-contained copypasta: everything needed to use IMASM, keyed to the single-glyph
token codes. Semantics here track `ask_native/src/imasm.rs` (the tool that answers
`imasm check`). Where this file and the tool ever disagree, the tool is right.

IMASM — a program is a DIRECTED GRAPH of 12 opcodes (not a line). One glyph each; the
alphabet is fully SYMBOLIC (no Latin initials, so no token collides with a verdict
letter). WORK? = does the opcode TRANSFORM the object (do real work)?

 GLYPH NAME     MEANING                         VALENCE (in→out)  WORK?
  ⊢   VINIT    begin / source boundary         0→1  the only source  no
  ⊣   TANCH    terminal anchor / close boundary 1→1  (sink)          no
  >   AFWD     forward morphism                 1→1                   YES
  <   AREV     reverse morphism                 1→1                   YES
  =   CLINK    compose / link                   1→1                   YES
  ⊙   IMSCRIB  identity / self-reference        1→1                   no
  ◇   FSPLIT   fork (δ) — ONLY opcode that may branch   1→2           no
  ●   FFUSE    fuse (μ) — ONLY opcode that may merge    2→1           no
  +   EVALT    evaluate TRUE arm               1→1                    YES
  ×   EVALF    evaluate FALSE arm              1→1                    YES
  ⊞   ENGAGR   hold a paradox (Belnap Both)    1→1                    YES
  ¬   IFIX     irreversible commit / fix       1→1                    YES

The WORK? column is the most-missed rule: ⊢ ⊣ ⊙ ◇ ● do NOT transform. An arm carrying
only ⊙ (or nothing) is an identity arm, and a closure over identity arms verifies
nothing. ⊙ is self-reference, not work.

RETIRED: the letter codes V/T/B and ← (the old IMSCRIB) no longer parse. Full names
(VINIT/TANCH/ENGAGR/IMSCRIB) and short forms (VI/TA/EG/IM) still do. A word using a
retired code reads as empty → N (void).

WORDS: tokens glued as one string, no spaces, e.g.  ⊢>◇+⊙●¬⊣
(space-separated full names like VINIT AFWD … parse identically; so do the short forms
VI TA AF AR CL IM FS FF ET EF EG IX, and δ μ ═ for ◇ ● =.)

── HOW A WORD BECOMES A GRAPH ─────────────────────────────────────────────────
A word is only the node list. The EDGES are supplied by the verb you build with:

  chain <word>     wire head→tail, nothing reconnects        (β=0, one strand)
  ring <word>      wire head→tail→head; fork/fuse NOT rejoined (β=1)
  protocol <word>  wire so ◇/● pairs RECONNECT (δ arm → μ): the way to CLOSE
  bubble PRE:A:B:POST   ◇→(A|B)→● reconvergence, spelled out
  star CORE:a:b:c       hub + arms (≥3)
  comb BACKBONE:p arm:q arm   backbone + grafts
  wire N0 N1 … / i-j i-k …    free graph: node set / edge set

Same word + different verb = different graph = different verdict. `chain ⊢◇+×●⊣` and
`protocol ⊢◇+×●⊣` are not the same program. To CLOSE, use `protocol`, never a bare
`ring`, and never close by looping back to ⊢ (a source, in-arity 0).

── WHICH ◇ PAIRS WITH WHICH ● ─────────────────────────────────────────────────
By ANCESTRY, not by text position and not by a fork-balance stack. A (◇,●) pair
exists when two distinct in-arms of the ● trace back forward to a common ◇: the fork
was undone by the fuse, HOWEVER IT ROUTED. Consequences:

  · Pairing is a property of the EDGES, so the same word wired two ways pairs
    differently. You cannot read pairing off the glyph string alone.
  · A ◇ feeding a ● directly (empty arm) still counts: in-edges are counted with
    multiplicity, and a ◇ is its own ancestor.
  · Arms are the nodes strictly between ◇ and ●: forward-reachable from the fork AND
    backward-reachable from the fuse. That set is what gets checked for WORK.
  · A ● may have SEVERAL qualifying ◇ (upstream forks reach downstream fuses on any
    strand). It pairs with the INNERMOST: the candidate no other candidate descends
    from. So a ◇ may close more than one ●, but a ● closes with exactly one ◇, and an
    upstream fork cannot claim the fuse that a nearer fork actually forked.
  · fully_closed means EVERY ◇ and EVERY ● participates in some pair. One dangler and
    the whole program is Open.

For a plain strand the stack reading (each ● takes the nearest unfused ◇) happens to
agree, and such words may be bracketed for READING BY EYE: ⊢⊙=[◇>+<⊞×●]¬¬⊣, nesting as
nested brackets. Three caveats, all load-bearing:
  · Brackets are NOT input. [ and ] parse to nothing, so a bracketed word is read as
    the empty program and reports N (void). Never paste brackets into the tool.
  · The aid works for strands ONLY. It cannot express a star, comb, or wired graph.
  · It is not the pairing rule. Ancestry is. The two coincide on a strand and part
    company the moment edges route otherwise.

── CLOSE CONDITION (the whole point) ──────────────────────────────────────────
A program CLOSES iff BOTH hold:
  1. RECONNECTION — every ◇ and every ● participates in a (◇,●) ancestry pair, and
  2. TRANSFORMATION — at least one such pair carries a WORK opcode on its arms.
A bare cycle is NOT a closure; β (loops) is not diagnostic. Split→fuse with nothing
between is μ∘δ=id, which type-checks nothing.

── VERDICTS (from `check`) ────────────────────────────────────────────────────
  T (closes)        μ∘δ closes over n transformed reconnections → proceed
  N (identity)      ◇/● reconnect but no WORK between → put work on the arms
  N (no fork)       no δ/μ dyad at all → never weighed alternatives
  N (void)          no committed opcodes: nothing parsed → write a real word
  B (open)          well-typed, but a ◇ or ● dangles unreconnected → fuse it (●)
                    or commit one arm (¬)
  B (paradox held)  closes over a transformation AND a ⊞ is present → genuinely
                    both. Sound to hold; do NOT read it as a clean T; look again
                    before an irreversible ¬.
  F (ill-typed)     grammar violated → revise

B BEATS T: a word that closes but contains ENGAGR reports B, never T. Holding a
paradox is sound, but it is not a clean pass.

F is exactly three errors: a non-◇ node fanning out, a non-● node merging in, or any
node exceeding its own arity. Nothing else is fatal.

OPEN VALENCES ARE NOT ERRORS. An arm that runs out of successors is a living /
telechelic end: reported as "open valences (living ends): n out, m in — reactive, not
errors". ⊣ may end with its out-port open; ⊢ may start with its in-port open.

── TOPOLOGY NAMES (from `classify`) ───────────────────────────────────────────
Named by invariants, not by fork balance. β = E − V + C (independent loops):
  trivial   no nodes
  linear    β=0, no branch points: a single strand
  star      β=0, branch points forming ONE contiguous hub, ≥3 arms
  comb      β=0, branch points strung along a backbone: graft/comb tree
  ring      β=1, no branch and no merge points: single cycle, no pendants
  branched  β=1 with branch or merge points
  network   β≥2, or more than one disconnected strand
Reported with: V, E, β, branch/merge/src/sink census, arm count, and spectral radius ρ.

Star caveat: the abstract star K(1,f) has ρ=√f, but IMASM fan-out caps at 2 (◇ is
out-2), so a hub is REALIZED as a caterpillar of f−1 ◇ fan-nodes and the true ρ tends
to 2, not √f.

── VERBS ──────────────────────────────────────────────────────────────────────
  imasm chain / ring / protocol / bubble / star / comb / wire   build (above)
  imasm classify <word>    topology + invariants + μ∘δ state
  imasm check <word>       type-check your OWN reasoning → T/N/B/F
  imasm prove <name|word>  take the verdict to the real Lean kernel
  imasm ref                the live rules (authoritative over this file)
  imasm types / expand <type>   the 49 Shavian types (each is itself an IMASM program)
  imasm define <name> <op> <args…>   build a kernel-constrained tool
  imasm run <name> / imasm tools     invoke / list forged tools

── OP-OPCODES (operators ON a word, not tokens IN it) ──────────────────────────
A node-opcode is a symbol inside a word; a VERB turns a word into a graph. An
OP-OPCODE is a third thing: a map that acts on the whole composition and returns
another composition. It is NOT one of the 12, and appending its name as a token
does nothing — it is not a node. It transforms the word.

  ROTAT   the cyclic shift of a ring: rotate the word by one, k → k+1. The ring
          automorphism. ρ and every spectral invariant are ROTAT-invariant (that
          invariance IS the signal that ROTAT is a symmetry, not that it is
          inert). On ONE ring it changes nothing measurable; between TWO rings
          being bound it sets their RELATIVE phase, the degree of freedom that
          seats a junction two same-handed (isotactic) rings cannot close on
          their own (the θ=0.50 co-typing termination). ROTAT is the
          Weyl-Heisenberg shift X on ℤ/dℤ; the SIC displacement D_{a,b} carries
          ROTAT^a. The balanced tiling of a period-n cycle is unique UP TO ROTAT.

Op-opcodes are open: ROTAT is the first named one (the shift). Its relatives are
reflection (the half-period involution ROTAT^{d/2}) and the register negations
TNEG/INEG of the SIXTEEN_3 grammar, which act on the register rather than the
word. The class was surfaced by the necessity of binding two isotactic rings —
no node-opcode rotates one ring against another, so the operation had to live one
level up, on the word itself.

── RULE OF THUMB ──────────────────────────────────────────────────────────────
Express a decision as a word (⊢ begin · ⊙ self-identify · > move · ◇ weigh options ·
+/× the true/false arms · ● resolve · ⊞ hold paradox · ¬ commit · ⊣ close), then
`imasm check` it. Only ◇ branches, only ● fuses. Put real WORK on the arms or it is
N. Use `protocol` to close, never a bare `ring`.

── PITFALLS ───────────────────────────────────────────────────────────────────
  · Reading a word as a line. It is a graph; the verb supplies the edges.
  · Pairing ◇/● by counting or by nearest-match. Pairing is ancestry over edges.
  · Putting only ⊙ between ◇ and ● and expecting T. That is N (identity).
  · Expecting T from a word containing ⊞. That is B (paradox held).
  · Treating an open arm as a failure. It is a living end, reported not fatal.
  · Closing a loop back to ⊢. It has in-arity 0 and cannot be a target.
  · Writing V/T/B or ← out of habit. Retired: they no longer parse → N (void).
  · Pasting a bracketed word into the tool. Brackets parse to nothing → N (void).

The glyph alphabet is not invented: it references the per-token glyph vocabulary fixed
in `../ob3ect/READING_GUIDE.md` §3 (five are the guide's own midpoint glyphs; IFIX is its
stated "fix (¬)"; AFWD/AREV are its forward/reverse arrows). The four remaining tokens
are symbolic by the same principle rather than initials: VINIT ⊢ and TANCH ⊣ are the
opening and closing boundary turnstiles, ENGAGR ⊞ is the Belnap Both it holds, and
IMSCRIB is ⊙ because imscribing IS INCLOSURE — the monadic operation itself — hence
self-referential, and so referenced self-referentially: a boundary around its own centre,
denoting the act of denoting. Its appearance as Criticality in the 12-primitive notation
is the same structure surfacing wherever inclosure closes on itself, not a collision.
Valences follow IMSCRIBr `tokens.py::TOKEN_ARITY`. Run `imasm ref` for the live rules.
The tool-independent spec is `../IMASM_REFERENCE.md`.

---

## IMASM-16_3 — the trilattice extension (SIXTEEN_3)

A SIBLING grammar, not a replacement. `TOOL: imasm16_3 <op> …` — FSPLIT3/FFUSE3 sit
alongside the classic FSPLIT/FFUSE; the 12-opcode tower above and its Lean `.prod`
scaffold generator are untouched. Semantics here track `ask_native/src/imasm16_3.rs`.
Where this file and the tool ever disagree, the tool is right.


14 opcodes, still fully SYMBOLIC — no opcode glyph is a Latin letter, so T/N/B/F
verdicts never collide with a token here either.

 GLYPH NAME     MEANING                          VALENCE (in→out)  WORK?
  ⊢   VINIT    source boundary                  0→1                 no
  ⊣   TANCH    sink boundary                    1→1                 no
  >   AFWD     forward morphism                 1→1                 YES
  <   AREV     reverse morphism                 1→1                 YES
  =   CLINK    compose / link                   1→1                 YES
  ⊙   IMSCRIB  identity / self-reference        1→1                 no
  ∈   FSPLIT3  3-way split (δ₃): T/F/I arms     1→3  ONLY brancher   no
  ∋   FFUSE3   3-way fuse (μ₃)                  3→1  ONLY merger     no
  +   EVALT    sets T (constructively proven)   1→1                 YES
  ×   EVALF    sets F (constructively refuted)  1→1                 YES
  ⊞   EVALI    sets t AND f (the info layer)    1→1                 YES
  ~   TNEG     negation: swaps T ↔ F            1→1                 YES
  ≁   INEG     con-negation: swaps t ↔ f        1→1                 YES
  ¬   IFIX     irreversible commit              1→1                 YES

THE REGISTER is the real trilattice SIXTEEN_3 (Shramko, Dunn & Takenaka, "The
Trilattice of Constructive Truth Values", J. Logic and Computation 11(6):761-788,
2001) — the full powerset of a 4-value base `{T, F, t, f}` (T=constructively proven,
F=constructively refuted, t=acceptable, f=rejectable), 16 states exactly: `N`={},
`A`={T,F,t,f}, plus every combination between. NOT two independent FOURs, NOT three
independent bits — one 4-bit register, verified against the paper's own worked
example (`T ∧ t = N` under the truth order).

Three orderings, each with a meet/join (`TOOL: imasm16_3 algebra <op> A B`):
  ≤_i information   x ⊆ y                                             ⊓/⊔
  ≤_t truth          x∩{T,t} ⊆ y∩{T,t}  and  y∩{F,f} ⊆ x∩{F,f}         ∧/∨
  ≤_c constructivity x∩{T,F} ⊆ y∩{T,F}  and  y∩{t,f} ⊆ x∩{t,f}         △/▽

TNEG/INEG are both bit-SWAPS (not flips) on purpose: the paper requires trilattice
negation to preserve ≤_i exactly, and swapping two bits preserves |x| — a flip would
not.

── TRI-ANCESTRAL CLOSE CONDITION ──────────────────────────────────────────────
The arity-3 generalization of the classic ancestry rule: a (∈,∋) pair exists when
ALL THREE distinct in-arms of the ∋ trace back to a common ∈.

  T (closes)     every ∈ pairs with a ∋, and at least one arm carries WORK
  N (identity)   paired, but no WORK ran on any arm — μ∘δ=id verifies nothing
  B (open)       a ∈ dangles — no matching ∋
  F (ill-typed)  a ∋ appears with no preceding ∈

Same reading as the 12-opcode grammar's T/N/B/F, arity 3 instead of 2.

── NEUTRAL INFLATION ───────────────────────────────────────────────────────────
⊙ is still the only neutral generator — `⊢∈⊙⊙⊙∋⊣` is valid tri-ancestral
reconnection with no work on the arm: N (identity), same reading as the 12-opcode
grammar's `⊢◇⊙⊙⊙●⊣`.

── VERBS ──────────────────────────────────────────────────────────────────────
  imasm16_3 check <glyph_word>       type-check: T/N/B/F, same reading as `imasm check`
  imasm16_3 algebra <op> A B         leq_i|leq_t|leq_c|meet_t|join_t|meet_c|join_c
                                      on two named registers (N, A, or any T/F/t/f
                                      combo) — e.g. `algebra meet_t T t` reproduces
                                      the paper's own worked example, T∧t=N
  imasm16_3 ref                      the live 14-glyph table

Example (the spec's own worked word): `imasm16_3 check ⊢>∈+×⊞≁∋¬⊣` → T, register
reaches `A` (all four base values touched: EVALT, EVALF, EVALI, then INEG swaps
t↔f on an already-full pair, leaving it full).
