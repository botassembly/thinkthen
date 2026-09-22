#!/usr/bin/env python3
"""The SQLite surface's untrusted-schema refusals (group 3).

A database file that arrives from somewhere else can carry a view or a
trigger, and an application that opens it and loads this extension must
never let that file spend money or read files. SQLITE_DIRECTONLY is the
rule: the functions and both table-valued modules may only be invoked from
top-level SQL, whatever the host's `trusted_schema` setting says. This
script runs in Python, whose SQLite defaults `trusted_schema` to on — the
setting under which the calls were reachable before the flag, so this
suite would have failed then.

Run through ./check.sh.
"""

from __future__ import annotations

import os
import pathlib
import sqlite3
import sys
import tempfile

os.environ.setdefault("ENGINE_NULL", "1")

HERE = pathlib.Path(__file__).resolve().parent
LIB = HERE.parent / "target" / "release" / "libthinkthen0.so"

FAILURES = 0
STEP = 0


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


def main() -> int:
    # The host's default the rule has to hold under.
    probe = sqlite3.connect(":memory:")
    trusted = probe.execute("PRAGMA trusted_schema").fetchone()[0]
    check("this host trusts schema by default", trusted, 1)
    probe.close()

    # A database file from somewhere else, holding a view and a trigger.
    scratch = tempfile.mkdtemp(prefix="thinkthen-schema-")
    other = pathlib.Path(scratch) / "downloaded.db"
    seed = sqlite3.connect(str(other))
    seed.execute("CREATE TABLE t(body TEXT)")
    seed.execute(
        "CREATE VIEW v AS SELECT thinkthen_decide('Is this a complaint?', body) AS d FROM t"
    )
    seed.execute(
        "CREATE TRIGGER tr AFTER INSERT ON t BEGIN "
        "SELECT thinkthen_decide('Is this a complaint?', new.body); END"
    )
    seed.commit()
    seed.close()

    connection = sqlite3.connect(":memory:")
    connection.enable_load_extension(True)
    connection.load_extension(str(LIB))
    connection.execute("ATTACH DATABASE ? AS other", (str(other),))

    message = refusal_of(lambda: connection.execute("SELECT * FROM other.v").fetchall())
    check("a view in an attached file refuses", "unsafe use of thinkthen_decide()" in message, True)

    message = refusal_of(
        lambda: connection.execute("INSERT INTO other.t VALUES ('i want a refund')")
    )
    check("a trigger in an attached file refuses", "unsafe use of thinkthen_decide()" in message, True)
    # An insert starts a transaction; close it so the seed connection can
    # write the next view whichever way the checks above went.
    connection.commit()

    # The same rule covers the table-valued functions, whose paid call is
    # the module itself.
    seed = sqlite3.connect(str(other))
    seed.execute(
        "CREATE VIEW names AS SELECT * FROM thinkthen_recognize('Maria Chen called', 'person')"
    )
    seed.commit()
    seed.close()
    message = refusal_of(lambda: connection.execute("SELECT * FROM other.names").fetchall())
    check(
        "a view over the recognize module refuses",
        'unsafe use of virtual table "thinkthen_recognize"' in message,
        True,
    )

    # The control: top-level SQL still reaches every door.
    connection.execute("CREATE TABLE local(body TEXT)")
    connection.execute("INSERT INTO local VALUES ('i want a refund now')")
    held = connection.execute(
        "SELECT thinkthen_decide('Is this a complaint?', body) FROM local"
    ).fetchone()[0]
    check("top-level SQL still answers", held, 1)

    connection.close()
    for path in pathlib.Path(scratch).glob("*"):
        path.unlink()
    pathlib.Path(scratch).rmdir()
    print("failed" if FAILURES else "the untrusted-schema refusals hold")
    return 1 if FAILURES else 0


if __name__ == "__main__":
    sys.exit(main())
