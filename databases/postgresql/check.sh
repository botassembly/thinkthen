#!/usr/bin/env bash
# The PostgreSQL surface's check (ticket 0111). Linux only: it builds the
# extension against /usr/bin/pg_config, runs a local PostgreSQL 16.15 from
# the pinned package as this user on a socket with no TCP port, and restarts
# it for each test with that test's own loopback backend (ticket 0117) and
# answer cache. No test reaches a paid backend: the server gets a fake key
# beside a 127.0.0.1 address alone. Exit 0 passes, 77 is "not run".
# The surfaces rung runs `sh check.sh`, and the steps need bash.
[ -n "${BASH_VERSION:-}" ] || exec bash "$0" "$@"
set -euo pipefail
unset THINKTHEN_API_KEY RUSTFLAGS CARGO_ENCODED_RUSTFLAGS
cd "$(dirname "$0")"
profile=${THINKTHEN_TEST_PROFILE:-routine}
case $profile in routine|full|stress) ;; *) echo "postgresql: unknown THINKTHEN_TEST_PROFILE: $profile" >&2; exit 2 ;; esac
[ "$profile" = routine ] || unset THINKTHEN_CONFORMANCE_IDS
[ "$(uname -s)" = Linux ] || { echo "not run: check.sh runs on Linux only (uname: $(uname -s))"; exit 77; }
. ../../sdlc/scripts/scratch.sh
. ./runtime.sh
runtime_ready
REPO=$(cd ../.. && pwd)
# macOS has no `timeout` (ticket 0128). runtime.sh uses it too.
LIMIT=$REPO/sdlc/scripts/time-limit
# The extension version names its SQL files (ticket 0128).
EXT_VERSION=$(sed -n "s/^default_version = '\(.*\)'$/\1/p" thinkthen.control)
BACKEND=${CARGO_TARGET_DIR:-$REPO/target}/debug/conformance-backend

[ -n "${THINKTHEN_ARTIFACT:-}" ] || {
	echo "== build"
	cargo fmt --check
	cargo clippy --locked --offline --all-targets -- -D warnings
	cargo test --locked --offline --lib
}
(cd "$REPO" && cargo build --locked --offline --quiet --package conformance-backend)
export RUSTFLAGS="--remap-path-prefix=$HOME=/build"
# package.sh reads the same pg_config and target folder (ticket 0128).
PG_CONFIG=${PG_CONFIG:-/usr/bin/pg_config}
EXT=${CARGO_TARGET_DIR:-target}/release/thinkthen-pg16
# The shipped build, which `package.sh --reuse` packs (ticket 0128).
SHIPPED=$EXT-shipped

runtime_open
cleanup() {
	[ ! -f "$DATA/postmaster.pid" ] || "$BIN/pg_ctl" -D "$DATA" -m immediate stop >/dev/null 2>&1 || true
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
		[ "$1" = twenty_thousand_warm_rows ] || return 0
	else
		[ "$1" != twenty_thousand_warm_rows ] || return 0
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
rows() { echo "(SELECT array_agg('record ' || g) FROM generate_series(1, $1) g)"; }
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
	STEPS=${STEPS:-examples slide_sample recognize_and_relate_as_drawn conformance the_fake_key_stays_in_the_environment}
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
no_home_in_library() { same "$(grep -ac -- "$HOME" "$SHIPPED$("$PG_CONFIG" --pkglibdir)/thinkthen.so" || true)" 0; }
check no_home_in_library
no_catch_unwind() { same "$(grep -rc catch_unwind src | awk -F: '{s += $2} END {print s}')" 0; }
check no_catch_unwind
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
darwin_reports_not_run() {
	mkdir -p "$RUN/shim"
	printf '#!/bin/sh\necho Darwin\n' >"$RUN/shim/uname"
	chmod +x "$RUN/shim/uname"
	set +e
	# A missing toolchain folder stops a run that passes the kernel check
	# at "not run", before it builds, sweeps, or starts a server.
	out=$(PATH="$RUN/shim:$PATH" THINKTHEN_TOOLCHAINS="$RUN/none" sh "$LIMIT" 60 bash ./check.sh 2>&1)
	code=$?
	set -e
	same "$code" 77
	has "$out" "not run: check.sh runs on Linux only (uname: Darwin)"
}
check darwin_reports_not_run

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
	q -c "GRANT EXECUTE ON FUNCTION thinkthen_decide(text, text) TO PUBLIC" \
		-c "CREATE FUNCTION tt_deliberate(x integer) RETURNS integer LANGUAGE sql AS 'SELECT \$1'" >/dev/null
	out=$(q -c "SELECT has_function_privilege('public', 'thinkthen_decide(text, text)', 'EXECUTE')")
	q -c "REVOKE EXECUTE ON FUNCTION thinkthen_decide(text, text) FROM PUBLIC" -c "DROP FUNCTION tt_deliberate(integer)" >/dev/null
	same "$out" t
}
check a_deliberate_grant_survives
bare_text_is_never_a_path() {
	fresh generic
	out=$(q -c "SELECT thinkthen_decide('refund.json', 'x')")
	has "$out" "thinkthen usage: a question file is named with the @ spelling: '@refund.json'; bare text is never a path (retryable: no)"
	out=$(q -c "SELECT thinkthen_annotate('form.json', 'x')")
	has "$out" "'@form.json'; bare text is never a path"
	same "$(bcount)" 0
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
	q -c "CREATE ROLE tt_exec LOGIN" -c "GRANT EXECUTE ON FUNCTION thinkthen_decide(text, text) TO tt_exec" >/dev/null
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
	for call in "SELECT count(*) FROM thinkthen_decide('@no-such-file.json', ARRAY['a', 'b'])" \
		"SELECT thinkthen_warm('@no-such-file.json', body) FROM tickets"; do
		has "$(q -c "$call")" "thinkthen local: the question file '@no-such-file.json' $DID_NOT_READ"
	done
	for call in "SELECT count(*) FROM thinkthen_decide('@broken.json', ARRAY['a', 'b'])" \
		"SELECT thinkthen_warm('@broken.json', body) FROM tickets"; do
		has "$(q -c "$call")" "thinkthen local: the question file '@broken.json' does not parse: "
	done
	has "$(q -c "SELECT thinkthen_warm('{\"choose\":\"Which?\",\"options\":[\"a\",\"b\"]}', body) FROM tickets")" \
		"thinkthen usage: thinkthen_warm takes a decide question; ask others with thinkthen_decide (retryable: no)"
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
	for call in "SELECT thinkthen_decide('$Q', 'b')" "SELECT count(*) FROM thinkthen_decide('$Q', ARRAY['c', 'd'])" \
		"SELECT thinkthen_warm('$Q', body) FROM tickets"; do
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
	held "SET thinkthen.deadline_ms = 1000; SELECT count(*) FROM thinkthen_decide('$Q', $(rows 200))"
	wait "$HELD" || true
	within $(($(now_ms) - start)) 1500
	has "$(cat "$RUN/held.out")" "thinkthen deadline"
	same "$(bcount)" 8
	brelease
}
check batch_deadline
batch_cancel() {
	fresh arm/held "thinkthen.throttle = 8"
	held "SELECT count(*) FROM thinkthen_decide('$Q', $(rows 200))"
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
	held "SELECT count(*) FROM thinkthen_decide('$Q', $(rows 200))"
	bwait 8
	q -c "SELECT pg_log_backend_memory_contexts($(victim))" >/dev/null
	sleep 0.3
	brelease
	wait "$HELD"
	same "$(cat "$RUN/held.out")" 200
	same "$(bcount)" 200
}
check benign_interrupt_finishes
a_timed_out_batch_leaves_the_session_working() {
	fresh arm/held "thinkthen.throttle = 8"
	out=$(q -c "SET statement_timeout = '500ms'" -c "SELECT count(*) FROM thinkthen_decide('$Q', $(rows 200))" \
		-c "\\! echo release > $RUN/b.in" -c "RESET statement_timeout" -c "SELECT thinkthen_decide('$Q', 'after')")
	has "$out" "canceling statement due to statement timeout"
	same "$(tail -n1 <<<"$out")" t
	same "$(bcount)" 9
}
check a_timed_out_batch_leaves_the_session_working
a_small_batch_answers_at_once() {
	fresh generic
	q -c "SELECT thinkthen_decide('$Q', 'warm up')" >/dev/null
	ms=$(timed "SELECT count(*) INTO n FROM thinkthen_decide('$Q', ARRAY['a', 'b'])")
	within "$ms" 90
}
check a_small_batch_answers_at_once

echo "== the answer cache"
# Throttle 32, because each loopback request takes about 41 ms on a reused connection.
warm_then_decide_sends_nothing() {
	fresh generic "thinkthen.throttle = 32"
	pairs="FROM generate_series(1, 20) g"
	same "$(q -c "SELECT thinkthen_warm('$Q', 'order ' || g) $pairs")" 20
	same "$(bcount)" 20
	same "$(q -c "SELECT thinkthen_warm('$Q', 'order ' || g) $pairs" -c "SELECT count(*) $pairs WHERE thinkthen_decide('$Q', 'order ' || g)" | tail -1)" 20
	same "$(q -c "SELECT count(*) $pairs WHERE thinkthen_decide('$Q', 'order ' || g)")" 20
	same "$(bcount)" 20
}
check warm_then_decide_sends_nothing
# Ticket 0129: warm takes the banded file decide uses, and decide then reads the cache.
warm_takes_the_banded_file_decide_uses() {
	fresh generic
	pairs="FROM generate_series(1, 20) g"
	same "$(q -c "SELECT thinkthen_warm('@refund.json', 'refund ' || g) $pairs")" 20
	same "$(bcount)" 20
	same "$(q -c "SELECT count(*) $pairs WHERE thinkthen_decide('@refund.json', 'refund ' || g)")" 20
	same "$(bcount)" 20
}
check warm_takes_the_banded_file_decide_uses
another_model_sends_again() {
	fresh generic
	q -c "SELECT count(*) FROM generate_series(1, 2) g WHERE thinkthen_decide('{\"decide\":\"Is it red?\",\"model\":\"judge-a\"}', 'item ' || g)" \
		-c "SELECT count(*) FROM generate_series(1, 2) g WHERE thinkthen_decide('{\"decide\":\"Is it red?\",\"model\":\"judge-b\"}', 'item ' || g)" >/dev/null
	same "$(bcount)" 4
}
check another_model_sends_again
# The warm pass sends each row once, and a second pass sends nothing. A
# cached row costs a file read, so the second pass takes at most half of
# the first. Both passes slow together on a busy machine, and the ratio holds.
twenty_thousand_warm_rows() {
	fresh generic "thinkthen.throttle = 32"
	warm="SELECT thinkthen_warm('$Q', 'order ' || g) INTO n FROM generate_series(1, 20000) g"
	cold=$(QTIMEOUT=180 timed "$warm")
	same "$(bcount)" 20000
	cached=$(QTIMEOUT=180 timed "$warm")
	same "$(bcount)" 20000
	[ -n "$cold" ]
	[ -n "$cached" ]
	echo "         warm passes: cold $cold ms, cached $cached ms"
	[ $((cached * 2)) -le "$cold" ] || { echo "the cached pass took $cached ms, over half of the cold pass's $cold ms" >&2; return 1; }
}
check twenty_thousand_warm_rows
warm_refuses_row_past_limit() {
	fresh generic
	has "$(q -c "SELECT thinkthen_warm('$Q', 'order ' || g) FROM generate_series(1, 20001) g")" \
		"thinkthen usage: thinkthen_warm takes at most 20,000 rows per call (retryable: no)"
	same "$(bcount)" 0
}
check warm_refuses_row_past_limit
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
	[ -z "$(find "$SCRATCH" -type f | head -1)" ]
}
check the_environment_seeds_the_cache
the_cache_setting_names_the_folder() {
	mkdir -p "$RUN/named-cache"
	fresh generic "thinkthen.cache = '$RUN/named-cache'"
	same "$(q -c "SELECT thinkthen_warm('$Q', t) FROM (VALUES ('a'), ('b')) v(t)")" 2
	[ -n "$(find "$RUN/named-cache" -type f | head -1)" ]
	[ -z "$(find "$CACHEDIR" -type f | head -1)" ]
}
check the_cache_setting_names_the_folder

echo "== engine settings"
throttle_setting_holds_eight() {
	fresh arm/held "thinkthen.throttle = 8"
	held "SELECT count(*) FROM thinkthen_decide('$Q', $(rows 64))"
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
	has "$(q -c "SELECT count(*) FROM thinkthen_decide('$Q', ARRAY['a', 'b', 'c', 'd'])")" \
		"thinkthen usage: this engine answers at most 3 records in one call (retryable: no)"
	fresh generic "thinkthen.max_requests = 0"
	out=$(q -c '\set VERBOSITY verbose' -c "SELECT thinkthen_decide('$Q', 'a')")
	has "$out" 22023
	has "$out" "a request limit is a whole number of 1 or more"
	same "$(bcount)" 0
}
check request_limit_refuses_before_sending
a_changed_limit_rebuilds() {
	fresh generic
	out=$(q -c "SET thinkthen.max_requests = 3" -c "SELECT count(*) FROM thinkthen_decide('$Q', ARRAY['a', 'b', 'c'])" \
		-c "SET thinkthen.max_requests = 2" -c "SELECT count(*) FROM thinkthen_decide('$Q', ARRAY['d', 'e', 'f'])")
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
    same "$out" t
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
	q -c "CREATE ROLE tt_limited LOGIN" -c "GRANT EXECUTE ON FUNCTION thinkthen_decide(text, text[]) TO tt_limited" \
		-c "ALTER ROLE tt_limited SET thinkthen.max_requests = 2" >/dev/null
	has "$(PGUSER_AS=tt_limited q -c "SELECT count(*) FROM thinkthen_decide('$Q', ARRAY['a', 'b', 'c'])")" \
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
	out=$(q -c "SELECT thinkthen_decide('$Q', 'row 1')" -c "SELECT count(*) FROM thinkthen_decide('$Q', ARRAY['a', 'b', 'c', 'd'])")
	# A new backend starts from zero: a cached answer sends nothing, and a
	# four-record batch sends the three the total leaves, then refuses.
	same "$out" "t
ERROR:  thinkthen usage: thinkthen.max_requests_total allows 3 requests in this backend, and they are spent (retryable: no)"
	same "$(bcount)" 6
}
check the_total_holds_across_rows
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
	q -c "CREATE ROLE tt_plain LOGIN" -c "GRANT EXECUTE ON FUNCTION thinkthen_decide(text, text), thinkthen_usage() TO tt_plain" >/dev/null
	out=""
	for role in tt_plain postgres; do
		out+=$(PGUSER_AS=$role q -c "SET thinkthen.api_key = '$secret'" -c "SELECT thinkthen_decide('$Q', 'a')")
		out+=$(PGUSER_AS=$role q -c "SELECT count(*) FROM thinkthen_usage()" -c "SET thinkthen.api_key = '$secret'" -c "SELECT thinkthen_decide('$Q', 'a')")
	done
	out+=$(PGOPTIONS="-c thinkthen.api_key=$secret" PGUSER_AS=tt_plain q -c "SELECT thinkthen_decide('$Q', 'a')")
	same "$(grep -oF "thinkthen usage: thinkthen.api_key is not read; unset it and set THINKTHEN_API_KEY in the server's environment (retryable: no)" <<<"$out" | wc -l)" 5
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
	held "SELECT count(*) FROM thinkthen_decide('$Q', $(rows 64))"
	bwait 8
	masks_hold
	brelease
	wait "$HELD"
}
check signal_masks
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
	has "$out" "the panic probe fired"
	same "$(tail -n1 <<<"$out")" 1
}
check a_panic_is_an_error

echo "== the update path"
an_update_cannot_grant_public() {
	printf 'CREATE FUNCTION thinkthen_rehearsal_probe(x integer) RETURNS integer LANGUAGE sql AS %s;\n' "'SELECT \$1'" \
		>.runtime/tree/usr/share/postgresql/16/extension/thinkthen--$EXT_VERSION--$EXT_VERSION-probe.sql
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
		echo "         $line"
		case $line in
		"pass $id")
			case $id in
			13-filter-records|15-rank-records|16-rank-stable-tie) same "$(bcount)" 3 ;;
			14-filter-none) same "$(bcount)" 2 ;;
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
[ "$FAILED" = 0 ]
