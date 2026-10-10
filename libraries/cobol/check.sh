#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
REPO=$(CDPATH= cd -- "$ROOT/../.." && pwd)
case "$(uname -s):$(uname -m)" in Linux:x86_64) ;; *) echo 'COBOL gate unavailable on this host' >&2; exit 77 ;; esac
case "${THINKTHEN_TEST_PROFILE:-routine}" in routine|full|smoke) ;; stress) exit 77 ;; *) exit 2 ;; esac
for tool in cobc cc clang nm python3 tar node; do
  command -v "$tool" >/dev/null 2>&1 || { echo "COBOL gate missing $tool" >&2; exit 77; }
done
. "$REPO/sdlc/scripts/scratch.sh"
usage_home
scratch_dir fixture_config
export XDG_CONFIG_HOME="$fixture_config"
python3 "$REPO/sdlc/generators/results/generate.py" --target cobol --check
python3 "$ROOT/checks/privacy.py"
for config in "$ROOT"/ratchet.*.json; do node "$REPO/sdlc/scripts/ratchet.mjs" "$config"; done
scratch_dir evidence
. "$REPO/sdlc/scripts/installed.sh"
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
  installed_unpack
  package=$scratch
  library=$package/native/lib/libthinkthen.so
  header=$package/native/include/thinkthen.h
  [ -f "$library" ] && [ -f "$header" ] && [ -f "$package/native/lib/libthinkthen.a" ] || {
    echo 'COBOL archive missing bundled native files' >&2; exit 1;
  }
else
  package=$ROOT
  if [ -n "${THINKTHEN_C_LIBRARY:-}" ]; then
    library=$THINKTHEN_C_LIBRARY
    header=${THINKTHEN_C_HEADER:-$REPO/libraries/c/include/thinkthen.h}
  else
    native_install "$REPO" "$evidence/native"
    library=$evidence/native/lib/libthinkthen.so
    header=$evidence/native/include/thinkthen.h
  fi
fi
if [ ! -x "$REPO/target/debug/conformance-backend" ]; then
  CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$REPO/target" cargo build --locked --offline \
    --manifest-path "$REPO/Cargo.toml" --package conformance-backend -j2
fi
python3 "$ROOT/checks/exports.py" "$library" "$header" "$evidence/exports.txt" "$package"
THINKTHEN_SESSION_PACKAGE="$package" THINKTHEN_C_HEADER="$header" THINKTHEN_C_LIBRARY="$library" \
  python3 "$ROOT/checks/session.py"
THINKTHEN_PARITY_PACKAGE="$package" THINKTHEN_C_HEADER="$header" THINKTHEN_C_LIBRARY="$library" \
  python3 "$ROOT/checks/native_parity.py"
THINKTHEN_C_LIBRARY="$library" THINKTHEN_USAGE_COBOL_PACKAGE="$package" THINKTHEN_C_HEADER="$header" \
  python3 "$REPO/libraries/ada/checks/usage_installed.py" --family cobol
printf '%s\n' 'COBOL generated installed session gate PASS'
