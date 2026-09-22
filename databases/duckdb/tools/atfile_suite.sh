#!/usr/bin/env bash
# The @file question file is read once per query, not once per row
# (review item 7): a twenty-thousand-row query that names one question
# file opens it exactly once. The count comes from strace's file
# syscalls; without strace the step skips with a note, the way the wire
# suite skips without its stub. Runs offline on the null backend.
set -euo pipefail
cd "$(dirname "$0")"
ROOT=$(cd .. && pwd)
EXT="$ROOT/build/release/thinkthen.duckdb_extension"
CLI="$ROOT/duckdb-bin/duckdb"
CUT="$ROOT/tools/null-cut.json"

if ! command -v strace >/dev/null 2>&1; then
  echo "skip     no strace on this host; the open count is not measured"
  exit 0
fi

TRACE=$(mktemp /tmp/thinkthen-atfile-XXXX.trace)
trap 'rm -f "$TRACE"' EXIT

# The result must be used, or DuckDB's optimizer drops the unused scalar
# projection and the file is never read at all.
ENGINE_NULL=1 strace -f -e trace=openat -o "$TRACE" \
  "$CLI" -unsigned -noheader -list \
  -c "LOAD '$EXT'; SELECT sum(thinkthen_decide('@$CUT', 'I demand a refund today')::INT) FROM range(20000);" \
  >/dev/null 2>&1 || true

OPENS=$(grep -c "null-cut.json" "$TRACE" || true)
if [[ "$OPENS" == "1" ]]; then
  echo "ok       one question file open for twenty thousand rows"
else
  echo "FAILED   the question file was opened $OPENS times for twenty thousand rows"
  exit 1
fi

# The cached question follows the file: an edited file is read again.
EDITED=$(mktemp /tmp/thinkthen-edited-XXXX.json)
cp "$CUT" "$EDITED"
python3 - "$EDITED" <<'PY'
import json
import sys
path = sys.argv[1]
with open(path) as held:
    question = json.load(held)
question["decide"] = "Is this a complaint about a charge?"
with open(path, "w") as out:
    json.dump(question, out)
PY
ENGINE_NULL=1 strace -f -e trace=openat -o "$TRACE" \
  "$CLI" -unsigned -noheader -list \
  -c "LOAD '$EXT'; SELECT thinkthen_decide('@$EDITED', 'I demand a refund today');" \
  >/dev/null 2>&1 || true
EDITED_OPENS=$(grep -c "thinkthen-edited" "$TRACE" || true)
rm -f "$EDITED"
if [[ "$EDITED_OPENS" -ge 1 ]]; then
  echo "ok       an edited question file is read again"
else
  echo "FAILED   the edited question file was never read"
  exit 1
fi
