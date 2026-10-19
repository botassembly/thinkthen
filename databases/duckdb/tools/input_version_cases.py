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
from unittest.mock import patch
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from pathlib import Path

from inputs import HERE, PLATFORMS, authority, selected, versions
from release_pack_cases import CARGO, REPO, child_env, copy_packer, pack, reject_lane_cleanup
from validate_inputs import archive_tree

TARGET = 'x86_64-unknown-linux-gnu'


def run(args, env, cwd=None):
    return subprocess.run(args, env=env, cwd=cwd, capture_output=True, text=True, check=False, timeout=90)


def script(path, text):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text('#!/bin/sh\nset -eu\n' + text + '\n')
    path.chmod(0o755)


class Cases(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        reject_lane_cleanup()  # Before the first owned scratch cleanup.


    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='thinkthen-0403-portable-')
        self.root = Path(self.temp.name)
        self.home = self.root / 'home'
        self.home.mkdir()
        self.env = child_env(keep=CARGO, home=self.home, LC_ALL='C.UTF-8',
                             PYTHONDONTWRITEBYTECODE='1', GIT_CONFIG_NOSYSTEM='1', GIT_CONFIG_GLOBAL='/dev/null',
                             CARGO_NET_OFFLINE='true', RUSTC_WRAPPER='',
                             THINKTHEN_API_KEY='sk-loopback-0403-fixture', THINKTHEN_BASE_URL=BACKEND)


    def tearDown(self):
        self.temp.cleanup()


    def test_profiles_validate_before_tools_and_never_fall_back(self):
        marker = self.root / 'compiler-ran'
        script(self.root / 'shim/cargo', f'touch "{marker}"; exit 99')
        env = {**self.env, 'PATH': f'{self.root}/shim:{self.env["PATH"]}',
               'THINKTHEN_TOOLCHAINS': str(self.root / 'missing-tools')}
        entry = ['/bin/sh', str(HERE.parent / 'check.sh'), '9000']
        result = run(entry, {**env, 'THINKTHEN_TEST_PROFILE': 'invalid'})
        self.assertEqual(result.returncode, 2, result.stderr)
        self.assertEqual(result.stdout, '', 'an invalid profile started tool setup')
        self.assertEqual(result.stderr, 'duckdb: unknown THINKTHEN_TEST_PROFILE: invalid\n')
        for profile in ('routine', 'full', 'stress', 'smoke'):
            # The prepared PATH and compiler sentinel belong to this missing-host
            # fixture. It exercises host refusal, without provisioning gate tools.
            result = run(entry, {**env, 'THINKTHEN_TEST_PROFILE': profile,
                                 'THINKTHEN_RESOLVED_TOOLS': 'duckdb',
                                 'THINKTHEN_ARTIFACT': str(self.root / 'selected.tar.gz')})
            self.assertEqual(result.returncode, 77, result.stderr)
            self.assertIn('toolchain is missing', result.stdout)
        self.assertFalse(marker.exists(), 'a profile compiled instead of refusing missing tools')
        result = run([sys.executable, str(HERE / 'source_hosts.py')],
                     {**env, 'THINKTHEN_DUCKDB_VERSION': 'v9.9.9'})
        self.assertEqual(result.returncode, 2, result.stderr)
        self.assertIn('unsupported DuckDB version', result.stderr)
        self.assertFalse(marker.exists())


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


    def test_source_hosts_fix_the_cli_identity_before_compiling(self):
        import source_hosts
        cache = self.root / 'cache'
        hosts = [(version, selected(version, TARGET)['source_commit'], selected(version, TARGET)['tools_commit'])
                 for version in versions()]
        for version, _, _ in hosts:
            (cache / version / 'source/.git').mkdir(parents=True)
        with patch.object(os, 'environ', self.env), \
                patch.object(source_hosts, 'ROOT', self.root / 'duckdb'), \
                patch.object(source_hosts, 'prepare'), \
                patch.object(source_hosts, 'prepare_backend'), \
                patch.object(source_hosts, 'stage'), \
                patch.object(source_hosts, 'cli_runner'), \
                patch('named_options.check'), patch('native_records.check'), \
                patch.object(source_hosts.subprocess, 'run') as compiler:
            self.assertEqual(source_hosts.check_hosts(cache, False), 0)
        builds = [call.args[0] for call in compiler.call_args_list if call.args[0][-1] == 'release']
        self.assertEqual(len(builds), len(hosts))
        for command, (version, commit, _) in zip(builds, hosts):
            with self.subTest(version=version):
                flags = next(value.split('=', 1)[1] for value in command if value.startswith('EXT_RELEASE_FLAGS='))
                # Git's default abbreviation can be seven characters in a shallow
                # checkout. Keep the CLI identity accepted by installed consumers.
                self.assertIn('-DGIT_COMMIT_HASH=' + commit[:10], flags.split())


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
            (folder / 'source.cc').rename(folder / 'saved-original')
            (folder / 'source.cc').symlink_to(folder / 'saved-original')
            with self.assertRaisesRegex(ValueError, 'raw Git source file differs'):
                archive_tree.raw_git(folder, commit)
            self.assertFalse(marker.exists(), 'the raw guard ran a Git filter')
            with self.assertRaisesRegex(ValueError, 'differs from the pinned commit'):
                archive_tree.raw_git(folder, '0' * 40)


    def test_raw_git_refuses_untracked_linked_input_directory(self):
        folder = self.root / 'source'; commit = self.source_fixture(folder)
        outside = self.root / 'outside'; outside.mkdir()
        script(outside / 'executable', 'exit 0')
        (folder / 'linked-input').symlink_to(outside, target_is_directory=True)
        with self.assertRaisesRegex(ValueError, 'unexpected source input: linked-input'):
            archive_tree.raw_git(folder, commit)


    def test_python_source_guard_checks_raw_inputs_and_managed_core(self):
        core = self.root / 'core'; core_commit = self.source_fixture(core)
        python = self.root / 'python'; self.source_fixture(python)
        (python / 'external').mkdir()
        (python / 'external/duckdb').symlink_to(core, target_is_directory=True)
        run(['git', '-C', str(python), 'update-index', '--add', '--cacheinfo',
             f'160000,{core_commit},external/duckdb'], self.env)
        run(['git', '-C', str(python), '-c', 'user.name=fixture', '-c',
             'user.email=fixture@example.invalid', 'commit', '-qm', 'core'], self.env)
        commit = run(['git', '-C', str(python), 'rev-parse', 'HEAD'], self.env).stdout.strip()
        command = [sys.executable, str(HERE / 'validate_inputs.py'), '--source', str(core),
                   '--commit', core_commit, '--python', str(python), '--python-commit', commit]
        self.assertEqual(run(command, self.env).returncode, 0)
        for flag in ('--assume-unchanged', '--skip-worktree'):
            run(['git', '-C', str(python), 'update-index', flag, 'source.cc'], self.env)
            (python / 'source.cc').write_text('hidden Python build mutation\n')
            rejected = run(command, self.env)
            self.assertNotEqual(rejected.returncode, 0)
            self.assertIn('raw Git source file differs: source.cc', rejected.stderr)
            (python / 'source.cc').write_text('original\n')
        (python / 'hidden').write_text('untracked Python build input\n')
        self.assertIn('unexpected source input: hidden', run(command, self.env).stderr)
        (python / 'hidden').unlink()
        (core / 'source.cc').write_text('changed approved core\n')
        self.assertIn('raw Git source file differs: source.cc', run(command, self.env).stderr)


    def test_raw_git_ignores_replacement_refs(self):
        source = self.root / 'replace-source'
        commit = self.source_fixture(source)
        def git(*args, data=None):
            done = subprocess.run(['git', '-C', str(source), *args], input=data, env=self.env,
                                  capture_output=True, text=True, check=True)
            return done.stdout.strip()
        blob = git('rev-parse', commit + ':source.cc')
        changed = git('hash-object', '-w', '--stdin', data='replacement\n')
        rows = git('ls-tree', commit).replace(blob, changed)
        tree = git('mktree', data=rows + '\n')
        replacement = git('-c', 'user.name=fixture', '-c', 'user.email=fixture@example.invalid',
                          'commit-tree', tree, '-p', commit, data='replacement\n')
        for original, substitute in ((commit, replacement), (git('rev-parse', commit + '^{tree}'), tree),
                                     (blob, changed)):
            git('replace', original, substitute)
            try:
                self.assertEqual(archive_tree.raw_git(source, commit), 3)
                (source / 'source.cc').write_text('replacement\n')
                with self.assertRaisesRegex(ValueError, 'raw Git source file differs'):
                    archive_tree.raw_git(source, commit)
            finally:
                git('replace', '-d', original)
                (source / 'source.cc').write_text('original\n')

    def test_source_authority_refuses_incomplete_pins_and_old_selectors(self):
        self.assertEqual(versions(), ['v1.5.6', 'v2.0-cyanoptera'])
        for target, platform in PLATFORMS.items():
            self.assertEqual(selected('v1.5.6', target)['platform'], platform)
        for version in ('v1.5.5', 'v1.5.4', 'v1.4.5'):
            with self.assertRaisesRegex(ValueError, 'unsupported DuckDB version'):
                selected(version, TARGET)
        original = (HERE / 'version.env').read_text()
        for name in ('RELEASE_SOURCE', 'PREVIEW_TOOLS', 'PREVIEW_PYTHON'):
            damaged = re.sub(r'^DUCKDB_ROUTINE_' + name + '=.*$', '', original, flags=re.M)
            path = self.root / 'version.env'
            path.write_text(damaged)
            with self.assertRaisesRegex(ValueError, 'missing or malformed'):
                authority(path)
            result = run(['node', '--input-type=module', '-e',
                          "import {readDuckDBVersions} from './site/scripts/duckdb-inputs.mjs'; readDuckDBVersions(process.argv[1]);",
                          str(path)], self.env, REPO)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('missing or malformed', result.stderr)
    def test_changed_source_refuses_build_before_compiler(self):
        tree = self.root / 'build-tree'
        copy_packer(tree)
        cache = self.root / 'source-cache'
        source = cache / 'duckdb/community/v1.5.6/source'
        source.parent.mkdir(parents=True)
        commit = self.source_fixture(source)
        run(['git', '-C', str(source), 'tag', 'v1.5.6'], self.env)
        pins = tree / 'databases/duckdb/tools/version.env'
        pins.write_text(pins.read_text().replace(selected('v1.5.6', TARGET)['source_commit'], commit))
        run(['git', '-C', str(source), 'update-index', '--assume-unchanged', 'source.cc'], self.env)
        (source / 'source.cc').write_text('masked source change\n')
        marker = self.root / 'compiler-starts'
        shim = self.root / 'build-shim'
        for tool in ('cargo', 'cmake', 'make', 'rustc'):
            script(shim / tool, f'touch "{marker}"; exit 99')
        env = {**self.env, 'PATH': f'{shim}:{self.env["PATH"]}', 'THINKTHEN_TOOLCHAINS': str(cache)}
        result = run(['/bin/sh', str(tree / 'databases/duckdb/cpp/build.sh')], env)
        self.assertNotEqual(result.returncode, 0, result.stderr)
        self.assertIn('raw Git source file differs: source.cc', result.stderr)
        self.assertFalse(marker.exists(), 'changed source reached a compiler')

    def test_release_repository_keeps_both_source_hosts(self):
        import json
        tree = self.root / 'tree'
        copy_packer(tree)
        cache = self.root / 'tools'
        authority_path = tree / 'databases/duckdb/tools/version.env'
        text = authority_path.read_text()
        for version in versions():
            identity = selected(version, TARGET)
            source = cache / 'duckdb/community' / version / 'source'
            source.parent.mkdir(parents=True)
            self.source_fixture(source)
            (source / 'LICENSE').write_text('fixture MIT license\n')
            (source / 'NOTICE').write_text('fixture notice\n')
            run(['git', '-C', str(source), 'add', '.'], self.env)
            run(['git', '-C', str(source), '-c', 'user.name=fixture', '-c', 'user.email=fixture@example.invalid',
                 'commit', '-qm', 'legal fixture'], self.env)
            commit = run(['git', '-C', str(source), 'rev-parse', 'HEAD'], self.env).stdout.strip()
            text = text.replace(identity['source_commit'], commit)
            metadata = version if version == 'v1.5.6' else commit[:10]
            fields = [b''] * 16
            for index, value in ((3, 'CPP'), (4, '0.2.0'), (5, metadata), (6, 'linux_amd64'), (7, '4')):
                fields[index] = value.encode()
            data = b'synthetic-metadata-only\n' + b'\x00\x93\x04\x10duckdb_signature\x80\x04'
            data += b''.join(value.ljust(32, b'\0') for value in fields)
            member = tree / 'databases/duckdb/build/artifacts/cpp' / version / TARGET / 'thinkthen.duckdb_extension'
            member.parent.mkdir(parents=True)
            member.write_bytes(data)
        authority_path.write_text(text)
        registry = self.root / 'registry/example'
        registry.mkdir(parents=True)
        (registry / 'LICENSE').write_text('MIT fixture\n')
        metadata = {'packages': [{'name': 'example', 'version': '1.0.0', 'license': 'MIT',
                                 'manifest_path': str(registry / 'Cargo.toml')}]}
        shim = self.root / 'shim'
        script(shim / 'rustc', f'echo "host: {TARGET}"')
        script(shim / 'readelf', 'echo "Machine: Advanced Micro Devices X86-64"')
        script(shim / 'cargo', "printf '%s\\n' '" + json.dumps(metadata) + "'")
        env = {**self.env, 'PATH': f'{shim}:{self.env["PATH"]}', 'THINKTHEN_TOOLCHAINS': str(cache)}
        result = pack(tree, TARGET, self.root / 'positive', env)
        self.assertEqual(result.returncode, 0, result.stderr)
        archive = self.root / 'positive' / f'thinkthen-duckdb-0.2.0-{TARGET}.tar.gz'
        with tarfile.open(archive) as packed:
            extensions = [member.name for member in packed if member.name.endswith('.duckdb_extension')]
        self.assertEqual(len(extensions), 2)
        self.assertIn('./v1.5.6/linux_amd64/thinkthen.duckdb_extension', extensions)
        self.assertTrue(any(name.startswith('./' + commit[:10] + '/') for name in extensions))
        (cache / 'duckdb/community/v1.5.6/source/source.cc').write_text('changed source\n')
        result = pack(tree, TARGET, self.root / 'bad-source', env)
        self.assertEqual(result.returncode, 1, result.stderr)
        self.assertIn('raw Git source file differs', result.stderr)
        self.assertEqual(list((self.root / 'bad-source').glob('*.tar.gz*')), [])


if __name__ == '__main__':
    class Counter(BaseHTTPRequestHandler):
        count = 0
        def do_POST(self):
            type(self).count += 1
            self.send_response(500); self.send_header("Connection", "close"); self.end_headers()
        def log_message(self, *_):
            pass
    with ThreadingHTTPServer(('127.0.0.1', 0), Counter) as server:
        BACKEND = f'http://127.0.0.1:{server.server_port}/v1'
        thread = threading.Thread(target=server.serve_forever, daemon=True); thread.start()
        result = unittest.TextTestRunner(verbosity=1).run(unittest.defaultTestLoader.loadTestsFromTestCase(Cases))
        server.shutdown(); thread.join()
    print(f'portable cases: {result.testsRun}; failures: {len(result.failures)}; errors: {len(result.errors)}; counted loopback requests: {Counter.count}')
    sys.exit(0 if result.wasSuccessful() and Counter.count == 0 else 1)
