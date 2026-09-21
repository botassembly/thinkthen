#!/usr/bin/env bash
# The Rust surface's check: the null suite, the conformance slice, and the
# wire suite when the stub is up on this surface's port (8213). The wire
# stub is experiments/205-thinkthen-libs/shared running with STUB_PORT=8213
# STUB_DELAY_MS=300.
set -euo pipefail
cd "$(dirname "$0")"

echo "== rust surface: build and null suite"
ENGINE_NULL=1 cargo test --quiet

echo "== rust surface: conformance slice"
ENGINE_NULL=1 cargo run --quiet --example conformance

if curl -sf --max-time 1 http://127.0.0.1:8213/v1/stats >/dev/null 2>&1; then
  echo "== rust surface: wire suite against the stub on 8213"
  ENGINE_BASE_URL=http://127.0.0.1:8213/v1 ENGINE_WIDTH=32 \
    cargo test --quiet --test wire -- --nocapture
else
  echo "== rust surface: wire suite skipped, no stub on 8213"
fi
