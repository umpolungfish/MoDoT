# IMASM Quick Reference

A self-contained copypasta: everything needed to use IMASM, keyed to the single-glyph
token codes. Semantics here track `ask_native/src/imasm.rs` (the tool that answers
`imasm check`). Where this file and the tool ever disagree, the tool is right.

```
IMASM — a program is a DIRECTED GRAPH of 12 opcodes (not a line). One glyph each.
WORK? = does the opcode TRANSFORM the object (do real work)?

 GLYPH NAME     MEANING                         VALENCE (in→out)  WORK?
  V   VINIT    begin / source                   0→1  the only source  no
  T   TANCH    terminal anchor / close boundary 1→1  (sink)           no
  >   AFWD     forward morphism                 1→1                   YES
  <   AREV     reverse morphism                 1→1                   YES
  =   CLINK    compose / link                   1→1                   YES
  ←   IMSCRIB  identity / self-reference        1→1                   no
  ◇   FSPLIT   fork (δ) — ONLY opcode that may branch   1→2           no
  ●   FFUSE    fuse (μ) — ONLY opcode that may merge    2→1           no
  +   EVALT    evaluate TRUE arm               1→1                    YES
  ×   EVALF    evaluate FALSE arm              1→1                    YES
  B   ENGAGR   hold a paradox (Belnap Both)    1→1                    YES
  ¬   IFIX     irreversible commit / fix       1→1                    YES

The WORK? column is the most-missed rule: V T ← ◇ ● do NOT transform. An arm carrying
only ← (or nothing) is an identity arm, and a closure over identity arms verifies
nothing. ← is self-reference, not work.

WORDS: tokens glued as one string, no spaces, e.g.  V>◇+←●¬T
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

Same word + different verb = different graph = different verdict. `chain V◇+×●T` and
`protocol V◇+×●T` are not the same program. To CLOSE, use `protocol`, never a bare
`ring`, and never close by looping back to V (a source, in-arity 0).

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
  · fully_closed means EVERY ◇ and EVERY ● participates in some pair. One dangler and
    the whole program is Open.

For a plain strand the stack reading (each ● takes the nearest unfused ◇) happens to
agree, and such words may be bracketed for READING BY EYE: V←=[◇>+<B×●]¬¬T, nesting as
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
  B (paradox held)  closes over a transformation AND a B is present → genuinely
                    both. Sound to hold; do NOT read it as a clean T; look again
                    before an irreversible ¬.
  F (ill-typed)     grammar violated → revise

B BEATS T: a word that closes but contains ENGAGR reports B, never T. Holding a
paradox is sound, but it is not a clean pass.

F is exactly three errors: a non-◇ node fanning out, a non-● node merging in, or any
node exceeding its own arity. Nothing else is fatal.

OPEN VALENCES ARE NOT ERRORS. An arm that runs out of successors is a living /
telechelic end: reported as "open valences (living ends): n out, m in — reactive, not
errors". T may end with its out-port open; V may start with its in-port open.

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

── RULE OF THUMB ──────────────────────────────────────────────────────────────
Express a decision as a word (V begin · ← self-identify · > move · ◇ weigh options ·
+/× the true/false arms · ● resolve · B hold paradox · ¬ commit · T close), then
`imasm check` it. Only ◇ branches, only ● fuses. Put real WORK on the arms or it is
N. Use `protocol` to close, never a bare `ring`.

── PITFALLS ───────────────────────────────────────────────────────────────────
  · Reading a word as a line. It is a graph; the verb supplies the edges.
  · Pairing ◇/● by counting or by nearest-match. Pairing is ancestry over edges.
  · Putting only ← between ◇ and ● and expecting T. That is N (identity).
  · Expecting T from a word containing B. That is B (paradox held).
  · Treating an open arm as a failure. It is a living end, reported not fatal.
  · Closing a loop back to V. V has in-arity 0 and cannot be a target.
  · Pasting a bracketed word into the tool. Brackets parse to nothing → N (void).
```

The glyph alphabet is not invented: it references the per-token glyph vocabulary fixed
in `../ob3ect/READING_GUIDE.md` §3 (six are the guide's own midpoint glyphs; IFIX is its
stated "fix (¬)"; ENGAGR is the Belnap B it holds; AFWD/AREV are its forward/reverse
arrows; VINIT/TANCH keep their initials). Valences follow IMSCRIBr `tokens.py::TOKEN_ARITY`.
Run `imasm ref` for the live rules.
