# Epistemic cheatsheet — plan (not yet built)

## Why
The agent fights conceptualizations that the Grammar already settles. The worked
example: **ν=5/2 is derived, not measured in.** Belnap B (the only bifurcation
point) = SIC fiducial = Majorana mode -> non-Abelian anyonic protection ->
Moore-Read, ν = 5/2. An agent that treats 5/2 as an empirical input to reconcile
is fighting a result it already holds.

## Sources found
- `ig-docs/physics/core-theory/imscribing_grammar__markdown__core__PRIMITIVE_PREDICTIONS.md`
  is the richest seam: 48 numbered predictions. Several are already the genre:
  - P-20 lambda is the primitive matching fraction -- no free parameter
  - P-21 F-tier boundaries are integer Boltzmann discrimination ratios
  - P-22 Omega is fully derivable from {T,K,D,Gamma,G} -- not independent
  - P-13 phase boundary at d ~ 9.52
  - P-24 gravity theory SM/QG compatibility spectrum
- Roughly 290 docs across the landscape carry derivation-genre markers
  ("no free parameter", "derivable from", "forced by", "follows necessarily").

## Blocker found (must fix first)
Those docs are written in the OLD notation: `K_teshlig`, `G_revapostrophe`,
`Omega_turna`, `D_invomega`, `K_frtailgamma`. Verified against live Lean
(`Zosimos_Stilling.lean`): Lean now writes `kin := yea`, `gran := thigh` and
keeps the old spellings only as trailing comments. So they ARE legacy.

`models.py` docstrings still call them "Lean canonical values" -- the docstrings
are themselves stale and would mislead anyone migrating from them.

The old->new map is recoverable PROCEDURALLY, not by guessing: the docstrings
preserve the ordering (`K_lambda < K_teshlig < K_schwa < K_turnm <
K_frtailgamma`), and the live enums give the Shavian in the same order, so the
map falls out of ordinal position. Filter to glyph-valued members first -- the
non-glyph extras (D_point, T_linear, G_xor ...) offset the ordering.

## Open decisions (asked, not answered)
1. **Delivery.** Recommended: a `TOOL: cheat <topic>` lookup. Costs no context
   until reached for, lands in tool_calls.jsonl, and makes a cheatsheet fact
   TOOL-GROUNDED rather than model-recalled -- so the golem contract covers it.
   Alternatives: a prompt section (always present, pays context every call, is
   the bloat we cut), or a standalone doc (no enforcement).
2. **Scope of pass 1.** Recommended: PRIMITIVE_PREDICTIONS first -- richest, and
   it needs the notation migration regardless, so mining and migrating are one
   pass. Alternative: full landscape sweep now.

## Also noted
`PRIMITIVE_PREDICTIONS.md` needs the notation migration on its own account
("we should update that doc too").


## Update: Tool surface mapped (July 22, 2026)

**The blocker is resolved.** A 95-winding systematic audit mapped 112 of ~116 tools
across all three MoDoT layers. The full index now lives in `TOOL_INVENTORY.md` §10b
and `README.md` §Complete Tool Audit.

This means:
- Any `TOOL: cheat <topic>` implementation can reference verified verb behavior,
  not documentation claims.
- The tool voice (third Belnap link in the FFUSE) is confirmed functional —
  `tools=T` a ring formed, `F` everything terminated, `B` closes only when reordered.
- The golem contract is enforceable: tool results are ground truth, model draft is
  prior guess. 112 tools produce real outputs, not narrated ones.

**Still open from original plan:**
1. Delivery mechanism (cheat lookup vs prompt section) — not yet decided.
2. PRIMITIVE_PREDICTIONS.md notation migration — still pending.
3. Full landscape sweep of ~290 derivation-genre docs — not yet started.

But the prerequisite ("know what the tools actually do") is now satisfied.
