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

if [ "${1:-}" = --fetch ]; then
	zip="$HOME_DIR/duckdb_cli.zip"
	curl -fsSL -o "$zip" "https://github.com/duckdb/duckdb/releases/download/$DUCKDB_VERSION/duckdb_cli-linux-amd64.zip"
	unzip -o -q "$zip" duckdb -d "$HOME_DIR"
	rm -f -- "$zip"
fi
if [ -f "$HOME_DIR/duckdb" ]; then
	echo "$DUCKDB_CLI_SHA256  $HOME_DIR/duckdb" | sha256sum -c --quiet - || {
		echo "setup: $HOME_DIR/duckdb does not match the pinned sha256; remove it and run tools/setup.sh --fetch" >&2
		exit 1
	}
fi

python3 "$HERE/source_checks.py" --requirements "$HERE/requirements.txt" >/dev/null
if [ ! -x "$HOME_DIR/venv/bin/python" ]; then
	uv venv --offline --python 3.13 "$HOME_DIR/venv"
fi
uv pip install --offline --quiet --python "$HOME_DIR/venv/bin/python" -r "$HERE/requirements.txt"
(cd "$HOME_DIR" && "$HOME_DIR/venv/bin/python" "$HERE/../vendor/configure_helper.py" -o "$HOME_DIR" -p)
echo "setup: ready under $HOME_DIR"
