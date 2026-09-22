#!/usr/bin/env bash
# The TypeScript surface's check: build the addon, run the slide sample,
# every verb, the error kinds, the conformance slice (all offline against
# the null backend), and, when the stub is reachable, the wire proofs.
# scripts/check_surfaces.sh runs this file because it is executable.
set -euo pipefail
cd "$(dirname "$0")"

step() { printf '\n== %s\n' "$*"; }

step "build"
# npm ci installs exactly package-lock.json; the first run needs the
# network (or npm's cache), every later run is offline.
npm ci --no-audit --no-fund --silent
# The gate's copy arms the stand-in's compile-time synthesized partial
# failure so conformance case 74 and the marker test run; `npm run build`
# is the packaging path and never carries the fixture.
npm run build:synthetic --silent

step "the defect kind maps into the failure envelope (shim unit test)"
(cd addon && cargo test --quiet --lib)

step "offline suites (null backend): verbs, errors, conformance, the ten examples, the fast-backend cancel"
ENGINE_NULL=1 node --test tests/*.test.mjs

step "wire suites"
stub_url="http://127.0.0.1:${STUB_PORT:-8212}/v1"
if curl -sf --max-time 1 "$stub_url/stats" >/dev/null 2>&1; then
  ENGINE_BASE_URL="$stub_url" ENGINE_WIDTH=32 THEN_TS_WIRE_URL="$stub_url" \
    node --test tests/bulk.test.mjs
else
  echo "wire tests skipped: no stub on ${STUB_PORT:-8211}"
fi

step "dead address"
THINKTHEN_BASE_URL="http://127.0.0.1:1" THEN_TS_DEAD=1 \
  node --test --test-name-pattern "refused address" tests/errors.test.mjs

step "types"
# A wrong branch is a compile error: the sample itself, typed.
node_modules/.bin/tsc --noEmit --strict --module node16 --moduleResolution node16 \
  --target es2022 tests/types.test.ts

echo "typescript surface: all checks green"
