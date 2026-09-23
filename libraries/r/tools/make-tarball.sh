#!/usr/bin/env bash
# Build the R package's tarball with the contract crates and the whole
# registry dependency tree vendored in.
#
# The package's src/rust/Cargo.toml points at ../../../../../contract and
# ../../../../../standin for in-tree builds (R CMD INSTALL from
# libraries/r/thinkthen). That path cannot resolve once the package leaves
# the repository, so a tarball build stages a copy with the crates under
# src/rust/vendor/ and rewrites the two path lines. The vendored copies
# keep the repository's relative layout (vendor/contract,
# vendor/standin, vendor/crates/thinkthen-core), so the crates' own
# relative paths keep resolving.
#
# The fourth review's fresh-install probe caught the tarball installing
# with an empty CARGO_HOME and --offline against no vendored registry:
# "no matching package named serde". The tarball now also vendors every
# registry dependency (cargo vendor) under vendor/registry with a
# rust/.cargo/config.toml redirecting crates-io there, resolves
# thinkthen-core's workspace inheritance into concrete values, and skips
# the document step (the wrappers ship in the tarball; the install never
# builds the debug binary that step pulled in).
#
# Usage:
#   tools/make-tarball.sh               stage, vendor, R CMD build
#   tools/make-tarball.sh --stage-only  stage and print the result
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

# The vendored trees are their own workspace roots (contract and
# standin each declare one), and cargo refuses multiple roots in one
# tree: the package's own [workspace] section excludes them.
sed -i '/^\[workspace\]$/a exclude = ["vendor"]' \
  "$STAGE/thinkthen/src/rust/Cargo.toml"

grep -n "vendor/" "$STAGE/thinkthen/src/rust/Cargo.toml"
for check in vendor/contract/Cargo.toml vendor/standin/Cargo.toml vendor/crates/thinkthen-core/Cargo.toml; do
  test -f "$STAGE/thinkthen/src/rust/$check" || { echo "missing $check" >&2; exit 1; }
done
# The vendored crates' own relative paths must still resolve in the stage.
grep -q 'path = "../crates/thinkthen-core"' "$STAGE/thinkthen/src/rust/vendor/contract/Cargo.toml"
grep -q 'path = "../contract"' "$STAGE/thinkthen/src/rust/vendor/standin/Cargo.toml"

# Resolve thinkthen-core's workspace inheritance into concrete values:
# the repository root's workspace is not present in the tarball, so
# `edition.workspace = true` has nothing to inherit from and the build
# dies parsing the manifest. The values are read from the root manifest
# at build time, not hardcoded here.
CORE="$STAGE/thinkthen/src/rust/vendor/crates/thinkthen-core/Cargo.toml"
EDITION=$(sed -n 's/^edition = "\(.*\)"$/\1/p' "$ROOT/Cargo.toml" | head -1)
RUST_VERSION=$(sed -n 's/^rust-version = "\(.*\)"$/\1/p' "$ROOT/Cargo.toml" | head -1)
test -n "$EDITION" && test -n "$RUST_VERSION" || { echo "could not read the workspace values" >&2; exit 1; }
sed -i \
  -e "s/^edition.workspace = true$/edition = \"$EDITION\"/" \
  -e "s/^rust-version.workspace = true$/rust-version = \"$RUST_VERSION\"/" \
  "$CORE"
# The [lints] table inherits too; the install needs no workspace lints.
python3 - "$CORE" << 'PYEOF'
import sys
path = sys.argv[1]
lines = open(path).read().splitlines(keepends=True)
out = []
skipping = False
for line in lines:
    if line.strip() == "[lints]":
        skipping = True
        continue
    if skipping:
        if line.startswith((" ", "\t")) or line.strip() == "workspace = true":
            continue
        skipping = False
    out.append(line)
open(path, "w").write("".join(out))
PYEOF
grep -q "workspace = true" "$CORE" && { echo "unresolved workspace inheritance remains in thinkthen-core" >&2; exit 1; }

# Vendor the registry dependency tree. This one step may use the
# network when the repository's src/.cargo cache lacks a crate of the
# graph (a maintainer step); the tarball it produces installs offline.
mkdir -p "$STAGE/thinkthen/src/rust/vendor/registry"
(cd "$STAGE/thinkthen/src/rust" && \
  CARGO_HOME="$PKG/src/.cargo" cargo vendor --locked vendor/registry \
  > "$STAGE/vendor-config.txt")
grep -q 'directory = "vendor/registry"' "$STAGE/vendor-config.txt" \
  || grep -q 'vendor/registry' "$STAGE/vendor-config.txt" \
  || { echo "cargo vendor produced an unexpected config" >&2; cat "$STAGE/vendor-config.txt" >&2; exit 1; }

# The config cargo printed, pointed at the vendor tree, installed where
# Makevars' CARGO_HOME finds it.
mkdir -p "$STAGE/thinkthen/src/rust/.cargo"
cat > "$STAGE/thinkthen/src/rust/.cargo/config.toml" << EOF
[source.crates-io]
replace-with = "vendored-sources"

[source.vendored-sources]
directory = "vendor/registry"
EOF

# The wrappers ship in the tarball; the install skips the document step.
test -f "$STAGE/thinkthen/R/extendr-wrappers.R" || { echo "the tarball must ship R/extendr-wrappers.R" >&2; exit 1; }

if [ "$stage_only" = yes ]; then
  echo "stage-only: the vendored crates, the resolved core manifest, and the registry tree all check out"
  exit 0
fi

# The tarball is packed directly from the stage, not through
# R CMD build: build strips dotfiles (.gitignore and friends) from the
# vendored registry trees, and cargo verifies each vendored file against
# its .cargo-checksum.json, so a stripped file fails the install with a
# checksum error. R CMD INSTALL accepts this layout unchanged.
tarball="$STAGE/thinkthen_$(grep -m1 '^Version:' "$STAGE/thinkthen/DESCRIPTION" | sed 's/Version: *//;s/ *$//').tar.gz"
(cd "$STAGE" && GZIP=-9 tar -czf "$(basename "$tarball")" thinkthen)
tar -tzf "$tarball" > "$STAGE/list.txt"
grep -q "thinkthen/src/rust/vendor/contract/Cargo.toml" "$STAGE/list.txt"
grep -q "thinkthen/src/rust/vendor/registry" "$STAGE/list.txt" || { echo "the registry tree is missing from the tarball" >&2; exit 1; }
grep -q "thinkthen/src/rust/.cargo/config.toml" "$STAGE/list.txt" || { echo "the vendored-sources config is missing from the tarball (R CMD build strips dotdirs)" >&2; exit 1; }
cp "$tarball" "$ROOT/libraries/r/"
sha256sum "$ROOT/libraries/r/$(basename "$tarball")"
echo "tarball built with the contract crates and the registry tree vendored"
