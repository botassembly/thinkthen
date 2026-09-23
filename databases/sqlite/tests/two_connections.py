#!/usr/bin/env python3
"""The SQLite surface's per-connection interrupt (group 2).

Two connections load the extension, the second closes, and the first's
long warm must still hear its own interrupt. The interrupt poll reads the
calling connection's handle from SQLite's own context, never a process-wide
one, so a closed connection can never be read and a second connection can
never confuse the first.

Two modes:

- `python3 tests/two_connections.py` (the null backend, always run): the
  closed connection is the one the old global kept, and the surviving
  connection must still end its warm promptly and keep answering. This
  proves the closed handle is never read; on a fast backend SQLite's own
  step loop also hears an interrupt, so this mode alone cannot isolate the
  poll.
- `STUB_PORT=8218 python3 tests/two_connections.py wire` (against the
  loopback stub, run only when it is up): a 300 ms backend, so SQLite's
  step loop cannot hear the interrupt while our callback waits — only the
  poll can carry the stop. The warm would run about 9.6 s to completion
  (1,024 rows, one request a row, width 32); with the old global reading
  the closed second connection the interrupt would never land, so the
  bound and the `thinkthen cancelled` message cannot both hold.
"""

from __future__ import annotations

import os
import pathlib
import sqlite3
import sys
import threading
import time
import urllib.request

MODE = sys.argv[1] if len(sys.argv) > 1 else "null"

HERE = pathlib.Path(__file__).resolve().parent
def _library() -> "pathlib.Path":
    """The built extension, whatever the platform named it: .so or
    .dylib (the name is derived from the crate's `thinkthen0`)."""
    for candidate in (HERE.parent / "target" / "release").glob("libthinkthen0.*"):
        if candidate.suffix in (".so", ".dylib"):
            return candidate
    raise SystemExit("no built extension under target/release; run cargo build --release")


LIB = _library()
ROWS = 1_024 if MODE == "wire" else 1_000_000
AFTER = 0.5
BOUND = AFTER + 1.5

if MODE == "wire":
    PORT = int(os.environ.get("STUB_PORT", "8218"))
    BASE = os.environ.get("ENGINE_BASE_URL", f"http://127.0.0.1:{PORT}/v1")
    os.environ["ENGINE_BASE_URL"] = BASE
    os.environ["ENGINE_WIDTH"] = "32"
    os.environ.pop("ENGINE_NULL", None)
else:
    os.environ["ENGINE_NULL"] = "1"
    os.environ["ENGINE_WIDTH"] = "32"
    os.environ.pop("ENGINE_BASE_URL", None)

FAILURES = 0


def check(name: str, held, wanted) -> None:
    global FAILURES
    if held == wanted:
        print(f"ok  {name}")
    else:
        FAILURES += 1
        print(f"FAIL {name}: held {held!r}, wanted {wanted!r}")


def fresh() -> sqlite3.Connection:
    connection = sqlite3.connect(":memory:")
    connection.enable_load_extension(True)
    connection.load_extension(str(LIB))
    return connection


def stub_is_up() -> bool:
    try:
        with urllib.request.urlopen(f"{BASE}/stats", timeout=1):
            return True
    except OSError:
        return False


def main() -> int:
    if MODE == "wire" and not stub_is_up():
        print(f"skip     no stub on {BASE}")
        return 0
    if MODE == "wire":
        request = urllib.request.Request(f"{BASE}/reset", data=b"", method="POST")
        urllib.request.urlopen(request, timeout=5).read()

    first = fresh()
    second = fresh()
    # The second load is the one the old global kept; closing it left the
    # poll holding a freed handle.
    second.close()

    first.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, body TEXT)")
    first.executemany(
        "INSERT INTO t(body) VALUES (?)",
        [(f"row {index} asks for a refund",) for index in range(ROWS)],
    )

    started = time.monotonic()

    def stop() -> None:
        time.sleep(AFTER)
        first.interrupt()

    threading.Thread(target=stop, daemon=True).start()
    text = ""
    try:
        first.execute("SELECT thinkthen_warm('Is this a complaint?', body) FROM t").fetchall()
        check("the warm ends early", "completed", "an error")
    except sqlite3.OperationalError as failure:
        text = str(failure)
    elapsed = time.monotonic() - started
    check("the stop is an error, not a count", bool(text), True)
    if MODE == "wire":
        check(
            "the poll carries the stop, not SQLite's step loop",
            "thinkthen cancelled" in text,
            True,
        )
    check(
        f"the stop lands within about a tick, past {AFTER}s ({elapsed:.2f}s)",
        elapsed < BOUND,
        True,
    )

    # The surviving connection is still healthy: the poll read its own
    # handle, and the closed one was never touched.
    held = first.execute(
        "SELECT thinkthen_decide('Is this a complaint?', 'i want a refund now')"
    ).fetchone()[0]
    check("the live connection still answers", held, 1)
    first.close()

    print("failed" if FAILURES else f"the per-connection interrupt holds ({MODE})")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    sys.exit(main())
