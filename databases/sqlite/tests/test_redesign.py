#!/usr/bin/env python3
"""Selected installed-host witnesses for the accepted SQLite call shape."""

import json
import pathlib

from helper import Backend, Child, child, environment, expect, main


def test_six_song_e1_keyed_join_sends_one_exact_packed_body() -> None:
    """The accepted E1 query preserves six gapped ids in one captured send."""
    backend = Backend()
    titles = {"1": "Here Comes the Sun", "2": "Yellow Submarine", "5": "Octopus's Garden",
              "9": "Penny Lane", "14": "A Day in the Life", "17": "Hey Jude"}
    held = child(f"""
db = connect()
db.execute("CREATE TABLE songs(id INTEGER PRIMARY KEY, title TEXT)")
db.executemany("INSERT INTO songs VALUES (?,?)", {[(int(key), title) for key, title in titles.items()]!r})
question = "The text is the title of a song by the Beatles. It appears on the album Abbey Road."
sql = "SELECT s.id, d.value, d.probability FROM songs AS s JOIN thinkthen_decide_many(?, (SELECT json_group_object(id,title) FROM songs), ?) AS d ON d.key = CAST(s.id AS TEXT) ORDER BY s.id"
say(rows=run(db, sql, (question, '{{"threshold":"0.3:0.7"}}')))
""", environment(backend, "arm/full/capture"))
    expect(held["rows"], [[1, 1, 0.9], [2, 1, 0.9], [5, 1, 0.9], [9, 1, 0.9], [14, 1, 0.9], [17, 1, 0.9]], "E1 keyed gap and values")
    expected = (
        '{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{'
        '"q1":{"type":"noul","instructions":"The text is \\"Here Comes the Sun\\". The text is the title of a song by the Beatles. It appears on the album Abbey Road."},'
        '"q2":{"type":"noul","instructions":"The text is \\"A Day in the Life\\". The text is the title of a song by the Beatles. It appears on the album Abbey Road."},'
        '"q3":{"type":"noul","instructions":"The text is \\"Hey Jude\\". The text is the title of a song by the Beatles. It appears on the album Abbey Road."},'
        '"q4":{"type":"noul","instructions":"The text is \\"Yellow Submarine\\". The text is the title of a song by the Beatles. It appears on the album Abbey Road."},'
        '"q5":{"type":"noul","instructions":"The text is \\"Octopus\'s Garden\\". The text is the title of a song by the Beatles. It appears on the album Abbey Road."},'
        '"q6":{"type":"noul","instructions":"The text is \\"Penny Lane\\". The text is the title of a song by the Beatles. It appears on the album Abbey Road."}}}'
    )
    expect(backend.capture(), [expected], "one literal E1 request body")
    expect(backend.close(), 1, "one E1 packed send")


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


def test_two_alternating_slots_reuse_rows_with_disk_cache_disabled() -> None:
    """A later A result comes from the connection slot, not the engine cache."""
    backend = Backend()
    held = child("""
db = connect()
db.execute("SELECT thinkthen_configure(?)", ('{"cache":false}',))
sql = "SELECT key,value FROM thinkthen_decide_many(?,?)"
a = ('Is it red?', '{"a":"red one"}')
b = ('Is it red?', '{"b":"blue two"}')
say(first=run(db, sql, a), second=run(db, sql, b), again=run(db, sql, a))
""", environment(backend, "generic"))
    expect(held, {"first": [["a", 1]], "second": [["b", 1]], "again": [["a", 1]]}, "alternating answers")
    expect(backend.close(), 2, "two sends; the final A used its connection slot")


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


def test_scalar_settings_use_shared_fields_and_refuse_before_send() -> None:
    backend = Backend()
    held = child("""
db = connect()
say(unknown=run(db, "SELECT thinkthen_decide(?,?,?)", ("asks for a refund", "Refund me please.", '{"unknwon":1}')),
    duplicate=run(db, "SELECT thinkthen_decide(?,?,?)", ('{"decide":"asks for a refund","threshold":0.5}', "Refund me please.", '{"threshold":0.7}')),
    spent=run(db, "SELECT thinkthen_decide(?,?,?)", ("asks for a refund", "Refund me please.", '{"deadline_ms":0}')),
    good=run(db, "SELECT thinkthen_decide(?,?,?)", ("asks for a refund", "Refund me please.", '{}')),
    choose=run(db, "SELECT thinkthen_choose(?,?,?)", ("Which team?", "Refund me please.", '{"options":["billing","shipping"]}')))
""", environment(backend, "arm/full/capture"))
    expect(held["unknown"].startswith("thinkthen usage:"), True, "unknown settings key")
    expect(held["duplicate"].startswith("thinkthen usage:"), True, "repeated question field")
    expect(held["spent"].startswith("thinkthen deadline:"), True, "spent validated deadline")
    expect(held["good"], [[1]], "ordinary answer")
    expect(held["choose"], [["billing"]], "choice members from settings")
    bodies = backend.capture()
    expect(bodies[0], '{"state":"Refund me please.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}', "independent one-record body")
    expect(len(bodies), 2, "only valid calls sent")
    expect(backend.close(), 2, "invalid settings and spent deadline sent nothing")


def test_correlated_and_exists_probe_a_thousand_rows_without_rejudging() -> None:
    """The bounded correlated shapes re-enter xFilter but reuse one answer set."""
    backend = Backend()
    waiting = Child("""
db = connect()
db.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, body TEXT)")
db.executemany("INSERT INTO t VALUES (?, ?)", [(at, f'row {at}') for at in range(1000)])
db.execute("CREATE TEMP TABLE packed AS SELECT json_group_object(id, body) AS payload FROM t")
source = '(SELECT payload FROM packed)'
correlated = run(db, "SELECT count(*) FROM t WHERE (SELECT value FROM thinkthen_decide_many('Is it red?', " + source + ") WHERE key = CAST(t.id AS TEXT)) = 1")
say(correlated=correlated)
sys.stdin.readline()
exists = run(db, "SELECT count(*) FROM t WHERE EXISTS (SELECT 1 FROM thinkthen_decide_many('Is it red?', " + source + ") WHERE key = CAST(t.id AS TEXT) AND value = 1)")
say(exists=exists)
""", environment(backend, "generic"))
    expect(waiting.read(timeout=60), {"correlated": [[1000]]}, "correlated rows")
    expect(backend.count(), 3, "three sends for the one thousand packed records")
    waiting.send()
    expect(waiting.result(timeout=60), {"exists": [[1000]]}, "EXISTS rows")
    expect(backend.close(), 3, "no new send for the second thousand key probes")


def test_aggregate_once_join_is_bounded_at_one_hundred_thousand() -> None:
    """The large functional join aggregates once; it is never correlated."""
    backend = Backend()
    held = child("""
db = connect()
db.execute("CREATE TABLE t(id INTEGER PRIMARY KEY, body TEXT)")
db.executemany("INSERT INTO t VALUES (?,?)", [(at, f'row {at}') for at in range(100000)])
db.execute("CREATE TEMP TABLE packed AS SELECT json_group_object(id,body) AS payload FROM t")
sql = "SELECT count(d.key) FROM t LEFT JOIN thinkthen_decide_many('Is it red?', (SELECT payload FROM packed)) AS d ON d.key = CAST(t.id AS TEXT)"
say(plan=run(db, 'EXPLAIN QUERY PLAN ' + sql), rows=run(db, sql))
""", environment(backend, "generic"), timeout=60)
    expect(held["rows"], [[100000]], "all original keys joined")
    expect(any("SCAN t" in row[-1] for row in held["plan"]), True, "outer table scan")
    expect(any("VIRTUAL TABLE INDEX" in row[-1] for row in held["plan"]), True, "keyed virtual lookup")
    sends = backend.close()
    expect(sends > 0, True, "the independent listener saw live sends")


if __name__ == "__main__":
    raise SystemExit(main(globals()))
