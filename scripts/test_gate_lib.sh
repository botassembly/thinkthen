#!/usr/bin/env bash
# The gate's counting and wire verdict, tested on planted step logs, and
# every surface check held to the counted wire-skip spelling
# (surfaces-review-5: a gate that lost every wire suite printed the same
# green summary, "(wire: stub up)", because no surface's skip line counted).
# Usage: bash scripts/test_gate_lib.sh
set -euo pipefail
cd "$(dirname "$0")/.."
. scripts/gate_lib.sh

step_log=$(mktemp "${TMPDIR:-/tmp}/gate-lib-test.XXXXXX")
trap 'rm -f "$step_log"' EXIT
bad=0
expect() {
  if [ "$2" != "$3" ]; then
    echo "FAIL $1: expected $3, got $2"
    bad=1
  fi
}
reset() { green=0; skipped=0; diverged=0; failed=0; wire_ran=0; wire_lost=0; }

# A surface that skipped its wire suite while its stub was expected fails.
reset
wire_expected=" 8213 "
printf 'ok       one\nskip     wire-rust: no stub on 127.0.0.1:8213\n' >"$step_log"
count_lines "$step_log"
status=0
wire_verdict rust 8213 >/dev/null || status=$?
expect "lost wire suite exit" "$status" 1
expect "lost wire suite failed" "$failed" 1
expect "lost wire suite counted as a skip" "$skipped" 1
expect "lost wire suite lost" "$wire_lost" 1

# The same skip with no stub expected is an honest, counted skip.
reset
wire_expected=" "
count_lines "$step_log"
status=0
wire_verdict rust 8213 >/dev/null || status=$?
expect "unexpected stub exit" "$status" 0
expect "unexpected stub failed" "$failed" 0
expect "unexpected stub skipped" "$skipped" 1

# A surface that ran its wire suite counts as run.
reset
wire_expected=" 8213 "
printf 'ok       wire one\n' >"$step_log"
wire_verdict rust 8213
expect "wire ran" "$wire_ran" 1

# A diverge line is never green and never failed.
reset
printf 'diverge  18-x: recorded\n' >"$step_log"
count_lines "$step_log"
expect "diverge green" "$green" 0
expect "diverge counted" "$diverged" 1

# Every surface check prints the counted spelling for its own wire skip.
for pair in libraries/python:python libraries/typescript:typescript libraries/rust:rust \
  libraries/ruby:ruby libraries/r:r libraries/c:c databases/duckdb:duckdb \
  databases/sqlite:sqlite databases/postgresql:postgresql; do
  dir=${pair%%:*}
  name=${pair#*:}
  if ! grep -qE "skip     wire-$name: no stub on 127\.0\.0\.1:" "$dir/check.sh"; then
    echo "FAIL $dir/check.sh prints no counted wire-$name skip line"
    bad=1
  fi
done

[ "$bad" -eq 0 ] && echo "ok:      the gate counts wire skips and fails a lost wire suite"
exit "$bad"
