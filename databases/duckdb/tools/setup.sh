#!/bin/sh
# Pinned stock DuckDB hosts, C++ source, archives and local test tools.
# --inputs prints the selected native input identity without a download.
set -eu
HERE=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
. "$HERE/version.env"
case $(uname -s):$(uname -m) in
Linux:x86_64)
	target=x86_64-unknown-linux-gnu platform=linux_amd64
	cli_asset=duckdb_cli-linux-amd64.zip
	cli_zip_hash=$DUCKDB_CLI_ZIP_SHA256 cli_hash=$DUCKDB_CLI_SHA256
	static_asset=static-libs-linux-amd64.zip static_hash=$DUCKDB_STATIC_ZIP_SHA256
	older_asset=duckdb_cli-linux-amd64.zip
	older_zip_hash=$DUCKDB_OLDER_CLI_ZIP_SHA256 older_hash=$DUCKDB_OLDER_CLI_SHA256
	manifest=archive-sha256.txt
	;;
Linux:aarch64)
	target=aarch64-unknown-linux-gnu platform=linux_arm64
	cli_asset=duckdb_cli-linux-arm64.zip
	cli_zip_hash=$DUCKDB_LINUX_ARM64_CLI_ZIP_SHA256 cli_hash=$DUCKDB_LINUX_ARM64_CLI_SHA256
	static_asset=static-libs-linux-arm64.zip static_hash=$DUCKDB_LINUX_ARM64_STATIC_ZIP_SHA256
	older_asset=duckdb_cli-linux-arm64.zip
	older_zip_hash=$DUCKDB_LINUX_ARM64_OLDER_CLI_ZIP_SHA256 older_hash=$DUCKDB_LINUX_ARM64_OLDER_CLI_SHA256
	manifest=archive-sha256-linux-arm64.txt
	;;
Darwin:arm64)
	target=aarch64-apple-darwin platform=osx_arm64
	cli_asset=duckdb_cli-osx-arm64.zip
	cli_zip_hash=$DUCKDB_OSX_ARM64_CLI_ZIP_SHA256 cli_hash=$DUCKDB_OSX_ARM64_CLI_SHA256
	static_asset=static-libs-osx-arm64.zip static_hash=$DUCKDB_OSX_ARM64_STATIC_ZIP_SHA256
	older_asset=duckdb_cli-osx-arm64.zip
	older_zip_hash=$DUCKDB_OSX_ARM64_OLDER_CLI_ZIP_SHA256 older_hash=$DUCKDB_OSX_ARM64_OLDER_CLI_SHA256
	manifest=archive-sha256-osx-arm64.txt
	;;
*) echo "setup: no pinned DuckDB C++ inputs for $(uname -s):$(uname -m)" >&2; exit 77 ;;
esac
case ${1:-} in
--target) echo "$target"; exit 0 ;;
--inputs) printf '%s %s %s %s %s %s %s\n' "$target" "$platform" "$cli_asset" "$cli_zip_hash" "$static_asset" "$static_hash" "$manifest"; exit 0 ;;
--fetch|'') ;;
*) echo 'usage: setup.sh [--fetch|--target|--inputs]' >&2; exit 2 ;;
esac
HOME_DIR=${THINKTHEN_TOOLCHAINS:-$HOME/.cache/thinkthen-toolchains}/duckdb/$DUCKDB_VERSION
mkdir -p "$HOME_DIR"
sha256() { if command -v sha256sum >/dev/null 2>&1; then sha256sum "$1"; else shasum -a 256 "$1"; fi; }
verify() {
	[ -f "$2" ] && [ "$(sha256 "$2" | cut -d ' ' -f 1)" = "$1" ] || {
		echo "setup: $2 differs from its pinned SHA-256" >&2
		exit 1
	}
}
if [ "${1:-}" = --fetch ]; then
	zip="$HOME_DIR/$cli_asset"
	[ -f "$zip" ] || curl -fsSL -o "$zip" "https://github.com/duckdb/duckdb/releases/download/$DUCKDB_VERSION/$cli_asset"
	verify "$cli_zip_hash" "$zip"
	unzip -o -q "$zip" duckdb -d "$HOME_DIR"
	[ -d "$HOME_DIR/source" ] || git clone --quiet --depth 1 --branch "$DUCKDB_VERSION" https://github.com/duckdb/duckdb.git "$HOME_DIR/source"
	static_zip="$HOME_DIR/$static_asset"
	[ -f "$static_zip" ] || curl -fsSL -o "$static_zip" "https://github.com/duckdb/duckdb/releases/download/$DUCKDB_VERSION/$static_asset"
	verify "$static_hash" "$static_zip"
	mkdir -p "$HOME_DIR/static-libs"
	unzip -o -q "$static_zip" -d "$HOME_DIR/static-libs"
	older_zip="$HOME_DIR/duckdb_cli-$DUCKDB_OLDER_VERSION-$target.zip"
	[ -f "$older_zip" ] || curl -fsSL -o "$older_zip" "https://github.com/duckdb/duckdb/releases/download/$DUCKDB_OLDER_VERSION/$older_asset"
	verify "$older_zip_hash" "$older_zip"
	mkdir -p "$HOME_DIR/older-host"
	unzip -o -q "$older_zip" duckdb -d "$HOME_DIR/older-host"
	if [ "$target" = aarch64-apple-darwin ]; then
		wheel=${DUCKDB_OSX_ARM64_CMAKE_WHEEL_URL##*/}
		[ -f "$HOME_DIR/$wheel" ] || curl -fsSL -o "$HOME_DIR/$wheel" "$DUCKDB_OSX_ARM64_CMAKE_WHEEL_URL"
		verify "$DUCKDB_OSX_ARM64_CMAKE_WHEEL_SHA256" "$HOME_DIR/$wheel"
	fi
fi
[ -x "$HOME_DIR/duckdb" ] || { echo "setup: the pinned $DUCKDB_VERSION host is missing; run tools/setup.sh --fetch" >&2; exit 77; }
verify "$cli_hash" "$HOME_DIR/duckdb"
[ -x "$HOME_DIR/older-host/duckdb" ] || { echo "setup: the pinned $DUCKDB_OLDER_VERSION host is missing; run tools/setup.sh --fetch" >&2; exit 77; }
verify "$older_hash" "$HOME_DIR/older-host/duckdb"
[ -d "$HOME_DIR/source" ] && [ -f "$HOME_DIR/static-libs/libduckdb_static.a" ] || {
	echo 'setup: DuckDB C++ source or static archives are missing; run tools/setup.sh --fetch' >&2
	exit 77
}
[ "$(git -C "$HOME_DIR/source" rev-parse HEAD)" = "$DUCKDB_CPP_SOURCE_COMMIT" ] || {
	echo 'setup: DuckDB C++ source differs from the pinned commit' >&2
	exit 1
}
expected=$(wc -l <"$HERE/../cpp/$manifest" | tr -d ' ')
actual=$(find "$HOME_DIR/static-libs" -name 'lib*.a' -type f | wc -l | tr -d ' ')
[ "$actual" = "$expected" ] || { echo 'setup: DuckDB static archive set differs from its manifest' >&2; exit 1; }
while read -r hash name; do verify "$hash" "$HOME_DIR/static-libs/$name"; done <"$HERE/../cpp/$manifest"
python3 "$HERE/source_checks.py" --requirements "$HERE/requirements.txt" >/dev/null
if [ ! -x "$HOME_DIR/venv/bin/python" ]; then uv venv --offline --python 3.13 "$HOME_DIR/venv"; fi
if [ "${1:-}" = --fetch ]; then
	uv pip install --quiet --python "$HOME_DIR/venv/bin/python" -r "$HERE/requirements.txt"
else
	uv pip install --offline --quiet --python "$HOME_DIR/venv/bin/python" -r "$HERE/requirements.txt"
fi
if [ "$target" = aarch64-apple-darwin ]; then
	wheel=${DUCKDB_OSX_ARM64_CMAKE_WHEEL_URL##*/}
	verify "$DUCKDB_OSX_ARM64_CMAKE_WHEEL_SHA256" "$HOME_DIR/$wheel"
	uv pip install --offline --quiet --python "$HOME_DIR/venv/bin/python" "$HOME_DIR/$wheel"
	[ "$("$HOME_DIR/venv/bin/cmake" --version | sed -n '1s/^cmake version //p')" = "$DUCKDB_OSX_ARM64_CMAKE_VERSION" ] || {
		echo 'setup: the project-local CMake version differs from its pin' >&2
		exit 1
	}
fi
(cd "$HOME_DIR" && "$HOME_DIR/venv/bin/python" "$HERE/../vendor/configure_helper.py" -o "$HOME_DIR" -p)
[ "$(cat "$HOME_DIR/platform.txt")" = "$platform" ] || { echo 'setup: stock DuckDB reports another platform' >&2; exit 1; }
echo "setup: ready under $HOME_DIR"
