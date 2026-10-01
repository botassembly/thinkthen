#!/bin/sh
# The DuckDB surface (tickets 0110, 0118, and 0201). The surfaces rung passes its
# loopback port as $1. Each suite also starts its own backend, so counts
# never mix. A missing toolchain reports "not run" (exit 77), never "pass".
set -eu
HERE=$(cd -- "$(dirname -- "$0")" && pwd)
cd -- "$HERE"
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress|smoke) ;; *) echo "duckdb: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS
# macOS has no `timeout` (ticket 0128).
LIMIT=$HERE/../../sdlc/scripts/time-limit
. "$HERE/tools/version.env"
TOOLS=${THINKTHEN_TOOLCHAINS:-$HOME/.cache/thinkthen-toolchains}/duckdb/$DUCKDB_VERSION
CLI=${THINKTHEN_DUCKDB_CLI:-$TOOLS/duckdb}
PY=$TOOLS/venv/bin/python
case $(uname -s):$(uname -m) in
Linux:x86_64 | Linux:aarch64 | Darwin:arm64 | Darwin:x86_64) ;;
*) echo "not run: no pinned DuckDB C++ host for $(uname -s):$(uname -m)"; exit 77 ;;
esac
if [ ! -x "$CLI" ] || [ ! -x "$PY" ] || [ ! -f "$TOOLS/platform.txt" ]; then
	echo "not run: the DuckDB $DUCKDB_VERSION toolchain is missing; run databases/duckdb/tools/setup.sh --fetch"
	exit 77
fi
case $(uname -s):$(uname -m) in
Linux:x86_64) expected_platform=linux_amd64 ;;
Linux:aarch64) expected_platform=linux_arm64 ;;
Darwin:arm64) expected_platform=osx_arm64 ;;
Darwin:x86_64) expected_platform=osx_amd64 ;;
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
# Shared rule 6: no suite sees a real key or a remote address. The replay smoke keeps the
# loopback address, placeholder key and scratch cache that sdlc/scripts/smoke set.
[ "$profile" = smoke ] || unset THINKTHEN_API_KEY THINKTHEN_BASE_URL THINKTHEN_CACHE
REPO=$(cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
usage_home
export THINKTHEN_BACKEND_BIN="${CARGO_TARGET_DIR:-$REPO/target}/debug/conformance-backend"
[ "$profile" = smoke ] || [ -x "$THINKTHEN_BACKEND_BIN" ] || {
	echo "check: the loopback backend is not built; run cargo build --package conformance-backend at the repository root" >&2
	exit 1
}
export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"
if [ "$profile" = smoke ]; then
	smoke_guard
	# The replay smoke (ticket 0335): the loadable extension copied to a scratch folder, as the
	# archive lays it out, and loaded from there by its path.
	sh cpp/build.sh
	scratch_dir installed
	cp build/thinkthen.duckdb_extension "$installed/"
	cd "$installed"
	"$PY" -c 'import os, sys, duckdb
db = duckdb.connect(config={"allow_unsigned_extensions": "true"})
db.execute(f"LOAD \x27{sys.argv[1]}\x27")
(value,), = db.execute("SELECT thinkthen_decide(?, ?)",
                       [os.environ["THINKTHEN_TEST_SMOKE_QUESTION"], os.environ["THINKTHEN_TEST_SMOKE_TEXT"]]).fetchall()
print("smoke:", {True: "true", False: "false", None: "null"}[value])' "$installed/thinkthen.duckdb_extension"
	exit
fi
stock_cli() {
	echo "== the stock CLI loads the extension"
	scratch_dir home && scratch_dir cache && scratch_dir config
	answer=$(env -i PATH="$PATH" HOME="$home" XDG_CACHE_HOME="$cache" XDG_CONFIG_HOME="$config" \
		THINKTHEN_API_KEY=sk-loopback-duckdb-check THINKTHEN_BASE_URL="http://127.0.0.1:$PORT/generic/v1" \
		sh "$LIMIT" 60 "$CLI" -unsigned -noheader -list -c "LOAD '${THINKTHEN_DUCKDB_EXTENSION:-$HERE/build/thinkthen.duckdb_extension}'; SELECT thinkthen_decide('Is it a refund?', 'refund now');")
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
	# Ticket 0374: the shipped extension keeps its own panic hook.
	own_panic_hook "$THINKTHEN_DUCKDB_EXTENSION"
	stock_cli
	"$PY" cpp/verify_package.py --extension "$THINKTHEN_DUCKDB_EXTENSION" --different-host "$TOOLS/older-host/duckdb"
	"$PY" cpp/verify_interrupt.py --extension "$THINKTHEN_DUCKDB_EXTENSION"
	sh "$LIMIT" 900 "$PY" tools/conformance.py
	sh "$LIMIT" 900 "$PY" tools/plan_suite.py p1_native_struct_is_keyless_and_sends_nothing plan_refusals_and_named_binding_never_send positive_process_total_denies_the_next_actual_send
	sh "$LIMIT" 900 "$PY" tools/find_suite.py original_duplicate_and_ties null_empty_and_invalid_units_do_not_send portable_find_settings_and_removed_slots held_find_and_spent_statement_budget
	sh "$LIMIT" 900 "$PY" tools/portable_suite.py named_decide_binds_by_name_and_settings_refuse_before_send listed_settings_and_members_are_distinct_overloads keyed_decide_preserves_key_body_and_one_send keyed_verbs_keep_members_order_and_refuse_duplicate_keys six_song_vector_and_keyed_forms_share_one_packed_body removed_warm_forms_refuse_without_transport probability_and_details_share_one_portable_request annotate_accepts_call_settings_and_refuses_question_only_keys try_details_keeps_recoverable_settings_failures_beside_an_answer recognize_named_settings_and_old_deadline_boundary
	sh "$LIMIT" 900 "$PY" tools/verbs_suite.py b13c_try_details_members b13c_try_details_prepared b13c_try_details_blank_context_keeps_good_siblings b13c_try_details_whole_request_failure b13c_context_and_batch_one_wire_identity portable_vector_first_seen_context b13c_packed_total_admits_one_attempt b13c_try_details_total_one_preserves_answered_rows b13c_try_details_total_zero_sends_nothing b13c_try_details_split_denials b13c_try_details_denied_retry_keeps_later_answer portable_batch_identity
	sh "$LIMIT" 900 "$PY" tools/settings_suite.py portable_decide_zero_budget the_process_request_total_holds_across_calls environment_token_cap_refuses_before_any_send
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
sh "$LIMIT" 900 "$PY" tools/portable_suite.py
sh "$LIMIT" 900 "$PY" tools/plan_suite.py
sh "$LIMIT" 900 "$PY" tools/find_suite.py original_duplicate_and_ties null_empty_and_invalid_units_do_not_send portable_find_settings_and_removed_slots held_find_and_spent_statement_budget
THINKTHEN_DUCKDB_CLI_PATH="$CLI" sh "$LIMIT" 900 "$PY" tools/site_examples.py
sh tools/selftests.sh "$PY"
echo "check: databases/duckdb passes"
