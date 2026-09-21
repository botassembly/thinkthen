#!/usr/bin/env bash
# The fast-backend cancel proof, end to end through the CLI (lane B item 5):
# against the null backend — no stub, so the wait never idles at 300 ms — a
# long query interrupted at t+1 s must end within about a poll tick, not
# after the whole batch (the poll-bug shape: SIGINT one second into a
# three-million-record null batch once surfaced 8.48 s later, at batch end).
#
# What this proves and what it cannot, stated plainly: it proves the CLI's
# SIGINT path ends a long null-backend query promptly and never hangs. It
# cannot isolate the engine's own tick, because DuckDB's abort ends a
# native query within a chunk under the same signal (measured 2026-09-21);
# the crate's `cancel_tests` Rust test carries that discriminating proof —
# the same 8M-record batch with no cancel ran 53.55 s, so its 1.5 s bound
# cannot be met by a batch that runs to completion. The stub-backed wire
# suite proves the wire-side shape at 300 ms.
set -euo pipefail
cd "$(dirname "$0")"
ROOT=$(cd .. && pwd)
CLI="$ROOT/duckdb-bin/duckdb"
EXT="$ROOT/build/release/thinkthen.duckdb_extension"
OUT=$(mktemp)
SQL="LOAD '$EXT'; SELECT count(thinkthen_decide('Is this a complaint?', 'i want a refund now')) FROM range(3000000);"

ENGINE_NULL=1 "$CLI" -unsigned -noheader -list -c "$SQL" > "$OUT" 2>&1 &
pid=$!
sleep 1
start=$(date +%s.%N)
kill -INT "$pid"
rc=0
wait "$pid" || rc=$?
end=$(date +%s.%N)
elapsed=$(awk -v a="$start" -v b="$end" 'BEGIN { printf "%.2f", b - a }')
echo "the CLI exited $rc ${elapsed}s after SIGINT, one tick expected"

status=0
if awk -v e="$elapsed" 'BEGIN { exit !(e <= 2.0) }'; then
    echo "ok       the interrupt ended the query within about a tick (${elapsed}s)"
else
    echo "FAILED   the interrupt waited ${elapsed}s; the query was not stoppable"
    status=1
fi
if grep -q "3000000" "$OUT"; then
    echo "FAILED   the query ran to completion and printed its count"
    status=1
else
    echo "ok       the query did not run to completion"
fi
rm -f "$OUT"
exit "$status"
