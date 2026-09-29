#!/bin/sh
# The SQLite surface's check (ticket 0109). The surface rung passes its
# loopback port as $1. Every test here starts its own backend, because each
# one counts sends, so the check leaves the shared port unused. Exit 77 means
# "not run": a missing toolchain never reports a pass.
set -eu
unset THINKTHEN_API_KEY
here=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
cd -- "$here"
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress) ;; *) echo "sqlite: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS
# macOS has no `timeout` (ticket 0128).
LIMIT=$here/../../sdlc/scripts/time-limit
source=${SQLITE_AMALGAMATION:-$HOME/.cache/thinkthen-toolchains/sqlite-amalgamation-3500000}
old=${SQLITE_OLD_AMALGAMATION:-$HOME/.cache/thinkthen-toolchains/sqlite-amalgamation-3490000}
python=${THINKTHEN_PYTHON:-python3}
case $(uname -s) in Darwin) suffix=dylib ;; *) suffix=so ;; esac

not_run() {
	echo "not run  databases/sqlite: $1; run databases/sqlite/setup.sh once"
	exit 77
}
command -v cc >/dev/null || not_run "no cc to build the SQLite 3.50.0 host"
if [ "$suffix" = dylib ]; then
	command -v shasum >/dev/null || not_run "no shasum to check the pinned source on macOS"
	hash_check() { shasum -a 256 -c "$1"; }
else
	command -v sha256sum >/dev/null || not_run "no sha256sum to check the pinned source on Linux"
	hash_check() { sha256sum --check --quiet "$1"; }
fi
"$python" -c 'import sys; sys.exit(sys.version_info < (3, 10))' 2>/dev/null || not_run "no Python 3.10 or later"
(cd -- "$source" 2>/dev/null && hash_check "$here/amalgamation.sha256") >/dev/null 2>&1 ||
	not_run "no SQLite 3.50.0 amalgamation with the pinned hashes in $source"
if [ "$suffix" = dylib ]; then
	(cd -- "$old" 2>/dev/null && hash_check "$here/amalgamation-3490000.sha256") >/dev/null 2>&1 ||
		not_run "no SQLite 3.49.0 amalgamation with the pinned hashes in $old"
fi
cargo fetch --locked --offline --quiet 2>/dev/null || not_run "the cargo cache lacks a locked crate"
host=$(SQLITE_AMALGAMATION=$source SQLITE_OLD_AMALGAMATION=$old sh tests/host_sqlite.sh)
export THINKTHEN_SQLITE_CLI="$host/sqlite3"
if [ "$suffix" = dylib ]; then
	export THINKTHEN_SQLITE_PROBE_PINNED="$host/load-probe-3500000"
	export THINKTHEN_SQLITE_PROBE_OLD="$host/load-probe-3490000"
else
	export LD_LIBRARY_PATH="$host${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
fi

step() { echo "== sqlite: $1"; }

if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
	# The installed-file mode (ticket 0128): the shared cases and the examples load the
	# library unpacked from the release archive, by its path.
	. ../../sdlc/scripts/scratch.sh
	. ../../sdlc/scripts/installed.sh
	installed_unpack
	THINKTHEN_SQLITE_EXTENSION=$(echo "$scratch"/libthinkthen0.*)
	[ -f "$THINKTHEN_SQLITE_EXTENSION" ] || { echo "FAIL     the archive holds no libthinkthen0 library" >&2; exit 1; }
	export THINKTHEN_SQLITE_EXTENSION
	for test in tests/examples.py tests/conformance.py tests/test_probability.py tests/test_portable_batch.py; do
		step "$test, installed"
		sh "$LIMIT" 300 "$python" "$test"
	done
	step "keyed rows, slot reuse and the JSON-only package counter, installed"
	sh "$LIMIT" 300 "$python" -c 'import sys; sys.path.insert(0, "tests"); from test_redesign import test_six_song_e1_keyed_join_sends_one_exact_packed_body as e1, test_keyed_decide_reuses_one_packed_reply_across_key_probes as keyed, test_two_alternating_slots_reuse_rows_with_disk_cache_disabled as slots; from test_settings import test_recording_counter_excludes_lock_files as counter; e1(); keyed(); slots(); counter()'
	step "selected find value, budget and interrupt boundaries, installed"
	sh "$LIMIT" 300 "$python" -c 'import sys; sys.path.insert(0, "tests"); from test_values import test_find_preserves_duplicate_positions_and_strict_ties as answers, test_find_null_empty_and_invalid_inputs_never_send as invalid; from test_try_budget import test_find_uses_the_scalar_budget_and_total_before_a_second_send as budget; from test_interrupt import test_find_is_cancelled_while_its_one_send_is_held as interrupt; answers(); invalid(); budget(); interrupt()'
	step "the pinned native host's successful and failed load results, installed"
	sh "$LIMIT" 60 "$python" -c 'import sys; sys.path.insert(0, "tests"); from test_schema import test_a_host_below_the_floor_refuses_the_load as failed, test_pinned_host_keeps_a_successful_registration_available as successful; successful(); failed()'
	echo "pass     databases/sqlite, installed"
	exit 0
fi

step "every cargo call passes --locked and --offline"
if grep -nE '^[^#]*cargo (build|test|clippy|fetch|run|check|doc)' check.sh | grep -vE -- '--locked.*--offline|--offline.*--locked'; then
	echo "FAIL     the cargo calls above lack --locked or --offline" >&2
	exit 1
fi

step "format, lints, and unit tests"
cargo fmt --check
cargo clippy --locked --offline --quiet --all-targets --all-features -- -D warnings
cargo test --locked --offline --quiet --lib

step "the release build carries no home path"
RUSTFLAGS="--remap-path-prefix=$HOME=/build" cargo build --locked --offline --quiet --release
# One build-folder rule, as tests/helper.py reads it: CARGO_TARGET_DIR, or each workspace's own target.
library=${CARGO_TARGET_DIR:-$here/target}/release/libthinkthen0.$suffix
if grep -aFq -- "$HOME" "$library"; then
	echo "FAIL     $library names the builder's home" >&2
	exit 1
fi

step "one exported symbol and one panic guard"
if [ "$suffix" = dylib ]; then
	symbols=$(nm -gU "$library" | awk '{ print $NF }')
	[ "$symbols" = _sqlite3_thinkthen_init ] || { echo "FAIL     the library exports: $symbols" >&2; exit 1; }
else
	symbols=$(nm -D --defined-only -- "$library" | awk '{ print $NF }')
	[ "$symbols" = sqlite3_thinkthen_init ] || { echo "FAIL     the library exports: $symbols" >&2; exit 1; }
fi
guards=$(grep -rn 'catch_unwind(' src | wc -l)
[ "$guards" = 1 ] || { echo "FAIL     src holds $guards catch_unwind calls, not 1" >&2; exit 1; }

step "the loopback backend"
cargo build --locked --offline --quiet --manifest-path ../../Cargo.toml --package conformance-backend

failed=""
if [ "$profile" = stress ]; then
	for test in tests/test_interrupt.py tests/test_settings.py; do
		step "$test, stress"
		sh "$LIMIT" 300 "$python" "$test" || failed="$failed $test"
	done
	[ -z "$failed" ] || { echo "FAIL     databases/sqlite:$failed" >&2; exit 1; }
	echo "pass     databases/sqlite, stress"
	exit 0
fi
for test in tests/test_*.py tests/examples.py tests/conformance.py; do
	step "$test"
	sh "$LIMIT" 300 "$python" "$test" || failed="$failed $test"
done
[ -z "$failed" ] || { echo "FAIL     databases/sqlite:$failed" >&2; exit 1; }
echo "pass     databases/sqlite"
