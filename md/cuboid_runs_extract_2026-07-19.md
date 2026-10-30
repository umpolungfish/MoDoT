# Transcript review: the perfect-cuboid runs (2026-07-19 evening)

Mined from `crystal_fs/tool_calls.jsonl`. Two runs, both on the perfect-cuboid campaign, launched after the day's regimen ob3ect landed. The auto-designer itself leaves no tool-call trace (the ob3ect JSON appeared with no run active); only agent runs log.

## Run 19f7cc65776-3f27da (16:46 to 17:05, 268 calls)

The unhealthy one. Three pathologies, all harness-shaped:

1. **`imasm prove cuboid_descent` called 86 times with identical arguments and identical output**, exactly one call per round for 17 minutes. The agent heard the same verdict every round and re-asked anyway. The per-round regularity points at the harness prompt (a re-verify step executed every round), not at the model deciding 86 separate times. A third of the run's budget burned on one already-answered question.
2. **34 calls carried a literal `…` placeholder as an argument** (`set` 20, `click` 14), plus flags the tools do not take (`--excite` passed to `set`). The agent was emitting template ellipsis from help text or truncated echoes instead of real catalog names. Every one missed.
3. **Two genuine kernel refusals** (`define cuboid_close`, `define cuboid_descent` in ill-typed forms) before a well-typed `cuboid_descent` was admitted (protocol ⊢ ⊙ ◇ + > < × > < ● ¬ ⊣). This part is the gate working as designed.

## Run 19f7cdabfde-3f43db (17:08 onward, 174+ calls)

The healthy sibling: catalog lookups, cl9nk entry reads, distance computations, one ob3ect design call, three refusals total. Same campaign, no loop, no placeholder args.

## Verdicts

- The gate refused ill-typed tools and admitted the well-typed one. Kernel behavior correct throughout.
- The 86-fold re-prove and the `…` arguments are harness defects to fix in the round prompt: do not instruct re-verification of an already-proved tool every round, and never let un-substituted placeholder text reach a tool line.
- Blind spot: the ob3ect auto-designer writes no tool-call records. If its transcripts matter for review, it needs the same logging.

## Day totals for context

1712 calls across 35 runs. The morning block (06:10 to 11:10, six runs) worked residual_notation_floor, monotone_integer_winding, extradimensional_entity, and perfect_cuboid_lifted. The vita mouth's oracle calls (speak, corpus, harvest) go through the ask CLI directly and do not appear in tool_calls at all; only agent runs log there.
