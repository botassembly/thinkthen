#!/bin/sh
# Build the addon and copy it to thinkthen-<platform>-<arch>.node, the name
# loader.js picks for this host (ticket 0128). Every path under $HOME is
# remapped, so the artifact never names who built it or where.
set -eu
cd -- "$(dirname -- "$0")"
python3 ../../sdlc/scripts/package-inventory.py npm --write >/dev/null
RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build" \
    cargo build --quiet --release --locked --offline
case $(uname -s) in Darwin) library=libthinkthen_typescript.dylib ;; *) library=libthinkthen_typescript.so ;; esac
cp -- "${CARGO_TARGET_DIR:-target}/release/$library" "$(node -p 'require("./native-platforms.json")[process.platform + "-" + process.arch]')"
