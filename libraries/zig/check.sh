#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../.." && pwd)
host_home=$HOME
export CARGO_HOME=${CARGO_HOME:-$host_home/.cargo} RUSTUP_HOME=${RUSTUP_HOME:-$host_home/.rustup}
zig=${THINKTHEN_ZIG:-$(command -v zig || true)}
[ -x "$zig" ] || exit 77
export THINKTHEN_ZIG="$zig"
for tool in cargo python3 node nm bwrap flock git tar; do command -v "$tool" >/dev/null 2>&1 || exit 77; done
export PATH="$(dirname "$zig"):$PATH"
lock=${THINKTHEN_HEAVY_LOCK:-${XDG_RUNTIME_DIR:-/tmp}/thinkthen-zig.lock}
if [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ]; then
    THINKTHEN_HEAVY_LOCK_HELD=$lock
    export THINKTHEN_HEAVY_LOCK_HELD
    exec flock -w 180 -E 75 -o "$lock" /bin/sh "$0" "$@"
fi
mkdir -p "$here/target/native/include" "$here/target/native/lib" "$here/target/home" "$here/target/cache" "$here/target/scratch" "$here/target/logs"
node "$root/sdlc/scripts/ratchet.mjs" "$here/ratchet.zig.json"
node "$root/sdlc/scripts/ratchet.mjs" "$here/ratchet.py.json"
RUSTC_WRAPPER= CARGO_NET_OFFLINE=true cargo build --manifest-path "$root/libraries/c/Cargo.toml" --locked --offline --lib -j2
native="$root/libraries/c/target/debug"
python3 "$root/sdlc/scripts/check-c-exports.py" "$root/libraries/c/include/thinkthen.h" "$native/libthinkthen_c.so"
cp "$root/libraries/c/include/thinkthen.h" "$here/target/native/include/thinkthen.h"
cp "$native/libthinkthen_c.so" "$here/target/native/lib/libthinkthen.so"
cp "$native/libthinkthen_c.a" "$here/target/native/lib/libthinkthen.a"
ln -sf libthinkthen.so "$here/target/native/lib/libthinkthen.so.0"
export HOME="$here/target/home" ZIG_GLOBAL_CACHE_DIR="$here/target/cache"
"$zig" fmt --check "$here/src/thinkthen.zig" "$here/build.zig" "$here/Tests/type_case.zig" "$here/Tests/settings.zig" "$here/Tests/build.zig"
"$zig" build -j2 -Dnative="$here/target/native" -Dlink-mode=shared --build-file "$here/build.zig" --cache-dir "$here/target/scratch/package-cache" --global-cache-dir "$here/target/cache"
"$zig" build -j2 -Dnative="$here/target/native" -Dlink-mode=shared --build-file "$here/Tests/build.zig" --cache-dir "$here/target/scratch/tests-cache" --global-cache-dir "$here/target/cache"
python3 "$here/Tests/types.py"
python3 "$here/Tests/run_matrix.py"
python3 "$here/Tests/run_settings.py"
python3 "$here/Tests/package_local.py"
python3 "$here/Tests/guard.py" "$here/target/artifacts/thinkthen-zig-0.0.1-src.tar.gz"
plant=$(mktemp "$here/target/logs/private-plant-XXXXXX")
printf '%s\n' '/home/ian/private' >"$plant"
if python3 "$here/Tests/guard.py" "$plant" >"$plant.log" 2>&1; then echo 'Zig privacy plant passed' >&2; exit 1; fi
grep -q 'rejected private byte pattern' "$plant.log"
rm -f "$plant" "$plant.log"
python3 "$here/Tests/installed.py"
echo 'Zig package PASS: public J1, 42 exact bodies in four installed consumers'
