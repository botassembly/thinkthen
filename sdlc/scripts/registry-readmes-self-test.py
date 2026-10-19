#!/usr/bin/env python3
"""Exercise the package-text release boundary with positive and negative archives."""
import importlib.util
import ctypes
import signal
import io
import os
import sys
import json
from pathlib import Path
import tarfile
import tempfile
import shutil
import subprocess
import struct
import yaml
import zipfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('registry_readmes', Path(__file__).with_name('registry-readmes.py'))
readmes = importlib.util.module_from_spec(spec)
spec.loader.exec_module(readmes)

sys.path.insert(0, str(readmes.REPO))
from conformance.source_fixtures import stage_checkout


REAL_RUN_BUILD = readmes.run_build


def fixture_build(command, work, env, input):
    # Synthetic compilers test permits and cleanup without depending on host load.
    return REAL_RUN_BUILD(command, work, dict(env, GITHUB_ACTIONS='true',
                                            RUNNER_ENVIRONMENT='github-hosted'), input)


def archive(path, files):
    with tarfile.open(path, 'w:gz') as package:
        for name, data in files.items():
            entry = tarfile.TarInfo(name)
            entry.size = len(data)
            if name.endswith('/configure'): entry.mode = 0o755
            package.addfile(entry, io.BytesIO(data))


def cli_root(root):
    for name in ('sdlc/scripts/registry-readmes.py', 'README.md', 'specification/request.schema.json'):
        target = root / name; target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(readmes.REPO / name, target)
    return root / 'sdlc/scripts/registry-readmes.py'


def cli_examples(runner, packages, family):
    return subprocess.run([sys.executable, str(runner), 'examples', str(packages), '--family', family],
                          capture_output=True, text=True)


class PackageText(unittest.TestCase):
    def test_c_owned_calls_keep_exact_answers_and_do_not_read_nested_values(self):
        calls = [{'answer': {'function': function, 'value': values}, 'facts': {'records': 3}}
                 for function, values in [('decide', [True, False, True]),
                                          ('choose', ['billing', 'shipping', 'account'])]]
        readmes.check_c_calls('\n'.join(map(json.dumps, calls)))
        for replacement in (False, [1, 0, 1], [True, False, None], {'value': [True, False, True]}, '\"value\":[true,false,true]'):
            changed = json.loads(json.dumps(calls))
            changed[0]['answer']['value'] = replacement
            changed[0]['value'] = {'nested': [True, False, True]}
            with self.assertRaises(readmes.Refusal):
                readmes.check_c_calls('\n'.join(map(json.dumps, changed)))

    def test_public_package_uses_reviewed_text(self):
        with tempfile.TemporaryDirectory() as scratch:
            package = Path(scratch) / 'thinkthen.tgz'
            definition = json.loads((readmes.REPO / 'libraries/typescript/package.json').read_text())
            files = {'package/README.md': readmes.source('typescript').read_bytes(),
                     'package/package.json': json.dumps(definition).encode()}
            archive(package, files)
            readmes.check_package('typescript', package)
            plants = [
                (b'ADR 0047', 'adr'),
                (b'ticket 0578', 'ticket'),
                (b'sdlc/planning/python.md', 'internal-path'),
                (b'thinkthen-python', 'internal-crate'),
                (b'thinkthen-core', 'internal-project'),
                (b'pip install thinkthen==0.1.2', 'stale-version'),
                (b'ThinkThen release 0.1.2', 'stale-version'),
                (b'gem install thinkthen -v 0.1.2', 'stale-version'),
                (b'npm install thinkthen@0.1.2', 'stale-version'),
                (b'Unreviewed wording', 'readme-source-mismatch'),
            ]
            for text, reason in plants:
                with self.subTest(reason=reason, text=text):
                    archive(package, dict(files, **{'package/README.md': text}))
                    with self.assertRaisesRegex(readmes.Refusal, reason):
                        readmes.check_package('typescript', package)
            for text, reason in [('ADR 0047', 'adr'), ('New public description', 'description-source-mismatch')]:
                with self.subTest(description=text):
                    changed = dict(definition, description=text)
                    archive(package, dict(files, **{'package/package.json': json.dumps(changed).encode()}))
                    with self.assertRaisesRegex(readmes.Refusal, reason):
                        readmes.check_package('typescript', package)

    def test_python_metadata_terminator_preserves_approved_body(self):
        with tempfile.TemporaryDirectory() as scratch:
            package = Path(scratch) / 'thinkthen.whl'
            body = readmes.source('python').read_bytes()
            header = ('Metadata-Version: 2.1\nSummary: ' +
                      readmes.description('python') + '\n\n').encode()
            for changed, cause in [(body, None), (body + b'\n', None),
                                   (body + b'\n\n', 'readme-needs-under-40-lines'),
                                   (body.replace(b'The library is free', b'The library costs money'),
                                    'readme-source-mismatch')]:
                with self.subTest(cause=cause):
                    with zipfile.ZipFile(package, 'w') as bundle:
                        bundle.writestr('thinkthen.dist-info/METADATA', header + changed)
                    if cause:
                        with self.assertRaisesRegex(readmes.Refusal, cause):
                            readmes.check_package('python', package)
                    else:
                        readmes.check_package('python', package)

    def test_python_metadata_cli_rejects_changed_words_and_long_source(self):
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            runner = cli_root(root)
            for name in ('crates/thinkthen/Cargo.toml', 'libraries/python/pyproject.toml'):
                target = root / name
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(readmes.REPO / name, target)
            page = readmes.source('python', root)
            body = readmes.source('python').read_bytes()
            page.write_bytes(body)
            header = ('Metadata-Version: 2.1\nSummary: ' +
                      readmes.description('python') + '\n\n').encode()
            changed = body.replace(b'The library is free', b'The library costs money')
            for suffix in ('.whl', '.tar.gz'):
                package = root / ('thinkthen' + suffix)
                for source_body, packed, cause in (
                    (body, body, None),
                    (body, body + b'\n', None),
                    (body, changed, 'readme-source-mismatch'),
                    (body, changed + b'\n', 'readme-needs-under-40-lines'),
                    (body, body + b'\n\n', 'readme-needs-under-40-lines'),
                    (body + b'\n', body + b'\n\n', 'readme-needs-under-40-lines'),
                ):
                    with self.subTest(suffix=suffix, cause=cause):
                        page.write_bytes(source_body)
                        if suffix == '.whl':
                            with zipfile.ZipFile(package, 'w') as bundle:
                                bundle.writestr('thinkthen.dist-info/METADATA', header + packed)
                        else:
                            archive(package, {'thinkthen/PKG-INFO': header + packed})
                        done = subprocess.run([sys.executable, str(runner), 'python', str(package)],
                                              capture_output=True, text=True)
                        self.assertEqual(done.returncode, 1 if cause else 0, done.stderr)
                        if cause:
                            self.assertIn(cause, done.stderr)

    def test_npm_assembles_public_readme_and_refuses_adr(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch)
            source = tree / 'npm'
            source.mkdir()
            for name in ('package.json', 'README.md', 'LICENSE'):
                shutil.copyfile(readmes.REPO / 'libraries/typescript' / name, source / name)
            def pack():
                subprocess.run(['npm', 'pack', '--offline', '--ignore-scripts', '--silent',
                                '--cache', str(tree / 'cache'), '--pack-destination', str(tree)],
                               cwd=source, check=True, capture_output=True)
                return tree / ('thinkthen-' + readmes.version() + '.tgz')
            readmes.check_package('typescript', pack())
            (source / 'README.md').write_text('ADR 0047\n')
            with self.assertRaisesRegex(readmes.Refusal, 'adr'):
                readmes.check_package('typescript', pack())

    def test_windows_command_assembles_root_public_page(self):
        spec = importlib.util.spec_from_file_location('command', readmes.REPO / 'sdlc/scripts/release-windows-command.py')
        command = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(command)
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch)
            binary = tree / 'thinkthen.exe'
            data = bytearray(128)
            data[:2] = b'MZ'
            struct.pack_into('<I', data, 60, 64)
            data[64:68] = b'PE\0\0'
            for offset, value in ((68, 0x8664), (86, 2), (88, 0x20B)):
                struct.pack_into('<H', data, offset, value)
            binary.write_bytes(data)
            package = tree / ('thinkthen-' + readmes.version() + '-x86_64-pc-windows-msvc.zip')
            command.pack(binary, package)
            readmes.check_package('command', package)
            with zipfile.ZipFile(package, 'w') as bundle:
                bundle.writestr('thinkthen.exe', data)
                bundle.writestr('README.md', b'ADR 0047')
            with self.assertRaisesRegex(readmes.Refusal, 'shared-opening'):
                readmes.check_package('command', package)

    def test_dependency_versions_are_allowed(self):
        readmes.check_text(b'Rust 1.95.0; DuckDB 1.5.6; Zig 0.15.2; JDK 22; jsonlite 2.0.0', '0.2.0')
        readmes.check_text(b'tar -xzf thinkthen-c-0.2.0-TARGET.tar.gz', '0.2.0')

    def test_missing_readme_refuses(self):
        with tempfile.TemporaryDirectory() as scratch:
            package = Path(scratch) / 'thinkthen.tgz'
            archive(package, {'package/package.json': b'{}'})
            with self.assertRaisesRegex(readmes.Refusal, 'missing-or-repeated-member'):
                readmes.check_package('typescript', package)

    def test_example_checks_actual_page_and_refuses_single_function(self):
        filename, code, output = readmes.example('python')
        self.assertEqual(filename, 'sample.py')
        self.assertIn('engine.decide(', code)
        self.assertIn('engine.choose(', code)
        self.assertIsNone(output)
        self.assertIn('assert is_refund is expected', code)
        self.assertIn('assert team == expected', code)
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            page = root / 'libraries/python/README-PUBLIC.md'
            page.parent.mkdir(parents=True)
            page.write_text('```python\nprint(engine.decide("Refund?", "Refund please.").value)\n```\n')
            with self.assertRaisesRegex(readmes.Refusal, 'example-needs-two-functions: python'):
                readmes.example('python', root)

    def test_database_routes_through_release_cli(self):
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            runner = cli_root(root)
            packages = root / 'packages'; packages.mkdir()
            contract = json.loads((root / 'specification/request.schema.json').read_text())
            functions = [variant['const'] for variant in contract['$defs']['RequestFunction']['oneOf']]
            for family in ('sqlite', 'postgresql', 'duckdb'):
                page = readmes.source(family, root); page.parent.mkdir(parents=True)
                admitted = 'missing-or-ambiguous-example-package: ' + family
                cases = []
                for suffix in ('', '_details'):
                    cases.append((''.join("SELECT thinkthen_" + function + suffix
                        + "('thinkthen_unknown(', 'i');\n" for function in functions), admitted))
                for quote in ('"', '`', '[') if family == 'sqlite' else ('"',):
                    close = ']' if quote == '[' else quote
                    cases.append(('SELECT ' + quote + 'thinkthen_decide' + close + " /* call */ ('q', 'i'); "
                        + 'SELECT ' + quote + 'thinkthen_choose_details' + close + "('q', 'i');", admitted))
                    cases.append(('SELECT ' + quote + 'thinkthen_unknown' + close + "('q', 'i');",
                                  'unsupported-database-example-route: thinkthen_unknown'))
                    cases.append(('SELECT 1 AS ' + quote + 'thinkthen_decide(' + close
                        + ', 2 AS ' + quote + 'thinkthen_choose(' + close + ';',
                        'example-needs-two-functions: ' + family))
                helpers = readmes.DATABASE_HELPERS[family]
                for helper in helpers:
                    one = "SELECT thinkthen_decide_details('q', " + helper + "('i'));"
                    cases.append((one, 'example-needs-two-functions: ' + family))
                    cases.append((one + "SELECT thinkthen_choose_details('q', " + helper + "('i'));", admitted))
                for prose in ("SELECT 'thinkthen_decide(' AS prose; -- thinkthen_choose('q', 'i');",
                              'SELECT 1 AS "thinkthen_decide(", 2 AS "thinkthen_choose(";',
                              '/* outer /* inner */ thinkthen_decide( thinkthen_choose( */',
                              "SELECT E'escaped \\\' thinkthen_decide( thinkthen_choose(';",
                              "SELECT $body$thinkthen_decide( thinkthen_choose($body$;"):
                    cases.append((prose, 'example-needs-two-functions: ' + family))
                for route in ('thinkthen_call', 'thinkthen_complete', 'thinkthen_decide_complete',
                              'thinkthen_unknown', 'tt_unknown_details', '"thinkthen_complete" /* call */'):
                    cases.append(("SELECT thinkthen_decide('q', 'i'); SELECT thinkthen_choose_details('q', 'i'); SELECT "
                        + route + "('q', 'i');", 'unsupported-database-example-route: '
                        + route.split()[0].strip('"')))
                for code, error in cases:
                    with self.subTest(family=family, code=code):
                        page.write_text('```sql\n' + code + '\n```\n')
                        done = cli_examples(runner, packages, family)
                        self.assertNotEqual(done.returncode, 0)
                        self.assertIn(error, done.stderr)

    def test_typed_host_calls_through_release_cli(self):
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            runner = cli_root(root)
            packages = root / 'packages'; packages.mkdir()
            functions = [variant['const'] for variant in json.loads(
                (root / 'specification/request.schema.json').read_text())['$defs']['RequestFunction']['oneOf']]
            cases = [('csharp', readmes.source('csharp').read_text(), True),
                     ('csharp', ''.join('engine.' + name.title() + 'Async(q, input);\n'
                                      for name in functions), True),
                     ('crate', 'Question::decide("q"); Question::choose::<Team>("q");', True),
                     ('crate', 'Question::decide("q"); Question::choose::<Option<Team>>("q");', True)]
            for family, current in [('csharp', 'engine.DecideAsync(q, input);'),
                                    ('python', 'engine.decide(q, input);')]:
                for suffix in ('Answer', 'AnswerAsync', 'ValueAsync', 'Complete', 'Unknown', '_Answer'):
                    cases.append((family, current + ' engine.Choose' + suffix + '(q, input);', False))
                for false_call in ('"engine.ChooseAsync(q, input)"',
                                   '// engine.ChooseAsync(q, input)',
                                   '/* engine.ChooseAsync(q, input) */',
                                   'engine.ChooseAsync;', 'engine.UnknownAsync(q, input);'):
                    cases.append((family, current + ' ' + false_call, False))
            for code in ('engine.decide(q, input); # engine.choose(q, input)',
                         'engine.decide(q, input); """engine.choose(q, input)"""',
                         'engine.decide(q, input); "engine.choose(q, input)"'):
                cases.append(('python', code, False))
            cases.append(('go', 'client.Decide(q, input); client.ChooseAsync(q, input);', False))
            cases.extend([
                ('ada', "for I in Texts'Range loop Inputs.Calls.Decide(q, i); "
                        "Inputs.Calls.Choose(q, i); Put_Line(Long_Float'Image(p)); end loop;", True),
                ('ada', "C := 'x'; Inputs.Calls.Decide(q, i); C := '''; Inputs.Calls.Choose(q, i);", True),
                ('crate', "fn sample<'a, 'b>() { Question::decide(q); Question::choose::<&'a Team>(q); }", True),
                ('crate', 'Question::decide(q); /* outer /* inner */ " */ Question::choose(q);', True),
                ('crate', 'Question::decide(q); "continued \\\nnot a call"; Question::choose(q);', True),
            ])
            for family, current, decoys in (
                    ('ada', 'Inputs.Calls.Decide(q, i);', (
                        'Inputs.Calls.ChooseUnknown(q, i);', 'Inputs.Calls.Choose;',
                        'Inputs.Calls.Decide(q, i);', '-- Inputs.Calls.Choose(q, i);',
                        '"Inputs.Calls.Choose(q, i)"', '"quote "" Inputs.Calls.Choose(q, i) """',
                        "'(' & 'C' & 'h' & 'o' & 'o' & 's' & 'e'")),
                    ('crate', 'Question::decide(q);', (
                        'Question::decide(q);', 'Question::chooseUnknown(q);',
                        '// Question::choose(q);', '/* Question::choose(q); */',
                        '/* outer /* inner */ Question::choose(q); */',
                        '/* outer /* middle /* inner */ */ Question::choose(q); */',
                        '"Question::choose(q)"', 'r#"Question::choose(q)"#',
                        '"continued \\\nQuestion::choose(q)"',
                        'br##"Question::choose(q)"##', "'x'", "'\\u{27}'"))):
                cases.extend((family, current + '\n' + decoy, False) for decoy in decoys)
            for function in functions:
                other = 'choose' if function == 'decide' else 'decide'
                cases.append(('csharp', 'engine.' + function.title() + 'Async<Input<Record>>(q, input); '
                              + 'engine.' + other.title() + 'Async(q, input);', True))
            for false_call in ('engine.ChooseAsyncUnknown<Input>(q, input);',
                               'engine.ChooseAnswerAsync<Input>(q, input);',
                               '@"engine.ChooseAsync<Input>(q, input)"',
                               '"""engine.ChooseAsync<Input>(q, input)"""'):
                cases.append(('csharp', 'engine.DecideAsync(q, input); ' + false_call, False))
            for family, current, decoy in (
                    ('c', 'thinkthen_decide(q, input);', 'thinkthen_choose(q, input)'),
                    ('cobol', 'CALL "TT_DECIDE" USING q input.', 'CALL "TT_CHOOSE" USING q input.')):
                cases.append((family, readmes.source(family).read_text(), True))
                for prose in ('/* ' + decoy + ' */', '// ' + decoy,
                              '"' + decoy.replace('"', '\\"') + '"') if family == 'c' else (
                                  '*> ' + decoy, 'DISPLAY \'CALL "TT_CHOOSE" USING q input.\'.'):
                    cases.append((family, current + '\n' + prose, False))
            named = 'thinkthen_named_request_v1 chosen = {.function = {"choose",6}};'
            called = 'thinkthen_run_call(engine,&chosen,0,0,0,&result,&error);'
            cases.append(('c', 'thinkthen_decide(q,input);' + named + called, True))
            for decoy in (named, named + '/* ' + called + ' */', named + '// ' + called,
                          named + '"' + called + '"', named + called.replace('&chosen', '&other')):
                cases.append(('c', 'thinkthen_decide(q,input);' + decoy, False))
            for family, code, admitted in cases:
                with self.subTest(family=family, code=code):
                    page = readmes.source(family, root); page.parent.mkdir(parents=True, exist_ok=True)
                    page.write_text(code if code.startswith('# ThinkThen') else
                                    '```' + readmes.SAMPLES[family][0] + '\n' + code + '\n```\n')
                    done = cli_examples(runner, packages, family)
                    self.assertNotEqual(done.returncode, 0)
                    reason = ('missing-or-ambiguous-example-package' if admitted else 'example-needs-two-functions')
                    self.assertIn(reason + ': ' + family, done.stderr)

    def test_python_assertions_execute_and_refuse_wrong_answers(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch); root = tree / 'root'; packages = tree / 'packages'
            packages.mkdir()
            runner = cli_root(root)
            for name in ('rust-toolchain.toml', 'site/recordings/thinkthen.jsonl',
                         'crates/thinkthen/Cargo.toml', 'libraries/python/pyproject.toml'):
                target = root / name; target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(readmes.REPO / name, target)
            page = readmes.source('python', root)
            approved = readmes.source('python').read_text()
            module = ('from types import SimpleNamespace\n'
                      'class Engine:\n'
                      ' def __enter__(self): return self\n'
                      ' def __exit__(self, *args): pass\n'
                      ' def decide(self, question, text):\n'
                      '  return SimpleNamespace(value=not text.startswith("Thanks"), probability=.97)\n'
                      ' def choose(self, question, text, options):\n'
                      '  return SimpleNamespace(value="billing" if "invoice" in text else "shipping" if "parcel" in text else "account")\n'
                      ' def score(self, question, text): return SimpleNamespace(value=3)\n')
            cases = [('', True), ('        assert is_refund is not expected\n', False),
                     ('        assert engine.score(refund, text).value == 3\n', True),
                     ('        assert engine.score(refund, text).value == 4\n', False),
                     ('        assert engine.decide(refund, text).probability == .97\n', True),
                     ('        assert engine.decide(refund, text).probability == .01\n', False)]
            for assertion, passes in cases:
                with self.subTest(assertion=assertion):
                    text = approved.replace('        assert is_refund is expected\n',
                                            assertion or '        assert is_refund is expected\n')
                    page.write_text(text)
                    release = readmes.version(root)
                    dist = 'thinkthen-' + release + '.dist-info/'
                    wheel = packages / ('thinkthen-' + release + '-py3-none-any.whl')
                    summary = readmes.description('python', root)
                    with zipfile.ZipFile(wheel, 'w') as bundle:
                        bundle.writestr('thinkthen/__init__.py', module)
                        bundle.writestr(dist + 'METADATA', 'Metadata-Version: 2.1\nName: thinkthen\nVersion: '
                                        + release + '\nSummary: ' + summary + '\n\n' + text)
                        bundle.writestr(dist + 'WHEEL', 'Wheel-Version: 1.0\nGenerator: fixture\nRoot-Is-Purelib: true\nTag: py3-none-any\n')
                        bundle.writestr(dist + 'RECORD', '')
                    done = cli_examples(runner, packages, 'python')
                    self.assertEqual(done.returncode == 0, passes, done.stderr)
                    if not passes:
                        self.assertIn('example-did-not-run: python', done.stderr)
                        self.assertIn('AssertionError', done.stderr)

    def test_archive_replay_preserves_package_roots_with_vendored_readmes(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch); packages = tree / 'packages'; packages.mkdir()
            archive(packages / 'thinkthen_0.2.0.tar.gz', {
                'thinkthen/README.md': readmes.source('r').read_bytes(),
                'thinkthen/src/rust/vendor/dependency/README.md': b'Vendored dependency',
                'thinkthen/configure': b'#!/bin/sh\nexit 0\n',
                'thinkthen/DESCRIPTION': (readmes.REPO / 'libraries/r/thinkthen/DESCRIPTION').read_bytes()})
            installed = readmes.unpack_example_package('r', packages, tree / 'installed')
            self.assertTrue((installed / 'DESCRIPTION').is_file())
            self.assertTrue(os.access(installed / 'configure', os.X_OK))
            self.assertEqual(installed, tree / 'installed/thinkthen')
            prefix = 'thinkthen-' + readmes.version()
            archive(packages / (prefix + '.crate'), {
                prefix + '/README.md': readmes.source('crate').read_bytes(),
                prefix + '/src/transforms/README.md': b'Bundled transform documentation',
                prefix + '/Cargo.toml': (readmes.REPO / 'crates/thinkthen/Cargo.toml').read_bytes()})
            command_prefix = 'thinkthen-cli-' + readmes.version()
            command_source = packages / (command_prefix + '.crate')
            archive(command_source, {
                command_prefix + '/README.md': readmes.source('command').read_bytes(),
                command_prefix + '/Cargo.toml': (readmes.REPO / 'crates/thinkthen-cli/Cargo.toml').read_bytes()})
            readmes.check_package('crate', command_source)
            crate = readmes.unpack_example_package('crate', packages, tree / 'installed-crate')
            self.assertEqual(crate, tree / 'installed-crate' / prefix)
            self.assertTrue((crate / 'Cargo.toml').is_file())

    def test_duckdb_replay_loads_the_installed_source_member(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch); packages = tree / 'packages'; packages.mkdir()
            archive(packages / 'thinkthen-duckdb-0.2.0-x86_64-unknown-linux-gnu.tar.gz', {
                'README.md': readmes.source('duckdb').read_bytes(),
                'd4355d01c5/linux_amd64/thinkthen.duckdb_extension': b'preview',
                'v1.5.6/linux_amd64/thinkthen.duckdb_extension': b'stable'})
            work = tree / 'work'; work.mkdir()
            def run(command, cwd, env, input, timeout):
                if '-c' in command:
                    return subprocess.CompletedProcess(command, 0, '069cc9f9b5\n', '')
                self.assertIn('-bail', command)
                self.assertIn('/v1.5.6/linux_amd64/thinkthen.duckdb_extension', input)
                self.assertNotIn('/d4355d01c5/', input)
                return subprocess.CompletedProcess(command, 0, '', '')
            with patch.object(readmes, 'run_process', side_effect=run):
                readmes.run_example('duckdb', packages, work)

    def test_all_usage_blocks_are_selected_in_order(self):
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            page = readmes.source('r', root)
            page.parent.mkdir(parents=True)
            page.write_text('```r\ninstall.packages("thinkthen")\n```\n'
                            '```r\ntt_decide("Q?", "one")\n```\n'
                            '```r\ntt_choose("Q?", "two")\n```\n')
            samples = readmes.examples('r', root)
            self.assertEqual(len(samples), 2)
            self.assertIn('tt_decide(', samples[0][1])
            self.assertIn('tt_choose(', samples[1][1])

    def test_replay_collects_every_broken_usage_block(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch)
            root = tree / 'root'; (root / 'site/recordings').mkdir(parents=True)
            shutil.copyfile(readmes.REPO / 'rust-toolchain.toml', root / 'rust-toolchain.toml')
            shutil.copyfile(readmes.REPO / 'site/recordings/thinkthen.jsonl', root / 'site/recordings/thinkthen.jsonl')
            (root / 'README.md').write_text('```sh\nthinkthen decide fail\n```\n'
                                          '```sh\nthinkthen choose fail\n```\n')
            packages = tree / 'packages'; packages.mkdir()
            package = packages / ('thinkthen-' + readmes.version() + '-x86_64-unknown-linux-musl.tar.gz')
            archive(package, {'README.md': b'Wrong public opening',
                              'thinkthen': b'#!/bin/sh\n[ "$2" != fail ] || exit 77\nprintf "fixture\\n"\n'})
            with patch.object(readmes, 'PUBLIC_FAMILIES', ('command',)):
                with self.assertRaisesRegex(readmes.Refusal, 'example-did-not-run: command') as rejected:
                    readmes.check_examples(packages, root)
                self.assertEqual(str(rejected.exception).count('example-did-not-run: command'), 2)
                self.assertEqual(str(rejected.exception).count('command packed text: shared-opening'), 2)
                self.assertIn('command block 1:', str(rejected.exception))
                self.assertIn('command block 2:', str(rejected.exception))

    def test_preserved_root_checks_only_opening_and_versions(self):
        for family in ('command', 'packagist'):
            with self.subTest(family=family), tempfile.TemporaryDirectory() as scratch:
                root = Path(scratch)
                (root / 'crates/thinkthen').mkdir(parents=True)
                shutil.copyfile(readmes.REPO / 'crates/thinkthen/Cargo.toml',
                                root / 'crates/thinkthen/Cargo.toml')
                approved = readmes.source(family).read_text()
                self.assertGreaterEqual(len(approved.splitlines()), 40)
                (root / 'README.md').write_text(approved)
                readmes.check_readme(family, approved.encode(), root)
                body = approved + '\nADR 0047; ticket 0578; sdlc/planning/a.md\n'
                readmes.check_readme(family, body.encode(), root)
                for text, cause in [(approved.replace(readmes.OPENING, 'Wrong pitch'), 'shared-opening'),
                                    (approved + '\nThinkThen release 0.1.2\n', 'stale-version')]:
                    (root / 'README.md').write_text(text)
                    with self.assertRaisesRegex(readmes.Refusal, cause):
                        readmes.check_readme(family, text.encode(), root)

    def test_package_pages_keep_whole_body_rules(self):
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            page = readmes.source('python', root)
            page.parent.mkdir(parents=True)
            (root / 'crates/thinkthen').mkdir(parents=True)
            shutil.copyfile(readmes.REPO / 'crates/thinkthen/Cargo.toml',
                            root / 'crates/thinkthen/Cargo.toml')
            for plant, cause in [('ADR 0047', 'adr'), ('ticket 0578', 'ticket'),
                                 ('sdlc/planning/a.md', 'internal-path'), ('\n' * 40, 'readme-needs-under-40-lines')]:
                text = '# ThinkThen\n\n' + readmes.OPENING + '\n\n' + plant + '\n'
                page.write_text(text)
                with self.assertRaisesRegex(readmes.Refusal, cause):
                    readmes.check_readme('python', text.encode(), root)

    def test_composer_replays_root_cli_without_a_php_example(self):
        self.assertEqual(readmes.examples('packagist'), readmes.examples('command'))
        samples = readmes.examples('command')
        self.assertEqual(len(samples), 2)
        self.assertIn('< message.txt', samples[0][1])
        self.assertIn('thinkthen-first-run', samples[1][1])

    def test_preserved_root_replay_uses_real_demo_and_first_run_fixtures(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch); packages = tree / 'packages'; packages.mkdir()
            demo = readmes.REPO / 'demos/01-refund-gate'
            first_run = readmes.REPO / 'demos/27-test-with-no-network'
            script = ('#!' + sys.executable + '\nimport os, pathlib, sys\n'
                      'text = sys.stdin.read()\n'
                      'if "--replay" in sys.argv:\n'
                      '    assert text == ' + repr((first_run / 'report.txt').read_text()) + '\n'
                      '    assert pathlib.Path("recording/thinkthen.jsonl").is_file()\n'
                      'else:\n'
                      '    assert text == ' + repr((demo / 'message.txt').read_text()) + '\n'
                      '    assert pathlib.Path(os.environ["THINKTHEN_CACHE"], "thinkthen.jsonl").read_bytes() == '
                      + repr((demo / 'recording/thinkthen.jsonl').read_bytes()) + '\n'
                      'print("true")\n')
            archive(packages / ('thinkthen-' + readmes.version() + '-x86_64-unknown-linux-musl.tar.gz'),
                    {'README.md': readmes.source('command').read_bytes(), 'thinkthen': script.encode()})
            archive(packages / 'thinkthen-first-run.tar.gz', {
                'thinkthen-first-run/report.txt': (first_run / 'report.txt').read_bytes(),
                'thinkthen-first-run/recording/thinkthen.jsonl':
                    (first_run / 'recording/thinkthen.jsonl').read_bytes()})
            for family in ('command', 'packagist'):
                for index in range(len(readmes.examples(family))):
                    with self.subTest(family=family, index=index):
                        work = tree / (family + str(index)); work.mkdir()
                        readmes.run_example(family, packages, work, sample_index=index)

    def test_release_replays_every_public_package_on_a_prepared_host(self):
        jobs = yaml.safe_load((readmes.REPO / '.github/workflows/release.yml').read_text())['jobs']
        steps = jobs['smoke']['steps']
        replay = next(step for step in steps if 'public-examples' in step.get('run', ''))
        self.assertEqual(replay['if'], "matrix.target == 'x86_64-unknown-linux-gnu' || runner.os == 'macOS'")
        self.assertLess(next(i for i, step in enumerate(steps) if 'host-setup' in step.get('run', '')), steps.index(replay))
        script = (readmes.REPO / 'sdlc/scripts/release-workflow').read_text()
        routing = script.split('\npublic-examples)\n')[1].split('\ncrate-smoke)')[0]
        families = set(readmes.re.findall(r'--family ([a-z-]+)', routing))
        self.assertEqual(families, set(readmes.PUBLIC_FAMILIES))
        self.assertIn('flutter', families)
        linux = routing.split('*-apple-darwin)')[0]
        self.assertEqual(readmes.re.findall(r'--family ([a-z-]+)', linux)[-1], 'postgresql')
        self.assertNotIn('registry-readmes.py examples', script.split('\ndraft)\n')[1].split('\n\t;;')[0])
        self.assertIn('smoke', jobs['draft']['needs'])
        for name, path in [('r-package', 'r-dist'), ('crate-package', 'crate-dist')]:
            self.assertTrue(any(step.get('with', {}).get('name') == name
                                and step['with']['path'] == path for step in steps))
        # Execute the actual workflow staging step with the runner replaced at its CLI boundary.
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            for name in ('r-dist', 'crate-dist', 'smoke-files', 'sdlc/scripts'):
                (root / name).mkdir(parents=True)
            (root / 'r-dist/thinkthen.tar.gz').write_text('R package')
            (root / 'crate-dist/thinkthen.crate').write_text('Rust package')
            runner = root / 'sdlc/scripts/release-workflow'
            runner.write_text('#!/bin/sh\nset -eu\n[ "$1" = public-examples ]\n[ -f "$3/thinkthen.crate" ]\nif [ "$2" = x86_64-unknown-linux-gnu ]; then [ -f "$3/thinkthen.tar.gz" ]; fi\n')
            runner.chmod(0o755)
            for target in ('x86_64-unknown-linux-gnu', 'aarch64-apple-darwin'):
                subprocess.run(['/bin/sh', '-eu', '-c', replay['run']], cwd=root,
                               env={'PATH': '/usr/bin:/bin', 'TARGET': target}, check=True)
            stage_checkout(readmes.REPO, root)
            home = root / 'home'
            prepared = [root / 'resolved/ruby/bin']
            sqlite = root / 'sqlite'
            duckdb_pin = readmes.re.search(r'(?m)^DUCKDB_ROUTINE_RELEASE=(.*)$',
                                          (root / 'databases/duckdb/tools/version.env').read_text())[1]
            duckdb = root / 'databases/duckdb/build/community' / duckdb_pin / 'build/release'
            wrong = root / 'wrong'; wrong.mkdir()
            host_name = wrong / 'uname'
            host_name.write_text('#!/bin/sh\ncase $1 in -s) echo Linux ;; *) echo x86_64 ;; esac\n')
            host_name.chmod(0o755)
            for binary, folders in [('ruby', prepared), ('sqlite3', [sqlite]), ('duckdb', [duckdb])]:
                for folder in folders + [wrong]:
                    folder.mkdir(parents=True, exist_ok=True)
                    executable = folder / binary
                    executable.write_text('#!/bin/sh\nprintf "' + ('wrong' if folder == wrong else 'prepared') + '\\n"\n')
                    executable.chmod(0o755)
                    if binary == 'ruby' and folder != wrong:
                        (folder.parent / 'thinkthen-toolchain.stamp').touch()
            host = root / 'databases/sqlite/tests/host_sqlite.sh'; host.parent.mkdir(parents=True, exist_ok=True)
            host.write_text('#!/bin/sh\nprintf "%s\\n" ' + readmes.json.dumps(str(sqlite)) + '\n')
            checker = root / 'sdlc/scripts/registry-readmes.py'
            checker.write_text('import subprocess\nfor name in ("ruby", "sqlite3", "duckdb"):\n'
                               '    assert subprocess.check_output([name], text=True) == "prepared\\n", name\n')
            done = subprocess.run(['/bin/sh', str(root / 'sdlc/scripts/release-workflow'),
                                   'public-examples', 'x86_64-unknown-linux-gnu', 'smoke-files'],
                                  env={'PATH': os.pathsep.join((str(prepared[0]), str(wrong), os.environ['PATH'])),
                                       'HOME': str(home)},
                                  capture_output=True, text=True)
            self.assertEqual(done.returncode, 0, done.stdout + done.stderr)

    @unittest.skipUnless(sys.platform in ('linux', 'darwin'), 'compiler replay requires POSIX process groups')
    def test_compilers_lock_without_flock_and_replay_uses_prepared_cache(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch)
            root = tree / 'root'
            (root / 'site/recordings').mkdir(parents=True)
            shutil.copyfile(readmes.REPO / 'rust-toolchain.toml', root / 'rust-toolchain.toml')
            shutil.copyfile(readmes.REPO / 'site/recordings/thinkthen.jsonl', root / 'site/recordings/thinkthen.jsonl')
            packages = tree / 'packages'; packages.mkdir()
            tools = tree / 'tools'; tools.mkdir()
            cache = tree / 'prepared-pub-cache'; cache.mkdir()
            lock = tree / 'compiler.lock'
            for family, language, compiler in [('objective-c', 'objc', 'swift'), ('dart', 'dart', 'dart')]:
                page = readmes.source(family, root)
                page.parent.mkdir(parents=True)
                calls = '[client decide:q]; [client choose:q];' if family == 'objective-c' else 'decide(); choose();'
                page.write_text('```' + language + '\n' + calls + '\n```\n```text\nfixture\n```\n')
                files = {'README.md': readmes.source(family).read_bytes()}
                if family == 'dart':
                    files['pubspec.yaml'] = (readmes.REPO / 'libraries/dart/pubspec.yaml').read_bytes()
                    native = b'fixture native library'
                    target = {'x86_64': 'x86_64', 'arm64': 'aarch64'}.get(os.uname().machine, os.uname().machine) + '-unknown-linux-gnu'
                    files['native-assets.json'] = json.dumps({'assets': {'fixture': {
                        'target': target, 'file': 'libthinkthen.so',
                        'sha256': readmes.hashlib.sha256(native).hexdigest()}}}).encode()
                    archive(packages / ('thinkthen-c-' + readmes.version() + '-x86_64-unknown-linux-gnu.tar.gz'), {
                        'README.md': readmes.source('c').read_bytes(), 'lib/libthinkthen.so': native})
                archive(packages / ('thinkthen-' + family + '-' + readmes.version() + '-x86_64-unknown-linux-gnu.tar.gz'), files)
                tool = tools / compiler
                tool.write_text('#!' + sys.executable + '\n' +
                    'import fcntl, os, pathlib, sys\n' +
                    'with open(' + repr(str(lock)) + ', "a") as lock:\n' +
                    '    try: fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)\n' +
                    '    except BlockingIOError: pass\n' +
                    '    else: sys.exit("compiler ran without lock")\n' +
                    'assert os.environ["CARGO_BUILD_JOBS"] == "2"\n' +
                    ('assert os.environ["PUB_CACHE"] == ' + repr(str(cache)) + '\n' if family == 'dart' else '') +
                    'out = pathlib.Path(sys.argv[-1]) / "bundle/bin/sample" if sys.argv[1:3] == ["build", "cli"] else pathlib.Path(".build/debug/RegistryExample") if sys.argv[1] == "build" else None\n' +
                    'if out:\n' +
                    '    out.parent.mkdir(parents=True, exist_ok=True)\n' +
                    '    out.write_text(' + repr('#!' + sys.executable + '\nimport fcntl\nwith open(' + repr(str(lock)) + ', "a") as lock: fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)\nprint("fixture")\n') + ')\n' +
                    '    out.chmod(0o755)\n')
                tool.chmod(0o755)
                work = tree / family; work.mkdir()
                with self.subTest(family=family), patch.object(readmes, 'run_build', side_effect=fixture_build), \
                        patch.dict(os.environ, {'PATH': str(tools), 'PUB_CACHE': str(cache),
                                               'THINKTHEN_HEAVY_LOCK': str(lock)}, clear=True):
                    readmes.run_example(family, packages, work, root)

    def test_flutter_replay_uses_installed_wrapper_and_core_and_collects_failures(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch); packages = tree / 'packages'; packages.mkdir()
            root = tree / 'root'; (root / 'site/recordings').mkdir(parents=True)
            for name in ('rust-toolchain.toml', 'site/recordings/thinkthen.jsonl'):
                shutil.copyfile(readmes.REPO / name, root / name)
            page = readmes.source('flutter', root); page.parent.mkdir(parents=True)
            page.write_text('```dart\ndecide(); choose(); broken();\n```\n')
            native = b'fixture native library'
            target = {'arm64': 'aarch64'}.get(os.uname().machine, os.uname().machine) + '-unknown-linux-gnu'
            archive(packages / ('thinkthen-flutter-' + readmes.version() + '-x86_64-unknown-linux-gnu.tar.gz'), {
                'flutter/README.md': readmes.source('flutter').read_bytes(),
                'flutter/pubspec.yaml': (readmes.REPO / 'libraries/dart/flutter/pubspec.yaml').read_bytes(),
                'flutter/lib/thinkthen_flutter.dart': b'installed wrapper',
                'flutter/example/README.md': b'Bundled application example'})
            archive(packages / ('thinkthen-dart-' + readmes.version() + '-x86_64-unknown-linux-gnu.tar.gz'), {
                'README.md': readmes.source('dart').read_bytes(),
                'pubspec.yaml': (readmes.REPO / 'libraries/dart/pubspec.yaml').read_bytes(),
                'native-assets.json': json.dumps({'assets': {'fixture': {
                    'target': target, 'file': 'libthinkthen.so',
                    'sha256': readmes.hashlib.sha256(native).hexdigest()}}}).encode()})
            archive(packages / ('thinkthen-c-' + readmes.version() + '-x86_64-unknown-linux-gnu.tar.gz'), {
                'README.md': readmes.source('c').read_bytes(), 'lib/libthinkthen.so': native})
            calls = []
            def build(command, work, env, input):
                calls.append(command)
                spec = yaml.safe_load((work / 'pubspec.yaml').read_text())
                wrapper = Path(spec['dependencies']['thinkthen_flutter']['path'])
                core = Path(spec['dependency_overrides']['thinkthen_dart']['path'])
                self.assertEqual((wrapper / 'lib/thinkthen_flutter.dart').read_bytes(), b'installed wrapper')
                self.assertTrue((core / 'native-assets.json').is_file())
                self.assertEqual(yaml.safe_load((wrapper / 'pubspec.yaml').read_text())['publish_to'], 'none')
                self.assertTrue(spec['hooks']['user_defines']['thinkthen_dart']['offline'])
                return subprocess.CompletedProcess(command, 77 if command[1] == 'build' else 0, '', 'broken Flutter example')
            with patch.object(readmes, 'run_build', side_effect=build):
                with self.assertRaisesRegex(readmes.Refusal, 'broken Flutter example') as error:
                    readmes.check_examples(packages, root, families=('flutter', 'python'))
            self.assertEqual(calls[0][1:], ['pub', 'get', '--offline'])
            self.assertEqual(calls[1][1:3], ['build', 'cli'])
            self.assertIn('flutter block 1: example-did-not-run: flutter', str(error.exception))
            self.assertIn('python', str(error.exception))

    def test_release_refuses_when_examples_cannot_run(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch)
            root = tree / 'root'; root.mkdir()
            runner = cli_root(root)
            families = ('command', 'typescript', 'duckdb', 'flutter')
            for family in families:
                page = readmes.source(family, root)
                page.parent.mkdir(parents=True, exist_ok=True)
                language = readmes.SAMPLES[family][0]
                code = ("SELECT thinkthen_decide('q', 'i'); SELECT thinkthen_choose('q', 'i');"
                        if family == 'duckdb' else 'decide(); choose();')
                page.write_text(readmes.OPENING + '\n\n```' + language + '\n' + code + '\n```\n')
            packages = tree / 'packages'; packages.mkdir()
            done = subprocess.run([sys.executable, str(runner), 'examples', str(packages),
                                   *[arg for family in families for arg in ('--family', family)]],
                                  capture_output=True, text=True)
            self.assertNotEqual(done.returncode, 0)
            self.assertEqual(done.stderr.removeprefix('registry-readmes: ').splitlines(), [
                family + ' block 1: missing-or-ambiguous-example-package: ' + family
                for family in families])

    @unittest.skipUnless(sys.platform in ('linux', 'darwin'), 'compiler replay requires POSIX process groups')
    def test_timed_out_compiler_and_worker_exit_before_replay_releases_build(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch)
            root = tree / 'root'
            (root / 'site/recordings').mkdir(parents=True)
            shutil.copyfile(readmes.REPO / 'rust-toolchain.toml', root / 'rust-toolchain.toml')
            shutil.copyfile(readmes.REPO / 'site/recordings/thinkthen.jsonl', root / 'site/recordings/thinkthen.jsonl')
            page = readmes.source('swift', root)
            page.parent.mkdir(parents=True)
            page.write_text('```swift\ndecide(); choose();\n```\n```text\nfixture\n```\n')
            packages = tree / 'packages'; packages.mkdir()
            archive(packages / ('thinkthen-swift-' + readmes.version() + '-x86_64-unknown-linux-gnu.tar.gz'),
                    {'README.md': readmes.source('swift').read_bytes()})
            tools = tree / 'tools'; tools.mkdir()
            marker = tree / 'compiler.pid'
            compiler = tools / 'swift'
            compiler.write_text('#!' + sys.executable + '\n' +
                'import os, pathlib, signal, subprocess, sys, time\n' +
                'marker = pathlib.Path(' + repr(str(marker)) + ')\n' +
                'if pathlib.Path.cwd().name.startswith("first"):\n' +
                '    hard = pathlib.Path.cwd().name != "first"\n' +
                '    child = subprocess.Popen([sys.executable, "-c", ("import os, signal, time; " + ("os.setsid(); " if sys.platform == "linux" and pathlib.Path.cwd().name == "first-detached" else "") + "signal.signal(signal.SIGTERM, signal.SIG_IGN); time.sleep(30)") if hard else "import time; time.sleep(30)"])\n' +
                '    marker.with_suffix(".worker").write_text(str(child.pid))\n' +
                '    def stop(signum, frame): raise SystemExit(77)\n' +
                '    signal.signal(signal.SIGTERM, signal.SIG_IGN if hard else stop)\n' +
                '    marker.write_text(str(os.getpid()))\n' +
                '    try: time.sleep(2)\n' +
                '    finally:\n' +
                '        if child: child.terminate(); child.wait()\n' +
                '    sys.exit(77)\n' +
                'for pid_file in (marker, marker.with_suffix(".worker")):\n' +
                '    try: os.kill(int(pid_file.read_text()), 0)\n' +
                '    except ProcessLookupError: pass\n' +
                '    else: sys.exit("another compiler is still alive")\n' +
                'out = pathlib.Path(".build/debug/RegistryExample")\n' +
                'out.parent.mkdir(parents=True)\n' +
                'out.write_text(' + repr('#!' + sys.executable + '\nprint("fixture")\n') + ')\n' +
                'out.chmod(0o755)\n')
            compiler.chmod(0o755)
            popen = subprocess.Popen
            communicate = subprocess.Popen.communicate
            def bounded_popen(command, *args, **kwargs):
                if len(command) > 1 and sys.platform == 'linux' and 'build-admission.py' in str(command[1]):
                    command = list(command)
                    command[command.index('--timeout') + 1] = '.4'
                return popen(command, *args, **kwargs)
            def bounded_communicate(process, input=None, timeout=None):
                return communicate(process, input, .4 if timeout == 300 else timeout)
            cases = [(str(tools), 'first'), (str(tools) + os.pathsep + '/usr/bin:/bin', 'first-hard')]
            if sys.platform == 'linux':
                cases.append((str(tools), 'first-detached'))
            for search_path, first_name in cases:
                with self.subTest(path=search_path), patch.object(readmes, 'run_build', side_effect=fixture_build), \
                        patch.dict(os.environ, {
                    'PATH': search_path, 'THINKTHEN_HEAVY_LOCK': str(tree / 'compiler.lock')}, clear=True), \
                        patch.object(subprocess, 'Popen', bounded_popen), \
                        patch.object(popen, 'communicate', bounded_communicate):
                    marker.unlink(missing_ok=True)
                    first = tree / first_name; first.mkdir(exist_ok=True)
                    second = tree / 'second'; second.mkdir(exist_ok=True)
                    failure = readmes.Refusal if sys.platform == 'linux' else subprocess.TimeoutExpired
                    with self.assertRaises(failure) as error:
                        readmes.run_example('swift', packages, first, root)
                    if sys.platform == 'linux':
                        self.assertIn('build timed out', str(error.exception))
                    self.assertTrue(marker.exists(), 'first compiler did not start')
                    self.assertNotEqual(marker.read_text(), marker.with_suffix('.worker').read_text())
                    readmes.run_example('swift', packages, second, root)
                    for pid_file in (marker, marker.with_suffix('.worker')):
                        with self.assertRaises(ProcessLookupError):
                            os.kill(int(pid_file.read_text()), 0)
                shutil.rmtree(first)
                shutil.rmtree(second)

    @unittest.skipUnless(sys.platform == 'linux', 'nested workers require Linux subreaping')
    def test_build_reaps_nested_workers_before_returning_its_permit(self):
        # Own survivors only so a failing regression can clean up its fixture.
        libc = ctypes.CDLL(None, use_errno=True)
        previous = ctypes.c_int()
        self.assertEqual(libc.prctl(37, ctypes.byref(previous), 0, 0, 0), 0)
        self.assertEqual(libc.prctl(36, 1, 0, 0, 0), 0)
        pids = []
        try:
            with tempfile.TemporaryDirectory() as scratch:
                tree = Path(scratch)
                marker = tree / 'workers'
                compiler = tree / 'compiler.py'
                compiler.write_text(
                    'import os, pathlib, signal, subprocess, sys, time\n'
                    'signal.signal(signal.SIGTERM, signal.SIG_IGN)\n'
                    'os.setsid() if sys.argv[1] != "35" else None\n'
                    'with pathlib.Path(sys.argv[2]).open("a") as marker: marker.write(str(os.getpid()) + "\\n")\n'
                    'if int(sys.argv[1]):\n'
                    '    subprocess.Popen([sys.executable, __file__, str(int(sys.argv[1]) - 1), sys.argv[2]], '
                    'stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)\n'
                    'time.sleep(30)\n')
                lock = tree / 'build.lock'
                popen = subprocess.Popen
                communicate = subprocess.Popen.communicate
                def bounded_popen(command, *args, **kwargs):
                    if 'build-admission.py' in str(command[1]):
                        command = list(command)
                        command[command.index('--timeout') + 1] = '2'
                    return popen(command, *args, **kwargs)
                def bounded_communicate(process, input=None, timeout=None):
                    return communicate(process, input, 2 if timeout == 300 else timeout)
                with patch.dict(os.environ, {'THINKTHEN_HEAVY_LOCK': str(lock)}, clear=True), \
                        patch.object(subprocess, 'Popen', bounded_popen), \
                        patch.object(popen, 'communicate', bounded_communicate):
                    try:
                        done = readmes.run_build([sys.executable, str(compiler), '35', str(marker)],
                                                tree, {'PATH': '/usr/bin:/bin', 'GITHUB_ACTIONS': 'true',
                                                       'RUNNER_ENVIRONMENT': 'github-hosted'}, None)
                    except subprocess.TimeoutExpired:
                        done = None
                pids = [int(pid) for pid in marker.read_text().splitlines()]
                self.assertEqual(len(pids), 36, 'compiler did not launch all nested workers')
                self.assertEqual(len(set(pids)), 36, 'compiler and worker PIDs must differ')
                remaining = [pid for pid in pids if Path('/proc', str(pid)).exists()]
                # A new process must acquire every permit and the legacy lock.
                probe = subprocess.run([sys.executable, '-c',
                    'import fcntl, sys; '
                    'lock = open(sys.argv[1], "a+b"); '
                    'fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB); '
                    '[fcntl.lockf(lock, fcntl.LOCK_EX | fcntl.LOCK_NB, 1, slot) for slot in range(4)]',
                    str(lock)], capture_output=True, text=True)
                self.assertEqual(probe.returncode, 0, probe.stderr)
                self.assertEqual(remaining, [], 'returned with live or zombie descendants')
                self.assertIsNotNone(done, 'outer timeout interrupted the supervisor')
                self.assertEqual(done.returncode, 124)
                self.assertIn('build timed out', done.stderr)
        finally:
            for pid in pids:
                try:
                    os.kill(pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            while True:
                try:
                    os.waitpid(-1, 0)
                except ChildProcessError:
                    break
            libc.prctl(36, previous.value, 0, 0, 0)

    def test_replay_refuses_a_command_that_does_not_run(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch)
            packages = tree / 'packages'; packages.mkdir()
            package = packages / ('thinkthen-' + readmes.version() + '-x86_64-unknown-linux-musl.tar.gz')
            archive(package, {'README.md': readmes.source('command').read_bytes(),
                              'thinkthen': b'#!/bin/sh\nexit 77\n'})
            archive(packages / 'thinkthen-first-run.tar.gz', {'thinkthen-first-run/report.txt': b'fixture',
                    'thinkthen-first-run/recording/thinkthen.jsonl': b''})
            work = tree / 'work'; work.mkdir()
            with self.assertRaisesRegex(readmes.Refusal, 'example-did-not-run: command'):
                readmes.run_example('command', packages, work)


if __name__ == '__main__':
    unittest.main()
