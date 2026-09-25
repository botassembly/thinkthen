#!/usr/bin/env python3
"""What each function takes, refuses, and returns (ticket 0109 decisions 3, 4, 5, 8, 10, 11, and 16)."""

from __future__ import annotations

import json
import os
import sys

from helper import Backend, child, environment, expect, main

BANDED = json.dumps({"decide": "Is it red?", "threshold": "0.2:0.8"})
SCALARS = ("thinkthen_decide", "thinkthen_choose", "thinkthen_score", "thinkthen_tag", "thinkthen_details", "thinkthen_annotate")


def test_a_null_text_is_null_and_a_bad_value_is_refused_before_any_send() -> None:
    """Branch case 81 and G11, for every scalar that sends."""
    backend = Backend()
    held = child(f"""
db = connect()
say(**{{name: [run(db, f"SELECT {{name}}(?, ?)", pair) for pair in (
    ("Is it red?", None), (None, "a red door"), ("Is it red?", b"a red door"), ("Is it red?", "a red\\x00door"),
    ("Is it red?", 7), ("Is it red?", 1.5))] + [run(db, f"SELECT {{name}}('Is it red?', CAST(x'ff' AS TEXT))")]
    for name in {SCALARS!r}}})
""", environment(backend))
    for name in SCALARS:
        expect(held[name], [
            [[None]], [[None]],
            "thinkthen usage: the text is a BLOB; pass text",
            "thinkthen usage: the text holds a NUL byte",
            "thinkthen usage: the text is a number; pass text",
            "thinkthen usage: the text is a number; pass text",
            "thinkthen usage: the text is not UTF-8",
        ], name)
    expect(backend.close(), 0, "sends")


def test_a_banded_question_is_refused_where_it_has_no_answer() -> None:
    """Decision 4: score, choose, and tag name the two functions that take it. Warm
    takes a band (ticket 0129) and still refuses a choose question."""
    backend = Backend()
    choose = json.dumps({"choose": "Which colour?", "options": ["red", "blue"]})
    held = child(f"""
db = connect()
say(**{{name: run(db, f"SELECT {{name}}(?, 'a red door')", ({BANDED!r},))
    for name in ("thinkthen_score", "thinkthen_choose", "thinkthen_tag")}},
    thinkthen_warm=run(db, "SELECT thinkthen_warm(?, 'a red door')", ({choose!r},)))
""", environment(backend))
    wanted = "thinkthen usage: {} does not take a banded question; use thinkthen_decide or thinkthen_details"
    expect(held, {name: wanted.format(name) for name in held} | {
        "thinkthen_warm": "thinkthen usage: thinkthen_warm takes a decide question; ask others with thinkthen_decide",
    }, "the refusals")
    expect(backend.close(), 0, "sends")


def test_recognize_counts_offsets_as_substr_does_and_refuses_relations() -> None:
    """Case 41 and decision 10."""
    backend = Backend()
    case = "41-offsets-past-an-accent-and-an-emoji"
    kinds = json.dumps({"version": 1, "recognize": {"kinds": {"person": "A person's name."}}, "threshold": 0.5})
    relations = json.dumps({"kinds": {"person": "A person."}, "relations": [{"name": "knows", "source": "person", "target": "person"}]})
    held = child(f"""
db = connect()
text = "Le caf\\u00e9 \\U0001F600 Maria Chen arrived."
say(names=run(db, "SELECT name, start, \\"end\\", substr(?, start + 1, \\"end\\" - start) FROM thinkthen_recognize(?, ?)", (text, text, {kinds!r})),
    relations=run(db, "SELECT * FROM thinkthen_recognize('Ada met Bo.', ?)", ({relations!r},)))
""", environment(backend, f"case/{case}"))
    expect(held, {
        "names": [["Maria Chen", 10, 20, "Maria Chen"]],
        "relations": "thinkthen usage: thinkthen_recognize takes no relations; relate rows with thinkthen_relate",
    }, "the rows")
    expect(backend.close(), 1, "sends: the relations spec sent nothing")


RELATE = """
db = connect()
db.execute("CREATE TABLE e(id INTEGER, name TEXT, kind TEXT)")
db.executemany("INSERT INTO e VALUES (?, ?, ?)", {rows!r})
say(edges=run(db, "SELECT relation, source, target FROM thinkthen_relate('e', 'id', 'name', 'kind', 'works_for=person:organization') ORDER BY source, target"))
"""


def test_relate_returns_each_edge_once_for_every_row_holding_its_ends() -> None:
    """Decision 11: two rows share one name and kind, and each edge joins back to both ids."""
    backend = Backend()
    held = child(RELATE.format(rows=[(1, "Ada", "person"), (2, "Acme", "organization"), (3, "Ada", "person")]), environment(backend))
    expect(held["edges"], [["works_for", 1, 2], ["works_for", 3, 2]], "the edges")
    expect(backend.close(), 1, "sends: two distinct entities")


def test_relate_refuses_a_blank_name_and_the_256th_pair_before_any_send() -> None:
    backend = Backend()
    blank = child(RELATE.format(rows=[(1, "Ada", "person"), (7, None, "organization")]), environment(backend))
    many = child(RELATE.format(rows=[(at, f"name {at}", "person") for at in range(300)]), environment(backend))
    expect((blank["edges"], many["edges"]), (
        "thinkthen usage: the row with id 7 has no name",
        "thinkthen usage: thinkthen_relate takes at most 255 distinct name and kind pairs",
    ), "the refusals")
    expect(backend.close(), 0, "sends")


def test_warm_keys_its_groups_by_question_and_text() -> None:
    """R2-22: two questions over one text are two pairs, and the decide after reads the cache."""
    backend = Backend()
    held = child("""
db = connect()
db.execute("CREATE TABLE t(q TEXT, body TEXT)")
db.executemany("INSERT INTO t VALUES (?, ?)", [("Is it red?", "a red door"), ("Is it old?", "a red door"), ("Is it red?", "a red door")])
warm = run(db, "SELECT thinkthen_warm(q, body) FROM t")
say(warm=warm, decide=run(db, "SELECT thinkthen_decide('Is it old?', 'a red door')"), usage=json.loads(run(db, "SELECT thinkthen_usage()")[0][0]))
""", environment(backend))
    expect((held["warm"], held["decide"]), ([[2]], [[1]]), "the answers")
    expect((held["usage"]["requests_sent"], held["usage"]["cache_answers"]), (2, 1), "the usage totals")
    expect(backend.close(), 2, "sends")


def test_usage_refuses_the_reset_spelling() -> None:
    held = child("""
db = connect()
say(reset=run(db, "SELECT thinkthen_usage('reset')"), other=run(db, "SELECT thinkthen_usage(1)"))
""", environment(None))
    expect(held, {
        "reset": "thinkthen usage: the reset spelling is removed; the counters are cumulative, so take two snapshots and subtract them",
        "other": "thinkthen usage: thinkthen_usage takes no arguments; the counters are cumulative, so subtract two snapshots",
    }, "the refusals")


SECRET_KEY, USER, PASSWORD = "sk-sentinel-3141", "sentinel-user", "sentinel-password"
EVERY_CALL = """
db = connect()
db.execute("CREATE TABLE e(id INTEGER, name TEXT, kind TEXT)")
db.executemany("INSERT INTO e VALUES (?, ?, ?)", [(1, "Ada", "person"), (2, "Acme", "organization")])
calls = ["SELECT thinkthen_throttle(4)", "SELECT thinkthen_max_requests(NULL)", "SELECT thinkthen_max_requests_total(1000)", "SELECT thinkthen_cache_bytes(1000000)",
         "SELECT thinkthen_cache('" + os.environ["SCRATCH"] + "/cache')"]
calls += [f"SELECT {name}('Is it red?', 'a red door'{deadline})" for name in ("thinkthen_decide", "thinkthen_details", "thinkthen_warm") for deadline in ("", ", 0")]
calls += ["SELECT thinkthen_choose('{\\"choose\\":\\"Which?\\",\\"options\\":[\\"a\\",\\"b\\"]}', 'x')",
          "SELECT thinkthen_score('{\\"score\\":\\"How?\\",\\"levels\\":[\\"l\\",\\"h\\"]}', 'x')",
          "SELECT thinkthen_tag('{\\"tag\\":\\"Which?\\",\\"labels\\":[\\"a\\",\\"b\\"]}', 'x')",
          "SELECT thinkthen_annotate('{\\"version\\":1,\\"questions\\":{\\"k\\":{\\"decide\\":\\"Is it red?\\"}}}', 'x')",
          "SELECT * FROM thinkthen_recognize('Ada joined Acme.', 'person')",
          "SELECT * FROM thinkthen_relate('e', 'id', 'name', 'kind', 'works_for=person:organization')",
          "SELECT thinkthen_decide('@/nonexistent/q.json', 'x')", "SELECT thinkthen_usage()", "SELECT thinkthen_throttle(8)"]
say(said=[run(db, sql) for sql in calls])
"""


def test_no_message_carries_the_key_or_the_address_credentials() -> None:
    """Decision 16: every function's answer or error, on a busy backend and on a refused address."""
    backend = Backend()
    busy = child(EVERY_CALL, environment(backend, "arm/503", THINKTHEN_API_KEY=SECRET_KEY))
    refused = child(EVERY_CALL, environment(backend, THINKTHEN_API_KEY=SECRET_KEY, THINKTHEN_BASE_URL=f"http://{USER}:{PASSWORD}@127.0.0.1:{backend.port}/generic/v1"))
    said = json.dumps([busy, refused])
    expect([secret for secret in (SECRET_KEY, USER, PASSWORD) if secret in said], [], f"the sentinels in {said}")
    expect(sum("thinkthen backend (retryable)" in str(one) for one in busy["said"]), 9, f"the busy errors in {busy}")
    expect(refused["said"][0], "thinkthen usage: THINKTHEN_BASE_URL: a base address carries no user information", "the refused address")


def test_a_child_never_sees_the_callers_key() -> None:
    """Decision 16: the helper removes the caller's key and sets the loopback key."""
    os.environ["THINKTHEN_API_KEY"] = SECRET_KEY
    try:
        held = child("say(key=os.environ.get('THINKTHEN_API_KEY'))", environment(Backend()))
        bare = child("say(key=os.environ.get('THINKTHEN_API_KEY'))", environment(None))
    finally:
        del os.environ["THINKTHEN_API_KEY"]
    expect((held["key"], bare["key"]), ("sk-sqlite-loopback", None), "the child's key")


if __name__ == "__main__":
    sys.exit(main(globals()))
