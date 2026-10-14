"""Native SQL collection checks shared by matching installed host runners."""
from pathlib import Path
import tempfile

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
    sources(run)
    rich_overloads(run)
    null_members(run)


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
        expect(rows(got[3]), [[['"alpha"'], 'example.txt']], "files_only accepts actual located input")
        expect(backend.count(), 1, "location preserves cache identity; refusal sends nothing")


def sources(run):
    with tempfile.TemporaryDirectory(prefix='duckdb-native-source-') as directory, Backend() as backend:
        path = Path(directory) / 'records.txt'
        path.write_text('alpha\nbeta\n')
        quoted = "'" + str(path).replace("'", "''") + "'"
        got = run([
            "SET thinkthen_model='jev-1.13.0'",
            "SELECT ordinal,value,result.source.first_line,result.source.last_line,"
            "facts.records,facts.requests_sent,failure.kind FROM thinkthen_decide('Refund?',"
            f"thinkthen_source({quoted}),reading := thinkthen_reading(batch := 2)) ORDER BY ordinal",
            "SELECT failure.kind,facts.requests_sent FROM thinkthen_decide('Other?',"
            f"thinkthen_source({quoted}),reading := thinkthen_reading(deadline_ms := 0))",
        ], backend.base())
        expect(rows(got[1]), [[0, True, 1, 1, 2, 1, None], [1, True, 2, 2, 2, 1, None]],
               "native source readings retain physical coordinates and final facts")
        expect(rows(got[2]), [['deadline', None]], "unstarted source deadline has no invented facts")
        expect(backend.count(), 1, "spent source deadline sends nothing")


def rich_overloads(run):
    """Only an explicit named reading selects the two rich overloads."""
    question = "{'version':1,'relate':{'relations':[{'name':'near','source':'person','target':'person'}]}}"
    inputs = "[thinkthen_record(json := {'name':'Ada','kind':'person'}),thinkthen_record(json := {'name':'Bob','kind':'person'})]"
    with Backend() as backend:
        calls = [("rank", "{'decide':'Refund?'}", "[thinkthen_record(text := 'alpha')]"),
                 ("relate", question, inputs)]
        for verb, authored, records in calls:
            got = run([
                f"SELECT failure.kind FROM thinkthen_{verb}(question := {authored}, records := {records}, reading := NULL)",
                f"SELECT failure.kind FROM thinkthen_{verb}(question := {authored}, records := {records}, reading := {{'deadline_ms':'bad'}})",
                f"SELECT * FROM thinkthen_{verb}(question := {authored}, records := {records})",
                f"SELECT count(*) FROM thinkthen_{verb}(question := NULL, records := {records}, reading := {{'deadline_ms':'bad'}})",
            ], backend.base())
            expect(rows(got[0]), [[None]], "explicit NULL reading executes the rich call")
            expect(rows(got[1]), [['usage']], "invalid reading refuses")
            expect('error' in got[2], True, "missing reading does not silently choose a rich call")
            expect(rows(got[3]), [[0]], "required NULL masks invalid reading")
        expect(backend.count(), 2, "only explicit NULL reading calls send")


def null_members(run):
    """Authored JSON null remains present in ancillary native record members."""
    recognize = "{'version':1,'recognize':{'kinds':{'person':'A person name.'}}}"
    cases = [("decide", "'Refund?'", "context"),
             ("choose", "{'choose':'Team?','options':['billing','shipping']}", "options"),
             ("recognize", recognize, "examples"),
             ("recognize", recognize, "seed_spans")]
    with Backend() as backend:
        got = run([
            f"SELECT failure.kind,facts.requests_sent FROM thinkthen_{verb}({question}, "
            f"[thinkthen_record(text := 'Ada', {member} := 'null'::JSON)])"
            for verb, question, member in cases
        ], backend.base())
        expect([rows(item) for item in got], [[['usage', None]]] * len(cases),
               "authored null ancillary members refuse without invented facts")
        expect(backend.count(), 0, "authored null refuses before sends")
