#!/usr/bin/env python3
"""Exercise check outcomes through the real checkout and smoke entrypoint."""
import argparse
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT))
from conformance.source_fixtures import fixture_tools, fixture_tool_groups


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
    assert unknown.returncode == 2 and 'unknown test profile' in unknown.stdout, unknown
    missing = run(['sh', 'libraries/go/check.sh'], dict(os.environ, THINKTHEN_ARTIFACT='/absent/package'))
    assert missing.returncode == 1 and 'missing input THINKTHEN_ARTIFACT' in missing.stdout, missing
    if sys.platform == 'darwin':
        go = run(['sh', 'libraries/go/check.sh'])
        assert go.returncode == 77 and 'sdlc/tools.json libraries.go =' in go.stdout, go
        caller = run([*helper, 'verdict', 'libraries/go', str(go.returncode)])
        assert caller.returncode == 0 and 'unsupported libraries/go' in caller.stdout, caller
        print(go.stdout + caller.stdout, end='')
    if args.smoke_binaries:
        with tempfile.TemporaryDirectory(prefix='thinkthen-hidden-tools-') as temporary:
            path = Path(temporary)
            fixture_tools(ROOT, path, ('test', 'python'), exclude=('uv',))
            # Inject loss after resolution: a fresh resolver may legitimately
            # restore a pinned tool. Run the actual smoke and binding checks
            # with the resolved-group marker and remove only their tool paths.
            env = dict(os.environ, PATH=str(path), THINKTHEN_RESOLVED_TOOLS=fixture_tool_groups(ROOT))
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
