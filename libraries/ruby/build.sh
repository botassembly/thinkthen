#!/usr/bin/env bash
# Build the Ruby extension and the gem inside the builder container.
#
# No host install happens: the builder image carries its own pinned Rust
# toolchain with clippy and rustfmt (the old host ~/.rustup mount could
# not work from a Mac and is gone), and the built .so lands only inside
# this folder.
#
# `build.sh synthetic` arms the stand-in's synthesized partial failure (a
# compile-time feature), the build the gate runs so conformance case 74
# and the marker test can run. The plain build, the one a package is made
# from, never carries the fixture.
set -euo pipefail
cd "$(dirname "$0")"

root="$(cd ../.. && pwd)"
image="thinkthen-ruby-builder:local"
stamp=".runtimes/builder-image.built"
features=""
if [ "${1:-}" = "synthetic" ]; then
  features="--features synthetic-partial"
fi

# Rebuild when the Dockerfile is newer than the image: a stale image
# ignores image changes (surfaces-review-4) and silently keeps whatever
# toolchain it froze.
if ! docker image inspect "$image" >/dev/null 2>&1 || [ Dockerfile -nt "$stamp" ]; then
  echo "building $image from Dockerfile (removed with: docker rmi $image)"
  docker build -q -t "$image" -f Dockerfile ../ruby >/dev/null
  touch "$stamp"
fi

docker run --rm \
  -v "$root":/src \
  -w /src/libraries/ruby \
  "$image" \
  bash -eu -c "
    # The container builds offline: every crate the lockfile names lives
    # in CARGO_HOME, inside the mounted repository, so no build needs the
    # network. The first pull is the one network step: on a fresh clone,
    # run `cargo fetch --locked` in this folder once, which populates
    # .runtimes/cargo; every later build — here and in check.sh — passes
    # --offline and needs none. --locked refuses lockfile drift the gem
    # would otherwise swallow silently.
    export CARGO_HOME=/src/libraries/ruby/.runtimes/cargo
    mkdir -p lib/thinkthen
    cargo build --release --locked --offline $features
    cp target/release/libthinkthen_native.so lib/thinkthen/thinkthen.so
    gem build thinkthen.gemspec --silent >/dev/null
  "

echo "built: $(ls -1 *.gem 2>/dev/null | tail -1) and lib/thinkthen/thinkthen.so"
