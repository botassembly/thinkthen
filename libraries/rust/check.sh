#!/bin/sh
# The Rust examples surface. The surface rung passes its loopback port as $1.
set -eu
tools_group=rust
. "$(dirname -- "$0")/../../sdlc/scripts/resolved-tools"
cd -- "$(dirname -- "$0")"
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress|smoke) ;; *) echo "rust: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS
if [ "$profile" = routine ]; then
    export THINKTHEN_CONFORMANCE_IDS="${THINKTHEN_CONFORMANCE_IDS:-$PWD/../../conformance/routine-ids.txt}"
fi
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
. ../../sdlc/scripts/scratch.sh
usage_home
root=$(CDPATH= cd ../.. && pwd)
if [ "$profile" = stress ] && [ -z "${THINKTHEN_ARTIFACT:-}" ]; then
    cargo package --locked --offline --no-verify --manifest-path "$root/crates/thinkthen/Cargo.toml" -p thinkthen
    version=$(sed -n 's/^version = "\(.*\)"$/\1/p' "$root/crates/thinkthen/Cargo.toml" | head -n 1)
    THINKTHEN_ARTIFACT="$root/target/package/thinkthen-$version.crate"
fi
native_cases() {
    python3 - "$root" "$1" <<'RUSTNATIVE'
import os,sys,tomllib
from pathlib import Path
root=Path(sys.argv[1]);sys.path.insert(0,str(root/'conformance/children'))
from native_fixture import run
binary=Path(os.environ.get('CARGO_TARGET_DIR',str(root/'libraries/rust/target')))/'debug/thinkthen-rust-consumer'
binary.parent.mkdir(parents=True,exist_ok=True)
if os.environ.get('THINKTHEN_COVERAGE_ROOT'):
    os.environ['CARGO_TARGET_DIR'] = str(Path(os.environ['THINKTHEN_TEST_BACKEND']).parents[1])
compiler_env={'RUSTUP_TOOLCHAIN':tomllib.loads((root/'rust-toolchain.toml').read_text())['toolchain']['channel']}
sys.exit(bool(run('rust',[str(binary)],root,rust_manifest=Path(sys.argv[2]),extra_env=compiler_env)))
RUSTNATIVE
}
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    [ -f "$THINKTHEN_ARTIFACT" ] && [ ! -L "$THINKTHEN_ARTIFACT" ] || { echo 'rust: installed crate is missing or linked' >&2; exit 1; }
    scratch_dir installed
    mkdir "$installed/crate" "$installed/consumer"
    tar -xzf "$THINKTHEN_ARTIFACT" -C "$installed/crate" --strip-components=1
    python3 - "$root" "$installed" <<'RUSTCONSUMER'
import shutil,sys
from pathlib import Path
root,installed=map(Path,sys.argv[1:])
project=installed/'consumer'
shutil.copytree(root/'libraries/rust/consumer',project,dirs_exist_ok=True,ignore=shutil.ignore_patterns('target'))
manifest=project/'Cargo.toml'
manifest.write_text(manifest.read_text().replace('../../../crates/thinkthen',str(installed/'crate')).replace('../../../conformance/backend',str(root/'conformance/backend')))
RUSTCONSUMER
    if [ -n "${THINKTHEN_COVERAGE_ROOT:-}" ]; then
        separator=$(printf '\037')
        CARGO_ENCODED_RUSTFLAGS="${CARGO_ENCODED_RUSTFLAGS:-}${CARGO_ENCODED_RUSTFLAGS:+$separator}--remap-path-prefix=$installed/crate=$THINKTHEN_COVERAGE_ROOT/crates/thinkthen"
        export CARGO_ENCODED_RUSTFLAGS
    fi
    if [ "$profile" != stress ]; then native_cases "$installed/consumer/Cargo.toml"; fi
    cargo test --locked --offline --manifest-path "$installed/consumer/Cargo.toml" --test sessions -- --test-threads=1
    if [ "$profile" = stress ]; then
        cargo test --locked --offline --manifest-path "$installed/consumer/Cargo.toml" --features session-stress --test sessions repeated_session_contention_never_loses_wake_or_settlement -- --exact --test-threads=1
    fi
    echo 'rust: pass, installed'
    exit 0
fi
if [ "$profile" = smoke ]; then
    smoke_guard
    cargo build --quiet --locked --offline --bin smoke
    "${CARGO_TARGET_DIR:-target}/debug/smoke"
    exit
fi

if [ "$profile" = full ]; then
    native_cases "$PWD/consumer/Cargo.toml"
    echo 'rust: pass, source'
    exit 0
fi

cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
cargo test --locked --offline
# The rung's own backend answers the slide as the test's backends do.
cargo build --quiet --locked --offline --example slide
. ../../sdlc/scripts/scratch.sh
scratch_dir cache
env -i THINKTHEN_CACHE="$cache" THINKTHEN_API_KEY=sk-examples-loopback \
    THINKTHEN_BASE_URL="http://127.0.0.1:$1/generic/v1" \
    "${CARGO_TARGET_DIR:-target}/debug/examples/slide" >"$cache/out" 2>"$cache/err"
[ ! -s "$cache/err" ] || { cat -- "$cache/err" >&2; exit 1; }
diff -u examples/slide.txt "$cache/out"
