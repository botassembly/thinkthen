#!/bin/sh
# Exercise the installed owned API; complete shared parity is release-only.
set -eu
cd -- "$(dirname -- "$0")"
repo=$(cd ../.. && pwd)
case "${THINKTHEN_TEST_PROFILE:-routine}" in
 routine|full|smoke) ;; stress) echo 'cpp: not run: no stress gate'; exit 77 ;;
 *) echo 'cpp: unknown profile' >&2; exit 2 ;;
esac
. "$repo/sdlc/scripts/scratch.sh"
usage_home
export CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
lock=${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-6.lock}
if [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ]; then
 export THINKTHEN_HEAVY_LOCK_HELD="$lock"
 exec flock -w 180 -o "$lock" /bin/sh "$repo/libraries/cpp/check.sh" "$@"
fi
cmake_bin=${THINKTHEN_CMAKE_BIN:-$(command -v cmake || true)}
python_bin=${THINKTHEN_PYTHON_BIN:-$(command -v python3 || true)}
for tool in "$cmake_bin" "$python_bin"; do
 case "$tool" in /*) ;; *) echo 'cpp: tool path must be absolute' >&2; exit 77 ;; esac
 [ -x "$tool" ] || { echo 'cpp: missing tool' >&2; exit 77; }
done
scratch_dir output
out=$output
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
 [ -n "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'cpp: missing matching C archive' >&2; exit 1; }
 . "$repo/sdlc/scripts/installed.sh"
 installed_unpack; wrapper=$scratch
 THINKTHEN_ARTIFACT=$THINKTHEN_C_ARTIFACT
 installed_unpack; native=$scratch
 header=$native/include/thinkthen.h
 shared=$native/lib/libthinkthen.so
 static=$native/lib/libthinkthen.a
else
 wrapper=$repo/libraries/cpp
 "$python_bin" "$repo/sdlc/generators/results/generate.py" --target cpp --check
 "$python_bin" "$repo/sdlc/generators/results/generate.py" --target cpp --inputs --check
 for kind in cpp hpp py; do node "$repo/sdlc/scripts/ratchet.mjs" "ratchet.$kind.json"; done
 export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"
 cargo build --locked --offline --manifest-path "$repo/libraries/c/Cargo.toml" --lib -j2
 header=$repo/libraries/c/include/thinkthen.h
 shared=$repo/libraries/c/target/debug/libthinkthen_c.so
 static=$out/libthinkthen.a
 sh "$repo/libraries/c/localize.sh" "$repo/libraries/c/target/debug/libthinkthen_c.a" "$static"
fi
THINKTHEN_NATIVE_HEADER="$header" THINKTHEN_NATIVE_SHARED="$shared" "$python_bin" fixtures/exports.py
"$cmake_bin" -S "$wrapper" -B "$out/package-build" -DCMAKE_BUILD_TYPE=Release \
 -DCMAKE_INSTALL_PREFIX="$out/install" -DTHINKTHEN_C_HEADER="$header" \
 -DTHINKTHEN_NATIVE_SHARED="$shared" -DTHINKTHEN_NATIVE_STATIC="$static"
"$cmake_bin" --build "$out/package-build" --parallel 2
"$cmake_bin" --install "$out/package-build"
"$python_bin" fixtures/guard.py "$out/install"
for mode in shared static; do
 "$cmake_bin" -S fixtures/owned -B "$out/owned-$mode" -DCMAKE_PREFIX_PATH="$out/install" \
  -DTHINKTHEN_LINK_STATIC_C="$([ "$mode" = static ] && echo ON || echo OFF)"
 "$cmake_bin" --build "$out/owned-$mode" --parallel 2
 "$python_bin" fixtures/owned_calls.py "$out/install" "$out/owned-$mode/owned_consumer" "$out/owned-cases" "$mode"
done
"$python_bin" fixtures/owned_calls.py "$out/install" "$out/owned-shared/owned_consumer" "$out/owned-cases" usage
c++ -std=c++17 -Wall -Wextra -Werror -I "$out/install/include" fixtures/tests/json_test.cpp -o "$out/json-test"
"$out/json-test"
cargo build --locked --offline --manifest-path "$repo/Cargo.toml" --package conformance-backend -j2
"$python_bin" fixtures/session_cases.py "$out/owned-shared/session_consumer"
echo 'cpp: pass'
