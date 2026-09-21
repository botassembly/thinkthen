#!/usr/bin/env bash
# Build and check every surface. Phase A checks the contract, the stand-in,
# and the conformance file; each surface hooks in as it lands in Phase B,
# by a function of its own that this script calls.
#
# The wire tests need the stub from
# experiments/205-thinkthen-libs/shared running on the loopback, with
# STUB_PORT and STUB_DELAY_MS set before it starts. When the stub is not
# reachable the wire tests skip and the script says so; every surface's
# conformance replay needs no network at all.
set -euo pipefail
cd "$(dirname "$0")/.."
repo=$(pwd)

fail=0
step() { printf '\n== %s\n' "$*"; }
skip_note=''

# The stub's address, when a stub is up.
stub_url="http://127.0.0.1:${STUB_PORT:-8231}/v1"
if curl -sf --max-time 1 "$stub_url/stats" >/dev/null 2>&1; then
  wire=yes
else
  wire=no
  skip_note='wire tests skipped: no stub on the loopback'
fi

step "contract"
(cd contract && cargo test --quiet)

step "stand-in, null backend"
(cd standin && cargo test --quiet --lib)

step "stand-in, wire against the stub"
if [ "$wire" = yes ]; then
  (cd standin && ENGINE_BASE_URL="$stub_url" ENGINE_WIDTH=32 \
    cargo test --quiet --test wire -- --nocapture)
else
  echo "$skip_note"
fi

step "conformance file"
(cd conformance && python3 tools/validate_conformance.py conformance.json)

step "generated function lists"
python3 scripts/generate_functions.py --check

# Each surface lands in Phase B with a check of its own. A surface is
# checked by running its slide sample against the stand-in and its slice of
# the conformance file; the hook is the surface's folder, named here in the
# slide order.
step "surfaces"
for surface in python typescript ruby r rust c; do
  if [ -x "libraries/$surface/check.sh" ]; then
    (cd "libraries/$surface" && ./check.sh) || fail=1
  else
    echo "not landed: libraries/$surface"
  fi
done
for engine in duckdb sqlite postgresql; do
  if [ -x "databases/$engine/check.sh" ]; then
    (cd "databases/$engine" && ./check.sh) || fail=1
  else
    echo "not landed: databases/$engine"
  fi
done

if [ "$fail" -ne 0 ]; then
  echo 'a surface check failed' >&2
  exit 1
fi
echo 'all landed checks green'
