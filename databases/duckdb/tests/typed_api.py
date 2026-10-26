"""Exercise typed SQL through the installed DuckDB host, without live services."""
import argparse
from contextlib import contextmanager
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'databases'))
from testing import child_environment
from cases import invocation
from admission_cases import cases as admission_cases, diagnostic


def literal(value):
    return "'" + value.replace("'", "''") + "'"


def contract_row(row, function):
    common = {'position', 'id', 'status', 'probability', 'error_code', 'error_message',
              'error_http_status', 'source', 'detail'}
    answers = ({'rank'} if function == 'rank' else
               {'found', 'found_text', 'found_start', 'found_end'} if function == 'find' else {'answer'})
    assert common <= row.keys(), row
    assert row.keys() <= common | answers | {'record'} | ({'kept'} if function == 'filter' else set()), row
    result = {'function': function, 'position': row['position'], 'status': row['status'],
              'source': row['source']}
    for key in ('id', 'probability', 'detail'):
        if row.get(key) is not None:
            result[key] = row[key]
    if row['status'] == 'failed':
        result['error'] = {'code': row['error_code'], 'message': row['error_message']}
        if row['error_http_status'] is not None:
            result['error']['http_status'] = row['error_http_status']
    elif row['status'] == 'answered':
        result['answer'] = row.get('rank') if function == 'rank' else row.get('found') if function == 'find' else row['answer']
    return result


class Host:
    def __init__(self, package, environment):
        self.process = subprocess.Popen([str(package / 'sql'), str(package / 'thinkthen.duckdb_extension')],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, env=environment)

    def query(self, statement):
        self.process.stdin.write(json.dumps(statement) + '\n')
        self.process.stdin.flush()
        line = self.process.stdout.readline()
        assert line, statement
        return json.loads(line)

    def ok(self, statement):
        result = self.query(statement)
        assert 'error' not in result, result
        return result['rows']

    def failure(self, statement):
        result = self.query(statement)
        assert 'error' in result, result
        return result['error']

    def __enter__(self):
        return self

    def __exit__(self, *failure):
        self.process.stdin.close()
        try:
            assert self.process.wait(timeout=30) == 0
        finally:
            if self.process.poll() is None:
                self.process.kill()
                self.process.wait()


@contextmanager
def fixture(options, path=None, held=False):
    arguments = ['--held', 'headers'] if held else [str(path)]
    with subprocess.Popen([str(options.loopback), *arguments], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, env=child_environment()) as server:
        try:
            port = int(server.stdout.readline())
            backend = 'systemone' if held else json.loads(path.read_text()).get('client', {}).get('backend', 'systemone')
            variable = options.systemone_url_env if backend == 'systemone' else options.openai_url_env
            environment = dict(child_environment(), **{variable: f'http://127.0.0.1:{port}/v1'},
                               THINKTHEN_API_KEY='loopback-test-key', OPENAI_API_KEY='loopback-test-key')
            yield server, environment
        finally:
            server.stdin.write('shutdown\n')
            server.stdin.flush()
            try:
                server.wait(timeout=30)
            finally:
                if server.poll() is None:
                    server.kill()
                    server.wait()


def command(server, value):
    server.stdin.write(value + '\n')
    server.stdin.flush()
    return server.stdout.readline().strip()


def abi_mismatch(options):
    # Feature: the loader refuses an incompatible engine before opening a client.
    result = subprocess.run([str(options.package / 'sql'), str(options.abi_mismatch_extension)],
        input='', text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=child_environment())
    assert result.returncode != 0, result.stdout
    assert 'usage.abi' in result.stdout + result.stderr, (result.stdout, result.stderr)


def conformance(options):
    # Feature: SQL bindings preserve every function's typed contract on both wires.
    covered = set()
    with Host(options.package, child_environment()) as host:
        host.ok("SET thinkthen_backend = 'stub'")
        construction = json.loads((ROOT / 'construction/stub.json').read_text())
        for case in construction:
            function = case['request']['question']['function']
            rows = host.ok('SELECT to_json(t) AS row FROM ' + invocation(case['request']) + ' t')
            assert [contract_row(json.loads(row['row']), function) for row in rows] == case['expected'], function
    paths = sorted((ROOT / 'conformance').rglob('*.json'))
    for path in paths:
        case = json.loads(path.read_text())
        if 'request' not in case or not isinstance(case.get('expected'), list):
            continue  # Invalid calls and pre-call cancel have separate native cases.
        function = case['request']['question']['function']
        backend = case.get('client', {}).get('backend', 'systemone')
        assert backend in ('systemone', 'openai', 'stub'), path
        with fixture(options, path) as (server, environment), Host(options.package, environment) as host:
            for key, value in case.get('client', {}).items():
                entries = value.items() if key == 'retry' else [(key, value)]
                for setting, item in entries:
                    host.ok('SET thinkthen_' + setting + ' = ' + (literal(item) if isinstance(item, str) else str(item)))
            rows = host.ok('SELECT to_json(t) AS row FROM ' + invocation(case['request']) + ' t')
            actual = [contract_row(json.loads(row['row']), function) for row in rows]
            assert actual == case['expected'], (path, actual, case['expected'])
            assert int(command(server, 'count')) == case['requests'], path
            if function == 'decide':
                missing = host.ok("SELECT position,status,answer,error_code FROM thinkthen_decide((SELECT NULL::VARCHAR AS record),'Ready?')")
                assert missing == [{'position': '1', 'status': 'failed', 'answer': None, 'error_code': 'record.missing'}], missing
                assert int(command(server, 'count')) == case['requests'], path
        if backend != 'stub':
            covered.add((function, backend))
    functions = {'decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate'}
    assert covered == {(function, backend) for function in functions for backend in ('systemone', 'openai')}, covered


def admission(options):
    # The installed SQL host reports complete library diagnostics.
    with Host(options.package, child_environment()) as host:
        host.ok("SET thinkthen_backend = 'stub'")
        # DuckDB's JSON error envelope separates the message from SQL locations.
        host.ok('SET errors_as_json = true')
        for case, request, expected in admission_cases(ROOT, 'duckdb'):
            result = host.query('SELECT * FROM ' + invocation(request))
            assert 'error' in result, (case['name'], result)
            assert 'secret-placeholder' not in json.dumps(result), case['name']
            envelope = json.loads(result['error'])
            assert envelope['exception_type'] == 'Invalid Input', envelope
            code, separator, message = envelope['exception_message'].partition(': ')
            assert separator, result
            diagnostic(case, code, message, expected)


def native(options):
    # Feature: DuckDB preserves native input values and rejects bound name collisions.
    with Host(options.package, child_environment()) as host:
        host.ok("SET thinkthen_backend = 'stub'")
        rows = host.ok("""SELECT document_id, position, status, answer, error_code,
            typeof(answer) AS answer_type, typeof(probability) AS probability_type
            FROM thinkthen_decide((SELECT * FROM (VALUES
              (81, 'alpha'), (26, NULL), (47, 'beta')) src(document_id,record)), '{literal?')""")
        assert [(row['document_id'], row['position']) for row in rows] == [('81', '1'), ('26', '2'), ('47', '3')]
        assert rows[1]['status'] == 'failed' and rows[1]['answer'] is None and rows[1]['error_code'] == 'record.missing'
        assert {row['answer_type'] for row in rows} == {'BOOLEAN'}
        assert {row['probability_type'] for row in rows} == {'DOUBLE'}
        for column, function in [('Answer', 'decide'), ('POSITION', 'decide'), ('id', 'decide'), ('KePt', 'filter')]:
            error = host.failure(f"SELECT * FROM thinkthen_{function}((SELECT 'x' AS record, 5 AS \"{column}\"), 'Ready?')")
            assert column.lower() in error.lower(), error
        ranked = host.ok("""SELECT document_id, position, rank FROM thinkthen_rank(
            (SELECT * FROM (VALUES (81,'alpha'),(26,NULL),(47,'beta')) s(document_id,record)),
            'Priority?', levels := ['low','high'], top := 1)""")
        assert len(ranked) == 2 and ranked[-1] == {'document_id': '26', 'position': '2', 'rank': None}, ranked
        for row in ranked:
            assert row['document_id'] == {'1': '81', '2': '26', '3': '47'}[row['position']], row
        filtered = host.ok("""SELECT position, status, kept FROM thinkthen_filter(
            (SELECT * FROM (VALUES ('alpha'),(NULL),('beta')) s(record)), 'Ready?')""")
        assert len(filtered) == 3 and filtered[1]['kept'] == 'false', filtered
        assert host.ok("SELECT current_setting('thinkthen_backend') AS backend") == [{'backend': 'stub'}]
        assert host.ok("2:SELECT current_setting('thinkthen_backend') AS backend") == [{'backend': 'systemone'}]
        for setting, value in [('api_key', "'do-not-store'"), ('url', "'https://invalid.example'"), ('key_source', "'OTHER_KEY'"), ('mode', "'proxy'"), ('settings', "'{}'"), ('batch', '1.5')]:
            host.failure(f'SET thinkthen_{setting} = {value}')
        for arguments in ["choices := ['one']", "choices := ['a','b'], low := 0.2", "choices := ['a','b'], threshold := 0.8, low := 0.2, high := 0.7"]:
            host.failure("SELECT * FROM thinkthen_choose((SELECT 'x' AS record), 'Which?', " + arguments + ')')
        assert host.ok("SELECT 1 AS alive") == [{'alive': '1'}]


def cancellation(options):
    # Feature: database interruption closes a held request through the installed extension.
    with fixture(options, held=True) as (server, environment):
        with subprocess.Popen([str(options.package / 'sql'), str(options.package / 'thinkthen.duckdb_extension'), 'cancel'],
                              stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, env=environment) as process:
            try:
                assert command(server, 'arrived') == 'arrived'
                process.stdin.write('c')
                process.stdin.flush()
                assert command(server, 'closed') == 'closed'
                result = json.loads(process.stdout.readline())
                assert 'interrupt' in result.get('error', '').lower(), result
                assert process.wait(timeout=30) == 0
            finally:
                if process.poll() is None:
                    process.kill()
                    process.wait()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('package', type=Path)
    parser.add_argument('loopback', type=Path)
    parser.add_argument('--admission-only', action='store_true')
    parser.add_argument('--abi-mismatch-extension', type=Path, required=True)
    parser.add_argument('--systemone-url-env', required=True)
    parser.add_argument('--openai-url-env', required=True,
                        help='Process-owned loopback routing variable approved by the engine contract')
    options = parser.parse_args()
    admission(options)
    if options.admission_only:
        sys.exit(0)
    abi_mismatch(options)
    native(options)
    conformance(options)
    cancellation(options)
