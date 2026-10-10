#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../.." && pwd)
swift=${THINKTHEN_SWIFT:-$(command -v swift || true)}
[ -x "$swift" ] || exit 77
case $(uname -s) in
Darwin)
    [ -f "${THINKTHEN_ARTIFACT:-}" ] || { echo 'Swift Apple checks require the final installed package; no native build is started' >&2; exit 1; }
    . "$root/sdlc/scripts/installed.sh"
    . "$root/sdlc/scripts/scratch.sh"
    installed_unpack
    [ -f "$scratch/CThinkThen.xcframework/Info.plist" ] || { echo 'Swift Apple package is missing its XCFramework' >&2; exit 1; }
    scratch_dir apple_consumer
    python3 "$here/Tests/fixtures/installed.py" --package "$scratch" --out "$apple_consumer"
    if [ "${THINKTHEN_TEST_PROFILE:-}" = full ]; then
        python3 "$here/Tests/fixtures/complete_parity.py" --binary "$apple_consumer/installed/Parity"
    else
        python3 "$here/Tests/fixtures/complete_parity.py" --binary "$apple_consumer/installed/Parity" --routine
    fi
    exit 0 ;;
Linux) ;;
*) echo 'Swift installed checks require Linux or macOS' >&2; exit 77 ;;
esac
lock=${THINKTHEN_HEAVY_LOCK:-${XDG_RUNTIME_DIR:-/tmp}/thinkthen-swift.lock}
if [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ]; then
    export THINKTHEN_HEAVY_LOCK_HELD=$lock
    exec flock -w 180 -E 75 -o "$lock" /bin/sh "$0" "$@"
fi
. "$root/sdlc/scripts/scratch.sh"
node "$root/sdlc/scripts/ratchet.mjs" "$here/ratchet.swift.json"
node "$root/sdlc/scripts/ratchet.mjs" "$here/ratchet.py.json"
for mode in '' --inputs --bridge; do
    python3 -I "$root/sdlc/generators/results/generate.py" --target swift ${mode:+"$mode"} --check
done
if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ]; then
    smoke_guard
    . "$root/sdlc/scripts/installed.sh"
    scratch_dir smoke
    native_install "$root" "$smoke/native"
    mkdir -p "$smoke/CThinkThen/include"
    cp "$here/Sources/CThinkThen/module.modulemap" "$smoke/CThinkThen/"
    cp "$here/Sources/CThinkThen/include/loader.h" "$smoke/CThinkThen/include/"
    cp "$smoke/native/include/thinkthen.h" "$smoke/CThinkThen/include/"
    swiftc -swift-version 6 -j2 -I "$smoke/CThinkThen" "$here"/Sources/ThinkThen/*.swift "$here/Tests/fixtures/smoke.swift" -L "$smoke/native/lib" -lthinkthen -Xlinker -rpath -Xlinker "$smoke/native/lib" -o "$smoke/consumer"
    "$smoke/consumer"
    exit 0
fi
scratch_dir package_scratch
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    . "$root/sdlc/scripts/installed.sh"
    installed_unpack
    package=$scratch
    [ -f "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'Swift installed: matching C archive missing' >&2; exit 1; }
    THINKTHEN_ARTIFACT=$THINKTHEN_C_ARTIFACT
    installed_unpack
    native=$scratch
    case $(uname -m) in x86_64) triple=x86_64-unknown-linux-gnu ;; aarch64) triple=aarch64-unknown-linux-gnu ;; *) echo "Swift has no native asset for this host" >&2; exit 1 ;; esac
    cmp "$package/Sources/CThinkThen/include/thinkthen.h" "$native/include/thinkthen.h"
    cmp "$package/Sources/ThinkThen/Native/$triple/libthinkthen.so" "$native/lib/libthinkthen.so"
else
    export CARGO_BUILD_JOBS=${CARGO_BUILD_JOBS:-2} CARGO_NET_OFFLINE=true
    RUSTC_WRAPPER= RUSTFLAGS="${RUSTFLAGS:-} --remap-path-prefix=$HOME=/build" cargo build --locked --offline --manifest-path "$root/libraries/c/Cargo.toml" --lib -j2
    package=$package_scratch/package
    python3 "$here/Tests/fixtures/package_local.py" --native "$root/libraries/c/target/debug/libthinkthen_c.so" --out "$package"
    python3 "$here/Tests/fixtures/guard.py" "$package_scratch/thinkthen-swift-0.2.0.tar.gz"
fi
scratch_dir consumer_scratch
python3 "$here/Tests/fixtures/installed.py" --package "$package" --out "$consumer_scratch"
if [ "${THINKTHEN_TEST_PROFILE:-}" = full ]; then
    python3 "$here/Tests/fixtures/complete_parity.py" --binary "$consumer_scratch/installed/Parity"
else
    python3 "$here/Tests/fixtures/complete_parity.py" --binary "$consumer_scratch/installed/Parity" --routine
fi
