"""Selected installed DuckDB proofs for the portable SQL call forms."""

from __future__ import annotations

import json

from harness import Backend, case, expect, main, rows, run, said


@case
def named_decide_binds_by_name_and_settings_refuse_before_send():
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_decide(settings := '{\"threshold\":0.7}', input := 'refund now', question := 'Is it a refund?')",
            "SELECT thinkthen_decide('Is it a refund?', 'refund now', '{\"threshold\":1.5}')",
            "SELECT thinkthen_decide('Is it a refund?', 'refund now', '{\"threshold\":0.7}', '0.8')",
            "SELECT thinkthen_decide(stare := 'x', question := 'Is it a refund?', input := 'refund now')",
            "SELECT thinkthen_decide('Is it a refund?', 'refund now', 0)",
        ], backend.base())
        expect(rows(got[0]), [[True]], "out-of-order named decide")
        expect(said(got[1]).startswith("thinkthen usage:"), True, "invalid settings")
        expect(said(got[2]), "thinkthen usage: settings repeats `threshold` from the question or named arguments", "duplicate threshold")
        expect("does not support the supplied arguments" in got[3]["error"], True, "unknown name")
        expect(said(got[4]), "thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'", "removed deadline slot")
        expect(backend.count(), 1, "only the valid call sends")


@case
def listed_settings_and_members_are_distinct_overloads():
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_choose('Which team?', 'refund now', '{\"options\":[\"billing\",\"shipping\"]}')",
            "SELECT thinkthen_choose('Which team?', 'refund now', ['billing','shipping'])",
            "SELECT thinkthen_choose('Which team?', 'refund now', ['billing','shipping'], '{\"threshold\":0.7}')",
            "SELECT thinkthen_choose('Which team?', 'refund now', ['billing','shipping'], 0)",
            "SELECT thinkthen_choose('Which team?', 'refund now', ['billing','shipping'], '{\"options\":[\"billing\",\"shipping\"]}')",
            "SELECT thinkthen_choose(stare := 'x', question := 'Which team?', input := 'refund now')",
        ], backend.base())
        for result in got[:3]:
            expect(rows(result), [["billing"]], "listed overload value")
        expect(said(got[3]), "thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'", "removed listed deadline")
        expect(said(got[4]), "thinkthen usage: a members argument and settings both name members", "duplicate members")
        expect("does not support the supplied arguments" in got[5]["error"], True, "unknown listed name")
        expect(backend.count(), 1, "the same question and input reuse one cached request across local cuts")


@case
def keyed_decide_preserves_key_body_and_one_send():
    with Backend() as backend:
        got = run([
            "SELECT key, value, probability FROM thinkthen_decide_many('asks for a refund', '{\"7\":\"Refund me please.\"}')",
        ], backend.base("arm/full/capture"))
        expect(rows(got[0]), [["7", True, 0.9]], "keyed row")
        expect(backend.count(), 1, "one keyed request")
        expect(backend.capture(), [
            '{"state":"Refund me please.","model":"jev-1.13.0","questions":{"q1":{"type":"noul","instructions":"asks for a refund"}}}'
        ], "independent one-record body")


@case
def keyed_verbs_keep_members_order_and_refuse_duplicate_keys():
    with Backend() as backend:
        got = run([
            "SELECT key, value, probability FROM thinkthen_choose_many('Which team?', '{\"9\":\"refund now\"}', '{\"options\":[\"billing\",\"shipping\"]}')",
            "SELECT key, value FROM thinkthen_score_many('How strong?', '{\"9\":\"refund now\"}', '{\"levels\":[\"weak\",\"strong\"]}')",
            "SELECT key, value FROM thinkthen_tag_many('Which topics?', '{\"9\":\"refund now\"}', '{\"labels\":[\"billing\",\"shipping\"]}')",
            "SELECT * FROM thinkthen_decide_many('Is it a refund?', '{\"9\":\"refund now\",\"9\":\"good morning\"}')",
            "SELECT * FROM thinkthen_choose_many('Which team?', '{\"9\":4}', '{\"options\":[\"billing\",\"shipping\"]}')",
        ], backend.base())
        expect(rows(got[0]), [["9", "billing", 0.9]], "choose keyed value and probability")
        # The fixed backend assigns 0.9 to level zero and 0.1 to level one.
        expect(rows(got[1]), [["9", 0.1]], "score weighted position")
        expect(rows(got[2]), [["9", ["billing", "shipping"]]], "tag keyed value")
        expect(said(got[3]).startswith("thinkthen usage:"), True, "duplicate keyed input")
        expect(said(got[4]).startswith("thinkthen usage:"), True, "non-text keyed input")
        expect(backend.count(), 3, "invalid keyed inputs send nothing")


@case
def six_song_vector_and_keyed_forms_share_one_packed_body():
    question = "The text is the title of a song by the Beatles. It appears on the album Abbey Road."
    settings = '{"threshold":"0.3:0.7"}'
    songs = [(1, "Here Comes the Sun"), (2, "Yellow Submarine"),
             (5, "Octopus's Garden"), (9, "Penny Lane"),
             (14, "A Day in the Life"), (17, "Hey Jude")]
    quote = lambda value: "'" + value.replace("'", "''") + "'"
    values = ", ".join(f"({key}, {quote(title)})" for key, title in songs)
    keyed = json.dumps({str(key): title for key, title in songs})
    with Backend() as backend:
        got = run([
            f"SELECT id, thinkthen_decide({quote(question)}, title, {quote(settings)}) FROM (VALUES {values}) s(id,title) ORDER BY id",
            f"SELECT key, value, probability FROM thinkthen_decide_many({quote(question)}, {quote(keyed)}, {quote(settings)}) ORDER BY CAST(key AS BIGINT)",
        ], backend.base("arm/full/capture"))
        expect(rows(got[0]), [[1, True], [2, True], [5, True], [9, True], [14, True], [17, True]],
               "six vector positions")
        expect(rows(got[1]), [["1", True, 0.9], ["2", True, 0.9], ["5", True, 0.9],
                              ["9", True, 0.9], ["14", True, 0.9], ["17", True, 0.9]],
               "keyed ids with gaps")
        expect(backend.count(), 1, "the keyed form reads the vector's cached packed request")
        first = backend.capture()
        expect(len(first), 1, "one captured packed request")
        expect(rows(run([
            f"SELECT key, value, probability FROM thinkthen_decide_many({quote(question)}, {quote(keyed)}, {quote(settings)}) ORDER BY CAST(key AS BIGINT)"
        ], backend.base("arm/full/capture"))[0]),
               [["1", True, 0.9], ["2", True, 0.9], ["5", True, 0.9],
                ["9", True, 0.9], ["14", True, 0.9], ["17", True, 0.9]],
               "fresh keyed execution")
        expect(backend.count(), 2, "fresh cache requires one more packed request")
        bodies = backend.capture()
        expect(len(bodies), 2, "two independent captured requests")
        expect(bodies[0], bodies[1], "vector and keyed requests have identical bytes")


@case
def removed_warm_forms_refuse_without_transport():
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_warm('Is it a refund?', 'refund now')",
            "SELECT thinkthen_warm('Is it a refund?', 'refund now', 'shared')",
        ], backend.base())
        for result in got:
            expect(said(result),
                   "thinkthen usage: thinkthen_warm was removed; pack records with thinkthen_decide_many",
                   "removed warm sentence")
        expect(backend.count(), 0, "removed warm opens no request")


@case
def probability_and_details_share_one_portable_request():
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_probability('Is it a refund?', 'refund now', '{\"threshold\":0.7}')",
            "SELECT thinkthen_details('Is it a refund?', 'refund now', '{\"threshold\":0.7}')",
            "SELECT thinkthen_probability('Is it a refund?', 'refund now', 0)",
        ], backend.base())
        expect(rows(got[0]), [[0.9]], "probability from one answer")
        detail = json.loads(rows(got[1])[0][0])
        expect(detail["value"], True, "details retains the decision")
        expect(said(got[2]),
               "thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'",
               "removed probability deadline")
        expect(backend.count(), 1, "probability and details read one cache identity")


@case
def annotate_accepts_call_settings_and_refuses_question_only_keys():
    question_set = '{"version":1,"questions":{"refund":{"decide":"Is it a refund?"}}}'
    with Backend() as backend:
        got = run([
            f"SELECT thinkthen_annotate('{question_set}', 'refund now', '{{\"batch\":\"max\"}}')",
            f"SELECT thinkthen_annotate('{question_set}', 'refund now', '{{\"none\":true}}')",
            f"SELECT thinkthen_annotate('{question_set}', 'refund now', 0)",
        ], backend.base())
        expect(json.loads(rows(got[0])[0][0]), {"refund": True}, "annotated set value")
        expect(said(got[1]).startswith("thinkthen usage:"), True, "none is find-only")
        expect(said(got[2]),
               "thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'",
               "removed annotate deadline")
        expect(backend.count(), 1, "invalid annotation settings send nothing")


@case
def try_details_keeps_recoverable_settings_failures_beside_an_answer():
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_try_details('Is it a refund?', 'refund now', '{\"threshold\":0.7}')",
            "SELECT thinkthen_try_details('Is it a refund?', 'refund now', '{\"threshold\":1.5}')",
            "SELECT thinkthen_try_details('Is it a refund?', 'refund now', NULL)",
            "SELECT thinkthen_try_details('Is it a refund?', 'refund now', 1)",
        ], backend.base())
        expect(json.loads(rows(got[0])[0][0])["status"], "answered", "portable answer")
        failed = json.loads(rows(got[1])[0][0])
        expect((failed["status"], failed["error"]["kind"]), ("failed", "usage"),
               "invalid settings retain the safe failure carrier")
        expect(json.loads(rows(got[2])[0][0])["status"], "answered", "NULL settings are absent")
        expect(said(got[3]),
               "thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'",
               "removed try-details deadline")
        expect(backend.count(), 1, "only one request identity was sent")


@case
def recognize_named_settings_and_old_deadline_boundary():
    with Backend() as backend:
        got = run([
            "SELECT r.text, r.start FROM (SELECT unnest(thinkthen_recognize(settings := '{\"model\":\"jev-1.13.0\"}', kinds := ['person'], input := 'Le café 😀 Maria Chen arrived.')) AS r)",
            "SELECT thinkthen_recognize('Maria Chen arrived.', ['person'], '{\"none\":true}')",
            "SELECT thinkthen_relations('Maria Chen arrived.', '{\"version\":1,\"recognize\":{\"kinds\":{\"person\":\"A name of a person.\"}}}', 0)",
        ], backend.base())
        expect(rows(got[0]), [["arrived.", 21]], "named recognize model and code-point start")
        expect(said(got[1]).startswith("thinkthen usage:"), True, "none belongs only to find")
        expect(said(got[2]),
               "thinkthen usage: the deadline and context moved into the settings object; pass '{\"deadline_ms\": …, \"context\": …}'",
               "removed relations deadline")
        expect(backend.count(), 2, "the one valid recognized name uses steps 1 and 2 only")


if __name__ == "__main__":
    raise SystemExit(main())
