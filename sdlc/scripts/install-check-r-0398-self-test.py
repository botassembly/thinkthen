#!/usr/bin/env python3
"""Offline regression for the published R binary repository layout."""
import io
from pathlib import Path
import tarfile
import tempfile
import unittest
from unittest.mock import patch

from install_check import Check, check_result
from install_check_channels import r_universe


class RepositoryPaths(unittest.TestCase):
    def test_binary_index_and_archive_precede_install(self):
        contrib = 'https://botassembly.r-universe.dev/bin/linux/resolute-x86_64/4.6/src/contrib/'
        filename = 'thinkthen_0.1.2.tar.gz'
        for file_field in ('', f'File: {filename}\n'):
            with self.subTest(file_field=file_field), tempfile.TemporaryDirectory() as own:
                check = Check(Path(own), 'r-universe', '0.1.2')
                archive = check.root / filename
                files = {'thinkthen/DESCRIPTION': b'Package: thinkthen\nVersion: 0.1.2\nBuilt: R 4.6.1; x86_64-pc-linux-gnu; 2026-10-04; unix\n',
                         'thinkthen/libs/thinkthen.so': b'fixture binary'}
                with tarfile.open(archive, 'w:gz') as packed:
                    for name, data in files.items():
                        member = tarfile.TarInfo(name)
                        member.size = len(data)
                        packed.addfile(member, io.BytesIO(data))
                reads, commands = [], []

                def text(url, name):
                    reads.append(url)
                    self.assertEqual(reads, [contrib + 'PACKAGES'])
                    self.assertEqual(name, 'r-binary-index')
                    return 'Package: thinkthen\nVersion: 0.1.2\n' + file_field

                def fetch(url, destination):
                    reads.append(url)
                    self.assertEqual(reads, [contrib + 'PACKAGES', contrib + filename])
                    self.assertEqual(destination, archive)
                    return archive

                def run(*args):
                    self.assertEqual(reads, [contrib + 'PACKAGES', contrib + filename])
                    commands.append(tuple(str(arg) for arg in args))
                    return '0.1.2' if args[0] == 'Rscript' else ''

                with patch.object(check, 'text', text), patch.object(check, 'fetch', fetch), \
                     patch.object(check, 'run', run), patch.object(check, 'response', return_value={'value': True, 'requests_sent': 0}):
                    installed, reply, _ = r_universe(check)
                check_result(check.version, installed, reply)
                self.assertEqual(commands[0], ('R', 'CMD', 'INSTALL', '--no-test-load',
                                               '--library=' + check.env['R_LIBS_USER'], str(archive)))
                self.assertEqual(commands[1][0], 'Rscript')
                self.assertEqual(len(commands), 2)


if __name__ == '__main__':
    unittest.main()
