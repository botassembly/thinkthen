#!/bin/sh
# The Rust Polars surface (ticket 0120). The surface rung passes its loopback
# port as $1. The tests start their own backends, so this check does not use it.
set -eu
cd -- "$(dirname -- "$0")"

# The probe: every crate the lock names must already be in cargo's cache.
if ! cargo fetch --locked --offline >/dev/null 2>&1; then
	echo 'polars: not run; the crate cache lacks the locked crates.' >&2
	echo 'polars: on a networked machine, run: cargo fetch --locked --manifest-path libraries/polars/Cargo.toml' >&2
	exit 77
fi

# No paid backend: a fake key, a closed loopback port, and scratch folders.
unset THINKTHEN_API_KEY
export THINKTHEN_API_KEY=sk-polars-loopback
export THINKTHEN_BASE_URL=http://127.0.0.1:9/v1
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
export XDG_CACHE_HOME="$scratch/cache" XDG_CONFIG_HOME="$scratch/config" THINKTHEN_CACHE="$scratch/thinkthen"

cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
cargo test --locked --offline
