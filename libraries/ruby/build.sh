#!/usr/bin/env bash
# Build the Ruby extension and the gem inside the builder container.
#
# No host install happens: the Rust toolchain is mounted read-only from the
# host (glibc-compatible: the builder is Debian trixie, the host Ubuntu
# 24.04), and the built .so lands only inside this folder.
#
# `build.sh synthetic` arms the stand-in's synthesized partial failure (a
# compile-time feature), the build the gate runs so conformance case 74
# and the marker test can run. The plain build, the one a package is made
# from, never carries the fixture.
set -euo pipefail
cd "$(dirname "$0")"

root="$(cd ../.. && pwd)"
image="thinkthen-ruby-builder:local"
features=""
if [ "${1:-}" = "synthetic" ]; then
  features="--features synthetic-partial"
fi

if ! docker image inspect "$image" >/dev/null 2>&1; then
  echo "building $image from Dockerfile (removed with: docker rmi $image)"
  docker build -q -t "$image" -f Dockerfile ../ruby >/dev/null
fi

docker run --rm \
  -v "$root":/src \
  -v "$HOME/.rustup":/root/.rustup:ro \
  -w /src/libraries/ruby \
  "$image" \
  bash -eu -c "
    export PATH=/root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:\$PATH CARGO_HOME=/src/libraries/ruby/.runtimes/cargo
    mkdir -p lib/thinkthen
    cargo build --release $features
    cp target/release/libthinkthen_native.so lib/thinkthen/thinkthen.so
    gem build thinkthen.gemspec --silent >/dev/null
  "

echo "built: $(ls -1 *.gem 2>/dev/null | tail -1) and lib/thinkthen/thinkthen.so"
