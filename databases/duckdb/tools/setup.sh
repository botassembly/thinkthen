#!/bin/sh
# Preparation (--fetch) uses public dependencies; validation stays offline.
set -eu
HERE=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
INPUTS_HERE=$HERE
. "$HERE/inputs.sh"
DUCKDB_VERSIONS=$(input_versions)
host_target=$(input_target)
input_select "${THINKTHEN_DUCKDB_VERSION:-}" "$host_target"
case ${1:-} in
--target) echo "$target"; exit 0 ;;
--inputs) printf '%s %s %s %s %s %s %s\n' "$target" "$platform" "$cli_asset" "$cli_zip_hash" "$static_asset" "$static_hash" "$manifest"; exit 0 ;;
--fetch|'') ;;
*) echo 'usage: setup.sh [--fetch|--target|--inputs]' >&2; exit 2 ;;
esac
cache_root=${THINKTHEN_TOOLCHAINS:-$HOME/.cache/thinkthen-toolchains}
lock=$cache_root/.duckdb-cache-mutation.lock
if [ "${THINKTHEN_DUCKDB_CACHE_LOCK_HELD:-}" != "$lock" ]; then
	exec python3 "$HERE/cache_lock.py" "$lock" /bin/sh "$0" "$@"
fi
. "$HERE/../../../sdlc/scripts/fetch.sh"
. "$HERE/../../../sdlc/scripts/scratch.sh"
. "$HERE/prepare_inputs.sh"
for version in $DUCKDB_VERSIONS; do
	input_select "$version" "$host_target"
	if [ "${1:-}" = --fetch ]; then
		input_prepare_source_static
		zip=$TOOLS/$cli_asset
		[ -e "$zip" ] || fetch_url "$zip" "https://github.com/duckdb/duckdb/releases/download/$DUCKDB_VERSION/$cli_asset"
		input_verify "$cli_zip_hash" "$zip"
		if [ ! -e "$TOOLS/duckdb" ]; then
			scratch_dir extracted_cli
			unzip -q "$zip" duckdb -d "$extracted_cli"
			input_verify "$cli_hash" "$extracted_cli/duckdb"
			cp "$extracted_cli/duckdb" "$TOOLS/duckdb"
		fi
	fi
	[ -x "$TOOLS/duckdb" ] && [ -d "$TOOLS/source" ] && [ -f "$TOOLS/static-libs/libduckdb_static.a" ] || {
		echo "setup: the pinned $DUCKDB_VERSION inputs are missing; run tools/setup.sh --fetch" >&2; exit 77;
	}
	input_verify "$cli_hash" "$TOOLS/duckdb"
	python3 "$HERE/validate_inputs.py" --source "$TOOLS/source" --commit "$duckdb_source_commit" --static "$TOOLS/static-libs" --manifest "$HERE/../cpp/$manifest"
	python3 "$HERE/source_checks.py" --requirements "$HERE/$requirements" >/dev/null
	if [ ! -x "$TOOLS/venv/bin/python" ]; then
		uv venv --offline --python 3.13 "$TOOLS/venv" || { echo 'setup: cached Python 3.13 is missing' >&2; exit 77; }
	fi
	if [ "${1:-}" = --fetch ]; then
		uv pip install --quiet --python "$TOOLS/venv/bin/python" -r "$HERE/$requirements"
	else
		uv pip install --offline --quiet --python "$TOOLS/venv/bin/python" -r "$HERE/$requirements" || { echo "setup: cached $DUCKDB_VERSION Python requirements are missing" >&2; exit 77; }
	fi
	case $target in *-apple-darwin)
		# The tool pin stays separate from the supported-version list.
		cmake_row=$(python3 - "$HERE/version.env" <<'PY'
import sys
from pathlib import Path
sys.path.insert(0, str(Path(sys.argv[1]).parent))
from inputs import authority
pins = authority()
print(' '.join(pins['DUCKDB_OSX_ARM64_CMAKE_' + k] for k in ('VERSION', 'WHEEL_SHA256', 'WHEEL_URL')))
PY
)
		read -r cmake_version cmake_hash cmake_url <<EOF
$cmake_row
EOF
		wheel=${cmake_url##*/}
		if [ "${1:-}" = --fetch ] && [ ! -e "$TOOLS/$wheel" ]; then fetch_url "$TOOLS/$wheel" "$cmake_url"; fi
		[ -f "$TOOLS/$wheel" ] || { echo 'setup: pinned CMake wheel is missing' >&2; exit 77; }
		input_verify "$cmake_hash" "$TOOLS/$wheel"
		uv pip install --offline --quiet --python "$TOOLS/venv/bin/python" "$TOOLS/$wheel"
		[ "$("$TOOLS/venv/bin/cmake" --version | sed -n '1s/^cmake version //p')" = "$cmake_version" ] || {
			echo 'setup: the project-local CMake version differs from its pin' >&2; exit 1;
		}
	;; esac
	observed=$("$TOOLS/venv/bin/python" -c 'import duckdb; print("v" + duckdb.__version__)')
	[ "$observed" = "$DUCKDB_VERSION" ] || { echo 'setup: the Python DuckDB version differs from its pin' >&2; exit 1; }
	(cd "$TOOLS" && "$TOOLS/venv/bin/python" "$HERE/../vendor/configure_helper.py" -o "$TOOLS" -p)
	[ "$(cat "$TOOLS/platform.txt")" = "$platform" ] || { echo 'setup: stock DuckDB reports another platform' >&2; exit 1; }
	echo "setup: ready under $TOOLS"
done
