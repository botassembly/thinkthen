#!/bin/sh
# One-time setup under ~/.cache/thinkthen-toolchains/duckdb/<version>/
# (shared rule 2). With no argument it works offline: a venv from uv's
# CPython 3.13 and tools/requirements.txt, and platform.txt. `--fetch` is
# the one networked step: it downloads the stock CLI and refuses a binary
# whose sha256 differs from tools/version.env.
set -eu
HERE=$(cd -- "$(dirname -- "$0")" && pwd)
. "$HERE/version.env"
HOME_DIR=${THINKTHEN_TOOLCHAINS:-$HOME/.cache/thinkthen-toolchains}/duckdb/$DUCKDB_VERSION
mkdir -p "$HOME_DIR"
case $(uname -s):$(uname -m) in
Linux:x86_64) ;;
*) echo "setup: the pinned DuckDB C++ inputs target Linux x86_64" >&2; exit 77 ;;
esac

if [ "${1:-}" = --fetch ]; then
	zip="$HOME_DIR/duckdb_cli.zip"
	curl -fsSL -o "$zip" "https://github.com/duckdb/duckdb/releases/download/$DUCKDB_VERSION/duckdb_cli-linux-amd64.zip"
	unzip -o -q "$zip" duckdb -d "$HOME_DIR"
	rm -f -- "$zip"
	if [ ! -d "$HOME_DIR/source" ]; then
		git clone --quiet --depth 1 --branch "$DUCKDB_VERSION" https://github.com/duckdb/duckdb.git "$HOME_DIR/source"
	fi
	static_zip="$HOME_DIR/static-libs-linux-amd64.zip"
	if [ ! -f "$static_zip" ]; then
		curl -fsSL -o "$static_zip" "https://github.com/duckdb/duckdb/releases/download/$DUCKDB_VERSION/static-libs-linux-amd64.zip"
	fi
	older_zip="$HOME_DIR/duckdb_cli-$DUCKDB_OLDER_VERSION.zip"
	if [ ! -f "$older_zip" ]; then
		curl -fsSL -o "$older_zip" "https://github.com/duckdb/duckdb/releases/download/$DUCKDB_OLDER_VERSION/duckdb_cli-linux-amd64.zip"
	fi
	echo "$DUCKDB_OLDER_CLI_ZIP_SHA256  $older_zip" | sha256sum -c --quiet - || {
		echo "setup: $older_zip differs from the pinned release" >&2
		exit 1
	}
	mkdir -p "$HOME_DIR/older-host"
	unzip -o -q "$older_zip" duckdb -d "$HOME_DIR/older-host"
	echo "$DUCKDB_STATIC_ZIP_SHA256  $static_zip" | sha256sum -c --quiet - || {
		echo "setup: $static_zip differs from the pinned release" >&2
		exit 1
	}
	mkdir -p "$HOME_DIR/static-libs"
	unzip -o -q "$static_zip" -d "$HOME_DIR/static-libs"
fi
if [ -f "$HOME_DIR/duckdb" ]; then
	echo "$DUCKDB_CLI_SHA256  $HOME_DIR/duckdb" | sha256sum -c --quiet - || {
		echo "setup: $HOME_DIR/duckdb does not match the pinned sha256; remove it and run tools/setup.sh --fetch" >&2
		exit 1
	}
fi
if [ ! -x "$HOME_DIR/older-host/duckdb" ]; then
	echo "setup: the pinned $DUCKDB_OLDER_VERSION host is missing; run tools/setup.sh --fetch" >&2
	exit 77
fi
echo "$DUCKDB_OLDER_CLI_SHA256  $HOME_DIR/older-host/duckdb" | sha256sum -c --quiet - || {
	echo "setup: the $DUCKDB_OLDER_VERSION host differs from the pinned release" >&2
	exit 1
}
if [ ! -d "$HOME_DIR/source" ] || [ ! -f "$HOME_DIR/static-libs/libduckdb_static.a" ]; then
	echo "setup: DuckDB C++ source or static archives are missing; run tools/setup.sh --fetch" >&2
	exit 77
fi
[ "$(git -C "$HOME_DIR/source" rev-parse HEAD)" = "$DUCKDB_CPP_SOURCE_COMMIT" ] || {
	echo "setup: DuckDB C++ source differs from the pinned commit" >&2
	exit 1
}
(cd "$HOME_DIR/static-libs" && sha256sum -c --quiet "$HERE/../cpp/archive-sha256.txt") || {
	echo "setup: DuckDB static archives differ from the fixed manifest" >&2
	exit 1
}

python3 "$HERE/source_checks.py" --requirements "$HERE/requirements.txt" >/dev/null
if [ ! -x "$HOME_DIR/venv/bin/python" ]; then
	uv venv --offline --python 3.13 "$HOME_DIR/venv"
fi
uv pip install --offline --quiet --python "$HOME_DIR/venv/bin/python" -r "$HERE/requirements.txt"
(cd "$HOME_DIR" && "$HOME_DIR/venv/bin/python" "$HERE/../vendor/configure_helper.py" -o "$HOME_DIR" -p)
echo "setup: ready under $HOME_DIR"
