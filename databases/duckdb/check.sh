#!/usr/bin/env bash
# The DuckDB surface's check: the build, the null suite, the conformance
# slice, the slide as drawn, and the wire suite when the stub is up on
# this surface's port (8217). The wire stub is
# experiments/205-thinkthen-libs/shared running with STUB_PORT=8217.
set -euo pipefail
cd "$(dirname "$0")"

echo "== duckdb surface: build the extension"
make release >/dev/null
test -s build/release/thinkthen.duckdb_extension

echo "== duckdb surface: the error-mapping test"
cargo test --release --quiet --lib

echo "== duckdb surface: null suite"
tools/null_suite.sh

echo "== duckdb surface: conformance slice"
python3 tools/conformance.py

echo "== duckdb surface: slide sample, as drawn"
ENGINE_NULL=1 tools/run_slide.sh >/dev/null
echo "ok       the slide runs as drawn (output above its run in NOTES)"

echo "== duckdb surface: recognize and relate acceptance, calls as drawn"
ENGINE_NULL=1 tools/run_recognize.sh >/dev/null
if grep -q "Binder Error: Table function cannot contain subqueries" tools/recognize-run/run.log \
  && grep -q "thinkthen usage: relate takes at most 255 records" tools/recognize-run/run.log; then
  echo "ok       the recognize call runs as drawn; the relate subquery line is the pinned divergence"
  echo "ok       the working replacements ran: the edges, the relations, the mentions join, the 255 refusal"
else
  echo "FAILED   the recognize acceptance's pinned lines did not appear; see tools/recognize-run/run.log"
  exit 1
fi

if curl -sf --max-time 1 http://127.0.0.1:8217/v1/stats >/dev/null 2>&1; then
  echo "== duckdb surface: wire suite against the stub on 8217"
  ENGINE_BASE_URL=http://127.0.0.1:8217/v1 tools/wire_suite.sh
else
  echo "== duckdb surface: wire suite skipped, no stub on 8217"
fi
