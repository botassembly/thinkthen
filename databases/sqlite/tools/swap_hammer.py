#!/usr/bin/env python3
# usage: THINKTHEN_NULL=1 python3 tools/swap_hammer.py target/release/libthinkthen0.so [iterations]
"""The review-4 item-7 probe for SQLite: the @file check-then-open race.

A manual probe, too slow for the gate (tools/file_door.py holds the
deterministic shapes the gate runs). It hammers `@q.json` reads while a
swapper flashes a fifo, or a symlink to a fifo, over the name. Pre-fix,
a swapped-in fifo parked the process in open() (the reviewer hung at
iteration 59). Post-fix every read succeeds on a regular file or
refuses with the one refused message, and no call takes more than a
second. Symlinks are followed on purpose: this surface confines nothing
(review 5).

Runs against LIBSQLITE, default the .runtimes build that
tests/host_sqlite.sh makes. Exit 0 = no hang and no other message.
"""
import ctypes
import json
import os
import shutil
import sys
import tempfile
import threading
import time
from pathlib import Path

LIB = sys.argv[1]
ITERATIONS = int(sys.argv[2]) if len(sys.argv) > 2 else 20_000
LIBSQLITE = os.environ.get("LIBSQLITE") or str(
    Path(__file__).resolve().parent.parent / ".runtimes" / "host" / "libsqlite3.so.0"
)
QUESTION = json.dumps({"decide": "is the order late?", "threshold": 0.5})


def swap(work: Path, stop: threading.Event) -> None:
    """Keep the regular file in place, flashing a fifo or a link to one."""
    target = work / "q.json"
    toggle = True
    while not stop.is_set():
        flash = work / "flash"
        if toggle:
            os.mkfifo(flash)
        else:
            os.symlink(work / "fifo", flash)
        os.replace(flash, target)
        spare = work / "spare.json"
        spare.write_text(QUESTION)
        os.replace(spare, target)
        toggle = not toggle


def main() -> int:
    os.environ.setdefault("THINKTHEN_NULL", "1")
    work = Path(tempfile.mkdtemp(prefix="swap-hammer-"))
    os.mkfifo(work / "fifo")
    (work / "q.json").write_text(QUESTION)
    sql = ctypes.CDLL(LIBSQLITE)
    sql.sqlite3_errmsg.restype = ctypes.c_char_p
    db = ctypes.c_void_p()
    sql.sqlite3_open_v2(b":memory:", ctypes.byref(db), 0x2, None)
    sql.sqlite3_enable_load_extension(db, 1)
    error = ctypes.c_char_p()
    if sql.sqlite3_load_extension(db, str(Path(LIB).with_suffix("")).encode(), None, ctypes.byref(error)) != 0:
        print(f"load failed: {error.value and error.value.decode()}")
        return 1
    stop = threading.Event()
    swapper = threading.Thread(target=swap, args=(work, stop), daemon=True)
    swapper.start()
    counts = {"read": 0, "refused": 0, "other": 0, "slow": 0}
    for i in range(ITERATIONS):
        # A distinct spelling per call dodges the question cache.
        query = f"select thinkthen_decide('@{work}/{'./' * (i % 400)}q.json', 'late')".encode()
        tail = ctypes.c_void_p()
        started = time.monotonic()
        sql.sqlite3_prepare_v2(db, query, -1, ctypes.byref(tail), None)
        rc = sql.sqlite3_step(tail)
        message = ctypes.string_at(sql.sqlite3_errmsg(db)).decode(errors="replace")
        sql.sqlite3_finalize(tail)
        if time.monotonic() - started > 1:
            counts["slow"] += 1
        if rc == 100:
            counts["read"] += 1
        elif "did not read" in message:
            counts["refused"] += 1
        else:
            counts["other"] += 1
            if counts["other"] < 4:
                print(f"other at {i}: {message[:120]}")
    stop.set()
    swapper.join(timeout=2)
    shutil.rmtree(work, ignore_errors=True)
    print(f"hammer done: {counts} iterations={ITERATIONS}")
    return 1 if counts["slow"] or counts["other"] else 0


if __name__ == "__main__":
    sys.exit(main())
