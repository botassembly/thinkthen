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
if [ -n "${THINKTHEN_DUCKDB_VERSION:-}" ]; then DUCKDB_VERSIONS=$DUCKDB_VERSION; fi
case ${1:-} in
--target) echo "$target"; exit 0 ;;
--inputs) printf '%s %s %s %s %s\n' "$target" "$platform" "$duckdb_source_commit" "$duckdb_tools_commit" "$duckdb_python_commit"; exit 0 ;;
--fetch|'') ;;
*) echo 'usage: setup.sh [--fetch [WHEEL_ROOT]|--target|--inputs]' >&2; exit 2 ;;
esac
# A supplied wheel root comes from this release run's build output. Never build in this path.
wheel_root=${2:-}
cache_root=${THINKTHEN_TOOLCHAINS:-$HOME/.cache/thinkthen-toolchains}
lock=$cache_root/.duckdb-cache-mutation.lock
if [ "${THINKTHEN_DUCKDB_CACHE_LOCK_HELD:-}" != "$lock" ]; then
	exec python3 "$HERE/cache_lock.py" "$lock" /bin/sh "$0" "$@"
fi
if [ "${1:-}" = --fetch ]; then
	python3 "$HERE/source_hosts.py" --fetch
	python3 "$HERE/../community/prepare.py"
fi
for version in $DUCKDB_VERSIONS; do
	input_select "$version" "$host_target"
	[ -d "$TOOLS/source/.git" ] && [ -d "$TOOLS/tools/.git" ] || {
		echo "not run: DuckDB $version source inputs are missing; run tools/setup.sh --fetch" >&2; exit 77;
	}
	python3 "$HERE/validate_inputs.py" --source "$TOOLS/source" --commit "$duckdb_source_commit"
	if [ "${1:-}" = --fetch ]; then
		[ -x "$TOOLS/venv/bin/python" ] || python3.13 -m venv "$TOOLS/venv"
		"$TOOLS/venv/bin/python" -m pip install -r "$HERE/requirements.txt"
		if [ -n "$wheel_root" ]; then
			[ -d "$wheel_root/$version" ] || { echo "setup: missing built DuckDB wheel for $version" >&2; exit 1; }
			"$TOOLS/venv/bin/python" -m pip install --no-index --no-deps "$wheel_root/$version/"*.whl
		else
		mkdir -p "$TOOLS/wheels"
		case $version in
		v1.5.6) "$TOOLS/venv/bin/python" -m pip download --only-binary=:all: --no-deps --dest "$TOOLS/wheels" duckdb==1.5.6 ;;
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
                ['checkout', '--quiet', '--detach', sys.argv[2]]):
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
			python3 "$HERE/validate_inputs.py" --source "$TOOLS/source" --commit "$duckdb_source_commit" \
				--python "$TOOLS/python" --python-commit "$duckdb_python_commit"
			python3 - "$TOOLS" <<'PY'
import shutil
import sys
from pathlib import Path
folder = Path(sys.argv[1])
staging = folder / 'python-build'
if staging.exists():
    shutil.rmtree(staging)
shutil.copytree(folder / 'python', staging / 'source', symlinks=True)
PY
			python3 "$HERE/validate_inputs.py" --source "$TOOLS/source" --commit "$duckdb_source_commit" \
				--python "$TOOLS/python-build/source" --python-commit "$duckdb_python_commit"
			"$TOOLS/venv/bin/python" -m pip install 'scikit-build-core==0.11.6' 'nanobind==3.0.1'
			heavy_compile env CMAKE_ARGS="-DDUCKDB_SOURCE_PATH=$TOOLS/source -DBUILD_EXTENSIONS=core_functions;json -DNATIVE_ARCH=OFF" \
			CMAKE_BUILD_PARALLEL_LEVEL=2 RUSTC_WRAPPER= PYTHONDONTWRITEBYTECODE=1 \
			SKBUILD_BUILD_DIR="$TOOLS/python-build/cmake" \
			CXXFLAGS="${CXXFLAGS:-} -ffile-prefix-map=$HOME=/build" \
			"$TOOLS/venv/bin/python" -m pip wheel --no-build-isolation --no-deps --wheel-dir "$TOOLS/wheels" "$TOOLS/python-build/source"
		;; esac
		"$TOOLS/venv/bin/python" -m pip install --no-index --no-deps "$TOOLS/wheels/"*.whl
		fi
	fi
	[ -x "$TOOLS/venv/bin/python" ] || { echo "not run: DuckDB $version Python host is missing; run tools/setup.sh --fetch" >&2; exit 77; }
	observed=$(python3 - "$TOOLS/venv/bin/python" "$HERE" <<'PY'
import subprocess
import sys
import tempfile
from pathlib import Path
sys.path.insert(0, sys.argv[2])
from harness import child_env
code = "import duckdb; con=duckdb.connect(config={'autoinstall_known_extensions':'false'}); "
code += "assert con.execute(\"SELECT CAST('{}' AS JSON)::VARCHAR\").fetchone()[0] == '{}'; "
code += "print(con.execute('PRAGMA version').fetchone()[1])"
with tempfile.TemporaryDirectory(prefix='thinkthen-duckdb-python-host-') as folder:
    print(subprocess.check_output([sys.argv[1], '-c', code], text=True,
                                  env=child_env('http://127.0.0.1:9/v1', Path(folder), keyless=True)).strip())
PY
)
	[ "$observed" = "$(printf '%.10s' "$duckdb_source_commit")" ] || {
		echo 'setup: the Python DuckDB source differs from its pin' >&2; exit 1;
	}
	echo "setup: ready under $TOOLS"
done
