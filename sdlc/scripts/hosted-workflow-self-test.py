#!/usr/bin/env python3
"""Plant hosted candidate routing and parameter failures at the actual entry points."""
from copy import deepcopy
import os
from pathlib import Path
import subprocess
import sys
import tempfile

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
parameter_cases()
print('hosted workflows: standard runners, exact commits, installed Apple routing and skipped checks hold')
