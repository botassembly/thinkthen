#!/bin/sh
# Fetch public source inputs and prepare matching Python hosts explicitly.
set -eu
HERE=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
INPUTS_HERE=$HERE
heavy_scripts=$HERE/../../../sdlc/scripts
. "$heavy_scripts/heavy-compile"
. "$HERE/inputs.sh"
DUCKDB_VERSIONS=$(input_versions)
host_target=$(input_target)
input_select "${THINKTHEN_DUCKDB_VERSION:-}" "$host_target"
case ${1:-} in
--target) echo "$target"; exit 0 ;;
--inputs) printf '%s %s %s %s %s\n' "$target" "$platform" "$duckdb_source_commit" "$duckdb_tools_commit" "$duckdb_python_commit"; exit 0 ;;
--fetch|'') ;;
*) echo 'usage: setup.sh [--fetch|--target|--inputs]' >&2; exit 2 ;;
esac
cache_root=${THINKTHEN_TOOLCHAINS:-$HOME/.cache/thinkthen-toolchains}
lock=$cache_root/.duckdb-cache-mutation.lock
if [ "${THINKTHEN_DUCKDB_CACHE_LOCK_HELD:-}" != "$lock" ]; then
	exec python3 "$HERE/cache_lock.py" "$lock" /bin/sh "$0" "$@"
fi
if [ "${1:-}" = --fetch ]; then python3 "$HERE/source_hosts.py" --fetch; fi
for version in $DUCKDB_VERSIONS; do
	input_select "$version" "$host_target"
	[ -d "$TOOLS/source/.git" ] && [ -d "$TOOLS/tools/.git" ] || {
		echo "not run: DuckDB $version source inputs are missing; run tools/setup.sh --fetch" >&2; exit 77;
	}
	python3 "$HERE/validate_inputs.py" --source "$TOOLS/source" --commit "$duckdb_source_commit"
	if [ "${1:-}" = --fetch ]; then
		[ -x "$TOOLS/venv/bin/python" ] || python3 -m venv "$TOOLS/venv"
		"$TOOLS/venv/bin/python" -m pip install -r "$HERE/requirements.txt"
		case $version in
		v1.5.6) "$TOOLS/venv/bin/python" -m pip install duckdb==1.5.6 ;;
		*)
			python3 - "$TOOLS/python" "$duckdb_python_commit" "$HERE" <<'PY'
import sys
from pathlib import Path
sys.path.insert(0, sys.argv[3])
import subprocess
folder = Path(sys.argv[1])
folder.mkdir(parents=True, exist_ok=True)
if not (folder / '.git').exists():
    subprocess.run(['git', 'init', '--quiet', str(folder)], check=True)
for command in (['fetch', '--no-tags', '--depth', '1', 'https://github.com/duckdb/duckdb-python.git', sys.argv[2]],
                ['checkout', '--quiet', '--detach', sys.argv[2]],
                ['diff', '--exit-code', sys.argv[2], '--', '.', ':!external/duckdb']):
    subprocess.run(['git', '-C', str(folder)] + command, check=True)
# The Python backend also reads its core submodule for package metadata.
# Its compiler and metadata must use the same approved core checkout.
core = folder / 'external/duckdb'
if not core.is_symlink():
    core.rmdir()
    core.symlink_to(folder.parent / 'source', target_is_directory=True)
if core.resolve() != (folder.parent / 'source').resolve():
    raise SystemExit('setup: the Python core differs from its approved source')
subprocess.run(['git', '-C', str(folder), 'submodule', 'init', 'external/duckdb'], check=True)
PY
			"$TOOLS/venv/bin/python" -m pip install 'scikit-build-core==0.11.6' 'nanobind==3.0.1' 'cmake==4.0.2' 'ninja==1.13.0'
			heavy_compile env CMAKE_ARGS="-DDUCKDB_SOURCE_PATH=$TOOLS/source -DBUILD_EXTENSIONS=core_functions -DNATIVE_ARCH=OFF" \
			CMAKE_BUILD_PARALLEL_LEVEL=2 RUSTC_WRAPPER= \
			CXXFLAGS="${CXXFLAGS:-} -ffile-prefix-map=$HOME=/build" \
			"$TOOLS/venv/bin/python" -m pip install --no-build-isolation --no-deps "$TOOLS/python"
		;; esac
	fi
	[ -x "$TOOLS/venv/bin/python" ] || { echo "not run: DuckDB $version Python host is missing; run tools/setup.sh --fetch" >&2; exit 77; }
	observed=$("$TOOLS/venv/bin/python" -c 'import duckdb; print(duckdb.sql("PRAGMA version").fetchone()[1])')
	[ "$observed" = "$(printf '%.10s' "$duckdb_source_commit")" ] || {
		echo 'setup: the Python DuckDB source differs from its pin' >&2; exit 1;
	}
	echo "setup: ready under $TOOLS"
done
