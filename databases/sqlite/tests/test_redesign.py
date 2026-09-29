#!/usr/bin/env python3
"""Selected installed-host witnesses for the accepted SQLite call shape."""

import json
import pathlib

from helper import Backend, child, environment, expect, main


def test_keyed_decide_reuses_one_packed_reply_across_key_probes() -> None:
    backend = Backend()
    packed = json.dumps({"a": "A short note.", "b": "Another short note."})
    held = child(f"""
db = connect()
sql = "SELECT key, value, probability FROM thinkthen_decide_many(?, ?, ?)"
args = ("Does this need attention?", {packed!r}, '{{}}')
say(first=run(db, sql, args),
    a=run(db, sql + " WHERE key = 'a'", args),
    b=run(db, sql + " WHERE key = 'b'", args),
    missing=run(db, sql + " WHERE key = 'missing'", args),
    plan=run(db, "EXPLAIN QUERY PLAN " + sql + " WHERE key = 'a'", args))
""", environment(backend, "generic"))
    expect(held["first"], [["a", 1, 0.9], ["b", 1, 0.9]], "keyed answers")
    expect(held["a"], [["a", 1, 0.9]], "first key lookup")
    expect(held["b"], [["b", 1, 0.9]], "second key lookup")
    expect(held["missing"], [], "absent key")
    expect(any("VIRTUAL TABLE INDEX" in row[-1] for row in held["plan"]), True, "planned virtual index")
    expect(backend.close(), 1, "one packed backend request")


def test_each_keyed_shape_and_probability_boundary() -> None:
    backend = Backend()
    held = child("""
db = connect()
packed = '{"7":"A short note."}'
say(choose=run(db, "SELECT key,value,probability FROM thinkthen_choose_many(?,?,?)", ("Which team?", packed, '{"options":["billing","shipping"]}')),
    score=run(db, "SELECT key,value FROM thinkthen_score_many(?,?,?)", ("How strong?", packed, '{"levels":["low","high"]}')),
    bad_score=run(db, "SELECT probability FROM thinkthen_score_many(?,?,?)", ("How strong?", packed, '{"levels":["low","high"]}')),
    bad_tag=run(db, "SELECT probability FROM thinkthen_tag_many(?,?,?)", ("Which labels?", packed, '{"labels":["billing"]}')))
""", environment(backend, "generic"))
    expect(held["choose"], [["7", "billing", 0.9]], "choice and winning probability")
    expect(held["score"], [["7", 0.1]], "score row without probability")
    for name in ("bad_score", "bad_tag"):
        expect("no such column: probability" in held[name], True, name)
    expect(backend.close(), 2, "one send per packed choose and score")

    case = next(item for item in json.loads((pathlib.Path(__file__).resolve().parents[3] / "conformance/cases.json").read_text())["cases"] if item["id"] == "09-tag-two")
    evidence = case["exchanges"][0]["evidence"]
    question = json.dumps(case["question"], separators=(",", ":"))
    backend = Backend()
    held = child(f"""
db = connect()
say(rows=run(db, "SELECT key,value FROM thinkthen_tag_many(?,?,?)", ({question!r}, {json.dumps({'7': evidence})!r}, '{{}}')))
""", environment(backend, "case/09-tag-two"))
    expect(held["rows"], [["7", json.dumps(case["expect"]["success"]["answers"][0]["bare"], separators=(",", ":"))]], "tag JSON row")
    expect(backend.close(), 1, "one tagged packed send")


if __name__ == "__main__":
    raise SystemExit(main(globals()))
