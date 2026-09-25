#!/usr/bin/env python3
"""The four engine settings (ticket 0109 decision 2), each in a fresh child
with its own backend and cache folder."""

from __future__ import annotations

import os
import pathlib
import sys
import time

from helper import Backend, Child, child, environment, expect, main

AFTER_BUILD = "thinkthen usage: settings apply before the first call; this process already built its engine"


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


def test_throttle_is_checked_and_caps_a_warm() -> None:
    backend = Backend()
    warm = Child("""
db = connect()
db.execute("SELECT thinkthen_throttle(8)")
db.execute("CREATE TABLE t(body TEXT)")
db.executemany("INSERT INTO t VALUES (?)", [(f"row {at}",) for at in range(200)])
say(warm=run(db, "SELECT thinkthen_warm('Is it red?', body) FROM t"))
""", environment(backend, "arm/held"))
    expect(backend.wait(8), 8, "sends held at throttle 8")
    time.sleep(0.3)
    expect(backend.count(), 8, "sends after 300 ms")
    backend.release()
    expect(warm.result(), {"warm": [[200]]}, "the warm")
    expect(backend.close(), 200, "sends")


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
rows = "SELECT thinkthen_warm('Is it red?', t) FROM (SELECT 'a' t UNION ALL SELECT 'b' UNION ALL SELECT 'c')"
say(limit=run(db, "SELECT thinkthen_max_requests(2)"), clear=run(db, "SELECT thinkthen_max_requests(NULL)"), warm=run(db, rows))
""", environment(backend))
    expect(held, {"limit": [[2]], "clear": [[None]], "warm": [[3]]}, "the cleared limit")
    expect(backend.close(), 3, "sends")


SPENT = "thinkthen usage: this process has sent its total of {} requests (thinkthen_max_requests_total)"


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


def test_a_warm_flush_sends_only_the_remaining_total() -> None:
    """Decision 17: with 100 of 103 left, a 150-row flush sends exactly 100."""
    backend = Backend()
    env = environment(backend)
    held = child("""
db = connect()
db.execute("SELECT thinkthen_max_requests_total(103)")
spent = [run(db, f"SELECT thinkthen_decide('Is it red?', 'door {at}')") for at in range(3)]
db.execute("CREATE TABLE t(body TEXT)")
db.executemany("INSERT INTO t VALUES (?)", [(f"row {at}",) for at in range(150)])
say(spent=spent, warm=run(db, "SELECT thinkthen_warm('Is it red?', body) FROM t"), next=run(db, "SELECT thinkthen_decide('Is it red?', 'door 0')"))
""", env)
    expect(held, {"spent": [[[1]]] * 3, "warm": "thinkthen usage: this warm pass stopped at the remaining total of 100 requests (thinkthen_max_requests_total)",
                  "next": SPENT.format(103)}, "the calls: a spent total refuses even a cached answer")
    expect(backend.count(), 103, "sends")
    again = child("""
db = connect()
say(row=run(db, "SELECT thinkthen_decide('Is it red?', 'row 0')"))
""", env)
    expect(again, {"row": [[1]]}, "a judged row of the cut warm, with no total in a new process")
    expect(backend.close(), 103, "sends: the judged part stayed in the cache")


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


def test_cache_bytes_is_checked_at_the_setting_call() -> None:
    backend = Backend()
    held = child("""
db = connect()
say(zero=run(db, "SELECT thinkthen_cache_bytes(0)"),
    cap=run(db, "SELECT thinkthen_cache_bytes(1000000)"),
    answer=run(db, "SELECT thinkthen_decide('Is it red?', 'a red door')"))
""", environment(backend))
    expect(held, {
        "zero": "thinkthen usage: a cache cap is a whole number of bytes above zero",
        "cap": [[1000000]],
        "answer": [[1]],
    }, "the cache cap")
    expect(backend.close(), 1, "sends")


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
    sys.exit(main(globals()))
