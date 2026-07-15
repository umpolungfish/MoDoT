# CLINK L9: the formalization has not elaborated

`p4ramill/Imscribing/CLINK_L9.lean` reports "14 theorems, 0 sorries, all proved by
`native_decide` or `rfl`". The file does not elaborate. Every field of every definition
names a constructor that is defined nowhere in the tree.

`P_pm_sym`, `Phi_c_complex`, `H_inf`, `Omega_Z`, `R_dagger`, `S_n_n`, `G_broad` occur in
exactly one file: `CLINK_L9.lean` itself, which uses them. Zero constructor definitions
exist for any of them, in either `Imscribing/Primitives/Core.lean` or `Primitives/Core.lean`.
`Criticality.Phi_c` is retired notation.

**Zero sorries is trivially true of a file that never compiles.** A `sorry` grep returning
empty is not a proof and never was. The structural work below may well be sound. It is
simply not yet verified, and it currently sits in p4rakernel where the sorry-count reads as
settled.

---

## 1. Reference table: the live vocabulary

Every axis, its `Imscription` field, and its constructors as they are actually defined.
Use these tokens throughout. Nothing else resolves.

| Axis | field | constructors (in ordinal order) |
|------|-------|---------------------------------|
| Ð | `dim`  | `dead` `ash` `array` `if'` |
| Þ | `top`  | `judge` `eat` `mime` `oil` `are` |
| Ř | `rel`  | `ado` `tot` `ear` `ian` |
| Φ | `pol`  | `church` `yew` `out` `nun` `or'` |
| ƒ | `fid`  | `age` `they` `peep` |
| Ç | `kin`  | `yea` `loll` `egg` `on` `air` |
| Γ | `gram` | `vow` `gag` `measure` `ooze` |
| ɢ | `gran` | `bib` `thigh` `ice` |
| ⊙ | `crit` | `woe` `monad` `roar` `err` `haha` |
| Ħ | `chir` | `fee` `kick` `sure` `wool` |
| Σ | `stoi` | `hung` `so` `up` |
| Ω | `prot` | `awe` `oak` `ah` `zoo` |

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

You **MUST** verify by ELABORATION, not by grep. A file that has not been elaborated has
not been checked, whatever its sorry count says. State plainly which theorems elaborated
and which did not.

You **MUST** report a theorem that fails to elaborate as a frontier and escalate, never as
a defeat. A red build is held, not refuted.

---

## 3. The open structural question

Do not treat this as settled in either direction. It is the reason the work matters.

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
