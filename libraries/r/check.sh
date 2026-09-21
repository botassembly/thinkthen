#!/usr/bin/env bash
# The R surface's check: the null suite, the conformance slice, and the
# slide sample against the null backend, then the wire suite when the stub
# is up on this surface's port (8215). The wire stub is
# experiments/205-thinkthen-libs/shared running with STUB_PORT=8215
# STUB_DELAY_MS=300.
set -euo pipefail
cd "$(dirname "$0")"

echo "== r surface: null suite"
ENGINE_NULL=1 Rscript tests_null.R

echo "== r surface: conformance slice"
ENGINE_NULL=1 Rscript conformance.R

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
