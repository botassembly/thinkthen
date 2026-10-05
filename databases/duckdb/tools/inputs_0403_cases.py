"""Portable routing/refusal regressions. Synthetic artifacts establish no native ABI proof."""
from __future__ import annotations

import hashlib
import os
import re
import subprocess
import sys
import tarfile
import tempfile
import threading
import unittest
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

from inputs import HERE, PLATFORMS, authority, selected, versions
from release_pack_cases import REPO, copy_packer, pack, reject_lane_cleanup
from validate_inputs import archive_tree, inventory, static_archives

TARGET = 'x86_64-unknown-linux-gnu'
NEWEST = 'v1.5.5'
OLDER = 'v1.5.4'
OLDER_STATIC = {
    TARGET: ('linux_amd64', '44edc1b55365624b4aa4a4f1d8087f75c4bfaceed4494b71059f54a8fa2f6e45', 22),
    'aarch64-unknown-linux-gnu': ('linux_arm64', '68133154f3f62f5b8704656ece5b67989a3089537e4944e0b372518843155e62', 22),
    'aarch64-apple-darwin': ('osx_arm64', '7d6d51110134c031e8a4944a8aa6514d68d462c479346a42f3e38bd7e4158c83', 21),
    'x86_64-apple-darwin': ('osx_amd64', 'f102a62959e3cc7f2147c3181e6b86be158f70183889bb7b40624a4c18d886f3', 21),
}


def run(args, env, cwd=None):
    return subprocess.run(args, env=env, cwd=cwd, capture_output=True, text=True, check=False, timeout=90)


def script(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text('#!/bin/sh\nset -eu\n' + text + '\n')
    path.chmod(0o755)


def footer(version=NEWEST):
    # Independent encoding: 22-byte prefix, 16 fixed 32-byte fields.
    fields = [b''] * 16
    for index, value in ((3, 'CPP'), (4, '0.2.0'), (5, version), (6, 'linux_amd64'), (7, '4')):
        fields[index] = value.encode()
    return b'synthetic-metadata-only\n' + b'\x00\x93\x04\x10duckdb_signature\x80\x04' + b''.join(v.ljust(32, b'\0') for v in fields)


class Cases(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        reject_lane_cleanup()  # Before the first owned scratch cleanup.

    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='thinkthen-0403-portable-')
        self.root = Path(self.temp.name)
        self.home = self.root / 'home'
        self.home.mkdir()
        self.env = {'PATH': os.environ.get('PATH', '/usr/bin:/bin'), 'HOME': str(self.home), 'LC_ALL': 'C.UTF-8',
                    'PYTHONDONTWRITEBYTECODE': '1', 'GIT_CONFIG_NOSYSTEM': '1', 'GIT_CONFIG_GLOBAL': '/dev/null',
                    'CARGO_NET_OFFLINE': 'true', 'RUSTC_WRAPPER': '', 'THINKTHEN_API_KEY': 'sk-loopback-0403-fixture',
                    'THINKTHEN_BASE_URL': BACKEND}

    def tearDown(self):
        self.temp.cleanup()

    def test_literal_inputs_and_separate_paths(self):
        expected = dict(re.findall(r"^(linux|linux_arm64|mac|mac_intel)='([^']+)'$", (HERE / 'selftests.sh').read_text(), re.M))
        shim = self.root / 'shim'
        script(shim / 'uname', 'case $1 in -s) echo "$MOCK_KERNEL";; -m) echo "$MOCK_CHIP";; esac')
        for name, kernel, chip in [('linux', 'Linux', 'x86_64'), ('linux_arm64', 'Linux', 'aarch64'), ('mac', 'Darwin', 'arm64'), ('mac_intel', 'Darwin', 'x86_64')]:
            env = {**self.env, 'PATH': f'{shim}:{self.env["PATH"]}', 'MOCK_KERNEL': kernel, 'MOCK_CHIP': chip}
            result = run(['/bin/sh', str(HERE / 'setup.sh'), '--inputs'], env)
            self.assertEqual((result.returncode, result.stdout.strip()), (0, expected[name]))
        for target, (platform, digest, count) in OLDER_STATIC.items():
            item = selected(OLDER, target)
            self.assertEqual((item['platform'], item['static_zip_sha256']), (platform, digest))
            self.assertEqual(item['source_commit'], '08e34c447bae34eaee3723cac61f2878b6bdf787')
            self.assertEqual(len(inventory(HERE.parent / 'cpp' / item['manifest'])), count)
        self.assertEqual(versions(), [NEWEST, OLDER])
        with self.assertRaisesRegex(ValueError, 'unsupported DuckDB target'):
            selected(NEWEST, 'x86_64-pc-windows-msvc')

    def test_bad_authority_and_selector_send_no_tool_work(self):
        script(self.root / 'shim/cmake', f'touch "{self.root}/cmake-ran"; exit 99')
        env = {**self.env, 'PATH': f'{self.root}/shim:{self.env["PATH"]}', 'THINKTHEN_DUCKDB_VERSION': 'v9.9.9'}
        for entry in (HERE / 'setup.sh', HERE.parent / 'cpp/build.sh'):
            result = run(['/bin/sh', str(entry), '--inputs'] if entry.name == 'setup.sh' else ['/bin/sh', str(entry)], env)
            self.assertEqual(result.returncode, 2)
            self.assertIn('unsupported DuckDB version: v9.9.9', result.stderr)
        self.assertFalse((self.root / 'cmake-ran').exists())
        original = (HERE / 'version.env').read_text()
        for value in ('"v1.5.5 v1.5.5"', '"v1.5.5  v1.5.4"', '"../v1.5.4"', '$(touch marker)', 'v1.5.5\nDUCKDB_VERSIONS=v1.5.4'):
            path = self.root / 'version.env'
            path.write_text(original.replace('"v1.5.5 v1.5.4"', value))
            with self.assertRaisesRegex(ValueError, 'malformed|duplicate'):
                authority(path)

    def test_static_input_failure_causes(self):
        static = self.root / 'static'; static.mkdir()
        archive = static / 'libfixture.a'; archive.write_bytes(b'fixture archive')
        manifest = self.root / 'manifest.txt'
        manifest.write_text(hashlib.sha256(archive.read_bytes()).hexdigest() + ' libfixture.a\n')
        static_archives(static, manifest)
        archive.write_bytes(b'changed')
        with self.assertRaisesRegex(ValueError, 'differs from the pinned release'):
            static_archives(static, manifest)
        extra = static / 'libextra.a'; extra.write_bytes(b'')
        with self.assertRaisesRegex(ValueError, 'set differs'):
            static_archives(static, manifest)
        manifest.write_text(manifest.read_text() * 2)
        with self.assertRaisesRegex(ValueError, 'duplicate'):
            inventory(manifest)

    def source_fixture(self, folder):
        folder.mkdir()
        run(['git', 'init', '-q', str(folder)], self.env)
        (folder / 'source.cc').write_text('original\n')
        (folder / '.gitattributes').write_text('source.cc filter=marker\n')
        (folder / '.gitignore').write_text('hidden\n')
        run(['git', '-C', str(folder), 'add', '.'], self.env)
        result = run(['git', '-C', str(folder), '-c', 'user.name=fixture', '-c', 'user.email=fixture@example.invalid', 'commit', '-qm', 'fixture'], self.env)
        self.assertEqual(result.returncode, 0, result.stderr)
        return run(['git', '-C', str(folder), 'rev-parse', 'HEAD'], self.env).stdout.strip()

    def test_raw_git_bypasses_filters_index_and_flags(self):
        for flag in ('--assume-unchanged', '--skip-worktree'):
            folder = self.root / flag[2:]; commit = self.source_fixture(folder)
            # Set index flags before arming the filter. update-index may refresh via filters.
            run(['git', '-C', str(folder), 'update-index', flag, 'source.cc'], self.env)
            marker = self.root / (flag[2:] + '-filter-ran')
            for kind in ('clean', 'smudge'):
                run(['git', '-C', str(folder), 'config', f'filter.marker.{kind}', f'touch {marker}; cat'], self.env)
            self.assertEqual(archive_tree.raw_git(folder, commit), 3)
            (folder / 'source.cc').write_text('masked mutation\n')
            with self.assertRaisesRegex(ValueError, 'raw Git source file differs'):
                archive_tree.raw_git(folder, commit)
            (folder / 'source.cc').write_text('original\n')
            (folder / 'source.cc').chmod(0o755)
            with self.assertRaisesRegex(ValueError, 'raw Git source file differs'):
                archive_tree.raw_git(folder, commit)
            (folder / 'source.cc').chmod(0o644)
            script(folder / 'hidden', 'exit 0')
            with self.assertRaisesRegex(ValueError, 'unexpected source input: hidden'):
                archive_tree.raw_git(folder, commit)
            self.assertFalse(marker.exists(), 'the raw guard ran a Git filter')
            with self.assertRaisesRegex(ValueError, 'differs from the pinned commit'):
                archive_tree.raw_git(folder, '0' * 40)

    def test_node_reader_requires_final_canonical(self):
        authority_file = HERE / 'version.env'
        code = """import {readDuckDBVersions,nativeDuckDBTarget,duckDBArtifact} from './site/scripts/duckdb-inputs.mjs';
import assert from 'node:assert/strict';
assert.deepEqual(readDuckDBVersions(process.argv[1]), ['v1.5.5','v1.5.4']);
assert.deepEqual(nativeDuckDBTarget('darwin','x64'), ['x86_64-apple-darwin','osx_amd64']);
assert.throws(() => duckDBArtifact(process.argv[2], 'v1.5.5', 'x86_64-unknown-linux-gnu'), /ENOENT/);
"""
        obsolete = self.root / 'databases/duckdb/build/artifacts/cpp' / TARGET
        obsolete.mkdir(parents=True)
        (obsolete / 'thinkthen.duckdb_extension').write_bytes(footer())
        (self.root / 'databases/duckdb/build/thinkthen.duckdb_extension').write_bytes(footer())
        result = run(['node', '--input-type=module', '-e', code, str(authority_file), str(self.root)], self.env, REPO)
        self.assertEqual(result.returncode, 0, result.stderr)

    def test_real_scratch_packer_refuses_obsolete_and_footer(self):
        tree = self.root / 'tree'; copy_packer(tree)
        shim = self.root / 'shim'
        script(shim / 'rustc', f'echo "host: {TARGET}"')
        script(shim / 'readelf', 'echo "Machine: Advanced Micro Devices X86-64"')
        env = {**self.env, 'PATH': f'{shim}:{self.env["PATH"]}'}
        member = tree / 'databases/duckdb/build/artifacts/cpp' / NEWEST / TARGET / 'thinkthen.duckdb_extension'
        member.parent.mkdir(parents=True)
        member.write_bytes(footer(OLDER))
        result = pack(tree, TARGET, self.root / 'wrong', env)
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertIn('DuckDB footer DuckDB version is', result.stderr)
        self.assertEqual(list((self.root / 'wrong').glob('*.tar.gz*')), [])
        member.rename(self.root / 'backup')
        obsolete = tree / 'databases/duckdb/build/artifacts/cpp' / TARGET / member.name
        obsolete.parent.mkdir(parents=True); obsolete.write_bytes(footer())
        (tree / 'databases/duckdb/build' / member.name).write_bytes(footer())
        result = pack(tree, TARGET, self.root / 'missing', env)
        self.assertEqual(result.returncode, 1)
        self.assertIn(f'{member.relative_to(tree)} is missing', result.stderr)
        result = pack(tree, TARGET, self.root / 'conflict', {**env, 'THINKTHEN_DUCKDB_VERSION': OLDER})
        self.assertEqual(result.returncode, 2)
        self.assertIn('single-version archive requires the default DuckDB selector', result.stderr)
        self.assertEqual(list((self.root / 'missing').glob('*.tar.gz*')), [])

    def test_separate_build_outputs_and_alias_ownership(self):
        from build_0403_cases import routing
        routing(self, footer, script)

    def test_synthetic_old_format_pack_and_source_guard(self):
        tree = self.root / 'tree'; copy_packer(tree)
        source = self.root / 'dependency-source'; self.source_fixture(source)
        (source / 'LICENSE').write_text('fixture MIT license\n')
        (source / 'NOTICE').write_text('fixture source notice\n')
        run(['git', '-C', str(source), 'add', '.'], self.env)
        run(['git', '-C', str(source), '-c', 'user.name=fixture', '-c', 'user.email=fixture@example.invalid', 'commit', '-qm', 'legal fixture'], self.env)
        commit = run(['git', '-C', str(source), 'rev-parse', 'HEAD'], self.env).stdout.strip()
        authority_path = tree / 'databases/duckdb/tools/version.env'
        text = authority_path.read_text().replace('d8cdaa33fda8df955cc76ef58a280f68f4cd43fa', commit)
        authority_path.write_text(text)
        static = self.root / 'static'; static.mkdir()
        (static / 'libduckdb_static.a').write_bytes(b'synthetic archive')
        (tree / 'databases/duckdb/cpp/archive-sha256.txt').write_text(hashlib.sha256(b'synthetic archive').hexdigest() + ' libduckdb_static.a\n')
        shim = self.root / 'shim'
        script(shim / 'rustc', f'echo "host: {TARGET}"')
        script(shim / 'readelf', 'echo "Machine: Advanced Micro Devices X86-64"')
        script(shim / 'cargo', """echo '{"packages":[]}'""")
        env = {**self.env, 'PATH': f'{shim}:{self.env["PATH"]}', 'THINKTHEN_DUCKDB_CPP_SOURCE': str(source),
               'THINKTHEN_DUCKDB_CPP_STATIC_DIR': str(static)}
        member = tree / 'databases/duckdb/build/artifacts/cpp' / NEWEST / TARGET / 'thinkthen.duckdb_extension'
        member.parent.mkdir(parents=True); member.write_bytes(footer())
        result = pack(tree, TARGET, self.root / 'positive', env)
        self.assertEqual(result.returncode, 0, result.stderr)
        archive = self.root / 'positive' / f'thinkthen-duckdb-0.2.0-{TARGET}.tar.gz'
        self.assertTrue(Path(str(archive) + '.sha256').is_file())
        with tarfile.open(archive) as packed:
            extensions = [m for m in packed if m.name.endswith('.duckdb_extension')]
            self.assertEqual([m.name for m in extensions], ['./thinkthen.duckdb_extension'])
            self.assertEqual(packed.extractfile(extensions[0]).read(), footer())
        (source / 'source.cc').write_text('changed source\n')
        result = pack(tree, TARGET, self.root / 'bad-source', env)
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertIn('raw Git source file differs: source.cc', result.stderr)
        self.assertEqual(list((self.root / 'bad-source').glob('*.tar.gz*')), [])


if __name__ == '__main__':
    class Counter(BaseHTTPRequestHandler):
        count = 0
        def do_POST(self):
            type(self).count += 1
            self.send_response(500); self.end_headers()
        def log_message(self, *_):
            pass
    with ThreadingHTTPServer(('127.0.0.1', 0), Counter) as server:
        BACKEND = f'http://127.0.0.1:{server.server_port}/v1'
        thread = threading.Thread(target=server.serve_forever, daemon=True); thread.start()
        result = unittest.TextTestRunner(verbosity=1).run(unittest.defaultTestLoader.loadTestsFromTestCase(Cases))
        server.shutdown(); thread.join()
    print(f'portable cases: {result.testsRun}; failures: {len(result.failures)}; errors: {len(result.errors)}; counted loopback requests: {Counter.count}')
    sys.exit(0 if result.wasSuccessful() and Counter.count == 0 else 1)
