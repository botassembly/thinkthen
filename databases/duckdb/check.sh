#!/bin/sh
# The DuckDB surface (tickets 0110, 0118, and 0201). The surfaces rung passes its
# loopback port as $1. Each suite also starts its own backend, so counts
# never mix. A missing toolchain reports "not run" (exit 77), never "pass".
set -eu
HERE=$(cd -- "$(dirname -- "$0")" && pwd)
cd -- "$HERE"
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress) ;; *) echo "duckdb: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS
# macOS has no `timeout` (ticket 0128).
LIMIT=$HERE/../../sdlc/scripts/time-limit
. "$HERE/tools/version.env"
TOOLS=${THINKTHEN_TOOLCHAINS:-$HOME/.cache/thinkthen-toolchains}/duckdb/$DUCKDB_VERSION
CLI=${THINKTHEN_DUCKDB_CLI:-$TOOLS/duckdb}
PY=$TOOLS/venv/bin/python
case $(uname -s):$(uname -m) in
Linux:x86_64 | Darwin:arm64) ;;
*) echo "not run: no pinned DuckDB C++ host for $(uname -s):$(uname -m)"; exit 77 ;;
esac
if [ ! -x "$CLI" ] || [ ! -x "$PY" ] || [ ! -f "$TOOLS/platform.txt" ]; then
	echo "not run: the DuckDB $DUCKDB_VERSION toolchain is missing; run databases/duckdb/tools/setup.sh --fetch"
	exit 77
fi
case $(uname -s):$(uname -m) in
Linux:x86_64) expected_platform=linux_amd64 ;;
Darwin:arm64) expected_platform=osx_arm64 ;;
esac
[ "$(cat "$TOOLS/platform.txt")" = "$expected_platform" ] || {
	echo "check: the stock DuckDB platform differs from $expected_platform" >&2
	exit 1
}
if [ -n "${CHECK_SETUP_ONLY:-}" ]; then
	echo "setup ok"
	exit 0
fi
PORT=${1:?check.sh takes the loopback port}
# Shared rule 6: no suite sees a real key or a remote address.
unset THINKTHEN_API_KEY THINKTHEN_BASE_URL THINKTHEN_CACHE
REPO=$(cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"
export THINKTHEN_BACKEND_BIN="${CARGO_TARGET_DIR:-$REPO/target}/debug/conformance-backend"
[ -x "$THINKTHEN_BACKEND_BIN" ] || {
	echo "check: the loopback backend is not built; run cargo build --package conformance-backend at the repository root" >&2
	exit 1
}
export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"
stock_cli() {
	echo "== the stock CLI loads the extension"
	scratch_dir home && scratch_dir cache && scratch_dir config
	answer=$(env -i PATH="$PATH" HOME="$home" XDG_CACHE_HOME="$cache" XDG_CONFIG_HOME="$config" \
		THINKTHEN_API_KEY=sk-loopback-duckdb-check THINKTHEN_BASE_URL="http://127.0.0.1:$PORT/generic/v1" \
		sh "$LIMIT" 60 "$CLI" -unsigned -noheader -list -c "LOAD '${THINKTHEN_DUCKDB_EXTENSION:-build/thinkthen.duckdb_extension}'; SELECT thinkthen_decide('Is it a refund?', 'refund now');")
	[ "$answer" = true ] || {
		echo "check: the stock CLI read '$answer', not true" >&2
		exit 1
	}
}
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
	# The installed-file mode (ticket 0128): the stock CLI and the shared cases load the
	# extension unpacked from the release archive, by its path.
	. "$REPO/sdlc/scripts/installed.sh"
	installed_unpack
	export THINKTHEN_DUCKDB_EXTENSION="$scratch/thinkthen.duckdb_extension"
	stock_cli
	"$PY" cpp/verify_package.py --extension "$THINKTHEN_DUCKDB_EXTENSION" --different-host "$TOOLS/older-host/duckdb"
	"$PY" cpp/verify_interrupt.py --extension "$THINKTHEN_DUCKDB_EXTENSION"
	sh "$LIMIT" 900 "$PY" tools/conformance.py
	sh "$LIMIT" 900 "$PY" tools/verbs_suite.py b13c_try_details_members b13c_try_details_prepared b13c_try_details_blank_context_keeps_good_siblings b13c_try_details_whole_request_failure b13c_context_and_batch_one_wire_identity b13c_warm_first_seen_context b13c_packed_total_admits_one_attempt
	sh "$LIMIT" 900 "$PY" tools/settings_suite.py b13c_warm_zero_budget the_process_request_total_holds_across_calls
	echo "check: databases/duckdb passes, installed"
	exit 0
fi

echo "== build, lint, and unit tests"
cargo fmt --all --manifest-path bridge/Cargo.toml -- --check
cargo clippy --locked --offline --manifest-path bridge/Cargo.toml --all-targets --all-features -- -D warnings
cargo test --locked --offline --manifest-path bridge/Cargo.toml --lib
sh cpp/build.sh

echo "== source checks and deny"
python3 tools/source_checks.py
cargo deny --locked --offline --manifest-path bridge/Cargo.toml check --config ../../deny.toml advisories bans licenses sources

stock_cli
"$PY" cpp/verify_package.py --extension build/thinkthen.duckdb_extension --different-host "$TOOLS/older-host/duckdb"
"$PY" cpp/verify_interrupt.py --extension build/thinkthen.duckdb_extension

if [ "$profile" = stress ]; then
	echo "== opt-in host campaigns"
	for suite in signal_suite settings_suite databases_suite relate_suite; do
		sh "$LIMIT" 900 "$PY" "tools/$suite.py"
	done
	echo "check: databases/duckdb passes, stress"
	exit 0
fi

echo "== suites"
for suite in verbs_suite settings_suite signal_suite relate_suite databases_suite conformance; do
	sh "$LIMIT" 900 "$PY" "tools/$suite.py"
done
THINKTHEN_DUCKDB_CLI_PATH="$CLI" sh "$LIMIT" 900 "$PY" tools/site_examples.py
sh tools/selftests.sh "$PY"
echo "check: databases/duckdb passes"
