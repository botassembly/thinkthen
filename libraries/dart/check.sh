#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
TT_DART=${TT_DART:-$(command -v dart || echo "$HOME/.local/opt/flutter/bin/dart")}
TT_FLUTTER=${TT_FLUTTER:-$(command -v flutter || echo "$HOME/.local/opt/flutter/bin/flutter")}
[ -x "$TT_DART" ] && [ -x "$TT_FLUTTER" ] || exit 77
python3 "$ROOT/checks/privacy.py"
node "$ROOT/../../sdlc/scripts/ratchet.mjs" "$ROOT/ratchet.dart.json"
if [ "${1:-}" = --native-assets ]; then
  shift
  [ "$#" -eq 3 ] || { echo 'usage: check.sh --native-assets NATIVE_LIBRARY SCRATCH PUB_CACHE' >&2; exit 2; }
  exec python3 "$ROOT/checks/installed_native_assets.py" "$TT_DART" "$TT_FLUTTER" "$1" "$2" "$3"
fi
. "$ROOT/../../sdlc/scripts/scratch.sh"
. "$ROOT/../../sdlc/scripts/installed.sh"
usage_home
scratch_dir installed
native_install "$(cd "$ROOT/../.." && pwd)" "$installed/native"
exec python3 "$ROOT/checks/installed_native_assets.py" "$TT_DART" "$TT_FLUTTER" "$installed/native/lib/libthinkthen.so" "$installed/consumer" "${PUB_CACHE:-$HOME/.pub-cache}"
