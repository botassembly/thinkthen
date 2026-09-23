#!/usr/bin/env bash
# The panic guards: a panic inside a C callback must become the query's
# error, never an abort that takes the host process with it. The test-only
# ENGINE_TEST_PANIC names one boundary to panic in; the guard has to catch
# it. Without the guards, each of these armed calls would abort the CLI
# (a Rust panic must not unwind across `extern "C"`, so the process dies
# with the "cannot unwind" abort and exit code 134) instead of printing
# the contained defect and exiting with the ordinary query error.
set -euo pipefail
cd "$(dirname "$0")"
ROOT=$(cd .. && pwd)
EXT="$ROOT/build/panic/thinkthen.duckdb_extension"
CLI="$ROOT/duckdb-bin/duckdb"

# The panic arm is compile-time (review 4): build the armed extension
# once here, beside the default one the rest of the check loads.
mkdir -p "$ROOT/build/panic"
DUCKDB_EXTENSION_NAME=thinkthen DUCKDB_EXTENSION_MIN_DUCKDB_VERSION="$(cat "$ROOT/tools/version.env" | grep -oP 'DUCKDB_VERSION=\K.*')" \
  cargo build --locked --release --quiet --features test-panic \
  --target-dir "$ROOT/target-panic" >/dev/null 2>&1 || { echo "FAILED   the panic-armed build"; exit 1; }
"$ROOT/configure/venv/bin/python" "$ROOT/extension-ci-tools/scripts/append_extension_metadata.py" \
  -o "$EXT" -l "$ROOT/target-panic/release/libthinkthen.so" \
  -n thinkthen -dv "$(grep -oP 'DUCKDB_VERSION=\K.*' "$ROOT/tools/version.env")" \
  -evf "$ROOT/configure/extension_version.txt" -pf "$ROOT/configure/platform.txt" --abi-type C_STRUCT_UNSTABLE >/dev/null

armed() {
  local boundary=$1 sql=$2
  ENGINE_NULL=1 ENGINE_TEST_PANIC="$boundary" "$CLI" -unsigned -noheader -list -c "
LOAD '$EXT';
$sql
" 2>&1
}

expect_contained() {
  local name=$1 boundary=$2 sql=$3
  local out rc
  set +e
  out=$(armed "$boundary" "$sql")
  rc=$?
  set -e
  if [ "$rc" -eq 134 ] || [ "$rc" -eq 139 ]; then
    echo "FAILED   $name: the panic aborted the process (exit $rc)"
    echo "$out" | head -3
    exit 1
  fi
  # The load boundary reports through the contract's catch_panic; every
  # other boundary keeps the callback shape.
  if ! grep -qE "thinkthen defect: (the $boundary callback panicked|a panic crossed the extension load)" <<<"$out"; then
    echo "FAILED   $name: the contained defect is missing (exit $rc)"
    echo "$out" | head -5
    exit 1
  fi
  if ! grep -q "the test arm fired" <<<"$out"; then
    echo "FAILED   $name: the panic message is missing (exit $rc)"
    exit 1
  fi
  echo "ok       $name: the panic became the callback's error (exit $rc)"
}

expect_contained "the extension load" "extension load" "SELECT 1;"
expect_contained "relate bind" "relate bind" "SELECT * FROM thinkthen_relate('SELECT 1, ''x''', ['caused_by']);"
expect_contained "usage scan" "usage scan" "SELECT * FROM thinkthen_usage();"

# Unset, the arm is dormant: the same calls answer as always.
out=$(ENGINE_NULL=1 "$CLI" -unsigned -noheader -list -c "LOAD '$EXT'; SELECT count(*) FROM thinkthen_usage();" 2>&1)
if [[ "$out" == *"panicked"* ]]; then
  echo "FAILED   the unset arm still fired: $out"
  exit 1
fi
echo "ok       the arm is dormant when unset"
