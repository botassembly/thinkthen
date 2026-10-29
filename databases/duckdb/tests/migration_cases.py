"""Run the canonical SQL corpus through installed hosts and private test routes."""
from contextlib import contextmanager
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'databases'))
from testing import child_environment, read_line
SOURCE = json.loads((ROOT / 'schemas/row.schema.json').read_text())['$defs']['Source']


class SqlError(AssertionError):
    """The real native SQL host rejected a statement."""


def command(server, value, timeout=30):
    server.stdin.write(value + '\n')
    server.stdin.flush()
    return read_line(server.stdout, timeout=timeout).strip()


@contextmanager
def fixture(loopback, *arguments):
    with subprocess.Popen([str(loopback), *map(str, arguments)], stdin=subprocess.PIPE,
                          stdout=subprocess.PIPE, text=True, env=child_environment()) as server:
        try:
            port = int(read_line(server.stdout))
            yield server, f'http://127.0.0.1:{port}/v1'
            command(server, 'shutdown')
            assert server.wait(timeout=30) == 0
        finally:
            if server.poll() is None:
                server.kill()
                server.wait()


def expected_row(row):
    result = {key: value for key, value in row.items() if key != 'source'}
    if row.get('function') == 'annotate' and isinstance(row.get('answer'), dict):
        result['answer'] = {name: expected_row(child) for name, child in row['answer'].items()}
    return result


def strict_equal(actual, expected):
    """Compare JSON values without Python's Boolean/number equivalence."""
    if isinstance(expected, dict):
        return (isinstance(actual, dict) and actual.keys() == expected.keys()
                and all(strict_equal(actual[key], value) for key, value in expected.items()))
    if isinstance(expected, list):
        return (isinstance(actual, list) and len(actual) == len(expected)
                and all(strict_equal(left, right) for left, right in zip(actual, expected)))
    if isinstance(expected, bool) or isinstance(actual, bool):
        return type(actual) is type(expected) and actual == expected
    return actual == expected


def validate_source(source):
    assert isinstance(source, dict), source
    assert set(SOURCE['required']) <= source.keys() <= SOURCE['properties'].keys(), source
    assert all(isinstance(source[key], str) for key in SOURCE['required']), source
    assert len(source['question_identity']) == 64, source
    assert source.get('name') is None or isinstance(source['name'], str), source


def semantic(row, expected=None, *, annotation=False):
    if annotation:
        assert 'source' not in row, row
    source = row.pop('source', None)
    if not annotation:
        validate_source(source)
    if expected is not None and 'source' in expected:
        compared = {key: value for key, value in expected['source'].items() if key != 'question_identity'}
        assert {key: source[key] for key in compared} == compared, (source, expected['source'])
    if 'error' in row:
        assert row['error']['message']
        row['error'].pop('message')
    if row.get('function') == 'annotate' and 'answer' in row:
        for name, child in row['answer'].items():
            semantic(child, None if expected is None else expected.get('answer', {}).get(name), annotation=True)
    return row


def decode(native, function, backend, model, name=None, expected=None):
    position, status, answer, probability, error, detail, kept, identifier, source = native
    source = json.loads(source) if isinstance(source, str) else source
    assert source['backend'] == backend and source['model'] == model, source
    assert len(source['question_identity']) == 64, source
    assert source.get('name') == name, source
    row = {'function': function, 'position': int(position), 'status': status, 'source': source}
    for name, value in [('id', identifier), ('answer', answer), ('error', error), ('detail', detail)]:
        if value is not None:
            row[name] = json.loads(value) if isinstance(value, str) else value
    if probability is not None:
        row['probability'] = float(probability)
    if function == 'filter':
        expected_kept = status == 'answered' and row.get('answer') is True
        assert kept in (expected_kept, str(expected_kept).lower()), (kept, row)
    return semantic(row, expected)


def run_cases(factory, loopback):
    directory = ROOT / 'conformance/typed'
    manifest = json.loads((directory / 'manifest.json').read_text())
    covered = set()
    for file in manifest['cases']:
        case = json.loads((directory / file).read_text())
        function = case['request']['question']['function']
        if case.get('cancelled'):
            reason = case.get('not_applicable', {}).get(factory.surface)
            assert isinstance(reason, str) and reason.strip(), (file, factory.surface)
            print(f'{factory.surface} {file}: not applicable: {reason}')
            continue
        for backend in manifest['backends']:
            settings = {**manifest['client'], **case.get('client', {}), 'backend': backend}
            with fixture(loopback, directory / file) as (server, url), factory(url) as host:
                if isinstance(case['expected'], dict):
                    try:
                        host.configure(settings)
                    except SqlError as error:
                        assert case['expected']['error'] in str(error), (file, error)
                    else:
                        host.call_failure(case['request'], case['expected']['error'])
                    assert int(command(server, 'count')) == case['requests'], file
                    continue
                host.configure(settings)
                native = host.call(case['request'])
                assert len(native) == len(case['expected']), (file, backend, len(native), len(case['expected']))
                actual = [decode(row, function, backend, settings['model'], case['request'].get('name'), expected)
                          for row, expected in zip(native, case['expected'])]
                expected = [expected_row(row) for row in case['expected']]
                assert strict_equal(actual, expected), (file, backend, actual, expected)
                requests = int(command(server, 'count'))
                assert requests == case['requests'], (file, backend, requests, case['requests'])
                missing = host.call({'question': {'function': 'decide', 'question': 'Ready?'}, 'records': [None]})
                assert len(missing) == 1 and missing[0][1] == 'failed', missing
                failure = json.loads(missing[0][4]) if isinstance(missing[0][4], str) else missing[0][4]
                assert failure['code'] == 'record.missing', failure
                assert int(command(server, 'count')) == requests, file
            covered.add((function, backend))
    assert len(covered) == 20, covered


def forbidden_settings(host):
    host.configure({'backend': 'stub'})
    for field in ('url', 'api_key', 'key_source'):
        host.configure_failure({'backend': 'stub', field: None}, 'usage.settings')
    for field, value in [('adapter', 'stub'), ('mode', 'proxy')]:
        host.configure_failure({field: value}, 'usage.settings')


def options_cases(factory):
    """Exercise the shared option corpus at the public native SQL boundary."""
    for case in json.loads((ROOT / 'construction/options.json').read_text()):
        with factory() as host:
            settings = {key: value for key, value in case['client'].items()
                        if value is not None}
            host.configure(settings)
            reason = case.get('not_applicable', {}).get(factory.surface)
            if reason is not None:
                assert isinstance(reason, str) and reason.strip(), (case['name'], factory.surface)
                assert 'expected_error' in case, case['name']
                print(f"{factory.surface} {case['name']}: canonical diagnostic not applicable: {reason}")
                host.native_failure(case['request'])
            elif 'expected_error' in case:
                host.call_failure(case['request'], case['expected_error'])
            else:
                request = case.get('equivalent_request', case['request'])
                function = request['question']['function']
                actual = [decode(row, function, 'stub', 'stub', request.get('name')) for row in host.call(request)]
                expected = [semantic(json.loads(json.dumps(row))) for row in case['expected']]
                assert strict_equal(actual, expected), (case['name'], actual, expected)
    with factory() as host:
        host.configure({'backend': 'stub'})
        host.configure_failure({'max_retries': 4}, 'usage.settings')
