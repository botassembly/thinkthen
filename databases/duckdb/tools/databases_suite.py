"""Relate on the caller's own database: the registry, the identity probe,
and the reaper (ticket 0118 decisions 1 and 2, rows R1-16, R2-13, R3-6,
R4-3, R4-5, R4-4, R5-26, R3-23, and R3-1).

Each case runs one Python script in a child that loads the extension into
several databases in one process. The script prints one JSON line per
result, and a failed statement prints its error.
"""

from __future__ import annotations

import json
import subprocess
import sys
import tempfile
from pathlib import Path

from harness import EXTENSION, HOOKS, Backend, case, child_env, expect, main

PRELUDE = r"""
import json, os, sys, time
import duckdb
EXT = sys.argv[1]
def db(path=":memory:", **config):
    con = duckdb.connect(path, config={"allow_unsigned_extensions": "true", **config})
    con.execute(f"LOAD '{EXT}'")
    return con
def staff(con, people, table="t"):
    con.execute(f"CREATE TABLE {table} AS SELECT i AS id, 'Person ' || i AS name, 'person' AS kind FROM range({people}) r(i)"
                f" UNION ALL SELECT 1000, 'Acme', 'organization'")
RELATE = "SELECT count(*) FROM thinkthen_relate('SELECT id, name, kind FROM {}', ['works_for=person:organization'])"
def say(value):
    print(json.dumps(value), flush=True)
def edges(con, table="t"):
    try:
        return con.execute(RELATE.format(table)).fetchone()[0]
    except Exception as error:
        return str(error).split("\n")[0]
"""


def script(body: str, base: str, *, extension: Path = EXTENSION, timeout: float = 120) -> list:
    with tempfile.TemporaryDirectory(prefix="thinkthen-duckdb-") as folder:
        done = subprocess.run(
            [sys.executable, "-c", PRELUDE + body, str(extension), folder],
            env=child_env(base, Path(folder) / "env"),
            capture_output=True,
            text=True,
            timeout=timeout,
            check=False,
        )
    if done.returncode != 0:
        raise AssertionError(f"the child exited {done.returncode}: {done.stderr[-600:]}")
    return [json.loads(line) for line in done.stdout.splitlines() if line.strip()]


@case
def r1_16_two_file_databases_each_relate_over_their_own_table():
    with Backend() as backend:
        got = script(
            """
a, b = db(sys.argv[2] + "/a.db"), db(sys.argv[2] + "/b.db")
staff(a, 4); staff(b, 11)
say([edges(a), edges(b), edges(a)])
""",
            backend.base(),
        )
        expect(got, [[4, 11, 4]], "edges per database")


@case
def r2_13_in_memory_databases_and_use_route_by_identity():
    with Backend() as backend:
        got = script(
            """
a, b = db(), db()
staff(a, 4); staff(b, 11)
a.execute("ATTACH ':memory:' AS other"); a.execute("USE other"); staff(a, 2)
say([edges(a), edges(b)])
""",
            backend.base(),
        )
        expect(got, [[2, 11]], "edges on A's used database and on B")


@case
def r3_6_no_setting_names_the_identity_and_b_cannot_read_a():
    with Backend() as backend:
        got = script(
            """
a, b = db(), db()
staff(a, 4)
try:
    b.execute("SET thinkthen_instance_token = 'x'"); said = "set"
except Exception as error:
    said = str(error).split(":")[0]
say([said, edges(b)])
""",
            backend.base(),
        )
        expect(got[0][0], "Catalog Error", "the identity setting")
        expect(got[0][1].split(" with name")[0], "Invalid Input Error: thinkthen usage: the relate query failed: Catalog Error: Table", "B's relate over A's table")


@case
def r4_3_a_released_database_reconnects_to_its_own_table():
    """A closes, the reaper drops A and frees its file, B loads, and A
    reconnects. A's relate reads A's table, and with external access off
    it refuses a rules file (R4-5's proof)."""
    with Backend() as backend:
        got = script(
            """
import subprocess
path = sys.argv[2] + "/a.db"
a = db(path); staff(a, 4); a.close()
time.sleep(4.5)
freed = subprocess.run([sys.executable, "-c", f"import duckdb; duckdb.connect({path!r}).close()"]).returncode
b = db(); staff(b, 11)
a = db(path)
open(sys.argv[2] + "/r.json", "w").write('{"version": 1, "relate": {"relations": [{"name": "works_for", "source": "person", "target": "organization"}]}}')
a.execute("SET enable_external_access = false")
try:
    a.execute(f"SELECT * FROM thinkthen_relate('SELECT id, name, kind FROM t', '@{sys.argv[2]}/r.json')"); refused = "read"
except Exception as error:
    refused = str(error).split("\\n")[0]
say([freed, edges(a), edges(b), refused])
""",
            backend.base(),
        )
        freed, a_edges, b_edges, refused = got[0]
        expect([freed, a_edges, b_edges], [0, 4, 11], "freed, A's edges, B's edges")
        expect(refused.split("thinkthen ")[-1].split(" was not read")[0].split("file ")[0], "local: the question ", "the rules file with access off")
        expect(refused.endswith("this database's file settings refuse it"), True, f"the refusal: {refused}")


@case
def r4_4_a_forged_probe_is_refused():
    with Backend() as backend:
        got = script(
            """
a = db(); staff(a, 4)
probe = a.execute("SELECT database_name FROM duckdb_databases() WHERE database_name LIKE 'thinkthen_instance_%'").fetchall()
name = probe[0][0]
a.execute(f"DETACH {name}"); a.execute(f"ATTACH ':memory:' AS {name}")
say([len(probe), len(name), edges(a)])
""",
            backend.base(),
        )
        count, length, forged = got[0]
        expect([count, length], [1, len("thinkthen_instance_") + 32], "one probe of 32 hex characters")
        expect(forged.split(": ", 1)[1], "thinkthen defect: the calling database's kept connection was released with its last caller; LOAD the extension again", "the forged relate")


@case
def r5_26_two_processes_mint_distinct_identities():
    with Backend() as backend:
        body = """
a = db()
say(a.execute("SELECT database_name FROM duckdb_databases() WHERE database_name LIKE 'thinkthen_instance_%'").fetchone()[0])
"""
        first, second = script(body, backend.base()), script(body, backend.base())
        expect(first != second and len(first[0]) == len(second[0]) == 51, True, f"{first} and {second}")


@case
def a_relate_queued_past_its_limit_refuses_with_zero_sends():
    """Decision 2: one database's gate serializes relate. A held relate keeps
    the gate, and a second relate under a 1-second limit reads the queue
    sentence without sending. A SIGINT then ends the held one."""
    with Backend() as backend:
        got = script(
            """
import signal, threading
signal.signal(signal.SIGINT, lambda number, frame: None)
a = db(); staff(a, 1)
held = threading.Thread(target=lambda: say(["held", edges(a.cursor())]))
held.start(); time.sleep(1)
second = a.cursor(); second.execute("SET thinkthen_relate_seconds = 1")
say(["queued", edges(second)])
os.kill(os.getpid(), signal.SIGINT); held.join()
""",
            backend.base("arm/held"),
        )
        expect(dict(got), {"queued": "Invalid Input Error: thinkthen deadline: the relate query waited past its 1-second limit in the queue behind another relate on this database and did not run; retry after that relate ends or raise SET thinkthen_relate_seconds (0 turns the limit off)", "held": "Invalid Input Error: thinkthen cancelled: the call was cancelled"}, "the two relates")
        expect(backend.count(), 1, "counted sends")


@case
def r3_23_idle_databases_cost_little():
    with Backend() as backend:
        got = script(
            """
import resource
idle = [db() for _ in range(500)]
time.sleep(3)
def spent():
    usage = resource.getrusage(resource.RUSAGE_SELF)
    return usage.ru_utime + usage.ru_stime
before = spent(); time.sleep(10); after = spent()
say(round((after - before) / 10 * 100, 2))
""",
            backend.base(),
            timeout=300,
        )
        expect(got[0] < 5.0, True, f"500 idle databases took {got[0]} percent of one core")


@case
def r3_1_a_bound_relate_outlives_a_forced_reaper_pass():
    with Backend() as backend:
        got = script(
            """
answered = 0
for _ in range(200):
    a = db(); staff(a, 2)
    a.execute("PREPARE r AS " + RELATE.format("t"))
    a.execute("SELECT * FROM thinkthen_test_hook_reap()").fetchall()
    answered += a.execute("EXECUTE r").fetchone()[0] == 2
    a.close()
say(answered)
""",
            backend.base(),
            extension=HOOKS,
            timeout=600,
        )
        expect(got, [200], "rounds answered")


if __name__ == "__main__":
    sys.exit(main())
