#!/usr/bin/env bash
# The SQLite surface's check: build the extension, run the slide sample
# exactly as drawn in the stock CLI, the null suite, the conformance
# slice, then the wire suite when the stub is up on this surface's port
# (8218). The wire stub is the in-repo tools/wire-stub, which
# scripts/check_surfaces.sh builds and starts with STUB_PORT=8218
# STUB_DELAY_MS=300.
set -euo pipefail
cd "$(dirname "$0")"

# Experimental on macOS: the library spellings below take the Darwin forms
# (dylib, DYLD_LIBRARY_PATH); Linux is the gate's platform.
case "$(uname -s)" in
  Darwin) LIB_EXT=dylib; LIB_PATH_VAR=DYLD_LIBRARY_PATH ;;
  *)      LIB_EXT=so;    LIB_PATH_VAR=LD_LIBRARY_PATH ;;
esac

echo "== sqlite surface: build the extension"
# The annotate partial-failure fixture is compiled in only for a build
# that asks for it (standin/Cargo.toml, the `synthetic-partial` feature):
# the conformance slice replays that record (case 74). The release
# artifact built by package.sh carries no fixture.
export RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=$HOME=/build"
cargo build --release --quiet --features synthetic-partial --locked
cp target/release/libthinkthen0.$LIB_EXT thinkthen.so

# The floor is 3.50.0 (below it a CHECK constraint in an untrusted
# database reaches the functions), so the Python tests run against a host
# at or above the floor. A host already new enough (macOS 26 carries
# 3.51.0) is used as-is; otherwise the amalgamation in .runtimes supplies
# one (see tests/host_sqlite.sh for the one-time fetch).
if ! python3 -c 'import sqlite3,sys; sys.exit(0 if tuple(int(x) for x in sqlite3.sqlite_version.split(".")) >= (3,50,0) else 1)'; then
    HOST_DIR=$(tests/host_sqlite.sh)
    export "$LIB_PATH_VAR=$HOST_DIR${!LIB_PATH_VAR:+:${!LIB_PATH_VAR}}"
fi
HOST=$(python3 -c 'import sqlite3;print(sqlite3.sqlite_version)')
python3 -c 'import sqlite3,sys; sys.exit(0 if tuple(int(x) for x in sqlite3.sqlite_version.split(".")) >= (3,50,0) else 1)' \
    || { echo "FAILED   the test host is below the floor: $HOST" >&2; exit 1; }
echo "   host SQLite $HOST"

echo "== sqlite surface: the error-mapping test"
cargo test --release --quiet --lib --features synthetic-partial --locked

echo "== sqlite surface: the slide, as drawn, in the stock CLI"
if [ ! -x .runtimes/sqlite3 ]; then
    echo "   (no stock CLI in .runtimes; fetch it per README)"
    exit 1
fi
ENGINE_NULL=1 .runtimes/sqlite3 :memory: < tests/slide.sql \
    | sed 's/^/   /'

echo "== sqlite surface: null suite"
ENGINE_NULL=1 python3 tests/null_suite.py

echo "== sqlite surface: the untrusted-schema refusals, at the floor"
python3 tests/schema_refusal.py

echo "== sqlite surface: the named-file door (review 4, item 7)"
# The deterministic shapes the swap race rides on: a symlink refuses at
# the door (pre-fix it read through to the parser), a fifo refuses at
# the door (pre-fix a straddled swap parked the process in open()).
THINKTHEN_NULL=1 python3 tools/file_door.py target/release/libthinkthen0.so

echo "== sqlite surface: single-row deadline, offline"
python3 tests/single_row_cancel.py null

echo "== sqlite surface: single-row deadline and interrupt, loopback"
python3 tests/single_row_cancel.py wire

echo "== sqlite surface: two connections, one closed"
python3 tests/two_connections.py

echo "== sqlite surface: fast-backend interrupt"
python3 tests/cancel_fast.py

echo "== sqlite surface: volatile stays the flag"
# Punch-list item 5's SQLite half: no function carries
# SQLITE_DETERMINISTIC (0x800 in pragma_function_list's flags), and an
# index expression refuses the call outright.
volatile_out=$(ENGINE_NULL=1 .runtimes/sqlite3 :memory: 2>&1 <<'SQL' || true
.load ./thinkthen
SELECT 'flagged', count(*) FROM pragma_function_list WHERE name LIKE 'thinkthen%' AND (flags & 0x800) != 0;
SELECT 'known', count(DISTINCT name) FROM pragma_function_list WHERE name LIKE 'thinkthen%';
SELECT 'registrations', count(*) FROM pragma_function_list WHERE name LIKE 'thinkthen%';
CREATE TABLE t(body TEXT);
CREATE INDEX idx ON t(thinkthen_decide('Is this a complaint?', body));
SQL
)
printf '%s\n' "$volatile_out" | sed 's/^/   /'
grep -q "^flagged|0$" <<<"$volatile_out" \
    || { echo "FAILED   a function carries SQLITE_DETERMINISTIC" >&2; exit 1; }
grep -q "^known|8$" <<<"$volatile_out" \
    || { echo "FAILED   the function names are not the eight" >&2; exit 1; }
# The six judgment scalars carry a third arity, the deadline door; the
# registration count is part of the surface's shape.
grep -q "^registrations|14$" <<<"$volatile_out" \
    || { echo "FAILED   the registrations are not the fourteen" >&2; exit 1; }
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
    echo "== sqlite surface: per-connection interrupt on the wire"
    STUB_PORT=8218 python3 tests/two_connections.py wire
else
    echo "== sqlite surface: wire suite skipped, no stub on 8218"
fi
