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
    native_objects(run)
    original_settings_and_nulls(run)
    packed_settings_failures(run)


def native_objects(run):
    """Native SQL objects retain authored question maps, controls and keyed order."""
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_decide({'decide':'Refund?','true':{'meaning':'refund','optional':NULL},'false':{'meaning':'other','enabled':FALSE}}, 'hello', settings := {'batch':1,'deadline_ms':-1})",
            "SELECT thinkthen_decide('{\"decide\":\"Refund?\",\"true\":{\"meaning\":\"refund\",\"optional\":null},\"false\":{\"meaning\":\"other\",\"enabled\":false}}', 'hello', '{\"batch\":1,\"deadline_ms\":-1}')",
            "SELECT thinkthen_choose('Which team?', 'hello', {'billing':'Charges','shipping':NULL}, settings := {'batch':'max'})",
            "SELECT thinkthen_score('How strong?', 'hello', {'weak':NULL,'strong':'Strong evidence'}, settings := {'batch':'max'})",
            "SELECT thinkthen_tag('Which topics?', 'hello', {'billing':'Charges','shipping':NULL}, settings := {'batch':'max'})",
            "SELECT key, value FROM thinkthen_decide_many({'decide':'Refund?'}, MAP(['first','second'],['alpha','beta']), settings := {'batch':'max'})",
            "SELECT p.records, len(json_keys(json_extract(p.first_body, '$.questions'))) FROM (SELECT thinkthen_plan({'decide':'Refund?'}, {'first':'alpha','second':'beta'}, settings := {'batch':'max'}, model := 'jev-1.13.0') p)",
            "SELECT thinkthen_decide({'decide':'Refund?','threshold':0.7}, 'bad', settings := {'threshold':0.8})",
        ], backend.base())
        expect([rows(item)[0][0] for item in got[:5]], [True, True, 'billing', 0.1, ['billing','shipping']], 'native object answers')
        expect(rows(got[5]), [['first',True],['second',True]], 'native keyed input order')
        expect(rows(got[6]), [[2,2]], 'native plan packed body')
        expect(said(got[7]).startswith('thinkthen usage:'), True, 'native object conflict refusal')
        expect(backend.count(), 5, 'equivalent authored objects reuse cache; plan and conflict send nothing')


def original_settings_and_nulls(run):
    """Malformed originals and duplicate keys never send; required NULL stops casts."""
    with Backend() as backend:
        got = run([
            "SELECT thinkthen_decide('Refund?', 'bad', '{\"batch\":1]', context := 'policy')",
            "SELECT thinkthen_choose('Which?', 'bad', ['a','b'], '{\"batch\":1]', context := 'policy')",
            "SELECT thinkthen_decide('Refund?', 'bad', '{\"batch\":1,\"batch\":2}', context := 'policy')",
            "SELECT thinkthen_decide('Refund?', 'bad', '{\"context\":\"policy\"}', context := 'other')",
            "SELECT thinkthen_decide(NULL, 'bad', '{\"batch\":1]', deadline_ms := 'bad')",
            "SELECT thinkthen_decide('Refund?', NULL, allow_none := 'bad')",
            "SELECT thinkthen_choose(NULL, 'bad', ['a','b'], deadline_ms := 'bad')",
            "SELECT thinkthen_tag('Topics?', NULL, ['a','b'], allow_none := 'bad')",
            "SELECT thinkthen_find(NULL, ['a','b'], deadline_ms := 'bad')",
            "SELECT thinkthen_recognize(NULL, ['person'], allow_none := 'bad')",
            "SELECT thinkthen_plan(NULL, {'first':'bad'}, deadline_ms := 'bad')",
            "SELECT * FROM thinkthen_decide_many(NULL, {'first':'bad'}, allow_none := 'bad')",
        ], backend.base())
        for item in got[:4]:
            expect(said(item).startswith('thinkthen usage:'), True, 'original settings refusal')
        expect(said(got[2]), 'thinkthen usage: a JSON record holds each member name once, and one name arrived twice (retryable: no)', 'duplicate keys remain refused')
        expect([rows(item)[0][0] for item in got[4:11]], [None]*7, 'required NULL precedes typed control casts')
        expect(rows(got[11]), [], 'required NULL collection returns no rows')
        expect(backend.count(), 0, 'original settings failures and NULL inputs send nothing')


def packed_settings_failures(run):
    """Packed safe failures preserve invalid originals and later valid records."""
    with Backend() as backend:
        got = run([
            "SELECT id, thinkthen_try_details('Refund?', body, settings, context := 'policy') AS detail "
            "FROM (VALUES (1,'alpha','{\"batch\":\"max\"}'), (2,'broken','{\"batch\":1]'), "
            "(3,'beta','{\"batch\":\"max\"}'), (4,'broken','{\"batch\":1]'), "
            "(5,'duplicate','{\"batch\":1,\"batch\":2}'), "
            "(6,'duplicate','{\"batch\":1,\"batch\":2}'), (7,'gamma','{\"batch\":1}')) "
            "s(id,body,settings) ORDER BY id",
            "SELECT thinkthen_try_details('Refund?', 'broken', settings) "
            "FROM (VALUES ('{\"batch\":1]'), ('{\"batch\":1,\"batch\":2}')) s(settings)",
        ], backend.base('arm/full/capture'))
        import json
        values = [(key, json.loads(detail) if isinstance(detail, str) else detail)
                  for key, detail in rows(got[0])]
        expect([key for key, _ in values], list(range(1, 8)), 'original row order')
        failed = {'status': 'failed', 'error': {'kind': 'usage', 'message':
                  "check the row's question and arguments, or raise the process request total when it is spent",
                  'retryable': False}}
        expect([value for _, value in values if value['status'] == 'failed'], [failed]*4,
               'four exact old safe failures without added facts')
        expect([json.loads(value) if isinstance(value, str) else value for value, in rows(got[1])],
               [failed]*2, 'original-only settings retain the same safe failure')
        expect([(key, value['status'], value['details']['value']) for key, value in values
                if value['status'] == 'answered'], [(1,'answered',True), (3,'answered',True), (7,'answered',True)],
               'valid siblings and later independent record answer')
        prefix = '{"state":"policy","model":"jev-1.13.0","questions":'
        def question(text):
            return '{"type":"noul","instructions":"The text is ' + chr(92) + '"' + text + chr(92) + '". Refund?"}'
        expected = [prefix + '{"q1":' + question('alpha') + ',"q2":' + question('beta') + '}}',
                    prefix + '{"q1":' + question('gamma') + '}}']
        expect(backend.count(), 2, 'invalid originals send nothing; valid groups send once')
        expect(sorted(backend.capture()), sorted(expected), 'exact valid send multiset')
