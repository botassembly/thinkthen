#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO=$(CDPATH= cd -- "$ROOT/../.." && pwd)
case "$(uname -s):$(uname -m)" in Linux:x86_64) ;; *) echo 'Ada gate unavailable on this host' >&2; exit 77 ;; esac
case "${THINKTHEN_TEST_PROFILE:-routine}" in routine|full) ;; stress) exit 77 ;; *) exit 2 ;; esac
for tool in gprbuild gnatmake gcc cargo flock nm node python3; do
  command -v "$tool" >/dev/null 2>&1 || { echo "Ada gate missing $tool" >&2; exit 77; }
done
python3 -c 'import jsonschema' >/dev/null 2>&1 || { echo 'Ada gate missing Python jsonschema' >&2; exit 77; }
if [ "${TT_ADA_LOCKED:-0}" != 1 ]; then
  TT_ADA_LOCKED=1 exec flock -w 180 -E 75 -o "${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-3.lock}" env TT_ADA_LOCKED=1 "$0" "$@"
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
