#!/usr/bin/env python3
"""Exercise missing-tool refusal and PATH isolation through the resolver command."""
import gzip
import hashlib
from http.server import BaseHTTPRequestHandler, HTTPServer
import json
import os
import platform
import shutil
import shlex
from pathlib import Path
import subprocess
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[2]
SCRIPT = ROOT / 'sdlc/scripts/tools.py'
sys.path.insert(0, str(ROOT))
from conformance.children.children import child_env
from conformance.source_fixtures import fixture_tool_groups
from sdlc.scripts.tools import download, fixture_path


class PinnedDownloads(unittest.TestCase):
    def test_identity_and_gzip_preserve_representation_and_clean_up_failures(self):
        keys = b'-----BEGIN PGP PUBLIC KEY BLOCK-----\nfixture keys\n'
        contents = {'keys': keys, 'archive': gzip.compress(b'archive contents')}
        pin = hashlib.sha256(keys).hexdigest()
        requests = []

        class Handler(BaseHTTPRequestHandler):
            def do_GET(self):
                requests.append(self.headers.get('Accept-Encoding'))
                encoding, tampered, kind = self.path.strip('/').split('/')
                body = contents[kind] + (b'tampered' if tampered == 'yes' else b'')
                if encoding == 'gzip':
                    body = gzip.compress(body)
                    if tampered == 'malformed':
                        body = body[:-4]
                self.send_response(200)
                self.send_header('Content-Encoding', encoding)
                self.send_header('Content-Length', str(len(body)))
                self.send_header("Connection", "close")
                self.end_headers()
                self.wfile.write(body)

            def log_message(self, *_):
                pass

        failures = [('identity/yes', pin, RuntimeError, 'checksum'),
                    ('gzip/yes', pin, RuntimeError, 'checksum'),
                    ('identity/no', '0' * 64, RuntimeError, 'checksum'),
                    ('gzip/no', '0' * 64, RuntimeError, 'checksum'),
                    ('gzip/malformed', pin, EOFError, 'end-of-stream'),
                    ('unknown/no', pin, RuntimeError, 'unsupported.*encoding')]
        with HTTPServer(('127.0.0.1', 0), Handler) as server:
            thread = threading.Thread(target=server.serve_forever)
            thread.start()
            try:
                with tempfile.TemporaryDirectory(prefix='pinned-download-') as folder:
                    folder = Path(folder)
                    output = folder / 'download'
                    base = f'http://127.0.0.1:{server.server_port}/'
                    for kind, expected in contents.items():
                        for encoding in ('identity', 'gzip'):
                            with self.subTest(kind=kind, encoding=encoding):
                                download(base + encoding + '/no/' + kind, output,
                                         hashlib.sha256(expected).hexdigest())
                                self.assertEqual(output.read_bytes(), expected)
                                self.assertEqual(list(folder.iterdir()), [output])
                    for existing in (False, True):
                        for route, checksum, error, message in failures:
                            with self.subTest(existing=existing, route=route, checksum=checksum):
                                output.unlink(missing_ok=True)
                                if existing:
                                    output.write_bytes(keys)
                                with self.assertRaisesRegex(error, message):
                                    download(base + route + '/keys', output, checksum)
                                self.assertEqual(list(folder.iterdir()), [output] if existing else [])
                                if existing:
                                    self.assertEqual(output.read_bytes(), keys)
            finally:
                server.shutdown()
                thread.join()
        expected_requests = len(contents) * 2 + len(failures) * 2
        self.assertEqual(requests, ['identity'] * expected_requests)


class FixtureToolPaths(unittest.TestCase):
    def test_fakes_win_and_missing_tools_keep_runtime_and_independent_multicall_commands(self):
        with tempfile.TemporaryDirectory(prefix='fixture-tool-path-') as temporary:
            root = Path(temporary)
            runtime = root / 'runtime'; runtime.mkdir()
            fake = root / 'fake'; fake.mkdir()
            (runtime / 'sh').symlink_to(shutil.which('sh'))
            for name in ('node-core', 'python-core', 'rustup'):
                executable = runtime / name
                executable.write_text('#!/bin/sh\nprintf runtime\n')
                executable.chmod(0o755)
            for name, target in (('node', 'node-core'), ('nodejs', 'node-core'),
                                 ('python3.13', 'python-core'), ('python3', 'python-core'),
                                 ('python', 'python-core'), ('cargo', 'rustup'), ('rustc', 'rustup')):
                (runtime / name).symlink_to(target)
            node = fake / 'node'
            node.write_text('#!/bin/sh\nprintf fake\n'); node.chmod(0o755)
            unavailable = {runtime / 'protected': PermissionError, runtime / 'vanished': FileNotFoundError}
            for path in unavailable:
                path.write_text('#!/bin/sh\nexit 0\n'); path.chmod(0o755)
            is_file = Path.is_file
            def available(path, *args, **kwargs):
                if path in unavailable:
                    raise unavailable[path]('unavailable fixture entry')
                return is_file(path, *args, **kwargs)
            with patch.dict(os.environ, {'PATH': str(runtime)}):
                positive = child_env(home=root / 'home', PATH=fixture_path(fake))
                self.assertEqual(subprocess.check_output(['node'], env=positive, text=True), 'fake')
                node.unlink()
                absent = ('node', 'python', 'python3', 'python3.13', 'cargo')
                with patch.object(Path, 'is_file', available):
                    negative = child_env(home=root / 'home', PATH=fixture_path(fake, exclude=absent))
                for name in (*absent, 'nodejs', 'protected', 'vanished'):
                    self.assertIsNone(shutil.which(name, path=negative['PATH']), name)
                for name in ('rustup', 'rustc'):
                    self.assertEqual(subprocess.check_output([name], env=negative, text=True), 'runtime')
                self.assertEqual(subprocess.check_output(['sh', '-c', 'printf runtime'],
                                                        env=negative, text=True), 'runtime')

    def test_fixture_groups_include_every_declared_rung_without_bootstrapping_providers(self):
        manifest = json.loads((ROOT / 'sdlc/tools.json').read_text())
        manifest['tools']['system']['groups'].append('fixture-extra-group')
        with tempfile.TemporaryDirectory(prefix='fixture-tool-groups-') as temporary:
            root = Path(temporary)
            (root / 'sdlc').mkdir()
            (root / 'sdlc/tools.json').write_text(json.dumps(manifest))
            groups = set(fixture_tool_groups(root).split(','))
        expected = {group for tool in manifest['tools'].values() for group in tool['groups']}
        self.assertTrue(expected <= groups)
        self.assertIn('lint', groups)


class ToolResolution(unittest.TestCase):
    def test_reports_every_missing_tool_before_starting_the_command(self):
        self.check(['thinkthen-absent-first', 'thinkthen-absent-second'], False)

    def test_ignores_an_inherited_tool_path(self):
        self.check(['thinkthen-inherited-tool'], False, inherited=True)

    def test_starts_the_command_after_system_tools_resolve(self):
        self.check(['sh'], True)

    def test_reports_a_missing_dated_rust_toolchain_before_starting_work(self):
        pin = json.loads((ROOT / 'sdlc/tools.json').read_text())['tools']['rust-nightly']
        self.check([], False, requirement={**pin, 'version': 'nightly-1900-01-01',
                                         'groups': ['test']}, diagnostic='selected')

    def test_reports_an_unavailable_cargo_subcommand_before_starting_work(self):
        self.check(['sh'], False, requirement={
            'commands': ['sh'], 'groups': ['test'], 'platforms': ['darwin', 'linux'],
            'probes': [{'argv': ['sh', '-c', 'exit 1'], 'contains': 'cargo fmt'}],
        }, diagnostic='selected')

    def test_reports_unprepared_advisories_before_starting_work(self):
        self.check(['sh'], False, requirement={
            'commands': ['sh'], 'groups': ['test'], 'platforms': ['darwin', 'linux'],
            'readiness': {'argv': ['sh', '-c', 'exit 1'], 'contains': 'advisories ok'},
        }, diagnostic='selected')

    @unittest.skipUnless(platform.system() == 'Darwin', 'macOS owns native Cargo resolution')
    def test_cargo_dispatch_ignores_ambient_plugins_and_aliases_but_shares_cache_locks(self):
        cargo = subprocess.check_output(['rustup', 'which', 'cargo'], text=True).strip()
        owned = ROOT / '.tools'
        owned.mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='resolver-cargo-', dir=owned) as folder:
            root = Path(folder)
            resolved = root / 'resolved'
            binary = resolved / 'bin'
            ambient = root / 'ambient'
            binary.mkdir(parents=True)
            (ambient / 'bin').mkdir(parents=True)
            (binary / 'cargo').symlink_to(cargo)
            plugins = ('fmt', 'clippy', 'nextest', 'deny', 'public-api')
            for name in plugins:
                for directory, output in ((binary, 'owned-'), (ambient / 'bin', 'ambient-')):
                    executable = directory / ('cargo-' + name)
                    executable.write_text('#!/bin/sh\nprintf "%s\\n" ' + output + name + '\n')
                    executable.chmod(0o755)
            (ambient / 'config.toml').write_text('[alias]\nnextest = "!echo ambient-config"\n')
            (ambient / 'credentials.toml').write_text('fixture credential, never forwarded\n')
            caches = ('registry', 'git', 'advisory-dbs')
            locks = ('.package-cache', '.package-cache-mutate', '.global-cache')
            (ambient / 'registry').mkdir()
            (ambient / '.package-cache').write_text('shared fixture\n')
            manifest = {'libraries': {}, 'databases': {}, 'python_packages': {}, 'tools': {
                'rust': {'commands': [], 'version': 'unused', 'groups': ['common'],
                         'platforms': ['darwin']},
                'plugins': {'commands': ['cargo', *['cargo-' + name for name in plugins]],
                            'groups': ['test'], 'platforms': ['darwin'],
                            'probes': [{'argv': ['cargo', name, '--version'],
                                        'contains': 'owned-' + name} for name in plugins]}}}
            source = root / 'tools.json'
            source.write_text(json.dumps(manifest))
            env = {'HOME': str(root), 'CARGO_HOME': str(ambient), 'PATH': '/usr/bin:/bin',
                   'CARGO_ALIAS_NEXTEST': '!echo ambient-env'}
            command = '; '.join('cargo ' + name + ' --version' for name in plugins)
            argv = [sys.executable, str(SCRIPT), '--check', '--manifest', str(source),
                    '--root', str(resolved), 'test', '--', '/bin/sh', '-ec', command]
            children = [subprocess.Popen(argv, env=env, text=True, stdout=subprocess.PIPE,
                                        stderr=subprocess.PIPE) for _ in range(2)]
            for child in children:
                stdout, stderr = child.communicate()
                self.assertEqual(child.returncode, 0, stderr)
                self.assertEqual(stdout, ''.join('owned-' + name + '\n' for name in plugins))
            runtime = resolved / 'cargo-runtime'
            for name in (*caches, *locks):
                self.assertTrue((runtime / name).samefile(ambient / name), name)
            self.assertEqual((ambient / '.package-cache').read_text(), 'shared fixture\n')
            for name in ('bin', 'config', 'config.toml', 'credentials', 'credentials.toml'):
                self.assertFalse((runtime / name).exists(), name)

    @unittest.skipUnless(platform.system() == 'Darwin' and
                         'test' in os.environ.get('THINKTHEN_RESOLVED_TOOLS', '').split(','),
                         'requires the prepared native macOS test environment')
    def test_refuses_a_hidden_manifest_tool_before_starting_work(self):
        tool = Path(shutil.which('jq')).resolve()
        self.assertIn(ROOT / '.tools', tool.parents)
        hidden = tool.with_name(tool.name + '.hidden')
        tool.rename(hidden)
        try:
            result = subprocess.run([sys.executable, str(SCRIPT), '--check', 'test', '--',
                                     '/bin/sh', '-c', 'echo command-started'],
                                    text=True, capture_output=True)
        finally:
            hidden.rename(tool)
        self.assertEqual(result.returncode, 2, result.stderr)
        self.assertIn('jq', result.stderr)
        self.assertNotIn('command-started', result.stdout)

    @unittest.skipUnless(platform.system() == 'Darwin', 'macOS provisions npm providers')
    def test_npm_provider_can_start_its_owned_installer_child(self):
        owned = ROOT / '.tools'
        owned.mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='resolver-npm-', dir=owned) as folder:
            root = Path(folder)
            resolved = root / 'resolved'
            binary = resolved / 'bin'
            binary.mkdir(parents=True)
            for name in ('python3', 'fixture-prettier'):
                executable = binary / name
                executable.write_text('#!/bin/sh\nexit 0\n')
                executable.chmod(0o755)
            installer = resolved / 'bootstrap/mise/bin/mise'
            installer.parent.mkdir(parents=True)
            path = json.dumps({'PATH': str(binary) + ':/usr/bin:/bin'})
            installer.write_text('#!/bin/sh\nset -eu\ncase "$1" in\n'
                                 'install) mise fixture-child ;;\n'
                                 'fixture-child) printf installed > "$HOME/installer-child" ;;\n'
                                 'env) printf "%s\\n" ' + shlex.quote(path) + ' ;;\nesac\n')
            installer.chmod(0o755)
            manifest = {'libraries': {}, 'databases': {}, 'python_packages': {},
                        'tools': {'rust': {'commands': [], 'version': 'unused',
                                           'groups': ['common'], 'platforms': ['darwin']},
                                  'prettier': {'commands': ['fixture-prettier'], 'version': 'fixture',
                                               'provider': 'npm:prettier', 'groups': ['test'],
                                               'platforms': ['darwin']}}}
            source = root / 'tools.json'
            source.write_text(json.dumps(manifest))
            result = subprocess.run([sys.executable, str(SCRIPT), '--manifest', str(source),
                                     '--root', str(resolved), 'test', '--',
                                     '/bin/sh', '-c', 'echo command-started'],
                                    env={'HOME': str(root), 'PATH': '/usr/bin:/bin'},
                                    text=True, capture_output=True)
            self.assertEqual(result.returncode, 0, result.stderr)
            self.assertEqual((root / 'installer-child').read_text(), 'installed')
            self.assertEqual(result.stdout, 'command-started\n')

    def check(self, commands, succeeds, inherited=False, requirement=None, diagnostic=None):
        owned = ROOT / '.tools'
        owned.mkdir(exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='resolver-test-', dir=owned) as folder:
            root = Path(folder)
            fake = root / 'inherited'
            fake.mkdir()
            tool = fake / 'thinkthen-inherited-tool'
            tool.write_text('#!/bin/sh\nexit 0\n')
            tool.chmod(0o755)
            manifest = {
                'libraries': {}, 'databases': {}, 'linux_prefix': str(root),
                'tools': {
                    'rust': {'commands': [], 'version': 'unused', 'groups': ['common'],
                             'platforms': ['darwin', 'linux']},
                    'selected': requirement or {'commands': commands, 'groups': ['test'],
                                                'platforms': ['darwin', 'linux']},
                },
            }
            source = root / 'tools.json'
            source.write_text(json.dumps(manifest))
            env = {'HOME': str(root), 'PATH': str(fake) if inherited else '/usr/bin:/bin'}
            result = subprocess.run([sys.executable, str(SCRIPT), '--check', '--manifest', str(source),
                                     '--root', str(root / 'resolved'), 'test', '--',
                                     '/bin/sh', '-c', 'echo command-started'],
                                    env=env, text=True, capture_output=True)
            self.assertEqual(result.returncode, 0 if succeeds else 2, result.stderr)
            self.assertEqual(result.stdout, 'command-started\n' if succeeds else '')
            if not succeeds:
                self.assertIn('missing tools:', result.stderr)
                for command in commands:
                    self.assertIn(command, result.stderr)
                if diagnostic:
                    self.assertIn(diagnostic, result.stderr)



class LinuxSourceIdentity(unittest.TestCase):
    def test_reused_clone_discards_source_edits_and_keeps_build_output(self):
        with tempfile.TemporaryDirectory(prefix='linux-source-') as folder:
            root = Path(folder)
            env = child_env(home=root / 'home', PATH='/usr/bin:/bin')
            for relative in ('rust-toolchain.toml', 'sdlc/scripts/linux-check', 'sdlc/tools.json',
                             'sdlc/linux/Dockerfile', 'sdlc/linux/install.py',
                             'sdlc/scripts/tools.py', 'databases/duckdb/tools/version.env',
                             'conformance/children/children.py'):
                destination = root / relative
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(ROOT / relative, destination)
            (root / '.gitignore').write_text('.linux-*/\ntarget/\n')
            (root / 'source.txt').write_text('committed\n')
            git = ['git', '-C', str(root)]
            subprocess.run([*git, 'init', '-q'], env=env, check=True)
            subprocess.run([*git, 'add', '.'], env=env, check=True)
            subprocess.run([*git, '-c', 'user.name=Test', '-c', 'user.email=test@example.invalid',
                            'commit', '-qm', 'Create source fixture'], env=env, check=True)
            binary = root / 'fake-bin'
            binary.mkdir()
            docker = binary / 'docker'
            calls = root / 'docker-calls.jsonl'
            docker.write_text('#!' + sys.executable + '\nimport json,sys\n'
                              + 'with open(' + repr(str(calls)) + ', "a") as log:\n'
                              + '    log.write(json.dumps(sys.argv[1:]) + "\\n")\n'
                              + 'if sys.argv[1] == "image": print("sha256:test")\n')
            docker.chmod(0o755)
            isolated = root / 'isolated-bin'
            isolated.mkdir()
            (isolated / 'docker').write_text('#!/bin/sh\nexit 97\n')
            (isolated / 'docker').chmod(0o755)
            env['PATH'] = str(isolated) + os.pathsep + env['PATH']
            # The host Docker executable remains callable after tool PATH isolation.
            command = [sys.executable, str(root / 'sdlc/scripts/linux-check'),
                       '--docker', str(docker), 'test']
            first = subprocess.run(command, env=env, capture_output=True, text=True)
            self.assertEqual(first.returncode, 0, first.stderr)
            source = next(root.glob('.linux-*/src')).resolve()
            run = next(json.loads(line) for line in calls.read_text().splitlines()
                       if json.loads(line)[0] == 'run')
            fixture_mounts = [run[index + 1] for index, value in enumerate(run)
                              if value == '--tmpfs']
            owner = ':uid=' + str(os.getuid()) + ',gid=' + str(os.getgid()) + ',mode=0700,exec'
            self.assertEqual(fixture_mounts, [str(source / relative) + owner
                             for relative in ('target/tmp', 'target/library-only/tmp')])
            self.assertIn(str(source.parent) + ':' + str(source.parent), run)
            (source / 'source.txt').write_text('left by previous check\n')
            (source / 'stray.txt').write_text('untracked source\n')
            (source / 'target').mkdir(exist_ok=True)
            (source / 'target/cached').write_text('build output\n')
            second = subprocess.run(command, env=env, capture_output=True, text=True)
            self.assertEqual(second.returncode, 0, second.stderr)
            self.assertEqual((source / 'source.txt').read_text(), 'committed\n')
            self.assertFalse((source / 'stray.txt').exists())
            self.assertEqual((source / 'target/cached').read_text(), 'build output\n')


if __name__ == '__main__':
    unittest.main()
