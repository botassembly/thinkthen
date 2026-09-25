#!/usr/bin/env bash
# Build the R package's source tarball with the thinkthen crate and the
# whole registry tree vendored in, so it installs with an empty cargo home
# and no network (R4-7, R5-38).
#
# It runs only inside a `git archive` tree of the commit under test, never
# in a Git checkout, so `cargo package` needs no --allow-dirty. The thinkthen
# crate comes from `cargo package`, whose manifest carries no workspace
# inheritance. The registry tree comes from `cargo vendor` against the
# builder's own cargo home. The package's src/rust/.cargo never serves as a
# cargo home for that step.
#
# Usage: tools/make-tarball.sh OUT_DIR
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/../../.." && pwd)
OUT=$(cd "${1:?usage: make-tarball.sh OUT_DIR}" && pwd)
if [ -e "$ROOT/.git" ]; then
  echo "make-tarball: run this inside a git archive tree, not a checkout" >&2
  exit 1
fi
STAGE=$(mktemp -d)
trap 'rm -rf -- "$STAGE"' EXIT
PKG="$STAGE/thinkthen"
cp -R "$ROOT/libraries/r/thinkthen" "$PKG"
rm -rf -- "$PKG/src/rust/.cargo" "$PKG/src/rust/target"

# The crate as cargo publishes it, unpacked where the package's manifest
# will point.
(cd "$ROOT" && cargo package --locked --offline --no-verify -p thinkthen >/dev/null)
crate=$(ls "${CARGO_TARGET_DIR:-$ROOT/target}"/package/thinkthen-*.crate)
mkdir -p "$PKG/src/rust/vendor"
tar -xzf "$crate" -C "$PKG/src/rust/vendor"
mv "$PKG"/src/rust/vendor/thinkthen-* "$PKG/src/rust/vendor/thinkthen"
if grep -q "workspace = true" "$PKG/src/rust/vendor/thinkthen/Cargo.toml"; then
  echo "make-tarball: the packaged thinkthen manifest still inherits from a workspace" >&2
  exit 1
fi

# perl -pi, because GNU and BSD sed spell in-place editing differently (R3-32).
perl -pi -e 's|path = "../../../../../crates/thinkthen"|path = "vendor/thinkthen"|;' \
  -e '$_ .= "exclude = [\"vendor\"]\n" if /^\[workspace\]$/' "$PKG/src/rust/Cargo.toml"
grep -q 'path = "vendor/thinkthen"' "$PKG/src/rust/Cargo.toml"

(cd "$PKG/src/rust" && cargo vendor --locked --offline vendor/registry >/dev/null)
mkdir -p "$PKG/src/rust/.cargo"
printf '%s\n' '[source.crates-io]' 'replace-with = "vendored-sources"' '' \
  '[source.vendored-sources]' 'directory = "vendor/registry"' >"$PKG/src/rust/.cargo/config.toml"
test -f "$PKG/R/extendr-wrappers.R"

# Packed straight from the stage: R CMD build strips dotfiles that cargo's
# vendored checksums name.
version=$(sed -n 's/^Version: *//p' "$PKG/DESCRIPTION")
tarball="$OUT/thinkthen_$version.tar.gz"
(cd "$STAGE" && GZIP=-9 tar -czf "$tarball" thinkthen)
listed=$(tar -tzf "$tarball")
for want in thinkthen/LICENSE thinkthen/src/rust/.cargo/config.toml thinkthen/src/rust/vendor/thinkthen/Cargo.toml; do
  grep -qx "$want" <<<"$listed" || { echo "make-tarball: $want is missing from the tarball" >&2; exit 1; }
done
grep -qx 'License: MIT + file LICENSE' "$PKG/DESCRIPTION"
echo "$tarball"
