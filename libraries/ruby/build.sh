#!/bin/sh
# Build the extension and the gem with the pinned Ruby that check.sh
# exported as RUBY. Every cargo call runs --locked --offline.
set -eu
cd -- "$(dirname -- "$0")"
cargo build --release --locked --offline
target=${CARGO_TARGET_DIR:-target}/release
# The library file's name follows the host: lib<name>.so, lib<name>.dylib.
built=$(find "$target" -maxdepth 1 -name 'libthinkthen_ruby.*' ! -name '*.d' ! -name '*.rlib' | head -n 1)
dlext=$("$RUBY" -e 'print RbConfig::CONFIG["DLEXT"]')
mkdir -p lib/thinkthen
rm -f lib/thinkthen/thinkthen.*
cp -- "$built" "lib/thinkthen/thinkthen.$dlext"
rm -f -- *.gem
"$(dirname -- "$RUBY")/gem" build thinkthen.gemspec --silent >/dev/null
