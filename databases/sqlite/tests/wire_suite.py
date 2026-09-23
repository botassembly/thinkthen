#!/usr/bin/env python3
"""The SQLite surface's wire suite, against the stub on 8218.

Proves what only the wire can: the warm pass runs at the process width and
holds the gate, the queries after a warm send nothing, a stopped query
ends between records with the cancelled kind and nothing served past the
return, a refused address names the backend kind, and a child forked
after a call answers on its own wire call (the engine's process check).
Run through ./check.sh with the stub up: STUB_PORT=8218 STUB_DELAY_MS=300.
"""

import json
import os
import signal
import sqlite3
import subprocess
import sys
import threading
import time
import urllib.request

import pathlib

HERE = pathlib.Path(__file__).resolve().parent
def _library() -> "pathlib.Path":
    """The built extension, whatever the platform named it: .so or
    .dylib (the name is derived from the crate's `thinkthen0`)."""
    for candidate in (HERE.parent / "target" / "release").glob("libthinkthen0.*"):
        if candidate.suffix in (".so", ".dylib"):
            return candidate
    raise SystemExit("no built extension under target/release; run cargo build --release")


LIB = _library()
PORT = int(os.environ.get("STUB_PORT", "8218"))
BASE = os.environ.get("ENGINE_BASE_URL", f"http://127.0.0.1:{PORT}/v1")

os.environ["ENGINE_BASE_URL"] = BASE
os.environ["ENGINE_WIDTH"] = "32"
os.environ.pop("ENGINE_NULL", None)

FAILURES = 0


def check(name, held, wanted):
    global FAILURES
    if held == wanted:
        print(f"ok  {name}")
    else:
        FAILURES += 1
        print(f"FAIL {name}: held {held!r}, wanted {wanted!r}")


def stats():
    with urllib.request.urlopen(f"{BASE}/stats", timeout=5) as reply:
        return json.loads(reply.read())


def reset():
    request = urllib.request.Request(f"{BASE}/reset", data=b"", method="POST")
    urllib.request.urlopen(request, timeout=5).read()


def fresh():
    connection = sqlite3.connect(":memory:")
    connection.enable_load_extension(True)
    connection.load_extension(str(LIB))
    return connection


def rows(count, marker="refund"):
    return [(f"row {index} asks for a {marker}",) for index in range(count)]


def suite_width():
    reset()
    conn = fresh()
    conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, body TEXT)")
    conn.executemany("INSERT INTO t(body) VALUES (?)", rows(128))
    started = time.monotonic()
    judged = conn.execute(
        "SELECT thinkthen_warm('Is this a complaint?', body) FROM t"
    ).fetchone()[0]
    wall = time.monotonic() - started
    held = stats()
    check("warm judges every row", judged, 128)
    check("warm holds the gate", held["max_in_flight"], 32)
    check("warm sends one request a row", held["requests"], 128)
    check(
        "warm runs at the width (about 1.2 s)",
        0.9 < wall < 3.0,
        True,
    )
    before = stats()["requests"]
    kept = conn.execute(
        "SELECT count(*) FROM t WHERE thinkthen_decide('Is this a complaint?', body)"
    ).fetchone()[0]
    after = stats()["requests"]
    check("the query after a warm sends nothing", after, before)
    check("the query reads the saved answers", kept, 128)


def suite_interrupt():
    reset()
    conn = fresh()
    conn.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, body TEXT)")
    conn.executemany("INSERT INTO t(body) VALUES (?)", rows(200, "quiet"))
    started = time.monotonic()

    def stop():
        time.sleep(0.7)
        conn.interrupt()

    threading.Thread(target=stop, daemon=True).start()
    try:
        conn.execute(
            "SELECT thinkthen_warm('Is this a complaint?', body) FROM t"
        ).fetchall()
        check("an interrupted warm raises", "completed", "an error")
        return
    except sqlite3.OperationalError as failure:
        elapsed = time.monotonic() - started
        text = str(failure)
    frozen = stats()["requests"]
    time.sleep(1.0)
    check(
        "the stop names the cancelled kind",
        "thinkthen cancelled" in text,
        True,
    )
    check(
        "the stop lands within one round of the signal",
        elapsed < 2.2,
        True,
    )
    check("nothing is served past the return", stats()["requests"], frozen)


def suite_backend_refused():
    # The engine builds once a process, so the refused arm runs in its own
    # child with the address pointed at a closed port before the first
    # call.
    child = f"""
import sqlite3
conn = sqlite3.connect(":memory:")
conn.enable_load_extension(True)
conn.load_extension({str(LIB)!r})
try:
    conn.execute(
        "SELECT thinkthen_decide('Is this a complaint?', 'refund please')"
    ).fetchone()
    print("no error")
except sqlite3.OperationalError as failure:
    print(failure)
"""
    held = subprocess.run(
        [sys.executable, "-c", child],
        env={**os.environ, "ENGINE_BASE_URL": "http://127.0.0.1:9/v1"},
        capture_output=True,
        text=True,
        timeout=90,
    ).stdout.strip()
    check(
        "a refused address names the backend kind, final",
        held.startswith("thinkthen backend:") and "retryable" not in held,
        True,
    )


def suite_fork():
    conn = fresh()
    conn.execute(
        "SELECT thinkthen_decide('Is this a complaint?', 'refund please')"
    ).fetchone()
    read, write = os.pipe()
    pid = os.fork()
    if pid == 0:
        code = 1
        try:
            signal.alarm(10)
            child = sqlite3.connect(":memory:")
            child.enable_load_extension(True)
            child.load_extension(str(LIB))
            held = child.execute(
                "SELECT thinkthen_decide('Is this a complaint?', 'refund please')"
            ).fetchone()[0]
            os.write(write, b"answered" if held is not None else b"empty")
            code = 0
        except BaseException:
            os.write(write, b"hang-or-error")
        finally:
            os._exit(code)
    os.close(write)
    with os.fdopen(read, "rb") as handle:
        heard = handle.read()
    _, status = os.waitpid(pid, 0)
    check("a forked child answers on its own call", heard, b"answered")
    check("the child exits clean", os.waitstatus_to_exitcode(status), 0)


def main():
    print(f"wire suite against the stub on {PORT}")
    suite_width()
    suite_interrupt()
    suite_backend_refused()
    suite_fork()
    print("wire suite done" if not FAILURES else "wire suite failed")
    sys.exit(1 if FAILURES else 0)


if __name__ == "__main__":
    main()
