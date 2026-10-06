"""Public DuckDB rank-set macros over independent saved SQL exchanges."""
import json
import pathlib
import sys
import tempfile

from harness import Backend, case, expect, main, rows, run, said

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[2] / 'sqlite/tests'))
from rank_backend import EXPECTED, ONE, RECORDS, SET, RankBackend
from conditional_backend import ConditionalBackend


def quoted(text):
    return "'" + text.replace("'", "''") + "'"


def ranked(spec=SET, records=RECORDS, settings='{"batch":1}'):
    return 'SELECT key,rank,probability,question_name,facts FROM thinkthen_rank_set(' + ','.join(map(quoted, [spec,records,settings])) + ')'


@case
def turns_preserve_keys_duplicate_text_names_facts_and_limit():
    with RankBackend() as backend:
        got = run(["SET thinkthen_model='judge-rank'", ranked(), ranked() + ' LIMIT 3'], backend.base)
        expect([row[:4] for row in rows(got[1])], EXPECTED, 'independent turns')
        expect([row[:4] for row in rows(got[2])], EXPECTED[:3], 'LIMIT after merge')
        facts = [json.loads(row[4]) for row in rows(got[1])]
        expect(all(fact == facts[0] for fact in facts), True, 'one combined call')
        expect((facts[0]['records'],facts[0]['requests_sent']), (4,3), 'records and actual sends')
        expect(backend.count(), 3, 'second call uses native cache')


@case
def one_member_matches_plain_wire_and_individual_member_recordings_replay_zero():
    with RankBackend() as backend, tempfile.TemporaryDirectory() as folder:
        plain = 'SELECT key,rank,probability FROM thinkthen_rank(' + ','.join(map(quoted,['First?',RECORDS,'{"batch":1}'])) + ')'
        got = run(["SET thinkthen_model='judge-rank'", plain], backend.base)
        first = list(backend.bodies)
        start = backend.count()
        one = run(["SET thinkthen_model='judge-rank'", ranked(ONE)], backend.base)
        expect([row[:3] for row in rows(one[1])], rows(got[1]), 'plain one-member equivalence')
        expect(sorted(json.dumps(body) for body in backend.bodies[start:]),
               sorted(json.dumps(body) for body in first), 'exact same wire')
        seeded = run(["SET thinkthen_model='judge-rank'", 'SET thinkthen_record=' + quoted(folder),
                      plain, plain.replace('First?', 'Second?')], backend.base)
        expect(len(rows(seeded[2])), 4, 'first individual recording')
        expect(len(rows(seeded[3])), 4, 'second individual recording')
        before = backend.count()
        replay = run(["SET thinkthen_model='judge-rank'", 'SET thinkthen_replay=' + quoted(folder),
                      'SET thinkthen_max_requests_total=0', ranked(), ranked(SET.replace('Second?', 'Missing private question?'))],
                     backend.base, keyless=True)
        expect([row[:4] for row in rows(replay[3])], EXPECTED, 'strict replay from plain members')
        facts = json.loads(rows(replay[3])[0][4])
        expect((facts['records'],facts['requests_sent'],facts['cache_answers']), (4,0,0), 'combined replay facts')
        expect(said(replay[4]), 'thinkthen local: the replay folder holds no answer for this question (retryable: no)', 'strict miss')
        expect(backend.count(), before, 'keyless zero-budget replay and miss send nothing')
        reordered = '{"version":1,"questions":{"renamed":{"decide":"Second?"},"last":{"decide":"First?"}}}'
        renamed = run(["SET thinkthen_model='judge-rank'", 'SET thinkthen_replay=' + quoted(folder),
                       'SET thinkthen_max_requests_total=0', ranked(reordered)], backend.base, keyless=True)
        expect([row[:4] for row in rows(renamed[3])], [['a',1,1.0,'renamed'],['d',2,1.0,'renamed'],
                                                      ['c',3,.99,'renamed'],['b',4,.6,'last']], 'reorder and rename reuse member keys')
        expect(backend.count(), before, 'reordered strict replay sends zero')


@case
def descriptions_and_author_order_survive_inline_and_file_sets():
    described = SET.replace('"decide":"First?"', '"decide":"First?","true":"supports first","false":"opposes first"')
    reverse = '{"version":1,"questions":{"renamed":{"decide":"Second?"},"last":{"decide":"First?"}}}'
    with RankBackend() as backend, tempfile.TemporaryDirectory() as folder:
        path = pathlib.Path(folder) / 'rank.json'
        path.write_text(described)
        got = run(["SET thinkthen_model='judge-rank'", ranked('@' + str(path), settings='{"batch":1,"context":"shared note"}'),
                   ranked(reverse)], backend.base)
        expect([row[:4] for row in rows(got[1])], EXPECTED, 'described members')
        expect([row[:4] for row in rows(got[2])], [['a',1,1.0,'renamed'],['d',2,1.0,'renamed'],
                                                  ['c',3,0.99,'renamed'],['b',4,0.6,'last']], 'saved member order')
        questions = [question for body in backend.bodies for question in body['questions'].values() if 'criteria' in question]
        expect(questions[0]['criteria'], {'true':'supports first','false':'opposes first'}, 'authored meanings')
        expect(backend.bodies[0]['model'], 'judge-rank', 'session model')
        expect(json.dumps(backend.bodies[0]).count('shared note'), 1, 'shared context once')


@case
def null_and_invalid_set_admission_send_nothing():
    bad = ['{"version":1,"questions":{}}', SET.replace('"decide":"Second?"', '"score":"Second?","levels":["low","high"]'),
           SET.replace('"decide":"First?"', '"decide":"First?","threshold":0.5'),
           SET.replace('"decide":"First?"', '"decide":"First?","on":""'),
           SET.replace('"second"','"first"'), SET.replace('"version":1','"version":1,"threshold":0.5')]
    with RankBackend() as backend:
        statements = [ranked(spec) for spec in bad] + [ranked(settings=setting) for setting in
            ['{"model":"private-model"}', '{"true":"secret"}', '{"threshold":0.5}']]
        statements += [ranked(records='{"a":"a","bad":" "}'),ranked(records='{"a":"a","a":"b"}')]
        got = run(statements + [ranked(records='{}'),
                                "SELECT count(*) FROM thinkthen_rank_set(NULL,'broken')",
                                "SELECT count(*) FROM thinkthen_rank_set('@missing',NULL)",
                                ranked(settings='{"deadline_ms":0}')], backend.base)
        for result in got[:len(statements)]:
            expect(said(result).startswith('thinkthen usage:'), True, 'Usage admission')
            expect('private-model' in said(result), False, 'model secrecy')
        expect([rows(result) for result in got[-4:-1]], [[],[[0]],[[0]]], 'empty and NULL')
        expect(said(got[-1]).startswith('thinkthen deadline:'), True, 'pre-send deadline')
        expect(backend.count(), 0, 'all admissions send zero')


@case
def set_backend_error_returns_no_rows_and_withholds_evidence():
    with Backend() as backend, ConditionalBackend(backend.base(), sentinel='private evidence') as refused:
        result = run(['SET thinkthen_max_retries=0', ranked(records='{"a":"private evidence"}', settings='{"batch":"max"}')], refused.base)
        expect(said(result[1]).startswith('thinkthen backend:'), True, 'Backend error')
        expect('private evidence' in said(result[1]), False, 'evidence secrecy')
        expect(refused.count(), 1, 'one failed packed call, no rows or NULL')


@case
def set_ties_keep_keyed_member_order_and_public_types():
    with Backend() as backend:
        query = ranked(records='{"z":"one","a":"two","m":"three"}', settings='{"batch":"max"}')
        got = run([query, 'SELECT typeof(key),typeof(rank),typeof(probability),typeof(question_name),typeof(facts) FROM (' + query + ') LIMIT 1'], backend.base())
        expect([row[:4] for row in rows(got[0])], [['z',1,.9,'first'],['a',2,.9,'first'],['m',3,.9,'first']], 'stable set ties')
        expect(rows(got[1]), [['VARCHAR','BIGINT','DOUBLE','VARCHAR','VARCHAR']], 'public SQL types')
        expect(backend.count(), 1, 'one shared packed call')


if __name__ == '__main__':
    raise SystemExit(main())
