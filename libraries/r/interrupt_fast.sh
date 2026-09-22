#!/usr/bin/env bash
# The fast-backend interrupt proof, parent half (lane B item 5, the
# poll-bug shape). Backgrounds the child directly (not a subshell, or the
# signal reaches the wrong process), sends SIGINT one second into a
# two-million-record null-backend column, and requires the child to report
# the interrupt within 1.5 s — the batch runs about 9 s deaf, so the
# threshold separates the two behaviors. Offline; check.sh calls it.
set -euo pipefail
cd "$(dirname "$0")"

export ENGINE_NULL=1
OUT=$(mktemp)

Rscript interrupt_fast_child.R >"$OUT" 2>&1 &
CHILD=$!
sleep 1
kill -INT "$CHILD"
wait "$CHILD" 2>/dev/null || true

cat "$OUT"
ELAPSED=$(sed -n 's/^elapsed: //p' "$OUT")
OUTCOME=$(sed -n 's/^outcome: //p' "$OUT")
AFTER=$(sed -n 's/^after: //p' "$OUT")
rm -f "$OUT"

if [ "$OUTCOME" != "interrupt" ]; then
  echo "FAIL: the child answered '$OUTCOME' instead of interrupting"
  exit 1
fi
if [ -z "$ELAPSED" ] || ! awk -v one="$ELAPSED" 'BEGIN { exit !(one < 1.5) }'; then
  echo "FAIL: the interrupt landed at ${ELAPSED:-unknown} s (the batch runs about 9 s deaf)"
  exit 1
fi
if [ "$AFTER" != "TRUE" ]; then
  echo "FAIL: the call after the interrupt did not answer"
  exit 1
fi
echo "OK: the interrupt landed at ${ELAPSED} s, within a tick of the signal, and the session answered after it"
