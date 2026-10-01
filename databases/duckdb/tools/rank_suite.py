"""`thinkthen_rank` over one keyed object: exact rows, ties, packing, settings
and refusals, each counted at the loopback backend (ticket 0378)."""

from __future__ import annotations

import json

from harness import Backend, case, expect, main, rows, run, said

QUESTION = "'Is it a refund?'"
SEVEN = json.dumps({f"k{place}": f"record {place}" for place in range(7)})


def keyed(records: dict[str, str]) -> str:
    return "'" + json.dumps(records).replace("'", "''") + "'"


def ranked(question: str, records: str, settings: str = "") -> str:
    tail = f", '{settings}'" if settings else ""
    return f"SELECT key, rank, probability FROM thinkthen_rank({question}, {records}{tail})"


@case
def rank_orders_recorded_probabilities_exactly():
    records = keyed({"0": "item 0", "1": "item 1", "2": "item 2"})
    with Backend() as backend:
        got = run([
            ranked("'How relevant is this?'", records, '{"batch":1}'),
            f"SELECT typeof(key), typeof(rank), typeof(probability) FROM thinkthen_rank('How relevant is this?', {records}, '{{\"batch\":1}}') LIMIT 1",
        ], backend.base("case/15-rank-records"))
        expect(rows(got[0]), [["1", 1, 0.9], ["2", 2, 0.5], ["0", 3, 0.2]], "best first with exact probabilities")
        expect(rows(got[1]), [["VARCHAR", "BIGINT", "DOUBLE"]], "column types")
        expect(backend.count(), 3, "one recorded request per record, then the cache")


@case
def ties_keep_keyed_member_order():
    with Backend() as backend:
        got = run([ranked(QUESTION, keyed({"z": "one", "a": "two", "m": "three"}))], backend.base())
        expect(rows(got[0]), [["z", 1, 0.9], ["a", 2, 0.9], ["m", 3, 0.9]], "member order, not key order")
        expect(backend.count(), 1, "one packed request")


@case
def batching_packs_one_call_by_the_setting():
    for settings, sent in (("", 1), ('{"batch":3}', 3), ('{"batch":1}', 7)):
        with Backend() as backend:
            got = run([f"SELECT count(*), max(rank) FROM thinkthen_rank({QUESTION}, '{SEVEN}'{', ' + repr(settings) if settings else ''})"],
                      backend.base())
            expect(rows(got[0]), [[7, 7]], f"seven ranked rows at {settings or 'the default'}")
            expect(backend.count(), sent, f"requests at {settings or 'the default'}")


@case
def a_join_reads_one_rank_call():
    table = "(VALUES ('k0'), ('k3'), ('k6')) t(id)"
    with Backend() as backend:
        got = run([f"SELECT t.id, r.rank FROM {table} JOIN thinkthen_rank({QUESTION}, '{SEVEN}') r ON r.key = t.id ORDER BY r.rank"],
                  backend.base())
        expect(rows(got[0]), [["k0", 1], ["k3", 4], ["k6", 7]], "joined ranks")
        expect(backend.count(), 1, "one packed request for the join")


@case
def the_readme_join_builds_its_object_from_rows():
    tickets = "(VALUES (1, 'refund please'), (2, 'thanks'), (3, 'money back')) tickets(id, body)"
    with Backend() as backend:
        got = run([f"WITH t AS (SELECT * FROM {tickets}) SELECT t.id, refund.rank FROM t "
                   "JOIN thinkthen_rank('Does the writer ask for a refund?', (SELECT json_group_object(id, body) FROM t)) refund "
                   "ON refund.key = CAST(t.id AS VARCHAR) ORDER BY refund.rank"], backend.base())
        expect(rows(got[0]), [[1, 1], [2, 2], [3, 3]], "ranks from a json_group_object")
        expect(backend.count(), 1, "one packed request")


@case
def the_question_is_literal_text():
    with Backend() as backend:
        got = run([ranked("'@nofile'", keyed({"a": "refund now"}))], backend.base("arm/full/capture"))
        expect(rows(got[0]), [["a", 1, 0.9]], "an @ question answers")
        bodies = backend.capture()
        expect(len(bodies), 1, "one request")
        instructions = json.loads(bodies[0])["questions"]["q1"]["instructions"]
        expect(instructions, 'The text is "refund now". @nofile', "the @ text is the question")


@case
def empty_and_null_inputs_send_nothing():
    with Backend() as backend:
        got = run([
            f"SELECT count(*) FROM thinkthen_rank({QUESTION}, '{{}}')",
            "SELECT count(*) FROM thinkthen_rank(NULL, '{\"a\":\"b\"}')",
            f"SELECT count(*) FROM thinkthen_rank({QUESTION}, NULL)",
        ], backend.base())
        expect([rows(result) for result in got], [[[0]], [[0]], [[0]]], "no rows")
        expect(backend.count(), 0, "nothing sent")


@case
def refusals_come_before_any_send():
    blank = {f"k{place}": f"record {place}" for place in range(7)} | {"k3": "  "}
    cases = [
        (ranked(QUESTION, keyed({"a": "x"}), '{"threshold":0.7}'),
         "thinkthen usage: the settings key `threshold` does not belong to this verb (retryable: no)"),
        (ranked(QUESTION, keyed({"a": "x"}), '{"true":"yes"}'),
         "thinkthen usage: the settings key `true` does not belong to this verb (retryable: no)"),
        (ranked(QUESTION, keyed({"a": "x"}), '{"none":true}'),
         "thinkthen usage: the settings key `none` does not belong to this verb (retryable: no)"),
        (ranked(QUESTION, keyed({"a": "x"}), '{"options":["a","b"]}'),
         "thinkthen usage: the settings key `options` does not belong to this verb (retryable: no)"),
        (ranked("'   '", keyed({"a": "x"})), "thinkthen usage: a question is text, not white space (retryable: no)"),
        (ranked(QUESTION, "'{\"a\":\"x\",\"a\":\"y\"}'"),
         "thinkthen usage: a keyed input repeats a key at line 1 column 17 (retryable: no)"),
        (ranked(QUESTION, "'{\"a\":4}'"), "thinkthen usage: each keyed input value is text at line 1 column 7 (retryable: no)"),
        (ranked(QUESTION, keyed(blank)), "thinkthen usage: evidence is text, not white space (retryable: no)"),
        (ranked(QUESTION, keyed({"a": "x"}), '{"deadline_ms":0}'),
         "thinkthen deadline: the deadline of 0 s passed before the call answered (retryable: no)"),
    ]
    with Backend() as backend:
        got = run([statement for statement, _ in cases], backend.base())
        expect([said(result) for result in got], [sentence for _, sentence in cases], "whole refusals")
        expect(backend.count(), 0, "no refusal sends")


@case
def model_and_context_reach_the_request():
    with Backend() as backend:
        got = run([ranked(QUESTION, keyed({"a": "one", "b": "two"}), '{"model":"judge-b","context":"shared note"}')],
                  backend.base("arm/full/capture"))
        expect([row[:2] for row in rows(got[0])], [["a", 1], ["b", 2]], "two ranked rows")
        bodies = backend.capture()
        expect(len(bodies), 1, "one packed request")
        body = bodies[0]
        expect(json.loads(body)["model"], "judge-b", "the settings model")
        expect(body.count("shared note"), 1, "the context once per request")


if __name__ == "__main__":
    raise SystemExit(main())
