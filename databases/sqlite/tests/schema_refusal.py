#!/usr/bin/env python3
"""The SQLite surface's untrusted-schema refusals, at the floor (group 3).

A database file that arrives from somewhere else can carry schema objects
— a view, a trigger, a DEFAULT, a CHECK constraint, a generated column, an
index expression — and an application that opens it and loads this
extension must never let that file spend money or read files. Every
function carries `SQLITE_DIRECTONLY` and both table-valued modules are
direct-only, so only top-level SQL may call them, whatever the host's
`trusted_schema` setting says.

That promise holds from SQLite 3.50.0, the surface's floor: below it,
SQLite resolves a volatile function inside a CHECK constraint without the
from-DDL mark, so `SQLITE_DIRECTONLY` is not enforced there (the matrix in
NOTES.md, 2026-09-22: 3.45.1 and 3.49.0 run the CHECK, 3.50.0 refuses).
This suite therefore also proves the floor itself:

- under the floor host the extension refuses to load, naming 3.50.0, the
  host, and the reason;
- at the floor host every schema object refuses, CHECK included.

Every hostile file here is written through `writable_schema`, because a
floor host refuses to author these shapes with `CREATE` — that refusal is
itself part of the fix, and a crafted file is what arrives from somewhere
else anyway.

Run through ./check.sh, which puts a floor host in front of the Python
tests when the stock library is older (tests/host_sqlite.sh).
"""

from __future__ import annotations

import os
import pathlib
import sqlite3
import subprocess
import sys
import tempfile

os.environ.setdefault("ENGINE_NULL", "1")

HERE = pathlib.Path(__file__).resolve().parent
LIB = HERE.parent / "target" / "release" / "libthinkthen0.so"

FAILURES = 0
STEP = 0

DECIDE = "thinkthen_decide('@/etc/hostname', body)"


def check(name: str, held, wanted) -> None:
    global FAILURES, STEP
    STEP += 1
    if held == wanted:
        print(f"ok  {STEP} {name}")
    else:
        FAILURES += 1
        print(f"FAIL {STEP} {name}: held {held!r}, wanted {wanted!r}")


def refusal_of(run) -> str:
    """The message a refused schema use raises, or '' when it ran."""
    try:
        run()
    except sqlite3.Error as failure:
        return str(failure)
    return ""


def craft(scratch: pathlib.Path, name: str, steps: list[tuple]) -> pathlib.Path:
    """Write one hostile database through writable_schema: the file a
    crafted download holds, and the only way to author these shapes at the
    floor (CREATE refuses them)."""
    path = scratch / f"{name}.db"
    seed = sqlite3.connect(str(path))
    for statement, *parameters in steps:
        seed.execute(statement, parameters[0] if parameters else ())
    seed.commit()
    seed.close()
    return path


def victim(trusted: str) -> sqlite3.Connection:
    connection = sqlite3.connect(":memory:")
    connection.enable_load_extension(True)
    connection.load_extension(str(LIB))
    connection.execute(f"PRAGMA trusted_schema={trusted}")
    return connection


def attached_refusal(connection: sqlite3.Connection, path: pathlib.Path, statement: str) -> str:
    """Attach the hostile file and use it. A refusal may come from the
    schema load itself (SQLite refuses the whole database) or from
    compiling or running the statement; both are refusals."""
    try:
        connection.execute("ATTACH DATABASE ? AS other", (str(path),))
    except sqlite3.Error as failure:
        return str(failure)
    message = refusal_of(lambda: connection.execute(statement))
    try:
        connection.execute("DETACH DATABASE other")
    except sqlite3.Error:
        pass
    return message


def main() -> int:
    print(f"   host SQLite {sqlite3.sqlite_version}")

    # The old-host arm: the stock host, with the floor host's library taken
    # off the path, must refuse the load by name. Skipped on a host already
    # at or above the floor (macOS 26 carries 3.51.0).
    probe = subprocess.run(
        [sys.executable, "-c", "import sqlite3;print(sqlite3.sqlite_version)"],
        capture_output=True, text=True, check=True,
        env={k: v for k, v in os.environ.items() if k != "LD_LIBRARY_PATH"},
    )
    stock = probe.stdout.strip()
    stock_version = tuple(int(part) for part in stock.split("."))
    if stock_version < (3, 50, 0):
        script = (
            "import sqlite3,sys\n"
            "con=sqlite3.connect(':memory:'); con.enable_load_extension(True)\n"
            "try:\n"
            f"    con.load_extension({str(LIB)!r})\n"
            "except sqlite3.Error as failure:\n"
            "    print(failure); sys.exit(0)\n"
            "print('LOADED'); sys.exit(1)\n"
        )
        old = subprocess.run(
            [sys.executable, "-c", script],
            capture_output=True, text=True, check=True,
            env={k: v for k, v in os.environ.items() if k != "LD_LIBRARY_PATH"},
        )
        message = old.stdout.strip()
        check(f"the stock host {stock} refuses to load", "LOADED" not in message, True)
        check("the refusal names the floor", "3.50.0" in message, True)
        check("the refusal names the host", f"this host is {stock}" in message, True)
        check("the refusal names the reason", "CHECK constraint" in message, True)
    else:
        print(f"skip     the stock host is {stock}, at or above the floor")

    scratch = pathlib.Path(tempfile.mkdtemp(prefix="thinkthen-schema-"))

    # One file per shape: one bad object fails a whole database's schema
    # load, so a file each keeps every refusal its own evidence.
    check_db = craft(scratch, "check", [
        ("CREATE TABLE t(body TEXT)",),
        ("PRAGMA writable_schema=ON",),
        ("UPDATE sqlite_schema SET sql=? WHERE type='table' AND name='t'",
         (f"CREATE TABLE t(body TEXT CHECK({DECIDE} IS NOT NULL))",)),
        ("PRAGMA writable_schema=OFF",),
    ])
    default_db = craft(scratch, "default", [
        ("CREATE TABLE t(body TEXT)",),
        ("PRAGMA writable_schema=ON",),
        ("UPDATE sqlite_schema SET sql=? WHERE type='table' AND name='t'",
         ("CREATE TABLE t(body TEXT DEFAULT (thinkthen_decide('@/etc/hostname', 'x')))",)),
        ("PRAGMA writable_schema=OFF",),
    ])
    view_db = craft(scratch, "view", [
        ("CREATE TABLE t(body TEXT)",),
        ("PRAGMA writable_schema=ON",),
        ("INSERT INTO sqlite_schema(type,name,tbl_name,rootpage,sql) "
         "VALUES('view','v','v',0,?)",
         (f"CREATE VIEW v AS SELECT {DECIDE} AS d FROM t",)),
        ("PRAGMA writable_schema=OFF",),
    ])
    trigger_db = craft(scratch, "trigger", [
        ("CREATE TABLE t(body TEXT)",),
        ("PRAGMA writable_schema=ON",),
        ("INSERT INTO sqlite_schema(type,name,tbl_name,rootpage,sql) "
         "VALUES('trigger','tr','t',0,?)",
         (f"CREATE TRIGGER tr AFTER INSERT ON t BEGIN SELECT {DECIDE}; END",)),
        ("PRAGMA writable_schema=OFF",),
    ])
    gencol_db = craft(scratch, "gencol", [
        ("CREATE TABLE t(body TEXT)",),
        ("PRAGMA writable_schema=ON",),
        ("INSERT INTO sqlite_schema(type,name,tbl_name,rootpage,sql) "
         "VALUES('table','t2','t2',0,?)",
         (f"CREATE TABLE t2(body TEXT, jud AS ({DECIDE}))",)),
        ("PRAGMA writable_schema=OFF",),
    ])
    expridx_db = craft(scratch, "expridx", [
        ("CREATE TABLE t(body TEXT)",),
        ("CREATE INDEX xi ON t(body)",),
        ("PRAGMA writable_schema=ON",),
        ("UPDATE sqlite_schema SET sql=? WHERE type='index' AND name='xi'",
         (f"CREATE INDEX xi ON t({DECIDE})",)),
        ("PRAGMA writable_schema=OFF",),
    ])
    partidx_db = craft(scratch, "partidx", [
        ("CREATE TABLE t(body TEXT)",),
        ("CREATE INDEX pi ON t(body)",),
        ("PRAGMA writable_schema=ON",),
        ("UPDATE sqlite_schema SET sql=? WHERE type='index' AND name='pi'",
         (f"CREATE INDEX pi ON t(body) WHERE {DECIDE}",)),
        ("PRAGMA writable_schema=OFF",),
    ])
    names_db = craft(scratch, "names", [
        ("CREATE TABLE t(body TEXT)",),
        ("PRAGMA writable_schema=ON",),
        ("INSERT INTO sqlite_schema(type,name,tbl_name,rootpage,sql) "
         "VALUES('view','names','names',0,?)",
         ("CREATE VIEW names AS SELECT * FROM thinkthen_recognize('Maria Chen called', 'person')",)),
        ("PRAGMA writable_schema=OFF",),
    ])

    # The victim's use of each shape, and the refusal each must draw.
    uses = [
        ("a CHECK constraint", check_db, "INSERT INTO other.t VALUES ('x')"),
        ("a DEFAULT", default_db, "INSERT INTO other.t DEFAULT VALUES"),
        ("a view", view_db, "SELECT * FROM other.v"),
        ("a trigger", trigger_db, "INSERT INTO other.t VALUES ('i want a refund')"),
        ("a crafted generated column", gencol_db, "SELECT * FROM other.t"),
        ("a crafted index expression", expridx_db, "SELECT * FROM other.t"),
        ("a crafted partial index", partidx_db, "SELECT * FROM other.t"),
    ]

    # Both settings: trusted off (a hardened host) and on (Python's own
    # default, the setting the calls were reachable under before the flag).
    for trusted in ("OFF", "ON"):
        for label, path, statement in uses:
            connection = victim(trusted)
            message = attached_refusal(connection, path, statement)
            if label in ("a crafted generated column", "a crafted index expression", "a crafted partial index"):
                refused = "unsafe use" in message or "non-deterministic" in message
                check(f"{label} refuses (trusted {trusted})", refused, True)
            else:
                check(
                    f"{label} refuses (trusted {trusted})",
                    "unsafe use of thinkthen_decide()" in message,
                    True,
                )
            connection.close()

        connection = victim(trusted)
        message = attached_refusal(connection, names_db, "SELECT * FROM other.names")
        check(
            f"a view over the recognize module refuses (trusted {trusted})",
            'unsafe use of virtual table "thinkthen_recognize"' in message,
            True,
        )

        # The control: top-level SQL still reaches every door, on the same
        # connection setting.
        connection.execute("CREATE TABLE local(body TEXT)")
        connection.execute("INSERT INTO local VALUES ('i want a refund now')")
        held = connection.execute(
            "SELECT thinkthen_decide('Is this a complaint?', body, -1) FROM local"
        ).fetchone()[0]
        check(f"top-level SQL still answers (trusted {trusted})", held, 1)
        connection.close()

    for path in scratch.glob("*"):
        path.unlink()
    scratch.rmdir()
    print("failed" if FAILURES else "the untrusted-schema refusals hold at the floor")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    sys.exit(main())
