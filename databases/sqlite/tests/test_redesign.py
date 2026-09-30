#!/usr/bin/env python3
"""Selected installed-host witnesses for the accepted SQLite call shape."""

import json
import pathlib
import tempfile

from helper import Backend, Child, child, environment, expect, main


def test_plan_p1_exact_body_and_bad_inputs_never_send() -> None:
    backend = Backend()
    held = child("""
db = connect()
good = run(db, "SELECT thinkthen_plan(?, ?, ?)", ('asks for a refund', '{"7":"Refund me please."}', '{}'))
invalid = run(db, "SELECT thinkthen_plan(?, ?, ?)", ('asks for a refund', '{"7":"Refund me please."}', '{"unknwon":1}'))
repeated = run(db, "SELECT thinkthen_plan(?, ?, ?)", ('asks for a refund', '{"7":"one","7":"two"}', '{}'))
conflict = run(db, "SELECT thinkthen_plan(?, ?, ?)", ('{"decide":"asks for a refund","threshold":0.4}', '{"7":"Refund me please."}', '{"threshold":0.7}'))
choice = run(db, "SELECT thinkthen_plan(?, ?, ?)", ('Which team?', '{"x":"Refund me please."}', '{"options":["billing","shipping"]}'))
say(good=good, invalid=invalid, repeated=repeated, conflict=conflict, choice=choice)
""", environment(backend))
    plan = json.loads(held["good"][0][0])
    expect({key: value for key, value in plan.items() if key != "first_body_utf8"},
           {"records": 1, "requests": 1, "estimated_bytes": 182,
            "estimated_input_tokens": {"lower": 93, "upper": 166}, "upper_bound": False}, "P1 native JSON text")
    expect(plan["first_body_utf8"],
           '{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"Refund me please.\\". asks for a refund"}}}',
           "independent 182-byte first body")
    expect(len(plan["first_body_utf8"].encode()), 182, "P1 first body byte count")
    expect(held["good"][0][0], json.dumps(plan, sort_keys=True, separators=(",", ":")),
           "P1 plan text: compact, members in alphabetical order, byte for byte")
    choice = json.loads(held["choice"][0][0])
    expect((choice["records"], choice["requests"]), (1, 1), "choose settings plan")
    expect(all(member in choice["first_body_utf8"] for member in ("billing", "shipping")), True,
           "settings members entered the selected request")
    for name in ("invalid", "repeated", "conflict"):
        expect(held[name].startswith("thinkthen usage:"), True, name)
    expect(backend.close(), 0, "all previews and refusals sent nothing")


def test_process_total_refuses_the_second_packed_send() -> None:
    backend = Backend()
    held = child("""
db = connect()
db.execute("SELECT thinkthen_configure(?)", ('{"batch":1,"max_requests_total":1}',))
say(result=run(db, "SELECT key,value FROM thinkthen_decide_many(?, ?)",
               ('Is it red?', '{"a":"one red row","b":"another red row"}')))
""", environment(backend))
    expect(held["result"], "thinkthen usage: this process has sent its total of 1 requests (thinkthen_configure) (retryable: no)",
           "later packed attempt names the configured total")
    expect(backend.close(), 1, "one live send, refused second send")


def test_key_filter_obeys_sqlite_text_affinity_null_and_collation() -> None:
    backend = Backend()
    held = child("""
db = connect()
db.execute("CREATE TABLE ordinary(key TEXT, value INTEGER)")
db.executemany("INSERT INTO ordinary VALUES (?,1)", [('a',), ('7',)])
base = "SELECT key FROM thinkthen_decide_many(?, ?) WHERE "
packed = '{"a":"red","7":"red"}'
cases = [('key = ?', (None,)), ('key COLLATE NOCASE = ?', ('A',)), ('key = ?', (7,))]
say(actual=[run(db, base + predicate, ('Is it red?', packed) + args) for predicate, args in cases],
    ordinary=[run(db, 'SELECT key FROM ordinary WHERE ' + predicate, args) for predicate, args in cases])
""", environment(backend))
    expect(held["actual"], held["ordinary"], "the ordinary TEXT column is the comparison oracle")
    expect(held["actual"], [[], [["a"]], [["7"]]], "NULL, NOCASE and numeric affinity")
    expect(backend.close(), 1, "one judged result across three residual predicates")


def test_repeated_decoded_key_refuses_before_any_send() -> None:
    backend = Backend()
    held = child(r"""
db = connect()
say(raw=run(db, "SELECT count(*) FROM thinkthen_decide_many(?, ?)", ('Is it red?', '{"x":"red","x":"blue"}')),
    escaped=run(db, "SELECT count(*) FROM thinkthen_decide_many(?, ?)", ('Is it red?', r'{"x":"red","\u0078":"blue"}')))
""", environment(backend))
    for value in held.values():
        expect(value.startswith("thinkthen usage:"), True, "duplicate key refusal")
    expect(backend.close(), 0, "neither repeated spelling sent")


def test_changed_question_file_invalidates_connection_rows() -> None:
    backend = Backend()
    with tempfile.TemporaryDirectory() as folder:
        source = pathlib.Path(folder) / "question.json"
        source.write_text('{"decide":"Is it red?","model":"judge-a"}')
        held = child(f"""
import os
db = connect()
sql = "SELECT key,value FROM thinkthen_decide_many(?, ?)"
args = ('@{source}', '{{"x":"a red door"}}')
first = run(db, sql, args)
same = run(db, sql, args)
before = os.stat({str(source)!r}).st_mtime
open({str(source)!r}, 'w').write('{{"decide":"Is it red?","model":"judge-b"}}')
os.utime({str(source)!r}, (before + 2, before + 2))
changed = run(db, sql, args)
say(first=first, same=same, changed=changed)
""", environment(backend, "arm/full/capture"))
    expect(held, {"first": [["x", 1]], "same": [["x", 1]], "changed": [["x", 1]]}, "stable answer rows")
    bodies = [json.loads(body) for body in backend.capture()]
    expect([body["model"] for body in bodies], ["judge-a", "judge-b"], "file model reread")
    expect(backend.close(), 2, "unchanged reused; changed file sent once")


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
        '"q2":{"type":"noul","instructions":"The text is \\"Yellow Submarine\\". The text is the title of a song by the Beatles. It appears on the album Abbey Road."},'
        '"q3":{"type":"noul","instructions":"The text is \\"Octopus\'s Garden\\". The text is the title of a song by the Beatles. It appears on the album Abbey Road."},'
        '"q4":{"type":"noul","instructions":"The text is \\"Penny Lane\\". The text is the title of a song by the Beatles. It appears on the album Abbey Road."},'
        '"q5":{"type":"noul","instructions":"The text is \\"A Day in the Life\\". The text is the title of a song by the Beatles. It appears on the album Abbey Road."},'
        '"q6":{"type":"noul","instructions":"The text is \\"Hey Jude\\". The text is the title of a song by the Beatles. It appears on the album Abbey Road."}}}'
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
    expect(bodies[0], '{"state":"Each question quotes the text it asks about.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"The text is \\"Refund me please.\\". asks for a refund"}}}', "independent one-record body")
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
    expect(backend.count(), 1, "one send for the one thousand packed records, with no content cut (ADR 0111)")
    waiting.send()
    expect(waiting.result(timeout=60), {"exists": [[1000]]}, "EXISTS rows")
    expect(backend.close(), 1, "no new send for the second thousand key probes")


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
""", environment(backend, "generic"), timeout=300)
    # A hang guard: the 100,000-row join ran past 60 s under load (ticket 0352).
    expect(held["rows"], [[100000]], "all original keys joined")
    expect(any("SCAN t" in row[-1] for row in held["plan"]), True, "outer table scan")
    expect(any("VIRTUAL TABLE INDEX" in row[-1] for row in held["plan"]), True, "keyed virtual lookup")
    sends = backend.close()
    expect(sends > 0, True, "the independent listener saw live sends")


if __name__ == "__main__":
    raise SystemExit(main(globals()))
