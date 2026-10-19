"""Exercise the installed blocking C entry point with canonical loopback cases."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
ROOT = Path(__file__).resolve().parents[3]
sys.path[:0] = [str(ROOT / 'conformance'), str(ROOT / 'conformance/children')]
from children import child_env, read_process_line
import c_parity as shared
import parity
from cli_parity import project_row
from session_cases import descriptor
from installed_cases import static_flags

# One actual case for each public function; run each with inline and callback input.
CASES = ('01-decide-yes-captured', '06-choose-billing', '09-tag-two', '11-score-middle',
         '13-filter-records', '15-rank-records', '17-annotate-mixed', '18-find-second',
         '42-recognize-C01-relations', '51-same-kind-alerts',
         '20-usage-fault', '21-backend-fault', '23-cancelled-fault', '24-deadline-fault')


def invoke(binary, value, settings, env, home, backend, mode):
    source = descriptor(value, home)
    items = source['input'].get('items', [])
    if mode != 'plain':
        source['input'] = {'kind': 'feed', 'name': 'records'}
    args = [str(binary), json.dumps(settings), source['verb'], json.dumps(source['question']),
            json.dumps(source['input']), json.dumps(source['options']),
            'held' if source['held_cancel'] else 'cancel' if source['cancel'] else '-', mode]
    if mode != 'plain':
        args += [json.dumps(item) for item in items]
    if source['held_cancel']:
        process = subprocess.Popen(args, cwd=home, env=env, stdin=subprocess.PIPE,
                                   stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        try:
            assert backend.read('wait 1') == 'wait 1'
            process.stdin.write('!'); process.stdin.flush()
            assert read_process_line(process.stdout) == 'cancel-fired\n'
            backend.process.stdin.write('release\n'); backend.process.stdin.flush()
            stdout, stderr = process.communicate(timeout=60)
            output = subprocess.CompletedProcess(args, process.returncode, stdout, stderr)
        finally:
            backend.process.stdin.write('release\n'); backend.process.stdin.flush()
            if process.poll() is None:
                process.kill(); process.wait()
    else:
        output = subprocess.run(args, cwd=home, env=env, capture_output=True, text=True, timeout=120)
    assert output.returncode == 0 and not output.stderr, (output.returncode, output.stderr)
    return json.loads(output.stdout)


def project(payload, value):
    if payload.get('code'):
        return {**payload, **(payload['facts'] or {})}
    rows = []
    results = payload['value']
    for result in results if isinstance(results, list) else [results]:
        result = {**result, **result.get('source', {})}
        if value['verb'] == 'relate' and 'input_sources' in result:
            sources = {entry['index']: entry['source'] for entry in result['input_sources']}
            result['input'] = [{'record': original, **sources.get(index, {})}
                               for index, original in enumerate(result['input'])]
        rows.append(project_row(result, dict(value, context_present=False, contexts=None,
                                             candidate_orders=None), len(rows)))
    return {'code': 0, 'schema': 'thinkthen.result/2', 'rows': rows,
            **payload['facts'], 'call_result': payload, 'settled': payload.get('answer')}


def run(native, scratch):
    binary = scratch / 'blocking-call'
    env = child_env(home=scratch / 'home')
    subprocess.run(['cc', '-std=c11', '-Wall', '-Wextra', '-Werror', '-pthread',
                    '-I', str(ROOT / 'libraries/c/tests/c'),
                    str(ROOT / 'libraries/c/tests/c/run_call.c'),
                    *static_flags(native, env), '-o', str(binary)], env=env, check=True)
    rows = parity.required_cases(parity.inventory(), 'c')
    cases = {case['id']: case for case in json.loads((ROOT / 'conformance/cases.json').read_text())['cases']}
    named = {case['id']: case for case in json.loads((ROOT / 'conformance/named-inputs.json').read_text())['cases']}
    backend_binary = Path(os.environ.get('CARGO_TARGET_DIR', str(ROOT / 'target'))) / 'debug/conformance-backend'
    for name in CASES:
        row = rows[name]
        value = shared.document(row, cases, named)
        for mode in ('plain', 'fed'):
            with tempfile.TemporaryDirectory(prefix='blocking-c-', dir=scratch) as folder:
                home = Path(folder)
                isolated = child_env(home=home, PATH=os.environ.get('PATH', '/usr/bin:/bin'))
                backend = shared.Backend(backend_binary, isolated)
                try:
                    isolated.update(THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.port}/{value["arm"]}',
                                    THINKTHEN_API_KEY='sk-conformance-loopback')
                    shared.prepare(home, value)
                    settings = {'cache': False, 'model': 'jev-1.13.0', 'batch': 1, 'max_retries': 0}
                    payload = invoke(binary, value, settings, isolated, home, backend, mode)
                    shared.assertions(row, value, project(payload, value), int(backend.read('count')))
                    if payload.get('code'):
                        assert payload['retryable'] is False, payload
                        if payload['facts']:
                            assert payload['facts']['requests_sent'] == int(backend.read('count')), payload
                    print(f'installed blocking C: {name} {mode} PASS', flush=True)
                finally:
                    backend.close()
    # Producer failure and cancellation while provider work remains held use the same caller.
    for probe in ('producer', 'held'):
        with tempfile.TemporaryDirectory(prefix='blocking-c-stop-', dir=scratch) as folder:
            home = Path(folder)
            isolated = child_env(home=home, PATH=os.environ.get('PATH', '/usr/bin:/bin'))
            backend = shared.Backend(backend_binary, isolated)
            try:
                isolated.update(THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.port}/arm/{"held" if probe == "held" else "full"}/v1',
                                THINKTHEN_API_KEY='sk-conformance-loopback')
                value = shared.document(rows[CASES[0]], cases, named)
                value['held_cancel'] = probe == 'held'
                payload = invoke(binary, value, {'cache': False, 'max_retries': 0}, isolated, home,
                                 backend, 'fail' if probe == 'producer' else 'fed')
                assert payload['code'] == (4 if probe == 'producer' else 5), payload
                assert payload['retryable'] is False and payload['message'], payload
                if probe == 'producer':
                    assert payload['message'] == 'session producer raised an exception', payload
                    assert int(backend.read('count')) == 0
                else:
                    assert payload['facts']['requests_sent'] == 1, payload
                print(f'installed blocking C: {probe} PASS', flush=True)
            finally:
                backend.close()
