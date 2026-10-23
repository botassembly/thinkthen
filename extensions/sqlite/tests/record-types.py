"""Prove SQLite TEXT versus JSON subtype through the installed shell extension."""
import json
import math
from pathlib import Path
import subprocess
import sys

SHELL, EXTENSION, ROOT = sys.argv[1:]
sys.path.insert(0, str(Path(ROOT) / "databases"))
from testing import child_environment


def quote(value):
    return "'" + value.replace("'", "''") + "'"


def rows(source, options="{}", setup=""):
    sql = f""".bail on
.load {EXTENSION} sqlite3_thinkthensqlite_init
SELECT thinkthen_configure('{{"adapter":"stub"}}');
{setup}
.mode json
SELECT position,status,answer,p,error,detail FROM thinkthen_decide(
  {quote(source)},'Ready?',{quote(options)});
"""
    output = subprocess.run([SHELL, ":memory:"], input=sql, text=True,
                            capture_output=True, env=child_environment(), check=True).stdout
    start = output.index("[")
    result = json.loads(output[start:])
    for row in result:
        for key in ("answer", "error", "detail"):
            if row[key] is not None:
                row[key] = json.loads(row[key])
    return result


def same_rows(actual, expected):
    # SQLite 3.41's shell JSON can print 0.47 as 0.4699999999999999 on ARM.
    # The native consumer checks exact doubles; allow only that display error here.
    return len(actual) == len(expected) and all(
        math.isclose(row["p"], wanted["p"], rel_tol=0, abs_tol=1e-15)
        and {key: value for key, value in row.items() if key != "p"}
        == {key: value for key, value in wanted.items() if key != "p"}
        for row, wanted in zip(actual, expected)
    )


case = json.loads((Path(ROOT) / "construction/stub.json").read_text())[0]
expected = [{"position": n, "error": None, **{k: v for k, v in row.items() if k != "id"}}
            for n, row in enumerate(case["expected"], 1)]
values = ",".join(f"({n},{quote(text)})" for n, text in enumerate(case["request"]["records"], 1))
setup = f"CREATE TABLE input(n INTEGER PRIMARY KEY, body TEXT); INSERT INTO input VALUES {values};"
# Plain TEXT is not parsed as JSON. Use the shared construction answers.
actual = rows("SELECT body AS record FROM input ORDER BY n", setup=setup)
assert same_rows(actual, expected), (actual, expected)
# JSON constructors retain their subtype through the source statement.
assert same_rows(rows("SELECT json_object('body',body) AS record FROM input ORDER BY n",
                      '{"on":"/body"}', setup), expected)
assert same_rows(rows("SELECT json_array(body) AS record FROM input ORDER BY n",
                      '{"on":"/0"}', setup), expected)
# Stored JSON is TEXT until the caller explicitly applies json(column).
objects = ",".join(f"({n},{quote(json.dumps({'body': text}))})"
                   for n, text in enumerate(case["request"]["records"], 1))
setup = f"CREATE TABLE input(n INTEGER PRIMARY KEY, document TEXT); INSERT INTO input VALUES {objects};"
assert same_rows(rows("SELECT json(document) AS record FROM input ORDER BY n",
                      '{"on":"/body"}', setup), expected)
raw = rows("SELECT document AS record FROM input ORDER BY n", '{"on":"/body"}', setup)
quoted = rows("SELECT json_quote(document) AS record FROM input ORDER BY n", setup=setup)
assert len(raw) == len(expected) and all(row["status"] == "answered" for row in raw)
assert same_rows(raw, quoted) and not same_rows(raw, expected), (raw, quoted, expected)
print(f"SQLite {subprocess.check_output([SHELL, '--version'], text=True).split()[0]}: TEXT, JSON subtype, JSON-looking TEXT and stored JSON pass")

# Unindexed ORDER BY must never silently turn a JSON object/array into text.
setup = "CREATE TABLE input(n INTEGER PRIMARY KEY, sort_key TEXT, body TEXT);"
setup += "INSERT INTO input VALUES(1,'b','alpha'),(2,'a','evidence 5'),(3,'c','alpha');"
orders = ("sort_key", "sort_key DESC", "lower(sort_key)", "sort_key LIMIT 2", "n")
failures = []
for expression, options in (("json_object('body',body)", '{"on":"/body"}'),
                            ("json_array(body)", '{"on":"/0"}')):
    for order in orders:
        source = f"SELECT {expression} AS record FROM input ORDER BY {order}"
        text_source = f"SELECT body AS record FROM input ORDER BY {order}"
        wanted = rows(text_source, setup=setup)
        try:
            actual = rows(source, options, setup)
        except subprocess.CalledProcessError as failure:
            assert "stage" in failure.stderr and "INTEGER PRIMARY KEY" in failure.stderr, failure.stderr
        else:
            if actual != wanted:
                failures.append(source)
        # Staging preserves arbitrary order independently of subtype propagation.
        stage = setup + f"CREATE TEMP TABLE ordered_input(position INTEGER PRIMARY KEY, document TEXT); INSERT INTO ordered_input(document) SELECT {expression} FROM input ORDER BY {order};"
        assert rows("SELECT json(document) AS record FROM ordered_input ORDER BY position", options, stage) == wanted

# A final constructor after a nested sort remains JSON; other values remain text.
assert rows("SELECT json_object('body',body) AS record FROM (SELECT body FROM input ORDER BY sort_key LIMIT -1)",
            '{"on":"/body"}', setup) == rows("SELECT body AS record FROM input ORDER BY sort_key", setup=setup)
for text in ('ordinary message', '{broken', '[broken', 'null', '42', '"a string"'):
    source = "SELECT body AS record FROM input ORDER BY sort_key"
    values = f"CREATE TABLE input(sort_key TEXT, body TEXT); INSERT INTO input VALUES('b',{quote(text)}),('a',{quote(text)});"
    assert rows(source, setup=values) == rows("SELECT body AS record FROM input", setup=values)
for text in ('{"body":"alpha"}', '["alpha"]', '{"huge":1e400}', '{"wide":18446744073709551615}'):
    values = f"CREATE TABLE input(sort_key TEXT, body TEXT); INSERT INTO input VALUES('b',{quote(text)}),('a',{quote(text)});"
    try:
        rows("SELECT body AS record FROM input ORDER BY sort_key", setup=values)
    except subprocess.CalledProcessError as failure:
        assert "stage" in failure.stderr and "INTEGER PRIMARY KEY" in failure.stderr, failure.stderr
    else:
        failures.append(("ambiguous sorted JSON-looking TEXT", text))
assert not failures, failures
print("SQLite sorted JSON retains its record kind or explains staging; ordinary sorted text stays text")
