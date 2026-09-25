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
here=$PWD
root=$(cd ../.. && pwd)
rust=thinkthen/src/rust
scratch=$(mktemp -d)
trap 'rm -rf -- "$scratch"' EXIT
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
  for (p in c("dplyr", "tidyr", "igraph")) if (identical(v(p), "0")) quit(status = 1)' ||
  not_run "jsonlite 2.0.0 or a tested Suggests package is missing; run libraries/r/tools/setup.sh on a networked machine"
cargo fetch --locked --offline --manifest-path "$rust/Cargo.toml" >/dev/null 2>&1 ||
  not_run "the cargo cache misses a crate; run cargo fetch --locked --manifest-path libraries/r/thinkthen/src/rust/Cargo.toml on a networked machine"

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
[ "$(grep -c 'catch_unwind(' $rust/src/*.rs | awk -F: '{ n += $2 } END { print n }')" = 1 ] ||
  { echo "R1-31: expected exactly one catch_unwind site" >&2; exit 1; }

echo "== r: the Rust half"
(cd "$rust" && cargo fmt --check &&
  cargo clippy --locked --offline --all-targets -- -D warnings &&
  cargo test --locked --offline --lib --quiet)

echo "== r: install the production build"
R CMD INSTALL -l rlib thinkthen >"$scratch/install.log" 2>&1 || { cat "$scratch/install.log" >&2; exit 1; }

echo "== r: the tests, one backend each"
backend=${CARGO_TARGET_DIR:-$root/target}/debug/conformance-backend
[ -x "$backend" ] || (cd "$root" && cargo build --locked --offline --quiet --package conformance-backend)
if bash tests/with-backend.sh "$backend" tests/hook.R http://192.0.2.1/v1 >"$scratch/guard" 2>&1; then
  echo "the loopback guard ran a non-loopback address" >&2; exit 1
fi
grep -q "refused 192.0.2.1, which is not a loopback address" "$scratch/guard" && ! grep -q "checks passed" "$scratch/guard" ||
  { cat "$scratch/guard" >&2; exit 1; }
for file in tests/*.R examples/examples.R examples/slide_check.R; do
  [ "$file" = tests/helper.R ] && continue
  bash tests/with-backend.sh "$backend" "$file"
done

echo "== r: the tarball from $(git -C "$root" rev-parse --short HEAD), installed with an empty cargo home"
mkdir -p "$scratch/tree" "$scratch/lib" "$scratch/cargo" "$scratch/out"
git -C "$root" archive HEAD | tar -x -C "$scratch/tree"
# The cargo cache test first, against the builder's cargo home, so a
# vendoring plant reads fail and never not run (R5-38).
tarball=$(CARGO_NET_OFFLINE=true CARGO_TARGET_DIR="$scratch/target" "$scratch/tree/libraries/r/tools/make-tarball.sh" "$scratch/out")
CARGO_HOME="$scratch/cargo" CARGO_NET_OFFLINE=true R CMD INSTALL -l "$scratch/lib" "$tarball" >"$scratch/tarball.log" 2>&1 ||
  { cat "$scratch/tarball.log" >&2; exit 1; }
cat >"$scratch/answer.R" <<'EOF'
source(file.path(Sys.getenv("TT_TESTS"), "helper.R"))
check("the tarball install answers", isTRUE(tt_decide("Is this a complaint?", "I want a refund")))
finish("tarball", 1L)
EOF
R_LIBS="$scratch/lib:$libs" bash tests/with-backend.sh "$backend" "$scratch/answer.R"
echo "r: check passed"
