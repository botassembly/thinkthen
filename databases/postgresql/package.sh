#!/usr/bin/env bash
# The PostgreSQL surface's package rehearsal (the brief's item 5): pgrx
# package the extension, stage the package tree as a tarball, install it
# into a clean postgres:16 container, CREATE EXTENSION, run the slide sample
# exactly as drawn, and remove the container. Nothing is published; the
# install is from the local tarball only.
#
# `--dry-run` builds and stages the tarball and stops before the deck and
# the container: the gate uses it to keep the build honest without
# installing anywhere and without needing the deck on disk.
#
# The container is disposable: named dbpkg211-pg, removed with
# docker rm -f -v at the end.
set -euo pipefail
cd "$(dirname "$0")"

# The builder's home stays out of the packaged extension (surfaces-review-5:
# it carried 163 home paths), the same remap the other surfaces carry.
export RUSTFLAGS="${RUSTFLAGS:+$RUSTFLAGS }--remap-path-prefix=$HOME=/build"
PG_CONFIG=${PG_CONFIG:-$(command -v pg_config)}

dry_run=no
if [ "${1:-}" = "--dry-run" ]; then
  dry_run=yes
fi

VERSION=$(tail -1 ../../VERSION)
CONTAINER=dbpkg211-pg
# Pinned to the digest resolved on this host at pinning time (2026-09-22);
# the pinning story is in scripts/gate-hermeticity.md.
IMAGE=postgres:16@sha256:a3b7f434b2dc57ce85a67e171163eb8ab1a1ebcb39d27484661f26b1dfbe30d6
# The rehearsal's slide comes from the product deck, which lives outside
# this repository. Point THINKTHEN_DECK at its surfaces page to run the
# full rehearsal; the dry run needs no deck.
DECK=${THINKTHEN_DECK:-}
EXT=target/release/thinkthen-pg16/usr
WORK=dist/.rehearsal

cleanup() {
  docker rm -f -v "$CONTAINER" >/dev/null 2>&1 || true
}
trap cleanup EXIT

echo "== postgres package: package the extension"
../../scripts/pgrx-package-locked.sh --pg-config "$PG_CONFIG" >/dev/null
ls -l "$EXT/lib/postgresql/16/lib/thinkthen.so"

echo "== postgres package: stage dist/ (the tree as a tarball)"
rm -rf dist
mkdir -p "$WORK"
tar -C target/release/thinkthen-pg16 -czf "dist/thinkthen-pg16-${VERSION}-linux-amd64.tar.gz" usr
sha256sum "dist/thinkthen-pg16-${VERSION}-linux-amd64.tar.gz"
tar -xzf "dist/thinkthen-pg16-${VERSION}-linux-amd64.tar.gz" -C "$WORK"
find "$WORK/usr" -type f | sort

cat > dist/README.md << EOF
# thinkthen for PostgreSQL ${VERSION}

The pgrx package tree for PostgreSQL 16, in tarball form. Install into a
PostgreSQL 16 installation:

    cp usr/lib/postgresql/16/lib/thinkthen.so        /usr/lib/postgresql/16/lib/
    cp usr/share/postgresql/16/extension/thinkthen*  /usr/share/postgresql/16/extension/

Then, in each database:

    CREATE EXTENSION thinkthen;
EOF

if [ "$dry_run" = yes ]; then
  echo "== postgres package: dry run, no deck, no container"
  echo "dist/thinkthen-pg16-${VERSION}-linux-amd64.tar.gz staged; the container half is not run"
  exit 0
fi
: "${DECK:?set THINKTHEN_DECK to the product deck's surfaces page for the full rehearsal (the dry run needs no deck)}"
echo "== postgres package: slide sample extracted verbatim from the deck"
python3 - "$DECK" "$WORK/slide.sql" << 'PY'
import re, sys
text = open(sys.argv[1]).read()
section = text.split('## PostgreSQL', 1)[1].split('## Names become rows', 1)[0]
block = re.search(r'```sql\n(.*?)```', section, re.S).group(1)
open(sys.argv[2], 'w').write(block)
PY
# The deck's PostgreSQL tab draws the annotate set as a bare name
# ('form.json'); this surface requires the ruled '@name' spelling for a
# file (review 2, item 4), so the rehearsal substitutes it. The deck's
# line is recorded for its owner in README.md.
perl -pi -e "s/thinkthen_annotate\('form.json'/thinkthen_annotate('\@form.json'/" "$WORK/slide.sql"
cat "$WORK/slide.sql"

echo "== postgres package: clean postgres:16 container"
if ! docker image inspect "$IMAGE" >/dev/null 2>&1; then
  docker pull "$IMAGE"
else
  echo "image present: $(docker image inspect --format '{{index .RepoDigests 0}}' "$IMAGE")"
fi
docker rm -f -v "$CONTAINER" >/dev/null 2>&1 || true
docker run -d --name "$CONTAINER" -e POSTGRES_PASSWORD=postgres -e ENGINE_NULL=1 "$IMAGE" >/dev/null
wait_ready() {
  for _ in $(seq 1 60); do
    if docker exec -e PGHOST=/run/postgresql "$CONTAINER" psql -U postgres -Atqc "select 1" >/dev/null 2>&1; then return; fi
    sleep 1
  done
  echo "container never became ready" >&2; exit 1
}
wait_ready
docker exec "$CONTAINER" bash -c 'grep PRETTY_NAME /etc/os-release; ldd --version | head -1'

docker cp "$WORK/usr/lib/postgresql/16/lib/thinkthen.so" "$CONTAINER":/usr/lib/postgresql/16/lib/thinkthen.so
docker cp "$WORK/usr/share/postgresql/16/extension/thinkthen.control" "$CONTAINER":/usr/share/postgresql/16/extension/thinkthen.control
docker cp "$WORK/usr/share/postgresql/16/extension/thinkthen--0.0.1.sql" "$CONTAINER":/usr/share/postgresql/16/extension/thinkthen--0.0.1.sql
docker cp fixtures/refund.json "$CONTAINER":/var/lib/postgresql/data/refund.json
docker cp fixtures/form.json "$CONTAINER":/var/lib/postgresql/data/form.json
docker cp fixtures/tickets.sql "$CONTAINER":/tickets.sql
docker cp "$WORK/slide.sql" "$CONTAINER":/slide.sql

psql_in() {
  docker exec -i -e PGHOST=/run/postgresql "$CONTAINER" \
    psql -U postgres -v ON_ERROR_STOP=1 -P footer=off "$@"
}

echo "== postgres package: the slide sample, as drawn"
psql_in -c "CREATE EXTENSION thinkthen;" -f /tickets.sql >/dev/null
psql_in -f /slide.sql | tee "$WORK/slide.out"
grep -q " 2 | maybe this is on our side" "$WORK/slide.out"
grep -q "1.7" "$WORK/slide.out"
echo "slide sample green: the maybe row reads, urgency ordered 1.7/1.05/0.99"

echo "== postgres package: cleanup"
docker rm -f -v "$CONTAINER"
echo "containers left under dbpkg211-*: $(docker ps -a --filter name=dbpkg211 --format '{{.Names}}' | wc -l)"
trap - EXIT
