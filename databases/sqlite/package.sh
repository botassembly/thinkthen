#!/usr/bin/env bash
# The SQLite surface's package rehearsal (the brief's item 5): build the
# loadable extension pinned to glibc 2.28, stage it as dist/thinkthen.so —
# the name the slide's `.load ./thinkthen` resolves — install it in a clean
# ubuntu:24.04 container with the stock sqlite3 CLI, run the slide sample
# exactly as drawn, and remove the container. Nothing is published; the
# install is from the local file only.
#
# Why zig: the host build wants glibc 2.39, and the DuckDB rehearsal showed
# what that costs on older distributions; the pin is 2.28 here too.
# RUSTFLAGS carries the host's libsqlite3 for the link step (zig has no
# stub for it); the artifact still demands only libsqlite3.so.0 at runtime,
# which every SQLite host has.
#
# The container is disposable: named dbpkg211-sqlite, removed with
# docker rm -f -v at the end.
set -euo pipefail
cd "$(dirname "$0")"

VERSION=$(cat ../../VERSION)
CONTAINER=dbpkg211-sqlite
IMAGE=ubuntu:24.04
TARGET=x86_64-unknown-linux-gnu.2.28
LIB=target/x86_64-unknown-linux-gnu/release/libthinkthen0.so
SLIDE=tests/slide.sql

cleanup() {
  docker rm -f -v "$CONTAINER" >/dev/null 2>&1 || true
}
trap cleanup EXIT

echo "== sqlite package: build the loadable extension at glibc 2.28"
RUSTFLAGS="-L /usr/lib/x86_64-linux-gnu" cargo zigbuild --release --target "$TARGET" 2>&1 | tail -1
test -s "$LIB"

echo "== sqlite package: stage dist/"
rm -rf dist
mkdir -p dist
cp "$LIB" dist/thinkthen.so
sha256sum dist/thinkthen.so
objdump -T "$LIB" | grep -o 'GLIBC_[0-9.]*' | sort -V | uniq | tail -1

cat > dist/README.md << EOF
# thinkthen for SQLite ${VERSION}

A loadable extension for SQLite 3.41 or newer with visible symbols. The
slide's install line is the whole install:

    .load ./thinkthen

Keep thinkthen.so beside the working directory the CLI loads from, or give
the full path in .load. The extension links the host's own libsqlite3.so.0
for its interrupt call; every SQLite host has it.
EOF

echo "== sqlite package: clean ubuntu:24.04 container with stock sqlite3"
if ! docker image inspect "$IMAGE" >/dev/null 2>&1; then
  docker pull "$IMAGE"
else
  echo "image present: $(docker image inspect --format '{{index .RepoDigests 0}}' "$IMAGE")"
fi
docker rm -f -v "$CONTAINER" >/dev/null 2>&1 || true
docker run -d --name "$CONTAINER" -e ENGINE_NULL=1 "$IMAGE" sleep infinity >/dev/null
docker exec "$CONTAINER" apt-get update -qq >/dev/null
docker exec "$CONTAINER" apt-get install -y -qq --no-install-recommends sqlite3 >/dev/null
docker exec "$CONTAINER" sqlite3 --version
docker exec "$CONTAINER" mkdir -p /pkg
docker cp dist/thinkthen.so "$CONTAINER":/pkg/thinkthen.so
docker cp "$SLIDE" "$CONTAINER":/pkg/slide.sql

echo "== sqlite package: the slide sample, as drawn, in the container"
docker exec -i -w /pkg "$CONTAINER" sqlite3 :memory: < "$SLIDE"

echo "== sqlite package: cleanup"
docker rm -f -v "$CONTAINER"
echo "containers left under dbpkg211-*: $(docker ps -a --filter name=dbpkg211 --format '{{.Names}}' | wc -l)"
trap - EXIT
