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
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
. "$root/sdlc/scripts/scratch.sh"
usage_home
if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ]; then
    smoke_guard
    # The replay smoke (ticket 0335): the package module over the installed C door.
    . "$root/sdlc/scripts/installed.sh"
    scratch_dir smoke
    native_install "$root" "$smoke/native"
    HOME="$smoke" "$zig" build-exe -j2 --cache-dir "$here/target/scratch/smoke-cache" --global-cache-dir "$here/target/cache" \
        --dep thinkthen -Mroot="$here/examples/smoke.zig" -I "$smoke/native/include" -Mthinkthen="$here/src/thinkthen.zig" \
        -L "$smoke/native/lib" -lthinkthen -lc -rpath "$smoke/native/lib" -femit-bin="$smoke/smoke"
    "$smoke/smoke"
    exit
fi
mkdir -p "$here/target/native/include" "$here/target/native/lib" "$here/target/home" "$here/target/cache" "$here/target/scratch" "$here/target/logs"
node "$root/sdlc/scripts/ratchet.mjs" "$here/ratchet.zig.json"
node "$root/sdlc/scripts/ratchet.mjs" "$here/ratchet.py.json"
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    [ -f "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'Zig installed: C archive missing' >&2; exit 1; }
    . "$root/sdlc/scripts/scratch.sh"
    . "$root/sdlc/scripts/installed.sh"
    installed_scratch
    release_root=$scratch
    mkdir "$release_root/package" "$release_root/native" "$release_root/project"
    tar -xzf "$THINKTHEN_ARTIFACT" -C "$release_root/package"
    tar -xzf "$THINKTHEN_C_ARTIFACT" -C "$release_root/native"
    native=$release_root/native
    tar -xOzf "$THINKTHEN_C_ARTIFACT" ./include/thinkthen.h | cmp - "$native/include/thinkthen.h"
    tar -xOzf "$THINKTHEN_C_ARTIFACT" ./lib/libthinkthen.so | cmp - "$native/lib/libthinkthen.so"
    tar -xOzf "$THINKTHEN_C_ARTIFACT" ./lib/libthinkthen.a | cmp - "$native/lib/libthinkthen.a"
    python3 "$root/sdlc/scripts/check-c-exports.py" "$native/include/thinkthen.h" "$native/lib/libthinkthen.so"
    cp "$here/Tests/build.zig" "$here/Tests/portable_batch.zig" "$release_root/project/"
    sed 's|.path = "../"|.path = "../package"|' "$here/Tests/build.zig.zon" >"$release_root/project/build.zig.zon"
    export HOME="$here/target/home" ZIG_GLOBAL_CACHE_DIR="$here/target/cache"
    RUSTC_WRAPPER= CARGO_NET_OFFLINE=true cargo build --locked --offline --manifest-path "$root/Cargo.toml" --package conformance-backend -j2
    THINKTHEN_PORTABLE_ZIG_PROJECT="$release_root/project" THINKTHEN_NATIVE_ROOT="$native" \
        python3 "$here/Tests/portable_batch.py"
    echo 'Zig installed release PASS: three literal portable sends'
    exit 0
fi
case ${THINKTHEN_FOCUSED:-} in
    portable-batch) "$zig" fmt --check "$here/Tests/portable_batch.zig" "$here/Tests/build.zig"; python3 "$here/Tests/portable_batch.py"; exit 0 ;;
    '') ;;
    *) echo "Zig: unknown focused selector: $THINKTHEN_FOCUSED" >&2; exit 2 ;;
esac
RUSTC_WRAPPER= CARGO_NET_OFFLINE=true cargo build --manifest-path "$root/libraries/c/Cargo.toml" --locked --offline --lib -j2
native="$root/libraries/c/target/debug"
python3 "$root/sdlc/scripts/check-c-exports.py" "$root/libraries/c/include/thinkthen.h" "$native/libthinkthen_c.so"
cp "$root/libraries/c/include/thinkthen.h" "$here/target/native/include/thinkthen.h"
cp "$native/libthinkthen_c.so" "$here/target/native/lib/libthinkthen.so"
sh "$root/libraries/c/localize.sh" "$native/libthinkthen_c.a" "$here/target/native/lib/libthinkthen.a"
ln -sf libthinkthen.so "$here/target/native/lib/libthinkthen.so.0"
export HOME="$here/target/home" ZIG_GLOBAL_CACHE_DIR="$here/target/cache"
"$zig" fmt --check "$here/src/thinkthen.zig" "$here/src/complete.zig" "$here/src/native.zig" "$here/Tests/native_consumer.zig" "$here/Tests/carriers.zig" "$here/build.zig" "$here/Tests/type_case.zig" "$here/Tests/settings.zig" "$here/Tests/portable_batch.zig" "$here/Tests/build.zig"
"$zig" build -j2 -Dnative="$here/target/native" -Dlink-mode=shared --build-file "$here/build.zig" --cache-dir "$here/target/scratch/package-cache" --global-cache-dir "$here/target/cache"
"$zig" build -j2 -Dnative="$here/target/native" -Dlink-mode=shared --build-file "$here/Tests/build.zig" --cache-dir "$here/target/scratch/tests-cache" --global-cache-dir "$here/target/cache"
"$zig" test -j2 --cache-dir "$here/target/scratch/carrier-cache" --global-cache-dir "$here/target/cache" --dep thinkthen -Mroot="$here/Tests/carriers.zig" -I "$here/target/native/include" -Mthinkthen="$here/src/thinkthen.zig" -lc
python3 "$here/Tests/public_types.py"
python3 "$here/Tests/run_matrix.py"
python3 "$here/Tests/run_matrix.py" facts
python3 "$here/Tests/run_matrix.py" facts-allocation
python3 "$here/Tests/run_settings.py"
python3 "$here/Tests/portable_batch.py"
python3 "$here/Tests/package_local.py"
python3 "$here/Tests/guard.py" "$here/target/artifacts/thinkthen-zig-0.0.1-src.tar.gz"
plant=$(mktemp "$here/target/logs/private-plant-XXXXXX")
printf '%s\n' '/home/private/file' >"$plant"
if python3 "$here/Tests/guard.py" "$plant" >"$plant.log" 2>&1; then echo 'Zig privacy plant passed' >&2; exit 1; fi
grep -q 'rejected private byte pattern' "$plant.log"
rm -f "$plant" "$plant.log"
python3 "$here/Tests/installed.py"
echo 'Zig package PASS: public J1, 41 exact bodies in four installed consumers'
