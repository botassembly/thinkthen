"""Privacy checks follow indexed source and explicit shipping membership."""
import gzip
import io
import os
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile
import unittest
import zipfile

from conformance.children import privacy


class PrivacyTests(unittest.TestCase):
    def test_private_markers_and_valid_build_prefix(self):
        for marker in (b'/home/private', b'/Users/private', b'auth.json', b'-----BEGIN PRIVATE KEY-----',
                       b'tt-canary-private', b'PRIVATE_PATH_MARKER', b'PRIVATE_KEY_MARKER', b'sk-private-SECRET123',
                       b'/build/home/private'):
            with self.subTest(marker=marker), self.assertRaisesRegex(privacy.PrivacyError, 'private pattern'):
                privacy.check_bytes('shipping member', marker)
        privacy.check_bytes('remapped native', b'/build/host/workspace/source.rs')

    def test_compressed_members_names_and_link_targets(self):
        with tempfile.TemporaryDirectory() as temporary:
            folder = Path(temporary)
            for kind in ('zip', 'tar', 'gz'):
                with self.subTest(kind=kind):
                    path = folder / ('shipping.' + kind)
                    if kind == 'zip':
                        with zipfile.ZipFile(path, 'w', compression=zipfile.ZIP_DEFLATED) as bundle:
                            bundle.writestr('source.txt', b'PRIVATE_KEY_MARKER')
                    elif kind == 'tar':
                        with tarfile.open(path, 'w:gz') as bundle:
                            member = tarfile.TarInfo('source.txt')
                            member.size = len(b'PRIVATE_KEY_MARKER')
                            bundle.addfile(member, io.BytesIO(b'PRIVATE_KEY_MARKER'))
                    else:
                        path.write_bytes(gzip.compress(b'PRIVATE_KEY_MARKER'))
                    with self.assertRaisesRegex(privacy.PrivacyError, 'private pattern'):
                        privacy.check_file(path)
            path = folder / 'names.zip'
            with zipfile.ZipFile(path, 'w', compression=zipfile.ZIP_DEFLATED) as bundle:
                bundle.writestr('tt-canary-private', b'public content')
            with self.assertRaises(privacy.PrivacyError):
                privacy.check_file(path)
            path = folder / 'links.tar.gz'
            with tarfile.open(path, 'w:gz') as bundle:
                link = tarfile.TarInfo('public-link')
                link.type = tarfile.SYMTYPE
                link.linkname = '/home/private'
                bundle.addfile(link)
            with self.assertRaises(privacy.PrivacyError):
                privacy.check_file(path)

    def test_tracked_sources_and_shipped_archives_ignore_working_output(self):
        with tempfile.TemporaryDirectory() as temporary:
            checkout = Path(temporary) / 'checkout'
            root = checkout / 'libraries/dart'
            root.mkdir(parents=True)
            env = {name: value for name, value in os.environ.items() if not name.startswith('GIT_')}
            subprocess.run(['git', 'init', '-q', str(checkout)], env=env, check=True)
            for relative in ('pubspec.yaml', 'flutter/pubspec.yaml', 'flutter/example/pubspec.yaml',
                             'checks/session_consumer/pubspec.yaml', 'lib/target/pubspec.yaml'):
                path = root / relative
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text('name: fixture')
                subprocess.run(['git', '-C', str(root), 'add', relative], env=env, check=True)
                path.write_text('PRIVATE_KEY_MARKER')
                self.assertIn(path, privacy.source_files(root, ['*.yaml']))
                with self.assertRaises(privacy.PrivacyError):
                    privacy.check_file(path)
                path.write_text('name: fixture')
            for output in ('target/core-wave', 'target/core-wave-flutter', 'flutter/build', '.dart_tool', 'cache', 'untracked'):
                generated = root / output / 'pubspec.yaml'
                generated.parent.mkdir(parents=True, exist_ok=True)
                generated.write_text('PRIVATE_KEY_MARKER')
            for path in privacy.source_files(root, ['*.yaml']):
                privacy.check_file(path)
            shipped = root / 'target/shipped.tar.gz'
            with tarfile.open(shipped, 'w:gz') as bundle:
                bundle.add(generated, arcname='pubspec.yaml')
            with self.assertRaises(privacy.PrivacyError):
                privacy.check_file(shipped)
            # The Dart development package must still copy only indexed inputs.
            sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'libraries/dart/checks'))
            from native_assets import development
            definition = root / 'native-assets.json'
            definition.write_bytes((Path(__file__).resolve().parents[2] / 'libraries/dart/native-assets.json').read_bytes())
            subprocess.run(['git', '-C', str(root), 'add', 'native-assets.json'], env=env, check=True)
            native = Path(temporary) / 'libthinkthen.so'
            native.write_bytes(b'development fixture')
            copied = development(root, native, Path(temporary) / 'owned')
            for path in privacy.source_files(root, ['*']):
                self.assertTrue((copied / path.relative_to(root)).is_file())
            for output in ('target', 'cache', 'untracked', 'flutter/build', '.dart_tool'):
                self.assertFalse((copied / output).exists())

    def test_manifest_is_authoritative_and_cannot_escape_prefix(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            prefix = root / 'installed'
            prefix.mkdir()
            public = prefix / 'public.h'
            public.write_text('public source')
            (prefix / 'later-output').write_bytes(b'PRIVATE_KEY_MARKER')
            manifest = root / 'manifest'
            manifest.write_text(str(public) + '\n')
            command = [sys.executable, str(Path(privacy.__file__)), '--prefix', str(prefix), '--manifest', str(manifest)]
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn('PRIVATE_SOURCE_PASS', result.stdout)
            public.write_bytes(b'PRIVATE_KEY_MARKER')
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertEqual(result.returncode, 1)
            self.assertIn('private pattern in public.h', result.stderr)
            outside = root / 'outside.h'
            outside.write_text('public source')
            manifest.write_text(str(outside) + '\n')
            result = subprocess.run(command, capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('INSTALL_MEMBER_OUTSIDE_PREFIX', result.stderr)


if __name__ == '__main__':
    unittest.main()
