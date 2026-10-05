#!/usr/bin/env python3
"""Portable 0381 A regressions. Synthetic files are not native Windows proof."""
import copy
import contextlib
import io
import os
import shutil
from pathlib import Path
import runpy
import struct
import subprocess
import sys
import tempfile
from unittest import mock
import warnings
import zipfile
import yaml

REPO = Path(__file__).resolve().parents[2]
FIXTURE = runpy.run_path(str(Path(__file__).with_name('release-windows-c-fixture.py')))
C = FIXTURE['C']
ROUTES = runpy.run_path(str(Path(__file__).with_name('release-windows-c-workflow.py')))
VERSION = next(line.split('"')[1] for line in (REPO / 'crates/thinkthen/Cargo.toml').read_text().splitlines()
               if line.startswith('version = "'))


def refused(action, cause):
    try:
        action()
    except (ValueError, zipfile.BadZipFile) as error:
        assert cause in str(error), (cause, str(error))
    else:
        raise AssertionError('plant passed: ' + cause)


def workflow_plants():
    original = yaml.safe_load((REPO / '.github/workflows/release.yml').read_text())['jobs']
    assert ROUTES['rules']('release.yml', original) == []
    cases = 0
    for key, label in [('CLIPPY', 'C Clippy'), ('TEST', 'C door tests'), ('PACK', 'C packing'),
                       ('PLATFORM', 'platform verification'), ('CHECK', 'downloaded C validation'),
                       ('CONSUME', 'downloaded C consumption')]:
        command = ROUTES[key]
        kind = 'smoke' if key in ('CHECK', 'CONSUME') else 'build'
        for replacement in ('', 'echo ' + command, '# ' + command, command + ' || true'):
            jobs = copy.deepcopy(original)
            step = next(step for step in jobs[kind]['steps'] if command in step.get('run', ''))
            step['run'] = step['run'].replace(command, replacement)
            failures = ROUTES['rules']('release.yml', jobs)
            assert any(label in error for error in failures), (key, replacement, failures)
            cases += 1
    for kind in ('build', 'smoke'):
        for plant in ('missing-setup', 'late-setup', 'wrong-target', 'disabled-route', 'masked-route', 'wrong-route-target', 'job-disabled'):
            jobs = copy.deepcopy(original)
            steps = jobs[kind]['steps']
            setup = next(step for step in steps if step.get('run') == ROUTES['SETUP'])
            route = next(step for step in steps if ROUTES['PACK' if kind == 'build' else 'CONSUME'] in step.get('run', ''))
            if plant == 'missing-setup':
                steps.remove(setup)
            elif plant == 'late-setup':
                steps.remove(setup)
                steps.append(setup)
            elif plant == 'wrong-target':
                setup['if'] = "matrix.target == 'x86_64-unknown-linux-gnu'"
            elif plant == 'disabled-route':
                route['if'] = 'false'
            elif plant == 'wrong-route-target':
                route['if'] = "matrix.target == 'x86_64-unknown-linux-gnu'"
            elif plant == 'masked-route':
                route['continue-on-error'] = True
            else:
                jobs[kind]['if'] = 'false'
            failures = ROUTES['rules']('release.yml', jobs)
            if plant in ('disabled-route', 'masked-route', 'wrong-route-target'):
                cause = 'Windows C packing' if kind == 'build' else 'Windows downloaded C consumption'
            elif plant == 'job-disabled':
                cause = f'Windows C {kind} job must execute'
            else:
                cause = f'Windows {kind} must select x64 MSVC'
            assert any(cause in error for error in failures), (kind, plant, failures)
            cases += 1
    for kind, boundary in [('build', 'upload'), ('smoke', 'download')]:
        for plant in ('missing', 'misordered'):
            jobs = copy.deepcopy(original)
            steps = jobs[kind]['steps']
            asset = next(step for step in steps if (step.get('with') or {}).get('name') == 'platform-${{ matrix.target }}')
            steps.remove(asset)
            if plant == 'misordered':
                steps.insert(0, asset) if kind == 'build' else steps.append(asset)
            failures = ROUTES['rules']('release.yml', jobs)
            cause = 'verify before upload' if kind == 'build' else 'download before archive consumption'
            assert any(cause in error for error in failures), (boundary, plant, failures)
            cases += 1
    windows = yaml.safe_load((REPO / '.github/workflows/windows.yml').read_text())['jobs']
    assert ROUTES['standalone']('windows.yml', windows) == []
    for plant in ('missing', 'late', 'disabled'):
        jobs = copy.deepcopy(windows)
        steps = jobs['stage-0']['steps']
        setup = next(step for step in steps if step.get('run') == ROUTES['SETUP'])
        if plant == 'disabled':
            setup['if'] = 'false'
        else:
            steps.remove(setup)
            if plant == 'late':
                steps.append(setup)
        assert ROUTES['standalone']('windows.yml', jobs) == [
            'windows.yml: standalone Windows must select x64 MSVC before C checks']
        cases += 1
    return cases


def archives(root):
    archive = FIXTURE['create'](root, VERSION)
    first = archive.read_bytes()
    assert C.check(archive)[1] == FIXTURE['pe']()
    FIXTURE['create'](root, VERSION)
    assert first == archive.read_bytes()
    with mock.patch.object(C.zipfile.sys, 'platform', 'win32'):
        FIXTURE['create'](root, VERSION)
    assert first == archive.read_bytes(), '0381_C_ZIP_HOST_METADATA_DIFFERS'
    original = C.check(archive)
    for members, cause in [(C.MEMBERS + ('extra',), 'exactly header'),
                           (C.MEMBERS[:-1], 'exactly header'),
                           (C.MEMBERS + (C.MEMBERS[0],), 'exactly header'),
                           (('../include/thinkthen.h',) + C.MEMBERS[1:], 'exactly header')]:
        with warnings.catch_warnings(), zipfile.ZipFile(archive, 'w') as out:
            warnings.simplefilter('ignore')
            for name in members:
                info = zipfile.ZipInfo(name)
                info.external_attr = 0o100644 << 16
                out.writestr(info, original[C.MEMBERS.index(name)] if name in C.MEMBERS else b'extra')
        C.checksum(archive)
        refused(lambda: C.check(archive), cause)
    for mode in (0o120777, 0o040755, 0o010644):
        with zipfile.ZipFile(archive, 'w') as out:
            for name, data in zip(C.MEMBERS, original):
                info = zipfile.ZipInfo(name)
                info.external_attr = mode << 16
                out.writestr(info, data)
        C.checksum(archive)
        refused(lambda: C.check(archive), 'regular files')
    FIXTURE['create'](root, VERSION)
    sidecar = archive.with_name(archive.name + '.sha256')
    sidecar.write_text('bad')
    refused(lambda: C.check(archive), 'differs from its checksum')
    sidecar.unlink()
    refused(lambda: C.check(archive), 'missing or linked')
    refused(lambda: C.dll(b'not PE'), 'not a Windows DLL')
    for at, value in [(68, 0xaa64), (86, 2), (88, 0x10b)]:
        binary = bytearray(FIXTURE['pe']())
        struct.pack_into('<H', binary, at, value)
        refused(lambda: C.dll(binary), 'PE32+ x86-64 DLL')
    library = FIXTURE['library'](C.HEADER.read_bytes())
    refused(lambda: C.imports(library[:-2]), 'truncated member')
    refused(lambda: C.imports(b'not ar'), 'COFF archive')
    refused(lambda: C.imports(library.replace(b'thinkthen.dll', b'thinkthen_bad')), 'target thinkthen.dll')
    broken_table = bytearray(library)
    struct.pack_into('>I', broken_table, 8+60+4, 1)
    refused(lambda: C.imports(broken_table), 'points outside COFF members')
    bad = bytearray(library)
    short = bad.index(b'\0\0\xff\xff')
    struct.pack_into('<H', bad, short+6, 0xaa64)
    refused(lambda: C.imports(bad), 'x64 imports')
    # Header parser's oracle is independent literal declarations.
    assert C.declarations('/* thinkthen_hidden(void); */ void thinkthen_z(void); int thinkthen_a(int n);') == [
        'thinkthen_a', 'thinkthen_z']
    for text, cause in [('/*', 'unterminated'), ('void thinkthen_a(void); void thinkthen_a(void);', 'duplicate'),
                        ('int thinkthen_bad;', 'malformed'), ('typedef int thinkthen_bad(void);', 'malformed'),
                        ('int thinkthen_bad(', 'malformed'), ('int thinkthen_bad();', 'malformed')]:
        refused(lambda: C.declarations(text), cause)
    saved = 'ordinal hint RVA      name\n      1    0 00001000 thinkthen_a\n      2    1 00001020 other_export\n\n Summary\n'
    assert C.exports(saved) == ['other_export', 'thinkthen_a']  # Never prefix-filter DLL exports.
    for line in ['1 0 00001000 thinkthen_a = other.dll.func', '1 0 00001000 _thinkthen_a@8',
                 '1 00001000 [NONAME]']:
        refused(lambda: C.exports('ordinal hint RVA name\n' + line), 'decorated, forwarded or ordinal-only')
    # lib.exe errors propagate; newly owned staging goes and caller input remains.
    binary = root / 'caller.dll'
    binary.write_bytes(FIXTURE['pe']())
    folders = []
    def broken(header, folder):
        folders.append(folder)
        raise ValueError('lib.exe failed (exit 47)')
    with mock.patch.object(C, 'import_library', broken):
        refused(lambda: C.build_pack(binary, C.HEADER, archive), 'lib.exe failed (exit 47)')
    assert all(not folder.exists() for folder in folders)
    assert binary.read_bytes() == FIXTURE['pe']()
    # No public cleanup interface accepts caller-selected paths; constructor failure cannot remove a lane.
    with mock.patch.object(C.tempfile, 'TemporaryDirectory', side_effect=OSError('refused scratch')):
        try:
            C.build_pack(binary, C.HEADER, archive)
        except OSError:
            pass
    assert C.HEADER.is_file() and binary.is_file()
    FIXTURE['create'](root, VERSION)
    archive_bytes = archive.read_bytes()
    smoke = runpy.run_path(str(Path(__file__).with_name('release-windows-c-smoke.py')))
    owned_temporary = tempfile.TemporaryDirectory
    folders = []
    def capture_scratch(*args, **kwargs):
        owned = owned_temporary(*args, **kwargs)
        folders.append(Path(owned.name))
        return owned
    with mock.patch.object(smoke['C'], 'run', side_effect=ValueError('cl.exe failed (exit 48)')):
        with mock.patch.object(tempfile, 'TemporaryDirectory', capture_scratch):
            refused(lambda: smoke['consume'](archive), 'cl.exe failed (exit 48)')
    assert folders and all(not folder.exists() for folder in folders)
    assert archive.read_bytes() == archive_bytes, 'consumer setup failure changed caller archive'



def pack_routing(root):
    source = root / 'source'
    scripts = source / 'sdlc/scripts'
    scripts.mkdir(parents=True)
    for name in ('release-pack', 'scratch.sh', 'release-windows-command.py', 'release-windows-c.py', 'release-bounded.py', 'release-owned-job.py'):
        shutil.copyfile(REPO / 'sdlc/scripts' / name, scripts / name)
    for manifest in ('crates/thinkthen/Cargo.toml', 'libraries/c/Cargo.toml'):
        path = source / manifest
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(f'[package]\nversion = "{VERSION}"\n')
    include = source / 'libraries/c/include'
    include.mkdir()
    shutil.copyfile(C.HEADER, include / 'thinkthen.h')
    shutil.copytree(REPO / 'demos/27-test-with-no-network', source / 'demos/27-test-with-no-network')
    python = source / 'libraries/python'
    python.mkdir()
    (python / 'build-wheel.sh').write_text(
        f'#!/bin/sh\nmkdir -p libraries/python/target/release-wheel\n'
        f'printf "wheel fixture" >libraries/python/target/release-wheel/thinkthen-{VERSION}-cp310-abi3-win_amd64.whl\n')
    tools = root / 'tools'
    tools.mkdir()
    saved_dll = root / 'saved.dll'
    saved_dll.write_bytes(FIXTURE['pe']())
    saved_command = root / 'saved.exe'
    command_bytes = bytearray(FIXTURE['pe']())
    struct.pack_into('<H', command_bytes, 86, 2)
    saved_command.write_bytes(command_bytes)
    saved_library = root / 'saved.lib'
    saved_library.write_bytes(FIXTURE['library'](C.HEADER.read_bytes()))
    names = C.declarations(C.HEADER.read_text())
    export_output = 'ordinal hint RVA name\n' + ''.join(f'{i+1} {i:X} {4096+i*16:08X} {name}\n' for i, name in enumerate(names)) + 'Summary\n'
    bodies = {
        'rustc': f'#!/bin/sh\nprintf "host: {C.TARGET}\\n"\n',
        'cygpath': '#!/bin/sh\nprintf "%s\\n" "$2"\n',
        'cargo': f'#!{sys.executable}\n' +
            "import os, pathlib, shutil, sys\n" +
            "args=sys.argv[1:]; assert args[0]=='build' and '--locked' in args\n" +
            "assert os.environ.get('CARGO_NET_OFFLINE')=='true'\n" +
            "profile='release' if '--release' in args else 'debug'\n" +
            "out=pathlib.Path(os.environ['CARGO_TARGET_DIR'])\n" +
            f"if '--package' in args: out=out/{C.TARGET!r}\n" +
            "out=out/profile; out.mkdir(parents=True,exist_ok=True)\n" +
            f"shutil.copyfile({str(saved_dll)!r},out/'thinkthen_c.dll')\n" +
            f"shutil.copyfile({str(saved_command)!r},out/'thinkthen.exe')\n",
        'lib.exe': f'#!{sys.executable}\n' +
            "import pathlib, shutil, sys\n" +
            "assert '/MACHINE:X64' in sys.argv\n" +
            "definition=pathlib.Path(next(arg[5:] for arg in sys.argv if arg.startswith('/DEF:')))\n" +
            "assert definition.read_text().startswith('LIBRARY thinkthen.dll\\nEXPORTS\\n')\n" +
            f"shutil.copyfile({str(saved_library)!r},next(arg[5:] for arg in sys.argv if arg.startswith('/OUT:')))\n",
        'dumpbin.exe': f'#!{sys.executable}\nimport sys\n' +
            f"print({export_output!r} if '/exports' in sys.argv else '8664 machine (x64) thinkthen.dll')\n",
    }
    for name, body in bodies.items():
        path = tools / name
        path.write_text(body)
        path.chmod(0o755)
    scratch = root / 'tmp'
    scratch.mkdir()
    home = root / 'home'
    home.mkdir()
    env = {'PATH': str(tools) + os.pathsep + os.environ['PATH'], 'HOME': str(home),
           'LANG': 'C.UTF-8', 'LC_ALL': 'C.UTF-8', 'TMPDIR': str(scratch), 'CARGO_NET_OFFLINE': 'true',
           'CARGO_TARGET_DIR': str(root / 'build')}
    for reuse in (False, True):
        destination = root / ('reuse' if reuse else 'release')
        args = ['sh', str(scripts / 'release-pack')] + (['--reuse'] if reuse else [])
        result = subprocess.run(args + [C.TARGET, str(destination), 'c'], env=env, capture_output=True, text=True)
        assert result.returncode == 0, result.stderr
        assert C.check(destination / f'thinkthen-c-{VERSION}-{C.TARGET}.zip')
        assert not list(scratch.iterdir()), 'packer retained owned staging'
    default_output = root / 'default-output'
    result = subprocess.run(['sh', str(scripts / 'release-pack'), C.TARGET, str(default_output)],
                            env=env, capture_output=True, text=True)
    assert result.returncode == 0, result.stderr
    assert {path.name for path in default_output.iterdir()} == {
        f'thinkthen-{VERSION}-{C.TARGET}.zip', f'thinkthen-{VERSION}-{C.TARGET}.zip.sha256',
        f'thinkthen-c-{VERSION}-{C.TARGET}.zip', f'thinkthen-c-{VERSION}-{C.TARGET}.zip.sha256',
        f'thinkthen-{VERSION}-cp310-abi3-win_amd64.whl', f'thinkthen-{VERSION}-cp310-abi3-win_amd64.whl.sha256',
        'thinkthen-first-run.tar.gz', 'thinkthen-first-run.tar.gz.sha256'}
    assert not list(scratch.iterdir())
    # Real tool failure propagates through the real packer and cleans scratch.
    (tools / 'lib.exe').write_text('#!/bin/sh\nexit 47\n')
    failed = root / 'failed'
    result = subprocess.run(['sh', str(scripts / 'release-pack'), C.TARGET, str(failed), 'c'],
                            env=env, capture_output=True, text=True)
    assert result.returncode == 1 and 'lib.exe failed (exit 47)' in result.stderr, result.stderr
    assert not list(scratch.iterdir()) and not list(failed.iterdir())
    assert saved_library.is_file() and saved_dll.is_file()


def msvc_setup_routing(root):
    variables = {'PATH': 'pinned-tools', 'SystemRoot': 'system', 'TEMP': str(root), 'TMP': str(root),
                 'ProgramFiles(x86)': str(root / 'Program Files'), 'CL': 'masked-warning',
                 '_CL_': 'masked-warning', 'LINK': 'masked-link', 'FAKE_SERVICE_API_KEY': 'fake-credential'}
    expected = {name: value for name, value in variables.items()
                if name not in ('CL', '_CL_', 'LINK', 'FAKE_SERVICE_API_KEY')}
    output = io.StringIO()
    setup = root / 'Visual Studio/VC/Auxiliary/Build/vcvarsall.bat'
    namespace = runpy.run_path(str(Path(__file__).with_name('release-msvc.py')))
    capture = mock.Mock()
    def compiler_environment(command, **options):
        assert options == {'env': expected, 'timeout': 60, 'text': True}
        if isinstance(command, list):
            assert command[0] == str(root / 'Program Files/Microsoft Visual Studio/Installer/vswhere.exe')
            assert command[1:] == ['-latest', '-products', '*', '-requires',
                'Microsoft.VisualStudio.Component.VC.Tools.x86.x64', '-property', 'installationPath']
            return subprocess.CompletedProcess(command, 0, str(root / 'Visual Studio'), '')
        assert command == f'cmd.exe /d /s /c ""{setup}" x64 >nul && set"'
        return subprocess.CompletedProcess(command, 0,
            'PATH=x64-tools\nINCLUDE=inc\nLIB=libs\nLIBPATH=libpath\nCL=masked-warning\nFAKE_SERVICE_API_KEY=fake-credential\n', '')
    capture.side_effect = compiler_environment
    namespace['main'].__globals__['CAPTURE'] = capture
    with mock.patch.dict(os.environ, variables, clear=True):
        with mock.patch.object(Path, 'is_file', return_value=True), contextlib.redirect_stdout(output):
            namespace['main']()
    assert capture.call_count == 2
    assert output.getvalue().splitlines() == ['PATH=x64-tools', 'INCLUDE=inc', 'LIB=libs', 'LIBPATH=libpath']
    capture.reset_mock()
    with mock.patch.dict(os.environ, variables, clear=True), mock.patch.object(Path, 'is_file', return_value=False):
        try:
            namespace['main']()
        except SystemExit as error:
            assert str(error) == 'release-msvc: installed x64 compiler environment is missing'
        else:
            raise AssertionError('missing MSVC setup passed preflight')
    assert capture.call_count == 1, 'missing setup executed native cmd'



def main():
    with tempfile.TemporaryDirectory(prefix='thinkthen-0381-portable-') as temporary:
        archives(Path(temporary))
        pack_routing(Path(temporary))
        msvc_setup_routing(Path(temporary))
    runpy.run_path(str(Path(__file__).with_name('release-process-cleanup-test.py')))['main']()
    plants = workflow_plants()
    print(f'Windows C portable self-test: archive/header/export refusals and {plants} workflow plants passed; native proof pending')

if __name__ == '__main__':
    main()
