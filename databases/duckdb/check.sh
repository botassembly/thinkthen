#!/bin/sh
# The DuckDB surface (tickets 0110, 0118, and 0201). The surfaces rung passes its
# loopback port as $1. Each suite also starts its own backend, so counts
# never mix. A missing toolchain reports "not run" (exit 1), never "pass".
set -eu
tools_group=duckdb
. "$(dirname -- "$0")/../../sdlc/scripts/resolved-tools"
HERE=$(cd -- "$(dirname -- "$0")" && pwd)
cd -- "$HERE"
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress|smoke) ;; *) echo "duckdb: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS
# macOS has no `timeout` (ticket 0128).
LIMIT=$HERE/../../sdlc/scripts/time-limit
INPUTS_HERE=$HERE/tools
. "$INPUTS_HERE/inputs.sh"
DUCKDB_VERSIONS=$(input_versions)
DEFAULT_VERSION=${DUCKDB_VERSIONS%% *}
host_target=$(input_target)
[ "${THINKTHEN_DUCKDB_VERSION:-$DEFAULT_VERSION}" = "$DEFAULT_VERSION" ] || {
	echo 'check: the two-version check starts with the default DuckDB selector' >&2; exit 2;
}
select_host() {
	input_select "$1" "$host_target"
	CLI=$HERE/build/community/$DUCKDB_VERSION/build/release/duckdb
	[ "$DUCKDB_VERSION" != "$DEFAULT_VERSION" ] || CLI=${THINKTHEN_DUCKDB_CLI:-$CLI}
	PY=$TOOLS/venv/bin/python
	if [ ! -x "$CLI" ] || [ ! -x "$PY" ]; then
		echo "not run: the DuckDB $DUCKDB_VERSION toolchain is missing; run databases/duckdb/tools/setup.sh --fetch"
		exit 1
	fi
	observed=$("$PY" -c 'import duckdb; print(duckdb.sql("PRAGMA version").fetchall()[0][1])')
	[ "$observed" = "$(printf '%.10s' "$duckdb_source_commit")" ] || { echo 'check: the Python DuckDB source differs from its pin' >&2; exit 1; }
	observed=$("$CLI" -noheader -list -c 'SELECT source_id FROM pragma_version();')
	case $observed in
	"$(printf '%.8s' "$duckdb_source_commit")"|"$(printf '%.10s' "$duckdb_source_commit")") ;;
	*) echo 'check: the CLI DuckDB source differs from its pin' >&2; exit 1 ;;
	esac
	observed=$("$PY" -c 'import duckdb; print(duckdb.sql("PRAGMA platform").fetchall()[0][0])')
	[ "$observed" = "$platform" ] || { echo 'check: the Python DuckDB platform differs from its target' >&2; exit 1; }
	observed=$("$CLI" -noheader -list -c 'SELECT platform FROM pragma_platform();')
	[ "$observed" = "$platform" ] || { echo 'check: the CLI DuckDB platform differs from its target' >&2; exit 1; }
	THINKTHEN_DUCKDB_VERSION=$DUCKDB_VERSION
	THINKTHEN_DUCKDB_EXTENSION=$HERE/build/artifacts/cpp/$DUCKDB_VERSION/$host_target/thinkthen.duckdb_extension
	export THINKTHEN_DUCKDB_VERSION THINKTHEN_DUCKDB_EXTENSION
}
for version in $DUCKDB_VERSIONS; do select_host "$version"; done
select_host "$DEFAULT_VERSION"
python3 "$HERE/../../conformance/children/privacy.py" --sources "$HERE" --public --artifact-env
if [ -n "${CHECK_SETUP_ONLY:-}" ]; then echo "setup ok"; exit 0; fi
PORT=${1:?check.sh takes the loopback port}
# Shared rule 6: no suite sees a real key or a remote address. The replay smoke keeps the
# loopback address, placeholder key and scratch cache that sdlc/scripts/smoke set.
[ "$profile" = smoke ] || unset THINKTHEN_API_KEY THINKTHEN_BASE_URL THINKTHEN_CACHE
REPO=$(cd -- ../.. && pwd)
. "$REPO/sdlc/scripts/scratch.sh"
. "$REPO/sdlc/scripts/installed.sh"
heavy_scripts=$REPO/sdlc/scripts
. "$heavy_scripts/heavy-compile"
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
usage_home
export THINKTHEN_BACKEND_BIN="${CARGO_TARGET_DIR:-$REPO/target}/debug/conformance-backend"
[ "$profile" = smoke ] || [ -x "$THINKTHEN_BACKEND_BIN" ] || {
	echo "check: the loopback backend is not built; run cargo build --package conformance-backend at the repository root" >&2
	exit 1
}
if [ "${THINKTHEN_FULL_COMPILER:-}" != 1 ]; then
export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"
fi
if [ "$profile" = smoke ]; then
	smoke_guard
	# The replay smoke (ticket 0335): the loadable extension copied to a scratch folder, as the
	# archive lays it out, and loaded from there by its path.
	if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
		release=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$REPO/crates/thinkthen/Cargo.toml" | head -n 1)
		python3 cpp/verify_repository.py "$THINKTHEN_ARTIFACT" "$host_target" "$release"
		installed_unpack
		python3 cpp/verify_repository.py "$scratch" "$host_target" "$release"
		THINKTHEN_DUCKDB_EXTENSION=$scratch/$duckdb_metadata_version/$platform/thinkthen.duckdb_extension
		export THINKTHEN_DUCKDB_EXTENSION
	else
		sh cpp/build.sh
	fi
	scratch_dir installed
	cp "$THINKTHEN_DUCKDB_EXTENSION" "$installed/"
	cd "$installed"
	"$PY" -c 'import os, sys, duckdb
db = duckdb.connect(config={"allow_unsigned_extensions": "true"})
db.execute("SET enable_progress_bar = false")
db.execute(f"LOAD \x27{sys.argv[1]}\x27")
(value,), = db.execute("SELECT thinkthen_decide(?, ?)",
                       [os.environ["THINKTHEN_TEST_SMOKE_QUESTION"], os.environ["THINKTHEN_TEST_SMOKE_TEXT"]]).fetchall()
print("smoke:", {True: "true", False: "false", None: "null"}[value])' "$installed/thinkthen.duckdb_extension"
	exit
fi
python3 tools/receipt_types.py --check
python3 tools/request_helpers.py --check
stock_cli() {
	echo "== the stock CLI loads the extension"
	scratch_dir home && scratch_dir cache && scratch_dir config
	answer=$(env -i PATH="$PATH" HOME="$home" XDG_CACHE_HOME="$cache" XDG_CONFIG_HOME="$config" \
		THINKTHEN_API_KEY=sk-loopback-duckdb-check THINKTHEN_BASE_URL="http://127.0.0.1:$PORT/generic/v1" \
		sh "$LIMIT" 60 "$CLI" -unsigned -noheader -list -c "LOAD '$THINKTHEN_DUCKDB_EXTENSION'; SELECT thinkthen_decide('Is it a refund?', 'refund now');")
	[ "$answer" = true ] || {
		echo "check: the stock CLI read '$answer', not true" >&2
		exit 1
	}
}
verify_selected_package() {
	for other in $DUCKDB_VERSIONS; do
		[ "$other" != "$DUCKDB_VERSION" ] || continue
		other_tools=$HERE/build/community/$other/build/release
		if [ "${1:-}" = installed ]; then
			"$PY" cpp/verify_package.py --extension "$THINKTHEN_DUCKDB_EXTENSION" --version "$DUCKDB_VERSION" --target "$host_target" --matching-host "$CLI" --different-host "$other_tools/duckdb" --different-version "$other" --repository-extension "$scratch/$(python3 tools/inputs.py select "$other" "$host_target" | cut -d " " -f 7)/$platform/thinkthen.duckdb_extension"
		else
			"$PY" cpp/verify_package.py --extension "$THINKTHEN_DUCKDB_EXTENSION" --version "$DUCKDB_VERSION" --target "$host_target" --matching-host "$CLI" --different-host "$other_tools/duckdb" --different-version "$other" --repository-extension "$HERE/build/artifacts/cpp/$other/$host_target/thinkthen.duckdb_extension"
		fi
	done
}
native_records() {
	(cd tools && "$PY" -c 'from harness import run; from native_records import check; check(run)')
}
older_suites() {
    for verb in decide choose tag score filter rank find annotate recognize relate; do
        [ "$profile" = routine ] || sh "$LIMIT" 300 python3 ../sqlite/tests/complete/parity.py duckdb "complete-$verb" >"$home/complete-$verb.log"
    done
	sh "$LIMIT" 900 "$PY" tools/images_suite.py
	sh "$LIMIT" 900 "$PY" tools/conformance.py
	sh "$LIMIT" 900 "$PY" tools/files_suite.py
	sh "$LIMIT" 900 "$PY" tools/rank_suite.py
	sh "$LIMIT" 900 "$PY" tools/rank_sets_suite.py
	sh "$LIMIT" 900 "$PY" tools/find_suite.py original_duplicate_and_ties null_empty_and_invalid_units_do_not_send portable_find_settings_and_removed_slots held_find_and_spent_statement_budget
	sh "$LIMIT" 900 "$PY" tools/portable_suite.py named_decide_binds_by_name_and_settings_refuse_before_send listed_settings_and_members_are_distinct_overloads keyed_decide_preserves_key_body_and_one_send keyed_verbs_keep_members_order_and_refuse_duplicate_keys six_song_vector_and_keyed_forms_share_one_packed_body removed_warm_and_probability_forms_refuse_without_transport rank_and_details_share_one_portable_request annotate_accepts_call_settings_and_refuses_question_only_keys recognize_named_settings_and_old_deadline_boundary
}
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
	# The installed-file mode (ticket 0128): the stock CLI and the shared cases load the
	# extension unpacked from the release archive, by its path.
	. "$REPO/sdlc/scripts/installed.sh"
	release=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$REPO/crates/thinkthen/Cargo.toml" | head -n 1)
	python3 cpp/verify_repository.py "$THINKTHEN_ARTIFACT" "$host_target" "$release"
	installed_unpack
	python3 cpp/verify_repository.py "$scratch" "$host_target" "$release"
	python3 tools/source_checks.py
	for version in $DUCKDB_VERSIONS; do
		select_host "$version"
		export THINKTHEN_DUCKDB_EXTENSION="$scratch/$duckdb_metadata_version/$platform/thinkthen.duckdb_extension"
		own_panic_hook "$THINKTHEN_DUCKDB_EXTENSION"
		stock_cli
		verify_selected_package installed
		native_records
		[ "$profile" = full ] || [ "$version" = "$DEFAULT_VERSION" ] || older_suites
	done
	select_host "$DEFAULT_VERSION"
	export THINKTHEN_DUCKDB_EXTENSION="$scratch/$duckdb_metadata_version/$platform/thinkthen.duckdb_extension"
	"$PY" cpp/verify_complete_tables.py --extension "$THINKTHEN_DUCKDB_EXTENSION"
	"$PY" cpp/verify_interrupt.py --extension "$THINKTHEN_DUCKDB_EXTENSION"
	[ "$profile" = routine ] || sh "$LIMIT" 1800 python3 ../sqlite/tests/complete/parity.py duckdb
	sh "$LIMIT" 300 python3 ../sqlite/tests/complete/facts.py duckdb
	[ "$profile" = full ] || sh "$LIMIT" 900 "$PY" tools/conformance.py
	sh "$LIMIT" 900 "$PY" tools/plan_suite.py p1_native_struct_is_keyless_and_sends_nothing plan_refusals_and_named_binding_never_send positive_process_total_denies_the_next_actual_send
	sh "$LIMIT" 900 "$PY" tools/find_suite.py original_duplicate_and_ties null_empty_and_invalid_units_do_not_send portable_find_settings_and_removed_slots held_find_and_spent_statement_budget
	sh "$LIMIT" 900 "$PY" tools/portable_suite.py named_decide_binds_by_name_and_settings_refuse_before_send listed_settings_and_members_are_distinct_overloads keyed_decide_preserves_key_body_and_one_send keyed_verbs_keep_members_order_and_refuse_duplicate_keys six_song_vector_and_keyed_forms_share_one_packed_body removed_warm_and_probability_forms_refuse_without_transport rank_and_details_share_one_portable_request annotate_accepts_call_settings_and_refuses_question_only_keys recognize_named_settings_and_old_deadline_boundary
	sh "$LIMIT" 900 "$PY" tools/verbs_suite.py b13c_context_and_batch_one_wire_identity portable_vector_first_seen_context b13c_packed_total_admits_one_attempt portable_batch_identity
	[ "$profile" = full ] || sh "$LIMIT" 900 "$PY" tools/rank_suite.py
	[ "$profile" = full ] || sh "$LIMIT" 900 "$PY" tools/rank_sets_suite.py
	sh "$LIMIT" 900 "$PY" tools/settings_suite.py portable_decide_zero_budget the_process_request_total_holds_across_calls environment_token_cap_refuses_before_any_send
	echo "check: databases/duckdb passes, installed"
	exit 0
fi

if [ "$profile" != full ]; then
echo "== build, lint, and unit tests"
cargo fmt --all --manifest-path bridge/Cargo.toml -- --check
heavy_compile cargo clippy --locked --offline --manifest-path bridge/Cargo.toml --all-targets --all-features -- -D warnings
heavy_compile cargo test --locked --offline --manifest-path bridge/Cargo.toml --lib
fi
for version in $DUCKDB_VERSIONS; do
	THINKTHEN_DUCKDB_VERSION=$version sh cpp/build.sh
done

if [ "$profile" != full ]; then
echo "== source checks and deny"
python3 tools/source_checks.py
python3 tools/input_version_cases.py
node "$REPO/site/scripts/check-duckdb-versions.mjs"
"$PY" tools/release_pack_cases.py "$host_target"
cargo deny --locked --offline --manifest-path bridge/Cargo.toml check --config ../../deny.toml advisories bans licenses sources

fi

for version in $DUCKDB_VERSIONS; do
	select_host "$version"
	stock_cli
	verify_selected_package
	[ "$profile" = full ] || [ "$version" = "$DEFAULT_VERSION" ] || older_suites
done
select_host "$DEFAULT_VERSION"
"$PY" cpp/verify_complete_tables.py --extension "$THINKTHEN_DUCKDB_EXTENSION"
"$PY" cpp/verify_interrupt.py --extension "$THINKTHEN_DUCKDB_EXTENSION"

if [ "$profile" = stress ]; then
	echo "== opt-in host campaigns"
	for suite in signal_suite settings_suite databases_suite relate_suite; do
		sh "$LIMIT" 900 "$PY" "tools/$suite.py"
	done
	echo "check: databases/duckdb passes, stress"
	exit 0
fi

echo "== complete native calls"
[ "$profile" = routine ] || sh "$LIMIT" 1800 python3 ../sqlite/tests/complete/parity.py duckdb
sh "$LIMIT" 300 python3 ../sqlite/tests/complete/facts.py duckdb
if [ "$profile" = full ]; then
    own_panic_hook "$THINKTHEN_DUCKDB_EXTENSION"
    sh "$LIMIT" 900 "$PY" tools/settings_suite.py portable_decide_zero_budget the_process_request_total_holds_across_calls environment_token_cap_refuses_before_any_send
    sh "$LIMIT" 900 "$PY" tools/find_suite.py held_find_and_spent_statement_budget
    echo "check: databases/duckdb passes, full"
    exit 0
fi
echo "== suites"
sh "$LIMIT" 900 "$PY" tools/images_suite.py
sh "$LIMIT" 900 "$PY" tools/backend_setups.py
sh "$LIMIT" 900 "$PY" tools/named_backends.py
for suite in files_suite verbs_suite rank_suite rank_sets_suite settings_suite signal_suite relate_suite databases_suite conformance; do
	sh "$LIMIT" 900 "$PY" "tools/$suite.py"
done
sh "$LIMIT" 900 "$PY" tools/portable_suite.py
sh "$LIMIT" 900 "$PY" tools/plan_suite.py
sh "$LIMIT" 900 "$PY" tools/find_suite.py original_duplicate_and_ties null_empty_and_invalid_units_do_not_send portable_find_settings_and_removed_slots held_find_and_spent_statement_budget
THINKTHEN_DUCKDB_CLI_PATH="$CLI" sh "$LIMIT" 900 "$PY" tools/site_examples.py
sh tools/selftests.sh "$PY"
echo "check: databases/duckdb passes"
