#!/usr/bin/env bash
# The PostgreSQL surface's check (ticket 0111). It builds the
# extension against pinned PostgreSQL 16.15 headers and runs a local server from
# the pinned package as this user on a socket with no TCP port, and restarts
# it for each test with that test's own loopback backend (ticket 0117) and
# answer cache. No test reaches a paid backend: the server gets a fake key
# beside a 127.0.0.1 address alone. Exit 0 passes, 77 is "not run".
# The surfaces rung runs `sh check.sh`, and the steps need bash 5 outside POSIX mode:
# EPOCHREALTIME, `exec {fd}>` and process substitution. macOS's sh is bash 3.2 in
# POSIX mode, so a set BASH_VERSION is not enough (ticket 0375).
if [ "${BASH_VERSINFO:-0}" -lt 5 ] || shopt -qo posix 2>/dev/null; then
	if [ -n "${THINKTHEN_PG_CHECK_REEXEC:-}" ]; then
		echo "not run: databases/postgresql/check.sh needs bash 5 on PATH outside POSIX mode, found ${BASH_VERSION:-no bash}"
		exit 77
	fi
	unset POSIXLY_CORRECT
	THINKTHEN_PG_CHECK_REEXEC=1 exec bash "$0" "$@"
fi
unset THINKTHEN_PG_CHECK_REEXEC
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
cargo build --locked --offline --quiet --bin thinkthen_read_inputs
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
	# Load and wall-clock limits run only under the stress profile. A step
	# named STEP_within_N_ms reruns STEP and holds its recorded time to N ms.
	case $1 in twenty_thousand_keyed_rows|*_within_*_ms) stress_only=yes ;; *) stress_only=no ;; esac
	if [ "$profile" = stress ]; then
		[ "$stress_only" = yes ] || return 0
	else
		[ "$stress_only" = no ] || return 0
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
# A routine step records its elapsed time here; its stress twin checks it.
# Issue: sdlc/issues/closed/2026-09-30-postgresql-check-keeps-wall-clock-limits-under-load.md
took() { echo "$2" >"$RUN/$1.ms"; }
took_within() { "$1"; within "$(cat "$RUN/$1.ms")" "$2"; }
Q='{"decide":"Is this a complaint?"}'
rows() { echo "(SELECT jsonb_object_agg(g::text, 'record ' || g ORDER BY g) FROM generate_series(1, $1) g)"; }
# A statement's elapsed milliseconds, read inside the server.
timed() {
	q -c "DO \$t\$ DECLARE s timestamptz := clock_timestamp(); n bigint; BEGIN $1; RAISE NOTICE 'elapsed %', round(extract(epoch FROM clock_timestamp() - s) * 1000); END \$t\$;" |
		sed -n 's/.*elapsed \([0-9]*\).*/\1/p'
}
# proxy_start NAME COMMAND...: a loopback proxy that prints its port, then reads `quit`
# from a FIFO and prints its count. Its port lands in PROXYPORT. A proxy that prints no
# port in 5 s fails the step with its error output, never silently (ticket 0386).
proxy_start() {
	PROXYNAME=$1
	shift
	rm -f "$RUN/$PROXYNAME.in" && mkfifo "$RUN/$PROXYNAME.in"
	"$@" <"$RUN/$PROXYNAME.in" >"$RUN/$PROXYNAME.out" 2>"$RUN/$PROXYNAME.err" &
	PROXYPID=$!
	exec {PROXYFD}>"$RUN/$PROXYNAME.in"
	trap 'if [ -n "${PROXYPID:-}" ]; then printf "quit\n" >&"$PROXYFD"; wait "$PROXYPID" || true; fi' EXIT
	for _ in $(seq 100); do [ -s "$RUN/$PROXYNAME.out" ] && break; sleep 0.05; done
	PROXYPORT=$(head -1 "$RUN/$PROXYNAME.out")
	[ -n "$PROXYPORT" ] || { echo "the $PROXYNAME printed no port in 5 s" >&2; cat "$RUN/$PROXYNAME.err" >&2; return 1; }
}
# proxy_stop: send `quit`, wait, and put the proxy's count in PROXYCOUNT. It sets
# variables, so a step calls it directly, never inside $(...).
proxy_stop() {
	printf 'quit\n' >&"$PROXYFD"
	wait "$PROXYPID" || { PROXYPID=; echo "the $PROXYNAME failed" >&2; cat "$RUN/$PROXYNAME.err" >&2; return 1; }
	PROXYPID=
	exec {PROXYFD}>&-
	PROXYCOUNT=$(tail -1 "$RUN/$PROXYNAME.out")
}
# The pid of the backend running a thinkthen call.
victim() { q -c "SELECT pid FROM pg_stat_activity WHERE query LIKE '%thinkthen_%' AND pid <> pg_backend_pid() LIMIT 1"; }

echo "== package"
[ -z "${THINKTHEN_ARTIFACT:-}" ] || {
	# The installed-file mode (ticket 0128): the server loads the files unpacked from the
	# release archive, and runs the drawn SQL, the examples, and the shared cases.
	mkdir "$RUN/artifact" && tar -xzf "$THINKTHEN_ARTIFACT" -C "$RUN/artifact"
	runtime_install "$RUN/artifact/lib" "$RUN/artifact/extension"
	STEPS=${STEPS:-examples slide_sample plain_question_contract portable_batch_identity recognize_and_relate_as_drawn complete_question_resolution_keeps_privilege_and_content_boundaries client_reader_validates_file_formats complete_cases conformance find_inputs find_proxy_cases find_cancel find_signatures_are_owned_and_private the_fake_key_stays_in_the_environment token_variable_refuses_before_sending}
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
sys.path.insert(0, "../sqlite/tests")
from question_keys import question_keys
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
assert p["upper_bound"] is False and p["first_body_utf8"] == expected, p
text = "{\"records\": 1, \"requests\": 1, \"upper_bound\": false, \"estimated_bytes\": 182, \"first_body_utf8\": " + json.dumps(expected) + ", \"estimated_input_tokens\": {\"lower\": 93, \"upper\": 166}}"
assert sys.argv[1] == text, (sys.argv[1], text)' "$out"
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
# Ticket 0378: one keyed object ranks best first, with refusals before any send.
R='Is it a refund?'
rank_exact_ties_and_types() {
	fresh case/15-rank-records
	same "$(q -c "SELECT string_agg(key || ':' || rank || ':' || probability, ' ' ORDER BY rank) FROM thinkthen_rank('How relevant is this?', '{\"0\":\"item 0\",\"1\":\"item 1\",\"2\":\"item 2\"}'::jsonb, '{\"batch\":1}'::json)")" '1:1:0.9 2:2:0.5 0:3:0.2'
	same "$(bcount)" 3
	same "$(q -c "SELECT pg_typeof(key)::text || ',' || pg_typeof(rank)::text || ',' || pg_typeof(probability)::text FROM thinkthen_rank('How relevant is this?', '{\"0\":\"item 0\"}'::jsonb, '{\"batch\":1}'::json)")" 'text,bigint,double precision'
	fresh generic "thinkthen.batch = 'max'"
	# jsonb hands keys over in bytewise order, so ties follow it, not member order or key length.
	same "$(q -c "SELECT string_agg(key || ':' || rank, ' ' ORDER BY rank) FROM thinkthen_rank('$R', '{\"z\":\"one\",\"b\":\"two\",\"aa\":\"three\"}'::jsonb)")" 'aa:1 b:2 z:3'
	same "$(bcount)" 1
}
check rank_exact_ties_and_types
rank_batches_by_the_setting() {
	local pair
	# The check sets THINKTHEN_BATCH=1, so `max` stands in for the default here.
	for pair in max:1 3:3 1:7; do
		fresh generic "thinkthen.batch = '${pair%:*}'"
		same "$(q -c "SELECT count(*) || ':' || max(rank) FROM thinkthen_rank('$R', $(rows 7))")" 7:7
		same "$(bcount)" "${pair#*:}"
	done
	fresh generic
	same "$(q -c "SELECT count(*) FROM thinkthen_rank('$R', $(rows 7), '{\"batch\":3}'::json)")" 7
	same "$(bcount)" 3
}
check rank_batches_by_the_setting
rank_question_is_literal_and_settings_reach_the_body() {
	fresh arm/full/capture "thinkthen.batch = 'max'"
	same "$(q -c "SELECT key || ':' || rank FROM thinkthen_rank('@nofile', '{\"a\":\"refund now\"}'::jsonb)")" 'a:1'
	same "$(q -c "SELECT string_agg(key, ' ' ORDER BY rank) FROM thinkthen_rank('$R', '{\"a\":\"one\",\"b\":\"two\"}'::jsonb, '{\"model\":\"judge-b\",\"context\":\"shared note\"}'::json)")" 'a b'
	same "$(bcount)" 2
	bcapture | python3 -c 'import json,sys
bodies=[json.loads(body) for body in json.load(sys.stdin)["bodies"]]
assert len(bodies) == 2, bodies
assert bodies[0]["questions"]["q1"]["instructions"] == "The text is \"refund now\". @nofile", bodies[0]
assert bodies[1]["model"] == "judge-b" and json.dumps(bodies[1]).count("shared note") == 1, bodies[1]'
}
check rank_question_is_literal_and_settings_reach_the_body
rank_empty_null_and_refusals_send_nothing() {
	fresh generic "thinkthen.batch = 'max'"
	same "$(q -c "SELECT count(*) FROM thinkthen_rank('$R', '{}'::jsonb)")" 0
	same "$(q -c "SELECT count(*) FROM thinkthen_rank('$R', NULL)")" 0
	has "$(q -c "SELECT count(*) FROM thinkthen_rank(NULL, '{\"a\":\"b\"}'::jsonb)")" 'thinkthen usage: the question is empty (retryable: no)'
	local name value
	for name in threshold:0.7 'true:"yes"' none:true 'options:["a","b"]'; do
		value=${name#*:} name=${name%%:*}
		has "$(q -c "SELECT count(*) FROM thinkthen_rank('$R', '{\"a\":\"x\"}'::jsonb, '{\"$name\":$value}'::json)")" \
			"thinkthen usage: the settings key \`$name\` does not belong to this verb (retryable: no)"
	done
	has "$(q -c "SELECT count(*) FROM thinkthen_rank('   ', '{\"a\":\"x\"}'::jsonb)")" 'thinkthen usage: a question is text, not white space (retryable: no)'
	has "$(q -c "SELECT count(*) FROM thinkthen_rank('$R', '{\"a\":4}'::jsonb)")" 'thinkthen usage: keyed input value for a is text (retryable: no)'
	has "$(q -c "SELECT count(*) FROM thinkthen_rank('$R', '{\"1\":\"one\",\"2\":\"two\",\"3\":\"three\",\"4\":\"  \",\"5\":\"five\",\"6\":\"six\",\"7\":\"seven\"}'::jsonb)")" \
		'thinkthen usage: evidence is text, not white space (retryable: no)'
	has "$(q -c "SELECT count(*) FROM thinkthen_rank('$R', '{\"a\":\"x\"}'::jsonb, '{\"deadline_ms\":0}'::json)")" \
		'thinkthen deadline: the deadline of 0 s passed before the call answered (retryable: no)'
	same "$(bcount)" 0
}
check rank_empty_null_and_refusals_send_nothing
rank_sets_turns_keys_descriptions_and_refusals() {
	fresh generic
	proxy_start ranksets python3 ../sqlite/tests/rank_backend.py "$RUN/ranksets.bodies"
	pg_stop
	pg_start "http://127.0.0.1:$PROXYPORT/v1" "$CACHEDIR"
	python3 tests/rank_sets_cases.py invalid "$SOCK"
	proxy_stop
	same "$PROXYCOUNT" 0
	proxy_start ranksetsvalid python3 ../sqlite/tests/rank_backend.py "$RUN/ranksets.bodies"
	pg_stop
	pg_start "http://127.0.0.1:$PROXYPORT/v1" "$CACHEDIR"
	python3 tests/rank_sets_cases.py verify "$SOCK" "$RUN"
	proxy_stop
	same "$PROXYCOUNT" 6
	python3 tests/rank_sets_cases.py bodies "$RUN/ranksets.bodies"
}
check rank_sets_turns_keys_descriptions_and_refusals
rank_sets_plain_member_recordings_replay_without_sends() {
	fresh generic
	proxy_start rankreplay python3 ../sqlite/tests/rank_backend.py "$RUN/rankreplay.bodies"
	pg_stop
	pg_start "http://127.0.0.1:$PROXYPORT/v1" "$CACHEDIR"
	python3 tests/rank_sets_cases.py members "$SOCK" "$RUN/rank-record"
	pg_stop
	mkdir "$RUN/rank-replay-cache"
	pg_start "http://127.0.0.1:$PROXYPORT/v1" "$RUN/rank-replay-cache"
	python3 tests/rank_sets_cases.py replay "$SOCK" "$RUN/rank-record"
	proxy_stop
	same "$PROXYCOUNT" 6
}
check rank_sets_plain_member_recordings_replay_without_sends
rank_set_ties_keep_key_order_and_public_types() {
	fresh generic
	local spec='{"version":1,"questions":{"first":{"decide":"First?"},"second":{"decide":"Second?"}}}'
	local input='{"z":"one","b":"two","aa":"three"}'
	same "$(q -c "SELECT string_agg(key || ':' || rank || ':' || probability || ':' || question_name, ' ' ORDER BY rank) FROM thinkthen_rank_set('$spec','$input'::jsonb,'{\"batch\":\"max\"}'::json)")" 'aa:1:0.9:first b:2:0.9:first z:3:0.9:first'
	same "$(q -c "SELECT pg_typeof(key)::text || ',' || pg_typeof(rank)::text || ',' || pg_typeof(probability)::text || ',' || pg_typeof(question_name)::text || ',' || pg_typeof(facts)::text FROM thinkthen_rank_set('$spec','$input'::jsonb,'{\"batch\":\"max\"}'::json) LIMIT 1")" 'text,bigint,double precision,text,jsonb'
	same "$(bcount)" 1
}
check rank_set_ties_keep_key_order_and_public_types
rank_set_backend_failure_preserves_error_and_secrecy() {
	fresh generic
	proxy_start rankfailure python3 ../sqlite/tests/conditional_backend.py "http://127.0.0.1:$BPORT/generic/v1" 'private evidence'
	pg_stop
	pg_start "http://127.0.0.1:$PROXYPORT/v1" "$CACHEDIR"
	python3 tests/rank_sets_cases.py failure "$SOCK"
	proxy_stop
	same "$PROXYCOUNT" 1
}
check rank_set_backend_failure_preserves_error_and_secrecy
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
# ADR 0111 slice 3: a record missing its `on` part fails alone, and its
# neighbours' answers stay in the question store for a rerun.
an_annotate_row_missing_its_part_fails_alone() {
	fresh generic
	parted='{"version":1,"questions":{"refund":{"decide":"Is it a refund?","on":"/body"}}}'
	rows="(1, '{\"body\":\"one\"}'), (2, '{\"body\":\"two\"}')"
	out=$(q -c "SELECT thinkthen_annotate('$parted', x)::text FROM (VALUES $rows, (3, '{\"note\":\"three\"}')) t(i, x) ORDER BY i")
	has "$out" 'thinkthen usage: the record holds nothing at `/body` (retryable: no)'
	same "$(bcount)" 2
	same "$(q -c "SELECT thinkthen_annotate('$parted', x)::text FROM (VALUES $rows) t(i, x) ORDER BY i")" "$(printf '{"refund": true}\n{"refund": true}')"
	same "$(bcount)" 2
}
check an_annotate_row_missing_its_part_fails_alone
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
	same "$(q -c "SELECT count(*) FROM pg_proc p JOIN pg_depend d ON d.objid = p.oid AND d.classid = 'pg_proc'::regclass JOIN pg_extension e ON e.oid = d.refobjid AND d.refclassid = 'pg_extension'::regclass WHERE e.extname = 'thinkthen' AND p.oid::regprocedure::text = ANY (ARRAY['thinkthen_decide_many(text,jsonb,json)', 'thinkthen_choose_many(text,jsonb,json)', 'thinkthen_score_many(text,jsonb,json)', 'thinkthen_tag_many(text,jsonb,json)', 'thinkthen_plan(text,jsonb,json)', 'thinkthen_rank(text,jsonb,json)']) AND p.pronargdefaults = 1 AND p.prokind = 'f' AND p.proparallel = 'r' AND NOT has_function_privilege('public', p.oid, 'EXECUTE')")" 6
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
explicit_question_loader_keeps_authored_json_and_privilege_gate() {
	fresh generic
	mkdir -p "$RUN/question-loader"
	printf ' {"decide":"Is red visible?"}\n' >"$RUN/question-loader/question.json"
	printf '{"version":1,"questions":{"second":{"decide":"Second?"},"first":{"decide":"First?"}}}\n' >"$RUN/question-loader/set.json"
	printf '{"decide":"one","decide":"two"}\n' >"$RUN/question-loader/duplicate.json"
	printf '{"version":1,"questions":{"first":{"decide":"First?"},"first":{"decide":"Second?"}}}\n' >"$RUN/question-loader/duplicate-set.json"
	printf '{"decide":"private-loader-evidence"' >"$RUN/question-loader/broken.json"
	printf '{"decide":"literal at filename?"}' >"$RUN/question-loader/@literal"
	q -c 'CREATE ROLE tt_loader LOGIN' -c 'GRANT EXECUTE ON FUNCTION thinkthen_question_file(text) TO tt_loader' >/dev/null
	same "$(q -c "SELECT has_function_privilege('public','thinkthen_question_file(text)','EXECUTE')")" f
	same "$(q -c 'SELECT thinkthen_question_file(NULL) IS NULL')" t
	has "$(q -c "SELECT thinkthen_question_file('')")" 'thinkthen usage: question file path must be nonempty text without NUL (retryable: no)'
	same "$(q -c "SELECT thinkthen_question_file('$RUN/question-loader/question.json') = pg_read_file('$RUN/question-loader/question.json')")" t
	same "$(q -c "SELECT thinkthen_question_file('$RUN/question-loader/set.json') = pg_read_file('$RUN/question-loader/set.json')")" t
	has "$(PGUSER_AS=tt_loader q -c "SELECT thinkthen_question_file('$RUN/question-loader/question.json')")" 'a named file needs pg_read_server_files'
	fresh generic "thinkthen.file_directory = '$RUN/question-loader'"
	same "$(PGUSER_AS=tt_loader q -c "SELECT thinkthen_question_file('question.json') = ' {\"decide\":\"Is red visible?\"}' || chr(10)")" t
	same "$(PGUSER_AS=tt_loader q -c "SELECT thinkthen_question_file('@literal')")" '{"decide":"literal at filename?"}'
	ln -s question.json "$RUN/question-loader/symlink.json"
	ln "$RUN/question-loader/set.json" "$RUN/question-loader/hardlink.json"
	mkfifo "$RUN/question-loader/fifo"
	for name in symlink.json hardlink.json fifo . ../postgresql.conf.base; do
		has "$(PGUSER_AS=tt_loader q -c "SELECT thinkthen_question_file('$name')")" 'thinkthen local:'
	done
	for name in duplicate.json duplicate-set.json broken.json; do
		has "$(PGUSER_AS=tt_loader q -c "SELECT thinkthen_question_file('$name')")" 'the question file does not parse as a question or question set'
	done
	python3 -c 'import pathlib,sys; pathlib.Path(sys.argv[1]).write_bytes(b" " * 1048577)' "$RUN/question-loader/big.json"
	has "$(PGUSER_AS=tt_loader q -c "SELECT thinkthen_question_file('big.json')")" 'over the 1048576 byte cap'
	same "$(bcount)" 0
}
check explicit_question_loader_keeps_authored_json_and_privilege_gate

stored_image_composites_use_native_order_and_strict_replay() {
	export LIQUIDAI_API_KEY=sk-sql-image-loopback THINKTHEN_BACKEND=liquid
	fresh generic
	proxy_start images python3 ../sqlite/tests/image_backend.py "$RUN/images.bodies"
	pg_stop
	pg_start "http://127.0.0.1:$PROXYPORT/v1" "$CACHEDIR"
	python3 tests/image_cases.py invalid "$SOCK"
	# Recording is a server setting; the first public consumer stores originals
	# in a persisted SQL table and reads its array in ordinal order.
	q -c "ALTER SYSTEM SET thinkthen.record = '$RUN/image-record'" >/dev/null
	pg_stop
	pg_start "http://127.0.0.1:$PROXYPORT/v1" "$CACHEDIR"
	python3 tests/image_cases.py verify "$SOCK"
	q -c "ALTER SYSTEM RESET thinkthen.record" -c "ALTER SYSTEM SET thinkthen.replay = '$RUN/image-record'" -c 'ALTER SYSTEM SET thinkthen.max_requests_total = 0' >/dev/null
	pg_stop
	pg_start "http://127.0.0.1:$PROXYPORT/v1" "$CACHEDIR"
	python3 tests/image_cases.py replay "$SOCK"
	proxy_stop
	same "$PROXYCOUNT" 3
	python3 tests/image_cases.py bodies "$RUN/images.bodies"
	q -c 'ALTER SYSTEM RESET thinkthen.replay' -c 'ALTER SYSTEM RESET thinkthen.max_requests_total' >/dev/null
}
check stored_image_composites_use_native_order_and_strict_replay
client_reader_keeps_keys_find_indexes_native_spans_and_relation_ends() {
	fresh generic
	cargo build --locked --offline --example client_files
	"${CARGO_TARGET_DIR:-target}/debug/examples/client_files" "$SOCK" "$RUN/client-source.txt"
	# Two rank records, one find, two native recognition stages per source,
	# and one relation request; readers and span mapping add no sends.
	same "$(bcount)" 8
}
check client_reader_keeps_keys_find_indexes_native_spans_and_relation_ends
DID_NOT_READ="did not read: it must be a regular file at most 1048576 bytes"
dev_zero_refuses_fast() {
	fresh generic
	start=$(now_ms)
	out=$(q -c "SELECT thinkthen_decide('@/dev/zero', 'x')" -c "SELECT 1")
	took dev_zero_refuses_fast $(($(now_ms) - start))
	has "$out" "thinkthen local: the question file '@/dev/zero' $DID_NOT_READ"
	same "$(tail -n1 <<<"$out")" 1
	head -c 1048577 /dev/zero | tr '\0' ' ' >"$RUN/big.json"
	has "$(q -c "SELECT thinkthen_decide('@$RUN/big.json', 'x')")" "the question file '@$RUN/big.json' is over the 1048576 byte cap"
}
check dev_zero_refuses_fast
dev_zero_refuses_fast_within_2000_ms() { took_within dev_zero_refuses_fast 2000; }
check dev_zero_refuses_fast_within_2000_ms
the_file_gate() {
	mkdir -p "$RUN/files"
	cp fixtures/refund.json "$RUN/files/"
	fresh generic
	q -c "CREATE ROLE tt_exec LOGIN" -c "GRANT EXECUTE ON FUNCTION thinkthen_decide(text,text,json,text,text,text,text,bigint) TO tt_exec" >/dev/null
	out=$(PGUSER_AS=tt_exec q -c "SELECT thinkthen_decide('@/etc/hostname', 'x')")
	has "$out" "a named file needs pg_read_server_files, or an administrator's thinkthen.file_directory"
	cp fixtures/refund.json "$RUN/anywhere.json"
	fresh generic "thinkthen.file_directory = '$RUN/files'"
	same "$(PGUSER_AS=tt_exec q -c "SELECT thinkthen_decide('@$RUN/files/refund.json', 'I want a refund')")" t
	# 0370: a relative name resolves inside the folder, and `..` still refuses.
	same "$(PGUSER_AS=tt_exec q -c "SELECT thinkthen_decide('@refund.json', 'I want a refund')")" t
	has "$(PGUSER_AS=tt_exec q -c "SELECT thinkthen_decide('@../anywhere.json', 'x')")" "'@../anywhere.json' $DID_NOT_READ"
	# A superuser's relative name resolves inside the folder too.
	cp fixtures/refund.json "$RUN/files/only-here.json"
	same "$(q -c "SELECT thinkthen_decide('@only-here.json', 'I want a refund')")" t
	has "$(PGUSER_AS=tt_exec q -c "SELECT thinkthen_decide('@/etc/hostname', 'x')")" "'@/etc/hostname' $DID_NOT_READ"
	q -c "GRANT pg_read_server_files TO tt_exec" >/dev/null
	same "$(PGUSER_AS=tt_exec q -c "SELECT thinkthen_decide('@$RUN/anywhere.json', 'I want a refund')")" t
}
check the_file_gate
complete_question_resolution_keeps_privilege_and_content_boundaries() {
	local catalog="$SCRATCH/.config/thinkthen/questions" allowed="$RUN/complete-questions" kind name
	mkdir -p "$catalog" "$allowed"
	trap "$(printf 'rm -f -- %q; rmdir -- %q' "$catalog/refund.json" "$catalog")" EXIT
	printf '{"decide":"Refund?"}' >"$catalog/refund.json"
	printf '{"decide":"Refund?"}' >"$allowed/refund.json"
	printf '{"score":"Strength?","levels":["low","medium","high"]}' >"$allowed/wrong.json"
	printf '{"decide":"one","decide":"two"}' >"$allowed/duplicate.json"
	printf '{"decide":"private-complete-evidence"' >"$allowed/broken.json"
	printf '{"decide":"Refund?","threshold":0.6}' >"$allowed/context.json"
	fresh generic
	q -c 'CREATE ROLE tt_complete LOGIN' -c 'GRANT EXECUTE ON FUNCTION thinkthen_decide_complete(text,text,text) TO tt_complete' >/dev/null
	same "$(q -c "SELECT count(*) FROM pg_proc WHERE proname ~ '^thinkthen_(decide|choose|tag|score|filter|rank|find|annotate|recognize|relate)_complete$' AND pronargdefaults = 1 AND proparallel = 'r' AND NOT has_function_privilege('public', oid, 'EXECUTE')")" 10
	# Metadata selection cannot disclose a missing/invalid catalog to an unauthorized role.
	for name in '@@bad/name' '@@absent' '@missing.json'; do
		has "$(PGUSER_AS=tt_complete q -c "SELECT thinkthen_decide_complete('$name','{\"records\":[]}')->'native'->'error'->>'message'")" "a named file needs pg_read_server_files"
	done
	same "$(q -c "SELECT thinkthen_decide_complete('@@refund','{\"records\":[]}')->'native'->'error' IS NOT NULL")" f
	same "$(q -c "SELECT thinkthen_decide_complete('@@bad/name','{\"records\":[]}')->'native'->'error'->>'kind'")" usage
	same "$(bcount)" 0
	fresh generic "thinkthen.file_directory = '$allowed'"
	same "$(PGUSER_AS=tt_complete q -c "SELECT thinkthen_decide_complete('@refund.json','{\"records\":[]}')->'native'->'error' IS NOT NULL")" f
	same "$(PGUSER_AS=tt_complete q -c "SELECT thinkthen_decide_complete('@@refund','{\"records\":[]}')->'native'->'error'->>'kind'")" local
	ln -s refund.json "$allowed/symlink.json"
	ln "$allowed/refund.json" "$allowed/hardlink.json"
	ln -s "$catalog" "$allowed/hop"
	mkfifo "$allowed/fifo"
	python3 -c 'import pathlib,sys; pathlib.Path(sys.argv[1]).write_bytes(b" " * 1048577)' "$allowed/big.json"
	for name in symlink.json hardlink.json hop/refund.json fifo big.json duplicate.json broken.json ../complete-questions/refund.json; do
		kind=$(PGUSER_AS=tt_complete q -c "SELECT thinkthen_decide_complete('@$name','{\"records\":[]}')->'native'->'error'->>'kind'")
		same "$kind" local
	done
	same "$(PGUSER_AS=tt_complete q -c "SELECT thinkthen_decide_complete('@wrong.json','{\"records\":[]}')->'native'->'error'->>'kind'")" usage
	same "$(PGUSER_AS=tt_complete q -c "SELECT thinkthen_decide_complete('@context.json','{\"records\":[]}','{\"threshold\":0.7}')->'native'->'error'->>'kind'")" usage
	same "$(PGUSER_AS=tt_complete q -c "SELECT thinkthen_decide_complete('@context.json','{\"records\":[]}','{\"context\":\" \"}')->'native'->'error'->>'kind'")" usage
	same "$(bcount)" 0
	q -c 'GRANT pg_read_server_files TO tt_complete' >/dev/null
	same "$(PGUSER_AS=tt_complete q -c "SELECT thinkthen_decide_complete('@@refund','{\"records\":[]}')->'native'->'error' IS NOT NULL")" f
	same "$(bcount)" 0
}
check complete_question_resolution_keeps_privilege_and_content_boundaries
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
# The routine checks prove order: the cancel, timeout or deadline ends the
# statement while the held arm still keeps its reply, since the release comes
# after the wait. Each time limit runs only under the stress profile.
single_cancel() {
	fresh arm/held
	held "SELECT thinkthen_decide('$Q', 'held')"
	bwait 1
	start=$(now_ms)
	q -c "SELECT pg_cancel_backend($(victim))" >/dev/null
	wait "$HELD" || true
	elapsed=$(($(now_ms) - start))
	has "$(cat "$RUN/held.out")" "canceling statement due to user request"
	brelease
	sleep 0.2
	same "$(bcount)" 1
	took single_cancel "$elapsed"
}
check single_cancel
single_cancel_within_200_ms() { took_within single_cancel 200; }
check single_cancel_within_200_ms
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
rank_set_cancel() {
	fresh arm/held
	held "SELECT key FROM thinkthen_rank_set('{\"version\":1,\"questions\":{\"first\":{\"decide\":\"First?\"},\"second\":{\"decide\":\"Second?\"}}}', '{\"a\":\"a\",\"b\":\"b\"}'::jsonb, '{\"batch\":\"max\"}'::json)"
	bwait 1
	q -c "SELECT pg_cancel_backend($(victim))" >/dev/null
	wait "$HELD" || true
	has "$(cat "$RUN/held.out")" 'canceling statement due to user request'
	brelease
	same "$(bcount)" 1
}
check rank_set_cancel
single_statement_timeout() {
	fresh arm/held
	start=$(now_ms)
	held "SET statement_timeout = '300ms'; SELECT thinkthen_decide('$Q', 'held')"
	wait "$HELD" || true
	took single_statement_timeout $(($(now_ms) - start))
	has "$(cat "$RUN/held.out")" "canceling statement due to statement timeout"
	brelease
	sleep 0.2
	same "$(bcount)" 1
}
check single_statement_timeout
single_statement_timeout_within_500_ms() { took_within single_statement_timeout 500; }
check single_statement_timeout_within_500_ms
single_deadline() {
	fresh arm/held
	start=$(now_ms)
	held "SET thinkthen.deadline_ms = 200; SELECT thinkthen_decide('$Q', 'held')"
	wait "$HELD" || true
	took single_deadline $(($(now_ms) - start))
	has "$(cat "$RUN/held.out")" "57014"
	has "$(cat "$RUN/held.out")" "thinkthen deadline"
	brelease
	sleep 0.2
	same "$(bcount)" 1
}
check single_deadline
single_deadline_within_1000_ms() { took_within single_deadline 1000; }
check single_deadline_within_1000_ms
batch_deadline() {
	fresh arm/held "thinkthen.throttle = 8"
	start=$(now_ms)
	held "SET thinkthen.batch = '2'; SET thinkthen.deadline_ms = 1000; SELECT count(*) FROM thinkthen_decide_many('$Q', $(rows 200))"
	wait "$HELD" || true
	took batch_deadline $(($(now_ms) - start))
	has "$(cat "$RUN/held.out")" "thinkthen deadline"
	same "$(bcount)" 8
	brelease
}
check batch_deadline
batch_deadline_within_1500_ms() { took_within batch_deadline 1500; }
check batch_deadline_within_1500_ms
batch_cancel() {
	fresh arm/held "thinkthen.throttle = 8"
	held "SET thinkthen.batch = '2'; SELECT count(*) FROM thinkthen_decide_many('$Q', $(rows 200))"
	bwait 8
	start=$(now_ms)
	q -c "SELECT pg_cancel_backend($(victim))" >/dev/null
	wait "$HELD" || true
	took batch_cancel $(($(now_ms) - start))
	has "$(cat "$RUN/held.out")" "canceling statement due to user request"
	sleep 0.3
	same "$(bcount)" 8
	brelease
	sleep 0.3
	same "$(bcount)" 8
}
check batch_cancel
batch_cancel_within_200_ms() { took_within batch_cancel 200; }
check batch_cancel_within_200_ms
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
	[ -n "$ms" ]
	same "$(bcount)" 2
	took a_small_batch_answers_at_once "$ms"
}
check a_small_batch_answers_at_once
a_small_batch_answers_at_once_within_90_ms() { took_within a_small_batch_answers_at_once 90; }
check a_small_batch_answers_at_once_within_90_ms

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
    has "$out" 'thinkthen_probability was removed; order records with thinkthen_rank'
    out=$(q -c "SELECT thinkthen_probability('$Q', 'one', 'old context')")
    has "$out" 'thinkthen_probability was removed; order records with thinkthen_rank'
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
	[ -z "$(find "$SCRATCH" -type f -not -path '*/.local/state/thinkthen/*' -not -path '*/Application Support/thinkthen/usage/*' | head -1)" ]
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
	[ -z "$(find "$SCRATCH" -type f -not -path '*/.local/state/thinkthen/*' -not -path '*/Application Support/thinkthen/usage/*' | head -1)" ]
}
check the_platform_cache_stays_off
# ADR 0113: each backend adds its sends to the server user's usage totals when
# it exits. The check holds the usage lock while both connections disconnect,
# so only the exit hook's wait writes the counts.
usage_total() {
	env -i PATH="$PATH" HOME="$SCRATCH" XDG_STATE_HOME="$SCRATCH/.local/state" "$COMMAND" status --json |
		python3 -c 'import json, sys; print(json.load(sys.stdin)["usage"]["total"][sys.argv[1]])' "$1"
}
usage_requests() { usage_total requests_sent; }
each_backend_adds_its_sends_at_exit() {
	fresh generic
	folder=$SCRATCH/.local/state/thinkthen
	[ "$PG_HOST" != Darwin ] || folder="$SCRATCH/Library/Application Support/thinkthen/usage"
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
# A second connection answers from the named cache, sends nothing, and its
# exit adds one cache answer.
a_cached_rerun_adds_a_cache_answer() {
	fresh generic
	before="$(usage_requests) $(usage_total cache_answers)"
	same "$(q -c "SELECT thinkthen_decide('$Q', 'cached rerun')")" t
	same "$(q -c "SELECT thinkthen_decide('$Q', 'cached rerun')")" t
	same "$(bcount)" 1
	wanted="$((${before% *} + 1)) $((${before#* } + 1))"
	for _ in $(seq 50); do [ "$(usage_requests) $(usage_total cache_answers)" = "$wanted" ] && break; sleep 0.1; done
	same "$(usage_requests) $(usage_total cache_answers)" "$wanted"
}
check a_cached_rerun_adds_a_cache_answer
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
omitted_and_explicit_throttles_hold_packed_requests() {
    for cap in 8 6; do
        if [ "$cap" = 8 ]; then fresh arm/held; else fresh arm/held "thinkthen.throttle = 6"; fi
        held "SET thinkthen.batch = '2'; SELECT count(*) FROM thinkthen_decide_many('$Q', $(rows 64))"
        bwait "$cap" || :
        sleep 0.3
        arrived=$(bcount)
        printf 'held throttle: expected %s, observed %s\n' "$cap" "$arrived"
        same "$arrived" "$cap"
        brelease
        wait "$HELD"
        same "$(cat "$RUN/held.out")" 64
        same "$(bcount)" 32
    done
}
check omitted_and_explicit_throttles_hold_packed_requests
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
    proxy_start proxy python3 ../sqlite/tests/conditional_backend.py "http://127.0.0.1:$BPORT/generic/v1" 'private evidence'
    pg_start "http://127.0.0.1:$PROXYPORT/v1" "$CACHEDIR"
    out=$(q -c "WITH rows(i,q,e) AS (VALUES (1, '$Q', 'first'), (2, '$Q', 'private evidence'), (3, '$Q', 'last')),
        measured AS MATERIALIZED (SELECT i, thinkthen_try_details(q,e) AS v FROM rows)
        SELECT i::text || ':' || (v->>'status') || ':' || coalesce(v->'error'->>'kind','ok') || ':' ||
            CASE WHEN v::text LIKE '%private evidence%' THEN 'leaked' ELSE 'safe' END FROM measured ORDER BY i")
    same "$out" "$(printf '1:answered:ok:safe\n2:failed:backend:safe\n3:answered:ok:safe')"
    same "$(bcount)" 2
    proxy_stop
    same "$PROXYCOUNT" 3
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
	same "$(($(grep -oF "thinkthen usage: thinkthen.max_requests_total allows 3 requests in this backend, and they are spent (retryable: no)" <<<"$out" | wc -l)))" 2
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
named_backend_settings() {
    export LIQUIDAI_API_KEY=fake-pg-liquid LLAMACPP_API_KEY=fake-pg-llamacpp MLX_API_KEY=fake-pg-mlx OLLAMA_API_KEY=fake-pg-ollama OPENROUTER_API_KEY=fake-pg-openrouter PERPLEXITY_API_KEY=fake-pg-perplexity TYPESAFE_API_KEY=fake-pg-typesafe
    export THINKTHEN_TEST_MARKERS='{"liquid":"fake-pg-liquid","llamacpp":"fake-pg-llamacpp","mlx":"fake-pg-mlx","ollama":"fake-pg-ollama","openrouter":"fake-pg-openrouter","perplexity":"fake-pg-perplexity","typesafe":"fake-pg-typesafe"}'
    fresh arm/full/capture
    python3 tests/named_backends.py "$SOCK" "$SCRATCH" "$SCRATCH/.config" "http://127.0.0.1:$BPORT/arm/full/capture/v1" "$RUN/b.out" "$RUN"
}
check named_backend_settings

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
	q -c "CREATE ROLE tt_plain LOGIN" -c "GRANT EXECUTE ON FUNCTION thinkthen_decide(text,text,json,text,text,text,text,bigint), thinkthen_rank(text,jsonb,json), thinkthen_usage() TO tt_plain" >/dev/null
	out=""
	for role in tt_plain postgres; do
		out+=$(PGUSER_AS=$role q -c "SET thinkthen.api_key = '$secret'" -c "SELECT thinkthen_decide('$Q', 'a')")
		out+=$(PGUSER_AS=$role q -c "SELECT count(*) FROM thinkthen_usage()" -c "SET thinkthen.api_key = '$secret'" -c "SELECT thinkthen_decide('$Q', 'a')")
	done
	out+=$(PGOPTIONS="-c thinkthen.api_key=$secret" PGUSER_AS=tt_plain q -c "SELECT thinkthen_decide('$Q', 'a')")
	out+=$(PGUSER_AS=tt_plain q -c "SET thinkthen.api_key = '$secret'" -c "SELECT count(*) FROM thinkthen_rank('$R', '{\"a\":\"x\"}'::jsonb)")
	has "$out" "thinkthen usage: thinkthen.api_key is not read; unset it and set THINKTHEN_API_KEY in the server's environment"
	same "$(($(grep -oF "WARNING:  thinkthen.api_key is never read; unset it and set THINKTHEN_API_KEY in the server's environment" <<<"$out" | wc -l)))" 5
	same "$(($(grep -oF "thinkthen usage: thinkthen.api_key is not read; unset it and set THINKTHEN_API_KEY in the server's environment" <<<"$out" | wc -l)))" 6
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
		proxy_start find-proxy python3 tests/find_cases.py proxy "http://127.0.0.1:$BPORT/generic/v1" "$mode"
		pg_start "http://127.0.0.1:$PROXYPORT/v1" "$CACHEDIR"
		python3 tests/find_cases.py verify "$SOCK" "$mode"
		proxy_stop
		same "$PROXYCOUNT" 1
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

client_reader_validates_file_formats() { python3 tests/read_inputs.py; }
check client_reader_validates_file_formats

complete_cases() {
    pg_stop
    [ -z "${BPID:-}" ] || backend_stop
    export THINKTHEN_POSTGRESQL_BIN=$BIN THINKTHEN_POSTGRESQL_DATA=$DATA THINKTHEN_POSTGRESQL_SOCKET=$SOCK
    sh "$LIMIT" 1800 python3 ../sqlite/tests/complete/parity.py postgresql
    sh "$LIMIT" 300 python3 ../sqlite/tests/complete/facts.py postgresql
}
check complete_cases

echo "== conformance"
# Each case on its own server, backend, and cache folder. Every case counts
# as pass, fail, or not run, and the three sum to the selected count.
conformance() {
	pass=0 fail=0 skipped=0
	: >"$SCRATCH/not-a-folder"
	plan_file=$RUN/conformance.plan
	python3 tests/runner.py plan >"$plan_file"
	# macOS wc pads its count with spaces; arithmetic drops them (ticket 0375).
	selected=$(($(wc -l <"$plan_file")))
	while IFS=$'\t' read -r -u 3 id arm setting; do
		if [ "$arm" != - ]; then
			fresh "$arm" ${setting:+"${setting//@SCRATCH@/$SCRATCH}"}
		fi
		line=$(BPORT=${BPORT:-} SCRATCH=$SCRATCH STORE=${CACHEDIR:-} python3 tests/runner.py "$SOCK" "$id")
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
	total=$(python3 -c 'import json, sys; print(json.load(open(sys.argv[1]))["case_count"])' ../../conformance/cases.json)
	echo "         conformance: total=$total selected=$selected pass=$pass fail=$fail not_run=$skipped unselected=$((total - selected))"
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
