#!/usr/bin/env python3
"""SQLite conversion of the shared engine settings and removed SQL names."""

from __future__ import annotations

import json
import datetime
import fcntl
import os
import pathlib
import sqlite3
import sys
import tempfile
import time

from helper import Backend, Child, child, environment, expect, main

AFTER_BUILD = "thinkthen usage: settings apply before the first call; this process already built its engine (retryable: no)"
CORPUS = pathlib.Path(__file__).resolve().parents[3] / "conformance/settings.json"


def recording_entries(folder: str) -> int:
    """The answers a recording keeps: one store row per question (ADR 0111)."""
    store = pathlib.Path(folder) / "thinkthen.sqlite"
    if not store.is_file():
        return 0
    with sqlite3.connect(store) as connection:
        return connection.execute("SELECT count(*) FROM answers").fetchone()[0]


def test_usage_status_keeps_good_answers_and_observes_writer_failure() -> None:
    backend = Backend()
    env = environment(backend)
    usage = pathlib.Path(env["XDG_STATE_HOME"]) / "thinkthen"
    usage.mkdir(mode=0o700, parents=True)
    with (usage / ".lock").open("w") as lock:
        os.chmod(lock.name, 0o600)
        fcntl.flock(lock, fcntl.LOCK_EX)
        worker = Child("""
import time
db = connect()
before = json.loads(run(db, "SELECT thinkthen_usage_status()")[0][0])
configure = run(db, 'SELECT thinkthen_configure(?)', ('{"cache":false}',))
answer = run(db, "SELECT thinkthen_decide('Is it a refund?', 'refund now')")
pending = json.loads(run(db, "SELECT thinkthen_usage_status()")[0][0])
say(before=before, answer=answer, pending=pending)
sys.stdin.readline()
end = time.monotonic() + 3
while time.monotonic() < end:
    status = json.loads(run(db, "SELECT thinkthen_usage_status()")[0][0])
    if status['state'] == 'failed': break
    time.sleep(.01)
say(status=status, again=json.loads(run(db, "SELECT thinkthen_usage_status()")[0][0]),
    usage=json.loads(run(db, "SELECT thinkthen_usage()")[0][0]))
""", env)
        started = worker.read()
        expect(started, {"before": {"state": "disabled"}, "answer": [[1]],
                         "pending": {"state": "pending"}}, "live status keeps good answer")
        month = usage / (datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m") + ".json")
        month.write_text("not JSON")
        month.chmod(0o600)
        fcntl.flock(lock, fcntl.LOCK_UN)
        worker.send()
        done = worker.result()
    failure = {"state": "failed", "advice": "check the usage folder permissions and free space"}
    expect(done["status"], failure, "safe writer failure")
    expect(done["again"], failure, "latched failure")
    expect(done["usage"]["requests_sent"], 1, "in-memory counts survive failure")
    expect(backend.close(), 1, "observation adds no send")


def test_usage_status_written_does_not_build_an_engine() -> None:
    backend = Backend()
    done = child("""
import time
db = connect()
before = json.loads(run(db, "SELECT thinkthen_usage_status()")[0][0])
configured = run(db, 'SELECT thinkthen_configure(?)', ('{"cache":false}',))
answer = run(db, "SELECT thinkthen_decide('Is it a refund?', 'refund now')")
end = time.monotonic() + 3
while time.monotonic() < end:
    status = json.loads(run(db, "SELECT thinkthen_usage_status()")[0][0])
    if status['state'] == 'written': break
    time.sleep(.01)
say(before=before, status=status, answer=answer)
""", environment(backend))
    expect(done, {"before": {"state": "disabled"}, "status": {"state": "written"},
                  "answer": [[1]]}, "written current deltas and unbuilt observation")
    expect(backend.close(), 1, "written observation adds no send")


def test_recording_counter_excludes_lock_files() -> None:
    """A cache lock is not a recorded answer (old clean package failure B2)."""
    with tempfile.TemporaryDirectory() as folder:
        root = pathlib.Path(folder)
        (root / ".locks").mkdir()
        (root / ".locks" / "one.lock").write_text("lock")
        (root / "one.json").write_text("{}")
        (root / ".thinkthen-backend.json").write_text("{}")
        with sqlite3.connect(root / "thinkthen.sqlite") as connection:
            connection.execute("CREATE TABLE answers(key TEXT)")
            connection.execute("INSERT INTO answers VALUES ('one')")
        connection.close()
        expect(recording_entries(folder), 1, "only the store's answer rows count")


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
valid = run(db, "SELECT thinkthen_configure(?)", ('{"batch":1,"throttle":8,"model":"judge-b"}',))
invalid = [run(db, "SELECT thinkthen_configure(?)", (text,)) for text in
           ('{"batch":0}', '{"throttle":33}', '{"unknwon":1}', '{"batch":1,"batch":2}',
            '{"base_url":"http://"}', '{"max_estimated_input_tokens_total":1}',
            '{"usd_per_million_input":"0"}')]
answer = run(db, "SELECT thinkthen_details('Is it red?', 'a red door')")
after = run(db, "SELECT thinkthen_configure('{}')")
say(zero=zero, invalid=invalid, valid=valid, answer=answer, after=after)
""", environment(backend))
    expect(json.loads(held["zero"][0][0])["requests_sent"], 0, "usage builds no engine")
    expect(all(value.startswith("thinkthen usage:") for value in held["invalid"]), True, "seven invalid objects")
    expect(held["invalid"][-1],
           "thinkthen usage: settings prices require both input and output fields (retryable: no)", "paired host prices")
    expect(held["invalid"][-2],
           "thinkthen usage: settings JSON has unknown key max_estimated_input_tokens_total (retryable: no)", "unimplemented host cap")
    expect(held["valid"], [['{"batch":1,"throttle":8,"model":"judge-b"}']], "the selected object")
    expect(json.loads(held["answer"][0][0])["meta"]["model"], "judge-b", "prior selection survived")
    expect(held["after"], AFTER_BUILD, "late configuration")
    expect(backend.close(), 1, "only the valid call sent")


def test_zero_total_refuses_without_replacing_valid_configuration() -> None:
    """SQLite keeps its 1+ rule and a failed replacement keeps prior settings."""
    backend = Backend()
    held = child("""
db = connect()
valid = run(db, "SELECT thinkthen_configure(?)", ('{"model":"judge-b","max_requests_total":null}',))
zero = run(db, "SELECT thinkthen_configure(?)", ('{"max_requests_total":0}',))
details = run(db, "SELECT thinkthen_details('Is it red?', 'a red door')")
say(valid=valid, zero=zero, details=details)
""", environment(backend))
    expect(held["valid"], [['{"model":"judge-b","max_requests_total":null}']], "null resets the total")
    expect(held["zero"], "thinkthen usage: a request total is a whole number of 1 or more (retryable: no)", "zero refused")
    expect(json.loads(held["details"][0][0])["meta"]["model"], "judge-b", "prior model survived")
    expect(backend.close(), 1, "only the valid configuration sent")


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
    expect(recording_entries(env["THINKTHEN_CACHE"]), 1, "seeded cache")


def test_a_platform_cache_stays_off_and_an_open_named_folder_is_refused() -> None:
    """Ticket 0318: SQL caches only in a folder the operator names, and
    refuses a named folder that others can write."""
    backend = Backend()
    env = environment(backend)
    del env["THINKTHEN_CACHE"]
    twice = """
db = connect()
say(first=run(db, "SELECT thinkthen_decide('Is this a complaint?', 'i want a refund')"),
    second=run(db, "SELECT thinkthen_decide('Is this a complaint?', 'i want a refund')"))
"""
    held = child(twice, env)
    expect((held["first"], held["second"]), ([[1]], [[1]]), "both answer")
    expect(backend.close(), 2, "no platform cache answers the second call")
    cache_home = pathlib.Path(env["XDG_CACHE_HOME"])
    expect(list(cache_home.iterdir()) if cache_home.exists() else [], [], "no platform cache folder")
    if sys.platform != "darwin":
        expect([path.name for path in pathlib.Path(env["XDG_STATE_HOME"]).iterdir()], ["thinkthen"],
               "the state folder holds only the count-only usage totals")
    backend = Backend()
    env = environment(backend)
    folder = pathlib.Path(env["SCRATCH"]) / "open"
    folder.mkdir()
    folder.chmod(0o777)
    held = child(twice.replace("db = connect()", "db = connect()\ndb.execute('SELECT thinkthen_configure(?)', ('{\"cache\":\"%s\"}',))" % folder), env)
    refusal = "thinkthen usage: the answer folder belongs to another user or others can write it, so they could choose its answers; make it this process user's own with mode 0700, or name another folder (retryable: no)"
    expect((held["first"], held["second"]), (refusal, refusal), "open folder refused")
    expect(backend.close(), 0, "a refused folder sends nothing")


def test_environment_token_cap_refuses_before_any_send() -> None:
    """Regression: from_env stops reading THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL."""
    backend = Backend()
    held = child("""
db = connect()
db.execute("SELECT thinkthen_configure(?)", ('{"cache":false}',))
say(refused=run(db, "SELECT thinkthen_decide('Is it late?', 'the train left at noon')"))
""", environment(backend, THINKTHEN_MAX_ESTIMATED_INPUT_TOKENS_TOTAL="10"))
    expect(held["refused"], "thinkthen usage: max_estimated_input_tokens_total=10 (encoded-body-bytes-908-v1) would be exceeded before this call's first request (retryable: no)", "token cap")
    expect(backend.close(), 0, "the refused call sends nothing")


def test_omitted_and_explicit_throttles_hold_packed_requests() -> None:
    """Stress selector owns the paired nine-request held witness."""
    for settings, cap in (({"batch": 2}, 8), ({"throttle": 6, "batch": 2}, 6)):
        backend = Backend()
        packed = json.dumps({str(at): f"row {at}" for at in range(18)})
        waiting = Child(f"""
db = connect()
db.execute("SELECT thinkthen_configure(?)", ({json.dumps(settings)!r},))
say(rows=run(db, "SELECT count(*) FROM thinkthen_decide_many(?, ?)", ("Is it red?", {packed!r})))
""", environment(backend, "arm/held"))
        expect(backend.wait(cap), cap, "held arrivals")
        time.sleep(0.3)
        expect(backend.count(), cap, "the next request stays queued")
        backend.release()
        expect(waiting.result()["rows"], [[18]], "all records")
        expect(backend.close(), 9, "nine packed requests")


if __name__ == "__main__":
    os.chdir(pathlib.Path(__file__).resolve().parent)
    stress = os.environ.get("THINKTHEN_TEST_PROFILE") == "stress"
    sys.exit(main({name: value for name, value in globals().items()
                   if name.startswith("test_") and (name == "test_omitted_and_explicit_throttles_hold_packed_requests") == stress}))
