#!/usr/bin/env python3
"""`thinkthen_rank` over one keyed object: exact rows, ties, packing, settings
and refusals, each counted at the loopback backend (ticket 0378)."""

from __future__ import annotations

import json
import sys

from helper import Backend, child, environment, expect, main

QUESTION = "Is it a refund?"
SEVEN = json.dumps({f"k{place}": f"record {place}" for place in range(7)})
RANK = "SELECT key, rank, probability FROM thinkthen_rank(?, ?, ?)"


def ranked(backend: Backend, arm: str, calls: dict[str, tuple[str, tuple]]) -> dict:
    fields = ", ".join(f"{name}=run(db, {sql!r}, {parameters!r})" for name, (sql, parameters) in calls.items())
    return child(f"db = connect()\nsay({fields})\n", environment(backend, arm))


def test_rank_orders_recorded_probabilities_exactly() -> None:
    records = json.dumps({"0": "item 0", "1": "item 1", "2": "item 2"})
    backend = Backend()
    held = ranked(backend, "case/15-rank-records", {
        "rows": (RANK, ("How relevant is this?", records, '{"batch":1}')),
        "types": ("SELECT typeof(key), typeof(rank), typeof(probability) FROM thinkthen_rank(?, ?, ?) LIMIT 1",
                  ("How relevant is this?", records, '{"batch":1}')),
    })
    expect(held["rows"], [["1", 1, 0.9], ["2", 2, 0.5], ["0", 3, 0.2]], "best first with exact probabilities")
    expect(held["types"], [["text", "integer", "real"]], "column types")
    expect(backend.close(), 3, "one recorded request per record, then the connection's rows")


def test_ties_keep_keyed_member_order() -> None:
    backend = Backend()
    held = ranked(backend, "generic", {"rows": (RANK, (QUESTION, '{"z":"one","a":"two","m":"three"}', "{}"))})
    expect(held["rows"], [["z", 1, 0.9], ["a", 2, 0.9], ["m", 3, 0.9]], "member order, not key order")
    expect(backend.close(), 1, "one packed request")


def test_batching_packs_one_call_by_the_setting() -> None:
    for settings, sent in (("{}", 1), ('{"batch":3}', 3), ('{"batch":1}', 7)):
        backend = Backend()
        held = ranked(backend, "generic", {
            "rows": ("SELECT count(*), max(rank) FROM thinkthen_rank(?, ?, ?)", (QUESTION, SEVEN, settings)),
        })
        expect(held["rows"], [[7, 7]], f"seven ranked rows at {settings}")
        expect(backend.close(), sent, f"requests at {settings}")


def test_a_join_reads_one_rank_call() -> None:
    backend = Backend()
    held = ranked(backend, "generic", {
        "rows": ("WITH t(id) AS (VALUES ('k0'), ('k1'), ('k2'), ('k3'), ('k4'), ('k5'), ('k6')) "
                 "SELECT t.id, r.rank FROM t JOIN thinkthen_rank(?, ?) r ON r.key = t.id ORDER BY r.rank",
                 (QUESTION, SEVEN)),
        "probe": ("SELECT key, rank FROM thinkthen_rank(?, ?) WHERE lookup_key = 'k3'", (QUESTION, SEVEN)),
    })
    expect(held["rows"], [[f"k{place}", place + 1] for place in range(7)], "joined ranks")
    expect(held["probe"], [["k3", 4]], "the one probed row")
    expect(backend.close(), 1, "one packed request for the join and the probe")


def test_the_question_is_literal_text() -> None:
    backend = Backend()
    held = ranked(backend, "arm/full/capture", {"rows": (RANK, ("@nofile", '{"a":"refund now"}', "{}"))})
    expect(held["rows"], [["a", 1, 0.9]], "an @ question answers")
    bodies = backend.capture()
    expect(len(bodies), 1, "one request")
    instructions = json.loads(bodies[0])["questions"]["q1"]["instructions"]
    expect(instructions, 'The text is "refund now". @nofile', "the @ text is the question")
    backend.close()


def test_empty_and_null_inputs() -> None:
    backend = Backend()
    held = ranked(backend, "generic", {
        "empty": ("SELECT count(*) FROM thinkthen_rank(?, '{}')", (QUESTION,)),
        "question": ("SELECT count(*) FROM thinkthen_rank(NULL, ?)", ('{"a":"b"}',)),
        "records": ("SELECT count(*) FROM thinkthen_rank(?, NULL)", (QUESTION,)),
    })
    expect(held["empty"], [[0]], "an empty object ranks nothing")
    expect(held["question"], "thinkthen usage: the question is required (retryable: no)", "a NULL question")
    expect(held["records"], "thinkthen usage: the keyed records is required (retryable: no)", "NULL keyed records")
    expect(backend.close(), 0, "nothing sent")


def test_refusals_come_before_any_send() -> None:
    one = '{"a":"x"}'
    blank = json.dumps({f"k{place}": f"record {place}" for place in range(7)} | {"k3": "  "})
    cases = [
        ((QUESTION, one, '{"threshold":0.7}'), "thinkthen usage: the settings key `threshold` does not belong to this verb (retryable: no)"),
        ((QUESTION, one, '{"true":"yes"}'), "thinkthen usage: the settings key `true` does not belong to this verb (retryable: no)"),
        ((QUESTION, one, '{"none":true}'), "thinkthen usage: the settings key `none` does not belong to this verb (retryable: no)"),
        ((QUESTION, one, '{"options":["a","b"]}'), "thinkthen usage: the settings key `options` does not belong to this verb (retryable: no)"),
        (("   ", one, "{}"), "thinkthen usage: a question is text, not white space (retryable: no)"),
        ((QUESTION, '{"a":"x","a":"y"}', "{}"), "thinkthen usage: keyed records are invalid: a keyed record name is repeated at line 1 column 17 (retryable: no)"),
        ((QUESTION, '{"a":4}', "{}"), "thinkthen usage: keyed records are invalid: each keyed record is nonblank text at line 1 column 7 (retryable: no)"),
        ((QUESTION, blank, "{}"), "thinkthen usage: keyed records are invalid: each keyed record is nonblank text at line 1 column 65 (retryable: no)"),
        ((QUESTION, one, '{"deadline_ms":0}'), "thinkthen deadline: the deadline of 0 s passed before the call answered (retryable: no)"),
    ]
    backend = Backend()
    held = ranked(backend, "generic", {f"c{at}": (RANK, arguments) for at, (arguments, _) in enumerate(cases)})
    expect([held[f"c{at}"] for at in range(len(cases))], [sentence for _, sentence in cases], "whole refusals")
    expect(backend.close(), 0, "no refusal sends")


def test_model_and_context_reach_the_request() -> None:
    backend = Backend()
    held = ranked(backend, "arm/full/capture", {
        "rows": ("SELECT key, rank FROM thinkthen_rank(?, ?, ?)",
                 (QUESTION, '{"a":"one","b":"two"}', '{"model":"judge-b","context":"shared note"}')),
    })
    expect(held["rows"], [["a", 1], ["b", 2]], "two ranked rows")
    bodies = backend.capture()
    expect(len(bodies), 1, "one packed request")
    expect(json.loads(bodies[0])["model"], "judge-b", "the settings model")
    expect(bodies[0].count("shared note"), 1, "the context once per request")
    backend.close()


if __name__ == "__main__":
    sys.exit(main(globals()))
