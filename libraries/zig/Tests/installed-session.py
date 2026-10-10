"""Build one installed Zig session caller using only its bundled native engine."""
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
import tomllib

REPO = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(REPO / 'conformance/children'))
from children import child_env
from backend import Backend

zig = Path(shutil.which('zig')).resolve()
archive = Path(sys.argv[1]).resolve()
root = REPO / 'libraries/zig/target'
root.mkdir(exist_ok=True)
with tempfile.TemporaryDirectory(prefix='installed-session-', dir=root) as temporary:
    trial = Path(temporary)
    package, project, home = (trial / name for name in ('package', 'project', 'home'))
    for directory in (package, project, home):
        directory.mkdir()
    with tarfile.open(archive) as source:
        source.extractall(package, filter='data')
    tests = REPO / 'libraries/zig/Tests'
    shutil.copy2(tests / 'session-build.zig', project / 'build.zig')
    shutil.copy2(tests / 'session.zig', project / 'session.zig')
    (project / 'build.zig.zon').write_text((tests / 'build.zig.zon').read_text().replace('.path = "../"', '.path = "../package"'))
    backend = Backend()
    try:
        env = child_env(home=home, ZIG_GLOBAL_CACHE_DIR=str(root / 'cache'),
                        THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.server_port}/generic/v1',
                        THINKTHEN_API_KEY='tt-canary-273', THINKTHEN_CACHE=str(home / 'native-cache'))
        subprocess.run([str(zig), 'build', '-j2'], cwd=project, env=env, check=True, timeout=120)
        preview_env = child_env(home=home, THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.server_port}/generic/v1')
        preview = subprocess.run([str(project / 'zig-out/bin/session'), 'plan'], cwd=project, env=preview_env, capture_output=True, text=True, timeout=5)
        assert preview.returncode == 0, preview.stderr
        assert preview.stderr == 'installed Zig plan PASS\n', preview.stderr
        assert len(backend.arrivals) == 0, backend.arrivals
        result = subprocess.run([str(project / 'zig-out/bin/session')], cwd=project, env=env,
                                capture_output=True, text=True, timeout=5)
        assert result.returncode == 0, result.stderr
        assert result.stderr == 'installed Zig session PASS\n', result.stderr
        assert len(backend.arrivals) == 1, backend.arrivals
        failed = subprocess.run([str(project / 'zig-out/bin/session'), 'failure'], cwd=project, env=env,
                                capture_output=True, text=True, timeout=5)
        assert failed.returncode == 0, failed.stderr
        assert failed.stderr == 'installed Zig failure PASS\n', failed.stderr
        assert len(backend.arrivals) == 2, backend.arrivals
        version = tomllib.loads((REPO / 'crates/thinkthen/Cargo.toml').read_text())['package']['version']
        assert backend.user_agents == [f'thinkthen/{version} (zig)'] * 2, backend.user_agents
        print('installed Zig: bundled static native session PASS')
    finally:
        backend.close()
