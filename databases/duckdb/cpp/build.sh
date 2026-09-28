#!/bin/sh
# Build the pinned C++ extension and Rust bridge from installed inputs.
set -eu
HERE=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH='' cd -- "$HERE/.." && pwd)
REPO=$(CDPATH='' cd -- "$ROOT/../.." && pwd)
. "$ROOT/tools/version.env"
TOOLS=${THINKTHEN_TOOLCHAINS:-$HOME/.cache/thinkthen-toolchains}/duckdb/$DUCKDB_VERSION
SOURCE=${THINKTHEN_DUCKDB_CPP_SOURCE:-$TOOLS/source}
STATIC=${THINKTHEN_DUCKDB_CPP_STATIC_DIR:-$TOOLS/static-libs}
BUILD=${THINKTHEN_DUCKDB_CPP_BUILD:-$ROOT/build/cpp}
TARGET=${CARGO_TARGET_DIR:-$ROOT/bridge/target}
case $TARGET in /*) ;; *) TARGET=$REPO/$TARGET ;; esac
VERSION=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$REPO/crates/thinkthen/Cargo.toml" | head -n 1)
[ -n "$VERSION" ] || { echo 'duckdb: the ThinkThen version is missing' >&2; exit 1; }

[ -d "$SOURCE" ] && [ -f "$STATIC/libduckdb_static.a" ] || {
	echo 'duckdb: pinned C++ source or static archives are missing; run tools/setup.sh --fetch' >&2
	exit 77
}
cd -- "$REPO"
cargo build --locked --offline --release --manifest-path "$ROOT/bridge/Cargo.toml"
cmake -S "$SOURCE" -B "$BUILD" -G 'Unix Makefiles' -DCMAKE_BUILD_TYPE=Release \
	-DBUILD_UNITTESTS=OFF -DBUILD_SHELL=OFF -DEXTENSION_STATIC_BUILD=OFF \
	-DDUCKDB_EXTENSION_CONFIGS="$HERE/extension_config.cmake" \
	-DTHINKTHEN_EXTENSION_VERSION="$VERSION" \
	-DTHINKTHEN_RUST_STATICLIB="$TARGET/release/libthinkthen_duckdb_bridge.a" \
	-DTHINKTHEN_DUCKDB_STATIC_DIR="$STATIC"
cmake --build "$BUILD" --target thinkthen_loadable_extension -j 2
mkdir -p "$ROOT/build/artifacts/cpp/x86_64-unknown-linux-gnu"
cp -- "$BUILD/extension/thinkthen/thinkthen.duckdb_extension" "$ROOT/build/thinkthen.duckdb_extension"
chmod 644 "$ROOT/build/thinkthen.duckdb_extension"
cp -- "$ROOT/build/thinkthen.duckdb_extension" "$ROOT/build/artifacts/cpp/x86_64-unknown-linux-gnu/thinkthen.duckdb_extension"
