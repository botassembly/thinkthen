#!/usr/bin/env python3
# usage: THINKTHEN_NULL=1 python3 tools/file_door.py target/release/libthinkthen0.so
"""The review-4 item-7 door probe for SQLite: deterministic, no race.

The stable shapes the swap race rides on, each checked alone:

- a regular file reads (the happy path stays happy);
- a symlink to a regular file reads its target: this surface confines
  nothing, so a link reads what naming the target would (review 5
  relaxed the review-4 refusal of every link, which blocked users and
  closed nothing);
- a fifo, named directly or through a symlink, refuses at the door
  without hanging: pre-fix, a straddled swap parked the process in
  open() (the reviewer hung at iteration 59).

Runs against the extension's own floor host: LIBSQLITE, default the
.runtimes build that tests/host_sqlite.sh makes. Exit 0 = every door
assertion held.
"""
import ctypes
import json
import os
import shutil
import sys
import tempfile
from pathlib import Path

LIB = sys.argv[1]
LIBSQLITE = os.environ.get("LIBSQLITE") or os.path.join(
    os.path.dirname(os.path.dirname(os.path.abspath(__file__))), ".runtimes", "host", "libsqlite3.so.0"
)

GOOD = json.dumps({"decide": "is the order late?", "threshold": 0.5})


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
    os.symlink(good, link)
    rc, message = one(sql, db, f"@{link}")
    if rc != 100:
        failures.append(f"symlink to a regular file refused: {message[:120]}")

    fifo = held / "pipe.json"
    os.mkfifo(fifo)
    rc, message = one(sql, db, f"@{fifo}")
    if "did not read" not in message:
        failures.append(f"fifo was not refused at the door: {message[:120]}")

    fifo_link = held / "pipe-link.json"
    os.symlink(fifo, fifo_link)
    rc, message = one(sql, db, f"@{fifo_link}")
    if "did not read" not in message:
        failures.append(f"symlink to a fifo was not refused at the door: {message[:120]}")

    shutil.rmtree(held, ignore_errors=True)
    for failure in failures:
        print(f"FAIL {failure}")
    print("file door: regular reads, symlink reads its target, fifo refused at the door directly and through a link")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
