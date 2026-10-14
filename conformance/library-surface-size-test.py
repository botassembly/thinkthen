#!/usr/bin/env python3
"""Exercise the size CLI on an isolated tracked library history."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / 'sdlc/scripts/library-size'
sys.path.insert(0, str(SCRIPT.parents[2]))
from conformance.children.children import child_env


class SizeReport(unittest.TestCase):
    def test_disjoint_counts_ratios_history_and_opt_in_storage(self):
        with tempfile.TemporaryDirectory(prefix='library-size-') as folder:
            root = Path(folder)
            env = child_env(home=folder, GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull)
            def run(*args):
                return subprocess.check_output(args, cwd=root, env=env, text=True, stderr=subprocess.PIPE)
            def write(path, text):
                target = root / path
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(text)
            contract = json.loads((SCRIPT.parents[2] / 'conformance/library-surface.json').read_text())
            libraries = {name: contract['libraries'][name] for name in ['python', 'jvm', 'dart']}
            manifest = {'libraries': libraries, 'databases': {'postgresql': contract['databases']['postgresql']}, 'exceptions': {'python': {'paths': ['libraries/python/adapter.py']}}}
            write('conformance/library-surface.json', json.dumps(manifest))
            write('sdlc/generators/results/registry.py', "from types import SimpleNamespace\nOUTPUTS = (SimpleNamespace(target='python', path='libraries/python/generated.py'), SimpleNamespace(target='python', path='sdlc/generators/side/inputs.json'), SimpleNamespace(target='postgresql', path='databases/postgresql/src/results_generated.rs'))\ndef generated_paths():\n    return {output.path for output in OUTPUTS}\n")
            write('sdlc/generators/results/sources.py', (SCRIPT.parent.parent / 'generators/results/sources.py').read_text())
            write('databases/postgresql/src/native.rs', 'native\n' * 2)
            write('databases/postgresql/src/results_generated.rs', 'generated\n' * 3)
            write('databases/postgresql/tests/native.rs', 'test\n' * 4)
            write('libraries/python/main.py', 'one\n\ntwo\n')
            write('libraries/python/adapter.py', 'adapter\n')
            write('libraries/python/generated.py', 'generated\n' * 5)
            write('sdlc/generators/side/inputs.json', '{\n}\n')
            write('libraries/python/tests/check.py', 'test\n' * 4)
            write('libraries/jvm/session/Test.java', 'java\n' * 2)
            write('libraries/jvm/session/Test.kt', 'kotlin\n')
            write('libraries/jvm/session/Test.scala', 'scala\n')
            write('libraries/dart/main.dart', 'dart\n' * 2)
            write('libraries/dart/checks/native.mjs', 'test\n' * 3)
            write('libraries/dart/checks/native.d.mts', 'declaration\n' * 2)
            write('libraries/dart/src/tests.rs', 'rust test\n')
            write('libraries/dart/flutter/example/lib/main.dart', 'sample\n' * 3)
            write('libraries/dart/checks/check.dart', 'test\n' * 2)
            write('libraries/dart/flutter/example/linux/flutter/generated_plugin_registrant.cc', 'generated\n' * 4)
            write('libraries/jvm/tests/helper.py', 'python test\n' * 2)
            run('git', 'init', '-q')
            run('git', 'config', 'user.name', 'Fixture')
            run('git', 'config', 'user.email', 'fixture@example.invalid')
            run('git', 'add', '.')
            run('git', 'commit', '-qm', 'Create fixture')
            run('git', 'tag', 'v1.0.0')
            write('databases/postgresql/src/native.rs', 'native\n' * 3)
            write('databases/postgresql/src/results_generated.rs', 'generated\n' * 6)
            write('libraries/python/main.py', 'one\n\ntwo\nthree\n\n')
            write('libraries/python/generated.py', 'generated\n' * 7)
            write('libraries/python/tests/check.py', 'test\n' * 8)
            write('libraries/python/untracked.py', 'ignore\n' * 100)
            run('git', 'add', 'libraries/python/main.py', 'libraries/python/generated.py', 'libraries/python/tests/check.py', 'databases')
            run('git', 'commit', '-qm', 'Add handwritten behavior')
            report = json.loads(run(str(SCRIPT), '--root', str(root), '--json'))
            python = report['libraries']['python']
            self.assertEqual((python['handwritten'], python['generated'], python['test'], python['exception']), (4, 9, 8, 1))
            self.assertEqual(report['medians']['handwritten'], 3.5)
            database = report['libraries']['postgresql']
            self.assertEqual((database['handwritten'], database['generated'], database['test']), (3, 6, 4))
            self.assertEqual(report['commits'][0]['handwritten_added']['postgresql'], 1)
            write('sdlc/scripts/ratchet.mjs', (SCRIPT.parent / 'ratchet.mjs').read_text())
            write('databases/postgresql/ratchet.json', json.dumps({
                'directory': ['src'], 'extension': '.rs', 'scope': 'handwritten-production', 'max': 3}))
            run('node', 'sdlc/scripts/ratchet.mjs', 'databases/postgresql/ratchet.json')
            write('databases/postgresql/src/results_generated.rs', 'generated\n' * 99)
            run('node', 'sdlc/scripts/ratchet.mjs', 'databases/postgresql/ratchet.json')
            write('databases/postgresql/src/native.rs', 'native\n' * 4)
            failure = subprocess.run(['node', 'sdlc/scripts/ratchet.mjs', 'databases/postgresql/ratchet.json'],
                                     cwd=root, env=env, text=True, capture_output=True)
            self.assertEqual(failure.returncode, 1)
            self.assertIn('is 4 non-blank lines, ceiling is 3', failure.stderr)
            write('databases/postgresql/src/native.rs', 'native\n' * 3)
            write('databases/postgresql/src/results_generated.rs', 'generated\n' * 6)
            self.assertEqual(report['libraries']['dart']['ratios']['handwritten'], 0.5714)
            self.assertEqual(report['libraries']['jvm']['languages']['kotlin']['handwritten'], 1)
            self.assertEqual(report['libraries']['jvm']['languages']['python']['test'], 2)
            self.assertEqual(report['libraries']['dart']['test'], 11)
            self.assertEqual(report['libraries']['dart']['generated'], 4)
            self.assertEqual(report['commits'][0]['handwritten_added']['python'], 1)
            self.assertFalse((root / 'sdlc/records').exists())
            run(str(SCRIPT), '--root', str(root), '--since', 'v1.0.0', '--store', 'candidate')
            record = root / 'sdlc/records/coverage/candidate-library-size.md'
            self.assertTrue(record.is_file())
            self.assertIn('| Library |', record.read_text())
            self.assertEqual(record.read_text().count(report['revision']), 1)
            self.assertLess(len(record.read_text().splitlines()), 20)
            failure = subprocess.run([str(SCRIPT), '--root', str(root), '--store', '../escape'], env=env, capture_output=True)
            self.assertEqual(failure.returncode, 2)
            self.assertFalse((root / 'sdlc/records/escape-library-size.md').exists())
            branch = run('git', 'symbolic-ref', '--short', 'HEAD').strip()
            run('git', 'switch', '-qc', 'release', 'v1.0.0')
            write('release.txt', 'separate release branch\n')
            run('git', 'add', 'release.txt')
            run('git', 'commit', '-qm', 'Release separately')
            run('git', 'tag', 'v1.0.1')
            run('git', 'switch', '-q', branch)
            run('git', 'switch', '-qc', 'feature')
            write('libraries/python/feature.py', 'feature\n' * 2)
            run('git', 'add', 'libraries/python/feature.py')
            run('git', 'commit', '-qm', 'Add feature')
            feature = run('git', 'rev-parse', 'HEAD').strip()
            run('git', 'switch', '-q', branch)
            run('git', 'merge', '--no-ff', '-qm', 'Merge feature', 'feature')
            run('git', 'tag', 'rc/99.0.0-rc.1')
            committed = json.loads(run(str(SCRIPT), '--root', str(root), '--json'))
            self.assertEqual(committed['since'], 'v1.0.1')
            changes = {row['commit']: row for row in committed['commits']}
            self.assertEqual(changes[feature]['handwritten_added']['python'], 2)
            self.assertEqual(committed['commits'][-1]['kind'], 'merge_resolution')
            self.assertEqual(committed['commits'][-1]['handwritten_added']['python'], 0)
            write('libraries/python/main.py', 'dirty\n' * 99)
            (root / 'libraries/python/generated.py').unlink()
            write('conformance/library-surface.json', '{}')
            write('sdlc/generators/results/registry.py', 'invalid Python')
            write('sdlc/generators/results/sources.py', 'invalid classifier')
            dirty = json.loads(run(str(SCRIPT), '--root', str(root), '--json'))
            self.assertEqual(dirty, committed)



if __name__ == '__main__':
    unittest.main()
