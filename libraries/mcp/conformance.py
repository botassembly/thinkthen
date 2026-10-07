"""Actual named MCP calls over the existing shared loopback case arms.

This WIP consumer qualifies only the behavior cells it actually checks. Missing
complete execution is a failing process; unimplemented parity cells remain
missing in the existing parity runner. Protocol fixtures never emit parity.
"""
import json
import math
from pathlib import Path
import re
import sys
import tempfile
from client import Client, ProtocolError, ToolError

ROOT = Path(__file__).resolve().parents[2]
DOCUMENT = json.loads((ROOT / 'conformance/cases.json').read_text())
# These cases need the same already-described atomic/set call shapes. More
# negatives, ties, empty outputs and numeric distributions use existing arms.
CASES = ('01-decide-yes-captured', '02-decide-no', '03-decide-band-unsure',
         '04-decide-band-yes', '05-decide-meanings', '06-choose-billing',
         '07-choose-unsure', '08-choose-tie', '09-tag-two', '10-tag-none',
         '11-score-middle', '12-score-upper', '13-filter-records', '14-filter-none',
         '15-rank-records', '16-rank-stable-tie', '18-find-second', '19-find-none',
         '17-annotate-mixed', '32-score-equal-distribution', '33-tag-threshold-excludes',
         '37-annotate-choose-one', '38-annotate-score-one',
         '41-offsets-past-an-accent-and-an-emoji', '51-same-kind-alerts')


def same(actual, expected):
    if type(actual) is bool or type(expected) is bool:
        assert actual is expected, 'Boolean value changed'
    elif isinstance(actual, (int, float)) and isinstance(expected, (int, float)):
        assert math.isclose(actual, expected, abs_tol=1e-9), 'numeric value changed'
    elif isinstance(expected, list):
        assert isinstance(actual, list) and len(actual) == len(expected), 'row count changed'
        for one, other in zip(actual, expected):
            same(one, other)
    elif isinstance(expected, dict):
        assert isinstance(actual, dict), 'object type changed'
        for name, value in expected.items():
            assert name in actual, 'required result field missing'
            same(actual[name], value)
    else:
        assert type(actual) is type(expected) and actual == expected, 'value or type changed'


def complete(result):
    assert result['schema'] == 'thinkthen.result/2', 'complete result/2 unavailable'
    assert re.fullmatch('[0-9a-f]{64}', result['answer_id']), 'native answer identity missing'
    meta = result['meta']
    assert meta['origin'] == 'live', 'loopback result has incorrect provenance'
    assert meta['cached'] is False, 'loopback response was claimed as cached'
    assert len(meta['observations']) == len(meta['requests']) == len(meta['question_sources'])
    assert meta['requests_sent'] > 0, 'live attempt observations missing'
    return result


def arguments(case):
    question = case.get('question_set', case.get('question'))
    verb = case['verb']
    if verb == 'find':
        return {'question': question['find'], 'records': question['units'],
                'options': {'none': question['none']}}
    if verb == 'rank':
        return {'question': question['decide'],
                'records': [row['evidence'] for row in case['exchanges']], 'options': {'batch': 1}}
    if verb == 'relate':
        return {'question': question, 'records': case['entities']}
    if verb == 'filter':
        return {'question': question, 'records': [row['evidence'] for row in case['exchanges']],
                'options': {'batch': 1}}
    return {'question': question, 'evidence': case.get('text', case['exchanges'][0]['evidence'])}


def check(case, reply):
    assert set(reply) >= {'value', 'facts'}, 'native call serializer unavailable'
    assert reply['facts']['requests_sent'] == len(case['exchanges']), 'attempt count changed'
    expected = case['expect']['success']
    result = reply['value']
    verb = case['verb']
    if verb in ('filter', 'rank'):
        operation = expected['operation']
        indexes = operation.get('indexes', [row['index'] for row in operation.get('ranking', [])])
        assert len(result) == len(indexes), 'output row count changed'
        for place, (row, index) in enumerate(zip(result, indexes), 1):
            complete(row)
            same(row['input'], case['exchanges'][index]['evidence'])
            same(row['value'], place if verb == 'rank' else True)
            same(row['answer'], expected['answers'][index]['details']['answer'])
        return
    result = complete(result)
    if verb == 'annotate':
        for wanted in expected['answers']:
            name = wanted['name']
            same(result['value'][name], wanted['bare'])
            same(result['answers'][name]['answer'], wanted['details']['answer'])
        return
    if verb == 'find':
        selected = expected['operation']['selected']
        same(result['value'], None if selected is None else case['question']['units'][selected])
        same(result['answer']['probabilities'], expected['answers'][0]['details']['answer']['probabilities'])
        return
    wanted = expected['answers'][0]
    same(result['value'], wanted['bare'])
    if 'answer' in wanted['details']:
        same(result['answer'], wanted['details']['answer'])


def run(port, binary):
    assert port.isdecimal() and 0 < int(port) <= 65535, 'loopback port required'
    by_id = {case['id']: case for case in DOCUMENT['cases']}
    failed = False
    for ident in CASES:
        case = by_id[ident]
        try:
            with tempfile.TemporaryDirectory(prefix='thinkthen-mcp-consumer-') as folder:
                env = {'PATH': '/usr/bin:/bin', 'LANG': 'C.UTF-8', 'HOME': folder,
                       'XDG_CONFIG_HOME': folder, 'XDG_CACHE_HOME': folder, 'XDG_STATE_HOME': folder,
                       'THINKTHEN_API_KEY': 'sk-mcp-loopback-only'}
                command = (binary, 'mcp', '--url', f'http://127.0.0.1:{port}/case/{ident}/v1',
                           '--no-cache', '--max-retries', '0')
                with Client.launch(command, env=env) as client:
                    tools = client.tools()
                    assert [tool['name'] for tool in tools] == DOCUMENT['parity']['functions']
                    assert all(isinstance(tool['inputSchema'], dict) and isinstance(tool['outputSchema'], dict)
                               for tool in tools), 'tool schemas missing'
                    reply = getattr(client, case['verb'])(**arguments(case))
                    check(case, reply)
            status = 'pass'
        except (AssertionError, KeyError, OSError, ProtocolError, ToolError):
            status = 'fail'
            failed = True
        print('parity: ' + json.dumps({'consumer': 'mcp', 'case': ident,
                                      'checks': ['named', 'runtime'], 'status': status}), flush=True)
    return int(failed)


if __name__ == '__main__':
    if len(sys.argv) != 3:
        raise SystemExit('usage: conformance.py LOOPBACK_PORT ABSOLUTE_THINKTHEN_BINARY')
    raise SystemExit(run(*sys.argv[1:]))
