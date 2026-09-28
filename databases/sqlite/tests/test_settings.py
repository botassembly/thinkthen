#!/usr/bin/env python3
"""The four engine settings (ticket 0109 decision 2), each in a fresh child
with its own backend and cache folder."""

from __future__ import annotations

import os
import pathlib
import json
import sys
import tempfile
import time

from helper import Backend, Child, child, environment, expect, main

AFTER_BUILD = "thinkthen usage: settings apply before the first call; this process already built its engine"


def test_saved_calibration_identity_in_sql_details() -> None:
    """The public SQL details route keeps the shared saved name and warning."""
    case = json.loads((pathlib.Path(__file__).resolve().parents[3] / "conformance" / "calibration.json").read_text())
    backend = Backend()
    question = json.dumps(case["question"], separators=(",", ":"))
    profile = json.dumps(case["runtime_profile"], separators=(",", ":"))
    def quote(value: str) -> str:
        return "'" + value.replace("'", "''") + "'"
    setup = f"SELECT thinkthen_profile({quote(profile)})"
    sql = f"SELECT thinkthen_details({quote(question)}, {quote(case['evidence'])})"
    held = child(f"db = connect()\nrun(db, {setup!r})\n"
                 f"say(details=run(db, {sql!r}))\n", environment(backend))
    details = json.loads(held["details"][0][0])
    expect(details["meta"]["question_sha256"], case["question_sha256"], "saved profile digest")
    expect(details["meta"]["profile_warning"], case["warning"], "saved profile mismatch")
    expect(details["meta"]["model"], case["model"], "actual backend model")
    expect(backend.close(), 1, "one SQL details request")


def test_shared_settings_corpus() -> None:
    """Run the shared nine setting cases through SQLite's public SQL functions."""
    corpus = json.loads((pathlib.Path(__file__).resolve().parents[3] / "conformance" / "settings.json").read_text())
    expect(corpus["schema"], "thinkthen.settings-cases/1", "settings corpus version")
    for shared in corpus["cases"]:
        backend = Backend()
        with tempfile.TemporaryDirectory() as folder:
            for step in shared["steps"]:
                setup = []
                for name, value in step["settings"].items():
                    if value == "$PROFILE":
                        value = json.dumps(shared["profile"], separators=(",", ":"))
                    elif value == "$FOLDER":
                        value = folder
                    if name == "cache" and value is False:
                        setup.append("SELECT thinkthen_cache(NULL)")
                    else:
                        literal = str(value) if isinstance(value, int) else "'" + str(value).replace("'", "''") + "'"
                        setup.append(f"SELECT thinkthen_{name}({literal})")
                setup_rows = ""
                if step.get("verb") == "relate":
                    entities = [(at, one["name"], one["kind"]) for at, one in enumerate(shared["entities"])]
                    setup_rows = ("db.execute('CREATE TABLE e(id INTEGER, name TEXT, kind TEXT)')\n"
                                  f"db.executemany('INSERT INTO e VALUES (?, ?, ?)', {entities!r})\n")
                    sql = f"SELECT count(*) FROM thinkthen_relate('e', 'id', 'name', 'kind', '{shared['relation']}')"
                elif step.get("verb") == "decide_many":
                    setup.append("SELECT thinkthen_batch(1)")
                    values = ", ".join("('" + value.replace("'", "''") + "')" for value in step["records"])
                    sql = f"SELECT thinkthen_warm('Is this a refund?', column1) FROM (VALUES {values})"
                elif "model" in step:
                    sql = f"SELECT json_extract(thinkthen_details('Is this a refund?', '{step['text']}'), '$.meta.model')"
                else:
                    sql = f"SELECT thinkthen_decide('Is this a refund?', '{step['text']}')"
                result = child(f"db = connect()\nsetup = {setup!r}\n"
                               f"for statement in setup:\n    assert not isinstance(run(db, statement), str)\n"
                               f"{setup_rows}"
                               f"say(value=run(db, {sql!r}))\n",
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
                saved = sum(path.is_file() and path.name != ".thinkthen-backend.json" for path in pathlib.Path(folder).rglob("*"))
                expect(saved, shared["entries"], f"{shared['id']} saved entries")
        backend.close()


def entries(folder: str) -> int:
    return sum(1 for path in pathlib.Path(folder).rglob("*") if path.is_file())


def test_settings_keep_the_environment_seed() -> None:
    """The throttle setting keeps `THINKTHEN_CACHE` from the environment."""
    backend = Backend()
    env = environment(backend)
    env.update(HOME=env["SCRATCH"] + "/home", XDG_CACHE_HOME=env["SCRATCH"] + "/home/.cache")
    held = child("""
db = connect()
say(throttle=run(db, "SELECT thinkthen_throttle(8)"),
    first=run(db, "SELECT thinkthen_decide('Is this a complaint?', 'i want a refund')"),
    second=run(db, "SELECT thinkthen_decide('Is this a complaint?', 'i want a refund')"))
""", env)
    expect(held, {"throttle": [[8]], "first": [[1]], "second": [[1]]}, "the answers")
    expect(backend.close(), 1, "sends")
    expect(entries(env["THINKTHEN_CACHE"]) > 0, True, "the entry is in THINKTHEN_CACHE")


def warm_at_throttle(rows: int) -> None:
    backend = Backend()
    warm = Child("""
db = connect()
db.execute("SELECT thinkthen_throttle(8)")
db.execute("SELECT thinkthen_batch(2)")
db.execute("CREATE TABLE t(body TEXT)")
db.executemany("INSERT INTO t VALUES (?)", [(f"row {at}",) for at in range(int(os.environ["ROWS"]))])
say(warm=run(db, "SELECT thinkthen_warm('Is it red?', body) FROM t"))
""", environment(backend, "arm/held") | {"ROWS": str(rows)})
    expect(backend.wait(8), 8, "sends held at throttle 8")
    time.sleep(0.3)
    expect(backend.count(), 8, "sends after 300 ms")
    backend.release()
    expect(warm.result(), {"warm": [[rows]]}, "the warm")
    expect(backend.close(), rows // 2, "packed sends")


def test_throttle_holds_eight_before_the_ninth() -> None:
    """A held eighth request prevents the ninth from starting."""
    warm_at_throttle(18)


def test_warm_deadline_spans_a_completed_chunk_and_later_row() -> None:
    """A pause after row 256 cannot grant the next flush a fresh deadline."""
    backend = Backend()
    held = child("""
db = connect()
db.execute("SELECT thinkthen_batch(1)")
db.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, body TEXT)")
db.executemany("INSERT INTO t VALUES (?, ?)", [(n, f"door {n}") for n in range(1, 258)])
def paused(body, number):
    if number == 257:
        time.sleep(4.1)
    return body
db.create_function("paused", 2, paused)
say(warm=run(db, "SELECT thinkthen_warm('Is it red?', paused(body,id), 4000) FROM t"))
""", environment(backend))
    expect(held["warm"].startswith("thinkthen deadline:"), True, "one warm deadline")
    expect(backend.close(), 256, "only the completed chunk sent")


def test_throttle_is_checked_and_caps_a_warm() -> None:
    """The original 200-row warm checks sustained throttle behavior."""
    warm_at_throttle(200)


def test_throttle_answers_and_refuses_before_any_send() -> None:
    backend = Backend()
    held = child("""
db = connect()
say(eight=run(db, "SELECT thinkthen_throttle(8)"),
    zero=run(db, "SELECT thinkthen_throttle(0)"),
    over=run(db, "SELECT thinkthen_throttle(33)"))
""", environment(backend))
    refused = "thinkthen usage: a throttle is a whole number from 1 through 32"
    expect(held, {"eight": [[8]], "zero": refused, "over": refused}, "the throttle calls")
    expect(backend.close(), 0, "sends")


def test_max_requests_limits_a_warm_and_null_clears_it() -> None:
    backend = Backend()
    held = child("""
db = connect()
db.execute("SELECT thinkthen_batch(1)")
rows = "SELECT thinkthen_warm('Is it red?', t) FROM (SELECT 'a' t UNION ALL SELECT 'b' UNION ALL SELECT 'c')"
say(limit=run(db, "SELECT thinkthen_max_requests(2)"), warm=run(db, rows))
""", environment(backend))
    expect(held["limit"], [[2]], "the setting")
    expect(held["warm"], "thinkthen usage: this engine answers at most 2 records in one call", "the warm")
    # A streaming call sends the records inside the limit, then refuses (EngineBuilder::max_requests).
    expect(backend.close(), 2, "sends")
    backend = Backend()
    held = child("""
db = connect()
db.execute("SELECT thinkthen_batch(1)")
rows = "SELECT thinkthen_warm('Is it red?', t) FROM (SELECT 'a' t UNION ALL SELECT 'b' UNION ALL SELECT 'c')"
say(limit=run(db, "SELECT thinkthen_max_requests(2)"), clear=run(db, "SELECT thinkthen_max_requests(NULL)"), warm=run(db, rows))
""", environment(backend))
    expect(held, {"limit": [[2]], "clear": [[None]], "warm": [[3]]}, "the cleared limit")
    expect(backend.close(), 3, "sends")


SPENT = "thinkthen usage: this process has sent its total of {} requests (thinkthen_max_requests_total)"


def test_new_settings_and_deadline_overloads_refuse_before_sending() -> None:
    backend = Backend()
    profile = '{"schema":"thinkthen.backend-profile/1","name":"tiny","max_evidence_bytes":3}'
    held = child(f"""
db = connect()
db.execute("CREATE TABLE e(id, name, kind)")
db.executemany("INSERT INTO e VALUES (?, ?, ?)", [(1, "Ada", "person"), (2, "Bo", "person")])
say(model=run(db, "SELECT thinkthen_model('jev-1.13.0')"),
    timeout=run(db, "SELECT thinkthen_timeout(2)"),
    retries=run(db, "SELECT thinkthen_max_retries(0)"),
    profile=run(db, "SELECT thinkthen_profile(?)", ({profile!r},)),
    clear_profile=run(db, "SELECT thinkthen_profile(NULL)"),
    record=run(db, "SELECT thinkthen_record(NULL)"),
    replay=run(db, "SELECT thinkthen_replay(NULL)"),
    warm=run(db, "SELECT thinkthen_warm('Is it red?', 'red', 0)"),
    recognize=run(db, "SELECT * FROM thinkthen_recognize('Ada', 'person', 0)"),
    relate=run(db, "SELECT * FROM thinkthen_relate('e', 'id', 'name', 'kind', 'knows=person:person', NULL, NULL, NULL, 0)"))
""", environment(backend))
    expect(held["model"], [["jev-1.13.0"]], "model setter")
    expect(held["timeout"], [[2]], "timeout setter")
    expect(held["retries"], [[0]], "retry setter")
    expect(held["profile"], [[profile]], "JSON profile setter")
    expect(held["clear_profile"], [[None]], "profile clear")
    expect(held["record"], [[None]], "record NULL")
    expect(held["replay"], [[None]], "replay NULL")
    for name in ("warm", "recognize", "relate"):
        expect(held[name].startswith("thinkthen deadline:"), True, name)
    expect(backend.close(), 0, "deadline zero sends nothing")


def test_retry_spends_the_process_total_before_a_second_send() -> None:
    backend = Backend()
    held = child("""
db = connect()
say(total=run(db, "SELECT thinkthen_max_requests_total(1)"),
    retries=run(db, "SELECT thinkthen_max_retries(1)"),
    refused=run(db, "SELECT thinkthen_decide('Is it red?', 'a red door')"))
""", environment(backend, "arm/503"))
    expect(held, {"total": [[1]], "retries": [[1]], "refused": SPENT.format(1)}, "one retry refused")
    expect(backend.close(), 1, "one attempt, no retry")


def test_record_and_replay_share_a_folder_without_a_replay_send() -> None:
    backend = Backend()
    env = environment(backend)
    folder = env["SCRATCH"] + "/saved"
    recorded = child(f"""
db = connect()
db.execute("SELECT thinkthen_cache(NULL)")
say(record=run(db, "SELECT thinkthen_record(?)", ({folder!r},)),
    first=run(db, "SELECT thinkthen_decide('Is it red?', 'a red door')"),
    second=run(db, "SELECT thinkthen_decide('Is it red?', 'a red door')"))
""", env)
    expect(recorded, {"record": [[folder]], "first": [[1]], "second": [[1]]}, "record every call")
    expect(backend.count(), 2, "record sends twice")
    replayed = child(f"""
db = connect()
db.execute("SELECT thinkthen_cache(NULL)")
say(replay=run(db, "SELECT thinkthen_replay(?)", ({folder!r},)),
    saved=run(db, "SELECT thinkthen_decide('Is it red?', 'a red door')"),
    missing=run(db, "SELECT thinkthen_decide('Is it red?', 'another door')"))
""", env)
    expect(replayed["replay"], [[folder]], "replay folder")
    expect(replayed["saved"], [[1]], "saved answer")
    expect(replayed["missing"].startswith("thinkthen local:"), True, "strict miss")
    expect(backend.close(), 2, "replay sends nothing")


def test_inline_profile_refuses_an_over_limit_request_before_sending() -> None:
    backend = Backend()
    profile = '{"schema":"thinkthen.backend-profile/1","name":"small","max_evidence_bytes":4}'
    held = child(f"""
db = connect()
say(profile=run(db, "SELECT thinkthen_profile(?)", ({profile!r},)),
    refused=run(db, "SELECT thinkthen_decide('Is it red?', 'a red door')"))
""", environment(backend))
    expect(held["profile"], [[profile]], "valid JSON profile")
    expect(held["refused"].startswith("thinkthen usage: profile small allows at most 4 evidence bytes"), True, "profile refusal")
    expect(backend.close(), 0, "profile refuses before send")


def test_timeout_setting_limits_a_slow_attempt() -> None:
    backend = Backend()
    held = child("""
db = connect()
say(timeout=run(db, "SELECT thinkthen_timeout(1)"),
    retries=run(db, "SELECT thinkthen_max_retries(0)"),
    result=run(db, "SELECT thinkthen_decide('Is it red?', 'slow')"))
""", environment(backend, "arm/delay/2000"))
    expect(held["timeout"], [[1]], "one-second timeout")
    expect(held["retries"], [[0]], "no retry")
    expect(held["result"].startswith("thinkthen backend: the backend timed out"), True, "timeout refusal")
    expect(backend.close(), 1, "one slow attempt")


def test_the_request_total_holds_across_one_row_calls() -> None:
    """Decision 17: a WHERE over 10 rows is 10 one-record calls, and a total of 3 stops the fourth."""
    backend = Backend()
    held = child("""
db = connect()
db.execute("CREATE TABLE t(body TEXT)")
db.executemany("INSERT INTO t VALUES (?)", [(f"row {at}",) for at in range(10)])
say(total=run(db, "SELECT thinkthen_max_requests_total(3)"), rows=run(db, "SELECT body FROM t WHERE thinkthen_decide('Is it red?', body)"),
    next=run(db, "SELECT thinkthen_decide('Is it blue?', 'a blue door')"),
    zero=run(db, "SELECT thinkthen_max_requests_total(0)"))
""", environment(backend))
    expect(held, {"total": [[3]], "rows": SPENT.format(3), "next": SPENT.format(3),
                  "zero": "thinkthen usage: a request total is a whole number of 1 or more"}, "the calls")
    expect(backend.close(), 3, "sends")


def test_a_packed_warm_spends_one_actual_attempt_without_a_partial_count() -> None:
    """Four records form two requests; total one admits one and returns no count."""
    import hashlib

    backend = Backend()
    env = environment(backend)
    unlimited = env["SCRATCH"] + "/unlimited"
    limited = env["SCRATCH"] + "/limited"
    expected = [
        r'{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"red one\". Is it red?"},"q2":{"type":"noul","instructions":"The text is \"red two\". Is it red?"}}}',
        r'{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \"red three\". Is it red?"},"q2":{"type":"noul","instructions":"The text is \"red four\". Is it red?"}}}',
    ]
    code = """
db = connect()
db.execute("SELECT thinkthen_batch(2)")
db.execute("SELECT thinkthen_cache(NULL)")
db.execute("SELECT thinkthen_max_retries(0)")
db.execute("SELECT thinkthen_record(?)", (os.environ["RECORD"],))
if os.environ["TOTAL"] == "1":
    db.execute("SELECT thinkthen_max_requests_total(1)")
db.execute("CREATE TABLE t(body TEXT)")
db.executemany("INSERT INTO t VALUES (?)", [("red one",), ("red two",), ("red three",), ("red four",)])
say(warm=run(db, "SELECT thinkthen_warm('Is it red?', body) FROM t"),
    usage=json.loads(run(db, "SELECT thinkthen_usage()")[0][0]))
"""
    def records(folder: str) -> dict[str, dict]:
        return {path.stem: json.loads(path.read_text()) for path in pathlib.Path(folder).glob("*.json")
                if not path.name.startswith(".")}
    def digest(body: str) -> str:
        url = backend.base("generic") + "/systemone"
        return hashlib.sha256(b"systemone\n" + url.encode() + b"\n" + body.encode()).hexdigest()
    wanted = {digest(body): json.loads(body) for body in expected}
    first = child(code, env | {"RECORD": unlimited, "TOTAL": "0"})
    expect(first["warm"], [[4]], "all four records answered")
    expect(first["usage"]["requests_sent"], 2, "two packed attempts")
    got = records(unlimited)
    expect({name: one["request"] for name, one in got.items()}, wanted, "two exact bodies and digests")
    second = child(code, env | {"RECORD": limited, "TOTAL": "1"})
    expect(second["warm"], SPENT.format(1), "fatal spent-total Usage with no aggregate count")
    expect(second["usage"]["requests_sent"], 1, "one admitted transport attempt")
    got = records(limited)
    expect(len(got), 1, "one recorded request")
    expect(next(iter(got.items()))[0] in wanted, True, "admitted body is one of the two pinned requests")
    expect(backend.close(), 3, "two unlimited and one total-limited attempts")


def test_cache_names_a_folder_and_null_turns_it_off() -> None:
    backend = Backend()
    folder = environment(None)["SCRATCH"] + "/named"
    ask = f"""
db = connect()
say(folder=run(db, "SELECT thinkthen_cache(?)", ({folder!r},)),
    answer=run(db, "SELECT thinkthen_decide('Is it red?', 'a red door')"))
"""
    for _ in range(2):
        expect(child(ask, environment(backend)), {"folder": [[folder]], "answer": [[1]]}, "the answer")
    expect(backend.count(), 1, "sends: the second child read the named folder")
    expect(entries(folder) > 0, True, "the named folder holds the entry")
    held = child("""
db = connect()
say(off=run(db, "SELECT thinkthen_cache(NULL)"),
    warm=run(db, "SELECT thinkthen_warm('Is it blue?', 'a blue door')"),
    again=run(db, "SELECT thinkthen_decide('Is it blue?', 'a blue door')"))
""", environment(backend))
    expect(held, {"off": [[None]], "warm": [[1]], "again": [[1]]}, "the answers with the cache off")
    expect(backend.close(), 3, "sends: the query after the warm sent again")


def test_a_setting_after_the_first_call_is_refused() -> None:
    """`thinkthen_usage` builds no engine, so a setting after it still applies."""
    backend = Backend()
    held = child("""
db = connect()
say(usage=run(db, "SELECT thinkthen_usage()"),
    before=run(db, "SELECT thinkthen_throttle(8)"),
    answer=run(db, "SELECT thinkthen_decide('Is it red?', 'a red door')"),
    after=run(db, "SELECT thinkthen_throttle(8)"))
""", environment(backend))
    zero = '{"requests_sent":0,"cache_answers":0,"input_tokens":0,"output_tokens":0}'
    expect(held, {"usage": [[zero]], "before": [[8]], "answer": [[1]], "after": AFTER_BUILD}, "the calls")
    expect(backend.close(), 1, "sends")


if __name__ == "__main__":
    os.chdir(pathlib.Path(__file__).resolve().parent)
    only_stress = os.environ.get("THINKTHEN_TEST_PROFILE") == "stress"
    sys.exit(main({name: value for name, value in globals().items()
                   if name.startswith("test_") and
                   (name == "test_throttle_is_checked_and_caps_a_warm") == only_stress}))
