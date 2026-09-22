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

echo "== sqlite surface: the error-mapping test"
cargo test --release --quiet --lib

echo "== sqlite surface: the slide, as drawn, in the stock CLI"
if [ ! -x .runtimes/sqlite3 ]; then
    echo "   (no stock CLI in .runtimes; fetch it per README)"
    exit 1
fi
ENGINE_NULL=1 .runtimes/sqlite3 :memory: < tests/slide.sql \
    | sed 's/^/   /'

echo "== sqlite surface: null suite"
ENGINE_NULL=1 python3 tests/null_suite.py

echo "== sqlite surface: fast-backend interrupt"
python3 tests/cancel_fast.py

echo "== sqlite surface: volatile stays the flag"
# Punch-list item 5's SQLite half: no function carries
# SQLITE_DETERMINISTIC (0x800 in pragma_function_list's flags), and an
# index expression refuses the call outright.
volatile_out=$(ENGINE_NULL=1 .runtimes/sqlite3 :memory: 2>&1 <<'SQL' || true
.load ./thinkthen
SELECT 'flagged', count(*) FROM pragma_function_list WHERE name LIKE 'thinkthen%' AND (flags & 0x800) != 0;
SELECT 'known', count(*) FROM pragma_function_list WHERE name LIKE 'thinkthen%';
CREATE TABLE t(body TEXT);
CREATE INDEX idx ON t(thinkthen_decide('Is this a complaint?', body));
SQL
)
printf '%s\n' "$volatile_out" | sed 's/^/   /'
grep -q "^flagged|0$" <<<"$volatile_out" \
    || { echo "FAILED   a function carries SQLITE_DETERMINISTIC" >&2; exit 1; }
grep -q "^known|8$" <<<"$volatile_out" \
    || { echo "FAILED   the function list is not the eight" >&2; exit 1; }
grep -q "unsafe use of thinkthen_decide()" <<<"$volatile_out" \
    || { echo "FAILED   an index expression accepted the call" >&2; exit 1; }
echo "ok       no deterministic flag on any of the eight; an index expression refuses"

echo "== sqlite surface: the function examples"
python3 tests/examples.py

echo "== sqlite surface: the table-valued functions"
ENGINE_NULL=1 python3 tests/tvf_suite.py

echo "== sqlite surface: conformance slice, offline"
ENGINE_NULL=1 python3 tests/conformance_driver.py

if curl -sf --max-time 1 http://127.0.0.1:8218/v1/stats >/dev/null 2>&1; then
    echo "== sqlite surface: wire suite against the stub on 8218"
    curl -s -X POST http://127.0.0.1:8218/v1/reset >/dev/null
    STUB_PORT=8218 python3 tests/wire_suite.py
else
    echo "== sqlite surface: wire suite skipped, no stub on 8218"
fi
