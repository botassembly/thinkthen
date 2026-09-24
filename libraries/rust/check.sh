#!/bin/sh
# The Rust examples surface. The surface rung passes its loopback port as $1.
set -eu
cd -- "$(dirname -- "$0")"

cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
cargo test --locked --offline
# The rung's own backend answers the slide as the test's backends do.
cargo build --quiet --locked --offline --example slide
cache=$(mktemp -d)
trap 'rm -rf -- "$cache"' EXIT
env -i THINKTHEN_CACHE="$cache" THINKTHEN_API_KEY=sk-examples-loopback \
    THINKTHEN_BASE_URL="http://127.0.0.1:$1/generic/v1" \
    "${CARGO_TARGET_DIR:-target}/debug/examples/slide" | diff -u examples/slide.txt -
