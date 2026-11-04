#!/usr/bin/env python3
"""Exercise the pre-release Pages artifact through its command interface."""
from contextlib import redirect_stdout
import hashlib
import io
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile
import unittest
import urllib.error
from unittest.mock import patch

SCRIPT = Path(__file__).with_name('pages-installers.py')
OLD = b'#!/bin/sh\necho install-the-published-version\n'
DIGEST = hashlib.sha256(OLD).hexdigest()


class PagesInstallers(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.snapshot = self.root / 'snapshot'
        self.snapshot.mkdir()
        (self.snapshot / 'install.sh').write_bytes(OLD)
        (self.snapshot / 'powershell-status').write_text('404\n')
        self.dist = self.root / 'dist'
        self.dist.mkdir()
        (self.dist / 'index.html').write_bytes(b'<h1>New documentation</h1>')
        (self.dist / 'install.sh').write_bytes(b'unreleased shell installer')
        (self.dist / 'install.ps1').write_bytes(b'unreleased PowerShell installer')

    def restore(self):
        return subprocess.run([sys.executable, SCRIPT, 'restore', self.snapshot,
                               self.dist, '--expected-sha', DIGEST], capture_output=True)

    def test_keeps_new_docs_and_restores_only_held_installers(self):
        result = self.restore()
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual((self.dist / 'install.sh').read_bytes(), OLD)
        self.assertFalse((self.dist / 'install.ps1').exists())
        self.assertEqual((self.dist / 'index.html').read_bytes(), b'<h1>New documentation</h1>')
        self.assertEqual((self.dist / 'CNAME').read_text(), 'thinkthen.dev\n')

    def test_rejects_changed_installer_before_mutating_output(self):
        (self.snapshot / 'install.sh').write_bytes(b'changed live installer')
        self.assertNotEqual(self.restore().returncode, 0)
        self.assertEqual((self.dist / 'install.sh').read_bytes(), b'unreleased shell installer')

    def test_rejects_missing_status_before_mutating_output(self):
        (self.snapshot / 'powershell-status').unlink()
        self.assertNotEqual(self.restore().returncode, 0)
        self.assertTrue((self.dist / 'install.ps1').exists())

    def test_rejects_symlink_without_touching_its_target(self):
        outside = self.root / 'outside'
        outside.write_bytes(b'owned bytes')
        (self.dist / 'CNAME').symlink_to(outside)
        self.assertNotEqual(self.restore().returncode, 0)
        self.assertEqual(outside.read_bytes(), b'owned bytes')

    def capture(self, responses):
        destination = self.root / 'captured'
        arguments = [str(SCRIPT), 'snapshot', str(destination), '--expected-sha', DIGEST]
        with patch.object(sys, 'argv', arguments), patch('urllib.request.urlopen', side_effect=responses):
            with redirect_stdout(io.StringIO()):
                runpy.run_path(str(SCRIPT), run_name='__main__')
        return destination

    @staticmethod
    def response(body):
        result = io.BytesIO(body)
        result.status = 200
        return result

    def test_snapshot_requires_expected_shell_and_missing_powershell(self):
        missing = urllib.error.HTTPError('https://thinkthen.dev/install.ps1', 404, '', {}, None)
        captured = self.capture([self.response(OLD), missing])
        self.assertEqual((captured / 'install.sh').read_bytes(), OLD)
        self.assertEqual((captured / 'powershell-status').read_text(), '404\n')

    def test_snapshot_refuses_a_live_powershell_installer(self):
        with self.assertRaisesRegex(ValueError, 'must remain HTTP 404'):
            self.capture([self.response(OLD), self.response(b'published too early')])

    def test_snapshot_refuses_changed_live_shell(self):
        with self.assertRaisesRegex(ValueError, 'differs from the approved'):
            self.capture([self.response(b'changed shell')])

    def test_snapshot_refuses_unexpected_http_error(self):
        failure = urllib.error.HTTPError('https://thinkthen.dev/install.ps1', 503, '', {}, None)
        with self.assertRaises(urllib.error.HTTPError):
            self.capture([self.response(OLD), failure])


if __name__ == '__main__':
    unittest.main()
