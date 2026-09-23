#!/usr/bin/env bash
# The DuckDB surface's package rehearsal (the brief's item 5): build the
# release extension with a pinned glibc baseline, stage it under dist/ as a
# user receives it, load it in a clean official duckdb container at the
# pinned version, run the slide sample exactly as drawn, and remove the
# container. Nothing is published; the install is from the local file only.
#
# Why zig: `make release` builds against this box's glibc 2.39 and the
# official image (Debian bookworm, glibc 2.36) refuses to load it —
# GLIBC_2.39 not found. The zigbuild pins the artifact at glibc 2.28
# (manylinux_2_28, the common Linux binary baseline), which loads in the
# official image and on older distributions. Needs: zig, cargo-zigbuild.
#
# The container is disposable: created as dbpkg211-duckdb, started once,
# removed with docker rm -f -v at the end.
set -euo pipefail

# This rehearsal cross-builds for Linux (glibc 2.28) with cargo-zigbuild.
# Experimental on macOS: build natively there instead; the Mac build and
# its verification commands are recorded in
# sdlc/records/surfaces-notes/NOTES-packaging.md.
if [ "$(uname -s)" = Darwin ]; then
  echo "this rehearsal cross-builds for Linux; on macOS build natively (see the packaging notes)" >&2
  exit 1
fi
cd "$(dirname "$0")"

VERSION=$(tail -1 ../../VERSION)
CONTAINER=dbpkg211-duckdb
cleanup() {
  docker rm -f -v "$CONTAINER" >/dev/null 2>&1 || true
}
trap cleanup EXIT

# The one DuckDB version pin (tools/version.env).
source "$(dirname "$0")/tools/version.env"
# The duckdb/duckdb:1.5.5 digest was not resolvable offline at pinning
# time (no local copy); the pull below prints the resolved digest to
# record. The pinning story is in scripts/gate-hermeticity.md.
IMAGE="duckdb/duckdb:${DUCKDB_VERSION#v}"
# The build target follows the host (review 3: x86_64 was hard-coded);
# the glibc-2.28 floor stays the Linux x86_64 default.
TARGET="${DUCKDB_TARGET:-x86_64-unknown-linux-gnu.2.28}"
case "$(uname -m)" in
  arm64|aarch64) TARGET="${DUCKDB_TARGET:-aarch64-unknown-linux-gnu.2.28}" ;;
esac
LIB="target/${TARGET%%.*}/release/libthinkthen.so"
WORK=dist/.rehearsal
SLIDE=tools/slide.sql

echo "== duckdb package: build the release extension at glibc 2.28"
DUCKDB_EXTENSION_NAME=thinkthen DUCKDB_EXTENSION_MIN_DUCKDB_VERSION="$DUCKDB_VERSION" \
  cargo zigbuild --release --target "$TARGET" 2>&1 | tail -2
test -s "$LIB"

echo "== duckdb package: stage dist/ (metadata footer appended)"
rm -rf dist
mkdir -p "$WORK"
configure/venv/bin/python3 extension-ci-tools/scripts/append_extension_metadata.py \
  -l "$LIB" \
  -o dist/thinkthen.duckdb_extension \
  -n thinkthen \
  -dv "$DUCKDB_VERSION" \
  -evf configure/extension_version.txt \
  -pf configure/platform.txt --abi-type C_STRUCT_UNSTABLE >/dev/null
sha256sum dist/thinkthen.duckdb_extension
objdump -T "$LIB" | grep -o 'GLIBC_[0-9.]*' | sort -V | uniq | tail -1

cat > dist/README.md << EOF
# thinkthen for DuckDB ${VERSION}

Built for DuckDB "$DUCKDB_VERSION", the pinned version; the host CLI and the extension
must move together. The Linux binary is pinned to glibc 2.28
(manylinux_2_28), so it loads in the official duckdb image and on older
distributions.

    duckdb -unsigned
    LOAD '/path/to/thinkthen.duckdb_extension';

The community-repository form (\`INSTALL thinkthen FROM community;\`) awaits
the signed community build. Nothing was published from this rehearsal.
EOF

echo "== duckdb package: fixture parquet with the stock "$DUCKDB_VERSION" CLI"
./duckdb-bin/duckdb -c "
CREATE TABLE tickets(id INTEGER, body VARCHAR);
INSERT INTO tickets VALUES
 (1, 'I was charged twice and I want a refund today.'),
 (2, 'Please refund my shipping label fee.'),
 (3, 'Thanks, the fix worked.'),
 (4, 'Maybe look at my billing question later.');
COPY tickets TO '$PWD/$WORK/tickets.parquet' (FORMAT parquet);
" >/dev/null
ls -l "$WORK/tickets.parquet"

printf "SELECT 'duckdb ' || version() AS duckdb;\nLOAD '/pkg/thinkthen.duckdb_extension';\n" > "$WORK/load.sql"

echo "== duckdb package: clean container from the official image"
if ! docker image inspect "$IMAGE" >/dev/null 2>&1; then
  docker pull "$IMAGE"
  echo "image digest to record in the pinning story: $(docker image inspect --format '{{index .RepoDigests 0}}' "$IMAGE")"
else
  echo "image present: $(docker image inspect --format '{{index .RepoDigests 0}}' "$IMAGE")"
fi
docker rm -f -v "$CONTAINER" >/dev/null 2>&1 || true
docker create --name "$CONTAINER" -i -w /pkg -e ENGINE_NULL=1 --entrypoint /duckdb "$IMAGE" \
  -unsigned -init /pkg/load.sql >/dev/null
docker cp dist/thinkthen.duckdb_extension "$CONTAINER":/pkg/thinkthen.duckdb_extension
docker cp "$WORK/tickets.parquet" "$CONTAINER":/pkg/tickets.parquet
docker cp "$WORK/load.sql" "$CONTAINER":/pkg/load.sql
docker cp "$SLIDE" "$CONTAINER":/pkg/slide.sql

echo "== duckdb package: the slide sample, as drawn, in the container"
docker start -ai "$CONTAINER" < "$SLIDE"

echo "== duckdb package: cleanup"
docker rm -f -v "$CONTAINER"
echo "containers left under dbpkg211-*: $(docker ps -a --filter name=dbpkg211 --format '{{.Names}}' | wc -l)"
trap - EXIT
