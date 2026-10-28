"""Prepare the cross builder or the native Windows installed-package checker."""
import json
import os
from pathlib import Path
import platform
import re
import runpy
import shutil
import subprocess
import tempfile
import zipfile

from release_prepare_common import ROOT, TOOLS, fetch, run, prepared


def selections():
    return runpy.run_path(str(ROOT / 'scripts/release-windows-prepare'))


def zig_version():
    return re.search(r'minimum_zig_version = "([^"]+)"', (ROOT / 'sdk/zig/build.zig.zon').read_text())[1]


def swift_version():
    return re.search(r'swift/([0-9.]+)-ubuntu', (ROOT / 'scripts/floors/prepare-swift').read_text())[1]


def swift_components(env):
    version = swift_version()
    for base in (Path(env['LOCALAPPDATA']) / 'Programs/Swift', Path(env['PROGRAMFILES']) / 'Swift'):
        toolchain = base / f'Toolchains/{version}+Asserts/usr/bin'
        if not (toolchain / 'swift.exe').is_file():
            continue
        runtime = base / f'Runtimes/{version}/usr/bin'
        sdk = base / f'Platforms/{version}/Windows.platform/Developer/SDKs/Windows.sdk'
        if not runtime.is_dir() or not sdk.is_dir():
            raise ValueError('Pinned Swift installation is incomplete: ' + str(base))
        return toolchain, runtime, sdk
    return None


def native_run(command, *arguments, env, **options):
    # Windows executable lookup cannot use subprocess's supplied env PATH.
    # https://docs.python.org/3/library/subprocess.html#popen-constructor
    executable = shutil.which(str(command), path=env['PATH'])
    if executable is None:
        raise ValueError('native Windows tool is missing: ' + str(command))
    return run(executable, *arguments, env=env, **options)


def environment():
    if os.name != 'nt':
        raise ValueError('installed Windows checks require native Windows')
    env = {key.upper(): value for key, value in os.environ.items()}
    vswhere = Path(env['PROGRAMFILES(X86)']) / 'Microsoft Visual Studio/Installer/vswhere.exe'
    installation = subprocess.check_output([str(vswhere), '-latest', '-products', '*',
        '-requires', 'Microsoft.VisualStudio.Component.VC.Tools.x86.x64', '-property', 'installationPath'], text=True).strip()
    vcvars = Path(installation) / 'VC/Auxiliary/Build/vcvars64.bat'
    selected = subprocess.check_output(f'call "{vcvars}" >nul && set', shell=True, text=True)
    env.update((key.upper(), value) for line in selected.splitlines() if '=' in line and not line.startswith('=') for key, value in [line.split('=', 1)])
    paths = [TOOLS / 'windows/R-4.6.1/bin/x64', TOOLS / 'windows-check/bin', TOOLS / 'zig' / zig_version(), Path('C:/msys64/ucrt64/bin')]
    for variable in ('JAVA_HOME', 'JAVA_HOME_22_X64'):
        if env.get(variable):
            paths.append(Path(env[variable]) / 'bin')
            break
    swift = swift_components(env)
    if swift is not None:
        toolchain, runtime, sdk = swift
        paths.extend((toolchain, runtime))
        env['SDKROOT'] = str(sdk)
    paths.extend(sorted((Path(env['PROGRAMFILES']) / 'R').glob('R-4.6*/bin/x64'), reverse=True)[:1])
    env['PATH'] = os.pathsep.join(map(str, paths)) + os.pathsep + env['PATH']
    return env


def native():
    from release_windows_r import runtime
    runtime()
    pins = selections()['INPUTS']
    folder = TOOLS / 'windows-check'
    folder.mkdir(parents=True, exist_ok=True)
    binary = folder / 'bin'
    binary.mkdir(exist_ok=True)
    url, digest = pins['duckdb-cli.zip']
    with zipfile.ZipFile(fetch(url, folder / 'duckdb-cli.zip', digest)) as archive:
        archive.extractall(binary)
    release = zig_version()
    zig = TOOLS / 'zig' / release
    if not (zig / 'zig.exe').exists():
        url = f'https://ziglang.org/download/{release}/zig-x86_64-windows-{release}.zip'
        with zipfile.ZipFile(fetch(url, folder / 'zig.zip')) as archive:
            archive.extractall(folder / 'zig')
        zig.parent.mkdir(parents=True, exist_ok=True)
        shutil.move(next((folder / 'zig').iterdir()), zig)
    env = environment()
    if swift_components(env) is None:
        # Official signed installer; its version matches the Linux Swift closure.
        version = swift_version()
        name = f'swift-{version}-RELEASE-windows10.exe'
        installer = fetch(f'https://download.swift.org/swift-{version}-release/windows10/swift-{version}-RELEASE/{name}', folder / name)
        run('powershell', '-NoProfile', '-Command',
            'if ((Get-AuthenticodeSignature -LiteralPath $args[0]).Status -ne "Valid") {exit 1}', installer)
        run(installer, '-quiet', '-norestart')
        env = environment()
        if swift_components(env) is None:
            raise ValueError('Pinned Swift installation is missing after preparation')
    # The runner provides MSYS2. Install its UCRT compiler for Go's cgo consumer.
    run('C:/msys64/usr/bin/bash.exe', '-lc', 'pacman --noconfirm -S --needed mingw-w64-ucrt-x86_64-gcc')
    sqlite = binary / 'sqlite3.exe'
    if not sqlite.exists():
        url, digest = pins['sqlite-amalgamation-3410000.zip']
        with tempfile.TemporaryDirectory(prefix='thinkthen-sqlite-host-') as temporary:
            with zipfile.ZipFile(fetch(url, folder / 'sqlite.zip', digest)) as archive:
                archive.extractall(temporary)
            source = next(Path(temporary).iterdir())
            native_run('clang-cl', '/nologo', '/O2', '/D_CRT_SECURE_NO_WARNINGS',
                source / 'shell.c', source / 'sqlite3.c', '/Fe:' + str(sqlite),
                '/link', 'advapi32.lib', 'user32.lib', env=env, cwd=temporary)
    for command in ('node', 'npm', 'ruby', 'gem', 'Rscript', 'java', 'dotnet', 'go', 'gcc', 'clang-cl', 'swift', 'zig', 'sqlite3', 'duckdb'):
        if not shutil.which(command, path=env['PATH']):
            raise ValueError('native Windows runtime is missing: ' + command)
    requirements = json.loads((ROOT / 'scripts/runtime-floors.json').read_text())['runtimes']
    for name, key, arguments in [('node', 'typescript', ['--version']), ('ruby', 'ruby', ['--version']),
                                 ('java', 'java', ['--version']), ('go', 'go', ['version']),
                                 ('swift', 'swift', ['--version']), ('zig', 'zig', ['version'])]:
        minimum = re.search(r'[0-9]+(?:\.[0-9]+)*', requirements[key])[0]
        actual = native_run(name, *arguments, env=env, capture_output=True, text=True).stdout
        found = re.search(r'[0-9]+(?:\.[0-9]+)*', actual)[0]
        if tuple(map(int, found.split('.'))) < tuple(map(int, minimum.split('.'))):
            raise ValueError(name + ' version is below ' + minimum)
    runtimes = native_run('dotnet', '--list-runtimes', env=env, capture_output=True, text=True).stdout
    if not re.search(r'^Microsoft.NETCore.App ' + re.search(r'[0-9]+', requirements['csharp'])[0] + r'\.', runtimes, re.M):
        raise ValueError('missing required .NET runtime')
    r_version = native_run('Rscript', '--version', env=env, capture_output=True, text=True)
    required_r = next(name.split('-')[1] for name in pins if name.startswith('R-')).rsplit('.', 1)[0]
    if required_r not in r_version.stdout + r_version.stderr:
        raise ValueError('R runtime does not match the candidate binary package')
    print('release: native Windows runtime preparation complete; installed checks remain mandatory')


def prepare_wine():
    # Wine runs only the x64 command and C smoke consumers.
    run('sudo', 'apt-get', 'update')
    run('sudo', 'apt-get', 'install', '-y', '--no-install-recommends', 'wine64', 'wine', 'cmake', 'ninja-build')


def prepare():
    if not os.environ.get('PM_JOB'):
        raise ValueError('run preparation through pm job')
    if os.name == 'nt':
        native()
        return
    if (platform.system(), platform.machine()) != ('Linux', 'x86_64'):
        raise ValueError('Windows cross builds require Linux x86-64')
    from release_host_tools import host_tools
    host_tools()
    if not shutil.which('llvm-lib-21') or not shutil.which('llvm-rc-21'):
        key = fetch('https://apt.llvm.org/llvm-snapshot.gpg.key', TOOLS / 'sources/llvm-snapshot.gpg.key')
        run('sudo', 'gpg', '--dearmor', '--yes', '--output', '/usr/share/keyrings/thinkthen-llvm.gpg', key)
        run('sudo', 'tee', '/etc/apt/sources.list.d/thinkthen-llvm.list', input='deb [signed-by=/usr/share/keyrings/thinkthen-llvm.gpg] https://apt.llvm.org/noble/ llvm-toolchain-noble-21 main\n', text=True)
        run('sudo', 'apt-get', 'update')
        run('sudo', 'apt-get', 'install', '-y', 'clang-21', 'lld-21', 'llvm-21', 'llvm-21-tools')
    prepare_wine()
    prepared('sh', '-eu', '-c', 'for tool in clang-cl lld-link llvm-readobj llvm-lib-21 llvm-rc-21; do command -v "$tool"; done')
    pins = selections()
    prepared('scripts/release-windows-prepare', 'inputs')
    prepared('scripts/release-windows-prepare', 'msvc', pins['REVIEWED_SDK'], pins['REVIEWED_CRT'])
    print('release: Windows cross inputs are prepared; candidate builds run offline')
