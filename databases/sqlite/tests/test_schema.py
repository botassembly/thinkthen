#!/usr/bin/env python3
"""A database file from somewhere else never reaches a function (R1-17, R2-1, decision 12).

Every function is direct-only, so only top-level SQL calls it, whatever
`trusted_schema` says. That holds from SQLite 3.50.0. Below it a CHECK
constraint reaches a volatile function, so the extension refuses to load
there. Each hostile file is written through `writable_schema`, because the
floor host refuses to author these shapes with CREATE. A crafted file is
what arrives from somewhere else anyway.
"""

from __future__ import annotations

import os
import json
import subprocess
import sys

from helper import LIB, Backend, NotRun, child, environment, expect, main

DECIDE = "thinkthen_decide('@/etc/hostname', body)"


def native_probe(which: str) -> dict[str, str]:
    """Run the pinned macOS SQLite host against the installed extension."""
    program = os.environ[f"THINKTHEN_SQLITE_PROBE_{which}"]
    result = subprocess.run([program, str(LIB)], env=environment(None), capture_output=True, text=True, check=True)
    expect(result.stderr, "", "native host diagnostics on stderr")
    return dict(line.split("=", 1) for line in result.stdout.splitlines())
ATTACK = """
import pathlib
def craft(name, steps):
    path = pathlib.Path(os.environ["SCRATCH"]) / f"{name}.db"
    seed = sqlite3.connect(path)
    for statement, *parameters in steps:
        seed.execute(statement, *parameters)
    seed.commit()
    seed.close()
    return path
def replaced(kind, name, sql, *extra):
    return [("CREATE TABLE t(body TEXT)",), *extra, ("PRAGMA writable_schema=ON",), ("UPDATE sqlite_schema SET sql=? WHERE type=? AND name=?", (sql, kind, name)), ("PRAGMA writable_schema=OFF",)]
def added(kind, name, table, sql):
    return [("CREATE TABLE t(body TEXT)",), ("PRAGMA writable_schema=ON",), ("INSERT INTO sqlite_schema(type,name,tbl_name,rootpage,sql) VALUES(?,?,?,0,?)", (kind, name, table, sql)), ("PRAGMA writable_schema=OFF",)]
index = ("CREATE INDEX x ON t(body)",)
shapes = {
    "a CHECK constraint": (replaced("table", "t", f"CREATE TABLE t(body TEXT CHECK({DECIDE} IS NOT NULL))"), "INSERT INTO other.t VALUES ('x')"),
    "a DEFAULT": (replaced("table", "t", "CREATE TABLE t(body TEXT DEFAULT (thinkthen_decide('@/etc/hostname', 'x')))"), "INSERT INTO other.t DEFAULT VALUES"),
    "a view": (added("view", "v", "v", f"CREATE VIEW v AS SELECT {DECIDE} AS d FROM t"), "SELECT * FROM other.v"),
    "a trigger": (added("trigger", "tr", "t", f"CREATE TRIGGER tr AFTER INSERT ON t BEGIN SELECT {DECIDE}; END"), "INSERT INTO other.t VALUES ('x')"),
    "a generated column": (added("table", "t2", "t2", f"CREATE TABLE t2(body TEXT, j AS ({DECIDE}))"), "SELECT * FROM other.t2"),
    "an index expression": (replaced("index", "x", f"CREATE INDEX x ON t({DECIDE})", index), "SELECT * FROM other.t"),
    "a partial index": (replaced("index", "x", f"CREATE INDEX x ON t(body) WHERE {DECIDE}", index), "SELECT * FROM other.t"),
    "a view over recognize": (added("view", "names", "names", "CREATE VIEW names AS SELECT * FROM thinkthen_recognize('Maria Chen called', 'person')"), "SELECT * FROM other.names"),
}
said = {}
for trusted in ("OFF", "ON"):
    for label, (steps, use) in shapes.items():
        path = craft(label.replace(" ", "-") + trusted, steps)
        db = connect()
        db.execute(f"PRAGMA trusted_schema={trusted}")
        attached = run(db, "ATTACH DATABASE ? AS other", (str(path),))
        said[f"{label} ({trusted})"] = attached if isinstance(attached, str) else run(db, use)
    db.execute("CREATE TABLE here(body TEXT)")
    db.execute("INSERT INTO here VALUES ('a red door')")
    said[f"top-level SQL ({trusted})"] = run(db, "SELECT thinkthen_decide('Is it red?', body) FROM here")
say(**said)
"""


def test_every_schema_object_refuses_and_top_level_sql_answers() -> None:
    """R1-17, under trusted_schema off and on."""
    backend = Backend()
    held = child(f"DECIDE = {DECIDE!r}\n" + ATTACK, environment(backend))
    unsafe, loaded = "unsafe use of thinkthen_decide()", "malformed database schema ({}) - unsafe use of thinkthen_decide()"
    wanted = {"a CHECK constraint": loaded.format("t"), "a DEFAULT": unsafe, "a view": unsafe, "a trigger": unsafe,
              "a generated column": loaded.format("t2"), "an index expression": loaded.format("x"),
              "a partial index": loaded.format("x"), "a view over recognize": 'unsafe use of virtual table "thinkthen_recognize"',
              "top-level SQL": [[1]]}
    expect(held, {f"{label} ({trusted})": said for trusted in ("OFF", "ON") for label, said in wanted.items()}, "the results")
    expect(backend.close(), 1, "sends: only the top-level call, and its twin read the cache")


def test_a_host_below_the_floor_refuses_the_load() -> None:
    """R2-1: the stock library, with the floor host off the path."""
    if sys.platform == "darwin":
        held = native_probe("OLD")
        version = held["version"]
        expect((version, held["load"]), ("3.49.0", "1"), "genuine below-floor host and refused load")
        said, later = held["error"], held["later"]
        expect(held["later_call"], "0", "later SQL after a refused load")
        expect(held["resident"] in ("0", "1"), True, "a measured macOS residency result")
        print(f"native SQLite 3.49.0 failed-load resident={held['resident']}")
    else:
        stock = {name: value for name, value in environment(None).items() if name != "LD_LIBRARY_PATH"}
        version = subprocess.run([sys.executable, "-c", "import sqlite3; print(sqlite3.sqlite_version)"], env=stock,
                                 capture_output=True, text=True, check=True).stdout.strip()
        if tuple(map(int, version.split("."))) >= (3, 50, 0):
            raise NotRun(f"the stock host is {version}, at or above the floor")
        held = child(f"""
db = sqlite3.connect(":memory:")
db.enable_load_extension(True)
try:
    db.load_extension({str(LIB)!r})
    said = "loaded"
except sqlite3.Error as failure:
    said = str(failure)
db.close()
later = sqlite3.connect(":memory:")
say(said=said, later=later.execute("SELECT 7").fetchone()[0],
    resident={str(LIB)!r} in __import__("pathlib").Path("/proc/self/maps").read_text()
    if sys.platform.startswith("linux") else None)
""", stock)
        said, later = held["said"], held["later"]
    number = sum(int(part) * scale for part, scale in zip(version.split("."), (1_000_000, 1_000, 1)))
    expect(said, "error during initialization: thinkthen needs SQLite 3.50.0 or newer (below 3.50.0 a CHECK constraint in an untrusted"
           " database reaches the functions, so a schema could spend money or read files); this host is"
           f" {version} ({number})", "the refusal")
    expect(later, "7" if sys.platform == "darwin" else 7, "the host still works after a failed load")
    if sys.platform.startswith("linux"):
        expect(held["resident"], False, "a failed load leaves no pinned Linux DSO")


def test_pinned_host_keeps_a_successful_registration_available() -> None:
    """A close after a successful load keeps registered worker code mapped."""
    if sys.platform == "darwin":
        held = native_probe("PINNED")
        expect((held["version"], held["load"], held["call"]), ("3.50.0", "0", "0"), "pinned host load and call")
        expect(json.loads(held["usage"])["requests_sent"], 0, "no send from thinkthen_usage")
        expect((held["later_call"], held["later"], held["resident"]), ("0", "7", "1"),
               "later SQL and observed macOS residency")
        return
    held = child(f"""
db = connect()
usage = json.loads(db.execute("SELECT thinkthen_usage()").fetchone()[0])
db.close()
later = sqlite3.connect(":memory:")
say(count=usage["requests_sent"], later=later.execute("SELECT 7").fetchone()[0],
    resident={str(LIB)!r} in __import__("pathlib").Path("/proc/self/maps").read_text()
    if sys.platform.startswith("linux") else None)
""", environment(None))
    expect((held["count"], held["later"]), (0, 7), "successful installed load and later use")
    if sys.platform.startswith("linux"):
        expect(held["resident"], True, "successful load retains worker code on Linux")


def test_every_function_is_direct_only_and_volatile() -> None:
    """Decision 12: every registered function is direct-only and volatile."""
    held = child("""
db = connect()
rows = db.execute("SELECT name, flags FROM pragma_function_list WHERE name LIKE 'thinkthen%'").fetchall()
db.execute("CREATE TABLE t(body TEXT)")
say(registered=bool(rows), direct=all(flags & 0x80000 for _, flags in rows),
    deterministic=[name for name, flags in rows if flags & 0x800],
    index=run(db, "CREATE INDEX x ON t(thinkthen_decide('Is it red?', body))"))
""", environment(None))
    expect(held, {"registered": True, "direct": True, "deterministic": [], "index": "unsafe use of thinkthen_decide()"}, "the list")


if __name__ == "__main__":
    os.environ.pop("THINKTHEN_API_KEY", None)
    sys.exit(main(globals()))
