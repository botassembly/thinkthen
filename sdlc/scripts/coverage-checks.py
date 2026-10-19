"""Focused regressions for coverage process ownership and missing profiles."""
import json
import os
import platform
import plistlib
from pathlib import Path
import runpy
import signal
import shutil
import time
import sys
import tempfile
import tarfile
import subprocess
import unittest
from unittest.mock import patch

import coverage_installed_managed
import coverage_installed_c_hosts

DRIVER = runpy.run_path(str(Path(__file__).with_name('coverage')))


class PlatformCommandEvidence(unittest.TestCase):
    def test_platform_commands_refuse_a_foreign_host_and_native_slice(self):
        command = [sys.executable, str(Path(__file__).with_name('coverage')), '--release', '--platform']
        foreign = 'apple' if sys.platform == 'linux' else 'linux'
        result = subprocess.run([*command, foreign], capture_output=True)
        self.assertEqual(result.returncode, 1)
        self.assertIn(b'requires its own host', result.stderr)
        result = subprocess.run([*command, foreign, '--native'], capture_output=True)
        self.assertEqual(result.returncode, 1)
        self.assertIn(b'cannot qualify', result.stderr)

    def test_join_requires_both_hosts_and_all_collections_on_the_current_commit(self):
        revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip()
        expected = set(DRIVER['NATIVE_COLLECTIONS']) | {name for helper in DRIVER['installed_helpers']() for name in helper.COLLECTIONS}
        with tempfile.TemporaryDirectory() as temporary:
            paths = [Path(temporary) / name for name in ('linux.json', 'apple.json')]
            payloads = [dict(source=revision, platform_collection=platform_name, unmeasured_collections=[],
                             totals=dict(mapped_nonblank=100, **{name: 50 for name in names}))
                        for platform_name, names in (('linux', expected - {'installed-objective-c'}),
                                                    ('apple', {'installed-swift', 'installed-objective-c'}))]
            command = [sys.executable, str(Path(__file__).with_name('coverage')), '--release', '--join', *map(str, paths)]
            for payload, path in zip(payloads, paths):
                path.write_text(json.dumps(payload))
            self.assertEqual(subprocess.run(command, capture_output=True).returncode, 0)
            for change in ('absent', 'source', 'missing', 'apple', 'pending', '77', 'linux'):
                with self.subTest(change=change):
                    values = json.loads(json.dumps(payloads))
                    if change == 'source':
                        values[1]['source'] = '0' * 40
                    elif change == 'missing':
                        values[1]['unmeasured_collections'] = ['installed-objective-c']
                    elif change in ('pending', '77'):
                        values[1]['totals']['installed-objective-c'] = change
                    elif change == 'apple':
                        del values[1]['totals']['installed-objective-c']
                    elif change == 'linux':
                        del values[0]['totals']['installed-c']
                    for value, path in zip(values, paths):
                        path.write_text(json.dumps(value))
                    if change == 'absent':
                        paths[1].unlink()
                    result = subprocess.run(command, capture_output=True)
                    self.assertEqual(result.returncode, 1, result.stderr.decode())



class RetainedLineEvidence(unittest.TestCase):
    def test_equal_totals_retain_distinct_lines_and_production_denominator(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            prefix = 'crates/thinkthen/src/'
            source = root / prefix / 'probe.rs'
            source.parent.mkdir(parents=True)
            source.write_text('first\n\nthird\nfourth\nfifth\ntest only\n')
            zero = source.with_name('zero.rs')
            zero.write_text('zero hit\n')
            test_only = source.with_name('test_only.rs')
            test_only.write_text('test mapping\n')
            production = f'SF:{source}\nDA:1,0\nDA:2,0\nDA:3,0\nDA:4,0\nDA:5,0\nSF:{zero}\nDA:1,0\n'
            results = []
            for index, (outside, unit) in enumerate(((1, 3), (3, 1))):
                run = root / f'run-{index}'
                run.mkdir()
                collections = {
                    'rust-integration': f'SF:{source}\nDA:{outside},1\nDA:2,1\nDA:6,1\nSF:{test_only}\nDA:1,1\n',
                    'source-unit': f'SF:{source}\nDA:{unit},1\n',
                    'installed-c': f'SF:{source}\nDA:4,1\n'}
                DRIVER['report'](root, run, f'lines-{index}', production, collections, 0)
                payload = json.loads((run / 'totals.json').read_text())
                records = root / 'sdlc/records/coverage'
                self.assertEqual(list(records.glob('*.json')), [])
                self.assertTrue(next(records.glob(f'*-lines-{index}-installed.md')).is_file())
                self.assertEqual(payload['line_sets'], {
                    prefix + 'probe.rs': {'mapped_nonblank': [1, 3, 4, 5],
                                         'rust-integration': [outside], 'source-unit': [unit], 'installed-c': [4]},
                    prefix + 'zero.rs': {'mapped_nonblank': [1],
                                        'rust-integration': [], 'source-unit': [], 'installed-c': []}})
                self.assertEqual(payload['totals'], {'mapped_nonblank': 5, 'rust-integration': 1,
                                                    'source-unit': 1, 'installed-c': 1,
                                                    'observed_unit_only': 1, 'unobserved': 2})
                results.append(payload)
            self.assertEqual(results[0]['totals'], results[1]['totals'])
            self.assertEqual(results[0]['files'], results[1]['files'])
            self.assertNotEqual(results[0]['line_sets'], results[1]['line_sets'])


class InstalledScriptInvocation(unittest.TestCase):
    @unittest.skipUnless(sys.platform == 'linux' and platform.machine() == 'x86_64',
                         'Ada routine gate requires Linux x86_64')
    def test_ada_collector_invokes_gate_with_clean_path(self):
        checkout = Path(__file__).resolve().parents[2]
        with tempfile.TemporaryDirectory() as temporary:
            run = Path(temporary)
            tools = run / 'tools'
            tools.mkdir()
            for name in ('gprbuild', 'gnatmake', 'gcc', 'python3', 'nm', 'node'):
                (tools / name).symlink_to('/bin/true')
            for name in ('uname', 'dirname', 'env', 'sh'):
                (tools / name).symlink_to(shutil.which(name))
            library = run / 'libthinkthen.so'
            library.touch()
            script_dir = run / 'ada'
            script_dir.mkdir()
            # Keep the real gate through its host and tool checks.
            source = (checkout / 'libraries/ada/check.sh').read_text()
            prefix = source.split('. "$REPO/sdlc/scripts/scratch.sh"')[0]
            script = script_dir / 'check.sh'
            script.write_text(prefix + 'exit 23\n')
            script.chmod(0o700)
            env = {'PATH': str(tools)}
            invoked = []
            def command(args, cwd, settings, log, **kwargs):
                if 'package_owned.py' in ' '.join(args):
                    Path(args[-1]).mkdir()
                if cwd == checkout / 'libraries/ada':
                    invoked.append(args)
                    result = subprocess.run(args, cwd=script_dir, env=env, capture_output=True)
                    self.assertEqual(result.returncode, 23, result.stderr.decode())
            with patch.object(coverage_installed_c_hosts.shutil, 'which', return_value='/bin/true'):
                collections = coverage_installed_c_hosts.prepare(dict(
                    root=checkout, run=run, env=env, command=command,
                    missing_tool=DRIVER['MissingTool'], c_shared=library,
                    c_target=run, initializer=run / 'profile.o'))
                next(c for c in collections if c['name'] == 'installed-ada')['runner'](run)
            self.assertEqual(len(invoked), 1)
            with tarfile.open(run / 'stage-c-hosts/cpp.tar.gz') as package:
                for source in (checkout / 'libraries/cpp/examples').rglob('*'):
                    if source.is_file():
                        member = './' + source.relative_to(checkout / 'libraries/cpp').as_posix()
                        self.assertEqual(package.extractfile(member).read(), source.read_bytes())


class ManagedToolAvailability(unittest.TestCase):
    def test_missing_tools_stop_collection_before_package_commands(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            header = root / 'libraries/c/include/thinkthen.h'
            header.parent.mkdir(parents=True)
            header.write_text('fixture header')
            native = root / 'libthinkthen.so'
            native.write_bytes(b'fixture native')
            tools = root / 'tools'
            tools.mkdir()
            for tool in ('node', 'bwrap', 'configured-dotnet', 'configured-dart',
                         'configured-flutter', 'configured-swift'):
                binary = tools / tool
                binary.write_text('#!/bin/sh\nexit 0\n')
                binary.chmod(0o700)
            env = {'HOME': str(root), 'PATH': str(tools),
                   'THINKTHEN_DOTNET': str(tools / 'configured-dotnet'),
                   'TT_DART': str(tools / 'configured-dart'), 'TT_FLUTTER': str(tools / 'configured-flutter'),
                   'THINKTHEN_SWIFT': str(tools / 'configured-swift')}
            cases = {'csharp': ('/usr/bin/bwrap', 'dotnet'), 'dart': ('node', 'dart'),
                     'flutter': ('node', '/usr/bin/xvfb-run', 'dart', 'flutter'),
                     'swift': ('node', 'bwrap', 'swift')}
            def command(*args, **kwargs):
                self.fail('missing tool reached a package command')
            real_which = coverage_installed_managed.shutil.which
            for language, requirements in cases.items():
                for index, absent in enumerate(requirements):
                    with self.subTest(language=language, missing=absent):
                        run = root / f'{language}-{index}'
                        run.mkdir()
                        def which(tool, path=None):
                            if tool == absent or Path(tool).name.removeprefix('configured-') == absent:
                                return None
                            if tool in ('/usr/bin/bwrap', '/usr/bin/xvfb-run'):
                                return tool
                            return real_which(tool, path=path)
                        with patch.dict(os.environ, env, clear=True), patch.object(
                                coverage_installed_managed.shutil, 'which', side_effect=which), patch.object(
                                coverage_installed_managed.sys, 'platform', 'linux'):
                            collections = coverage_installed_managed.prepare(dict(
                                root=root, run=run, env=env, command=command,
                                missing_tool=DRIVER['MissingTool'], c_shared=native))
                            collection = next(c for c in collections if c['name'] == 'installed-' + language)
                            with self.assertRaisesRegex(DRIVER['MissingTool'], absent):
                                collection['runner'](run)

    def test_apple_staging_supplies_the_real_package_and_consumer_inputs(self):
        checkout = Path(__file__).resolve().parents[2]
        with tempfile.TemporaryDirectory() as temporary:
            run = Path(temporary)
            native = run / 'target-c/debug/libthinkthen_c.a'
            native.parent.mkdir(parents=True)
            native.write_bytes(b'measured native archive')
            def command(args, cwd, env, log, **kwargs):
                # XCFramework assembly requires macOS; package validation below is real.
                framework = Path(args[args.index('-output') + 1])
                arch = platform.machine()
                identifier = 'macos-' + arch
                slice = framework / identifier
                slice.mkdir(parents=True)
                library = Path(args[args.index('-library') + 1])
                shutil.copy2(library, slice / library.name)
                shutil.copytree(args[args.index('-headers') + 1], slice / 'Headers')
                metadata = dict(CFBundlePackageType='XFWK', XCFrameworkFormatVersion='1.0',
                                AvailableLibraries=[dict(LibraryIdentifier=identifier,
                                    LibraryPath=library.name, HeadersPath='Headers',
                                    SupportedArchitectures=[arch], SupportedPlatform='macos')])
                (framework / 'Info.plist').write_bytes(plistlib.dumps(metadata))
            context = dict(root=checkout, run=run, env={}, command=command,
                           missing_tool=DRIVER['MissingTool'], c_target=native.parents[1])
            collections = coverage_installed_managed.prepare_apple(context)
            self.assertEqual({row['name'] for row in collections}, {'installed-swift', 'installed-objective-c'})
            for language in ('swift', 'objective-c'):
                with self.subTest(language=language):
                    package = run / ('unpacked-' + language)
                    package.mkdir()
                    with coverage_installed_managed.tarfile.open(context['apple_artifacts'][language]) as artifact:
                        artifact.extractall(package, filter='data')
                    self.assertEqual((package / 'README.md').read_bytes(),
                                     (checkout / 'libraries' / language / 'README-PUBLIC.md').read_bytes())
                    if language == 'swift':
                        result = subprocess.run(['swift', 'package', '--package-path', str(package), 'describe'],
                                                env={'PATH': os.environ['PATH'], 'HOME': str(run)},
                                                capture_output=True, timeout=60)
                        self.assertEqual(result.returncode, 0, result.stderr.decode())
                    else:
                        # Run the actual consumer through extraction, README replay setup,
                        # and source staging. Stop at the unavailable Apple compiler.
                        class CompilerReached(Exception):
                            pass
                        def compiler(args, **kwargs):
                            self.assertEqual(args[:2], ['xcrun', 'clang'])
                            stage = run / 'consumer'
                            self.assertIn('TTFoundationClient', (stage / 'Sources/Public/main.m').read_text())
                            raise CompilerReached
                        with patch.dict(os.environ, THINKTHEN_ARTIFACT=context['apple_artifacts'][language],
                                        THINKTHEN_COVERAGE_STAGE=str(run / 'consumer')), patch.object(
                                sys, 'platform', 'darwin'), patch.object(subprocess, 'run', side_effect=compiler):
                            with self.assertRaises(CompilerReached):
                                runpy.run_path(str(checkout / 'libraries/objective-c/checks/foundation_installed.py'))

    def test_apple_collector_requires_matching_archive_and_retains_executables(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            native = root / 'target-c/debug/libthinkthen_c.a'
            native.parent.mkdir(parents=True)
            native.write_bytes(b'instrumented archive')
            header = root / 'libraries/c/include/thinkthen.h'
            header.parent.mkdir(parents=True)
            header.write_bytes(b'current header')
            package = root / 'package'
            slice = package / 'CThinkThen.xcframework/macos-arm64'
            (slice / 'Headers').mkdir(parents=True)
            (slice / 'Headers/thinkthen.h').write_bytes(header.read_bytes())
            metadata = {'AvailableLibraries': [dict(SupportedPlatform='macos', SupportedArchitectures=['arm64'],
                         LibraryIdentifier='macos-arm64', LibraryPath='libthinkthen.a', HeadersPath='Headers')]}
            (slice.parent / 'Info.plist').write_bytes(plistlib.dumps(metadata))
            env = {'PATH': '/usr/bin:/bin'}
            def command(args, cwd, settings, log, **kwargs):
                self.assertEqual(args, ['sh', './check.sh'])
                self.assertEqual(settings['THINKTHEN_COVERAGE_INITIALIZER'], str(root / 'profile.o'))
                retained = Path(settings['THINKTHEN_COVERAGE_STAGE'])
                (retained / 'installed').mkdir(parents=True)
                for binary in ('Owned', 'Parity'):
                    (retained / 'installed' / binary).write_bytes(binary.encode())
            context = dict(root=root, command=command, missing_tool=DRIVER['MissingTool'],
                           c_target=native.parents[1], initializer=root / 'profile.o')
            sources = ('Sources/Foundation/TTResult.h', 'Sources/Foundation/module.modulemap', 'README-PUBLIC.md', 'README.md')
            for name in sources:
                original = root / 'libraries/swift' / name
                original.parent.mkdir(parents=True, exist_ok=True)
                original.write_bytes(b'technical source' if name == 'README.md' else b'checkout source')
                (package / name).parent.mkdir(parents=True, exist_ok=True)
            subprocess.run(['git', 'init', '-q', str(root)], check=True)
            subprocess.run(['git', '-C', str(root), 'add', 'libraries/swift'], check=True)
            for mismatch in ('archive', *sources, 'missing', 'extra', None):
                for name in sources:
                    (package / name).write_bytes(b'changed source' if mismatch == name else b'checkout source')
                if mismatch == 'missing':
                    (package / sources[0]).unlink()
                extra = package / 'unknown.txt'
                if mismatch == 'extra':
                    extra.write_text('untracked package input')
                elif extra.exists():
                    extra.unlink()
                (slice / 'libthinkthen.a').write_bytes(b'ordinary archive' if mismatch == 'archive' else native.read_bytes())
                artifact = root / 'swift.tar.gz'
                coverage_installed_managed.archive(package, artifact, [item.name for item in package.iterdir()])
                stage = root / str(mismatch).replace('/', '-')
                stage.mkdir()
                retained = stage / 'retained'
                with patch.dict(os.environ, THINKTHEN_SWIFT_ARTIFACT=str(artifact)), patch.object(
                        coverage_installed_managed.platform, 'machine', return_value='arm64'), patch.object(
                        coverage_installed_managed.shutil, 'which', return_value='/usr/bin/tool'):
                    if mismatch:
                        error = 'differs from the instrumented' if mismatch == 'archive' else 'source differs'
                        with self.assertRaisesRegex(RuntimeError, error):
                            coverage_installed_managed.apple_collection('swift', context, env, stage, retained, stage)
                    else:
                        coverage_installed_managed.apple_collection('swift', context, env, stage, retained, stage)
                        self.assertEqual((retained / 'Parity').read_bytes(), b'Parity')


    def test_csharp_requires_executable_bwrap_at_the_consumer_path(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for tool in ('bwrap', 'dotnet'):
                binary = root / tool
                binary.write_text('#!/bin/sh\nexit 0\n')
                binary.chmod(0o700)
            env = {'HOME': str(root), 'PATH': str(root)}
            fixed = root / 'fixed-bwrap'
            real_which = coverage_installed_managed.shutil.which
            self.assertIsNotNone(real_which('bwrap', path=env['PATH']))
            def which(tool, path=None):
                return real_which(str(fixed) if tool == '/usr/bin/bwrap' else tool, path=path)
            for available in (False, True):
                with self.subTest(fixed_path_exists=available):
                    if available:
                        fixed.write_text('#!/bin/sh\nexit 0\n')
                        fixed.chmod(0o600)
                    with patch.object(coverage_installed_managed.shutil, 'which', side_effect=which):
                        with self.assertRaisesRegex(DRIVER['MissingTool'], '/usr/bin/bwrap'):
                            coverage_installed_managed.require_tools('csharp', env, DRIVER['MissingTool'])


class ProcessOwnership(unittest.TestCase):
    @unittest.skipUnless(sys.platform == 'linux', 'Linux adopted-child ownership')
    def test_orphan_disappears_before_harness_exits(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            script = """import os, pathlib, time
print('before orphan', flush=True)
parent = os.fork()
if parent == 0:
    orphan = os.fork()
    if orphan == 0:
        os._exit(0)
    pathlib.Path('orphan.pid').write_text(str(orphan))
    os._exit(0)
os.waitpid(parent, 0)
orphan = int(pathlib.Path('orphan.pid').read_text())
deadline = time.monotonic() + 3
while True:
    try:
        os.kill(orphan, 0)
    except ProcessLookupError:
        print('orphan reaped while harness alive', flush=True)
        break
    if time.monotonic() >= deadline:
        raise AssertionError('orphan remains visible while harness alive')
    time.sleep(0.01)
"""
            for capture in (False, True):
                with self.subTest(capture=capture):
                    result = DRIVER['command']([sys.executable, '-c', script], root,
                                               {'PATH': os.defpath}, root / 'active.log',
                                               capture=capture, timeout=5)
                    self.assertIn('orphan reaped while harness alive', (root / 'active.log').read_text())
                    if capture:
                        self.assertEqual(result, b'before orphan\norphan reaped while harness alive\n')

    @unittest.skipUnless(sys.platform == 'linux', 'Linux adopted-child ownership')
    def test_direct_and_foreign_wait_statuses_and_signal_mask_survive(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            foreign = subprocess.Popen([sys.executable, '-c', 'import sys; sys.exit(23)'],
                                       env={'PATH': os.defpath})
            previous_handler = signal.getsignal(signal.SIGCHLD)
            previous_mask = signal.pthread_sigmask(signal.SIG_BLOCK, [])
            try:
                for attempt in range(10):
                    with self.assertRaisesRegex(RuntimeError, 'exit 47'):
                        DRIVER['command']([sys.executable, '-c', 'import sys; sys.exit(47)'],
                                          root, {'PATH': os.defpath}, root / 'immediate.log', capture=True)
                script = "import json, signal; print(json.dumps(sorted(map(int, signal.pthread_sigmask(signal.SIG_BLOCK, [])))))"
                result = DRIVER['command']([sys.executable, '-c', script], root,
                                           {'PATH': os.defpath}, root / 'mask.log', capture=True)
                self.assertEqual(json.loads(result), sorted(map(int, previous_mask)))
                self.assertEqual(signal.getsignal(signal.SIGCHLD), previous_handler)
                self.assertEqual(signal.pthread_sigmask(signal.SIG_BLOCK, []), previous_mask)
                self.assertEqual(foreign.wait(timeout=3), 23)
            finally:
                if foreign.poll() is None:
                    foreign.kill()
                foreign.wait()

    def test_timeout_stops_and_reaps_descendants_with_diagnostics(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            script = """import os, pathlib, time
child = os.fork()
if child == 0:
    print('descendant diagnostic', flush=True)
    time.sleep(30)
else:
    pathlib.Path('child.pid').write_text(str(child))
    time.sleep(30)
"""
            for capture in (False, True):
                with self.assertRaisesRegex(RuntimeError, r'timed out \(exit -9\)'):
                    DRIVER['command']([sys.executable, '-c', script], root, os.environ,
                                      root / 'timeout.log', capture=capture, timeout=0.3)
                child = int((root / 'child.pid').read_text())
                with self.assertRaises(ProcessLookupError):
                    os.kill(child, 0)
                self.assertIn('descendant diagnostic', (root / 'timeout.log').read_text())

    def test_real_exit_and_diagnostics_survive(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            for code, exception, message in ((77, DRIVER['MissingTool'], 'see'),
                                             (4, RuntimeError, 'exit 4'),
                                             (-signal.SIGTERM, RuntimeError, 'exit -15')):
                script = f"import os, sys; print('diagnostic', flush=True); "
                script += f'os.kill(os.getpid(), {-code})' if code < 0 else f'sys.exit({code})'
                with self.assertRaisesRegex(exception, message):
                    DRIVER['command']([sys.executable, '-c', script], root, os.environ, root / 'exit.log')
                self.assertIn('diagnostic', (root / 'exit.log').read_text())


class ProfileCauses(unittest.TestCase):
    def test_real_signals_and_file_limits_remain_unmeasured(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            collection = root / 'rust-integration'
            collection.mkdir()
            source = Path(__file__).with_name('coverage-profile.c').resolve()
            subprocess.run(['cc', '-fPIC', '-Wall', '-Wextra', '-Werror', '-c', str(source),
                            '-DPROFILE_PATH=' + json.dumps(str(collection / '%m.profraw')),
                            '-DPROFILE_DIR=' + json.dumps(str(collection)), '-o', str(root / 'profile.o')], check=True)
            program = 'fn main() { let a: Vec<_> = std::env::args().collect(); if a.len() > 1 { unsafe { unsafe extern "C" { fn raise(s: i32) -> i32; } raise(a[1].parse().unwrap()); } } }'
            subprocess.run(['rustc', '--edition', '2024', '-C', 'instrument-coverage', '-C',
                            'link-arg=' + str(root / 'profile.o'), '-o', str(root / 'probe'), '-'],
                           input=program.encode(), check=True)
            env = {'PATH': os.environ['PATH']}
            for signum in (signal.SIGINT, signal.SIGTERM):
                result = subprocess.run([str(root / 'probe'), str(signum)], cwd=root, env=env, capture_output=True)
                self.assertEqual(result.returncode, -signum)
                self.assertEqual(result.stderr, b'')
            result = subprocess.run(['sh', '-c', 'ulimit -f 1; exec "$1"', 'limit', str(root / 'probe')],
                                    cwd=root, env=env, capture_output=True)
            self.assertEqual(result.returncode, 0)
            self.assertEqual(result.stderr, b'')
            subprocess.run([str(root / 'probe')], cwd=root, env=env, check=True)
            self.assertFalse((root / 'default.profraw').exists())
            self.assertEqual(len(list(collection.glob('limited-fsize-*'))), 1)
            missing = [p for p in collection.glob('process-*') if not (p / 'normal-exit').exists()]
            self.assertEqual(len(missing), 2)
            self.assertTrue(list(collection.glob('*.profraw')))
            (root / 'crates/thinkthen/src').mkdir(parents=True)
            (root / 'crates/thinkthen/src/probe.rs').write_text('line one\nline two\n')
            lcov = f'SF:{root}/crates/thinkthen/src/probe.rs\nDA:1,0\nDA:2,0\n'
            for name in ('cli-source', 'source-unit'):
                (root / name).mkdir()
            DRIVER['report'](root, root, 'probe', lcov,
                             {name: lcov for name in ('rust-integration', 'cli-source', 'source-unit')}, 0)
            report = json.loads((root / 'totals.json').read_text())
            self.assertEqual(report['totals']['unobserved'], 2)
            self.assertNotIn('unreached', report['totals'])
            self.assertNotIn('unit_only', report['totals'])
            self.assertEqual(len(report['missing_child_profiles']['rust-integration']['no_normal_exit']), 2)


class CollectionClassification(unittest.TestCase):
    def test_c_library_target_launches_with_its_metadata_identity(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            package_dir = root / 'libraries/c'
            package_dir.mkdir(parents=True)
            harness = root / 'c-harness'
            harness.write_text(f'#!{sys.executable}\n' + "import os, pathlib, sys\n"
                               "assert os.environ['CARGO_PKG_NAME'] == 'thinkthen-c'\n"
                               "assert pathlib.Path.cwd() == pathlib.Path(os.environ['CARGO_MANIFEST_DIR'])\n"
                               "assert sys.argv[1:] == ['--test-threads=2']\n")
            harness.chmod(0o700)
            package = {'name': 'thinkthen-c', 'manifest_path': str(package_dir / 'Cargo.toml')}
            target = {'kind': ['cdylib', 'staticlib'], 'name': 'thinkthen_c'}
            artifact = {'reason': 'compiler-artifact', 'target': target,
                        'executable': str(harness), 'profile': {'test': True}}
            production = {**artifact, 'profile': {'test': False}}
            artifacts = DRIVER['test_artifacts']('\n'.join(map(json.dumps, (artifact, production))), package)
            self.assertEqual(len(artifacts), 1)
            DRIVER['collect_tests'](root, {}, artifacts, 'c-source-unit')
            self.assertTrue((root / 'c-source-unit/thinkthen_c.log').is_file())

    def test_c_source_units_add_unit_hits_without_test_only_denominator(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / 'libraries/c/src/probe.rs'
            source.parent.mkdir(parents=True)
            source.write_text('production one\nproduction two\ntest only\n')
            production = f'SF:{source}\nDA:1,0\nDA:2,0\n'
            c_units = f'SF:{source}\nDA:1,1\nDA:2,1\nDA:3,1\n'
            outside = f'SF:{source}\nDA:1,1\nDA:2,0\n'
            collections = {name: production for name in DRIVER['NATIVE_COLLECTIONS']}
            collections.update({'c-source-unit': c_units, 'rust-integration': outside})
            for name in collections:
                (root / name).mkdir()
            DRIVER['report'](root, root, 'probe', production, collections, 0)
            totals = json.loads((root / 'totals.json').read_text())['totals']
            self.assertEqual(totals['mapped_nonblank'], 2)
            self.assertEqual(totals['c-source-unit'], 2)
            self.assertEqual(totals['observed_unit_only'], 1)
            self.assertEqual(totals['unobserved'], 0)


class InstalledEvidenceRetention(unittest.TestCase):
    def test_missing_surface_preserves_successes_without_its_mappings(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            source = root / 'crates/thinkthen/src/probe.rs'
            source.parent.mkdir(parents=True)
            source.write_text('available production\nunavailable production\n')
            executable = root / 'native'
            executable.touch()
            available = root / 'available.so'
            available.touch()
            unavailable = root / 'unavailable.so'
            unavailable.touch()
            for name in ('inventory', *DRIVER['NATIVE_COLLECTIONS']):
                (root / name).mkdir()
            def missing(folder):
                raise DRIVER['MissingTool']('Objective-C requires macOS')
            def completed(folder):
                (folder / 'assertions-passed').touch()
            installed = [dict(name='installed-objective-c', objects=[unavailable], runner=missing),
                         dict(name='installed-c', objects=[available], runner=completed)]
            exported = []
            def export(run, llvm, objects, name, env):
                exported.append(name)
                self.assertNotIn(unavailable, objects)
                if name == 'inventory':
                    self.assertIn(available, objects)
                return f'SF:{source}\nDA:1,1\n' + ('DA:2,0\n' if unavailable in objects else '')
            context = DRIVER['measure'].__globals__
            with patch.dict(context, ROOT=root, export=export):
                for release, code in ((False, 77), (True, 1)):
                    if release:
                        for collection in installed:
                            shutil.rmtree(root / collection['name'])
                    def main():
                        DRIVER['measure'](root, {}, root, executable, [], [], installed, 'fixture',
                                          time.monotonic(), False)
                    with patch.dict(DRIVER['entry'].__globals__, main=main), patch.object(
                            sys, 'argv', ['coverage'] + (['--release'] if release else [])):
                        self.assertEqual(DRIVER['entry'](), code)
                    result = json.loads((root / 'totals.json').read_text())
                    records = root / 'sdlc/records/coverage'
                    self.assertEqual(list(records.glob('*.json')), [])
                    summary = next(records.glob('*.md'))
                    self.assertEqual(result['unmeasured_collections'], ['installed-objective-c'])
                    self.assertEqual(result['unmeasured_reasons'],
                                     {'installed-objective-c': 'Objective-C requires macOS'})
                    self.assertIn('installed-objective-c: Objective-C requires macOS',
                                  summary.read_text())
                    self.assertEqual(result['totals']['mapped_nonblank'], 1)
                    self.assertEqual(result['totals']['installed-c'], 1)
                    self.assertNotIn('installed-objective-c', result['totals'])
                    self.assertIn('partial', result['scope'])
                    self.assertTrue((root / 'installed-c/assertions-passed').exists())
                    self.assertIn('requires macOS', (root / 'installed-objective-c/unmeasured.txt').read_text())
            self.assertNotIn('installed-objective-c', exported)
            lcov = f'SF:{source}\nDA:1,1\n'
            DRIVER['report'](root, root, 'labels', lcov, {'rust-integration': lcov}, 0,
                             missing=('installed-objective-c',))
            labels = json.loads((root / 'totals.json').read_text())
            self.assertEqual(labels['unmeasured_collections'], ['installed-objective-c'])
            self.assertEqual(labels['unmeasured_reasons'], {})

    def test_routine_failure_missing_object_and_export_failure_remain_fatal(self):
        for cause in ('assertion', 'retained object', 'empty objects', 'export', 'export exit 77'):
            with self.subTest(cause=cause), tempfile.TemporaryDirectory() as temporary:
                root = Path(temporary)
                object_path = root / 'consumer'
                if cause != 'retained object':
                    object_path.touch()
                def runner(folder):
                    if cause == 'assertion':
                        raise RuntimeError('consumer assertion failed')
                installed = [dict(name='installed-c', objects=[] if cause == 'empty objects' else [object_path], runner=runner)]
                def export(*args):
                    if cause == 'export exit 77':
                        raise DRIVER['MissingTool']('export exit 77')
                    raise RuntimeError('export failed')
                with patch.dict(DRIVER['measure'].__globals__, export=export):
                    with self.assertRaises(RuntimeError):
                        DRIVER['measure'](root, {}, root, object_path, [], [], installed,
                                          'fixture', time.monotonic(), False)
                self.assertFalse((root / 'totals.json').exists())
                self.assertFalse((root / 'installed-c/unmeasured.txt').exists())


if __name__ == '__main__':
    unittest.main()
