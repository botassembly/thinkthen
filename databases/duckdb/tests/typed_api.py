"""Exercise typed SQL through the installed DuckDB host, without live services."""
import argparse
from contextlib import contextmanager
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'databases'))
from testing import child_environment, read_line
from cases import invocation
from admission_cases import cases as admission_cases, diagnostic
from migration_cases import validate_source


def literal(value):
    return "'" + value.replace("'", "''") + "'"


def contract_row(row, function):
    common = {'position', 'id', 'status', 'probability', 'error_code', 'error_message',
              'error_http_status', 'source', 'detail'}
    answers = ({'rank'} if function == 'rank' else
               {'found', 'found_text', 'found_start', 'found_end'} if function == 'find' else {'answer'})
    assert common <= row.keys(), row
    assert row.keys() <= common | answers | {'record'} | ({'kept'} if function == 'filter' else set()), row
    validate_source(row['source'])
    if function == 'filter':
        assert type(row['kept']) is bool, row
        assert row['kept'] is (row['status'] == 'answered' and row['answer'] is True), row
    if function == 'annotate' and row['answer'] is not None:
        for child in row['answer'].values():
            assert child.keys() <= {'function', 'status', 'answer', 'probability', 'error', 'detail'}, child
    result = {'function': function, 'position': row['position'], 'status': row['status'],
              'source': row['source']}
    for key in ('id', 'probability', 'detail'):
        if row.get(key) is not None:
            result[key] = row[key]
    if row['status'] != 'answered':
        answer = row.get('rank') if function == 'rank' else row.get('found') if function == 'find' else row['answer']
        assert answer is None, row
    if row['status'] != 'failed':
        assert all(row[key] is None for key in ('error_code', 'error_message', 'error_http_status')), row
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
        line = read_line(self.process.stdout)
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
            port = int(read_line(server.stdout))
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


def abi_mismatch(options):
    # Feature: the loader refuses an incompatible engine before opening a client.
    result = subprocess.run([str(options.package / 'sql'), str(options.abi_mismatch_extension)],
        input='', text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=child_environment(), timeout=30)
    assert result.returncode != 0, result.stdout
    assert 'usage.abi' in result.stdout + result.stderr, (result.stdout, result.stderr)


def admission(options):
    # The installed SQL host reports complete library diagnostics.
    with Host(options.package, child_environment()) as host:
        host.ok("SET thinkthen_backend = 'stub'")
        # DuckDB's JSON error envelope separates the message from SQL locations.
        host.ok('SET errors_as_json = true')
        host.failure("SET thinkthen_max_retries = 'secret-placeholder'")
        for statement in (
            "SELECT * FROM thinkthen_decide((SELECT 'evidence' AS record), NULL)",
            "SELECT * FROM thinkthen_choose((SELECT 'evidence' AS record), 'Which?', choices := ['one',NULL,'two'])",
        ):
            envelope = json.loads(host.failure(statement))
            assert envelope['exception_message'].startswith('usage.question: '), envelope
        failed = host.ok("SELECT to_json(t) AS result FROM thinkthen_filter((SELECT NULL::VARCHAR AS record), 'Ready?') t")
        row = json.loads(failed[0]['result'])
        contract_row(row, 'filter')
        assert row['status'] == 'failed' and row['error_code'] == 'record.missing' and row['kept'] is False, row
        for case, request, expected in admission_cases(ROOT, 'duckdb'):
            result = host.query('SELECT * FROM ' + invocation(request))
            assert 'error' in result, (case['name'], result)
            assert 'secret-placeholder' not in json.dumps(result), case['name']
            envelope = json.loads(result['error'])
            assert envelope['exception_type'] == 'Invalid Input', envelope
            code, separator, message = envelope['exception_message'].partition(': ')
            assert separator, result
            diagnostic(case, code, message, expected)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('package', type=Path)
    parser.add_argument('--abi-mismatch-extension', type=Path, required=True)
    options = parser.parse_args()
    abi_mismatch(options)
