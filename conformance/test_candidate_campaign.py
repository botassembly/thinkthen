"""Regressions for candidate exclusions and retained npm release inputs."""
import contextlib
import io
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile
import unittest
from unittest.mock import patch

from conformance import parity
from conformance.children.children import child_env
from conformance.source_fixtures import stage_files, stage_generated_source


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
    def test_full_smoke_fails_exit77_and_only_delegates_apple(self):
        with tempfile.TemporaryDirectory(prefix='thinkthen-smoke-candidate-') as folder:
            root = Path(folder)
            scripts, tools, binaries = root / 'sdlc/scripts', root / 'tools', root / 'bin'
            for directory in (scripts, tools, binaries):
                directory.mkdir(parents=True)
            for name in ('smoke', 'full-compiler', 'verdict.sh', 'scratch.sh'):
                shutil.copyfile(parity.ROOT / 'sdlc/scripts' / name, scripts / name)
            (scripts / 'heavy-lock').write_text('')
            backend = binaries / 'conformance-backend'
            backend.write_text('#!/bin/sh\necho 12345\nwhile read line; do echo 1; done\n')
            binary = binaries / 'thinkthen'
            binary.write_text('#!/bin/sh\ncat >/dev/null\necho true\n')
            jq = tools / 'jq'
            jq.write_text('#!/bin/sh\necho true\n')
            for executable in (backend, binary, jq):
                executable.chmod(0o755)
            for member in ('c', 'swift', 'objective-c'):
                library = root / 'libraries' / member
                library.mkdir(parents=True)
                (library / 'check.sh').write_text('exit 77\n')
            env = {'PATH': str(tools) + ':' + os.defpath, 'HOME': str(root / 'home')}
            for full, member, delegate, expected in [
                    (False, 'c', '', 0), (True, 'c', '', 1),
                    (True, 'c', 'required', 1), (True, 'objective-c', '', 1),
                    (True, 'objective-c', 'required', 0), (True, 'swift', 'required', 0)]:
                with self.subTest(full=full, member=member, delegate=delegate):
                    args = ['sh', str(scripts / 'smoke')]
                    if full:
                        args.append('--full-functional')
                    args += ['--one', str(binaries), 'libraries/' + member]
                    result = subprocess.run(args, cwd=root,
                                            env=env | {'THINKTHEN_MACOS_APPLE_CHECKS': delegate},
                                            capture_output=True, text=True, check=False)
                    self.assertEqual(result.returncode, expected, result.stdout + result.stderr)
                    if full and member != 'c' and delegate:
                        self.assertIn('pending', result.stdout)
                        self.assertNotIn('pass', result.stdout)


class RetainedNpmArtifact(unittest.TestCase):
    def test_full_preparation_keeps_the_package_release_pack_reuses(self):
        with tempfile.TemporaryDirectory(prefix='thinkthen-npm-retained-') as folder:
            root = Path(folder)
            library = root / 'libraries/typescript'
            scripts = root / 'sdlc/scripts'
            tools = root / 'home/.cache/thinkthen-toolchains/node-v22.22.3-linux-x64/bin'
            for directory in (library, scripts, tools, root / 'sdlc/generators/results'):
                directory.mkdir(parents=True)
            stage_generated_source(parity.ROOT, root, 'typescript')
            stage_files(parity.ROOT, root, ('sdlc/scripts/registry-readmes.py',
                        'libraries/typescript/README.md', 'libraries/typescript/package.json',
                        'crates/thinkthen/Cargo.toml'))
            shutil.copyfile(parity.ROOT / 'libraries/typescript/check.sh', library / 'check.sh')
            shutil.copyfile(parity.ROOT / 'sdlc/scripts/scratch.sh', scripts / 'scratch.sh')
            (library / 'build-addon.sh').write_text('printf addon > thinkthen-linux-x64.node\n')
            (library / 'fixture').write_text('package')
            packed_input = root / 'packed-input.tgz'
            with tarfile.open(packed_input, 'w:gz') as archive:
                for name in ('README.md', 'package.json', 'fixture'):
                    archive.add(library / name, arcname='package/' + name)
            node = shutil.which('node')
            (tools / 'node').write_text('#!/bin/sh\ncase "$*" in *prettier*) exec "' + node + '" "$@" ;; --version) echo v22.22.3 ;; *version*) echo 0.2.0 ;; *) echo linux-x64 ;; esac\n')
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
            shutil.copyfile(parity.ROOT / 'sdlc/scripts/release-pack', scripts / 'release-pack')
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
