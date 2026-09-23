#!/usr/bin/env bash
# The review-4 item-15 wire probe, standalone: warm then decide must add
# zero sends. Everything runs in ONE psql session — the answers cache and
# the usage counters are per-backend, so separate sessions would reset
# both and the delta would be vacuous. Short, bounded, printed as it goes.
set -uo pipefail
cd "$(dirname "$0")/.."

NAME=laneb-pg-warm
PG_IMAGE=postgres:16@sha256:a3b7f434b2dc57ce85a67e171163eb8ab1a1ebcb39d27484661f26b1dfbe30d6
EXT=target/release/thinkthen-pg16/usr
PORT=5471

OUT_DIR=$(mktemp -d)
cleanup() { docker rm -f -v "$NAME" >/dev/null 2>&1 || true; rm -rf "$OUT_DIR"; }
trap cleanup EXIT

for candidate in 5471 5472 5473; do
  if ! ss -ltn 2>/dev/null | grep -q ":$candidate "; then PORT=$candidate; break; fi
done
echo "port: $PORT"

docker rm -f -v "$NAME" >/dev/null 2>&1 || true
docker run -d --name "$NAME" --network host \
  -e POSTGRES_PASSWORD=postgres -e PGPORT=$PORT \
  -e ENGINE_BASE_URL=http://127.0.0.1:8219/v1 -e ENGINE_WIDTH=32 \
  "$PG_IMAGE" >/dev/null || { echo "container start failed"; exit 1; }
for _ in $(seq 1 60); do
  docker exec -e PGHOST=/run/postgresql "$NAME" psql -U postgres -Atqc "select 1" >/dev/null 2>&1 && break
  sleep 1
done
echo "container ready"

docker cp "$EXT/lib/postgresql/16/lib/thinkthen.so" "$NAME:/usr/lib/postgresql/16/lib/thinkthen.so" >/dev/null
docker cp "$EXT/share/postgresql/16/extension/thinkthen.control" "$NAME:/usr/share/postgresql/16/extension/thinkthen.control" >/dev/null
docker cp "$EXT/share/postgresql/16/extension/thinkthen--0.0.1.sql" "$NAME:/usr/share/postgresql/16/extension/thinkthen--0.0.1.sql" >/dev/null
docker cp fixtures/refund.json "$NAME:/var/lib/postgresql/data/refund.json" >/dev/null

echo "warm starting (2,000 rows at width 32 against the 300 ms stub)"
# One session: warm, count, read, count — the cache and the counters live
# in the one backend.
timeout 120 docker exec -e PGHOST=/run/postgresql "$NAME" \
  psql -U postgres -v ON_ERROR_STOP=1 -Atq \
  -c "CREATE EXTENSION thinkthen;" \
  -c "SELECT thinkthen_warm('@refund.json',
      'order ' || g || ': charged twice, please refund')
    FROM generate_series(1, 2000) g;" \
  -c "SELECT requests FROM thinkthen_usage();" \
  -c "SELECT count(*) FROM generate_series(1, 2000) g
    WHERE thinkthen_decide('@refund.json',
      'order ' || g || ': charged twice, please refund');" \
  -c "SELECT requests FROM thinkthen_usage();" \
  > "$OUT_DIR/warm-wire.out" 2>"$OUT_DIR/warm-wire.err"
rc=$?
echo "session rc=$rc"
cat "$OUT_DIR/warm-wire.err" >&2
[ "$rc" = "0" ] || { echo "FAIL the session failed"; exit 1; }

# Four answer lines: 2000 (warm count), requests, read-count, requests.
mapfile -t lines < <(grep -vE "^$" "$OUT_DIR/warm-wire.out")
warm_count=${lines[0]}
before=${lines[1]}
read_count=${lines[2]}
after=${lines[3]}
echo "warm judged: $warm_count; requests after warm: $before"
echo "read true-count: $read_count; requests after read: $after"

[ "$warm_count" = "2000" ] || { echo "FAIL warm judged $warm_count rows"; exit 1; }
[ "$before" -gt 0 ] 2>/dev/null || { echo "FAIL warm sent nothing ($before) — is the stub up?"; exit 1; }
delta=$((after - before))
[ "$delta" = "0" ] || { echo "FAIL the read pass added $delta sends"; exit 1; }
echo "PASS warm sent $before rounds; the decide pass over the same 2,000 pairs added 0 sends"
