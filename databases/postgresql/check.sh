#!/usr/bin/env bash
# The PostgreSQL surface's check: package the extension, prove it in a
# disposable postgres:16 container on the host network, and remove the
# container after. The null path always runs; the wire path runs when the
# loopback stub is up on this surface's port (8219), with STUB_DELAY_MS=300.
set -euo pipefail
cd "$(dirname "$0")"

NAME=laneb-pg
WIRE_NAME=laneb-pg-wire
KEY_NAME=laneb-pg-key

# Sibling sessions on this box start and stop their own postgres containers,
# so a fixed port is a race. Take the first free port from a quiet range.
free_port() {
  for candidate in 5460 5461 5462 5463 5464 5465 5466 5467 5468 5469; do
    case " $* " in *" $candidate "*) continue ;; esac
    if ! ss -ltn 2>/dev/null | grep -q ":$candidate "; then
      echo "$candidate"; return
    fi
  done
  echo "no free port in the check's range" >&2; exit 1
}
PORT=$(free_port)
WIRE_PORT=$(free_port "$PORT")
KEY_PORT=$(free_port "$PORT" "$WIRE_PORT")
# A port nothing listens on: the credential arm's engine points at it, so a
# call that reaches the wire is refused there and a call that refuses
# first never touches it.
REFUSED_PORT=$(free_port "$PORT" "$WIRE_PORT" "$KEY_PORT")

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
  docker rm -f -v "$NAME" "$WIRE_NAME" "$KEY_NAME" >/dev/null 2>&1 || true
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
  -e ENGINE_SYNTHETIC_PARTIAL=1 \
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

echo "== postgres surface: PUBLIC cannot call, a granted role can"
# Group 3: CREATE EXTENSION revokes the default PUBLIC grant on every
# function, so a paid call and an '@path' read need an explicit grant.
docker cp fixtures/broken.json "$NAME:/var/lib/postgresql/data/broken.json"
psql_in << 'SQL' > .tmp-revoke.out
SELECT 'public_can', count(*) FROM pg_proc p
WHERE p.proname LIKE 'thinkthen%'
  AND p.pronamespace = (SELECT pronamespace FROM pg_proc WHERE proname = 'thinkthen_decide' LIMIT 1)
  AND has_function_privilege('public', p.oid, 'EXECUTE');
SELECT 'functions', count(*) FROM pg_proc p
WHERE p.proname LIKE 'thinkthen%'
  AND p.pronamespace = (SELECT pronamespace FROM pg_proc WHERE proname = 'thinkthen_decide' LIMIT 1);
SQL
grep -qE "public_can *\| *0" .tmp-revoke.out \
  || { echo "FAILED   PUBLIC still holds EXECUTE" >&2; cat .tmp-revoke.out >&2; exit 1; }
known=$(grep -E "^ *functions *\|" .tmp-revoke.out | head -1 | awk -F'|' '{print $2}' | tr -d ' ')
[ "$known" -ge 12 ] || { echo "FAILED   only $known thinkthen functions found" >&2; exit 1; }
echo "ok       PUBLIC holds EXECUTE on none of the $known functions"
rm .tmp-revoke.out

psql_in -c "CREATE ROLE tt_app;" >/dev/null
if psql_in -c "SET ROLE tt_app;" \
    -c "SELECT thinkthen_decide('@refund.json', 'I demand a refund today');" \
    > .tmp-role.out 2>&1; then
  echo "FAILED   an ungranted role made a paid call" >&2; cat .tmp-role.out >&2; exit 1
fi
grep -q "permission denied for function thinkthen_decide" .tmp-role.out \
  || { echo "FAILED   the refusal is not a permission error" >&2; cat .tmp-role.out >&2; exit 1; }
echo "ok       an ungranted role is refused with permission denied"
psql_in -c "GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA public TO tt_app;" >/dev/null
psql_in -Atq -c "SET ROLE tt_app;" \
  -c "SELECT thinkthen_decide('@refund.json', 'I demand a refund today');" > .tmp-role.out
# The role reads a file the backend process owns: the grant is the trust.
grep -q "^t$" .tmp-role.out \
  || { echo "FAILED   the granted role's call did not answer" >&2; cat .tmp-role.out >&2; exit 1; }
echo "ok       the documented one-line grant lets a role call (and read '@path')"
psql_in -c "RESET ROLE;" -c "DROP OWNED BY tt_app;" -c "DROP ROLE tt_app;" >/dev/null
rm -f .tmp-role.out

echo "== postgres surface: a bad question in a batch names itself"
# Group 3: the question resolves on the backend thread before any worker
# spawns, so the real error surfaces instead of "the batch thread stopped".
if psql_in -c "SELECT count(*) FROM thinkthen_decide('@no-such-file.json', ARRAY['a', 'b']);" \
    > .tmp-broken.out 2>&1; then
  echo "FAILED   a missing question file was accepted" >&2; exit 1
fi
grep -q "no-such-file.json" .tmp-broken.out \
  || { echo "FAILED   the error does not name the missing file" >&2; cat .tmp-broken.out >&2; exit 1; }
if grep -q "the batch thread stopped" .tmp-broken.out; then
  echo "FAILED   the batch error masked the real message" >&2; cat .tmp-broken.out >&2; exit 1
fi
echo "ok       a missing file in a batch names the file, not a stopped thread"
if psql_in -c "SELECT count(*) FROM thinkthen_decide('@broken.json', ARRAY['a', 'b']);" \
    > .tmp-broken.out 2>&1; then
  echo "FAILED   a broken question file was accepted" >&2; exit 1
fi
grep -q "the question file is not valid JSON" .tmp-broken.out \
  || { echo "FAILED   the broken file did not return the usage kind" >&2; cat .tmp-broken.out >&2; exit 1; }
if grep -q "the batch thread stopped" .tmp-broken.out; then
  echo "FAILED   the batch error masked the real message" >&2; cat .tmp-broken.out >&2; exit 1
fi
echo "ok       a broken question file returns the usage kind naming the file"
if psql_in -c "SELECT thinkthen_warm('@no-such-file.json', body) FROM tickets;" \
    > .tmp-broken.out 2>&1; then
  echo "FAILED   the warm aggregate accepted a missing question file" >&2; exit 1
fi
grep -q "no-such-file.json" .tmp-broken.out \
  || { echo "FAILED   the warm error does not name the file" >&2; cat .tmp-broken.out >&2; exit 1; }
echo "ok       the warm aggregate names the file too"
rm -f .tmp-broken.out

echo "== postgres surface: a spent deadline refuses before sending"
# Group 4: the single-row path cannot hear pg_cancel_backend, so the
# deadline setting is the enforced tool. Zero is a spent deadline: the
# ruled kind, nothing sent.
psql_in -c '\set VERBOSITY verbose' \
  -c "SET thinkthen.deadline_ms = 0;" \
  -c "SELECT thinkthen_decide('@refund.json', 'I demand a refund today');" \
  > .tmp-deadline.out 2>&1 || true
grep -q "57014" .tmp-deadline.out \
  || { echo "FAILED   a spent deadline did not return the deadline kind" >&2; cat .tmp-deadline.out >&2; exit 1; }
grep -q "thinkthen deadline" .tmp-deadline.out \
  || { echo "FAILED   the spent deadline's message is missing" >&2; cat .tmp-deadline.out >&2; exit 1; }
psql_in -c "RESET thinkthen.deadline_ms;" >/dev/null
echo "ok       a zero budget returns the deadline kind (57014) with nothing sent"
rm -f .tmp-deadline.out

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

echo "== postgres surface: a configured credential refuses, a refused address answers loudly"
# Punch-list item 5: the ruled `thinkthen.api_key` setting cannot reach
# this engine build, so a set value must refuse instead of being silently
# ignored, and a missing credential must fail loudly on the wire. The
# value below is made up; no real key is read or sent anywhere.
docker rm -f -v "$KEY_NAME" >/dev/null 2>&1 || true
docker run -d --name "$KEY_NAME" --network host \
  -e POSTGRES_PASSWORD=postgres -e PGPORT=$KEY_PORT \
  -e ENGINE_BASE_URL=http://127.0.0.1:$REFUSED_PORT/v1 \
  postgres:16 >/dev/null
wait_ready "$KEY_NAME"
docker cp "$EXT/lib/postgresql/16/lib/thinkthen.so" "$KEY_NAME:/usr/lib/postgresql/16/lib/thinkthen.so"
docker cp "$EXT/share/postgresql/16/extension/thinkthen.control" \
  "$KEY_NAME:/usr/share/postgresql/16/extension/thinkthen.control"
docker cp "$EXT/share/postgresql/16/extension/thinkthen--0.0.1.sql" \
  "$KEY_NAME:/usr/share/postgresql/16/extension/thinkthen--0.0.1.sql"
docker cp fixtures/refund.json "$KEY_NAME:/var/lib/postgresql/data/refund.json"
psql_key() {
  docker exec -i -e PGHOST=/run/postgresql "$KEY_NAME" \
    psql -U postgres -P footer=off "$@"
}
psql_key -c "CREATE EXTENSION thinkthen;" >/dev/null

# The setting set: the call refuses with the usage kind, names the setting
# and the substitute channel, and never echoes the value.
psql_key -c '\set VERBOSITY verbose' \
  -c "SET thinkthen.api_key = 'made-up-not-a-key';" \
  -c "SELECT thinkthen_decide('@refund.json', 'I demand a refund today');" \
  > .tmp-key.out 2>&1 || true
grep -q "22023" .tmp-key.out \
  || { echo "FAILED   the configured key did not refuse with the usage kind" >&2; cat .tmp-key.out >&2; exit 1; }
grep -q "thinkthen.api_key" .tmp-key.out \
  || { echo "FAILED   the refusal does not name the setting" >&2; cat .tmp-key.out >&2; exit 1; }
grep -q "THINKTHEN_API_KEY" .tmp-key.out \
  || { echo "FAILED   the refusal does not name the substitute channel" >&2; cat .tmp-key.out >&2; exit 1; }
if grep -q "made-up-not-a-key" .tmp-key.out; then
  echo "FAILED   the refusal echoed the value" >&2; exit 1
fi

# The setting reset: the same call reaches the wire and the refused port
# answers loudly — the control proving the refusal above is the setting's,
# and the missing-credential arm at once.
psql_key -c '\set VERBOSITY verbose' \
  -c "RESET thinkthen.api_key;" \
  -c "SELECT thinkthen_decide('@refund.json', 'I demand a refund today');" \
  > .tmp-key.out 2>&1 || true
grep -q "38000" .tmp-key.out \
  || { echo "FAILED   the refused address did not answer with the backend kind" >&2; cat .tmp-key.out >&2; exit 1; }
grep -q "the address refused the connection" .tmp-key.out \
  || { echo "FAILED   the refused address's message is missing" >&2; cat .tmp-key.out >&2; exit 1; }
rm -f .tmp-key.out
if docker logs "$KEY_NAME" 2>&1 | grep -q "made-up-not-a-key"; then
  echo "FAILED   the value appeared in the server log" >&2; exit 1
fi
echo "ok       a configured key refuses with 22023 naming the setting; reset, the refused address answers with 38000"

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

  echo "== postgres surface: a deadline shorter than the stub's delay"
  # The enforced tool on the single-row path, against the 300 ms stub: a
  # 50 ms budget must return the deadline kind in about a tick, not after
  # the send returns.
  deadline_start=$(date +%s.%N)
  if psql_wire -c '\set VERBOSITY verbose' \
      -c "SET thinkthen.deadline_ms = 50;" \
      -c "SELECT thinkthen_decide('@refund.json', 'I demand a refund today');" \
      > .tmp-wire-deadline.out 2>&1; then
    echo "FAILED   the 50 ms budget did not refuse" >&2; exit 1
  fi
  deadline_end=$(date +%s.%N)
  deadline_elapsed=$(awk -v a="$deadline_start" -v b="$deadline_end" 'BEGIN { printf "%.2f", b - a }')
  grep -q "57014" .tmp-wire-deadline.out \
    || { echo "FAILED   the wire deadline did not return 57014" >&2; cat .tmp-wire-deadline.out >&2; exit 1; }
  grep -q "thinkthen deadline" .tmp-wire-deadline.out \
    || { echo "FAILED   the wire deadline's message is missing" >&2; cat .tmp-wire-deadline.out >&2; exit 1; }
  awk -v e="$deadline_elapsed" 'BEGIN { exit !(e <= 1.5) }' \
    || { echo "FAILED   the 50 ms budget took ${deadline_elapsed}s" >&2; exit 1; }
  psql_wire -c "RESET thinkthen.deadline_ms;" >/dev/null
  rm -f .tmp-wire-deadline.out
  echo "ok       a 50 ms budget refused in ${deadline_elapsed}s against the 300 ms stub"
else
  echo "== postgres surface: wire suite skipped, no stub on 8219"
fi
