# MoDoT native `ask`

Host-native language interface for the organism. **Lives in MoDoT** — non-Python replacement for `momonados_agent.py --ask / --file / -i`.

## Provider & model

```bash
export MODOT_PROVIDER=openrouter          # or: gemini
export MODOT_MODEL=google/gemini-3-pro-preview

# keys (by provider)
export OPENROUTER_API_KEY=…               # when MODOT_PROVIDER=openrouter
export GEMINI_API_KEY=…                   # when MODOT_PROVIDER=gemini
```

Or per-run:

```bash
./ask --provider openrouter --model google/gemini-3-flash-preview --file questions/q7.txt
./ask --provider gemini --model gemini-2.0-flash -i
```

| Variable | Values |
|----------|--------|
| **`MODOT_PROVIDER`** | `openrouter` \| `gemini` |
| **`MODOT_MODEL`** | model id (OpenRouter-style or bare Gemini id) |

Defaults: provider inferred from which key is set (`openrouter` preferred); model `google/gemini-3-flash-preview`.

## Run

```bash
cd MoDoT
./ask --file questions/q7.txt
./ask --ask "…"
./ask -i
```

From repo root: `./ask …` or `./MoDoT/ask …`
