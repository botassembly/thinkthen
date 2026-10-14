"""Build one installed Zig session caller using only its bundled native engine."""
import os
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
from usage import run_usage
from backend import Backend

zig = Path(shutil.which(os.environ.get('THINKTHEN_ZIG', 'zig'))).resolve()
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
    fragment = (REPO / 'libraries/zig/README-PUBLIC.md').read_text().split('```zig\n', 1)[1].split('```', 1)[0]
    (project / 'public.zig').write_text('const std = @import("std");\nconst tt = @import("thinkthen");\npub fn main() !void {\nvar gpa = std.heap.DebugAllocator(.{}){};\ndefer std.debug.assert(gpa.deinit() == .ok);\nconst a = gpa.allocator();\nvar engine = switch (try tt.Engine.init(a)) { .ok => |v| v, .failed => |f| { tt.releaseFailure(a, f); return error.EngineBuild; } };\ndefer engine.deinit();\n' + fragment + '}\n')
    backend = Backend(key='tt-canary-273', arrivals='wire', retry_ms=True, concurrent=True)
    try:
        env = child_env(home=home, ZIG_GLOBAL_CACHE_DIR=str(root / 'cache'),
                        THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.server_port}/generic/v1',
                        THINKTHEN_API_KEY='tt-canary-273', THINKTHEN_CACHE=str(home / 'native-cache'))
        subprocess.run([sys.executable, str(REPO / 'sdlc/scripts/build-admission.py'), '--lock', '/tmp/thinkthen-heavy.lock', '--', str(zig), 'build', '-j2'], cwd=project, env=env, check=True, timeout=120)
        if os.environ.get('THINKTHEN_COVERAGE_STAGE'):
            stage = Path(os.environ['THINKTHEN_COVERAGE_STAGE']); stage.mkdir(parents=True, exist_ok=True)
            for name in ('session', 'example'):
                shutil.copy2(project / 'zig-out/bin' / name, stage / name)
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
        before = len(backend.arrivals)
        example = subprocess.run([str(project / 'zig-out/bin/example')], cwd=project, env=env, capture_output=True, text=True, timeout=5)
        assert example.returncode == 0 and example.stderr == 'answer=true\nrecords=1 requests=1\n', example.stderr
        assert len(backend.arrivals) == before + 1, backend.arrivals
        before = len(backend.arrivals)
        helpers = subprocess.run([str(project / 'zig-out/bin/session'), 'helpers'], cwd=project, env=env, capture_output=True, text=True, timeout=5)
        assert helpers.returncode == 0 and helpers.stderr == 'installed Zig helpers PASS\n', helpers.stderr
        assert len(backend.arrivals) == before + 2, backend.arrivals
        before = len(backend.arrivals)
        public = subprocess.run([str(project / 'zig-out/bin/public-example')], cwd=project, env=env, capture_output=True, text=True, timeout=5)
        assert public.returncode == 0 and public.stderr == 'refund=true records=3\n', public.stderr
        assert len(backend.arrivals) == before + 3, backend.arrivals
        for mode in ('usage-written', 'usage-failed', 'usage-disabled'):
            mode_home = home / mode
            mode_home.mkdir()
            mode_env = child_env(home=mode_home, THINKTHEN_BASE_URL=env['THINKTHEN_BASE_URL'], THINKTHEN_API_KEY='tt-canary-273')
            if mode == 'usage-disabled': mode_env.update(HOME='relative', XDG_STATE_HOME='')
            before = len(backend.arrivals)
            result = run_usage([str(project / 'zig-out/bin/session'), mode], cwd=project, env=mode_env, state=mode_home / 'state/thinkthen', mode=mode.removeprefix('usage-'), timeout=15)
            assert result.returncode == 0 and result.stdout == 'USAGE_PASS\n' and not result.stderr, (mode, result.stdout, result.stderr)
            assert len(backend.arrivals) - before == 1, backend.arrivals
            print('installed Zig', mode, 'PASS retained typed row/facts/status')
        print('installed Zig: bundled static native session PASS')
    finally:
        backend.close()
