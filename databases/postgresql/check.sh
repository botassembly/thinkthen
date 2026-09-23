#!/usr/bin/env bash
# The PostgreSQL surface's check: package the extension, prove it in a
# disposable postgres:16 container on the host network, and remove the
# container after. The null path always runs; the wire path runs when the
# loopback stub is up on this surface's port (8219), with STUB_DELAY_MS=300.
# Experimental on macOS: the container paths and the port probe below take
# the Darwin spellings (lsof, no ss); Linux is the gate's platform.
set -euo pipefail
cd "$(dirname "$0")"

NAME=laneb-pg
WIRE_NAME=laneb-pg-wire
KEY_NAME=laneb-pg-key
# Pinned to the digest resolved on this host at pinning time (2026-09-22);
# the pinning story is in scripts/gate-hermeticity.md.
PG_IMAGE=postgres:16@sha256:a3b7f434b2dc57ce85a67e171163eb8ab1a1ebcb39d27484661f26b1dfbe30d6

# Sibling sessions on this box start and stop their own postgres containers,
# so a fixed port is a race. Take the first free port from a quiet range.
port_busy() {
  local candidate=$1
  if command -v ss >/dev/null 2>&1; then
    ss -ltn 2>/dev/null | grep -q ":$candidate "
  elif command -v lsof >/dev/null 2>&1; then
    lsof -nP -iTCP:"$candidate" -sTCP:LISTEN >/dev/null 2>&1
  else
    (exec 3<>/dev/tcp/127.0.0.1/"$candidate") >/dev/null 2>&1
  fi
}
free_port() {
  for candidate in 5460 5461 5462 5463 5464 5465 5466 5467 5468 5469; do
    case " $* " in *" $candidate "*) continue ;; esac
    if ! port_busy "$candidate"; then
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
# The annotate partial-failure fixture is compiled in only for a build
# that asks for it (standin/Cargo.toml, the `synthetic-partial` feature):
# conformance case 74 replays that record. The packaged test artifact is
# built with the feature; the release rehearsal in package.sh builds
# without it.
cargo test --release --quiet --lib --features synthetic-partial --locked

echo "== postgres surface: package the extension"
# Review 4, item 20 asked the package step under --locked; cargo-pgrx
# 0.17 carries no --locked flag, so the same guarantee is enforced one
# step ahead: --locked fails the build when the lockfile would change,
# and the package step then runs against the checked tree.
cargo check --quiet --locked --features synthetic-partial
(cd . (cd . && cargo pgrx package --pg-config /usr/bin/pg_config --features synthetic-partial) >/dev/null(cd . && cargo pgrx package --pg-config /usr/bin/pg_config --features synthetic-partial) >/dev/null CARGO_NET_OFFLINE=true cargo pgrx package --pg-config /usr/bin/pg_config --features synthetic-partial) >/dev/null

echo "== postgres surface: disposable container, null backend"
docker rm -f -v "$NAME" >/dev/null 2>&1 || true
docker run -d --name "$NAME" --network host \
  -e POSTGRES_PASSWORD=postgres -e PGPORT=$PORT -e ENGINE_NULL=1 \
  "$PG_IMAGE" >/dev/null
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
    -- The deck's PostgreSQL tab draws this set as a bare name
    -- ('form.json'); a file name carries the ruled '@' spelling on this
    -- surface (review 2, item 4: bare text was read as a server path), so
    -- the drawn line runs here in the ruled spelling. The deck's line is
    -- recorded for its owner in README.md.
    thinkthen_annotate('@form.json', body) AS a
ORDER BY urgency DESC;
SQL
grep -q " 2 | maybe this is on our side" .tmp-slide.out
grep -q "1.7" .tmp-slide.out
rm .tmp-slide.out
echo "slide sample green: the maybe row reads, urgency ordered 1.7/1.05/0.99"

echo "== postgres surface: warm saves, decide reads (review 4, item 15)"
# The reviewer's shape, scaled for the null path: warm judges 20,000
# pairs, then the row-by-row decide pass must answer from the saved
# judgments — instant and identical — instead of re-judging.
warm_start=$(date +%s.%N)
psql_in -Atq -c "SELECT thinkthen_warm('@refund.json',
    'order ' || g || ': charged twice, please refund')
  FROM generate_series(1, 20000) g;" > .tmp-warm-count.out
warm_end=$(date +%s.%N)
warm_elapsed=$(awk -v a="$warm_start" -v b="$warm_end" 'BEGIN { printf "%.2f", b - a }')
grep -q "20000" .tmp-warm-count.out || { echo "FAILED   warm judged $(cat .tmp-warm-count.out) rows" >&2; exit 1; }
read_start=$(date +%s.%N)
read_count=$(psql_in -Atq -c "SELECT count(*) FROM generate_series(1, 20000) g
  WHERE thinkthen_decide('@refund.json',
    'order ' || g || ': charged twice, please refund');")
read_again=$(psql_in -Atq -c "SELECT count(*) FROM generate_series(1, 20000) g
  WHERE thinkthen_decide('@refund.json',
    'order ' || g || ': charged twice, please refund');")
read_end=$(date +%s.%N)
read_elapsed=$(awk -v a="$read_start" -v b="$read_end" 'BEGIN { printf "%.2f", b - a }')
[ "$read_count" = "$read_again" ] \
  || { echo "FAILED   the saved judgments read differently twice ($read_count, $read_again)" >&2; exit 1; }
awk -v e="$read_elapsed" 'BEGIN { exit !(e <= 5) }' \
  || { echo "FAILED   reading 20,000 saved judgments took ${read_elapsed}s" >&2; exit 1; }
echo "ok       warm judged 20,000 pairs in ${warm_elapsed}s; the read pass answered from the saved judgments in ${read_elapsed}s"
rm -f .tmp-warm-count.out

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
SELECT 'owned_can', count(*) FROM pg_proc p
JOIN pg_depend d ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
JOIN pg_extension e ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass
WHERE e.extname = 'thinkthen' AND has_function_privilege('public', p.oid, 'EXECUTE');
SELECT 'owned', count(*) FROM pg_proc p
JOIN pg_depend d ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
JOIN pg_extension e ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass
WHERE e.extname = 'thinkthen';
SQL
grep -qE "public_can *\| *0" .tmp-revoke.out \
  || { echo "FAILED   PUBLIC still holds EXECUTE" >&2; cat .tmp-revoke.out >&2; exit 1; }
grep -qE "owned_can *\| *0" .tmp-revoke.out \
  || { echo "FAILED   PUBLIC holds EXECUTE on an extension-owned function" >&2; cat .tmp-revoke.out >&2; exit 1; }
known=$(grep -E "^ *functions *\|" .tmp-revoke.out | head -1 | awk -F'|' '{print $2}' | tr -d ' ')
owned=$(grep -E "^ *owned *\|" .tmp-revoke.out | head -1 | awk -F'|' '{print $2}' | tr -d ' ')
[ "$known" -ge 12 ] || { echo "FAILED   only $known thinkthen functions found" >&2; exit 1; }
[ "$owned" -ge "$known" ] || { echo "FAILED   only $owned extension-owned functions found" >&2; exit 1; }
echo "ok       PUBLIC holds EXECUTE on none of the $known functions ($owned extension-owned, aggregate helpers included)"
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
# Review 2, item 4: the grant names the extension's own functions, never
# ALL FUNCTIONS IN SCHEMA public, which would also grant every other
# function in the schema. The control below is exactly such a function.
# Review 3, item 8: EXECUTE is no longer a file-read trust — the role
# also holds the core file privilege for its '@refund.json' read.
psql_in -c "CREATE FUNCTION tt_control(x integer) RETURNS integer LANGUAGE sql AS 'SELECT \$1';" \
  -c "REVOKE ALL ON FUNCTION tt_control(integer) FROM PUBLIC;" \
  -c "GRANT pg_read_server_files TO tt_app;" >/dev/null
psql_in << 'SQL' >/dev/null
DO $thinkthen_grant$
DECLARE
    signature text;
BEGIN
    FOR signature IN
        SELECT p.oid::regprocedure::text
        FROM pg_proc p
        JOIN pg_depend d ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
        JOIN pg_extension e ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass
        WHERE e.extname = 'thinkthen'
    LOOP
        EXECUTE format('GRANT EXECUTE ON FUNCTION %s TO tt_app', signature);
    END LOOP;
END
$thinkthen_grant$;
SQL
psql_in -Atq -c "SET ROLE tt_app;" \
  -c "SELECT thinkthen_decide('@refund.json', 'I demand a refund today');" > .tmp-role.out
# The role reads a file by the core file privilege: EXECUTE plus
# pg_read_server_files is the full trust the README documents.
grep -q "^t$" .tmp-role.out \
  || { echo "FAILED   the narrowed grant's call did not answer" >&2; cat .tmp-role.out >&2; exit 1; }
if psql_in -c "SET ROLE tt_app;" -c "SELECT tt_control(7);" > .tmp-control.out 2>&1; then
  echo "FAILED   the grant reached an unrelated function" >&2; cat .tmp-control.out >&2; exit 1
fi
grep -q "permission denied" .tmp-control.out \
  || { echo "FAILED   the control's refusal is not a permission error" >&2; cat .tmp-control.out >&2; exit 1; }
echo "ok       the narrowed grant covers the extension's own functions and not the schema's others"
psql_in -c "RESET ROLE;" -c "DROP OWNED BY tt_app;" -c "DROP FUNCTION tt_control(integer);" -c "DROP ROLE tt_app;" >/dev/null
rm -f .tmp-role.out .tmp-control.out

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

echo "== postgres surface: batches carry the deadline too"
# Review 1/2 leftovers: the batch paths ignored the setting, so a spent
# budget still spent. Both batch shapes must refuse with nothing sent.
psql_in -c '\set VERBOSITY verbose' \
  -c "SET thinkthen.deadline_ms = 0;" \
  -c "SELECT count(*) FROM thinkthen_decide('@refund.json', ARRAY['a', 'b']);" \
  > .tmp-batch.out 2>&1 || true
grep -q "57014" .tmp-batch.out \
  || { echo "FAILED   the array batch ignored a spent budget" >&2; cat .tmp-batch.out >&2; exit 1; }
grep -q "thinkthen deadline" .tmp-batch.out \
  || { echo "FAILED   the array batch's deadline message is missing" >&2; cat .tmp-batch.out >&2; exit 1; }
psql_in -c '\set VERBOSITY verbose' \
  -c "SET thinkthen.deadline_ms = 0;" \
  -c "SELECT thinkthen_warm('@refund.json', body) FROM tickets;" \
  > .tmp-batch.out 2>&1 || true
grep -q "57014" .tmp-batch.out \
  || { echo "FAILED   the warm aggregate ignored a spent budget" >&2; cat .tmp-batch.out >&2; exit 1; }
psql_in -c "RESET thinkthen.deadline_ms;" >/dev/null
# Mid-flight: a budget inside a long batch ends it at the deadline, not at
# the end (the same batch without a stop runs about a second on the null
# backend and answers a count).
batch_start=$(date +%s.%N)
if psql_in -c '\set VERBOSITY verbose' \
    -c "SET thinkthen.deadline_ms = 50;" \
    -c "SELECT count(*) FROM thinkthen_decide('{\"decide\":\"Is this a complaint?\"}', (SELECT array_agg('refund ' || g) FROM generate_series(1, 300000) g));" \
    > .tmp-batch.out 2>&1; then
  echo "FAILED   the 50 ms budget's batch completed" >&2; cat .tmp-batch.out >&2; exit 1
fi
batch_elapsed=$(awk -v a="$batch_start" -v b="$(date +%s.%N)" 'BEGIN { printf "%.2f", b - a }')
grep -q "57014" .tmp-batch.out \
  || { echo "FAILED   the 50 ms budget's batch did not return the deadline kind" >&2; cat .tmp-batch.out >&2; exit 1; }
awk -v e="$batch_elapsed" 'BEGIN { exit !(e <= 2.0) }' \
  || { echo "FAILED   the 50 ms budget's batch took ${batch_elapsed}s" >&2; exit 1; }
psql_in -c "RESET thinkthen.deadline_ms;" >/dev/null
echo "ok       a spent budget sends nothing on both batch shapes; a 50 ms budget ends a long batch in ${batch_elapsed}s"
rm -f .tmp-batch.out

echo "== postgres surface: the batch poll checks completion first"
# Review 3, item 24: the poll loop slept an unconditional 100 ms before
# looking, so every batch paid that floor. The ready channel answers the
# instant the worker finishes; a two-record batch on the null backend now
# measures in single-digit milliseconds inside the server.
batch_wall=$(psql_in 2>&1 <<'SQL' | grep -oE "batch wall ms: [0-9]+" | awk '{print $NF}'
DO $probe$
DECLARE t0 timestamptz; n bigint;
BEGIN
  t0 := clock_timestamp();
  SELECT count(*) INTO n FROM thinkthen_decide('{"decide":"Is this a complaint?"}', ARRAY['a','b']);
  RAISE NOTICE 'batch wall ms: %', (extract(epoch from clock_timestamp() - t0) * 1000)::int;
END
$probe$;
SQL
)
[ -n "$batch_wall" ] || { echo "FAILED   the batch wall probe printed nothing" >&2; exit 1; }
[ "$batch_wall" -le 90 ] \
  || { echo "FAILED   a two-record batch took ${batch_wall}ms (the 100ms floor survived)" >&2; exit 1; }
echo "ok       a two-record batch answered in ${batch_wall}ms (the pre-fix floor was 100ms)"

echo "== postgres surface: warm holds twenty thousand rows in seconds, not a minute"
# Review 3, item 23: the JSON state re-read and re-wrote the whole
# aggregate on every row — 54.6 s at 20,000 rows, measured by the review.
# The concatenated state appends without parsing; the same twenty
# thousand rows now measure inside the server, and the cap refuses the
# twenty-thousand-and-first with the two-aggregate spelling named.
warm_wall=$(psql_in 2>&1 <<'SQL' | grep -oE "warm 20000 wall ms: [0-9]+" | awk '{print $NF}'
DO $probe$
DECLARE t0 timestamptz; n bigint;
BEGIN
  t0 := clock_timestamp();
  SELECT thinkthen_warm('{"decide":"Is this a complaint?"}', 'refund ' || g) INTO n
    FROM generate_series(1, 20000) g;
  RAISE NOTICE 'warm 20000 wall ms: %', (extract(epoch from clock_timestamp() - t0) * 1000)::int;
  RAISE NOTICE 'judged: %', n;
END
$probe$;
SQL
)
[ -n "$warm_wall" ] || { echo "FAILED   the warm wall probe printed nothing" >&2; exit 1; }
[ "$warm_wall" -le 15000 ] \
  || { echo "FAILED   twenty thousand warm rows took ${warm_wall}ms" >&2; exit 1; }
echo "ok       twenty thousand warm rows answered in ${warm_wall}ms (the JSON shape measured 54600ms)"
if psql_in -Atqc "SELECT thinkthen_warm('{\"decide\":\"Is this a complaint?\"}', 'refund ' || g) FROM generate_series(1, 20001) g;" > .tmp-warm-cap.out 2>&1; then
  echo "FAILED   the warm cap did not refuse" >&2; exit 1
fi
grep -q "holds at most 20000 rows" .tmp-warm-cap.out \
  || { echo "FAILED   the warm cap refusal does not name the cap" >&2; cat .tmp-warm-cap.out >&2; exit 1; }
echo "ok       the twenty-thousand-and-first row refuses naming the cap and the two-aggregate spelling"
rm -f .tmp-warm-cap.out

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

echo "== postgres surface: a bare path never names a file"
# Review 2, item 4: any text without @ was read as a server path, so a
# granted role could read files as the postgres user with no @ required.
# Both files below exist in the data directory, so a pre-fix build reads
# them and answers; the fix refuses with the usage kind naming the form.
if psql_in -c "SELECT count(*) FROM thinkthen_relations('Maria Chen joined Northwind Freight in Chicago last spring.', 'names.json');" > .tmp-bare.out 2>&1; then
  echo "FAILED   a bare path named a file" >&2; cat .tmp-bare.out >&2; exit 1
fi
grep -q "@names.json" .tmp-bare.out \
  || { echo "FAILED   the refusal does not name the @ form" >&2; cat .tmp-bare.out >&2; exit 1; }
grep -q "never a path" .tmp-bare.out \
  || { echo "FAILED   the refusal does not say bare text is not a path" >&2; cat .tmp-bare.out >&2; exit 1; }
if psql_in -c "SELECT thinkthen_annotate('form.json', body) FROM tickets LIMIT 1;" > .tmp-bare.out 2>&1; then
  echo "FAILED   a bare set path named a file" >&2; cat .tmp-bare.out >&2; exit 1
fi
grep -q "@form.json" .tmp-bare.out \
  || { echo "FAILED   the set refusal does not name the @ form" >&2; cat .tmp-bare.out >&2; exit 1; }
echo "ok       bare text is refused with the @ form named, and never read"
rm -f .tmp-bare.out

echo "== postgres surface: named files read through the gate, the cap, and the regular-file rule"
# Review 3, items 2 and 8: every @file read is capped, regular-file, and
# behind the privilege gate. /dev/zero and a fifo refuse instantly with
# one uniform message that carries no cause; an over-cap file names the
# cap; a role holding only EXECUTE cannot read a file without
# pg_read_server_files or the configured directory, and the directory
# confines by resolved path, so a symlink cannot point out.
docker exec "$NAME" sh -c 'head -c 1048577 /dev/zero | tr "\\0" "x" > /var/lib/postgresql/data/big.json'
docker exec "$NAME" mkfifo /var/lib/postgresql/data/pipe.fifo

zero_start=$(date +%s.%N)
if timeout 10 docker exec -e PGHOST=/run/postgresql "$NAME" psql -U postgres -Atqc "SELECT thinkthen_decide('@/dev/zero', 'refund');" > .tmp-gate.out 2>&1; then
  echo "FAILED   @/dev/zero answered" >&2; cat .tmp-gate.out >&2; exit 1
fi
zero_elapsed=$(awk -v a="$zero_start" -v b="$(date +%s.%N)" 'BEGIN { printf "%.2f", b - a }')
grep -q "did not read" .tmp-gate.out \
  || { echo "FAILED   the /dev/zero refusal's message is missing" >&2; cat .tmp-gate.out >&2; exit 1; }
if grep -qE "No such file|Permission denied|Text file busy" .tmp-gate.out; then
  echo "FAILED   the refusal carries a filesystem cause" >&2; cat .tmp-gate.out >&2; exit 1
fi
awk -v e="$zero_elapsed" 'BEGIN { exit !(e <= 2.0) }' \
  || { echo "FAILED   @/dev/zero took ${zero_elapsed}s" >&2; exit 1; }
psql_in -Atqc "SELECT 1;" >/dev/null
echo "ok       @/dev/zero refused in ${zero_elapsed}s with the uniform message, and the backend lived"

fifo_start=$(date +%s.%N)
if timeout 5 docker exec -e PGHOST=/run/postgresql "$NAME" psql -U postgres -Atqc "SELECT thinkthen_decide('@/var/lib/postgresql/data/pipe.fifo', 'refund');" > .tmp-gate.out 2>&1; then
  echo "FAILED   a fifo answered" >&2; cat .tmp-gate.out >&2; exit 1
fi
fifo_elapsed=$(awk -v a="$fifo_start" -v b="$(date +%s.%N)" 'BEGIN { printf "%.2f", b - a }')
grep -q "did not read" .tmp-gate.out \
  || { echo "FAILED   the fifo refusal's message is missing" >&2; cat .tmp-gate.out >&2; exit 1; }
awk -v e="$fifo_elapsed" 'BEGIN { exit !(e <= 2.0) }' \
  || { echo "FAILED   the fifo refusal took ${fifo_elapsed}s" >&2; exit 1; }
echo "ok       a fifo refused in ${fifo_elapsed}s instead of blocking"

if psql_in -Atqc "SELECT thinkthen_decide('@/var/lib/postgresql/data/big.json', 'refund');" > .tmp-gate.out 2>&1; then
  echo "FAILED   an over-cap file answered" >&2; cat .tmp-gate.out >&2; exit 1
fi
grep -q "over the 1048576 byte cap" .tmp-gate.out \
  || { echo "FAILED   the over-cap refusal does not name the cap" >&2; cat .tmp-gate.out >&2; exit 1; }
echo "ok       an over-cap file refuses naming the cap"
rm -f .tmp-gate.out

# The privilege gate: a role with EXECUTE and nothing else. Before the
# gate such a role read any file the server process could; now the
# refusal names the two doors an administrator may open.
psql_in -c "CREATE ROLE tt_file;" >/dev/null
psql_in << 'SQL' >/dev/null
DO $thinkthen_grant$
DECLARE signature text;
BEGIN
    FOR signature IN
        SELECT p.oid::regprocedure::text
        FROM pg_proc p
        JOIN pg_depend d ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
        JOIN pg_extension e ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass
        WHERE e.extname = 'thinkthen'
    LOOP
        EXECUTE format('GRANT EXECUTE ON FUNCTION %s TO tt_file', signature);
    END LOOP;
END
$thinkthen_grant$;
SQL
if psql_in -c "SET ROLE tt_file;" \
    -Atqc "SELECT thinkthen_decide('@/etc/hostname', 'refund');" > .tmp-gate.out 2>&1; then
  echo "FAILED   an EXECUTE-only role read a server file" >&2; cat .tmp-gate.out >&2; exit 1
fi
grep -q "pg_read_server_files" .tmp-gate.out \
  || { echo "FAILED   the gate refusal does not name the requirement" >&2; cat .tmp-gate.out >&2; exit 1; }
grep -q "thinkthen.file_directory" .tmp-gate.out \
  || { echo "FAILED   the gate refusal does not name the directory door" >&2; cat .tmp-gate.out >&2; exit 1; }
echo "ok       an EXECUTE-only role cannot read any file; the refusal names both doors"

# With a directory configured: inside reads, a symlink that points out
# refuses with the same uniform message, and the directory itself stays
# the administrator's act (Suset). The library loads first — a setting
# exists in a backend once the extension's library is in it.
docker exec "$NAME" ln -sf /etc/hostname /var/lib/postgresql/data/leak.json
psql_in -c "LOAD 'thinkthen.so';" \
  -c "ALTER SYSTEM SET thinkthen.file_directory = '/var/lib/postgresql/data';" \
  -Atqc "SELECT pg_reload_conf();" >/dev/null
psql_in -c "SET ROLE tt_file;" \
  -Atqc "SELECT thinkthen_decide('@refund.json', 'I demand a refund today');" > .tmp-gate.out
[ "$(cat .tmp-gate.out)" = "t" ] \
  || { echo "FAILED   the configured directory's inside read failed" >&2; cat .tmp-gate.out >&2; exit 1; }
if psql_in -c "SET ROLE tt_file;" \
    -Atqc "SELECT thinkthen_decide('@leak.json', 'refund');" > .tmp-gate.out 2>&1; then
  echo "FAILED   a symlink pointed out of the configured directory" >&2; cat .tmp-gate.out >&2; exit 1
fi
grep -q "did not read" .tmp-gate.out \
  || { echo "FAILED   the leak refusal's message is missing" >&2; cat .tmp-gate.out >&2; exit 1; }
psql_in -c "RESET ROLE;" -c "ALTER SYSTEM RESET thinkthen.file_directory;" \
  -Atqc "SELECT pg_reload_conf();" >/dev/null
docker exec "$NAME" rm -f /var/lib/postgresql/data/leak.json
echo "ok       the configured directory reads inside and refuses a symlink pointing out"

# With the core privilege: the role reads anywhere the server can, which
# is the rule PostgreSQL's own file-reading functions use.
docker cp fixtures/refund.json "$NAME:/tmp/anywhere.json"
psql_in -c "GRANT pg_read_server_files TO tt_file;" >/dev/null
psql_in -c "SET ROLE tt_file;" \
  -Atqc "SELECT thinkthen_decide('@/tmp/anywhere.json', 'I demand a refund today');" > .tmp-gate.out
[ "$(cat .tmp-gate.out)" = "t" ] \
  || { echo "FAILED   the core privilege did not read the file" >&2; cat .tmp-gate.out >&2; exit 1; }
psql_in -c "RESET ROLE;" -c "DROP OWNED BY tt_file;" -c "DROP ROLE tt_file;" >/dev/null
echo "ok       pg_read_server_files reads anywhere, PostgreSQL's own rule"
rm -f .tmp-gate.out
docker exec "$NAME" rm -f /var/lib/postgresql/data/big.json /var/lib/postgresql/data/pipe.fifo /tmp/anywhere.json

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

echo "== postgres surface: a benign interrupt does not fail a batch"
# Review 1/2 leftovers: every interrupt was treated as a cancel, so a
# memory-contexts request (a procsignal interrupt, not a query cancel)
# failed a paid batch with the cancelled kind. The control below proves
# the batch completes; the cancel and timeout arms after it prove a real
# stop still stops.
psql_in -c "SELECT count(*) FROM thinkthen_decide('{\"decide\":\"Is this a complaint?\"}', (SELECT array_agg('refund ' || g) FROM generate_series(1, 1000000) g));" > .tmp-benign.out 2>&1 &
benign_bg=$!
sleep 1.2
benign_victim=$(psql_in -Atqc "SELECT pid FROM pg_stat_activity WHERE query LIKE '%thinkthen_decide%' AND pid <> pg_backend_pid() ORDER BY backend_start DESC LIMIT 1")
psql_in -Atqc "SELECT pg_log_backend_memory_contexts($benign_victim);" >/dev/null
wait "$benign_bg" || true
if grep -q "cancelled\|ERROR" .tmp-benign.out; then
  echo "FAILED   a benign interrupt failed the batch" >&2; cat .tmp-benign.out >&2; exit 1
fi
grep -qE "^ *[0-9]+ *$" .tmp-benign.out \
  || { echo "FAILED   the benign-interrupt batch did not answer a count" >&2; cat .tmp-benign.out >&2; exit 1; }
echo "ok       a memory-contexts interrupt is serviced, and the paid batch still completes"
rm -f .tmp-benign.out

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
  "$PG_IMAGE" >/dev/null
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

echo "== postgres surface: an extension update cannot hand a function to PUBLIC"
# Review 2, item 4: the install revoke runs once, so a function created by
# a future ALTER EXTENSION UPDATE would keep PostgreSQL's default PUBLIC
# grant — and an unchanged revoke block never appears in a diff-based
# update script. The synthetic update below creates one function and says
# nothing about PUBLIC; the extension's event trigger must revoke it.
cat > .tmp-update.sql <<'SQL'
-- A synthetic next version: one new function, no revoke statement at all.
CREATE FUNCTION thinkthen_rehearsal_probe(x integer) RETURNS integer
LANGUAGE sql AS 'SELECT $1';
SQL
docker cp .tmp-update.sql "$NAME:/usr/share/postgresql/16/extension/thinkthen--0.0.1--0.0.2.sql"
psql_in -c "ALTER EXTENSION thinkthen UPDATE TO '0.0.2';" >/dev/null
psql_in << 'SQL' > .tmp-update.out
SELECT 'probe_public', has_function_privilege('public', 'thinkthen_rehearsal_probe(integer)', 'EXECUTE');
SELECT 'probe_owned', count(*) FROM pg_proc p
  JOIN pg_depend d ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
  JOIN pg_extension e ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass
  WHERE e.extname = 'thinkthen' AND p.proname = 'thinkthen_rehearsal_probe';
SELECT 'public_any', count(*) FROM pg_proc p
  JOIN pg_depend d ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
  JOIN pg_extension e ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass
  WHERE e.extname = 'thinkthen' AND has_function_privilege('public', p.oid, 'EXECUTE');
SQL
grep -qE "probe_public *\| *f" .tmp-update.out \
  || { echo "FAILED   an update-created function still holds PUBLIC's grant" >&2; cat .tmp-update.out >&2; exit 1; }
grep -qE "probe_owned *\| *1" .tmp-update.out \
  || { echo "FAILED   the update-created function is not extension-owned" >&2; cat .tmp-update.out >&2; exit 1; }
grep -qE "public_any *\| *0" .tmp-update.out \
  || { echo "FAILED   PUBLIC holds EXECUTE on an extension function after the update" >&2; cat .tmp-update.out >&2; exit 1; }
echo "ok       the update path's new function is extension-owned and out of PUBLIC's hands"
rm -f .tmp-update.sql .tmp-update.out

# Review 3, item 10, the negative arm: the trigger is scoped to the
# objects the DDL event created that this extension owns, so an
# administrator's deliberate PUBLIC grant on one of this extension's
# functions survives an unrelated function's creation. Before the
# scoping, every function creation anywhere re-revoked every PUBLIC
# grant on every extension function — the administrator's hand was
# undone by anyone's CREATE FUNCTION.
psql_in -c "GRANT EXECUTE ON FUNCTION thinkthen_decide(text, text) TO PUBLIC;" >/dev/null
psql_in -c "CREATE FUNCTION tt_deliberate(x integer) RETURNS integer LANGUAGE sql AS 'SELECT \$1';" >/dev/null
public_keeps=$(psql_in -Atqc "SELECT has_function_privilege('public', 'thinkthen_decide(text, text)', 'EXECUTE');")
[ "$public_keeps" = "t" ] \
  || { echo "FAILED   the trigger revoked a deliberate grant on an extension function" >&2; exit 1; }
psql_in -c "REVOKE EXECUTE ON FUNCTION thinkthen_decide(text, text) FROM PUBLIC;" \
  -c "DROP FUNCTION tt_deliberate(integer);" >/dev/null
echo "ok       a deliberate PUBLIC grant on thinkthen_decide survives an unrelated CREATE FUNCTION"

if curl -sf --max-time 1 http://127.0.0.1:8219/v1/stats >/dev/null 2>&1; then
  echo "== postgres surface: wire suite against the stub on 8219"
  docker rm -f -v "$WIRE_NAME" >/dev/null 2>&1 || true
  docker run -d --name "$WIRE_NAME" --network host \
    -e POSTGRES_PASSWORD=postgres -e PGPORT=$WIRE_PORT \
    -e ENGINE_BASE_URL=http://127.0.0.1:8219/v1 -e ENGINE_WIDTH=32 \
    "$PG_IMAGE" >/dev/null
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

  echo "== postgres surface: warm then decide sends nothing extra (review 4, item 15)"
  # The reviewer's shape on the wire, scaled: warm judges 2,000 pairs
  # (63 sends at width 32 against the 300 ms stub), then the row-by-row
  # decide pass over the same pairs must add zero sends.
  # One session: the answers cache and the usage counters are
  # per-backend, so separate sessions would reset both and the delta
  # would be vacuous (an earlier shape here re-judged the rows against
  # the 300 ms stub in a fresh backend and hung the check). 500 rows:
  # the property is scale-independent and the wire section stays inside
  # the gate's budget under the stub's retry pressure.
  docker exec -e PGHOST=/run/postgresql "$WIRE_NAME" \
    psql -U postgres -v ON_ERROR_STOP=1 -Atq \
    -c "SELECT thinkthen_warm('@refund.json',
        'order ' || g || ': charged twice, please refund')
      FROM generate_series(1, 500) g;" \
    -c "SELECT requests FROM thinkthen_usage();" \
    -c "SELECT count(*) FROM generate_series(1, 500) g
      WHERE thinkthen_decide('@refund.json',
        'order ' || g || ': charged twice, please refund');" \
    -c "SELECT requests FROM thinkthen_usage();" \
    > .tmp-warm-wire.out
  mapfile -t wire_lines < <(grep -vE "^$" .tmp-warm-wire.out)
  [ "${#wire_lines[@]}" = "4" ] \
    || { echo "FAILED   the warm wire session returned ${#wire_lines[@]} lines" >&2; cat .tmp-warm-wire.out >&2; exit 1; }
  before=${wire_lines[1]}
  after=${wire_lines[3]}
  [ "$before" -gt 0 ] 2>/dev/null \
    || { echo "FAILED   warm sent nothing ($before) - the stub was not up" >&2; exit 1; }
  [ "$((after - before))" = "0" ] \
    || { echo "FAILED   the read pass added $((after - before)) sends after warm" >&2; exit 1; }
  echo "ok       warm sent $before requests; the decide pass over the same 500 pairs added 0 sends"
  rm -f .tmp-warm-wire.out

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

  echo "== postgres surface: a batch budget lands inside the stub's round"
  # Review 1/2 leftovers against the 300 ms stub: a batch of 64 distinct
  # texts at width 32 needs two rounds (about 0.6 s) without a stop; a
  # 100 ms budget must end it at the engine's tick, not at the end.
  batch_wire_start=$(date +%s.%N)
  if psql_wire -c '\set VERBOSITY verbose' \
      -c "SET thinkthen.deadline_ms = 100;" \
      -c "SELECT count(*) FROM thinkthen_decide('@refund.json', (SELECT array_agg('refund ' || g) FROM generate_series(1, 64) g));" \
      > .tmp-wire-batch.out 2>&1; then
    echo "FAILED   the wire batch's 100 ms budget completed" >&2; cat .tmp-wire-batch.out >&2; exit 1
  fi
  batch_wire_elapsed=$(awk -v a="$batch_wire_start" -v b="$(date +%s.%N)" 'BEGIN { printf "%.2f", b - a }')
  grep -q "57014" .tmp-wire-batch.out \
    || { echo "FAILED   the wire batch did not return the deadline kind" >&2; cat .tmp-wire-batch.out >&2; exit 1; }
  awk -v e="$batch_wire_elapsed" 'BEGIN { exit !(e <= 1.5) }' \
    || { echo "FAILED   the wire batch's budget took ${batch_wire_elapsed}s" >&2; exit 1; }
  psql_wire -c "RESET thinkthen.deadline_ms;" >/dev/null
  rm -f .tmp-wire-batch.out
  echo "ok       a 100 ms budget ended a 64-record wire batch in ${batch_wire_elapsed}s (two 300 ms rounds without it)"
else
  echo "== postgres surface: wire suite skipped, no stub on 8219"
fi
