#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../.." && pwd)
lock=${THINKTHEN_HEAVY_LOCK:-${XDG_RUNTIME_DIR:-/tmp}/thinkthen-jvm.lock}
if [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ] && command -v flock >/dev/null 2>&1; then
    THINKTHEN_HEAVY_LOCK_HELD=$lock
    export THINKTHEN_HEAVY_LOCK_HELD
    exec flock -o "$lock" /bin/sh "$0" "$@"
fi
for tool in javac kotlinc scalac python3 bwrap; do command -v "$tool" >/dev/null 2>&1 || exit 77; done
mkdir -p "$here/target/native"
RUSTC_WRAPPER= CARGO_NET_OFFLINE=true cargo build --manifest-path "$root/libraries/c/Cargo.toml" --locked --offline --lib -j2
native="$root/libraries/c/target/debug/libthinkthen_c.so"
python3 "$root/sdlc/scripts/check-c-exports.py" "$root/libraries/c/include/thinkthen.h" "$native"
cp "$native" "$here/target/native/libthinkthen.so"
sh "$here/build.sh"
python3 "$here/tests/package_check.py"
python3 "$here/tests/installed.py"
python3 "$here/tests/types.py"
echo 'JVM package PASS: installed Java, Kotlin, Scala consumers and J1 corpus'
