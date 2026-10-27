#!/bin/sh
# Build one native package from prepared dependencies and a matching Rust archive.
set -eu
cd "$(dirname "$0")/../.."
. scripts/native-build-env
: "${THINKTHEN_DUCKDB_DEPS:?Set the prepared DuckDB dependency directory}"
: "${THINKTHEN_SQL_ARCHIVE:?Set the matching PIC Rust archive}"
: "${THINKTHEN_DUCKDB_OUTPUT:?Set the staging directory}"
source=$THINKTHEN_DUCKDB_DEPS/duckdb-1.4.3
case $(uname -s) in
    Darwin) native="c;m;iconv;System;c;m;$(scripts/native-frameworks --cmake)" ;;
    Linux) native='c;m;rt;pthread;gcc_s;util;rt;pthread;m;dl;c' ;;
    *) exit 2 ;;
esac
build=$(mktemp -d)
trap 'rm -rf "$build"' EXIT HUP INT TERM
mkdir -p "$THINKTHEN_DUCKDB_OUTPUT"
cmake -S databases/duckdb -B "$build" \
    -DDUCKDB_SOURCE="$source" -DDUCKDB_STATIC_DIR="$THINKTHEN_DUCKDB_DEPS/static" \
    -DTHINKTHEN_ARCHIVE="$THINKTHEN_SQL_ARCHIVE" -DTHINKTHEN_NATIVE_LIBS="$native" \
    -DTHINKTHEN_TEST_BUILD="${THINKTHEN_SQL_TEST_BUILD:-OFF}" \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_OSX_DEPLOYMENT_TARGET="$MACOSX_DEPLOYMENT_TARGET"
cmake --build "$build" --parallel 2
cp "$build/thinkthen.duckdb_extension" "$THINKTHEN_DUCKDB_OUTPUT/"
# Strip before appending metadata: stripping afterward removes the footer.
strip -S "$THINKTHEN_DUCKDB_OUTPUT/thinkthen.duckdb_extension"
platform=$("$THINKTHEN_DUCKDB_DEPS/cli/duckdb" -unsigned -csv -noheader -init /dev/null ':memory:' 'PRAGMA platform')
printf '%s' "$platform" > "$build/platform"
# Use the pinned database's own footer writer; unsigned does not skip metadata.
printf '\000' > "$build/null"
cmake -DEXTENSION="$THINKTHEN_DUCKDB_OUTPUT/thinkthen.duckdb_extension" \
    -DNULL_FILE="$build/null" -DPLATFORM_FILE="$build/platform" \
    -DVERSION_FIELD=v1.4.3 -DABI_TYPE=CPP -P "$source/scripts/append_metadata.cmake"
python3 scripts/package_inventory.py stage duckdb "$THINKTHEN_DUCKDB_OUTPUT"
python3 scripts/package_inventory.py duckdb-terms "$source" "$THINKTHEN_DUCKDB_OUTPUT"
