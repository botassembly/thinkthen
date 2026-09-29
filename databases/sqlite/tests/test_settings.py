#!/usr/bin/env python3
"""SQLite conversion of the shared engine settings and removed SQL names."""

from __future__ import annotations

import json
import os
import pathlib
import sys
import tempfile
import time

from helper import Backend, Child, child, environment, expect, main

AFTER_BUILD = "thinkthen usage: settings apply before the first call; this process already built its engine"
CORPUS = pathlib.Path(__file__).resolve().parents[3] / "conformance/settings.json"


def recording_entries(folder: str) -> int:
    return sum(path.is_file() and path.name != ".thinkthen-backend.json"
               for path in pathlib.Path(folder).rglob("*.json"))


def test_recording_counter_excludes_lock_files() -> None:
    """A cache lock is not a recorded answer (old clean package failure B2)."""
    with tempfile.TemporaryDirectory() as folder:
        root = pathlib.Path(folder)
        (root / ".locks").mkdir()
        (root / ".locks" / "one.lock").write_text("lock")
        (root / "one.json").write_text("{}")
        (root / ".thinkthen-backend.json").write_text("{}")
        expect(recording_entries(folder), 1, "only recorded JSON answers count")


def test_shared_settings_corpus() -> None:
    """The retained nine-case corpus crosses SQLite's one-object converter."""
    corpus = json.loads(CORPUS.read_text())
    expect(corpus["schema"], "thinkthen.settings-cases/1", "settings corpus version")
    for shared in corpus["cases"]:
        backend = Backend()
        with tempfile.TemporaryDirectory() as folder:
            for step in shared["steps"]:
                configured = {}
                for name, value in step["settings"].items():
                    if value == "$PROFILE":
                        value = json.dumps(shared["profile"], separators=(",", ":"))
                    elif value == "$FOLDER":
                        value = folder
                    configured[name] = value
                if step.get("verb") == "decide_many":
                    configured["batch"] = 1
                source = json.dumps(configured)
                setup_rows = ""
                if step.get("verb") == "relate":
                    entities = [(at, one["name"], one["kind"]) for at, one in enumerate(shared["entities"])]
                    setup_rows = ("db.execute('CREATE TABLE e(id INTEGER, name TEXT, kind TEXT)')\n"
                                  f"db.executemany('INSERT INTO e VALUES (?, ?, ?)', {entities!r})\n")
                    sql = f"SELECT count(*) FROM thinkthen_relate('SELECT id, name, kind FROM e', '{shared['relation']}')"
                elif step.get("verb") == "decide_many":
                    packed = json.dumps({str(at): value for at, value in enumerate(step["records"])})
                    sql = "SELECT count(*) FROM thinkthen_decide_many('Is this a refund?', '" + packed.replace("'", "''") + "')"
                elif "model" in step:
                    sql = f"SELECT json_extract(thinkthen_details('Is this a refund?', '{step['text']}'), '$.meta.model')"
                else:
                    sql = f"SELECT thinkthen_decide('Is this a refund?', '{step['text']}')"
                result = child(f"db = connect()\n"
                               f"assert not isinstance(run(db, 'SELECT thinkthen_configure(?)', ({source!r},)), str)\n"
                               f"{setup_rows}say(value=run(db, {sql!r}))\n",
                               environment(backend, shared["arm"].removesuffix("/v1")))["value"]
                label = shared["id"]
                if "error" in step:
                    expect(isinstance(result, str) and result.startswith(f"thinkthen {step['error']}"), True, label)
                elif "model" in step:
                    expect(result, [[step["model"]]], label)
                elif step.get("verb") == "relate":
                    expect(result, [[step["edges"]]], label)
                else:
                    expect(result, [[1]], label)
                expect(backend.count(), step["count"], f"{label} listener count")
            if "entries" in shared:
                expect(recording_entries(folder), shared["entries"], f"{shared['id']} saved entries")
        backend.close()


def test_configure_is_transactional_and_stops_after_build() -> None:
    backend = Backend()
    held = child("""
db = connect()
zero = run(db, "SELECT thinkthen_usage()")
invalid = [run(db, "SELECT thinkthen_configure(?)", (text,)) for text in
           ('{"batch":0}', '{"throttle":33}', '{"unknwon":1}', '{"batch":1,"batch":2}', '{"base_url":"https://example.invalid"}')]
valid = run(db, "SELECT thinkthen_configure(?)", ('{"batch":1,"throttle":8}',))
answer = run(db, "SELECT thinkthen_decide('Is it red?', 'a red door')")
after = run(db, "SELECT thinkthen_configure('{}')")
say(zero=zero, invalid=invalid, valid=valid, answer=answer, after=after)
""", environment(backend))
    expect(json.loads(held["zero"][0][0])["requests_sent"], 0, "usage builds no engine")
    expect(all(value.startswith("thinkthen usage:") for value in held["invalid"]), True, "five invalid objects")
    expect(held["valid"], [['{"batch":1,"throttle":8}']], "the selected object")
    expect(held["answer"], [[1]], "the first call")
    expect(held["after"], AFTER_BUILD, "late configuration")
    expect(backend.close(), 1, "only the valid call sent")


def test_removed_setters_and_warm_refuse_by_name() -> None:
    backend = Backend()
    held = child("""
db = connect()
names = ('throttle','batch','max_requests','max_request_bytes','max_requests_total',
         'cache','model','timeout','max_retries','profile','record','replay')
say(refused={name: run(db, 'SELECT thinkthen_' + name + '(1)') for name in names},
    warm=run(db, "SELECT thinkthen_warm('q','e')"),
    old=run(db, "SELECT thinkthen_recognize_document('x','{}')"),
    deadline=run(db, "SELECT thinkthen_decide('q','e',-1)"))
""", environment(backend))
    for name, result in held["refused"].items():
        expect(result, f"thinkthen usage: thinkthen_{name} was replaced by thinkthen_configure", name)
    expect(held["warm"], "thinkthen usage: thinkthen_warm was removed; pack records with thinkthen_decide_many", "warm")
    expect(held["old"], "thinkthen usage: thinkthen_recognize_document was renamed thinkthen_relations", "relations rename")
    expect(held["deadline"], "thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'", "old deadline")
    expect(backend.close(), 0, "removed names send nothing")


def test_environment_cache_seed_survives_configure() -> None:
    backend = Backend()
    env = environment(backend)
    held = child("""
db = connect()
say(config=run(db, "SELECT thinkthen_configure(?)", ('{"throttle":8}',)),
    first=run(db, "SELECT thinkthen_decide('Is this a complaint?', 'i want a refund')"),
    second=run(db, "SELECT thinkthen_decide('Is this a complaint?', 'i want a refund')"))
""", env)
    expect(held["first"], [[1]], "first answer")
    expect(held["second"], [[1]], "cache answer")
    expect(backend.close(), 1, "one send")
    expect(any(path.suffix == ".json" for path in pathlib.Path(env["THINKTHEN_CACHE"]).rglob("*.json")), True, "seeded cache")


def test_throttle_holds_eight_before_the_ninth() -> None:
    """Stress selector only: an owned packed call holds eight live requests."""
    backend = Backend()
    packed = json.dumps({str(at): f"row {at}" for at in range(18)})
    waiting = Child(f"""
db = connect()
db.execute("SELECT thinkthen_configure(?)", ('{{"throttle":8,"batch":2}}',))
say(rows=run(db, "SELECT count(*) FROM thinkthen_decide_many(?, ?)", ("Is it red?", {packed!r})))
""", environment(backend, "arm/held"))
    expect(backend.wait(8), 8, "eight held arrivals")
    time.sleep(0.3)
    expect(backend.count(), 8, "ninth has not started")
    backend.release()
    expect(waiting.result()["rows"], [[18]], "all records")
    expect(backend.close(), 9, "nine packed requests")


if __name__ == "__main__":
    os.chdir(pathlib.Path(__file__).resolve().parent)
    stress = os.environ.get("THINKTHEN_TEST_PROFILE") == "stress"
    sys.exit(main({name: value for name, value in globals().items()
                   if name.startswith("test_") and (name == "test_throttle_holds_eight_before_the_ninth") == stress}))
