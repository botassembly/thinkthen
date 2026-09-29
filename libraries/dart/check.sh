#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
CHECKS="$ROOT/checks"
FLUTTER="$ROOT/flutter"
TT_DART=${TT_DART:-$(command -v dart || true)}
TT_FLUTTER=${TT_FLUTTER:-$(command -v flutter || true)}
if [ ! -x "$TT_DART" ] || [ ! -x "$TT_FLUTTER" ]; then
  echo 'Dart or Flutter executable unavailable; set TT_DART and TT_FLUTTER' >&2
  exit 77
fi
if ! command -v cargo >/dev/null || ! command -v nm >/dev/null || ! command -v xvfb-run >/dev/null; then
  echo 'Native build or Linux Flutter host tool unavailable' >&2
  exit 77
fi
mkdir -p "$CHECKS/logs" "$CHECKS/scratch" "$FLUTTER/logs" "$FLUTTER/scratch"
python3 "$CHECKS/privacy.py"
node "$ROOT/../../sdlc/scripts/ratchet.mjs" "$ROOT/ratchet.dart.json"
export TT_DART TT_FLUTTER
export TT_NATIVE_LIBRARY="$CHECKS/scratch/libthinkthen.so"
export CARGO_TARGET_DIR=${CARGO_TARGET_DIR:-"$CHECKS/scratch/native-target"}
export CARGO_NET_OFFLINE=true
export CARGO_BUILD_RUSTC_WRAPPER=
export PUB_CACHE=${PUB_CACHE:-"$CHECKS/scratch/pub-cache"}
export FLUTTER_SUPPRESS_ANALYTICS=true
export PATH="$(dirname "$TT_DART"):$(dirname "$TT_FLUTTER"):$PATH"
LOCK=${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-3.lock}
flock -w 180 -E 75 -o "$LOCK" cargo build --offline --release -j2 --manifest-path "$ROOT/../c/Cargo.toml"
cp "$CARGO_TARGET_DIR/release/libthinkthen_c.so" "$TT_NATIVE_LIBRARY"
python3 "$CHECKS/exports.py" "$TT_NATIVE_LIBRARY" "$ROOT/../c/include/thinkthen.h" "$CHECKS/logs/exports.txt"
"$TT_DART" pub get --offline --directory "$ROOT"
"$TT_DART" format --output=none --set-exit-if-changed "$ROOT/lib" "$CHECKS/consumers"
"$TT_DART" analyze "$ROOT/lib"
for consumer in alpha bravo; do
  "$TT_DART" pub get --offline --directory "$CHECKS/consumers/$consumer"
  "$TT_DART" analyze "$CHECKS/consumers/$consumer/bin"
  (cd "$CHECKS/consumers/$consumer" && "$TT_DART" run bin/schema_cases.dart "$ROOT/../../specification/fixtures/types/corpus.json")
  python3 "$CHECKS/run.py" "$consumer"
done
python3 "$CHECKS/schema.py"
python3 "$CHECKS/types.py"
python3 "$CHECKS/request_identity.py" "$CHECKS/expected-requests.json"
python3 "$CHECKS/request_identity.py" "$FLUTTER/expected-requests.json"
python3 "$CHECKS/plant-check.py"
(cd "$FLUTTER" && "$TT_FLUTTER" pub get --offline)
(cd "$FLUTTER/example" && "$TT_FLUTTER" pub get --offline)
"$TT_FLUTTER" analyze --no-pub "$FLUTTER/lib"
(cd "$FLUTTER/example" && "$TT_FLUTTER" analyze --no-pub lib test)
python3 "$FLUTTER/run.py"
python3 "$FLUTTER/plant-check.py"
python3 "$FLUTTER/embedder.py"
