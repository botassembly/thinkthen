#!/bin/sh
# Compile this revision's debug engine once and load a fresh fixture extension.
set -eu
. scripts/native-tools-env
export CARGO_TARGET_DIR="$HOME/.local/share/thinkthen-build-tools/duckdb-development-target"
cargo build --offline --locked -p thinkthen-ffi -p loopback --bin loopback --lib --features thinkthen-ffi/fixture-routing
export THINKTHEN_DUCKDB_DEPS="$HOME/.local/share/thinkthen-build-tools/duckdb/1.4.3"
export THINKTHEN_SQL_ARCHIVE="$CARGO_TARGET_DIR/debug/libthinkthen_ffi.a"
scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT HUP INT TERM
THINKTHEN_SQL_TEST_BUILD=ON THINKTHEN_DUCKDB_OUTPUT="$scratch/package" sh databases/duckdb/build.sh
mkdir "$scratch/installed"
tar -czf "$scratch/package.tar.gz" -C "$scratch/package" .
tar -xzf "$scratch/package.tar.gz" -C "$scratch/installed"
source=$THINKTHEN_DUCKDB_DEPS/duckdb-1.4.3
case $(uname -s) in
    Darwin) library=$THINKTHEN_DUCKDB_DEPS/lib/libduckdb.dylib; rpath='@loader_path' ;;
    Linux) library=$THINKTHEN_DUCKDB_DEPS/lib/libduckdb.so; rpath='$ORIGIN' ;;
esac
cp "$library" "$scratch/installed/"
c++ -std=c++17 -pthread -Wall -Wextra -Werror -Wno-unused-parameter \
    -I "$source/src/include" -I "$source/third_party/yyjson/include" \
    databases/duckdb/tests/runner.cpp "$THINKTHEN_DUCKDB_DEPS/static/libduckdb_yyjson.a" \
    "$library" -Wl,-rpath,"$rpath" -o "$scratch/installed/sql"
python3 databases/duckdb/tests/run.py "$scratch/installed" "$CARGO_TARGET_DIR/debug/loopback"
