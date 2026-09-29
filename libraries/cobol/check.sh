#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO=$(CDPATH= cd -- "$ROOT/../.." && pwd)
case "$(uname -s):$(uname -m)" in Linux:x86_64) ;; *) echo 'COBOL gate unavailable on this host' >&2; exit 77 ;; esac
case "${THINKTHEN_TEST_PROFILE:-routine}" in routine|full) ;; stress) exit 77 ;; *) exit 2 ;; esac
for tool in cobc cc cargo flock nm node python3; do
  command -v "$tool" >/dev/null 2>&1 || { echo "COBOL gate missing $tool" >&2; exit 77; }
done
python3 -c 'import jsonschema' >/dev/null 2>&1 || { echo 'COBOL gate missing Python jsonschema' >&2; exit 77; }
if [ "${TT_COBOL_LOCKED:-0}" != 1 ]; then
  TT_COBOL_LOCKED=1 exec flock -w 180 -E 75 -o "${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-3.lock}" env TT_COBOL_LOCKED=1 "$0" "$@"
fi
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
cobc $COMMON -I "$ROOT/copybooks" -A "$FLAGS" -o "$TARGET/door" "$ROOT/checks/door.cob" "$ROOT/src/tt_engine.cob" "$ROOT/src/tt_call.cob" "$ROOT/src/tt_error.cob" -L "$CARGO_TARGET_DIR/debug" -lthinkthen_c
cobc $COMMON -I "$ROOT/copybooks" -A "$FLAGS" -o "$TARGET/failure" "$ROOT/checks/failure.cob" "$ROOT/src/tt_decide.cob" "$ROOT/src/tt_error.cob" -L "$CARGO_TARGET_DIR/debug" -lthinkthen_c
cobc $COMMON -I "$ROOT/copybooks" -A "$FLAGS" -o "$TARGET/matrix" "$ROOT/checks/matrix.cob" "$ROOT/src/tt_decide.cob" "$ROOT/src/tt_error.cob" -L "$CARGO_TARGET_DIR/debug" -lthinkthen_c
python3 "$ROOT/checks/public_types.py"
python3 "$ROOT/checks/installed.py"
python3 "$ROOT/checks/failure.py"
python3 "$ROOT/checks/run_matrix.py"
python3 "$ROOT/checks/negative.py"
echo 'COBOL package gate passed'
