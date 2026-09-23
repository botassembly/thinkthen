#!/usr/bin/env python3
# usage: THINKTHEN_NULL=1 python3 tools/question_cache_probe.py target/release/libthinkthen0.so
"""The review-4 item-15 probe for SQLite's question cache.

Three shapes, all from the reviewers:

- Stale: rewrite an @file question and the next call must parse the NEW
  bytes; delete it and the next call must refuse. Pre-fix, the first
  parse served forever.
- Bound: many distinct questions must not grow the process without
  limit. Pre-fix, 300,000 questions cost 422 MB; the bound is 4096
  entries, and this probe drives 50,000 two-kilobyte questions and
  refuses to pass on more than 50 MB of growth.
- Answers (review 4, R4-17): 200,000 distinct texts for one question
  must not grow the process past 40 MB. Pre-fix the saved-answer map
  had no bound, and 300,000 texts held 316 MB.
- Stamp race (review 5): the file is replaced while its first read is
  under way, and the replacement then stays. The cached parse must carry
  the stamp of the bytes it parsed, so the next call re-reads. Pre-fix,
  the stamp came from a second open and 5 of 400 entries served the old
  parse for good; this probe runs 1,500 trials and allows none.

One limit stays, by design: a rewrite that keeps the size and restores
the modified time (touch -d) is not seen. A content hash would read the
file on every row.

Runs against the floor host (LIBSQLITE, default the .runtimes build).
Exit 0 = every assertion held.
"""
import ctypes
import json
import os
import shutil
import resource
import sys
import tempfile
import threading
from pathlib import Path

LIB = sys.argv[1]
LIBSQLITE = os.environ.get("LIBSQLITE") or str(
    Path(__file__).resolve().parent.parent / ".runtimes" / "host" / "libsqlite3.so.0"
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

    # Stamp race: swap the file while its first read runs, then leave the
    # replacement in place. A later call must see the replacement.
    good_text = json.dumps({"decide": "Is it red?"})
    bad_text = json.dumps({"choose": "Is it red?", "pad": 1})
    raced = held / "r.json"
    stale = 0
    for trial in range(1500):
        raced.write_text(good_text)
        stop = threading.Event()

        def swap() -> None:
            while True:
                spare = held / "spare.json"
                spare.write_text(good_text)
                os.replace(spare, raced)
                spare.write_text(bad_text)
                os.replace(spare, raced)
                if stop.is_set():
                    return  # the bad file stays, untouched from here on

        swapper = threading.Thread(target=swap)
        swapper.start()
        spelled = f"@{held}/" + "./" * trial + "r.json"
        first, _ = ask(sql, db, spelled)
        stop.set()
        swapper.join()
        later, _ = ask(sql, db, spelled)
        if first == 100 and later == 100:
            stale += 1
    print(f"stamp race: {stale} of 1500 entries served a replaced file's old parse")
    if stale:
        failures.append(f"the stamp race left {stale} stale entries")

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

    # The answer map's bound (review 4, R4-17): 200,000 distinct texts
    # for one question, about 420 bytes each, in one statement. Pre-fix
    # the map kept every answer: 300,000 texts held 316 MB. The budget is
    # 16 MiB of real memory; growth past 40 MB fails.
    before = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    tail = ctypes.c_void_p()
    many = (
        b"with recursive s(x) as (select 1 union all select x+1 from s where x<200000) "
        b"select count(thinkthen_decide('Is it red?', 'evidence '||x||' '||hex(randomblob(200)))) from s"
    )
    sql.sqlite3_prepare_v2(db, many, -1, ctypes.byref(tail), None)
    rc = sql.sqlite3_step(tail)
    sql.sqlite3_finalize(tail)
    growth_mb = (resource.getrusage(resource.RUSAGE_SELF).ru_maxrss - before) / 1024
    print(f"answers: 200,000 distinct texts, RSS growth {growth_mb:.1f} MB")
    if rc != 100:
        failures.append(f"the answer-map run failed: rc={rc}")
    elif growth_mb > 40:
        failures.append(f"the saved answers grew {growth_mb:.1f} MB past their 16 MiB budget")

    shutil.rmtree(held, ignore_errors=True)
    for failure in failures:
        print(f"FAIL {failure}")
    print("question cache: rewrite re-reads, delete refuses, the stamp matches the parse, growth bounded" if not failures else "question cache: see failures")
    return 1 if failures else 0


if __name__ == "__main__":
    sys.exit(main())
