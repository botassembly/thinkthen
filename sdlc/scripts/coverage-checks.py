"""Focused regressions for coverage process ownership and missing profiles."""
import json
import os
from pathlib import Path
import runpy
import signal
import sys
import tempfile
import subprocess
import unittest

DRIVER = runpy.run_path(str(Path(__file__).with_name('coverage')))


class ProcessOwnership(unittest.TestCase):
    def test_timeout_stops_and_reaps_descendants_with_diagnostics(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            script = """import os, pathlib, time
child = os.fork()
if child == 0:
    print('descendant diagnostic', flush=True)
    time.sleep(30)
else:
    pathlib.Path('child.pid').write_text(str(child))
    time.sleep(30)
"""
            for capture in (False, True):
                with self.assertRaisesRegex(RuntimeError, r'timed out \(exit -9\)'):
                    DRIVER['command']([sys.executable, '-c', script], root, os.environ,
                                      root / 'timeout.log', capture=capture, timeout=0.3)
                child = int((root / 'child.pid').read_text())
                with self.assertRaises(ProcessLookupError):
                    os.kill(child, 0)
                self.assertIn('descendant diagnostic', (root / 'timeout.log').read_text())

    def test_real_exit_and_diagnostics_survive(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for code, exception, message in ((77, DRIVER['MissingTool'], 'see'),
                                             (4, RuntimeError, 'exit 4'),
                                             (-signal.SIGTERM, RuntimeError, 'exit -15')):
                script = f"import os, sys; print('diagnostic', flush=True); "
                script += f'os.kill(os.getpid(), {-code})' if code < 0 else f'sys.exit({code})'
                with self.assertRaisesRegex(exception, message):
                    DRIVER['command']([sys.executable, '-c', script], root, os.environ, root / 'exit.log')
                self.assertIn('diagnostic', (root / 'exit.log').read_text())


class ProfileCauses(unittest.TestCase):
    def test_real_signals_and_file_limits_remain_unmeasured(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            collection = root / 'rust-integration'
            collection.mkdir()
            source = Path(__file__).with_name('coverage-profile.c').resolve()
            subprocess.run(['cc', '-fPIC', '-Wall', '-Wextra', '-Werror', '-c', str(source),
                            '-DPROFILE_PATH=' + json.dumps(str(collection / '%m.profraw')),
                            '-DPROFILE_DIR=' + json.dumps(str(collection)), '-o', str(root / 'profile.o')], check=True)
            program = 'fn main() { let a: Vec<_> = std::env::args().collect(); if a.len() > 1 { unsafe { unsafe extern "C" { fn raise(s: i32) -> i32; } raise(a[1].parse().unwrap()); } } }'
            subprocess.run(['rustc', '--edition', '2024', '-C', 'instrument-coverage', '-C',
                            'link-arg=' + str(root / 'profile.o'), '-o', str(root / 'probe'), '-'],
                           input=program.encode(), check=True)
            env = {'PATH': os.environ['PATH']}
            for signum in (signal.SIGINT, signal.SIGTERM):
                result = subprocess.run([str(root / 'probe'), str(signum)], cwd=root, env=env, capture_output=True)
                self.assertEqual(result.returncode, -signum)
                self.assertEqual(result.stderr, b'')
            result = subprocess.run(['sh', '-c', 'ulimit -f 1; exec "$1"', 'limit', str(root / 'probe')],
                                    cwd=root, env=env, capture_output=True)
            self.assertEqual(result.returncode, 0)
            self.assertEqual(result.stderr, b'')
            subprocess.run([str(root / 'probe')], cwd=root, env=env, check=True)
            self.assertFalse((root / 'default.profraw').exists())
            self.assertEqual(len(list(collection.glob('limited-fsize-*'))), 1)
            missing = [p for p in collection.glob('process-*') if not (p / 'normal-exit').exists()]
            self.assertEqual(len(missing), 2)
            self.assertTrue(list(collection.glob('*.profraw')))
            (root / 'crates/thinkthen/src').mkdir(parents=True)
            (root / 'crates/thinkthen/src/probe.rs').write_text('line one\nline two\n')
            lcov = f'SF:{root}/crates/thinkthen/src/probe.rs\nDA:1,0\nDA:2,0\n'
            for name in ('cli-source', 'source-unit'):
                (root / name).mkdir()
            DRIVER['report'](root, root, 'probe', lcov,
                             {name: lcov for name in ('rust-integration', 'cli-source', 'source-unit')}, 0)
            report = json.loads((root / 'totals.json').read_text())
            self.assertEqual(report['totals']['unobserved'], 2)
            self.assertNotIn('unreached', report['totals'])
            self.assertNotIn('unit_only', report['totals'])
            self.assertEqual(len(report['missing_child_profiles']['rust-integration']['no_normal_exit']), 2)


class CollectionClassification(unittest.TestCase):
    def test_c_library_target_launches_with_its_metadata_identity(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            package_dir = root / 'libraries/c'
            package_dir.mkdir(parents=True)
            harness = root / 'c-harness'
            harness.write_text(f'#!{sys.executable}\n' + "import os, pathlib, sys\n"
                               "assert os.environ['CARGO_PKG_NAME'] == 'thinkthen-c'\n"
                               "assert pathlib.Path.cwd() == pathlib.Path(os.environ['CARGO_MANIFEST_DIR'])\n"
                               "assert sys.argv[1:] == ['--test-threads=2']\n")
            harness.chmod(0o700)
            package = {'name': 'thinkthen-c', 'manifest_path': str(package_dir / 'Cargo.toml')}
            target = {'kind': ['cdylib', 'staticlib'], 'name': 'thinkthen_c'}
            artifact = {'reason': 'compiler-artifact', 'target': target,
                        'executable': str(harness), 'profile': {'test': True}}
            production = {**artifact, 'profile': {'test': False}}
            artifacts = DRIVER['test_artifacts']('\n'.join(map(json.dumps, (artifact, production))), package)
            self.assertEqual(len(artifacts), 1)
            DRIVER['collect_tests'](root, {}, artifacts, 'c-source-unit')
            self.assertTrue((root / 'c-source-unit/thinkthen_c.log').is_file())

    def test_c_source_units_add_unit_hits_without_test_only_denominator(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / 'libraries/c/src/probe.rs'
            source.parent.mkdir(parents=True)
            source.write_text('production one\nproduction two\ntest only\n')
            production = f'SF:{source}\nDA:1,0\nDA:2,0\n'
            c_units = f'SF:{source}\nDA:1,1\nDA:2,1\nDA:3,1\n'
            outside = f'SF:{source}\nDA:1,1\nDA:2,0\n'
            collections = {name: production for name in DRIVER['NATIVE_COLLECTIONS']}
            collections.update({'c-source-unit': c_units, 'rust-integration': outside})
            for name in collections:
                (root / name).mkdir()
            DRIVER['report'](root, root, 'probe', production, collections, 0)
            totals = json.loads((root / 'totals.json').read_text())['totals']
            self.assertEqual(totals['mapped_nonblank'], 2)
            self.assertEqual(totals['c-source-unit'], 2)
            self.assertEqual(totals['observed_unit_only'], 1)
            self.assertEqual(totals['unobserved'], 0)


if __name__ == '__main__':
    unittest.main()
