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
reset() { green=0; skipped=0; diverged=0; failed=0; wire_ran=0; wire_lost=0; wire_failed=0; }

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

# A TAP skip or todo is a skip, never green.
reset
printf 'ok 1 - ran\nok 2 - held # SKIP no stub\nok 3 - later # TODO\n' >"$step_log"
count_lines "$step_log"
expect "tap green" "$green" 1
expect "tap skipped" "$skipped" 2

# A surface that failed before its wire section never ran it.
reset
wire_expected=" 8213 "
printf 'FAIL     early\n' >"$step_log"
wire_verdict rust 8213 1
expect "failed surface wire ran" "$wire_ran" 0

# A surface whose check.sh exited nonzero without a line of its own gets a
# counted FAIL naming its exit and last line.
reset
printf '== duckdb surface: build the extension\nmake[1]: *** [check_configure] Error 1\n\n' >"$step_log"
count_lines "$step_log"
said=$(surface_verdict duckdb 2; echo "failed=$failed")
expect "a quiet surface exit" "$said" "FAIL     surface-duckdb: check.sh exited 2; last line: make[1]: *** [check_configure] Error 1
failed=1"

# A surface that named its missing setup step is counted once, by its
# own line, and its expected wire suite is lost, not "in a failed surface".
reset
wire_expected=" 8218 "
printf 'FAIL     surface-sqlite: not set up (fetch an amalgamation)\n' >"$step_log"
count_lines "$step_log"
said=$(surface_verdict sqlite 1; echo "status=$? failed=$failed")
expect "a surface not set up" "$said" "status=1 failed=1"
status=0
said=$(wire_verdict sqlite 8218 1; echo "failed=$failed lost=$wire_lost")
expect "a never-started wire suite" "$said" "FAIL     wire-sqlite: the stub on 8218 was expected and the surface never started
failed=2 lost=1"

# A surface that failed after starting counts its wire suite as neither
# run nor lost, and the summary names it.
reset
wire_expected=" 8213 "
printf 'FAIL     one test\n' >"$step_log"
wire_failed=0
wire_verdict rust 8213 1
expect "a failed surface's wire suite" "$wire_ran:$wire_lost:$wire_failed" "0:0:1"

# A gate step that exited nonzero without a failure line gets one.
reset
printf 'Traceback\nAssertionError: boom\n' >"$step_log"
said=$(step_verdict "public names" 1; echo "failed=$failed")
expect "a quiet step exit" "$said" "FAIL     step public names: exited 1; last line: AssertionError: boom
failed=1"

# Each surface that needs a one-time setup step names it in the counted
# form when the step is missing.
for pair in libraries/python:python libraries/typescript:typescript libraries/ruby:ruby \
  databases/duckdb:duckdb databases/sqlite:sqlite; do
  if ! grep -qE "FAIL     surface-${pair#*:}: not set up \(" "${pair%%:*}/check.sh"; then
    echo "FAIL ${pair%%:*}/check.sh names no missing setup step in the counted form"
    bad=1
  fi
done

# The gate calls the verdict for every surface and hands each suite the
# requirement, and the stand-in's own wire run carries it too.
for needle in 'surface_verdict "$surface" "$status"' 'surface_verdict "$engine" "$status"' 'step_verdict "$label" "$status"' 'wire_verdict "$surface" "$port" "$status"' 'wire_verdict "$engine" "$port" "$status"' \
  'THINKTHEN_WIRE_REQUIRED=$required ./check.sh' 'THINKTHEN_WIRE_REQUIRED=1 ENGINE_BASE_URL'; do
  if ! grep -qF -- "$needle" scripts/check_surfaces.sh; then
    echo "FAIL scripts/check_surfaces.sh no longer carries: $needle"
    bad=1
  fi
done

# A running stub with another delay is refused, and one with the asked
# delay is reused. Ports 8640-8649 belong to this test.
stub_bin=$PWD/tools/wire-stub/target/release/stub-backend
stub_pids=()
if [ -x "$stub_bin" ]; then
  STUB_PORT=8640 STUB_DELAY_MS=0 "$stub_bin" >/dev/null 2>&1 &
  held_pid=$!
  trap 'rm -f "$step_log"; kill "$held_pid" 2>/dev/null || true' EXIT
  for _ in $(seq 1 50); do
    curl -sf --max-time 1 http://127.0.0.1:8640/v1/stats >/dev/null 2>&1 && break
    sleep 0.1
  done
  reset; fail=0; wire_expected=" "
  said=$(start_stub 8640 300; echo "failed=$failed fail=$fail expected=[$wire_expected]")
  expect "a stub with another delay" "$said" "FAIL     stub on 8640: it answers with delay '0' ms and this run needs 300 ms; stop it or free the port
failed=1 fail=1 expected=[ ]"
  said=$(start_stub 8640 0; echo "expected=[$wire_expected]")
  expect "a stub with the asked delay" "$said" "stub already up on 8640 with delay 0ms; using it
expected=[ 8640 ]"
  kill "$held_pid" 2>/dev/null || true
  wait "$held_pid" 2>/dev/null || true
else
  echo "skip     stub-delay-refusal: no stub binary at tools/wire-stub/target/release"
fi

# A built artifact that carries the builder's home fails the gate's
# artifact check, even when every build script carries the remap.
planted=$(mktemp -d "${TMPDIR:-/tmp}/gate-artifact.XXXXXX")
mkdir -p "$planted/release"
printf 'x\0%s/src/lib.rs\0' "$HOME" >"$planted/release/libplanted.so"
status=0
said=$(CARGO_TARGET_DIR=$planted bash scripts/check_artifact_paths.sh) || status=$?
expect "a planted home path" "$status:$said" "1:FAIL     $planted/release/libplanted.so: 1 strings carry the builder's home"

# A surface whose check passed must leave an artifact for the scan
# (surfaces-review-7 R7-5: the gate form passed with 0 artifacts).
printf 'x\0/build/src/lib.rs\0' >"$planted/release/libplanted.so"
mkdir -p "$planted/empty"
status=0
said=$(CARGO_TARGET_DIR=$planted bash scripts/check_artifact_paths.sh --built "$planted/empty") || status=$?
expect "a built surface with no artifact" "$status:$said" "1:FAIL     $planted/empty: its check passed and left no built artifact to scan"
status=0
said=$(CARGO_TARGET_DIR=$planted bash scripts/check_artifact_paths.sh --built "$planted") || status=$?
expect "a built surface with a clean artifact" "$status" "0"
rm -rf "$planted"

# The portable-shell check names each GNU-only spelling and passes the
# portable ones (surfaces-review-7 R3-32).
planted=$(mktemp "${TMPDIR:-/tmp}/gate-shell.XXXXXX")
gnu_sed='sed -i "s/a/b/" f'  # portable-shell: data
gnu_date='t=$(date +%s.%N)'  # portable-shell: data
gnu_timeout='if timeout 5 true; then :; fi'  # portable-shell: data
printf '%s\n' "$gnu_sed" "$gnu_date" "$gnu_timeout" \
  '# timeout 5 in a comment' 'sed -i.bak "s/a/b/" f' '"$TIMEOUT" 5 true' 'psql --timeout 5' >"$planted"  # portable-shell: data
status=0
said=$(bash scripts/check_portable_shell.sh "$planted") || status=$?
rm -f "$planted"
expect "GNU-only shell spellings" "$status:$said" "1:FAIL     $planted:1:$gnu_sed
FAIL     $planted:2:$gnu_date
FAIL     $planted:3:$gnu_timeout"

# The data mark exempts only its own line. A file that marks one line
# still fails on a real use on another line.
planted=$(mktemp "${TMPDIR:-/tmp}/gate-shell.XXXXXX")
printf '%s\n' "echo '$gnu_sed'  # portable-shell: data" "$gnu_sed" >"$planted"
status=0
said=$(bash scripts/check_portable_shell.sh "$planted") || status=$?
rm -f "$planted"
expect "a real use beside a data line" "$status:$said" "1:FAIL     $planted:2:$gnu_sed"

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
