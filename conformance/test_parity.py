"""Behavior regressions for the shared consumer assertion/table interface."""
import json
import base64
import copy
from pathlib import Path
import subprocess
import contextlib
import io
import os
import shutil
import sys
import tempfile
import unittest
from unittest.mock import patch
from conformance import parity
from conformance import c_images


class ImageRequestArrivals(unittest.TestCase):
    def setUp(self):
        corpus = json.loads((c_images.ROOT / 'conformance/cases.json').read_text())
        row = next(row for row in corpus['parity']['required_cases']
                   if row['id'] == 'image-admission-packing-overflow-choose')
        self.steps = [step for step in c_images.project(row)['steps']
                      if step['image_scenario']['id'] == 'liquid-d1-complete-question-split']

    def evidence(self, step):
        images = [(c_images.ROOT / path).read_bytes() for path in step['image_paths']]
        got = {'code': 0, 'rows': [
            {'index': at, 'input': caption, 'images': [raw.hex() for raw in images],
             'value': 'red', 'probabilities': {'red': .8, 'blue': .2}}
            for at, caption in enumerate(step['items'])]}
        urls = ['data:image/png;base64,' + base64.b64encode(raw).decode() for raw in images]
        orders = step.get('candidate_orders', [['red', 'blue']] * len(step['items']))
        bodies = [{'state': caption, 'model': 'd1', 'questions': {'q1': {
            'type': 'choice', 'instructions': 'Which color?',
            'criteria': dict.fromkeys(order)}}, 'images': urls}
            for caption, order in zip(step['items'], orders)]
        return got, bodies

    def assert_bodies(self, step, got, bodies):
        c_images.assert_images(step, got, [json.dumps(body, separators=(',', ':')) for body in bodies])

    def test_split_requests_allow_both_arrival_orders(self):
        for step in self.steps:
            got, bodies = self.evidence(step)
            for arrivals in [bodies, bodies[::-1]]:
                with self.subTest(candidate_orders=step.get('candidate_orders'), reversed=arrivals is not bodies):
                    self.assert_bodies(step, got, arrivals)

    def test_split_requests_keep_exact_contents_and_multiplicity(self):
        for step in self.steps:
            got, bodies = self.evidence(step)
            changed_images = copy.deepcopy(bodies)
            changed_images[0]['images'][0:2] = changed_images[0]['images'][1::-1]
            dropped_duplicate = copy.deepcopy(bodies)
            dropped_duplicate[0]['images'].pop()
            changed_question = copy.deepcopy(bodies)
            changed_question[0]['questions']['q1']['instructions'] = 'Changed question'
            for changed in [[bodies[0], bodies[0]], changed_images, dropped_duplicate, changed_question, bodies[:1]]:
                with self.subTest(candidate_orders=step.get('candidate_orders')), self.assertRaises(AssertionError):
                    self.assert_bodies(step, got, changed)
            with self.assertRaises(AssertionError):
                c_images.assert_images(step, got, [json.dumps(body) for body in bodies])
        step = self.steps[0]
        got, bodies = self.evidence(step)
        bodies[0]['questions']['q1']['criteria'] = dict.fromkeys(['blue', 'red'])
        with self.assertRaisesRegex(AssertionError, 'candidate order changed'):
            self.assert_bodies(step, got, bodies)


class PublicAssertions(unittest.TestCase):
    def setUp(self):
        self.cases = {
            'typed-decide': {'id': 'typed-decide', 'verb': 'decide', 'kind': 'typed-result',
                             'checks': ['named', 'compile', 'runtime'], 'preconditions': []},
            'files-decide': {'id': 'files-decide', 'verb': 'decide', 'kind': 'located',
                             'checks': ['named', 'compile', 'runtime'], 'preconditions': []},
        }

    def cell(self, **changes):
        row = {'consumer': 'rust', 'case': 'typed-decide',
               'checks': ['named', 'compile', 'runtime'], 'status': 'pass'}
        return parity.PREFIX + json.dumps(row | changes)

    def test_unknown_duplicate_skipped_and_compatibility_cells_refuse(self):
        for output, cause in [
            (self.cell(case='unknown'), 'unknown parity cell'),
            (self.cell(consumer='javascript'), 'unknown parity cell'),
            (self.cell() + '\n' + self.cell(), 'duplicate parity cell'),
            (self.cell(status='skip'), 'skipped or unknown status'),
            ('conformance: not_run=1 unselected=0\n' + self.cell(), 'required cases skipped'),
            ('conformance: not_run=0 unselected=2\n' + self.cell(), 'required cases skipped'),
            ('typed-decide: not run by the public API\n' + self.cell(), 'required case reported not run'),
            (self.cell(checks=['named', 'runtime']), 'missing named/compiler/runtime'),
            (self.cell(checks=['compile', 'runtime', 'runtime']), 'missing named/compiler/runtime'),
        ]:
            with self.subTest(cause=cause), self.assertRaisesRegex(ValueError, cause):
                parity.cells(output, ['rust'], self.cases)

    def test_zero_skip_counters_allow_complete_asserted_cells(self):
        seen = parity.cells('conformance: not_run=0 unselected=0\n' + self.cell(), ['rust'], self.cases)
        self.assertEqual(seen, {('rust', 'typed-decide'): 'pass'})

    def test_failed_case_keeps_independent_pass_and_missing_cell_visible(self):
        seen = parity.cells(self.cell(), ['rust'], self.cases)
        row = parity.summarize('rust', self.cases, 0, seen, None, None)
        self.assertEqual(row['cells'], {'typed-decide': 'pass', 'files-decide': 'missing'})
        partial = parity.summarize('rust', self.cases, 1, seen, None, None)
        self.assertEqual(partial['cells'], {'typed-decide': 'pass', 'files-decide': 'missing'})
        self.assertEqual(partial['named_typed_functions'], ['decide'])
        for code in [77, 127, None]:
            with self.subTest(code=code):
                row = parity.summarize('rust', self.cases, code, seen, None, None)
                self.assertEqual(set(row['cells'].values()), {'fail'})
                self.assertIsNone(row['named_typed_functions'])

    def test_each_public_variant_requires_its_own_execution(self):
        cases = {'typed-decide': self.cases['typed-decide']}
        seen = parity.cells(self.cell(consumer='java'), ['java', 'kotlin', 'scala'], cases)
        for consumer in ['kotlin', 'scala']:
            row = parity.summarize(consumer, cases, 0, seen, None, None)
            self.assertEqual(row['cells']['typed-decide'], 'missing')

    def test_consumer_specific_cases_cannot_pass_at_another_door(self):
        self.cases['typed-decide']['consumers'] = ['mcp']
        with self.assertRaisesRegex(ValueError, 'wrong public variant'):
            parity.cells(self.cell(), ['rust'], self.cases)

    def test_written_ruling_and_successful_baseline_do_not_claim_support(self):
        contract = {'consumers': [{'id': 'rust', 'owner': '0431',
                                  'rulings': ['client reader must execute']}],
                    'required_cases': list(self.cases.values())}
        row = parity.summarize('rust', self.cases, 0, {}, None, None)
        table = parity.support_table(contract, [row])
        self.assertIn('| rust | not checked | not checked | not checked;', table)
        self.assertIn('written ruling: client reader must execute', table)
        row = parity.summarize('rust', self.cases, 0, {('rust', 'typed-decide'): 'pass'}, None, None)
        self.assertIn('| rust | 1/10 | not checked |', parity.support_table(contract, [row]))

    def test_owned_environment_uses_loopback_and_no_ambient_settings(self):
        with tempfile.TemporaryDirectory() as scratch, patch.object(parity.os, 'environ', {
            'HOME': '/ambient/home', 'THINKTHEN_BASE_URL': 'https://example.invalid',
            'THINKTHEN_API_KEY': 'ambient-secret', 'THINKTHEN_CONFORMANCE_IDS': '/selector',
            'THINKTHEN_TEST_PROFILE': 'routine', 'OTHER_API_KEY': 'unrelated-secret',
            'THINKTHEN_TOOLCHAINS': str(Path(scratch) / 'tools'), 'CARGO_BUILD_JOBS': '1',
            'R_LIBS_USER': str(Path(scratch) / 'r-library'), 'PUB_CACHE': str(Path(scratch) / 'pub-cache'),
            'UV_CACHE_DIR': str(Path(scratch) / 'uv-cache'),
            'UV_PYTHON_INSTALL_DIR': str(Path(scratch) / 'uv-python'),
        }):
            (Path(scratch) / 'tools').mkdir()
            env = parity.consumer_environment(scratch, '12345')
            self.assertEqual(env['THINKTHEN_BASE_URL'], 'http://127.0.0.1:12345/generic/v1')
            self.assertEqual(env['THINKTHEN_API_KEY'], 'sk-conformance-loopback')
            self.assertEqual(env['THINKTHEN_TEST_PROFILE'], 'full')
            for name in ['HOME', 'XDG_CONFIG_HOME', 'XDG_CACHE_HOME', 'XDG_STATE_HOME', 'APPDATA']:
                self.assertTrue(Path(env[name]).is_relative_to(scratch))
            self.assertNotIn('THINKTHEN_CONFORMANCE_IDS', env)
            self.assertNotIn('OTHER_API_KEY', env)
            self.assertEqual(env['CARGO_BUILD_JOBS'], '1')
            self.assertEqual(env['R_LIBS_USER'], str(Path(scratch) / 'r-library'))
            self.assertEqual(env['PUB_CACHE'], str(Path(scratch) / 'pub-cache'))
            self.assertEqual(env['UV_CACHE_DIR'], str(Path(scratch) / 'uv-cache'))
            self.assertEqual(env['UV_PYTHON_INSTALL_DIR'], str(Path(scratch) / 'uv-python'))
            self.assertEqual((Path(env['HOME']) / '.cache/thinkthen-toolchains').resolve(),
                             Path(scratch) / 'tools')
            self.assertFalse((Path(env['HOME']) / '.config').exists())

    def test_full_run_fails_missing_and_exit77_without_executing_sdk(self):
        contract = {'functions': ['decide'], 'consumers': [
            {'id': 'rust', 'owner': '0431', 'baseline': ['sh', 'stub.sh']}],
                    'required_cases': list(self.cases.values())}
        for output, code in [(self.cell(), 0), (self.cell(), 77), ('', 127),
                             (self.cell(case='unknown'), 0),
                             (self.cell() + '\n' + self.cell(), 0), (self.cell(status='skip'), 0)]:
            with self.subTest(code=code, output=output), tempfile.TemporaryDirectory() as scratch:
                def command(args, **kwargs):
                    kwargs['stdout'].write(output)
                    self.assertEqual(kwargs['env']['THINKTHEN_TEST_PROFILE'], 'full')
                    return subprocess.CompletedProcess(args, code)
                with contextlib.redirect_stdout(io.StringIO()), patch.object(parity, 'ROOT', Path(scratch)), patch.object(parity, 'inventory', return_value=contract), patch.object(parity.subprocess, 'run', side_effect=command), patch.object(parity.os, 'environ', {}):
                    self.assertEqual(parity.run('12345'), 1)
                matrix = json.loads((Path(scratch) / 'target/parity/matrix.json').read_text())
                self.assertNotEqual(matrix[0]['cells']['files-decide'], 'pass')

    def test_actual_runner_failure_and_missing_command_cannot_qualify_cells(self):
        for baseline, expected_exit in [
            ([sys.executable, '-c', 'import sys; print(sys.argv[1]); sys.exit(77)', self.cell()], 77),
            (['/nonexistent/thinkthen-parity-required-toolchain'], None),
        ]:
            with self.subTest(baseline=baseline), tempfile.TemporaryDirectory() as scratch:
                root = Path(scratch)
                scripts = root / 'sdlc/scripts'
                scripts.mkdir(parents=True)
                for filename in ['time-limit', 'scratch.sh']:
                    shutil.copyfile(parity.ROOT / 'sdlc/scripts' / filename, scripts / filename)
                contract = {'functions': ['decide'], 'consumers': [
                    {'id': 'rust', 'owner': '0431', 'baseline': baseline}],
                    'required_cases': list(self.cases.values())}
                with contextlib.redirect_stdout(io.StringIO()), patch.object(parity, 'ROOT', root), patch.object(parity, 'inventory', return_value=contract), patch.object(parity.os, 'environ', {}):
                    self.assertEqual(parity.run('12345'), 1)
                row = json.loads((root / 'target/parity/matrix.json').read_text())[0]
                if expected_exit is not None:
                    self.assertEqual(row['baseline_exit'], expected_exit)
                else:
                    self.assertNotEqual(row['baseline_exit'], 0)
                    self.assertIn('cannot run', (root / 'target/parity/rust.log').read_text())
                self.assertEqual(set(row['cells'].values()), {'fail'})

    def test_full_run_refuses_selectors_and_non_loopback_port_arguments(self):
        with patch.object(parity, 'inventory', return_value={}), patch.object(parity.os, 'environ', {'THINKTHEN_CONFORMANCE_IDS': ''}):
            with self.assertRaisesRegex(ValueError, 'refuses a case selector'):
                parity.run('12345')
        for port in ['https://example.invalid', '0', '65536', '-1']:
            with self.assertRaisesRegex(ValueError, 'owned loopback port'):
                parity.run(port)


class InstalledInputs(unittest.TestCase):
    def setUp(self):
        self.consumers = {row['id']: row for row in parity.all_consumers(parity.inventory())}
        self.native = 'thinkthen-c-0.2.0-x86_64-unknown-linux-gnu.tar.gz'
        self.command = 'thinkthen-0.2.0-x86_64-unknown-linux-musl.tar.gz'
        self.dart = 'thinkthen-dart-0.2.0-x86_64-unknown-linux-gnu.tar.gz'

    def packages(self, folder):
        names = [self.command, self.native, self.dart, 'thinkthen-0.2.0.crate',
                 'thinkthen-0.2.0-cp310-abi3-linux_x86_64.whl', 'thinkthen-0.2.0.tgz',
                 'thinkthen-0.2.0-x86_64-linux.gem', 'thinkthen_0.2.0.tar.gz']
        names += [f'thinkthen-{family}-0.2.0-x86_64-unknown-linux-gnu.tar.gz' for family in
                  ['go', 'csharp', 'jvm', 'cpp', 'swift', 'zig', 'objective-c', 'php',
                   'flutter', 'ada', 'cobol', 'sqlite', 'duckdb', 'postgresql16']]
        for name in names:
            (folder / name).touch()
        # The plain diagnostic gem is shipped alongside the actual native gem.
        (folder / 'thinkthen-0.2.0.gem').touch()

    def select(self, folder):
        with patch.object(parity.subprocess, 'check_output', return_value='host: x86_64-unknown-linux-gnu\n'):
            return parity.installed_artifacts(folder, self.consumers)

    def test_each_group_uses_its_actual_package_and_shared_native_inputs(self):
        with tempfile.TemporaryDirectory() as scratch:
            folder = Path(scratch)
            self.packages(folder)
            inputs, command, native = self.select(folder)
            self.assertEqual(set(inputs), set(self.consumers))
            self.assertEqual(Path(command).name, self.command)
            self.assertEqual(Path(native).name, self.native)
            for group in [('cli', 'mcp'), ('python', 'pandas', 'python-polars'),
                          ('typescript', 'javascript'), ('java', 'kotlin', 'scala'),
                          ('dart', 'flutter'), ('rust', 'rust-polars')]:
                self.assertEqual(len({inputs[row]['THINKTHEN_ARTIFACT'] for row in group}), 1)
            self.assertEqual(Path(inputs['postgresql']['THINKTHEN_ARTIFACT']).name,
                             'thinkthen-postgresql16-0.2.0-x86_64-unknown-linux-gnu.tar.gz')
            for row in inputs.values():
                self.assertEqual(row['THINKTHEN_C_ARTIFACT'], native)
                self.assertEqual(Path(row['THINKTHEN_DART_ARTIFACT']).name, self.dart)
                self.assertTrue(Path(row['THINKTHEN_ARTIFACT']).is_relative_to(folder))

    def test_missing_ambiguous_linked_and_non_file_inputs_refuse(self):
        for defect in ['missing', 'ambiguous', 'linked', 'directory']:
            with self.subTest(defect=defect), tempfile.TemporaryDirectory() as scratch:
                folder = Path(scratch)
                self.packages(folder)
                if defect == 'ambiguous':
                    (folder / 'thinkthen-0.2.0-x86_64-unknown-linux-gnu.tar.gz').touch()
                else:
                    required = folder / self.native
                    required.unlink()
                    if defect == 'linked':
                        required.symlink_to(folder / self.dart)
                    elif defect == 'directory':
                        required.mkdir()
                with self.assertRaisesRegex(ValueError, 'one regular artifact'):
                    self.select(folder)

    def test_installed_c_inputs_refuse_before_any_checkout_build(self):
        for inputs in [dict(THINKTHEN_ARTIFACT='/missing/archive'),
                       dict(THINKTHEN_C_HEADER='/missing/header'),
                       dict(THINKTHEN_C_HEADER='/missing/header', THINKTHEN_C_LIBRARY='/missing/library')]:
            with self.subTest(inputs=inputs):
                result = subprocess.run([sys.executable, str(parity.ROOT / 'conformance/c_parity.py')],
                                        env={'PATH': '/nonexistent', **inputs}, capture_output=True,
                                        text=True, check=False)
                self.assertNotEqual(result.returncode, 0)
                self.assertNotIn("'cargo'", result.stderr)
                self.assertTrue('explicit header and library' in result.stderr or 'No such file or directory' in result.stderr)


class FullCheckpoint(unittest.TestCase):
    def test_required_parity_keeps_private_safety_and_release_checks_without_duplicate_consumers(self):
        for entry, args in [('test-full-cases', ['--run']),
                            ('surfaces', ['--full-functional']), ('surfaces', ['--parity']),
                            ('surfaces', ['--full-functional', '--publish', 'checkpoint/surfaces/test',
                                          '--publish-root', '/unused-publication-root']),
                            ('test-full-cases', ['--run', '--artifacts', '.']),
                            ('surfaces', ['--full-functional', '--artifacts', '.'])]:
            installed = '--artifacts' in args
            for failed in [None, 'safety', 'parity'] + ([] if installed else ['smoke']):
                with self.subTest(entry=entry, failed=failed), tempfile.TemporaryDirectory() as folder:
                    root = Path(folder)
                    scripts = root / 'sdlc/scripts'
                    scripts.mkdir(parents=True)
                    for name in ['test-full-cases', 'surfaces', 'verdict.sh']:
                        shutil.copyfile(parity.ROOT / 'sdlc/scripts' / name, scripts / name)
                    files = {
                        'sdlc/scripts/heavy-lock': '',
                        'sdlc/scripts/scratch.sh': 'usage_guard() { :; }\nscratch_dir() { packed=$PWD/packed; mkdir -p "$packed"; }\n',
                        'sdlc/scripts/installed.sh': 'backend_start() { port=12345; }\n',
                        'sdlc/scripts/test': 'printf "workspace\\n" >> calls\n',
                        'sdlc/surfaces.txt': 'libraries/c landed\n',
                        'libraries/c/Cargo.toml': '',
                        'libraries/c/check.sh': 'printf "safety\\n" >> calls\nexit "$SAFETY_CODE"\n',
                        'sdlc/scripts/release-pack': 'if [ "${4:-}" = crate ]; then printf "extra-pack\\n" >> calls; else printf "pack\\n" >> calls; fi\n',
                        'sdlc/scripts/release-smoke': 'printf "smoke\\n" >> calls\nexit "$SMOKE_CODE"\n',
                        'sdlc/scripts/publish-builds': 'if [ "$1" = --check ]; then printf "stage-check\\n" >> calls; else printf "stage\\n" >> calls; fi\n',
                        'bin/git': '#!/bin/sh\nif [ "$1" = rev-parse ]; then printf "source-hash\\n"; fi\n',
                        'bin/rustc': '#!/bin/sh\nprintf "host: x86_64-unknown-linux-gnu\\n"\n',
                        'conformance/parity.py': (
                            'import os,sys\n'
                            'if sys.argv[1:] != ["--validate"]:\n'
                            ' assert "THINKTHEN_CONFORMANCE_IDS" not in os.environ\n'
                            ' assert os.environ["THINKTHEN_TEST_PROFILE"] == "full"\n'
                            ' assert ("--artifacts" in sys.argv) == bool(os.environ["INSTALLED"])\n'
                            ' with open("calls", "a") as f: f.write("parity\\n")\n'
                            ' sys.exit(int(os.environ["PARITY_CODE"]))\n'),
                    }
                    for path, text in files.items():
                        target = root / path
                        target.parent.mkdir(parents=True, exist_ok=True)
                        target.write_text(text)
                    (root / 'bin/rustc').chmod(0o755)
                    (root / 'bin/git').chmod(0o755)
                    env = {'PATH': str(root / 'bin') + ':' + os.defpath,
                           'HOME': str(root / 'home'), 'SAFETY_CODE': '77' if failed == 'safety' else '0',
                           'PARITY_CODE': '1' if failed == 'parity' else '0',
                           'SMOKE_CODE': '77' if failed == 'smoke' else '0',
                           'INSTALLED': 'yes' if installed else '',
                           'THINKTHEN_CONFORMANCE_IDS': '/unwanted-selector'}
                    result = subprocess.run(['sh', str(scripts / entry), *args], cwd=root,
                                            env=env, capture_output=True, text=True, check=False)
                    expected = ['safety'] if failed == 'safety' else ['safety', 'parity']
                    if failed not in ('safety', 'parity') and not installed:
                        expected += ['pack', 'smoke']
                    if entry == 'test-full-cases':
                        expected.insert(0, 'workspace')
                    if '--publish' in args:
                        expected.insert(0, 'stage-check')
                        if failed is None:
                            expected += ['extra-pack', 'stage']
                    self.assertEqual((root / 'calls').read_text().splitlines(), expected, result.stderr)
                    self.assertEqual(result.returncode == 0, failed is None, result.stdout + result.stderr)


if __name__ == '__main__':
    unittest.main()
