#!/usr/bin/env python3
"""Explicit initial loading permits reviewed schema calls on one connection."""

import os
import pathlib
import subprocess
import sys

from helper import CLI, LIB, Backend, child, environment, expect, main
from test_schema import ATTACK, DECIDE

TRUSTED = """
def trusted_connect(path=":memory:", setting="ON", authorizer=None):
    db = sqlite3.connect(path, isolation_level=None)
    db.enable_load_extension(True)
    db.execute(f"PRAGMA trusted_schema={setting}")
    if authorizer:
        db.set_authorizer(authorizer)
    db.execute("SELECT load_extension(?, 'sqlite3_thinkthen_trusted_init')", (LIB,))
    return db
"""


def test_trusted_views_and_ingestion_trigger_answer_across_schemas() -> None:
    backend = Backend()
    held = child(TRUSTED + r"""
db = trusted_connect()
db.execute('SELECT thinkthen_configure(?)', ('{"cache":false}',))
db.execute("CREATE TABLE t(body TEXT, answer INTEGER)")
db.execute("CREATE VIEW v AS SELECT thinkthen_decide('Is it red?', body) FROM t")
db.execute("CREATE TRIGGER tr AFTER INSERT ON t BEGIN UPDATE t SET answer=thinkthen_decide('Is it red?', NEW.body) WHERE rowid=NEW.rowid; END")
db.execute("INSERT INTO t(body) VALUES ('a red scarf')")
trigger = run(db, "SELECT answer FROM t")
view = run(db, "SELECT * FROM v")
db.execute("CREATE TEMP VIEW temporary AS SELECT thinkthen_decide('Is it red?', 'a red door')")
temp = run(db, "SELECT * FROM temporary")
db.execute("ATTACH ':memory:' AS later")
db.execute("CREATE VIEW later.v AS SELECT thinkthen_decide('Is it red?', 'a red coat')")
attached = run(db, "SELECT * FROM later.v")
import pathlib
question = pathlib.Path(os.environ['SCRATCH']) / 'question.json'
question.write_text('{"decide":"Is it red?"}')
db.execute("CREATE VIEW named AS SELECT thinkthen_decide('@" + str(question) + "', 'a red book')")
named = run(db, "SELECT * FROM named")
ordinary = connect()
ordinary.execute("PRAGMA trusted_schema=ON")
ordinary.execute("CREATE VIEW v AS SELECT thinkthen_decide('Is it red?', 'red')")
say(trigger=trigger, view=view, temp=temp, attached=attached, named=named,
    ordinary=run(ordinary, "SELECT * FROM v"))
""", environment(backend))
    expect(held, {"trigger": [[1]], "view": [[1]], "temp": [[1]], "attached": [[1]], "named": [[1]],
                  "ordinary": "unsafe use of thinkthen_decide()"}, "connection-wide trusted schema calls")
    expect(backend.close(), 5, "five schema judgments, default connection sent nothing")


def test_trusted_keyed_view_uses_one_packed_request() -> None:
    backend = Backend()
    held = child(TRUSTED + r"""
db = trusted_connect()
db.execute("CREATE VIEW packed AS SELECT key,value FROM thinkthen_decide_many('Is it red?', '{\"a\":\"a red coat\",\"b\":\"another red coat\"}')")
say(rows=run(db, "SELECT * FROM packed"))
""", environment(backend))
    expect(held, {"rows": [["a", 1], ["b", 1]]}, "trusted keyed table view")
    expect(backend.close(), 1, "one packed schema request")


def test_trusted_schema_off_retains_every_hostile_refusal() -> None:
    backend = Backend()
    # Reuse the crafted hostile databases, including named-file arguments.
    attack = ATTACK[:ATTACK.index("said = {}")]
    held = child(TRUSTED + f"DECIDE = {DECIDE!r}\n" + attack + r"""
said = {}
for label, (steps, use) in shapes.items():
    path = craft(label.replace(" ", "-"), steps)
    db = trusted_connect(setting="OFF")
    attached = run(db, "ATTACH DATABASE ? AS other", (str(path),))
    said[f"{label} (OFF)"] = attached if isinstance(attached, str) else run(db, use)
db.execute("CREATE VIEW keyed AS SELECT * FROM thinkthen_decide_many('Is it red?', '{\"a\":\"red\"}')")
said['a view over keyed judgments (OFF)'] = run(db, 'SELECT * FROM keyed')
say(**said)
""", environment(backend))
    unsafe = "unsafe use of thinkthen_decide()"
    malformed = "malformed database schema ({}) - " + unsafe
    wanted = {"a CHECK constraint": malformed.format("t"), "a DEFAULT": unsafe, "a view": unsafe,
              "a trigger": unsafe, "a generated column": malformed.format("t2"),
              "an index expression": malformed.format("x"), "a partial index": malformed.format("x"),
              "a view over recognize": 'unsafe use of virtual table "thinkthen_recognize"',
              "a view over keyed judgments": 'unsafe use of virtual table "thinkthen_decide_many"'}
    expect(held, {f"{label} (OFF)": result for label, result in wanted.items()}, "trusted OFF hostile shapes")
    expect(backend.close(), 0, "all schema refusals sent nothing")


def test_trusted_registration_keeps_controls_direct_only_and_judgments_volatile() -> None:
    backend = Backend()
    held = child(TRUSTED + r"""
db = trusted_connect(setting="OFF")
initial = db.execute("PRAGMA trusted_schema").fetchone()[0]
db.execute("PRAGMA trusted_schema=ON")
rows = db.execute("SELECT name, narg, flags FROM pragma_function_list WHERE name LIKE 'thinkthen%'").fetchall()
judgments = {'thinkthen_decide', 'thinkthen_choose', 'thinkthen_score', 'thinkthen_tag', 'thinkthen_annotate',
             'thinkthen_details', 'thinkthen_try_details', 'thinkthen_find', 'thinkthen_relations', 'thinkthen_plan'}
calls = {'configure': "thinkthen_configure('{}')", 'budget': 'thinkthen_budget_ms(0)', 'usage': 'thinkthen_usage()',
         'removed': 'thinkthen_warm()', 'positional': "thinkthen_decide('Is it red?', 'red', 0, '')"}
refusals = {}
for label, call in calls.items():
    db.execute(f"CREATE VIEW {label} AS SELECT {call}")
    refusals[label] = run(db, f"SELECT * FROM {label}")
db.execute("CREATE VIEW preview AS SELECT thinkthen_plan('Is it red?', '{\"a\":\"red\"}')")
plan = json.loads(db.execute("SELECT * FROM preview").fetchone()[0])
db.execute("CREATE TABLE t(body TEXT)")
say(initial=initial, registered=bool(rows),
    direct=[name for name, arity, flags in rows if name in judgments and arity != 4 and flags & 0x80000],
    controls=all(flags & 0x80000 for name, arity, flags in rows if name not in judgments or arity == 4),
    innocuous=[name for name, _, flags in rows if flags & 0x200000],
    deterministic=[name for name, _, flags in rows if flags & 0x800], refusals=refusals,
    plan=plan['records'], index=run(db, "CREATE INDEX x ON t(thinkthen_decide('Is it red?', body))"))
""", environment(backend))
    expect(held, {"initial": 0, "registered": True, "direct": [], "controls": True, "innocuous": [],
                  "deterministic": [], "refusals": {"configure": "unsafe use of thinkthen_configure()",
                  "budget": "unsafe use of thinkthen_budget_ms()", "usage": "unsafe use of thinkthen_usage()",
                  "removed": "unsafe use of thinkthen_warm()", "positional": "unsafe use of thinkthen_decide()"}, "plan": 1,
                  "index": "non-deterministic functions prohibited in index expressions"}, "trusted function contract")
    expect(backend.close(), 0, "controls, preview and index refusal sent nothing")


def test_initial_load_preserves_authorizer_and_active_reload_preserves_mode() -> None:
    held = child(TRUSTED + r"""
def deny_delete(action, *unused):
    return sqlite3.SQLITE_DENY if action == sqlite3.SQLITE_DELETE else sqlite3.SQLITE_OK
results = []
for entry in ('sqlite3_thinkthen_init', 'sqlite3_thinkthen_trusted_init'):
    db = sqlite3.connect(':memory:', isolation_level=None)
    db.enable_load_extension(True)
    db.set_authorizer(deny_delete)
    db.execute('SELECT load_extension(?, ?)', (LIB, entry))
    db.execute('CREATE TABLE t(body TEXT)')
    db.execute("INSERT INTO t VALUES ('red')")
    deletion = run(db, 'DELETE FROM t')
    before = db.execute("SELECT flags FROM pragma_function_list WHERE name='thinkthen_decide' ORDER BY narg").fetchall()
    opposite = 'sqlite3_thinkthen_trusted_init' if entry == 'sqlite3_thinkthen_init' else 'sqlite3_thinkthen_init'
    reload = run(db, 'SELECT load_extension(?, ?)', (LIB, opposite))
    after = db.execute("SELECT flags FROM pragma_function_list WHERE name='thinkthen_decide' ORDER BY narg").fetchall()
    results.append([deletion, before == after, reload])
say(results=results)
""", environment(None))
    for deletion, unchanged, reload in held["results"]:
        expect(deletion, "not authorized", "existing authorizer still denies deletion")
        expect(unchanged, True, "failed active reload preserves registration")
        expect(reload, "error during initialization: unable to delete/modify user-function due to active statements",
               "SQLite refuses changing mode through active SQL")


def test_native_temp_views_remain_caller_sql_in_both_modes_under_off() -> None:
    backend = Backend()
    env = environment(backend)
    env.pop("THINKTHEN_API_KEY")
    question = pathlib.Path(env["SCRATCH"]) / "empty-question.json"
    question.write_text('{"decide":""}')
    for entry in ("sqlite3_thinkthen_init", "sqlite3_thinkthen_trusted_init"):
        held = subprocess.run([CLI, "-batch", ":memory:"], env=env, capture_output=True, text=True,
                              input=f"""
.bail on
.load {LIB} {entry}
PRAGMA trusted_schema=OFF;
SELECT sqlite_version();
PRAGMA trusted_schema;
CREATE TEMP VIEW preview AS SELECT thinkthen_plan('Is it red?', '{{"a":"red"}}') AS answer;
SELECT json_extract(answer, '$.records') FROM preview;
CREATE TEMP VIEW packed AS SELECT * FROM thinkthen_decide_many('Is it red?', '{{}}');
SELECT count(*) FROM packed;
CREATE TEMP VIEW controls AS SELECT thinkthen_budget_ms(0) AS answer;
SELECT answer FROM controls;
CREATE TEMP VIEW totals AS SELECT thinkthen_usage() AS answer;
SELECT json_extract(answer, '$.requests_sent') FROM totals;
CREATE TEMP VIEW named AS SELECT thinkthen_decide('@{question}', 'red');
SELECT * FROM named;
""")
        expect(held.stdout.splitlines(), ["3.50.0", "0", "1", "0", "0", "0"],
               f"{entry}: pinned host permits keyless TEMP plan, table and controls under OFF")
        expect(held.returncode, 1, f"{entry}: TEMP question-file callback reaches its local parser refusal")
        expect("thinkthen local:" in held.stderr, True, f"{entry}: file parser reports a local error")
        expect("unsafe use" in held.stderr, False, f"{entry}: SQLite permits caller-created TEMP objects")
    expect(backend.close(), 0, "keyless TEMP previews, empty tables and local refusals send nothing")


if __name__ == "__main__":
    os.environ.pop("THINKTHEN_API_KEY", None)
    sys.exit(main(globals()))
