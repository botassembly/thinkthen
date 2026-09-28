#!/usr/bin/env python3
"""SQL row values and the explicit connection budget (ADR 0080)."""

import json
import sys
import threading
from pathlib import Path

from conditional_backend import ConditionalBackend

from helper import Backend, Child, child, environment, expect, main


def test_bad_row_returns_a_value_and_later_row_answers() -> None:
    backend = Backend()
    held = child("""
db = connect()
rows = run(db, "WITH t(i,q,e) AS (VALUES (1, 'Is it red?', 'a red door'), (2, '  ', 'private evidence'), (3, '@private-missing.json', 'private evidence'), (4, 'Is it red?', 'a red door')) SELECT thinkthen_try_details(q, e) FROM t ORDER BY i")
say(rows=rows, missing=run(db, "SELECT thinkthen_try_details(NULL, 'bad')"), usage=run(db, "SELECT thinkthen_usage()"))
""", environment(backend))
    rows = [json.loads(row[0]) for row in held["rows"]]
    expect([row["status"] for row in rows], ["answered", "failed", "failed", "answered"], "row statuses")
    expect(rows[1]["error"], {
        "kind": "usage",
        "message": "check the row's question and arguments, or raise the process request total when it is spent",
        "retryable": False,
    }, "safe failure")
    expect(rows[2]["error"]["kind"], "local", "named file failure")
    expect(rows[2]["error"], {"kind": "local", "message": "check the named file and its permissions", "retryable": False}, "safe local failure")
    for row in rows[1:3]:
        for private in ("private evidence", "private-missing.json"):
            expect(private in json.dumps(row), False, "private text stays out of failed values")
    expect(held["missing"], [[None]], "SQL NULL")
    expect(backend.close(), 1, "only the first good row sent; the second was cached")


def test_zero_budget_sends_nothing_and_is_connection_owned() -> None:
    backend = Backend()
    held = child("""
a, b = connect(), connect()
a.execute('SELECT thinkthen_budget_ms(0)')
say(spent=run(a, "SELECT thinkthen_try_details('Is it red?', 'a red door')"),
    other=run(b, "SELECT thinkthen_try_details('Is it red?', 'a red door')"),
    cleared=run(a, 'SELECT thinkthen_budget_ms(-1)'))
""", environment(backend))
    expect(held["spent"], "thinkthen deadline: the connection's ThinkThen budget passed", "zero budget")
    expect(json.loads(held["other"][0][0])["status"], "answered", "other connection")
    expect(held["cleared"], [[-1]], "clear budget")
    expect(backend.close(), 1, "only the other connection sent")


def test_find_uses_the_scalar_budget_and_total_before_a_second_send() -> None:
    backend = Backend()
    held = child("""
db = connect()
units = '["first","second"]'
db.execute('SELECT thinkthen_budget_ms(0)')
expired = run(db, 'SELECT thinkthen_find(?, ?)', ('Which?', units))
db.execute('SELECT thinkthen_budget_ms(-1)')
db.execute('SELECT thinkthen_max_requests_total(1)')
first = run(db, 'SELECT thinkthen_find(?, ?)', ('Which?', units))
spent = run(db, 'SELECT thinkthen_find(?, ?)', ('Which other?', units))
say(expired=expired, first=first, spent=spent)
""", environment(backend))
    expect(held["expired"], "thinkthen deadline: the connection's ThinkThen budget passed", "expired query")
    expect(json.loads(held["first"][0][0])["index"], 0, "one completed find")
    expect(held["spent"], "thinkthen usage: this process has sent its total of 1 requests (thinkthen_max_requests_total)", "ordinary scalar preflight")
    expect(backend.close(), 1, "expired and spent calls send nothing")


def test_backend_failure_keeps_a_later_good_row() -> None:
    backend = Backend()
    with ConditionalBackend(backend.base()) as proxy:
        held = child("""
db = connect()
rows = run(db, "WITH t(i,q,e) AS (VALUES (1, 'Is it a refund?', 'first'), (2, 'Is it a refund?', 'private evidence'), (3, 'Is it a refund?', 'last')) SELECT thinkthen_try_details(q,e) FROM t ORDER BY i")
say(rows=rows)
""", environment(backend, THINKTHEN_BASE_URL=proxy.base))
        expect(proxy.count(), 3, "three requests reached the proxy")
    rows = [json.loads(row[0]) for row in held["rows"]]
    expect([row["status"] for row in rows], ["answered", "failed", "answered"], "good rows after backend refusal")
    expect(rows[1]["error"], {"kind": "backend", "message": "the backend did not answer; retry if allowed", "retryable": False}, "typed backend failure")
    expect("private evidence" in json.dumps(rows[1]), False, "failed value hides evidence")
    expect(backend.close(), 2, "two good requests reached the generic backend")


def test_unresolved_answer_and_spent_total_keep_their_shapes() -> None:
    backend = Backend()
    held = child("""
db = connect()
db.execute('SELECT thinkthen_max_requests_total(1)')
unresolved = run(db, '''SELECT thinkthen_try_details('{"decide":"Is it red?","threshold":"0:1"}', 'red door')''')
spent = run(db, "SELECT thinkthen_try_details('Is it blue?', 'a blue door')")
say(unresolved=unresolved, spent=spent)
""", environment(backend))
    unresolved = json.loads(held["unresolved"][0][0])
    expect(unresolved["status"], "answered", "unresolved is an answer")
    expect(unresolved["details"]["value"], None, "unresolved details use JSON null")
    expect("error" in unresolved, False, "answered envelope has no error")
    spent = json.loads(held["spent"][0][0])
    expect(spent["error"], {"kind": "usage", "message": "check the row's question and arguments, or raise the process request total when it is spent", "retryable": False}, "spent total becomes safe value")
    expect("details" in spent, False, "failed envelope has no details")
    expect(backend.close(), 1, "spent total sends nothing")


def test_budget_ends_a_held_send_without_erasing_it() -> None:
    backend = Backend()
    release = threading.Timer(2, backend.release)
    release.start()
    try:
        held = Child("""
db = connect()
db.execute('SELECT thinkthen_budget_ms(150)')
started = time.monotonic()
error = run(db, "SELECT thinkthen_try_details('Is it red?', 'a red door')")
say(error=error, took=time.monotonic()-started)
""", environment(backend, "arm/held")).result()
    finally:
        release.cancel()
    expect(held["error"], "thinkthen deadline: the connection's ThinkThen budget passed", "deadline remains fatal")
    expect(held["took"] < 0.4, True, "prompt return")
    expect(backend.close(), 1, "sent attempt remains counted")


def test_one_budget_spans_two_rows_in_one_statement() -> None:
    backend = Backend()
    held = Child("""
db = connect()
db.execute('SELECT thinkthen_budget_ms(700)')
started = time.monotonic()
result = run(db, "WITH t(i,e) AS (VALUES (1, 'first row'), (2, 'second row')) SELECT thinkthen_try_details('Is it red?', e) FROM t ORDER BY i")
say(result=result, took=time.monotonic()-started)
""", environment(backend, "arm/held"))
    try:
        expect(backend.wait(1), 1, "first row sent")
        backend.round()
        expect(backend.wait(2), 2, "second row sent under the same budget")
        result = held.result()
        expect(result["result"], "thinkthen deadline: the connection's ThinkThen budget passed", "second held row expires")
        expect(result["took"] < 1.2, True, "original budget bounds both rows")
    finally:
        backend.release()
    expect(backend.close(), 2, "both attempts remain counted")


if __name__ == "__main__":
    sys.exit(main(globals()))
