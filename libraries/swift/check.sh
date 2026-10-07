#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../.." && pwd)
host_home=$HOME
export CARGO_HOME=${CARGO_HOME:-$host_home/.cargo} RUSTUP_HOME=${RUSTUP_HOME:-$host_home/.rustup}
swift=${THINKTHEN_SWIFT:-$(command -v swift || true)}
swiftc=${THINKTHEN_SWIFTC:-$(command -v swiftc || true)}
for tool in "$swift" "$swiftc"; do [ -x "$tool" ] || exit 77; done
export THINKTHEN_SWIFT="$swift"
for tool in cargo python3 node nm bwrap flock git tar; do command -v "$tool" >/dev/null 2>&1 || exit 77; done
export PATH="$(dirname "$swift"):$(dirname "$swiftc"):$PATH"
lock=${THINKTHEN_HEAVY_LOCK:-${XDG_RUNTIME_DIR:-/tmp}/thinkthen-swift.lock}
if [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ]; then
    THINKTHEN_HEAVY_LOCK_HELD=$lock
    export THINKTHEN_HEAVY_LOCK_HELD
    exec flock -w 180 -E 75 -o "$lock" /bin/sh "$0" "$@"
fi
# ADR 0113: every branch's engines write a scratch usage folder, never the real one.
. "$root/sdlc/scripts/scratch.sh"
usage_home
if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ]; then
    # The replay smoke (ticket 0335): the package source over the installed C door's module.
    smoke_guard
    . "$root/sdlc/scripts/installed.sh"
    scratch_dir smoke
    native_install "$root" "$smoke/native"
    mkdir -p "$smoke/CThinkThen/include" "$smoke/main"
    cp "$here/Sources/CThinkThen/module.modulemap" "$smoke/CThinkThen/"
    cp "$smoke/native/include/thinkthen.h" "$smoke/CThinkThen/include/"
    cp "$here/Tests/fixtures/smoke.swift" "$smoke/main/main.swift"
    HOME="$smoke" SWIFTPM_MODULECACHE_OVERRIDE="$smoke/module-cache" "$swiftc" -j 2 -module-cache-path "$smoke/module-cache" \
        -I "$smoke/CThinkThen" "$here/Sources/ThinkThen/ThinkThen.swift" "$here/Sources/ThinkThen/Complete.swift" "$smoke/main/main.swift" -L "$smoke/native/lib" \
        -lthinkthen -Xlinker -rpath -Xlinker "$smoke/native/lib" -o "$smoke/smoke"
    "$smoke/smoke"
    exit
fi
mkdir -p "$here/target/native/lib" "$here/target/scratch/matrix-main" "$here/target/home" "$here/target/cache" "$here/target/logs"
node "$root/sdlc/scripts/ratchet.mjs" "$here/ratchet.swift.json"
node "$root/sdlc/scripts/ratchet.mjs" "$here/ratchet.py.json"
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    [ -f "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'Swift installed: C archive missing' >&2; exit 1; }
    . "$root/sdlc/scripts/scratch.sh"
    . "$root/sdlc/scripts/installed.sh"
    installed_unpack
    wrapper=$scratch
    THINKTHEN_ARTIFACT=$THINKTHEN_C_ARTIFACT
    installed_unpack
    native=$scratch
    cmp "$wrapper/Sources/CThinkThen/include/thinkthen.h" "$native/include/thinkthen.h"
    python3 "$root/sdlc/scripts/check-c-exports.py" "$native/include/thinkthen.h" "$native/lib/libthinkthen.so"
    tar -xOzf "$THINKTHEN_C_ARTIFACT" ./include/thinkthen.h | cmp - "$native/include/thinkthen.h"
    tar -xOzf "$THINKTHEN_C_ARTIFACT" ./lib/libthinkthen.so | cmp - "$native/lib/libthinkthen.so"
    tar -xOzf "$THINKTHEN_C_ARTIFACT" ./lib/libthinkthen.a | cmp - "$native/lib/libthinkthen.a"
    export HOME="$here/target/home" XDG_CACHE_HOME="$here/target/cache" SWIFTPM_MODULECACHE_OVERRIDE="$here/target/cache/module-cache"
    "$swift" build --package-path "$wrapper" --scratch-path "$here/target/scratch/swift-build-release" \
        --jobs 2 -Xlinker -L -Xlinker "$native/lib" -Xlinker -rpath -Xlinker "$native/lib"
    RUSTC_WRAPPER= CARGO_NET_OFFLINE=true cargo build --locked --offline --manifest-path "$root/Cargo.toml" --package conformance-backend -j2
    THINKTHEN_PORTABLE_SWIFT_SOURCE="$wrapper" THINKTHEN_NATIVE_ROOT="$native" \
        python3 "$here/Tests/fixtures/portable_batch.py"
    echo 'Swift installed release PASS: three literal portable sends'
    exit 0
fi
# SwiftPM runs no copy step for a system library, so the check copies the one C header in.
mkdir -p "$here/Sources/CThinkThen/include"
cp "$root/libraries/c/include/thinkthen.h" "$here/Sources/CThinkThen/include/thinkthen.h"
case ${THINKTHEN_FOCUSED:-} in
    portable-batch) python3 "$here/Tests/fixtures/portable_batch.py"; exit 0 ;;
    '') ;;
    *) echo "Swift: unknown focused selector: $THINKTHEN_FOCUSED" >&2; exit 2 ;;
esac
RUSTC_WRAPPER= CARGO_NET_OFFLINE=true cargo build --manifest-path "$root/libraries/c/Cargo.toml" --locked --offline --lib -j2
native="$root/libraries/c/target/debug/libthinkthen_c.so"
python3 "$root/sdlc/scripts/check-c-exports.py" "$root/libraries/c/include/thinkthen.h" "$native"
cp "$native" "$here/target/native/lib/libthinkthen.so"
ln -sf libthinkthen.so "$here/target/native/lib/libthinkthen.so.0"
export HOME="$here/target/home" XDG_CACHE_HOME="$here/target/cache" SWIFTPM_MODULECACHE_OVERRIDE="$here/target/cache/module-cache"
export THINKTHEN_SWIFT_BUILD_DIR="$here/target/scratch/swift-build-product"
"$swift" build --package-path "$here" --scratch-path "$THINKTHEN_SWIFT_BUILD_DIR" --jobs 2 -Xlinker -L -Xlinker "$here/target/native/lib" -Xlinker -rpath -Xlinker "$here/target/native/lib"
cp "$here/Tests/fixtures/matrix.swift" "$here/target/scratch/matrix-main/main.swift"
"$swiftc" -j 2 -I "$here/Sources/CThinkThen" "$here/Sources/ThinkThen/ThinkThen.swift" "$here/Sources/ThinkThen/Complete.swift" "$here/target/scratch/matrix-main/main.swift" -L "$here/target/native/lib" -lthinkthen -Xlinker -rpath -Xlinker "$here/target/native/lib" -o "$here/target/scratch/swift-matrix"
cp "$here/Tests/fixtures/settings.swift" "$here/target/scratch/matrix-main/main.swift"
"$swiftc" -j 2 -I "$here/Sources/CThinkThen" "$here/Sources/ThinkThen/ThinkThen.swift" "$here/Sources/ThinkThen/Complete.swift" "$here/target/scratch/matrix-main/main.swift" -L "$here/target/native/lib" -lthinkthen -Xlinker -rpath -Xlinker "$here/target/native/lib" -o "$here/target/scratch/swift-settings"
"$swiftc" -j 2 "$here/Sources/ThinkThen/Complete.swift" "$here/Tests/Carriers/main.swift" -o "$here/target/scratch/carriers"
"$here/target/scratch/carriers"
python3 "$here/Tests/fixtures/public_types.py"
python3 "$here/Tests/fixtures/run_matrix.py"
python3 "$here/Tests/fixtures/run_settings.py"
python3 "$here/Tests/fixtures/portable_batch.py"
python3 "$here/Tests/fixtures/packed_negative.py"
python3 "$here/Tests/fixtures/package_local.py"
python3 "$here/Tests/fixtures/guard.py" "$here/target/artifacts/thinkthen-swift-0.0.1.zip"
plant=$(mktemp "$here/target/logs/private-plant-XXXXXX")
printf '%s\n' '-----BEGIN PRIVATE KEY-----' >"$plant"
if python3 "$here/Tests/fixtures/guard.py" "$plant" >"$plant.log" 2>&1; then echo 'Swift privacy plant passed' >&2; exit 1; fi
grep -q 'rejected private byte pattern' "$plant.log"
rm -f "$plant" "$plant.log"
python3 "$here/Tests/fixtures/isolated_consumer.py" alpha "$here/target/logs"
python3 "$here/Tests/fixtures/isolated_consumer.py" beta "$here/target/logs"
echo 'Swift package PASS: public J1, 29 exact bodies, two installed consumers'
