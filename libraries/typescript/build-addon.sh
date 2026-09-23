#!/usr/bin/env bash
# Build the Node addon with every path under $HOME remapped to a neutral
# prefix, mirroring the Python surface's wheel build: cargo embeds
# absolute source paths in panic locations, and an artifact that names
# /home/<builder> leaks who built it and where. Every build path — the
# gate's, the packaging rehearsal's — goes through this file so no build
# can forget the remap. Pass --synthetic to arm the compile-time fixture.
set -euo pipefail
cd "$(dirname "$0")"

export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"
# --locked at the call site (surfaces-review-4, item 20): the lockfile is
# the authority the check and the artifact both answer to.
if [ "${1:-}" = "--synthetic" ]; then
  exec node_modules/.bin/napi build --release --platform --features synthetic-partial --cargo-cwd ./addon --cargo-flags=--locked --js loader.cjs --dts loader.d.ts .
fi
exec node_modules/.bin/napi build --release --platform --cargo-cwd ./addon --cargo-flags=--locked --js loader.cjs --dts loader.d.ts .
