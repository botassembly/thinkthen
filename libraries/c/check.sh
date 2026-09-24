#!/bin/sh
# The C surface. The surface rung passes its loopback port as $1.
set -eu
cd -- "$(dirname -- "$0")"

command -v cc >/dev/null 2>&1 || { echo 'libraries/c: no C compiler' >&2; exit 77; }
cargo fmt --check
cargo clippy --locked --offline --all-targets -- -D warnings
cargo test --locked --offline
# The rung's own backend answers the slide as the test's backends do.
cargo build --quiet --locked --offline --lib
target=${CARGO_TARGET_DIR:-target}/debug
cache=$(mktemp -d)
trap 'rm -rf -- "$cache"' EXIT
cc -std=c11 -Wall -Wextra -Werror -I include examples/slide.c -L "$target" -lthinkthen_c \
    -Wl,-rpath,"$target" -o "$cache/slide"
env -i THINKTHEN_CACHE="$cache/cache" THINKTHEN_API_KEY=sk-examples-loopback \
    THINKTHEN_BASE_URL="http://127.0.0.1:$1/generic/v1" \
    "$cache/slide" >"$cache/out" 2>"$cache/err"
[ ! -s "$cache/err" ] || { cat -- "$cache/err" >&2; exit 1; }
diff -u examples/slide.txt "$cache/out"
