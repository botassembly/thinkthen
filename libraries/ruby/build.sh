#!/bin/sh
# Build the extension and the gem with the pinned Ruby that check.sh
# exported as RUBY. Every cargo call runs --locked --offline.
set -eu
cd -- "$(dirname -- "$0")"
# Cargo embeds source paths in panic locations. The remap keeps the builder's
# home out of the gem's extension, as in the other bindings (ticket 0128).
RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"
# RUSTFLAGS from the environment replaces .cargo/config.toml target rustflags.
# Preserve the Darwin extension's load-time Ruby symbol lookup when remapping.
case $(uname -s) in
  Darwin) RUSTFLAGS="$RUSTFLAGS -C link-arg=-Wl,-undefined,dynamic_lookup" ;;
esac
export RUSTFLAGS
cargo build --release --locked --offline
target=${CARGO_TARGET_DIR:-target}/release
# The library file's name follows the host: lib<name>.so, lib<name>.dylib.
built=$(find "$target" -maxdepth 1 -name 'libthinkthen_ruby.*' ! -name '*.d' ! -name '*.rlib' | head -n 1)
dlext=$("$RUBY" -e 'print RbConfig::CONFIG["DLEXT"]')
mkdir -p lib/thinkthen
rm -f lib/thinkthen/thinkthen.*
cp -- "$built" "lib/thinkthen/thinkthen.$dlext"
if [ "$(uname -s)" = Darwin ]; then
  # rustc gives the copied dylib an absolute builder-path install name.
  # A packaged extension must not retain that path in its Mach-O load commands.
  install_name_tool -id "@rpath/thinkthen.$dlext" "lib/thinkthen/thinkthen.$dlext"
fi
rm -f -- *.gem
"$(dirname -- "$RUBY")/gem" build thinkthen.gemspec --silent >/dev/null
