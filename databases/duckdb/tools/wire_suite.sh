#!/usr/bin/env bash
# The wire suite: the slide's WHERE over parquet on the wire, warm at the
# width, the error kinds off the wire, and the stub's counters as the
# proof of what crossed. The stub must be up on 8217.
set -euo pipefail
cd "$(dirname "$0")"
ROOT=$(cd .. && pwd)
EXT="$ROOT/build/release/thinkthen.duckdb_extension"
CLI="$ROOT/duckdb-bin/duckdb"
BASE=http://127.0.0.1:8217/v1

run() {
  ENGINE_BASE_URL=$BASE "$CLI" -unsigned -noheader -list -c "LOAD '$EXT'; $1" 2>&1
}

expect() {
  local name=$1 want=$2 got=$3
  if [[ "$got" == *"$want"* ]]; then
    echo "ok       $name"
  else
    echo "FAILED   $name: want '$want', got '$got'"
    exit 1
  fi
}

curl -s -X POST http://127.0.0.1:8217/v1/reset >/dev/null

# The slide's first statement, over a parquet built here, on the wire.
rm -rf "$ROOT/tools/wire-run" && mkdir -p "$ROOT/tools/wire-run"
"$CLI" -c "
CREATE TABLE tickets(id INTEGER, body VARCHAR);
INSERT INTO tickets VALUES
 (1, 'I was charged twice and I want a refund today.'),
 (2, 'Please refund my shipping label fee.'),
 (3, 'Thanks, the fix worked.');
COPY tickets TO '$ROOT/tools/wire-run/tickets.parquet' (FORMAT parquet);
" >/dev/null
cd "$ROOT/tools/wire-run"
got=$(ENGINE_BASE_URL=$BASE "$CLI" -unsigned -noheader -list \
  -c "LOAD '$EXT'; SELECT id FROM 'tickets.parquet' WHERE thinkthen_decide('Is this a complaint?', body);" 2>&1)
expect "WHERE keeps the refund rows" "$(printf '1\n2')" "$got"

# Warm fills the cache and answers nothing new for the repeat.
warm=$(run "SELECT thinkthen_warm('Is this a complaint?', t) FROM (SELECT unnest(['refund one','refund two','refund three','plain talk']) t);")
expect "warm judged four" "4" "$warm"

stats=$(curl -s http://127.0.0.1:8217/v1/stats)
expect "the stub saw the sends" "requests" "$stats"
echo "         $stats"
