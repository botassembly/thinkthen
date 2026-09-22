#!/usr/bin/env bash
# Build the R package's tarball with the contract crates copied in.
#
# The package's src/rust/Cargo.toml points at ../../../../../contract and
# ../../../../../standin for in-tree builds (R CMD INSTALL from
# libraries/r/thinkthen). That path cannot resolve once the package leaves
# the repository, so a tarball build stages a copy with the crates vendored
# under src/rust/vendor/ and rewrites the two path lines. The vendored
# copies keep the repository's relative layout (vendor/contract,
# vendor/standin, vendor/crates/thinkthen-core), so the crates' own
# relative paths keep resolving.
#
# Usage:
#   tools/make-tarball.sh               stage, rewrite, R CMD build
#   tools/make-tarball.sh --stage-only  stage and rewrite, print the result
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/../../.." && pwd)
PKG="$ROOT/libraries/r/thinkthen"
STAGE=$(mktemp -d)
trap 'rm -rf "$STAGE"' EXIT

stage_only=no
if [ "${1:-}" = "--stage-only" ]; then
  stage_only=yes
fi

mkdir -p "$STAGE/thinkthen"
tar -C "$PKG" --exclude=target --exclude=.git --exclude=.cargo -cf - . \
  | tar -C "$STAGE/thinkthen" -xf -

mkdir -p "$STAGE/thinkthen/src/rust/vendor"
for crate in contract standin crates/thinkthen-core; do
  mkdir -p "$STAGE/thinkthen/src/rust/vendor/$(dirname "$crate")"
  tar -C "$ROOT" --exclude=target --exclude=.git --exclude=.cargo -cf - "$crate" \
    | tar -C "$STAGE/thinkthen/src/rust/vendor" -xf -
done

sed -i \
  -e "s|path = '../../../../../contract'|path = 'vendor/contract'|" \
  -e "s|path = '../../../../../standin'|path = 'vendor/standin'|" \
  "$STAGE/thinkthen/src/rust/Cargo.toml"

grep -n "vendor/" "$STAGE/thinkthen/src/rust/Cargo.toml"
for check in vendor/contract/Cargo.toml vendor/standin/Cargo.toml vendor/crates/thinkthen-core/Cargo.toml; do
  test -f "$STAGE/thinkthen/src/rust/$check" || { echo "missing $check" >&2; exit 1; }
done
# The vendored crates' own relative paths must still resolve in the stage.
grep -q 'path = "../crates/thinkthen-core"' "$STAGE/thinkthen/src/rust/vendor/contract/Cargo.toml"
grep -q 'path = "../contract"' "$STAGE/thinkthen/src/rust/vendor/standin/Cargo.toml"

if [ "$stage_only" = yes ]; then
  echo "stage-only: the tarball's Cargo.toml points at the vendored crates"
  exit 0
fi

(cd "$STAGE" && R CMD build thinkthen >/dev/null)
tarball=$(ls "$STAGE"/thinkthen_*.tar.gz)
tar -tzf "$tarball" > "$STAGE/list.txt"
grep -q "thinkthen/src/rust/vendor/contract/Cargo.toml" "$STAGE/list.txt"
cp "$tarball" "$ROOT/libraries/r/"
sha256sum "$ROOT/libraries/r/$(basename "$tarball")"
echo "tarball built with the contract crates vendored"
