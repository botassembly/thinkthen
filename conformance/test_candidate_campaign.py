"""Regressions for candidate exclusions and retained npm release inputs."""
import contextlib
import io
import json
import os
import platform
import signal
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile
import unittest
from unittest.mock import patch

from conformance import parity
from conformance.children.children import child_env
from conformance.source_fixtures import stage_checkout, fixture_tool_groups


class CandidatePlatforms(unittest.TestCase):
    def test_candidate_requires_execution_and_explicit_apple_delegation(self):
        inventory = parity.inventory()
        consumers = {row['id']: row for row in parity.all_consumers(inventory)}
        for members, delegation, exit_code, expected in [
                (('objective-c', 'c'), '', 0, 1),
                (('objective-c', 'swift', 'c'), 'required', 0, 0),
                (('objective-c', 'swift'), 'required', 0, 1),
                (('objective-c', 'c'), 'required', 77, 1),
                (('objective-c', 'c'), 'required', 0, 1)]:
            contract = inventory | {'consumers': [consumers[key] for key in members],
                                    'pending_consumers': []}
            missing = members == ('objective-c', 'c') and delegation and exit_code == 0
            with self.subTest(members=members, delegation=delegation, code=exit_code), tempfile.TemporaryDirectory() as folder:
                root = Path(folder)
                def command(args, **kwargs):
                    self.assertNotIn('libraries/objective-c/check.sh', args)
                    consumer = 'swift' if 'libraries/swift/check.sh' in args else 'c'
                    for index, case in enumerate(parity.required_cases(contract, consumer).values()):
                        if missing and index == 0:
                            continue
                        kwargs['stdout'].write(parity.PREFIX + json.dumps({
                            'consumer': consumer, 'case': case['id'], 'status': 'pass',
                            'checks': case.get('checks', ['named', 'runtime'])}) + '\n')
                    return subprocess.CompletedProcess(args, exit_code)
                with contextlib.redirect_stdout(io.StringIO()), patch.object(parity, 'ROOT', root), patch.object(parity, 'inventory', return_value=contract), patch.object(parity.sys, 'platform', 'linux'), patch.object(parity.subprocess, 'run', side_effect=command), patch.object(os, 'environ', {'PATH': os.defpath, 'THINKTHEN_MACOS_APPLE_CHECKS': delegation}):
                    self.assertEqual(parity.run('12345'), expected)
                matrix = json.loads((root / 'target/parity/matrix.json').read_text())
                self.assertTrue(any(row.get('platform_pending') for row in matrix))
                if delegation:
                    swift = next((row for row in matrix if row['consumer'] == 'swift'), None)
                    if swift:
                        self.assertTrue(swift.get('platform_pending'))
                        self.assertEqual(swift['adopted_case_count'], 0)


class CandidateSmokes(unittest.TestCase):
    @unittest.skipUnless(shutil.which('cc'), 'C compiler required')
    def test_four_smokes_admit_real_nested_compilers(self):
        # Four orchestrators must leave all four permits for their child builds.
        with tempfile.TemporaryDirectory(prefix='thinkthen-smoke-nested-') as folder:
            root = Path(folder)
            stage_checkout(parity.ROOT, root)
            scripts, tools, binaries = root / 'sdlc/scripts', root / 'tools', root / 'bin'
            for directory in (scripts, tools, binaries):
                directory.mkdir(parents=True, exist_ok=True)
            (binaries / 'conformance-backend').write_text(
                '#!/bin/sh\necho 12345\nwhile read line; do echo 1; done\n')
            (binaries / 'thinkthen').write_text('#!/bin/sh\ncat >/dev/null\necho true\n')
            (tools / 'jq').write_text('#!/bin/sh\necho true\n')
            for executable in (*binaries.iterdir(), tools / 'jq'):
                executable.chmod(0o755)
            (root / 'main.c').write_text('int main(void) { return 0; }\n')
            # Rendezvous makes the old four outer reservations deadlock reliably.
            (root / 'compile.py').write_text(
                'from pathlib import Path\nimport os,subprocess,sys,time\n'
                'slot=sys.argv[1]; Path("ready-"+slot).touch()\n'
                'env={"PATH":os.environ["PATH"]}\n'
                'deadline=time.monotonic()+5\n'
                'while len(list(Path(".").glob("ready-*"))) != 4:\n'
                ' assert time.monotonic()<deadline, "smoke children did not start"\n'
                ' time.sleep(0.01)\n'
                # This fixture tests reservations; host load has its own admission cases.
                'subprocess.run([sys.executable,"-c",'
                '"import os,runpy,sys; os.getloadavg=lambda:(0,0,0); '
                'sys.argv=sys.argv[1:]; runpy.run_path(sys.argv[0],run_name=\'__main__\')",'
                '"sdlc/scripts/build-admission.py",'
                '"--lock","build.lock","--timeout","3","--","cc","main.c","-o","built-"+slot],env=env,check=True)\n'
                'subprocess.run(["./built-"+slot],env=env,check=True)\n'
                'print("smoke: true")\n')
            env = child_env(home=root / 'home')
            env['THINKTHEN_RESOLVED_TOOLS'] = fixture_tool_groups(parity.ROOT)
            env['PATH'] = str(tools) + os.pathsep + env['PATH']
            children = []
            try:
                for slot in range(4):
                    surface = 'libraries/child' + str(slot)
                    (root / surface).mkdir(parents=True, exist_ok=True)
                    (root / surface / 'check.sh').write_text(
                        'exec python3 compile.py ' + str(slot) + '\n')
                    children.append(subprocess.Popen(
                        ['sh', str(scripts / 'smoke'), '--one', str(binaries), surface],
                        cwd=root, env=env, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, start_new_session=True))
                for child in children:
                    try:
                        log, _ = child.communicate(timeout=10)
                    except subprocess.TimeoutExpired as error:
                        self.fail(f'smoke stalled: ready={sorted(path.name for path in root.glob("ready-*"))}, '
                                  f'built={sorted(path.name for path in root.glob("built-*"))}; {error.output}')
                    self.assertEqual(child.returncode, 0, log)
                self.assertEqual(len(list(root.glob('built-*'))), 4)
            finally:
                for child in children:
                    if child.poll() is None:
                        os.killpg(child.pid, signal.SIGKILL)
                    child.communicate(timeout=3)

    def test_smoke_fails_required_exit77_and_reports_declared_unsupported(self):
        with tempfile.TemporaryDirectory(prefix='thinkthen-smoke-candidate-') as folder:
            root = Path(folder)
            stage_checkout(parity.ROOT, root)
            scripts, tools, binaries = root / 'sdlc/scripts', root / 'tools', root / 'bin'
            for directory in (scripts, tools, binaries):
                directory.mkdir(parents=True, exist_ok=True)
            (scripts / 'allow-list').write_text('')
            backend = binaries / 'conformance-backend'
            backend.write_text('#!/bin/sh\necho 12345\nwhile read line; do echo 1; done\n')
            binary = binaries / 'thinkthen'
            binary.write_text('#!/bin/sh\ncat >/dev/null\necho true\n')
            jq = tools / 'jq'
            jq.write_text('#!/bin/sh\necho true\n')
            uname = tools / 'uname'
            uname.write_text('#!/bin/sh\ncase "$1" in -s) echo Linux ;; -m) echo x86_64 ;; esac\n')
            for executable in (backend, binary, jq, uname):
                executable.chmod(0o755)
            for member in ('c', 'swift', 'objective-c', 'go'):
                library = root / 'libraries' / member
                library.mkdir(parents=True, exist_ok=True)
                (library / 'check.sh').write_text('exit 77\n')
            env = {'PATH': str(tools) + ':' + os.defpath, 'HOME': str(root / 'home')}
            env['THINKTHEN_RESOLVED_TOOLS'] = fixture_tool_groups(parity.ROOT)
            manifest = json.loads((root / 'sdlc/tools.json').read_text())
            arch = {'aarch64': 'arm64'}.get(platform.machine(), platform.machine())
            host = platform.system().lower() + '-' + arch
            for member in ('c', 'swift', 'objective-c', 'go'):
                required = host in manifest['libraries'][member]['check_platforms']
                with self.subTest(member=member, required=required):
                    args = ['sh', str(scripts / 'smoke'), '--one', str(binaries), 'libraries/' + member]
                    result = subprocess.run(args, cwd=root, env=env,
                                            capture_output=True, text=True, check=False)
                    self.assertEqual(result.returncode, int(required), result.stdout + result.stderr)
                    self.assertIn('FAIL' if required else 'unsupported', result.stdout)
                    self.assertNotIn('pass libraries/' + member, result.stdout)



class RetainedNpmArtifact(unittest.TestCase):
    def test_full_preparation_keeps_the_package_release_pack_reuses(self):
        with tempfile.TemporaryDirectory(prefix='thinkthen-npm-retained-') as folder:
            root = Path(folder)
            stage_checkout(parity.ROOT, root)
            library = root / 'libraries/typescript'
            scripts = root / 'sdlc/scripts'
            tools = root / 'home/.cache/thinkthen-toolchains/node-v22.22.3-linux-x64/bin'
            for directory in (library, scripts, tools, root / 'sdlc/generators/results'):
                directory.mkdir(parents=True, exist_ok=True)
            (library / 'build-addon.sh').write_text('printf addon > thinkthen-linux-x64.node\n')
            (library / 'fixture').write_text('package')
            packed_input = root / 'packed-input.tgz'
            with tarfile.open(packed_input, 'w:gz') as archive:
                for name in ('README.md', 'package.json', 'fixture'):
                    archive.add(library / name, arcname='package/' + name)
            node = shutil.which('node')
            (tools / 'node').write_text('#!/bin/sh\ncase "$*" in *prettier*) exec "' + node + '" "$@" ;; --version) echo v22.22.3 ;; *version*) echo 0.2.0 ;; *) echo linux-x64 ;; esac\n')
            (tools / 'uname').write_text('#!/bin/sh\ncase "$1" in -s) echo Linux ;; -m) echo x86_64 ;; esac\n')
            (tools / 'npm').write_text('''#!/bin/sh
set -eu
while [ "$1" != --pack-destination ]; do shift; done
mkdir -p "$2"
cp "''' + str(packed_input) + '''" "$2/thinkthen-0.2.0.tgz"
''')
            for path in tools.iterdir():
                path.chmod(0o755)
            (scripts / 'installed.sh').write_text('exit 91\n')
            env = child_env(home=root / 'home') | {'HOME': str(root / 'home'),
                   'PATH': str(tools) + ':' + os.environ['PATH'], 'THINKTHEN_TEST_PROFILE': 'full'}
            env['THINKTHEN_RESOLVED_TOOLS'] = fixture_tool_groups(parity.ROOT)
            for name in ('THINKTHEN_ARTIFACT', 'THINKTHEN_PARITY_PREPARED', 'THINKTHEN_PARITY_PREPARE'):
                env.pop(name, None)
            # Stop at installation: the real full preparation must already retain its input.
            result = subprocess.run(['sh', str(library / 'check.sh')], env=env,
                                    capture_output=True, text=True, check=False)
            artifact = library / 'target/pack/thinkthen-0.2.0.tgz'
            self.assertTrue(artifact.is_file(), result.stdout + result.stderr)
            self.assertEqual(artifact.read_bytes(), packed_input.read_bytes())
            with tarfile.open(artifact) as archive:
                self.assertEqual(archive.extractfile('package/fixture').read(), b'package')
            self.assertEqual(result.returncode, 91, result.stdout + result.stderr)
            (tools / 'rustc').write_text('#!/bin/sh\necho "host: x86_64-unknown-linux-gnu"\n')
            (tools / 'rustc').chmod(0o755)
            output = root / 'output'
            command = ['sh', str(scripts / 'release-pack'), '--reuse',
                       'x86_64-unknown-linux-gnu', str(output), 'typescript']
            reused = subprocess.run(command, env=env, capture_output=True, text=True)
            self.assertEqual(reused.returncode, 0, reused.stdout + reused.stderr)
            self.assertEqual((output / artifact.name).read_bytes(), artifact.read_bytes())
            artifact.unlink()
            missing = subprocess.run(command, env=env, capture_output=True, text=True)
            self.assertEqual(missing.returncode, 1, missing.stdout + missing.stderr)
            self.assertIn('missing; build it first', missing.stderr)


if __name__ == '__main__':
    unittest.main()
