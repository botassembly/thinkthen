"""Selected installed SQL find boundaries on the existing counted loopback harness."""

from __future__ import annotations

import json
import sys

from harness import Backend, case, expect, main, rows, run, said
from signal_suite import held_cancel
from verbs_budget import PackedReplies


@case
def original_duplicate_and_ties() -> None:
    """The second duplicate can win; equal real leaders favor first, none ties favor none."""
    cases = (
        ("['same','same','other']", False, {"u001": 0.1, "u002": 0.8, "u003": 0.1},
         {"index": 1, "value": "same", "probability": 0.8,
          "candidates": [{"index": 0, "probability": 0.1}, {"index": 1, "probability": 0.8},
                         {"index": 2, "probability": 0.1}]}),
        ("['first','second']", False, {"u001": 0.5, "u002": 0.5},
         {"index": 0, "value": "first", "probability": 0.5,
          "candidates": [{"index": 0, "probability": 0.5}, {"index": 1, "probability": 0.5}]}),
        ("['first','second']", True, {"u001": 0.4, "u002": 0.2, "none": 0.4},
         {"index": None, "value": None, "probability": 0.4,
          "candidates": [{"index": 0, "probability": 0.4}, {"index": 1, "probability": 0.2},
                         {"index": None, "probability": 0.4}]}),
    )
    for slot, (units, none, probabilities, wanted) in enumerate(cases):
        reply = json.dumps({"model": "jev-latest", "answers": {
            "q1": {"type": "choice", "probabilities": probabilities}}}).encode()

        def respond(_body, _questions):
            return 200, {"Content-Type": "application/json"}, reply

        with PackedReplies(responder=respond) as backend:
            arguments = f"'Which unit?', {units}"
            if slot == 1:
                arguments += ", '{\"none\":false}'"
            elif slot == 2:
                arguments += ", '{\"none\":true,\"deadline_ms\":-1}'"
            sql = f"SELECT thinkthen_find({arguments})"
            expect(rows(run([sql], backend.base)[0]), [[wanted]], "literal original-index result")
            expect(len(backend.bodies), 1, "one complete find request")


@case
def null_empty_and_invalid_units_do_not_send() -> None:
    """A bad later row is found before this chunk's first transport."""
    nulls = (
        "SELECT thinkthen_find(NULL, ['a','b'])",
        "SELECT thinkthen_find('Which?', NULL::VARCHAR[])",
        "SELECT thinkthen_find('Which?', []::VARCHAR[])",
    )
    invalid = (
        "SELECT thinkthen_find('Which?', ['one'])",
        "SELECT thinkthen_find('Which?', ['one', NULL])",
        "SELECT thinkthen_find('Which?', ['one', '  '])",
        "SELECT thinkthen_find('   ', ['one','two'])",
        "SELECT thinkthen_find('Which?', list_transform(range(256), x -> 'x'))",
        "SELECT thinkthen_find('Which?', list_transform(range(255), x -> 'x'), '{\"none\":true}')",
        "SELECT thinkthen_find('Which?', [repeat('x', 16777216), 'y'])",
        "SELECT thinkthen_find('Which?', ['a','b'], '{\"deadline_ms\":-2}')",
        "SELECT thinkthen_find('Which?', units) FROM (VALUES (0, ['a','b']), (1, ['a',NULL])) t(i,units) ORDER BY i",
    )
    with Backend() as backend:
        got = run([*nulls, *invalid], backend.base())
        for sql, result in zip(nulls, got[:len(nulls)], strict=True):
            expect(rows(result), [[None]], sql)
        for sql, result in zip(invalid, got[len(nulls):], strict=True):
            expect(said(result).startswith("thinkthen usage: "), True, sql)
        expect(backend.count(), 0, "NULL, empty and invalid groups send nothing")
        shape = rows(run(["SELECT typeof(thinkthen_find('Which?', ['a','b']))"], backend.base())[0])
        expect(shape, [["STRUCT(\"index\" BIGINT, \"value\" VARCHAR, probability DOUBLE, candidates STRUCT(\"index\" BIGINT, probability DOUBLE)[])"]],
               "typed find result")


@case
def portable_find_settings_and_removed_slots() -> None:
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_find(settings := '{\"none\":true}', units := ['first','second'], question := 'Which unit?')",
            "SELECT thinkthen_find('Which unit?', ['first','second'], TRUE)",
            "SELECT thinkthen_find('Which unit?', ['first','second'], FALSE, 1)",
            "SELECT thinkthen_find('Which unit?', ['first','second'], '{\"unknwon\":1}')",
        ], backend.base())
        expect(rows(got[0])[0][0]["value"], "first", "named find settings")
        for result in got[1:3]:
            expect(said(result), "thinkthen usage: find's none and deadline moved into the settings object",
                   "removed find slots")
        expect(said(got[3]).startswith("thinkthen usage:"), True, "shared settings refusal")
        expect(backend.count(), 1, "invalid find calls send nothing")


@case
def held_find_and_spent_statement_budget() -> None:
    sql = "SELECT thinkthen_find('Which unit?', ['first','second'])"
    held_cancel(sql, 1)
    with Backend() as backend:
        got = run(["SET thinkthen_query_budget_ms = 0", sql], backend.base())
        expect(said(got[1]), "thinkthen deadline: the query has spent its time budget (retryable: no)", "spent statement")
        expect(backend.count(), 0, "spent statement sends nothing")


if __name__ == "__main__":
    sys.exit(main())
