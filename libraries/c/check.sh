#!/usr/bin/env bash
# The C surface's check: build the door's two libraries, run the slide
# sample as drawn with a plain cc, the null suite, the conformance slice
# through ctypes, and the wire twin when the stub is up on this surface's
# port (8216). The wire stub is the in-repo tools/wire-stub, which
# scripts/check_surfaces.sh builds and starts with STUB_PORT=8216
# STUB_DELAY_MS=300.
set -euo pipefail
cd "$(dirname "$0")"

# Experimental on macOS: the library spelling below takes the Darwin form
# (dylib); Linux is the gate's platform.
case "$(uname -s)" in
  Darwin) LIB_EXT=dylib ;;
  *)      LIB_EXT=so ;;
esac

echo "== c surface: build the door (libthinkthen.$LIB_EXT and libthinkthen.a)"
# Every release artifact remaps the builder's home to a neutral prefix
# (surfaces-review-4: the built libraries carried 141 and 152 home-path
# strings; cargo embeds absolute source paths in panic locations).
export RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=$HOME=/build"
# The C-compiled dependency objects (ring) answer to the C compiler, not
# the Rust remap; the same neutral prefix goes to both.
export CFLAGS="-ffile-prefix-map=$HOME=/build ${CFLAGS:-}"
cargo build --release --quiet --locked
ls -l target/release/libthinkthen.$LIB_EXT target/release/libthinkthen.a

echo "== c surface: compile the slide with a plain cc"
mkdir -p build
cc -std=c11 -Wall -Wextra -I../../contract/include examples/slide.c \
    -o build/slide -Ltarget/release -lthinkthen \
    -Wl,-rpath,"$PWD/target/release"

echo "== c surface: the slide, as drawn, on the null backend"
THINKTHEN_NULL=1 ./build/slide

echo "== c surface: free from an atexit handler, after thread-local teardown"
cc -std=c11 -Wall -Wextra -I../../contract/include tests/atexit_free.c \
    -o build/atexit_free -Ltarget/release -lthinkthen \
    -Wl,-rpath,"$PWD/target/release"
THINKTHEN_NULL=1 ./build/atexit_free

echo "== c surface: compile the recognize and relate example with a plain cc"
cc -std=c11 -Wall -Wextra -I../../contract/include examples/recognize.c \
    -o build/recognize -Ltarget/release -lthinkthen \
    -Wl,-rpath,"$PWD/target/release"

echo "== c surface: the deck's recognize and relate sections, as drawn"
THINKTHEN_NULL=1 ./build/recognize

if command -v clang >/dev/null 2>&1; then
    echo "== c surface: the leak check (address sanitizer) over the two new strings"
    clang -std=c11 -Wall -Wextra -fsanitize=address -I../../contract/include \
        examples/recognize.c -o build/recognize_asan -Ltarget/release \
        -lthinkthen -Wl,-rpath,"$PWD/target/release"
    THINKTHEN_NULL=1 ASAN_OPTIONS=detect_leaks=1 ./build/recognize_asan >/dev/null
    echo "asan and lsan clean"

    echo "== c surface: the per-thread error slot under the sanitizer (the review's repro)"
    clang -std=c11 -Wall -Wextra -fsanitize=address -pthread -I../../contract/include \
        tests/error_threads.c -o build/error_threads -Ltarget/release \
        -lthinkthen -Wl,-rpath,"$PWD/target/release"
    THINKTHEN_NULL=1 ASAN_OPTIONS=detect_leaks=1 ./build/error_threads
else
    echo "== c surface: leak check skipped, no clang"
fi

echo "== c surface: null suite (the door, the error surface, and the twins)"
# The annotate partial-failure fixture is compiled in only for a build that
# asks for it (standin/Cargo.toml, the `synthetic-partial` feature): the
# door's shape test and conformance case 74 replay that record. The release
# library the ctypes driver loads is rebuilt with the same feature, then
# restored to the production shape below.
cargo build --release --quiet --features synthetic-partial --locked
THINKTHEN_NULL=1 cargo test --quiet --features synthetic-partial --lib --test door --locked -- --test-threads=1

echo "== c surface: the null and length matrix"
THINKTHEN_NULL=1 cargo test --quiet --test null_matrix --locked -- --test-threads=1

echo "== c surface: the cancel token, fired and fired across threads"
THINKTHEN_NULL=1 cargo test --quiet --test cancel --locked -- --test-threads=1

echo "== c surface: fast-backend deadline, the poll-bug shape"
THINKTHEN_NULL=1 cargo test --quiet --test deadline_fast --locked -- --test-threads=1

echo "== c surface: four threads over one engine"
THINKTHEN_NULL=1 cargo test --quiet --test concurrency --locked

echo "== c surface: two threads read their own error messages"
THINKTHEN_NULL=1 cargo test --quiet --test error_threads --locked

echo "== c surface: failures stay on their own engine, and a reused address starts clean"
cargo test --quiet --test error_engines --locked -- --test-threads=1

echo "== c surface: fork after the first call answers in the child"
THINKTHEN_NULL=1 cargo test --quiet --test fork --locked -- --test-threads=1

echo "== c surface: the function examples"
python3 examples.py

echo "== c surface: conformance slice through ctypes"
THINKTHEN_NULL=1 python3 conformance_driver.py

echo "== c surface: back to the production shape (no fixture code compiled)"
cargo build --release --quiet --locked

if curl -sf --max-time 1 http://127.0.0.1:8216/v1/stats >/dev/null 2>&1; then
    echo "== c surface: the slide on the wire, stub on 8216"
    ENGINE_BASE_URL=http://127.0.0.1:8216/v1 ./build/slide
    echo "== c surface: wire suite (a dead address proves the backend kind)"
    cargo test --quiet --test wire --locked
else
    echo "== c surface: wire twin skipped, no stub on 8216"
fi
