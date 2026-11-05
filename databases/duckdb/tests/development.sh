#!/bin/sh
# Compile this revision's debug engine once and load a fresh fixture extension.
set -eu
duckdb_version=$(python3 scripts/duckdb_release.py --version)
. scripts/native-tools-env
export CARGO_TARGET_DIR="$HOME/.local/share/thinkthen-build-tools/duckdb-development-target"
if [ "${1:-}" = --docs-only ]; then
    export CARGO_TARGET_DIR="$CARGO_TARGET_DIR/ordinary"
    exec scripts/duckdb-gate --official-hosts-only
fi
if [ "${1:-}" = --shared-sql ]; then
    export CARGO_TARGET_DIR="$CARGO_TARGET_DIR/ordinary"
    cargo fmt --all --check
    cargo test --offline --locked -p thinkthen --features native --test public_contract --test shared_interface
    cargo test --offline --locked -p thinkthen-sql --test request --test public_contract
    scripts/c-gate
    cargo test --offline --locked -p thinkthen-ffi --test typed_projection database_caller_retains_null_invalid_and_terminal_error_protocol
    exit
fi
if [ -z "${THINKTHEN_SQL_ARCHIVE:-}" ]; then
    cargo build --offline --locked -p thinkthen-ffi -p loopback --bin loopback --lib --features thinkthen-ffi/fixture-routing
    THINKTHEN_SQL_ARCHIVE="$CARGO_TARGET_DIR/debug/libthinkthen_ffi.a"
    THINKTHEN_LOOPBACK="$CARGO_TARGET_DIR/debug/loopback"
fi
: "${THINKTHEN_LOOPBACK:?Provide the matching fixture loopback}"
export THINKTHEN_DUCKDB_DEPS="$HOME/.local/share/thinkthen-build-tools/duckdb/$duckdb_version"
export THINKTHEN_SQL_ARCHIVE
if [ "${1:-}" = --full ]; then
    fixture_archive=$THINKTHEN_SQL_ARCHIVE
    cargo build --offline --locked -p thinkthen-ffi --lib --target-dir "$CARGO_TARGET_DIR/ordinary"
    export THINKTHEN_SQL_TEST_ARCHIVE="$fixture_archive"
    export THINKTHEN_SQL_ARCHIVE="$CARGO_TARGET_DIR/ordinary/debug/libthinkthen_ffi.a"
    export THINKTHEN_LOOPBACK
    exec scripts/duckdb-gate
fi
sanitizer=
if [ "${1:-}" = --union-tsan ]; then
    shift
    set -- --union-only "$@"
    sanitizer=-fsanitize=thread
    export CFLAGS="${CFLAGS:-} $sanitizer" CXXFLAGS="${CXXFLAGS:-} $sanitizer" LDFLAGS="${LDFLAGS:-} $sanitizer"
fi
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT HUP INT TERM
THINKTHEN_SQL_TEST_BUILD=ON THINKTHEN_DUCKDB_OUTPUT="$scratch/package" sh databases/duckdb/build.sh
mkdir "$scratch/installed"
tar -czf "$scratch/package.tar.gz" -C "$scratch/package" .
tar -xzf "$scratch/package.tar.gz" -C "$scratch/installed"
source=$THINKTHEN_DUCKDB_DEPS/duckdb-$duckdb_version
case $(uname -s) in
    Darwin) library=$THINKTHEN_DUCKDB_DEPS/lib/libduckdb.dylib; rpath='@loader_path' ;;
    Linux) library=$THINKTHEN_DUCKDB_DEPS/lib/libduckdb.so; rpath='$ORIGIN' ;;
esac
cp "$library" "$scratch/installed/"
c++ ${sanitizer:+"$sanitizer"} -std=c++17 -pthread -Wall -Wextra -Werror -Wno-unused-parameter \
    -I "$source/src/include" -I "$source/third_party/yyjson/include" \
    databases/duckdb/tests/runner.cpp \
    "$library" -Wl,-rpath,"$rpath" -o "$scratch/installed/sql"
if [ "${1:-}" = --stage-only ]; then
    : "${THINKTHEN_DUCKDB_ARTIFACT_DIR:?Provide the consumer staging directory}"
    mkdir -p "$THINKTHEN_DUCKDB_ARTIFACT_DIR"
    cp "$scratch/installed/"* "$THINKTHEN_DUCKDB_ARTIFACT_DIR/"
    exit 0
fi
if [ "${1:-}" = --unknown-only ]; then
    python3 databases/unknown-code-check.py duckdb "$scratch/installed/sql" "$scratch/installed/thinkthen.duckdb_extension"
else
    python3 databases/duckdb/tests/run.py "$scratch/installed" "$THINKTHEN_LOOPBACK" "$@"
fi
