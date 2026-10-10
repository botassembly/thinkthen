"""Exercise a source archive through its bundled native engine only."""
import os
from pathlib import Path
import subprocess
import sys
import tarfile
import tempfile
ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
archive = Path(os.environ['THINKTHEN_ARTIFACT']).resolve(strict=True)
with tempfile.TemporaryDirectory(prefix='package-', dir=ROOT / 'libraries/zig/target') as folder:
    scratch = Path(folder)
    package = scratch / 'package'
    package.mkdir()
    with tarfile.open(archive) as source: source.extractall(package, filter='data')
    subprocess.run([sys.executable, str(ROOT / 'libraries/zig/Tests/installed-session.py'), str(archive)],
                   env=child_env(home=scratch, THINKTHEN_ZIG=os.environ.get('THINKTHEN_ZIG', 'zig')), check=True)
    subprocess.run([sys.executable, str(ROOT / 'libraries/zig/Tests/complete_parity.py')],
                   env=child_env(home=scratch, THINKTHEN_ZIG=os.environ.get('THINKTHEN_ZIG', 'zig'),
                                 THINKTHEN_TEST_PROFILE=os.environ.get('THINKTHEN_TEST_PROFILE', 'routine'),
                                 THINKTHEN_ZIG_CASES=os.environ.get('THINKTHEN_ZIG_CASES', ''),
                                 THINKTHEN_ARTIFACT=str(archive), THINKTHEN_PARITY_PACKAGE=str(package)), check=True)
