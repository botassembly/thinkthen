#!/usr/bin/env python3
"""The SQLite surface's fast-backend interrupt, end to end (lane B item 5).

Against the null backend — no stub, so the engine's wait never idles at
300 ms — a warm over a million rows interrupted mid-flight must end within
about a tick, not after the batch drains.

What this proves and what it cannot, stated plainly: it proves the
statement ends promptly under SQLite's own `interrupt()` and never runs to
completion (5.5 s for this table). It cannot isolate the engine's poll
tick, because SQLite's own step loop aborts between warm flushes on a fast
backend and its `interrupted` error is what surfaces (measured
2026-09-21). The crate's `cancel_tests` test carries the discriminating
proof — it wires the poll exactly as `hear_interrupts` does and fails in
53.74 s with the busy-arm tick removed — and the stub-backed wire suite
proves the slow-backend shape where our poll carries the stop.
"""

from __future__ import annotations

import os
import pathlib
import sqlite3
import sys
import threading
import time

os.environ["ENGINE_NULL"] = "1"
os.environ["ENGINE_WIDTH"] = "32"
os.environ.pop("ENGINE_BASE_URL", None)

HERE = pathlib.Path(__file__).resolve().parent
LIB = HERE.parent / "target" / "release" / "libthinkthen0.so"
ROWS = 1_000_000
AFTER = 0.5
BOUND = AFTER + 1.5

FAILURES = 0


def check(name: str, held, wanted) -> None:
    global FAILURES
    if held == wanted:
        print(f"ok  {name}")
    else:
        FAILURES += 1
        print(f"FAIL {name}: held {held!r}, wanted {wanted!r}")


def main() -> int:
    connection = sqlite3.connect(":memory:")
    connection.enable_load_extension(True)
    connection.load_extension(str(LIB))
    connection.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, body TEXT)")
    connection.executemany(
        "INSERT INTO t(body) VALUES (?)",
        [(f"row {index} asks for a refund",) for index in range(ROWS)],
    )

    started = time.monotonic()

    def stop() -> None:
        time.sleep(AFTER)
        connection.interrupt()

    threading.Thread(target=stop, daemon=True).start()
    try:
        connection.execute(
            "SELECT thinkthen_warm('Is this a complaint?', body) FROM t"
        ).fetchall()
        check("an interrupted warm raises", "completed", "an error")
        return 1
    except sqlite3.OperationalError as failure:
        elapsed = time.monotonic() - started
        text = str(failure)
    check("the stop is an error, not a count", bool(text), True)
    check(
        f"the stop lands within about a tick, past {AFTER}s ({elapsed:.2f}s)",
        elapsed < BOUND,
        True,
    )
    print(f"{'failed' if FAILURES else 'the fast-backend interrupt holds'}")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    sys.exit(main())
