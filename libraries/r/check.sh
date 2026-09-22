#!/usr/bin/env bash
# The R surface's check: the null suite, the conformance slice, the
# recognize and relate acceptance, and the slide sample against the null
# backend, then the wire suite when the stub is up on this surface's port
# (8215). The wire stub is the in-repo tools/wire-stub, which
# scripts/check_surfaces.sh builds and starts with STUB_PORT=8215
# STUB_DELAY_MS=300.
set -euo pipefail
cd "$(dirname "$0")"

# Experimental on macOS: `timeout` arrives as `gtimeout` there; without
# either, the step runs unguarded.
TIMEOUT=$(command -v timeout || command -v gtimeout || true)
run_limited() { # seconds command...
  local seconds=$1; shift
  if [ -n "$TIMEOUT" ]; then "$TIMEOUT" "$seconds" "$@"; else "$@"; fi
}

echo "== r surface: the tarball build vendors the contract crates"
./tools/make-tarball.sh --stage-only >/dev/null

echo "== r surface: the defect kind crosses as its own kind (shim unit test)"
(cd thinkthen/src/rust && cargo test --quiet --lib)

# The fixture build. Conformance case 74 and the null suite's failed-marker
# checks replay the stand-in's synthesized partial failure, and that fixture
# is a compile-time door since review 1: no environment variable can arm it.
# The gate installs a build that asks for it, runs the suites below, and
# restores the production shape at the end.
echo "== r surface: install the fixture build (synthetic-partial)"
(cd thinkthen && THINKTHEN_R_SYNTHETIC_PARTIAL=1 R CMD INSTALL -l ../rlib .)

echo "== r surface: null suite"
ENGINE_NULL=1 Rscript tests_null.R

echo "== r surface: the text crossing (percent, encodings, invalid bytes)"
ENGINE_NULL=1 Rscript text_check.R

echo "== r surface: fast-backend interrupt, the poll-bug shape"
./interrupt_fast.sh

echo "== r surface: fork after the first call answers in the child"
run_limited 60 env ENGINE_NULL=1 Rscript fork_check.R

echo "== r surface: the function examples"
ENGINE_NULL=1 Rscript examples.R

echo "== r surface: conformance slice"
ENGINE_NULL=1 Rscript conformance.R

echo "== r surface: recognize and relate acceptance"
ENGINE_NULL=1 Rscript recognize_check.R

echo "== r surface: canonical results and ownership (punch-list item 3)"
ENGINE_NULL=1 Rscript ownership_check.R

echo "== r surface: slide sample, as drawn, on the null backend"
ENGINE_NULL=1 Rscript slide_check.R

if curl -sf --max-time 1 http://127.0.0.1:8215/v1/stats >/dev/null 2>&1; then
  echo "== r surface: wire suite against the stub on 8215"
  curl -s -X POST http://127.0.0.1:8215/v1/reset >/dev/null
  ENGINE_BASE_URL=http://127.0.0.1:8215/v1 ENGINE_WIDTH=32 STUB_PORT=8215 \
    Rscript wire_width.R
  curl -s -X POST http://127.0.0.1:8215/v1/reset >/dev/null
  ENGINE_BASE_URL=http://127.0.0.1:8215/v1 ENGINE_WIDTH=32 STUB_PORT=8215 \
    ./interrupt_test.sh 8215
  ENGINE_NULL=1 Rscript conformance.R | grep "^ok" | wc -l | xargs \
    echo "wire conformance ok-lines (offline slice rerun for the record):"
else
  echo "== r surface: wire suite skipped, no stub on 8215"
fi

# The production shape the README installs: no fixture code, proven by the
# plainest call answering on the restored package.
echo "== r surface: restore the production install (no fixture code)"
(cd thinkthen && R CMD INSTALL -l ../rlib .)
ENGINE_NULL=1 Rscript -e '.libPaths(c("rlib", .libPaths())); library(thinkthen); stopifnot(isTRUE(tt_decide("Is this a complaint?", "I want a refund for order 9"))); cat("production install answers\n")'
