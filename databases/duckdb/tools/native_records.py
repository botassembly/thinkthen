"""Native SQL collection checks shared by matching installed host runners."""
from harness import Backend, expect, rows


def check(run):
    with Backend() as backend:
        got = run([
            "SET thinkthen_model='jev-1.13.0'",
            "SELECT ordinal, value, result.value, facts.records, facts.requests_sent, failure.kind "
            "FROM thinkthen_decide({'decide':'Refund?','true':{'meaning':'refund'},"
            "'false':{'meaning':'other'}}, [{'text':'alpha'},{'text':'beta'}], "
            "reading := {'context':'policy','batch':2,'attempts':true}) ORDER BY ordinal",
            "SELECT ordinal, value, facts.requests_sent, failure.kind FROM "
            "thinkthen_choose({'choose':'Which team?','options':['billing','shipping']}, "
            "[{'text':'alpha'}])",
            "SELECT failure.kind, facts.requests_sent FROM thinkthen_decide('Refund?', "
            "[{'text':'alpha'}], reading := {'deadline_ms':0})",
            "SELECT count(*) FROM thinkthen_decide(NULL, [{'text':'alpha'}], "
            "reading := {'deadline_ms':'bad'})",
        ], backend.base())
        expect(rows(got[1]), [[0, True, '{"meaning":"refund"}', 2, 1, None],
                              [1, True, '{"meaning":"refund"}', 2, 1, None]],
               "plain native answers retain distinct authored complete values")
        expect(rows(got[2]), [[0, 'billing', 1, None]], "native collection choice")
        expect(rows(got[3]), [['deadline', 0]], "typed deadline failure retains zero sends")
        expect(rows(got[4]), [[0]], "required NULL shields invalid reading")
        expect(backend.count(), 2, "only the two admitted calls send")
    helpers(run)


def helpers(run):
    with Backend() as backend:
        got = run([
            "SET thinkthen_model='jev-1.13.0'",
            "SELECT ordinal,value,result.source.file,result.source.first_line,result.source.last_line "
            "FROM thinkthen_decide('Refund?', [thinkthen_record(text := 'alpha', "
            "source := thinkthen_location('example.txt',2,3))], "
            "reading := thinkthen_reading(batch := 1,stage_context := NULL))",
            "SELECT failure.kind,facts.requests_sent FROM thinkthen_filter('Refund?', "
            "[thinkthen_record(text := 'alpha')], reading := thinkthen_reading(files_only := true))",
            "SELECT value,result.source.file FROM thinkthen_filter('Refund?', "
            "[thinkthen_record(text := 'alpha',source := thinkthen_location('example.txt',2,3))], "
            "reading := thinkthen_reading(files_only := true))",
        ], backend.base())
        expect(rows(got[1]), [[0, True, 'example.txt', 2, 3]], "located native record")
        expect(rows(got[2]), [['usage', None]], "unlocated refusal has no invented facts")
        expect(rows(got[3]), [[True, 'example.txt']], "files_only accepts actual located input")
        expect(backend.count(), 1, "location preserves cache identity; refusal sends nothing")
