#!/usr/bin/env bash
# The Ruby surface's check: build the extension and the gem inside the
# builder container, run the surface tests and the conformance slice
# offline, run the slide sample, and run the interrupt proof when the stub
# is up on this surface's port (8214).
set -euo pipefail
cd "$(dirname "$0")"

root="$(cd ../.. && pwd)"
image="thinkthen-ruby-builder:local"
stub_url="http://127.0.0.1:8214/v1"

./build.sh

wire=no
if curl -sf --max-time 1 "$stub_url/stats" >/dev/null 2>&1; then
  wire=yes
fi

docker_run() {
  docker run --rm --network host \
    -v "$root":/src \
    -v "$HOME/.rustup":/root/.rustup:ro \
    -w /src/libraries/ruby \
    -e ENGINE_NULL=1 \
    "$image" \
    bash -eu -c "export PATH=/root/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/bin:\$PATH CARGO_HOME=/src/libraries/ruby/.runtimes/cargo; $1"
}

echo "== ruby surface: the defect kind maps to its error class (shim unit test)"
docker_run 'cargo test --quiet --lib'

echo "== ruby surface: surface tests, null backend"
docker_run 'ruby -I lib -I tests tests/test_surface.rb'

echo "== ruby surface: fast-backend cancel, the poll-bug shape"
docker_run 'ruby -I lib tests/test_cancel_fast.rb'
docker_run 'ruby -I lib tests/test_fork.rb'

echo "== ruby surface: the function examples"
docker_run 'ruby -I lib tests/examples.rb'

echo "== ruby surface: conformance slice, offline"
docker_run 'ruby -I lib tests/conformance.rb'

echo "== ruby surface: slide sample, as drawn"
docker_run 'ruby -I lib tests/slide_sample.rb'

if [ "$wire" = yes ]; then
  echo "== ruby surface: interrupt proof on the wire"
  docker run --rm --network host \
    -v "$root":/src \
    -v "$HOME/.rustup":/root/.rustup:ro \
    -w /src/libraries/ruby \
    -e ENGINE_BASE_URL="$stub_url" \
    -e ENGINE_WIDTH=8 \
    "$image" \
    bash -eu -c 'ruby -I lib tests/test_cancel.rb'
else
  echo "== ruby surface: interrupt proof skipped, no stub on 8214"
fi
