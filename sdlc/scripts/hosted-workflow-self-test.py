#!/usr/bin/env python3
"""Plant hosted candidate routing and parameter failures at the actual entry points."""
from copy import deepcopy
import hashlib
import os
import io
import runpy
import tarfile
from pathlib import Path
import subprocess
import sys
import tempfile
from unittest.mock import patch

import yaml

REPO = Path(__file__).resolve().parents[2]
def workflow_command(name, doc):
    # Run the public gate against an isolated workflow tree, with its real scripts.
    with tempfile.TemporaryDirectory(prefix='thinkthen-hosted-gate-') as temporary:
        root = Path(temporary)
        for source in REPO.iterdir():
            if source.name not in ('.git', '.github', 'sdlc'):
                if source.name in ('.gitignore', '.gitattributes'):
                    (root / source.name).write_bytes(source.read_bytes())
                else:
                    (root / source.name).symlink_to(source)
        scripts = root / 'sdlc/scripts'
        scripts.mkdir(parents=True)
        for source in (REPO / 'sdlc/scripts').iterdir():
            target = scripts / source.name
            if source.name == 'workflows':
                target.write_bytes(source.read_bytes())
            else:
                target.symlink_to(source)
        (root / 'sdlc/generators').symlink_to(REPO / 'sdlc/generators')
        workflows = root / '.github/workflows'
        workflows.mkdir(parents=True)
        for source in (REPO / '.github/workflows').glob('*.y*ml'):
            (workflows / source.name).write_bytes(source.read_bytes())
        original = (REPO / '.github/workflows' / name).read_text()
        (workflows / name).write_text(original if doc == yaml.safe_load(original) else yaml.safe_dump(doc))
        subprocess.run(['git', 'init', '-q', str(root)], check=True)
        subprocess.run(['git', '-C', str(root), 'add', '.github/workflows'], check=True)
        result = subprocess.run([sys.executable, str(scripts / 'workflows')],
                                cwd=REPO, capture_output=True, text=True, timeout=30)
        return result.returncode, result.stdout + result.stderr


def workflow_cases():
    release = yaml.safe_load((REPO / '.github/workflows/release.yml').read_text())
    windows = yaml.safe_load((REPO / '.github/workflows/windows.yml').read_text())
    assert workflow_command('release.yml', release)[0] == 0
    assert workflow_command('windows.yml', windows)[0] == 0
    gate = yaml.safe_load((REPO / '.github/workflows/gate.yml').read_text())
    assert workflow_command('gate.yml', gate)[0] == 0
    for defect in ('options', 'declaration', 'dynamic', 'paid', 'string', 'default'):
        doc = deepcopy(gate)
        platform = doc.get('on', doc.get(True))['workflow_dispatch']['inputs']['platform']
        if defect == 'options': platform['options'] = {option: True for option in platform['options']}
        elif defect == 'declaration': doc.get('on', doc.get(True))['workflow_dispatch']['inputs']['platform'] = 'ubuntu-24.04'
        elif defect == 'dynamic': platform['options'] = ['${{ inputs.runner }}']
        elif defect == 'paid': platform['options'].append('macos-15-large')
        elif defect == 'string': platform['type'] = 'string'
        else: platform['default'] = 'macos-15-large'
        code, output = workflow_command('gate.yml', doc)
        assert code == 1 and 'standard free hosted runners' in output, (defect, code, output)
    for defect in ('paid', 'matrix-paid', 'skip-apple', 'mask-apple', 'wrong-sha', 'missing-language', 'mixed-paid', 'job-mask', 'upload-always'):
        doc = deepcopy(release)
        apple = doc['jobs']['apple-packages']
        proof = next(step for step in apple['steps'] if step.get('name') == 'Qualify installed Swift and Objective-C Foundation')
        if defect == 'paid':
            apple['runs-on'] = 'macos-15-large'
        elif defect == 'matrix-paid':
            apple['strategy']['matrix']['include'][0]['runner'] = 'macos-15-large'
        elif defect == 'skip-apple':
            proof['if'] = 'false'
        elif defect == 'mask-apple':
            proof['continue-on-error'] = True
        elif defect == 'wrong-sha':
            proof['env']['SHA'] = '${{ github.sha }}'
        elif defect == 'job-mask':
            apple['continue-on-error'] = True
        elif defect == 'upload-always':
            next(step for step in apple['steps'] if (step.get('with') or {}).get('name') == 'apple-packages-${{ matrix.target }}')['if'] = '${{ always() }}'
        elif defect == 'mixed-paid':
            apple['strategy']['matrix']['runner'] = ['macos-15']
            apple['strategy']['matrix']['include'][0]['runner'] = 'macos-15-large'
        else:
            proof['run'] = '\n'.join(proof['run'].splitlines()[:-1])
        code, output = workflow_command('release.yml', doc)
        assert code == 1 and ('standard free hosted runners' if 'paid' in defect else 'both installed Apple languages') in output, (defect, output)
    for defect in ('missing', 'skip', 'mask', 'wrong-sha', 'runner', 'upload'):
        doc = deepcopy(release)
        job = doc['jobs']['apple-coverage']
        if defect == 'missing':
            del doc['jobs']['apple-coverage']
        elif defect == 'runner':
            job['runs-on'] = 'macos-15-large'
        else:
            step = next(step for step in job['steps'] if 'sdlc/scripts/coverage' in step.get('run', ''))
            if defect == 'skip':
                step['if'] = 'false'
            elif defect == 'mask':
                step['continue-on-error'] = True
            elif defect == 'wrong-sha':
                job['steps'][0]['with']['ref'] = '${{ github.sha }}'
            else:
                job['steps'][-1]['if'] = '${{ always() }}'
        code, output = workflow_command('release.yml', doc)
        assert code == 1, (defect, output)
    for defect in ('skip', 'mask', 'wrong-sha', 'late', 'guard-bypass', 'missing-input'):
        doc = deepcopy(windows)
        steps = doc['jobs']['stage-0']['steps']
        guard = next(step for step in steps if step.get('name') == 'Require the exact candidate commit')
        if defect == 'skip':
            guard['if'] = 'false'
        elif defect == 'mask':
            guard['continue-on-error'] = True
        elif defect == 'wrong-sha':
            guard['env']['CANDIDATE_SHA'] = '${{ github.sha }}'
        elif defect == 'guard-bypass':
            next(step for step in steps if 'cargo test' in step.get('run', ''))['if'] = '${{ !cancelled() }}'
        elif defect == 'missing-input':
            doc.get('on', doc.get(True))['workflow_dispatch']['inputs'].pop('candidate_sha')
        else:
            steps.remove(guard)
            steps.append(guard)
        code, output = workflow_command('windows.yml', doc)
        assert code == 1 and 'candidate SHA' in output, (defect, output)


    for defect in ('wrong-test', 'skip-guard', 'paid', 'unknown-input', 'late-lane'):
        doc = deepcopy(windows)
        job = doc['jobs']['recognition-cancel']
        if defect == 'wrong-test':
            job['steps'][-1]['run'] = 'cargo test --workspace'
        elif defect == 'skip-guard':
            next(step for step in job['steps'] if step.get('id') == 'candidate')['if'] = 'false'
        elif defect == 'paid':
            job['runs-on'] = 'windows-latest-8-cores'
        elif defect == 'unknown-input':
            steps = doc['jobs']['stage-0']['steps']
            steps.remove(next(step for step in steps if step.get('name') == 'Require the standard Windows lane'))
        else:
            steps = doc['jobs']['stage-0']['steps']
            lane = next(step for step in steps if step.get('name') == 'Require the standard Windows lane')
            steps.remove(lane)
            steps.insert(next(i for i, step in enumerate(steps) if step.get('id') == 'candidate') + 1, lane)
        code, output = workflow_command('windows.yml', doc)
        assert code == 1, (defect, output)
    lane = next(step for step in windows['jobs']['stage-0']['steps']
                if step.get('name') == 'Require the standard Windows lane')
    for value, wanted in [('windows', 0), ('unknown', 1)]:
        result = subprocess.run(['/bin/sh', '-c', lane['run']], env={'LANE': value})
        assert result.returncode == wanted, value


def linux_tool_destination_case():
    gate = yaml.safe_load((REPO / '.github/workflows/gate.yml').read_text())
    step = next(step['run'] for step in gate['jobs']['linux-release']['steps']
                if step.get('name') == 'Install existing pinned Linux host and consumer tools')
    block, checksum = step.split('command -v mustmatch\n', 1)[1].split('printf ', 1)
    assert '. sdlc/scripts/fetch.sh\n' in block and 'fetch_url "$HOME/.local/bin/jq" ' in block
    assert '"$JQ_SHA256" "$HOME/.local/bin/jq" | sha256sum --check --strict' in checksum.splitlines()[0]
    url = 'https://github.com/jqlang/jq/releases/download/$JQ_VERSION/jq-linux-amd64'
    with tempfile.TemporaryDirectory(prefix='thinkthen-linux-tool-') as temporary:
        root = Path(temporary)
        payload = root / 'jq-fixture'
        payload.write_bytes(b'pinned jq fixture\n')
        home = root / 'home'
        env = {'PATH': os.environ['PATH'], 'HOME': str(home),
               'JQ_SHA256': hashlib.sha256(payload.read_bytes()).hexdigest()}
        assert not (home / '.local/bin').exists()
        # Keep missing-parent failures deterministic without waiting for retry pauses.
        command = 'sleep() { :; }\n' + block.replace(url, payload.as_uri())
        result = subprocess.run(['/bin/sh', '-ec', command],
                                cwd=REPO, env=env, capture_output=True, text=True, timeout=5)
        assert result.returncode == 0, (result.returncode, result.stdout, result.stderr)
        assert (home / '.local/bin/jq').read_bytes() == payload.read_bytes()
        assert hashlib.sha256((home / '.local/bin/jq').read_bytes()).hexdigest() == env['JQ_SHA256']


def linux_nested_tools_case():
    gate = yaml.safe_load((REPO / '.github/workflows/gate.yml').read_text())
    steps = gate['jobs']['linux-release']['steps']
    assert gate['jobs']['linux-coverage']['steps'] == steps
    selected = next(step['run'] for step in steps
                    if step.get('name') == 'Select existing Go, Swift, Zig, PHP, Dart and native language tools')
    handoff = next(line for line in selected.splitlines()
                   if '"$THINKTHEN_SCALA_HOME/bin"' in line and '>> "$GITHUB_PATH"' in line)
    assert 'python3 databases/duckdb/tools/source_hosts.py' in selected
    with tempfile.TemporaryDirectory(prefix='thinkthen-nested-tools-') as temporary:
        root = Path(temporary).resolve()
        path_file = root / 'github-path'
        path_file.touch()
        env = {'PATH': os.environ['PATH'], 'HOME': str(root), 'GITHUB_PATH': str(path_file)}
        tools = {'THINKTHEN_JDK_HOME': 'javac', 'THINKTHEN_KOTLIN_HOME': 'kotlinc',
                 'THINKTHEN_SCALA_HOME': 'scalac'}
        for variable, binary in tools.items():
            home = root / variable
            (home / 'bin').mkdir(parents=True)
            executable = home / 'bin' / binary
            executable.write_text('#!/bin/sh\nexit 0\n')
            executable.chmod(0o755)
            env[variable] = str(home)
        subprocess.run(['/bin/sh', '-ec', handoff], env=env, check=True, timeout=5)
        env['PATH'] = os.pathsep.join(path_file.read_text().splitlines() + [env['PATH']])
        # Use the real nested gate boundary: homes are removed, PATH must still select each SDK.
        command = '. sdlc/scripts/allow-list\n'
        command += '\n'.join(f'[ -z "${{{variable}+present}}" ]; [ "$(command -v {binary})" = "{env[variable]}/bin/{binary}" ]'
                             for variable, binary in tools.items())
        subprocess.run(['/bin/sh', '-ec', command], cwd=REPO, env=env, check=True, timeout=5)


def linux_coverage_destination_case():
    gate = yaml.safe_load((REPO / '.github/workflows/gate.yml').read_text())
    job = gate['jobs']['linux-coverage']
    assert job['steps'] == gate['jobs']['linux-release']['steps']
    setup = next(step['run'] for step in job['steps']
                 if step.get('name') == 'Require the frozen dispatch commit and candidate')
    selection = next(line.strip() for line in setup.splitlines() if line.strip().startswith('work=$('))
    command = selection + '\nprintf "%s" "$work/$STAGE/coverage"\n'
    driver = runpy.run_path(str(REPO / 'sdlc/scripts/coverage'))

    class AcceptedOutput(Exception):
        pass

    # Use Linux absolute paths even when macOS redirects /home through its own filesystem.
    # Stop at directory creation: exercise the privacy rule without scanning or building a tree.
    def accepts(output):
        path = Path(output)
        resolved = path if path == Path('/home') or Path('/home') in path.parents else path.resolve()
        with patch.dict(driver['main'].__globals__, tools=lambda: {}, installed_helpers=lambda: []), \
                patch.object(sys, 'platform', 'linux'), \
                patch.object(sys, 'argv', ['coverage', '--release', '--platform', 'linux', '--output', output]), \
                patch.object(subprocess, 'check_output', side_effect=['0' * 40, b'']), \
                patch.object(Path, 'resolve', return_value=resolved), \
                patch.object(Path, 'mkdir', side_effect=AcceptedOutput) as create:
            try:
                driver['main']()
            except AcceptedOutput:
                create.assert_called_once_with(parents=True, exist_ok=False)
                return True
            except RuntimeError as error:
                assert str(error) == 'installed coverage output must be outside /home to preserve package privacy'
                create.assert_not_called()
                return False
        raise AssertionError('coverage did not reach its output boundary')

    assert not accepts('/home/runner/work/_temp/thinkthen-linux-release/coverage/coverage')
    assert not accepts('/home')
    with tempfile.TemporaryDirectory(prefix='thinkthen-linux-coverage-') as temporary:
        root = Path(temporary)
        for roomy in (False, True):
            for name, body in (
                    ('df', 'case "$2" in /mnt) free=' + ('900' if roomy else '50') +
                     ' ;; *) free=100 ;; esac\nprintf "Filesystem Blocks Used Available Capacity Mounted\\nx 1 1 %s 1%% %s\\n" "$free" "$2"'),
                    ('sudo', 'exit 0')):
                tool = root / name
                tool.write_text('#!/bin/sh\n' + body + '\n')
                tool.chmod(0o755)
            env = {'PATH': f'{root}:{os.defpath}', 'STAGE': 'coverage',
                   'RUNNER_TEMP': '/home/runner/work/_temp'}
            result = subprocess.run(['/bin/sh', '-ec', command], cwd=REPO, env=env,
                                    capture_output=True, text=True, timeout=5)
            assert result.returncode == 0, result.stderr
            expected = '/mnt/thinkthen-container' if roomy else '/tmp/thinkthen-linux-release'
            assert result.stdout == expected + '/coverage/coverage', result.stdout
            assert accepts(result.stdout), result.stdout
            if not roomy:
                old = command.replace('/tmp/thinkthen-linux-release', '"$RUNNER_TEMP/thinkthen-linux-release"')
                result = subprocess.run(['/bin/sh', '-ec', old], cwd=REPO, env=env,
                                        capture_output=True, text=True, timeout=5)
                assert result.returncode == 0, result.stderr
                assert not accepts(result.stdout), result.stdout


def flutter_locked_setup_case():
    gate = yaml.safe_load((REPO / '.github/workflows/gate.yml').read_text())
    step = next(step['run'] for step in gate['jobs']['linux-release']['steps']
                if step.get('name') == 'Acquire Flutter from its pinned official source commit')
    acquisition = step.split('flutter=', 1)[0]
    command = '(cd "$HOME' + step.split('(cd "$HOME', 1)[1].split('git diff --exit-code', 1)[0]
    command += 'git diff --exit-code\n'
    with tempfile.TemporaryDirectory(prefix='thinkthen-flutter-locks-') as temporary:
        root = Path(temporary).resolve()
        sdk = root / 'home/.local/opt/flutter'
        # Exercise real Git acquisition through an isolated transport, without downloading an SDK.
        source = root / 'source'
        subprocess.run(['git', 'init', '-q', str(source)], check=True)
        git = ['git', '-C', str(source)]
        commit = git + ['-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid',
                        'commit', '-q', '--allow-empty', '-m', 'SDK fixture']
        subprocess.run(commit, check=True)
        pin = subprocess.check_output(git + ['rev-parse', 'HEAD'], text=True).strip()
        subprocess.run(git + ['tag', '3.47.5'], check=True)
        acquire = acquisition.replace('https://github.com/flutter/flutter.git', source.as_uri())
        acquire = acquire.replace('6a19cca56475dbfba1478ee68d7bd0c2ef891da1', pin)
        env = {'PATH': os.environ['PATH'], 'HOME': str(root / 'home')}
        result = subprocess.run(['/bin/sh', '-ec', acquire], env=env,
                                capture_output=True, text=True, timeout=5)
        assert result.returncode == 0, result.stderr
        tags = subprocess.check_output(['git', '-C', str(sdk), 'tag', '--points-at', 'HEAD'], text=True)
        assert tags.splitlines() == ['3.47.5'], 'Flutter cannot identify its release without its tag'
        assert subprocess.check_output(['git', '-C', str(sdk), 'rev-parse', 'HEAD'], text=True).strip() == pin
        # A moved release ref must fail against the approved commit before any SDK command runs.
        subprocess.run(commit, check=True)
        subprocess.run(git + ['tag', '-f', '3.47.5'], check=True, capture_output=True)
        result = subprocess.run(['/bin/sh', '-ec', acquire],
                                env={**env, 'HOME': str(root / 'moved-tag-home')},
                                capture_output=True, text=True, timeout=5)
        assert result.returncode == 1, (result.returncode, result.stderr)
        tools = sdk / 'packages/flutter_tools'
        tools.mkdir(parents=True)
        (tools / 'pubspec.lock').write_text('SDK lock fixture\n')
        for source in (REPO / 'libraries/dart').rglob('pubspec.*'):
            if source.name not in ('pubspec.yaml', 'pubspec.lock'):
                continue
            target = root / source.relative_to(REPO)
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(source.read_bytes())
        example = root / 'libraries/dart/flutter/example'
        for source in (REPO / 'libraries/dart/flutter/example').rglob('generated_*'):
            target = root / source.relative_to(REPO)
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(source.read_bytes())
        manifest = yaml.safe_load((example / 'pubspec.yaml').read_text())
        lock = yaml.safe_load((example / 'pubspec.lock').read_text())
        for package in ('thinkthen_dart', 'thinkthen_flutter'):
            path = manifest['dependencies'][package]['path']
            assert lock['packages'][package]['source'] == 'path'
            assert lock['packages'][package]['description']['path'] == path
            assert (example / path / 'pubspec.yaml').is_file()
        # Checked-in Linux generation must agree with local plugin declarations.
        ffi_plugins = []
        for dependency in manifest['dependencies'].values():
            if not isinstance(dependency, dict) or 'path' not in dependency:
                continue
            package = yaml.safe_load((example / dependency['path'] / 'pubspec.yaml').read_text())
            linux = package.get('flutter', {}).get('plugin', {}).get('platforms', {}).get('linux', {})
            if linux.get('ffiPlugin'):
                ffi_plugins.append(package['name'])
        generated = example / 'linux/flutter/generated_plugins.cmake'
        listed = generated.read_text().split('list(APPEND FLUTTER_FFI_PLUGIN_LIST\n', 1)[1].split(')', 1)[0]
        assert listed.split() == sorted(ffi_plugins), 'Generated Linux FFI plugins differ from manifests'
        subprocess.run(['git', 'init', '-q', str(root)], check=True)
        subprocess.run(['git', 'add', 'libraries'], cwd=root, check=True)
        subprocess.run(['git', '-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid',
                        'commit', '-qm', 'Tracked consumer fixture'], cwd=root, check=True)
        assert not (root / 'libraries/dart/flutter/pubspec.lock').exists()
        required = set(yaml.safe_load((root / 'libraries/dart/pubspec.lock').read_text())['packages'])
        resolver = root / 'resolver'
        resolver.write_text(f'#!{sys.executable}\n' +
                            'from pathlib import Path\nimport sys\nimport yaml\n' +
                            'assert sys.argv[1:] == ["pub", "get", "--enforce-lockfile"]\n' +
                            'assert Path("pubspec.lock").is_file(), "Cannot enforce a missing pubspec.lock"\n' +
                            # Check local sources and closure entries; the real SDK owns version solving.
                            'if Path("pubspec.yaml").is_file():\n' +
                            '    lock = yaml.safe_load(Path("pubspec.lock").read_text())["packages"]\n' +
                            f'    assert set({sorted(required)!r}).issubset(lock), "Incomplete dependency closure"\n' +
                            '    override = Path("pubspec_overrides.yaml")\n' +
                            '    overrides = yaml.safe_load(override.read_text()).get("dependency_overrides", {}) if override.exists() else {}\n' +
                            '    pending, seen = [Path.cwd()], set()\n' +
                            '    while pending:\n' +
                            '        project = pending.pop().resolve()\n' +
                            '        if project in seen: continue\n' +
                            '        seen.add(project)\n' +
                            '        manifest = yaml.safe_load((project / "pubspec.yaml").read_text())\n' +
                            '        deps = dict(manifest.get("dependencies", {}))\n' +
                            '        if project == Path.cwd(): deps.update(manifest.get("dev_dependencies", {}))\n' +
                            '        for name, declared in deps.items():\n' +
                            '            spec = overrides.get(name, declared)\n' +
                            '            source = next((kind for kind in ("path", "sdk", "git") if isinstance(spec, dict) and kind in spec), "hosted")\n' +
                            '            assert lock[name]["source"] == source, "Dependency source conflict: " + name\n' +
                            '            if source == "path":\n' +
                            '                base = Path.cwd() if name in overrides else project\n' +
                            '                target = (base / spec["path"]).resolve()\n' +
                            '                assert target == (Path.cwd() / lock[name]["description"]["path"]).resolve(), "Locked path conflict: " + name\n' +
                            '                assert yaml.safe_load((target / "pubspec.yaml").read_text())["name"] == name\n' +
                            '                pending.append(target)\n' +
                            'with open(sys.argv[0] + ".calls", "a") as calls:\n' +
                            '    calls.write(str(Path.cwd()) + "\\n")\n')
        resolver.chmod(0o755)
        before = {path: path.read_bytes() for path in root.rglob('pubspec.*')}
        env = {'PATH': os.environ['PATH'], 'HOME': str(root / 'home'),
               'dart': str(resolver), 'flutter': str(resolver)}
        result = subprocess.run(['/bin/sh', '-ec', command], cwd=root, env=env,
                                capture_output=True, text=True, timeout=5)
        assert result.returncode == 0, (result.returncode, result.stdout, result.stderr)
        calls = Path(str(resolver) + '.calls').read_text().splitlines()
        expected = [str(tools), str(root / 'libraries/dart'),
                    str(root / 'libraries/dart/checks/session_consumer'), str(example)]
        assert calls == expected, (calls, expected)
        assert {path: path.read_bytes() for path in root.rglob('pubspec.*')} == before
        override = example / 'pubspec_overrides.yaml'
        assert not override.exists()
        original = generated.read_bytes()
        generated.write_text(generated.read_text() + '# Changed generation\n')
        result = subprocess.run(['/bin/sh', '-ec', command], cwd=root, env=env,
                                capture_output=True, text=True, timeout=5)
        assert result.returncode == 1 and 'generated_plugins.cmake' in result.stdout, result
        generated.write_bytes(original)
        for broken in (command.replace('dependency_overrides:', 'ignored_overrides:'),
                       command.replace('path: ../..', 'path: ..')):
            result = subprocess.run(['/bin/sh', '-ec', broken], cwd=root, env=env,
                                    capture_output=True, text=True, timeout=5)
            assert result.returncode != 0, result.stdout
            assert 'conflict: thinkthen_dart' in result.stderr, result.stderr
            assert not override.exists(), 'Failed acquisition retained its owned override'
            assert {path: path.read_bytes() for path in root.rglob('pubspec.*')} == before
        # Every retained consumer lock must carry the local Dart package's full closure.
        for consumer in (example, root / 'libraries/dart/checks/session_consumer'):
            locked = consumer / 'pubspec.lock'
            original = locked.read_bytes()
            incomplete = yaml.safe_load(original)
            incomplete['packages'].pop('record_use')
            locked.write_text(yaml.safe_dump(incomplete))
            result = subprocess.run(['/bin/sh', '-ec', command], cwd=root, env=env,
                                    capture_output=True, text=True, timeout=5)
            assert result.returncode != 0 and 'Incomplete dependency closure' in result.stderr, result.stderr
            assert not override.exists(), 'Failed acquisition retained its owned override'
            locked.write_bytes(original)
            assert {path: path.read_bytes() for path in root.rglob('pubspec.*')} == before
        override.write_text('owned by another caller\n')
        result = subprocess.run(['/bin/sh', '-ec', command], cwd=root, env=env,
                                capture_output=True, text=True, timeout=5)
        assert result.returncode != 0
        assert override.read_text() == 'owned by another caller\n'
        override.unlink()
        caller_target = root / 'caller-owned.yaml'
        override.symlink_to(caller_target)
        result = subprocess.run(['/bin/sh', '-ec', command], cwd=root, env=env,
                                capture_output=True, text=True, timeout=5)
        assert result.returncode != 0, 'Acquisition accepted a caller-owned dangling override'
        assert override.is_symlink(), 'Acquisition removed a caller-owned override'
        assert not caller_target.exists(), 'Acquisition wrote through a caller-owned override'
        override.unlink()


def parameter_cases():
    head = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=REPO, text=True).strip()
    guard = REPO / 'sdlc/scripts/hosted-candidate.py'
    cases = [(head, head, 0), (head, '0' * 40, 1), ('0' * 40, '0' * 40, 1)]
    cases += [(value, head, 1) for value in ('', '--help', head.upper(), head[:39],
                                           head + '\n', '$(touch /tmp/hosted-plant)', 'x; exit 0')]
    for candidate, dispatch, wanted in cases:
        result = subprocess.run([sys.executable, str(guard), 'checkout', candidate, dispatch],
                                cwd=REPO, capture_output=True, text=True, timeout=5)
        assert result.returncode == wanted, (candidate, result.stderr)
        if wanted:
            assert result.stderr.startswith('hosted-candidate: '), result.stderr
    with tempfile.TemporaryDirectory(prefix='thinkthen-apple-workflow-') as temporary:
        root = Path(temporary)
        for name in ('sdlc/scripts', 'crates/thinkthen', 'libraries/swift', 'libraries/objective-c', 'bin', 'packages'):
            (root / name).mkdir(parents=True)
        for name in ('release-workflow', 'scratch.sh', 'fetch.sh', 'verdict.sh', 'hosted-candidate.py'):
            (root / 'sdlc/scripts' / name).write_bytes((REPO / 'sdlc/scripts' / name).read_bytes())
        (root / 'crates/thinkthen/Cargo.toml').write_text('version = "0.2.0"\n')
        for tool, body in [('uname', 'case $1 in -s) echo Darwin ;; *) echo arm64 ;; esac'),
                           ('git', f'echo {head}'), ('shasum', 'exit 0'),
                           ('tar', f'echo source_commit={head}')]:
            path = root / 'bin' / tool
            path.write_text('#!/bin/sh\n' + body + '\n')
            path.chmod(0o755)
        env = {'PATH': f'{root}/bin:{os.defpath}', 'HOME': str(root), 'LANG': 'C.UTF-8'}
        # Exercise the workflow caller with the real archived, non-executable helper.
        archived = subprocess.check_output(['git', 'archive', 'HEAD',
                                            'libraries/swift/build-xcframework.sh'], cwd=REPO)
        with tarfile.open(fileobj=io.BytesIO(archived)) as source:
            source.extractall(root, filter='data')
        release = yaml.safe_load((REPO / '.github/workflows/release.yml').read_text())
        apple = release['jobs']['apple-packages']
        assembly = next(step['run'] for step in apple['steps'] if step.get('name') ==
                        'Assemble Swift and Foundation from both captured C packages')
        command = next(line for line in assembly.splitlines() if 'build-xcframework.sh' in line)
        (root / 'bin/uname').write_text('#!/bin/sh\necho Linux\n')
        for cell in apple['strategy']['matrix']['include']:
            result = subprocess.run(['/bin/sh', '-c', command], cwd=root,
                                    env={**env, 'TARGET': cell['target']},
                                    capture_output=True, text=True, timeout=5)
            assert result.returncode == 2 and 'Apple native tools are required' in result.stderr, (
                cell['target'], result.returncode, result.stderr)
        (root / 'bin/uname').write_text('#!/bin/sh\ncase $1 in -s) echo Darwin ;; *) echo arm64 ;; esac\n')
        for status in (0, 77, 1):
            (root / 'libraries/swift/check.sh').write_text(f'echo selected-swift\nexit {status}\n')
            (root / 'libraries/objective-c/check.sh').write_text('echo WRONG_LANGUAGE\nexit 1\n')
            result = subprocess.run(['/bin/sh', str(root / 'sdlc/scripts/release-workflow'), 'apple-check',
                                     'aarch64-apple-darwin', str(root / 'packages'), 'swift', head],
                                    cwd=root, env=env, capture_output=True, text=True, timeout=5)
            assert result.returncode == (0 if status == 0 else 1), result.stderr
            assert 'selected-swift' in result.stdout and 'WRONG_LANGUAGE' not in result.stdout
            assert ('surfaces: pass' if status == 0 else 'surfaces: FAIL') in result.stdout
        for language in ('all', 'swift objective-c', '../swift', 'swift; exit 0'):
            result = subprocess.run(['/bin/sh', str(root / 'sdlc/scripts/release-workflow'), 'apple-check',
                                     'aarch64-apple-darwin', str(root / 'packages'), language, head],
                                    cwd=root, env=env, capture_output=True, text=True, timeout=5)
            assert result.returncode == 1 and 'select exactly one Apple language' in result.stderr
            assert 'selected-swift' not in result.stdout


workflow_cases()
linux_tool_destination_case()
linux_nested_tools_case()
linux_coverage_destination_case()
flutter_locked_setup_case()
parameter_cases()
print('hosted workflows: standard runners, exact commits, installed Apple routing and skipped checks hold')
