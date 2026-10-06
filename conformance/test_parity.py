"""Behavior regressions for the shared consumer assertion/table interface."""
import json
from pathlib import Path
import subprocess
import contextlib
import io
import shutil
import sys
import tempfile
import unittest
from unittest.mock import patch
from conformance import parity


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

    def test_missing_cells_remain_missing_and_failed_process_invalidates_pass(self):
        seen = parity.cells(self.cell(), ['rust'], self.cases)
        row = parity.summarize('rust', self.cases, 0, seen, None, None)
        self.assertEqual(row['cells'], {'typed-decide': 'pass', 'files-decide': 'missing'})
        for code in [1, 77, 127]:
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
        }):
            env = parity.consumer_environment(scratch, '12345')
            self.assertEqual(env['THINKTHEN_BASE_URL'], 'http://127.0.0.1:12345/generic/v1')
            self.assertEqual(env['THINKTHEN_API_KEY'], 'sk-conformance-loopback')
            self.assertEqual(env['THINKTHEN_TEST_PROFILE'], 'full')
            for name in ['HOME', 'XDG_CONFIG_HOME', 'XDG_CACHE_HOME', 'XDG_STATE_HOME', 'APPDATA']:
                self.assertTrue(Path(env[name]).is_relative_to(scratch))
            self.assertNotIn('THINKTHEN_CONFORMANCE_IDS', env)
            self.assertNotIn('OTHER_API_KEY', env)

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


if __name__ == '__main__':
    unittest.main()
