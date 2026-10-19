#!/usr/bin/env python3
"""Exercise check outcomes through the real checkout and smoke entrypoint."""
import argparse
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]


def run(args, env=None):
    return subprocess.run(args, cwd=ROOT, env=env, text=True, capture_output=True, timeout=90)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--smoke-binaries', type=Path)
    args = parser.parse_args()
    helper = [sys.executable, 'sdlc/scripts/check_verdict.py']
    required = run(['sh', '-c', 'REPO=$PWD; . sdlc/scripts/verdict.sh; verdict "required check" 77'])
    assert required.returncode == 1 and 'FAIL required check (not run)' in required.stdout, required
    env = dict(os.environ, THINKTHEN_TEST_PROFILE='unknown')
    unknown = run(['sh', 'libraries/go/check.sh'], env)
    assert unknown.returncode == 2 and unknown.stderr.strip() == 'go: unknown profile', unknown
    missing = run(['sh', 'libraries/go/check.sh'], dict(os.environ, THINKTHEN_ARTIFACT='/absent/package'))
    assert missing.returncode == 1 and 'missing input THINKTHEN_ARTIFACT' in missing.stdout, missing
    with tempfile.TemporaryDirectory(prefix='thinkthen-check-inputs-') as temporary:
        directory = Path(temporary)
        package = directory / 'package'
        package.touch()
        inputs = [('libraries/go', name, False) for name in
                  ('THINKTHEN_ARTIFACT', 'THINKTHEN_C_ARTIFACT', 'THINKTHEN_COMMAND')]
        inputs += [(f'databases/{surface}', name, False)
                   for surface in ('duckdb', 'sqlite', 'postgresql')
                   for name in ('THINKTHEN_ARTIFACT', 'THINKTHEN_C_ARTIFACT', 'THINKTHEN_COMMAND')]
        inputs += [('libraries/dart', 'THINKTHEN_DART_ARTIFACT', False),
                   ('libraries/cobol', 'THINKTHEN_C_HEADER', False),
                   ('libraries/cobol', 'THINKTHEN_C_LIBRARY', False),
                   ('libraries/php', 'THINKTHEN_COMPLETE_LIBRARY', False)]
        inputs += [(f'libraries/{name}', 'THINKTHEN_PARITY_PREPARED', True)
                   for name in ('python', 'jvm', 'typescript', 'dart')]
        for surface, name, is_directory in inputs:
            baseline = run([*helper, 'platform', surface])
            assert baseline.returncode in (0, 77), baseline
            for value in (directory / 'missing', package if is_directory else directory):
                invalid = run(['sh', f'{surface}/check.sh'], dict(os.environ, **{name: str(value)}))
                assert invalid.returncode == 1 and f'missing input' in invalid.stdout and name in invalid.stdout, invalid
            valid = run([*helper, 'platform', surface],
                        dict(os.environ, **{name: str(directory if is_directory else package)}))
            assert valid.returncode == baseline.returncode, valid
    if sys.platform == 'darwin':
        go = run(['sh', 'libraries/go/check.sh'])
        assert go.returncode == 77 and 'sdlc/tools.json libraries.go =' in go.stdout, go
        caller = run([*helper, 'verdict', 'libraries/go', str(go.returncode)])
        assert caller.returncode == 0 and 'unsupported libraries/go' in caller.stdout, caller
        print(go.stdout + caller.stdout, end='')
    if args.smoke_binaries:
        with tempfile.TemporaryDirectory(prefix='thinkthen-hidden-tools-') as temporary:
            path = Path(temporary)
            for name in ('awk', 'sh', 'dirname', 'python3', 'jq', 'mktemp', 'mkdir', 'mkfifo',
                         'head', 'tail', 'sed', 'uname', 'ln', 'rm', 'sleep'):
                source = shutil.which(name)
                assert source, name
                (path / name).symlink_to(source)
            # Inject loss after resolution: a fresh resolver may legitimately
            # restore a pinned tool. Run the actual smoke and binding checks
            # with the resolved-group marker and remove only their tool paths.
            env = dict(os.environ, PATH=str(path), THINKTHEN_RESOLVED_TOOLS='python')
            smoke = run(['sh', 'sdlc/scripts/smoke', '--one', str(args.smoke_binaries.resolve()),
                         'libraries/python'], env)
            print(smoke.stdout + smoke.stderr, end='')
            assert smoke.returncode != 0, smoke
            assert 'smoke: FAIL libraries/python' in smoke.stdout, smoke
            assert 'libraries/python: not run: no uv' in smoke.stdout, smoke
        if sys.platform == 'darwin':
            go_smoke = run(['sh', 'sdlc/scripts/smoke', '--one', str(args.smoke_binaries.resolve()),
                            'libraries/go'])
            print(go_smoke.stdout + go_smoke.stderr, end='')
            assert go_smoke.returncode == 0, go_smoke
            assert 'smoke: unsupported libraries/go' in go_smoke.stdout, go_smoke
    print('check verdict: required not-run fails; declared unsupported platform passes')


if __name__ == '__main__':
    main()
