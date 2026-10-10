#!/usr/bin/env bash
# The R surface's check (ticket 0108). The surfaces rung passes its loopback
# port as $1; each R test file still gets its own backend through
# tests/with-backend.sh (decision 17). A missing R, R package, or cargo
# cache entry exits 77, which the rung reports as not run, never as a pass.
[ -n "${BASH_VERSION:-}" ] || exec bash "$0" "$@"
# The real key never reaches a test (decision 18).
[ -z "${THINKTHEN_API_KEY+set}" ] || exec env -u THINKTHEN_API_KEY bash "$0" "$@"
set -euo pipefail
cd -- "$(dirname -- "$0")"
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress|smoke) ;; *) echo "r: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS
here=$PWD
root=$(cd ../.. && pwd)
rust=thinkthen/src/rust
. "$root/sdlc/scripts/scratch.sh"
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
usage_home
scratch_dir scratch
not_run() { echo "not run: $1" >&2; exit 77; }

echo "== r: the host tools"
command -v Rscript >/dev/null || not_run "no R on this host; install R 4.2 or later"
Rscript -e 'if (getRversion() < "4.2") quit(status = 1)' || not_run "R is older than 4.2"
command -v cargo >/dev/null || not_run "no cargo on this host"
# The library order of decision 16, as absolute paths.
libs=$here/rlib
for lib in "$HOME/.cache/thinkthen-toolchains/r-library" \
  "$(Rscript -e 'cat(path.expand(strsplit(Sys.getenv("R_LIBS_USER"), ":")[[1]]), sep = "\n")')"; do
  [ -d "$lib" ] && libs=$libs:$(cd "$lib" && pwd)
done
export R_LIBS=$libs
mkdir -p rlib
R_LIBS=$libs Rscript -e 'v <- function(p) tryCatch(packageVersion(p), error = function(e) "0")
  if (v("jsonlite") < "2.0.0") quit(status = 1)
  for (p in c("dplyr", "dbplyr", "purrr", "tidyr", "igraph")) if (identical(v(p), "0")) quit(status = 1)' ||
  not_run "jsonlite 2.0.0 or a tested Suggests package is missing; run libraries/r/tools/setup.sh on a networked machine"
if [ -n "${THINKTHEN_ARTIFACT:-}" ]; then
  [ -f "$THINKTHEN_ARTIFACT" ] && [ ! -L "$THINKTHEN_ARTIFACT" ] || { echo 'r: installed tarball is missing or linked' >&2; exit 1; }
  mkdir -p "$scratch/lib"
  pinned=$(sed -n 's/^channel = "\(.*\)"/\1/p' "$root/rust-toolchain.toml")
  RUSTUP_TOOLCHAIN=$pinned CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$root/target/r" \
    R CMD INSTALL -l "$scratch/lib" "$THINKTHEN_ARTIFACT" >"$scratch/install.log" 2>&1 || { cat "$scratch/install.log" >&2; exit 1; }
  installed_libs="$scratch/lib:$libs"
  R_LIBS="$installed_libs" Rscript -e 'stopifnot(startsWith(find.package("thinkthen"), commandArgs(TRUE)[1]))' "$scratch/lib/"
  . "$root/sdlc/scripts/installed.sh"
  own_panic_hook "$scratch/lib/thinkthen/libs/thinkthen.so"
  python3 - "$root" "$installed_libs" <<'RNATIVE'
import sys
from pathlib import Path
root=Path(sys.argv[1]);sys.path.insert(0,str(root/'libraries/python/tests'))
from native_fixture import run
sys.exit(bool(run('r',['Rscript','--vanilla',str(root/'libraries/r/tests/native_case.R')],root,{'R_LIBS':sys.argv[2]})))
RNATIVE
  echo 'r: check passed, installed'
  exit 0
fi
cargo fetch --locked --offline --manifest-path "$rust/Cargo.toml" >/dev/null 2>&1 ||
  not_run "the cargo cache misses a crate; run cargo fetch --locked --manifest-path libraries/r/thinkthen/src/rust/Cargo.toml on a networked machine"

# The package installs build in the lane's own target folder and keep it, so a second install reuses the build.
lane_target=$root/target/r

if [ "$profile" = smoke ]; then
  smoke_guard
  # The replay smoke (ticket 0335): the package installed in a scratch library, loaded from there.
  CARGO_TARGET_DIR=$lane_target R CMD INSTALL -l "$scratch" thinkthen >"$scratch/install.log" 2>&1 ||
    { cat "$scratch/install.log" >&2; exit 1; }
  cd "$scratch"
  R_LIBS="$scratch:$libs" THINKTHEN_API_KEY=sk-smoke-loopback Rscript -e 'library(thinkthen)
    stopifnot(startsWith(find.package("thinkthen"), commandArgs(TRUE)[1]))
    value <- tt_decide(Sys.getenv("THINKTHEN_TEST_SMOKE_QUESTION"), Sys.getenv("THINKTHEN_TEST_SMOKE_TEXT"))$results[[1]]$value
    cat(sprintf("smoke: %s\n", if (is.null(value) || is.na(value)) "null" else tolower(value)))' "$scratch"
  # Ticket 0439: reuse the installed public consumer for the guide's strict replay.
  mkdir -p "$scratch/sample/recording" "$scratch/home" "$scratch/config" "$scratch/cache"
  # The seed preserves request identity, including the counted loopback URL.
  cp -R "$THINKTHEN_CACHE/." "$scratch/sample/recording/"
  printf '%s' "$THINKTHEN_TEST_SMOKE_TEXT" >"$scratch/sample/report.txt"
  printf '%s' "$THINKTHEN_TEST_SMOKE_QUESTION" >"$scratch/sample/question.txt"
  python3 - "$root/sdlc/scripts" "$scratch" <<'PYCONSUMER'
import sys
from pathlib import Path
sys.path.insert(0, sys.argv[1])
from install_check_consumers import write_consumer
write_consumer(Path(sys.argv[2]), "r", "consumer.R")
PYCONSUMER
  replay=$(env -i PATH="$PATH" HOME="$scratch/home" LANG="${LANG:-C.UTF-8}" \
    LC_ALL="${LC_ALL:-C.UTF-8}" R_LIBS="$scratch:$libs" XDG_CONFIG_HOME="$scratch/config" \
    XDG_CACHE_HOME="$scratch/cache" XDG_STATE_HOME="$XDG_STATE_HOME" \
    THINKTHEN_BASE_URL="$THINKTHEN_BASE_URL" THINKTHEN_API_KEY=sk-smoke-loopback \
    Rscript --vanilla "$scratch/consumer.R" "$scratch/sample")
  [ "$replay" = '{"value":true,"requests_sent":0}' ] || { echo 'r: saved-answer replay failed' >&2; exit 1; }
  echo 'smoke: true'
  exit
fi

echo "== r: generated native result conversions"
python3 "$root/sdlc/generators/results/generate.py" --target r --check

echo "== r: script and source counts"
# R4-19: every cargo call that resolves crates is locked and offline.
calls=$(grep -nE '(^|[;&|(]|then|do) *cargo (build|test|clippy|run|vendor|package|fetch)' \
  thinkthen/tools/config.R check.sh tools/make-tarball.sh | grep -v -- '--locked --offline' || true)
[ -z "$calls" ] || { echo "a cargo call lacks --locked --offline:"; echo "$calls"; exit 1; } >&2
# R1-13: one R_CheckUserInterrupt() call, in the function R_ToplevelExec runs.
held=$(awk '/^ *\/\//{ next } /fn [a-z_]+\(/ { match($0, /fn [a-z_]+/); f = substr($0, RSTART + 3, RLENGTH - 3) }
  /R_CheckUserInterrupt\(\)/ && !/fn R_CheckUserInterrupt/ { n++; at = f } /R_ToplevelExec\([a-z_]+,/ { match($0, /R_ToplevelExec\([a-z_]+/); run = substr($0, RSTART + 15, RLENGTH - 15) }
  END { print n + 0, (at == run) }' $rust/src/*.rs)
[ "$held" = "1 1" ] || { echo "R1-13: expected one R_CheckUserInterrupt() call, inside the R_ToplevelExec callback ($held)" >&2; exit 1; }
# R1-31: one panic guard.
[ "$(rg -n 'catch_unwind\(' "$rust/src" --glob '*.rs' | wc -l)" = 0 ] ||
  { echo "R1-31: the worker catches panics only through thinkthen::contained" >&2; exit 1; }

echo "== r: the Rust half"
(cd "$rust" && cargo fmt --check &&
  cargo clippy --locked --offline --all-targets -- -D warnings &&
  cargo test --locked --offline --lib --quiet)

echo "== r: install the production build"
CARGO_TARGET_DIR=$lane_target R CMD INSTALL -l rlib thinkthen >"$scratch/install.log" 2>&1 || { cat "$scratch/install.log" >&2; exit 1; }
# Ticket 0304 slice 3b, and 0351 on macOS: the package exports no bundled
# SQLite name. Mach-O names carry a leading underscore.
case $(uname -s) in
  Linux) exported=$(nm -D --defined-only rlib/thinkthen/libs/thinkthen.so) ;;
  Darwin) exported=$(nm -gU rlib/thinkthen/libs/thinkthen.so | sed 's/ _/ /') ;;
  *) exported= ;;
esac
leaked=$(printf '%s\n' "$exported" | awk '$3 ~ /^sqlite3_/' | wc -l | tr -d ' ')
[ "$leaked" = 0 ] || { echo "r: the package exports $leaked sqlite3_ names" >&2; exit 1; }

echo "== r: the tests, one backend each"
backend=${CARGO_TARGET_DIR:-$root/target}/debug/conformance-backend
[ -x "$backend" ] || (cd "$root" && cargo build --locked --offline --quiet --package conformance-backend)
if bash tests/with-backend.sh "$backend" tests/hook.R http://192.0.2.1/v1 >"$scratch/guard" 2>&1; then
  echo "the loopback guard ran a non-loopback address" >&2; exit 1
fi
grep -q "refused 192.0.2.1, which is not a loopback address" "$scratch/guard" && ! grep -q "checks passed" "$scratch/guard" ||
  { cat "$scratch/guard" >&2; exit 1; }
if [ "$profile" = stress ]; then
  # interrupt.R checks its 0.5 s promises only here (ticket 0356).
  THINKTHEN_TEST_PROFILE=stress bash tests/with-backend.sh "$backend" tests/interrupt.R
  echo "r: pass, stress"
  exit 0
fi
for file in tests/*.R examples/examples.R examples/slide_check.R; do
  [ "$file" = tests/helper.R ] && continue
  [ "$file" = tests/native_case.R ] && continue
  if [ "$file" = tests/named_backends.R ]; then
    for name in liquid ollama openrouter perplexity typesafe; do
      TT_NAMED_BACKEND=$name TT_TEST_MARKERS='{"captured":"fake-named-r-captured","later":"fake-named-r-later"}' bash tests/with-backend.sh "$backend" "$file"
    done
  else
    bash tests/with-backend.sh "$backend" "$file"
  fi
done

python3 tests/request_surface.py
python3 tests/record_feeds.py
python3 tests/feed_cases.py

echo "== r: the tarball from $(git -C "$root" rev-parse --short HEAD), installed with an empty cargo home"
mkdir -p "$scratch/tree" "$scratch/lib" "$scratch/cargo" "$scratch/out"
git -C "$root" archive HEAD | tar -x -C "$scratch/tree"
# The cargo cache test first, against the builder's cargo home, so a
# vendoring plant reads fail and never not run (R5-38).
tarball=$(CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$scratch/target" "$scratch/tree/libraries/r/tools/make-tarball.sh" "$scratch/out")
# The empty cargo home keeps rustup's own home, and the repository's pinned
# toolchain builds, because a tarball carries no rust-toolchain.toml.
pinned=$(sed -n 's/^channel = "\(.*\)"/\1/p' "$root/rust-toolchain.toml")
RUSTUP_TOOLCHAIN=$pinned CARGO_HOME="$scratch/cargo" CARGO_NET_OFFLINE=true R CMD INSTALL -l "$scratch/lib" "$tarball" >"$scratch/tarball.log" 2>&1 ||
  { cat "$scratch/tarball.log" >&2; exit 1; }
cat >"$scratch/answer.R" <<'EOF'
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
check("the tarball install answers", isTRUE(tt_decide("Is this a complaint?", "I want a refund")$results[[1]]$value))
finish("tarball", 1L)
EOF
R_LIBS="$scratch/lib:$libs" bash tests/with-backend.sh "$backend" "$scratch/answer.R"
# Ticket 0374: the tarball install keeps its own panic hook, and its facts checks pass, the
# token cap variable's refusal before any send among them.
. "$root/sdlc/scripts/installed.sh"
own_panic_hook "$scratch/lib/thinkthen/libs/thinkthen.so"
R_LIBS="$scratch/lib:$libs" Rscript -e 'stopifnot(startsWith(find.package("thinkthen"), commandArgs(TRUE)[1]))' "$scratch/lib/" ||
  { echo "r: the facts run would load thinkthen from outside the tarball install" >&2; exit 1; }
R_LIBS="$scratch/lib:$libs" bash tests/with-backend.sh "$backend" tests/facts.R

native_libs="$scratch/lib:$libs"

python3 - "$root" "$native_libs" >"$scratch/native-vendored.log" <<'RNATIVE'
import sys
from pathlib import Path
root=Path(sys.argv[1]);sys.path.insert(0,str(root/'libraries/python/tests'))
from native_fixture import run
sys.exit(bool(run('r',['Rscript','--vanilla',str(root/'libraries/r/tests/native_case.R')],root,{'R_LIBS':sys.argv[2]})))
RNATIVE

echo "== r: outside the repository, against a stand-in for crates.io with no network (tickets 0128, 0395)"
# R-universe builds this package's folder alone. The copy has no repository
# around it, so the published shape asks crates.io for the engine. A private
# CARGO_HOME replaces crates-io with a directory source: the tarball's vendored
# registry tree plus its packed thinkthen crate. Cargo sees thinkthen as a
# registry package, as on crates.io, and CARGO_NET_OFFLINE refuses any fetch.
# A [patch.crates-io] path would keep the lock's path entry and hide 0395.
mkdir -p "$scratch/outside" "$scratch/home" "$scratch/stand-in" "$scratch/outside-lib"
tar -xzf "$tarball" -C "$scratch/stand-in" thinkthen/src/rust/vendor
registry=$scratch/stand-in/thinkthen/src/rust/vendor/registry
mv "$scratch/stand-in/thinkthen/src/rust/vendor/thinkthen" "$registry/thinkthen"
printf '{"files":{},"package":null}\n' >"$registry/thinkthen/.cargo-checksum.json"
printf '[source.crates-io]\nreplace-with = "stand-in"\n\n[source.stand-in]\ndirectory = "%s"\n' "$registry" >"$scratch/home/config.toml"
cp -R "$scratch/tree/libraries/r/thinkthen" "$scratch/outside/thinkthen"
(cd "$scratch/outside" && RUSTUP_TOOLCHAIN=$pinned CARGO_HOME="$scratch/home" CARGO_TARGET_DIR="$scratch/outside-target" CARGO_NET_OFFLINE=true \
  R CMD INSTALL -l "$scratch/outside-lib" thinkthen >"$scratch/outside.log" 2>&1) ||
  { cat "$scratch/outside.log" >&2; exit 1; }
grep -q '^thinkthen = { version = "=' "$scratch/outside/thinkthen/src/rust/Cargo.toml" ||
  { echo "the outside build kept the path dependency" >&2; exit 1; }
# The lock names the registry engine, and no package appears, goes, or moves.
# The stand-in holds one version of each crate, so only the container proof of
# ticket 0395 shows cargo keeping a pin it could have raised.
lock=$scratch/outside/thinkthen/src/rust/Cargo.lock
grep -A2 '^name = "thinkthen"$' "$lock" | grep -qx 'source = "registry+https://github.com/rust-lang/crates.io-index"' ||
  { echo "the outside build's lock does not take thinkthen from crates.io" >&2; exit 1; }
pins() { awk '/^name = /{ n = $3 } /^version = /{ print n, $3 }' "$1" | sort; }
[ "$(pins "$scratch/tree/libraries/r/thinkthen/src/rust/Cargo.lock")" = "$(pins "$lock")" ] ||
  { echo "the outside build changed a locked package" >&2; exit 1; }
R_LIBS="$scratch/outside-lib:$libs" bash tests/with-backend.sh "$backend" "$scratch/answer.R"
native_libs="$scratch/outside-lib:$libs"

python3 - "$root" "$native_libs" <<'RNATIVE'
import sys
from pathlib import Path
root=Path(sys.argv[1]);sys.path.insert(0,str(root/'libraries/python/tests'))
from native_fixture import run
sys.exit(bool(run('r',['Rscript','--vanilla',str(root/'libraries/r/tests/native_case.R')],root,{'R_LIBS':sys.argv[2]})))
RNATIVE
echo "r: check passed"
