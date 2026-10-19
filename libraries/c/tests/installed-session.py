"""Run the existing typed owner case against an extracted prebuilt C archive."""
from pathlib import Path
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
        for source in ('tests/c/session.c', 'examples/session.c'):
            binary = root / ('owner-session' if source.startswith('tests/') else 'example-session')
            subprocess.run([shutil.which('cc'), '-std=c11', '-D_GNU_SOURCE', '-Wall', '-Wextra', '-Werror',
                            '-pthread', '-I', str(REPO / 'libraries/c/tests/c'),
                            str(REPO / 'libraries/c' / source), *flags, '-o', str(binary)],
                           env=env, check=True, timeout=60)
            result = subprocess.run([str(binary)], env=env, capture_output=True, text=True, timeout=5)
            assert result.returncode == 0, result.stderr
            if source == 'tests/c/session.c':
                before = len(backend.arrivals)
                refused = subprocess.run([str(binary)], env={**env, 'SESSION_NAMED': '1', 'SESSION_NAMED_REFUSALS': '1'},
                                         capture_output=True, text=True, timeout=5)
                assert refused.returncode == 0, refused.stderr
                assert len(backend.arrivals) == before, 'named refusals sent a request'
                named = subprocess.run([str(binary)], env={**env, 'SESSION_NAMED': '1'},
                                       capture_output=True, text=True, timeout=5)
                assert named.returncode == 0, named.stderr
                assert len(backend.arrivals) == before + 2, 'named find and relate send count'
            if source.startswith('examples/'):
                assert result.stdout == 'records=1 requests=1\n', result.stdout
        assert len(backend.arrivals) == 4, backend.arrivals
        print('installed C: typed owner case and documented caller PASS')
    finally:
        backend.close()

    run(native, root)
    from session_wait import run as session_wait
    session_wait(native, root)
