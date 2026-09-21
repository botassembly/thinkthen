#!/usr/bin/env python3
"""The SQLite surface's table-valued functions: `recognize` and `relate`.

Every call here runs as written in the deck's `recognize-surfaces.md`
against the recordings; nothing touches the wire, and the usage counter is
the proof a join sends nothing. Run through ./check.sh.
"""

import json
import os
import pathlib
import sqlite3
import sys

HERE = pathlib.Path(__file__).resolve().parent
LIB = HERE.parent / "target" / "release" / "libthinkthen0.so"

os.environ.setdefault("ENGINE_NULL", "1")

FAILURES = 0


def check(name, held, wanted):
    global FAILURES
    if held == wanted:
        print(f"ok  {n():3} {name}")
    else:
        FAILURES += 1
        print(f"FAIL {n():3} {name}: held {held!r}, wanted {wanted!r}")


STEP = 0


def n():
    global STEP
    STEP += 1
    return STEP


def fresh():
    connection = sqlite3.connect(":memory:")
    connection.enable_load_extension(True)
    connection.load_extension(str(LIB))
    return connection


C01 = "Maria Chen joined Northwind Freight in Chicago last spring."
EMOJI = "Le café 😀 Maria Chen arrived."
NOENT = "Athletics is her favorite subject at school."
DUP = "Chicago sent a delegation in March, and Chicago hosted the reply in June."
ALERTS = [
    "Checkout returns 500 at the payment step.",
    "Card charges are failing for every customer.",
    "The nightly export ran two hours late.",
    "The payments database ran out of disk space.",
]
DEMO_EDGES = sorted(
    [
        ("caused_by", 1, 2, 0.59),
        ("caused_by", 1, 4, 0.94),
        ("caused_by", 2, 4, 0.94),
        ("caused_by", 3, 4, 0.84),
    ]
)


def main():
    conn = fresh()

    # The deck's recognize call, as drawn.
    conn.execute("CREATE TABLE tickets(id integer primary key, body text)")
    conn.execute("INSERT INTO tickets(body) VALUES (?)", (C01,))
    drawn = conn.execute(
        "SELECT t.id, n.text, n.kind FROM tickets t, "
        "thinkthen_recognize(t.body, 'person,organization') n"
    ).fetchall()
    check(
        "the deck's recognize call returns rows, two kinds only",
        drawn,
        [(1, "Maria Chen", "person"), (1, "Northwind Freight", "organization")],
    )

    # The five ruled columns, with strength.
    five = conn.execute(
        "SELECT text, kind, start, end, strength FROM "
        "thinkthen_recognize(?, 'person,organization,place')",
        (C01,),
    ).fetchall()
    check(
        "the five columns carry kind, offsets, and strength",
        five,
        [
            ("Maria Chen", "person", 0, 10, 0.98),
            ("Northwind Freight", "organization", 18, 35, 1.0),
            ("Chicago", "place", 39, 46, 0.6693),
        ],
    )

    # No kinds given: the ruled default of person, organization, place.
    defaulted = conn.execute(
        "SELECT text FROM thinkthen_recognize(?, NULL)", (C01,)
    ).fetchall()
    check(
        "a NULL kinds argument takes the default three kinds",
        [row[0] for row in defaulted],
        ["Maria Chen", "Northwind Freight", "Chicago"],
    )

    # Offsets are SQLite's own indexing: substr slices the name exactly,
    # with an accented letter and an emoji ahead of it.
    for text, start, end in conn.execute(
        "SELECT text, start, end FROM thinkthen_recognize(?, 'person')",
        (EMOJI,),
    ).fetchall():
        sliced = conn.execute(
            "SELECT substr(?, ?, ?)", (EMOJI, start + 1, end - start)
        ).fetchone()[0]
        check(
            "the emoji text slices to the name with no conversion",
            (text, start, end, sliced),
            ("Maria Chen", 10, 20, "Maria Chen"),
        )

    # A text with no names: an empty result, exit 0.
    check(
        "a text with no names gives no rows",
        conn.execute("SELECT * FROM thinkthen_recognize(?, 'person')", (NOENT,)).fetchall(),
        [],
    )

    # The same name twice: two rows, two offsets, recorded ids distinct.
    dup = conn.execute(
        "SELECT text, kind, start, end FROM thinkthen_recognize(?, 'place')", (DUP,)
    ).fetchall()
    check(
        "a repeated name gives two rows with distinct offsets",
        dup,
        [
            ("Chicago", "place", 0, 7),
            ("Chicago", "place", 40, 47),
        ],
    )

    # A text the recordings do not hold is refused, naming the recording.
    try:
        conn.execute("SELECT * FROM thinkthen_recognize('unrecorded text', 'person')").fetchall()
        check("an unrecorded text is refused", "no error", "an error")
    except sqlite3.Error as failure:
        check(
            "an unrecorded text is refused, naming the recording",
            "no recorded answer" in str(failure),
            True,
        )

    # The deck's relate call, as drawn.
    conn.execute("CREATE TABLE alerts(id integer primary key, body text)")
    conn.executemany("INSERT INTO alerts(body) VALUES (?)", [(a,) for a in ALERTS])
    edges = sorted(
        conn.execute(
            "SELECT name, source, target, probability FROM "
            "thinkthen_relate('alerts', 'id', 'body', 'caused_by')"
        ).fetchall()
    )
    check("the deck's relate call returns the recorded edges", edges, DEMO_EDGES)

    # Two rules at once, and text ids riding through.
    both = sorted(
        conn.execute(
            "SELECT name, source, target, probability FROM "
            "thinkthen_relate('alerts', 'id', 'body', 'caused_by', 'same_as')"
        ).fetchall()
    )
    check("two relation arguments ride at once", both, DEMO_EDGES)
    conn.execute("CREATE TABLE alerts_text(alert_id text primary key, body text)")
    conn.executemany(
        "INSERT INTO alerts_text(body) VALUES (?)", [(a,) for a in ALERTS]
    )
    conn.execute("UPDATE alerts_text SET alert_id = 'a' || rowid")
    text_ids = sorted(
        conn.execute(
            "SELECT name, source, target, probability FROM "
            "thinkthen_relate('alerts_text', 'alert_id', 'body', 'caused_by')"
        ).fetchall()
    )
    check(
        "a text id column rides through the edges",
        text_ids,
        sorted(
            [
                ("caused_by", "a1", "a2", 0.59),
                ("caused_by", "a1", "a4", 0.94),
                ("caused_by", "a2", "a4", 0.94),
                ("caused_by", "a3", "a4", 0.84),
            ]
        ),
    )

    # Unrecorded records and unrecorded rules are refused, naming what is
    # missing and what the recording covers.
    conn.execute("CREATE TABLE strangers(id integer primary key, body text)")
    conn.execute("INSERT INTO strangers(body) VALUES ('nothing recorded here')")
    try:
        conn.execute(
            "SELECT * FROM thinkthen_relate('strangers', 'id', 'body', 'caused_by')"
        ).fetchall()
        check("unrecorded records are refused", "no error", "an error")
    except sqlite3.Error as failure:
        check(
            "unrecorded records are refused, naming the recording",
            "no recorded answer" in str(failure),
            True,
        )
    try:
        conn.execute(
            "SELECT * FROM thinkthen_relate('alerts', 'id', 'body', 'invented')"
        ).fetchall()
        check("an unrecorded rule is refused", "no error", "an error")
    except sqlite3.Error as failure:
        check(
            "an unrecorded rule is refused, naming the covered rules",
            "caused_by" in str(failure) and "same_as" in str(failure),
            True,
        )

    # More than 255 records is refused, naming the limit.
    conn.execute("CREATE TABLE big(n integer primary key, body text)")
    conn.executemany(
        "INSERT INTO big(body) VALUES (?)",
        [(f"row {i}",) for i in range(256)],
    )
    try:
        conn.execute(
            "SELECT * FROM thinkthen_relate('big', 'n', 'body', 'caused_by')"
        ).fetchall()
        check("256 records are refused", "no error", "an error")
    except sqlite3.Error as failure:
        check(
            "256 records are refused, naming the 255 limit",
            "255" in str(failure),
            True,
        )

    # "Names become rows": read each ticket once, then join by equality.
    # The whole pattern sends nothing, and the usage counter is the proof.
    conn.execute("CREATE TABLE accounts(name text, owner text)")
    conn.execute("INSERT INTO accounts VALUES ('Northwind Freight', 'Freight Team')")
    before = json.loads(conn.execute("SELECT thinkthen_usage()").fetchone()[0])
    conn.execute(
        "CREATE TABLE mentions AS SELECT t.id, n.text AS name, n.kind AS kind "
        "FROM tickets t, thinkthen_recognize(t.body, 'person,organization,place') n"
    )
    joined = conn.execute(
        "SELECT a.owner, count(*) AS tickets FROM mentions m "
        "JOIN accounts a ON m.name = a.name GROUP BY a.owner ORDER BY tickets DESC"
    ).fetchall()
    after = json.loads(conn.execute("SELECT thinkthen_usage()").fetchone()[0])
    check(
        "the join by equality answers the name's owner",
        joined,
        [("Freight Team", 1)],
    )
    check(
        "the whole pattern sends no request",
        (before["requests"], after["requests"]),
        (0, 0),
    )

    print("tvf suite done" if not FAILURES else "tvf suite diverged")
    sys.exit(1 if FAILURES else 0)


if __name__ == "__main__":
    main()
