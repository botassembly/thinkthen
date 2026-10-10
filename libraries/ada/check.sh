#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO=$(CDPATH= cd -- "$ROOT/../.." && pwd)
case "$(uname -s):$(uname -m)" in Linux:x86_64) ;; *) exit 77 ;; esac
case "${THINKTHEN_TEST_PROFILE:-routine}" in routine|full|smoke) ;; stress) exit 77 ;; *) exit 2 ;; esac
for tool in gprbuild gnatmake gcc python3 flock nm node; do
  command -v "$tool" >/dev/null 2>&1 || { echo "Ada gate missing $tool" >&2; exit 77; }
done
lock=${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-3.lock}
if [ "${TT_ADA_LOCKED:-0}" != 1 ] && [ "${THINKTHEN_HEAVY_LOCK_HELD:-}" != "$lock" ]; then
  TT_ADA_LOCKED=1 exec flock -w 180 -E 75 -o "$lock" env TT_ADA_LOCKED=1 "$0" "$@"
fi
. "$REPO/sdlc/scripts/scratch.sh"
usage_home
scratch_dir fixture_config
export XDG_CONFIG_HOME="$fixture_config"
unset THINKTHEN_API_KEY
export CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
export CARGO_TARGET_DIR="$REPO/libraries/c/target"
THINKTHEN_C_LIBRARY=${THINKTHEN_C_LIBRARY:-$CARGO_TARGET_DIR/debug/libthinkthen_c.so}
THINKTHEN_C_STATIC_LIBRARY=${THINKTHEN_C_STATIC_LIBRARY:-$CARGO_TARGET_DIR/debug/libthinkthen_c.a}
THINKTHEN_BACKEND_BIN=${THINKTHEN_BACKEND_BIN:-$REPO/target/debug/conformance-backend}
export THINKTHEN_C_LIBRARY THINKTHEN_C_STATIC_LIBRARY THINKTHEN_BACKEND_BIN
if [ -n "${THINKTHEN_C_ARTIFACT:-}" ]; then
  . "$REPO/sdlc/scripts/installed.sh"
  scratch_dir native
  tar -xzf "$THINKTHEN_C_ARTIFACT" -C "$native" ./include/thinkthen.h ./lib/libthinkthen.so ./lib/libthinkthen.a
  THINKTHEN_C_LIBRARY=$native/lib/libthinkthen.so
  THINKTHEN_C_STATIC_LIBRARY=$native/lib/libthinkthen.a
fi
if [ ! -f "$THINKTHEN_C_LIBRARY" ] || [ ! -f "$THINKTHEN_C_STATIC_LIBRARY" ]; then
  cargo build --locked --offline --manifest-path "$REPO/libraries/c/Cargo.toml" --lib -j2
fi
if [ ! -x "$THINKTHEN_BACKEND_BIN" ]; then
  CARGO_TARGET_DIR="$REPO/target" cargo build --locked --offline --manifest-path "$REPO/Cargo.toml" --package conformance-backend -j2
fi
python3 "$ROOT/checks/privacy.py"
for config in "$ROOT"/ratchet.*.json; do node "$REPO/sdlc/scripts/ratchet.mjs" "$config"; done
python3 "$REPO/sdlc/scripts/check-c-exports.py" "$REPO/libraries/c/include/thinkthen.h" "$THINKTHEN_C_LIBRARY"
if [ "${THINKTHEN_TEST_PROFILE:-}" = smoke ]; then
  smoke_guard
  scratch_dir smoke
  cp "$THINKTHEN_C_LIBRARY" "$smoke/libthinkthen.so.0"
  ln -s libthinkthen.so.0 "$smoke/libthinkthen.so"
  gnatmake -q -gnat2022 -I"$ROOT/src" "$ROOT/examples/smoke.adb" -D "$smoke" -o "$smoke/smoke" \
    -largs -L"$smoke" -lthinkthen -Wl,-rpath,"$smoke" >&2
  "$smoke/smoke"
  exit
fi
python3 "$ROOT/checks/native_session.py"
python3 "$ROOT/checks/usage_installed.py" --family ada
# Full generated shared cases belong to the release profile.
if [ "${THINKTHEN_TEST_PROFILE:-routine}" = full ]; then
  python3 "$ROOT/checks/native_session.py" --full
fi
echo 'Ada installed typed package gate passed'
