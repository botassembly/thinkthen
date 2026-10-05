#!/bin/sh
# Build the pinned C++ extension and Rust bridge from installed inputs.
set -eu
HERE=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
ROOT=$(CDPATH='' cd -- "$HERE/.." && pwd)
REPO=$(CDPATH='' cd -- "$ROOT/../.." && pwd)
INPUTS_HERE=$ROOT/tools
. "$INPUTS_HERE/inputs.sh"
DUCKDB_VERSIONS=$(input_versions)
DEFAULT_VERSION=${DUCKDB_VERSIONS%% *}
HOST_TARGET=$(input_target)
input_select "${THINKTHEN_DUCKDB_VERSION:-}" "$HOST_TARGET"
SOURCE=${THINKTHEN_DUCKDB_CPP_SOURCE:-$TOOLS/source}
STATIC=${THINKTHEN_DUCKDB_CPP_STATIC_DIR:-$TOOLS/static-libs}
BUILD_BASE=${THINKTHEN_DUCKDB_CPP_BUILD:-$ROOT/build/cpp}
for absolute in "$SOURCE" "$STATIC" "$BUILD_BASE"; do
	case $absolute in /*) ;; *) echo 'duckdb: source, static and CMake base paths must be absolute' >&2; exit 2 ;; esac
done
BUILD=$BUILD_BASE/$DUCKDB_VERSION/$HOST_TARGET
ARTIFACT=$ROOT/build/artifacts/cpp/$DUCKDB_VERSION/$HOST_TARGET/thinkthen.duckdb_extension
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
python3 "$INPUTS_HERE/validate_inputs.py" --source "$SOURCE" --commit "$duckdb_source_commit" --static "$STATIC" --manifest "$HERE/$manifest"
cd -- "$REPO"
cargo build --locked --offline --release --manifest-path "$ROOT/bridge/Cargo.toml"
set -- -DCMAKE_BUILD_TYPE=Release "-DCMAKE_C_FLAGS=$CFLAGS" "-DCMAKE_CXX_FLAGS=$CXXFLAGS"
case $HOST_TARGET in *-apple-darwin) set -- "$@" -DCMAKE_OSX_DEPLOYMENT_TARGET=15.0 ;; esac
"$CMAKE" -S "$SOURCE" -B "$BUILD" -G 'Unix Makefiles' "$@" \
	-DBUILD_UNITTESTS=OFF -DBUILD_SHELL=OFF -DEXTENSION_STATIC_BUILD=OFF \
	-DDUCKDB_EXTENSION_CONFIGS="$HERE/extension_config.cmake" \
	-DTHINKTHEN_EXTENSION_VERSION="$VERSION" \
	-DTHINKTHEN_RUST_STATICLIB="$CARGO_OUT/release/libthinkthen_duckdb_bridge.a" \
	-DTHINKTHEN_DUCKDB_STATIC_DIR="$STATIC" \
	-DTHINKTHEN_DUCKDB_SOURCE_COMMIT="$duckdb_source_commit" \
	-DTHINKTHEN_DUCKDB_ARCHIVE_MANIFEST="$HERE/$manifest"
"$CMAKE" --build "$BUILD" --target thinkthen_loadable_extension -j 2
case $HOST_TARGET in
x86_64-unknown-linux-gnu) elf_arch='Advanced Micro Devices X86-64' ;;
aarch64-unknown-linux-gnu) elf_arch=AArch64 ;;
esac
if [ -n "${elf_arch:-}" ]; then
	machine=$(readelf -h "$BUILD/extension/thinkthen/thinkthen.duckdb_extension" | sed -n 's/^[[:space:]]*Machine:[[:space:]]*//p')
	[ "$machine" = "$elf_arch" ] || { echo "duckdb: built extension is $machine, not $elf_arch" >&2; exit 1; }
fi
. "$REPO/sdlc/scripts/scratch.sh"
mkdir -p "${ARTIFACT%/*}"
scratch_dir processed "${ARTIFACT%/*}/processed.XXXXXX"
FINAL=$processed/thinkthen.duckdb_extension
case $HOST_TARGET in
aarch64-apple-darwin) mac_arch=arm64 mac_platform=osx_arm64 ;;
x86_64-apple-darwin) mac_arch=x86_64 mac_platform=osx_amd64 ;;
esac
if [ -n "${mac_arch:-}" ]; then
	[ "$(lipo -archs "$BUILD/extension/thinkthen/thinkthen.duckdb_extension")" = "$mac_arch" ] || {
		echo "duckdb: built extension is not one $mac_arch Mach-O" >&2
		exit 1
	}
	python3 "$HERE/strip_macos.py" "$BUILD/extension/thinkthen/thinkthen.duckdb_extension" "$FINAL" "$mac_platform"
	exports=$(nm -gU "$FINAL")
	bridge_exports=$(printf '%s\n' "$exports" | grep -c -E ' (_sqlite3_|_rust_|__R|__ZN.*17h[0-9a-f]{16}E$)|[Pp][Aa][Nn][Ii][Cc]' || true)
	[ "$bridge_exports" = 0 ] || {
		echo "duckdb: the macOS extension exports $bridge_exports SQLite, Rust or panic names" >&2
		exit 1
	}
else
	cp -- "$BUILD/extension/thinkthen/thinkthen.duckdb_extension" "$FINAL"
fi
chmod 644 "$FINAL"
python3 "$HERE/verify_footer.py" "$FINAL" "$HOST_TARGET" "$DUCKDB_VERSION" "$VERSION"
mv "$FINAL" "$ARTIFACT"
if [ "$DUCKDB_VERSION" = "$DEFAULT_VERSION" ]; then
	# Sole alias writer: only a validated default build refreshes the convenience copy.
	scratch_dir alias_stage "$ROOT/build/alias.XXXXXX"
	cp "$ARTIFACT" "$alias_stage/thinkthen.duckdb_extension"
	mv "$alias_stage/thinkthen.duckdb_extension" "$ROOT/build/thinkthen.duckdb_extension"
fi
