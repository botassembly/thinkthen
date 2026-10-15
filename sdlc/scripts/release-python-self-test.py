#!/usr/bin/env python3
"""Exercise release interpreter selection and community builds from older callers."""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO))
from conformance.children.children import child_env


def community():
    older = sys.argv[1] if len(sys.argv) > 1 else None
    with tempfile.TemporaryDirectory(prefix='thinkthen-community-python-') as temporary:
        root = Path(temporary)
        community = root / 'databases/duckdb/community'
        community.mkdir(parents=True)
        for source in (REPO / 'databases/duckdb/community').glob('*.py'):
            shutil.copyfile(source, community / source.name)
        shutil.copyfile(REPO / 'rust-toolchain.toml', root / 'rust-toolchain.toml')
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
        env = child_env(home=root / 'home') | {'PATH': str(tools),
            'RUSTUP_TOOLCHAIN': 'ambient-wrong-pin', 'CARGO_NET_OFFLINE': 'true'}
        caller = [older or sys.executable, str(driver)]
        for mode in ('path', 'manylinux'):
            (root / 'calls').write_text('')
            for helper, args in (('prepare.py', []), ('build_rust.py',
                    ['--target', 'x86_64-unknown-linux-gnu', '--output', str(root / 'out')])):
                result = subprocess.run(caller + [mode, sys.executable, str(community / helper), *args],
                    cwd=root.parent, env=env, capture_output=True, text=True, timeout=15)
                assert result.returncode == 0, (helper, mode, result.stderr)
        calls = [json.loads(line) for line in (root / 'calls').read_text().splitlines()]
        import tomllib
        pin = tomllib.loads((root / 'rust-toolchain.toml').read_text())['toolchain']
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
        assert str(root / 'databases/duckdb/bridge/Cargo.toml') in build[1], build
        assert (root / 'out/native-libs.rsp').read_text() == '-ldl -lpthread\n'
        # A named older interpreter must not cause recursion or permit an import failure.
        (tools / 'python3.13').unlink()
        if older:
            (tools / 'python3.13').symlink_to(older)
        for helper in ('prepare.py', 'build_rust.py'):
            result = subprocess.run(caller + ['path', sys.executable, str(community / helper)],
                cwd=root.parent, env=env, capture_output=True, text=True, timeout=15)
            assert result.returncode != 0 and 'install Python 3.11 or later' in result.stderr, result.stderr
        print('community Python: older callers preserve preparation pins and offline build inputs; missing modern host refuses')


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
        env = child_env(home=root / 'home') | {'PATH': str(tools) + ':' + '/usr/bin:/bin',
                                             'PYO3_PYTHON': floor}
        result = subprocess.run(['sh', str(REPO / 'libraries/python/build-wheel.sh')],
                                env=env, capture_output=True, text=True, timeout=15)
        assert result.returncode == 97, result.stderr
        args, selected = json.loads(result_path.read_text())
        assert args[args.index('--interpreter') + 1] == floor and selected == floor, args
        assert {'--locked', '--offline', '--release'} <= set(args), args
        print('release wheel: modern tools forward the explicit floor interpreter to maturin')


if __name__ == '__main__':
    community()
    wheel()
