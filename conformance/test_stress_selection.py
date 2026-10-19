"""Select declared stress cases before running registered surface checks."""
import os
import json
import platform
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

from conformance import parity
from conformance.children.children import child_env
from conformance.source_fixtures import stage_checkout, fixture_tool_groups, stub_other_surfaces


class StressSelection(unittest.TestCase):
    def test_postgresql_failure_survives_a_later_unavailable_major(self):
        with tempfile.TemporaryDirectory(prefix='thinkthen-pg-selection-') as folder:
            root = Path(folder)
            stage_checkout(parity.ROOT, root)
            scripts = root / 'sdlc/scripts'
            scripts.mkdir(parents=True, exist_ok=True)
            (scripts / 'allow-list').write_text('')
            (scripts / 'scratch.sh').write_text('usage_guard() { :; }\n')
            (scripts / 'installed.sh').write_text('backend_start() { port=12345; }\n')
            stub_other_surfaces(root, {'databases/postgresql'})
            (root / 'bin').mkdir(exist_ok=True)
            (root / 'bin/python3').symlink_to(sys.executable)
            conformance = root / 'conformance'
            conformance.mkdir(exist_ok=True)
            (conformance / 'parity.py').write_text('')
            surface = root / 'databases/postgresql'
            surface.mkdir(parents=True, exist_ok=True)
            (surface / 'Cargo.toml').touch()
            (surface / 'check.sh').write_text(
                'echo "$1" >> "$FIXTURE_ROOT/calls"\n'
                'case $1 in 16) exit "$CODE16" ;; 18) exit "$CODE18" ;; esac\n')
            env = child_env(home=root / 'home', PATH=str(root / 'bin') + os.pathsep + os.environ['PATH'], FIXTURE_ROOT=str(root))
            env['THINKTHEN_RESOLVED_TOOLS'] = fixture_tool_groups(parity.ROOT)
            for first, last in ((1, 77), (77, 1), (77, 77), (0, 77)):
                with self.subTest(first=first, last=last):
                    calls = root / 'calls'
                    calls.write_text('')
                    result = subprocess.run(['sh', str(scripts / 'surfaces')], cwd=root,
                                            env=env | {'CODE16': str(first), 'CODE18': str(last)},
                                            capture_output=True, text=True)
                    self.assertEqual(calls.read_text().splitlines(), ['16', '18'],
                                     result.stdout + result.stderr)
                    self.assertEqual(result.returncode, int(1 in (first, last)),
                                     result.stdout + result.stderr)

    def test_actual_registry_excludes_unsupported_php_and_requires_selected_checks(self):
        selected = {'libraries/c', 'libraries/rust', 'libraries/polars',
                    'libraries/typescript', 'libraries/r',
                    'databases/sqlite', 'databases/duckdb', 'databases/postgresql'}
        with tempfile.TemporaryDirectory(prefix='thinkthen-stress-selection-') as folder:
            root = Path(folder)
            stage_checkout(parity.ROOT, root)
            subprocess.run(['git', 'init', '-q', str(root)],
                           env=child_env(home=root / 'home', PATH=os.defpath), check=True)
            scripts = root / 'sdlc/scripts'
            scripts.mkdir(parents=True, exist_ok=True)
            (scripts / 'allow-list').write_text('')
            (scripts / 'scratch.sh').write_text('usage_guard() { :; }\n')
            (scripts / 'installed.sh').write_text('backend_start() { port=12345; }\n')
            for name in ('release-pack', 'release-smoke'):
                (scripts / name).write_text('echo unexpected-package >> calls\nexit 9\n')
            conformance = root / 'conformance'
            conformance.mkdir(exist_ok=True)
            (conformance / 'parity.py').write_text('')
            rows = [line.split() for line in (root / 'sdlc/surfaces.txt').read_text().splitlines()
                    if line.strip() and not line.startswith('#')]
            for row in rows:
                surface, state = row[:2]
                library = root / surface
                library.mkdir(parents=True, exist_ok=True)
                if surface == 'libraries/php':
                    continue
                (library / 'check.sh').write_text(
                    'test "$THINKTHEN_TEST_PROFILE" = stress || exit 8\n'
                    f'echo {surface} >> "$FIXTURE_ROOT/calls"\n'
                    f'[ "$FAILED_SURFACE" != {surface} ] || exit 77\n')
            php = root / 'libraries/php/check.sh'
            tools = root / 'bin'
            tools.mkdir(exist_ok=True)
            # Shell helpers must use the supported Python running this fixture.
            (tools / 'python3').symlink_to(sys.executable)
            env = {'PATH': str(tools) + ':' + os.defpath, 'HOME': str(root / 'home'),
                   'FIXTURE_ROOT': str(root), 'FAILED_SURFACE': '',
                   'THINKTHEN_TEST_PROFILE': 'stress'}
            env['THINKTHEN_RESOLVED_TOOLS'] = fixture_tool_groups(parity.ROOT)
            subprocess.run(['git', 'add', 'libraries'], cwd=root, env=env, check=True)
            unsupported = subprocess.run(['sh', str(php)], env=env,
                                         capture_output=True, text=True)
            arch = {'aarch64': 'arm64'}.get(platform.machine(), platform.machine())
            host = platform.system().lower() + '-' + arch
            manifest = json.loads((root / 'sdlc/tools.json').read_text())
            supported = host in manifest['libraries']['php']['check_platforms']
            self.assertEqual(unsupported.returncode, 1 if supported else 77, unsupported.stdout + unsupported.stderr)
            self.assertIn('no stress gate' if supported else 'not run', unsupported.stdout)
            for failed in ('', 'libraries/typescript'):
                with self.subTest(failed=failed):
                    calls = root / 'calls'
                    calls.write_text('')
                    result = subprocess.run(['sh', str(scripts / 'surfaces'), '--stress'],
                                            cwd=root, env=env | {'FAILED_SURFACE': failed},
                                            capture_output=True, text=True)
                    self.assertEqual(set(calls.read_text().splitlines()), selected,
                                     result.stdout + result.stderr)
                    self.assertEqual(result.returncode, int(bool(failed)),
                                     result.stdout + result.stderr)
                    self.assertIn('not selected libraries/php (no stress cases)', result.stdout)
                    self.assertNotIn('pass libraries/php', result.stdout)
                    if failed:
                        self.assertIn('FAIL libraries/typescript (not run)', result.stdout)


if __name__ == '__main__':
    unittest.main()
