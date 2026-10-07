"""Installed consumers refuse missing products while checkout implementations exist."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[3]


class InstalledProducts(unittest.TestCase):
    def test_absent_extracted_package_never_uses_checkout(self):
        callers = (
            ('ada/checks/native_parity.py', []),
            ('cobol/checks/native_parity.py', []),
            ('swift/Tests/fixtures/complete_parity.py', []),
            ('zig/Tests/complete_parity.py', []),
            ('objective-c/checks/complete_parity.py', []),
            ('php/fixtures/complete_parity.py', ['php']),
        )
        with tempfile.TemporaryDirectory(prefix='thinkthen-missing-products-') as folder:
            missing = str(Path(folder) / 'absent-package')
            env = {'PATH': os.environ.get('PATH', '/usr/bin:/bin'), 'HOME': folder,
                   'THINKTHEN_ARTIFACT': 'installed-mode', 'THINKTHEN_PARITY_PACKAGE': missing}
            for caller, arguments in callers:
                with self.subTest(caller=caller):
                    result = subprocess.run([sys.executable, str(ROOT / 'libraries' / caller), *arguments],
                                            cwd=ROOT, env=env, capture_output=True, text=True, timeout=30)
                    self.assertNotEqual(result.returncode, 0)
                    self.assertIn(missing, result.stderr)
                    self.assertNotIn('parity: ', result.stdout)

    def test_go_requires_its_installed_caller_before_execution(self):
        env = {'PATH': os.environ.get('PATH', '/usr/bin:/bin'), 'THINKTHEN_ARTIFACT': 'installed-mode'}
        result = subprocess.run([sys.executable, str(ROOT / 'libraries/go/fixtures/type_cases.py'), 'native'],
                                cwd=ROOT, env=env, capture_output=True, text=True, timeout=30)
        self.assertNotEqual(result.returncode, 0)
        self.assertIn('installed Go parity requires', result.stderr)
        self.assertNotIn('parity: ', result.stdout)


if __name__ == '__main__':
    unittest.main()
