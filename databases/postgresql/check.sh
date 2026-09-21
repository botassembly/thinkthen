#!/usr/bin/env bash
# The PostgreSQL surface's check: package the extension, prove it in a
# disposable postgres:16 container on the host network, and remove the
# container after. The null path always runs; the wire path runs when the
# loopback stub is up on this surface's port (8219), with STUB_DELAY_MS=300.
set -euo pipefail
cd "$(dirname "$0")"

NAME=thinkthen-pg
WIRE_NAME=thinkthen-pg-wire

# Sibling sessions on this box start and stop their own postgres containers,
# so a fixed port is a race. Take the first free port from a quiet range.
free_port() {
  for candidate in 5460 5461 5462 5463 5464 5465 5466 5467 5468 5469; do
    [ "$candidate" = "${1:-}" ] && continue
    if ! ss -ltn 2>/dev/null | grep -q ":$candidate "; then
      echo "$candidate"; return
    fi
  done
  echo "no free port in the check's range" >&2; exit 1
}
PORT=$(free_port)
WIRE_PORT=$(free_port "$PORT")

# Wait on the container's own socket, the same path the checks use. A TCP
# probe races the entrypoint's init phase and a sibling session's port.
wait_ready() {
  for _ in $(seq 1 60); do
    if docker exec -e PGHOST=/run/postgresql "$1" psql -U postgres -Atqc "select 1" \
        >/dev/null 2>&1; then return; fi
    sleep 1
  done
  echo "$1 never became ready" >&2; exit 1
}

EXT=target/release/thinkthen-pg16/usr

cleanup() {
  docker rm -f -v "$NAME" "$WIRE_NAME" >/dev/null 2>&1 || true
}
trap cleanup EXIT

echo "== postgres surface: package the extension"
(cd . && cargo pgrx package --pg-config /usr/bin/pg_config) >/dev/null

echo "== postgres surface: disposable container, null backend"
docker rm -f -v "$NAME" >/dev/null 2>&1 || true
docker run -d --name "$NAME" --network host \
  -e POSTGRES_PASSWORD=postgres -e PGPORT=$PORT -e ENGINE_NULL=1 \
  postgres:16 >/dev/null
wait_ready "$NAME"

docker cp "$EXT/lib/postgresql/16/lib/thinkthen.so" "$NAME:/usr/lib/postgresql/16/lib/thinkthen.so"
docker cp "$EXT/share/postgresql/16/extension/thinkthen.control" \
  "$NAME:/usr/share/postgresql/16/extension/thinkthen.control"
docker cp "$EXT/share/postgresql/16/extension/thinkthen--0.0.1.sql" \
  "$NAME:/usr/share/postgresql/16/extension/thinkthen--0.0.1.sql"
# The backend resolves '@file' against its working directory, which the
# official image sets to the data directory.
docker cp fixtures/refund.json "$NAME:/var/lib/postgresql/data/refund.json"
docker cp fixtures/form.json  "$NAME:/var/lib/postgresql/data/form.json"
docker cp fixtures/tickets.sql "$NAME:/tickets.sql"

psql_in() {
  # The socket lives at /run/postgresql, not /var/run/postgresql.
  docker exec -i -e PGHOST=/run/postgresql "$NAME" \
    psql -U postgres -v ON_ERROR_STOP=1 -P footer=off "$@"
}

echo "== postgres surface: the slide sample, as drawn"
psql_in -c "CREATE EXTENSION thinkthen;" -f /tickets.sql >/dev/null
psql_in << 'SQL' > .tmp-slide.out
SELECT id, body
FROM tickets
WHERE thinkthen_decide('@refund.json', body) IS NULL;

SELECT id, a->>'team' AS team,
    (a->>'urgency')::float AS urgency
FROM tickets,
    thinkthen_annotate('form.json', body) AS a
ORDER BY urgency DESC;
SQL
grep -q " 2 | maybe this is on our side" .tmp-slide.out
grep -q "1.7" .tmp-slide.out
rm .tmp-slide.out
echo "slide sample green: the maybe row reads, urgency ordered 1.7/1.05/0.99"

echo "== postgres surface: conformance slice, offline"
python3 runner.py "$NAME"

if curl -sf --max-time 1 http://127.0.0.1:8219/v1/stats >/dev/null 2>&1; then
  echo "== postgres surface: wire suite against the stub on 8219"
  docker rm -f -v "$WIRE_NAME" >/dev/null 2>&1 || true
  docker run -d --name "$WIRE_NAME" --network host \
    -e POSTGRES_PASSWORD=postgres -e PGPORT=$WIRE_PORT \
    -e ENGINE_BASE_URL=http://127.0.0.1:8219/v1 -e ENGINE_WIDTH=32 \
    postgres:16 >/dev/null
  wait_ready "$WIRE_NAME"
  docker cp "$EXT/lib/postgresql/16/lib/thinkthen.so" "$WIRE_NAME:/usr/lib/postgresql/16/lib/thinkthen.so"
  docker cp "$EXT/share/postgresql/16/extension/thinkthen.control" \
    "$WIRE_NAME:/usr/share/postgresql/16/extension/thinkthen.control"
  docker cp "$EXT/share/postgresql/16/extension/thinkthen--0.0.1.sql" \
    "$WIRE_NAME:/usr/share/postgresql/16/extension/thinkthen--0.0.1.sql"
  docker cp fixtures/refund.json "$WIRE_NAME:/var/lib/postgresql/data/refund.json"
  psql_wire() {
    docker exec -e PGHOST=/run/postgresql "$WIRE_NAME" \
      psql -U postgres -v ON_ERROR_STOP=1 -Atq "$@"
  }
  psql_wire -c "CREATE EXTENSION thinkthen;" \
    -c "SELECT thinkthen_decide('@refund.json', 'please refund the duplicate');" | grep -q t
  psql_wire -c "SELECT * FROM thinkthen_usage();" | grep -Eq "^[0-9]+\|[0-9]+\|[0-9]+$"
  echo "wire green: decide answers on the wire, usage counts sends and tokens"
else
  echo "== postgres surface: wire suite skipped, no stub on 8219"
fi
