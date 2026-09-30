#!/bin/sh
set -eu
ROOT=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
CHECKS="$ROOT/checks"
FLUTTER="$ROOT/flutter"
TT_DART=${TT_DART:-$(command -v dart || true)}
TT_FLUTTER=${TT_FLUTTER:-$(command -v flutter || true)}
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
. "$ROOT/../../sdlc/scripts/scratch.sh"
usage_home
case ${THINKTHEN_ARTIFACT:-} in
  */thinkthen-flutter-*.tar.gz | thinkthen-flutter-*.tar.gz)
    [ -x "$TT_DART" ] && [ -x "$TT_FLUTTER" ] || { echo 'Flutter installed: Dart or Flutter executable unavailable' >&2; exit 77; }
    [ -f "${THINKTHEN_C_ARTIFACT:-}" ] && [ -f "${THINKTHEN_DART_ARTIFACT:-}" ] || { echo 'Flutter installed: C or Dart archive missing' >&2; exit 1; }
    for tool in tar cmp nm readelf python3 flock xvfb-run ninja pkg-config; do command -v "$tool" >/dev/null 2>&1 || exit 77; done
    unset THINKTHEN_API_KEY
    . "$ROOT/../../sdlc/scripts/scratch.sh"
    . "$ROOT/../../sdlc/scripts/installed.sh"
    installed_scratch
    mkdir "$scratch/dart" "$scratch/flutter-stage" "$scratch/native"
    tar -xzf "$THINKTHEN_DART_ARTIFACT" -C "$scratch/dart"
    tar -xzf "$THINKTHEN_ARTIFACT" -C "$scratch/flutter-stage"
    tar -xzf "$THINKTHEN_C_ARTIFACT" -C "$scratch/native"
    tar -xOzf "$THINKTHEN_C_ARTIFACT" ./include/thinkthen.h | cmp - "$scratch/native/include/thinkthen.h"
    tar -xOzf "$THINKTHEN_C_ARTIFACT" ./lib/libthinkthen.so | cmp - "$scratch/native/lib/libthinkthen.so"
    python3 "$ROOT/../../sdlc/scripts/check-c-exports.py" "$scratch/native/include/thinkthen.h" "$scratch/native/lib/libthinkthen.so"
    readelf -d "$scratch/native/lib/libthinkthen.so" | grep -q 'Library soname: \[libthinkthen.so.0\]'
    [ "$(readlink "$scratch/native/lib/libthinkthen.so.0")" = libthinkthen.so ] || exit 1
    export TT_DART TT_FLUTTER PUB_CACHE=${PUB_CACHE:-"$HOME/.pub-cache"} FLUTTER_SUPPRESS_ANALYTICS=true
    [ -d "$PUB_CACHE/hosted/pub.dev/ffi-2.2.0" ] || { echo 'Flutter installed: offline ffi 2.2.0 cache missing' >&2; exit 77; }
    export PATH="$(dirname "$TT_DART"):$(dirname "$TT_FLUTTER"):$PATH"
    flock -w 180 -E 75 -o "${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-3.lock}" \
      python3 "$FLUTTER/installed.py" "$scratch/dart" "$scratch/flutter-stage" "$scratch/native"
    echo 'Flutter installed release PASS: one decoded-body Linux app call'
    exit 0 ;;
esac
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
  [ -x "$TT_DART" ] || { echo 'Dart installed: Dart executable unavailable' >&2; exit 77; }
  [ -f "${THINKTHEN_C_ARTIFACT:-}" ] || { echo 'Dart installed: C archive missing' >&2; exit 1; }
  for tool in cargo nm readelf python3 flock; do command -v "$tool" >/dev/null 2>&1 || exit 77; done
  unset THINKTHEN_API_KEY
  . "$ROOT/../../sdlc/scripts/scratch.sh"
  . "$ROOT/../../sdlc/scripts/installed.sh"
  installed_unpack
  package=$scratch
  scratch_dir native
  tar -xzf "$THINKTHEN_C_ARTIFACT" -C "$native"
  python3 "$ROOT/../../sdlc/scripts/check-c-exports.py" "$native/include/thinkthen.h" "$native/lib/libthinkthen.so"
  readelf -d "$native/lib/libthinkthen.so" | grep -q 'Library soname: \[libthinkthen.so.0\]'
  [ "$(readlink "$native/lib/libthinkthen.so.0")" = libthinkthen.so ] || exit 1
  export PUB_CACHE=${PUB_CACHE:-"$HOME/.pub-cache"}
  [ -d "$PUB_CACHE/hosted/pub.dev/ffi-2.2.0" ] || { echo 'Dart installed: offline ffi 2.2.0 cache missing' >&2; exit 77; }
  scratch_dir locked_package
  tar -xzf "$THINKTHEN_ARTIFACT" -C "$locked_package"
  "$TT_DART" pub get --offline --enforce-lockfile --directory "$locked_package"
  cmp "$package/pubspec.lock" "$locked_package/pubspec.lock" || { echo 'Dart installed: archived lock changed' >&2; exit 1; }
  flock -w 180 -E 75 -o "${THINKTHEN_HEAVY_LOCK:-/run/user/1000/thinkthen-codex-7.lock}" \
    env CARGO_TARGET_DIR="$ROOT/../../target" CARGO_NET_OFFLINE=true CARGO_BUILD_RUSTC_WRAPPER= RUSTC_WRAPPER= \
    cargo build --locked --offline --manifest-path "$ROOT/../../Cargo.toml" --package conformance-backend -j2
  TT_DART="$TT_DART" TT_NATIVE_LIBRARY="$native/lib/libthinkthen.so" \
    THINKTHEN_RELEASE_DART_DIR="$package" THINKTHEN_RELEASE_C_DIR="$native" \
    THINKTHEN_BACKEND_BIN="$ROOT/../../target/debug/conformance-backend" \
    python3 "$CHECKS/portable_batch.py"
  python3 "$CHECKS/release_plants.py" "$package" "$native"
  echo 'Dart installed release PASS: five typed rows and three literal requests'
  exit 0
fi
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
python3 "$CHECKS/public_types.py"
python3 "$CHECKS/portable_batch.py"
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
