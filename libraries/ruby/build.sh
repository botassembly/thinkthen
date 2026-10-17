#!/bin/sh
# Build both pinned Ruby extensions into one platform gem.
# Every cargo call runs --locked --offline.
set -eu
cd -- "$(dirname -- "$0")"
# Keep direct gem builds clean too. release-pack may already pass the same
# remaps; repeating them preserves the direct route without replacing its flags.
repo=$(CDPATH='' cd ../.. && pwd)
cargo_home=${CARGO_HOME:-$HOME/.cargo}
if [ "${THINKTHEN_FULL_COMPILER:-}" != 1 ]; then
RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$repo=/build/source --remap-path-prefix=$cargo_home=/build/cargo --remap-path-prefix=/home=/build/host --remap-path-prefix=/Users=/build/users"
fi
# RUSTFLAGS from the environment replaces .cargo/config.toml target rustflags.
# Preserve the Darwin extension's load-time Ruby symbol lookup when remapping.
case $(uname -s) in
  Darwin) RUSTFLAGS="$RUSTFLAGS -C link-arg=-Wl,-undefined,dynamic_lookup" ;;
esac
export RUSTFLAGS
. ./toolchain.env
base_target=${CARGO_TARGET_DIR:-$PWD/target}
case $base_target in /*) ;; *) base_target=$PWD/$base_target ;; esac
for THINKTHEN_RUBY_MINOR in $RUBY_MINORS; do
export THINKTHEN_RUBY_MINOR
. ./toolchain.env
prefix=${THINKTHEN_TOOLCHAINS:-$HOME/.cache/thinkthen-toolchains}/ruby/$RUBY_VERSION
[ "$(uname -s)" != Darwin ] || prefix=$prefix-$(uname -m)-darwin
unset RBCONFIG
RUBY=$prefix/bin/ruby
[ -x "$RUBY" ] || { echo "build ruby: run setup-ruby.sh for Ruby $RUBY_VERSION" >&2; exit 77; }
CARGO_TARGET_DIR=$base_target/ruby-$THINKTHEN_RUBY_MINOR
export RUBY CARGO_TARGET_DIR
cargo build --release --locked --offline
target=$CARGO_TARGET_DIR/release
# The library file's name follows the host: lib<name>.so, lib<name>.dylib.
built=$(find "$target" -maxdepth 1 -name 'libthinkthen_ruby.*' ! -name '*.d' ! -name '*.rlib' | head -n 1)
dlext=$("$RUBY" -e 'print RbConfig::CONFIG["DLEXT"]')
extension=lib/thinkthen/$THINKTHEN_RUBY_MINOR/thinkthen.$dlext
mkdir -p "$(dirname "$extension")"
cp -- "$built" "$extension"
if [ "$(uname -s)" = Darwin ]; then
  # rustc gives the copied dylib an absolute builder-path install name.
  # A packaged extension must not retain that path in its Mach-O load commands.
  install_name_tool -id "@rpath/thinkthen.$dlext" "$extension"
fi
done
rm -f -- *.gem
THINKTHEN_RUBY_FALLBACK=0 "$(dirname -- "$RUBY")/gem" build thinkthen.gemspec --silent >/dev/null
THINKTHEN_RUBY_FALLBACK=1 "$(dirname -- "$RUBY")/gem" build thinkthen.gemspec --silent >/dev/null
