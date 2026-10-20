"""Exercise the installed blocking C entry point with canonical loopback cases."""
import json
from contextlib import chdir
import os
from pathlib import Path
import subprocess
import shutil
import sqlite3
import sys
import tempfile
ROOT = Path(__file__).resolve().parents[3]
sys.path[:0] = [str(ROOT / 'conformance'), str(ROOT / 'conformance/children')]
from children import child_env, read_process_line
import c_parity as shared
import parity
from cli_parity import project_row
from packet_dump import expanded, difference, decimal_json
from fixture_suite import selected_case_ids, request, native_request, assert_required
from c_images import assert_images
from installed_cases import static_flags


def invoke(binary, value, settings, env, home, backend, mode, *, dumps=None):
    source = native_request(request(value, Path('fixture-source'), home, settings,
                                    relative_sources=True))
    items = source['input'].get('items', [])
    if source['cancel']:
        # An empty feed plus a pre-fired token cannot race a full-input send.
        mode, items = 'fed', []
    if mode != 'plain':
        source['input'] = {'kind': 'feed', 'name': 'records'}
    question = source['question']
    raw = question.get('raw')
    question = ('{"kind":"definition","value":' + raw + '}' if raw is not None
                else json.dumps(question))
    args = [str(binary), json.dumps(settings), source['verb'], question,
            json.dumps(source['input']), json.dumps(source['options']),
            'held' if source['held_cancel'] else 'cancel' if source['cancel'] else '-', mode]
    if mode != 'plain':
        args += [json.dumps(item) for item in items]
    arguments = home / 'call-arguments.txt'
    arguments.write_text('\n'.join(args[1:]) + '\n')
    args = [str(binary), str(arguments)]
    if source['held_cancel']:
        before = int(backend.read('count'))
        process = subprocess.Popen(args, cwd=home, env=env, stdin=subprocess.PIPE,
                                   stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        try:
            admitted = f'wait {before + 1}'
            assert backend.read(admitted) == admitted
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
    assert 'sk-conformance-loopback' not in output.stdout, 'C call leaked the key'
    if dumps is not None:
        dumps.append(decimal_json(output.stdout))
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


def check_owned(row, value, payload, raw_payload, count, sent, expected_dump, home, backend):
    """Pin the owned C fields; native stop locations and prefixes remain Rust proof."""
    projected = project(payload, value)
    expected = value['expect']
    if 'completed_prefix' in expected:
        value = {**value, 'expect': {key: item for key, item in expected.items()
                                   if key not in ('completed_prefix', 'stopped_at')}}
    shared.assertions(row, value, projected, count)
    if payload.get('code'):
        packets = expected_dump.get('packets', [])
        failure = expected_dump.get('error') or next(packet['failure'] for packet in packets
                                                    if packet.get('failure'))
        native = failure['error']
        message = ('invalid session arguments or input' if native['message'] == 'invalid canonical request'
                   else native['message'])
        for prefix, replacement in ((str(home), '$FOLDER'), (str(ROOT), '$ROOT')):
            message = message.replace(replacement, prefix)
        actual = {key: payload[key] for key in ('code', 'message', 'retryable')}
        wanted = {'code': shared.ERRORS[native['kind']], 'message': message,
                  'retryable': native['retryable']}
        assert actual == wanted, difference(wanted, actual)
        if 'facts' not in failure:
            assert payload['facts'] is None, payload
        if payload['facts'] is not None:
            assert payload['facts']['requests_sent'] == sent, payload
    else:
        assert {'value', 'answer', 'facts'} <= payload.keys(), payload
        assert isinstance(payload['facts'], dict) and isinstance(payload['answer'], dict), payload
        assert payload['facts']['requests_sent'] == sent, payload
        terminal = next(packet for packet in expected_dump['packets']
                        if packet['kind'] == 'terminal')
        assert raw_payload['answer'] == terminal['answer'], row['id'] + ':\n' + difference(
            terminal['answer'], raw_payload['answer'])
        body_checks = ('selected_item', 'per_item_context', 'capture_request')
        bodies = (json.loads(backend.read('capture'))['bodies']
                  if any(key in value['expect'] for key in body_checks) else [])
        # Existing shared assertions own detail fields, presence and input mapping.
        # Pass through the actual native Call; do not manufacture session packets.
        with chdir(home):
            assert_required({'native': True, 'results': payload['value'],
                             'facts': payload['facts'], 'answer': payload['answer']},
                            row, value, bodies, Path('fixture-source'))


def compile_consumer(native, scratch, source, binary, env):
    """Compile a fixture outside the checkout against the installed static archive."""
    for name in (source, 'platform.h'):
        shutil.copyfile(ROOT / 'libraries/c/tests/c' / name, scratch / name)
    subprocess.run(['cc', '-std=c11', '-Wall', '-Wextra', '-Werror', '-pthread',
                    '-I', str(scratch), str(scratch / source),
                    *static_flags(native, env), '-o', str(binary)],
                   cwd=scratch, env=env, check=True)


def run(native, scratch):
    binary = scratch / 'blocking-call'
    env = child_env(home=scratch / 'home')
    compile_consumer(native, scratch, 'run_call.c', binary, env)
    rows = parity.required_cases(parity.inventory(), 'c')
    corpus = json.loads((ROOT / 'conformance/cases.json').read_text())
    cases = {case['id']: case for case in corpus['cases']}
    named = {case['id']: case for case in json.loads((ROOT / 'conformance/named-inputs.json').read_text())['cases']}
    backend_binary = Path(os.environ.get('CARGO_TARGET_DIR', str(ROOT / 'target'))) / 'debug/conformance-backend'
    selected = os.environ.get('THINKTHEN_CONFORMANCE_IDS')
    if selected:
        wanted = selected_case_ids(selected)
        assert wanted <= rows.keys(), 'unknown installed C case'
        rows = {name: row for name, row in rows.items() if name in wanted}
    elif os.environ.get('THINKTHEN_TEST_PROFILE', 'routine') == 'routine':
        wanted = set((ROOT / 'conformance/routine-ids.txt').read_text().splitlines())
        rows = {name: row for name, row in rows.items() if name in wanted}
    oracle = {case['id']: case['dumps'] for case in expanded(json.loads(
        (ROOT / 'conformance/fixtures/packets.json').read_text()))['cases']}
    for name, row in rows.items():
        value = shared.document(row, cases, named)
        with tempfile.TemporaryDirectory(prefix='blocking-c-', dir=scratch) as folder:
            home = Path(folder)
            (home / 'fixture-source').symlink_to(ROOT, target_is_directory=True)
            isolated = child_env(home=home, LANG='C.UTF-8', LC_ALL='C.UTF-8', THINKTHEN_USAGE='off')
            backend = shared.Backend(backend_binary, isolated)
            try:
                shared.prepare(home, value)
                dumps = []
                for index, step in enumerate(value.get('steps', [value])):
                    if step.get('copy_store'):
                        (home / 'refreshed').mkdir()
                        with sqlite3.connect(home / 'saved/thinkthen.sqlite') as old:
                            with sqlite3.connect(home / 'refreshed/thinkthen.sqlite') as new:
                                old.backup(new)
                    if step.get('damage_store'):
                        with sqlite3.connect(home / 'saved/thinkthen.sqlite') as saved:
                            saved.execute("UPDATE answers SET answer='damaged fixture answer'")
                    if value.get('image_variants'):
                        backend.close()
                        backend = shared.Backend(backend_binary, isolated)
                        shared.prepare(home, step)
                    isolated.update(THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.port}/{step["arm"]}',
                                    THINKTHEN_API_KEY='sk-conformance-loopback')
                    settings = {'cache': False, 'batch': 1, 'throttle': 1, 'max_retries': 0,
                                'model': 'jev-latest' if 'steps' in value else 'jev-1.13.0',
                                **step.get('settings', {}), 'base_url': isolated['THINKTHEN_BASE_URL']}
                    replacements = {'$FOLDER': str(home / 'saved'), '$REFRESH': str(home / 'refreshed'),
                                    '$PROFILE': str(home / 'profile.json')}
                    settings = {key: replacements.get(item, item) if isinstance(item, str) else item
                                for key, item in settings.items()}
                    if row['kind'] in ('images', 'image-location'):
                        settings['record'] = str(home / 'recorded')
                    before = int(backend.read('count'))
                    mode = 'fed' if step.get('incremental') else 'plain'
                    payload = invoke(binary, step, settings, isolated, home, backend, mode, dumps=dumps)
                    count = int(backend.read('count'))
                    check_owned(row, step, payload, dumps[-1],
                                count - before if step.get('count_delta') else count,
                                count - before, oracle[name][index], home, backend)
                    if step.get('stored_answers') == 0:
                        database = home / 'saved/thinkthen.sqlite'
                        if database.exists():
                            with sqlite3.connect(database) as saved:
                                assert saved.execute('SELECT count(*) FROM answers').fetchone()[0] == 0
                    if value.get('image_variants'):
                        assert_images(step, project(payload, step), json.loads(backend.read('capture'))['bodies'])
                    if row['kind'] in ('images', 'image-location') and not payload.get('code'):
                        replay = {key: item for key, item in settings.items() if key != 'record'}
                        replay['replay'] = str(home / 'recorded')
                        repeated = invoke(binary, step, replay, isolated, home, backend, mode)
                        assert repeated['facts']['requests_sent'] == 0
                        assert int(backend.read('count')) == count
                        first = project(payload, step)['rows']
                        second = project(repeated, step)['rows']
                        assert [r['answer_id'] for r in second] == [r['answer_id'] for r in first]
                print('parity: ' + json.dumps({'consumer': 'c', 'case': name,
                      'checks': row.get('checks', ['named', 'runtime']), 'status': 'pass'}), flush=True)
            finally:
                backend.close()
    # One shared behavior table drives the installed C and Rust callers.
    for probe in corpus['call_behaviors']:
        with tempfile.TemporaryDirectory(prefix='blocking-c-stop-', dir=scratch) as folder:
            home = Path(folder)
            (home / 'fixture-source').symlink_to(ROOT, target_is_directory=True)
            isolated = child_env(home=home, PATH=os.environ.get('PATH', '/usr/bin:/bin'), THINKTHEN_USAGE='off')
            backend = shared.Backend(backend_binary, isolated)
            try:
                isolated.update(THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.port}/arm/{"held" if probe == "held-cancel" else "full"}/v1',
                                THINKTHEN_API_KEY='sk-conformance-loopback')
                value = shared.document(parity.required_cases(parity.inventory(), 'c')['01-decide-yes-captured'], cases, named)
                value['held_cancel'] = probe == 'held-cancel'
                if probe == 'expired-deadline':
                    value['operation'] = {'injection': 'expired_deadline'}
                mode = {'producer': 'fail', 'missing-producer': 'missing',
                        'held-cancel': 'fed', 'expired-deadline': 'plain'}[probe]
                payload = invoke(binary, value, {'cache': False, 'max_retries': 0}, isolated, home,
                                 backend, mode)
                expected = corpus['call_behaviors'][probe]
                assert payload['code'] == shared.ERRORS[expected['kind']], payload
                assert payload['retryable'] is False and payload['message'] == expected['message'], payload
                assert int(backend.read('count')) == expected['requests_sent'], payload
                if 'facts_requests_sent' in expected:
                    assert payload['facts']['requests_sent'] == expected['facts_requests_sent'], payload
                print(f'installed blocking C: {probe} PASS', flush=True)
            finally:
                backend.close()
