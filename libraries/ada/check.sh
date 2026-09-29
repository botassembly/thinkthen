#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO=$(CDPATH= cd -- "$ROOT/../.." && pwd)
case "$(uname -s):$(uname -m)" in Linux:x86_64) ;; *) echo 'Ada gate unavailable on this host' >&2; exit 77 ;; esac
case "${THINKTHEN_TEST_PROFILE:-routine}" in routine|full) ;; stress) exit 77 ;; *) exit 2 ;; esac
for tool in gprbuild gnatmake gcc cargo flock nm node python3; do
  command -v "$tool" >/dev/null 2>&1 || { echo "Ada gate missing $tool" >&2; exit 77; }
done
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
  for tool in tar cmp ldd; do command -v "$tool" >/dev/null 2>&1 || exit 77; done
  [ -f "$THINKTHEN_ARTIFACT" ] && [ -f "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'Ada installed: wrapper or C archive missing' >&2; exit 1; }
fi
python3 -c 'import jsonschema' >/dev/null 2>&1 || { echo 'Ada gate missing Python jsonschema' >&2; exit 77; }
if [ "${TT_ADA_LOCKED:-0}" != 1 ]; then
  TT_ADA_LOCKED=1 exec flock -w 180 -E 75 -o "${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-3.lock}" env TT_ADA_LOCKED=1 "$0" "$@"
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
  echo 'Ada installed release PASS: five typed rows and three literal sends'
  exit 0
fi
unset THINKTHEN_API_KEY
export CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
export CARGO_TARGET_DIR="$REPO/libraries/c/target"
NATIVE="$CARGO_TARGET_DIR/debug/libthinkthen_c.so"
TARGET="$ROOT/checks/target"
mkdir -p "$TARGET/legacy"
python3 "$ROOT/checks/privacy.py"
for config in "$ROOT"/ratchet.*.json; do node "$REPO/sdlc/scripts/ratchet.mjs" "$config"; done
cargo build --locked --offline --manifest-path "$REPO/libraries/c/Cargo.toml" --lib -j2
ln -sf "$NATIVE" "$CARGO_TARGET_DIR/debug/libthinkthen.so.0"
CARGO_TARGET_DIR="$REPO/target" cargo build --locked --offline --manifest-path "$REPO/Cargo.toml" --package conformance-backend -j2
THINKTHEN_PORTABLE_NATIVE="$CARGO_TARGET_DIR/debug" THINKTHEN_BACKEND_BIN="$REPO/target/debug/conformance-backend" python3 "$ROOT/checks/portable_batch.py"
ln -sf "$NATIVE" "$TARGET/libthinkthen.so.0"
python3 "$ROOT/checks/exports.py" "$NATIVE" "$REPO/libraries/c/include/thinkthen.h" "$TARGET/exports.txt"
gprbuild -P "$ROOT/thinkthen.gpr" -j2
for name in door failure package_bulk; do
  gnatmake -gnat2022 -I"$ROOT/src" "$ROOT/checks/$name.adb" -D "$TARGET" -o "$TARGET/$name" -largs -L"$CARGO_TARGET_DIR/debug" -lthinkthen_c
done
for name in main direct; do
  gnatmake -gnat2022 -I"$ROOT/checks/legacy" "$ROOT/checks/legacy/$name.adb" -D "$TARGET/legacy" -o "$TARGET/legacy/$name" -largs -L"$CARGO_TARGET_DIR/debug" -lthinkthen_c
done
python3 "$ROOT/checks/types.py"
python3 "$ROOT/checks/installed.py"
python3 "$ROOT/checks/failure.py"
python3 "$ROOT/checks/typed_matrix.py"
python3 "$ROOT/checks/run_matrix.py"
python3 "$ROOT/checks/negative.py"
echo 'Ada package gate passed'
