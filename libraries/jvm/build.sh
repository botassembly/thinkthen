#!/bin/sh
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../.." && pwd)
# The compiler inventory and Maven/native packaging have one owner.
if [ "$#" -gt 0 ]; then
    exec python3 "$here/build-session.py" "$@"
fi
native=${THINKTHEN_JVM_NATIVE:-$root/libraries/c/target/debug/libthinkthen_c.so}
target=${THINKTHEN_JVM_TARGET:-$(rustc -vV | sed -n 's/^host: //p')}
exec python3 "$here/build-session.py" --out "${THINKTHEN_JVM_OUT:-$here/target/session}" --native "$native" --target "$target"
