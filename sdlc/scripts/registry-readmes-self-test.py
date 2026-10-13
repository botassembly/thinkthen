#!/usr/bin/env python3
"""Exercise the package-text release boundary with positive and negative archives."""
import importlib.util
import io
import json
from pathlib import Path
import tarfile
import tempfile
import shutil
import subprocess
import struct
import yaml
import zipfile
import unittest

spec = importlib.util.spec_from_file_location('registry_readmes', Path(__file__).with_name('registry-readmes.py'))
readmes = importlib.util.module_from_spec(spec)
spec.loader.exec_module(readmes)


def archive(path, files):
    with tarfile.open(path, 'w:gz') as package:
        for name, data in files.items():
            entry = tarfile.TarInfo(name)
            entry.size = len(data)
            package.addfile(entry, io.BytesIO(data))


class PackageText(unittest.TestCase):
    def test_public_package_uses_reviewed_text(self):
        with tempfile.TemporaryDirectory() as scratch:
            package = Path(scratch) / 'thinkthen.tgz'
            definition = json.loads((readmes.REPO / 'libraries/typescript/package.json').read_text())
            files = {'package/README.md': readmes.source('typescript').read_bytes(),
                     'package/package.json': json.dumps(definition).encode()}
            archive(package, files)
            readmes.check_package('typescript', package)
            plants = [
                (b'ADR 0047', 'adr'),
                (b'ticket 0578', 'ticket'),
                (b'sdlc/planning/python.md', 'internal-path'),
                (b'thinkthen-python', 'internal-crate'),
                (b'thinkthen-core', 'internal-project'),
                (b'pip install thinkthen==0.1.2', 'stale-version'),
                (b'ThinkThen release 0.1.2', 'stale-version'),
                (b'gem install thinkthen -v 0.1.2', 'stale-version'),
                (b'npm install thinkthen@0.1.2', 'stale-version'),
                (b'Unreviewed wording', 'readme-source-mismatch'),
            ]
            for text, reason in plants:
                with self.subTest(reason=reason, text=text):
                    archive(package, dict(files, **{'package/README.md': text}))
                    with self.assertRaisesRegex(readmes.Refusal, reason):
                        readmes.check_package('typescript', package)
            for text, reason in [('ADR 0047', 'adr'), ('New public description', 'description-source-mismatch')]:
                with self.subTest(description=text):
                    changed = dict(definition, description=text)
                    archive(package, dict(files, **{'package/package.json': json.dumps(changed).encode()}))
                    with self.assertRaisesRegex(readmes.Refusal, reason):
                        readmes.check_package('typescript', package)

    def test_npm_assembles_public_readme_and_refuses_adr(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch)
            source = tree / 'npm'
            source.mkdir()
            for name in ('package.json', 'README.md', 'LICENSE'):
                shutil.copyfile(readmes.REPO / 'libraries/typescript' / name, source / name)
            def pack():
                subprocess.run(['npm', 'pack', '--offline', '--ignore-scripts', '--silent',
                                '--cache', str(tree / 'cache'), '--pack-destination', str(tree)],
                               cwd=source, check=True, capture_output=True)
                return tree / ('thinkthen-' + readmes.version() + '.tgz')
            readmes.check_package('typescript', pack())
            (source / 'README.md').write_text('ADR 0047\n')
            with self.assertRaisesRegex(readmes.Refusal, 'adr'):
                readmes.check_package('typescript', pack())

    def test_windows_command_assembles_root_public_page(self):
        spec = importlib.util.spec_from_file_location('command', readmes.REPO / 'sdlc/scripts/release-windows-command.py')
        command = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(command)
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch)
            binary = tree / 'thinkthen.exe'
            data = bytearray(128)
            data[:2] = b'MZ'
            struct.pack_into('<I', data, 60, 64)
            data[64:68] = b'PE\0\0'
            for offset, value in ((68, 0x8664), (86, 2), (88, 0x20B)):
                struct.pack_into('<H', data, offset, value)
            binary.write_bytes(data)
            package = tree / ('thinkthen-' + readmes.version() + '-x86_64-pc-windows-msvc.zip')
            command.pack(binary, package)
            readmes.check_package('command', package)
            with zipfile.ZipFile(package, 'w') as bundle:
                bundle.writestr('thinkthen.exe', data)
                bundle.writestr('README.md', b'ADR 0047')
            with self.assertRaisesRegex(readmes.Refusal, 'adr'):
                readmes.check_package('command', package)

    def test_dependency_versions_are_allowed(self):
        readmes.check_text(b'Rust 1.95.0; DuckDB 1.5.6; Zig 0.15.2; JDK 22; jsonlite 2.0.0', '0.2.0')
        readmes.check_text(b'tar -xzf thinkthen-c-0.2.0-TARGET.tar.gz', '0.2.0')

    def test_missing_readme_refuses(self):
        with tempfile.TemporaryDirectory() as scratch:
            package = Path(scratch) / 'thinkthen.tgz'
            archive(package, {'package/package.json': b'{}'})
            with self.assertRaisesRegex(readmes.Refusal, 'missing-or-repeated-member'):
                readmes.check_package('typescript', package)

    def test_example_checks_actual_page_and_refuses_single_function(self):
        filename, code, output = readmes.example('python')
        self.assertEqual(filename, 'sample.py')
        self.assertIn('engine.decide(', code)
        self.assertIn('engine.choose(', code)
        self.assertEqual(output, 'True\nFalse\nTrue\nbilling\nshipping\naccount\n')
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            page = root / 'libraries/python/README-PUBLIC.md'
            page.parent.mkdir(parents=True)
            page.write_text('```python\nprint(engine.decide("Refund?", "Refund please.").value)\n```\n')
            with self.assertRaisesRegex(readmes.Refusal, 'example-needs-two-functions: python'):
                readmes.example('python', root)

    def test_release_replays_every_public_package_on_a_supported_host(self):
        jobs = yaml.safe_load((readmes.REPO / '.github/workflows/release.yml').read_text())['jobs']
        mac_steps = [step for step in jobs['smoke']['steps']
                     if 'registry-readmes.py examples' in step.get('run', '')]
        self.assertEqual(len(mac_steps), 1)
        self.assertEqual(mac_steps[0]['if'], "runner.os == 'macOS'")
        mac_families = set(readmes.re.findall(r'--family ([a-z-]+)', mac_steps[0]['run']))
        draft = (readmes.REPO / 'sdlc/scripts/release-workflow').read_text().split('\ndraft)\n')[1].split('\n\t;;')[0]
        linux_families = set(readmes.re.findall(r'--family ([a-z-]+)', draft))
        self.assertEqual(mac_families, {'objective-c'})
        self.assertNotIn('objective-c', linux_families)
        self.assertEqual(linux_families | mac_families, set(readmes.published_families()))
        self.assertIn('smoke', jobs['draft']['needs'])
        self.assertTrue(any(step.get('with', {}).get('name') == 'crate-package'
                            and step['with']['path'] == 'crate-dist' for step in jobs['draft']['steps']))
        self.assertTrue(any('cp r-dist/* crate-dist/*.crate all-files/' in step.get('run', '')
                            for step in jobs['draft']['steps']))

    def test_release_refuses_when_examples_cannot_run(self):
        with tempfile.TemporaryDirectory() as scratch:
            with self.assertRaisesRegex(readmes.Refusal, 'missing-or-ambiguous-example-package: command') as error:
                readmes.check_examples(Path(scratch))
            self.assertIn('example-needs-two-functions: typescript', str(error.exception))
            self.assertIn('missing-or-ambiguous-example-package: duckdb', str(error.exception))

    def test_replay_refuses_a_command_that_does_not_run(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch)
            packages = tree / 'packages'; packages.mkdir()
            package = packages / ('thinkthen-' + readmes.version() + '-x86_64-unknown-linux-musl.tar.gz')
            archive(package, {'README.md': readmes.source('command').read_bytes(),
                              'thinkthen': b'#!/bin/sh\nexit 77\n'})
            archive(packages / 'thinkthen-first-run.tar.gz', {'thinkthen-first-run/report.txt': b'fixture',
                    'thinkthen-first-run/recording/thinkthen.jsonl': b''})
            work = tree / 'work'; work.mkdir()
            with self.assertRaisesRegex(readmes.Refusal, 'example-did-not-run: command'):
                readmes.run_example('command', packages, work)


if __name__ == '__main__':
    unittest.main()
