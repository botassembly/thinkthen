"""Build the outside consumer with its dependency's declared Rust compiler."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tomllib


def toml_value(value):
    """Encode the parsed consumer manifest's scalar, array and inline tables."""
    if isinstance(value, dict):
        return '{' + ', '.join(json.dumps(k) + ' = ' + toml_value(v)
                               for k, v in value.items()) + '}'
    if isinstance(value, list):
        return '[' + ', '.join(map(toml_value, value)) + ']'
    return json.dumps(value)


def value_tree_cases(root, worker):
    """Use the installed settled-test worker in a separate existing case fixture."""
    from contextlib import chdir
    import sqlite3
    import tempfile
    sys.path[:0] = [str(root / 'conformance'), str(root / 'conformance/children')]
    import c_parity as shared
    import parity
    from children import child_env
    from fixture_suite import native_request, request, selected_case_ids
    from packet_dump import decimal_json, difference, expanded, folded
    from packet_normalize import canonical

    rows = parity.required_cases(parity.inventory(), 'rust')
    selected = os.environ.get('THINKTHEN_CONFORMANCE_IDS')
    if selected:
        wanted = selected_case_ids(selected)
        assert wanted <= rows.keys(), 'unknown Rust value-tree case'
        rows = {name: row for name, row in rows.items() if name in wanted}
    cases = {case['id']: case for case in json.loads(
        (root / 'conformance/cases.json').read_text())['cases']}
    named = {case['id']: case for case in json.loads(
        (root / 'conformance/named-inputs.json').read_text())['cases']}
    oracle = {case['id']: case['dumps'] for case in expanded(json.loads(
        (root / 'conformance/fixtures/packets.json').read_text()))['cases']}
    backend_binary = Path(os.environ.get('THINKTHEN_TEST_BACKEND', str(
        Path(os.environ.get('CARGO_TARGET_DIR', str(root / 'target'))) / 'debug/conformance-backend')))
    command = [str(worker), '--exact', 'shared_value_tree_fixture', '--nocapture']
    for name, row in rows.items():
        value = shared.document(row, cases, named)
        with tempfile.TemporaryDirectory(prefix='thinkthen-rust-tree-') as temporary:
            home = Path(temporary) / 'home'
            home.mkdir()
            with chdir(home):
                (home / 'fixture-source').symlink_to(root, target_is_directory=True)
                env = child_env(home=home, LANG='C.UTF-8', LC_ALL='C.UTF-8',
                                THINKTHEN_VALUE_TREE_STDIN='1',
                                THINKTHEN_API_KEY='sk-conformance-loopback',
                                LIQUIDAI_API_KEY='sk-conformance-loopback',
                                OPENROUTER_API_KEY='sk-conformance-loopback',
                                PERPLEXITY_API_KEY='sk-conformance-loopback')
                backend = shared.Backend(backend_binary, env)
                try:
                    shared.prepare(home, value)
                    wanted, observed = [], []
                    oracle_index = 0
                    for step in value.get('steps', [value]):
                        if step.get('copy_store'):
                            (home / 'refreshed').mkdir()
                            with sqlite3.connect(home / 'saved/thinkthen.sqlite') as source:
                                with sqlite3.connect(home / 'refreshed/thinkthen.sqlite') as target:
                                    source.backup(target)
                        if step.get('damage_store'):
                            with sqlite3.connect(home / 'saved/thinkthen.sqlite') as saved:
                                saved.execute("UPDATE answers SET answer='damaged fixture answer'")
                        if value.get('image_variants'):
                            backend.close()
                            backend = shared.Backend(backend_binary, env)
                            shared.prepare(home, step)
                        arm = step.get('override_arm', step['arm'] if value.get('image_variants') else value['arm'])
                        settings = {'cache': False, 'batch': 1, 'throttle': 1, 'max_retries': 0,
                                    'model': 'jev-latest' if 'steps' in value else 'jev-1.13.0',
                                    **step.get('settings', {}),
                                    'base_url': f'http://127.0.0.1:{backend.port}/{arm}'}
                        replacements = {'$FOLDER': str(home / 'saved'),
                                        '$REFRESH': str(home / 'refreshed'),
                                        '$PROFILE': str(home / 'profile.json')}
                        settings = {key: replacements.get(item, item) if isinstance(item, str) else item
                                    for key, item in settings.items()}
                        if row['kind'] in ('images', 'image-location'):
                            settings['record'] = str(home / 'recorded')
                        document = native_request(request(step, Path('fixture-source'), home,
                                                          settings, relative_sources=True))
                        if document['held_cancel']:
                            before = int(backend.read('count'))
                            child = subprocess.Popen(command, cwd=home, env=env, stdin=subprocess.PIPE,
                                                     stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                            try:
                                child.stdin.write(json.dumps(document) + '\n')
                                child.stdin.flush()
                                admission = f'wait {before + 1}'
                                assert backend.read(admission) == admission
                                stdout, stderr = child.communicate(input='cancel\n', timeout=120)
                                output = subprocess.CompletedProcess(command, child.returncode, stdout, stderr)
                            finally:
                                backend.process.stdin.write('release\n')
                                backend.process.stdin.flush()
                                if child.poll() is None:
                                    child.kill()
                                    child.wait()
                        else:
                            output = subprocess.run(command, input=json.dumps(document) + '\n',
                                                    cwd=home, env=env, capture_output=True, text=True, timeout=120)
                        assert output.returncode == 0 and not output.stderr, (
                            name, output.returncode, output.stdout[-2000:], output.stderr)
                        assert 'sk-conformance-loopback' not in output.stdout, 'tree fixture leaked its key'
                        text = output.stdout.split('packet_dump: ', 1)[1].splitlines()[0]
                        print('value-tree ' + name + ': ' + text, flush=True)
                        actual = decimal_json(text)
                        expected = folded(oracle[name][oracle_index])
                        oracle_index += 1
                        if row['kind'] in ('images', 'image-location') and 'error' not in expected:
                            # The packet pass also owns a replay immediately after each original step.
                            oracle_index += 1
                        if 'error' in actual:
                            assert 'error' in expected, name + ': unexpected native error'
                            assert actual['error']['error']['kind'] == expected['error']['kind'], name
                            print('value-tree ' + name + ': native error; no settled Call', flush=True)
                            continue
                        assert 'error' not in expected, name + ': missing native error'
                        if document['call_result_contract']:
                            assert name == 'library-call-result', 'unapproved scalar oracle projection'
                            source = document['input']
                            assert source['kind'] == 'records' and len(source['items']) == 1, name
                            original = source['items'][0]['original']
                            assert original['kind'] == 'text' and isinstance(original['text'], str), name
                            assert settings['batch'] == 1, name
                            assert len(expected['results']) == 1, name
                            result = expected['results'][0]
                            assert 'input' not in result and 'index' not in result, name
                            assert 'batch_setting' not in result['meta'], name
                            # Direct typed scalar calls omit the whole-call occurrence fields.
                            # Project only the approved expected fixture into Engine::call's scope.
                            result.update(input=original['text'], index=decimal_json('0'))
                            result['meta']['batch_setting'] = decimal_json('1')
                        tree = actual['host']
                        assert {'value', 'probability', 'resolution', 'results', 'facts'} <= tree.keys(), name
                        assert 'answer' not in tree, name + ': duplicate answer alias'
                        answer = {'function': document['verb'], **{key: tree[key] for key in
                            ('value', 'probability', 'pick', 'index', 'resolution') if key in tree}}
                        def fields(results, facts, answer):
                            return {'host': {'results': results if isinstance(results, list) else [results],
                                             'facts': facts, 'call_result': {'answer': answer}}}
                        wanted.append(fields(expected['results'], expected['facts'], expected.get('answer', expected.get('call_result', {}).get('answer'))))
                        observed.append(fields(tree['results'], tree['facts'], answer))
                        expected_fields = canonical(wanted, expected_oracle=True, absent_metadata=True)
                        actual_fields = canonical(observed, ((str(home), '$FOLDER'), (str(root), '$ROOT')),
                                                  absent_metadata=True)
                        assert expected_fields == actual_fields, name + ':\n' + difference(
                            expected_fields, actual_fields)
                finally:
                    backend.close()


def main():
    root = Path(sys.argv[1])
    project = root / 'libraries/rust/consumer'
    crate = root / 'crates/thinkthen'
    if len(sys.argv) > 2:
        installed = Path(sys.argv[2])
        crate = installed / 'crate'
        shutil.copytree(project, installed / 'consumer', dirs_exist_ok=True,
                        ignore=shutil.ignore_patterns('target'))
        project = installed / 'consumer'
        shutil.copyfile(root / 'crates/thinkthen/tests/packet_oracle/dump.rs',
                        project / 'src/dump.rs')
        shutil.copyfile(root / 'crates/thinkthen/tests/packet_oracle/dump.rs',
                        project / 'tests/settled/native.rs')
        manifest = tomllib.loads((project / 'Cargo.toml').read_text())
        manifest['dependencies']['thinkthen']['path'] = str(crate)
        manifest['dev-dependencies']['conformance-backend']['path'] = str(root / 'conformance/backend')
        (project / 'Cargo.toml').write_text('\n'.join(
            json.dumps(key) + ' = ' + toml_value(value)
            for key, value in manifest.items()) + '\n')
    package = tomllib.loads((crate / 'Cargo.toml').read_text())['package']
    compiler = package['rust-version']
    if isinstance(compiler, dict) and compiler.get('workspace'):
        compiler = tomllib.loads((root / 'Cargo.toml').read_text())['workspace']['package']['rust-version']
    target = Path(os.environ.get('CARGO_TARGET_DIR', str(root / 'libraries/rust/target')))
    binary = target / 'debug/thinkthen-rust-consumer'
    binary.parent.mkdir(parents=True, exist_ok=True)
    sys.path.insert(0, str(root / 'conformance/children'))
    from packet_dump import run
    compiler_env = {'RUSTUP_TOOLCHAIN': compiler}
    profile = os.environ.get('THINKTHEN_TEST_PROFILE', 'routine')
    if profile != 'stress':
        subprocess.run(['cargo', 'build', '--locked', '--offline', '--manifest-path',
                        str(project / 'Cargo.toml'), '--target-dir', str(target)],
                       env={**os.environ, **compiler_env}, check=True)
        if run('rust', [str(binary)], root):
            return True
        built = subprocess.run(['cargo', 'test', '--no-run', '--locked', '--offline',
                                '--manifest-path', str(project / 'Cargo.toml'),
                                '--target-dir', str(target), '--test', 'settled', '--message-format=json'],
                               env={**os.environ, **compiler_env}, check=True,
                               capture_output=True, text=True)
        artifacts = [json.loads(line) for line in built.stdout.splitlines() if line.startswith('{')]
        workers = [item['executable'] for item in artifacts
                   if item.get('reason') == 'compiler-artifact' and item.get('executable')
                   and item['target']['name'] == 'settled']
        assert len(workers) == 1, 'installed settled-test executable is missing'
        value_tree_cases(root, Path(workers[0]))
    if len(sys.argv) > 2 or profile == 'routine':
        command = ['cargo', 'test', '--locked', '--offline', '--manifest-path',
                   str(project / 'Cargo.toml'), '--target-dir', str(target)]
        subprocess.run([*command, '--test', 'sessions', '--test', 'settled', '--test', 'call', '--', '--test-threads=1'],
                       env={**os.environ, **compiler_env}, check=True)
        if profile == 'stress':
            subprocess.run([*command, '--features', 'session-stress', '--test', 'sessions',
                            'repeated_session_contention_never_loses_wake_or_settlement',
                            '--', '--exact', '--test-threads=1'],
                           env={**os.environ, **compiler_env}, check=True)
    return False


if __name__ == '__main__':
    sys.exit(main())
