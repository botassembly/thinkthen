"""Run the installed DuckDB extension through ordinary SQL and native interrupt."""
import json
from pathlib import Path
import subprocess
import sys
from contextlib import contextmanager

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from testing import child_environment
from cases import quote, invocation, rows, scrub, expected

root = Path(__file__).resolve().parents[3]
package, loopback = map(Path, sys.argv[1:])
runner, extension = package / 'sql', package / 'thinkthen.duckdb_extension'
env = child_environment()
decision_case = json.loads((root / 'construction/stub.json').read_text())[0]
decision_rows = dict(zip(decision_case['request']['records'], decision_case['expected']))


@contextmanager
def connection(environment=env):
    with subprocess.Popen([str(runner), str(extension)], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, env=environment) as process:
        try:
            yield process
            process.stdin.close()
            assert process.wait(timeout=30) == 0
        finally:
            if process.poll() is None:
                process.kill(); process.wait()


def sql(process, query):
    process.stdin.write(json.dumps(query) + '\n'); process.stdin.flush()
    line = process.stdout.readline()
    assert line, f'consumer stopped while running {query}'
    return json.loads(line)


def success(process, query):
    result = sql(process, query)
    assert 'error' not in result, result
    return result['rows']


@contextmanager
def fixture(*arguments):
    with subprocess.Popen([str(loopback), *map(str, arguments)], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, env=env) as server:
        try:
            url = f'http://127.0.0.1:{server.stdout.readline().strip()}/v1'
            yield server, dict(env, THINKTHEN_BASE_URL=url, THINKTHEN_API_KEY='loopback-test-key')
            server.stdin.write('shutdown\n'); server.stdin.flush()
            assert server.wait(timeout=30) == 0
        finally:
            if server.poll() is None:
                server.kill(); server.wait()


def command(server, value):
    server.stdin.write(value + '\n'); server.stdin.flush()
    return server.stdout.readline().strip()


with connection() as process:
    success(process, "SET thinkthen_settings = '{\"adapter\":\"stub\"}'")
    for case in json.loads((root / 'construction/stub.json').read_text()):
        actual = rows(success(process, 'SELECT * FROM ' + invocation(case['request'])))
        assert actual == expected(case), (case['request']['function'], actual, expected(case))
    # Session settings never leak into another connection of the same database.
    assert success(process, "2:SELECT current_setting('thinkthen_settings') AS setting") == [{'setting': '{}'}]
    assert 'session-only' in sql(process, "SET GLOBAL thinkthen_settings = '{}' ")['error']
    for setting in ('{"url":null}', '{"key_source":"OTHER_KEY"}', '{"url":"http://127.0.0.1","key_source":null}'):
        assert 'usage.settings' in sql(process, 'SET thinkthen_settings = ' + quote(setting))['error']
    for case in json.loads((root / 'conformance/public-contract/diagnostics.json').read_text()):
        request = {'function':'annotate','question':{'type':'inline','value':case['question']},'records':['alpha']}
        failure = sql(process, 'SELECT * FROM ' + invocation(request))['error']
        assert failure.endswith(case['code'] + ': ' + case['message']), failure
    # Failed replacement retains the preceding admitted session client.
    query = "SELECT * FROM thinkthen_decide((SELECT 'evidence' AS record), 'Ready?')"
    short = rows(success(process, query))
    assert rows(success(process, "SELECT * FROM thinkthen_decide((SELECT 'evidence' AS RECORD), 'Ready?')")) == short
    tagged = rows(success(process, "SELECT * FROM " + invocation({'function':'decide','question':{'type':'inline','value':'Ready?'},'records':['evidence']})))
    assert short == tagged
    success(process, "PREPARE bound_question AS SELECT * FROM thinkthen_decide((SELECT 'evidence' AS record), $1)")
    assert rows(success(process, "EXECUTE bound_question('Ready?')")) == short
    success(process, 'PREPARE decision AS ' + query)
    success(process, "SET thinkthen_settings = '{\"adapter\":\"stub\",\"batch\":1}'")
    assert rows(success(process, 'EXECUTE decision')) == short
    assert 'usage.question' in sql(process, "SELECT * FROM thinkthen_decide((SELECT 'x' AS record), '{bad')")['error']
    assert 'usage.question' in sql(process, "SELECT * FROM thinkthen_decide((SELECT 'x' AS record), 'Ready?', options := '')")['error']
    assert 'usage.question' in sql(process, "SELECT * FROM thinkthen_decide((SELECT 'x' AS record), 'Ready?', options := '{\"on\":\"/x\",\"on\":\"/y\"}')")['error']
    assert success(process, "SELECT count(*) AS n FROM thinkthen_decide((SELECT '' AS record WHERE false), 'Ready?')") == [{'n':'0'}]
    paired = success(process, "SELECT * FROM thinkthen_decide((SELECT CASE WHEN range % 7 = 0 THEN 'evidence 5' ELSE 'alpha' END AS record FROM range(8193) ORDER BY range), 'Ready?')")
    assert [int(row['position']) for row in paired] == list(range(1, 8194))
    for row, actual in zip(paired, rows(paired)):
        wanted = decision_rows['evidence 5' if (int(row['position']) - 1) % 7 == 0 else 'alpha']
        assert actual == {key: value for key, value in wanted.items() if key != 'id'}, row
    assert success(process, "SELECT count(*) AS n FROM thinkthen_decide((SELECT 'x' AS record), 'Ready?') a CROSS JOIN thinkthen_decide((SELECT 'y' AS record), 'Ready?') b") == [{'n':'1'}]
    for source in ("SELECT 'x' AS wrong", "SELECT 1 AS record", "SELECT 'x' AS record, 'y' AS extra"):
        assert 'one VARCHAR or JSON column named record' in sql(process, f"SELECT * FROM thinkthen_decide(({source}), 'Ready?')")['error']
    for function, question, options in (
        ('choose', {'type':'inline','value':'Which?'}, {'options':['a\0b','c\0d']}),
        ('annotate', {'type':'toolbox','value':{'questions':{'a\0b':{'decide':'Ready?'}}}}, {}),
    ):
        result = sql(process, 'SELECT * FROM ' + invocation({'function':function,'question':question,'options':options,'records':['evidence']}))
        assert 'usage.name' in result['error'], result
    for identifier in (18446744073709551615, {'key\0suffix': 'value'}):
        result = rows(success(process, 'SELECT * FROM ' + invocation({'function':'decide','question':{'type':'inline','value':'Ready?'},'records':[{'id':identifier,'body':'evidence'}]})))
        assert len(result) == 1 and result[0]['status'] in ('answered', 'unsure'), result
    # Rank drains every retained result after input ends, across output vectors.
    assert success(process, "SELECT min(position) AS first,max(position) AS last,count(*) AS n,count(DISTINCT position) AS unique FROM thinkthen_rank((SELECT range::VARCHAR AS record FROM range(4097)), 'Ready?')") == [{'first':'1','last':'4097','n':'4097','unique':'4097'}]

probability = {record: value['p'] for record, value in decision_rows.items()}
parallel_query = f"""SELECT count(*) AS n, min(position) AS first, max(position) AS last,
    count(DISTINCT position) AS unique,
    count(*) FILTER (WHERE p <> CASE WHEN (position - 1) % 7 = 0 THEN {probability['evidence 5']} ELSE {probability['alpha']} END
        OR answer::BOOLEAN <> ((position - 1) % 7 = 0) OR status <> 'answered') AS wrong
    FROM thinkthen_decide((SELECT CASE WHEN record_thread(i) % 7 = 0 THEN 'evidence 5' ELSE 'alpha' END AS record
                          FROM evidence ORDER BY i), 'Ready?')"""
parallel = subprocess.run([str(runner), str(extension), '--parallel', parallel_query], env=env,
                          text=True, stdout=subprocess.PIPE, check=True)
assert json.loads(parallel.stdout)['rows'] == [{'n':'500000','first':'1','last':'500000','unique':'500000','wrong':'0'}]
subprocess.run([str(runner), str(extension), '--abandon'], env=env, check=True, timeout=120)

with connection() as process:
    success(process, "SET thinkthen_settings = '{\"adapter\":\"stub\"}'")
    for column_type in ('VARCHAR', 'JSON'):
        for null_at in (0, 4096):
            result = sql(process, f"SELECT * FROM thinkthen_decide((SELECT CASE WHEN range = {null_at} THEN NULL ELSE 'null' END::{column_type} AS record FROM range(4097) ORDER BY range), 'Ready?')")
            assert 'SQL NULL' in result.get('error', ''), (column_type, null_at, result.get('error'), len(result.get('rows', [])))
            assert success(process, 'SELECT 1 AS alive') == [{'alive': '1'}]
    assert len(success(process, "SELECT * FROM thinkthen_decide((SELECT 'null'::JSON AS record), 'Ready?')")) == 1

with fixture(root / 'conformance/decide-yes.json') as (server, environment):
    cold = subprocess.run([str(runner), str(extension), '--cold'], env=environment,
                          text=True, stdout=subprocess.PIPE, check=True)
    actual = rows(json.loads(cold.stdout)['rows'])
    wanted = expected(json.loads((root / 'conformance/decide-yes.json').read_text()))
    assert actual == wanted * 2, actual
    assert int(command(server, 'count')) == 2

controls = 0
for path in sorted((root / 'conformance').glob('*.json')):
    case = json.loads(path.read_text())
    if case.get('cancelled'):
        assert path.name == 'cancel-before-call.json'
        controls += 1
        continue  # Native interruption is proved against a held real request below.
    with fixture(path) as (server, environment), connection(environment) as process:
        result = sql(process, 'SET thinkthen_settings = ' + quote(json.dumps(case['client'])))
        if 'error' not in result:
            result = sql(process, 'SELECT * FROM ' + invocation(case['request']))
        if isinstance(case['expected'], dict):
            assert case['expected']['error'] in result.get('error', ''), (path.name, result)
        else:
            assert 'error' not in result, (path.name, result)
            assert scrub(rows(result['rows'])) == scrub(expected(case)), (path.name, result, expected(case))
        assert int(command(server, 'count')) == case['requests'], path.name
assert controls == 1

with fixture('--unavailable') as (_, environment), connection(environment) as process:
    success(process, "SET thinkthen_settings = '{\"adapter\":\"stub\"}'")
    success(process, "PREPARE current_client AS SELECT * FROM thinkthen_decide((SELECT 'evidence' AS record), 'Ready?')")
    assert rows(success(process, 'EXECUTE current_client'))
    success(process, "SET thinkthen_settings = '{}'")
    result = sql(process, 'EXECUTE current_client')
    assert 'backend.transport:' in result.get('error', ''), result

for mode in ('headers', 'body', 'refusal-body'):
    with fixture('--held', mode) as (server, environment):
        with subprocess.Popen([str(runner), str(extension), 'cancel'], stdin=subprocess.PIPE,
                              stdout=subprocess.PIPE, text=True, env=environment) as child:
            try:
                assert command(server, 'arrived') == 'arrived'
                child.stdin.write('c'); child.stdin.flush()
                assert command(server, 'closed') == 'closed'
                result = json.loads(child.stdout.readline())
                assert 'interrupt' in result.get('error', '').lower(), result
                assert child.wait(timeout=30) == 0
                assert command(server, 'count') == '1'
            finally:
                if child.poll() is None:
                    child.kill(); child.wait()
print('DuckDB: all ten construction functions, 20 conformance cases, one native cancellation control and lifecycle checks pass')
