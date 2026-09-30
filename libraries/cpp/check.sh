#!/bin/sh
# Product C++ CMake package and installed-consumer gate. The surface rung supplies $1.
set -eu
cd -- "$(dirname -- "$0")"
repo=$(cd ../.. && pwd)
case "${THINKTHEN_TEST_PROFILE:-routine}" in
    routine|full) ;;
    stress) echo 'cpp: not run: no stress gate'; exit 77 ;;
    *) echo 'cpp: unknown profile' >&2; exit 2 ;;
esac
if [ "$(uname -s)" != Linux ] || [ "$(uname -m)" != x86_64 ]; then
    echo 'cpp: not run: this package gate is proven on Linux x86_64'; exit 77
fi
cmake_bin=${THINKTHEN_CMAKE_BIN:-$(command -v cmake || true)}
python_bin=${THINKTHEN_PYTHON_BIN:-$(command -v python3 || true)}
flock_bin=${THINKTHEN_FLOCK_BIN:-$(command -v flock || true)}
for named in "$cmake_bin" "$python_bin" "$flock_bin"; do
    case "$named" in /*) ;; *) echo "cpp: not run: tool path is not absolute: $named" >&2; exit 77 ;; esac
    [ -x "$named" ] || { echo "cpp: not run: tool is unavailable: $named" >&2; exit 77; }
done
for tool in cargo rustc c++ clang++ make nm readelf ldd node git cp cmp grep mkdir; do
    command -v "$tool" >/dev/null 2>&1 || { echo "cpp: not run: no $tool" >&2; exit 77; }
done
if [ -z "${THINKTHEN_ARTIFACT:-}" ]; then
    "$python_bin" -c 'import jsonschema' || { echo 'cpp: not run: Python jsonschema is unavailable' >&2; exit 77; }
fi
unset THINKTHEN_API_KEY
lock=${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-6.lock}
if [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ]; then
    export THINKTHEN_HEAVY_LOCK_HELD="$lock"
    exec "$flock_bin" -w 180 -o "$lock" /bin/sh "$repo/libraries/cpp/check.sh" "$@"
fi
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
. "$repo/sdlc/scripts/scratch.sh"
usage_home
export CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    [ -n "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'cpp: missing C archive' >&2; exit 1; }
    . "$repo/sdlc/scripts/scratch.sh"
    . "$repo/sdlc/scripts/installed.sh"
    installed_unpack
    wrapper=$scratch
    THINKTHEN_ARTIFACT=$THINKTHEN_C_ARTIFACT
    installed_unpack
    native=$scratch
    cargo build --locked --offline --manifest-path "$repo/Cargo.toml" --package conformance-backend -j2
    THINKTHEN_PORTABLE_NATIVE="$native" THINKTHEN_PORTABLE_CPP_INCLUDE="$wrapper/include" "$python_bin" fixtures/portable_batch.py
    "$python_bin" fixtures/installed_release.py "$wrapper" "$native"
    exit
fi
out=$repo/target/cpp
mkdir -p "$out/logs" "$out/guard-plant" "$out/consumer-source" \
    "$out/fixture-root/specification/fixtures/types"
cp -a fixtures/tests/. "$out/consumer-source/"
cp "$repo/specification/fixtures/types/corpus.json" \
    "$out/fixture-root/specification/fixtures/types/corpus.json"
export CPP_CORPUS_ROOT="$out/fixture-root"
node "$repo/sdlc/scripts/ratchet.mjs" ratchet.cpp.json
node "$repo/sdlc/scripts/ratchet.mjs" ratchet.hpp.json
node "$repo/sdlc/scripts/ratchet.mjs" ratchet.py.json
"$python_bin" fixtures/guard.py .
printf '%s\n' '-----BEGIN PRIVATE KEY----- planted' >"$out/guard-plant/README.md"
if "$python_bin" fixtures/guard.py "$out/guard-plant" >/dev/null 2>&1; then
    echo 'cpp: private marker plant passed' >&2; exit 1
fi
cargo build --locked --offline --manifest-path "$repo/libraries/c/Cargo.toml" --lib -j2
"$python_bin" fixtures/exports.py
header=$repo/libraries/c/include/thinkthen.h
shared=$repo/libraries/c/target/debug/libthinkthen_c.so
static=$out/libthinkthen.a
sh "$repo/libraries/c/localize.sh" "$repo/libraries/c/target/debug/libthinkthen_c.a" "$static"
"$cmake_bin" -S . -B "$out/package-build" -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_INSTALL_PREFIX="$out/install" -DTHINKTHEN_C_HEADER="$header" \
    -DTHINKTHEN_NATIVE_SHARED="$shared" -DTHINKTHEN_NATIVE_STATIC="$static"
"$cmake_bin" --build "$out/package-build" --parallel 2
"$cmake_bin" --install "$out/package-build"
cmp "$header" "$out/install/include/thinkthen/thinkthen.h"
cmp "$shared" "$out/install/lib/libthinkthen.so.0"
cmp "$static" "$out/install/lib/libthinkthen.a"
cargo build --locked --offline --manifest-path "$repo/Cargo.toml" --package conformance-backend -j2
THINKTHEN_PORTABLE_NATIVE="$out/install" "$python_bin" fixtures/portable_batch.py
for mode in shared static; do
    "$cmake_bin" -S "$out/consumer-source/$mode" -B "$out/installed-$mode-build" \
        -DCMAKE_PREFIX_PATH="$out/install" -DCMAKE_BUILD_TYPE=Release
    "$cmake_bin" --build "$out/installed-$mode-build" --parallel 2
done
ldd "$out/installed-shared-build/consumer" | grep -q "libthinkthen.so.0.*$out/install/lib"
if ldd "$out/installed-static-build/consumer" | grep -q 'libthinkthen.so.0'; then
    echo 'cpp: static-C consumer loaded shared ThinkThen' >&2; exit 1
fi
c++ -std=c++17 -Wall -Wextra -Werror -I "$out/install/include" \
    fixtures/tests/json_test.cpp -o "$out/json-test"
"$out/json-test"
c++ -std=c++17 -Wall -Wextra -Werror -I "$out/install/include" \
    fixtures/type_case.cpp -L "$out/install/lib" -l:libthinkthen.so.0 \
    -Wl,-rpath,"$out/install/lib" -o "$out/type-case"
"$python_bin" fixtures/type_cases.py
for mode in shared static; do
    CPP_CONSUMER="$out/installed-$mode-build/consumer" "$python_bin" fixtures/run.py
    CPP_CONSUMER="$out/installed-$mode-build/consumer" "$python_bin" fixtures/plant-check.py
done
"$cmake_bin" -S . -B "$out/multilib-package-build" -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_INSTALL_LIBDIR=lib/x86_64-linux-gnu -DCMAKE_INSTALL_PREFIX="$out/install-multilib" \
    -DTHINKTHEN_C_HEADER="$header" -DTHINKTHEN_NATIVE_SHARED="$shared" -DTHINKTHEN_NATIVE_STATIC="$static"
"$cmake_bin" --build "$out/multilib-package-build" --parallel 2
"$cmake_bin" --install "$out/multilib-package-build"
"$cmake_bin" -S "$out/consumer-source/shared" -B "$out/installed-multilib-build" \
    -DCMAKE_PREFIX_PATH="$out/install-multilib" -DCMAKE_BUILD_TYPE=Release
"$cmake_bin" --build "$out/installed-multilib-build" --parallel 2
ldd "$out/installed-multilib-build/consumer" | grep -q "libthinkthen.so.0.*$out/install-multilib/lib/x86_64-linux-gnu"
CPP_CONSUMER="$out/installed-multilib-build/consumer" "$python_bin" fixtures/run.py
clang++ -std=c++17 -Wall -Wextra -Werror -O1 -g -fsanitize=address,undefined \
    -fno-omit-frame-pointer -I "$out/install/include" fixtures/tests/json_test.cpp -o "$out/json-sanitized"
"$out/json-sanitized"
"$cmake_bin" -S "$out/consumer-source/shared" -B "$out/installed-sanitized-build" \
    -DCMAKE_CXX_COMPILER=clang++ -DCMAKE_PREFIX_PATH="$out/install" -DCMAKE_BUILD_TYPE=Debug \
    -DCMAKE_CXX_FLAGS='-fsanitize=address,undefined -fno-omit-frame-pointer' \
    -DCMAKE_EXE_LINKER_FLAGS='-fsanitize=address,undefined'
"$cmake_bin" --build "$out/installed-sanitized-build" --parallel 2
CPP_CONSUMER="$out/installed-sanitized-build/consumer" "$python_bin" fixtures/run.py
echo 'cpp: pass'
