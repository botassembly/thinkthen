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
if [ "$profile" = routine ]; then
    export THINKTHEN_CONFORMANCE_IDS="${THINKTHEN_CONFORMANCE_IDS:-$PWD/conformance/routine-ids.txt}"
fi

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
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
usage_home
scratch_dir scratch
export XDG_CACHE_HOME="$scratch/cache" XDG_CONFIG_HOME="$scratch/config" THINKTHEN_CACHE="$scratch/thinkthen"

# Its own target folder keeps these builds from evicting the other rungs'.
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-target}/polars"
consumer=libraries/polars/consumer/Cargo.toml
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    [ -f "$THINKTHEN_ARTIFACT" ] && [ ! -L "$THINKTHEN_ARTIFACT" ] || { echo 'polars: installed crate is missing or linked' >&2; exit 1; }
    # The same public consumer links the unpacked source archive.
    scratch_dir source
    scratch_dir installed
    tar -xzf "$THINKTHEN_ARTIFACT" -C "$source" --strip-components=1
    python3 - "$PWD" "$source" "$installed" <<'RUSTFRAME'
import shutil,sys
from pathlib import Path
root,source,installed=map(Path,sys.argv[1:])
consumer=installed/'consumer'
shutil.copytree(root/'libraries/polars/consumer',consumer,ignore=shutil.ignore_patterns('target'))
manifest=consumer/'Cargo.toml'
shared=installed/'rust/consumer/src'
shared.mkdir(parents=True)
shutil.copyfile(root/'libraries/rust/consumer/src/fixture.rs',shared/'fixture.rs')
manifest.write_text(manifest.read_text().replace('../../../crates/thinkthen',str(source)))
main=consumer/'src/main.rs'
main.write_text(main.read_text().replace('../../../rust/consumer/src/fixture.rs',str(shared/'fixture.rs')))
RUSTFRAME
    consumer="$installed/consumer/Cargo.toml"
else
set -- --locked --offline --package thinkthen --features polars
cargo clippy "$@" --lib --bins --test polars -- -D warnings
cargo clippy "$@" --no-default-features --lib -- -D warnings
if [ "$profile" = stress ]; then
    cargo test "$@" --test polars throttle_equality::two_hundred_series_records_match_the_slice -- --ignored --exact
    exit 0
fi
cargo test "$@" --test polars
if [ "$profile" = full ]; then
    cargo test "$@" --test polars release_only_ -- --ignored
fi
cargo test "$@" --doc PolarsEngine
fi

# Actual frame consumers reuse the same native corpus, framing and projectors.
cargo clippy --locked --offline --manifest-path "$consumer" --all-targets -- -D warnings
cargo build --locked --offline --manifest-path "$consumer"
python3 - "$PWD" "$CARGO_TARGET_DIR/debug/thinkthen-polars-consumer" <<'PYFRAME'
import sys
from pathlib import Path
root=Path(sys.argv[1]);sys.path.insert(0,str(root/'libraries/python/tests'))
from native_fixture import run
sys.exit(bool(run('rust-polars',[str(Path(sys.argv[2]).resolve())],root)))
PYFRAME
