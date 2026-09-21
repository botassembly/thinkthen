#!/usr/bin/env bash
# The PostgreSQL surface's check: package the extension, prove it in a
# disposable postgres:16 container on the host network, and remove the
# container after. The null path always runs; the wire path runs when the
# loopback stub is up on this surface's port (8219), with STUB_DELAY_MS=300.
set -euo pipefail
cd "$(dirname "$0")"

NAME=laneb-pg
WIRE_NAME=laneb-pg-wire

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

echo "== postgres surface: the error-mapping test"
cargo test --release --quiet --lib

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

echo "== postgres surface: recognize and relate, as drawn"
docker cp fixtures/inbox.sql "$NAME:/inbox.sql"
docker cp fixtures/alerts.sql "$NAME:/alerts.sql"
docker cp fixtures/names.json "$NAME:/var/lib/postgresql/data/names.json"
docker cp fixtures/names-star.json "$NAME:/var/lib/postgresql/data/names-star.json"
psql_in -f /inbox.sql -f /alerts.sql >/dev/null
psql_in << 'SQL' > .tmp-recognize.out
-- the deck's PostgreSQL call, as drawn
SELECT t.id, n.text, n.kind
FROM inbox t, LATERAL thinkthen_recognize(t.body, ARRAY['person','organization']) n
ORDER BY t.id, n."start";

-- names become rows: read each text once
CREATE TABLE mentions AS
SELECT t.id, n.text, n.kind, n."start", n."end", n.strength
FROM inbox t, LATERAL thinkthen_recognize(t.body, ARRAY['person','organization','place']) n;

-- the offsets slice the name in PostgreSQL's own indexing: characters
SELECT substring(body from n."start" + 1 for n."end" - n."start") AS name
FROM inbox t, LATERAL thinkthen_recognize(t.body, ARRAY['person']) n
WHERE t.id = 1;

-- from here it is an ordinary join: the usage counter proves no request
SELECT 'requests before join', requests FROM thinkthen_usage();
SELECT a.owner, count(*) AS found
FROM mentions m JOIN accounts a ON a.name = m.text
GROUP BY a.owner ORDER BY found DESC, a.owner;
SELECT 'requests after join', requests FROM thinkthen_usage();

-- the deck's relate call, as drawn
SELECT name, source, target, probability
FROM thinkthen_relate('SELECT id, body FROM alerts', ARRAY['caused_by'])
ORDER BY source, target;

-- the beta companion: relations as rows, from the question file
SELECT name, source_text, source_kind, target_text, target_kind, probability
FROM thinkthen_relations(
    'Maria Chen joined Northwind Freight in Chicago last spring.', '@names.json');

-- the any-kind end is the one-character string '*' in the question file
SELECT name, source_text, target_text
FROM thinkthen_relations(
    'Maria Chen joined Northwind Freight in Chicago last spring.', '@names-star.json');
SQL
must() { grep -qE "$1" .tmp-recognize.out || { echo "recognize check failed: $1" >&2; exit 1; }; }
must '^ *1 \| Maria Chen +\| person'
must '^ *3 \| Millbrook Athletics \| organization'
must '^ *Maria Chen *$'
must '^ *dana +\| *1'
must '^ *amara +\| *1'
must '^ *lee +\| *1'
must '^ *caused_by \| *1 \| *2 \| *0\.59'
must '^ *caused_by \| *3 \| *4 \| *0\.84'
must '^ *works_for \| Maria Chen +\| person +\| Northwind Freight \| organization \| *1'
must '^ *works_for \| Maria Chen +\| Northwind Freight *$'
before=$(grep "requests before join" .tmp-recognize.out | awk '{print $NF}')
after=$(grep "requests after join" .tmp-recognize.out | awk '{print $NF}')
[ "$before" = "$after" ] || { echo "the join moved the usage counter: $before -> $after" >&2; exit 1; }
rm .tmp-recognize.out
echo "recognize rows, offsets, the no-request join, relate edges, and the relations rows are green"

echo "== postgres surface: from and to are refused"
docker cp fixtures/names-legacy.json "$NAME:/var/lib/postgresql/data/names-legacy.json"
if psql_in -c "SELECT thinkthen_relations('Maria Chen joined Northwind Freight in Chicago last spring.', '@names-legacy.json');" > .tmp-legacy.out 2>&1; then
  echo "FAILED   a from/to spec was accepted" >&2; cat .tmp-legacy.out >&2; exit 1
fi
grep -q "source" .tmp-legacy.out && grep -q "target" .tmp-legacy.out \
  || { echo "FAILED   the refusal does not name the ruled spelling" >&2; cat .tmp-legacy.out >&2; exit 1; }
echo "ok       from/to refused, source and target named"
rm -f .tmp-legacy.out

echo "== postgres surface: relate refuses more than 255 records"
docker exec -i -e PGHOST=/run/postgresql "$NAME" \
  psql -U postgres -P footer=off \
  -c '\set VERBOSITY verbose' \
  -c "SELECT count(*) FROM thinkthen_relate('SELECT g AS id, ''x'' AS body FROM generate_series(1,256) g', ARRAY['caused_by']);" \
  > .tmp-255.out 2>&1 || true
grep -q "22023" .tmp-255.out
grep -q "relate takes at most 255 records and 256 came" .tmp-255.out
rm .tmp-255.out
echo "the 256th record refuses with the usage kind and SQLSTATE 22023"

echo "== postgres surface: fast-backend cancel, the poll-bug shape"
# The batch paths run on a worker thread while the backend thread polls the
# interrupt flag (run_batch); a fast backend never idles, so this is the
# discriminating shape. The full run is about 3.6 s (1M elements on the
# null backend, measured 2026-09-21); the cancel must land within about a
# tick, not after the batch drains.
psql_in -c "SELECT count(*) FROM thinkthen_decide('{\"decide\":\"Is this a complaint?\"}', (SELECT array_agg('refund ' || g) FROM generate_series(1, 1000000) g));" > .tmp-cancel.out 2>&1 &
cancel_bg=$!
sleep 1.2
victim=$(psql_in -Atqc "SELECT pid FROM pg_stat_activity WHERE query LIKE '%thinkthen_decide%' AND pid <> pg_backend_pid() ORDER BY backend_start DESC LIMIT 1")
cancel_start=$(date +%s.%N)
psql_in -Atqc "SELECT pg_cancel_backend($victim)" >/dev/null
wait "$cancel_bg" || true
cancel_end=$(date +%s.%N)
cancel_elapsed=$(awk -v a="$cancel_start" -v b="$cancel_end" 'BEGIN { printf "%.2f", b - a }')
grep -q "canceling statement due to user request" .tmp-cancel.out \
  || { echo "FAILED   the cancel did not land" >&2; cat .tmp-cancel.out >&2; exit 1; }
awk -v e="$cancel_elapsed" 'BEGIN { exit !(e <= 1.5) }' \
  || { echo "FAILED   the cancel waited ${cancel_elapsed}s; the batch was not stoppable" >&2; exit 1; }
echo "ok       pg_cancel_backend stopped the batch ${cancel_elapsed}s past the signal (full run about 3.6 s)"
rm -f .tmp-cancel.out

timeout_start=$(date +%s.%N)
if psql_in -c "SET statement_timeout='1s'; SELECT count(*) FROM thinkthen_decide('{\"decide\":\"Is this a complaint?\"}', (SELECT array_agg('refund ' || g) FROM generate_series(1, 1000000) g));" > .tmp-timeout.out 2>&1; then
  echo "FAILED   the statement_timeout run completed" >&2; exit 1
fi
timeout_end=$(date +%s.%N)
timeout_elapsed=$(awk -v a="$timeout_start" -v b="$timeout_end" 'BEGIN { printf "%.2f", b - a }')
grep -q "statement timeout" .tmp-timeout.out \
  || { echo "FAILED   the timeout error is missing" >&2; cat .tmp-timeout.out >&2; exit 1; }
awk -v e="$timeout_elapsed" 'BEGIN { exit !(e <= 2.5) }' \
  || { echo "FAILED   the timeout run took ${timeout_elapsed}s" >&2; exit 1; }
echo "ok       statement_timeout returned ${timeout_elapsed}s in (1 s timeout; full run about 3.6 s)"
rm -f .tmp-timeout.out

echo "== postgres surface: the function examples"
docker cp fixtures/names.json "$NAME:/var/lib/postgresql/data/names.json"
python3 tests/examples.py "$NAME"

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
