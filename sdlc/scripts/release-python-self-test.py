#!/usr/bin/env python3
"""Exercise release interpreter selection and community builds from older callers."""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile

REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO))
from conformance.children.children import child_env
from sdlc.scripts.tools import commands, fixture_path

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from conformance.children.children import fixture_run


def community():
    older = sys.argv[1] if len(sys.argv) > 1 else None
    with tempfile.TemporaryDirectory(prefix='thinkthen-community-python-') as temporary:
        root = Path(temporary)
        source_root = root / 'source'
        community = source_root / 'databases/duckdb/community'
        community.mkdir(parents=True)
        for source in (REPO / 'databases/duckdb/community').glob('*.py'):
            shutil.copyfile(source, community / source.name)
        shutil.copyfile(REPO / 'rust-toolchain.toml', source_root / 'rust-toolchain.toml')
        archive_path = root / 'source.tar'
        with tarfile.open(archive_path, 'w') as archive:
            for path in sorted(source_root.rglob('*')):
                archive.add(path, arcname=str(path.relative_to(source_root)), recursive=False)
        tools = root / 'tools'
        tools.mkdir()
        # Replace compilers at the process boundary; use the real helpers and manifest.
        probe = tools / 'probe'
        probe.write_text(f'''#!{sys.executable}
import json, os, pathlib, sys
with open({str(root / 'calls')!r}, 'a') as output:
    output.write(json.dumps([pathlib.Path(sys.argv[0]).name, sys.argv[1:], os.getcwd(),
                            os.environ.get('RUSTUP_TOOLCHAIN'), os.environ.get('CARGO_NET_OFFLINE')])+'\\n')
if pathlib.Path(sys.argv[0]).name == 'rustc': print('host: x86_64-unknown-linux-gnu')
if pathlib.Path(sys.argv[0]).name == 'cargo' and 'rustc' in sys.argv:
    print('native-static-libs: -ldl -lpthread', file=sys.stderr)
''')
        probe.chmod(0o755)
        for name in ('rustup', 'rustc', 'cargo'):
            (tools / name).symlink_to(probe)
        (tools / 'python3.13').symlink_to(sys.executable)
        driver = root / 'older.py'
        driver.write_text('''import runpy, shutil, sys
if sys.version_info >= (3, 11): sys.version_info = (3, 10)
class NoTomllib:
    def find_spec(self, fullname, *args):
        if fullname == 'tomllib': raise ModuleNotFoundError("No module named 'tomllib'")
sys.meta_path.insert(0, NoTomllib())
mode, modern = sys.argv[1:3]
which = shutil.which
def available(command):
    if mode == 'manylinux':
        return modern if command == '/opt/python/cp313-cp313/bin/python' else None
    if command.startswith('/opt/python/'):
        return None
    return which(command)
shutil.which = available
sys.argv = sys.argv[3:]
sys.path.insert(0, sys.argv[0].rsplit('/', 1)[0])
runpy.run_path(sys.argv[0], run_name='__main__')
''')
        env = child_env(home=root / 'home') | {'PATH': fixture_path(tools,
            exclude=(*[name for name in commands() if name.startswith('python')],
                     'python3.11', 'python3.12', 'python3.13')),
            'RUSTUP_TOOLCHAIN': 'ambient-wrong-pin', 'CARGO_NET_OFFLINE': 'true'}
        caller = [older or sys.executable, str(driver)]
        changed_sources = []
        for mode in ('modern', 'path', 'manylinux'):
            (root / 'calls').write_text('')
            for helper, args in (('prepare.py', []), ('build_rust.py',
                    ['--target', 'x86_64-unknown-linux-gnu', '--output', str(root / 'out')])):
                shutil.rmtree(source_root)
                with tarfile.open(archive_path) as archive:
                    archive.extractall(source_root, filter='data')
                command = ([sys.executable] if mode == 'modern' else caller + [mode, sys.executable])
                result = fixture_run(command + [str(community / helper), *args],
                    cwd=root.parent, env=env, capture_output=True, text=True)
                assert result.returncode == 0, (helper, mode, result.stderr)
                checked = fixture_run([sys.executable, str(REPO / 'sdlc/scripts/release-archive-tree.py'),
                    str(archive_path), str(source_root)], env=env, capture_output=True, text=True)
                if checked.returncode:
                    changed_sources.append((helper, mode, checked.stderr))
        assert not changed_sources, changed_sources
        calls = [json.loads(line) for line in (root / 'calls').read_text().splitlines()]
        import tomllib
        pin = tomllib.loads((source_root / 'rust-toolchain.toml').read_text())['toolchain']
        version = pin['channel']
        assert calls[0][:2] == ['rustup', ['toolchain', 'install', version, '--no-self-update',
            '--profile', pin['profile'], '--component', ','.join(pin['components'])]], calls
        assert calls[2][:2] == ['rustup', ['target', 'add', '--toolchain', version,
                                          'x86_64-unknown-linux-gnu']], calls
        assert calls[3][0] == 'cargo' and calls[3][1][:3] == ['+' + version, 'fetch', '--locked'], calls
        build = calls[-1]
        assert build[0] == 'cargo' and build[1][:4] == ['+' + version, 'rustc', '--locked', '--offline'], calls
        assert build[3:] == [version, 'true'], build
        assert all(call[2] == str(root.parent) for call in calls), calls
        assert str(source_root / 'databases/duckdb/bridge/Cargo.toml') in build[1], build
        assert (root / 'out/native-libs.rsp').read_text() == '-ldl -lpthread\n'
        # A named older interpreter must not cause recursion or permit an import failure.
        (tools / 'python3.13').unlink()
        if older:
            (tools / 'python3.13').symlink_to(older)
        for helper in ('prepare.py', 'build_rust.py'):
            result = fixture_run(caller + ['path', sys.executable, str(community / helper)],
                cwd=root.parent, env=env, capture_output=True, text=True)
            assert result.returncode != 0 and 'install Python 3.11 or later' in result.stderr, result.stderr
        print('community Python: modern and older callers preserve archived sources, preparation pins and offline inputs; missing modern host refuses')


def container_build_tools():
    """Configure C and C++ with the exact container bootstrap and no ambient Ninja."""
    if sys.platform != 'linux':
        print('release container: Linux build-tool regression requires a Linux host')
        return
    with tempfile.TemporaryDirectory(prefix='thinkthen-container-tools-') as temporary:
        root = Path(temporary)
        tools = root / 'bin'
        tools.mkdir()
        for name in ('cmake', 'make', 'cc', 'c++'):
            (tools / name).symlink_to(shutil.which(name))
        requirements = REPO / 'databases/duckdb/tools/requirements.txt'
        install = tools / 'python3'
        install.write_text(f'''#!{sys.executable}
import pathlib, sys
assert sys.argv[1:] == ['-m', 'pip', 'install', '--disable-pip-version-check', '-r', {str(requirements)!r}], sys.argv
pathlib.Path({str(tools / 'ninja')!r}).symlink_to({shutil.which('ninja')!r})
''')
        install.chmod(0o755)
        source = root / 'source'
        source.mkdir()
        (source / 'CMakeLists.txt').write_text('cmake_minimum_required(VERSION 3.18)\nproject(probe LANGUAGES C CXX)\n')
        container = (REPO / 'sdlc/scripts/release-container').read_text()
        bootstrap = container.split('\tINPUTS_HERE=', 1)[1].split('\tpython3 "$INPUTS_HERE/source_hosts.py" --fetch', 1)[0]
        bootstrap = 'INPUTS_HERE=' + bootstrap.replace('/work/src/databases/duckdb/tools', str(requirements.parent))
        env = child_env(home=root / 'home') | {'PATH': fixture_path(tools,
            exclude=('ninja', 'make', 'cmake', 'cc', 'c++')), 'TARGET': 'aarch64-unknown-linux-gnu'}
        command = bootstrap + '\ncmake -G Ninja -S "' + str(source) + '" -B "' + str(root / 'build') + '"\n'
        result = fixture_run(['/bin/sh', '-eu', '-c', command], env=env,
                                capture_output=True, text=True)
        assert result.returncode == 0, result.stdout + result.stderr
        cache = (root / 'build/CMakeCache.txt').read_text()
        assert 'CMAKE_MAKE_PROGRAM:FILEPATH=' + str(tools / 'ninja') in cache, cache
        for language in ('C', 'CXX'):
            assert f'CMAKE_{language}_COMPILER:FILEPATH=' in cache, cache
        for missing in ('make', 'cmake', 'cc', 'c++'):
            (tools / 'ninja').unlink()
            (tools / missing).unlink()
            refused = fixture_run(['/bin/sh', '-eu', '-c', bootstrap], env=env,
                                     capture_output=True, text=True)
            assert refused.returncode == 1, refused.stderr
            assert f'missing DuckDB build tool: {missing}' in refused.stderr, refused.stderr
            (tools / missing).symlink_to(shutil.which(missing))
        print('release container: pinned bootstrap provides Ninja, configures C and C++, and refuses missing build tools')


def wheel():
    with tempfile.TemporaryDirectory(prefix='thinkthen-wheel-python-') as temporary:
        root = Path(temporary)
        tools = root / 'tools'
        tools.mkdir()
        result_path = root / 'maturin.json'
        maturin = tools / 'maturin'
        maturin.write_text(f'''#!{sys.executable}
import json, os, sys
from pathlib import Path
Path({str(result_path)!r}).write_text(json.dumps([sys.argv[1:], os.environ['PYO3_PYTHON']]))
sys.exit(97)
''')
        maturin.chmod(0o755)
        floor = str(root / 'Python 3.10/python')
        env = child_env(home=root / 'home') | {'PATH': fixture_path(tools),
                                             'PYO3_PYTHON': floor}
        result = fixture_run(['sh', str(REPO / 'libraries/python/build-wheel.sh')],
                                env=env, capture_output=True, text=True)
        assert result.returncode == 97, result.stderr
        args, selected = json.loads(result_path.read_text())
        assert args[args.index('--interpreter') + 1] == floor and selected == floor, args
        assert {'--locked', '--offline', '--release'} <= set(args), args
        print('release wheel: modern tools forward the explicit floor interpreter to maturin')


if __name__ == '__main__':
    community()
    container_build_tools()
    wheel()
