"""Privacy checks follow indexed source and explicit shipping membership."""
import gzip
import io
import json
import os
from pathlib import Path
import platform
import shutil
import subprocess
import struct
import sys
import tarfile
import tempfile
import unittest
import zipfile

from conformance.children import privacy
from conformance.children.children import child_env
from conformance.source_fixtures import stage_checkout


class PrivacyTests(unittest.TestCase):
    def test_profiles_refuse_before_reading_an_artifact(self):
        repo = Path(__file__).resolve().parents[2]
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            env = child_env(home=root / 'home', THINKTHEN_TEST_PROFILE='invalid',
                            THINKTHEN_ARTIFACT=str(root / 'missing.tar.gz'))
            for surface in ('libraries/c', 'libraries/rust', 'libraries/polars', 'libraries/ruby',
                            'libraries/typescript', 'libraries/python', 'libraries/php', 'libraries/go',
                            'libraries/zig', 'libraries/r', 'databases/duckdb', 'databases/sqlite'):
                with self.subTest(surface=surface):
                    result = subprocess.run(['sh', str(repo / surface / 'check.sh')], env=env,
                                            capture_output=True, text=True, timeout=30)
                    self.assertEqual(result.returncode, 2, result.stderr)
                    self.assertIn('profile', result.stderr.lower())
                    self.assertNotIn('missing.tar.gz', result.stderr)

    def test_missing_artifacts_refuse_before_host_and_tool_checks(self):
        repo = Path(__file__).resolve().parents[2]
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            artifact = root / 'missing.tar.gz'
            env = child_env(home=root / 'home', THINKTHEN_ARTIFACT=str(artifact))
            for surface in ('libraries/php', 'libraries/go', 'libraries/ruby',
                            'libraries/objective-c', 'libraries/ada', 'libraries/cobol',
                            'libraries/typescript', 'libraries/swift'):
                with self.subTest(surface=surface):
                    result = subprocess.run(['sh', str(repo / surface / 'check.sh')], env=env,
                                            capture_output=True, text=True, timeout=30)
                    self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
                    self.assertEqual(result.stdout,
                                     f'{surface}: missing input THINKTHEN_ARTIFACT: {artifact}\n')
                    self.assertEqual(result.stderr, '')

    def test_selected_artifacts_must_exist_and_pass_privacy(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            command = [sys.executable, str(Path(privacy.__file__)), '--artifact-env']
            for name in ('THINKTHEN_ARTIFACT', 'THINKTHEN_DART_ARTIFACT', 'THINKTHEN_C_ARTIFACT'):
                with self.subTest(name=name):
                    path = root / 'distribution.txt'
                    env = child_env(home=root / 'home', THINKTHEN_RESOLVED_TOOLS='c',
                                    **{name: str(path)})
                    result = subprocess.run(command, env=env, capture_output=True, text=True)
                    self.assertEqual(result.returncode, 1)
                    self.assertIn('distribution.txt', result.stderr)
                    if name == 'THINKTHEN_ARTIFACT':
                        gate = ['sh', str(Path(privacy.__file__).resolve().parents[2] / 'libraries/c/check.sh')]
                        result = subprocess.run(gate, env=env, capture_output=True, text=True, timeout=30)
                        self.assertEqual(result.returncode, 1)
                        self.assertIn('missing input THINKTHEN_ARTIFACT', result.stdout)
                        self.assertIn('distribution.txt', result.stdout)
                    path.write_bytes(b'PRIVATE_KEY_MARKER')
                    result = subprocess.run(command, env=env, capture_output=True, text=True)
                    self.assertEqual(result.returncode, 1)
                    self.assertIn('private pattern in distribution.txt', result.stderr)
                    if name == 'THINKTHEN_ARTIFACT':
                        result = subprocess.run(gate, env=env, capture_output=True, text=True, timeout=30)
                        self.assertEqual(result.returncode, 1)
                        self.assertIn('private pattern in distribution.txt', result.stderr)
                    path.unlink()

    def test_generated_archives_refuse_private_bytes_before_installing(self):
        repo = Path(__file__).resolve().parents[2]
        for surface, profile in (('rust', 'stress'), ('zig', 'smoke')):
            with self.subTest(surface=surface), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary) / 'checkout'
                root.mkdir()
                stage_checkout(repo, root)
                tools = root / 'bin'
                tools.mkdir(exist_ok=True)
                (tools / 'python3').symlink_to(sys.executable)
                marker = root / 'installed-private-archive'
                tar = tools / 'tar'
                tar.write_text('#!/bin/sh\n'
                               'if [ "$1" = -xzf ]; then touch "$INSTALL_MARKER"; fi\n'
                               'exec "$REAL_TAR" "$@"\n')
                tar.chmod(0o755)
                env = child_env(home=root / 'home', PATH=str(tools) + ':' + os.environ.get('PATH', os.defpath),
                                THINKTHEN_RESOLVED_TOOLS=surface, THINKTHEN_TEST_PROFILE=profile,
                                INSTALL_MARKER=str(marker), REAL_TAR=shutil.which('tar'),
                                THINKTHEN_BASE_URL='http://127.0.0.1:1', THINKTHEN_CACHE=str(root / 'cache'))
                if surface == 'rust':
                    archive = root / 'hostile.crate'
                    with tarfile.open(archive, 'w:gz') as bundle:
                        member = tarfile.TarInfo('thinkthen-0.2.0/private.txt')
                        member.size = len(b'PRIVATE_KEY_MARKER')
                        bundle.addfile(member, io.BytesIO(b'PRIVATE_KEY_MARKER'))
                    cargo = tools / 'cargo'
                    cargo.write_text('#!' + sys.executable + '\n'
                                     'import os, pathlib, shutil, tomllib\n'
                                     'root = pathlib.Path(os.environ["ROUTE_ROOT"])\n'
                                     'version = tomllib.loads((root / "crates/thinkthen/Cargo.toml").read_text())["package"]["version"]\n'
                                     'target = root / "target/package"\n'
                                     'target.mkdir(parents=True, exist_ok=True)\n'
                                     'shutil.copyfile(root / "hostile.crate", target / f"thinkthen-{version}.crate")\n')
                    cargo.chmod(0o755)
                    env['ROUTE_ROOT'] = str(root)
                    (root / 'libraries/rust/check.py').write_text('raise SystemExit("installed consumer reached")\n')
                else:
                    # The package-only route needs no Linux compiler on the build host.
                    manifest = root / 'sdlc/tools.json'
                    tools_manifest = json.loads(manifest.read_text())
                    arch = {'aarch64': 'arm64'}.get(platform.machine(), platform.machine())
                    tools_manifest['libraries']['zig']['check_platforms'].append(platform.system().lower() + '-' + arch)
                    manifest.write_text(json.dumps(tools_manifest))
                    # The real Zig packager copies the selected native installation.
                    with (root / 'sdlc/scripts/installed.sh').open('a') as script:
                        script.write('\nnative_install() {\n'
                                     'mkdir -p "$2/include" "$2/lib"\n'
                                     'printf "public header" > "$2/include/thinkthen.h"\n'
                                     'printf "PRIVATE_KEY_MARKER" > "$2/lib/libthinkthen.a"\n}\n')
                    (root / 'build-tools/results/generate.py').write_text('')
                    zig = tools / 'zig'
                    zig.write_text('#!/bin/sh\n[ "$1" = fmt ] || exit 9\n')
                    zig.chmod(0o755)
                result = subprocess.run(['sh', str(root / f'libraries/{surface}/check.sh')],
                                        cwd=root, env=env, capture_output=True, text=True, timeout=90)
                self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
                self.assertIn('private pattern in thinkthen-', result.stderr)
                self.assertFalse(marker.exists(), result.stdout + result.stderr)

    def test_crate_fixture_allowances_keep_other_rules_and_paths_active(self):
        repo = Path(__file__).resolve().parents[2]
        members = ('src/config/tests.rs', 'tests/library/ca_bundle.rs',
                   'tests/library/tls/localhost.key', 'tests/library/tls/other-host.key')
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            archive = root / 'source.crate'
            unpacked = root / 'thinkthen-0.2.0'
            for name in members:
                source = repo / 'crates/thinkthen' / name
                privacy.check_file(source)
                dest = unpacked / name
                dest.parent.mkdir(parents=True, exist_ok=True)
                dest.write_bytes(source.read_bytes())
                privacy.check_file(dest)
            # Both stored and compressed archives must inspect each member by its exact path.
            for mode in ('w', 'w:gz'):
                with self.subTest(mode=mode):
                    with tarfile.open(archive, mode) as bundle:
                        bundle.add(unpacked, arcname=unpacked.name)
                    privacy.check_file(archive)
            for compression in (zipfile.ZIP_STORED, zipfile.ZIP_DEFLATED):
                with self.subTest(compression=compression):
                    with zipfile.ZipFile(archive, 'w', compression=compression) as bundle:
                        for name in members:
                            bundle.write(unpacked / name, unpacked.name + '/' + name)
                    privacy.check_file(archive)
            # Non-member bytes remain subject to all rules, even beside allowed content.
            for kind in ('tar', 'zip'):
                with self.subTest(trailer=kind):
                    if kind == 'tar':
                        with tarfile.open(archive, 'w') as bundle:
                            bundle.add(unpacked, arcname=unpacked.name)
                    else:
                        with zipfile.ZipFile(archive, 'w') as bundle:
                            bundle.write(unpacked / members[0], unpacked.name + '/' + members[0])
                            bundle.comment = b'PRIVATE_KEY_MARKER'
                    with archive.open('ab') as output:
                        output.write(b'PRIVATE_KEY_MARKER')
                    with self.assertRaisesRegex(privacy.PrivacyError, 'private pattern'):
                        privacy.check_file(archive)
            with zipfile.ZipFile(archive, 'w') as bundle:
                bundle.comment = b'PRIVATE_KEY_MARKER'
            with self.assertRaisesRegex(privacy.PrivacyError, 'private pattern'):
                privacy.check_file(archive)
            plants = [(name, b'PRIVATE_KEY_MARKER') for name in members]
            plants += [('src/config/nested/tests.rs', (unpacked / members[0]).read_bytes()),
                       ('tests/extra.rs', (unpacked / members[1]).read_bytes())]
            plants += [('src/config/tests.rs', b'/home/private'),
                       ('src/config/tests.rs', b'/Users/private'),
                       ('src/config/tests.rs', b'/home/person-extra'),
                       ('src/config/tests.rs', b'/Users/person-extra'),
                       ('tests/library/ca_bundle.rs', b'-----BEGIN PRIVATE KEY-----\\nREALKEY'),
                       ('src/config/production.rs', b'"/home/person"'),
                       ('src/config/production.rs', b'"/Users/person"'),
                       ('tests/extra.rs', b'-----BEGIN PRIVATE KEY-----'),
                       ('tests/library/tls/extra.key', b'-----BEGIN PRIVATE KEY-----'),
                       ('src/production.key', b'-----BEGIN PRIVATE KEY-----')]
            for name, plant in plants:
                with self.subTest(name=name, plant=plant):
                    original = (repo / 'crates/thinkthen' / name)
                    data = (original.read_bytes() if original.is_file() else b'') + b'\n' + plant
                    with tarfile.open(archive, 'w:gz') as bundle:
                        member = tarfile.TarInfo('thinkthen-0.2.0/' + name)
                        member.size = len(data)
                        bundle.addfile(member, io.BytesIO(data))
                    with self.assertRaisesRegex(privacy.PrivacyError, 'private pattern'):
                        privacy.check_file(archive)
            # Allowances never cover names, symlink targets or container metadata.
            with tarfile.open(archive, 'w:gz', format=tarfile.PAX_FORMAT) as bundle:
                member = tarfile.TarInfo('thinkthen-0.2.0/src/config/tests.rs')
                member.pax_headers = {'comment': '/home/person'}
                bundle.addfile(member, io.BytesIO())
            with self.assertRaisesRegex(privacy.PrivacyError, 'private pattern'):
                privacy.check_file(archive)

    def test_archive_allowances_never_hide_unscanned_bytes_or_alias_paths(self):
        with tempfile.TemporaryDirectory() as temporary:
            path = Path(temporary) / 'source.crate'
            for field in ('uname', 'gname', 'trailer'):
                with self.subTest(field=field):
                    content = io.BytesIO()
                    with tarfile.open(fileobj=content, mode='w') as bundle:
                        member = tarfile.TarInfo('public.txt')
                        if field != 'trailer':
                            setattr(member, field, 'PRIVATE_KEY_MARKER')
                        bundle.addfile(member)
                    data = content.getvalue()
                    if field == 'trailer':
                        data += b'PRIVATE_KEY_MARKER'
                    path.write_bytes(gzip.compress(data))
                    with self.assertRaises(privacy.PrivacyError):
                        privacy.check_file(path)
            # Stored ZIP sizes can declare more bytes than the reader returns.
            with zipfile.ZipFile(path, 'w') as bundle:
                bundle.writestr('public.txt', b'xPRIVATE_KEY_MARKER')
            data = bytearray(path.read_bytes())
            struct.pack_into('<I', data, 22, 1)
            central = data.index(b'PK\x01\x02')
            struct.pack_into('<I', data, central + 24, 1)
            import zlib
            crc = zlib.crc32(b'x')
            struct.pack_into('<I', data, 14, crc)
            struct.pack_into('<I', data, central + 16, crc)
            path.write_bytes(data)
            with self.subTest(kind='stored-zip-extra-payload'), self.assertRaises(privacy.PrivacyError):
                privacy.check_file(path)
            for name in ('/tmp/thinkthen-0.2.0/src/config/tests.rs',
                         'thinkthen-0.2.0/src/config/./tests.rs',
                         'thinkthen-0.2.0//src/config/tests.rs',
                         './thinkthen-0.2.0/src/config/tests.rs'):
                with self.subTest(name=name):
                    with zipfile.ZipFile(path, 'w') as bundle:
                        bundle.writestr(name, b'"/home/person"')
                    with self.assertRaises(privacy.PrivacyError):
                        privacy.check_file(path)

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
            checkout = Path(temporary).resolve() / 'checkout'
            root = checkout / 'libraries/dart'
            root.mkdir(parents=True)
            env = child_env(home=Path(temporary) / 'home')
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
            result = subprocess.run(command, env=child_env(home=root / 'home'), capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertIn('PRIVATE_SOURCE_PASS', result.stdout)
            public.write_bytes(b'PRIVATE_KEY_MARKER')
            result = subprocess.run(command, env=child_env(home=root / 'home'), capture_output=True, text=True)
            self.assertEqual(result.returncode, 1)
            self.assertIn('private pattern in public.h', result.stderr)
            outside = root / 'outside.h'
            outside.write_text('public source')
            manifest.write_text(str(outside) + '\n')
            result = subprocess.run(command, env=child_env(home=root / 'home'), capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('INSTALL_MEMBER_OUTSIDE_PREFIX', result.stderr)


if __name__ == '__main__':
    unittest.main()
