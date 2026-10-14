"""Select declared stress cases before running registered surface checks."""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

from conformance import parity
from conformance.children.children import child_env


class StressSelection(unittest.TestCase):
    def test_actual_registry_excludes_unsupported_php_and_requires_selected_checks(self):
        selected = {'libraries/polars', 'libraries/typescript', 'libraries/r',
                    'databases/sqlite', 'databases/duckdb', 'databases/postgresql'}
        with tempfile.TemporaryDirectory(prefix='thinkthen-stress-selection-') as folder:
            root = Path(folder)
            subprocess.run(['git', 'init', '-q', str(root)],
                           env=child_env(home=root / 'home', PATH=os.defpath), check=True)
            scripts = root / 'sdlc/scripts'
            scripts.mkdir(parents=True)
            for name in ('surfaces', 'verdict.sh', 'full-compiler', 'feature-inventory.py'):
                shutil.copyfile(parity.ROOT / 'sdlc/scripts' / name, scripts / name)
            shutil.copyfile(parity.ROOT / 'sdlc/surfaces.txt', root / 'sdlc/surfaces.txt')
            (scripts / 'heavy-lock').write_text('')
            (scripts / 'scratch.sh').write_text('usage_guard() { :; }\n')
            (scripts / 'installed.sh').write_text('backend_start() { port=12345; }\n')
            for name in ('release-pack', 'release-smoke'):
                (scripts / name).write_text('echo unexpected-package >> calls\nexit 9\n')
            conformance = root / 'conformance'
            conformance.mkdir()
            (conformance / 'parity.py').write_text('')
            manifest_names = {'Cargo.toml', 'composer.json', 'ThinkThen.csproj', 'pom.xml',
                              'pubspec.yaml', 'Package.swift', 'build.zig.zon', 'go.mod',
                              'CMakeLists.txt', 'thinkthen.gpr', 'source-package.json'}
            rows = [line.split() for line in (root / 'sdlc/surfaces.txt').read_text().splitlines()
                    if line.strip() and not line.startswith('#')]
            for row in rows:
                surface, state = row[:2]
                library = root / surface
                library.mkdir(parents=True)
                (library / 'check.sh').write_text(
                    'test "$THINKTHEN_TEST_PROFILE" = stress || exit 8\n'
                    f'echo {surface} >> "$FIXTURE_ROOT/calls"\n'
                    f'[ "$FAILED_SURFACE" != {surface} ] || exit 77\n')
                if state == 'feature':
                    shutil.copyfile(parity.ROOT / surface / 'README.md', library / 'README.md')
                    continue
                original = parity.ROOT / surface
                for manifest in original.rglob('*'):
                    if manifest.name in manifest_names and 'target' not in manifest.parts:
                        target = library / manifest.relative_to(original)
                        target.parent.mkdir(parents=True, exist_ok=True)
                        target.touch()
            php = root / 'libraries/php/check.sh'
            shutil.copyfile(parity.ROOT / 'libraries/php/check.sh', php)
            env = {'PATH': os.defpath, 'HOME': str(root / 'home'),
                   'FIXTURE_ROOT': str(root), 'FAILED_SURFACE': '',
                   'THINKTHEN_TEST_PROFILE': 'stress'}
            unsupported = subprocess.run(['sh', str(php)], env=env,
                                         capture_output=True, text=True)
            self.assertEqual(unsupported.returncode, 77, unsupported.stdout + unsupported.stderr)
            self.assertIn('no stress gate', unsupported.stdout)
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
