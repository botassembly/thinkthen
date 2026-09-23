"""A relate waiting behind another relate on the same database counts
its wait against thinkthen_relate_seconds (review 7 verification: the
wait was untimed, so at a 2 s limit the second relate ended at 4 s).

Two cursors of one database each run a slow relate under a 2 s limit;
the second starts 0.3 s after the first and must end within 2.6 s of its
own start. Runs offline on the null backend, through the build's venv.
"""

import os
import pathlib
import sys
import threading
import time

os.environ["ENGINE_NULL"] = "1"
import duckdb  # noqa: E402

EXT = pathlib.Path(__file__).resolve().parent.parent / "build" / "release" / "thinkthen.duckdb_extension"
SLOW = (
    "SELECT i, 'b' FROM (WITH RECURSIVE r(i) AS (SELECT 0 UNION ALL SELECT i + 1 FROM r WHERE i < 100000000) "
    "SELECT max(i) AS i FROM r)"
)

db = duckdb.connect(":memory:", config={"allow_unsigned_extensions": "true"})
db.execute(f"LOAD '{EXT}'")
first, second = db.cursor(), db.cursor()
for cursor in (first, second):
    cursor.execute("SET thinkthen_relate_seconds = 2")
said = {}


def run(name, cursor):
    start = time.monotonic()
    try:
        cursor.execute(f"SELECT count(*) FROM thinkthen_relate($$ {SLOW} $$, ['caused_by'])").fetchall()
        said[name] = (time.monotonic() - start, "answered")
    except Exception as error:  # the stop is the expected outcome
        said[name] = (time.monotonic() - start, str(error).splitlines()[0])


threads = [threading.Thread(target=run, args=("first", first))]
threads[0].start()
time.sleep(0.3)
threads.append(threading.Thread(target=run, args=("second", second)))
threads[1].start()
for thread in threads:
    thread.join(timeout=60)
elapsed, message = said.get("second", (99.0, "never ended"))
if elapsed > 2.6 or "limit" not in message:
    print(f"FAILED   the waiting relate ended at {elapsed:.2f}s of its own time: {message}")
    sys.exit(1)
print(f"ok       the waiting relate stopped at {elapsed:.2f}s under its 2 s limit: {message}")
after = second.execute("SELECT 42").fetchall()
if after != [(42,)]:
    print(f"FAILED   the cursor did not answer after the stop: {after}")
    sys.exit(1)
print("ok       the cursor answers its next query")
