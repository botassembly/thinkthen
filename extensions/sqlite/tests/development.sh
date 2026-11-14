#!/bin/sh
# Build only the SQLite adapter against an explicitly selected engine archive.
set -eu
. scripts/native-tools-env
if [ "${1:-}" = --databases ]; then
    uv tool run --from ruff==0.13.2 ruff check --select F401,F841,ERA,BLE,E722,S110 \
        extensions/rules.py extensions/sqlite/tests/docs.py extensions/sqlite/tests/ideal.py extensions/sqlite/tests/run.py \
        scripts/duckdb_ideal_guide.py scripts/sqlite_guide_examples.py databases/duckdb/generate-types.py \
        databases/duckdb/tests/admission_cases.py databases/duckdb/tests/cases.py databases/duckdb/tests/documented_sql.py \
        databases/duckdb/tests/ideal_guides.py databases/duckdb/tests/interface_cases.py databases/duckdb/tests/migration_cases.py \
        databases/duckdb/tests/run.py databases/duckdb/tests/typed_api.py
    scripts/guide-examples --inventory --surface sqlite --surface duckdb --surface sql
fi
if [ "${1:-}" = --examples-only ]; then
    : "${THINKTHEN_SQL_TEST_ARCHIVE:?Provide the already built fixture engine archive}"
    : "${THINKTHEN_LOOPBACK:?Provide the matching fixture loopback}"
    printf 'SQLite development source: %s\n' "$(git rev-parse HEAD)"
    shasum -a 256 "$THINKTHEN_SQL_TEST_ARCHIVE" "$THINKTHEN_LOOPBACK"
fi
if [ -z "${THINKTHEN_SQL_TEST_ARCHIVE:-}" ]; then
    # Compile this revision's debug fixture engine once, as the DuckDB development check does.
    CARGO_TARGET_DIR="$HOME/.local/share/thinkthen-build-tools/sqlite-development-target"
    export CARGO_TARGET_DIR
    cargo build --offline --locked -p thinkthen-ffi -p loopback --bin loopback --lib --features thinkthen-ffi/fixture-routing
    THINKTHEN_SQL_TEST_ARCHIVE="$CARGO_TARGET_DIR/debug/libthinkthen_ffi.a"
    THINKTHEN_LOOPBACK="$CARGO_TARGET_DIR/debug/loopback"
fi
: "${THINKTHEN_LOOPBACK:?Provide the matching fixture loopback}"
export THINKTHEN_LOOPBACK THINKTHEN_SQL_TEST_ARCHIVE
if [ "${1:-}" = --databases ]; then
    THINKTHEN_SQL_ARCHIVE="$THINKTHEN_SQL_TEST_ARCHIVE" scripts/duckdb-gate --development --ideal-guides
fi
. scripts/native-build-env
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT HUP INT TERM
python3 extensions/rules.py "$scratch/rules.h"
sqlite_flags=${THINKTHEN_SQLITE_CFLAGS:-}
sqlite="$THINKTHEN_SQLITE_ROOT/sqlite-3.41.0"
jansson="$THINKTHEN_JANSSON_ROOT"
for source in extension rows source question replay; do
    cc -std=c11 -Wall -Wextra -Werror -O2 -fPIC -fvisibility=hidden $sqlite_flags \
        -I "$scratch" -I "$sqlite" -I "$jansson/include" -I crates/thinkthen-ffi/include \
        -c "extensions/sqlite/src/$source.c" -o "$scratch/$source.o"
done
cc -std=c11 -Wall -Wextra -Werror -O2 -fPIC -fvisibility=hidden $sqlite_flags \
    -DTHINKTHEN_TEST_BUILD -DTHINKTHEN_TEST_FIXTURE -I crates/thinkthen-ffi/include \
    -I conformance/native -c databases/duckdb/tests/routes.c -o "$scratch/routes.o"
case $(uname -s) in
    Darwin) link="-dynamiclib $(scripts/native-frameworks) -liconv"; suffix=dylib ;;
    Linux) link=-shared; suffix=so ;;
esac
cc $link "$scratch"/*.o "$THINKTHEN_SQL_TEST_ARCHIVE" "$jansson/lib/libjansson.a" \
    -lpthread -ldl -lm -o "$scratch/thinkthen_sqlite.$suffix"
status=0
for version in 3.41.0 3.54.1; do
    sqlite="$THINKTHEN_SQLITE_ROOT/sqlite-$version"
    cc -std=c11 -Wall -Wextra -Werror -pthread -I "$sqlite" -I "$jansson/include" \
        extensions/sqlite/tests/consumer.c "$sqlite/libsqlite3.a" "$jansson/lib/libjansson.a" \
        -ldl -lm -o "$scratch/consumer"
    if [ "${1:-}" = --ideal-only ]; then
        THINKTHEN_SQLITE_CONSUMER="$scratch/consumer" THINKTHEN_SQLITE_EXTENSION="$scratch/thinkthen_sqlite.$suffix" scripts/guide-examples --surface sqlite || status=1
    elif [ "${1:-}" = --unknown-only ]; then
        python3 databases/unknown-code-check.py sqlite "$scratch/consumer" "$scratch/thinkthen_sqlite.$suffix"
    elif [ "${1:-}" = --examples-only ]; then
        python3 extensions/sqlite/tests/live.py "$scratch/consumer" "$scratch/thinkthen_sqlite.$suffix" extensions/sqlite/examples/all-ten.sql --stub
    else
        python3 extensions/sqlite/tests/run.py "$scratch/consumer" "$scratch/thinkthen_sqlite.$suffix" "$THINKTHEN_LOOPBACK" "$PWD" || status=1
        THINKTHEN_SQLITE_CONSUMER="$scratch/consumer" THINKTHEN_SQLITE_EXTENSION="$scratch/thinkthen_sqlite.$suffix" scripts/guide-examples --surface sqlite || status=1
    fi
done
exit "$status"
