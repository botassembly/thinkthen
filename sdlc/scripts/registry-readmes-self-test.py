#!/usr/bin/env python3
"""Exercise the package-text release boundary with positive and negative archives."""
import importlib.util
import ctypes
import signal
import io
import os
import sys
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
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('registry_readmes', Path(__file__).with_name('registry-readmes.py'))
readmes = importlib.util.module_from_spec(spec)
spec.loader.exec_module(readmes)


def archive(path, files):
    with tarfile.open(path, 'w:gz') as package:
        for name, data in files.items():
            entry = tarfile.TarInfo(name)
            entry.size = len(data)
            if name.endswith('/configure'): entry.mode = 0o755
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
            with self.assertRaisesRegex(readmes.Refusal, 'shared-opening'):
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

    def test_r_replay_installs_package_root_with_vendored_readmes(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch); packages = tree / 'packages'; packages.mkdir()
            archive(packages / 'thinkthen_0.2.0.tar.gz', {
                'thinkthen/README.md': readmes.source('r').read_bytes(),
                'thinkthen/src/rust/vendor/dependency/README.md': b'Vendored dependency',
                'thinkthen/configure': b'#!/bin/sh\nexit 0\n',
                'thinkthen/DESCRIPTION': (readmes.REPO / 'libraries/r/thinkthen/DESCRIPTION').read_bytes()})
            installed = readmes.unpack_example_package('r', packages, tree / 'installed')
            self.assertTrue((installed / 'DESCRIPTION').is_file())
            self.assertTrue(os.access(installed / 'configure', os.X_OK))
            self.assertEqual(installed, tree / 'installed/thinkthen')

    def test_duckdb_replay_loads_the_installed_source_member(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch); packages = tree / 'packages'; packages.mkdir()
            archive(packages / 'thinkthen-duckdb-0.2.0-x86_64-unknown-linux-gnu.tar.gz', {
                'README.md': readmes.source('duckdb').read_bytes(),
                'd4355d01c5/linux_amd64/thinkthen.duckdb_extension': b'preview',
                'v1.5.6/linux_amd64/thinkthen.duckdb_extension': b'stable'})
            work = tree / 'work'; work.mkdir()
            def run(command, cwd, env, input, timeout):
                if '-c' in command:
                    return subprocess.CompletedProcess(command, 0, '069cc9f9b5\n', '')
                self.assertIn('-bail', command)
                self.assertIn('/v1.5.6/linux_amd64/thinkthen.duckdb_extension', input)
                self.assertNotIn('/d4355d01c5/', input)
                return subprocess.CompletedProcess(command, 0, '', '')
            with patch.object(readmes, 'run_process', side_effect=run):
                readmes.run_example('duckdb', packages, work)

    def test_all_usage_blocks_are_selected_in_order(self):
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            page = readmes.source('r', root)
            page.parent.mkdir(parents=True)
            page.write_text('```r\ninstall.packages("thinkthen")\n```\n'
                            '```r\ntt_decide("Q?", "one")\n```\n'
                            '```r\ntt_choose("Q?", "two")\n```\n')
            samples = readmes.examples('r', root)
            self.assertEqual(len(samples), 2)
            self.assertIn('tt_decide(', samples[0][1])
            self.assertIn('tt_choose(', samples[1][1])

    def test_replay_collects_every_broken_usage_block(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch)
            root = tree / 'root'; (root / 'site/recordings').mkdir(parents=True)
            shutil.copyfile(readmes.REPO / 'rust-toolchain.toml', root / 'rust-toolchain.toml')
            shutil.copyfile(readmes.REPO / 'site/recordings/thinkthen.jsonl', root / 'site/recordings/thinkthen.jsonl')
            (root / 'README.md').write_text('```sh\nthinkthen decide fail\n```\n'
                                          '```sh\nthinkthen choose fail\n```\n')
            packages = tree / 'packages'; packages.mkdir()
            package = packages / ('thinkthen-' + readmes.version() + '-x86_64-unknown-linux-musl.tar.gz')
            archive(package, {'README.md': b'Wrong public opening',
                              'thinkthen': b'#!/bin/sh\n[ "$2" != fail ] || exit 77\nprintf "fixture\\n"\n'})
            with patch.object(readmes, 'published_families', return_value=('command',)):
                with self.assertRaisesRegex(readmes.Refusal, 'example-did-not-run: command') as rejected:
                    readmes.check_examples(packages, root)
                self.assertEqual(str(rejected.exception).count('example-did-not-run: command'), 2)
                self.assertEqual(str(rejected.exception).count('command packed text: shared-opening'), 2)
                self.assertIn('command block 1:', str(rejected.exception))
                self.assertIn('command block 2:', str(rejected.exception))

    def test_preserved_root_checks_only_opening_and_versions(self):
        for family in ('command', 'packagist'):
            with self.subTest(family=family), tempfile.TemporaryDirectory() as scratch:
                root = Path(scratch)
                (root / 'crates/thinkthen').mkdir(parents=True)
                shutil.copyfile(readmes.REPO / 'crates/thinkthen/Cargo.toml',
                                root / 'crates/thinkthen/Cargo.toml')
                approved = readmes.source(family).read_text()
                self.assertGreaterEqual(len(approved.splitlines()), 40)
                (root / 'README.md').write_text(approved)
                readmes.check_readme(family, approved.encode(), root)
                body = approved + '\nADR 0047; ticket 0578; sdlc/planning/a.md\n'
                readmes.check_readme(family, body.encode(), root)
                for text, cause in [(approved.replace(readmes.OPENING, 'Wrong pitch'), 'shared-opening'),
                                    (approved + '\nThinkThen release 0.1.2\n', 'stale-version')]:
                    (root / 'README.md').write_text(text)
                    with self.assertRaisesRegex(readmes.Refusal, cause):
                        readmes.check_readme(family, text.encode(), root)

    def test_package_pages_keep_whole_body_rules(self):
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            page = readmes.source('python', root)
            page.parent.mkdir(parents=True)
            (root / 'crates/thinkthen').mkdir(parents=True)
            shutil.copyfile(readmes.REPO / 'crates/thinkthen/Cargo.toml',
                            root / 'crates/thinkthen/Cargo.toml')
            for plant, cause in [('ADR 0047', 'adr'), ('ticket 0578', 'ticket'),
                                 ('sdlc/planning/a.md', 'internal-path'), ('\n' * 40, 'readme-needs-under-40-lines')]:
                text = '# ThinkThen\n\n' + readmes.OPENING + '\n\n' + plant + '\n'
                page.write_text(text)
                with self.assertRaisesRegex(readmes.Refusal, cause):
                    readmes.check_readme('python', text.encode(), root)

    def test_composer_replays_root_cli_without_a_php_example(self):
        self.assertEqual(readmes.examples('packagist'), readmes.examples('command'))
        samples = readmes.examples('command')
        self.assertEqual(len(samples), 2)
        self.assertIn('< message.txt', samples[0][1])
        self.assertIn('thinkthen-first-run', samples[1][1])

    def test_preserved_root_replay_uses_real_demo_and_first_run_fixtures(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch); packages = tree / 'packages'; packages.mkdir()
            demo = readmes.REPO / 'demos/01-refund-gate'
            first_run = readmes.REPO / 'demos/27-test-with-no-network'
            script = ('#!' + sys.executable + '\nimport os, pathlib, sys\n'
                      'text = sys.stdin.read()\n'
                      'if "--replay" in sys.argv:\n'
                      '    assert text == ' + repr((first_run / 'report.txt').read_text()) + '\n'
                      '    assert pathlib.Path("recording/thinkthen.jsonl").is_file()\n'
                      'else:\n'
                      '    assert text == ' + repr((demo / 'message.txt').read_text()) + '\n'
                      '    assert pathlib.Path(os.environ["THINKTHEN_CACHE"], "thinkthen.jsonl").read_bytes() == '
                      + repr((demo / 'recording/thinkthen.jsonl').read_bytes()) + '\n'
                      'print("true")\n')
            archive(packages / ('thinkthen-' + readmes.version() + '-x86_64-unknown-linux-musl.tar.gz'),
                    {'README.md': readmes.source('command').read_bytes(), 'thinkthen': script.encode()})
            archive(packages / 'thinkthen-first-run.tar.gz', {
                'thinkthen-first-run/report.txt': (first_run / 'report.txt').read_bytes(),
                'thinkthen-first-run/recording/thinkthen.jsonl':
                    (first_run / 'recording/thinkthen.jsonl').read_bytes()})
            for family in ('command', 'packagist'):
                for index in range(len(readmes.examples(family))):
                    with self.subTest(family=family, index=index):
                        work = tree / (family + str(index)); work.mkdir()
                        readmes.run_example(family, packages, work, sample_index=index)

    def test_release_replays_every_public_package_on_a_prepared_host(self):
        jobs = yaml.safe_load((readmes.REPO / '.github/workflows/release.yml').read_text())['jobs']
        steps = jobs['smoke']['steps']
        replay = next(step for step in steps if 'public-examples' in step.get('run', ''))
        self.assertEqual(replay['if'], "matrix.target == 'x86_64-unknown-linux-gnu' || runner.os == 'macOS'")
        self.assertLess(next(i for i, step in enumerate(steps) if 'host-setup' in step.get('run', '')), steps.index(replay))
        script = (readmes.REPO / 'sdlc/scripts/release-workflow').read_text()
        routing = script.split('\npublic-examples)\n')[1].split('\ncrate-smoke)')[0]
        families = set(readmes.re.findall(r'--family ([a-z-]+)', routing))
        self.assertEqual(families, set(readmes.published_families()))
        self.assertNotIn('registry-readmes.py examples', script.split('\ndraft)\n')[1].split('\n\t;;')[0])
        self.assertIn('smoke', jobs['draft']['needs'])
        for name, path in [('r-package', 'r-dist'), ('crate-package', 'crate-dist')]:
            self.assertTrue(any(step.get('with', {}).get('name') == name
                                and step['with']['path'] == path for step in steps))
        # Execute the actual workflow staging step with the runner replaced at its CLI boundary.
        with tempfile.TemporaryDirectory() as scratch:
            root = Path(scratch)
            for name in ('r-dist', 'crate-dist', 'smoke-files', 'sdlc/scripts'):
                (root / name).mkdir(parents=True)
            (root / 'r-dist/thinkthen.tar.gz').write_text('R package')
            (root / 'crate-dist/thinkthen.crate').write_text('Rust package')
            runner = root / 'sdlc/scripts/release-workflow'
            runner.write_text('#!/bin/sh\nset -eu\n[ "$1" = public-examples ]\n[ -f "$3/thinkthen.crate" ]\nif [ "$2" = x86_64-unknown-linux-gnu ]; then [ -f "$3/thinkthen.tar.gz" ]; fi\n')
            runner.chmod(0o755)
            for target in ('x86_64-unknown-linux-gnu', 'aarch64-apple-darwin'):
                subprocess.run(['/bin/sh', '-eu', '-c', replay['run']], cwd=root,
                               env={'PATH': '/usr/bin:/bin', 'TARGET': target}, check=True)

    @unittest.skipUnless(sys.platform in ('linux', 'darwin'), 'compiler replay requires POSIX process groups')
    def test_compilers_lock_without_flock_and_replay_uses_prepared_cache(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch)
            root = tree / 'root'
            (root / 'site/recordings').mkdir(parents=True)
            shutil.copyfile(readmes.REPO / 'rust-toolchain.toml', root / 'rust-toolchain.toml')
            shutil.copyfile(readmes.REPO / 'site/recordings/thinkthen.jsonl', root / 'site/recordings/thinkthen.jsonl')
            packages = tree / 'packages'; packages.mkdir()
            tools = tree / 'tools'; tools.mkdir()
            cache = tree / 'prepared-pub-cache'; cache.mkdir()
            lock = tree / 'compiler.lock'
            for family, language, compiler in [('objective-c', 'objc', 'swift'), ('dart', 'dart', 'dart')]:
                page = readmes.source(family, root)
                page.parent.mkdir(parents=True)
                page.write_text('```' + language + '\ndecide(); choose();\n```\n```text\nfixture\n```\n')
                files = {'README.md': readmes.source(family).read_bytes()}
                if family == 'dart':
                    files['pubspec.yaml'] = (readmes.REPO / 'libraries/dart/pubspec.yaml').read_bytes()
                archive(packages / ('thinkthen-' + family + '-' + readmes.version() + '-x86_64-unknown-linux-gnu.tar.gz'), files)
                tool = tools / compiler
                tool.write_text('#!' + sys.executable + '\n' +
                    'import fcntl, os, pathlib, sys\n' +
                    'with open(' + repr(str(lock)) + ', "a") as lock:\n' +
                    '    try: fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)\n' +
                    '    except BlockingIOError: pass\n' +
                    '    else: sys.exit("compiler ran without lock")\n' +
                    'assert os.environ["CARGO_BUILD_JOBS"] == "2"\n' +
                    ('assert os.environ["PUB_CACHE"] == ' + repr(str(cache)) + '\n' if family == 'dart' else '') +
                    'out = pathlib.Path(".build/debug/RegistryExample") if sys.argv[1] == "build" else pathlib.Path(sys.argv[-1]) if sys.argv[1] == "compile" else None\n' +
                    'if out:\n' +
                    '    out.parent.mkdir(parents=True, exist_ok=True)\n' +
                    '    out.write_text(' + repr('#!' + sys.executable + '\nimport fcntl\nwith open(' + repr(str(lock)) + ', "a") as lock: fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)\nprint("fixture")\n') + ')\n' +
                    '    out.chmod(0o755)\n')
                tool.chmod(0o755)
                work = tree / family; work.mkdir()
                with self.subTest(family=family), patch.dict(os.environ, {'PATH': str(tools), 'PUB_CACHE': str(cache), 'THINKTHEN_HEAVY_LOCK': str(lock)}, clear=True):
                    readmes.run_example(family, packages, work, root)

    def test_release_refuses_when_examples_cannot_run(self):
        with tempfile.TemporaryDirectory() as scratch:
            with self.assertRaisesRegex(readmes.Refusal, 'missing-or-ambiguous-example-package: command') as error:
                readmes.check_examples(Path(scratch))
            self.assertIn('missing-or-ambiguous-example-package: typescript', str(error.exception))
            self.assertIn('missing-or-ambiguous-example-package: duckdb', str(error.exception))

    @unittest.skipUnless(sys.platform in ('linux', 'darwin'), 'compiler replay requires POSIX process groups')
    def test_timed_out_compiler_and_worker_exit_before_replay_releases_build(self):
        with tempfile.TemporaryDirectory() as scratch:
            tree = Path(scratch)
            root = tree / 'root'
            (root / 'site/recordings').mkdir(parents=True)
            shutil.copyfile(readmes.REPO / 'rust-toolchain.toml', root / 'rust-toolchain.toml')
            shutil.copyfile(readmes.REPO / 'site/recordings/thinkthen.jsonl', root / 'site/recordings/thinkthen.jsonl')
            page = readmes.source('swift', root)
            page.parent.mkdir(parents=True)
            page.write_text('```swift\ndecide(); choose();\n```\n```text\nfixture\n```\n')
            packages = tree / 'packages'; packages.mkdir()
            archive(packages / ('thinkthen-swift-' + readmes.version() + '-x86_64-unknown-linux-gnu.tar.gz'),
                    {'README.md': readmes.source('swift').read_bytes()})
            tools = tree / 'tools'; tools.mkdir()
            marker = tree / 'compiler.pid'
            compiler = tools / 'swift'
            compiler.write_text('#!' + sys.executable + '\n' +
                'import os, pathlib, signal, subprocess, sys, time\n' +
                'marker = pathlib.Path(' + repr(str(marker)) + ')\n' +
                'if pathlib.Path.cwd().name.startswith("first"):\n' +
                '    hard = pathlib.Path.cwd().name != "first"\n' +
                '    child = subprocess.Popen([sys.executable, "-c", ("import os, signal, time; " + ("os.setsid(); " if sys.platform == "linux" and pathlib.Path.cwd().name == "first-detached" else "") + "signal.signal(signal.SIGTERM, signal.SIG_IGN); time.sleep(30)") if hard else "import time; time.sleep(30)"])\n' +
                '    marker.with_suffix(".worker").write_text(str(child.pid))\n' +
                '    def stop(signum, frame): raise SystemExit(77)\n' +
                '    signal.signal(signal.SIGTERM, signal.SIG_IGN if hard else stop)\n' +
                '    marker.write_text(str(os.getpid()))\n' +
                '    try: time.sleep(2)\n' +
                '    finally:\n' +
                '        if child: child.terminate(); child.wait()\n' +
                '    sys.exit(77)\n' +
                'for pid_file in (marker, marker.with_suffix(".worker")):\n' +
                '    try: os.kill(int(pid_file.read_text()), 0)\n' +
                '    except ProcessLookupError: pass\n' +
                '    else: sys.exit("another compiler is still alive")\n' +
                'out = pathlib.Path(".build/debug/RegistryExample")\n' +
                'out.parent.mkdir(parents=True)\n' +
                'out.write_text(' + repr('#!' + sys.executable + '\nprint("fixture")\n') + ')\n' +
                'out.chmod(0o755)\n')
            compiler.chmod(0o755)
            popen = subprocess.Popen
            communicate = subprocess.Popen.communicate
            def bounded_popen(command, *args, **kwargs):
                if len(command) > 1 and sys.platform == 'linux' and 'build-admission.py' in str(command[1]):
                    command = list(command)
                    command[command.index('--timeout') + 1] = '.4'
                return popen(command, *args, **kwargs)
            def bounded_communicate(process, input=None, timeout=None):
                return communicate(process, input, .4 if timeout == 300 else timeout)
            cases = [(str(tools), 'first'), (str(tools) + os.pathsep + '/usr/bin:/bin', 'first-hard')]
            if sys.platform == 'linux':
                cases.append((str(tools), 'first-detached'))
            for search_path, first_name in cases:
                with self.subTest(path=search_path), patch.dict(os.environ, {
                    'PATH': search_path, 'THINKTHEN_HEAVY_LOCK': str(tree / 'compiler.lock')}, clear=True), \
                        patch.object(subprocess, 'Popen', bounded_popen), \
                        patch.object(popen, 'communicate', bounded_communicate):
                    marker.unlink(missing_ok=True)
                    first = tree / first_name; first.mkdir(exist_ok=True)
                    second = tree / 'second'; second.mkdir(exist_ok=True)
                    failure = readmes.Refusal if sys.platform == 'linux' else subprocess.TimeoutExpired
                    with self.assertRaises(failure) as error:
                        readmes.run_example('swift', packages, first, root)
                    if sys.platform == 'linux':
                        self.assertIn('build timed out', str(error.exception))
                    self.assertTrue(marker.exists(), 'first compiler did not start')
                    self.assertNotEqual(marker.read_text(), marker.with_suffix('.worker').read_text())
                    readmes.run_example('swift', packages, second, root)
                    for pid_file in (marker, marker.with_suffix('.worker')):
                        with self.assertRaises(ProcessLookupError):
                            os.kill(int(pid_file.read_text()), 0)
                shutil.rmtree(first)
                shutil.rmtree(second)

    @unittest.skipUnless(sys.platform == 'linux', 'nested workers require Linux subreaping')
    def test_build_reaps_nested_workers_before_returning_its_permit(self):
        # Own survivors only so a failing regression can clean up its fixture.
        libc = ctypes.CDLL(None, use_errno=True)
        previous = ctypes.c_int()
        self.assertEqual(libc.prctl(37, ctypes.byref(previous), 0, 0, 0), 0)
        self.assertEqual(libc.prctl(36, 1, 0, 0, 0), 0)
        pids = []
        try:
            with tempfile.TemporaryDirectory() as scratch:
                tree = Path(scratch)
                marker = tree / 'workers'
                compiler = tree / 'compiler.py'
                compiler.write_text(
                    'import os, pathlib, signal, subprocess, sys, time\n'
                    'signal.signal(signal.SIGTERM, signal.SIG_IGN)\n'
                    'os.setsid() if sys.argv[1] != "35" else None\n'
                    'with pathlib.Path(sys.argv[2]).open("a") as marker: marker.write(str(os.getpid()) + "\\n")\n'
                    'if int(sys.argv[1]):\n'
                    '    subprocess.Popen([sys.executable, __file__, str(int(sys.argv[1]) - 1), sys.argv[2]], '
                    'stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)\n'
                    'time.sleep(30)\n')
                lock = tree / 'build.lock'
                popen = subprocess.Popen
                communicate = subprocess.Popen.communicate
                def bounded_popen(command, *args, **kwargs):
                    if 'build-admission.py' in str(command[1]):
                        command = list(command)
                        command[command.index('--timeout') + 1] = '2'
                    return popen(command, *args, **kwargs)
                def bounded_communicate(process, input=None, timeout=None):
                    return communicate(process, input, 2 if timeout == 300 else timeout)
                with patch.dict(os.environ, {'THINKTHEN_HEAVY_LOCK': str(lock)}, clear=True), \
                        patch.object(subprocess, 'Popen', bounded_popen), \
                        patch.object(popen, 'communicate', bounded_communicate):
                    try:
                        done = readmes.run_build([sys.executable, str(compiler), '35', str(marker)],
                                                tree, {'PATH': '/usr/bin:/bin'}, None)
                    except subprocess.TimeoutExpired:
                        done = None
                pids = [int(pid) for pid in marker.read_text().splitlines()]
                self.assertEqual(len(pids), 36, 'compiler did not launch all nested workers')
                self.assertEqual(len(set(pids)), 36, 'compiler and worker PIDs must differ')
                remaining = [pid for pid in pids if Path('/proc', str(pid)).exists()]
                # A new process must acquire every permit and the legacy lock.
                probe = subprocess.run([sys.executable, '-c',
                    'import fcntl, sys; '
                    'lock = open(sys.argv[1], "a+b"); '
                    'fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB); '
                    '[fcntl.lockf(lock, fcntl.LOCK_EX | fcntl.LOCK_NB, 1, slot) for slot in range(4)]',
                    str(lock)], capture_output=True, text=True)
                self.assertEqual(probe.returncode, 0, probe.stderr)
                self.assertEqual(remaining, [], 'returned with live or zombie descendants')
                self.assertIsNotNone(done, 'outer timeout interrupted the supervisor')
                self.assertEqual(done.returncode, 124)
                self.assertIn('build timed out', done.stderr)
        finally:
            for pid in pids:
                try:
                    os.kill(pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            while True:
                try:
                    os.waitpid(-1, 0)
                except ChildProcessError:
                    break
            libc.prctl(36, previous.value, 0, 0, 0)

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
