#!/bin/sh
# The Python surface's gate. The surface rung passes its loopback port as $1.
# A missing Python 3.12 or later, uv, maturin, or pinned package exits 77,
# which the rung reports as not run and never as a pass.
unset THINKTHEN_API_KEY
set -eu
cd -- "$(dirname -- "$0")"
here=$(pwd)
repo=$(cd ../.. && pwd)
. "$repo/sdlc/scripts/scratch.sh"
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
usage_home
port=${1:?usage: check.sh PORT}
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress|smoke) ;; *) echo "unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS

not_run() {
	echo "not run: libraries/python: $1"
	exit 77
}

command -v uv >/dev/null 2>&1 || not_run "no uv"
. ./host.sh
host=$(python_host)
[ -n "$host" ] || not_run "no stable Python 3.12 or later (the test pins need it)"
host_identity=$("$host" -c 'import os, sys; print(os.path.realpath(sys._base_executable), sys.version.split()[0])')
command -v maturin >/dev/null 2>&1 || not_run "no maturin"
if [ "$profile" = smoke ]; then
	smoke_guard
	# The replay smoke (ticket 0335): the wheel in a fresh venv, imported from outside the checkout.
	scratch_dir scratch
	PYO3_PYTHON=$host maturin build --quiet --locked --offline -o "$scratch/wheel"
	uv venv --quiet --offline --python "$host" "$scratch/venv"
	uv pip install --quiet --offline --no-deps --python "$scratch/venv/bin/python" "$scratch"/wheel/thinkthen-*.whl
	cd "$scratch"
	THINKTHEN_API_KEY=sk-smoke-loopback "$scratch/venv/bin/python" -c 'import os, sys, thinkthen
assert thinkthen.__file__.startswith(sys.argv[1]), thinkthen.__file__
value = thinkthen.decide(os.environ["THINKTHEN_TEST_SMOKE_QUESTION"], os.environ["THINKTHEN_TEST_SMOKE_TEXT"]).value
print("smoke:", {True: "true", False: "false", None: "null"}[value])' "$scratch/venv/"
	exit
fi

# One venv per checkout and pin file, outside the product cache and the
# repository, keyed by the checkout and, past the first, the pin file's name.
pinned() {
	venv=${XDG_CACHE_HOME:-$HOME/.cache}/thinkthen-toolchains/python/$(printf '%s' "$here$2$host_identity" |
		sha256sum | cut -c1-16)
	if [ -x "$venv/bin/python" ]; then
		venv_identity=$("$venv/bin/python" -c 'import os, sys; print(os.path.realpath(sys._base_executable), sys.version.split()[0])')
		[ "$venv_identity" = "$host_identity" ] || not_run "the cached venv uses another Python interpreter"
		return 0
	fi
	if ! { uv venv --quiet --offline --python "$host" "$venv" &&
		uv pip install --quiet --offline --require-hashes --python "$venv/bin/python" -r "$1"; }; then
		rm -rf -- "$venv"
		not_run "uv's cache lacks a pin of $1; on a networked machine run \`uv pip install --require-hashes --python $host -r libraries/python/$1\` into any venv once"
	fi
}
# Each lane's one fact: whether its pandas exports the Arrow stream, and that
# the extension it imports is the one this checkout built (ticket 0122).
precondition() {
	"$1" -c "import os, pandas as pd, thinkthen._thinkthen as ext
assert hasattr(pd.Series(['a'], dtype='str'), '__arrow_c_stream__') is $2, 'pandas ' + pd.__version__
assert os.path.dirname(ext.__file__) == os.path.abspath('thinkthen'), ext.__file__" ||
		{ echo "libraries/python: the $3 lane's precondition failed" >&2; exit 1; }
}
pinned requirements-dev.txt ""
python=$venv/bin/python
(cd "$repo" && cargo build --quiet --locked --offline --package conformance-backend)

if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
	# The installed-file mode (ticket 0128): the wheel in a fresh venv, and the shared cases and
	# examples from a copy of tests/, with no repository copy on the import path.
	. "$repo/sdlc/scripts/installed.sh"
	installed_tests "$repo" libraries/python
	uv venv --quiet --offline --python "$host" "$scratch/venv"
	uv pip install --quiet --offline --require-hashes --python "$scratch/venv/bin/python" -r requirements-dev.txt
	uv pip install --quiet --offline --no-deps --python "$scratch/venv/bin/python" "$THINKTHEN_ARTIFACT"
	cd "$scratch/libraries/python"
	unset PYTHONPATH
	"$scratch/venv/bin/python" -c 'import sys, thinkthen; sys.exit(not thinkthen.__file__.startswith(sys.argv[1]))' "$scratch/venv/" ||
		{ echo "libraries/python: thinkthen loaded from outside the fresh venv" >&2; exit 1; }
	"$scratch/venv/bin/python" -c 'import importlib.util, importlib.metadata
assert importlib.util.find_spec("thinkthen._labels")
assert importlib.util.find_spec("thinkthen.pydantic")
assert importlib.util.find_spec("thinkthen.judge")
assert importlib.util.find_spec("thinkthen.stream")
assert any(need.startswith("pydantic") and "2.11" in need and "<3" in need
           for need in importlib.metadata.requires("thinkthen"))' ||
		{ echo "libraries/python: the installed wheel lacks its label modules or optional extra" >&2; exit 1; }
	"$scratch/venv/bin/python" -m mypy --strict tests/type_contract.py
	THINKTHEN_API_KEY=sk-fake-loopback-python-0105 THINKTHEN_BASE_URL="http://127.0.0.1:$port/generic/v1" \
		THINKTHEN_CACHE="$scratch/cache" "$scratch/venv/bin/python" tests/conformance.py "$port"
	THINKTHEN_API_KEY=sk-fake-loopback-python-0105 "$scratch/venv/bin/python" tests/examples.py "$port"
	exit 0
fi

# Cargo embeds source paths in panic locations. The remap keeps the builder's
# home out of every built extension, as in build-wheel.sh.
RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"
PYO3_PYTHON=$python
export RUSTFLAGS PYO3_PYTHON

echo "== every cargo and maturin call is locked and offline (R4-19)"
awk '!/^[[:space:]]*(#|echo )/ && /(^|[[:space:](])(carg[o]|maturi[n]) [a-z]/ && !/carg[o] fmt/ \
	&& !(/--locked/ && /--offline/) { print FILENAME ": " $0; bad = 1 } END { exit bad }' \
	check.sh build-wheel.sh

echo "== panics are caught only through thinkthen::contained (R2-31)"
sites=$(grep -o 'catch_unwind(' src/*.rs | wc -l)
[ "$sites" -eq 0 ] || { echo "found $sites catch_unwind sites in src, expected none" >&2; exit 1; }

echo "== format, lints, and the Rust unit tests with libpython linked"
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
cargo clippy --locked --offline --all-targets --features probe -- -D warnings
libdir=$("$python" -c 'import sysconfig; print(sysconfig.get_config_var("LIBDIR"))')
home=$("$python" -c 'import sys; print(sys.base_prefix)')
LD_LIBRARY_PATH="$libdir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" PYTHONHOME=$home \
	cargo test --quiet --no-default-features --lib --locked --offline

echo "== the extension, with the test-only probe feature"
VIRTUAL_ENV=$venv maturin develop --quiet --locked --offline --features probe

echo "== the Python tests, each engine call in a child on its own backend: the door,"
echo "   Arrow safety, bounded exit/release, throttle, and the address proof"
"$python" -m mypy --strict tests/type_contract.py
precondition "$python" True "pandas 3"
if [ "$profile" = stress ]; then
	"$python" -m pytest -q -p no:cacheprovider --tb=short -m stress \
		tests/test_release.py tests/test_column_timing.py tests/test_pandas.py tests/test_arrow_safety.py
else
	"$python" -m pytest -q -p no:cacheprovider --tb=short -m 'not stress' tests/
fi

if [ "$profile" = stress ]; then
	echo "== the pandas 2 stress lane"
	pinned requirements-pandas2.txt requirements-pandas2.txt
	precondition "$venv/bin/python" False "pandas 2"
	"$venv/bin/python" -m pytest -q -p no:cacheprovider --tb=short -m stress tests/test_pandas.py
	exit 0
fi

echo "== the shared cases and the examples, on the rung's backend"
scratch_dir scratch
THINKTHEN_API_KEY=sk-fake-loopback-python-0105 \
	THINKTHEN_BASE_URL="http://127.0.0.1:$port/generic/v1" THINKTHEN_CACHE="$scratch/cache" \
	"$python" tests/conformance.py "$port"
THINKTHEN_API_KEY=sk-fake-loopback-python-0105 "$python" tests/examples.py "$port"

echo "== the release wheel and its contents"
sh build-wheel.sh

echo "== the pandas 2 lane: the same extension, the pandas tests, and the secrecy test"
pinned requirements-pandas2.txt requirements-pandas2.txt
precondition "$venv/bin/python" False "pandas 2"
"$venv/bin/python" -m pytest -q -p no:cacheprovider --tb=short -m 'not stress' tests/test_pandas.py tests/test_secrecy.py
echo "pandas 2 lane passed on pandas $("$venv/bin/python" -c 'import pandas; print(pandas.__version__)')"
