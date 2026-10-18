#!/usr/bin/env python3
"""Exercise missing-tool refusal and PATH isolation through the resolver command."""
import json
import os
import platform
import shutil
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / 'sdlc/scripts/tools.py'


class ToolResolution(unittest.TestCase):
    def test_reports_every_missing_tool_before_starting_the_command(self):
        self.check(['thinkthen-absent-first', 'thinkthen-absent-second'], False)

    def test_ignores_an_inherited_tool_path(self):
        self.check(['thinkthen-inherited-tool'], False, inherited=True)

    def test_starts_the_command_after_system_tools_resolve(self):
        self.check(['sh'], True)

    def test_reports_a_missing_dated_rust_toolchain_before_starting_work(self):
        pin = json.loads((ROOT / 'sdlc/tools.json').read_text())['tools']['rust-nightly']
        self.check([], False, requirement={**pin, 'version': 'nightly-1900-01-01',
                                         'groups': ['test']}, diagnostic='selected')

    @unittest.skipUnless(platform.system() == 'Darwin' and
                         'test' in os.environ.get('THINKTHEN_RESOLVED_TOOLS', '').split(','),
                         'requires the prepared native macOS test environment')
    def test_refuses_a_hidden_manifest_tool_before_starting_work(self):
        tool = Path(shutil.which('jq')).resolve()
        self.assertIn(ROOT / '.tools', tool.parents)
        hidden = tool.with_name(tool.name + '.hidden')
        tool.rename(hidden)
        try:
            result = subprocess.run([sys.executable, str(SCRIPT), '--check', 'test', '--',
                                     '/bin/sh', '-c', 'echo command-started'],
                                    text=True, capture_output=True)
        finally:
            hidden.rename(tool)
        self.assertEqual(result.returncode, 2, result.stderr)
        self.assertIn('jq', result.stderr)
        self.assertNotIn('command-started', result.stdout)

    def check(self, commands, succeeds, inherited=False, requirement=None, diagnostic=None):
        owned = ROOT / '.tools'
        owned.mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='resolver-test-', dir=owned) as folder:
            root = Path(folder)
            fake = root / 'inherited'
            fake.mkdir()
            tool = fake / 'thinkthen-inherited-tool'
            tool.write_text('#!/bin/sh\nexit 0\n')
            tool.chmod(0o755)
            manifest = {
                'libraries': {}, 'databases': {}, 'linux_prefix': str(root),
                'tools': {
                    'rust': {'commands': [], 'version': 'unused', 'groups': ['common'],
                             'platforms': ['darwin', 'linux']},
                    'selected': requirement or {'commands': commands, 'groups': ['test'],
                                                'platforms': ['darwin', 'linux']},
                },
            }
            source = root / 'tools.json'
            source.write_text(json.dumps(manifest))
            env = {'HOME': str(root), 'PATH': str(fake) if inherited else '/usr/bin:/bin'}
            result = subprocess.run([sys.executable, str(SCRIPT), '--check', '--manifest', str(source),
                                     '--root', str(root / 'resolved'), 'test', '--',
                                     '/bin/sh', '-c', 'echo command-started'],
                                    env=env, text=True, capture_output=True)
            self.assertEqual(result.returncode, 0 if succeeds else 2, result.stderr)
            self.assertEqual(result.stdout, 'command-started\n' if succeeds else '')
            if not succeeds:
                self.assertIn('missing tools:', result.stderr)
                for command in commands:
                    self.assertIn(command, result.stderr)
                if diagnostic:
                    self.assertIn(diagnostic, result.stderr)


class LinuxSourceIdentity(unittest.TestCase):
    def test_reused_clone_discards_source_edits_and_keeps_build_output(self):
        with tempfile.TemporaryDirectory(prefix='linux-source-') as folder:
            root = Path(folder)
            for relative in ('sdlc/scripts/linux-check', 'sdlc/tools.json',
                             'sdlc/linux/Dockerfile', 'sdlc/linux/install.py',
                             'sdlc/scripts/tools.py', 'databases/duckdb/tools/version.env'):
                destination = root / relative
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / relative, destination)
            (root / '.gitignore').write_text('.linux-*/\ntarget/\n')
            (root / 'source.txt').write_text('committed\n')
            git = ['git', '-C', str(root)]
            subprocess.run([*git, 'init', '-q'], check=True)
            subprocess.run([*git, 'add', '.'], check=True)
            subprocess.run([*git, '-c', 'user.name=Test', '-c', 'user.email=test@example.invalid',
                            'commit', '-qm', 'Create source fixture'], check=True)
            binary = root / 'fake-bin'
            binary.mkdir()
            docker = binary / 'docker'
            docker.write_text('#!/bin/sh\n'
                              'if [ "$1" = image ]; then echo sha256:test; fi\n')
            docker.chmod(0o755)
            env = dict(os.environ, PATH=str(binary) + os.pathsep + os.environ['PATH'])
            command = [sys.executable, str(root / 'sdlc/scripts/linux-check'), 'test']
            first = subprocess.run(command, env=env, capture_output=True, text=True)
            self.assertEqual(first.returncode, 0, first.stderr)
            source = next(root.glob('.linux-*/src'))
            (source / 'source.txt').write_text('left by previous check\n')
            (source / 'stray.txt').write_text('untracked source\n')
            (source / 'target').mkdir()
            (source / 'target/cached').write_text('build output\n')
            second = subprocess.run(command, env=env, capture_output=True, text=True)
            self.assertEqual(second.returncode, 0, second.stderr)
            self.assertEqual((source / 'source.txt').read_text(), 'committed\n')
            self.assertFalse((source / 'stray.txt').exists())
            self.assertEqual((source / 'target/cached').read_text(), 'build output\n')


if __name__ == '__main__':
    unittest.main()
