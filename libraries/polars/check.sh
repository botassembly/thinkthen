#!/bin/sh
# The Rust Polars lane (tickets 0120 and 0130): the `polars` feature of
# `thinkthen`, the one lane that compiles Polars. The surface rung passes its
# loopback port as $1. The tests start their own backends, so this check does
# not use it. It runs from the repository root, where the manifest is.
set -eu
cd -- "$(dirname -- "$0")/../.."
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress) ;; *) echo "polars: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS

# The probe: every crate the root lock names must already be in cargo's cache.
if ! cargo fetch --locked --offline >/dev/null 2>&1; then
	echo 'polars: not run; the crate cache lacks the locked crates.' >&2
	echo 'polars: on a networked machine, run: cargo fetch --locked' >&2
	exit 77
fi

# No paid backend: a fake key, a closed loopback port, and scratch folders.
unset THINKTHEN_API_KEY
export THINKTHEN_API_KEY=sk-polars-loopback
export THINKTHEN_BASE_URL=http://127.0.0.1:9/v1
. sdlc/scripts/scratch.sh
scratch_dir scratch
export XDG_CACHE_HOME="$scratch/cache" XDG_CONFIG_HOME="$scratch/config" THINKTHEN_CACHE="$scratch/thinkthen"

# Its own target folder keeps these builds from evicting the other rungs'.
# One flag set serves every step, the doctest's included.
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target}/polars"
export RUSTFLAGS='--cfg thinkthen_internal_doctest' RUSTDOCFLAGS='--cfg thinkthen_internal_doctest'
set -- --locked --offline --package thinkthen --features polars
cargo clippy "$@" --lib --bins --test 'polars_*' -- -D warnings
cargo clippy "$@" --no-default-features --lib -- -D warnings
if [ "$profile" = stress ]; then
    cargo test "$@" --test polars_throttle two_hundred_series_records_match_the_slice -- --ignored --exact
    exit 0
fi
cargo test "$@" --test 'polars_*'
cargo test "$@" --doc PolarsEngine
