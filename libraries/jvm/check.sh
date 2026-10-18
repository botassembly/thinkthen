#!/bin/sh
set -eu
python3 "$(dirname -- "$0")/../../sdlc/scripts/check_verdict.py" platform libraries/jvm
tools_group=jvm
. "$(dirname -- "$0")/../../sdlc/scripts/resolved-tools"
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../.." && pwd)
. "$root/sdlc/scripts/scratch.sh"
usage_home
if [ -n "${THINKTHEN_PARITY_PREPARE:-}${THINKTHEN_PARITY_PREPARED:-}" ]; then
    [ "${THINKTHEN_TEST_PROFILE:-routine}" = full ] || exit 2
fi
for tool in python3 bwrap; do command -v "$tool" >/dev/null 2>&1 || exit 1; done
# Resolve the supported compiler floor before any native build or package work.
python3 "$here/tests/toolchains.py" --stable
if [ -n "${THINKTHEN_PARITY_PREPARED:-}" ]; then
    package=$THINKTHEN_PARITY_PREPARED/package
elif [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
    . "$root/sdlc/scripts/installed.sh"
    installed_unpack
    if [ -n "${THINKTHEN_PARITY_PREPARE:-}" ]; then
        package=$THINKTHEN_PARITY_PREPARE/package
        mkdir -p "$package"
        cp -a "$scratch/." "$package/"
    else
        package=$scratch
    fi
else
    if [ -n "${THINKTHEN_PARITY_PREPARE:-}" ]; then
        package=$THINKTHEN_PARITY_PREPARE/package
        mkdir -p "$package"
    else
        scratch_dir package
    fi
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
if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ]; then
    smoke_guard
    floor=$(python3 -c 'import sys; sys.path.insert(0, sys.argv[1]); from toolchains import JDK_FLOOR; print(JDK_FLOOR)' "$here/tests")
    "$THINKTHEN_JDK_HOME/bin/javac" --release "$floor" -cp "$jars/thinkthen-session.jar" -d "$consumer" "$here/tests/SmokeConsumer.java"
    "$THINKTHEN_JDK_HOME/bin/java" --enable-native-access=ALL-UNNAMED -cp "$jars/*:$consumer" SmokeConsumer
    exit
fi
if [ -z "${THINKTHEN_PARITY_PREPARED:-}" ]; then
python3 "$here/tests/session_installed.py" --jars "$jars" --out "$consumer"
CARGO_NET_OFFLINE=true cargo build --manifest-path "$root/Cargo.toml" --locked --offline -p conformance-backend -j2
if [ "${THINKTHEN_TEST_PROFILE:-routine}" != full ] && [ "${THINKTHEN_PORTABLE_BATCH:-}" != 1 ]; then
    python3 "$here/tests/public_types.py" --jars "$jars" --out "$consumer/call-result" --case library-call-result --case settings-max-requests-refuses-past-the-limit
fi
python3 "$here/tests/usage_installed.py" --jars "$jars" --out "$consumer/usage"
fi
if [ -n "${THINKTHEN_PARITY_PREPARE:-}" ]; then
    python3 "$here/tests/public_types.py" --jars "$jars" --out "$THINKTHEN_PARITY_PREPARE/shared"
    exit 0
fi
if [ "${THINKTHEN_TEST_PROFILE:-routine}" = full ] || [ "${THINKTHEN_PORTABLE_BATCH:-}" = 1 ]; then
    if [ -n "${THINKTHEN_PARITY_PREPARED:-}" ]; then
        python3 "$here/tests/public_types.py" --jars "$jars" --out "$THINKTHEN_PARITY_PREPARED/shared"
    else
        python3 "$here/tests/public_types.py" --jars "$jars" --out "$consumer/shared"
    fi
fi
echo 'JVM stable package PASS: installed Java, Kotlin and Scala consumers'
