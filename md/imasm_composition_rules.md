# IMASM composition and flow: the locked rules

The engine long judged shape only. As of this document the dynamics are
locked too: what a register is, what each gate does, what information gets
shuttled where, and the one law by which programs compose. Implemented in
ask_native (imasm eval, imasm eval16, imasm compose); every rule below is
executable and kernel-adjacent, not aspirational.

## The carrier: one register, two readouts

The register is a SIXTEEN_3 value: a subset of the four base values
{T, F, t, f} (Shramko, Dunn and Takenaka's trilattice; T proven, F refuted,
t acceptable, f rejectable; sixteen states from N = {} to A = {T,F,t,f}).
FOUR is not a second system: it is the classical slice {T, F} of the same
carrier, with Belnap B = {T,F} and N = {}. One evaluator runs both; `eval`
renders the slice (N/T/F/B), `eval16` renders the full names. A value that
touches t or f has left the slice and keeps its 16_3 name in either view.

The trilattice's three orders remain the algebra of comparison (imasm16_3
algebra): ≤_i information (subset), ≤_t truth, ≤_c constructivity. Flow
uses ≤_i: shuttling only ever moves values up the information order.

## The gates

Each opcode is a gate over the carrier. Inputs join by union (the ≤_i join)
before the gate acts; the value leaving a gate rides every out-edge except
where δ fans.

| gate | action on the value |
|---|---|
| VINIT ⊢ | emits the seed (default B in the slice, A in full 16_3) |
| FSPLIT ◇ | δ fans: truth part x∩{T,t} on arm one, falsity part x∩{F,f} on arm two |
| EVALT + | pass-gate: truth part |
| EVALF × | pass-gate: falsity part |
| FFUSE ● | μ joins: union of its two arms |
| AREV < | the involution T↔F, t↔f (its own inverse) |
| AFWD >, CLINK =, IMSCRIB ⊙, ENGAGR ⊞ | carry |
| IFIX ¬ | carry and latch (the commit point) |
| TANCH ⊣ | readout |

Every gate is monotone in ≤_i, so evaluation is a Kleene iteration from
all-N edge values that settles in bounded rounds even on cyclic graphs. A
loop converges; it cannot oscillate. The registers of the machine are the
edges; IFIX marks where a value becomes commitment.

## Structure and flow are separate facts

The kernel's closure judgment (shape: does a worked dyad exist) and the
value judgment (flow: does the fuse recover what the fork was fed) are
independent measurements, and eval reports the second explicitly:

- `VINIT FSPLIT AFWD FFUSE EVALT` closes in shape AND in value: B splits
  to (T, F), fuses back to B. Lossless, the operational split_fuse_id.
- The same word with AREV on the truth arm still closes in shape, but the
  values run fed Tf, recovered Ff: NOT id. The arm inverted what it
  carried, and only flow can see that.

A full verification is therefore three verdicts, none implying another:
grammar (composition laws hold), kernel (the closure class proves green),
flow (μ∘δ = id on values, or a measured deviation).

## The composition law

Programs interact end-to-valence. A living end is an unfilled port: a node
whose in-degree or out-degree is below its arity. These are the reactive
ends the engine has always reported ("living ends: reactive, not errors");
they are the register ports of the program as a component.

`imasm compose <new> <A> <B>` binds A's free out-ends to B's free in-ends,
in node order, and the law is:

1. Composition CONSUMES valences and never mints one.
2. The composite must re-satisfy the grammar (only δ branches, only μ
   fuses); an ill-typed binding is refused whole.
3. A program with no living ends does not compose. It is a finished loop;
   it is done. Building is the act of consuming reactive ends, and the
   terminal object of composition is a closed worked loop.

Rule 3 is the thesis operating as type discipline: lines (open chains) are
the composable phase; loops are results. The first composite built under
the law demonstrated it exactly: two open chains, individually mere lines,
bound at two ends into a CLOSED worked dyad that flows losslessly and
proves green at the kernel.

Composites persist in the registry as wire specs (nodes / edges), so they
rebuild through the identical parse and are themselves composable while
they still have living ends. Continual composition is therefore
well-founded: each step consumes ends, ends are finite, and the fixed
point of composing is a program with none.

## Usage

    imasm eval  <name|word> [seed=N|T|F|B]      flow in the FOUR slice
    imasm eval16 <name|word> [seed=N|A|TFtf…]   flow over full SIXTEEN_3
    imasm compose <new> <A> <B>                 bind living ends, register
    imasm prove <name|word>                     closure class at the kernel
    imasm16_3 algebra <op> A B                  the three orders, meets, joins

The three-verdict discipline for any new program: define (grammar), prove
(kernel), eval (flow). Speak the expected readout before running eval; the
gates are deterministic, so the true name of a topology includes its flow.
