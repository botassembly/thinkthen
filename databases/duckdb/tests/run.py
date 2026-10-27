"""Exercise the installed DuckDB SQL host with the canonical ABI."""
from contextlib import contextmanager
import json
from pathlib import Path
import subprocess
import sys
from types import SimpleNamespace

from argument_cases import argument_order
from cases import canonical, invocation, quote
from migration_cases import child_environment, command, fixture, forbidden_settings, run_cases
from typed_api import admission
from streaming_cases import first_response, streaming

argument_order()

PACKAGE, LOOPBACK = map(Path, sys.argv[1:3])
LIMIT_ONLY = sys.argv[3:] == ['--limit-only']
STREAMING_ONLY = sys.argv[3:] == ['--streaming-only']
ADMISSION_ONLY = sys.argv[3:] == ['--admission-only']
assert not sys.argv[3:] or ADMISSION_ONLY or STREAMING_ONLY or LIMIT_ONLY, sys.argv[3:]


class Host:
    def __init__(self, url=None):
        env = child_environment()
        if url:
            env.update(THINKTHEN_TEST_LOOPBACK_URL=url, THINKTHEN_API_KEY='loopback-test-key', OPENAI_API_KEY='loopback-test-key')
        self.process = subprocess.Popen([str(PACKAGE / 'sql'), str(PACKAGE / 'thinkthen.duckdb_extension')],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, env=env)

    def query(self, statement):
        self.process.stdin.write(json.dumps(statement) + '\n')
        self.process.stdin.flush()
        result = json.loads(self.process.stdout.readline())
        assert 'error' not in result, result
        return result['rows']

    def configure(self, settings):
        for key, value in settings.items():
            for name, item in (value.items() if key == 'retry' else [(key, value)]):
                literal = quote(item) if isinstance(item, str) else str(item)
                self.query('SET thinkthen_' + name + ' = ' + literal)

    def configure_failure(self, settings, code):
        for key, value in settings.items():
            if key == 'backend':
                continue
            self.failure('SET thinkthen_' + key + ' = ' + ('NULL' if value is None else quote(value)))

    def failure(self, statement, text=None):
        self.process.stdin.write(json.dumps(statement) + '\n')
        self.process.stdin.flush()
        result = json.loads(self.process.stdout.readline())
        assert 'error' in result and (text is None or text in result['error']), result

    def call_failure(self, request, code):
        self.failure('SELECT * FROM ' + invocation(request), code)

    def call(self, request):
        rows = self.query('SELECT to_json(t) AS result FROM ' + invocation(request) + ' AS t')
        output = []
        for item in rows:
            row = canonical(json.loads(item['result']), request['question']['function'])
            output.append(tuple(row.get(name) if name in ('position', 'status', 'probability', 'kept') else
                                json.dumps(row[name]) if name in row else None
                                for name in ('position','status','answer','probability','error','detail','kept','id','source')))
        return output

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


if LIMIT_ONLY:
    first_response(Host, LOOPBACK)
    sys.exit(0)
if not STREAMING_ONLY:
    admission(SimpleNamespace(package=PACKAGE))
    if ADMISSION_ONLY:
        sys.exit(0)
streaming(Host, PACKAGE, LOOPBACK)
if STREAMING_ONLY:
    sys.exit(0)

with Host() as host:
    forbidden_settings(host)
    assert host.query('SELECT 1 AS alive') == [{'alive': '1'}]
run_cases(Host, LOOPBACK)
for mode in ('headers', 'body', 'refusal-body'):
    with fixture(LOOPBACK, '--held', mode) as (server, url):
        env = dict(child_environment(), THINKTHEN_TEST_LOOPBACK_URL=url, THINKTHEN_API_KEY='loopback-test-key')
        with subprocess.Popen([str(PACKAGE / 'sql'), str(PACKAGE / 'thinkthen.duckdb_extension'), 'cancel'],
            stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, env=env) as process:
            try:
                assert command(server, 'arrived') == 'arrived'
                process.stdin.write('c'); process.stdin.flush()
                assert command(server, 'closed') == 'closed'
                assert 'interrupt' in json.loads(process.stdout.readline())['error'].lower()
                assert process.wait(timeout=30) == 0
            finally:
                if process.poll() is None:
                    process.kill(); process.wait()


# Native DuckDB parameters, typed result layouts and position-bound source values.
with Host() as host:
    host.configure({'backend': 'stub'})
    def query(sql):
        return [json.loads(row['result']) for row in host.query('SELECT to_json(t) AS result FROM (' + sql + ') t')]
    rows = query("SELECT * FROM thinkthen_decide((SELECT 42 AS document_id, NULL::VARCHAR AS record, [1,2] AS marks), '{literal?', name := 'audit')")
    assert len(rows) == 1 and rows[0]['document_id'] == 42 and rows[0]['marks'] == [1, 2]
    assert rows[0]['position'] == 1 and rows[0]['status'] == 'failed' and rows[0]['error_code'] == 'record.missing'
    assert rows[0]['source']['name'] == 'audit'
    for column in ('id', 'Answer', 'POSITION', 'source', 'detail'):
        host.failure('SELECT * FROM thinkthen_decide((SELECT \'evidence\' AS record, 1 AS "' + column + '"), \'Ready?\')', 'usage.sql-selection')
    host.failure("SELECT * FROM thinkthen_filter((SELECT 'evidence' AS record, true AS kept), 'Ready?')", 'usage.sql-selection')
    host.failure("SELECT * FROM thinkthen_choose((SELECT 'evidence' AS record), 'Ready?')", 'usage.question')
    host.failure("SELECT * FROM thinkthen_decide((SELECT 'evidence' AS record), 'Ready?', threshold := 0.7, low := 0.2, high := 0.8)", 'usage.threshold')
    host.failure("SELECT * FROM thinkthen_annotate((SELECT 'evidence' AS record), toolbox := '{\"bad\":{\"function\":\"rank\",\"question\":\"Order?\",\"levels\":[\"low\",\"high\"]}}'::JSON)", 'usage.question')
    ranked = query("SELECT * FROM thinkthen_rank((SELECT range AS document_id, range::VARCHAR AS record, struct_pack(copy := range) AS original FROM range(20)), 'Order?', levels := ['low','medium','high'])")
    assert [row['document_id'] for row in ranked] != list(range(20)), ranked
    assert len(ranked) == 20 and {row['position'] for row in ranked} == set(range(1, 21))
    assert all(row['document_id'] + 1 == row['position'] and row['original']['copy'] == row['document_id'] for row in ranked)
    literal = query("SELECT * FROM thinkthen_decide((SELECT 'null' AS body), '{literal?', record := 'body')")
    assert len(literal) == 1 and literal[0]['status'] == 'answered', literal
    types = host.query("DESCRIBE SELECT * FROM thinkthen_choose((SELECT 'evidence' AS record), 'Ready?', choices := ['yes','no'])")
    columns = {row['column_name']: row['column_type'] for row in types}
    assert columns['answer'] == 'VARCHAR' and columns['probability'] == 'DOUBLE'
    assert columns['status'].startswith('ENUM(') and columns['error_code'].startswith('ENUM(')
    assert columns['detail'].startswith('STRUCT(') and columns['source'].startswith('STRUCT(')
    host.query("PREPARE reusable AS SELECT * FROM thinkthen_decide((SELECT 'evidence' AS record), $1)")
    assert len(host.query("EXECUTE reusable('Ready?')")) == 1
    host.query("2:SET thinkthen_backend='stub'")
    host.query("2:SET thinkthen_model='second-model'")
    host.query("SET thinkthen_model='snapshot-model'")
    first = host.query("SELECT current_setting('thinkthen_model') AS model")
    assert first == [{'model': 'snapshot-model'}], first
    assert host.query('2:SELECT current_setting(\'thinkthen_model\') AS model') == [{'model': 'second-model'}]
    for option in ('api_key', 'url', 'settings', 'adapter', 'mode', 'key_source'):
        host.failure('SET thinkthen_' + option + '=NULL')
    for option in ('model', 'batch', 'max_retries', 'initial_backoff_ms', 'attempt_timeout_ms'):
        host.query('RESET thinkthen_' + option)
    host.failure("SET thinkthen_backend='unregistered'")
    host.failure("SET GLOBAL thinkthen_batch=2", 'session-only')
    host.failure("SELECT * FROM thinkthen_decide((SELECT 'evidence' AS record), 'Ready?', name := '')", 'usage.name')
    continued = query("SELECT count(*) AS count, count(*) FILTER (WHERE document_id + 1 != position) AS misplaced FROM thinkthen_decide((SELECT range AS document_id, range::VARCHAR AS record FROM range(4099)), 'Ready?')")
    assert continued == [{'count': 4099, 'misplaced': 0}], continued
    host.query('SELECT 1 AS alive')
for lifecycle in ('--cold', '--abandon'):
    result = subprocess.run([str(PACKAGE / 'sql'), str(PACKAGE / 'thinkthen.duckdb_extension'), lifecycle],
                            env=child_environment(), capture_output=True, text=True, timeout=30)
    assert result.returncode == 0, (lifecycle, result.stdout, result.stderr)

print('DuckDB typed rows, both wire formats, native values, isolation, ownership and cancellation pass')
