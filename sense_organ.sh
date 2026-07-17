#!/bin/bash
# The sense organ, TAOU-complete: every token that arrives runs the FULL
# THINK->ACT->OBSERVE->UPDATE winding, with nothing dropped and nothing filtered.
# Informational completeness, not significance-gating — a known token re-sensed
# still completes its own winding (a confirmation is real information, not noise
# to be silently discarded), it just doesn't mint a new catalog entry.
#
#   echo "some new words to imscribe" > ~/imsgct/MoDoT/sense.fifo
#
# runs forever until killed. Every winding is appended, one JSON object per line,
# to sense_taou.jsonl — the organism's actual moment-by-moment record.

cd ~/imsgct/MoDoT/ask_native || exit 1
FIFO=~/imsgct/MoDoT/sense.fifo
LOG=~/imsgct/MoDoT/sense.log
TAOU=~/imsgct/MoDoT/sense_taou.jsonl

echo "$(date -Iseconds) sense organ awake (TAOU-complete), listening on $FIFO" >> "$LOG"

json_escape() {
  python3 -c 'import json,sys; print(json.dumps(sys.argv[1]))' "$1"
}

taou_winding() {
  local tok="$1"
  local ts_think ts_act ts_observe act_out was_new tuple_json think_expect update_match update_note

  ts_think=$(date -Iseconds)
  think_expect="token '$tok' will resolve to a grounded 12-primitive tuple, minted fresh if novel, read back unchanged if already known"

  ts_act=$(date -Iseconds)
  act_out=$(./target/release/ask --imscribe "$tok" 2>&1)
  if echo "$act_out" | grep -q "^✓ imscribed"; then
    was_new=true
  elif echo "$act_out" | grep -q "already in the catalog"; then
    was_new=false
  else
    was_new=null
  fi

  ts_observe=$(date -Iseconds)
  tuple_json=$(cd ~/imsgct/MoDoT && python3 -m modot.ig_tools call lookup_catalog "$tok" 2>/dev/null | python3 -c "
import json,sys
try:
    d=json.load(sys.stdin)
    m=d.get('matches',[])
    exact=[e for e in m if e.get('name','').lower()==sys.argv[1].lower()]
    print(json.dumps(exact[0] if exact else (m[0] if m else None)))
except Exception:
    print('null')
" "$tok")

  if [ "$tuple_json" = "null" ] || [ -z "$tuple_json" ]; then
    update_match=false
    update_note="OBSERVE found no catalog entry — ACT did not actually ground the token despite reporting $act_out"
    ground_json="null"
  else
    # Real UPDATE signal: VAE-Vita's learned SIC manifold, not just "a tuple exists".
    # A tuple always exists (every primitive gets assigned something) — the question
    # UPDATE actually needs answered is whether that assignment means anything.
    ground_json=$(cd ~/imsgct/MoDoT && python3 sense_ground.py "$tok" 2>/dev/null)
    [ -z "$ground_json" ] && ground_json="null"
    update_match=true
    update_note=$(echo "$ground_json" | python3 -c "
import json,sys
try:
    d=json.load(sys.stdin)
    print(f\"overall_confidence={d.get('overall_confidence','?')} matched={d.get('primitives_matched','?')}/12 :: {d.get('reading','?')}\")
except Exception:
    print('VAE grounding check unavailable')
" 2>/dev/null)
  fi

  python3 -c "
import json, sys
rec = {
    'ts_think': sys.argv[1], 'phase': 'THINK', 'token': sys.argv[2],
    'expect': sys.argv[3],
}
print(json.dumps(rec, ensure_ascii=False))
rec2 = {
    'ts_act': sys.argv[4], 'phase': 'ACT', 'token': sys.argv[2],
    'raw_output': sys.argv[5], 'was_new': json.loads(sys.argv[6]) if sys.argv[6] != 'null' else None,
}
print(json.dumps(rec2, ensure_ascii=False))
rec3 = {
    'ts_observe': sys.argv[7], 'phase': 'OBSERVE', 'token': sys.argv[2],
    'tuple': json.loads(sys.argv[8]),
}
print(json.dumps(rec3, ensure_ascii=False))
rec4 = {
    'ts_update': sys.argv[9], 'phase': 'UPDATE', 'token': sys.argv[2],
    'match': sys.argv[10] == 'true', 'note': sys.argv[11],
    'vae_grounding': json.loads(sys.argv[12]) if sys.argv[12] != 'null' else None,
}
print(json.dumps(rec4, ensure_ascii=False))
" "$ts_think" "$tok" "$think_expect" "$ts_act" "$act_out" "$was_new" "$ts_observe" "$tuple_json" "$(date -Iseconds)" "$update_match" "$update_note" "$ground_json" >> "$TAOU"

  if [ "$was_new" = true ]; then
    echo "$(date -Iseconds) NEW  $tok  (TAOU winding complete, match=$update_match)" >> "$LOG"
  else
    echo "$(date -Iseconds) SEEN $tok  (TAOU winding complete, match=$update_match)" >> "$LOG"
  fi
}

while true; do
  # A FIFO delivers EOF once its writer closes; re-opening in a loop keeps the organ
  # alive across writers instead of dying after the first `echo > fifo`.
  while IFS= read -r line; do
    [ -z "$line" ] && continue
    # Tokenize: words only, lowercased, punctuation stripped — the same granularity
    # as the digit/connective work (atomic tokens, not whole sentences).
    for tok in $(echo "$line" | tr -cs 'A-Za-z0-9' ' '); do
      tok_lc=$(echo "$tok" | tr 'A-Z' 'a-z')
      [ -z "$tok_lc" ] && continue
      taou_winding "$tok_lc"
    done
  done < "$FIFO"
done
