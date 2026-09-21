#!/usr/bin/env bash
# The C surface's check: build the door's two libraries, run the slide
# sample as drawn with a plain cc, the null suite, the conformance slice
# through ctypes, and the wire twin when the stub is up on this surface's
# port (8216). The wire stub is experiments/205-thinkthen-libs/shared
# running with STUB_PORT=8216 STUB_DELAY_MS=300.
set -euo pipefail
cd "$(dirname "$0")"

echo "== c surface: build the door (libthinkthen.so and libthinkthen.a)"
cargo build --release --quiet
ls -l target/release/libthinkthen.so target/release/libthinkthen.a

echo "== c surface: compile the slide with a plain cc"
mkdir -p build
cc -std=c11 -Wall -Wextra -I../../contract/include examples/slide.c \
    -o build/slide -Ltarget/release -lthinkthen \
    -Wl,-rpath,"$PWD/target/release"

echo "== c surface: the slide, as drawn, on the null backend"
ENGINE_NULL=1 ./build/slide

echo "== c surface: null suite"
ENGINE_NULL=1 cargo test --quiet --test door

echo "== c surface: conformance slice through ctypes"
ENGINE_NULL=1 python3 conformance_driver.py

if curl -sf --max-time 1 http://127.0.0.1:8216/v1/stats >/dev/null 2>&1; then
    echo "== c surface: the slide on the wire, stub on 8216"
    ENGINE_BASE_URL=http://127.0.0.1:8216/v1 ./build/slide
    echo "== c surface: wire suite (a dead address proves the backend kind)"
    cargo test --quiet --test wire
else
    echo "== c surface: wire twin skipped, no stub on 8216"
fi
