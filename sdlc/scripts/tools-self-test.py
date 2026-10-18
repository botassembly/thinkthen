#!/usr/bin/env python3
"""Exercise missing-tool refusal and PATH isolation through the resolver command."""
import json
import os
import platform
import shutil
import shlex
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / 'sdlc/scripts/tools.py'
sys.path.insert(0, str(ROOT))
from conformance.source_fixtures import stage_checkout


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

    def test_reports_an_unavailable_cargo_subcommand_before_starting_work(self):
        self.check(['sh'], False, requirement={
            'commands': ['sh'], 'groups': ['test'], 'platforms': ['darwin', 'linux'],
            'probes': [{'argv': ['sh', '-c', 'exit 1'], 'contains': 'cargo fmt'}],
        }, diagnostic='selected')

    def test_reports_unprepared_advisories_before_starting_work(self):
        self.check(['sh'], False, requirement={
            'commands': ['sh'], 'groups': ['test'], 'platforms': ['darwin', 'linux'],
            'readiness': {'argv': ['sh', '-c', 'exit 1'], 'contains': 'advisories ok'},
        }, diagnostic='selected')

    @unittest.skipUnless(platform.system() == 'Darwin', 'macOS owns native Cargo resolution')
    def test_cargo_dispatch_ignores_ambient_plugins_and_aliases_but_shares_cache_locks(self):
        cargo = subprocess.check_output(['rustup', 'which', 'cargo'], text=True).strip()
        owned = ROOT / '.tools'
        owned.mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='resolver-cargo-', dir=owned) as folder:
            root = Path(folder)
            resolved = root / 'resolved'
            binary = resolved / 'bin'
            ambient = root / 'ambient'
            binary.mkdir(parents=True)
            (ambient / 'bin').mkdir(parents=True)
            (binary / 'cargo').symlink_to(cargo)
            plugins = ('fmt', 'clippy', 'nextest', 'deny', 'public-api')
            for name in plugins:
                for directory, output in ((binary, 'owned-'), (ambient / 'bin', 'ambient-')):
                    executable = directory / ('cargo-' + name)
                    executable.write_text('#!/bin/sh\nprintf "%s\\n" ' + output + name + '\n')
                    executable.chmod(0o755)
            (ambient / 'config.toml').write_text('[alias]\nnextest = "!echo ambient-config"\n')
            (ambient / 'credentials.toml').write_text('fixture credential, never forwarded\n')
            caches = ('registry', 'git', 'advisory-dbs')
            locks = ('.package-cache', '.package-cache-mutate', '.global-cache')
            (ambient / 'registry').mkdir()
            (ambient / '.package-cache').write_text('shared fixture\n')
            manifest = {'libraries': {}, 'databases': {}, 'python_packages': {}, 'tools': {
                'rust': {'commands': [], 'version': 'unused', 'groups': ['common'],
                         'platforms': ['darwin']},
                'plugins': {'commands': ['cargo', *['cargo-' + name for name in plugins]],
                            'groups': ['test'], 'platforms': ['darwin'],
                            'probes': [{'argv': ['cargo', name, '--version'],
                                        'contains': 'owned-' + name} for name in plugins]}}}
            source = root / 'tools.json'
            source.write_text(json.dumps(manifest))
            env = {'HOME': str(root), 'CARGO_HOME': str(ambient), 'PATH': '/usr/bin:/bin',
                   'CARGO_ALIAS_NEXTEST': '!echo ambient-env'}
            command = '; '.join('cargo ' + name + ' --version' for name in plugins)
            argv = [sys.executable, str(SCRIPT), '--check', '--manifest', str(source),
                    '--root', str(resolved), 'test', '--', '/bin/sh', '-ec', command]
            children = [subprocess.Popen(argv, env=env, text=True, stdout=subprocess.PIPE,
                                        stderr=subprocess.PIPE) for _ in range(2)]
            for child in children:
                stdout, stderr = child.communicate()
                self.assertEqual(child.returncode, 0, stderr)
                self.assertEqual(stdout, ''.join('owned-' + name + '\n' for name in plugins))
            runtime = resolved / 'cargo-runtime'
            for name in (*caches, *locks):
                self.assertTrue((runtime / name).samefile(ambient / name), name)
            self.assertEqual((ambient / '.package-cache').read_text(), 'shared fixture\n')
            for name in ('bin', 'config', 'config.toml', 'credentials', 'credentials.toml'):
                self.assertFalse((runtime / name).exists(), name)

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

    @unittest.skipUnless(platform.system() == 'Darwin', 'macOS provisions npm providers')
    def test_npm_provider_can_start_its_owned_installer_child(self):
        owned = ROOT / '.tools'
        owned.mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='resolver-npm-', dir=owned) as folder:
            root = Path(folder)
            resolved = root / 'resolved'
            binary = resolved / 'bin'
            binary.mkdir(parents=True)
            for name in ('python3', 'fixture-prettier'):
                executable = binary / name
                executable.write_text('#!/bin/sh\nexit 0\n')
                executable.chmod(0o755)
            installer = resolved / 'bootstrap/mise/bin/mise'
            installer.parent.mkdir(parents=True)
            path = json.dumps({'PATH': str(binary) + ':/usr/bin:/bin'})
            installer.write_text('#!/bin/sh\nset -eu\ncase "$1" in\n'
                                 'install) mise fixture-child ;;\n'
                                 'fixture-child) printf installed > "$HOME/installer-child" ;;\n'
                                 'env) printf "%s\\n" ' + shlex.quote(path) + ' ;;\nesac\n')
            installer.chmod(0o755)
            manifest = {'libraries': {}, 'databases': {}, 'python_packages': {},
                        'tools': {'rust': {'commands': [], 'version': 'unused',
                                           'groups': ['common'], 'platforms': ['darwin']},
                                  'prettier': {'commands': ['fixture-prettier'], 'version': 'fixture',
                                               'provider': 'npm:prettier', 'groups': ['test'],
                                               'platforms': ['darwin']}}}
            source = root / 'tools.json'
            source.write_text(json.dumps(manifest))
            result = subprocess.run([sys.executable, str(SCRIPT), '--manifest', str(source),
                                     '--root', str(resolved), 'test', '--',
                                     '/bin/sh', '-c', 'echo command-started'],
                                    env={'HOME': str(root), 'PATH': '/usr/bin:/bin'},
                                    text=True, capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual((root / 'installer-child').read_text(), 'installed')
            self.assertEqual(result.stdout, 'command-started\n')

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
            stage_checkout(ROOT, root)
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
            calls = root / 'docker-calls.jsonl'
            docker.write_text('#!' + sys.executable + '\nimport json,sys\n'
                              + 'with open(' + repr(str(calls)) + ', "a") as log:\n'
                              + '    log.write(json.dumps(sys.argv[1:]) + "\\n")\n'
                              + 'if sys.argv[1] == "image": print("sha256:test")\n')
            docker.chmod(0o755)
            env = dict(os.environ, PATH=str(binary) + os.pathsep + os.environ['PATH'])
            command = [sys.executable, str(root / 'sdlc/scripts/linux-check'), 'test']
            first = subprocess.run(command, env=env, capture_output=True, text=True)
            self.assertEqual(first.returncode, 0, first.stderr)
            source = next(root.glob('.linux-*/src')).resolve()
            run = next(json.loads(line) for line in calls.read_text().splitlines()
                       if json.loads(line)[0] == 'run')
            fixture_mounts = [run[index + 1] for index, value in enumerate(run)
                              if value == '--tmpfs']
            owner = ':uid=' + str(os.getuid()) + ',gid=' + str(os.getgid()) + ',mode=0700,exec'
            self.assertEqual(fixture_mounts, [str(source / relative) + owner
                             for relative in ('target/tmp', 'target/library-only/tmp')])
            self.assertIn(str(source.parent) + ':' + str(source.parent), run)
            (source / 'source.txt').write_text('left by previous check\n')
            (source / 'stray.txt').write_text('untracked source\n')
            (source / 'target').mkdir(exist_ok=True)
            (source / 'target/cached').write_text('build output\n')
            second = subprocess.run(command, env=env, capture_output=True, text=True)
            self.assertEqual(second.returncode, 0, second.stderr)
            self.assertEqual((source / 'source.txt').read_text(), 'committed\n')
            self.assertFalse((source / 'stray.txt').exists())
            self.assertEqual((source / 'target/cached').read_text(), 'build output\n')


if __name__ == '__main__':
    unittest.main()
