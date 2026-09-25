#!/bin/sh
# The Python surface's gate. The surface rung passes its loopback port as $1.
# A missing Python 3.12 or later, uv, maturin, or pinned package exits 77,
# which the rung reports as not run and never as a pass.
unset THINKTHEN_API_KEY
set -eu
cd -- "$(dirname -- "$0")"
here=$(pwd)
repo=$(cd ../.. && pwd)
port=${1:?usage: check.sh PORT}

not_run() {
	echo "not run: libraries/python: $1"
	exit 77
}

host=
for candidate in python3.14 python3.13 python3.12 python3; do
	if command -v "$candidate" >/dev/null 2>&1 &&
		"$candidate" -c 'import sys; sys.exit(sys.version_info < (3, 12))' 2>/dev/null; then
		host=$(command -v "$candidate")
		break
	fi
done
[ -n "$host" ] || not_run "no Python 3.12 or later (the test pins need it)"
command -v uv >/dev/null 2>&1 || not_run "no uv"
command -v maturin >/dev/null 2>&1 || not_run "no maturin"

# One venv per checkout and pin file, outside the product cache and the
# repository, keyed by the checkout and, past the first, the pin file's name.
pinned() {
	venv=${XDG_CACHE_HOME:-$HOME/.cache}/thinkthen-toolchains/python/$(printf '%s' "$here$2" |
		sha256sum | cut -c1-16)
	[ -x "$venv/bin/python" ] && return 0
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

# Cargo embeds source paths in panic locations. The remap keeps the builder's
# home out of every built extension, as in build-wheel.sh.
RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"
PYO3_PYTHON=$python
export RUSTFLAGS PYO3_PYTHON

echo "== every cargo and maturin call is locked and offline (R4-19)"
awk '!/^[[:space:]]*(#|echo )/ && /(^|[[:space:](])(carg[o]|maturi[n]) [a-z]/ && !/carg[o] fmt/ \
	&& !(/--locked/ && /--offline/) { print FILENAME ": " $0; bad = 1 } END { exit bad }' \
	check.sh build-wheel.sh

echo "== one catch_unwind site (R2-31)"
sites=$(grep -o 'catch_unwind(' src/*.rs | wc -l)
[ "$sites" -eq 1 ] || { echo "found $sites catch_unwind sites in src, expected 1" >&2; exit 1; }

echo "== format, lints, and the Rust unit tests with libpython linked"
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
cargo clippy --locked --offline --all-targets --features probe -- -D warnings
libdir=$("$python" -c 'import sysconfig; print(sysconfig.get_config_var("LIBDIR"))')
home=$("$python" -c 'import sys; print(sys.base_prefix)')
LD_LIBRARY_PATH="$libdir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" PYTHONHOME=$home \
	cargo test --quiet --no-default-features --lib --locked --offline

echo "== every test NOTES.md names for a refused shape exists (R7-8)"
cited=$(sed -n 's/.*proved by `\([a-z0-9_]*\)`.*/\1/p' NOTES.md)
[ -n "$cited" ] || { echo "NOTES.md names no proving test" >&2; exit 1; }
for name in $cited; do
	grep -rqE "(fn|def) $name\(" src tests || { echo "NOTES.md names a missing test: $name" >&2; exit 1; }
done

echo "== the extension, with the test-only probe feature"
VIRTUAL_ENV=$venv maturin develop --quiet --locked --offline --features probe

echo "== the Python tests, each engine call in a child on its own backend: the door,"
echo "   the Arrow safety suite, the exit freeze, throttle equality, and the address proof"
precondition "$python" True "pandas 3"
"$python" -m pytest -q -p no:cacheprovider --tb=short tests/

echo "== the shared cases and the examples, on the rung's backend"
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
THINKTHEN_API_KEY=sk-fake-loopback-python-0105 \
	THINKTHEN_BASE_URL="http://127.0.0.1:$port/generic/v1" THINKTHEN_CACHE="$scratch/cache" \
	"$python" tests/conformance.py "$port"
THINKTHEN_API_KEY=sk-fake-loopback-python-0105 "$python" tests/examples.py "$port"

echo "== the release wheel and its contents"
sh build-wheel.sh

echo "== the pandas 2 lane: the same extension, the pandas tests, and the secrecy test"
pinned requirements-pandas2.txt requirements-pandas2.txt
precondition "$venv/bin/python" False "pandas 2"
"$venv/bin/python" -m pytest -q -p no:cacheprovider --tb=short tests/test_pandas.py tests/test_secrecy.py
echo "pandas 2 lane passed on pandas $("$venv/bin/python" -c 'import pandas; print(pandas.__version__)')"
