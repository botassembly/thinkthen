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
HOST_TARGET=$(sh "$ROOT/tools/setup.sh" --target)
RUST_TARGET=$(rustc -vV | sed -n 's/^host: //p')
[ "$HOST_TARGET" = "$RUST_TARGET" ] || { echo "duckdb: $RUST_TARGET is not the pinned host $HOST_TARGET" >&2; exit 1; }
CARGO_OUT=${CARGO_TARGET_DIR:-$ROOT/bridge/target}
case $CARGO_OUT in /*) ;; *) CARGO_OUT=$REPO/$CARGO_OUT ;; esac
CMAKE=$(command -v cmake || true)
case $HOST_TARGET in *-apple-darwin) CMAKE=$TOOLS/venv/bin/cmake ;; esac
[ -x "$CMAKE" ] || { echo "duckdb: project-local CMake is missing; run tools/setup.sh --fetch" >&2; exit 77; }
RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"
CFLAGS="${CFLAGS:+$CFLAGS }-ffile-prefix-map=$HOME=/build"
CXXFLAGS="${CXXFLAGS:+$CXXFLAGS }-ffile-prefix-map=$HOME=/build"
export RUSTFLAGS CFLAGS CXXFLAGS
case $HOST_TARGET in *-apple-darwin)
	MACOSX_DEPLOYMENT_TARGET=15.0
	export MACOSX_DEPLOYMENT_TARGET
	# A final Mach-O may report 15.0 even when prebuilt Rust objects require
	# a newer OS. Refuse that sysroot before it can enter a release archive.
	RUST_STDLIB=$(find "$(rustc --print sysroot)/lib/rustlib/$HOST_TARGET/lib" -name 'libstd-*.rlib' -print -quit)
	[ -n "$RUST_STDLIB" ] || { echo 'duckdb: the pinned Rust standard library is missing' >&2; exit 77; }
	if ! otool -l "$RUST_STDLIB" | awk '
		$1 == "minos" { found = 1; if ($2 + 0 > 15.0) bad = 1 }
		END { exit !found || bad }
	'; then
		echo 'duckdb: the Rust standard library requires macOS newer than 15.0' >&2
		exit 1
	fi
;; esac
VERSION=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$REPO/crates/thinkthen/Cargo.toml" | head -n 1)
[ -n "$VERSION" ] || { echo 'duckdb: the ThinkThen version is missing' >&2; exit 1; }

[ -d "$SOURCE" ] && [ -f "$STATIC/libduckdb_static.a" ] || {
	echo 'duckdb: pinned C++ source or static archives are missing; run tools/setup.sh --fetch' >&2
	exit 77
}
cd -- "$REPO"
cargo build --locked --offline --release --manifest-path "$ROOT/bridge/Cargo.toml"
set -- -DCMAKE_BUILD_TYPE=Release "-DCMAKE_C_FLAGS=$CFLAGS" "-DCMAKE_CXX_FLAGS=$CXXFLAGS"
case $HOST_TARGET in *-apple-darwin) set -- "$@" -DCMAKE_OSX_DEPLOYMENT_TARGET=15.0 ;; esac
"$CMAKE" -S "$SOURCE" -B "$BUILD" -G 'Unix Makefiles' "$@" \
	-DBUILD_UNITTESTS=OFF -DBUILD_SHELL=OFF -DEXTENSION_STATIC_BUILD=OFF \
	-DDUCKDB_EXTENSION_CONFIGS="$HERE/extension_config.cmake" \
	-DTHINKTHEN_EXTENSION_VERSION="$VERSION" \
	-DTHINKTHEN_RUST_STATICLIB="$CARGO_OUT/release/libthinkthen_duckdb_bridge.a" \
	-DTHINKTHEN_DUCKDB_STATIC_DIR="$STATIC"
"$CMAKE" --build "$BUILD" --target thinkthen_loadable_extension -j 2
if [ "$HOST_TARGET" = aarch64-unknown-linux-gnu ]; then
	machine=$(readelf -h "$BUILD/extension/thinkthen/thinkthen.duckdb_extension" | sed -n 's/^[[:space:]]*Machine:[[:space:]]*//p')
	[ "$machine" = AArch64 ] || { echo "duckdb: built extension is $machine, not AArch64" >&2; exit 1; }
fi
mkdir -p "$ROOT/build/artifacts/cpp/$HOST_TARGET"
case $HOST_TARGET in
aarch64-apple-darwin) mac_arch=arm64 mac_platform=osx_arm64 ;;
x86_64-apple-darwin) mac_arch=x86_64 mac_platform=osx_amd64 ;;
esac
if [ -n "${mac_arch:-}" ]; then
	[ "$(lipo -archs "$BUILD/extension/thinkthen/thinkthen.duckdb_extension")" = "$mac_arch" ] || {
		echo "duckdb: built extension is not one $mac_arch Mach-O" >&2
		exit 1
	}
	python3 "$HERE/strip_macos.py" "$BUILD/extension/thinkthen/thinkthen.duckdb_extension" "$ROOT/build/thinkthen.duckdb_extension" "$mac_platform"
	sqlite_exports=$(nm -gU "$ROOT/build/thinkthen.duckdb_extension" | grep -c ' _sqlite3_' || true)
	[ "$sqlite_exports" = 0 ] || {
		echo "duckdb: the macOS extension exports $sqlite_exports SQLite names" >&2
		exit 1
	}
else
	cp -- "$BUILD/extension/thinkthen/thinkthen.duckdb_extension" "$ROOT/build/thinkthen.duckdb_extension"
fi
chmod 644 "$ROOT/build/thinkthen.duckdb_extension"
cp -- "$ROOT/build/thinkthen.duckdb_extension" "$ROOT/build/artifacts/cpp/$HOST_TARGET/thinkthen.duckdb_extension"
