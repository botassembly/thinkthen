"""Run the installed typed Zig session against shared routine or release cases."""
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
ROOT = Path(__file__).resolve().parents[3]
PACKAGE = Path(os.environ.get('THINKTHEN_PARITY_PACKAGE', ROOT / 'libraries/zig')).resolve(strict=True)
NATIVE = Path(os.environ.get('THINKTHEN_NATIVE_ROOT', PACKAGE / 'native/x86_64-unknown-linux-gnu')).resolve(strict=True)
if os.environ.get('THINKTHEN_ARTIFACT') and not os.environ.get('THINKTHEN_PARITY_PACKAGE'):
    raise ValueError('installed zig parity requires its extracted package')
sys.path[:0] = [str(ROOT / 'conformance/children')]
from children import child_env
import session_cases
with tempfile.TemporaryDirectory(prefix='installed-session-', dir=ROOT / 'libraries/zig/target') as folder:
    scratch = Path(folder)
    for name in ('session_consumer.zig', 'request_fixture.zig'):
        shutil.copy2(Path(__file__).parent / name, scratch / name)
    binary = scratch / 'consumer'
    env = child_env(home=scratch, ZIG_GLOBAL_CACHE_DIR=str(ROOT / 'libraries/zig/target/cache'))
    subprocess.run([sys.executable, str(ROOT / 'sdlc/scripts/build-admission.py'), '--lock', '/tmp/thinkthen-heavy.lock', '--timeout', '900', '--', os.environ.get('THINKTHEN_ZIG', 'zig'), 'build-exe', '-j2', '-fllvm', '-flld',
                    '--cache-dir', str(ROOT / 'libraries/zig/target/scratch/parity-cache'),
                    str(NATIVE / 'lib/libthinkthen.a'), '--dep', 'thinkthen', '-Mroot=' + str(scratch / 'session_consumer.zig'),
                    '-I', str(NATIVE / 'include'), '-Mthinkthen=' + str(PACKAGE / 'src/thinkthen.zig'),
                    '-lc', '-lgcc_s', '-lutil', '-lrt', '-lpthread', '-lm', '-ldl', '-femit-bin=' + str(binary)],
                   env=env, check=True)
    if os.environ.get('THINKTHEN_COVERAGE_STAGE'):
        stage = Path(os.environ['THINKTHEN_COVERAGE_STAGE']); stage.mkdir(parents=True, exist_ok=True)
        shutil.copy2(binary, stage / 'consumer')
    session_cases.native_cases(binary, consumer='zig')
    if not os.environ.get('THINKTHEN_ZIG_CASES') and os.environ.get('THINKTHEN_TEST_PROFILE', 'routine') != 'full':
        os.environ['THINKTHEN_ZIG_CASES'] = 'library-call-result'
        session_cases.native_cases(binary, consumer='zig')
