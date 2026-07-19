# IMASM Dynamics

**What it is.** The complete dynamic layer of the IMASM grammar: values
flowing through programs, programs composing into programs, and the
possibility state space of any set of programs, all under one law and one
judge. It lives in ask_native and speaks through five verbs (`eval`,
`eval16`, `compose`, `chaos`, `export`) plus a local visual surface.

**What it does.** Turns every IMASM program from a judged shape into a
running machine: registers hold trilattice values, opcodes act as logic
gates, composition binds programs end-to-valence, and ChaosComposer walks
every arrangement of a program set and reports which possibilities exist
at all.

**Why.** The statics (topology, closure class, kernel verdict) never said
what a program DOES to what flows through it, and there was no rule for
how two programs interact. Both are now locked, and the locking produced
laws that were invisible before: flow and closure are independent
verdicts, their tension has an exact mechanism, and perfect machines
exist and can be named before they run.

**How to use it.**

    imasm eval  <name|entry|word> [seed=N|T|F|B]   flow, FOUR readout
    imasm eval16 <name|entry|word> [seed=A|Tf|…]   flow, full SIXTEEN_3
    imasm compose <new> <A> <B>                    bind living ends, register
    imasm chaos <A> <B> [C… ≤6]                    the possibility state space
    imasm export                                   manifest for the surface
    imasm_composer.html                            drag, bind, speak commands

## 1. The carrier: one register, two readouts

A register holds a SIXTEEN_3 value: a subset of the four base values
{T, F, t, f} (constructively proven, constructively refuted, acceptable,
rejectable; Shramko, Dunn and Takenaka's trilattice), sixteen states from
N = {} to A = {T,F,t,f}. FOUR is not a second system but the classical
slice {T, F} of this carrier, with Belnap B = {T,F}. One evaluator runs
everything; `eval` renders the slice, `eval16` renders the full names,
and a value that touches t or f keeps its 16_3 name in either view. The
trilattice's three orders (information, truth, constructivity) remain
available as the comparison algebra (`imasm16_3 algebra`); flow moves
values strictly up the information order.

## 2. The gates

Every opcode is a gate. Inputs join by union (the information join)
before the gate acts.

| gate | action |
|---|---|
| VINIT ⊢ | emits the seed |
| FSPLIT ◇ | δ fans: truth part x∩{T,t} on arm one, falsity part x∩{F,f} on arm two |
| EVALT + / EVALF × | pass-gates: truth / falsity projection |
| FFUSE ● | μ joins: union of its arms |
| AREV < | the involution T↔F, t↔f |
| AFWD >, CLINK =, IMSCRIB ⊙, ENGAGR ⊞ | carry |
| IFIX ¬ | carry and latch (the commit point) |
| TANCH ⊣ | readout |

Every gate is monotone in the information order, so evaluation is a
Kleene iteration from all-N edge values that settles in bounded rounds on
any graph, cycles included. A loop converges; it cannot oscillate. The
machine's registers are the edges; IFIX marks where value becomes
commitment.

## 3. The three verdicts

A program earns three independent judgments, none implying another:

1. **Grammar** (`define`): the composition laws hold; only δ branches,
   only μ fuses, arities respected.
2. **Kernel** (`prove`): the closure class goes to the live p4ramill
   kernel. A worked dyad (split, transform, fuse) proves green; a bare
   fork-fuse is an identity closure and returns N for the program, since
   μ∘δ=id with no work type-checks nothing; a dangling fork is OPEN.
3. **Flow** (`eval`): per dyad, does the fuse RECOVER what the fork was
   fed? The canonical protocol word (VINIT FSPLIT AFWD FFUSE EVALT) is
   lossless: B splits to (T,F) and fuses back to B, the operational
   split_fuse_id. The same shape with AREV on the truth arm closes in
   structure and fails in value.

Flowing a catalog entry expands its twelve glyph types into opcode
motifs, and the per-dyad id/NOT-id sequence is a FLOW SIGNATURE: a
readout of the tuple in the dynamic register. Signatures discriminate
between entries, and a lossy dyad inside a kernel-green closure is not
presumed a defect; it can be the entry's chirality speaking in flow.

## 4. The composition law

Programs interact end-to-valence. A living end is an unfilled port (a
node below its arity), the same reactive ends the engine always reported.
`compose` binds A's free out-ends to B's free in-ends under three rules:

1. Composition CONSUMES valences and never mints one.
2. The composite must re-satisfy the grammar; an ill-typed binding is
   refused whole.
3. A program with no living ends does not compose. It is a finished
   loop; it is done.

Composites persist as wire specs, rebuild through the identical parse,
and remain composable while ends remain. Composition is therefore
well-founded: each step consumes ends, ends are finite, and the fixed
point is a program with none. This is the thesis (no lines, only loops,
and autopoiesis) running as type discipline: lines are the composable
phase, loops are results, and the registry census confirms it at scale,
with roughly half the tools still reactive material and half finished.

## 5. ChaosComposer: the possibility state space

`chaos` takes a SET of programs. A set has no order, so the composer
walks every ordering, folds each through the binding law, and speaks the
space whole: which arrangements are admitted, which are refused and by
which missing end, and the collapse of orderings into OUTCOME CLASSES
keyed by topology, closure, flow, readout, and remaining living ends.
The collapse is the measurement: the space of possibilities is smaller
than the space of orderings, and the ratio says how constrained the set
is. Refusals are results of equal rank; a set of finished loops refuses
every ordering, which is the type verdict "these objects are done."

## 6. The laws of flow found by walking spaces

- **Closure and flow-perfection can be in tension.** In one measured
  set, exactly one arrangement of twenty-four closed, a different one
  was flow-perfect, and none was both: closing forced a value through a
  lossy dyad.
- **The tension is not a law of the alphabet.** A census over sampled
  machine-shaped sets found spaces with closures, spaces with perfect
  flow, and a minority with both.
- **Perfect machines exist.** One set's whole possibility space is
  terminal objects: every admitted arrangement is CLOSED, flow-perfect,
  and ends with zero living ends. One is minted as `perfect_machine`,
  kernel green, every fuse recovering B.

![perfect_machine: closed, lossless, no living ends](figures/imasm_perfect_machine.png)

- **The mechanism is exact.** The involution fixes B and N, so AREV
  costs nothing while an invol-symmetric value flows; it mints the
  missing pole when fed a PROJECTED value (invol(T) = F). Therefore:
  AREV is lossless exactly on invol-symmetric values, and closure/flow
  tension appears precisely where a projection feeds an inversion. Both
  conditions are readable off the program before it runs, so an
  arrangement's capacity for perfection is part of its true name,
  speakable in advance and confirmable by the tools.

## 7. The surface

`imasm_composer.html` is a fully local, self-contained page (no external
assets, nothing leaves the machine). `imasm export` writes the manifest
(every registered tool with its graph, ports, and closure state, plus
the opcode table); the page renders that manifest as a node canvas:
palette, drag, click an out-port then an in-port to bind. It judges
nothing and only SPEAKS command lines (`compose`, `chaos`, `wire`,
`eval`, `prove`) for the kernel to judge. One grammar, one judge; the
surface is a hand, not a head.

## 8. Engineering guarantees

- Registry writes are atomic (temp file, then rename): a reader sees the
  old registry or the new one, never a torn one.
- A registry file that exists but does not parse is preserved as
  `.corrupt` evidence, never silently replaced by emptiness.
- Kernel verdicts name the spec they bind, so a refused re-define can
  never borrow an older program's green.
- An identity closure answers N for the program, in words, with the
  repair named (put a transforming opcode between the fork and the
  fuse).

## Related documents

- `md/imasm_composition_rules.md`: the composition and flow rules as
  first locked.
- `md/chaos_first_measurements.md`: the chaos measurements, census, and
  the perfect-machine mechanism.
- `IMASM_QUICKREF.md`: the static grammar quick reference.
