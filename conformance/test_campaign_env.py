"""Run the real nested campaign rungs with isolated compiler and consumer probes."""
import json
import os
from pathlib import Path
import shlex
import subprocess
import sys
import tempfile
import unittest

from conformance.source_fixtures import stage_checkout, fixture_tool_groups, stub_other_surfaces
from conformance import parity
from conformance.children.children import child_env


class CampaignEnvironment(unittest.TestCase):
    def test_nested_full_rungs_restore_owned_compiler_settings(self):
        with tempfile.TemporaryDirectory(prefix='thinkthen-campaign-env-') as folder:
            root = Path(folder).resolve()
            stage_checkout(parity.ROOT, root)
            python = shlex.quote(sys.executable)
            scripts = root / 'sdlc/scripts'
            scripts.mkdir(parents=True, exist_ok=True)
            (scripts / 'installed.sh').write_text('backend_start() { port=12345; }\n')
            for name in ('release-pack', 'release-smoke'):
                (scripts / name).write_text('exit 0\n')
            stub_other_surfaces(root, {'libraries/c'})
            consumer = root / 'libraries/c'
            consumer.mkdir(parents=True, exist_ok=True)
            (consumer / 'Cargo.toml').write_text('[package]\nname="fixture"\n')
            probe = root / 'probe.py'
            probe.write_text(f'''import json,os,sys,tempfile
sys.path.insert(0,{str(parity.ROOT)!r})
from conformance.parity import consumer_environment
from pathlib import Path
keys=('RUSTFLAGS','RUSTC_WRAPPER','CARGO_BUILD_RUSTC_WRAPPER',
      'CARGO_PROFILE_DEV_OPT_LEVEL','CARGO_PROFILE_TEST_OPT_LEVEL',
      'CARGO_PROFILE_DEV_DEBUG','CARGO_PROFILE_TEST_DEBUG',
      'CFLAGS','CXXFLAGS','THINKTHEN_FULL_COMPILER','GITHUB_ACTIONS','RUNNER_ENVIRONMENT')
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
                f'{python} "{probe}" "$THINKTHEN_TEST_PROFILE"\necho "smoke: true"\n')
            binaries = root / 'target/smoke/debug'
            binaries.mkdir(parents=True, exist_ok=True)
            executable = binaries / 'thinkthen'
            executable.write_text('#!/bin/sh\necho true\n')
            executable.chmod(0o755)
            executable = binaries / 'conformance-backend'
            executable.write_text(f'#!{sys.executable}\nimport sys\nprint(12345,flush=True)\n'
                                  'for line in sys.stdin:\n    if line.strip()=="count":print(1,flush=True)\n')
            executable.chmod(0o755)
            tools = root / 'tools'
            tools.mkdir()
            commands = {
                'cargo': f'{python} "{probe}" compiler\n',
                'rustc': 'echo "host: x86_64-unknown-linux-gnu"\n',
                'uname': 'case "$1" in -s) echo Linux ;; -m) echo x86_64 ;; esac\n',
                'jq': 'case "$*" in *expect.success*) echo true ;; *) echo fixture ;; esac\n',
                'python3': f'''case "$1" in */feature-inventory.py|sdlc/scripts/feature-inventory.py) exec {python} "$@" ;; esac
if [ "$1" = "{scripts / 'build-admission.py'}" ] || [ "$1" = sdlc/scripts/build-admission.py ]; then
    exec {python} -c 'import os,runpy,sys; os.getloadavg=lambda:(0,0,0); sys.argv=sys.argv[1:]; runpy.run_path(sys.argv[0],run_name="__main__")' "$@"
fi
if [ "$#" -eq 2 ] && [ "$1" = conformance/parity.py ]; then
    case "$2" in
    --tool-environment) echo THINKTHEN_TOOLCHAINS={tools}; exit 0 ;;
    --validate) exit 0 ;;
    12345) exec {python} "{probe}" parity ;;
    esac
fi
printf 'unsupported Python invocation: %s\\n' "$*" >&2
exit 2
''',
            }
            for name, body in commands.items():
                executable = tools / name
                executable.write_text('#!/bin/sh\nset -eu\n' + body)
                executable.chmod(0o755)
            env = child_env(home=root / 'home') | {'PATH': str(tools) + ':' + os.defpath,
                   'GITHUB_ACTIONS': 'true', 'RUNNER_ENVIRONMENT': 'github-hosted',
                   'THINKTHEN_FULL_COMPILER': 'ambient', 'CFLAGS': '--ambient-c-flag',
                   'CXXFLAGS': '--ambient-cxx-flag', 'RUSTFLAGS': '--ambient-flag', 'RUSTC_WRAPPER': '/ambient/rustc-wrapper',
                   'CARGO_PROFILE_DEV_OPT_LEVEL': '0',
                   'CARGO_PROFILE_TEST_OPT_LEVEL': '0', 'CARGO_BUILD_RUSTC_WRAPPER': '/ambient/wrapper'}
            env['THINKTHEN_RESOLVED_TOOLS'] = fixture_tool_groups(parity.ROOT)
            result = subprocess.run(['sh', str(scripts / 'test-full-cases'), '--run'], cwd=root,
                                    env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            self.assertEqual(result.returncode, 0, result.stdout.decode())
            rows = [json.loads(line) for line in (root / 'probes.jsonl').read_text().splitlines()]
            self.assertTrue({'compiler', 'smoke', 'full', 'parity'} <= {row['stage'] for row in rows})
            expected = {'CARGO_PROFILE_DEV_OPT_LEVEL': '2', 'CARGO_PROFILE_TEST_OPT_LEVEL': '2',
                        'GITHUB_ACTIONS': 'true', 'RUNNER_ENVIRONMENT': 'github-hosted',
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
                                     'settings': {'RUSTC_WRAPPER': '/ambient/rustc-wrapper',
                                                  'GITHUB_ACTIONS': 'true', 'RUNNER_ENVIRONMENT': 'github-hosted'}}])
            with (scripts / 'test-full-cases').open('a') as stream:
                stream.write('\npython3 conformance/not-a-real-helper.py\n')
            (root / 'probes.jsonl').write_text('')
            result = subprocess.run(['sh', str(scripts / 'test-full-cases'), '--run'], cwd=root,
                                    env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
            self.assertEqual(result.returncode, 2, result.stdout.decode())
            self.assertIn(b'unsupported Python invocation: conformance/not-a-real-helper.py',
                          result.stdout)
            rows = [json.loads(line) for line in (root / 'probes.jsonl').read_text().splitlines()]
            self.assertTrue({'compiler', 'smoke', 'full', 'parity'} <= {row['stage'] for row in rows})
            for row in rows:
                self.assertEqual(row['settings'], expected, row['stage'])

    def test_duckdb_build_boundary_keeps_full_and_smoke_compiler_inputs_stable(self):
        from importlib.util import module_from_spec, spec_from_file_location
        spec = spec_from_file_location('campaign_duckdb_inputs', parity.ROOT / 'databases/duckdb/tools/inputs.py')
        authority = module_from_spec(spec)
        spec.loader.exec_module(authority)
        with tempfile.TemporaryDirectory(prefix='thinkthen-duckdb-env-') as folder:
            root = Path(folder).resolve()
            stage_checkout(parity.ROOT, root)
            scripts, duckdb = root / 'sdlc/scripts', root / 'databases/duckdb'
            tools = root / 'tools'
            for path in (scripts, duckdb / 'tools', duckdb / 'cpp', tools,
                         root / 'crates/thinkthen', root / 'conformance/children'):
                path.mkdir(parents=True, exist_ok=True)
            (scripts / 'installed.sh').write_text('smoke_guard() { :; }\n')
            (duckdb / 'tools/validate_inputs.py').write_text(
                'from types import SimpleNamespace\narchive_tree = SimpleNamespace(raw_git=lambda *args: None)\n')
            for name in ('receipt_types.py', 'request_helpers.py'):
                (duckdb / 'tools' / name).write_text('')
            cache = tools / 'cache'
            target = 'x86_64-unknown-linux-gnu'
            for version in authority.versions():
                identity = authority.selected(version, target)
                host = cache / 'duckdb/community' / version
                for name in ('source/.git', 'tools', 'venv/bin'):
                    (host / name).mkdir(parents=True, exist_ok=True)
                (host / 'source/PIN').write_text(identity['source_commit'])
                program = ('#!/bin/sh\ncase "$*" in\n'
                           f"*source_id*|*'PRAGMA version'*) echo {identity['source_commit'][:10]} ;;\n"
                           f"*platform*) echo {identity['platform']} ;;\nesac\n")
                python = host / 'venv/bin/python'
                python.write_text(program)
                python.chmod(0o755)
                cli = duckdb / 'build/community' / version / 'build/release/duckdb'
                cli.parent.mkdir(parents=True, exist_ok=True)
                cli.write_text(program)
                cli.chmod(0o755)
            (root / 'crates/thinkthen/Cargo.toml').write_text('version = "1.0.0"\n')
            (root / 'vcpkg.json').write_text('{}\n')
            capture = '''import json,os,sys
from pathlib import Path
with Path(CAPTURE_PATH).open('a') as out:
    out.write(json.dumps({'args':sys.argv[1:], 'flags':{k:os.environ.get(k) for k in
        ('RUSTFLAGS','CFLAGS','CXXFLAGS')}})+'\\n')
'''.replace('CAPTURE_PATH', repr(str(root / 'capture.jsonl')))
            # Stop at the real community compiler boundary without building a host.
            (tools / 'make').write_text(f'#!{sys.executable}\n' + capture + 'sys.exit(91)\n')
            (tools / 'cargo').write_text(f'#!{sys.executable}\n' + capture + '''
if '--message-format=json' in sys.argv:
    print(json.dumps({'reason':'compiler-artifact',
                      'target':{'name':'conformance-backend'},
                      'executable':str(Path.cwd() / 'target/debug/conformance-backend')}))
''')
            (tools / 'git').write_text(f'#!{sys.executable}\nimport pathlib,sys\n'
                                      'print((pathlib.Path(sys.argv[2]) / "PIN").read_text())\n')
            (root / 'probe.py').write_text(capture)
            (tools / 'uname').write_text('#!/bin/sh\ncase "$1" in -s) echo Linux ;; -m) echo x86_64 ;; esac\n')
            for path in (tools / 'cargo', tools / 'make', tools / 'git', tools / 'uname'):
                path.chmod(0o755)
            binary = root / 'target/debug/conformance-backend'
            binary.parent.mkdir(parents=True, exist_ok=True)
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
                           'HOME': str(private), 'THINKTHEN_TOOLCHAINS': str(cache),
                           'THINKTHEN_HEAVY_LOCK_HELD': '/tmp/thinkthen-heavy.lock',
                           'THINKTHEN_DUCKDB_CHECK_ARCHIVES': '1',
                           'THINKTHEN_TEST_PROFILE': profile, 'RUSTFLAGS': '--ambient',
                           'CFLAGS': '-O1', 'CXXFLAGS': '-O1'}
                    env['THINKTHEN_RESOLVED_TOOLS'] = fixture_tool_groups(parity.ROOT)
                    env.pop('THINKTHEN_FULL_COMPILER', None)
                    command = 'REPO=$PWD; '
                    if full:
                        command += '. sdlc/scripts/full-compiler; '
                    command += 'python3 probe.py caller; exec sh databases/duckdb/check.sh 12345'
                    result = subprocess.run(['sh', '-c', command], cwd=root, env=env,
                                            stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
                    # The community driver retains the failed compiler status in
                    # CalledProcessError and exits 1 instead of exec'ing it.
                    self.assertEqual(result.returncode, 1, result.stdout.decode())
                    self.assertIn('returned non-zero exit status 91', result.stdout.decode())
                    rows = [json.loads(line) for line in receipt.read_text().splitlines()]
                    cargo, backend, cmake = rows[-3:]
                    self.assertIn('--message-format=json', backend['args'])
                    self.assertEqual(backend['flags'], cargo['flags'])
                    self.assertIn('caller' if full else 'test', cargo['args'])
                    self.assertEqual(cmake['args'][-1], 'release')
                    self.assertIn('DUCKDB_SRCDIR=' + str(cache / 'duckdb/community' / authority.versions()[0] / 'source'), cmake['args'])
                    self.assertEqual(cmake['flags'], cargo['flags'])
                    for key in ('RUSTFLAGS', 'CFLAGS', 'CXXFLAGS'):
                        self.assertTrue(cmake['flags'][key], key)
                    if full:
                        self.assertNotIn(str(private), json.dumps(rows))
                        snapshots.append(cargo['flags'])
                    else:
                        self.assertIn(str(private), cargo['flags']['RUSTFLAGS'])
                        self.assertEqual(cargo['flags']['CFLAGS'], '-O1')
                        self.assertEqual(cargo['flags']['CXXFLAGS'], '-O1')
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
