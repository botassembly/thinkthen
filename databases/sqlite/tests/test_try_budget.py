#!/usr/bin/env python3
"""SQL row values and the explicit connection budget (ADR 0080)."""

import json
import os
import sys
import threading
from pathlib import Path

from conditional_backend import ConditionalBackend

from helper import Backend, Child, child, environment, expect, main

# Ticket 0352: the stress profile alone checks how fast a budget returns.
STRESS = os.environ.get("THINKTHEN_TEST_PROFILE") == "stress"


def test_zero_budget_sends_nothing_and_is_connection_owned() -> None:
    backend = Backend()
    held = child("""
a, b = connect(), connect()
a.execute('SELECT thinkthen_budget_ms(0)')
say(spent=run(a, "SELECT value FROM thinkthen_decide_details('Is it red?', 'a red door')"),
    other=run(b, "SELECT value FROM thinkthen_decide_details('Is it red?', 'a red door')"),
    cleared=run(a, 'SELECT thinkthen_budget_ms(-1)'))
""", environment(backend))
    expect(held["spent"], "thinkthen deadline: the connection's ThinkThen budget passed (retryable: no)", "zero budget")
    expect(held["other"], [[1]], "other connection")
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
db.execute('SELECT thinkthen_configure(?)', ('{"max_requests_total":1}',))
first = run(db, 'SELECT thinkthen_find(?, ?)', ('Which?', units))
spent = run(db, 'SELECT thinkthen_find(?, ?)', ('Which other?', units))
say(expired=expired, first=first, spent=spent)
""", environment(backend))
    expect(held["expired"], "thinkthen deadline: the connection's ThinkThen budget passed (retryable: no)", "expired query")
    expect(json.loads(held["first"][0][0])["index"], 0, "one completed find")
    expect(held["spent"], "thinkthen usage: this process has sent its total of 1 requests (thinkthen_configure) (retryable: no)", "ordinary scalar preflight")
    expect(backend.close(), 1, "expired and spent calls send nothing")


def test_budget_ends_a_held_send_without_erasing_it() -> None:
    backend = Backend()
    held = Child("""
db = connect()
db.execute('SELECT thinkthen_budget_ms(150)')
started = time.monotonic()
error = run(db, "SELECT value FROM thinkthen_decide_details('Is it red?', 'a red door')")
say(error=error, took=time.monotonic()-started)
""", environment(backend, "arm/held"))
    expect(backend.wait(1), 1, "the held send")
    # A guard, armed once the send is held: a budget that never ends the send fails in 30 s.
    release = threading.Timer(30, backend.release)
    release.start()
    try:
        held = held.result()
    finally:
        release.cancel()
    expect(held["error"], "thinkthen deadline: the connection's ThinkThen budget passed (retryable: no)", "deadline remains fatal")
    expect(held["took"] < (0.4 if STRESS else 10), True, "prompt return")
    expect(backend.close(), 1, "sent attempt remains counted")


def test_one_budget_spans_two_rows_in_one_statement() -> None:
    backend = Backend()
    held = Child("""
db = connect()
db.execute('SELECT thinkthen_budget_ms(700)')
started = time.monotonic()
result = run(db, "WITH t(i,e) AS (VALUES (1, 'first row'), (2, 'second row')) SELECT value FROM t, thinkthen_decide_details('Is it red?', e) ORDER BY i")
say(result=result, took=time.monotonic()-started)
""", environment(backend, "arm/held"))
    try:
        expect(backend.wait(1), 1, "first row sent")
        backend.round()
        expect(backend.wait(2), 2, "second row sent under the same budget")
        result = held.result()
        expect(result["result"], "thinkthen deadline: the connection's ThinkThen budget passed (retryable: no)", "second held row expires")
        expect(result["took"] < (1.2 if STRESS else 10), True, "original budget bounds both rows")
    finally:
        backend.release()
    expect(backend.close(), 2, "both attempts remain counted")


if __name__ == "__main__":
    sys.exit(main(globals()))
