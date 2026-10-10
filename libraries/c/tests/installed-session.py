"""Run the existing typed owner case against an extracted prebuilt C archive."""
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile

REPO = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(REPO / 'conformance/children'))
sys.path.insert(0, str(REPO / 'libraries/zig/Tests'))
from children import child_env
from backend import Backend

archive = Path(sys.argv[1]).resolve()
with tempfile.TemporaryDirectory(prefix='installed-c-', dir=REPO / 'libraries/c/target') as temporary:
    root = Path(temporary)
    native = root / 'native'
    native.mkdir()
    with tarfile.open(archive) as package:
        package.extractall(native, filter='data')
    home = root / 'home'
    home.mkdir()
    barrier = root / 'barrier'
    barrier.mkdir()
    backend = Backend(barrier)
    try:
        env = child_env(home=home, THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.server_port}/generic/v1',
                        THINKTHEN_API_KEY='tt-canary-273', THINKTHEN_CACHE=str(root / 'cache'))
        env['PKG_CONFIG_PATH'] = str(native / 'lib/pkgconfig')
        flags = subprocess.check_output(['pkg-config', '--cflags', '--libs', '--static', 'thinkthen'], env=env, text=True).split()
        # Select the archive explicitly so the proof needs no runtime library path.
        flags = [str(native / 'lib/libthinkthen.a') if flag == '-lthinkthen' else flag for flag in flags]
        for source in ('tests/c/session.c', 'examples/session.c'):
            binary = root / Path(source).stem
            subprocess.run([shutil.which('cc'), '-std=c11', '-D_GNU_SOURCE', '-Wall', '-Wextra', '-Werror',
                            '-pthread', '-I', str(REPO / 'libraries/c/tests/c'),
                            str(REPO / 'libraries/c' / source), *flags, '-o', str(binary)],
                           env=env, check=True, timeout=60)
            result = subprocess.run([str(binary)], env=env, capture_output=True, text=True, timeout=5)
            assert result.returncode == 0, result.stderr
            if source.startswith('examples/'):
                assert result.stdout == 'records=1 requests=1\n', result.stdout
        assert len(backend.arrivals) == 2, backend.arrivals
        print('installed C: typed owner case and documented caller PASS')
    finally:
        backend.close()

    from installed_cases import run
    run(native, root)
