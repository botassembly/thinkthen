#!/usr/bin/env python3
"""Exercise the package-text release boundary with positive and negative archives."""
import importlib.util
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
