#!/usr/bin/env bash
# The SQLite surface's check: build the extension, run the slide sample
# exactly as drawn in the stock CLI, the null suite, the conformance
# slice, then the wire suite when the stub is up on this surface's port
# (8218). The wire stub is experiments/205-thinkthen-libs/shared running
# with STUB_PORT=8218 STUB_DELAY_MS=300.
set -euo pipefail
cd "$(dirname "$0")"

echo "== sqlite surface: build the extension"
cargo build --release --quiet
cp target/release/libthinkthen0.so thinkthen.so

echo "== sqlite surface: the slide, as drawn, in the stock CLI"
if [ ! -x .runtimes/sqlite3 ]; then
    echo "   (no stock CLI in .runtimes; fetch it per README)"
    exit 1
fi
ENGINE_NULL=1 .runtimes/sqlite3 :memory: < tests/slide.sql \
    | sed 's/^/   /'

echo "== sqlite surface: null suite"
ENGINE_NULL=1 python3 tests/null_suite.py

echo "== sqlite surface: conformance slice, offline"
ENGINE_NULL=1 python3 tests/conformance_driver.py

if curl -sf --max-time 1 http://127.0.0.1:8218/v1/stats >/dev/null 2>&1; then
    echo "== sqlite surface: wire suite against the stub on 8218"
    curl -s -X POST http://127.0.0.1:8218/v1/reset >/dev/null
    STUB_PORT=8218 python3 tests/wire_suite.py
else
    echo "== sqlite surface: wire suite skipped, no stub on 8218"
fi
