#!/usr/bin/env python3
"""Exercise the size CLI on an isolated tracked library history."""
import json
import os
from pathlib import Path
import subprocess
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / 'sdlc/scripts/library-size'


class SizeReport(unittest.TestCase):
    def test_disjoint_counts_ratios_history_and_opt_in_storage(self):
        with tempfile.TemporaryDirectory(prefix='library-size-') as folder:
            root = Path(folder)
            env = dict(os.environ, HOME=folder, GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL=os.devnull)
            def run(*args):
                return subprocess.check_output(args, cwd=root, env=env, text=True, stderr=subprocess.PIPE)
            def write(path, text):
                target = root / path
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_text(text)
            libraries = {name: {'root': 'libraries/' + name, 'test_folders': ['tests']}
                         for name in ['python', 'jvm', 'dart']}
            manifest = {'libraries': libraries, 'exceptions': {'python': {'paths': ['libraries/python/adapter.py']}}}
            write('conformance/library-surface.json', json.dumps(manifest))
            write('sdlc/generators/results/registry.py', "def generated_paths():\n    return {'libraries/python/generated.py'}\n")
            write('libraries/python/main.py', 'one\n\ntwo\n')
            write('libraries/python/adapter.py', 'adapter\n')
            write('libraries/python/generated.py', 'generated\n' * 5)
            write('libraries/python/tests/check.py', 'test\n' * 4)
            write('libraries/jvm/session/Test.java', 'java\n' * 2)
            write('libraries/jvm/session/Test.kt', 'kotlin\n')
            write('libraries/jvm/session/Test.scala', 'scala\n')
            write('libraries/dart/main.dart', 'dart\n' * 2)
            write('libraries/dart/tests/native.mjs', 'test\n' * 3)
            write('libraries/dart/tests/native.d.mts', 'declaration\n' * 2)
            write('libraries/dart/src/tests.rs', 'rust test\n')
            write('libraries/jvm/tests/helper.py', 'python test\n' * 2)
            run('git', 'init', '-q')
            run('git', 'config', 'user.name', 'Fixture')
            run('git', 'config', 'user.email', 'fixture@example.invalid')
            run('git', 'add', '.')
            run('git', 'commit', '-qm', 'Create fixture')
            run('git', 'tag', 'v1.0.0')
            write('libraries/python/main.py', 'one\n\ntwo\nthree\n\n')
            write('libraries/python/generated.py', 'generated\n' * 7)
            write('libraries/python/tests/check.py', 'test\n' * 8)
            write('libraries/python/untracked.py', 'ignore\n' * 100)
            run('git', 'add', 'libraries/python/main.py', 'libraries/python/generated.py', 'libraries/python/tests/check.py')
            run('git', 'commit', '-qm', 'Add handwritten behavior')
            report = json.loads(run(str(SCRIPT), '--root', str(root), '--json'))
            python = report['libraries']['python']
            self.assertEqual((python['handwritten'], python['generated'], python['test'], python['exception']), (4, 7, 8, 1))
            self.assertEqual(report['medians']['handwritten'], 4)
            self.assertEqual(report['libraries']['dart']['ratios']['handwritten'], 0.5)
            self.assertEqual(report['libraries']['jvm']['languages']['kotlin']['handwritten'], 1)
            self.assertEqual(report['libraries']['jvm']['languages']['python']['test'], 2)
            self.assertEqual(report['libraries']['dart']['test'], 6)
            self.assertEqual(report['commits'][0]['handwritten_added']['python'], 1)
            self.assertFalse((root / 'sdlc/records').exists())
            run(str(SCRIPT), '--root', str(root), '--since', 'v1.0.0', '--store', 'candidate')
            self.assertTrue((root / 'sdlc/records/coverage/candidate-library-size.txt').is_file())
            failure = subprocess.run([str(SCRIPT), '--root', str(root), '--store', '../escape'], env=env, capture_output=True)
            self.assertEqual(failure.returncode, 2)
            self.assertFalse((root / 'sdlc/records/escape-library-size.txt').exists())
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
            dirty = json.loads(run(str(SCRIPT), '--root', str(root), '--json'))
            self.assertEqual(dirty, committed)



if __name__ == '__main__':
    unittest.main()
