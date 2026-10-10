"""Selected installed DuckDB proofs for the portable SQL call forms."""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
from pathlib import Path

from harness import EXTENSION, Backend, case, child_env, expect, main, rows, run, said


@case
def discovery_describes_every_registered_function_without_sending():
    query = ("SELECT function_name, function_type, parameter_types, return_type, description "
             "FROM duckdb_functions() WHERE starts_with(function_name, 'thinkthen_') "
             "ORDER BY function_name, function_type, parameter_types")
    with Backend() as backend:
        got = run([
            "SET thinkthen_max_requests_total=0",
            "SET enable_external_access=false",
            "SET thinkthen_refresh_cache=2",
            query, query,
            "SELECT thinkthen_usage_status()->>'state'",
        ], backend.base(), keyless=True)
        inventory = rows(got[3])
        expect(bool(inventory), True, "registered inventory is nonempty")
        expect(rows(got[4]), inventory, "discovery remains stable")
        missing = [row[:3] for row in inventory if not row[4] or not row[4].strip()]
        expect(missing, [], "every registered overload has a purpose")
        expect({row[1] for row in inventory}, {"scalar", "table", "macro", "table_macro"},
               "discovery covers native functions and SQL macros")
        signatures = [tuple([row[0], row[1], tuple(row[2])]) for row in inventory]
        expect(len(set(signatures)), len(signatures), "discovery has unique overloads")
        purposes = {row[0]: row[4] for row in inventory}
        expect(purposes["thinkthen_usage"], "Read count-only usage totals.", "usage purpose")
        expect("removed" in purposes["thinkthen_warm"], True, "warm documents its refusal")
        expect("removed" in purposes["thinkthen_probability"], True, "probability documents its refusal")
        expect(rows(got[5]), [["disabled"]], "discovery creates no engine")
        expect(backend.count(), 0, "loading and discovery send nothing")


@case
def named_decide_binds_by_name_and_settings_refuse_before_send():
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_decide(settings := '{\"threshold\":0.7}', input := 'refund now', question := 'Is it a refund?')",
            "SELECT thinkthen_decide('Is it a refund?', 'refund now', '{\"threshold\":1.5}')",
            "SELECT thinkthen_decide('Is it a refund?', 'refund now', '{\"threshold\":0.7}', '0.8')",
            "SELECT thinkthen_decide(stare := 'x', question := 'Is it a refund?', input := 'refund now')",
            "SELECT thinkthen_decide('Is it a refund?', 'refund now', 0)",
        ], backend.base())
        expect(rows(got[0]), [[True]], "out-of-order named decide")
        expect(said(got[1]).startswith("thinkthen usage:"), True, "invalid settings")
        expect(said(got[2]), "thinkthen usage: settings repeats `threshold` from the question or named arguments (retryable: no)", "duplicate threshold")
        expect("does not support the supplied arguments" in got[3]["error"], True, "unknown name")
        expect(said(got[4]), "thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'", "removed deadline slot")
        expect(backend.count(), 1, "only the valid call sends")


@case
def listed_settings_and_members_are_distinct_overloads():
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_choose('Which team?', 'refund now', '{\"options\":[\"billing\",\"shipping\"]}')",
            "SELECT thinkthen_choose('Which team?', 'refund now', ['billing','shipping'])",
            "SELECT thinkthen_choose('Which team?', 'refund now', ['billing','shipping'], '{\"threshold\":0.7}')",
            "SELECT thinkthen_choose('Which team?', 'refund now', ['billing','shipping'], 0)",
            "SELECT thinkthen_choose('Which team?', 'refund now', ['billing','shipping'], '{\"options\":[\"billing\",\"shipping\"]}')",
            "SELECT thinkthen_choose(stare := 'x', question := 'Which team?', input := 'refund now')",
        ], backend.base())
        for result in got[:3]:
            expect(rows(result), [["billing"]], "listed overload value")
        expect(said(got[3]), "thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'", "removed listed deadline")
        expect(said(got[4]), "thinkthen usage: a members argument and settings both name members (retryable: no)", "duplicate members")
        expect("does not support the supplied arguments" in got[5]["error"], True, "unknown listed name")
        expect(backend.count(), 1, "the same question and input reuse one cached request across local cuts")


@case
def listed_null_members_keep_settings_and_double_settings_refuse():
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_choose('Which team?', 'refund now', NULL, '{\"options\":[\"billing\",\"shipping\"]}')",
            "SELECT thinkthen_choose('Which team?', 'refund now', NULL::VARCHAR, '{\"options\":[\"billing\",\"shipping\"]}')",
            "SELECT thinkthen_choose('Which team?', 'refund now', NULL::VARCHAR[], '{\"options\":[\"billing\",\"shipping\"]}')",
            "SELECT thinkthen_score('How strong?', 'refund now', NULL::VARCHAR, '{\"levels\":[\"weak\",\"strong\"]}')",
            "SELECT thinkthen_tag('Which topics?', 'refund now', NULL::VARCHAR, '{\"labels\":[\"billing\",\"shipping\"]}')",
            "SELECT thinkthen_choose('Which team?', 'refund now', '{\"options\":[\"billing\",\"shipping\"]}', '{\"threshold\":0.7}')",
            "SELECT thinkthen_choose('Which team?', NULL::VARCHAR, NULL::VARCHAR, '{\"options\":[\"billing\",\"shipping\"]}')",
        ], backend.base())
        expect([rows(result) for result in got[:5]],
               [[['billing']], [['billing']], [['billing']], [[0.1]], [[['billing', 'shipping']]]],
               "bare and typed NULL members keep each verb's SQL output type")
        expect(said(got[5]), "thinkthen usage: a settings object follows members, not another settings object (retryable: no)",
               "two non-NULL settings objects refuse")
        expect(rows(got[6]), [[None]], "NULL input stays NULL")
        expect(backend.count(), 3, "three distinct valid questions send; refusals and NULL input send nothing")


@case
def ordinary_errors_show_retryability_while_removed_warm_stays_plain():
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_plan('asks for a refund', '[\"not keyed\"]')",
            "SELECT thinkthen_warm('Is it a refund?', 'refund now')",
            "SET thinkthen_max_retries = 0",
            "SELECT thinkthen_decide('Is it a refund?', 'refund now')",
        ], backend.base('arm/503'))
        expect(said(got[0]), "thinkthen usage: invalid type: sequence, expected one keyed JSON object of text values at line 1 column 0 (retryable: no)",
               "ordinary usage includes a no marker")
        expect(said(got[1]), "thinkthen usage: thinkthen_warm was removed; pack records with thinkthen_decide_many",
               "removed call keeps its plain sentence")
        expect(said(got[3]).endswith("(retryable: yes)"), True, "backend 503 marks retryability")
        expect(said(got[3]).startswith("thinkthen backend:"), True, "backend kind stays visible")
        expect(backend.count(), 1, "only the backend failure sends")


@case
def keyed_decide_preserves_key_body_and_one_send():
    with Backend() as backend:
        got = run([
            "SELECT key, value, probability FROM thinkthen_decide_many('asks for a refund', '{\"7\":\"Refund me please.\"}')",
        ], backend.base("arm/full/capture"))
        expect(rows(got[0]), [["7", True, 0.9]], "keyed row")
        expect(backend.count(), 1, "one keyed request")
        expect(backend.capture(), [
            '{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"Refund me please.\\". asks for a refund"}}}'
        ], "independent one-record body")


@case
def keyed_verbs_keep_members_order_and_refuse_duplicate_keys():
    with Backend() as backend:
        got = run([
            "SELECT key, value, probability FROM thinkthen_choose_many('Which team?', '{\"9\":\"refund now\"}', '{\"options\":[\"billing\",\"shipping\"]}')",
            "SELECT key, value FROM thinkthen_score_many('How strong?', '{\"9\":\"refund now\"}', '{\"levels\":[\"weak\",\"strong\"]}')",
            "SELECT key, value FROM thinkthen_tag_many('Which topics?', '{\"9\":\"refund now\"}', '{\"labels\":[\"billing\",\"shipping\"]}')",
            "SELECT * FROM thinkthen_decide_many('Is it a refund?', '{\"9\":\"refund now\",\"9\":\"good morning\"}')",
            "SELECT * FROM thinkthen_choose_many('Which team?', '{\"9\":4}', '{\"options\":[\"billing\",\"shipping\"]}')",
        ], backend.base())
        expect(rows(got[0]), [["9", "billing", 0.9]], "choose keyed value and probability")
        # The fixed backend assigns 0.9 to level zero and 0.1 to level one.
        expect(rows(got[1]), [["9", 0.1]], "score weighted position")
        expect(rows(got[2]), [["9", ["billing", "shipping"]]], "tag keyed value")
        expect(said(got[3]).startswith("thinkthen usage:"), True, "duplicate keyed input")
        expect(said(got[4]).startswith("thinkthen usage:"), True, "non-text keyed input")
        expect(backend.count(), 3, "invalid keyed inputs send nothing")


@case
def six_song_vector_and_keyed_forms_share_one_packed_body():
    question = "The text is the title of a song by the Beatles. It appears on the album Abbey Road."
    settings = '{"threshold":"0.3:0.7"}'
    songs = [(1, "Here Comes the Sun"), (2, "Yellow Submarine"),
             (5, "Octopus's Garden"), (9, "Penny Lane"),
             (14, "A Day in the Life"), (17, "Hey Jude")]
    quote = lambda value: "'" + value.replace("'", "''") + "'"
    values = ", ".join(f"({key}, {quote(title)})" for key, title in songs)
    keyed = json.dumps({str(key): title for key, title in songs})
    with Backend() as backend:
        got = run([
            f"SELECT id, thinkthen_decide({quote(question)}, title, {quote(settings)}) FROM (VALUES {values}) s(id,title) ORDER BY id",
            f"SELECT key, value, probability FROM thinkthen_decide_many({quote(question)}, {quote(keyed)}, {quote(settings)}) ORDER BY CAST(key AS BIGINT)",
        ], backend.base("arm/full/capture"))
        expect(rows(got[0]), [[1, True], [2, True], [5, True], [9, True], [14, True], [17, True]],
               "six vector positions")
        expect(rows(got[1]), [["1", True, 0.9], ["2", True, 0.9], ["5", True, 0.9],
                              ["9", True, 0.9], ["14", True, 0.9], ["17", True, 0.9]],
               "keyed ids with gaps")
        expect(backend.count(), 1, "the keyed form reads the vector's cached packed request")
        first = backend.capture()
        expect(len(first), 1, "one captured packed request")
        expect(rows(run([
            f"SELECT key, value, probability FROM thinkthen_decide_many({quote(question)}, {quote(keyed)}, {quote(settings)}) ORDER BY CAST(key AS BIGINT)"
        ], backend.base("arm/full/capture"))[0]),
               [["1", True, 0.9], ["2", True, 0.9], ["5", True, 0.9],
                ["9", True, 0.9], ["14", True, 0.9], ["17", True, 0.9]],
               "fresh keyed execution")
        expect(backend.count(), 2, "fresh cache requires one more packed request")
        bodies = backend.capture()
        expect(len(bodies), 2, "two independent captured requests")
        expect(bodies[0], bodies[1], "vector and keyed requests have identical bytes")


@case
def removed_warm_and_probability_forms_refuse_without_transport():
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_warm('Is it a refund?', 'refund now')",
            "SELECT thinkthen_warm('Is it a refund?', 'refund now', 'shared')",
            "SELECT thinkthen_probability('Is it a refund?', 'refund now')",
            "SELECT thinkthen_probability('Is it a refund?', 'refund now', '{\"threshold\":0.7}')",
            "SELECT thinkthen_probability('Is it a refund?', 'refund now', settings := '{}')",
            "SELECT thinkthen_probability('Is it a refund?', 'refund now', 0)",
            "SELECT thinkthen_probability('Is it a refund?', NULL)",
        ], backend.base())
        for result in got[:2]:
            expect(said(result),
                   "thinkthen usage: thinkthen_warm was removed; pack records with thinkthen_decide_many",
                   "removed warm sentence")
        for result in got[2:]:
            expect(said(result),
                   "thinkthen usage: thinkthen_probability was removed; order records with thinkthen_rank",
                   "removed probability sentence")
        expect(backend.count(), 0, "removed calls open no request")


@case
def rank_and_details_share_one_portable_request():
    with Backend() as backend:
        got = run([
            "SELECT key, rank, probability FROM thinkthen_rank('Is it a refund?', '{\"a\":\"refund now\"}', '{\"batch\":1}')",
            "SELECT thinkthen_details('Is it a refund?', 'refund now', '{\"batch\":1}')",
            "SELECT * FROM thinkthen_rank('Is it a refund?', '{\"a\":\"refund now\"}', '{\"threshold\":0.7}')",
        ], backend.base())
        expect(rows(got[0]), [["a", 1, 0.9]], "rank row from one answer")
        detail = json.loads(rows(got[1])[0][0])
        expect(detail["value"], True, "details retains the decision")
        expect(said(got[2]), "thinkthen usage: the settings key `threshold` does not belong to this verb (retryable: no)",
               "rank refuses a threshold")
        expect(backend.count(), 1, "rank and details read one cache identity")
        json_rows = run([
            "SELECT typeof(d), d->>'$.value' FROM (SELECT thinkthen_details('Is it a refund?', 'refund now', '{\"batch\":1}') AS d)",
            "SELECT typeof(d), d->>'$.status', d->>'$.error.kind' FROM (SELECT thinkthen_try_details('', 'x') AS d)",
            "SELECT typeof(thinkthen_usage_status()), json_type(thinkthen_usage_status())",
            "SELECT typeof(thinkthen_native_many(NULL, 'malformed', 'malformed', 0))",
        ], backend.base())
        expect(rows(json_rows[0]), [["JSON", "true"]], "details JSON operations")
        expect(rows(json_rows[1]), [["JSON", "failed", "usage"]], "try-details failure JSON operations")
        expect(rows(json_rows[2]), [["JSON", "OBJECT"]], "usage JSON operations")
        expect(rows(json_rows[3]), [["JSON"]], "native keyed result JSON type")
        complete = rows(run(["SELECT function_name FROM duckdb_functions() WHERE function_type='macro' AND starts_with(function_name, 'thinkthen_') AND ends_with(function_name, '_complete') ORDER BY function_name"], backend.base())[0])
        expect(bool(complete), True, "registered complete functions are present")
        calls = []
        for (name,) in complete:
            calls += [f"SELECT typeof(d), d IS NULL FROM (SELECT {name}(NULL, 'malformed', 'malformed') AS d)",
                      f"SELECT typeof(d), d->>'$.native.error.kind' FROM (SELECT {name}('malformed', 'malformed') AS d)"]
        results = run(calls, backend.base())
        for index, (name,) in enumerate(complete):
            expect(rows(results[2 * index]), [["JSON", True]], name + " NULL before malformed partners")
            expect(rows(results[2 * index + 1]), [["JSON", "usage"]], name + " failure JSON operations")
        expect(backend.count(), 2, "only the additional live details call sends")



@case
def annotate_accepts_call_settings_and_refuses_question_only_keys():
    question_set = '{"version":1,"questions":{"refund":{"decide":"Is it a refund?"}}}'
    with Backend() as backend:
        got = run([
            f"SELECT thinkthen_annotate('{question_set}', 'refund now', '{{\"batch\":\"max\"}}')",
            f"SELECT typeof(d), d->>'$.refund' FROM (SELECT thinkthen_annotate('{question_set}', 'refund now', '{{\"batch\":\"max\"}}') AS d)",
            f"SELECT thinkthen_annotate('{question_set}', 'refund now', '{{\"none\":true}}')",
            f"SELECT thinkthen_annotate('{question_set}', 'refund now', 0)",
        ], backend.base())
        expect(json.loads(rows(got[0])[0][0]), {"refund": True}, "annotated set value")
        expect(rows(got[1]), [["JSON", "true"]], "annotate JSON operations")
        expect(said(got[2]).startswith("thinkthen usage:"), True, "none is find-only")
        expect(said(got[3]),
               "thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'",
               "removed annotate deadline")
        expect(backend.count(), 1, "invalid annotation settings send nothing")


@case
def try_details_keeps_recoverable_settings_failures_beside_an_answer():
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_try_details('Is it a refund?', 'refund now', '{\"threshold\":0.7}')",
            "SELECT thinkthen_try_details('Is it a refund?', 'refund now', '{\"threshold\":1.5}')",
            "SELECT thinkthen_try_details('Is it a refund?', 'refund now', NULL)",
            "SELECT thinkthen_try_details('Is it a refund?', 'refund now', 1)",
        ], backend.base())
        expect(json.loads(rows(got[0])[0][0])["status"], "answered", "portable answer")
        failed = json.loads(rows(got[1])[0][0])
        expect((failed["status"], failed["error"]["kind"]), ("failed", "usage"),
               "invalid settings retain the safe failure carrier")
        expect(json.loads(rows(got[2])[0][0])["status"], "answered", "NULL settings are absent")
        expect(said(got[3]),
               "thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'",
               "removed try-details deadline")
        expect(backend.count(), 1, "only one request identity was sent")


@case
def recognize_named_settings_and_old_deadline_boundary():
    with Backend() as backend:
        got = run([
            "SELECT r.text, r.start FROM (SELECT unnest(thinkthen_recognize(settings := '{\"model\":\"jev-1.13.0\"}', kinds := ['person'], input := 'Le café 😀 Maria Chen arrived.')) AS r)",
            "SELECT thinkthen_recognize('Maria Chen arrived.', ['person'], '{\"none\":true}')",
            "SELECT thinkthen_relations('Maria Chen arrived.', '{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A name of a person.\"}}}', 0)",
        ], backend.base())
        expect(rows(got[0]), [["arrived.", 21]], "named recognize model and code-point start")
        expect(said(got[1]).startswith("thinkthen usage:"), True, "none belongs only to find")
        expect(said(got[2]),
               "thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'",
               "removed relations deadline")
        expect(backend.count(), 2, "the one valid recognized name uses steps 1 and 2 only")


REPLAY_CHILD = r"""
import json, os, sys
import duckdb
extension, folder, cap, queries, misses = sys.argv[1:]
cap = int(cap)
def opened():
    db = duckdb.connect(config={"allow_unsigned_extensions": "true"})
    db.execute("SET enable_progress_bar = false")
    db.execute(f"LOAD '{extension}'")
    db.execute("SET thinkthen_cache = 'off'")
    return db
if cap:
    spender = opened()
    spender.execute("SET thinkthen_max_requests_total = 1")
    assert spender.execute("SELECT thinkthen_decide('Is it a refund?', 'spend once')").fetchall() == [(True,)]
os.environ.pop("THINKTHEN_API_KEY", None)
db = opened()
db.execute(f"SET thinkthen_replay = '{folder}'")
db.execute(f"SET thinkthen_max_requests_total = {cap}")
before = dict(db.execute("SELECT * FROM thinkthen_usage()").fetchall())
answers = [db.execute(query).fetchall() for query in json.loads(queries)]
failures = []
for query in json.loads(misses):
    try:
        db.execute(query).fetchall()
    except duckdb.Error as error:
        failures.append({"error": str(error)})
    else:
        raise AssertionError("a strict replay miss answered")
after = dict(db.execute("SELECT * FROM thinkthen_usage()").fetchall())
print(json.dumps({"answers": answers, "before": before, "after": after, "failures": failures}))
"""


@case
def staged_replay_answers_with_zero_or_spent_total_and_no_key():
    rules = ('{"version":1,"recognize":{"kinds":{"person":null},'
             '"relations":[{"name":"near","source":"person","target":"person"}]}}')
    queries = [
        "SELECT thinkthen_recognize('Maria Chen arrived.', ['person'])",
        f"SELECT thinkthen_relations('Maria Chen arrived.', '{rules}')",
        "SELECT * FROM thinkthen_relate('SELECT 1 AS id, ''Ada'' AS name, ''person'' AS kind UNION ALL SELECT 2, ''Acme'', ''organization''', ['works_for=person:organization'])",
    ]
    misses = [query.replace('Maria Chen', 'Unknown name').replace("''Ada''", "''Unknown''")
              for query in queries]
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        recorded = run(["SET thinkthen_cache = 'off'", f"SET thinkthen_record = '{folder}'", *queries],
                       backend.base("arm/full"))
        answers = [rows(result) for result in recorded[2:]]
        expect(len(answers[0][0][0]), 2, "two stored names")
        expect(len(answers[1][0][0]), 2, "two stored nested relations")
        expect(answers[2], [["works_for", "1", "2", 0.9, False]], "stored explicit relation")
        recorded_sends = backend.count()
        expect(recorded_sends > 0, True, "recording used owned loopback responses")
        for cap in (0, 1):
            with tempfile.TemporaryDirectory() as home:
                done = subprocess.run(
                    [sys.executable, "-c", REPLAY_CHILD, str(EXTENSION), folder, str(cap),
                     json.dumps(queries), json.dumps(misses)],
                    env=child_env(backend.base("arm/full"), Path(home), keyless=cap == 0),
                    capture_output=True, text=True, timeout=60, check=False,
                )
            expect(done.returncode, 0, f"replay child exit: {done.stderr[-800:]}")
            got = json.loads(done.stdout)
            expect(got["answers"], answers, f"keyless strict replay at cap {cap}")
            for metric in ("requests_sent", "input_tokens", "output_tokens"):
                expect(got["before"][metric], cap, f"already spent {metric}")
                expect(got["after"][metric], cap, f"replay and misses add no {metric}")
            expect(got["after"]["cache_answers"] >= got["before"]["cache_answers"], True,
                   "replay may add cache answers")
            for failure in got["failures"]:
                expect(said(failure), "thinkthen local: the replay folder holds no answer for this question (retryable: no)",
                       "strict miss precedes quota and key lookup")
            expect(backend.count(), recorded_sends + cap, "only the positive-cap spender sent")
        live = run(["SET thinkthen_cache = 'off'", "SET thinkthen_max_requests_total = 0", *queries,
                    "SELECT * FROM thinkthen_usage()"], backend.base("arm/full"))
        for failure in live[2:-1]:
            expect(said(failure), "thinkthen usage: this process has spent its request total of 0; raise SET thinkthen_max_requests_total or RESET it (retryable: no)",
                   "live zero refuses at actual send reservation")
        for metric in ("requests_sent", "input_tokens", "output_tokens"):
            expect(dict(rows(live[-1]))[metric], 0, f"live zero {metric}")
        expect(backend.count(), recorded_sends + 1, "live zero sends nothing")
        for index, (query, miss) in enumerate(zip(queries, misses, strict=True)):
            limited = run(["SET thinkthen_cache = 'off'", "SET thinkthen_max_requests_total = 1",
                           query, miss, "SELECT * FROM thinkthen_usage()"], backend.base("arm/full"))
            spent = "thinkthen usage: this process has spent its request total of 1; raise SET thinkthen_max_requests_total or RESET it (retryable: no)"
            if index < 2:
                expect(said(limited[2]), spent, "recognition's next stage cannot send")
            else:
                expect(rows(limited[2]), answers[2], "one allowed explicit relation answers")
            expect(said(limited[3]), spent, "spent positive total denies the next call")
            for metric in ("requests_sent", "input_tokens", "output_tokens"):
                expect(dict(rows(limited[-1]))[metric], 1, f"one allowed live {metric}")
            expect(backend.count(), recorded_sends + 2 + index, "exactly one allowed live send per route")


if __name__ == "__main__":
    raise SystemExit(main())
