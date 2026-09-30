#!/usr/bin/env python3
"""What each function takes, refuses, and returns (ticket 0109 decisions 3, 4, 5, 8, 10, 11, and 16)."""

from __future__ import annotations

import json
import hashlib
import os
import sys

from helper import Backend, child, environment, expect, main
from conditional_backend import ConditionalBackend

BANDED = json.dumps({"decide": "Is it red?", "threshold": "0.2:0.8"})
SCALARS = ("thinkthen_decide", "thinkthen_choose", "thinkthen_score", "thinkthen_tag", "thinkthen_details", "thinkthen_annotate")


def test_find_preserves_duplicate_positions_and_strict_ties() -> None:
    """A canned reply selects the second equal text; a real tie beats a peer but loses to none."""
    backend = Backend()
    cases = (
        (["same", "same", "other"], False, {"u001": 0.1, "u002": 0.8, "u003": 0.1},
         {"index": 1, "value": "same", "probability": 0.8,
          "candidates": [{"index": 0, "probability": 0.1}, {"index": 1, "probability": 0.8},
                         {"index": 2, "probability": 0.1}]}),
        (["first", "second"], False, {"u001": 0.5, "u002": 0.5},
         {"index": 0, "value": "first", "probability": 0.5,
          "candidates": [{"index": 0, "probability": 0.5}, {"index": 1, "probability": 0.5}]}),
        (["first", "second"], True, {"u001": 0.4, "u002": 0.2, "none": 0.4},
         {"index": None, "value": None, "probability": 0.4,
          "candidates": [{"index": 0, "probability": 0.4}, {"index": 1, "probability": 0.2},
                         {"index": None, "probability": 0.4}]}),
    )
    for units, offered, probabilities, selected in cases:
        answer = json.dumps({"model": "jev-latest", "answers": {"q1": {
            "type": "choice", "probabilities": probabilities}}}).encode()
        with ConditionalBackend(backend.base(), reply=answer) as proxy:
            held = child(f"""
db = connect()
say(result=run(db, "SELECT thinkthen_find(?, ?, ?)",
               ("Which unit?", {json.dumps(units)!r}, {json.dumps({'none': offered})!r})))
""", environment(backend, THINKTHEN_BASE_URL=proxy.base))
            expect(proxy.count(), 1, "one find request")
        result = json.loads(held["result"][0][0])
        expect(result, selected, "original selected unit and input-order probabilities")
    expect(backend.close(), 0, "fixed replies did not reach the generic arm")


def test_find_null_empty_and_invalid_inputs_never_send() -> None:
    backend = Backend()
    held = child("""
db = connect()
good = json.dumps(['one', 'two'])
bad = [
    ('question NULL', None, good, '{}'),
    ('units NULL', 'Which?', None, '{}'),
    ('empty', 'Which?', '[]', '{}'),
    ('one', 'Which?', '["one"]', '{}'),
    ('member NULL', 'Which?', '["one",null]', '{}'),
    ('member number', 'Which?', '["one",7]', '{}'),
    ('member blank', 'Which?', '["one","  "]', '{}'),
    ('question number', 7, good, '{}'),
    ('units number', 'Which?', 7, '{}'),
    ('units blob', 'Which?', b'["one","two"]', '{}'),
    ('malformed', 'Which?', '[', '{}'),
    ('not array', 'Which?', '{}', '{}'),
    ('bad none', 'Which?', good, '{"none":2}'),
    ('bad question', '   ', good, '{}'),
    ('bad deadline', 'Which?', good, '{"deadline_ms":0.5}'),
    ('too many', 'Which?', json.dumps(['x'] * 256), '{}'),
    ('none too many', 'Which?', json.dumps(['x'] * 255), '{"none":true}'),
    ('too much text', 'Which?', json.dumps(['x' * (16 * 1024 * 1024), 'y']), '{}'),
]
say(results={name: run(db, 'SELECT thinkthen_find(?, ?, ?)', (q, units, settings)) for
             name, q, units, settings in bad})
""", environment(backend))
    for name in ("question NULL", "units NULL", "empty"):
        expect(held["results"][name], [[None]], name)
    for name in ("one", "member NULL", "member number", "member blank", "question number", "units number",
                 "units blob", "malformed", "not array", "bad none", "bad question", "bad deadline",
                 "too many", "none too many", "too much text"):
        expect(held["results"][name].startswith("thinkthen usage: "), True, name)
        expect(held["results"][name].endswith(" (retryable: no)"), True, name)
    expect(backend.close(), 0, "all refused inputs send nothing")

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
            "thinkthen usage: the text is a BLOB; pass text (retryable: no)",
            "thinkthen usage: the text holds a NUL byte (retryable: no)",
            "thinkthen usage: the text is a number; pass text (retryable: no)",
            "thinkthen usage: the text is a number; pass text (retryable: no)",
            "thinkthen usage: the text is not UTF-8 (retryable: no)",
        ], name)
    expect(backend.close(), 0, "sends")


def test_a_banded_question_is_refused_where_it_has_no_answer() -> None:
    """Score, choose and tag refuse a band; removed warm refuses by name."""
    backend = Backend()
    choose = json.dumps({"choose": "Which colour?", "options": ["red", "blue"]})
    held = child(f"""
db = connect()
say(**{{name: run(db, f"SELECT {{name}}(?, 'a red door')", ({BANDED!r},))
    for name in ("thinkthen_score", "thinkthen_choose", "thinkthen_tag")}},
    thinkthen_warm=run(db, "SELECT thinkthen_warm(?, 'a red door')", ({choose!r},)))
""", environment(backend))
    wanted = "thinkthen usage: {} does not take a banded question; use thinkthen_decide or thinkthen_details (retryable: no)"
    expect(held, {name: wanted.format(name) for name in held if name != "thinkthen_warm"} | {
        "thinkthen_warm": "thinkthen usage: thinkthen_warm was removed; pack records with thinkthen_decide_many",
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
say(names=run(db, "SELECT text, start, \\"end\\", length, substr(?, start + 1, length) FROM thinkthen_recognize(?, ?)", (text, text, {kinds!r})),
    relations=run(db, "SELECT * FROM thinkthen_recognize('Ada met Bo.', ?)", ({relations!r},)))
""", environment(backend, f"case/{case}"))
    expect(held, {
        "names": [["Maria Chen", 10, 20, 10, "Maria Chen"]],
        "relations": "thinkthen usage: thinkthen_recognize takes no relations; relate rows with thinkthen_relate",
    }, "the rows")
    expect(backend.close(), 2, "sends: the two recognize steps, and the relations spec sent nothing")


def test_recognize_document_accepts_spec_forms_and_refuses_bad_sql_values() -> None:
    """The document form keeps empty relations and handles arguments before sending."""
    backend = Backend()
    bare = {"kinds": {"person": "A person's name."}, "relations": [
        {"name": "knows", "source": "person", "target": "person"}]}
    full = {"version": 1, "recognize": bare}
    plain = {"kinds": {"person": "A person's name."}}
    bad = {"kinds": {"person": "A person."}, "relations": [
        {"name": "bad", "source": "absent", "target": "person"}]}
    held = child(f"""
db = connect()
path = os.environ['SCRATCH'] + '/recognize.json'
open(path, 'w').write({json.dumps(full)!r})
bad_path = os.environ['SCRATCH'] + '/bad-recognize.json'
open(bad_path, 'w').write({json.dumps(bad)!r})
say(forms=[run(db, 'SELECT thinkthen_relations(?, ?)', ('', spec))
           for spec in ({json.dumps(full)!r}, {json.dumps(bare)!r}, '@' + path, {json.dumps(plain)!r})],
    refused=[run(db, 'SELECT thinkthen_relations(?, ?)', pair) for pair in (
        (None, {json.dumps(full)!r}), ('', None), (b'x', {json.dumps(full)!r}),
        (7, {json.dumps(full)!r}), ('x\\x00y', {json.dumps(full)!r}),
        ('', b'{{}}'), ('', 7), ('', '{{}}\\x00'), ('', '{{'),
        ('', {json.dumps(bad)!r}),
        ('', '@' + bad_path),
    )] + [run(db, "SELECT thinkthen_relations(CAST(x'ff' AS TEXT), '{{}}')"),
         run(db, "SELECT thinkthen_relations('', CAST(x'ff' AS TEXT))")])
""", environment(backend))
    expect([json.loads(one[0][0]) for one in held["forms"]], [
        {"entities": [], "relations": []}, {"entities": [], "relations": []},
        {"entities": [], "relations": []}, {"entities": []},
    ], "forms and empty edge shape")
    expect(held["refused"][:8], [
        [[None]], [[None]],
        "thinkthen usage: the text is a BLOB; pass text (retryable: no)",
        "thinkthen usage: the text is a number; pass text (retryable: no)",
        "thinkthen usage: the text holds a NUL byte (retryable: no)",
        "thinkthen usage: the recognize spec is a BLOB; pass text (retryable: no)",
        "thinkthen usage: the recognize spec is a number; pass text (retryable: no)",
        "thinkthen usage: the recognize spec holds a NUL byte (retryable: no)",
    ], "SQL argument forms")
    expect(held["refused"][-5:-2], [
        "thinkthen usage: the recognize argument is not JSON: EOF while parsing an object at line 1 column 1 (retryable: no)",
        "thinkthen usage: a relation source and target name a kind or explicit `*` (retryable: no)",
        "thinkthen local: a relation source and target name a kind or explicit `*` (retryable: no)",
    ], "malformed inline and file specs")
    expect(held["refused"][-2:], [
        "thinkthen usage: the text is not UTF-8 (retryable: no)",
        "thinkthen usage: the recognize spec is not UTF-8 (retryable: no)",
    ], "invalid UTF-8")
    expect(backend.close(), 0, "zero sends")


def test_recognize_document_applies_a_nondefault_relation_threshold() -> None:
    """A stronger edge cut removes the edge without losing recognized entities."""
    evidence = "Maria Chen joined Northwind Freight in Chicago last spring."
    question = {"version": 1, "recognize": {
        "kinds": {"organization": "An organization."},
        "relations": [{"name": "knows", "source": "organization", "target": "organization"}],
    }, "threshold": 0.5, "relation_threshold": 0.5}
    backend = Backend()
    held = child(f"""
db = connect()
say(result=[run(db, 'SELECT thinkthen_relations(?, ?)', ({evidence!r}, spec))
            for spec in ({json.dumps(question)!r}, {json.dumps(dict(question, relation_threshold=1.0))!r})])
""", environment(backend))
    low, high = (json.loads(one[0][0]) for one in held["result"])
    expect(len(low["relations"]), 2, "generic answer has two directed relations to cut")
    expect(high, {"entities": low["entities"], "relations": []}, "nondefault edge cut")
    expect(backend.close(), 3, "the second local cut reuses the three cached phases")


RELATE = """
db = connect()
db.execute("CREATE TABLE e(id INTEGER, name TEXT, kind TEXT)")
db.executemany("INSERT INTO e VALUES (?, ?, ?)", {rows!r})
say(edges=run(db, "SELECT relation, source, target FROM thinkthen_relate('SELECT id, name, kind FROM e', 'works_for=person:organization') ORDER BY source, target"))
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
        "thinkthen usage: the row with id 7 has no name (retryable: no)",
        "thinkthen usage: thinkthen_relate takes at most 255 distinct name and kind pairs (retryable: no)",
    ), "the refusals")
    expect(backend.close(), 0, "sends")


def test_settings_context_keeps_scalar_shapes_and_validation_order() -> None:
    """The third settings slot reaches values and facts; bad values send nothing."""
    backend = Backend()
    questions = {
        "choose": {"choose": "Which team owns this?", "options": ["billing", "shipping", "other"]},
        "tag": {"tag": "Which labels apply?", "labels": ["billing", "urgent", "security"]},
        "score": {"score": "How severe is this?", "levels": ["low", "medium", "high"]},
    }
    held = child(f"""
db = connect()
questions = {questions!r}
settings = '{{"context":"shared context"}}'
shapes = {{name: run(db, f"SELECT thinkthen_{{name}}(?, 'red door', ?)", (json.dumps(q), settings))
          for name, q in questions.items()}}
plain = run(db, "SELECT thinkthen_details('Is it red?', 'red door')")
shapes['decide'] = run(db, "SELECT thinkthen_decide('Is it red?', 'red door', ?)", (settings,))
shapes['details'] = run(db, "SELECT thinkthen_details('Is it red?', 'red door', ?)", (settings,))
shapes['try'] = run(db, "SELECT thinkthen_try_details('Is it red?', 'red door', ?)", (settings,))
say(shapes=shapes, plain=plain,
    null=run(db, "SELECT thinkthen_try_details(NULL, 'bad', ?)", ('{{"context":"  "}}',)),
    slot=run(db, "SELECT thinkthen_decide('Is it red?', 'red door', -1)"),
    bad=run(db, "SELECT thinkthen_try_details('Is it red?', 'red door', ?)", ('{{"context":"  "}}',)),
    bad_ordinary=run(db, "SELECT thinkthen_decide('Is it red?', 'red door', ?)", ('{{"context":"  "}}',)))
""", environment(backend))
    shapes = held["shapes"]
    expect({name: shapes[name] for name in ("choose", "tag", "score", "decide")},
           {"choose": [["billing"]], "tag": [['["billing","urgent","security"]']],
            "score": [[0.15]], "decide": [[1]]}, "scalar values")
    plain = json.loads(held["plain"][0][0])
    details = json.loads(shapes["details"][0][0])
    expect(json.loads(shapes["try"][0][0]), {"status": "answered", "details": details}, "try details")
    expect("input" in details, False, "scalar details omit record input")
    expect(details["meta"]["question_sha256"], plain["meta"]["question_sha256"], "question identity")
    expect(details["meta"]["context_sha256"], hashlib.sha256(b"shared context").hexdigest(), "context identity")
    expect(details["meta"]["requests"] != plain["meta"]["requests"], True, "context changes request")
    expect(held["null"], [[None]], "NULL short-circuit")
    expect(held["slot"], "thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'", "old slot refusal")
    safe = {"status": "failed", "error": {"kind": "usage", "message":
            "check the row's question and arguments, or raise the process request total when it is spent", "retryable": False}}
    expect(json.loads(held["bad"][0][0]), safe, "safe failed value")
    expect(held["bad_ordinary"], "thinkthen usage: `context` is text that is not blank (retryable: no)", "bad context")
    expect(backend.close(), 5, "four typed questions and contextual/plain decide")

def test_usage_refuses_the_reset_spelling() -> None:
    held = child("""
db = connect()
say(reset=run(db, "SELECT thinkthen_usage('reset')"), other=run(db, "SELECT thinkthen_usage(1)"))
""", environment(None))
    expect(held, {
        "reset": "thinkthen usage: the reset spelling is removed; the counters are cumulative, so take two snapshots and subtract them",
        "other": "thinkthen usage: thinkthen_usage takes no arguments; the counters are cumulative, so subtract two snapshots (retryable: no)",
    }, "the refusals")


SECRET_KEY, USER, PASSWORD = "sk-sentinel-3141", "sentinel-user", "sentinel-password"
EVERY_CALL = """
db = connect()
db.execute("CREATE TABLE e(id INTEGER, name TEXT, kind TEXT)")
db.executemany("INSERT INTO e VALUES (?, ?, ?)", [(1, "Ada", "person"), (2, "Acme", "organization")])
calls = ["SELECT thinkthen_configure('{}')", "SELECT count(*) FROM thinkthen_decide_many('Is it red?', json_object('7','a red door'))",
         "SELECT thinkthen_max_requests(NULL)", "SELECT thinkthen_max_requests_total(1000)",
         "SELECT thinkthen_cache('" + os.environ["SCRATCH"] + "/cache')"]
calls += [f"SELECT {name}('Is it red?', 'a red door'{deadline})" for name in ("thinkthen_decide", "thinkthen_details", "thinkthen_warm") for deadline in ("", ", 0")]
calls += ["SELECT thinkthen_choose('{\\"choose\\":\\"Which?\\",\\"options\\":[\\"a\\",\\"b\\"]}', 'x')",
          "SELECT thinkthen_score('{\\"score\\":\\"How?\\",\\"levels\\":[\\"l\\",\\"h\\"]}', 'x')",
          "SELECT thinkthen_tag('{\\"tag\\":\\"Which?\\",\\"labels\\":[\\"a\\",\\"b\\"]}', 'x')",
          "SELECT thinkthen_annotate('{\\"version\\":1,\\"questions\\":{\\"k\\":{\\"decide\\":\\"Is it red?\\"}}}', 'x')",
          "SELECT * FROM thinkthen_recognize('Ada joined Acme.', 'person')",
          "SELECT * FROM thinkthen_relate('SELECT id, name, kind FROM e', 'works_for=person:organization')",
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
    expect(sum("thinkthen backend: the backend answered with status 503 (retryable: yes)" in str(one)
               for one in busy["said"]), 9, f"the busy errors in {busy}")
    expect("thinkthen usage: THINKTHEN_BASE_URL: a base address carries no user information (retryable: no)" in refused["said"], True, "the refused address")


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
