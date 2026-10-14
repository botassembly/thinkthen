#!/usr/bin/env python3
"""Plant hosted candidate routing and parameter failures at the actual entry points."""
from copy import deepcopy
import os
from pathlib import Path
import runpy
import subprocess
import sys
import tempfile

import yaml

REPO = Path(__file__).resolve().parents[2]
rules = runpy.run_path(str(REPO / 'sdlc/scripts/hosted-workflow.py'))['rules']


def workflow_cases():
    release = yaml.safe_load((REPO / '.github/workflows/release.yml').read_text())
    windows = yaml.safe_load((REPO / '.github/workflows/windows.yml').read_text())
    assert rules('release.yml', release) == []
    assert rules('windows.yml', windows) == []
    for defect in ('paid', 'matrix-paid', 'skip-apple', 'mask-apple', 'wrong-sha', 'missing-language'):
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
        else:
            proof['run'] = '\n'.join(proof['run'].splitlines()[:-1])
        assert rules('release.yml', doc), defect
    for defect in ('skip', 'mask', 'wrong-sha', 'late'):
        doc = deepcopy(windows)
        steps = doc['jobs']['stage-0']['steps']
        guard = next(step for step in steps if step.get('name') == 'Require the exact candidate commit')
        if defect == 'skip':
            guard['if'] = 'false'
        elif defect == 'mask':
            guard['continue-on-error'] = True
        elif defect == 'wrong-sha':
            guard['env']['CANDIDATE_SHA'] = '${{ github.sha }}'
        else:
            steps.remove(guard)
            steps.append(guard)
        assert rules('windows.yml', doc), defect


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
