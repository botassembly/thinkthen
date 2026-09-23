#!/usr/bin/env bash
# Build and check every surface. Phase A checks the contract, the stand-in,
# and the conformance file; each surface hooks in as it lands in Phase B,
# by a function of its own that this script calls.
#
# The wire tests need the loopback stub that lives in this repository
# (tools/wire-stub). This script builds it when the binary is missing and
# starts one stub per surface port that has no stub already, so the wire
# suites run rather than skip; it stops only the stubs it started. A stub
# already listening on a port is used as it is. Every surface's conformance
# replay needs no network at all.
#
# Every line a runner prints with `ok`, `skip`, `diverge`, or `FAIL` at its
# start is counted, and the last line states the four totals beside the
# wire state: `all landed checks green` appears only when no check failed
# and the skipped count is named with it. `skip` and `diverge` are defined
# once, in conformance/skiptable.py; neither is ever counted as green.
#
# Wire honesty (surfaces-review-5): each surface's check prints
# `skip     wire-<surface>: ...` when it finds no stub on its port, and that
# line is counted. A surface whose stub this script started (or found up)
# and which still skipped its wire suite fails the gate, and the summary
# names how many wire suites ran instead of a single "stub up".
set -euo pipefail
cd "$(dirname "$0")/.."
# The gate never fetches (surfaces-review-5). These switches hold every
# tool that honors one to its local cache, beside the per-call spellings
# scripts/check_offline_calls.py checks.
export CARGO_NET_OFFLINE=true RUSTUP_AUTO_INSTALL=0 UV_OFFLINE=1 npm_config_offline=true
repo=$(pwd)

# A stray key or address in the caller's shell must not turn a check into a
# paid call; the rung (sdlc/scripts/surfaces) carries the same guard.
unset THINKTHEN_API_KEY THINKTHEN_BASE_URL

fail=0
green=0
skipped=0
diverged=0
failed=0
step_log=$(mktemp)
stub_pids=()
# The ports whose stub is expected to answer for the whole run (a plain
# word list: macOS's bash 3.2 has no associative arrays).
wire_expected=" "
wire_ran=0
wire_lost=0
wire_failed=0
stop_stubs() {
  for pid in ${stub_pids[@]+${stub_pids[@]}}; do
    kill "$pid" 2>/dev/null || true
  done
}
trap 'rm -f "$step_log"; stop_stubs' EXIT

# The counting and the wire verdict live in one sourced file so their own
# test (scripts/test_gate_lib.sh) exercises the same code the gate runs.
# shellcheck source=scripts/gate_lib.sh
. "$repo/scripts/gate_lib.sh"

# Run one step, show its output as it happens, count its result lines, and
# keep its exit status.
run_step() {
  local label=$1
  shift
  printf '\n== %s\n' "$label"
  local status=0
  "$@" >"$step_log" 2>&1 || status=$?
  cat "$step_log"
  count_lines "$step_log"
  step_verdict "$label" "$status"
  return "$status"
}

# One surface's check.sh, counted the same way.
run_surface() {
  local dir=$1
  printf '\n== %s\n' "$dir"
  local status=0
  # A surface whose stub this run expects must not skip its wire tests:
  # THINKTHEN_WIRE_REQUIRED=1 turns a lost stub into a failure inside the
  # suites (surfaces-review-5: a stub that died mid-run passed green).
  local required=
  if wire_expected_on "$2"; then required=1; fi
  (cd "$dir" && THINKTHEN_WIRE_REQUIRED=$required ./check.sh) >"$step_log" 2>&1 || status=$?
  cat "$step_log"
  count_lines "$step_log"
  return "$status"
}

# The stub's address, when a stub is up.
stub_url="http://127.0.0.1:${STUB_PORT:-8231}/v1"

# Build the in-repo wire stub when its binary is missing (offline: the
# dependencies come from the local cargo cache).
stub_bin=$repo/tools/wire-stub/target/release/stub-backend
if [ ! -x "$stub_bin" ]; then
  printf '\n== wire stub: build tools/wire-stub\n'
  (cd tools/wire-stub && cargo build --release --offline --locked) || fail=1
fi

# Start one stub per surface port that has no stub already. Every port
# carries the same 300 ms delay the manual recipe used: the cancel and
# interrupt proofs need requests still in flight when they trip, and a
# zero-delay stub lets whole batches finish before anything can stop them.
# Only stubs this script starts are stopped.
# start_stub lives in scripts/gate_lib.sh, where its own test runs it.
printf '\n== wire stubs\n'
start_stub 8211 300   # python: the cancel proof needs a delay
start_stub 8212 300   # typescript: the abort and deadline proofs need a delay
start_stub 8213 300   # rust
start_stub 8214 300   # ruby: the interrupt proof needs a delay
start_stub 8215 300   # r
start_stub 8216 300   # c
start_stub 8217 300   # duckdb
start_stub 8218 300   # sqlite
start_stub 8219 300   # postgresql
start_stub 8231 300   # the stand-in wire test below

if curl -sf --max-time 1 "$stub_url/stats" >/dev/null 2>&1; then
  wire=yes
else
  wire=no
  skip_note='skip     wire-standin: no stub on 127.0.0.1:8231'
fi

run_step "surfaces ratchet" node sdlc/scripts/surfaces-ratchet.mjs || {
  # Rule 3 (surfaces-review-4): the gate runs at the exact tip, and a red
  # ratchet prints no summary at all — neither the green line nor the
  # counted one. The ratchet is the first check and stops the run.
  echo 'gate stops: the surfaces ratchet is red at this tip' >&2
  exit 1
}

# The ceiling rule and the lint rules run before any surface: a tip the
# ratchet or Clippy refuses cannot buy a green gate with green suites
# (third review: the lint rung was red at the tip the gate called green).
run_step "workspace lint rules" sdlc/scripts/lint-workspaces || fail=1

run_step "contract" bash -c 'cd contract && cargo test --quiet --locked' || fail=1

# surfaces-review-5: the loopback-stub tests (a signal never resends a
# paid request, billing, backoff) ran in no gate while only --lib ran.
# Every test file but wire, which the next step runs against the stub.
run_step "stand-in, null backend and loopback stubs" bash -c \
  'cd standin && set -- --lib && for t in tests/*.rs; do n=$(basename "$t" .rs); [ "$n" = wire ] || set -- "$@" --test "$n"; done && cargo test --quiet --locked "$@"' || fail=1

if [ "$wire" = yes ]; then
  if run_step "stand-in, wire against the stub" bash -c \
    'cd standin && THINKTHEN_WIRE_REQUIRED=1 ENGINE_BASE_URL="$0" ENGINE_WIDTH=32 cargo test --quiet --test wire --locked -- --nocapture' \
    "$stub_url"; then
    wire_ran=$((wire_ran + 1))
  else
    fail=1
    wire_failed=$((wire_failed + 1))
  fi
else
  printf '\n== stand-in, wire against the stub\n%s\n' "$skip_note"
  skipped=$((skipped + 1))
  if wire_expected_on 8231; then
    echo "FAIL     wire-standin: the stub on 8231 was expected and the wire suite did not run"
    failed=$((failed + 1))
    wire_lost=$((wire_lost + 1))
    fail=1
  fi
fi

run_step "conformance file" python3 conformance/tools/validate_conformance.py || fail=1

run_step "the one skip table" python3 conformance/skiptable.py validate || fail=1

run_step "the runners leave holding back to the table" python3 conformance/tools/test_runner_honesty.py || fail=1

run_step "conformance checker tests" python3 conformance/tools/test_validate_conformance.py || fail=1

run_step "generated function lists" python3 scripts/generate_functions.py --check || fail=1

run_step "public names" python3 scripts/check_public_names.py || fail=1

run_step "public-name check tests" python3 scripts/test_check_public_names.py || fail=1

run_step "private references" python3 scripts/check_no_private_refs.py || fail=1

run_step "private-reference check tests" python3 scripts/test_check_no_private_refs.py || fail=1

run_step "gate counting and wire-verdict tests" bash scripts/test_gate_lib.sh || fail=1

run_step "every build-tool call carries the lock" python3 scripts/check_locked_calls.py || fail=1

run_step "lock-checker tests" python3 scripts/test_check_locked_calls.py || fail=1

run_step "every package-manager call stays offline" python3 scripts/check_offline_calls.py || fail=1

run_step "offline-checker tests" python3 scripts/test_check_offline_calls.py || fail=1

# Each surface lands in Phase B with a check of its own. A surface is
# checked by running its slide sample against the stand-in and its slice of
# the conformance file; the hook is the surface's folder, named here in the
# slide order.
printf '\n== surfaces\n'
surface_port() {
  case $1 in
  python) echo 8211 ;; typescript) echo 8212 ;; rust) echo 8213 ;;
  ruby) echo 8214 ;; r) echo 8215 ;; c) echo 8216 ;;
  duckdb) echo 8217 ;; sqlite) echo 8218 ;; postgresql) echo 8219 ;;
  esac
}
for surface in python typescript ruby r rust c; do
  if [ -x "libraries/$surface/check.sh" ]; then
    port=$(surface_port "$surface")
    status=0
    run_surface "libraries/$surface" "$port" || status=$?
    surface_verdict "$surface" "$status" || fail=1
    wire_verdict "$surface" "$port" "$status" || fail=1
  else
    echo "not landed: libraries/$surface"
  fi
done
for engine in duckdb sqlite postgresql; do
  if [ -x "databases/$engine/check.sh" ]; then
    port=$(surface_port "$engine")
    status=0
    run_surface "databases/$engine" "$port" || status=$?
    surface_verdict "$engine" "$status" || fail=1
    wire_verdict "$engine" "$port" "$status" || fail=1
  else
    echo "not landed: databases/$engine"
  fi
done

# The package rehearsals build and stage their artifacts without installing
# anywhere (--dry-run); their container halves run by hand per
# sdlc/records/surfaces-notes/NOTES-packaging.md. DuckDB's package.sh gains the flag with its lane.
if command -v cargo-zigbuild >/dev/null 2>&1 && command -v zig >/dev/null 2>&1; then
  run_step "package dry-run: databases/sqlite" \
    bash -c 'cd databases/sqlite && ./package.sh --dry-run' || fail=1
else
  printf '\n== package dry-run: databases/sqlite\n%s\n' \
    'skip     package-dry-run-sqlite: cargo-zigbuild or zig is not on PATH'
  skipped=$((skipped + 1))
fi
if command -v cargo-pgrx >/dev/null 2>&1 && command -v pg_config >/dev/null 2>&1; then
  run_step "package dry-run: databases/postgresql" \
    bash -c 'cd databases/postgresql && ./package.sh --dry-run' || fail=1
else
  printf '\n== package dry-run: databases/postgresql\n%s\n' \
    'skip     package-dry-run-postgresql: cargo-pgrx or pg_config is not on PATH'
  skipped=$((skipped + 1))
fi

run_step "host builds remap the builder's home" bash scripts/check_artifact_paths.sh || fail=1

wire_word="$wire_ran of 10 wire suites ran in a passing surface, $wire_failed in a failed surface, $wire_lost lost"
if [ "$fail" -ne 0 ] || [ "$failed" -ne 0 ]; then
  echo "summary: green=$green skipped=$skipped diverged=$diverged failed=$failed (wire: $wire_word)"
  echo 'a surface check failed' >&2
  exit 1
fi
echo "all landed checks green: green=$green skipped=$skipped diverged=$diverged failed=$failed (wire: $wire_word)"
