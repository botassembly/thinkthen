#!/bin/sh
# The Rust examples surface. The surface rung passes its loopback port as $1.
set -eu
python3 "$(dirname -- "$0")/../../conformance/children/privacy.py" --sources "$(dirname -- "$0")" --public --artifact-env
cd -- "$(dirname -- "$0")"
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress|smoke) ;; *) echo "rust: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS
if [ "$profile" = routine ]; then
    export THINKTHEN_CONFORMANCE_IDS="${THINKTHEN_CONFORMANCE_IDS:-$PWD/../../conformance/routine-ids.txt}"
fi
if [ "$profile" = stress ]; then
    echo 'rust: not run: no port load campaign'
    exit 77
fi
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
. ../../sdlc/scripts/scratch.sh
usage_home
root=$(CDPATH= cd ../.. && pwd)
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    [ -f "$THINKTHEN_ARTIFACT" ] && [ ! -L "$THINKTHEN_ARTIFACT" ] || { echo 'rust: installed crate is missing or linked' >&2; exit 1; }
    scratch_dir installed
    mkdir "$installed/crate" "$installed/consumer"
    tar -xzf "$THINKTHEN_ARTIFACT" -C "$installed/crate" --strip-components=1
    python3 check.py "$root" "$installed"
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
    python3 check.py "$root"
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
