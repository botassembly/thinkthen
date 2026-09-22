#!/usr/bin/env bash
# The interrupt proof, parent half. Backgrounds the child directly (not a
# subshell, or the signal reaches the wrong process), sends SIGINT three
# seconds into a 9.7 s column, then reads the stub's counters at +1 s and
# +3 s AFTER the signal while the child is still alive and sleeping, and
# again after the child exits. The counter must be identical at every
# read: nothing served past the interrupt's return, proven before process
# exit.
set -euo pipefail
cd "$(dirname "$0")"
PORT="${1:-8215}"

Rscript interrupt_child.R &
CHILD=$!
sleep 3
kill -INT "$CHILD"

sleep 1
echo "parent at +1s: $(curl -s "http://127.0.0.1:${PORT}/v1/stats")"
sleep 2
echo "parent at +3s: $(curl -s "http://127.0.0.1:${PORT}/v1/stats")"

wait "$CHILD" || true

sleep 1
echo "parent at +5s: $(curl -s "http://127.0.0.1:${PORT}/v1/stats")"
