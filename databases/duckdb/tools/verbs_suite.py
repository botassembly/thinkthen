"""The verbs through the real engine on a loopback backend: row mapping,
NULL rows, results, details, recognize, deadlines, warm, usage, and secrecy.

Each case starts its own backend and runs its SQL in a fresh child.
"""

from __future__ import annotations

import json
import hashlib
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "sqlite" / "tests"))
from conditional_backend import ConditionalBackend

from harness import Backend, case, expect, main, rows, run, said
from verbs_budget import PACKED_PAIR_BODIES, PackedReplies
from verbs_complete import complete_question_files_keep_identity, complete_question_refusals_and_nulls, complete_question_rechecks_prepared_authority
from verbs_portable import portable_batch_identity

REFUND = "Does the writer ask for a refund?"
SHUFFLED = "(VALUES (1, 'good morning'), (2, NULL), (3, 'refund now'), (4, 'good morning'), (5, NULL), (6, 'refund now')) t(i, x)"

case(complete_question_files_keep_identity)
case(complete_question_refusals_and_nulls)
case(complete_question_rechecks_prepared_authority)
case(portable_batch_identity)


def column(result: dict) -> list:
    return [row[0] for row in rows(result)]


@case
def r1_1_answers_map_back_by_text():
    with Backend() as backend:
        got = run(
            [
                "SET thinkthen_batch = '1'",
                f"SELECT thinkthen_decide('{REFUND}', x) FROM {SHUFFLED} ORDER BY i",
                f"SELECT thinkthen_probability('{REFUND}', x) FROM {SHUFFLED} ORDER BY i",
            ],
            backend.base("case/28-decide-many-repeated-texts"),
        )
        expect(column(got[1]), [False, None, True, False, None, True], "decide rows")
        expect(column(got[2]), [0.03, None, 0.97, 0.03, None, 0.97], "probability rows")
        expect(backend.count(), 2, "counted sends for two distinct texts")


@case
def r1_1_score_maps_back_by_text():
    table = "(VALUES (1, 'refund now'), (2, NULL), (3, 'maybe so'), (4, 'refund now')) t(i, x)"
    got = run_generic(
        [f"SELECT thinkthen_score('How strong is the claim?', x, ['weak', 'moderate', 'strong']) FROM {table} ORDER BY i"]
    )
    values = column(got[0])
    expect(values[1], None, "the NULL row")
    expect(values[0], values[3], "a repeated text reads its own answer")


def run_generic(statements: list[str], **options) -> list[dict]:
    with Backend() as backend:
        return run(statements, backend.base(), **options)


@case
def verbs_answer_through_the_generic_arm():
    got = run_generic(
        [
            "SELECT thinkthen_decide('Is it a refund?', 'refund now')",
            "SELECT thinkthen_choose('Which team?', 'my card was charged twice', ['billing', 'shipping'])",
            "SELECT thinkthen_tag('Which topics?', 'charged twice and late', ['billing', 'shipping'])",
            "SELECT thinkthen_annotate('{\"version\": 1, \"questions\": {\"refund\": {\"decide\": \"Is it a refund?\"}}}', 'refund now')",
            "SELECT thinkthen_decide('Is it a refund?', 'refund now', NULL)",
            "SELECT thinkthen_choose('@q.json', 'text', ['a', 'b'])",
        ]
    )
    expect(column(got[0]), [True], "decide")
    expect(column(got[1]), ["billing"], "choose picks the first option at 0.9")
    expect(column(got[2]), [["billing", "shipping"]], "tag holds each label the generic arm answers yes")
    expect(json.loads(column(got[3])[0]), {"refund": True}, "annotate value_json")
    expect(column(got[4]), [None], "a NULL deadline gives a NULL row")
    expect(said(got[5]), "thinkthen usage: choose takes its question as plain text and its options in the list", "choose refuses a file")


@case
def probability_equals_details_with_no_added_send():
    with Backend() as backend:
        got = run(
            [
                "SELECT thinkthen_probability('Is it a refund?', 'refund now')",
                "SELECT CAST(thinkthen_details('Is it a refund?', 'refund now') ->> '$.answer.probability' AS DOUBLE)",
            ],
            backend.base(),
        )
        expect(column(got[0]), column(got[1]), "probability and details")
        expect(backend.count(), 1, "one counted send for both")


@case
def try_details_keeps_later_good_rows():
    """A bad question and an unreadable named file return safe values within one chunk."""
    with Backend() as backend:
        sql = """SELECT thinkthen_try_details(q, e) FROM (VALUES
            (1, 'Is it a refund?', 'first refund'),
            (2, '', 'private evidence'),
            (3, '@private-missing.json', 'private evidence'),
            (4, 'Is it a refund?', 'last refund'),
            (5, NULL, 'private evidence')) t(i,q,e) ORDER BY i"""
        got = run([sql], backend.base())
        values = [json.loads(value) if value is not None else None for value in column(got[0])]
        expect([value["status"] if value else None for value in values],
               ["answered", "failed", "failed", "answered", None], "row statuses")
        expect(values[1]["error"], {"kind": "usage", "message": "check the row's question and arguments, or raise the process request total when it is spent", "retryable": False}, "safe usage failure")
        expect(values[2]["error"], {"kind": "local", "message": "check the named file and its permissions", "retryable": False}, "safe local failure")
        for value in values[1:3]:
            for private in ("private evidence", "private-missing.json"):
                expect(private in json.dumps(value), False, "no private text in a failed value")
        expect(backend.count(), 1, "compatible good rows share one request")


@case
def try_details_keeps_a_good_row_after_a_backend_failure():
    with Backend() as backend, ConditionalBackend(backend.base()) as proxy:
        sql = """SELECT thinkthen_try_details(q,e) FROM (VALUES
            (1, 'Is it a refund?', 'first'),
            (2, 'Is it a refund?', 'private evidence'),
            (3, 'Is it a refund?', 'last')) t(i,q,e) ORDER BY i"""
        got = run(["SET thinkthen_batch = '1'", sql], proxy.base)
        values = [json.loads(value) for value in column(got[1])]
        expect([value["status"] for value in values], ["answered", "failed", "answered"], "good rows after backend failure")
        expect(values[1]["error"], {"kind": "backend", "message": "the backend did not answer; retry if allowed", "retryable": False}, "typed backend failure")
        expect("private evidence" in json.dumps(values[1]), False, "failed value hides evidence")
        expect(proxy.count(), 3, "three requests reached the proxy")
        expect(backend.count(), 2, "good requests reached the generic backend")


@case
def try_details_null_skips_invalid_settings():
    with Backend() as backend:
        got = run(
            ["SET thinkthen_throttle = 33", "SELECT thinkthen_try_details(NULL, 'private evidence', NULL)",
             "SELECT thinkthen_try_details('Is it a refund?', 'private evidence', NULL)"],
            backend.base(),
        )
        expect(column(got[1]), [None], "SQL NULL before settings")
        expect(json.loads(column(got[2])[0])["error"]["kind"], "usage", "NULL deadline still checks settings")
        expect(backend.count(), 0, "a NULL row sent nothing")


@case
def try_details_null_deadline_answers():
    with Backend() as backend:
        got = run(["SELECT thinkthen_try_details('Is it a refund?', 'refund now', NULL)"], backend.base())
        value = json.loads(column(got[0])[0])
        expect(value["status"], "answered", "NULL optional deadline")
        expect(backend.count(), 1, "NULL deadline sends once")


@case
def try_details_keeps_unresolved_and_spent_total_distinct():
    with Backend() as backend:
        question = '{"decide":"Is it red?","threshold":"0:1"}'
        got = run([f"SELECT thinkthen_try_details('{question}', 'red door')",
                   "SET thinkthen_max_requests_total = 1",
                   "SELECT thinkthen_try_details('Is it blue?', 'a blue door')"], backend.base())
        unresolved = json.loads(column(got[0])[0])
        expect(unresolved["status"], "answered", "unresolved is an answer")
        expect(unresolved["details"]["value"], None, "unresolved details use JSON null")
        expect("error" in unresolved, False, "answered envelope has no error")
        spent = json.loads(column(got[2])[0])
        expect(spent["error"], {"kind": "usage", "message": "check the row's question and arguments, or raise the process request total when it is spent", "retryable": False}, "spent total becomes safe value")
        expect("details" in spent, False, "failed envelope has no details")
        expect(backend.count(), 1, "spent total sends nothing")


@case
def case_41_offsets_count_code_points():
    text = "Le café 😀 Maria Chen arrived."
    got = run_generic([f"SELECT r.text, r.start, r.\"end\", r.length FROM (SELECT unnest(thinkthen_recognize('{text}', ['person'])) AS r)"])
    # The generic arm names the text's last piece. Its start is 21 and its
    # end 29 in code points, and they would be 25 and 33 in UTF-8 bytes.
    expect(rows(got[0]), [["arrived.", 21, 29, 8]], "names and code-point offsets")
    for name, start, end, _ in rows(got[0]):
        expect(text[start:end], name, "a recognized name sits at its code-point offsets")


@case
def r2_10_deadlines():
    with Backend() as backend:
        got = run(
            [
                "SELECT thinkthen_decide('Is it a refund?', 'a', 0)",
                "SELECT thinkthen_decide('Is it a refund?', 'b', -2)",
                "SELECT thinkthen_decide('Is it a refund?', 'c', 4294967296000)",
                "SELECT thinkthen_decide('Is it a refund?', 'd', 9223372036854775807)",
            ],
            backend.base(),
        )
        expect(said(got[0]).split(":")[0], "thinkthen deadline", "a zero deadline")
        for result in got[1:]:
            expect(said(result).split(":")[0], "thinkthen usage", "an out-of-range deadline")
        expect(backend.count(), 0, "counted sends")
        answered = run(["SELECT thinkthen_decide('Is it a refund?', 'e', -1)"], backend.base())
        expect(column(answered[0]), [True], "-1 runs with no deadline")


@case
def r2_22_warm_judges_one_question_per_group():
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        choose = Path(folder) / "choose.json"
        choose.write_text('{"choose": "Which colour?", "options": ["red", "blue"]}')
        got = run(
            [
                "SELECT thinkthen_warm(q, x) FROM (VALUES ('Is it a refund?', 'a'), ('Is it late?', 'b')) t(q, x)",
                f"SELECT thinkthen_warm('@{choose}', 'a')",
            ],
            backend.base(),
        )
        expect(said(got[0]), "thinkthen usage: thinkthen_warm judges one question per group, and this group carries more than one", "two questions")
        expect(said(got[1]), "thinkthen usage: thinkthen_warm takes a decide question; ask others with thinkthen_decide", "warm and a choose file")
        expect(backend.count(), 0, "counted sends")
        warmed = run(["SELECT thinkthen_warm('Is it a refund?', x) FROM (VALUES ('a'), ('b'), ('a')) t(x)"], backend.base())
        expect(column(warmed[0]), [2], "warm counts distinct texts")


@case
def r2_22_warm_takes_the_banded_file_decide_uses():
    """Ticket 0129: warm takes decide's banded file, and decide then reads the cache."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        banded = Path(folder) / "banded.json"
        banded.write_text(json.dumps({"decide": "Is it red?", "true": "Red paint.", "false": "Any other colour.", "model": "judge-b", "threshold": "0.85:0.95"}))
        texts = "(VALUES ('a red door'), ('a blue door'), ('a red door')) t(x)"
        got = run([f"SELECT thinkthen_warm('@{banded}', x) FROM {texts}", f"SELECT thinkthen_decide('@{banded}', x) FROM {texts}"], backend.base())
        expect(column(got[0]), [2], "warm counts distinct texts")
        expect(column(got[1]), [None, None, None], "decide reads unsure under the band")
        expect(backend.count(), 1, "one packed warm send fills decide's cache")


@case
def r2_22_warm_raises_the_engines_word():
    with Backend() as backend:
        got = run(["SELECT thinkthen_warm('Is it a refund?', 'a')"], backend.base("arm/refuse"))
        expect(said(got[0]).split(":")[0], "thinkthen backend", "a refused warm")


@case
def usage_counts_differences_around_a_call():
    got = run_generic(
        [
            "SELECT value FROM thinkthen_usage() WHERE metric = 'requests_sent'",
            "SELECT thinkthen_decide('Is it a refund?', 'refund now')",
            "SELECT metric, value FROM thinkthen_usage()",
        ]
    )
    before = column(got[0])[0]
    after = dict(rows(got[2]))
    expect(sorted(after), ["cache_answers", "input_tokens", "output_tokens", "requests_sent"], "usage metrics")
    expect(after["requests_sent"] - before, 1, "requests_sent moves by one")


SET = '{"version": 1, "questions": {"refund": {"decide": "Is it a refund?"}}}'
NAMES = '{"version": 1, "recognize": {"kinds": {"person": "A name of a person."}}}'
CALLS = [
    "SELECT thinkthen_decide('Is it a refund?', 'a')",
    "SELECT thinkthen_probability('Is it a refund?', 'a')",
    "SELECT thinkthen_details('Is it a refund?', 'a')",
    "SELECT thinkthen_choose('Which?', 'a', ['x', 'y'])",
    "SELECT thinkthen_tag('Which?', 'a', ['x', 'y'])",
    f"SELECT thinkthen_annotate('{SET}', 'a')",
    "SELECT thinkthen_recognize('Maria Chen arrived.', ['person'])",
    f"SELECT thinkthen_relations('Maria Chen arrived.', '{NAMES}')",
    "SELECT thinkthen_warm('Is it a refund?', 'a')",
    "SELECT * FROM thinkthen_relate('SELECT 1 AS id, ''a'' AS name UNION ALL SELECT 2, ''b''', ['near'])",
]


@case
def secrecy_no_key_or_credential_in_any_message():
    """Every verb's refusal, every details member, a usage error, and a
    local error carry neither the key nor a password in the address."""
    sentinel = "sk-sentinel-4417"
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        extra = {"THINKTHEN_API_KEY": sentinel}
        refused = run(CALLS, backend.base("arm/refuse"), extra=extra)
        expect([said(result).split(":")[0] for result in refused], ["thinkthen backend"] * len(CALLS), "each refusal's kind")
        answered = run(
            [
                *CALLS,
                "SELECT thinkthen_details('Is it a refund?', 'refund now')",
                "SELECT thinkthen_decide('{\"decide\": \"Is it?\", \"threshold\": 2}', 'a')",
                f"SELECT thinkthen_decide('@{folder}/missing.json', 'a')",
                f"SELECT * FROM thinkthen_relate('SELECT 1 AS id, ''a'' AS name', '@{folder}/missing.json')",
                "SELECT * FROM thinkthen_relate('SELECT * FROM missing_table', ['near'])",
            ],
            backend.base(),
            extra=extra,
        )
        expect(["error" in result for result in answered], [False] * 11 + [True] * 4, "which calls failed")
        expect([said(result).split(":")[0] for result in answered[11:]], ["thinkthen usage", "thinkthen local", "thinkthen local", "thinkthen usage"], "the usage and local kinds")
        expect(backend.count() > 0, True, f"counted sends: {backend.count()}")
        address = run(CALLS[:1], "http://127.0.0.1:1/unused", extra={"THINKTHEN_API_KEY": sentinel, "THINKTHEN_BASE_URL": f"http://user:pw-sentinel-4417@127.0.0.1:{backend.port}/v1"})
        for result in [*refused, *answered, *address]:
            text = json.dumps(result)
            if "sentinel-4417" in text:
                raise AssertionError(f"a message carried a secret: {text}")


@case
def an_atfile_read_stops_at_one_mib():
    got = run_generic(["SELECT thinkthen_decide('@/dev/zero', 'a')"], timeout=30)
    expect(said(got[0]), "thinkthen local: the question file /dev/zero was not read: it holds more than 1 MiB", "an endless file")


@case
def atfile_reads_through_the_callers_file_system():
    with tempfile.TemporaryDirectory() as folder:
        path = Path(folder) / "q.json"
        path.write_text('{"decide": "Is it a refund?"}')
        got = run_generic(
            [
                f"SELECT thinkthen_decide('@{path}', 'refund now')",
                f"SELECT thinkthen_decide('@{folder}/missing.json', 'refund now')",
                "SET enable_external_access = false",
                f"SELECT thinkthen_decide('@{path}', 'refund now')",
            ]
        )
        expect(column(got[0]), [True], "a file question")
        expect(said(got[1]), f"thinkthen local: the question file {folder}/missing.json was not read: it does not exist or could not be opened", "a missing file")
        expect(said(got[3]), f"thinkthen local: the question file {path} was not read: this database's file settings refuse it", "a refused file")


@case
def b13c_try_details_members():
    """One reply has good/failed/good, followed by an independent good request."""
    with PackedReplies() as backend:
        got = run(["SET threads = 1", "SET thinkthen_batch = '3'",
                   "SELECT thinkthen_try_details('Is it a refund?', x) FROM "
                   "(VALUES (1,'alpha'),(2,'beta'),(3,'gamma'),(4,'delta')) t(i,x) ORDER BY i"], backend.base)
        values = [json.loads(value) for value in column(got[2])]
        expect([value["status"] for value in values], ["answered", "failed", "answered", "answered"], "packed row outcomes")
        expect(values[1]["error"], {"kind": "backend", "message": "the backend did not answer; retry if allowed",
                                      "retryable": False}, "one safe failed member")
        expect(len(backend.bodies), 2, "one packed and one independent attempt")
        expected = [
            b'{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"alpha\\". Is it a refund?"},"q2":{"type":"noul","instructions":"The text is \\"beta\\". Is it a refund?"},"q3":{"type":"noul","instructions":"The text is \\"gamma\\". Is it a refund?"}}}',
            b'{"state":"delta","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"Is it a refund?"}}}',
        ]
        expect(set(backend.bodies), set(expected), "full first-seen request bodies")
        digests = [hashlib.sha256(b"systemone\n" + backend.base.encode() + b"/systemone\n" + body).hexdigest()
                   for body in expected]
        for place, request in [(0, 0), (2, 0), (3, 1)]:
            expect(values[place]["details"]["meta"]["requests"], [digests[request]], "member request identity")


@case
def b13c_try_details_prepared():
    """An invalid foldable prepared question stays a safe value beside a good answer."""
    with Backend() as backend:
        got = run(["PREPARE b13c AS SELECT thinkthen_try_details('', 'private evidence') AS value "
                   "UNION ALL SELECT thinkthen_try_details('Is it a refund?', 'refund now')",
                   "EXECUTE b13c"], backend.base())
        expect(rows(got[0]), [], "prepared statement")
        values = [json.loads(value) for value, in rows(got[1])]
        expect([value["status"] for value in values], ["failed", "answered"], "prepared safe and good rows")
        expect(values[0]["error"]["kind"], "usage", "invalid foldable question kind")
        expect(backend.count(), 1, "good prepared sibling alone sends")


@case
def b13c_try_details_blank_context_keeps_good_siblings():
    """A bad literal context is one safe Usage row without a transport attempt."""
    with PackedReplies() as backend:
        got = run(["SET threads = 1",
                   "SELECT thinkthen_try_details('Is it a refund?', x, -1, c) FROM "
                   "(VALUES (1,'alpha','shared'),(2,'private evidence','   '),"
                   "(3,'gamma','shared'),(4,NULL,'   ')) t(i,x,c) ORDER BY i",
                   "SELECT thinkthen_details('Is it a refund?', 'ordinary', -1, '   ')"], backend.base)
        values = [json.loads(value) if value else None for value in column(got[1])]
        expect([value["status"] if value else None for value in values],
               ["answered", "failed", "answered", None], "context row outcomes and NULL skip")
        expect(values[1]["error"], {"kind": "usage", "message":
               "check the row's question and arguments, or raise the process request total when it is spent",
               "retryable": False}, "safe context Usage")
        expect("private evidence" in json.dumps(values[1]), False, "failed context row hides evidence")
        expect(said(got[2]), "thinkthen usage: context is text, not white space", "ordinary scalar still throws")
        expect(backend.bodies, [b'{"state":"shared","model":"jev-1.13.0","questions":'
                              b'{"q1":{"type":"noul","instructions":"The text is \\"alpha\\". Is it a refund?"},'
                              b'"q2":{"type":"noul","instructions":"The text is \\"gamma\\". Is it a refund?"}}}'],
               "only the good siblings share one exact attempt")


@case
def b13c_try_details_whole_request_failure():
    """A refused packed request fails only its own members and admits the next batch."""
    with PackedReplies(fail_first_pair=True) as backend:
        got = run(["SET threads = 1", "SET thinkthen_batch = '2'",
                   "SET thinkthen_max_retries = 0",
                   "SELECT thinkthen_try_details('Is it a refund?', x) FROM "
                   "(VALUES (1,'alpha'),(2,'beta'),(3,'gamma'),(4,'delta')) t(i,x) ORDER BY i"], backend.base)
        values = [json.loads(value) for value in column(got[3])]
        expect([value["status"] for value in values], ["failed", "failed", "answered", "answered"],
               "recoverable request leaves later batch alive")
        for value in values[:2]:
            expect(value["error"], {"kind": "backend", "message": "the backend did not answer; retry if allowed",
                                    "retryable": True}, "safe whole-request 503")
        expect(len(backend.bodies), 2, "no member isolation resend")


@case
def b13c_context_and_batch_one_wire_identity():
    """The final literal context packs two members; batch one retains bare legacy bodies."""
    with PackedReplies() as backend:
        got = run(["SET threads = 1", "SET thinkthen_batch = 'max'",
                   "SELECT thinkthen_details('Is it a refund?', x, -1, 'shared') "
                   "FROM (VALUES (1,'alpha'),(2,'beta')) t(i,x) ORDER BY i",
                   "SET thinkthen_batch = '1'",
                   "SELECT thinkthen_details('Is it a refund?', x) "
                   "FROM (VALUES (1,'gamma'),(2,'delta')) t(i,x) ORDER BY i"], backend.base)
        expect(len(column(got[2])), 2, "context vector rows")
        expect(len(column(got[4])), 2, "batch-one vector rows")
        expect(len(backend.bodies), 3, "one packed and two bare requests")
        expected = {
            b'{"state":"shared","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"alpha\\". Is it a refund?"},"q2":{"type":"noul","instructions":"The text is \\"beta\\". Is it a refund?"}}}',
            b'{"state":"gamma","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"Is it a refund?"}}}',
            b'{"state":"delta","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"Is it a refund?"}}}',
        }
        expect(set(backend.bodies), expected, "context and batch-one full request bytes")
        details = [json.loads(value) for value in column(got[2])]
        expect(["input" in value for value in details], [False, False], "scalar details omit record input")
        expect(len({value["meta"]["context_sha256"] for value in details}), 1, "one context identity")


@case
def b13c_warm_first_seen_context():
    """One warm group sends its distinct texts in first-seen order with its literal context."""
    with PackedReplies(missing_second=False) as backend:
        got = run(["SET threads = 1",
                   "SELECT thinkthen_warm('Is it a refund?', x, 'shared') "
                   "FROM (VALUES (1,'zeta'),(2,'alpha'),(3,'zeta'),(4,'beta')) t(i,x)"], backend.base)
        expect(column(got[1]), [3], "warm distinct count")
        expect(len(backend.bodies), 1, "one warm request")
        expected = b'{"state":"shared","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"zeta\\". Is it a refund?"},"q2":{"type":"noul","instructions":"The text is \\"alpha\\". Is it a refund?"},"q3":{"type":"noul","instructions":"The text is \\"beta\\". Is it a refund?"}}}'
        expect(backend.bodies[0], expected, "first-seen warm body")


@case
def b13c_packed_total_admits_one_attempt():
    """The process total counts packed sends, independent of SQL row count or arrival order."""
    query = ("SELECT thinkthen_decide('Is it a refund?', x) FROM "
             "(VALUES (1,'alpha'),(2,'beta'),(3,'gamma'),(4,'delta')) t(i,x) ORDER BY i")
    with PackedReplies() as backend:
        got = run(["SET threads = 1", "SET thinkthen_batch = '2'", query], backend.base)
        expect(column(got[2]), [True, True, True, True], "four packed answers")
        expect(set(backend.bodies), PACKED_PAIR_BODIES, "both independently pinned packed bodies")
    with PackedReplies() as backend:
        got = run(["SET threads = 1", "SET thinkthen_batch = '2'",
                   "SET thinkthen_max_requests_total = 1", query], backend.base)
        expect(said(got[3]), "thinkthen usage: this process has spent its request total of 1; raise SET thinkthen_max_requests_total or RESET it", "spent total")
        expect(len(backend.bodies), 1, "only one actual attempt is admitted")
        expect(backend.bodies[0] in PACKED_PAIR_BODIES, True, "either packed request may arrive first")


@case
def b13c_try_details_total_one_preserves_answered_rows():
    """A spent total retains the packed answer at its original SQL positions."""
    query = ("SELECT thinkthen_try_details('Is it a refund?', x) FROM "
             "(VALUES (1,'alpha'),(2,'beta'),(3,'gamma'),(4,'delta'),(5,NULL)) t(i,x) ORDER BY i")
    with PackedReplies() as backend:
        got = run(["SET threads = 1", "SET thinkthen_batch = '2'",
                   "SET thinkthen_max_requests_total = 1", query], backend.base)
        expect(len(backend.bodies), 1, "one real request was admitted")
        expect(backend.bodies[0] in PACKED_PAIR_BODIES, True, "the sole send is a complete packed request")
        values = [json.loads(value) if value is not None else None for value in column(got[3])]
        answered = [0, 1] if b'alpha' in backend.bodies[0] else [2, 3]
        failed = [place for place in range(4) if place not in answered]
        expect(values[4], None, "NULL text remains SQL NULL")
        digest = hashlib.sha256(b"systemone\n" + backend.base.encode() + b"/systemone\n" + backend.bodies[0]).hexdigest()
        for place in answered:
            expect(values[place]["status"], "answered", f"answered SQL slot {place}")
            expect(values[place]["details"]["meta"]["requests"], [digest], f"answer identity {place}")
        for place in failed:
            expect(values[place], {"status": "failed", "error": {"kind": "usage", "message":
                   "check the row's question and arguments, or raise the process request total when it is spent",
                   "retryable": False}}, f"safe denied SQL slot {place}")


if __name__ == "__main__":
    sys.exit(main())
