#!/bin/sh
# The DuckDB surface (tickets 0110 and 0118). The surfaces rung passes its
# loopback port as $1. Each suite also starts its own backend, so counts
# never mix. A missing toolchain reports "not run" (exit 77), never "pass".
set -eu
HERE=$(cd -- "$(dirname -- "$0")" && pwd)
cd -- "$HERE"
# macOS has no `timeout` (ticket 0128).
LIMIT=$HERE/../../sdlc/scripts/time-limit
. "$HERE/tools/version.env"
TOOLS=${THINKTHEN_TOOLCHAINS:-$HOME/.cache/thinkthen-toolchains}/duckdb/$DUCKDB_VERSION
CLI=${THINKTHEN_DUCKDB_CLI:-$TOOLS/duckdb}
PY=$TOOLS/venv/bin/python
if [ "$(uname -s)" != Linux ]; then
	echo "not run: databases/duckdb checks on Linux only"
	exit 77
fi
if [ ! -x "$CLI" ] || [ ! -x "$PY" ] || [ ! -f "$TOOLS/platform.txt" ]; then
	echo "not run: the DuckDB $DUCKDB_VERSION toolchain is missing; run databases/duckdb/tools/setup.sh --fetch"
	exit 77
fi
if [ -n "${CHECK_SETUP_ONLY:-}" ]; then
	echo "setup ok"
	exit 0
fi
PORT=${1:?check.sh takes the loopback port}
# Shared rule 6: no suite sees a real key or a remote address.
unset THINKTHEN_API_KEY THINKTHEN_BASE_URL THINKTHEN_CACHE
REPO=$(cd -- ../.. && pwd)
export THINKTHEN_BACKEND_BIN="${CARGO_TARGET_DIR:-$REPO/target}/debug/conformance-backend"
[ -x "$THINKTHEN_BACKEND_BIN" ] || {
	echo "check: the loopback backend is not built; run cargo build --package conformance-backend at the repository root" >&2
	exit 1
}
export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"

echo "== build, lint, and unit tests"
cargo fmt --check
cargo clippy --locked --offline --all-targets --all-features -- -D warnings
cargo test --locked --offline --lib
cargo build --locked --offline --release
cargo build --locked --offline --release --features test-hooks --target-dir target/hooks
package() {
	mkdir -p "$(dirname -- "$2")"
	printf '%s' "$(sed -n 's/^version = "\(.*\)"/\1/p' Cargo.toml | head -n 1)" >build/extension_version.txt
	"$PY" vendor/append_extension_metadata.py -l "$1" -o "$2" -n thinkthen -dv "$DUCKDB_VERSION" \
		-evf build/extension_version.txt -pf "$TOOLS/platform.txt" --abi-type C_STRUCT_UNSTABLE >/dev/null
}
mkdir -p build
package target/release/libthinkthen_duckdb.so build/thinkthen.duckdb_extension
package target/hooks/release/libthinkthen_duckdb.so build/hooks/thinkthen.duckdb_extension

echo "== source checks and deny"
python3 tools/source_checks.py
cargo deny --locked --offline --manifest-path Cargo.toml check --config deny.toml advisories bans licenses sources
planted=$(mktemp)
trap 'rm -f -- "$planted"' EXIT
sed 's/^exceptions = \[{ crate = "zlib-rs".*/exceptions = []/' deny.toml >"$planted"
set +e
cargo deny --locked --offline --manifest-path Cargo.toml check --config "$planted" licenses >"$planted.out" 2>&1
code=$?
set -e
[ "$code" -ne 0 ] && grep -q '├ zlib-rs v' "$planted.out" || {
	echo "check: deny passed with the zlib-rs exception removed (exit $code)" >&2
	exit 1
}
rm -f -- "$planted.out"

echo "== the stock CLI loads the extension"
answer=$(env -i PATH="$PATH" HOME="$(mktemp -d)" XDG_CACHE_HOME="$(mktemp -d)" XDG_CONFIG_HOME="$(mktemp -d)" \
	THINKTHEN_API_KEY=sk-loopback-duckdb-check THINKTHEN_BASE_URL="http://127.0.0.1:$PORT/generic/v1" \
	sh "$LIMIT" 60 "$CLI" -unsigned -noheader -list -c "LOAD 'build/thinkthen.duckdb_extension'; SELECT thinkthen_decide('Is it a refund?', 'refund now');")
[ "$answer" = true ] || {
	echo "check: the stock CLI read '$answer', not true" >&2
	exit 1
}

echo "== suites"
for suite in verbs_suite settings_suite signal_suite relate_suite databases_suite conformance; do
	sh "$LIMIT" 900 "$PY" "tools/$suite.py"
done
THINKTHEN_DUCKDB_CLI_PATH="$CLI" sh "$LIMIT" 900 "$PY" tools/site_examples.py
sh tools/selftests.sh "$PY"
echo "check: databases/duckdb passes"
