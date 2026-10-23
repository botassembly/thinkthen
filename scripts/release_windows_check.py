"""Check the Windows candidate's inventory and run its installed CLI and C stub cases."""
import hashlib
import json
import os
import shutil
from pathlib import Path
import struct
import subprocess
import tempfile
import zipfile


def wine_executable():
    supplied = os.environ.get('THINKTHEN_WINE')
    candidates = [supplied] if supplied else [shutil.which('wine'), shutil.which('wine64'),
                    '/usr/lib/x86_64-linux-gnu/wine/wine', '/usr/lib/wine/wine64']
    for candidate in candidates:
        if candidate and Path(candidate).is_file(): return candidate
    raise RuntimeError('Prepare Wine before running Windows tools or the offline Wine smoke')


def pe(data, *, dll):
    """Require a complete x86-64 PE32+ image of the requested kind."""
    assert len(data) >= 64 and data[:2] == b'MZ', 'not a Windows image'
    offset = struct.unpack_from('<I', data, 60)[0]
    assert offset + 26 <= len(data) and data[offset:offset + 4] == b'PE\0\0', 'incomplete PE header'
    machine = struct.unpack_from('<H', data, offset + 4)[0]
    flags = struct.unpack_from('<H', data, offset + 22)[0]
    assert machine == 0x8664 and struct.unpack_from('<H', data, offset + 24)[0] == 0x20b, 'not x86-64 PE32+'
    assert flags & 2 and bool(flags & 0x2000) == dll, 'wrong Windows image kind'


def inventory(folder):
    rows = (folder / 'SHA256SUMS').read_text().splitlines()
    expected = {}
    for row in rows:
        digest, name = row.split('  ', 1)
        assert name not in expected and not Path(name).is_absolute() and '..' not in Path(name).parts
        expected[name] = digest
    actual = {p.relative_to(folder).as_posix() for p in folder.rglob('*') if p.is_file() and p != folder / 'SHA256SUMS'}
    assert actual == set(expected), ('candidate inventory differs', actual ^ set(expected))
    for name, digest in expected.items():
        path = folder / name
        assert not path.is_symlink() and hashlib.sha256(path.read_bytes()).hexdigest() == digest, name
    build = json.loads((folder / 'build.json').read_text())
    assert build['platform'] == 'windows-x86_64'
    packages = build['packages']
    assert set(packages) == {'command', 'c', 'cpp', 'python', 'typescript', 'go', 'java', 'kotlin', 'csharp', 'r', 'ruby', 'zig', 'swift', 'duckdb', 'sqlite'}
    from release_notice_check import verify
    for surface, name in packages.items():
        assert name in actual, (surface, name)
        verify(folder / name)
    for jar in (folder / 'kotlin/dependencies').glob('thinkthen-java-*.jar'): verify(jar)
    with zipfile.ZipFile(folder / packages['command']) as archive:
        assert {p for p in archive.namelist() if 'THIRD-PARTY-NOTICES' not in p.split('/')} == {'thinkthen.exe', 'README.md', 'LICENSE'}
        pe(archive.read('thinkthen.exe'), dll=False)
    return build


def smoke(folder):
    folder = Path(folder).resolve()
    build = inventory(folder)
    with tempfile.TemporaryDirectory(prefix='thinkthen-windows-check-') as temporary:
        scratch = Path(temporary)
        for name in ('command', 'c'):
            with zipfile.ZipFile(folder / build['packages'][name]) as archive:
                archive.extractall(scratch / name)
        shutil.copy2(folder / 'checks/c-consumer.exe', scratch / 'c/bin/c-consumer.exe')
        pe((scratch / 'c/bin/thinkthen_ffi.dll').read_bytes(), dll=True)
        prefix = scratch / 'wine'
        env = {key: value for key, value in os.environ.items() if key in ('PATH', 'HOME', 'LANG', 'SYSTEMROOT', 'TEMP', 'TMP')}
        env.update(WINEPREFIX=str(prefix), WINEDEBUG='-all', WINEDLLOVERRIDES='mscoree,mshtml=')
        if os.name == 'nt':
            from release_windows_sdks import runtime_environment
            env = runtime_environment(scratch, env)
        runner = [] if os.name == 'nt' else [wine_executable()]
        def run(binary, *args, payload=None, code=0):
            result = subprocess.run([*runner, str(binary), *args], input=payload, stdout=subprocess.PIPE,
                                    stderr=subprocess.PIPE, env=env, cwd=scratch, timeout=120)
            assert result.returncode == code, (binary.name, result.returncode, result.stderr.decode(errors='replace')[-2000:])
            return result.stdout.decode()
        command = scratch / 'command/thinkthen.exe'
        assert run(command, '--version').strip() == 'thinkthen ' + build['version']
        cases = json.loads((folder / 'checks/stub.json').read_text())
        for case in cases:
            lines = run(scratch / 'c/bin/c-consumer.exe', payload=json.dumps(case['request']).encode()).splitlines()
            rows = []
            assert len(lines) % 2 == 0
            for index in range(0, len(lines), 2):
                row = json.loads(lines[index])
                if lines[index + 1]: row['detail'] = json.loads(lines[index + 1])
                rows.append(row)
            assert rows == case['expected'], case['request']['function']
        first = cases[0]
        request = first['request']
        record = request['records'][0]
        question = request['question']['value']
        expected = first['expected'][0]
        output = run(command, 'decide', '--adapter', 'stub', question, '--lines', '--details',
                     payload=(record + '\n').encode())
        assert json.loads(output) == expected
    print('Windows candidate: checked inventory; installed CLI and C ABI shared stub cases pass')
    if os.name == 'nt':
        from release_windows_sdks import full
        full(folder, build)
