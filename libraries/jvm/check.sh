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
. "$root/sdlc/scripts/scratch.sh"
usage_home
for tool in python3 bwrap; do command -v "$tool" >/dev/null 2>&1 || exit 77; done
# Resolve the supported compiler floor before any native build or package work.
python3 "$here/tests/toolchains.py" --stable
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    . "$root/sdlc/scripts/installed.sh"
    installed_unpack
    package=$scratch
else
    scratch_dir package
    if [ -z "${THINKTHEN_JVM_NATIVE:-}" ]; then
        RUSTC_WRAPPER= CARGO_NET_OFFLINE=true cargo build --manifest-path "$root/libraries/c/Cargo.toml" --locked --offline --lib -j2
    fi
    THINKTHEN_JVM_OUT=$package sh "$here/build.sh"
fi
scratch_dir consumer
THINKTHEN_JDK_HOME=$(python3 -c 'import sys; sys.path.insert(0, sys.argv[1]); from toolchains import JDK; print(JDK)' "$here/tests")
THINKTHEN_KOTLIN_HOME=$(python3 -c 'import sys; sys.path.insert(0, sys.argv[1]); from toolchains import KOTLIN; print(KOTLIN)' "$here/tests")
THINKTHEN_SCALA_HOME=$(python3 -c 'import sys; sys.path.insert(0, sys.argv[1]); from toolchains import SCALA; print(SCALA)' "$here/tests")
export THINKTHEN_JDK_HOME THINKTHEN_KOTLIN_HOME THINKTHEN_SCALA_HOME
jars=$package/jars
[ -d "$jars" ] || jars=$package
python3 "$here/tests/session_installed.py" --jars "$jars" --out "$consumer"
python3 "$here/tests/usage_installed.py" --jars "$jars" --out "$consumer/usage"
if [ "${THINKTHEN_PORTABLE_BATCH:-}" = 1 ]; then
    python3 "$here/tests/public_types.py" --jars "$jars" --out "$consumer/shared"
fi
echo 'JVM stable package PASS: installed Java, Kotlin and Scala consumers'
