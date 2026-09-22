#!/usr/bin/env python3
"""The SQLite surface's null suite: every ruled behavior, on the null
backend, with no wire. Run through ./check.sh."""

import json
import os
import pathlib
import sqlite3
import sys
import tempfile

HERE = pathlib.Path(__file__).resolve().parent
LIB = HERE.parent / "target" / "release" / "libthinkthen0.so"

os.environ.setdefault("ENGINE_NULL", "1")

FAILURES = 0


def check(name, held, wanted):
    global FAILURES
    if held == wanted:
        print(f"ok  {n()} {name}")
    else:
        FAILURES += 1
        print(f"FAIL {n()} {name}: held {held!r}, wanted {wanted!r}")


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


conn = fresh()
conn.execute("CREATE TABLE reviews(id INTEGER PRIMARY KEY, body TEXT)")
rows = [
    ("i want a refund now",),
    ("good morning",),
    ("refund, please",),
    ("maybe later",),
    ("see you",),
]
conn.executemany("INSERT INTO reviews(body) VALUES (?)", rows)

# decide: a cut gives 1 and 0, a band gives NULL inside it.
check(
    "decide cut yes",
    conn.execute(
        "SELECT thinkthen_decide('Is this a complaint?', 'i want a refund now')"
    ).fetchone()[0],
    1,
)
check(
    "decide cut no",
    conn.execute(
        "SELECT thinkthen_decide('Is this a complaint?', 'good morning')"
    ).fetchone()[0],
    0,
)
check(
    "decide band unsure reads NULL",
    conn.execute(
        """SELECT thinkthen_decide('{"decide":"Is this a complaint?","threshold":"0.2:0.8"}',
               'maybe later')"""
    ).fetchone()[0],
    None,
)

# The filter pattern: warm fills, WHERE reads, count agrees, no new sends.
conn.execute("SELECT thinkthen_usage('reset')")
check(
    "warm judges every row",
    conn.execute(
        "SELECT thinkthen_warm('Is this a complaint?', body) FROM reviews"
    ).fetchone()[0],
    5,
)
before = json.loads(conn.execute("SELECT thinkthen_usage()").fetchone()[0])
kept = conn.execute(
    "SELECT id FROM reviews WHERE thinkthen_decide('Is this a complaint?', body)"
).fetchall()
middle = json.loads(conn.execute("SELECT thinkthen_usage()").fetchone()[0])
counted = conn.execute(
    "SELECT count(*) FROM reviews WHERE thinkthen_decide('Is this a complaint?', body)"
).fetchone()[0]
after = json.loads(conn.execute("SELECT thinkthen_usage()").fetchone()[0])
check("filter keeps the complaint rows", [row[0] for row in kept], [1, 3, 4])
check("count agrees with the filter", counted, 3)
check("the WHERE passes send nothing", middle["requests"], before["requests"])
check("the count sends nothing", after["requests"], before["requests"])
check("served answers counted", after["cache_answers"] >= 10, True)

# usage and its reset arm.
check(
    "usage reset zeroes the counters",
    json.loads(conn.execute("SELECT thinkthen_usage('reset')").fetchone()[0]),
    {"requests": 0, "cache_answers": 0, "tokens": 0},
)

# A question file named with the command's spelling.
with tempfile.NamedTemporaryFile(
    "w", suffix=".json", delete=False, dir=HERE.parent
) as handle:
    handle.write('{"decide": "Does the writer ask for a refund?", "threshold": 0.5}')
    named = handle.name
check(
    "@file question answers",
    conn.execute(
        f"SELECT thinkthen_decide('@{os.path.basename(named)}', 'refund, please')"
    ).fetchone()[0],
    1,
)
conn.execute(
    f"SELECT thinkthen_decide('@{os.path.basename(named)}', 'see you')"
).fetchone()
try:
    conn.execute(
        "SELECT thinkthen_decide('@no-such-question-file.json', 'text')"
    ).fetchall()
    check("a missing named file raises", "no error", "an error")
except sqlite3.OperationalError as failure:
    check(
        "a missing named file is a local failure that names the kind",
        "thinkthen local: cannot read question file" in str(failure),
        True,
    )

# A blank question is a usage failure and never reads as a value.
try:
    conn.execute("SELECT thinkthen_decide('   ', 'text')").fetchone()
    check("blank question raises", "no error", "an error")
except sqlite3.OperationalError as failure:
    check("blank question names usage", "thinkthen usage" in str(failure), True)

# score is the specification's number; the level rides in details.
check(
    "score is a REAL between 0 and K-1",
    0.0
    <= conn.execute(
        """SELECT thinkthen_score('{"score":"How urgent?","levels":["low","mid","high"]}',
               'i want a refund now')"""
    ).fetchone()[0]
    <= 2.0,
    True,
)
audit = json.loads(
    conn.execute(
        "SELECT thinkthen_details('Is this a complaint?', 'i want a refund now')"
    ).fetchone()[0]
)
check("details carries a digest", len(audit.get("digest", "")), 64)
check("details counts sends", audit.get("sends"), 1)
check("details nearest is null off score", audit.get("nearest"), None)
score_audit = json.loads(
    conn.execute(
        "SELECT thinkthen_details('{\"score\":\"How strong?\",\"levels\":[\"low\",\"mid\",\"high\"]}', 'maybe later')"
    ).fetchone()[0]
)
check("details carries the nearest level for a score", score_audit.get("nearest"), "mid")

# tag answers a JSON array in the question's order.
check(
    "tag holds the marked labels",
    json.loads(
        conn.execute(
            """SELECT thinkthen_tag('{"tag":"Mark it","labels":["refund","angry"]}',
                   'i want a refund now')"""
        ).fetchone()[0]
    ),
    ["refund"],
)

# annotate answers one object with a field per named question.
with tempfile.NamedTemporaryFile(
    "w", suffix=".json", delete=False, dir=HERE.parent
) as handle:
    handle.write(
        """{"questions": {
            "spam": {"decide": "Is this spam?", "threshold": 0.5},
            "band": {"decide": "Is this spam?", "threshold": "0.2:0.8"}
        }}"""
    )
    form = handle.name
fields = json.loads(
    conn.execute(
        f"SELECT thinkthen_annotate('@{os.path.basename(form)}', 'maybe later')"
    ).fetchone()[0]
)
check("annotate names its fields", sorted(fields.keys()), ["band", "spam"])
check("annotate's band field is unsure", fields["band"]["answer"], None)

os.unlink(named)
os.unlink(form)
print(f"{STEP - FAILURES} of {STEP} passed")
sys.exit(1 if FAILURES else 0)
