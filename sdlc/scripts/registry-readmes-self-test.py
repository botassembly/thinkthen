#!/usr/bin/env python3
"""Exercise the package-text release boundary with positive and negative archives."""
import importlib.util
import io
import json
from pathlib import Path
import tarfile
import tempfile
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

    def test_dependency_versions_are_allowed(self):
        readmes.check_text(b'Rust 1.95.0; DuckDB 1.5.6; Zig 0.15.2; JDK 22; jsonlite 2.0.0', '0.2.0')
        readmes.check_text(b'tar -xzf thinkthen-c-0.2.0-TARGET.tar.gz', '0.2.0')

    def test_missing_readme_refuses(self):
        with tempfile.TemporaryDirectory() as scratch:
            package = Path(scratch) / 'thinkthen.tgz'
            archive(package, {'package/package.json': b'{}'})
            with self.assertRaisesRegex(readmes.Refusal, 'missing-or-repeated-member'):
                readmes.check_package('typescript', package)


if __name__ == '__main__':
    unittest.main()
