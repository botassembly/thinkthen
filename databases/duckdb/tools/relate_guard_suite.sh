#!/usr/bin/env bash
# The relate guard suite: relate reads records and nothing else (review
# group 2 and item 1), and a relate inside a relate refuses instead of
# hanging (review item 6). Runs offline on the null backend.
set -euo pipefail
cd "$(dirname "$0")"
ROOT=$(cd .. && pwd)
EXT="$ROOT/build/release/thinkthen.duckdb_extension"
CLI="$ROOT/duckdb-bin/duckdb"
CUT="$ROOT/tools/null-cut.json"

run() {
  ENGINE_NULL=1 "$CLI" -unsigned -noheader -list -c "LOAD '$EXT'; $1" 2>&1
}

# The piped form, so the CLI keeps going past a raised error and the
# statements after it still run.
pipe_run() {
  ENGINE_NULL=1 "$CLI" -unsigned -noheader -list <<SQL 2>&1
LOAD '$EXT';
$1
SQL
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

refuse() {
  local name=$1 want=$2 got=$3
  if [[ "$got" != *"$want"* ]]; then
    echo "FAILED   $name: want the refusal '$want', got '$got'"
    exit 1
  fi
  echo "ok       $name"
}

# A DELETE inside the relate query cannot run: the query runs in a
# read-only transaction, so nothing it holds can write. The caller's own
# transaction rolls back with the table unchanged, which is the review's
# exact scenario: before the fix the DELETE ran on the kept connection,
# committed there, and survived the caller's ROLLBACK.
guarded=$(pipe_run "CREATE TABLE t(id INTEGER, body VARCHAR); INSERT INTO t VALUES (1, 'the payment failed');
BEGIN;
SELECT * FROM thinkthen_relate('DELETE FROM t RETURNING id, body', ['caused_by']);
ROLLBACK;
SELECT count(*) FROM t;" || true)
refuse "relate refuses a DELETE inside its query" \
  "must be a SELECT; this one is a DELETE" "$guarded"
expect "the DELETE never happened, the caller's ROLLBACK included" "1" \
  "$(printf '%s\n' "$guarded" | tail -1)"

# More than one statement is refused before anything runs.
refuse "relate refuses a second statement" \
  "the relate query is one SQL statement and this one holds 2" \
  "$(run "CREATE TABLE t(id INTEGER, body VARCHAR); INSERT INTO t VALUES (1, 'the payment failed'); SELECT * FROM thinkthen_relate('SELECT id, body FROM t; DELETE FROM t', ['caused_by']);")"

# A relate inside a relate used to wait forever on the connection the
# outer query holds; it now refuses with its own words. The timeout turns
# a regression back into a failure instead of a hung check.
nested=$(timeout 30 bash -c "ENGINE_NULL=1 '$CLI' -unsigned -noheader -list <<'SQL' 2>&1
LOAD '$EXT';
CREATE TABLE t(id INTEGER, body VARCHAR); INSERT INTO t VALUES (1, 'the payment failed');
SELECT * FROM thinkthen_relate('SELECT 1 AS id, (SELECT count(*) FROM thinkthen_relate(''SELECT id, body FROM t'', [''caused_by'']))::VARCHAR AS body FROM t', ['caused_by']);
SQL
" || true)
if [[ "$nested" == *"nested relate cannot run"* ]]; then
  echo "ok       a relate inside a relate refuses instead of hanging"
elif [[ -z "$nested" ]]; then
  echo "FAILED   the nested relate hung (the timeout killed it with no error)"
  exit 1
else
  echo "FAILED   the nested relate: want the nested refusal, got '$nested'"
  exit 1
fi

# A prepared relate re-checks the file access at execution: prepared
# while reads were allowed, executed after the switch went off, and it
# must refuse rather than read.
refuse "a prepared relate re-checks file access at execution" \
  "enable_external_access is off for this database" \
  "$(run "CREATE TABLE t(id INTEGER, body VARCHAR); INSERT INTO t VALUES (1, 'the payment failed'); PREPARE p AS SELECT * FROM thinkthen_relate('SELECT id, body FROM t', ['@$CUT']); SET enable_external_access=false; EXECUTE p;")"
