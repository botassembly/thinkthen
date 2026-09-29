#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../.." && pwd)
host_home=$HOME
export CARGO_HOME=${CARGO_HOME:-$host_home/.cargo} RUSTUP_HOME=${RUSTUP_HOME:-$host_home/.rustup}
swift=${THINKTHEN_SWIFT:-$(command -v swift || true)}
swiftc=${THINKTHEN_SWIFTC:-$(command -v swiftc || true)}
for tool in "$swift" "$swiftc"; do [ -x "$tool" ] || exit 77; done
for tool in cargo python3 node nm bwrap flock git tar; do command -v "$tool" >/dev/null 2>&1 || exit 77; done
export PATH="$(dirname "$swift"):$(dirname "$swiftc"):$PATH"
lock=${THINKTHEN_HEAVY_LOCK:-${XDG_RUNTIME_DIR:-/tmp}/thinkthen-swift.lock}
if [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ]; then
    THINKTHEN_HEAVY_LOCK_HELD=$lock
    export THINKTHEN_HEAVY_LOCK_HELD
    exec flock -w 180 -E 75 -o "$lock" /bin/sh "$0" "$@"
fi
mkdir -p "$here/target/native/lib" "$here/target/scratch/matrix-main" "$here/target/home" "$here/target/cache" "$here/target/logs"
node "$root/sdlc/scripts/ratchet.mjs" "$here/ratchet.swift.json"
node "$root/sdlc/scripts/ratchet.mjs" "$here/ratchet.py.json"
cmp "$root/libraries/c/include/thinkthen.h" "$here/Sources/CThinkThen/include/thinkthen.h"
RUSTC_WRAPPER= CARGO_NET_OFFLINE=true cargo build --manifest-path "$root/libraries/c/Cargo.toml" --locked --offline --lib -j2
native="$root/libraries/c/target/debug/libthinkthen_c.so"
python3 "$root/sdlc/scripts/check-c-exports.py" "$root/libraries/c/include/thinkthen.h" "$native"
cp "$native" "$here/target/native/lib/libthinkthen.so"
ln -sf libthinkthen.so "$here/target/native/lib/libthinkthen.so.0"
export HOME="$here/target/home" XDG_CACHE_HOME="$here/target/cache" SWIFTPM_MODULECACHE_OVERRIDE="$here/target/cache/module-cache"
export THINKTHEN_SWIFT_BUILD_DIR="$here/target/scratch/swift-build-product"
"$swift" build --package-path "$here" --scratch-path "$THINKTHEN_SWIFT_BUILD_DIR" --jobs 2 -Xlinker -L -Xlinker "$here/target/native/lib" -Xlinker -rpath -Xlinker "$here/target/native/lib"
cp "$here/Tests/fixtures/matrix.swift" "$here/target/scratch/matrix-main/main.swift"
"$swiftc" -j 2 -I "$here/Sources/CThinkThen" "$here/Sources/ThinkThen/ThinkThen.swift" "$here/target/scratch/matrix-main/main.swift" -L "$here/target/native/lib" -lthinkthen -Xlinker -rpath -Xlinker "$here/target/native/lib" -o "$here/target/scratch/swift-matrix"
cp "$here/Tests/fixtures/settings.swift" "$here/target/scratch/matrix-main/main.swift"
"$swiftc" -j 2 -I "$here/Sources/CThinkThen" "$here/Sources/ThinkThen/ThinkThen.swift" "$here/target/scratch/matrix-main/main.swift" -L "$here/target/native/lib" -lthinkthen -Xlinker -rpath -Xlinker "$here/target/native/lib" -o "$here/target/scratch/swift-settings"
python3 "$here/Tests/fixtures/types.py"
python3 "$here/Tests/fixtures/run_matrix.py"
python3 "$here/Tests/fixtures/run_settings.py"
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
echo 'Swift package PASS: public J1, 30 exact bodies, two installed consumers'
