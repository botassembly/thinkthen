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
def _library() -> "pathlib.Path":
    """The built extension, whatever the platform named it: .so or
    .dylib (the name is derived from the crate's `thinkthen0`)."""
    for candidate in (HERE.parent / "target" / "release").glob("libthinkthen0.*"):
        if candidate.suffix in (".so", ".dylib"):
            return candidate
    raise SystemExit("no built extension under target/release; run cargo build --release")


LIB = _library()

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
# The counters are cumulative, so every check here reads a delta between
# two snapshots (the punch-list ruling: no reset, subtract snapshots).
# Two of the five rows were already answered by the decide checks above,
# so warm judges the three uncached rows and serves the other two.
hits_before = json.loads(conn.execute("SELECT thinkthen_usage()").fetchone()[0])[
    "cache_answers"
]
check(
    "warm judges the uncached rows",
    conn.execute(
        "SELECT thinkthen_warm('Is this a complaint?', body) FROM reviews"
    ).fetchone()[0],
    3,
)
check(
    "warm serves the already-answered rows from the session map",
    json.loads(conn.execute("SELECT thinkthen_usage()").fetchone()[0])[
        "cache_answers"
    ]
    - hits_before,
    2,
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
check("served answers counted", after["cache_answers"] - before["cache_answers"] >= 10, True)

# usage is cumulative; the removed reset spelling refuses and names the
# subtraction pattern (the punch-list ruling).
try:
    conn.execute("SELECT thinkthen_usage('reset')").fetchall()
    check("the reset spelling refuses", "no error", "a usage error")
except sqlite3.OperationalError as failure:
    check(
        "the reset spelling refuses, naming the subtraction pattern",
        "cumulative" in str(failure) and "subtract" in str(failure),
        True,
    )
snapshot = json.loads(conn.execute("SELECT thinkthen_usage()").fetchone()[0])
conn.execute(
    "SELECT thinkthen_decide('Is this a complaint?', 'a sentence no earlier check asked')"
).fetchone()
grown = json.loads(conn.execute("SELECT thinkthen_usage()").fetchone()[0])
check(
    "the counters are cumulative, never reset",
    grown["requests"] == snapshot["requests"] + 1
    and grown["cache_answers"] >= snapshot["cache_answers"]
    and grown["tokens"] >= snapshot["tokens"],
    True,
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
        "a missing named file is a local failure that names no cause",
        "thinkthen local: the question file" in str(failure)
        and "did not read" in str(failure)
        and "No such file" not in str(failure),
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
        """{"version": 1, "questions": {
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

# The warm aggregate keys its groups by question (review 3, item 17):
# two questions over the same text judge twice, each under its own
# question, and both pairs land in the cache — the decide below answers
# from the cache, so the request counter does not move. Before the fix
# the first question judged the second's text, the (q2, text) pair never
# landed, and the decide below sent a new request.
warm_conn = fresh()
warm_conn.execute("CREATE TABLE t(q TEXT, body TEXT)")
warm_conn.executemany(
    "INSERT INTO t(q, body) VALUES (?, ?)",
    [
        ('{"decide":"Is this a complaint?","threshold":0.9}', "the shared text"),
        ('{"decide":"Is this spam?","threshold":0.9}', "the shared text"),
    ],
)
warm_conn.execute("SELECT thinkthen_warm(q, body) FROM t").fetchall()
sent = json.loads(warm_conn.execute("SELECT thinkthen_usage()").fetchone()[0])["requests"]
check("warm sent one round per question", sent >= 2, True)
warm_conn.execute(
    "SELECT thinkthen_decide('{\"decide\":\"Is this spam?\",\"threshold\":0.9}', 'the shared text')"
).fetchall()
after = json.loads(warm_conn.execute("SELECT thinkthen_usage()").fetchone()[0])["requests"]
check("the second question's pair is cached", after, sent)

# Every named file reads through the regular-file rule and the cap
# (review 3, item 2): a fifo refuses instead of blocking forever, an
# endless device refuses at once, and an over-cap file names the cap —
# with one message for every unreadable cause.
with tempfile.TemporaryDirectory() as held:
    fifo = os.path.join(held, "pipe.fifo")
    os.mkfifo(fifo)
    try:
        conn.execute(f"SELECT thinkthen_decide('@{fifo}', 'refund')").fetchone()
        check("a fifo refuses", "answered", "refused")
    except sqlite3.OperationalError as error:
        check(
            "a fifo refuses with the uniform message",
            "did not read" in str(error) and "regular file" in str(error),
            True,
        )
    try:
        conn.execute("SELECT thinkthen_decide('@/dev/zero', 'refund')").fetchone()
        check("an endless device refuses", "answered", "refused")
    except sqlite3.OperationalError as error:
        check("an endless device refuses with the uniform message", "did not read" in str(error), True)
    big = os.path.join(held, "big.json")
    with open(big, "wb") as handle:
        handle.write(b"x" * (1024 * 1024 + 1))
    try:
        conn.execute(f"SELECT thinkthen_decide('@{big}', 'refund')").fetchone()
        check("an over-cap file refuses", "answered", "refused")
    except sqlite3.OperationalError as error:
        check("an over-cap file names the cap", "over the 1048576 byte cap" in str(error), True)

print(f"{STEP - FAILURES} of {STEP} passed")
sys.exit(1 if FAILURES else 0)
