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
cmake_bin=${THINKTHEN_CMAKE_BIN:-$(command -v cmake || true)}
python_bin=${THINKTHEN_PYTHON_BIN:-$(command -v python3 || true)}
for tool in "$cmake_bin" "$python_bin"; do
 case "$tool" in /*) ;; *) echo 'cpp: tool path must be absolute' >&2; exit 77 ;; esac
 [ -x "$tool" ] || { echo 'cpp: missing tool' >&2; exit 77; }
done
scratch_dir output
out=${THINKTHEN_COVERAGE_STAGE:-$output}
mkdir -p "$out"
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
 node "$repo/sdlc/scripts/ratchet.mjs" ratchet.hpp.json
 . "$repo/sdlc/scripts/installed.sh"
 native_install "$repo" "$out/native"
 header=$out/native/include/thinkthen.h
 shared=$out/native/lib/libthinkthen.so
 static=$out/native/lib/libthinkthen.a
fi
THINKTHEN_NATIVE_HEADER="$header" THINKTHEN_NATIVE_SHARED="$shared" "$python_bin" fixtures/exports.py
"$cmake_bin" -S "$wrapper" -B "$out/package-build" -DCMAKE_BUILD_TYPE=Release \
 -DCMAKE_INSTALL_PREFIX="$out/install" -DTHINKTHEN_C_HEADER="$header" -DTHINKTHEN_ASAN=ON \
 -DTHINKTHEN_NATIVE_SHARED="$shared" -DTHINKTHEN_NATIVE_STATIC="$static"
"$cmake_bin" --build "$out/package-build" --parallel 2
"$cmake_bin" --install "$out/package-build"
install_manifest=$out/package-build/install_manifest.txt
"$python_bin" fixtures/guard.py "$out/install" "$install_manifest"
"$python_bin" fixtures/result_construction.py "$out/install"
if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ]; then
 smoke_guard
  "$cmake_bin" -S fixtures/owned -B "$out/owned-shared" -DCMAKE_PREFIX_PATH="$out/install"
 "$cmake_bin" --build "$out/owned-shared" --parallel 2
  "$out/owned-shared/owned_consumer" smoke
 exit
fi
for mode in shared static; do
  "$cmake_bin" -S fixtures/owned -B "$out/owned-$mode" -DCMAKE_PREFIX_PATH="$out/install" \
  -DTHINKTHEN_LINK_STATIC_C="$([ "$mode" = static ] && echo ON || echo OFF)"
 "$cmake_bin" --build "$out/owned-$mode" --parallel 2
  "$python_bin" fixtures/owned_calls.py "$out/install" "$out/owned-$mode/owned_consumer" "$out/owned-cases" "$mode" "$install_manifest"
done
"$python_bin" fixtures/owned_calls.py "$out/install" "$out/owned-shared/owned_consumer" "$out/owned-cases" usage "$install_manifest"
c++ -std=c++17 -Wall -Wextra -Werror -fsanitize=address -I "$out/install/include" fixtures/tests/json_test.cpp "$out/install/lib/libthinkthen_cpp_shared.a" "$out/install/lib/libthinkthen.so.0" -Wl,-rpath,"$out/install/lib" -o "$out/json-test"
"$out/json-test"
for test in result_lifetime answer_test result_concurrency; do
  c++ -std=c++17 -Wall -Wextra -Werror -fsanitize=address -fno-omit-frame-pointer -pthread -I "$out/install/include" "fixtures/tests/$test.cpp" "$out/install/lib/libthinkthen_cpp_shared.a" "$out/install/lib/libthinkthen.so.0" -Wl,-rpath,"$out/install/lib" -o "$out/$test"
  "$out/$test"
done
cargo build --locked --offline --manifest-path "$repo/Cargo.toml" --package conformance-backend -j2
"$python_bin" "$repo/conformance/children/session_cases.py" "$out/owned-shared/session_consumer"
echo 'cpp: pass'
