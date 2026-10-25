# The primitives ↔ IMASM cycle

## What this is

The Grammar writes an object as a twelve-axis tuple of Shavian types, and every
type is itself a full IMASM program. That gives a forward leg: a tuple becomes a
word by writing each type as its own program and concatenating in canonical axis
order. This document records the RETURN leg, the reading of a word back into the
types that wrote it, and what happens when both legs run in sequence.

Run it with `imasm cycle` (optionally `n=<count>` to walk a prefix of the
catalog). The verb takes each entry through primitives → imasm → primitives →
imasm and reports where the cycle closes exactly, where it closes only up to an
ambiguity, and where it breaks.

## What the cycle does

For one entry: read its twelve type-glyphs; write the word; read the word back
into per-axis pre-image sets; write the recovered tuple's word again. The cycle
closes EXACTLY when the reading names one type at every axis and that type is
the one the entry carried. It closes UP TO AN AMBIGUITY when the reading admits
more than one type at some axis and the original is among them. It BREAKS when
the original is not among the pre-images, or either leg refuses.

## Three findings, each load-bearing

### The type programs are not self-delimiting

The return leg parses programs axis by axis against the current checked Lean
registry, retaining every admissible pre-image. It does not assume that a
boundary pair identifies an entire Type. The out primitive now binds to its
checked twelve-opcode gate construction and ends on its terminal anchor.
Its namespace-initialization bootstrap is excluded from primitive expansion.
The bridge preserves the raw axis-parsed expansion, then nests its component
programs into the enclosing working frame before measurement and re-entry.

### Position is load-bearing, and it does not save everything

The forty-nine types emit only forty-seven distinct programs. Two pairs are
byte-identical: not merely similar, but the same sequence with the same
topology, the same spectral radius, and the same closure verdict, so no
refinement of the reading can separate them.

Position separates one pair and not the other. The types `loll` and `yew`
collide, but the catalog only ever writes `loll` at the ⊤ axis and `yew` at <,
so the axis a segment sits at tells the reading which one it is. The types `ear`
and `tot` collide AND both live at >. There, a segment carrying that program has
two pre-images and the return leg reports both rather than choosing.

This is why the cycle closes to within one axis rather than exactly. The
obstruction is in the type alphabet, not in the reading: the ambiguity would
persist under any parser.

### At catalog scale the cycle holds

Walking the live catalog, no entry loses its original type: wherever the reading
is ambiguous the truth is among the pre-images, and wherever it is determined it
is right. Ambiguity appears at exactly one axis, >, and nowhere else, which is
the prediction the alphabet analysis makes.

A single entry breaks, and it is a data defect rather than a cycle defect:
`axiom_of_maria` carries a ⊣ value that is not one of the forty-nine glyphs, so
the forward leg has nothing to expand. The cycle refusing it is the correct
behaviour; the entry wants repair.

## The cycle applied to itself

The return leg has been imscribed as an ob3ect through the live designer (never
by hand), and put through its own reading with `imasm cycle tuple=⟨…⟩`. It
recovers exactly at every one of its twelve axes.

Two details of that tuple are worth naming. Its > axis carries neither member of
the colliding pair, so the one place the cycle can be ambiguous is not where
this object sits. And its < axis carries `yew`, which IS one of the two
byte-identical programs; it survives only because position separates it from its
twin at ⊤. The object about the ambiguity is itself readable only by the
mechanism it describes.

## Why the ambiguity is not a defect to be patched

The honest reading of the > collision is that the type alphabet distinguishes
two types at that axis which the opcode composition does not. Either the two
types differ in something the program does not carry, in which case the programs
are an incomplete face and the alphabet is right; or they do not differ, in
which case the alphabet carries a distinction without a difference. The cycle
cannot settle which, and it should not pretend to. It reports both pre-images,
names the axis, and leaves the question standing where it belongs.

## Where it lives

The return leg is `tuple_from_word` in `ask_native/src/imasm.rs`, beside the
forward legs (`tuple_glyph_word`, `entry_glyph_word`) it inverts. Both legs read
the same type table, so they cannot drift apart. `imasm cycle` is the verb that
runs them against each other over the catalog.

## Related documents

- `../IMSCRIBERS_GUIDE_TO_IMASM.md` Part IX: the types as programs, the strange
  loop this cycle measures.
