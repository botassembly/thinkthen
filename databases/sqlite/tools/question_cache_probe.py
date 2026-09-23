#!/usr/bin/env python3
"""The review-4 item-15 probe for SQLite's question cache.

Two shapes, both from the reviewer:

- Stale: rewrite an @file question and the next call must parse the NEW
  bytes; delete it and the next call must refuse. Pre-fix, the first
  parse served forever.
- Bound: many distinct questions must not grow the process without
  limit. Pre-fix, 300,000 questions cost 422 MB; the bound is 4096
  entries, and this probe drives 50,000 two-kilobyte questions and
  refuses to pass on more than 50 MB of growth.

Runs against the floor host (LIBSQLITE, default the .runtimes build).
Exit 0 = every assertion held.
"""
import ctypes
import json
import os
import resource
import sys
import tempfile
from pathlib import Path

LIB = sys.argv[1]
LIBSQLITE = os.environ.get("LIBSQLITE") or next(
    (
        str(candidate)
        for candidate in (
            Path(__file__).resolve().parent.parent / ".runtimes" / "host" / "libsqlite3.so.0",
            Path("/tmp/sqlite350/lib/libsqlite3.so"),
        )
        if candidate.exists()
    ),
    "/tmp/sqlite350/lib/libsqlite3.so",
)


def ask(sql, db, question: str) -> tuple[int, str]:
    tail = ctypes.c_void_p()
    query = f"select thinkthen_decide('{question}', 'late')".encode()
    if sql.sqlite3_prepare_v2(db, query, -1, ctypes.byref(tail), None) != 0:
        return 1, ctypes.string_at(sql.sqlite3_errmsg(db)).decode(errors="replace")
    rc = sql.sqlite3_step(tail)
    message = ctypes.string_at(sql.sqlite3_errmsg(db)).decode(errors="replace")
    sql.sqlite3_finalize(tail)
    return rc, message


def main() -> int:
    os.environ.setdefault("THINKTHEN_NULL", "1")
    failures = []
    held = Path(tempfile.mkdtemp(prefix="qcache-"))
    good = held / "q.json"

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

    # Stale on rewrite: the second call must see the new bytes.
    good.write_text(json.dumps({"decide": "first question?", "threshold": 0.5}))
    rc, _ = ask(sql, db, f"@{good}")
    if rc != 100:
        failures.append(f"first parse failed: rc={rc}")
    good.write_text("{not json at all")
    rc, message = ask(sql, db, f"@{good}")
    if rc == 100 or "parse" not in message.lower() and "json" not in message.lower():
        # Pre-fix: the cached first parse answered, rc=100, no error.
        failures.append(f"rewrite served the stale parse: rc={rc} msg={message[:90]}")

    # Stale on delete: the call must refuse, not answer from the cache.
    good.unlink()
    rc, message = ask(sql, db, f"@{good}")
    if rc == 100:
        failures.append(f"delete served the stale parse: rc={rc}")

    # Bound: 50,000 distinct two-kilobyte questions, then the growth.
    before = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    filler = "x" * 2048
    for i in range(50_000):
        question = json.dumps({"decide": f"question {i} {filler}", "threshold": 0.5})
        rc, _ = ask(sql, db, question)
        if rc != 100:
            failures.append(f"bound run stopped at {i}: rc={rc}")
            break
    after = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    growth_mb = (after - before) / 1024
    print(f"bound: 50,000 distinct questions, RSS growth {growth_mb:.1f} MB")
    if growth_mb > 50:
        failures.append(f"the cache grew {growth_mb:.1f} MB past the 4096-entry bound")

    for failure in failures:
        print(f"FAIL {failure}")
    print("question cache: rewrite re-reads, delete refuses, growth bounded" if not failures else "question cache: see failures")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
