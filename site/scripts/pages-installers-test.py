#!/usr/bin/env python3
"""Exercise the held and release Pages artifacts through the command interface."""
from contextlib import redirect_stdout
import hashlib
import io
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile
import tarfile
import unittest
import urllib.error
from unittest.mock import patch

SCRIPT = Path(__file__).with_name('pages-installers.py')
ROOT = SCRIPT.parents[2]
OLD = b'#!/bin/sh\necho install-the-published-version\n'
DIGEST = hashlib.sha256(OLD).hexdigest()
INSTALLERS = ('install.sh', 'install.ps1')


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
                               self.dist, '--installers', 'held', '--expected-sha', DIGEST],
                              capture_output=True)

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

    def artifact(self, entries):
        artifact = self.root / 'artifact.tar'
        with tarfile.open(artifact, 'w') as archive:
            for name, body in entries:
                member = tarfile.TarInfo(name)
                member.size = len(body)
                archive.addfile(member, io.BytesIO(body))
        return artifact

    def verify(self, entries):
        return subprocess.run([sys.executable, SCRIPT, 'verify', self.artifact(entries),
                               '--installers', 'held', '--expected-sha', DIGEST], capture_output=True)

    def test_deploy_accepts_the_downloaded_pages_tar(self):
        result = self.verify([('./index.html', b'new docs'), ('./install.sh', OLD),
                              ('./CNAME', b'thinkthen.dev\n')])
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_deploy_rejects_changed_or_missing_held_files_in_tar(self):
        approved = [('install.sh', OLD), ('CNAME', b'thinkthen.dev\n')]
        cases = [
            [('install.sh', b'changed'), approved[1]],
            [approved[1]],
            [*approved, ('install.ps1', b'unreleased')],
            [*approved, ('./install.sh', b'duplicate')],
            [approved[0], ('CNAME', b'another.example\n')],
            [*approved, ('../install.ps1', b'unsafe path')],
        ]
        for entries in cases:
            with self.subTest(entries=[name for name, _ in entries]):
                self.assertNotEqual(self.verify(entries).returncode, 0)

    def command(self, arguments, responses):
        with patch.object(sys, 'argv', [str(SCRIPT), *map(str, arguments)]), \
                patch('urllib.request.urlopen', side_effect=responses) as opened:
            with redirect_stdout(io.StringIO()):
                runpy.run_path(str(SCRIPT), run_name='__main__')
        return opened

    def capture(self, responses):
        destination = self.root / 'captured'
        self.command(['snapshot', destination, '--installers', 'held', '--expected-sha', DIGEST], responses)
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

    def release(self, mode, *paths, sums):
        return self.command([mode, *paths, '--installers', 'release', '--expected-sha', DIGEST],
                            [self.response(sums)])

    @staticmethod
    def published(**changed):
        """Build a published SHA256SUMS whose installer lines match the repo's files."""
        lines = [f'{"a" * 64}  thinkthen-packages.tar.gz']
        for name in INSTALLERS:
            digest = hashlib.sha256((ROOT / name).read_bytes()).hexdigest()
            lines.append(f'{changed.get(name.replace(".", "_"), digest)}  {name}')
        return ('\n'.join(lines) + '\n').encode()

    def test_release_serves_both_repo_installers_when_they_match_published_sums(self):
        captured = self.root / 'captured'
        sums = self.published()
        opened = self.release('snapshot', captured, sums=sums)
        self.assertRegex(opened.call_args.args[0],
                         r'^https://github\.com/botassembly/thinkthen/releases/download/v[0-9.]+/SHA256SUMS$')
        self.release('restore', captured, self.dist, sums=sums)
        repo = {name: (ROOT / name).read_bytes() for name in INSTALLERS}
        for name, body in repo.items():
            self.assertEqual((self.dist / name).read_bytes(), body)
        self.assertEqual((self.dist / 'CNAME').read_text(), 'thinkthen.dev\n')
        tar = self.artifact([*repo.items(), ('CNAME', b'thinkthen.dev\n')])
        self.release('verify', tar, sums=sums)

    def test_release_refuses_repo_installers_that_differ_from_published_sums(self):
        captured = self.root / 'captured'
        for name in ('install_sh', 'install_ps1'):
            with self.subTest(name=name), self.assertRaisesRegex(ValueError, 'differs from the approved'):
                self.release('snapshot', captured, sums=self.published(**{name: '0' * 64}))
        self.assertFalse(captured.exists())

    def test_release_refuses_sums_that_do_not_list_each_installer_once(self):
        sums = self.published()
        cases = [b''.join(line for line in sums.splitlines(True) if b'install.ps1' not in line),
                 sums + sums.splitlines(True)[-1]]
        for case in cases:
            with self.subTest(case=case), self.assertRaisesRegex(ValueError, 'each installer once'):
                self.release('snapshot', self.root / 'captured', sums=case)

    def test_release_refuses_before_the_release_is_published(self):
        missing = urllib.error.HTTPError('https://github.com/', 404, '', {}, None)
        arguments = ['snapshot', self.root / 'captured', '--installers', 'release', '--expected-sha', DIGEST]
        with self.assertRaises(urllib.error.HTTPError):
            self.command(arguments, [missing])
        self.assertFalse((self.root / 'captured').exists())

    def test_release_deploy_refuses_a_tar_without_the_published_bytes(self):
        repo = {name: (ROOT / name).read_bytes() for name in INSTALLERS}
        domain = ('CNAME', b'thinkthen.dev\n')
        cases = [[('install.sh', repo['install.sh']), domain],
                 [('install.sh', repo['install.sh']), ('install.ps1', b'changed'), domain]]
        for entries in cases:
            with self.subTest(entries=[name for name, _ in entries]):
                with self.assertRaisesRegex(ValueError, 'install.ps1 differs from the approved'):
                    self.release('verify', self.artifact(entries), sums=self.published())

    def test_workflow_holds_the_installers_until_release_is_chosen(self):
        workflow = (ROOT / '.github/workflows/pages.yml').read_text()
        self.assertIn('default: held', workflow)
        self.assertEqual(workflow.count('--installers "$INSTALLERS"'), 3)


if __name__ == '__main__':
    unittest.main()
