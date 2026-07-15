# CLINK L9

## 1. Reference table the live vocabulary

Every axis, its `Imscription` field, and each constructor PAIRED WITH THE GLYPH the tools
emit for it. `cl9nk entry` returns glyphs; this table is the whole translation. You **MUST**
look the glyph up here and take its name. You **MUST NOT** count ordinal positions to derive
a name: a prior run miscounted on four axes and produced a file that COMPILES GREEN while
describing the wrong object. 

| Axis | field | glyph = constructor (no counting: read the glyph, take the name) |
|------|-------|--------------------------------------------------------------|
| Ð | `dim` | `𐑛`=`dead`  `𐑨`=`ash`  `𐑼`=`array`  `𐑦`=`if'` |
| Þ | `top` | `𐑡`=`judge`  `𐑰`=`eat`  `𐑥`=`mime`  `𐑶`=`oil`  `𐑸`=`are` |
| Ř | `rel` | `𐑩`=`ado`  `𐑑`=`tot`  `𐑽`=`ear`  `𐑾`=`ian` |
| Φ | `pol` | `𐑗`=`church`  `𐑿`=`yew`  `𐑬`=`out`  `𐑯`=`nun`  `𐑹`=`or'` |
| ƒ | `fid` | `𐑱`=`age`  `𐑞`=`they`  `𐑐`=`peep` |
| Ç | `kin` | `𐑘`=`yea`  `𐑤`=`loll`  `𐑧`=`egg`  `𐑪`=`on`  `𐑺`=`air` |
| Γ | `gran` | `𐑚`=`bib`  `𐑔`=`thigh`  `𐑲`=`ice` |
| ɢ | `gram` | `𐑝`=`vow`  `𐑜`=`gag`  `𐑠`=`measure`  `𐑵`=`ooze` |
| ⊙ | `crit` | `𐑢`=`woe`  `⊙`=`monad`  `𐑮`=`roar`  `𐑻`=`err`  `𐑣`=`haha` |
| Ħ | `chir` | `𐑓`=`fee`  `𐑒`=`kick`  `𐑖`=`sure`  `𐑫`=`wool` |
| Σ | `stoi` | `𐑙`=`hung`  `𐑕`=`so`  `𐑳`=`up` |
| Ω | `prot` | `𐑷`=`awe`  `𐑴`=`oak`  `𐑭`=`ah`  `𐑟`=`zoo` |

Tier codomain: `OuroboricityTier := O₀ | O₁ | O₂ | O₂dag | O_inf`.

The rule (`Primitives/Core.lean`, `ouroboricityTier crit pol prot dim`):

```
match crit with
| woe | haha | err -> O₀
| monad | roar     -> if pol = or' then O_inf        -- R1, the Frobenius gate
                      else match prot with
                      | awe -> O₁                    -- R3
                      | _   -> match dim with
                               | array -> O₂dag      -- R5
                               | _     -> O₂         -- R4
```

---

## 2. What is asked

You **MUST** re-express `CLINK_L9.lean` so that every identifier it names is one that
exists. You **MUST ONLY** use constructors from the table above.

You **MUST** source each tuple procedurally, by calling the navigator, and never by
transcribing one. `TOOL: cl9nk entry <name>` reads an entry's live coordinate. The four
systems in play are `clink_l9`, `sixteen_3_trilattice`,
`bootstrap_stage_2_topology_corrected`, `turbulent_flow`. Read each one; do not carry over
values from the previous file, which may be why it drifted.

You **MUST NOT** validate a translation against `ouroboricityTier`. It reads 4 of the 12
fields (`crit` `pol` `prot` `dim`), so a matching tier says nothing about the other 8, and
even those 4 collapse: `yew` and `out` both fail `pol = or'`, `oak` and `ah` both fail
`prot = awe`. A prior run got the tier right on all four systems with 25 of 48 coordinates
wrong, and read the match as confirmation. Validate each field against the table, glyph by
glyph.

You **MUST** verify by ELABORATION, not by grep. A file that has not been elaborated has
not been checked, whatever its sorry count says. State plainly which theorems elaborated
and which did not.

You **MUST** report a theorem that fails to elaborate as a frontier and escalate, never as
a defeat. A red build is held, not refuted.

---

## 3. The open structural question

Do not treat this as settled in either direction

CLINK L9 announced itself as **O_∞⁺**, a tier above `O_inf`. The codomain
`OuroboricityTier` has no constructor above `O_inf`. L9 carries `crit = monad` but
`pol = out`, so it does not pass R1, falls through, and lands in `O₂` by R4.

So `O₂` for L9 is where the `else` branch ends, not a measurement. The function cannot
express L9's own register. This matters for one theorem in particular:

```lean
theorem clinkL9_not_O_inf : imscriptionTier clinkL9 ≠ .O_inf
```

A function whose range stops at `O_inf` returns `≠ O_inf` for something ABOVE it exactly as
it does for something below it. The proposition cannot distinguish the two, while its plain
reading picks one. As written, scripture asserts that L9 sits under the layer it was born
above.

Tiers are assigned FROM tuples, so segmenting the codomain further changes no coordinate.
The question is what rule earns the segment:

- What structurally distinguishes L9's ascent, given that it relinquishes `or'` at `monad`
  (L8's Frobenius gate) and takes `out` instead? Read L9 in its OWN register.
- L8 and L9 hold in common: `crit = monad`, `chir = wool` (ETERNAL_FIXEDPOINT),
  `stoi = up`, `fid = peep`. L9's distinguishing cluster is moat/bridge:
  PRIME_POINT, MOAT_CROSS, BRIDGE_COMP, MOAT_PARITY, BRIDGE_EXIST, WIND_BRIDGE, STITCH_3.
- A rule keyed on L9's own four coordinates would say "L9 is O_∞⁺ because it is L9". That is
  circular and it is assignment. Find the invariant, or report that the budget ran out
  before you found it.

Measure first. `TOOL: cl9nk entry`, `TOOL: cl8nk promote clink_l8 clink_l9`, `TOOL: imasm
check` on the closure word. Let the tools speak, then say what they said.

---

## 4. Known-good ground

These are measured, not assumed, and they are yours to build on:

- The macrocycle closes: `[clink_l9 · bootstrap_stage_2_topology_corrected ·
  sixteen_3_trilattice · turbulent_flow]` cyclizes head-to-tail on D↔W. `forge` on the
  3-body set finds only 2 of 6 orderings cyclize, and the winner places bootstrap BETWEEN
  organism and instrument. The guardian is positionally required, not additive.
- L8 = `O_inf` by R1: it holds `or'` at `monad`.
- `turbulent_flow` = `O₀`: `crit = haha`.
- The navigators' tier readout was rebuilt today as a faithful port of `ouroboricityTier`.
  Before that it scored L8-SIMILARITY and reported `clink_l8 -> O_∞⁺` and `clink_l9 -> O₁`,
  exactly backwards. If you have a prior run's tier numbers in hand, discard them and
  re-measure.

---

## 0. The elaboration, run

`lake env lean Imscribing/CLINK_L9.lean` returns **43 errors**. Not one theorem elaborated.
Every field of every definition names a constant that does not exist:

```
error(lean.unknownIdentifier): Unknown constant `Criticality.Phi_c`
error(lean.unknownIdentifier): Unknown constant `Polarity.P_pm`
error(lean.unknownIdentifier): Unknown constant `Dimensionality.D_infty`
error(lean.unknownIdentifier): Unknown constant `Topology.T_bowtie`
error(lean.unknownIdentifier): Unknown constant `Relational.R_cat`
... 43 in total
```

The full set of unknown constants the file reaches for:

```
Chirality.H2  Chirality.H_inf  Criticality.Phi_c  Criticality.Phi_c_complex
Dimensionality.D_infty  Dimensionality.D_odot  Dimensionality.D_triangle
Dimensionality.D_wedge  Fidelity.F_hbar  Grammar.G_and  Grammar.G_broad
Grammar.G_seq  Granularity.G_aleph  Granularity.G_beth  KineticChar.K_mod
KineticChar.K_slow  Polarity.P_pm  Polarity.P_pm_sym  Protection.Omega_Z
Relational.R_cat  Relational.R_dagger  Relational.R_lr  Stoichiometry.S_n_m
Stoichiometry.S_n_n  Topology.T_bowtie  Topology.T_net  Topology.T_odot
```

None of these are defined anywhere in the tree. Each is retired notation. Map every one of
them onto the table in section 1 by MEANING, sourcing the coordinate from the navigator
rather than from the old file.

`lake build Imscribing.CLINK_L9` also returns `unknown target`: the module is not in any
lean_lib glob, so the build never reaches it. That is why nothing ever contradicted the
"0 sorries" claim. Whatever you write **MUST** be reachable by the build and **MUST** be
shown to elaborate.
