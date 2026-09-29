#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO=$(CDPATH= cd -- "$ROOT/../.." && pwd)
case "$(uname -s):$(uname -m)" in Linux:x86_64) ;; *) echo 'GNU Objective-C gate unavailable on this host' >&2; exit 77 ;; esac
case "${THINKTHEN_TEST_PROFILE:-routine}" in routine|full) ;; stress) exit 77 ;; *) exit 2 ;; esac
for tool in gcc cargo flock nm node python3 cmp; do
  command -v "$tool" >/dev/null 2>&1 || { echo "GNU Objective-C gate missing $tool" >&2; exit 77; }
done
python3 -c 'import jsonschema' >/dev/null 2>&1 || { echo 'GNU Objective-C gate missing Python jsonschema' >&2; exit 77; }
if [ "${TT_OBJC_LOCKED:-0}" != 1 ]; then
  TT_OBJC_LOCKED=1 exec flock -w 180 -E 75 -o "${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-3.lock}" env TT_OBJC_LOCKED=1 "$0" "$@"
fi
unset THINKTHEN_API_KEY
export CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
export CARGO_TARGET_DIR="$REPO/libraries/c/target"
NATIVE="$CARGO_TARGET_DIR/debug/libthinkthen_c.so"
TARGET="$ROOT/checks/target"
mkdir -p "$TARGET"
python3 "$ROOT/checks/privacy.py"
for config in "$ROOT"/ratchet.*.json; do node "$REPO/sdlc/scripts/ratchet.mjs" "$config"; done
cargo build --locked --offline --manifest-path "$REPO/libraries/c/Cargo.toml" --lib -j2
cmp "$ROOT/Sources/thinkthen.h" "$REPO/libraries/c/include/thinkthen.h"
ln -sf "$NATIVE" "$TARGET/libthinkthen.so.0"
python3 "$ROOT/checks/exports.py" "$NATIVE" "$REPO/libraries/c/include/thinkthen.h" "$TARGET/exports.txt"
for name in door failure main; do
  source="$ROOT/checks/$name.m"
  [ "$name" != main ] || source="$ROOT/checks/matrix.m"
  gcc -std=gnu11 -x objective-c -I"$ROOT/Sources" "$ROOT/Sources/ThinkThen.m" "$ROOT/Sources/TTJSON.c" "$source" -L"$CARGO_TARGET_DIR/debug" -lthinkthen_c -lobjc -pthread -lm -o "$TARGET/$name"
done
gcc -std=gnu11 -x objective-c -I"$ROOT/Sources" "$ROOT/checks/direct.m" -L"$CARGO_TARGET_DIR/debug" -lthinkthen_c -lobjc -pthread -o "$TARGET/direct"
python3 "$ROOT/checks/types.py"
python3 "$ROOT/checks/installed.py"
python3 "$ROOT/checks/failure.py"
python3 "$ROOT/checks/run_matrix.py"
python3 "$ROOT/checks/negative.py"
echo 'GNU Objective-C package gate passed'
