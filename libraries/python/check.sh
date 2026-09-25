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

# One venv per checkout, outside the product cache and the repository.
key=$(printf '%s' "$here" | sha256sum | cut -c1-16)
venv=${XDG_CACHE_HOME:-$HOME/.cache}/thinkthen-toolchains/python/$key
if [ ! -x "$venv/bin/python" ]; then
	if ! { uv venv --quiet --offline --python "$host" "$venv" &&
		uv pip install --quiet --offline --require-hashes --python "$venv/bin/python" \
			-r requirements-dev.txt; }; then
		rm -rf -- "$venv"
		not_run "uv's cache lacks a pinned package; on a networked machine run \`uv pip install --require-hashes --python $host -r libraries/python/requirements-dev.txt\` into any venv once"
	fi
fi
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
libdir=$("$python" -c 'import sysconfig; print(sysconfig.get_config_var("LIBDIR"))')
home=$("$python" -c 'import sys; print(sys.base_prefix)')
LD_LIBRARY_PATH="$libdir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" PYTHONHOME=$home \
	cargo test --quiet --no-default-features --lib --locked --offline

echo "== the extension, with the test-only probe feature"
VIRTUAL_ENV=$venv maturin develop --quiet --locked --offline --features probe

echo "== the Python tests, each engine call in a child on its own backend"
"$python" -m pytest -q -p no:cacheprovider tests/

echo "== the shared cases and the examples, on the rung's backend"
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
THINKTHEN_API_KEY=sk-fake-loopback-python-0105 \
	THINKTHEN_BASE_URL="http://127.0.0.1:$port/generic/v1" THINKTHEN_CACHE="$scratch/cache" \
	"$python" tests/conformance.py "$port"
THINKTHEN_API_KEY=sk-fake-loopback-python-0105 "$python" tests/examples.py "$port"

echo "== the release wheel and its contents"
sh build-wheel.sh
