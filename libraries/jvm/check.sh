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
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
. "$root/sdlc/scripts/scratch.sh"
usage_home
for tool in python3 bwrap; do command -v "$tool" >/dev/null 2>&1 || exit 77; done
python3 "$here/tests/toolchains.py"
if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ]; then
    # The replay smoke (ticket 0335): the door jar as build.sh packs it, over the installed C door.
    . "$root/sdlc/scripts/installed.sh"
    scratch_dir smoke
    native_install "$root" "$smoke/native"
    jdk=${THINKTHEN_JDK_HOME:+$THINKTHEN_JDK_HOME/bin/}
    "${jdk}javac" --enable-preview --release 21 -d "$smoke/door" "$here/door/thinkthen/Door.java" "$here/door/thinkthen/Json.java"
    "${jdk}jar" --create --file "$smoke/thinkthen-door.jar" -C "$smoke/door" .
    "${jdk}javac" --enable-preview --release 21 -cp "$smoke/thinkthen-door.jar" -d "$smoke/app" "$here/tests/Smoke.java"
    "${jdk}java" --enable-preview --enable-native-access=ALL-UNNAMED -XX:ActiveProcessorCount=2 \
        -Dthinkthen.library="$smoke/native/lib/libthinkthen.so" -cp "$smoke/thinkthen-door.jar:$smoke/app" Smoke
    exit
fi
[ "${THINKTHEN_PORTABLE_BATCH:-}" != 1 ] || [ -n "${THINKTHEN_ARTIFACT:-}" ] || {
    echo 'JVM portable batch needs an installed artifact' >&2; exit 2;
}
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    [ -f "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'JVM installed: C archive missing' >&2; exit 1; }
    . "$root/sdlc/scripts/scratch.sh"
    . "$root/sdlc/scripts/installed.sh"
    installed_unpack
    managed=$scratch
    scratch_dir native
    tar -xzf "$THINKTHEN_C_ARTIFACT" -C "$native"
    python3 "$root/sdlc/scripts/check-c-exports.py" "$native/include/thinkthen.h" "$native/lib/libthinkthen.so"
    THINKTHEN_RELEASE_JVM_DIR="$managed" THINKTHEN_RELEASE_C_DIR="$native" \
        python3 "$here/tests/installed.py"
    if [ "${THINKTHEN_PORTABLE_BATCH:-}" = 1 ]; then
        echo 'JVM portable batch PASS: Java, Kotlin and Scala installed files'
    else
        echo 'JVM installed release PASS: Java, Kotlin and Scala one call each'
    fi
    exit 0
fi
mkdir -p "$here/target/native"
RUSTC_WRAPPER= CARGO_NET_OFFLINE=true cargo build --manifest-path "$root/libraries/c/Cargo.toml" --locked --offline --lib -j2
native="$root/libraries/c/target/debug/libthinkthen_c.so"
python3 "$root/sdlc/scripts/check-c-exports.py" "$root/libraries/c/include/thinkthen.h" "$native"
cp "$native" "$here/target/native/libthinkthen.so"
sh "$here/build.sh"
python3 "$here/tests/package_check.py"
python3 "$here/tests/installed.py"
python3 "$here/tests/public_types.py"
echo 'JVM package PASS: installed Java, Kotlin, Scala consumers and J1 corpus'
