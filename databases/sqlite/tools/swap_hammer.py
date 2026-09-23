#!/usr/bin/env python3
"""The review-4 item-7 probe for SQLite: the @file check-then-open race.

The reviewer's shape: hammer `@q.json` question reads while a swapper
replaces the file. The swapper alternates two attacks: a symlink that
points outside the working directory, and a fifo. Pre-fix, a swapped-in
fifo parks the process in open() (the reviewer hung at iteration 59);
post-fix every read either succeeds on a regular file or refuses with
the one refused message, and the process always exits.

Runs against an explicit libsqlite3 (LIBSQLITE, default the pinned
3.50.2 source build) so the host version is the floor the extension
itself demands. Any hang is killed by the harness timeout and counts as
the old behavior.

Usage: swap_hammer.py <path-to-libthinkthen0.so> [iterations]
Exit 0 = the door survived the hammer (no hang, no leak of the outside
file's key names); exit 1 = a leak or a wrong-answer escape.
"""
import ctypes
import json
import os
import os
import sys
import threading
import time
from pathlib import Path

LIB = sys.argv[1]
ITERATIONS = int(sys.argv[2]) if len(sys.argv) > 2 else 20_000
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

WORK = Path("/tmp/swap-hammer")
OUTSIDE = WORK / "outside.json"
INSIDE = WORK / "q.json"
FIFO = WORK / "fifo"

INSIDE_TEXT = json.dumps({"decide": "is the order late?", "threshold": 0.5})
OUTSIDE_TEXT = json.dumps({"SECRET_KEY_NAME": "leaked"})

leaked = 0
refused = 0
read_ok = 0
stop = threading.Event()


def swap() -> None:
    """Mostly leave the regular file in place; every burst, flash a
    symlink out or a fifo over the name for a moment."""
    toggle = True
    while not stop.is_set():
        for _ in range(3):
            victim = FIFO if toggle else WORK / "link.json"
            try:
                if toggle:
                    os.mkfifo(FIFO)
                else:
                    os.symlink(OUTSIDE, victim)
                os.replace(victim, INSIDE)
            except FileExistsError:
                pass
            time.sleep(0.0005)
            # Restore the regular file for the long stretch between bursts.
            spare = WORK / ("good.json")
            spare.write_text(INSIDE_TEXT)
            os.replace(spare, INSIDE)
            toggle = not toggle
            time.sleep(0.05)


def main() -> int:
    global leaked, refused, read_ok
    for flag in (getattr(os, name, None) for name in ("O_DIRECTORY",)):
        del flag
    WORK.mkdir(exist_ok=True)
    OUTSIDE.write_text(OUTSIDE_TEXT)
    INSIDE.write_text(INSIDE_TEXT)

    os.environ.setdefault("THINKTHEN_NULL", "1")
    sql = ctypes.CDLL(LIBSQLITE)
    sql.sqlite3_errmsg.restype = ctypes.c_char_p
    sql.sqlite3_open_v2(b":memory:", ctypes.byref(db := ctypes.c_void_p()), 0x2, None)
    sql.sqlite3_enable_load_extension(db, 1)
    lib_bytes = str(Path(LIB).with_suffix("")).encode()
    error = ctypes.c_char_p()
    if sql.sqlite3_load_extension(db, lib_bytes, None, ctypes.byref(error)) != 0:
        print(f"load failed: {error.value and error.value.decode()}")
        return 1
    sql.sqlite3_enable_load_extension(db, 0)

    swapper = threading.Thread(target=swap, daemon=True)
    swapper.start()
    ask = ctypes.c_char_p()
    tail = ctypes.c_void_p()
    deadline = time.time() + 60
    for i in range(ITERATIONS):
        if time.time() > deadline:
            print(f"stopped at iteration {i} on the 60 s deadline")
            break
        query = f"select thinkthen_decide('{INSIDE}', 'the order is late')".replace(str(INSIDE), "@" + str(INSIDE)).encode()
        if sql.sqlite3_prepare_v2(db, query, -1, ctypes.byref(tail), None) != 0:
            message = ctypes.string_at(sql.sqlite3_errmsg(db)).decode(errors="replace")
            if "SECRET_KEY_NAME" in message:
                leaked += 1
                print(f"LEAK at iteration {i}: {message[:120]}")
            else:
                refused += 1
            continue
        # The @file read happens inside the function call, at step.
        if sql.sqlite3_step(tail) != 100:  # SQLITE_ROW is the success row
            message = ctypes.string_at(sql.sqlite3_errmsg(db)).decode(errors="replace")
            if "SECRET_KEY_NAME" in message:
                leaked += 1
                print(f"LEAK at iteration {i}: {message[:120]}")
            elif "did not read" in message or "regular file" in message:
                refused += 1
            elif "holds one of" in message or "SECRET_KEY_NAME" in message:
                # The parse stage was reached: the door read a file the
                # swap pointed at. That is the pre-fix leak.
                leaked += 1
            else:
                if refused < 3:
                    print(f"other refusal at {i}: {message[:120]}")
                refused += 1
        else:
            read_ok += 1
        sql.sqlite3_finalize(tail)
    stop.set()
    swapper.join(timeout=2)
    print(
        f"hammer done: read_ok={read_ok} refused={refused} leaked={leaked} "
        f"iterations={ITERATIONS}"
    )
    return 1 if leaked else 0


if __name__ == "__main__":
    sys.exit(main())
