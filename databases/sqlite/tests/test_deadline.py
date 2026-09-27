#!/usr/bin/env python3
"""The third argument is a deadline in milliseconds under ADR 0041 (ticket
0109 decision 9), and a busy backend's retries end within it."""

from __future__ import annotations

import sys
import threading

from helper import Backend, Child, child, environment, expect, main

CALLS = """
db = connect()
say(**{{str(at): run(db, "SELECT thinkthen_decide('Is it red?', 'a red door', " + deadline + ")") for at, deadline in enumerate({deadlines!r})}})
"""


def decided(deadlines: list[str], arm: str = "generic") -> tuple[list, int]:
    backend = Backend()
    held = child(CALLS.format(deadlines=deadlines), environment(backend, arm))
    return [held[str(at)] for at in range(len(deadlines))], backend.close()


def test_a_deadline_is_milliseconds_under_adr_0041() -> None:
    """R2-10 and R1-11: -1 answers, 0 is spent, and every refusal names the value."""
    held, sends = decided([
        "0", "-2", "4294967295001", "9223372036854775807", "1.5", "1e300", "'100'", "NULL",
    ])
    expect(held, [
        "thinkthen deadline: the deadline of 0 s passed before the call answered",
        "thinkthen usage: a deadline of -2 milliseconds is not -1, 0, or a positive budget of at most 4294967295 seconds",
        "thinkthen usage: a deadline of 4294967295001 milliseconds is not -1, 0, or a positive budget of at most 4294967295 seconds",
        "thinkthen usage: a deadline of 9223372036854775807 milliseconds is not -1, 0, or a positive budget of at most 4294967295 seconds",
        "thinkthen usage: a deadline of 1.5 is not a whole number of milliseconds",
        "thinkthen usage: a deadline of 1e300 is not a whole number of milliseconds",
        "thinkthen usage: a deadline of '100' is not a whole number of milliseconds",
        "thinkthen usage: a deadline of NULL is not a whole number of milliseconds",
    ], "the refusals")
    expect(sends, 0, "sends")
    held, sends = decided(["-1", "2000.0"])
    expect((held, sends), ([[[1]], [[1]]], 1), "-1 and a whole REAL answer")


def test_a_held_call_ends_at_its_deadline() -> None:
    """R2-10: the worker's options carry the deadline through a blocking send."""
    backend = Backend()
    release = threading.Timer(2, backend.release)
    release.start()
    held = Child("""
db = connect()
started = time.monotonic()
error = run(db, "SELECT thinkthen_decide('Is it red?', 'a red door', 200)")
say(error=error, took=time.monotonic() - started)
""", environment(backend, "arm/held")).result()
    release.cancel()
    expect(held["error"], "thinkthen deadline: the deadline of 200 ms passed before the call answered", "the error")
    expect(held["took"] < 0.3, True, f"the call took {held['took']} s")
    expect(backend.close(), 1, "sends")


def test_a_busy_backend_ends_after_its_retries() -> None:
    """R2-24: the engine retries three times (`max_retries: 3` in EngineBuilder::build),
    so a 503 arm takes four sends and ends retryable before a 150 ms deadline."""
    held, sends = decided(["150"], "arm/503")
    expect(held, ["thinkthen backend (retryable): the backend answered with status 503"], "the error")
    expect(sends, 4, "sends")


if __name__ == "__main__":
    sys.exit(main(globals()))
