#!/bin/sh
# Build the addon and copy it to thinkthen.node. Every path under $HOME is
# remapped, so the artifact never names who built it or where.
set -eu
cd -- "$(dirname -- "$0")"
RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build" \
    cargo build --quiet --release --locked --offline
cp -- "${CARGO_TARGET_DIR:-target}/release/libthinkthen_typescript.so" thinkthen.node
