#!/usr/bin/env bash
# The PostgreSQL surface's check (ticket 0111). It builds the
# extension against pinned PostgreSQL 16.15 headers and runs a local server from
# the pinned package as this user on a socket with no TCP port, and restarts
# it for each test with that test's own loopback backend (ticket 0117) and
# answer cache. No test reaches a paid backend: the server gets a fake key
# beside a 127.0.0.1 address alone. Exit 0 passes, 77 is "not run".
# The surfaces rung runs `sh check.sh`, and the steps need bash.
[ -n "${BASH_VERSION:-}" ] || exec bash "$0" "$@"
set -euo pipefail
unset THINKTHEN_API_KEY RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
# Retained one-record listener fixtures keep their historical wire identity.
# Packing cases explicitly select the new default with `SET thinkthen.batch`.
export THINKTHEN_BATCH=1
cd "$(dirname "$0")"
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress) ;; *) echo "postgresql: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS
case $(uname -s) in Linux|Darwin) ;; *) echo "not run: no PostgreSQL host route for $(uname -s)"; exit 77 ;; esac
. ../../sdlc/scripts/scratch.sh
# ADR 0113: this run's engines write a scratch usage folder, never the real one.
usage_home
. ./runtime.sh
runtime_ready
REPO=$(cd ../.. && pwd)
# macOS has no `timeout` (ticket 0128). runtime.sh uses it too.
LIMIT=$REPO/sdlc/scripts/time-limit
# The extension version names its SQL files (ticket 0128).
EXT_VERSION=$(sed -n "s/^default_version = '\(.*\)'$/\1/p" thinkthen.control)
BACKEND=${CARGO_TARGET_DIR:-$REPO/target}/debug/conformance-backend
COMMAND=${CARGO_TARGET_DIR:-$REPO/target}/debug/thinkthen

[ -n "${THINKTHEN_ARTIFACT:-}" ] || {
	echo "== build"
	cargo fmt --check
	cargo clippy --locked --offline --all-targets -- -D warnings
	cargo test --locked --offline --lib
}
(cd "$REPO" && cargo build --locked --offline --quiet --package conformance-backend --package thinkthen --bin conformance-backend --bin thinkthen)
export RUSTFLAGS="--remap-path-prefix=$HOME=/build"
# package.sh reads the same pg_config and target folder (ticket 0128).
PG_CONFIG=${PG_CONFIG:-$(if [ "$PG_HOST" = Darwin ]; then echo "$EXTRACTED/bin/pg_config"; else echo /usr/bin/pg_config; fi)}
EXT=${CARGO_TARGET_DIR:-target}/release/thinkthen-pg16
# The shipped build, which `package.sh --reuse` packs (ticket 0128).
SHIPPED=$EXT-shipped

runtime_open
cleanup() {
	[ ! -f "$DATA/postmaster.pid" ] || "$BIN/pg_ctl" -D "$DATA" -m immediate stop >/dev/null 2>&1 || true
	if [ "$PG_HOST" = Darwin ] && [ -n "${RUNTIME_LIBRARY_DIR:-}" ]; then
		rm -f -- "$RUNTIME_LIBRARY_DIR"/thinkthen.so "$RUNTIME_LIBRARY_DIR"/thinkthen.dylib "$RUNTIME_EXTENSION_DIR"/thinkthen.control "$RUNTIME_EXTENSION_DIR"/thinkthen--*.sql
	fi
	[ -z "${BPID:-}" ] || backend_stop
	scratch_clean
	rm -f .runtime/last-run
}
trap cleanup EXIT INT TERM

PASSED=0 FAILED=0
# Each step runs in a subshell under errexit, so its first failing line fails
# it. STEPS, when set, names the steps to run, for the planted-bug runs.
check() {
	if [ "$profile" = stress ]; then
		[ "$1" = twenty_thousand_keyed_rows ] || return 0
	else
		[ "$1" != twenty_thousand_keyed_rows ] || return 0
	fi
	case " ${STEPS:-$1} " in *" $1 "*) ;; *) return 0 ;; esac
	set +e
	(
		set -e
		"$@"
	)
	code=$?
	set -e
	if [ "$code" = 0 ]; then
		PASSED=$((PASSED + 1))
		echo "ok       $1"
	else
		FAILED=$((FAILED + 1))
		echo "FAILED   $1"
	fi
}
# has TEXT NEEDLE: match the whole literal needle, including any newlines.
has() { [[ $1 == *"$2"* ]] || { printf 'missing: %s\nin: %s\n' "$2" "$1" >&2; return 1; }; }
hasnt() { [[ $1 != *"$2"* ]] || { printf 'unexpected: %s\nin: %s\n' "$2" "$1" >&2; return 1; }; }
same() { [ "$1" = "$2" ] || { printf 'want: %s\ngot:  %s\n' "$2" "$1" >&2; return 1; }; }
a_multiline_needle_must_stay_whole() {
	has $'first\nlast' $'first\nlast'
	if has first $'first\nlast' 2>/dev/null; then return 1; fi
	if has last $'\nlast' 2>/dev/null; then return 1; fi
}
check a_multiline_needle_must_stay_whole
now_ms() { echo $((${EPOCHREALTIME/./} / 1000)); }
within() { [ "$1" -le "$2" ] || { echo "took ${1} ms, over ${2} ms" >&2; return 1; }; }
Q='{"decide":"Is this a complaint?"}'
rows() { echo "(SELECT jsonb_object_agg(g::text, 'record ' || g ORDER BY g) FROM generate_series(1, $1) g)"; }
# A statement's elapsed milliseconds, read inside the server.
timed() {
	q -c "DO \$t\$ DECLARE s timestamptz := clock_timestamp(); n bigint; BEGIN $1; RAISE NOTICE 'elapsed %', round(extract(epoch FROM clock_timestamp() - s) * 1000); END \$t\$;" |
		sed -n 's/.*elapsed \([0-9]*\).*/\1/p'
}
# The pid of the backend running a thinkthen call.
victim() { q -c "SELECT pid FROM pg_stat_activity WHERE query LIKE '%thinkthen_%' AND pid <> pg_backend_pid() LIMIT 1"; }

echo "== package"
[ -z "${THINKTHEN_ARTIFACT:-}" ] || {
	# The installed-file mode (ticket 0128): the server loads the files unpacked from the
	# release archive, and runs the drawn SQL, the examples, and the shared cases.
	mkdir "$RUN/artifact" && tar -xzf "$THINKTHEN_ARTIFACT" -C "$RUN/artifact"
	runtime_install "$RUN/artifact/lib" "$RUN/artifact/extension"
	STEPS=${STEPS:-examples slide_sample plain_question_contract portable_batch_identity recognize_and_relate_as_drawn conformance find_inputs find_proxy_cases find_cancel find_signatures_are_owned_and_private the_fake_key_stays_in_the_environment}
}
[ -n "${THINKTHEN_ARTIFACT:-}" ] || {
	./pgrx-package-locked.sh --pg-config "$PG_CONFIG" >/dev/null
	mkdir -p "$SHIPPED" && cp -a "$EXT/." "$SHIPPED/"
	./pgrx-package-locked.sh --pg-config "$PG_CONFIG" --features panic-probe >/dev/null
	runtime_install "$EXT$("$PG_CONFIG" --pkglibdir)" "$EXT$("$PG_CONFIG" --sharedir)/extension"
}
cp fixtures/*.json "$DATA/"
fresh generic
qs -v ON_ERROR_STOP=1 -c "CREATE EXTENSION thinkthen" -f fixtures/tickets.sql -f fixtures/alerts.sql -f fixtures/inbox.sql >/dev/null

echo "== the tree and the scripts"
shipped_lacks_probe() {
	same "$(grep -c thinkthen_panic_probe "$SHIPPED$("$PG_CONFIG" --sharedir)/extension/thinkthen--$EXT_VERSION.sql" || true)" 0
}
check shipped_lacks_probe
no_home_in_library() { same "$(grep -ac -- "$HOME" "$SHIPPED$("$PG_CONFIG" --pkglibdir)"/thinkthen.* || true)" 0; }
check no_home_in_library
# The backend guard is the one catch (ticket 0310), and each SQL function
# body runs under it.
one_catch_unwind() { same "$(grep -rl catch_unwind src)" src/call.rs; same "$(grep -c catch_unwind src/call.rs)" 1; }
check one_catch_unwind
every_function_is_guarded() {
	same "$(cat src/*.rs | grep -c '^#\[pg_extern')" "$(cat src/*.rs | grep -c '^    call::guarded(||')"
}
check every_function_is_guarded
every_cargo_call_is_locked_and_offline() {
	same "$(grep -hE '^\s*(\(.*&& )?cargo (build|test|clippy|check|run)' check.sh | grep -vc -- '--locked --offline' || true)" 0
}
check every_cargo_call_is_locked_and_offline
start_refuses_other_hosts() {
	for url in http://127.0.0.1:80@example.com/ http://127.0.0.1.example.com/ http://localhost:9/; do
		set +e
		pg_start "$url" "$RUN" 2>/dev/null
		code=$?
		set -e
		same "$code" 2
	done
}
check start_refuses_other_hosts
darwin_runs() {
	mkdir -p "$RUN/shim"
	printf '#!/bin/sh\necho Darwin\n' >"$RUN/shim/uname"
	chmod +x "$RUN/shim/uname"
	set +e
	# A missing toolchain folder stops a run that passes the kernel check
	# at "not run", before it builds, sweeps, or starts a server.
	out=$(PATH="$RUN/shim:$PATH" PG_CONFIG="$RUN/none/pg_config" sh "$LIMIT" 60 bash ./check.sh 2>&1)
	code=$?
	set -e
	same "$code" 77
	case $out in *"not run: brew is missing"*|*"the pinned pg_config at $RUN/none/pg_config is missing"*) ;; *) echo "$out" >&2; return 1 ;; esac
}
check darwin_runs

echo "== the drawn SQL and the examples"
examples() { python3 tests/examples.py "$SOCK"; }
check examples
slide_sample() {
	fresh generic
	out=$(q -c "SELECT id FROM (SELECT id, thinkthen_decide('@refund.json', body) AS asks_refund FROM tickets) AS judged WHERE asks_refund IS NULL" \
		-c "SELECT id, triage->>'team', (triage->>'urgency')::float AS urgency FROM tickets, thinkthen_annotate('@form.json', body) AS triage ORDER BY urgency DESC, id")
	same "$out" "$(printf '1|billing|0.15\n2|billing|0.15\n3|billing|0.15')"
}
check slide_sample
plain_question_contract() {
	fresh case/02-decide-no
	same "$(q -c "SELECT thinkthen_decide('Does this need attention?', 'A short note.')")" f
	same "$(bcount)" 1
	fresh case/06-choose-billing
	same "$(q -c "SELECT thinkthen_choose('Which team owns this?', 'Route this note.', ARRAY['billing','shipping','other'])")" billing
	same "$(bcount)" 1
	fresh case/11-score-middle
	same "$(q -c "SELECT thinkthen_score('How severe is this?', 'Rate this note.', ARRAY['low','medium','high'])")" 1
	same "$(bcount)" 1
	fresh case/09-tag-two
	same "$(q -c "SELECT array_to_string(thinkthen_tag('Which labels apply?', 'Classify this note.', ARRAY['billing','urgent','security']), ',')")" billing,urgent
	same "$(bcount)" 1
	fresh generic
	same "$(q -c "SELECT thinkthen_decide('Does this need attention?', 'A short note.', context => 'Use this policy note.')")" t
	same "$(bcount)" 1
	fresh generic
	same "$(q -c "SELECT count(*) FROM thinkthen_decide_many('Does this need attention?', '{\"a\":\"A short note.\"}'::jsonb)")" 1
	same "$(bcount)" 1
	fresh generic
	same "$(q -c "SELECT thinkthen_try_details('Does this need attention?', 'A short note.')->>'status'")" answered
	same "$(bcount)" 1
	fresh arm/full/capture
	digest=$(q -c "SELECT thinkthen_details('Does this need attention?', 'A short note.')->'meta'->'requests'->>0")
	same "$(bcount)" 1
	# A row names its question keys, not a request digest (ADR 0111 section 2).
	bcapture | python3 -c '
import json, sys
sys.path.insert(0, "../../conformance/children")
from portable import question_keys
case = next(one for one in json.load(open("../../conformance/cases.json"))["cases"] if one["id"] == "02-decide-no")
bodies = json.load(sys.stdin)["bodies"]
assert bodies == [case["exchanges"][0]["request"]], (bodies, case["exchanges"][0]["request"])
address = f"http://127.0.0.1:{sys.argv[1]}/arm/full/capture/v1/systemone"
expected = question_keys(address, bodies[0])[0]
assert sys.argv[2] == expected, (sys.argv[2], expected)
' "$BPORT" "$digest"
	has "$(q -c "SELECT thinkthen_decide('{broken', 'A short note.')")" "thinkthen usage:"
	has "$(q -c "SELECT thinkthen_decide('@missing-question.json', 'A short note.')")" "thinkthen local:"
	same "$(bcount)" 1
}
check plain_question_contract
plan_and_named() {
	fresh arm/full/capture
	local out
	out=$(q -c "SELECT thinkthen_plan('asks for a refund', '{\"7\":\"Refund me please.\"}'::jsonb, '{}'::json)::text")
	python3 -c 'import json,sys
p=json.loads(sys.argv[1]); expected="{\"state\":\"Each question quotes the text it asks about.\",\"model\":\"jev-1.13.0\",\"questions\":{\"q1\":{\"type\":\"noul\",\"instructions\":\"The text is \\\"Refund me please.\\\". asks for a refund\"}}}"
assert p["records"] == 1 and p["requests"] == 1, p
assert p["estimated_bytes"] == 182 and p["estimated_input_tokens"] == {"lower":93,"upper":166}, p
assert p["upper_bound"] is False and p["first_body_utf8"] == expected, p' "$out"
	same "$(bcount)" 0
	for sql in \
		"SELECT thinkthen_plan('asks for a refund', '{\"7\":\"Refund me please.\"}'::jsonb, '{\"bogus\":1}'::json)" \
		"SELECT thinkthen_plan('asks for a refund', '{\"7\":\"Refund me please.\"}'::jsonb, '{\"max_estimated_input_tokens_total\":5}'::json)" \
		"SELECT thinkthen_plan('asks for a refund', '{\"7\":\"Refund me please.\"}'::jsonb, '{\"model\":\"a\",\"model\":\"b\"}'::json)" \
		"SELECT thinkthen_decide('asks for a refund', 'Refund me please.', settings => '{\"threshold\":\"0.3:0.7\"}'::json, threshold => '0.3:0.7')"; do
		out=$(q -c "$sql")
		has "$out" 'thinkthen usage:'
	done
	same "$(bcount)" 0
	out=$(q -c "SELECT thinkthen_decide('asks for a refund', 'Refund me please.', threshold => '0.3:0.7')")
	hasnt "$out" ERROR
	same "$(bcount)" 1
	fresh arm/full/capture
	out=$(q -c "SELECT key || ':' || coalesce(value::text, 'null') FROM thinkthen_decide_many('asks for a refund', '{\"7\":\"Refund me please.\",\"9\":\"No thanks.\"}'::jsonb, '{\"batch\":\"max\"}'::json) ORDER BY key")
	same "$out" $'7:true\n9:true'
	same "$(bcount)" 1
	bcapture | python3 -c 'import json,sys
bodies=json.load(sys.stdin)["bodies"]
wanted="{\"state\":\"Each question quotes the text it asks about.\",\"model\":\"jev-1.13.0\",\"questions\":{\"q1\":{\"type\":\"noul\",\"instructions\":\"The text is \\\"Refund me please.\\\". asks for a refund\"},\"q2\":{\"type\":\"noul\",\"instructions\":\"The text is \\\"No thanks.\\\". asks for a refund\"}}}"
assert bodies == [wanted], (bodies, wanted)'
}
check plan_and_named
merged_settings_conflicts() {
	fresh arm/full/capture
	local out
	out=$(q -c '\set VERBOSITY verbose' -c "SELECT thinkthen_plan('{\"decide\":\"asks for a refund\",\"batch\":1}', '{\"7\":\"Refund me please.\"}'::jsonb, '{\"batch\":2}'::json)")
	has "$out" '22023'
	has "$out" 'thinkthen usage: settings repeats `batch` from the question or named arguments'
	same "$(bcount)" 0
	out=$(q -c '\set VERBOSITY verbose' -c "SELECT thinkthen_decide('asks for a refund', 'Refund me please.', settings => '{\"batch\":2}'::json, batch => '1')")
	has "$out" '22023'
	has "$out" 'thinkthen usage: a JSON record holds each member name once, and one name arrived twice'
	same "$(bcount)" 0
	out=$(q -c '\set VERBOSITY verbose' -c "SELECT thinkthen_plan('asks for a refund', '{\"7\":\"Refund me please.\"}'::jsonb, '{\"max_estimated_input_tokens_total\":5}'::json)")
	has "$out" '22023'
	has "$out" 'thinkthen usage: max_estimated_input_tokens_total is not supported by this PostgreSQL host'
	same "$(bcount)" 0
	out=$(q -c '\set VERBOSITY verbose' -c "SELECT thinkthen_plan('asks for a refund', '{\"7\":\"Refund me please.\"}'::jsonb, '{\"max_estimated_input_tokens_total\":5,\"max_estimated_input_tokens_total\":6}'::json)")
	has "$out" '22023'
	has "$out" 'thinkthen usage: a JSON record holds each member name once, and one name arrived twice'
	same "$(bcount)" 0
}
check merged_settings_conflicts
e1_keyed_join_body() {
	fresh arm/full/capture "thinkthen.batch = 'max'"
	local out
	out=$(q -c "CREATE TEMP TABLE songs(id integer, title text)" \
		-c "INSERT INTO songs VALUES (1,'Here Comes the Sun'),(2,'Yellow Submarine'),(5,'Octopus''s Garden'),(9,'Penny Lane'),(14,'A Day in the Life'),(17,'Hey Jude')" \
		-c "SELECT s.id::text || ':' || d.key FROM songs s JOIN thinkthen_decide_many('The text is the title of a song by the Beatles. It appears on the album Abbey Road.', (SELECT jsonb_object_agg(id, title) FROM songs), '{\"threshold\":\"0.3:0.7\"}'::json) d ON d.key = CAST(s.id AS text) ORDER BY s.id")
	same "$out" $'1:1\n2:2\n5:5\n9:9\n14:14\n17:17'
	same "$(bcount)" 1
	bcapture | python3 -c 'import json, pathlib, sys
bodies=json.load(sys.stdin)["bodies"]
expected=pathlib.Path("fixtures/e1.request.json").read_text().removesuffix("\n")
assert bodies == [expected], (bodies, expected)'
}
check e1_keyed_join_body
ignored_key_warns() {
	fresh generic
	local out
	out=$(q -c "SET thinkthen.api_key = 'ignored-local-fixture'" -c "SELECT thinkthen_decide('Is it red?', 'a red door')" \
		-c "RESET thinkthen.api_key" -c "SELECT thinkthen_decide('Is it red?', 'a red door')")
	has "$out" "WARNING:  thinkthen.api_key is never read; unset it and set THINKTHEN_API_KEY in the server's environment"
	has "$out" "thinkthen usage: thinkthen.api_key is not read; unset it and set THINKTHEN_API_KEY in the server's environment"
	same "$(tail -n1 <<<"$out")" t
	same "$(bcount)" 1
	out=$(q -c "SET thinkthen.api_key = ''")
	hasnt "$out" WARNING
}
check ignored_key_warns
keyed_request_total() {
	fresh generic "thinkthen.max_requests_total = 1"
	local out
	out=$(q -c '\set VERBOSITY verbose' -c "SET thinkthen.batch = '1'" -c "SET thinkthen.cache = 'off'" \
		-c "SELECT count(*) FROM thinkthen_decide_many('Is it red?', '{\"a\":\"red one\",\"b\":\"red two\"}'::jsonb)")
	has "$out" '22023'
	has "$out" 'thinkthen.max_requests_total allows 1 requests in this backend, and they are spent'
	same "$(bcount)" 1
}
check keyed_request_total
named_bindings() {
	fresh generic
	local out
	out=$(q -c "SELECT thinkthen_decide('Is it red?', 'red', misspelled => 'x')")
	has "$out" 'does not exist'
	same "$(bcount)" 0
	out=$(q -c "SELECT thinkthen_details('Is it red?', 'red', model => 'judge-b', threshold => '0.3:0.7')->'meta'->>'model'")
	same "$out" judge-b
	same "$(bcount)" 1
	fresh generic
	out=$(q -c "SELECT thinkthen_choose('Which?', 'one', settings => '{\"options\":[\"first\",\"last\"]}'::json)")
	same "$out" first
	same "$(bcount)" 1
	fresh generic
	out=$(q -c "SELECT thinkthen_decide('Is it red?', 'red', threshold => '0.3:0.7', settings => '{\"true\":\"A yes means red.\"}'::json)")
	same "$out" t
	same "$(bcount)" 1
}
check named_bindings
native_members_keep_question_duplicates() {
	fresh generic
	local out
	out=$(q -c '\set VERBOSITY verbose' -c "SELECT thinkthen_choose('{\"choose\":\"first?\",\"choose\":\"second?\"}', 'evidence', ARRAY['a','b'], deadline_ms => 0)")
	has "$out" '22023'
	has "$out" 'thinkthen usage: the question file is not JSON this tool reads: a JSON record holds each member name once, and one name arrived twice'
	same "$(bcount)" 0
	same "$(q -c "SELECT thinkthen_choose('{\"choose\":\"first?\"}', 'evidence', ARRAY['a','b'])")" a
	same "$(bcount)" 1
}
check native_members_keep_question_duplicates
keyed_all_four_shapes() {
	fresh generic "thinkthen.batch = 'max'"
	same "$(q -c "SELECT key || ':' || value::text || ':' || probability::text FROM thinkthen_decide_many('$Q', '{\"a\":\"refund now\"}'::jsonb)")" 'a:true:0.9'
	same "$(bcount)" 1
	fresh generic "thinkthen.batch = 'max'"
	same "$(q -c "SELECT key || ':' || value || ':' || probability::text FROM thinkthen_choose_many('{\"choose\":\"Which team handles this?\",\"options\":[\"the refund desk\",\"other desk\"]}', '{\"a\":\"refund now\"}'::jsonb)")" 'a:the refund desk:0.9'
	same "$(bcount)" 1
	fresh generic "thinkthen.batch = 'max'"
	same "$(q -c "SELECT key || ':' || value::text FROM thinkthen_score_many('{\"score\":\"How strong is the refund claim?\",\"levels\":[\"low\",\"mid\",\"high\"]}', '{\"a\":\"maybe later\"}'::jsonb)")" 'a:0.15'
	has "$(q -c "SELECT probability FROM thinkthen_score_many('{\"score\":\"How strong?\",\"levels\":[\"low\",\"high\"]}', '{\"a\":\"maybe later\"}'::jsonb)")" 'column "probability" does not exist'
	same "$(bcount)" 1
	fresh generic "thinkthen.batch = 'max'"
	same "$(q -c "SELECT key || ':' || array_to_string(value, ',') FROM thinkthen_tag_many('{\"tag\":\"What is in this text?\",\"labels\":[\"refund\",\"shipping\"]}', '{\"a\":\"the refund and the shipping\"}'::jsonb)")" 'a:refund,shipping'
	same "$(bcount)" 1
}
check keyed_all_four_shapes
find_settings_choose_the_model() {
	fresh arm/full/capture
	local out
	out=$(q -c "SELECT thinkthen_find('Which unit?', ARRAY['first','second'], '{\"model\":\"judge-b\"}'::json)->>'value'")
	hasnt "$out" ERROR
	same "$(bcount)" 1
	bcapture | python3 -c 'import json,sys
bodies=json.load(sys.stdin)["bodies"]
assert len(bodies) == 1 and json.loads(bodies[0])["model"] == "judge-b", bodies'
}
check find_settings_choose_the_model
annotate_settings_do_not_accept_question_fields() {
	fresh generic
	local out
	out=$(q -c "SELECT thinkthen_annotate('@form.json', 'maybe later', '{\"threshold\":0.7}'::json)")
	has "$out" 'annotate does not take setting `threshold`'
	out=$(q -c "SELECT thinkthen_annotate('@form.json', 'maybe later', '{\"deadline_ms\":0}'::json)")
	has "$out" 'thinkthen deadline:'
	same "$(bcount)" 0
	out=$(q -c "SELECT thinkthen_annotate('@form.json', 'maybe later', '{\"batch\":\"max\"}'::json)::text")
	hasnt "$out" ERROR
	same "$(bcount)" 1
}
check annotate_settings_do_not_accept_question_fields
recognize_and_relate_as_drawn() {
	fresh generic
	out=$(q -c "SELECT t.id, n.text, n.kind FROM inbox t, LATERAL thinkthen_recognize(t.body, ARRAY['person','organization']) n ORDER BY t.id, n.start" \
		-c "SELECT count(*) FROM thinkthen_relate('SELECT id, body FROM alerts', ARRAY['caused_by'])" \
		-c "SELECT count(*) FROM thinkthen_relations('Maria Chen joined Northwind Freight in Chicago last spring.', '@names.json')")
	hasnt "$out" ERROR
}
check recognize_and_relate_as_drawn
from_and_to_refused() {
	fresh generic
	out=$(q -c "SELECT * FROM thinkthen_relations('Maria Chen joined Northwind Freight.', '@names-legacy.json')")
	has "$out" source
	has "$out" target
	same "$(bcount)" 0
}
check from_and_to_refused
the_256th_row_refuses() {
	fresh generic
	out=$(q -c '\set VERBOSITY verbose' -c "SELECT count(*) FROM thinkthen_relate('SELECT g, ''x'' FROM generate_series(1, 256) g', ARRAY['caused_by'])")
	has "$out" "22023"
	has "$out" "thinkthen usage: thinkthen_relate reads at most 255 rows (retryable: no)"
	same "$(bcount)" 0
}
check the_256th_row_refuses

echo "== authority"
extension_owned() {
	q -c "SELECT count(*) FROM pg_proc p JOIN pg_depend d ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass
		JOIN pg_extension e ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass WHERE e.extname = 'thinkthen' $1"
}
public_holds_nothing() {
	fresh generic
	same "$(extension_owned "AND has_function_privilege('public', p.oid, 'EXECUTE')")" 0
	[ "$(extension_owned "")" -ge 14 ]
	q -c "CREATE ROLE tt_app LOGIN" >/dev/null
	out=$(PGUSER_AS=tt_app q -c "SELECT thinkthen_decide('$Q', 'I want a refund')")
	has "$out" "permission denied for function thinkthen_decide"
	same "$(bcount)" 0
}
check public_holds_nothing
batch_signatures_are_extension_owned_and_private() {
	same "$(q -c "SELECT count(*) FROM pg_proc p JOIN pg_depend d ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass JOIN pg_extension e ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass WHERE e.extname = 'thinkthen' AND p.oid::regprocedure::text = ANY (ARRAY['thinkthen_decide_many(text,jsonb,json)', 'thinkthen_choose_many(text,jsonb,json)', 'thinkthen_score_many(text,jsonb,json)', 'thinkthen_tag_many(text,jsonb,json)', 'thinkthen_plan(text,jsonb,json)']) AND p.pronargdefaults = 1 AND p.prokind = 'f' AND p.proparallel = 'r' AND NOT has_function_privilege('public', p.oid, 'EXECUTE')")" 5
	same "$(q -c "SELECT count(*) FROM pg_proc p WHERE p.oid::regprocedure::text = ANY (ARRAY['thinkthen_decide(text,text,json,text,text,text,text,bigint)', 'thinkthen_choose(text,text,text[],json,text,text,text,text,bigint)', 'thinkthen_score(text,text,text[],json,text,text,text,text,bigint)', 'thinkthen_tag(text,text,text[],json,text,text,text,text,bigint)']) AND p.pronargdefaults >= 6 AND p.proargnames[1] = 'question' AND p.proargnames[2] = 'input'")" 4
}
check batch_signatures_are_extension_owned_and_private
find_signatures_are_owned_and_private() {
	fresh generic
	same "$(q -c "SELECT count(*) FROM pg_proc p JOIN pg_depend d ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass JOIN pg_extension e ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass WHERE e.extname = 'thinkthen' AND p.oid::regprocedure::text = ANY (ARRAY['thinkthen_find(text,text[],json)', 'thinkthen_find(text,text[],boolean)']) AND p.provolatile = 'v' AND p.proparallel = 'r' AND NOT has_function_privilege('public', p.oid, 'EXECUTE')")" 2
	grep -q 'thinkthen_find' "$RUNTIME_EXTENSION_DIR/thinkthen--$EXT_VERSION.sql"
	q -c "CREATE ROLE tt_find_app LOGIN" >/dev/null
	has "$(PGUSER_AS=tt_find_app q -c "SELECT thinkthen_find('Which?', ARRAY[]::text[])")" 'permission denied for function thinkthen_find'
	q -c "GRANT EXECUTE ON FUNCTION thinkthen_find(text,text[],json) TO tt_find_app" >/dev/null
	same "$(PGUSER_AS=tt_find_app q -c "SELECT thinkthen_find('Which?', ARRAY[]::text[]) IS NULL")" t
	same "$(bcount)" 0
}
check find_signatures_are_owned_and_private
readme_grant_is_the_fixture() {
	awk '/^The narrowed grant, byte for byte/ {on = 1; next} on && /^```/ {if (++fence == 2) exit; next} on && fence == 1' README.md |
		cmp -s - fixtures/grant.sql
}
check readme_grant_is_the_fixture
the_grant_reaches_the_extension_alone() {
	q -c "CREATE ROLE the_app_role LOGIN" -c "CREATE FUNCTION tt_control(x integer) RETURNS integer LANGUAGE sql AS 'SELECT \$1'" \
		-c "REVOKE ALL ON FUNCTION tt_control(integer) FROM PUBLIC" -c "GRANT pg_read_server_files TO the_app_role" \
		-f fixtures/grant.sql >/dev/null
	same "$(PGUSER_AS=the_app_role q -c "SELECT thinkthen_decide('@refund.json', 'I want a refund')")" t
	has "$(PGUSER_AS=the_app_role q -c "SELECT tt_control(7)")" "permission denied for function tt_control"
}
check the_grant_reaches_the_extension_alone
a_deliberate_grant_survives() {
	q -c "GRANT EXECUTE ON FUNCTION thinkthen_decide(text,text,json,text,text,text,text,bigint) TO PUBLIC" \
		-c "CREATE FUNCTION tt_deliberate(x integer) RETURNS integer LANGUAGE sql AS 'SELECT \$1'" >/dev/null
	out=$(q -c "SELECT has_function_privilege('public', 'thinkthen_decide(text,text,json,text,text,text,text,bigint)', 'EXECUTE')")
	q -c "REVOKE EXECUTE ON FUNCTION thinkthen_decide(text,text,json,text,text,text,text,bigint) FROM PUBLIC" -c "DROP FUNCTION tt_deliberate(integer)" >/dev/null
	same "$out" t
}
check a_deliberate_grant_survives
bare_text_is_never_a_path() {
	fresh generic
	same "$(q -c "SELECT thinkthen_decide('refund.json', 'x')")" t
	out=$(q -c "SELECT thinkthen_annotate('form.json', 'x')")
	has "$out" "'@form.json'; bare text is never a path"
	same "$(bcount)" 1
}
check bare_text_is_never_a_path

echo "== named files"
DID_NOT_READ="did not read: it must be a regular file at most 1048576 bytes"
dev_zero_refuses_fast() {
	fresh generic
	start=$(now_ms)
	out=$(q -c "SELECT thinkthen_decide('@/dev/zero', 'x')" -c "SELECT 1")
	within $(($(now_ms) - start)) 2000
	has "$out" "thinkthen local: the question file '@/dev/zero' $DID_NOT_READ"
	same "$(tail -n1 <<<"$out")" 1
	head -c 1048577 /dev/zero | tr '\0' ' ' >"$RUN/big.json"
	has "$(q -c "SELECT thinkthen_decide('@$RUN/big.json', 'x')")" "the question file '@$RUN/big.json' is over the 1048576 byte cap"
}
check dev_zero_refuses_fast
the_file_gate() {
	mkdir -p "$RUN/files"
	cp fixtures/refund.json "$RUN/files/"
	fresh generic
	q -c "CREATE ROLE tt_exec LOGIN" -c "GRANT EXECUTE ON FUNCTION thinkthen_decide(text,text,json,text,text,text,text,bigint) TO tt_exec" >/dev/null
	out=$(PGUSER_AS=tt_exec q -c "SELECT thinkthen_decide('@/etc/hostname', 'x')")
	has "$out" "a named file needs pg_read_server_files, or an administrator's thinkthen.file_directory"
	fresh generic "thinkthen.file_directory = '$RUN/files'"
	same "$(PGUSER_AS=tt_exec q -c "SELECT thinkthen_decide('@$RUN/files/refund.json', 'I want a refund')")" t
	has "$(PGUSER_AS=tt_exec q -c "SELECT thinkthen_decide('@/etc/hostname', 'x')")" "'@/etc/hostname' $DID_NOT_READ"
	q -c "GRANT pg_read_server_files TO tt_exec" >/dev/null
	cp fixtures/refund.json "$RUN/anywhere.json"
	same "$(PGUSER_AS=tt_exec q -c "SELECT thinkthen_decide('@$RUN/anywhere.json', 'I want a refund')")" t
}
check the_file_gate
bad_files_name_themselves() {
	fresh generic
	for call in "SELECT thinkthen_decide('@no-such-file.json', 'a')" \
		"SELECT count(*) FROM thinkthen_decide_many('@no-such-file.json', '{\"a\":\"a\",\"b\":\"b\"}'::jsonb)"; do
		has "$(q -c "$call")" "thinkthen local: the question file '@no-such-file.json' $DID_NOT_READ"
	done
	for call in "SELECT thinkthen_decide('@broken.json', 'a')" \
		"SELECT count(*) FROM thinkthen_decide_many('@broken.json', '{\"a\":\"a\",\"b\":\"b\"}'::jsonb)"; do
		has "$(q -c "$call")" "thinkthen local: the question file '@broken.json' does not parse: "
	done
	has "$(q -c "SELECT count(*) FROM thinkthen_decide_many('{\"choose\":\"Which?\",\"options\":[\"a\",\"b\"]}', '{\"a\":\"one\"}'::jsonb)")" \
		'thinkthen usage:'
	same "$(bcount)" 0
}
check bad_files_name_themselves

echo "== deadlines and cancels"
start=$(now_ms)
q -c "SELECT 1" >/dev/null
echo "         psql start: $(($(now_ms) - start)) ms"
deadline_setting_range() {
	fresh generic
	has "$(q -c "LOAD 'thinkthen'" -c "SET thinkthen.deadline_ms = -2")" "-2 is outside the valid range for parameter \"thinkthen.deadline_ms\""
	same "$(q -c "SET thinkthen.deadline_ms = -1" -c "SELECT thinkthen_decide('$Q', 'a')")" t
	for call in "SELECT thinkthen_decide('$Q', 'b')" "SELECT count(*) FROM thinkthen_decide_many('$Q', '{\"c\":\"c\",\"d\":\"d\"}'::jsonb)" \
		"SELECT count(*) FROM thinkthen_decide_many('$Q', '{\"a\":\"first\",\"b\":\"second\"}'::jsonb)"; do
		out=$(q -c '\set VERBOSITY verbose' -c "SET thinkthen.deadline_ms = 0" -c "$call")
		has "$out" "57014"
		has "$out" "thinkthen deadline"
	done
	same "$(bcount)" 1
}
check deadline_setting_range
# held STATEMENT: run the statement in the background. Each timed step
# starts the server first, so its clock holds psql's start and nothing more.
held() {
	q -c '\set VERBOSITY verbose' -c "$1" >"$RUN/held.out" &
	HELD=$!
}
single_cancel() {
	fresh arm/held
	held "SELECT thinkthen_decide('$Q', 'held')"
	bwait 1
	start=$(now_ms)
	q -c "SELECT pg_cancel_backend($(victim))" >/dev/null
	wait "$HELD" || true
	within $(($(now_ms) - start)) 200
	has "$(cat "$RUN/held.out")" "canceling statement due to user request"
	brelease
	sleep 0.2
	same "$(bcount)" 1
}
check single_cancel
find_cancel() {
	fresh arm/held
	held "SELECT thinkthen_find('Which?', ARRAY['one','two'])"
	bwait 1
	q -c "SELECT pg_cancel_backend($(victim))" >/dev/null
	wait "$HELD" || true
	has "$(cat "$RUN/held.out")" "canceling statement due to user request"
	brelease
	same "$(bcount)" 1
}
check find_cancel
single_statement_timeout() {
	fresh arm/held
	start=$(now_ms)
	held "SET statement_timeout = '300ms'; SELECT thinkthen_decide('$Q', 'held')"
	wait "$HELD" || true
	within $(($(now_ms) - start)) 500
	has "$(cat "$RUN/held.out")" "canceling statement due to statement timeout"
	brelease
	sleep 0.2
	same "$(bcount)" 1
}
check single_statement_timeout
single_deadline() {
	fresh arm/held
	start=$(now_ms)
	held "SET thinkthen.deadline_ms = 200; SELECT thinkthen_decide('$Q', 'held')"
	wait "$HELD" || true
	within $(($(now_ms) - start)) 1000
	has "$(cat "$RUN/held.out")" "57014"
	has "$(cat "$RUN/held.out")" "thinkthen deadline"
	brelease
	sleep 0.2
	same "$(bcount)" 1
}
check single_deadline
batch_deadline() {
	fresh arm/held "thinkthen.throttle = 8"
	start=$(now_ms)
	held "SET thinkthen.batch = '2'; SET thinkthen.deadline_ms = 1000; SELECT count(*) FROM thinkthen_decide_many('$Q', $(rows 200))"
	wait "$HELD" || true
	within $(($(now_ms) - start)) 1500
	has "$(cat "$RUN/held.out")" "thinkthen deadline"
	same "$(bcount)" 8
	brelease
}
check batch_deadline
batch_cancel() {
	fresh arm/held "thinkthen.throttle = 8"
	held "SET thinkthen.batch = '2'; SELECT count(*) FROM thinkthen_decide_many('$Q', $(rows 200))"
	bwait 8
	start=$(now_ms)
	q -c "SELECT pg_cancel_backend($(victim))" >/dev/null
	wait "$HELD" || true
	within $(($(now_ms) - start)) 200
	has "$(cat "$RUN/held.out")" "canceling statement due to user request"
	sleep 0.3
	same "$(bcount)" 8
	brelease
	sleep 0.3
	same "$(bcount)" 8
}
check batch_cancel
benign_interrupt_finishes() {
	fresh arm/held "thinkthen.throttle = 8"
	held "SET thinkthen.batch = '2'; SELECT count(*) FROM thinkthen_decide_many('$Q', $(rows 200))"
	bwait 8
	q -c "SELECT pg_log_backend_memory_contexts($(victim))" >/dev/null
	sleep 0.3
	brelease
	wait "$HELD"
	same "$(cat "$RUN/held.out")" 200
	same "$(bcount)" 100
}
check benign_interrupt_finishes
a_timed_out_batch_leaves_the_session_working() {
	fresh arm/held "thinkthen.throttle = 8"
	out=$(q -c "SET statement_timeout = '500ms'" -c "SET thinkthen.batch = '2'" -c "SELECT count(*) FROM thinkthen_decide_many('$Q', $(rows 200))" \
		-c "\\! echo release > $RUN/b.in" -c "RESET statement_timeout" -c "SELECT thinkthen_decide('$Q', 'after')")
	has "$out" "canceling statement due to statement timeout"
	same "$(tail -n1 <<<"$out")" t
	same "$(bcount)" 9
}
check a_timed_out_batch_leaves_the_session_working
a_small_batch_answers_at_once() {
	fresh generic "thinkthen.batch = '2'"
	q -c "SELECT thinkthen_decide('$Q', 'warm up')" >/dev/null
	ms=$(timed "SELECT count(*) INTO n FROM thinkthen_decide_many('$Q', '{\"a\":\"a\",\"b\":\"b\"}'::jsonb)")
	within "$ms" 90
}
check a_small_batch_answers_at_once

echo "== the answer cache"
# Keyed input preserves one packed answer and reuses its exact cache identity.
keyed_answers_reuse_without_sending() {
    fresh generic "thinkthen.batch = 'max'"
    local input='{"a":"order 1","b":"order 2","c":"order 3"}'
    same "$(q -c "SELECT count(*) FROM thinkthen_decide_many('$Q', '$input'::jsonb)")" 3
    same "$(bcount)" 1
    same "$(q -c "SELECT count(*) FROM thinkthen_decide_many('$Q', '$input'::jsonb)")" 3
    same "$(bcount)" 1
}
check keyed_answers_reuse_without_sending
keyed_banded_file_reuses() {
    fresh generic "thinkthen.batch = 'max'"
    local input='{"a":"refund one","b":"refund two"}'
    same "$(q -c "SELECT count(*) FROM thinkthen_decide_many('@refund.json', '$input'::jsonb)")" 2
    same "$(bcount)" 1
    same "$(q -c "SELECT count(*) FROM thinkthen_decide_many('@refund.json', '$input'::jsonb)")" 2
    same "$(bcount)" 1
}
check keyed_banded_file_reuses
twenty_thousand_keyed_rows() {
	fresh generic "thinkthen.batch = 'max'" "thinkthen.throttle = 32"
	local input
	input=$(rows 20000)
	same "$(q -c "SELECT count(*) FROM thinkthen_decide_many('$Q', $input)")" 20000
	local first
	first=$(bcount)
	[ "$first" -gt 0 ]
	same "$(q -c "SELECT count(*) FROM thinkthen_decide_many('$Q', $input)")" 20000
	same "$(bcount)" "$first"
}
check twenty_thousand_keyed_rows
another_model_sends_again() {
    fresh generic
    q -c "SELECT count(*) FROM generate_series(1, 2) g WHERE thinkthen_decide('{\"decide\":\"Is it red?\",\"model\":\"judge-a\"}', 'item ' || g)" \
      -c "SELECT count(*) FROM generate_series(1, 2) g WHERE thinkthen_decide('{\"decide\":\"Is it red?\",\"model\":\"judge-b\"}', 'item ' || g)" >/dev/null
    same "$(bcount)" 4
}
check another_model_sends_again
removed_forms_refuse_before_send() {
    fresh generic
    local out
    out=$(q -c "SELECT thinkthen_warm('$Q', 'one')")
    has "$out" 'thinkthen_warm was removed; pack records with thinkthen_decide_many'
    out=$(q -c "SELECT thinkthen_probability('$Q', 'one')")
    has "$out" 'thinkthen_probability was removed; read the probability column of thinkthen_decide_many'
    out=$(q -c "SELECT count(*) FROM thinkthen_decide('$Q', ARRAY['one'])")
    has "$out" 'the array form was removed; pass a keyed jsonb object to thinkthen_decide_many'
    out=$(q -c "SELECT thinkthen_decide('$Q', 'one', 'old context')")
    has "$out" 'the context argument moved into the settings object or the context named parameter'
    same "$(bcount)" 0
}
check removed_forms_refuse_before_send
the_answer_map_is_gone() {
	fresh generic
	has "$(q -c "SELECT count(*) FROM thinkthen_usage()" -c "SHOW thinkthen.saved_answer_kb")" \
		'unrecognized configuration parameter "thinkthen.saved_answer_kb"'
}
check the_answer_map_is_gone
the_environment_seeds_the_cache() {
	fresh generic
	same "$(q -c "SELECT thinkthen_decide('$Q', 'seed')" -c "SELECT thinkthen_decide('$Q', 'seed')")" "$(printf 't\nt')"
	same "$(bcount)" 1
	[ -n "$(find "$CACHEDIR" -type f | head -1)" ]
	[ -z "$(find "$SCRATCH" -type f -not -path '*/thinkthen-usage/*' | head -1)" ]
}
check the_environment_seeds_the_cache
the_cache_setting_names_the_folder() {
	mkdir -p "$RUN/named-cache"
	fresh generic "thinkthen.cache = '$RUN/named-cache'"
	same "$(q -c "SELECT thinkthen_decide('$Q', 'a')")" t
	[ -n "$(find "$RUN/named-cache" -type f | head -1)" ]
	[ -z "$(find "$CACHEDIR" -type f | head -1)" ]
}
check the_cache_setting_names_the_folder
# Ticket 0318: every role shares a named cache, so none runs unless the
# operator names one, and a folder others can write is refused.
the_platform_cache_stays_off() {
	UNNAMED_CACHE=1 fresh generic
	same "$(q -c "SELECT thinkthen_decide('$Q', 'seed')" -c "SELECT thinkthen_decide('$Q', 'seed')")" "$(printf 't\nt')"
	same "$(bcount)" 2
	[ -z "$(find "$SCRATCH" -type f -not -path '*/thinkthen-usage/*' | head -1)" ]
}
check the_platform_cache_stays_off
# ADR 0113: each backend adds its sends to the server user's usage totals when
# it exits. The check holds the usage lock while both connections disconnect,
# so only the exit hook's wait writes the counts.
usage_requests() {
	env -i PATH="$PATH" HOME="$SCRATCH" XDG_CACHE_HOME="$SCRATCH/.cache" "$COMMAND" status --json |
		python3 -c 'import json, sys; print(json.load(sys.stdin)["usage"]["total"]["requests_sent"])'
}
each_backend_adds_its_sends_at_exit() {
	fresh generic
	folder=$SCRATCH/.cache/thinkthen-usage
	[ "$PG_HOST" != Darwin ] || folder=$SCRATCH/Library/Caches/thinkthen-usage
	mkdir -p -m 700 "$folder"
	(umask 077 && : >>"$folder/.lock")
	before=$(usage_requests)
	exec {HOLD}> >(exec python3 -c 'import fcntl, os, sys
lock = os.open(sys.argv[1], os.O_RDWR | os.O_CREAT, 0o600)
fcntl.flock(lock, fcntl.LOCK_EX)
open(sys.argv[2], "w").close()
sys.stdin.read()' "$folder/.lock" "$RUN/usage-held")
	for _ in $(seq 100); do [ -e "$RUN/usage-held" ] && break; sleep 0.05; done
	[ -e "$RUN/usage-held" ]
	q -c "SELECT thinkthen_decide('$Q', 'first connection')" >"$RUN/usage-first" &
	first=$!
	q -c "SELECT thinkthen_decide('$Q', 'second connection')" >"$RUN/usage-second" &
	wait "$first" "$!"
	sleep 0.3
	exec {HOLD}>&-
	same "$(cat "$RUN/usage-first" "$RUN/usage-second")" "$(printf 't\nt')"
	same "$(bcount)" 2
	for _ in $(seq 50); do [ "$(usage_requests)" = $((before + 2)) ] && break; sleep 0.1; done
	same "$(usage_requests)" $((before + 2))
}
check each_backend_adds_its_sends_at_exit
an_open_cache_folder_is_refused() {
	mkdir -p "$RUN/open-cache" && chmod 0777 "$RUN/open-cache"
	fresh generic "thinkthen.cache = '$RUN/open-cache'"
	has "$(q -c "SELECT thinkthen_decide('$Q', 'a')")" \
		"thinkthen usage: the answer folder belongs to another user or others can write it, so they could choose its answers; make it this process user's own with mode 0700, or name another folder"
	same "$(bcount)" 0
	[ -z "$(find "$RUN/open-cache" -type f | head -1)" ]
}
check an_open_cache_folder_is_refused

echo "== engine settings"
throttle_setting_holds_eight() {
	fresh arm/held "thinkthen.throttle = 8"
	held "SET thinkthen.batch = '2'; SELECT count(*) FROM thinkthen_decide_many('$Q', $(rows 64))"
	bwait 8
	sleep 0.3
	same "$(bcount)" 8
	brelease
	wait "$HELD"
	same "$(cat "$RUN/held.out")" 64
}
check throttle_setting_holds_eight
the_throttle_keeps_the_environment() {
	fresh generic "thinkthen.throttle = 8"
	same "$(q -c "SELECT thinkthen_decide('$Q', 'env')")" t
	same "$(bcount)" 1
}
check the_throttle_keeps_the_environment
request_limit_refuses_before_sending() {
	fresh generic "thinkthen.max_requests = 3"
	has "$(q -c "SELECT count(*) FROM thinkthen_decide_many('$Q', '{\"a\":\"a\",\"b\":\"b\",\"c\":\"c\",\"d\":\"d\"}'::jsonb)")" \
		"thinkthen usage: this engine answers at most 3 records in one call (retryable: no)"
	fresh generic "thinkthen.max_requests = 0"
	out=$(q -c '\set VERBOSITY verbose' -c "SELECT thinkthen_decide('$Q', 'a')")
	has "$out" 22023
	has "$out" "a request limit is a whole number of 1 or more"
	same "$(bcount)" 0
}
check request_limit_refuses_before_sending
# Ticket 0311: the server's environment carries the estimated input limit,
# and the next step's fresh server drops it.
token_variable_refuses_before_sending() {
	THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL=10 fresh generic
	out=$(q -c "SET thinkthen.cache = 'off'" -c "SELECT thinkthen_decide('$Q', 'a')")
	has "$out" "thinkthen usage: max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) would be exceeded before this call's first request"
	same "$(bcount)" 0
}
check token_variable_refuses_before_sending
a_changed_limit_rebuilds() {
	fresh generic
	out=$(q -c "SET thinkthen.max_requests = 3" -c "SELECT count(*) FROM thinkthen_decide_many('$Q', '{\"a\":\"a\",\"b\":\"b\",\"c\":\"c\"}'::jsonb)" \
		-c "SET thinkthen.max_requests = 2" -c "SELECT count(*) FROM thinkthen_decide_many('$Q', '{\"d\":\"d\",\"e\":\"e\",\"f\":\"f\"}'::jsonb)")
	same "$(head -n1 <<<"$out")" 3
	has "$out" "this engine answers at most 2 records in one call"
	same "$(bcount)" 3
}
check a_changed_limit_rebuilds
a_changed_throttle_refuses() {
    fresh generic
    out=$(q -c '\set VERBOSITY verbose' -c "SET thinkthen.throttle = 8" \
        -c "SELECT thinkthen_decide('$Q', 'first')" -c "SELECT thinkthen_decide('$Q', 'second')" \
        -c "SET thinkthen.throttle = 6" -c "SELECT thinkthen_decide('$Q', 'third')")
    has "$out" "throttle 8 is already active for this process; use throttle 8 or drop the throttle argument"
    has "$out" "22023"
    same "$(bcount)" 2
}
check a_changed_throttle_refuses
try_details_keeps_later_rows() {
    fresh generic
    out=$(q -c "WITH rows(i,q,e) AS (VALUES (1, '$Q', 'first'), (2, '{broken', 'bad'), (3, '@missing-private-question.json', 'bad'), (4, '$Q', 'last')),
        measured AS MATERIALIZED (SELECT i, thinkthen_try_details(q,e) AS v FROM rows)
        SELECT i::text || ':' || coalesce(v->>'status', 'null') || ':' ||
            coalesce(v->'error'->>'kind', 'ok') FROM measured ORDER BY i")
    same "$out" "$(printf '1:answered:ok\n2:failed:usage\n3:failed:local\n4:answered:ok')"
    same "$(bcount)" 2
}
check try_details_keeps_later_rows
try_details_keeps_good_after_backend_failure() {
    fresh generic
    pg_stop
    mkfifo "$RUN/proxy.in"
    python3 ../sqlite/tests/conditional_backend.py "http://127.0.0.1:$BPORT/generic/v1" 'private evidence' \
        <"$RUN/proxy.in" >"$RUN/proxy.out" 2>"$RUN/proxy.err" &
    PROXYPID=$!
    exec {PROXYFD}>"$RUN/proxy.in"
    trap 'if [ -n "${PROXYPID:-}" ]; then printf "quit\n" >&"$PROXYFD"; wait "$PROXYPID" || true; fi' EXIT
    for _ in $(seq 100); do [ -s "$RUN/proxy.out" ] && break; sleep 0.05; done
    proxyport=$(head -1 "$RUN/proxy.out")
    [ -n "$proxyport" ]
    pg_start "http://127.0.0.1:$proxyport/v1" "$CACHEDIR"
    out=$(q -c "WITH rows(i,q,e) AS (VALUES (1, '$Q', 'first'), (2, '$Q', 'private evidence'), (3, '$Q', 'last')),
        measured AS MATERIALIZED (SELECT i, thinkthen_try_details(q,e) AS v FROM rows)
        SELECT i::text || ':' || (v->>'status') || ':' || coalesce(v->'error'->>'kind','ok') || ':' ||
            CASE WHEN v::text LIKE '%private evidence%' THEN 'leaked' ELSE 'safe' END FROM measured ORDER BY i")
    same "$out" "$(printf '1:answered:ok:safe\n2:failed:backend:safe\n3:answered:ok:safe')"
    same "$(bcount)" 2
    printf 'quit\n' >&"$PROXYFD"
    wait "$PROXYPID"
    PROXYPID=
    exec {PROXYFD}>&-
    same "$(tail -1 "$RUN/proxy.out")" 3
}
check try_details_keeps_good_after_backend_failure
try_details_null_skips_settings() {
    fresh generic
    out=$(q -c "SET thinkthen.api_key = 'planted-private-key'" \
        -c "SELECT thinkthen_try_details(NULL, 'private evidence') IS NULL")
    has "$out" "WARNING:  thinkthen.api_key is never read; unset it and set THINKTHEN_API_KEY in the server's environment"
    same "$(tail -n1 <<<"$out")" t
    same "$(bcount)" 0
}
check try_details_null_skips_settings
try_details_keeps_unresolved_and_spent_total_distinct() {
    fresh generic "thinkthen.max_requests_total = 1"
    out=$(q -c "SELECT (v->>'status') || ':' || coalesce(v->'details'->>'value', 'null') || ':' || (v ? 'error')::text
        FROM (SELECT thinkthen_try_details('{\"decide\":\"Is it red?\",\"threshold\":\"0:1\"}', 'red door') AS v) s" \
        -c "SELECT (v->>'status') || ':' || (v->'error'->>'kind') || ':' || (v->'error'->>'message') || ':' || (v->'error'->>'retryable') || ':' || (v ? 'details')::text
        FROM (SELECT thinkthen_try_details('$Q', 'second row') AS v) s")
    same "$out" "$(printf 'answered:null:false\nfailed:usage:check the row\047s question and arguments, or raise the process request total when it is spent:false:false')"
    same "$(bcount)" 1
}
check try_details_keeps_unresolved_and_spent_total_distinct
try_details_keeps_native_timeout() {
    fresh arm/held
    held "SET statement_timeout = '300ms'; SELECT thinkthen_try_details('$Q', 'held')"
    bwait 1
    wait "$HELD" || true
    has "$(cat "$RUN/held.out")" "canceling statement due to statement timeout"
    brelease
    same "$(bcount)" 1
}
check try_details_keeps_native_timeout
a_role_limit_applies() {
	fresh generic
	q -c "CREATE ROLE tt_limited LOGIN" -c "GRANT EXECUTE ON FUNCTION thinkthen_decide_many(text,jsonb,json) TO tt_limited" \
		-c "ALTER ROLE tt_limited SET thinkthen.max_requests = 2" >/dev/null
	has "$(PGUSER_AS=tt_limited q -c "SELECT count(*) FROM thinkthen_decide_many('$Q', '{\"a\":\"a\",\"b\":\"b\",\"c\":\"c\"}'::jsonb)")" \
		"this engine answers at most 2 records in one call"
	same "$(bcount)" 0
}
check a_role_limit_applies
# A throttle outside 1 through 32 is refused where it is set, and calls keep
# working. A configuration file's 0 once broke every call.
THROTTLE_RANGE="thinkthen usage: a throttle is a whole number from 1 through 32 (retryable: no)"
throttle_setting_range() {
	fresh generic "thinkthen.throttle = 0"
	out=$(q -c "SELECT thinkthen_decide('$Q', 'from the file')")
	has "$out" "WARNING:  $THROTTLE_RANGE"
	same "$(tail -n1 <<<"$out")" t
	for value in 0 33 -2; do
		out=$(q -c '\set VERBOSITY verbose' -c "LOAD 'thinkthen'" -c "SET thinkthen.throttle = $value" -c "SELECT thinkthen_decide('$Q', 'set $value')")
		has "$out" "ERROR:  22023: $THROTTLE_RANGE"
		same "$(tail -n1 <<<"$out")" t
	done
	same "$(bcount)" 4
}
check throttle_setting_range

# Ian's ruling of 2026-09-25: the backend's total holds across row calls.
the_total_holds_across_rows() {
	fresh generic "thinkthen.max_requests_total = 3"
	out=$(q -c "SELECT count(*) FROM generate_series(1, 10) g WHERE thinkthen_decide('$Q', 'row ' || g)" \
		-c "SELECT thinkthen_decide('$Q', 'after')")
	same "$(grep -oF "thinkthen usage: thinkthen.max_requests_total allows 3 requests in this backend, and they are spent (retryable: no)" <<<"$out" | wc -l)" 2
	same "$(bcount)" 3
	fresh generic "thinkthen.max_requests_total = 1"
	out=$(q -c "SET thinkthen.batch = '2'" -c "SET thinkthen.cache = 'off'" \
		-c "SELECT count(*) FROM thinkthen_decide_many('$Q', '{\"a\":\"b\",\"b\":\"a\",\"c\":\"c\",\"d\":\"d\"}'::jsonb)")
	has "$out" "thinkthen usage: thinkthen.max_requests_total allows 1 requests in this backend, and they are spent (retryable: no)"
	same "$(bcount)" 1
}
check the_total_holds_across_rows

keyed_batch_preserves_rows_and_attempts() {
    local input='{"a":"b","b":"a","c":"c","d":"d"}'
    fresh generic
    out=$(q -c "SET thinkthen.batch = '1'" -c "SET thinkthen.cache = 'off'" \
        -c "SET thinkthen.record = '$RUN/singleton'" \
        -c "SELECT count(*) FROM thinkthen_decide_many('$Q', '$input'::jsonb)")
    same "$out" 4
    same "$(bcount)" 4
    same "$(python3 tests/batching_cases.py singleton "$RUN/singleton")" 'pass singleton'
    fresh generic
    out=$(q -c "SET thinkthen.batch = '2'" -c "SET thinkthen.cache = 'off'" \
        -c "SET thinkthen.record = '$RUN/packed'" \
        -c "SELECT key || ':' || coalesce(value::text, 'null') FROM thinkthen_decide_many('$Q', '$input'::jsonb) ORDER BY key")
    same "$out" $'a:true\nb:true\nc:true\nd:true'
    same "$(bcount)" 2
    same "$(python3 tests/batching_cases.py packed "$RUN/packed")" 'pass packed'
    fresh generic
    out=$(q -c "SET thinkthen.batch = 'max'" -c "SET thinkthen.cache = 'off'" \
        -c "SET thinkthen.record = '$RUN/max-packed'" \
        -c "SELECT count(*) FROM thinkthen_decide_many('$Q', '$input'::jsonb)")
    same "$out" 4
    same "$(bcount)" 1
    same "$(python3 tests/batching_cases.py max "$RUN/max-packed")" 'pass max'
    fresh generic "thinkthen.max_requests_total = 1"
    out=$(q -c "SET thinkthen.batch = '2'" -c "SET thinkthen.max_retries = 0" \
        -c "SET thinkthen.cache = 'off'" -c "SET thinkthen.record = '$RUN/one-packed'" \
        -c "SELECT count(*) FROM thinkthen_decide_many('$Q', '$input'::jsonb)")
    has "$out" 'thinkthen.max_requests_total allows 1 requests in this backend, and they are spent'
    same "$(bcount)" 1
    same "$(python3 tests/batching_cases.py one_of_packed "$RUN/one-packed")" 'pass one_of_packed'
}
check keyed_batch_preserves_rows_and_attempts

portable_batch_identity() {
	fresh arm/full/capture
	out=$(q -c "SET thinkthen.batch = 'max'" -c "SET thinkthen.max_retries = 0" \
		-c "$(python3 tests/portable_batch.py sql)")
	bcapture >"$RUN/portable.capture.json"
	python3 tests/portable_batch.py verify "$out" "$(bcount)" "$RUN/portable.capture.json"
}
check portable_batch_identity

context_settings_keep_shapes_and_legacy_refusal() {
    fresh generic
    out=$(q -c "SET thinkthen.batch = '2'" -c "SET thinkthen.cache = 'off'" \
        -c "SET thinkthen.record = '$RUN/context-packed'" \
        -c "SELECT key || ':' || coalesce(value::text, 'null') FROM thinkthen_decide_many('$Q', '{\"a\":\"b\",\"b\":\"a\"}'::jsonb, '{\"context\":\"shared reference\"}'::json) ORDER BY key")
    same "$out" $'a:true\nb:true'
    same "$(bcount)" 1
    same "$(python3 tests/batching_cases.py context "$RUN/context-packed")" 'pass context'
    fresh generic
    same "$(q -c "SELECT thinkthen_decide('$Q', 'one', context => 'shared reference')")" t
    same "$(bcount)" 1
    fresh generic
    out=$(q -c "SET thinkthen.cache = 'off'" \
        -c "SELECT thinkthen_choose('{\"choose\":\"Which?\",\"options\":[\"first\",\"last\"]}', 'one', context => 'shared reference')" \
        -c "SELECT thinkthen_score('{\"score\":\"Which?\",\"levels\":[\"low\",\"high\"]}', 'one', context => 'shared reference')" \
        -c "SELECT array_length(thinkthen_tag('{\"tag\":\"Which?\",\"labels\":[\"first\",\"last\"]}', 'one', context => 'shared reference'), 1)" \
        -c "SELECT thinkthen_details('$Q', 'one', context => 'shared reference')->>'value'" \
        -c "SELECT thinkthen_try_details('$Q', 'one', context => 'shared reference')->>'status'")
    same "$out" $'first\n0.1\n2\ntrue\nanswered'
    same "$(bcount)" 5
    fresh generic
    same "$(q -c "SELECT thinkthen_try_details(NULL, 'one', context => 'shared reference') IS NULL")" t
    has "$(q -c "SELECT thinkthen_decide('$Q', 'one', 'shared reference')")" 'the context argument moved into the settings object'
    has "$(q -c "SELECT thinkthen_decide('$Q', 'one', context => '   ')")" '`context` is text that is not blank'
    same "$(bcount)" 0
}
check context_settings_keep_shapes_and_legacy_refusal

keyed_contexts_do_not_project_a_scalar_cache_entry() {
    fresh generic "thinkthen.batch = '2'"
    out=$(q -c "SET thinkthen.record = '$RUN/context-groups'" \
        -c "SELECT count(*) FROM thinkthen_decide_many('$Q', '{\"a\":\"b\",\"b\":\"a\"}'::jsonb, '{\"context\":\"first\"}'::json)" \
        -c "SELECT count(*) FROM thinkthen_decide_many('$Q', '{\"c\":\"c\",\"d\":\"d\"}'::jsonb, '{\"context\":\"second\"}'::json)")
    same "$out" $'2\n2'
    same "$(bcount)" 2
    same "$(python3 tests/batching_cases.py warm "$RUN/context-groups")" 'pass warm'
    same "$(q -c "SELECT thinkthen_decide('$Q', 'b', context => 'first')")" t
    same "$(bcount)" 3
}
check keyed_contexts_do_not_project_a_scalar_cache_entry

batch_setting_and_replay_context_validate_before_send() {
    fresh generic
    for setting in 0 +1 -1 1.5 MAX no; do
        has "$(q -c "SET thinkthen.batch = '$setting'" -c "SELECT count(*) FROM thinkthen_decide_many('$Q', '{\"a\":\"one\",\"b\":\"two\"}'::jsonb)")" \
            'thinkthen.batch is max or a whole number of 1 or more'
    done
    same "$(bcount)" 0
    fresh generic
    out=$(q -c "SET thinkthen.record = '$RUN/context-replay'" -c "SELECT thinkthen_decide('$Q', 'one', context => 'alpha')" \
        -c "SET thinkthen.record = ''" -c "SET thinkthen.replay = '$RUN/context-replay'" \
        -c "SELECT thinkthen_decide('$Q', 'one', context => 'beta')")
    has "$out" 'thinkthen local: the replay folder holds no answer for this question'
    same "$(bcount)" 1
}
check batch_setting_and_replay_context_validate_before_send

sql_settings_and_retry_total() {
	fresh arm/503
	out=$(q -c "SET thinkthen.max_requests_total = 1" -c "SET thinkthen.max_retries = 1" \
		-c "SELECT thinkthen_decide('$Q', 'one')")
	has "$out" "thinkthen usage: thinkthen.max_requests_total allows 1 requests in this backend, and they are spent (retryable: no)"
	same "$(bcount)" 1
	fresh generic
	out=$(q -c "SET thinkthen.model = 'jev-1.13.0'" -c "SET thinkthen.timeout = '2s'" \
		-c "SET thinkthen.max_retries = 0" -c "SELECT thinkthen_decide('$Q', 'one')")
	same "$out" t
	same "$(bcount)" 1
	fresh arm/delay/2000
	out=$(q -c "SET thinkthen.timeout = '1s'" -c "SET thinkthen.max_retries = 0" \
		-c "SELECT thinkthen_decide('$Q', 'slow')")
	has "$out" "thinkthen backend: the backend timed out"
	same "$(bcount)" 1
	fresh generic
	out=$(q -c "SET thinkthen.profile = '{}'" -c "SELECT thinkthen_decide('$Q', 'one')")
	has "$out" "thinkthen usage: the profile JSON"
	same "$(bcount)" 0
	fresh generic
	out=$(q -c 'SET thinkthen.profile = '\''{"schema":"thinkthen.backend-profile/1","name":"small","max_evidence_bytes":4}'\''' \
		-c "SELECT thinkthen_decide('$Q', 'a red door')")
	has "$out" "thinkthen usage: profile small allows at most 4 evidence bytes"
	same "$(bcount)" 0
	fresh generic
	out=$(q -c "SET thinkthen.cache = 'off'" -c "SELECT thinkthen_decide('$Q', 'same')" \
		-c "SELECT thinkthen_decide('$Q', 'same')")
	same "$out" $'t\nt'
	same "$(bcount)" 2
	fresh generic
	out=$(q -c "SET thinkthen.cache = 'off'" -c "SET thinkthen.record = '$RUN/saved'" \
		-c "SELECT thinkthen_decide('$Q', 'saved')" -c "SET thinkthen.record = ''" \
		-c "SET thinkthen.replay = '$RUN/saved'" -c "SELECT thinkthen_decide('$Q', 'saved')" \
		-c "SELECT thinkthen_decide('$Q', 'missing')")
	has "$out" "thinkthen local: the replay folder holds no answer for this question"
	same "$(bcount)" 1
	q -c "CREATE ROLE tt_setting LOGIN" -c "GRANT EXECUTE ON FUNCTION thinkthen_usage() TO tt_setting" >/dev/null
	has "$(PGUSER_AS=tt_setting q -c "SELECT count(*) FROM thinkthen_usage()" -c "SET thinkthen.record = '$RUN/other'")" \
		'permission denied to set parameter "thinkthen.record"'
}
check sql_settings_and_retry_total
shared_settings_cases() {
	python3 tests/settings_cases.py plan >"$RUN/settings.plan"
	while IFS=$'\t' read -r id arm expected; do
		fresh "$arm"
		line=$(python3 tests/settings_cases.py "$SOCK" "$id" "$RUN/settings-$id")
		same "$line" "pass $id"
		actual=$(bcount)
		[ "$actual" = "$expected" ] || { echo "$id: wanted $expected sends, got $actual" >&2; return 1; }
	done <"$RUN/settings.plan"
}
check shared_settings_cases
calibration_saved_profile() {
	fresh generic
	line=$(python3 tests/settings_cases.py calibration "$SOCK")
	same "$line" 'pass calibration'
	same "$(bcount)" 1
}
check calibration_saved_profile
a_cancelled_send_counts_toward_the_total() {
	fresh arm/held "thinkthen.max_requests_total = 1"
	q -c "SELECT thinkthen_decide('$Q', 'held')" -c "SELECT thinkthen_decide('$Q', 'next')" >"$RUN/held.out" 2>&1 &
	HELD=$!
	bwait 1
	q -c "SELECT pg_cancel_backend($(victim))" >/dev/null
	sleep 0.3
	brelease
	wait "$HELD" || true
	has "$(cat "$RUN/held.out")" "thinkthen usage: thinkthen.max_requests_total allows 1 requests in this backend, and they are spent"
	same "$(bcount)" 1
}
check a_cancelled_send_counts_toward_the_total
echo "== secrecy, signals, preload, panics"
the_key_never_reaches_the_log() {
	secret=tt-secret-value-4417
	fresh generic
	q -c "CREATE ROLE tt_plain LOGIN" -c "GRANT EXECUTE ON FUNCTION thinkthen_decide(text,text,json,text,text,text,text,bigint), thinkthen_usage() TO tt_plain" >/dev/null
	out=""
	for role in tt_plain postgres; do
		out+=$(PGUSER_AS=$role q -c "SET thinkthen.api_key = '$secret'" -c "SELECT thinkthen_decide('$Q', 'a')")
		out+=$(PGUSER_AS=$role q -c "SELECT count(*) FROM thinkthen_usage()" -c "SET thinkthen.api_key = '$secret'" -c "SELECT thinkthen_decide('$Q', 'a')")
	done
	out+=$(PGOPTIONS="-c thinkthen.api_key=$secret" PGUSER_AS=tt_plain q -c "SELECT thinkthen_decide('$Q', 'a')")
	has "$out" "thinkthen usage: thinkthen.api_key is not read; unset it and set THINKTHEN_API_KEY in the server's environment"
	same "$(grep -oF "WARNING:  thinkthen.api_key is never read; unset it and set THINKTHEN_API_KEY in the server's environment" <<<"$out" | wc -l)" 4
	same "$(grep -oF "thinkthen usage: thinkthen.api_key is not read; unset it and set THINKTHEN_API_KEY in the server's environment" <<<"$out" | wc -l)" 5
	hasnt "$out" "$secret"
	hasnt "$(cat "$LOG")" "$secret"
	same "$(bcount)" 0
}
check the_key_never_reaches_the_log
# Every thread but the backend's main one blocks SIGINT, SIGTERM, SIGALRM, and SIGUSR1.
masks_hold() {
	pid=$(victim)
	threads=0
	for task in /proc/"$pid"/task/*; do
		[ "${task##*/}" != "$pid" ] || continue
		blocked=$((16#$(awk '/^SigBlk/ {print $2}' "$task/status")))
		for signal in 2 15 14 10; do [ $(((blocked >> (signal - 1)) & 1)) = 1 ] || return 1; done
		threads=$((threads + 1))
	done
	[ "$threads" -ge 1 ]
}
signal_masks() {
	fresh arm/held
	held "SELECT thinkthen_decide('$Q', 'held')"
	bwait 1
	masks_hold
	brelease
	wait "$HELD"
	fresh arm/held "thinkthen.throttle = 8"
	held "SET thinkthen.batch = '2'; SELECT count(*) FROM thinkthen_decide_many('$Q', $(rows 64))"
	bwait 8
	masks_hold
	brelease
	wait "$HELD"
}
[ "$PG_HOST" != Linux ] || check signal_masks
preload_forks_cleanly() {
	fresh generic "shared_preload_libraries = 'thinkthen'"
	same "$(q -c "SELECT thinkthen_decide('$Q', 'first')")" t
	same "$(q -c "SELECT thinkthen_decide('$Q', 'second')")" t
	same "$(bcount)" 2
}
check preload_forks_cleanly
a_panic_is_an_error() {
	fresh generic
	out=$(q -c '\set VERBOSITY verbose' -c "SELECT thinkthen_panic_probe()" -c "SELECT 1")
	has "$out" "XX000"
	has "$out" "ERROR:  XX000: thinkthen defect: the extension panicked (retryable: no)"
	hasnt "$out" "panic probe"
	log=$(cat "$LOG")
	has "$log" "thinkthen defect: the extension panicked (retryable: no)"
	hasnt "$log" "panic probe"
	same "$(tail -n1 <<<"$out")" 1
}
check a_panic_is_an_error

find_inputs() {
	fresh generic
	python3 tests/find_cases.py invalid "$SOCK"
	same "$(bcount)" 0
}
check find_inputs
find_proxy_cases() {
	for mode in duplicate real_tie none_tie max_units max_none; do
		fresh generic
		pg_stop
		rm -f "$RUN/find-proxy.in"
		mkfifo "$RUN/find-proxy.in"
		python3 tests/find_cases.py proxy "http://127.0.0.1:$BPORT/generic/v1" "$mode" \
			<"$RUN/find-proxy.in" >"$RUN/find-proxy.out" 2>"$RUN/find-proxy.err" &
		PROXYPID=$!
		exec {PROXYFD}>"$RUN/find-proxy.in"
		trap 'if [ -n "${PROXYPID:-}" ]; then printf "quit\n" >&"$PROXYFD"; wait "$PROXYPID" || true; fi' EXIT
		for _ in $(seq 100); do [ -s "$RUN/find-proxy.out" ] && break; sleep 0.05; done
		proxyport=$(head -1 "$RUN/find-proxy.out")
		[ -n "$proxyport" ]
		pg_start "http://127.0.0.1:$proxyport/v1" "$CACHEDIR"
		python3 tests/find_cases.py verify "$SOCK" "$mode"
		printf 'quit\n' >&"$PROXYFD"
		wait "$PROXYPID"
		PROXYPID=
		exec {PROXYFD}>&-
		same "$(tail -1 "$RUN/find-proxy.out")" 1
	done
}
check find_proxy_cases

echo "== the update path"
an_update_cannot_grant_public() {
	printf 'CREATE FUNCTION thinkthen_rehearsal_probe(x integer) RETURNS integer LANGUAGE sql AS %s;\n' "'SELECT \$1'" \
		>"$RUNTIME_EXTENSION_DIR/thinkthen--$EXT_VERSION--$EXT_VERSION-probe.sql"
	fresh generic
	q -c "ALTER EXTENSION thinkthen UPDATE TO '$EXT_VERSION-probe'" >/dev/null
	same "$(q -c "SELECT has_function_privilege('public', 'thinkthen_rehearsal_probe(integer)', 'EXECUTE')")" f
	same "$(extension_owned "AND has_function_privilege('public', p.oid, 'EXECUTE')")" 0
}
check an_update_cannot_grant_public

echo "== conformance"
# Each case on its own server, backend, and cache folder. Every case counts
# as pass, fail, or not run, and the three sum to the selected count.
conformance() {
	pass=0 fail=0 skipped=0
	: >"$SCRATCH/not-a-folder"
	plan_file=$RUN/conformance.plan
	python3 tests/runner.py plan >"$plan_file"
	selected=$(wc -l <"$plan_file")
	while IFS=$'\t' read -r -u 3 id arm setting; do
		if [ "$arm" != - ]; then
			fresh "$arm" ${setting:+"${setting//@SCRATCH@/$SCRATCH}"}
		fi
		line=$(BPORT=${BPORT:-} SCRATCH=$SCRATCH python3 tests/runner.py "$SOCK" "$id")
		if [ "$line" = "pass $id" ] && { [ "$id" = 18-find-second ] || [ "$id" = 19-find-none ]; }; then
			bcapture >"$RUN/$id.capture.json"
			line=$(BPORT=$BPORT python3 tests/runner.py capture "$id" "$RUN/$id.capture.json")
		fi
		echo "         $line"
		case $line in
		"pass $id")
			case $id in
			13-filter-records|15-rank-records|16-rank-stable-tie) same "$(bcount)" 3 ;;
			14-filter-none) same "$(bcount)" 2 ;;
			18-find-second|19-find-none) same "$(bcount)" 1 ;;
			26-filter-empty-list|31-usage-rank-blank-question) same "$(bcount)" 0 ;;
			esac
			pass=$((pass + 1)) ;;
		"not run $id: "*) skipped=$((skipped + 1)) ;;
		*) fail=$((fail + 1)) ;;
		esac
	done 3< "$plan_file"
	echo "         conformance: total=54 selected=$selected pass=$pass fail=$fail not_run=$skipped unselected=$((54 - selected))"
	same "$((pass + fail + skipped))" "$selected"
	same "$fail" 0
}
check conformance
runner_never_excuses_by_a_note() {
	fresh case/01-decide-yes-captured
	out=$(env -u THINKTHEN_CONFORMANCE_IDS BPORT=$BPORT python3 tests/runner.py --cases fixtures/runner-excuse.json "$SOCK" 01-decide-yes-captured)
	same "$out" "fail 01-decide-yes-captured: probability: got 0.99, expected 0.5"
}
check runner_never_excuses_by_a_note

# The fake key never reaches a psql output or the server log.
the_fake_key_stays_in_the_environment() {
	same "$(cat "$RUN/psql.all" "$RUN/server.all" "$LOG" 2>/dev/null | grep -c "$FAKE_KEY" || true)" 0
	[ -s "$RUN/psql.all" ]
	[ -s "$RUN/server.all" ]
}
check the_fake_key_stays_in_the_environment

echo "postgresql: $PASSED passed, $FAILED failed"
[ "$PASSED" -gt 0 ] || { echo 'postgresql: no selected step ran' >&2; exit 2; }
[ "$FAILED" = 0 ]
