#!/usr/bin/env python3
"""Exercise ticket and checkpoint dependency policy through existing entry points."""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
from conformance.children.children import child_env, fixture_run
from sdlc.scripts.tools import fixture_path


class DependencyPolicy(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='lint-policy-')
        self.addCleanup(self.temporary.cleanup)
        self.folder = Path(self.temporary.name)
        self.repo = self.folder / 'repo'
        self.repo.mkdir()
        self.bin = self.folder / 'bin'
        self.bin.mkdir()
        self.env = child_env(home=self.folder / 'home', PATH=fixture_path(self.bin),
                             GIT_CONFIG_NOSYSTEM='1', GIT_AUTHOR_NAME='Fixture',
                             GIT_AUTHOR_EMAIL='fixture@example.invalid',
                             GIT_COMMITTER_NAME='Fixture', GIT_COMMITTER_EMAIL='fixture@example.invalid')
        for name in ('dependency-check.py', 'linux-check', 'tools.py', 'allow-list', 'test-full-cases'):
            self.copy('sdlc/scripts/' + name)
        for name in ('sdlc/linux/install.py', 'sdlc/linux/Dockerfile', 'sdlc/tools.json',
                     'rust-toolchain.toml', 'conformance/children/children.py'):
            self.copy(name)
        for path in (ROOT / 'sdlc/tools').glob('*.json'):
            self.copy(str(path.relative_to(ROOT)))
        (self.repo / 'Cargo.lock').write_text('fixture lock\n')
        (self.repo / 'Cargo.toml').write_text('fixture manifest\n')
        (self.repo / '.gitignore').write_text('.linux-*\n__pycache__/\n')
        self.run_command(['git', 'init', '-q', '-b', 'main'])
        self.run_command(['git', 'add', '.'])
        self.run_command(['git', 'commit', '-qm', 'Create fixture'])
        self.base = self.run_command(['git', 'rev-parse', 'HEAD']).stdout.strip()
        self.run_command(['git', 'update-ref', 'refs/remotes/origin/main', self.base])
        self.run_command(['git', 'checkout', '-q', '--detach'])
        self.fake('cargo', """import pathlib, sys
root = pathlib.Path.cwd()
fault = (root / 'fault').read_text().strip()
sys.exit(31 if fault in sys.argv[1:] else 0)
""")

    def copy(self, relative):
        destination = self.repo / relative
        destination.parent.mkdir(parents=True, exist_ok=True)
        shutil.copyfile(ROOT / relative, destination)

    def fake(self, name, body):
        target = self.bin / name
        target.write_text('#!' + sys.executable + '\n' + body)
        target.chmod(0o755)

    def run_command(self, command, *, env=None, expected=0):
        result = fixture_run(command, cwd=self.repo, env=self.env if env is None else env,
                             text=True, capture_output=True)
        self.assertEqual(result.returncode, expected, result.stdout + result.stderr)
        return result

    def deny(self, mode='ticket', *, expected=0, binding=False):
        env = dict(self.env)
        if mode is not None:
            env['THINKTHEN_ADVISORY_MODE'] = mode
        command = [sys.executable, 'sdlc/scripts/dependency-check.py']
        if binding:
            command += ['--manifest-path', 'libraries/c/Cargo.toml', '--config', 'deny.toml', '--sources']
        return self.run_command(command, env=env, expected=expected)

    def test_unchanged_dependencies_skip_advisory_but_keep_bans_licenses_and_sources(self):
        for binding in (False, True):
            with self.subTest(binding=binding):
                (self.repo / 'fault').write_text('advisories')
                result = self.deny(binding=binding)
                if not binding:
                    self.assertEqual(len(result.stdout.splitlines()), 1)
                    self.assertIn('merge base ' + self.base, result.stdout)
                for fault in ('bans', 'licenses', *(['sources'] if binding else [])):
                    (self.repo / 'fault').write_text(fault)
                    self.deny(binding=binding, expected=31)

    def test_every_tracked_cargo_lock_or_manifest_change_checks_advisories(self):
        for relative in ('Cargo.lock', 'Cargo.toml', 'libraries/c/Cargo.lock', 'libraries/c/Cargo.toml'):
            with self.subTest(path=relative):
                path = self.repo / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text('changed\n')
                self.run_command(['git', 'add', relative])
                (self.repo / 'fault').write_text('advisories')
                self.deny(expected=31)
                for fault in ('bans', 'licenses'):
                    (self.repo / 'fault').write_text(fault)
                    self.deny(expected=31)
                self.run_command(['git', 'restore', '--source', self.base, '--staged', '--worktree', '--', relative])

    def test_checkpoint_release_default_unknown_and_missing_main_fail_closed(self):
        (self.repo / 'fault').write_text('advisories')
        for mode in ('checkpoint', 'release', None):
            with self.subTest(mode=mode):
                self.deny(mode, expected=31)
        self.deny('unknown', expected=2)
        self.run_command(['git', 'update-ref', '-d', 'refs/remotes/origin/main'])
        self.deny(expected=2)

    def test_linux_entry_carries_strict_context_and_the_fetched_main_baseline(self):
        calls = self.folder / 'docker-calls'
        self.fake('docker', """import json, pathlib, sys
with pathlib.Path(%r).open('a') as log:
    log.write(json.dumps(sys.argv[1:]) + '\\n')
if sys.argv[1:3] == ['image', 'inspect']:
    print('fixture-image')
""" % str(calls))
        cases = [('0746', None, ['lint', 'test'], 'ticket'),
                 ('0691', 'ticket', ['lint', 'test'], 'checkpoint'),
                 ('0746', 'checkpoint', ['lint'], 'checkpoint'),
                 ('0746', 'release', ['lint'], 'release'),
                 (None, None, ['lint'], 'checkpoint'),
                 ('0746', None, ['package'], 'checkpoint'),
                 ('0746', None, ['test-full-cases'], 'checkpoint')]
        for ticket, requested, checks, expected in cases:
            with self.subTest(ticket=ticket, requested=requested, checks=checks):
                calls.write_text('')
                env = dict(self.env)
                if ticket:
                    env['PM_JOB'] = ticket + ' fixture'
                if requested:
                    env['THINKTHEN_ADVISORY_MODE'] = requested
                self.run_command([sys.executable, 'sdlc/scripts/linux-check', '--docker',
                                  str(self.bin / 'docker'), *checks], env=env)
                commands = [json.loads(line) for line in calls.read_text().splitlines()]
                self.assertIn('THINKTHEN_ADVISORY_MODE=' + expected, commands[-1])
                source = next(self.repo.glob('.linux-*/src'))
                result = fixture_run(['git', 'rev-parse', 'refs/remotes/origin/main'],
                                     cwd=source, env=self.env, capture_output=True, text=True)
                self.assertEqual(result.returncode, 0, result.stderr)
                self.assertEqual(result.stdout.strip(), self.base)
        calls.write_text('')
        self.run_command([sys.executable, 'sdlc/scripts/linux-check', '--docker',
                          str(self.bin / 'docker'), 'lint'],
                         env=dict(self.env, THINKTHEN_ADVISORY_MODE='unknown'), expected=2)
        self.assertEqual(calls.read_text(), '')

    def test_native_checkpoint_routes_advisories_to_linux_and_preserves_failure(self):
        # Stop at the real campaign's Linux prerequisite, before package builds.
        (self.repo / 'sdlc/scripts/linux-check').write_text(
            "import os, sys\nprint('Linux prerequisite ' + os.environ['THINKTHEN_ADVISORY_MODE'], flush=True)\nsys.exit(31)\n")
        self.run_command(['git', 'add', 'sdlc/scripts/linux-check'])
        self.run_command(['git', 'commit', '-qm', 'Plant advisory prerequisite'])
        sha = self.run_command(['git', 'rev-parse', 'HEAD']).stdout.strip()
        self.run_command(['git', 'update-ref', 'refs/remotes/origin/main', sha])
        log = self.folder / 'checkpoint.log'
        self.run_command(['sh', 'sdlc/scripts/test-full-cases', '--checkpoint', sha, str(log)],
                         env=dict(self.env, THINKTHEN_ADVISORY_MODE='ticket'), expected=31)
        self.assertIn('Linux prerequisite checkpoint', log.read_text())
        self.assertNotIn('checkpoint: start test-full-cases', log.read_text())

    def test_sanitization_preserves_explicit_strict_mode(self):
        self.run_command(['sh', '-c', '. sdlc/scripts/allow-list; test "$THINKTHEN_ADVISORY_MODE" = checkpoint'],
                         env=dict(self.env, THINKTHEN_ADVISORY_MODE='checkpoint'))


if __name__ == '__main__':
    unittest.main()
