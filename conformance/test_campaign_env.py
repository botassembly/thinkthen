"""Run the real nested campaign rungs with isolated compiler and consumer probes."""
import json
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

from conformance import parity
from conformance.children.children import child_env


class CampaignEnvironment(unittest.TestCase):
    def test_nested_full_rungs_restore_owned_compiler_settings(self):
        with tempfile.TemporaryDirectory(prefix='thinkthen-campaign-env-') as folder:
            root = Path(folder)
            scripts = root / 'sdlc/scripts'
            scripts.mkdir(parents=True)
            for name in ('test-full-cases', 'surfaces', 'smoke', 'heavy-lock',
                         'build-admission.py', 'allow-list', 'full-compiler', 'scratch.sh', 'verdict.sh'):
                shutil.copyfile(parity.ROOT / 'sdlc/scripts' / name, scripts / name)
            (scripts / 'installed.sh').write_text('backend_start() { port=12345; }\n')
            for name in ('release-pack', 'release-smoke'):
                (scripts / name).write_text('exit 0\n')
            (root / 'sdlc/surfaces.txt').write_text('libraries/c landed\n')
            consumer = root / 'libraries/c'
            consumer.mkdir(parents=True)
            (consumer / 'Cargo.toml').write_text('[package]\nname="fixture"\n')
            probe = root / 'probe.py'
            probe.write_text(f'''import json,os,sys,tempfile
sys.path.insert(0,{str(parity.ROOT)!r})
from conformance.parity import consumer_environment
from pathlib import Path
keys=('RUSTFLAGS','RUSTC_WRAPPER','CARGO_BUILD_RUSTC_WRAPPER',
      'CARGO_PROFILE_DEV_OPT_LEVEL','CARGO_PROFILE_TEST_OPT_LEVEL',
      'CARGO_PROFILE_DEV_DEBUG','CARGO_PROFILE_TEST_DEBUG',
      'CFLAGS','CXXFLAGS','THINKTHEN_FULL_COMPILER')
env={{}}
for key in keys:
    if key in os.environ:
        env[key]=os.environ[key]
if sys.argv[1]=='parity':
    with tempfile.TemporaryDirectory() as scratch:
        env=consumer_environment(scratch,'12345')
with (Path({str(root)!r})/'probes.jsonl').open('a') as stream:
    stream.write(json.dumps({{'stage':sys.argv[1],'settings':{{k:env[k] for k in keys if k in env}}}})+'\\n')
''')
            consumer.joinpath('check.sh').write_text(
                f'/usr/bin/python3 "{probe}" "$THINKTHEN_TEST_PROFILE"\necho "smoke: true"\n')
            binaries = root / 'target/smoke/debug'
            binaries.mkdir(parents=True)
            executable = binaries / 'thinkthen'
            executable.write_text('#!/bin/sh\necho true\n')
            executable.chmod(0o755)
            executable = binaries / 'conformance-backend'
            executable.write_text('#!/usr/bin/python3\nimport sys\nprint(12345,flush=True)\n'
                                  'for line in sys.stdin:\n    if line.strip()=="count":print(1,flush=True)\n')
            executable.chmod(0o755)
            tools = root / 'tools'
            tools.mkdir()
            commands = {
                'cargo': f'/usr/bin/python3 "{probe}" compiler\n',
                'rustc': 'echo "host: x86_64-unknown-linux-gnu"\n',
                'jq': 'case "$*" in *expect.success*) echo true ;; *) echo fixture ;; esac\n',
                'python3': f'''if [ "$1" = "{scripts / 'build-admission.py'}" ]; then
    exec /usr/bin/python3 "$@"
fi
case "$*" in
*--tool-environment*) echo THINKTHEN_TOOLCHAINS={tools}; exit 0 ;;
*--validate*) exit 0 ;;
*) exec /usr/bin/python3 "{probe}" parity ;;
esac
''',
            }
            for name, body in commands.items():
                executable = tools / name
                executable.write_text('#!/bin/sh\nset -eu\n' + body)
                executable.chmod(0o755)
            env = child_env(home=root / 'home') | {'PATH': str(tools) + ':' + os.defpath,
                   'THINKTHEN_HEAVY_LOCK': str(root / 'lock'),
                   'THINKTHEN_FULL_COMPILER': 'ambient', 'CFLAGS': '--ambient-c-flag',
                   'CXXFLAGS': '--ambient-cxx-flag', 'RUSTFLAGS': '--ambient-flag', 'RUSTC_WRAPPER': '/ambient/rustc-wrapper',
                   'CARGO_PROFILE_DEV_OPT_LEVEL': '0',
                   'CARGO_PROFILE_TEST_OPT_LEVEL': '0', 'CARGO_BUILD_RUSTC_WRAPPER': '/ambient/wrapper'}
            result = subprocess.run(['sh', str(scripts / 'test-full-cases'), '--run'], cwd=root,
                                    env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            self.assertEqual(result.returncode, 0, result.stdout.decode())
            rows = [json.loads(line) for line in (root / 'probes.jsonl').read_text().splitlines()]
            self.assertTrue({'compiler', 'smoke', 'full', 'parity'} <= {row['stage'] for row in rows})
            expected = {'CARGO_PROFILE_DEV_OPT_LEVEL': '2', 'CARGO_PROFILE_TEST_OPT_LEVEL': '2',
                        'CARGO_PROFILE_DEV_DEBUG': '0', 'CARGO_PROFILE_TEST_DEBUG': '0',
                        'RUSTC_WRAPPER': '', 'CARGO_BUILD_RUSTC_WRAPPER': '',
                        'THINKTHEN_FULL_COMPILER': '1',
                        **{key: '-ffile-prefix-map=/home=/build/host '
                                '-ffile-prefix-map=/Users=/build/users '
                                f'-ffile-prefix-map={root}=/build/thinkthen'
                           for key in ('CFLAGS', 'CXXFLAGS')},
                        'RUSTFLAGS': '--remap-path-prefix=/home=/build/host '
                                     '--remap-path-prefix=/Users=/build/users '
                                     f'--remap-path-prefix={root}=/build/thinkthen'}
            for row in rows:
                self.assertEqual(row['settings'], expected, row['stage'])
            (root / 'probes.jsonl').write_text('')
            subprocess.run(['sh', str(scripts / 'surfaces')], cwd=root, env=env,
                           check=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            rows = [json.loads(line) for line in (root / 'probes.jsonl').read_text().splitlines()]
            self.assertEqual(rows, [{'stage': 'routine',
                                     'settings': {'RUSTC_WRAPPER': '/ambient/rustc-wrapper'}}])

    def test_duckdb_build_boundary_keeps_full_and_smoke_compiler_inputs_stable(self):
        with tempfile.TemporaryDirectory(prefix='thinkthen-duckdb-env-') as folder:
            root = Path(folder)
            scripts, duckdb = root / 'sdlc/scripts', root / 'databases/duckdb'
            tools = root / 'tools'
            for path in (scripts, duckdb / 'tools', duckdb / 'cpp', tools / 'venv/bin',
                         tools / 'source', tools / 'static-libs', root / 'crates/thinkthen'):
                path.mkdir(parents=True)
            for source in ('databases/duckdb/check.sh', 'databases/duckdb/cpp/build.sh',
                           'sdlc/scripts/scratch.sh', 'sdlc/scripts/full-compiler'):
                shutil.copyfile(parity.ROOT / source, root / source)
            (scripts / 'installed.sh').write_text('smoke_guard() { :; }\n')
            (duckdb / 'tools/prepare_inputs.sh').write_text('input_verify() { :; }\n')
            (duckdb / 'tools/validate_inputs.py').write_text('')
            (duckdb / 'tools/inputs.sh').write_text(f'''
input_versions() {{ echo v1; }}
input_target() {{ echo x86_64-unknown-linux-gnu; }}
input_select() {{ TOOLS={tools}; DUCKDB_VERSION=v1; platform=fixture;
    cli_hash=fixture; duckdb_source_commit=fixture; manifest=fixture; }}
''')
            (root / 'crates/thinkthen/Cargo.toml').write_text('version = "1.0.0"\n')
            (tools / 'platform.txt').write_text('fixture\n')
            (tools / 'static-libs/libduckdb_static.a').touch()
            (tools / 'venv/bin/python').write_text('#!/bin/sh\necho v1\n')
            capture = '''import json,os,sys
from pathlib import Path
with Path(os.environ['CAPTURE']).open('a') as out:
    out.write(json.dumps({'args':sys.argv[1:], 'flags':{k:os.environ.get(k) for k in
        ('RUSTFLAGS','CFLAGS','CXXFLAGS')}})+'\\n')
'''
            (scripts / 'release-archive-tree.py').write_text(capture + 'sys.exit(91)\n')
            (tools / 'cargo').write_text('#!/usr/bin/python3\n' + capture)
            for name, body in (('rustc', 'echo "host: x86_64-unknown-linux-gnu"'),
                               ('cmake', 'exit 0'), ('duckdb', 'exit 0')):
                (tools / name).write_text('#!/bin/sh\n' + body + '\n')
            for path in (tools / 'cargo', tools / 'rustc', tools / 'cmake',
                         tools / 'duckdb', tools / 'venv/bin/python'):
                path.chmod(0o755)
            binary = root / 'target/debug/conformance-backend'
            binary.parent.mkdir(parents=True)
            binary.write_text('#!/bin/sh\nexit 0\n')
            binary.chmod(0o755)
            snapshots = []
            for profile, full in (('full', True), ('smoke', True), ('routine', False)):
                for home in ('home-one', 'home-two'):
                    private = root / home
                    private.mkdir(exist_ok=True)
                    receipt = root / 'capture.jsonl'
                    receipt.write_text('')
                    env = child_env(home=private) | {'PATH': str(tools) + ':' + os.defpath,
                           'HOME': str(private), 'CAPTURE': str(receipt),
                           'THINKTHEN_TEST_PROFILE': profile, 'RUSTFLAGS': '--ambient',
                           'CFLAGS': '-O1', 'CXXFLAGS': '-O1'}
                    env.pop('THINKTHEN_FULL_COMPILER', None)
                    command = 'REPO=$PWD; '
                    if full:
                        command += '. sdlc/scripts/full-compiler; '
                    command += 'exec sh databases/duckdb/check.sh 12345'
                    result = subprocess.run(['sh', '-c', command], cwd=root, env=env,
                                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
                    self.assertEqual(result.returncode, 91, result.stdout.decode())
                    rows = [json.loads(line) for line in receipt.read_text().splitlines()]
                    cargo, cmake = rows[-2:]
                    self.assertIn('build', cargo['args'])
                    self.assertEqual(cmake['flags'], cargo['flags'])
                    for key in ('CFLAGS', 'CXXFLAGS'):
                        self.assertIn(f'-DCMAKE_{key[:-5]}_FLAGS={cargo["flags"][key]}', cmake['args'])
                    if full:
                        self.assertNotIn(str(private), json.dumps(rows))
                        snapshots.append(cargo['flags'])
                    else:
                        self.assertIn(str(private), cargo['flags']['RUSTFLAGS'])
                        self.assertIn(str(private), cargo['flags']['CFLAGS'])
            self.assertTrue(all(flags == snapshots[0] for flags in snapshots))


class TimeLimitCleanup(unittest.TestCase):
    def test_timeout_and_signals_finish_descendant_escalation(self):
        import ctypes
        import signal
        import sys
        # Linux otherwise leaves killed orphans to PID 1. Adopt and reap the fixture.
        linux = sys.platform.startswith('linux')
        if linux:
            self.assertEqual(ctypes.CDLL(None).prctl(36, 1, 0, 0, 0), 0)
        try:
            for cancellation, expected in ((None, 124), (signal.SIGTERM, 143), (signal.SIGINT, 130)):
                with self.subTest(signal=cancellation), tempfile.TemporaryDirectory() as folder:
                    fixture = Path(folder) / 'parent.py'
                    fixture.write_text('''import os,signal,time
child=os.fork()
if child==0:
    signal.signal(signal.SIGTERM,signal.SIG_IGN)
    os.write(1,(str(os.getpid())+'\\n').encode())
    while True:time.sleep(60)
signal.signal(signal.SIGTERM,lambda *_:exit(0))
while True:time.sleep(60)
''')
                    process = subprocess.Popen(['sh', str(parity.ROOT / 'sdlc/scripts/time-limit'),
                                                '1' if cancellation is None else '60', sys.executable,
                                                str(fixture)], stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                               env=child_env(home=Path(folder) / 'home'), text=True, preexec_fn=lambda: signal.signal(signal.SIGINT, signal.SIG_DFL))
                    descendant = int(process.stdout.readline())
                    try:
                        if cancellation is not None:
                            process.send_signal(cancellation)
                        _, error = process.communicate(timeout=8)
                        self.assertEqual(process.returncode, expected, error)
                        if linux:
                            reaped, status = os.waitpid(descendant, os.WNOHANG)
                            self.assertEqual(reaped, descendant, 'descendant survived helper return')
                            self.assertEqual(os.waitstatus_to_exitcode(status), -signal.SIGKILL)
                        else:
                            status = subprocess.run(['ps', '-o', 'stat=', '-p', str(descendant)],
                                                    env=child_env(home=Path(folder) / 'home'),
                                                    capture_output=True, text=True).stdout.strip()
                            self.assertTrue(not status or status.startswith('Z'),
                                            'descendant survived helper return: ' + status)
                    finally:
                        if process.poll() is None:
                            process.kill()
                        try:
                            os.kill(descendant, signal.SIGKILL)
                            if linux:
                                os.waitpid(descendant, 0)
                        except (ProcessLookupError, ChildProcessError):
                            pass
                        process.communicate()
        finally:
            if linux:
                ctypes.CDLL(None).prctl(36, 0, 0, 0, 0)

    def test_normal_exit_retains_stdin_and_exit_code(self):
        result = subprocess.run(['sh', str(parity.ROOT / 'sdlc/scripts/time-limit'), '10',
                                 'sh', '-c', 'read value; printf "%s" "$value"; exit 7'],
                                env=child_env(), input='owned input\n', text=True, capture_output=True, timeout=3)
        self.assertEqual((result.returncode, result.stdout), (7, 'owned input'))
