"""Run the JSON session and plan cases against an extracted prebuilt C archive."""
from pathlib import Path
import json
import os
import shutil
import subprocess
import sys
import tarfile
import tempfile

REPO = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(REPO / 'conformance/children'))
from children import child_env
from backend import Backend
from installed_cases import run, static_flags

archive = Path(sys.argv[1]).resolve()
(REPO / 'libraries/c/target').mkdir(parents=True, exist_ok=True)
with tempfile.TemporaryDirectory(prefix='installed-c-', dir=REPO / 'libraries/c/target') as temporary:
    root = Path(os.environ.get('THINKTHEN_COVERAGE_STAGE', temporary))
    root.mkdir(parents=True, exist_ok=True)
    native = root / 'native'
    native.mkdir()
    with tarfile.open(archive) as package:
        package.extractall(native, filter='data')
    home = root / 'home'
    home.mkdir()
    barrier = root / 'barrier'
    barrier.mkdir()
    backend = Backend(barrier, key='tt-canary-273', arrivals='wire', retry_ms=True, concurrent=True)
    try:
        env = child_env(home=home, THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.server_port}/generic/v1',
                        THINKTHEN_API_KEY='tt-canary-273', THINKTHEN_CACHE=str(root / 'cache'))
        flags = static_flags(native, env)
        for source in ('tests/c/session.c', 'examples/session.c', 'tests/c/request_preview.c'):
            binary = root / {'tests/c/session.c': 'owner-session',
                             'examples/session.c': 'example-session',
                             'tests/c/request_preview.c': 'request-preview'}[source]
            subprocess.run([shutil.which('cc'), '-std=c11', '-D_GNU_SOURCE', '-Wall', '-Wextra', '-Werror',
                            '-pthread', '-I', str(REPO / 'libraries/c/tests/c'),
                            str(REPO / 'libraries/c' / source), *flags, '-o', str(binary)],
                           env=env, check=True, timeout=60)
            result = subprocess.run([str(binary)], env=env, capture_output=True, text=True, timeout=5)
            assert result.returncode == 0, result.stderr
            if source == 'tests/c/session.c':
                before = len(backend.arrivals)
                collected = subprocess.run([str(binary)], env={**env, 'SESSION_COLLECTION': '1'},
                                           capture_output=True, text=True, timeout=5)
                assert collected.returncode == 0, collected.stderr
                assert len(backend.arrivals) == before + 2, 'JSON find and relate send count'
            if source == 'tests/c/request_preview.c':
                plans = [json.loads(line) for line in result.stdout.splitlines()]
                assert len(plans) == 2 and plans[0] == plans[1], plans
                assert plans[0]['records'] == 1 and plans[0]['requests'] == 1, plans
                assert len(backend.arrivals) == 4, 'JSON preview sent a request'
            if source.startswith('examples/'):
                assert result.stdout == 'records=1 requests=1\n', result.stdout
        assert len(backend.arrivals) == 4, backend.arrivals
        print('installed C: JSON session, plan and documented caller PASS')
    finally:
        backend.close()

    run(native, root)
    from session_wait import run as session_wait
    session_wait(native, root)
