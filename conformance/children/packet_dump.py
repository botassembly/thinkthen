"""One packet oracle, shared request preparation and decimal-safe comparison.

Dumpers return {"packets": [native packet JSON, ...]} and optional host
conversions. Only this comparator normalizes runtime identities and timing.
"""
import difflib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import tempfile
from decimal import Decimal

ROOT = Path(__file__).resolve().parents[2]
ORACLE = ROOT / 'conformance/fixtures/packets.json'
PRIVATE_CASE = '25-defect-fault'
IDENTITIES = {'call_id', 'observation_id', 'failure_id', 'answer_id',
              'sdk_request_id', 'request_sha256'}


def decimal_json(text):
    """Keep JSON numbers as exact decimal strings, including large integers."""
    def number(raw):
        value = Decimal(raw)
        return format(value.normalize(), 'f') if value else '0'
    return json.loads(text, parse_int=number, parse_float=number)


def canonical(document):
    """Keep packet structure, presence and identity equality; remove run entropy."""
    identities = {}

    def visit(value, key=''):
        if isinstance(value, dict):
            return {name: visit(child, name) for name, child in sorted(value.items())}
        if isinstance(value, list):
            return [visit(child, 'request_sha256' if key == 'requests' else key)
                    for child in value]
        if key in IDENTITIES and isinstance(value, str) and re.fullmatch('[0-9a-f]{64}', value):
            group = 'request_sha256' if key == 'requests' else key
            table = identities.setdefault(group, {})
            return table.setdefault(value, f'{group}:{len(table) + 1}')
        if key in ('seconds', 'wall_ms') and isinstance(value, str):
            assert Decimal(value) >= 0, (key, value)
            return '0'
        if key == 'url' and isinstance(value, str):
            return re.sub(r'http://127\.0\.0\.1:\d+/', 'http://127.0.0.1:PORT/', value)
        return value

    return visit(document)


def requests():
    """Use shared case composition, then the public canonical Request grammar."""
    sys.path.insert(0, str(ROOT / 'conformance'))
    from c_parity import document
    cases = json.loads((ROOT / 'conformance/cases.json').read_text())['cases']
    by_id = {case['id']: case for case in cases}
    rows = []
    for case in cases:
        if case['id'] == PRIVATE_CASE:
            continue
        value = document({'id': case['id'], 'kind': 'behavior', 'verb': case['verb'],
                          'input': {'case_ref': case['id']}}, by_id, {})
        question = value['question'].copy()
        options = {}
        if case['verb'] == 'find' and question.pop('none', False):
            options['none'] = True
        files = {}
        if value.get('question_form') == 'file':
            files['question.json'] = json.dumps(question)
            selector = {'kind': 'file', 'path': '$FOLDER/question.json'}
        else:
            selector = {'kind': 'definition', 'value': question}
        items = [{'original': {'kind': 'text', 'text': item} if value['text']
                  and isinstance(item, str) else {'kind': 'json', 'value': item}}
                 for item in value['items']]
        source = {'kind': 'records', 'items': items}
        injection = (value.get('operation') or {}).get('injection')
        if injection == 'recording_read_failure':
            source = {'kind': 'source', 'source': {'paths': ['$FOLDER/missing-input'],
                      'reading': {'unit': 'file'}, 'media': 'text'}}
        if injection == 'expired_deadline':
            options['deadline_ms'] = 0
        if injection == 'cancel_token':
            source = {'kind': 'feed', 'name': 'cancelled'}
        request = {'schema': 'thinkthen.request/1', 'call': {'function': case['verb'],
                   'question': selector, 'input': source, 'options': options}}
        rows.append({'id': case['id'], 'arm': value['arm'], 'request': request,
                     **({'cancel': True} if injection == 'cancel_token' else {}),
                     **({'files': files} if files else {})})
    return {'schema': 'thinkthen.packet-oracle/1', 'cases': rows,
            'excluded': [{'id': PRIVATE_CASE, 'reason':
                          'Private invariant injection has no admitted public input; existing core defect tests retain coverage.'}]}


def prepare(case, home, base):
    """Resolve owned fixture files without retaining machine paths in the oracle."""
    for name, text in case.get('files', {}).items():
        (home / name).write_text(text)
    request = json.loads(json.dumps(case['request']).replace('$FOLDER', str(home)))
    return {'request': request, 'base_url': base + '/' + case['arm'],
            'cancel': case.get('cancel', False)}


def difference(want, got):
    """Report the changed packet fields as a readable unified JSON diff."""
    left = json.dumps(want, ensure_ascii=False, sort_keys=True, indent=2).splitlines(True)
    right = json.dumps(got, ensure_ascii=False, sort_keys=True, indent=2).splitlines(True)
    return ''.join(difflib.unified_diff(left, right, fromfile='oracle', tofile='actual'))


def run(consumer, command, root):
    """Compare every admitted shared case against an installed public dumper."""
    from children import child_env, fixture_run
    sys.path.insert(0, str(root / 'conformance'))
    from c_parity import Backend
    backend_path = Path(os.environ.get('THINKTHEN_TEST_BACKEND',
                         str(Path(os.environ.get('CARGO_TARGET_DIR', str(root / 'target'))) /
                             'debug/conformance-backend')))
    oracle = json.loads(ORACLE.read_text())
    failures = 0
    for case in oracle['cases']:
        failed = False
        with tempfile.TemporaryDirectory(prefix='thinkthen-packet-') as tmp:
            home = Path(tmp)
            env = child_env(home=home)
            backend = Backend(backend_path, env)
            try:
                given = prepare(case, home, f'http://127.0.0.1:{backend.port}')
                child = fixture_run(command, input=json.dumps(given), text=True,
                                    capture_output=True, env=env, cwd=home, check=True)
                assert not child.stderr, child.stderr
                got = canonical(decimal_json(child.stdout))
                want = case['dump']
                if got != want:
                    raise AssertionError(difference(want, got))
            except Exception as error:
                failures += 1
                failed = True
                print(f'{case["id"]}: {error}', file=sys.stderr)
            finally:
                backend.close()
        print('parity: ' + json.dumps({'consumer': consumer, 'case': case['id'],
              'checks': ['named', 'runtime'], 'status': 'fail' if failed else 'pass'}), flush=True)
    return failures


def main():
    if '--requests' in sys.argv:
        print(json.dumps(requests()))
        return 0
    if '--check' in sys.argv:
        raw = json.loads(sys.stdin.read())
        for case in raw['cases']:
            case['dump'] = canonical(decimal_json(json.dumps(case['dump'])))
        if '--update' in sys.argv:
            ORACLE.write_text(json.dumps(raw, ensure_ascii=False, sort_keys=True, indent=2) + '\n')
            return 0
        want = json.loads(ORACLE.read_text())
        if raw != want:
            print(difference(want, raw), file=sys.stderr)
            return 1
        print('packet oracle: current')
        return 0
    raise SystemExit('use --requests or --check [--update]')


if __name__ == '__main__':
    sys.exit(main())
