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
# and the skipped count is named with it.
set -euo pipefail
cd "$(dirname "$0")/.."
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
stop_stubs() {
  for pid in ${stub_pids[@]+${stub_pids[@]}}; do
    kill "$pid" 2>/dev/null || true
  done
}
trap 'rm -f "$step_log"; stop_stubs' EXIT

# Count one step's own result lines into the running totals.
count_lines() {
  local file=$1
  green=$((green + $(grep -cE '^(ok[[:space:]]|ok:|OK:)' "$file" || true)))
  skipped=$((skipped + $(grep -cE '^skip[[:space:]]' "$file" || true)))
  diverged=$((diverged + $(grep -cE '^diverge[[:space:]]' "$file" || true)))
  # `not ok` is TAP, which node --test prints; a node failure must land in
  # the failed total and not only in the step's exit status.
  failed=$((failed + $(grep -cE '^(FAIL|not ok)' "$file" || true)))
}

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
  return "$status"
}

# One surface's check.sh, counted the same way.
run_surface() {
  local dir=$1
  printf '\n== %s\n' "$dir"
  local status=0
  (cd "$dir" && ./check.sh) >"$step_log" 2>&1 || status=$?
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
start_stub() {
  local port=$1 delay=$2
  if curl -sf --max-time 1 "http://127.0.0.1:$port/v1/stats" >/dev/null 2>&1; then
    echo "stub already up on $port; using it"
    return
  fi
  if [ ! -x "$stub_bin" ]; then
    echo "no stub binary; wire suites on $port will skip"
    return
  fi
  STUB_PORT=$port STUB_DELAY_MS=$delay "$stub_bin" >/dev/null 2>&1 &
  stub_pids+=($!)
  local _
  for _ in $(seq 1 30); do
    if curl -sf --max-time 1 "http://127.0.0.1:$port/v1/stats" >/dev/null 2>&1; then
      echo "stub up on $port (delay ${delay}ms)"
      return
    fi
    sleep 0.1
  done
  echo "stub on $port did not answer; its wire suites will skip"
}
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
  skip_note='wire tests skipped: no stub on the loopback'
fi

run_step "surfaces ratchet" node sdlc/scripts/surfaces-ratchet.mjs || fail=1

# The ceiling rule and the lint rules run before any surface: a tip the
# ratchet or Clippy refuses cannot buy a green gate with green suites
# (third review: the lint rung was red at the tip the gate called green).
run_step "workspace lint rules" sdlc/scripts/lint-workspaces || fail=1

run_step "contract" bash -c 'cd contract && cargo test --quiet --locked' || fail=1

run_step "stand-in, null backend" bash -c 'cd standin && cargo test --quiet --locked --lib' || fail=1

if [ "$wire" = yes ]; then
  run_step "stand-in, wire against the stub" bash -c \
    'cd standin && ENGINE_BASE_URL="$0" ENGINE_WIDTH=32 cargo test --quiet --test wire --locked -- --nocapture' \
    "$stub_url" || fail=1
else
  printf '\n== stand-in, wire against the stub\n%s\n' "$skip_note"
fi

run_step "conformance file" python3 conformance/tools/validate_conformance.py || fail=1

run_step "conformance checker tests" python3 conformance/tools/test_validate_conformance.py || fail=1

run_step "generated function lists" python3 scripts/generate_functions.py --check || fail=1

run_step "public names" python3 scripts/check_public_names.py || fail=1

run_step "public-name check tests" python3 scripts/test_check_public_names.py || fail=1

run_step "private references" python3 scripts/check_no_private_refs.py || fail=1

run_step "private-reference check tests" python3 scripts/test_check_no_private_refs.py || fail=1

# Each surface lands in Phase B with a check of its own. A surface is
# checked by running its slide sample against the stand-in and its slice of
# the conformance file; the hook is the surface's folder, named here in the
# slide order.
printf '\n== surfaces\n'
for surface in python typescript ruby r rust c; do
  if [ -x "libraries/$surface/check.sh" ]; then
    run_surface "libraries/$surface" || fail=1
  else
    echo "not landed: libraries/$surface"
  fi
done
for engine in duckdb sqlite postgresql; do
  if [ -x "databases/$engine/check.sh" ]; then
    run_surface "databases/$engine" || fail=1
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
    'skipped: cargo-zigbuild or zig is not on PATH'
fi
if command -v cargo-pgrx >/dev/null 2>&1 && command -v pg_config >/dev/null 2>&1; then
  run_step "package dry-run: databases/postgresql" \
    bash -c 'cd databases/postgresql && ./package.sh --dry-run' || fail=1
else
  printf '\n== package dry-run: databases/postgresql\n%s\n' \
    'skipped: cargo-pgrx or pg_config is not on PATH'
fi

wire_word=$([ "$wire" = yes ] && echo "stub up" || echo "no stub, wire suites skipped")
if [ "$fail" -ne 0 ] || [ "$failed" -ne 0 ]; then
  echo "summary: green=$green skipped=$skipped diverged=$diverged failed=$failed (wire: $wire_word)"
  echo 'a surface check failed' >&2
  exit 1
fi
echo "all landed checks green: green=$green skipped=$skipped diverged=$diverged failed=$failed (wire: $wire_word)"
