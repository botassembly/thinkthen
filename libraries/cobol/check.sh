#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO=$(CDPATH= cd -- "$ROOT/../.." && pwd)
case "$(uname -s):$(uname -m)" in Linux:x86_64) ;; *) echo 'COBOL gate unavailable on this host' >&2; exit 77 ;; esac
case "${THINKTHEN_TEST_PROFILE:-routine}" in routine|full|smoke) ;; stress) exit 77 ;; *) exit 2 ;; esac
for tool in cobc cc clang cargo flock nm python3; do
  command -v "$tool" >/dev/null 2>&1 || { echo "COBOL gate missing $tool" >&2; exit 77; }
done
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
  for tool in tar cmp ldd; do command -v "$tool" >/dev/null 2>&1 || exit 77; done
  [ -f "$THINKTHEN_ARTIFACT" ] && [ -f "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'COBOL installed: wrapper or C archive missing' >&2; exit 1; }
fi
# A caller that already holds this lock, such as the surfaces rung, exports
# THINKTHEN_HEAVY_LOCK_HELD; waiting on it again would only time out.
lock=${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-3.lock}
if [ "${TT_COBOL_LOCKED:-0}" != 1 ] && [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ]; then
  TT_COBOL_LOCKED=1 exec flock -w 180 -E 75 -o "$lock" env TT_COBOL_LOCKED=1 "$0" "$@"
fi
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
. "$REPO/sdlc/scripts/scratch.sh"
usage_home
# Isolate read-only test configuration from the caller's configuration.
scratch_dir fixture_config
export XDG_CONFIG_HOME="$fixture_config"
if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ]; then
  smoke_guard
  # The replay smoke (ticket 0335): the source package over the installed C door.
  . "$REPO/sdlc/scripts/installed.sh"
  scratch_dir smoke
  native_install "$REPO" "$smoke/native"
  python3 "$ROOT/checks/exports.py" "$smoke/native/lib/libthinkthen.so" "$smoke/native/include/thinkthen.h" "$smoke/exports.txt" "$ROOT"
  cobc -x -free -fstatic-call -fno-gen-c-decl-static-call -I "$ROOT/copybooks" \
    -A "-include $smoke/native/include/thinkthen.h -Wno-incompatible-pointer-types -Wno-implicit-function-declaration" \
    -o "$smoke/smoke" "$ROOT/examples/smoke.cob" "$ROOT/src/tt_decide.cob" "$ROOT/src/tt_error.cob" \
    -L "$smoke/native/lib" -lthinkthen -Q "-Wl,-rpath,$smoke/native/lib"
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
  python3 "$ROOT/checks/exports.py" "$consumer/lib/libthinkthen.so" "$consumer/include/thinkthen.h" "$consumer/exports.txt" "$wrapper"
  export CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
  export CARGO_TARGET_DIR="$REPO/target/0265-backend"
  cargo build --locked --offline --manifest-path "$REPO/Cargo.toml" --package conformance-backend -j2
  THINKTHEN_PORTABLE_PACKAGE="$wrapper" THINKTHEN_PORTABLE_NATIVE="$consumer" \
    THINKTHEN_BACKEND_BIN="$CARGO_TARGET_DIR/debug/conformance-backend" python3 "$ROOT/checks/portable_batch.py"
  THINKTHEN_PARITY_PACKAGE="$wrapper" THINKTHEN_C_HEADER="$consumer/include/thinkthen.h" \
    THINKTHEN_C_LIBRARY="$consumer/lib/libthinkthen.so" python3 "$ROOT/checks/native_parity.py"
  echo 'COBOL installed release PASS: five legacy rows and owned native results for direct and named backends'
  exit 0
fi
command -v node >/dev/null 2>&1 || { echo 'COBOL gate missing node' >&2; exit 77; }
python3 -c 'import jsonschema' >/dev/null 2>&1 || { echo 'COBOL gate missing Python jsonschema' >&2; exit 77; }
unset THINKTHEN_API_KEY
export CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
export CARGO_TARGET_DIR="$REPO/libraries/c/target"
export TT_COBC=$(command -v cobc) TT_CC=$(command -v cc) TT_NM=$(command -v nm)
export TT_NATIVE="$CARGO_TARGET_DIR/debug/libthinkthen_c.so"
export TT_HEADER="$REPO/libraries/c/include/thinkthen.h"
TARGET="$ROOT/checks/target"
mkdir -p "$TARGET"
python3 "$ROOT/checks/privacy.py"
for config in "$ROOT"/ratchet.*.json; do node "$REPO/sdlc/scripts/ratchet.mjs" "$config"; done
cargo build --locked --offline --manifest-path "$REPO/libraries/c/Cargo.toml" --lib -j2
ln -sf "$TT_NATIVE" "$CARGO_TARGET_DIR/debug/libthinkthen.so.0"
CARGO_TARGET_DIR="$REPO/target" cargo build --locked --offline --manifest-path "$REPO/Cargo.toml" --package conformance-backend -j2
THINKTHEN_PORTABLE_NATIVE="$CARGO_TARGET_DIR/debug" THINKTHEN_BACKEND_BIN="$REPO/target/debug/conformance-backend" python3 "$ROOT/checks/portable_batch.py"
ln -sf "$TT_NATIVE" "$TARGET/libthinkthen.so.0"
python3 "$ROOT/checks/exports.py" "$TT_NATIVE" "$TT_HEADER" "$TARGET/exports.txt"
cc -std=c11 -D_GNU_SOURCE -Wno-misleading-indentation -c "$ROOT/src/TTJSON.c" -o "$TARGET/ttjson.o"
cc -std=c11 -D_GNU_SOURCE -Wall -Wextra -c "$ROOT/src/tt_shape.c" -o "$TARGET/ttshape.o"
FLAGS="-include $TT_HEADER -Wno-incompatible-pointer-types -Wno-implicit-function-declaration"
COMMON="-x -free -fstatic-call -fno-gen-c-decl-static-call"
cobc $COMMON -I "$ROOT/copybooks" -A "$FLAGS" -o "$TARGET/direct" "$ROOT/examples/direct.cob" -L "$CARGO_TARGET_DIR/debug" -lthinkthen_c
cobc $COMMON -I "$ROOT/copybooks" -A "$FLAGS" -o "$TARGET/settings" "$ROOT/examples/settings.cob" "$ROOT/src/tt_engine.cob" "$ROOT/src/tt_error.cob" -L "$CARGO_TARGET_DIR/debug" -lthinkthen_c
cobc $COMMON -I "$ROOT/copybooks" -A "$FLAGS" -o "$TARGET/types" "$ROOT/examples/types.cob" "$ROOT/src/tt_validate.cob" "$TARGET/ttjson.o" "$TARGET/ttshape.o" -L "$CARGO_TARGET_DIR/debug" -lthinkthen_c -lm
cobc $COMMON -I "$ROOT/copybooks" -A "$FLAGS" -o "$TARGET/door" "$ROOT/checks/door.cob" "$ROOT/src/tt_engine.cob" "$ROOT/src/tt_call.cob" "$ROOT/src/tt_files.cob" "$ROOT/src/tt_error.cob" "$TARGET/ttjson.o" "$TARGET/ttshape.o" -L "$CARGO_TARGET_DIR/debug" -lthinkthen_c
cobc $COMMON -I "$ROOT/copybooks" -A "$FLAGS" -o "$TARGET/proofs" "$ROOT/checks/proofs.cob" "$ROOT/src/tt_engine.cob" "$ROOT/src/tt_call.cob" "$ROOT/src/tt_files.cob" "$ROOT/src/tt_decide.cob" "$ROOT/src/tt_plan.cob" "$ROOT/src/tt_validate.cob" "$ROOT/src/tt_error.cob" "$TARGET/ttjson.o" "$TARGET/ttshape.o" -L "$CARGO_TARGET_DIR/debug" -lthinkthen_c -lm
cobc $COMMON -I "$ROOT/copybooks" -A "$FLAGS" -o "$TARGET/failure" "$ROOT/checks/failure.cob" "$ROOT/src/tt_engine.cob" "$ROOT/src/tt_call.cob" "$ROOT/src/tt_files.cob" "$ROOT/src/tt_decide.cob" "$ROOT/src/tt_error.cob" "$TARGET/ttjson.o" "$TARGET/ttshape.o" -L "$CARGO_TARGET_DIR/debug" -lthinkthen_c -lm
cobc $COMMON -I "$ROOT/copybooks" -A "$FLAGS" -o "$TARGET/matrix" "$ROOT/checks/matrix.cob" "$ROOT/src/tt_decide.cob" "$ROOT/src/tt_error.cob" "$TARGET/ttjson.o" "$TARGET/ttshape.o" -L "$CARGO_TARGET_DIR/debug" -lthinkthen_c -lm
cobc $COMMON -I "$ROOT/copybooks" -A "$FLAGS" -o "$TARGET/files" "$ROOT/checks/files.cob" "$ROOT/src/tt_engine.cob" "$ROOT/src/tt_call.cob" "$ROOT/src/tt_files.cob" "$ROOT/src/tt_error.cob" "$TARGET/ttjson.o" "$TARGET/ttshape.o" -L "$CARGO_TARGET_DIR/debug" -lthinkthen_c
cc -std=c11 -Wall -Wextra -Werror -pedantic -c "$ROOT/src/tt_counted.c" -o "$TARGET/tt_counted.o"
cobc -x -free -I "$ROOT/copybooks" -o "$TARGET/counted_bounds" \
  "$ROOT/checks/counted_bounds.cob" "$TARGET/tt_counted.o"
"$TARGET/counted_bounds"
cc -std=c11 -Wall -Wextra -Werror -pedantic -I"$REPO/libraries/c/include" -c "$REPO/libraries/ada/checks/typed_boundary.c" -o "$TARGET/typed_boundary.o"
cobc -x -free -I "$ROOT/copybooks" -o "$TARGET/carrier_bounds" \
  "$ROOT/checks/carrier_bounds.cob" "$TARGET/typed_boundary.o" "$TARGET/tt_counted.o"
"$TARGET/carrier_bounds"
python3 "$ROOT/checks/files.py"
python3 "$ROOT/checks/public_types.py"
python3 "$ROOT/checks/installed.py"
python3 "$ROOT/checks/failure.py"
python3 "$ROOT/checks/run_matrix.py"
python3 "$ROOT/checks/negative.py"
python3 "$ROOT/checks/native_parity.py"
echo 'COBOL package gate passed'
