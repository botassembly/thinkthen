#!/usr/bin/env bash
# The interrupt proof, parent half. Backgrounds the child directly (not a
# subshell, or the signal reaches the wrong process), sends SIGINT three
# seconds into a 9.7 s column, then reads the stub's counters after a
# settle to prove nothing was served past the interrupt's return.
set -euo pipefail
cd "$(dirname "$0")"
PORT="${1:-8215}"

Rscript interrupt_child.R &
CHILD=$!
sleep 3
kill -INT "$CHILD"
wait "$CHILD" || true

sleep 2
echo "stub at +2s:  $(curl -s "http://127.0.0.1:${PORT}/v1/stats")"
