#!/usr/bin/env bash
# The security and boundary suite: `@file` reads respect the database's
# own `enable_external_access` switch (review group 3), and relate says
# which boundary it hit when the caller's temporary table is out of the
# stable C API's reach (ruled fix, option A). Runs offline on the null
# backend.
set -euo pipefail
cd "$(dirname "$0")"
ROOT=$(cd .. && pwd)
EXT="$ROOT/build/release/thinkthen.duckdb_extension"
CLI="$ROOT/duckdb-bin/duckdb"

run() {
  ENGINE_NULL=1 "$CLI" -unsigned -noheader -list -c "LOAD '$EXT'; $1" 2>&1
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
  if [[ "$got" == *"true"* || "$got" == *"false"* ]]; then
    echo "FAILED   $name: the call answered despite the refusal, got '$got'"
    exit 1
  fi
  echo "ok       $name"
}

CUT="$ROOT/tools/null-cut.json"

# With file access off, every @file door refuses and never reads.
refuse "decide @file refuses with access off" \
  "thinkthen local: the question file $CUT was not read: enable_external_access is off" \
  "$(run "SET enable_external_access=false; SELECT thinkthen_decide('@$CUT', 'I demand a refund today');")"

refuse "annotate @set refuses with access off" \
  "enable_external_access is off" \
  "$(run "SET enable_external_access=false; SELECT thinkthen_annotate('@$CUT', 'maybe later');")"

refuse "relate @rules file refuses with access off" \
  "enable_external_access is off for the calling database" \
  "$(run "SET enable_external_access=false; CREATE TABLE s(id INTEGER, body VARCHAR); INSERT INTO s VALUES (1, 'the payment failed'); SELECT * FROM thinkthen_relate('SELECT id, body FROM s', ['@$CUT']);")"

# The switch is what refuses, not the path: with access on the same call
# reads the file and answers.
expect "decide @file answers with access on" "true" \
  "$(run "SELECT thinkthen_decide('@$CUT', 'I demand a refund today');")"

# Plain text never touches a file and stays unaffected by the switch.
expect "plain text answers with access off" "true" \
  "$(run "SET enable_external_access=false; SELECT thinkthen_decide('Is this a complaint?', 'I demand a refund today');")"

# The boundary message: a temporary table cannot be reached by the
# stable C API, and the error says so instead of "table does not exist".
expect "relate names the temporary-table boundary" \
  "thinkthen local: the relate query names the temporary table tt" \
  "$(run "CREATE TEMP TABLE tt(id INTEGER, body VARCHAR); INSERT INTO tt VALUES (1, 'the payment failed'); SELECT * FROM thinkthen_relate('SELECT id, body FROM tt', ['caused_by']);")"

# A genuinely missing table keeps DuckDB's own error, untranslated.
expect "a missing non-temporary table keeps the raw error" \
  "Catalog Error: Table with name nope does not exist" \
  "$(run "SELECT * FROM thinkthen_relate('SELECT id, body FROM nope', ['caused_by']);")"
