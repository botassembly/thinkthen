#!/bin/sh
# Product Go package and installed-consumer gate. The surface rung supplies $1.
set -eu
cd -- "$(dirname -- "$0")"
repo=$(cd ../.. && pwd)
# Source checks use the same bundled link path as the installed Go module.
# Remove only the link made here, then run the existing scratch cleanup.
bundle_source() {
    bundle="$1/native/x86_64-unknown-linux-gnu"
    [ ! -e "$bundle" ] && [ ! -L "$bundle" ] || { echo 'go: bundled source path already exists' >&2; exit 1; }
    mkdir -p "$1/native"
    ln -s "$2" "$bundle"
    trap 'rm "$bundle"; rmdir "${bundle%/*}" 2>/dev/null || true; scratch_clean' EXIT
}

case "${THINKTHEN_TEST_PROFILE:-routine}" in
    routine|full|smoke) ;;
    stress) echo 'go: not run: no stress gate'; exit 77 ;;
    *) echo 'go: unknown profile' >&2; exit 2 ;;
esac
if [ "$(uname -s)" != Linux ] || [ "$(uname -m)" != x86_64 ]; then
    echo 'go: not run: this package gate is proven on Linux x86_64'; exit 77
fi
go_bin=${THINKTHEN_GO_BIN:-$(command -v go || true)}
python_bin=${THINKTHEN_PYTHON_BIN:-$(command -v python3 || true)}
flock_bin=${THINKTHEN_FLOCK_BIN:-$(command -v flock || true)}
for named in "$go_bin" "$python_bin" "$flock_bin"; do
    case "$named" in /*) ;; *) echo "go: not run: tool path is not absolute: $named" >&2; exit 77 ;; esac
    [ -x "$named" ] || { echo "go: not run: tool is unavailable: $named" >&2; exit 77; }
done
for tool in gofmt cargo rustc cc gcc pkg-config nm readelf node git cp ln grep cat mkdir; do
    command -v "$tool" >/dev/null 2>&1 || { echo "go: not run: no $tool" >&2; exit 77; }
done
go_version=$("$go_bin" version)
case $go_version in
    'go version go1.'*.*' linux/amd64') ;;
    *) echo 'go: not run: a stable Go 1.x release is required' >&2; exit 77 ;;
esac
release=${go_version#go version go1.}
minor=${release%%.*}
patch_and_host=${release#*.}
patch=${patch_and_host%% *}
case $minor in
    '' | *[!0-9]*) echo 'go: not run: a stable Go 1.x release is required' >&2; exit 77 ;;
esac
case $patch in
    '' | *[!0-9]*) echo 'go: not run: a stable Go 1.x release is required' >&2; exit 77 ;;
esac
[ "$minor" -ge 22 ] || { echo 'go: not run: Go 1.22 or newer is required' >&2; exit 77; }
if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ]; then
    # The replay smoke (ticket 0335): the package builds against the installed C door alone.
    . "$repo/sdlc/scripts/scratch.sh"
    usage_home
    smoke_guard
    . "$repo/sdlc/scripts/installed.sh"
    scratch_dir smoke
    native_install "$repo" "$smoke/native"
    bundle_source "$PWD" "$smoke/native"
    PKG_CONFIG_PATH="$smoke/native/lib/pkgconfig" GOCACHE="$repo/target/go/cache" GOMODCACHE="$repo/target/go/modcache" \
        GOPROXY=off GOSUMDB=off GOTOOLCHAIN=local CGO_ENABLED=1 CGO_LDFLAGS="-Wl,-rpath,$smoke/native/lib" \
        "$go_bin" build -buildvcs=false -o "$smoke/smoke" ./examples/smoke
    "$smoke/smoke"
    exit
fi
if [ -z "${THINKTHEN_ARTIFACT:-}" ]; then
    "$python_bin" -c 'import jsonschema' || { echo 'go: not run: Python jsonschema is unavailable' >&2; exit 77; }
fi
export THINKTHEN_GO_BIN="$go_bin"
unset THINKTHEN_API_KEY
lock=${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-6.lock}
if [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ]; then
    export THINKTHEN_HEAVY_LOCK_HELD="$lock"
    exec "$flock_bin" -w 180 -o "$lock" /bin/sh "$repo/libraries/go/check.sh" "$@"
fi
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
. "$repo/sdlc/scripts/scratch.sh"
usage_home
export CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    [ -n "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'go: missing C archive' >&2; exit 1; }
    . "$repo/sdlc/scripts/scratch.sh"
    . "$repo/sdlc/scripts/installed.sh"
    installed_unpack
    wrapper=$scratch
    THINKTHEN_ARTIFACT=$THINKTHEN_C_ARTIFACT
    installed_unpack
    native=$scratch
    if [ ! -d "$wrapper/native/x86_64-unknown-linux-gnu" ]; then bundle_source "$wrapper" "$native"; fi
    cp portable_batch_test.go "$wrapper/portable_batch_test.go"
    cargo build --locked --offline --manifest-path "$repo/Cargo.toml" --package conformance-backend -j2
    THINKTHEN_PORTABLE_NATIVE="$native" THINKTHEN_PORTABLE_MODULE="$wrapper" "$python_bin" fixtures/portable_batch.py
    mkdir "$wrapper/consumer"
    cp fixtures/type_case.go fixtures/native_case.go fixtures/native_output.go "$wrapper/consumer/"
    (cd "$wrapper" && PKG_CONFIG_PATH="$native/lib/pkgconfig" CGO_ENABLED=1 GOPROXY=off GOSUMDB=off GOTOOLCHAIN=local \
      CGO_CFLAGS="-I$native/include" CGO_LDFLAGS="-L$native/lib -Wl,-rpath,$native/lib" \
      go build -buildvcs=false -o "$wrapper/type-case" ./consumer)
    THINKTHEN_GO_TYPECASE="$wrapper/type-case" THINKTHEN_GO_NATIVE_LIB="$native/lib" \
      "$python_bin" fixtures/type_cases.py native
    "$python_bin" fixtures/installed_release.py "$wrapper" "$native"
    exit
fi
out=$repo/target/go
mkdir -p "$out/native/include" "$out/native/lib/pkgconfig" "$out/cache" "$out/modcache" "$out/consumers"
node "$repo/sdlc/scripts/ratchet.mjs" ratchet.go.json
node "$repo/sdlc/scripts/ratchet.mjs" ratchet.py.json
"$python_bin" fixtures/guard.py .
mkdir -p "$out/guard-plant"
printf '%s\n' '-----BEGIN PRIVATE KEY----- planted' >"$out/guard-plant/README.md"
if "$python_bin" fixtures/guard.py "$out/guard-plant" >/dev/null 2>&1; then
    echo 'go: private marker plant passed' >&2; exit 1
fi
test -z "$(gofmt -l .)"
cargo build --locked --offline --manifest-path "$repo/libraries/c/Cargo.toml" --lib -j2
cp "$repo/libraries/c/include/thinkthen.h" "$out/native/include/thinkthen.h"
cp "$repo/libraries/c/target/debug/libthinkthen_c.so" "$out/native/lib/libthinkthen.so"
sh "$repo/libraries/c/localize.sh" "$repo/libraries/c/target/debug/libthinkthen_c.a" "$out/native/lib/libthinkthen.a"
ln -sfn libthinkthen.so "$out/native/lib/libthinkthen.so.0"
cat >"$out/native/lib/pkgconfig/thinkthen.pc" <<EOF
prefix=$out/native
libdir=\${prefix}/lib
includedir=\${prefix}/include
Name: thinkthen
Description: ThinkThen C door
Version: 0.0.1
Libs: -L\${libdir} -lthinkthen
Cflags: -I\${includedir}
EOF
"$python_bin" fixtures/abi.py
bundle_source "$PWD" "$out/native"
export PKG_CONFIG_PATH="$out/native/lib/pkgconfig" LD_LIBRARY_PATH="$out/native/lib"
export GOCACHE="$out/cache" GOMODCACHE="$out/modcache" GOPROXY=off GOSUMDB=off GOTOOLCHAIN=local CGO_ENABLED=1
cargo build --locked --offline --manifest-path "$repo/Cargo.toml" --package conformance-backend -j2
THINKTHEN_PORTABLE_NATIVE="$out/native" "$python_bin" fixtures/portable_batch.py
"$go_bin" test -run '^(TestComplete|TestAtomicFixtures|TestRequestsKeep|TestNative)' .
"$go_bin" vet ./...
"$go_bin" build -o "$out/type-case" ./fixtures/type_case.go ./fixtures/native_case.go ./fixtures/native_output.go
"$python_bin" fixtures/packing_negative.py
"$python_bin" fixtures/type_cases.py
"$python_bin" fixtures/run_matrix.py
"$python_bin" fixtures/type_cases.py native
echo 'go: pass'
