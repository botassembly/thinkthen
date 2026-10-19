#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO=$(CDPATH= cd -- "$ROOT/../.." && pwd)
case "$(uname -s):$(uname -m)" in Linux:x86_64) ;; *) exit 77 ;; esac
case "${THINKTHEN_TEST_PROFILE:-routine}" in routine|full|smoke) ;; stress) exit 77 ;; *) exit 2 ;; esac
for tool in gprbuild gnatmake gcc python3 nm node; do
  command -v "$tool" >/dev/null 2>&1 || { echo "Ada gate missing $tool" >&2; exit 77; }
done
. "$REPO/sdlc/scripts/scratch.sh"
usage_home
scratch_dir fixture_config
export XDG_CONFIG_HOME="$fixture_config"
unset THINKTHEN_API_KEY
export CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER=
export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build --remap-path-prefix=$REPO=/build/thinkthen"
export CFLAGS="${CFLAGS:+$CFLAGS }-ffile-prefix-map=$HOME=/build -ffile-prefix-map=$REPO=/build/thinkthen"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-/tmp/thinkthen-target-ada}
THINKTHEN_C_LIBRARY=${THINKTHEN_C_LIBRARY:-$CARGO_TARGET_DIR/debug/libthinkthen_c.so}
THINKTHEN_C_STATIC_LIBRARY=${THINKTHEN_C_STATIC_LIBRARY:-$CARGO_TARGET_DIR/debug/libthinkthen_c.a}
THINKTHEN_BACKEND_BIN=${THINKTHEN_BACKEND_BIN:-$CARGO_TARGET_DIR/debug/conformance-backend}
export THINKTHEN_C_LIBRARY THINKTHEN_C_STATIC_LIBRARY THINKTHEN_BACKEND_BIN
if [ -n "${THINKTHEN_C_ARTIFACT:-}" ]; then
  . "$REPO/sdlc/scripts/installed.sh"
  scratch_dir native
  tar -xzf "$THINKTHEN_C_ARTIFACT" -C "$native" ./include/thinkthen.h ./lib/libthinkthen.so ./lib/libthinkthen.a
  THINKTHEN_C_LIBRARY=$native/lib/libthinkthen.so
  THINKTHEN_C_STATIC_LIBRARY=$native/lib/libthinkthen.a
fi
if [ -z "${THINKTHEN_C_ARTIFACT:-}" ]; then
  cargo build --locked --offline --manifest-path "$REPO/libraries/c/Cargo.toml" --lib -j2
fi
if [ ! -x "$THINKTHEN_BACKEND_BIN" ]; then
  cargo build --locked --offline --manifest-path "$REPO/Cargo.toml" --package conformance-backend -j2
fi
python3 "$ROOT/checks/generated_names.py"
python3 "$REPO/conformance/children/privacy.py" --sources "$ROOT" --include README.md --include LICENSE --include 'src/*' --include 'Src/*' --include 'Sources/*' --include 'examples/*' --include 'Examples/*' --artifact-env
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
# Install the package once and select the caller for this profile.
if [ "${THINKTHEN_TEST_PROFILE:-routine}" = full ]; then
  python3 "$ROOT/checks/native_session.py" --full
else
  python3 "$ROOT/checks/native_session.py"
fi
echo 'Ada installed typed package gate passed'
