"""One SQL behavior case shared by matching installed host runners."""
from harness import Backend, expect, rows, said


def check(run):
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_decide(input := 'refund now', question := 'Is it a refund?', "
            "context := 'policy', model := 'jev-1.13.0', batch := 1, deadline_ms := -1, "
            "true_reading := {'meaning': 'refund', 'optional': NULL}, false_reading := {'meaning': 'other', 'enabled': FALSE})",
            "SELECT thinkthen_decide('Is it a refund?', 'refund now', "
            "'{\"context\":\"policy\",\"model\":\"jev-1.13.0\",\"batch\":1,\"deadline_ms\":-1,"
            "\"true\":{\"meaning\":\"refund\",\"optional\":null},\"false\":{\"meaning\":\"other\",\"enabled\":false}}')",
            "SELECT thinkthen_decide('Is it a refund?', 'refund now', ' { } ', context := 'policy', "
            "model := 'jev-1.13.0', batch := 1, deadline_ms := -1, "
            "true_reading := {'meaning': 'refund', 'optional': NULL}, false_reading := {'meaning': 'other', 'enabled': FALSE})",
            "SELECT thinkthen_choose('Which team?', 'refund now', ['billing','shipping'], batch := 'max')",
            "SELECT thinkthen_choose('Which team?', 'refund now', ['billing','shipping'], '{\"threshold\":0.7}', batch := 'max')",
            "SELECT thinkthen_decide('Is it a refund?', 'other evidence', deadline_ms := 0)",
            "SELECT thinkthen_decide('Is it a refund?', 'other evidence', '{\"context\":\"policy\"}', context := 'other')",
        ], backend.base())
        for result in got[:3]:
            expect(rows(result), [[True]], "named controls retain typed authored readings")
        expect(rows(got[3]), [["billing"]], "listed members and named batch")
        expect(rows(got[4]), [["billing"]], "typed members coexist with settings and named controls")
        expect(said(got[5]).startswith("thinkthen deadline:"), True, "zero deadline stays present")
        expect(said(got[6]).startswith("thinkthen usage:"), True, "repeated named controls refuse")
        expect(backend.count(), 2, "equivalent controls share cache; refused controls send nothing")
