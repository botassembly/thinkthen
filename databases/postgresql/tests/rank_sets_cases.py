"""Public PostgreSQL set rank, keys, saved criteria and strict member replay."""
import json
import pathlib
import sys

from find_cases import quoted, sql

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[2] / 'sqlite/tests'))
from rank_backend import EXPECTED, ONE, RECORDS, SET


def ranked(spec=SET, records=RECORDS, settings='{"batch":1}', limit=''):
    return ('SELECT coalesce(json_agg(row_to_json(r) ORDER BY rank),\'[]\'::json) FROM '
            '(SELECT * FROM thinkthen_rank_set(' + quoted(spec) + ',' + quoted(records) + '::jsonb,'
            + quoted(settings) + '::json)' + limit + ') r')


def run(socket, *statements):
    out, error = sql(socket, "SET thinkthen.model='judge-rank'", *statements)
    assert not error, error
    return [json.loads(line) for line in out.splitlines()]


def values(rows):
    return [[row['key'],row['rank'],row['probability'],row['question_name']] for row in rows]


def verify(socket, folder):
    # Authored keyed order differs from PostgreSQL's bytewise-key tie order.
    keyed = '{"d":"a","c":"c","b":"b","a":"a"}'
    result, top, one = run(socket, ranked(records=keyed), ranked(limit=' LIMIT 3'), ranked(ONE))
    assert values(result) == EXPECTED, result
    assert values(top) == EXPECTED[:3], top
    assert [row[:3] for row in values(one)] == [['a',1,.7],['d',2,.7],['b',3,.6],['c',4,.5]], one
    facts = [row['facts'] for row in result]
    assert all(fact == facts[0] for fact in facts), facts
    assert (facts[0]['records'],facts[0]['requests_sent']) == (4,3), facts
    reversed_set = '{"version":1,"questions":{"renamed":{"decide":"Second?"},"last":{"decide":"First?"}}}'
    reverse = run(socket, ranked(reversed_set))[0]
    assert values(reverse) == [['a',1,1.0,'renamed'],['d',2,1.0,'renamed'],['c',3,.99,'renamed'],['b',4,.6,'last']], reverse
    described = SET.replace('"decide":"First?"', '"decide":"First?","true":"supports first","false":"opposes first"')
    path = pathlib.Path(folder) / 'rank-set.json'
    path.write_text(described)
    # Explicit privileged question loader preserves authored member order.
    statement = ranked(described, settings='{"batch":1,"context":"shared note"}')
    statement = statement.replace(quoted(described), 'thinkthen_question_file(' + quoted(str(path)) + ')', 1)
    rows = run(socket, statement)[0]
    assert values(rows) == EXPECTED, rows
    out, error = sql(socket, "SELECT has_function_privilege('public','thinkthen_rank_set(text,jsonb,json)','EXECUTE')")
    assert not error and out == 'f', (out,error)


def invalid(socket):
    bad = ['{"version":1,"questions":{}}', SET.replace('"decide":"Second?"','"score":"Second?","levels":["low","high"]'),
           SET.replace('"decide":"First?"','"decide":"First?","threshold":0.5'),
           SET.replace('"decide":"First?"','"decide":"First?","on":""'),
           SET.replace('"second"','"first"'), SET.replace('"version":1','"version":1,"threshold":0.5')]
    statements = [ranked(spec) for spec in bad] + [ranked(settings=setting) for setting in
        ['{"model":"private-model"}', '{"true":"secret"}', '{"threshold":0.5}']]
    statements += [ranked(records='{"a":"a","bad":" "}'), ranked(records='{"a":4}')]
    for statement in statements:
        out, error = sql(socket, statement)
        assert '22023' in error and 'thinkthen usage:' in error and not out, (out,error)
        assert 'private-model' not in error and 'secret' not in error, error
    out, error = sql(socket, ranked(settings='{"deadline_ms":0}'))
    assert not out and '57014' in error and 'thinkthen deadline:' in error, (out,error)
    out, error = sql(socket, ranked(records='{}'), ranked().replace(quoted(RECORDS)+'::jsonb','NULL::jsonb'))
    assert not error and out.splitlines() == ['[]','[]'], (out,error)


def members(socket, folder):
    rows = run(socket, 'SET thinkthen.record=' + quoted(folder), *['SELECT json_agg(row_to_json(r) ORDER BY rank) FROM thinkthen_rank('
                        + quoted(question) + ',\'{"a":"a","b":"b","c":"c"}\'::jsonb,\'{"batch":1}\'::json) r'
                        for question in ['First?','Second?']])
    assert [[row['key'] for row in order] for order in rows] == [['a','b','c'],['a','c','b']], rows


def replay(socket, folder):
    controls = ['SET thinkthen.replay=' + quoted(folder), 'SET thinkthen.max_requests_total=0']
    result = run(socket, *controls, ranked())[0]
    assert values(result) == EXPECTED, result
    facts = result[0]['facts']
    assert (facts['records'],facts['requests_sent'],facts['cache_answers']) == (4,0,0), facts
    one = run(socket, *controls, ranked(ONE))[0]
    assert [row[:3] for row in values(one)] == [['a',1,.7],['d',2,.7],['b',3,.6],['c',4,.5]], one
    renamed = '{"version":1,"questions":{"renamed":{"decide":"Second?"},"last":{"decide":"First?"}}}'
    reverse = run(socket, *controls, ranked(renamed))[0]
    assert values(reverse) == [['a',1,1.0,'renamed'],['d',2,1.0,'renamed'],['c',3,.99,'renamed'],['b',4,.6,'last']], reverse
    out, error = sql(socket, "SET thinkthen.model='judge-rank'", *controls, ranked(SET.replace('Second?','Missing private question?')))
    assert not out and '58030' in error and 'thinkthen local:' in error, (out,error)
    assert 'Missing private question?' not in error, error


def bodies(path):
    bodies = json.loads(pathlib.Path(path).read_text())
    assert len(bodies) == 6, len(bodies)
    questions = [question for body in bodies for question in body['questions'].values() if 'criteria' in question]
    assert questions[0]['criteria'] == {'true':'supports first','false':'opposes first'}, questions
    assert all(body['model'] == 'judge-rank' for body in bodies), bodies
    assert all(json.dumps(body).count('shared note') == 1 for body in bodies[3:]), bodies


def failure(socket):
    out, error = sql(socket, 'SET thinkthen.max_retries=0', ranked(records='{"a":"private evidence"}', settings='{"batch":"max"}'))
    assert not out and '38000' in error and 'thinkthen backend:' in error, (out,error)
    assert 'private evidence' not in error.split('CONTEXT:')[0], error


if __name__ == '__main__':
    mode, path, *rest = sys.argv[1:]
    if mode in ('verify', 'members', 'replay'):
        globals()[mode](path, rest[0])
    else:
        globals()[mode](path)
