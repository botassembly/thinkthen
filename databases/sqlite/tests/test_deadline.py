#!/usr/bin/env python3
"""The accepted settings slot carries deadlines through the SQLite worker."""

from __future__ import annotations

import sys

from helper import Backend, Child, child, environment, expect, main

CALLS = """
db = connect()
say(results=[run(db, "SELECT thinkthen_decide('Is it red?', 'a red door', ?)", (settings,))
             for settings in {settings!r}])
"""


def decided(settings: list[str], arm: str = "generic") -> tuple[list, int]:
    backend = Backend()
    held = child(CALLS.format(settings=settings), environment(backend, arm))
    return held["results"], backend.close()


def test_zero_settings_deadline_refuses_before_a_send() -> None:
    """A spent call sends nothing; -1 removes the deadline for the next call."""
    held, sends = decided(['{"deadline_ms":0}', '{"deadline_ms":-1}'])
    expect(held, [
        "thinkthen deadline: the deadline of 0 s passed before the call answered (retryable: no)",
        [[1]],
    ], "settings deadline results")
    expect(sends, 1, "only the unbounded call sent")


def test_a_held_call_ends_at_its_settings_deadline() -> None:
    """A held listener has not released the request when the deadline returns."""
    backend = Backend()
    held = Child("""
db = connect()
say(error=run(db, "SELECT thinkthen_decide('Is it red?', 'a red door', ?)",
              ('{"deadline_ms":200}',)))
""", environment(backend, "arm/held"))
    try:
        expect(backend.wait(1), 1, "one held request arrived")
        answer = held.result(timeout=5)
        expect(answer["error"],
               "thinkthen deadline: the deadline of 200 ms passed before the call answered (retryable: no)",
               "the call stopped before release")
        expect(backend.count(), 1, "no extra request before release")
    finally:
        backend.release()
    expect(backend.close(), 1, "the sent attempt remains counted")


def test_a_busy_backend_ends_after_its_retry_allowance() -> None:
    """A 503 response exhausts the default three retries without a deadline race."""
    held, sends = decided(['{"deadline_ms":-1}'], "arm/503")
    expect(held, ["thinkthen backend: the backend answered with status 503 (retryable: yes)"],
           "typed backend error")
    expect(sends, 4, "one attempt and three retries")


if __name__ == "__main__":
    sys.exit(main(globals()))
