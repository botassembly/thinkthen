"""Public SQL set rank: independently saved turns, replay and admission."""
import json
import pathlib
import tempfile

from helper import Backend, child, environment, expect, main
from conditional_backend import ConditionalBackend
from rank_backend import EXPECTED, ONE, RECORDS, SET, RankBackend

QUERY = 'SELECT key, rank, probability, question_name, facts FROM thinkthen_rank_set(?, ?, ?)'


def invoke(backend, calls, **config):
    env = environment(None, THINKTHEN_BASE_URL=backend.base,
                      THINKTHEN_API_KEY='sk-sql-rank-loopback')
    if config.pop('keyless', False):
        env.pop('THINKTHEN_API_KEY')
    config = {'model':'judge-rank', **config}
    fields = ','.join(f'{name}=run(db,{sql!r},{args!r})' for name, sql, args in calls)
    return child('db=connect()\ndb.execute("SELECT thinkthen_configure(?)", (' + repr(json.dumps(config)) + ',))\nsay(' + fields + ')', env)


def test_turns_keep_duplicate_keys_names_facts_and_limit():
    with RankBackend() as backend:
        got = invoke(backend, [('rows', QUERY, (SET, RECORDS, '{"batch":1}')),
                               ('top', QUERY + ' LIMIT 3', (SET, RECORDS, '{"batch":1}'))])
        expect([row[:4] for row in got['rows']], EXPECTED, 'independent merged order')
        expect([row[:4] for row in got['top']], EXPECTED[:3], 'LIMIT after merging')
        facts = [json.loads(row[4]) for row in got['rows']]
        expect(all(fact == facts[0] for fact in facts), True, 'same combined call facts')
        expect(facts[0]['records'], 4, 'original records, not eight member judgments')
        expect(facts[0]['requests_sent'], backend.count(), 'same call sends')
        expect(backend.count(), 3, 'duplicate text keeps keys and reuses native answers')


def test_plain_and_one_member_have_equal_wire_and_per_member_zero_send_replay():
    with RankBackend() as backend, tempfile.TemporaryDirectory() as folder:
        got = invoke(backend, [('plain', 'SELECT key,rank,probability FROM thinkthen_rank(?,?,?)',
                                ('First?', RECORDS, '{"batch":1}'))])
        plain_bodies = list(backend.bodies)
        start = backend.count()
        one = invoke(backend, [('rows', QUERY, (ONE, RECORDS, '{"batch":1}'))])
        expect([row[:3] for row in one['rows']], got['plain'], 'one-member equivalence')
        expect(sorted(json.dumps(body) for body in backend.bodies[start:]),
               sorted(json.dumps(body) for body in plain_bodies), 'equal public wire bodies')
        # Recording each plain member, not a prior set, exercises native keys.
        invoke(backend, [('first', 'SELECT key FROM thinkthen_rank(?,?,?)', ('First?', RECORDS, '{"batch":1}')),
                         ('second', 'SELECT key FROM thinkthen_rank(?,?,?)', ('Second?', RECORDS, '{"batch":1}'))],
               record=folder)
        before = backend.count()
        replay = invoke(backend, [('rows', QUERY, (SET, RECORDS, '{"batch":1}'))],
                        replay=folder, keyless=True)
        expect([row[:4] for row in replay['rows']], EXPECTED, 'per-member strict replay')
        expect(backend.count(), before, 'replay sends zero')
        facts = json.loads(replay['rows'][0][4])
        expect((facts['records'], facts['requests_sent'], facts['cache_answers']), (4, 0, 0), 'replayed combined facts')
        reordered = '{"version":1,"questions":{"renamed":{"decide":"Second?"},"last":{"decide":"First?"}}}'
        renamed = invoke(backend, [('rows', QUERY, (reordered, RECORDS, '{"batch":1}'))], replay=folder, keyless=True)
        expect([row[:4] for row in renamed['rows']], [['a',1,1.0,'renamed'],['d',2,1.0,'renamed'],
                                                     ['c',3,.99,'renamed'],['b',4,.6,'last']], 'reorder and rename reuse member keys')
        missing = SET.replace('Second?', 'Missing private question?')
        refused = invoke(backend, [('rows', QUERY, (missing, RECORDS, '{"batch":1}'))],
                         replay=folder, keyless=True)
        expect('thinkthen local:' in refused['rows'], True, 'missing strict key is Local')
        expect('Missing private question?' in refused['rows'], False, 'question secrecy')
        expect(backend.count(), before, 'missing replay sends zero')


def test_set_descriptions_order_and_file_use_native_admission():
    described = SET.replace('"decide":"First?"', '"decide":"First?","true":"supports first","false":"opposes first"')
    reversed_set = '{"version":1,"questions":{"renamed":{"decide":"Second?"},"last":{"decide":"First?"}}}'
    with RankBackend() as backend, tempfile.TemporaryDirectory() as folder:
        path = pathlib.Path(folder) / 'rank.json'
        path.write_text(described)
        got = invoke(backend, [('described', QUERY, ('@' + str(path), RECORDS, '{"batch":1,"context":"shared note"}')),
                               ('reversed', QUERY, (reversed_set, RECORDS, '{"batch":1}'))])
        expect([row[:4] for row in got['described']], EXPECTED, 'described member rank')
        expect([row[:4] for row in got['reversed']], [['a',1,1.0,'renamed'],['d',2,1.0,'renamed'],
                                                    ['c',3,0.99,'renamed'],['b',4,0.6,'last']], 'member order and names')
        described_questions = [q for body in backend.bodies for q in body['questions'].values() if 'First?' in q['instructions'] and 'criteria' in q]
        expect(len(described_questions) > 0, True, 'described criteria reached the wire')
        expect(described_questions[0]['criteria'], {'true':'supports first','false':'opposes first'}, 'independent meanings')
        expect(backend.bodies[0]['model'], 'judge-rank', 'existing model selection')
        expect(json.dumps(backend.bodies[0]).count('shared note'), 1, 'one shared context')


def test_invalid_set_controls_keys_and_empty_rows_send_zero():
    bad = ['{"version":1,"questions":{}}', SET.replace('"decide":"Second?"', '"score":"Second?","levels":["low","high"]'),
           SET.replace('"decide":"First?"', '"decide":"First?","threshold":0.5'),
           SET.replace('"decide":"First?"', '"decide":"First?","on":""'),
           SET.replace('"second"', '"first"'), SET.replace('"version":1', '"version":1,"threshold":0.5')]
    with RankBackend() as backend:
        calls = [(f'bad{i}', QUERY, (spec, RECORDS, '{}')) for i, spec in enumerate(bad)]
        calls += [(f'control{i}', QUERY, (SET, RECORDS, controls)) for i, controls in enumerate(
            ['{"model":"private-model"}', '{"true":"secret"}', '{"threshold":0.5}'])]
        calls += [('blank', QUERY, (SET, '{"a":"a","bad":" "}', '{}')),
                  ('duplicate', QUERY, (SET, '{"a":"a","a":"b"}', '{}')),
                  ('empty', QUERY, (SET, '{}', '{}')),
                  ('null', QUERY, (None, RECORDS, '{}')),
                  ('deadline', QUERY, (SET, RECORDS, '{"deadline_ms":0}'))]
        got = invoke(backend, calls)
        for name, _, _ in calls:
            if name == 'empty':
                expect(got[name], [], 'empty keyed input')
            else:
                expect(got[name].startswith('thinkthen deadline:' if name == 'deadline' else 'thinkthen usage:'), True, name)
                expect('private-model' in got[name], False, 'model secrecy')
        expect(backend.count(), 0, 'all refusals send zero')


def test_set_cancel_stops_its_shared_worker_without_later_sends():
    from test_interrupt import interrupted, settled, stopped_fast
    backend = Backend()
    try:
        query = "SELECT key FROM thinkthen_rank_set('" + SET + "','" + RECORDS + "','{\"batch\":\"max\"}')"
        result = interrupted(backend, query, 1)
        stopped_fast(result)
        settled(backend, 1)
    finally:
        backend.close()


def test_set_ties_keep_host_input_order_and_public_types():
    backend = Backend()
    try:
        got = child('db=connect()\nsay(rows=run(db,' + repr(QUERY) + ','
                    + repr((SET, '{"z":"one","a":"two","m":"three"}', '{"batch":"max"}')) + '),'
                    + 'types=run(db,"SELECT typeof(key),typeof(rank),typeof(probability),typeof(question_name),typeof(facts) FROM thinkthen_rank_set(?,?,?) LIMIT 1",'
                    + repr((SET, '{"z":"one","a":"two","m":"three"}', '{"batch":"max"}')) + '))', environment(backend))
        expect([row[:4] for row in got['rows']], [['z',1,.9,'first'],['a',2,.9,'first'],['m',3,.9,'first']], 'stable set ties')
        expect(got['types'], [['text','integer','real','text','text']], 'public SQL types')
        expect(backend.count(), 1, 'one shared packed call')
    finally:
        backend.close()


def test_set_backend_failure_keeps_error_and_evidence_secret():
    backend = Backend()
    try:
        with ConditionalBackend(backend.base(), sentinel='private evidence') as refused:
            got = invoke(refused, [('rows', QUERY, (SET, '{"a":"private evidence"}', '{"batch":"max"}'))], max_retries=0)
            expect(got['rows'].startswith('thinkthen backend:'), True, 'error stays Backend')
            expect('private evidence' in got['rows'], False, 'failure withholds evidence')
            expect(refused.count(), 1, 'one failed request, no rows or fabricated NULL')
    finally:
        backend.close()


if __name__ == '__main__':
    raise SystemExit(main(globals()))
