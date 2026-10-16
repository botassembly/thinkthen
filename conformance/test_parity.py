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


class NativeFixtureEnvironment(unittest.TestCase):
    def test_compiler_child_owns_folders_and_preserves_explicit_settings(self):
        sys.path.append(str(parity.ROOT / 'conformance/children'))
        sys.path.insert(0, str(parity.ROOT / 'conformance'))
        import parity as native_parity
        import native_fixture
        launch = subprocess.run
        def compiler(args, **kwargs):
            probe = "import os; assert os.environ.get('FAKE_SERVICE_API_KEY') is None; assert os.environ['APPDATA'] == os.environ['XDG_CONFIG_HOME']; assert os.environ['LOCALAPPDATA'] == os.environ['HOME'] + '/local'; assert os.environ['THINKTHEN_API_KEY'] == 'fixture-only'; assert os.environ['CARGO_BUILD_JOBS'] == '2'; assert os.environ['CARGO_PROFILE_DEV_OPT_LEVEL'] == '2'; assert os.environ['RUSTFLAGS'] == '--remap-path-prefix=/checkout=/build'; assert os.environ['RUSTC_WRAPPER'] == ''"
            return launch([sys.executable, '-c', probe], **kwargs)
        with patch.object(os, 'environ', {'PATH': os.defpath, 'FAKE_SERVICE_API_KEY': 'parent-only', 'APPDATA': '/ambient', 'CARGO_BUILD_JOBS': '2', 'CARGO_PROFILE_DEV_OPT_LEVEL': '2', 'RUSTFLAGS': '--remap-path-prefix=/checkout=/build', 'RUSTC_WRAPPER': ''}), patch.object(native_parity, 'required_cases', return_value={}), patch.object(subprocess, 'run', side_effect=compiler):
            native_fixture.run('rust', ['cargo', 'fixture.rs'], parity.ROOT,
                               extra_env={'THINKTHEN_API_KEY': 'fixture-only'},
                               rust_manifest=parity.ROOT/'libraries/rust/consumer/Cargo.toml')


class BindingCaseTiers(unittest.TestCase):
    def test_bindings_keep_every_host_case_and_sample_each_native_group(self):
        contract = parity.inventory()
        rows = {row['id']: row for row in contract['required_cases']}
        sample = set(parity.CORE_SAMPLE)
        for consumer in parity.all_consumers(contract):
            identity = consumer['id']
            selected = parity.required_cases(contract, identity)
            applicable = {key for key, row in rows.items()
                          if identity in row.get('consumers', [identity])}
            if identity == 'cli':
                applicable -= parity.SDK_ONLY_CLI_CASES
            expected = applicable if identity in ('cli', 'rust', 'c') else {
                key for key in applicable if rows[key]['tier'] == 'binding' or key in sample}
            self.assertEqual(set(selected), expected)
            for key in selected:
                self.assertEqual(selected[key].get('checks'),
                                 ['named', 'runtime'] if identity == 'cli' else rows[key].get('checks'))
                if identity != 'cli':
                    self.assertEqual(selected[key]['expect'], rows[key]['expect'])
        # These cases exercise conversion, ordered attachments, host feeds,
        # error decoding and cancellation rather than native limit repetition.
        retained = {'selected-object-false-unicode', 'item-root-null', 'wording-version-boolean',
                    'wording-version-string', 'wording-version-null',
                    'stream-staged-batch', 'finite-all-before-send', 'cancellation-held-call',
                    'images-decide', 'images-choose', 'images-score', 'image-file-decide',
                    'boundary-usage', 'boundary-cancelled', '40-decide-counters'}
        self.assertTrue(all(rows[key]['tier'] == 'binding' for key in retained))
        for kind in ('typed-result', 'result2', 'located'):
            self.assertEqual({row['verb'] for row in rows.values() if row['kind'] == kind
                              and row['tier'] == 'binding'}, set(contract['functions']))
        for group in contract['image_admission_scenarios']:
            candidates = [row for row in rows.values() if row['input'].get('scenarios_ref') == group]
            self.assertTrue(any(row['tier'] == 'binding' or row['id'] in sample for row in candidates))

    def test_core_sample_covers_each_native_refusal_group_and_failure_kind(self):
        contract = parity.inventory()
        named = {row['id']: row for row in json.loads(
            (parity.ROOT / 'conformance/named-inputs.json').read_text())['cases']}
        rows = {row['id']: row for row in contract['required_cases']}
        raw_grammar = {key for key, fixture in named.items()
                       if 'question_json' in fixture['input']}
        self.assertTrue(all(rows[key]['tier'] == 'core' for key in raw_grammar))
        groups = {}
        for row in contract['required_cases']:
            if row['tier'] != 'core':
                continue
            key = row['id']
            if row['kind'] == 'image-admission':
                group = row['input']['scenarios_ref']
                kinds = {scenario['expect'].get('kind', 'success') for scenario
                         in contract['image_admission_scenarios'][group]}
            else:
                group = ('question-json' if key in raw_grammar
                         else key.split('-')[0])
                kinds = {named[key]['expect'].get('error', 'success')}
            required, sampled = groups.setdefault(group, (set(), set()))
            required.update(kinds)
            if key in parity.CORE_SAMPLE:
                sampled.update(kinds)
        for group, (required, sampled) in groups.items():
            with self.subTest(group=group):
                self.assertEqual(sampled, required)

    def test_inventory_refuses_missing_tier_and_noncore_sample(self):
        document = json.loads(parity.DOCUMENT.read_text())
        for defect in ('missing-tier', 'unknown-tier', 'binding-sample'):
            with self.subTest(defect=defect), tempfile.TemporaryDirectory() as folder:
                changed = copy.deepcopy(document)
                row = next(row for row in changed['parity']['required_cases']
                           if row['id'] == 'image-admission-dimension-decide')
                if defect == 'missing-tier':
                    row.pop('tier')
                else:
                    row['tier'] = 'unknown' if defect == 'unknown-tier' else 'binding'
                fixture = Path(folder) / 'cases.json'
                fixture.write_text(json.dumps(changed))
                cause = 'core sample' if defect == 'binding-sample' else 'core or binding tier'
                with patch.object(parity, 'DOCUMENT', fixture), self.assertRaisesRegex(ValueError, cause):
                    parity.inventory()

    def test_missing_binding_or_sample_cell_still_fails(self):
        contract = parity.inventory()
        required = parity.required_cases(contract, 'python')
        for missing in ('typed-decide', 'image-admission-dimension-decide'):
            with self.subTest(missing=missing):
                output = '\n'.join(parity.PREFIX + json.dumps({
                    'consumer': 'python', 'case': key,
                    'checks': row.get('checks', ['named', 'runtime']), 'status': 'pass'})
                    for key, row in required.items() if key != missing)
                seen = parity.cells(output, ['python'], required, contract)
                summary = parity.summarize('python', required, 0, seen, None, 'fixture')
                self.assertEqual(summary['cells'][missing], 'missing')
                self.assertEqual(summary['adopted_case_count'], len(required) - 1)


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
            (self.cell(checks=['named', 'runtime']), 'missing named/compiler/runtime'),
            (self.cell(checks=['compile', 'runtime', 'runtime']), 'missing named/compiler/runtime'),
        ]:
            with self.subTest(cause=cause), self.assertRaisesRegex(ValueError, cause):
                parity.cells(output, ['rust'], self.cases)

    def test_installed_sqlite_cells_override_legacy_diagnostics_without_hiding_gaps(self):
        output = (parity.ROOT / 'conformance/fixtures/sqlite-installed-parity.txt').read_text()
        contract = parity.inventory()
        case_id = '18-annotate-two-groups'
        cases = {case_id: parity.required_cases(contract, 'sqlite')[case_id]}
        for defect in [None, 'missing', 'fail', 'skip', 'checks']:
            with self.subTest(defect=defect):
                lines = output.splitlines()
                at = next(at for at, line in enumerate(lines) if line.startswith(parity.PREFIX))
                cell = json.loads(lines[at][len(parity.PREFIX):])
                if defect == 'missing':
                    del lines[at]
                elif defect in ('fail', 'skip'):
                    cell['status'] = defect
                    lines[at] = parity.PREFIX + json.dumps(cell)
                elif defect == 'checks':
                    cell['checks'] = ['runtime']
                    lines[at] = parity.PREFIX + json.dumps(cell)
                changed = '\n'.join(lines)
                if defect in ('skip', 'checks'):
                    cause = ('skipped or unknown status' if defect == 'skip'
                             else 'missing named/compiler/runtime')
                    with self.assertRaisesRegex(ValueError, cause):
                        parity.cells(changed, ['sqlite'], cases, contract)
                    continue
                seen = parity.cells(changed, ['sqlite'], cases, contract)
                row = parity.summarize('sqlite', cases, 0, seen, None, None)
                if defect is None:
                    self.assertEqual(set(row['cells'].values()), {'pass'})
                    self.assertEqual(len(seen), len(cases))
                else:
                    self.assertEqual(row['cells'][cell['case']],
                                     'missing' if defect == 'missing' else 'fail')
                    self.assertNotEqual(set(row['cells'].values()), {'pass'})

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
            'npm_config_cache': str(Path(scratch) / 'npm-cache'),
            'UV_PYTHON_INSTALL_DIR': str(Path(scratch) / 'uv-python'),
            'CARGO_PROFILE_DEV_OPT_LEVEL': '2', 'CARGO_PROFILE_DEV_DEBUG': '0',
            'CARGO_PROFILE_TEST_OPT_LEVEL': '2', 'CARGO_PROFILE_TEST_DEBUG': '0',
            'RUSTFLAGS': '--remap-path-prefix=/checkout=/build/thinkthen',
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
            for name in ('CARGO_PROFILE_DEV_OPT_LEVEL', 'CARGO_PROFILE_TEST_OPT_LEVEL'):
                self.assertEqual(env[name], '2')
            self.assertEqual(env['RUSTFLAGS'], '--remap-path-prefix=/checkout=/build/thinkthen')
            self.assertEqual(env['R_LIBS_USER'], str(Path(scratch) / 'r-library'))
            self.assertEqual(env['PUB_CACHE'], str(Path(scratch) / 'pub-cache'))
            self.assertEqual(env['UV_CACHE_DIR'], str(Path(scratch) / 'uv-cache'))
            self.assertEqual(env['npm_config_cache'], str(Path(scratch) / 'npm-cache'))
            self.assertEqual(env['UV_PYTHON_INSTALL_DIR'], str(Path(scratch) / 'uv-python'))
            self.assertEqual((Path(env['HOME']) / '.cache/thinkthen-toolchains').resolve(),
                             Path(scratch) / 'tools')
            self.assertFalse((Path(env['HOME']) / '.config').exists())

    def test_default_tool_locations_survive_the_isolated_home(self):
        with tempfile.TemporaryDirectory() as scratch, patch.object(parity.os, 'environ', {'HOME': '/caller'}):
            env = parity.consumer_environment(scratch, '12345')
            expected = {'THINKTHEN_TOOLCHAINS': '/caller/.cache/thinkthen-toolchains',
                        'R_LIBS_USER': '/caller/.cache/thinkthen-toolchains/r-library',
                        'PUB_CACHE': '/caller/.pub-cache', 'UV_CACHE_DIR': '/caller/.cache/uv',
                        'UV_PYTHON_INSTALL_DIR': '/caller/.local/share/uv/python',
                        'npm_config_cache': '/caller/.npm', 'CARGO_BUILD_JOBS': '4'}
            for name, value in expected.items():
                self.assertEqual(env[name], value)
            self.assertNotEqual(env['HOME'], '/caller')

    def test_four_consumers_run_together_while_rust_consumers_share_one_lane(self):
        import threading
        ids = ['rust', 'rust-polars', 'one', 'two', 'three']
        contract = {'functions': ['decide'], 'consumers': [
            {'id': name, 'owner': '0431', 'baseline': ['sh', name]}
            for name in ids], 'required_cases': list(self.cases.values())}
        barrier = threading.Barrier(4, timeout=3)
        finished_rust = threading.Event()
        def command(args, **kwargs):
            name = args[-2]
            if name == 'rust-polars':
                self.assertTrue(finished_rust.is_set())
            else:
                barrier.wait()
            for case in self.cases:
                kwargs['stdout'].write(self.cell(consumer=name, case=case) + '\n')
            kwargs['stdout'].write('THINKTHEN_CONFORMANCE_DURATION ' + json.dumps({'consumer': name, 'duration_seconds': 0.25}) + '\n')
            if name == 'rust':
                finished_rust.set()
            return subprocess.CompletedProcess(args, 0)
        with tempfile.TemporaryDirectory() as scratch, contextlib.redirect_stdout(io.StringIO()), patch.object(parity, 'ROOT', Path(scratch)), patch.object(parity, 'inventory', return_value=contract), patch.object(parity.subprocess, 'run', side_effect=command), patch.object(parity.os, 'environ', {}):
            self.assertEqual(parity.run('12345'), 0)
            matrix = json.loads((Path(scratch) / 'target/parity/matrix.json').read_text())
            self.assertEqual({row['consumer'] for row in matrix}, set(ids))
            self.assertTrue(all(0 <= row['duration_seconds'] < 3 for row in matrix))
            self.assertTrue(all(row['duration_seconds'] != 0.25 for row in matrix))
            self.assertIn('Duration (s)', (Path(scratch) / 'target/parity/matrix.md').read_text())

    def test_family_prepares_once_and_gives_each_member_private_state_and_remaining_budget(self):
        import threading
        import time
        names = ['python', 'pandas', 'python-polars', 'java', 'kotlin', 'scala']
        contract = {'functions': ['decide'], 'consumers': [
            {'id': name, 'owner': '0431', 'baseline': ['sh', 'python' if name in names[:3] else 'jvm']}
            for name in names], 'required_cases': list(self.cases.values())}
        prepared, homes, active, peak = {}, {}, 0, 0
        lock = threading.Lock()
        first_four = threading.Barrier(4, timeout=3)
        def command(args, **kwargs):
            nonlocal active, peak
            env = kwargs['env']
            if destination := env.get('THINKTHEN_PARITY_PREPARE'):
                prepared[destination] = args[-2]
                Path(destination, 'installed-package').write_text('immutable')
                time.sleep(.03)
                return subprocess.CompletedProcess(args, 0)
            member = env['THINKTHEN_PARITY_CONSUMER']
            assets = Path(env['THINKTHEN_PARITY_PREPARED'])
            self.assertEqual((assets / 'installed-package').read_text(), 'immutable')
            self.assertEqual(args[2], '899')
            homes[member] = {key: env[key] for key in ('HOME', 'XDG_CONFIG_HOME', 'XDG_STATE_HOME', 'XDG_CACHE_HOME', 'TMPDIR')}
            # Each backend/cache fixture counter belongs to this member alone.
            Path(env['XDG_STATE_HOME'], 'counter').write_text(member)
            with lock:
                active += 1
                peak = max(peak, active)
                join = len(homes) <= 4
            if join:
                first_four.wait()
            time.sleep(.01)
            self.assertEqual(Path(env['XDG_STATE_HOME'], 'counter').read_text(), member)
            for case in self.cases:
                kwargs['stdout'].write(self.cell(consumer=member, case=case) + '\n')
            with lock:
                active -= 1
            return subprocess.CompletedProcess(args, 0)
        with tempfile.TemporaryDirectory() as scratch, contextlib.redirect_stdout(io.StringIO()), patch.object(parity, 'ROOT', Path(scratch)), patch.object(parity, 'inventory', return_value=contract), patch.object(parity.subprocess, 'run', side_effect=command), patch.object(parity.os, 'environ', {'PATH': os.defpath}):
            self.assertEqual(parity.run('12345'), 0)
            matrix = json.loads((Path(scratch) / 'target/parity/matrix.json').read_text())
            self.assertEqual(len(prepared), 2)
            self.assertEqual(peak, 4)
            self.assertEqual(len(homes), len(names))
            for key in next(iter(homes.values())):
                self.assertEqual(len({env[key] for env in homes.values()}), len(names))
            self.assertTrue(all(row['duration_seconds'] >= .04 for row in matrix))
            self.assertTrue(all(not Path(path).exists() for path in prepared))
            self.assertTrue(all(not Path(env['HOME']).exists() for env in homes.values()))

    def test_failed_or_exhausted_family_setup_fails_every_member_without_runtime(self):
        names = ['python', 'pandas', 'python-polars']
        contract = {'functions': ['decide'], 'consumers': [
            {'id': name, 'owner': '0431', 'baseline': ['sh', 'family']}
            for name in names], 'required_cases': list(self.cases.values())}
        for failure in (77, 0):
            clock = [0]
            def command(args, **kwargs):
                self.assertIn('THINKTHEN_PARITY_PREPARE', kwargs['env'])
                clock[0] = 901 if failure == 0 else 2
                return subprocess.CompletedProcess(args, failure)
            with self.subTest(failure=failure), tempfile.TemporaryDirectory() as scratch, contextlib.redirect_stdout(io.StringIO()), patch.object(parity, 'ROOT', Path(scratch)), patch.object(parity, 'inventory', return_value=contract), patch.object(parity.subprocess, 'run', side_effect=command), patch.object(parity.time, 'monotonic', side_effect=lambda: clock[0]), patch.object(parity.os, 'environ', {'PATH': os.defpath}):
                self.assertEqual(parity.run('12345'), 1)
                matrix = json.loads((Path(scratch) / 'target/parity/matrix.json').read_text())
                self.assertEqual({row['baseline_exit'] for row in matrix}, {failure or 124})
                self.assertTrue(all(row['adopted_case_count'] == 0 and 'preparation' in row['baseline_error'] for row in matrix))
                self.assertTrue(all(row['duration_seconds'] == clock[0] for row in matrix))

    def test_prepared_python_shell_uses_shared_installed_venv_and_private_fixture_copy(self):
        import venv
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            library = root / 'libraries/python'
            library.mkdir(parents=True)
            shutil.copyfile(parity.ROOT / 'libraries/python/check.sh', library / 'check.sh')
            (library / 'host.sh').write_text('python_host() { command -v python3; }\n')
            scripts = root / 'sdlc/scripts'
            scripts.mkdir(parents=True)
            shutil.copyfile(parity.ROOT / 'sdlc/scripts/scratch.sh', scripts / 'scratch.sh')
            assets = root / 'assets'
            venv.EnvBuilder(with_pip=False).create(assets / 'venv')
            site = next((assets / 'venv/lib').glob('python*/site-packages')) / 'thinkthen'
            site.mkdir()
            (site / '__init__.py').write_text('')
            (assets / 'tests').mkdir()
            (assets / 'tests/native_case.py').write_text('# compiled installed caller fixture\n')
            children = root / 'conformance/children'
            children.mkdir(parents=True)
            (children / 'native_fixture.py').write_text(
                'import os, sys\nfrom pathlib import Path\n'
                'def run(consumer, command, root, **kwargs):\n'
                ' assert consumer == os.environ["THINKTHEN_PARITY_CONSUMER"]\n'
                ' assert command[0] == sys.executable\n'
                ' assert Path(command[-1]).read_text() == "# compiled installed caller fixture\\n"\n'
                ' assert not Path(command[-1]).is_relative_to(os.environ["THINKTHEN_PARITY_PREPARED"])\n'
                ' print("private fixture: " + consumer)\n'
                ' return False\n')
            tools = root / 'tools'
            tools.mkdir()
            for tool in ('uv', 'maturin'):
                (tools / tool).write_text('#!/bin/sh\nexit 99\n')
                (tools / tool).chmod(0o755)
            for member in ('python', 'pandas', 'python-polars'):
                with tempfile.TemporaryDirectory() as home:
                    env = parity.consumer_environment(home, '12345')
                    env.update(PATH=str(tools) + ':' + os.environ['PATH'], THINKTHEN_PARITY_PREPARED=str(assets), THINKTHEN_PARITY_CONSUMER=member)
                    result = subprocess.run(['sh', str(library / 'check.sh')], env=env, capture_output=True, text=True)
                    self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
                    self.assertIn('private fixture: ' + member, result.stdout)
            self.assertEqual((assets / 'tests/native_case.py').read_text(), '# compiled installed caller fixture\n')

    def test_bare_rust_campaign_uses_the_complete_source_consumer(self):
        import shlex
        rust = next(row for row in parity.inventory()['consumers'] if row['id'] == 'rust')
        contract = {'functions': ['decide'], 'consumers': [rust],
                    'required_cases': list(self.cases.values())}
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            scripts = root / 'sdlc/scripts'
            scripts.mkdir(parents=True)
            shutil.copyfile(parity.ROOT / 'sdlc/scripts/time-limit', scripts / 'time-limit')
            shutil.copyfile(parity.ROOT / 'rust-toolchain.toml', root / 'rust-toolchain.toml')
            (scripts / 'scratch.sh').write_text((parity.ROOT / 'sdlc/scripts/scratch.sh').read_text() + '\nusage_home() { :; }\n')
            library = root / 'libraries/rust'
            library.mkdir(parents=True)
            shutil.copyfile(parity.ROOT / 'libraries/rust/check.sh', library / 'check.sh')
            tools = root / 'bin'
            tools.mkdir()
            runner = tools / 'python3'
            runner.write_text('#!/bin/sh\nset -eu\n'
                              'test "$THINKTHEN_TEST_PROFILE" = full\n'
                              'test -z "${THINKTHEN_CONFORMANCE_IDS+x}"\n'
                              'test "$1" = -\ntest "$3" = "$2/libraries/rust/consumer/Cargo.toml"\n'
                              'cat > "$2/invoked-driver"\n'
                              'exec ' + shlex.quote(sys.executable) + ' "$2/invoked-driver" "$2" "$3"\n')
            children = root / 'conformance/children'
            children.mkdir(parents=True)
            (children / 'native_fixture.py').write_text(
                'from pathlib import Path\nimport tomllib\n'
                'def run(consumer, command, root, rust_manifest, extra_env):\n'
                ' assert consumer == "rust"\n'
                ' assert Path(command[0]).parent.is_dir()\n'
                ' assert rust_manifest == root / "libraries/rust/consumer/Cargo.toml"\n' +
                ' assert extra_env == {"RUSTUP_TOOLCHAIN": tomllib.loads((root / "rust-toolchain.toml").read_text())["toolchain"]["channel"]}\n' +
                ''.join(' print(' + repr(self.cell(case=case)) + ')\n' for case in self.cases) +
                ' return False\n')
            runner.chmod(0o755)
            with contextlib.redirect_stdout(io.StringIO()), patch.object(parity, 'ROOT', root), patch.object(parity, 'inventory', return_value=contract), patch.object(parity.os, 'environ', {'PATH': str(tools) + ':' + os.defpath}):
                self.assertEqual(parity.run('12345'), 0)

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


class CliApplicability(unittest.TestCase):
    def setUp(self):
        self.contract = json.loads(parity.DOCUMENT.read_text())['parity']
        self.cli = next(row for row in self.contract['consumers'] if row['id'] == 'cli')

    def test_cli_resolves_actual_boundary_checks_without_changing_sdk_or_mcp(self):
        original = copy.deepcopy(self.contract['required_cases'])
        parity.validate_consumer_contracts(self.contract)
        cli = parity.required_cases(self.contract, 'cli')
        self.assertEqual(len(cli), 244)
        self.assertEqual(len(parity.required_cases(self.contract, 'rust')), 255)
        self.assertEqual(set(parity.required_cases(self.contract, 'mcp')),
                         {row['id'] for row in original if row['tier'] == 'binding'
                          or row['id'] in parity.CORE_SAMPLE})
        self.assertTrue(all(case['checks'] == ['named', 'runtime'] for case in cli.values()))
        self.assertEqual(cli['declaration-null']['expect']['error'], 'local')
        self.assertEqual(cli['cancellation-held-call']['cli_boundary'], 'signal-drain')
        self.assertEqual(parity.required_cases(self.contract, 'rust')['typed-decide']['checks'],
                         ['named', 'compile', 'runtime'])
        self.assertEqual(original, self.contract['required_cases'])

    def test_image_ruling_retains_base_assertions_and_refuses_unknown_or_weakened_scenarios(self):
        identity = 'image-admission-packing-overflow-choose'
        case = parity.required_cases(self.contract, 'cli')[identity]
        original = next(row for row in self.contract['required_cases'] if row['id'] == identity)
        self.assertEqual(case['expect'], original['expect'])
        self.assertEqual(case['cli_scenario_rulings'], parity.CLI_IMAGE_SCENARIOS)
        for defect in ['scenario', 'variant', 'sends', 'exit', 'message', 'result']:
            changed = copy.deepcopy(self.contract)
            cli = next(row for row in changed['consumers'] if row['id'] == 'cli')
            scenarios = cli['case_rulings'][identity]['expect']['scenarios']
            variants = scenarios['perplexity-decider-complete-question-split']
            expect = variants['same_state_choose_candidate_orders']
            if defect == 'scenario':
                scenarios['unrelated'] = scenarios.pop('perplexity-decider-complete-question-split')
            elif defect == 'variant':
                variants['unrelated'] = variants.pop('same_state_choose_candidate_orders')
            else:
                field, value = {'sends': ('requests_sent', 1), 'exit': ('exit', 5),
                                'message': ('message', ''), 'result': ('no_result', False)}[defect]
                expect[field] = value
            with self.subTest(defect=defect), self.assertRaisesRegex(ValueError, 'closed scenario refusal'):
                parity.validate_consumer_contracts(changed)

    def test_closed_rulings_refuse_unrelated_exclusions_and_other_consumers(self):
        for defect in ['missing-all', 'missing', 'unrelated', 'wrong-boundary', 'other-consumer', 'checks', 'expect', 'signal-expect']:
            with self.subTest(defect=defect):
                changed = copy.deepcopy(self.contract)
                cli = next(row for row in changed['consumers'] if row['id'] == 'cli')
                if defect == 'missing-all':
                    del cli['required_checks']
                    del cli['case_rulings']
                elif defect == 'missing':
                    del cli['case_rulings']['named-uppercase']
                elif defect == 'unrelated':
                    cli['case_rulings']['typed-decide'] = {'boundary': 'sdk-only', 'reason': 'unsupported'}
                elif defect == 'wrong-boundary':
                    cli['case_rulings']['declaration-null']['boundary'] = 'sdk-only'
                elif defect == 'other-consumer':
                    changed['consumers'][1]['case_rulings'] = cli['case_rulings']
                elif defect == 'checks':
                    cli['required_checks'] = ['runtime']
                elif defect == 'signal-expect':
                    cli['case_rulings']['cancellation-held-call']['expect']['requests_sent'] = 2
                else:
                    cli['case_rulings']['declaration-null']['expect'] = {'error': 'Usage'}
                with self.assertRaises(ValueError):
                    parity.validate_consumer_contracts(changed)

    def test_cells_require_actual_cli_checks_and_refuse_sdk_only_passes(self):
        cases = {case['id']: case for case in self.contract['required_cases']}
        def cell(case_id, checks):
            return parity.PREFIX + json.dumps({'consumer': 'cli', 'case': case_id,
                                               'checks': checks, 'status': 'pass'})
        self.assertEqual(parity.cells(cell('typed-decide', ['named', 'runtime']), ['cli'], cases,
                                      self.contract), {('cli', 'typed-decide'): 'pass'})
        for case_id, checks, cause in [
            ('named-uppercase', ['named', 'runtime'], 'sdk-only CLI case'),
            ('finite-all-before-send', ['named', 'runtime'], 'sdk-only CLI case'),
            ('typed-decide', ['named', 'compile', 'runtime'], 'missing named/compiler/runtime'),
            ('typed-decide', ['runtime'], 'missing named/compiler/runtime'),
        ]:
            with self.subTest(case_id=case_id, checks=checks), self.assertRaisesRegex(ValueError, cause):
                parity.cells(cell(case_id, checks), ['cli'], cases, self.contract)

    def test_matrix_keeps_rulings_separate_and_analogue_missing_when_unexecuted(self):
        with tempfile.TemporaryDirectory() as scratch:
            cli_contract = self.contract | {'consumers': [self.cli], 'pending_consumers': []}
            def command(args, **kwargs):
                return subprocess.CompletedProcess(args, 0)
            with contextlib.redirect_stdout(io.StringIO()), patch.object(parity, 'ROOT', Path(scratch)), patch.object(parity, 'inventory', return_value=cli_contract), patch.object(parity.subprocess, 'run', side_effect=command), patch.object(parity.os, 'environ', {}):
                self.assertEqual(parity.run('12345'), 1)
            row = json.loads((Path(scratch) / 'target/parity/matrix.json').read_text())[0]
            self.assertEqual(row['case_rulings'], self.cli['case_rulings'])
            self.assertNotIn('named-uppercase', row['cells'])
            self.assertEqual(row['cells']['declaration-null'], 'missing')
            table = (Path(scratch) / 'target/parity/matrix.md').read_text()
            self.assertIn('named-uppercase: sdk-only, outside CLI', table)
            self.assertIn('declaration-null: question-file, boundary coverage missing', table)


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
                  ['go', 'csharp', 'jvm', 'cpp', 'swift', 'zig', 'php',
                   'flutter', 'ada', 'cobol', 'sqlite', 'duckdb', 'postgresql16']]
        for name in names:
            (folder / name).touch()
        # The plain diagnostic gem is shipped alongside the actual native gem.
        (folder / 'thinkthen-0.2.0.gem').touch()

    def select(self, folder):
        with patch.object(parity.sys, 'platform', 'linux'), patch.object(parity.subprocess, 'check_output', return_value='host: x86_64-unknown-linux-gnu\n'):
            return parity.installed_artifacts(folder, self.consumers)

    def test_each_group_uses_its_actual_package_and_shared_native_inputs(self):
        with tempfile.TemporaryDirectory() as scratch:
            folder = Path(scratch)
            self.packages(folder)
            inputs, command, native = self.select(folder)
            self.assertEqual(set(inputs), set(self.consumers) - {'objective-c'})
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

    def test_apple_requires_its_objective_c_archive(self):
        with tempfile.TemporaryDirectory() as scratch:
            folder = Path(scratch)
            for name in [self.command, self.native, self.dart]:
                (folder / name.replace('x86_64-unknown-linux-musl', 'aarch64-apple-darwin')
                 .replace('x86_64-unknown-linux-gnu', 'aarch64-apple-darwin')).touch()
            with patch.object(parity.sys, 'platform', 'darwin'), patch.object(
                    parity.subprocess, 'check_output', return_value='host: aarch64-apple-darwin\n'):
                consumers = {'objective-c': self.consumers['objective-c']}
                with self.assertRaisesRegex(ValueError, 'thinkthen-objective-c'):
                    parity.installed_artifacts(folder, consumers)
                archive = folder / 'thinkthen-objective-c-0.2.0-aarch64-apple-darwin.tar.gz'
                archive.touch()
                inputs, _, _ = parity.installed_artifacts(folder, consumers)
                self.assertEqual(inputs['objective-c']['THINKTHEN_ARTIFACT'], str(archive))

    def test_apple_execution_requires_every_foundation_cell(self):
        contract = parity.inventory()
        contract = contract | {'consumers': [self.consumers['objective-c']], 'pending_consumers': []}
        with tempfile.TemporaryDirectory() as scratch:
            def command(args, **kwargs):
                self.assertIn('libraries/objective-c/check.sh', args)
                return subprocess.CompletedProcess(args, 0)
            with contextlib.redirect_stdout(io.StringIO()), patch.object(parity.sys, 'platform', 'darwin'), patch.object(parity, 'ROOT', Path(scratch)), patch.object(parity, 'inventory', return_value=contract), patch.object(parity.subprocess, 'run', side_effect=command), patch.object(parity.os, 'environ', {}):
                self.assertEqual(parity.run('12345'), 1)
            row = json.loads((Path(scratch) / 'target/parity/matrix.json').read_text())[0]
            self.assertNotIn('platform_pending', row)
            self.assertEqual(set(row['cells'].values()), {'missing'})

    def test_linux_keeps_apple_cells_pending_without_hiding_required_failures(self):
        contract = parity.inventory()
        contract = contract | {'consumers': [self.consumers[key] for key in ('objective-c', 'c', 'python')],
                               'pending_consumers': []}
        binding_case = 'typed-decide'
        self.assertEqual(parity.required_cases(contract, 'python')[binding_case]['tier'], 'binding')
        for missing in (None, 'c', 'python'):
            with self.subTest(missing=missing), tempfile.TemporaryDirectory() as scratch:
                def command(args, **kwargs):
                    self.assertNotIn('libraries/objective-c/check.sh', args)
                    consumer = 'python' if 'libraries/python/check.sh' in args else 'c'
                    required = parity.required_cases(contract, consumer)
                    omitted = binding_case if consumer == 'python' else next(iter(required))
                    for key, case in required.items():
                        if consumer == missing and key == omitted:
                            continue
                        kwargs['stdout'].write(parity.PREFIX + json.dumps({
                            'consumer': consumer, 'case': case['id'], 'status': 'pass',
                            'checks': case.get('checks', ['named', 'runtime'])}) + '\n')
                    return subprocess.CompletedProcess(args, 0)
                with contextlib.redirect_stdout(io.StringIO()), patch.object(parity.sys, 'platform', 'linux'), patch.object(parity, 'ROOT', Path(scratch)), patch.object(parity, 'inventory', return_value=contract), patch.object(parity.subprocess, 'run', side_effect=command), patch.object(parity.os, 'environ', {}):
                    self.assertEqual(parity.run('12345'), 1)
                matrix = json.loads((Path(scratch) / 'target/parity/matrix.json').read_text())
                for consumer in ('c', 'python'):
                    row = next(row for row in matrix if row['consumer'] == consumer)
                    required = parity.required_cases(contract, consumer)
                    omitted = binding_case if consumer == 'python' else next(iter(required))
                    self.assertEqual(row['cells'], {key: 'missing' if consumer == missing and key == omitted
                                                   else 'pass' for key in required})
                apple = next(row for row in matrix if row['consumer'] == 'objective-c')
                self.assertEqual(set(apple['cells']), set(parity.required_cases(contract, 'objective-c')))
                self.assertEqual(set(apple['cells'].values()), {'platform-pending'})
                self.assertIsNone(apple['named_typed_functions'])
                table = (Path(scratch) / 'target/parity/matrix.md').read_text()
                self.assertIn('Apple qualification pending', table)
                self.assertIn('no local pass', table)

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
    def test_binding_builds_run_only_in_release_and_fail_the_release_on_error(self):
        source = parity.ROOT / 'sdlc/scripts'
        with tempfile.TemporaryDirectory(prefix="thinkthen-suite-routing-") as folder:
            root = Path(folder)
            scripts = root / "sdlc/scripts"
            scripts.mkdir(parents=True)
            for name in ("test", "test-full-cases", "scratch.sh", "full-compiler"):
                shutil.copyfile(source / name, scripts / name)
            (scripts / "heavy-lock").write_text("")
            # Use the real scratch ownership/cleanup, with no host configuration reads.
            with (scripts / "scratch.sh").open("a") as stream:
                stream.write("\nusage_guard() { :; }\nconfig_home() { :; }\n")
            fake = root / "bin"
            fake.mkdir()
            log = root / "calls"
            command = '''#!/bin/sh
printf '%s %s\\n' "${0##*/}" "$*" >> "$ROUTING_LOG"
case ${0##*/} in
python3) case $* in *--tool-environment*) printf "THINKTHEN_TOOLCHAINS=%s/tools\\n" "$HOME" ;; esac ;;
uname) echo Darwin ;;
sh) case $* in *sdlc/scripts/smoke*) exit "$ROUTING_SMOKE_CODE" ;; esac ;;
esac
'''
            for name in ("cargo", "python3", "sh", "uname"):
                path = fake / name
                path.write_text(command)
                path.chmod(0o755)
            demo = root / "demos/16-triage-pipeline/self-test"
            demo.parent.mkdir(parents=True)
            demo.write_text("#!/bin/sh\nexit 0\n")
            demo.chmod(0o755)
            env = {"PATH": f"{fake}:/usr/bin:/bin", "HOME": folder,
                   "ROUTING_LOG": str(log), "ROUTING_SMOKE_CODE": "0"}

            def run(entry, *args):
                log.write_text("")
                result = subprocess.run(["/bin/sh", str(scripts / entry), *args],
                                        cwd=root, env=env, capture_output=True,
                                        text=True, timeout=5)
                return result, log.read_text().splitlines()

            routine, calls = run("test")
            self.assertEqual(routine.returncode, 0, routine.stderr)
            self.assertFalse(any("sdlc/scripts/smoke" in call or "--ignored" in call
                                 or "sdlc/scripts/surfaces" in call for call in calls), calls)
            self.assertEqual(sum("cargo nextest run" in call for call in calls), 2)
            release, calls = run("test-full-cases", "--run")
            self.assertEqual(release.returncode, 0, release.stderr)
            self.assertFalse(any('nextest' in call or 'sdlc/scripts/test ' in call for call in calls), calls)
            self.assertEqual(sum("sdlc/scripts/smoke" in call for call in calls), 1)
            self.assertEqual(sum("--ignored" in call for call in calls), 2)
            self.assertEqual(sum("sdlc/scripts/surfaces --full-functional" in call
                                 for call in calls), 1)
            env["ROUTING_SMOKE_CODE"] = "9"
            failed, _ = run("test-full-cases", "--run")
            self.assertEqual(failed.returncode, 1, failed.stderr)
            self.assertIn("a binding smoke failed (exit 9)", failed.stderr)


    def test_required_parity_keeps_private_safety_and_release_checks_without_duplicate_consumers(self):
        for entry, args in [('test-full-cases', ['--run']),
                            ('surfaces', ['--full-functional']), ('surfaces', ['--parity']),
                            ('surfaces', ['--full-functional', '--publish', 'checkpoint/surfaces/test',
                                          '--publish-root', '/unused-publication-root']),
                            ('test-full-cases', ['--run', '--artifacts', '.']),
                            ('surfaces', ['--full-functional', '--artifacts', '.'])]:
            installed = '--artifacts' in args
            failures = [None, 'safety', 'parity'] + ([] if installed else ['smoke'])
            if '--publish' in args:
                failures.append('docs')
            for failed in failures:
                with self.subTest(entry=entry, failed=failed), tempfile.TemporaryDirectory() as folder:
                    root = Path(folder)
                    scripts = root / 'sdlc/scripts'
                    scripts.mkdir(parents=True)
                    for name in ['test-full-cases', 'surfaces', 'verdict.sh', 'full-compiler']:
                        shutil.copyfile(parity.ROOT / 'sdlc/scripts' / name, scripts / name)
                    files = {
                        'sdlc/scripts/heavy-lock': '',
                        'sdlc/scripts/scratch.sh': (
                            'usage_guard() { :; }\n'
                            'config_home() { XDG_CONFIG_HOME=$PWD/config; export XDG_CONFIG_HOME; }\n'
                            'temp_home() { :; }\n'
                            'scratch_dir() { eval "$1=\\\"$PWD/$1\\\""; mkdir -p "$PWD/$1"; }\n'
                            'scratch_clean() { :; }\n'),
                        'sdlc/scripts/installed.sh': 'backend_start() { port=12345; }\n',
                        'sdlc/scripts/test': 'printf "workspace\\n" >> calls\n',
                        'sdlc/scripts/smoke': 'printf "binding-smoke\\n" >> calls\n',
                        'sdlc/surfaces.txt': 'libraries/c landed\n',
                        'libraries/c/Cargo.toml': '',
                        'libraries/c/check.sh': 'printf "safety\\n" >> calls\nexit "$SAFETY_CODE"\n',
                        'sdlc/scripts/release-pack': 'if [ "${4:-}" = crate ]; then printf "extra-pack\\n" >> calls; else printf "pack\\n" >> calls; fi\n',
                        'sdlc/scripts/release-smoke': 'printf "smoke\\n" >> calls\nexit "$SMOKE_CODE"\n',
                        'sdlc/scripts/publish-builds': 'if [ "$1" = --check ]; then printf "stage-check\\n" >> calls; else printf "stage\\n" >> calls; fi\n',
                        'bin/git': '#!/bin/sh\nif [ "$1" = rev-parse ]; then printf "source-hash\\n"; fi\n',
                        'bin/rustc': '#!/bin/sh\nprintf "host: x86_64-unknown-linux-gnu\\n"\n',
                        'site/package.json': '{}\n',
                        'bin/npm': (
                            '#!/bin/sh\nset -eu\n'
                            'test "$PWD" = "$FIXTURE_ROOT/site"\n'
                            'test "$*" = "run test-docs"\n'
                            'printf "docs\\n" >> "$FIXTURE_ROOT/calls"\n'
                            'exit "$DOCS_CODE"\n'),
                        'bin/cargo': (
                            '#!/bin/sh\nset -eu\n'
                            'test "$XDG_CONFIG_HOME" = "$PWD/config"\n'
                            'case "$*" in\n'
                            '"test --locked --offline --workspace --all-targets release_only_ -- --ignored") '
                            'printf "release-workspace\\n" >> calls ;;\n'
                            '"test --locked --offline --manifest-path conformance/consumer/Cargo.toml '
                            '--target-dir target/consumer --workspace release_only_ -- --ignored") '
                            'printf "release-consumer\\n" >> calls ;;\n'
                            '*) exit 2 ;;\nesac\n'),
                        'conformance/parity.py': (
                            'import os,sys\n'
                            'if sys.argv[1:] == ["--tool-environment"]: print("THINKTHEN_TOOLCHAINS=" + os.environ["HOME"] + "/tools"); sys.exit(0)\n'
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
                    (root / 'bin/cargo').chmod(0o755)
                    (root / 'bin/npm').chmod(0o755)
                    env = {'PATH': str(root / 'bin') + ':' + os.defpath,
                           'HOME': str(root / 'home'), 'SAFETY_CODE': '77' if failed == 'safety' else '0',
                           'PARITY_CODE': '1' if failed == 'parity' else '0',
                           'SMOKE_CODE': '77' if failed == 'smoke' else '0',
                           'DOCS_CODE': '1' if failed == 'docs' else '0',
                           'FIXTURE_ROOT': str(root),
                           'INSTALLED': 'yes' if installed else '',
                           'THINKTHEN_CONFORMANCE_IDS': '/unwanted-selector'}
                    result = subprocess.run(['sh', str(scripts / entry), *args], cwd=root,
                                            env=env, capture_output=True, text=True, check=False)
                    expected = ['safety'] if failed == 'safety' else ['safety', 'parity']
                    if failed not in ('safety', 'parity') and not installed:
                        expected += ['pack', 'smoke']
                    if entry == 'test-full-cases':
                        expected[:0] = ['release-workspace', 'release-consumer']
                    if '--publish' in args:
                        expected.insert(0, 'stage-check')
                        if failed in (None, 'docs'):
                            expected += ['extra-pack', 'docs']
                        if failed is None:
                            expected += ['stage']
                    calls = (root / 'calls').read_text().splitlines()
                    self.assertEqual(calls.count('binding-smoke'), int(entry == 'test-full-cases'))
                    self.assertEqual([call for call in calls if call != 'binding-smoke'], expected, result.stderr)
                    self.assertEqual(result.returncode == 0, failed is None, result.stdout + result.stderr)
                    if failed == 'docs':
                        self.assertIn('documentation replay did not pass', result.stdout)


if __name__ == '__main__':
    unittest.main()
