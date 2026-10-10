#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO=$(CDPATH= cd -- "$ROOT/../.." && pwd)
if [ "$(uname -s)" = Darwin ]; then
  python3 "$REPO/sdlc/generators/results/generate.py" --target objc --check
  exec python3 "$ROOT/checks/foundation_installed.py"
fi
# Preserve the legacy consumer until the Foundation replacement is qualified.
case "$(uname -s):$(uname -m)" in Linux:x86_64) ;; *) echo 'GNU Objective-C gate unavailable on this host' >&2; exit 77 ;; esac
case "${THINKTHEN_TEST_PROFILE:-routine}" in routine|full|smoke) ;; stress) exit 77 ;; *) exit 2 ;; esac
for tool in gcc cargo flock nm python3 cmp; do
  command -v "$tool" >/dev/null 2>&1 || { echo "GNU Objective-C gate missing $tool" >&2; exit 77; }
done
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
  for tool in tar ldd; do command -v "$tool" >/dev/null 2>&1 || exit 77; done
  [ -f "$THINKTHEN_ARTIFACT" ] && [ -f "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'Objective-C installed: wrapper or C archive missing' >&2; exit 1; }
fi
# A caller that already holds this lock, such as the surfaces rung, exports
# THINKTHEN_HEAVY_LOCK_HELD; waiting on it again would only time out.
lock=${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-3.lock}
if [ "${TT_OBJC_LOCKED:-0}" != 1 ] && [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ]; then
  TT_OBJC_LOCKED=1 exec flock -w 180 -E 75 -o "$lock" env TT_OBJC_LOCKED=1 "$0" "$@"
fi
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
. "$REPO/sdlc/scripts/scratch.sh"
usage_home
if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ]; then
  smoke_guard
  # The replay smoke (ticket 0335): the source package over the installed C door.
  . "$REPO/sdlc/scripts/installed.sh"
  scratch_dir smoke
  native_install "$REPO" "$smoke/native"
  gcc -std=gnu11 -x objective-c -I"$smoke/native/include" -I"$ROOT/Sources" "$ROOT/Sources/ThinkThen.m" "$ROOT/Sources/TTJSON.c" \
    "$ROOT/Examples/smoke.m" -L"$smoke/native/lib" -lthinkthen -Wl,-rpath,"$smoke/native/lib" -lobjc -pthread -lm -o "$smoke/smoke"
  "$smoke/smoke"
  exit
fi
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
  . "$REPO/sdlc/scripts/scratch.sh"
  . "$REPO/sdlc/scripts/installed.sh"
  installed_unpack
  wrapper=$scratch
  THINKTHEN_ARTIFACT=$THINKTHEN_C_ARTIFACT
  installed_unpack
  native=$scratch
  scratch_dir consumer
  mkdir -p "$consumer/include" "$consumer/lib"
  for member in include/thinkthen.h lib/libthinkthen.so lib/libthinkthen.a; do
    tar -xOzf "$THINKTHEN_C_ARTIFACT" "./$member" | cmp - "$native/$member"
    cp "$native/$member" "$consumer/$member"
    cmp "$native/$member" "$consumer/$member"
  done
  ln -s libthinkthen.so "$consumer/lib/libthinkthen.so.0"
  python3 "$REPO/sdlc/scripts/check-c-exports.py" "$consumer/include/thinkthen.h" "$consumer/lib/libthinkthen.so"
  export CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
  export CARGO_TARGET_DIR="$REPO/target/0265-backend"
  cargo build --locked --offline --manifest-path "$REPO/Cargo.toml" --package conformance-backend -j2
  THINKTHEN_PORTABLE_PACKAGE="$wrapper" THINKTHEN_PORTABLE_NATIVE="$consumer" \
    THINKTHEN_BACKEND_BIN="$CARGO_TARGET_DIR/debug/conformance-backend" python3 "$ROOT/checks/portable_batch.py"
  THINKTHEN_PARITY_PACKAGE="$wrapper" THINKTHEN_NATIVE_ROOT="$native" \
    python3 "$ROOT/checks/complete_parity.py"
  echo 'GNU Objective-C installed release PASS: five typed rows and three literal sends'
  exit 0
fi
command -v node >/dev/null 2>&1 || { echo 'GNU Objective-C gate missing node' >&2; exit 77; }
python3 -c 'import jsonschema' >/dev/null 2>&1 || { echo 'GNU Objective-C gate missing Python jsonschema' >&2; exit 77; }
unset THINKTHEN_API_KEY
export CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
export CARGO_TARGET_DIR="$REPO/libraries/c/target"
NATIVE="$CARGO_TARGET_DIR/debug/libthinkthen_c.so"
TARGET="$ROOT/checks/target"
mkdir -p "$TARGET"
python3 "$ROOT/checks/privacy.py"
for config in "$ROOT"/ratchet.*.json; do node "$REPO/sdlc/scripts/ratchet.mjs" "$config"; done
gcc -std=gnu11 -Wall -Wextra -Werror -x objective-c -I"$ROOT/Sources" "$ROOT/Sources/TTComplete.c" "$ROOT/checks/carriers.m" "$ROOT/Sources/TTJSON.c" -o "$TARGET/carriers"
"$TARGET/carriers"
cargo build --locked --offline --manifest-path "$REPO/libraries/c/Cargo.toml" --lib -j2
ln -sf "$NATIVE" "$CARGO_TARGET_DIR/debug/libthinkthen.so.0"
CARGO_TARGET_DIR="$REPO/target" cargo build --locked --offline --manifest-path "$REPO/Cargo.toml" --package conformance-backend -j2
THINKTHEN_PORTABLE_NATIVE="$CARGO_TARGET_DIR/debug" THINKTHEN_BACKEND_BIN="$REPO/target/debug/conformance-backend" python3 "$ROOT/checks/portable_batch.py"
ln -sf "$NATIVE" "$TARGET/libthinkthen.so.0"
python3 "$ROOT/checks/exports.py" "$NATIVE" "$REPO/libraries/c/include/thinkthen.h" "$TARGET/exports.txt"
for name in door failure main; do
  source="$ROOT/checks/$name.m"
  [ "$name" != main ] || source="$ROOT/checks/matrix.m"
  gcc -std=gnu11 -x objective-c -I"$REPO/libraries/c/include" -I"$ROOT/Sources" "$ROOT/Sources/ThinkThen.m" "$ROOT/Sources/TTJSON.c" "$source" -L"$CARGO_TARGET_DIR/debug" -lthinkthen_c -lobjc -pthread -lm -o "$TARGET/$name"
done
gcc -std=gnu11 -x objective-c -I"$REPO/libraries/c/include" -I"$ROOT/Sources" "$ROOT/Sources/ThinkThen.m" "$ROOT/Sources/TTJSON.c" "$ROOT/checks/nul_text.m" -L"$CARGO_TARGET_DIR/debug" -lthinkthen_c -lobjc -pthread -lm -Wl,--wrap=thinkthen_recognize -Wl,--wrap=thinkthen_free_string -o "$TARGET/nul_text"
gcc -std=gnu11 -x objective-c -I"$REPO/libraries/c/include" -I"$ROOT/Sources" "$ROOT/checks/direct.m" -L"$CARGO_TARGET_DIR/debug" -lthinkthen_c -lobjc -pthread -o "$TARGET/direct"
python3 "$ROOT/checks/public_types.py"
python3 "$ROOT/checks/installed.py"
python3 "$ROOT/checks/failure.py"
python3 "$ROOT/checks/run_matrix.py"
python3 "$ROOT/checks/negative.py"
echo 'GNU Objective-C package gate passed'
