#!/bin/sh
# The SQLite surface's check (ticket 0109). The surface rung passes its
# loopback port as $1. Every test here starts its own backend, because each
# one counts sends, so the check leaves the shared port unused. Exit 77 means
# "not run": a missing toolchain never reports a pass.
set -eu
unset THINKTHEN_API_KEY
here=$(CDPATH='' cd -- "$(dirname -- "$0")" && pwd)
cd -- "$here"
# macOS has no `timeout` (ticket 0128).
LIMIT=$here/../../sdlc/scripts/time-limit
source=${SQLITE_AMALGAMATION:-$HOME/.cache/thinkthen-toolchains/sqlite-amalgamation-3500000}

not_run() {
	echo "not run  databases/sqlite: $1; run databases/sqlite/setup.sh once"
	exit 77
}
command -v cc >/dev/null || not_run "no cc to build the SQLite 3.50.0 host"
python3 -c 'import sys; sys.exit(sys.version_info < (3, 10))' 2>/dev/null || not_run "no Python 3.10 or later"
(cd -- "$source" 2>/dev/null && sha256sum --check --quiet) <amalgamation.sha256 >/dev/null 2>&1 ||
	not_run "no SQLite 3.50.0 amalgamation with the pinned hashes in $source"
cargo fetch --locked --offline --quiet 2>/dev/null || not_run "the cargo cache lacks a locked crate"
host=$(SQLITE_AMALGAMATION=$source sh tests/host_sqlite.sh)
export THINKTHEN_SQLITE_CLI="$host/sqlite3"
export LD_LIBRARY_PATH="$host${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"

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
	for test in tests/examples.py tests/conformance.py; do
		step "$test, installed"
		sh "$LIMIT" 300 python3 "$test"
	done
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
library=${CARGO_TARGET_DIR:-$here/target}/release/libthinkthen0.so
found=$(strings -- "$library" | grep -c -- "$HOME" || true)
[ "$found" = 0 ] || { echo "FAIL     $found strings in $library name $HOME" >&2; exit 1; }

step "one exported symbol and one panic guard"
symbols=$(nm -D --defined-only -- "$library" | awk '{ print $NF }')
[ "$symbols" = sqlite3_thinkthen_init ] || { echo "FAIL     the library exports: $symbols" >&2; exit 1; }
guards=$(grep -rn 'catch_unwind(' src | wc -l)
[ "$guards" = 1 ] || { echo "FAIL     src holds $guards catch_unwind calls, not 1" >&2; exit 1; }

step "the loopback backend"
cargo build --locked --offline --quiet --manifest-path ../../Cargo.toml --package conformance-backend

failed=""
for test in tests/test_*.py tests/examples.py tests/conformance.py; do
	step "$test"
	sh "$LIMIT" 300 python3 "$test" || failed="$failed $test"
done
[ -z "$failed" ] || { echo "FAIL     databases/sqlite:$failed" >&2; exit 1; }
echo "pass     databases/sqlite"
