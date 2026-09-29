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


if __name__ == "__main__":
    raise SystemExit(main())
