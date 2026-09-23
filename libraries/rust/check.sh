#!/usr/bin/env bash
# The Rust surface's check: the null suite, the conformance slice, and the
# wire suite when the stub is up on this surface's port (8213). The
# wire stub is the in-repo tools/wire-stub, which scripts/check_surfaces.sh
# builds and starts with STUB_PORT=8213 STUB_DELAY_MS=300.
set -euo pipefail
cd "$(dirname "$0")"

echo "== rust surface: build and null suite"
# The annotate partial-failure fixture is armed only by the stand-in's
# compile-time `synthetic-partial` feature (standin/NOTES.md, finding 7);
# tests/verbs.rs replays that record. No environment variable can arm it.
ENGINE_NULL=1 cargo test --quiet --features synthetic-partial --locked

echo "== rust surface: fast-backend deadline, the poll-bug shape"
ENGINE_NULL=1 cargo test --quiet --test deadline_fast --locked

echo "== rust surface: the function examples"
ENGINE_NULL=1 cargo test --quiet --test examples --locked

echo "== rust surface: the Polars Series door (the 1.95 toolchain Polars needs)"
# Serial test threads: the request-count equality reads the process-wide
# usage counter, so the two measurements must not race a sibling test.
ENGINE_NULL=1 RUSTUP_TOOLCHAIN=1.95 cargo test --quiet --features polars --test polars_door --locked -- --test-threads=1

echo "== rust surface: conformance slice"
# Case 74 replays the stand-in's compile-time fixture, same as the null
# suite.
THINKTHEN_NULL=1 cargo run --quiet --locked --features synthetic-partial --example conformance

if curl -sf --max-time 1 http://127.0.0.1:8213/v1/stats >/dev/null 2>&1; then
  echo "== rust surface: wire suite against the stub on 8213"
  ENGINE_BASE_URL=http://127.0.0.1:8213/v1 ENGINE_WIDTH=32 \
    cargo test --quiet --test wire --locked -- --nocapture
else
  echo "skip     wire-rust: no stub on 127.0.0.1:8213"
fi
