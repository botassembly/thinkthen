#!/usr/bin/env python3
"""SQL row values and the explicit connection budget (ADR 0080)."""

import json
import sys
import threading
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
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
    expect(rows[1]["error"]["kind"], "backend", "typed backend failure")
    expect("private evidence" in json.dumps(rows[1]), False, "failed value hides evidence")
    expect(backend.close(), 2, "two good requests reached the generic backend")


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


if __name__ == "__main__":
    sys.exit(main(globals()))
