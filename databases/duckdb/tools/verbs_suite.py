"""The verbs through the real engine on a loopback backend: row mapping,
NULL rows, results, details, recognize, deadlines, warm, usage, and secrecy.

Each case starts its own backend and runs its SQL in a fresh child.
"""

from __future__ import annotations

import json
import sys
import tempfile
from pathlib import Path

from harness import Backend, case, expect, main, rows, run, said

REFUND = "Does the writer ask for a refund?"
SHUFFLED = "(VALUES (1, 'good morning'), (2, NULL), (3, 'refund now'), (4, 'good morning'), (5, NULL), (6, 'refund now')) t(i, x)"


def column(result: dict) -> list:
    return [row[0] for row in rows(result)]


@case
def r1_1_answers_map_back_by_text():
    with Backend() as backend:
        got = run(
            [
                f"SELECT thinkthen_decide('{REFUND}', x) FROM {SHUFFLED} ORDER BY i",
                f"SELECT thinkthen_probability('{REFUND}', x) FROM {SHUFFLED} ORDER BY i",
            ],
            backend.base("case/28-decide-many-repeated-texts"),
        )
        expect(column(got[0]), [False, None, True, False, None, True], "decide rows")
        expect(column(got[1]), [0.03, None, 0.97, 0.03, None, 0.97], "probability rows")
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
def r3_11_details_read_null_for_an_absent_member():
    got = run_generic(
        [
            "SELECT d.probability, d.answer, d.value, d.nearest, d.requests_sent FROM (SELECT thinkthen_details('{\"score\": \"How strong?\", \"levels\": [\"weak\", \"strong\"]}', 'claim') AS d)",
            "SELECT d.probability, d.answer, d.value FROM (SELECT thinkthen_details('Is it a refund?', 'refund now') AS d)",
        ]
    )
    probability, answer, value, nearest, sent = rows(got[0])[0]
    expect((probability, answer), (None, None), "score details carry no yes probability or answer word")
    expect(nearest, "weak", "score details name the nearest level")
    expect(sent, 1, "requests_sent")
    expect(json.loads(value) is not None, True, "score details fill value")
    expect(tuple(rows(got[1])[0]), (0.9, "yes", '"yes"'), "decide details")
    filled = run_generic(
        [
            "SELECT (thinkthen_details('{\"choose\": \"Which team?\", \"options\": [\"billing\", \"shipping\"]}', 'charged twice')).value",
            "SELECT (thinkthen_details('{\"tag\": \"Which topics?\", \"labels\": [\"billing\", \"shipping\"]}', 'charged twice')).value",
        ]
    )
    expect([column(result) for result in filled], [['"billing"'], ['["billing","shipping"]']], "choose and tag details fill value")


@case
def probability_equals_details_with_no_added_send():
    with Backend() as backend:
        got = run(
            [
                "SELECT thinkthen_probability('Is it a refund?', 'refund now')",
                "SELECT (thinkthen_details('Is it a refund?', 'refund now')).probability",
            ],
            backend.base(),
        )
        expect(column(got[0]), column(got[1]), "probability and details")
        expect(backend.count(), 1, "one counted send for both")


@case
def case_41_offsets_count_code_points():
    text = "Le café 😀 Maria Chen arrived."
    got = run_generic([f"SELECT r.name, r.start, r.\"end\" FROM (SELECT unnest(thinkthen_recognize('{text}', ['person'])) AS r)"])
    # The generic arm names the whole text. Its end is 29 in code points
    # and would be 33 in UTF-8 bytes.
    expect(rows(got[0]), [[text, 0, 29]], "names and code-point offsets")
    for name, start, end in rows(got[0]):
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
    with Backend() as backend:
        got = run(
            [
                "SELECT thinkthen_warm(q, x) FROM (VALUES ('Is it a refund?', 'a'), ('Is it late?', 'b')) t(q, x)",
                "SELECT thinkthen_warm('@q.json', 'a')",
            ],
            backend.base(),
        )
        expect(said(got[0]), "thinkthen usage: thinkthen_warm judges one question per group, and this group carries more than one", "two questions")
        expect(
            said(got[1]),
            "thinkthen usage: thinkthen_warm reads no '@file' question; pass the file's text from DuckDB's read_text, which applies this database's file settings",
            "warm and @file",
        )
        expect(backend.count(), 0, "counted sends")
        warmed = run(["SELECT thinkthen_warm('Is it a refund?', x) FROM (VALUES ('a'), ('b'), ('a')) t(x)"], backend.base())
        expect(column(warmed[0]), [2], "warm counts distinct texts")


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
                "SELECT d.* FROM (SELECT thinkthen_details('Is it a refund?', 'refund now') AS d)",
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


if __name__ == "__main__":
    sys.exit(main())
