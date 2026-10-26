"""Run the canonical SQL corpus through installed hosts and private test routes."""
from contextlib import contextmanager
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'databases'))
from testing import child_environment


def command(server, value):
    server.stdin.write(value + '\n')
    server.stdin.flush()
    return server.stdout.readline().strip()


@contextmanager
def fixture(loopback, *arguments):
    with subprocess.Popen([str(loopback), *map(str, arguments)], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, env=child_environment()) as server:
        try:
            port = int(server.stdout.readline())
            yield server, f'http://127.0.0.1:{port}/v1'
            command(server, 'shutdown')
            assert server.wait(timeout=30) == 0
        finally:
            if server.poll() is None:
                server.kill()
                server.wait()


def semantic(row):
    row.pop('source', None)
    if 'error' in row:
        assert row['error']['message']
        row['error'].pop('message')
    if row.get('function') == 'annotate' and 'answer' in row:
        for child in row['answer'].values():
            semantic(child)
    return row


def decode(native, function, backend, model):
    position, status, answer, probability, error, detail, kept, identifier, source = native
    source = json.loads(source) if isinstance(source, str) else source
    assert source['backend'] == backend and source['model'] == model, source
    assert len(source['question_identity']) == 64, source
    row = {'function': function, 'position': int(position), 'status': status}
    for name, value in [('id', identifier), ('answer', answer), ('error', error), ('detail', detail)]:
        if value is not None:
            row[name] = json.loads(value) if isinstance(value, str) else value
    if probability is not None:
        row['probability'] = float(probability)
    if function == 'filter':
        expected = status == 'answered' and row.get('answer') is True
        assert kept in (expected, str(expected).lower()), (kept, row)
    return semantic(row)


def run_cases(factory, loopback):
    directory = ROOT / 'conformance/typed'
    manifest = json.loads((directory / 'manifest.json').read_text())
    covered = set()
    for file in manifest['cases']:
        case = json.loads((directory / file).read_text())
        function = case['request']['question']['function']
        for backend in manifest['backends']:
            settings = {**manifest['client'], **case.get('client', {}), 'backend': backend}
            with fixture(loopback, directory / file) as (server, url), factory(url) as host:
                host.configure(settings)
                if isinstance(case['expected'], dict):
                    host.call_failure(case['request'], case['expected']['error'])
                    assert int(command(server, 'count')) == case['requests'], file
                    continue
                actual = [decode(row, function, backend, settings['model']) for row in host.call(case['request'])]
                assert actual == case['expected'], (file, backend, actual, case['expected'])
                requests = int(command(server, 'count'))
                # Unknown-length SQL input can flush a batch at a scheduling gap.
                if case['requests'] == 0:
                    assert requests == 0, (file, backend, requests)
                else:
                    maximum = case['requests'] * len(case['request']['records'])
                    assert 0 < requests <= maximum, (file, backend, requests, maximum)
                missing = host.call({'question': {'function': 'decide', 'question': 'Ready?'}, 'records': [None]})
                assert len(missing) == 1 and missing[0][1] == 'failed', missing
                failure = json.loads(missing[0][4]) if isinstance(missing[0][4], str) else missing[0][4]
                assert failure['code'] == 'record.missing', failure
                assert int(command(server, 'count')) == requests, file
            if case.get('exact_requests'):
                # One record proves its questions share a request without
                # requiring separate SQL input chunks to arrive together.
                request = {**case['request'], 'records': case['request']['records'][:1]}
                with fixture(loopback, directory / file) as (server, url), factory(url) as host:
                    host.configure(settings)
                    actual = [decode(row, function, backend, settings['model']) for row in host.call(request)]
                    assert actual == case['expected'][:1], (file, backend, actual)
                    assert int(command(server, 'count')) == case['requests'], (case['name'], backend)
            covered.add((function, backend))
    assert len(covered) == 20, covered


def forbidden_settings(host):
    host.configure({'backend': 'stub'})
    for field in ('url', 'api_key', 'key_source'):
        host.configure_failure({'backend': 'stub', field: None}, 'usage.settings')
    for field, value in [('adapter', 'stub'), ('mode', 'proxy')]:
        host.configure_failure({field: value}, 'usage.settings')
