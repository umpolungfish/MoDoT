#!/bin/bash
# Decouples the fast side (your terminal, writing instantly to sense_intake.log)
# from the slow side (sense_organ.sh, one LLM call per novel token). Without this,
# toggling terminal output straight into sense.fifo would block your shell the
# moment the pipe's kernel buffer filled and the reader couldn't keep up.
INTAKE=~/imsgct/MoDoT/sense_intake.log
FIFO=~/imsgct/MoDoT/sense.fifo
touch "$INTAKE"
tail -n0 -F "$INTAKE" > "$FIFO"
