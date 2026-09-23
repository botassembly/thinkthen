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

# The row cap bounds what a relate query holds, not only what it returns
# (review 5, finding 2): a window or a grouping over eight million rows
# read all of them before the LIMIT, 455 MB and 627 MB, and only then
# heard the 256-row refusal. The plan is read first now, and a step that
# holds its whole input refuses before anything runs. The sentences are
# pinned whole.
refuse "a window over eight million rows refuses at the plan" \
  "thinkthen usage: the relate query feeds about 8000000 rows into the WINDOW step before its LIMIT, and relate lets at most 1000000 rows into a sorting, grouping, windowing, or joining step; filter the rows first" \
  "$(run "SELECT count(*) FROM thinkthen_relate('SELECT i, ''b'' FROM (SELECT i, row_number() OVER (ORDER BY random()) AS rn FROM range(8000000) t(i))', ['caused_by']);")"
refuse "a grouping over eight million rows refuses at the plan" \
  "thinkthen usage: the relate query feeds about 8000000 rows into the HASH_GROUP_BY step before its LIMIT, and relate lets at most 1000000 rows into a sorting, grouping, windowing, or joining step; filter the rows first" \
  "$(run "SELECT count(*) FROM thinkthen_relate('SELECT i, ''b'' FROM (SELECT i % 8000000 AS i, count(*) FROM range(8000000) t(i) GROUP BY 1)', ['caused_by']);")"
refuse "a materialized WITH over eight million rows refuses at the plan" \
  "thinkthen usage: the relate query feeds about 8000000 rows into the CTE step before its LIMIT, and relate lets at most 1000000 rows into a sorting, grouping, windowing, or joining step; filter the rows first" \
  "$(run "SELECT count(*) FROM thinkthen_relate('WITH x AS MATERIALIZED (SELECT i FROM range(8000000) t(i)) SELECT i, ''b'' FROM x', ['caused_by']);")"
# An honest grouped record source over an ordinary table passes the plan
# and reaches the backend: the stand-in's own "no recorded answer" (so
# the CLI exits non-zero; the `|| true` keeps `set -e` from ending the
# suite on that expected error).
small_grouped=$(run "CREATE TABLE t AS SELECT i AS id, 'ticket ' || i AS body, i % 7 AS k FROM range(2000) r(i); SELECT count(*) FROM thinkthen_relate('SELECT k, string_agg(body, '' '') FROM t GROUP BY k', ['caused_by']);" || true)
if [[ "$small_grouped" != *"no recorded answer for the records"* ]]; then
  echo "FAILED   a small grouped relate query did not reach the backend: '$small_grouped'"
  exit 1
fi
echo "ok       a small grouped relate query passes the plan check and reaches the backend"
