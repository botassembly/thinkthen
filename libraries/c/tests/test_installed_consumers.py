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
            ('ada/checks/native_session.py', []),
            ('cobol/checks/native_parity.py', []),
            ('zig/Tests/complete_parity.py', []),
            ('objective-c/checks/foundation_installed.py', []),
            ('php/fixtures/complete_parity.py', ['php']),
        )
        with tempfile.TemporaryDirectory(prefix='thinkthen-missing-products-') as folder:
            missing = str(Path(folder) / 'absent-package')
            env = {'PATH': os.environ.get('PATH', '/usr/bin:/bin'), 'HOME': folder,
                   'THINKTHEN_ARTIFACT': missing, 'THINKTHEN_PARITY_PACKAGE': missing}
            for caller, arguments in callers:
                with self.subTest(caller=caller):
                    result = subprocess.run([sys.executable, str(ROOT / 'libraries' / caller), *arguments],
                                            cwd=ROOT, env=env, capture_output=True, text=True, timeout=30)
                    self.assertNotEqual(result.returncode, 0)
                    if caller == 'objective-c/checks/foundation_installed.py' and sys.platform != 'darwin':
                        self.assertEqual(result.returncode, 77)
                        self.assertIn('Foundation installed consumer requires macOS', result.stderr)
                    else:
                        self.assertIn(missing, result.stderr)
                    self.assertNotIn('parity: ', result.stdout)



if __name__ == '__main__':
    unittest.main()
