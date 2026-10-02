"""Relate on the caller's own database: routing, file access, and cleanup
(ticket 0118 decisions 1 and 2, rows R1-16, R2-13, R3-6, R4-3, R4-5,
and R3-23; ticket 0201's read-only file route).

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

from harness import EXTENSION, Backend, case, child_env, expect, main, select

PRELUDE = r"""
import json, os, sys, time
import duckdb
EXT = sys.argv[1]
def db(path=":memory:", **config):
    con = duckdb.connect(path, config={"allow_unsigned_extensions": "true", **config})
    con.execute("SET enable_progress_bar = false")
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


def script(body: str, base: str, *, extension: Path = EXTENSION, timeout: float = 120,
           held: Backend | None = None) -> list:
    """Run `body` in a child. With `held`, the child's first standard-input
    line comes once that backend counts one request, so the script can wait
    for its own held send without a sleep (ticket 0352)."""
    with tempfile.TemporaryDirectory(prefix="thinkthen-duckdb-") as folder:
        child = subprocess.Popen(
            [sys.executable, "-c", PRELUDE + body, str(extension), folder],
            env=child_env(base, Path(folder) / "env"),
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        try:
            if held is not None:
                expect(held.wait(1), 1, "the held request")
                child.stdin.write("held\n")
                child.stdin.flush()
            out, err = child.communicate(timeout=timeout)
        finally:
            if child.poll() is None:
                child.kill()
                child.communicate()
    if child.returncode != 0:
        raise AssertionError(f"the child exited {child.returncode}: {err[-600:]}")
    return [json.loads(line) for line in out.splitlines() if line.strip()]


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
def a_read_only_file_keeps_caller_permissions_and_committed_data():
    """The C++ route opens a separate read-only connection to a read-only file."""
    with Backend() as backend:
        got = script(
            """
path = sys.argv[2] + "/a.db"
a = db(path); staff(a, 2); a.close()
a = duckdb.connect(path, read_only=True, config={"allow_unsigned_extensions": "true"})
a.execute("SET enable_progress_bar = false")
a.execute(f"LOAD '{EXT}'")
allowed = edges(a)
a.execute("CREATE TEMP TABLE only_here AS SELECT 9 AS id, 'Uncommitted' AS name, 'person' AS kind")
hidden = edges(a, "only_here")
try:
    a.execute("SELECT count(*) FROM thinkthen_relate('DELETE FROM t', ['works_for=person:organization'])")
    mutation = "answered"
except Exception as error:
    mutation = str(error).split("\\n")[0]
rules = sys.argv[2] + "/r.json"
open(rules, "w").write('{"version": 1, "relate": {"relations": [{"name": "works_for", "source": "person", "target": "organization"}]}}')
file_edges = a.execute(f"SELECT count(*) FROM thinkthen_relate('SELECT id, name, kind FROM t', '@{rules}')").fetchone()[0]
a.execute("SET enable_external_access = false")
try:
    a.execute(f"SELECT count(*) FROM thinkthen_relate('SELECT id, name, kind FROM t', '@{rules}')")
    denied = "answered"
except Exception as error:
    denied = str(error).split("\\n")[0]
say([allowed, hidden, mutation, file_edges, denied])
""",
            backend.base(),
        )
        allowed, hidden, mutation, file_edges, denied = got[0]
        expect([allowed, file_edges], [2, 2], "committed rows in the read-only file")
        expect("relate runs on a separate connection, so it cannot see temporary tables" in hidden, True, "temporary rows remain invisible")
        expect("the relate query must be a SELECT" in mutation, True, "mutating query refusal")
        expect("this database's file settings refuse it" in denied, True, "caller file permissions")
        expect(backend.count(), 1, "one allowed send; the equivalent file rule reused the answer and refusals sent none")


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
    """A closes and frees its file, B loads, and A reconnects. A's relate
    reads A's table, and caller settings refuse a rules file."""
    with Backend() as backend:
        got = script(
            """
import subprocess
path = sys.argv[2] + "/a.db"
a = db(path); staff(a, 4); a.close()
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
        said_text = refused.split("thinkthen ", 1)[1]
        expect(said_text.replace(said_text.split(" ")[4], "PATH", 1), "local: the rules file PATH was not read: this database's file settings refuse it (retryable: no)", "the rules file with access off")


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
held.start(); sys.stdin.readline()
second = a.cursor(); second.execute("SET thinkthen_relate_seconds = 1")
say(["queued", edges(second)])
os.kill(os.getpid(), signal.SIGINT); held.join()
""",
            backend.base("arm/held"),
            held=backend,
        )
        expect(dict(got), {"queued": "Invalid Input Error: thinkthen deadline: the relate query waited past its 1-second limit in the queue behind another relate on this database and did not run; retry after that relate ends or raise SET thinkthen_relate_seconds (0 turns the limit off) (retryable: no)", "held": "Invalid Input Error: thinkthen cancelled: the call was cancelled (retryable: no)"}, "the two relates")
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
def decide_after_reopen_uses_the_new_callers_file_settings():
    """A new caller owns file access after the previous connection closes."""
    with Backend() as backend:
        got = script(
            """
open(sys.argv[2] + "/q.json", "w").write('{"decide": "Is it a refund?"}')
a = db(sys.argv[2] + "/a.db"); a.close()
a = db(sys.argv[2] + "/a.db")
say(a.execute(f"SELECT thinkthen_decide('@{sys.argv[2]}/q.json', 'refund now')").fetchone()[0])
""",
            backend.base(),
        )
        expect(got, [True], "the reopened caller's file answers")
        expect(backend.count(), 1, "one send after reopening")


if __name__ == "__main__":
    stress = {"r3_23_idle_databases_cost_little"}
    select(stress)
    sys.exit(main())
