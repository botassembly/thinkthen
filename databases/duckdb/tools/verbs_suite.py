"""The verbs through the real engine on a loopback backend: row mapping,
NULL rows, results, details, recognize, deadlines, warm, usage, and secrecy.

Each case starts its own backend and runs its SQL in a fresh child.
"""

from __future__ import annotations

import json
import sys
import tempfile
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / "sqlite" / "tests"))
from conditional_backend import ConditionalBackend

from harness import Backend, case, expect, main, rows, run, said
from harness import native_reading
from verbs_budget import (
    PACKED_PAIR_BODIES, PackedReplies,
)
from verbs_complete import native_complete_file_authorization_precedes_content_parsing, complete_question_files_keep_identity, complete_question_refusals_and_nulls, complete_question_rechecks_prepared_authority
from verbs_portable import portable_batch_identity
from question_keys import question_keys

REFUND = "Does the writer ask for a refund?"
SHUFFLED = "(VALUES (1, 'good morning'), (2, NULL), (3, 'refund now'), (4, 'good morning'), (5, NULL), (6, 'refund now')) t(i, x)"

case(native_complete_file_authorization_precedes_content_parsing)
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
            ],
            backend.base("case/28-decide-many-repeated-texts"),
        )
        expect(column(got[1]), [False, None, True, False, None, True], "decide rows")
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
    expect(column(got[4]), [True], "a NULL optional settings object uses defaults")
    expect(said(got[5]), "thinkthen local: the question file q.json was not read: it does not exist or could not be opened (retryable: no)",
           "choose reads the shared @file question form and refuses a missing file")


@case
def rank_probability_equals_details_with_no_added_send():
    with Backend() as backend:
        got = run(
            [
                "SELECT probability FROM thinkthen_rank('Is it a refund?', '{\"a\":\"refund now\"}')",
                "SELECT (thinkthen_decide_details('Is it a refund?', 'refund now')).result.answer.probability",
            ],
            backend.base(),
        )
        expect(column(got[0]), column(got[1]), "rank probability and details")
        expect(backend.count(), 1, "one counted send for both")












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
                "SELECT thinkthen_decide('Is it a refund?', 'a', '{\"deadline_ms\":0}')",
                "SELECT thinkthen_decide('Is it a refund?', 'b', '{\"deadline_ms\":-2}')",
                "SELECT thinkthen_decide('Is it a refund?', 'c', '{\"deadline_ms\":4294967296000}')",
                "SELECT thinkthen_decide('Is it a refund?', 'd', '{\"deadline_ms\":9223372036854775807}')",
            ],
            backend.base(),
        )
        expect(said(got[0]).split(":")[0], "thinkthen deadline", "a zero deadline")
        for result in got[1:]:
            expect(said(result).split(":")[0], "thinkthen usage", "an out-of-range deadline")
        expect(backend.count(), 0, "counted sends")
        answered = run(["SELECT thinkthen_decide('Is it a refund?', 'e', '{\"deadline_ms\":-1}')"], backend.base())
        expect(column(answered[0]), [True], "-1 runs with no deadline")


@case
def a_banded_file_and_repeat_decide_share_one_cache_answer():
    """A banded file remains valid and a repeated vector call uses its cache entry."""
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        banded = Path(folder) / "banded.json"
        banded.write_text(json.dumps({"decide": "Is it red?", "true": "Red paint.", "false": "Any other colour.", "model": "judge-b", "threshold": "0.85:0.95"}))
        texts = "(VALUES ('a red door'), ('a blue door'), ('a red door')) t(x)"
        query = f"SELECT thinkthen_decide('@{banded}', x) FROM {texts}"
        got = run([query, query], backend.base())
        expect(column(got[0]), [None, None, None], "the band leaves each row unsure")
        expect(column(got[1]), [None, None, None], "repeat reads the same banded answers")
        expect(backend.count(), 1, "one packed send fills the repeat cache")


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
    "SELECT * FROM thinkthen_rank('Is it a refund?', '{\"r\":\"a\"}')",
    "SELECT thinkthen_decide_details('Is it a refund?', 'a')",
    "SELECT thinkthen_choose('Which?', 'a', ['x', 'y'])",
    "SELECT thinkthen_tag('Which?', 'a', ['x', 'y'])",
    f"SELECT thinkthen_annotate('{SET}', 'a')",
    "SELECT thinkthen_recognize('Maria Chen arrived.', ['person'])",
    f"SELECT thinkthen_relations('Maria Chen arrived.', '{NAMES}')",
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
        expect([rows(result)[0][0]["failure"]["kind"] if index == 2 else said(result).split(":")[0].removeprefix("thinkthen ")
                for index, result in enumerate(refused)], ["backend"] * len(CALLS), "each refusal's kind")
        answered = run(
            [
                *CALLS,
                "SELECT thinkthen_decide_details('Is it a refund?', 'refund now')",
                "SELECT thinkthen_decide('{\"decide\": \"Is it?\", \"threshold\": 2}', 'a')",
                f"SELECT thinkthen_decide('@{folder}/missing.json', 'a')",
                f"SELECT * FROM thinkthen_relate('SELECT 1 AS id, ''a'' AS name', '@{folder}/missing.json')",
                "SELECT * FROM thinkthen_relate('SELECT * FROM missing_table', ['near'])",
            ],
            backend.base(),
            extra=extra,
        )
        expect(["error" in result for result in answered], [False] * 10 + [True] * 4, "which calls failed")
        expect([said(result).split(":")[0] for result in answered[10:]], ["thinkthen usage", "thinkthen local", "thinkthen local", "thinkthen usage"], "the usage and local kinds")
        expect(backend.count() > 0, True, f"counted sends: {backend.count()}")
        address = run(CALLS[:1], "http://127.0.0.1:1/unused", extra={"THINKTHEN_API_KEY": sentinel, "THINKTHEN_BASE_URL": f"http://user:pw-sentinel-4417@127.0.0.1:{backend.port}/v1"})
        for result in [*refused, *answered, *address]:
            text = json.dumps(result)
            if "sentinel-4417" in text:
                raise AssertionError(f"a message carried a secret: {text}")


@case
def an_atfile_read_stops_at_one_mib():
    got = run_generic(["SELECT thinkthen_decide('@/dev/zero', 'a')"], timeout=30)
    expect(said(got[0]), "thinkthen local: the question file /dev/zero was not read: it holds more than 1 MiB (retryable: no)", "an endless file")


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
        expect(said(got[1]), f"thinkthen local: the question file {folder}/missing.json was not read: it does not exist or could not be opened (retryable: no)", "a missing file")
        expect(said(got[3]), f"thinkthen local: the question file {path} was not read: this database's file settings refuse it (retryable: no)", "a refused file")










@case
def b13c_context_and_batch_one_wire_identity():
    """The final literal context packs two members; batch one retains bare legacy bodies."""
    with PackedReplies() as backend:
        got = run(["SET threads = 1", "SET thinkthen_batch = 'max'",
                   "SELECT thinkthen_decide_details('Is it a refund?', x, '{\"context\":\"shared\"}') "
                   "FROM (VALUES (1,'alpha'),(2,'beta')) t(i,x) ORDER BY i",
                   "SET thinkthen_batch = '1'",
                   "SELECT thinkthen_decide_details('Is it a refund?', x) "
                   "FROM (VALUES (1,'gamma'),(2,'delta')) t(i,x) ORDER BY i"], backend.base)
        expect(len(column(got[2])), 2, "context vector rows")
        expect(len(column(got[4])), 2, "batch-one vector rows")
        expect(len(backend.bodies), 3, "one packed and two bare requests")
        expected = {
            b'{"state":"shared","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"alpha\\". Is it a refund?"},"q2":{"type":"noul","instructions":"The text is \\"beta\\". Is it a refund?"}}}',
            b'{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"gamma\\". Is it a refund?"}}}',
            b'{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"delta\\". Is it a refund?"}}}',
        }
        expect(set(backend.bodies), expected, "context and batch-one full request bytes")
        details = [native_reading(value) for value in column(got[2])]
        expect([value["input"] for value in details], ['"alpha"', '"beta"'], "typed details retain actual record originals")
        expect(len({value["meta"]["context_sha256"] for value in details}), 1, "one context identity")


@case
def portable_vector_first_seen_context():
    """One vector sends distinct texts in first-seen order with its literal context."""
    with PackedReplies(missing_second=False) as backend:
        got = run(["SET threads = 1",
                   "SELECT thinkthen_decide('Is it a refund?', x, '{\"context\":\"shared\"}') "
                   "FROM (VALUES (1,'zeta'),(2,'alpha'),(3,'zeta'),(4,'beta')) t(i,x)"], backend.base)
        expect(column(got[1]), [True, True, True, True], "every vector row answered")
        expect(len(backend.bodies), 1, "one packed request")
        expected = b'{"state":"shared","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"zeta\\". Is it a refund?"},"q2":{"type":"noul","instructions":"The text is \\"alpha\\". Is it a refund?"},"q3":{"type":"noul","instructions":"The text is \\"beta\\". Is it a refund?"}}}'
        expect(backend.bodies[0], expected, "first-seen vector body")


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
        expect(said(got[3]), "thinkthen usage: this process has spent its request total of 1; raise SET thinkthen_max_requests_total or RESET it (retryable: no)", "spent total")
        expect(len(backend.bodies), 1, "only one actual attempt is admitted")
        expect(backend.bodies[0] in PACKED_PAIR_BODIES, True, "either packed request may arrive first")




if __name__ == "__main__":
    sys.exit(main())
