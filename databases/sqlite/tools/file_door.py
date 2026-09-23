#!/usr/bin/env python3
"""The review-4 item-7 door probe for SQLite: deterministic, no race.

The stable shapes the swap race rides on, each checked alone:

- a regular file reads (the happy path stays happy);
- a symlink — pointing anywhere — refuses at the door, never at the
  parser: pre-fix, the same call read the link's target and reached the
  question parser, which is the leak;
- a fifo refuses at the door without hanging: pre-fix, a straddled
  swap parked the process in open() (the reviewer hung at iteration 59).

Runs against the extension's own floor host: the pinned 3.50.2 source
build (LIBSQLITE, default /tmp/sqlite350) or any libsqlite3 >= 3.50 the
checker passes. Exit 0 = every door assertion held.
"""
import ctypes
import json
import os
import sys
import tempfile
from pathlib import Path

LIB = sys.argv[1]
def _default_host() -> str:
    """The floor-or-newer host the suite runs against: the .runtimes
    amalgamation build first, the pinned source build second."""
    for candidate in (
        os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), ".runtimes", "host", "libsqlite3.so.0"),
        "/tmp/sqlite350/lib/libsqlite3.so",
    ):
        if os.path.exists(candidate):
            return candidate
    return "/tmp/sqlite350/lib/libsqlite3.so"


LIBSQLITE = os.environ.get("LIBSQLITE") or _default_host()

GOOD = json.dumps({"decide": "is the order late?", "threshold": 0.5})
OUTSIDE = json.dumps({"SECRET_KEY_NAME": "leaked"})


def one(sql, db, question_file: str) -> tuple[int, str]:
    tail = ctypes.c_void_p()
    query = f"select thinkthen_decide('{question_file}', 'late')".encode()
    if sql.sqlite3_prepare_v2(db, query, -1, ctypes.byref(tail), None) != 0:
        return 1, ctypes.string_at(sql.sqlite3_errmsg(db)).decode(errors="replace")
    rc = sql.sqlite3_step(tail)
    message = ctypes.string_at(sql.sqlite3_errmsg(db)).decode(errors="replace")
    sql.sqlite3_finalize(tail)
    return rc, message


def main() -> int:
    os.environ.setdefault("THINKTHEN_NULL", "1")
    held = Path(tempfile.mkdtemp(prefix="file-door-"))
    good = held / "good.json"
    good.write_text(GOOD)
    outside = held / "outside.json"
    outside.write_text(OUTSIDE)

    sql = ctypes.CDLL(LIBSQLITE)
    sql.sqlite3_errmsg.restype = ctypes.c_char_p
    db = ctypes.c_void_p()
    sql.sqlite3_open_v2(b":memory:", ctypes.byref(db), 0x2, None)
    sql.sqlite3_enable_load_extension(db, 1)
    error = ctypes.c_char_p()
    if sql.sqlite3_load_extension(db, str(Path(LIB).with_suffix("")).encode(), None, ctypes.byref(error)) != 0:
        print(f"load failed: {error.value and error.value.decode()}")
        return 1
    sql.sqlite3_enable_load_extension(db, 0)

    failures = []

    rc, message = one(sql, db, f"@{good}")
    if rc != 100:
        failures.append(f"regular file refused: {message[:120]}")

    link = held / "link.json"
    os.symlink(outside, link)
    rc, message = one(sql, db, f"@{link}")
    # The door must refuse; a parse-stage message means the link's target
    # was read.
    if "did not read" not in message:
        failures.append(f"symlink read through to: {message[:120]}")
    if "SECRET_KEY_NAME" in message:
        failures.append(f"symlink leaked the target's key names: {message[:120]}")

    fifo = held / "pipe.json"
    os.mkfifo(fifo)
    rc, message = one(sql, db, f"@{fifo}")
    if "did not read" not in message:
        failures.append(f"fifo was not refused at the door: {message[:120]}")

    for failure in failures:
        print(f"FAIL {failure}")
    print("file door: regular reads, symlink refused at the door, fifo refused at the door")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
