#!/usr/bin/env bash
# Test the installed R source package through its named public calls.
[ -n "${BASH_VERSION:-}" ] || exec bash "$0" "$@"
[ -z "${THINKTHEN_API_KEY+set}" ] || exec env -u THINKTHEN_API_KEY bash "$0" "$@"
set -euo pipefail
cd -- "$(dirname -- "$0")"
root=$(cd ../.. && pwd)
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress|smoke) ;; *) echo "r: unknown profile: $profile" >&2; exit 2 ;; esac
if [ "$profile" = routine ]; then
  export THINKTHEN_CONFORMANCE_IDS="$root/conformance/routine-ids.txt"
else
  unset THINKTHEN_CONFORMANCE_IDS
fi
. "$root/sdlc/scripts/scratch.sh"
usage_home
scratch_dir scratch
not_run() { echo "not run: $1" >&2; exit 77; }
command -v Rscript >/dev/null || not_run "R 4.2 or later is required"
command -v cargo >/dev/null || not_run "cargo is required for the R source package"
libs=${R_LIBS:-$PWD/rlib}
for lib in "$HOME/.cache/thinkthen-toolchains/r-library"; do
  [ ! -d "$lib" ] || libs=$libs:$(cd "$lib" && pwd)
done
export R_LIBS=$libs
Rscript --vanilla -e 'stopifnot(getRversion() >= "4.2", packageVersion("jsonlite") >= "2.0.0")' ||
  not_run "R or jsonlite prerequisites are missing"
python3 "$root/sdlc/generators/results/generate.py" --target r --check
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
  artifact=$THINKTHEN_ARTIFACT
  [ -f "$artifact" ] && [ ! -L "$artifact" ] || { echo 'r: artifact is missing or linked' >&2; exit 1; }
else
  mkdir -p "$scratch/tree" "$scratch/out"
  git -C "$root" archive HEAD | tar -x -C "$scratch/tree"
  artifact=$(CARGO_NET_OFFLINE=true "$scratch/tree/libraries/r/tools/make-tarball.sh" "$scratch/out")
fi
mkdir -p "$scratch/lib"
pinned=$(sed -n 's/^channel = "\(.*\)"/\1/p' "$root/rust-toolchain.toml")
RUSTUP_TOOLCHAIN=$pinned CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$root/target/r}" \
  R CMD INSTALL -l "$scratch/lib" "$artifact" >"$scratch/install.log" 2>&1 || { cat "$scratch/install.log" >&2; exit 1; }
export R_LIBS="$scratch/lib:$libs"
Rscript --vanilla -e 'stopifnot(startsWith(find.package("thinkthen"),commandArgs(TRUE)[1]))' "$scratch/lib/"
. "$root/sdlc/scripts/installed.sh"
own_panic_hook "$scratch/lib/thinkthen/libs/thinkthen.so"
# Bundled SQLite names must not bind another installed package's SQLite.
case $(uname -s) in
  Linux) exported=$(nm -D --defined-only "$scratch/lib/thinkthen/libs/thinkthen.so") ;;
  Darwin) exported=$(nm -gU "$scratch/lib/thinkthen/libs/thinkthen.so" | sed 's/ _/ /') ;;
  *) exported= ;;
esac
leaked=$(printf '%s\n' "$exported" | awk '$3 ~ /^sqlite3_/ { n++ } END { print n+0 }')
[ "$leaked" = 0 ] || { echo "r: bundled SQLite symbols escaped the package" >&2; exit 1; }
backend=${THINKTHEN_TEST_BACKEND:-${CARGO_TARGET_DIR:-$root/target}/debug/conformance-backend}
[ -x "$backend" ] || not_run "the conformance backend is not built"
if [ "$profile" = stress ]; then
  bash tests/with-backend.sh "$backend" tests/interrupt.R
  echo 'r: stress checks passed'
  exit 0
fi
if [ "$profile" = smoke ]; then
  smoke_guard
  THINKTHEN_API_KEY=sk-smoke-loopback Rscript --vanilla -e 'library(thinkthen)
    value <- tt_decide(Sys.getenv("THINKTHEN_TEST_SMOKE_QUESTION"),Sys.getenv("THINKTHEN_TEST_SMOKE_TEXT"))$results[[1]]$value
    cat(sprintf("smoke: %s\n", if (is.null(value)) "null" else tolower(value)))'
  exit 0
fi
python3 - "$root" "$R_LIBS" <<'PY'
import sys
from pathlib import Path
root = Path(sys.argv[1])
sys.path.insert(0,str(root/'libraries/python/tests'))
from native_fixture import run
sys.exit(bool(run('r',['Rscript','--vanilla',str(root/'libraries/r/tests/native_case.R')],root,{'R_LIBS':sys.argv[2]})))
PY
for file in tests/requests.R tests/complete_input_admission.R tests/engine.R tests/facts.R \
  tests/verbs.R tests/files.R tests/recognize.R tests/text.R tests/threshold_strings.R tests/plan_identity.R \
  tests/portable_batch_identity.R tests/profile.R tests/fork.R tests/hook.R tests/startup.R \
  tests/settings_cases.R; do
  bash tests/with-backend.sh "$backend" "$file"
done
for name in liquid ollama openrouter perplexity typesafe; do
  TT_NAMED_BACKEND=$name TT_TEST_MARKERS='{"captured":"fake-named-r-captured","later":"fake-named-r-later"}' \
    bash tests/with-backend.sh "$backend" tests/named_backends.R
done
python3 tests/request_surface.py
python3 tests/record_feeds.py
# These existing ten producer cases add feed framing, not a second full parity run.
python3 tests/feed_cases.py
if [ "$profile" = full ]; then
  bash tests/with-backend.sh "$backend" tests/request_width.R
  bash tests/with-backend.sh "$backend" tests/interrupt.R
fi
echo 'r: installed checks passed'
