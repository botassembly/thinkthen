#!/usr/bin/env python3
"""The SQLite surface's single-row deadline and interrupt (group 4).

Single-row calls carry two stops the batch paths always had:

- the ruled third argument, a per-call deadline in milliseconds (`-1`
  none, `0` spent, a positive budget, any other negative refused), read by
  the engine's own guard between requests and waits;
- a watcher thread that reads the calling connection's
  `sqlite3_is_interrupted` while the calling thread sits inside the
  engine, so `sqlite3_interrupt` (Ctrl-C in the CLI, `Connection.interrupt`
  in Python) arms the call's cancel token. The ruled promise holds: no new
  request starts, sent requests finish.

Modes:

- `null` (offline, no wire): the argument's semantics — a spent budget
  returns the deadline kind and sends nothing, a budget below the sentinel
  is a usage refusal, the sentinel answers, and a non-number refuses.
- `wire` (no external stub): a loopback server that answers 503, so the
  engine's first wait is a one-second backoff. A 150 ms budget must return
  the deadline kind inside that backoff; the same call with no stop must
  fail later with the backend kind (the control); and an interrupt from
  another thread must land as the cancelled kind, promptly.

Run through ./check.sh.
"""

from __future__ import annotations

import http.server
import json
import os
import pathlib
import sqlite3
import sys
import threading
import time

HERE = pathlib.Path(__file__).resolve().parent
def _library() -> "pathlib.Path":
    """The built extension, whatever the platform named it: .so or
    .dylib (the name is derived from the crate's `thinkthen0`)."""
    for candidate in (HERE.parent / "target" / "release").glob("libthinkthen0.*"):
        if candidate.suffix in (".so", ".dylib"):
            return candidate
    raise SystemExit("no built extension under target/release; run cargo build --release")


LIB = _library()
MODE = sys.argv[1] if len(sys.argv) > 1 else "wire"

FAILURES = 0
STEP = 0


def check(name: str, held, wanted) -> None:
    global FAILURES, STEP
    STEP += 1
    if held == wanted:
        print(f"ok  {STEP} {name}")
    else:
        FAILURES += 1
        print(f"FAIL {STEP} {name}: held {held!r}, wanted {wanted!r}")


def fresh() -> sqlite3.Connection:
    connection = sqlite3.connect(":memory:")
    connection.enable_load_extension(True)
    connection.load_extension(str(LIB))
    return connection


def error_of(connection: sqlite3.Connection, call) -> tuple[str, float]:
    """Run one call; return (message, seconds). Raises when it succeeds."""
    started = time.monotonic()
    try:
        connection.execute(call).fetchone()
    except sqlite3.Error as failure:
        return str(failure), time.monotonic() - started
    raise AssertionError(f"the call answered: {call}")


class Refusing(http.server.BaseHTTPRequestHandler):
    """One loopback stub: every request draws a 503, the retried status."""

    def do_POST(self) -> None:  # noqa: N802 (http.server's spelling)
        length = int(self.headers.get("content-length") or 0)
        self.rfile.read(length)
        self.send_response(503)
        self.send_header("content-length", "0")
        self.end_headers()

    def log_message(self, *_: object) -> None:
        pass


def serve() -> tuple[http.server.ThreadingHTTPServer, int]:
    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), Refusing)
    threading.Thread(target=server.serve_forever, daemon=True).start()
    return server, server.server_address[1]


def null_arms() -> int:
    os.environ["ENGINE_NULL"] = "1"
    os.environ.pop("ENGINE_BASE_URL", None)
    connection = fresh()

    before = json.loads(connection.execute("SELECT thinkthen_usage()").fetchone()[0])
    message, _ = error_of(connection, "SELECT thinkthen_decide('Is this a complaint?', 'I want a refund', 0)")
    check("a spent budget returns the deadline kind", "thinkthen deadline" in message, True)
    after = json.loads(connection.execute("SELECT thinkthen_usage()").fetchone()[0])
    check("and sends nothing", after["requests"], before["requests"])

    message, _ = error_of(connection, "SELECT thinkthen_decide('Is this a complaint?', 'I want a refund', -2)")
    check("a negative below the sentinel is a usage refusal", "thinkthen usage" in message, True)
    check("and the refusal names the value", "-2" in message, True)

    message, _ = error_of(connection, "SELECT thinkthen_decide('Is this a complaint?', 'I want a refund', 'later')")
    check("a non-number deadline is a usage refusal", "not a number" in message, True)

    held = connection.execute(
        "SELECT thinkthen_decide('Is this a complaint?', 'I want a refund', -1)"
    ).fetchone()[0]
    check("the no-deadline sentinel answers", held, 1)

    message, _ = error_of(connection, "SELECT thinkthen_decide('Is this a complaint?', 'I want a refund', 9007199254740992)")
    check("an oversized budget is a usage refusal", "larger than" in message, True)
    return FAILURES


def wire_arms() -> int:
    server, port = serve()
    os.environ.pop("ENGINE_NULL", None)
    os.environ["ENGINE_BASE_URL"] = f"http://127.0.0.1:{port}/v1"

    connection = fresh()
    message, elapsed = error_of(
        connection, "SELECT thinkthen_decide('Is this a complaint?', 'I want a refund', 150)")
    check("a 150 ms budget returns the deadline kind", "thinkthen deadline" in message, True)
    check(f"and lands inside the first backoff ({elapsed:.2f} s)", elapsed < 1.5, True)

    # The control: no budget, no interrupt — the same call drains the
    # backoffs and fails later with the backend kind, so the two arms
    # above cannot pass by the call always failing fast.
    connection = fresh()
    message, elapsed = error_of(
        connection, "SELECT thinkthen_decide('Is this a complaint?', 'I want a refund')")
    check("the same call with no stop fails later", elapsed >= 2.0, True)
    check("with the backend kind", "thinkthen backend" in message, True)

    # The interrupt: the watcher thread reads this connection's flag while
    # the calling thread sits in the backoff, and the token lands there.
    connection = fresh()
    threading.Timer(0.25, connection.interrupt).start()
    message, elapsed = error_of(
        connection, "SELECT thinkthen_decide('Is this a complaint?', 'I want a refund')")
    check("an interrupt lands as the cancelled kind", "thinkthen cancelled" in message, True)
    check(f"and lands before the backoffs drain ({elapsed:.2f} s)", elapsed < 1.5, True)

    server.shutdown()
    return FAILURES


if __name__ == "__main__":
    sys.exit(null_arms() if MODE == "null" else wire_arms())
