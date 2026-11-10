"""Prepare the cross builder or the native Windows installed-package checker."""
import hashlib
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
import tarfile

from release_downloads import prepare_tool
from release_prepare_common import ROOT, TOOLS, PINS, fetch, run, prepared


# Official archive/hash: https://www.python.org/downloads/release/python-31317/
PYTHON_EMBED_URL = 'https://www.python.org/ftp/python/3.13.17/python-3.13.17-embed-amd64.zip'


def prepare_embedded_python(output):
    """Prepare a private standard-library runtime without pip or an installer."""
    archive = fetch(PYTHON_EMBED_URL, output / 'python.zip')
    with zipfile.ZipFile(archive) as bundle:
        names = bundle.namelist()
        if any('/' in name or '\\' in name or name in ('.', '..') for name in names):
            raise ValueError('Embedded Python archive must contain only flat files')
        required = {'python.exe', 'python313.dll', 'python313.zip', 'python313._pth'}
        if not required.issubset(names):
            raise ValueError('Embedded Python runtime is incomplete')
        original = bundle.read('python313._pth')
        paths = [line.strip() for line in original.decode('utf-8-sig').splitlines()
                 if line.strip() and not line.lstrip().startswith('#')]
        if paths != ['python313.zip', '.']:
            raise ValueError('Embedded Python has unexpected import paths')
        files = {name: hashlib.sha256(bundle.read(name)).hexdigest() for name in names}
    # Keep site disabled and add only these two owned module directories.
    configured = original + b'\n../source/scripts\n../capture/scripts\n'
    (output / 'python313._pth').write_bytes(configured)
    files['python313._pth'] = hashlib.sha256(configured).hexdigest()
    return {'url': PYTHON_EMBED_URL, 'archive_sha256': hashlib.sha256(archive.read_bytes()).hexdigest(),
            'original_pth_sha256': hashlib.sha256(original).hexdigest(), 'files': files}


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


def msys2_home():
    return TOOLS / 'windows-check/msys2' / PINS['windows_msys2']['release']


def prepare_msys2():
    """Install the exact compiler distribution with its complete package closure."""
    pin = PINS['windows_msys2']
    parent = TOOLS / 'windows-check'
    archive = fetch(pin['archive'], parent / 'msys2.7z')
    packages = fetch(pin['packages'], parent / 'msys2.packages.txt')
    destination = msys2_home()
    # Re-extract verified bytes, including on a warm host with modified tools.
    # The archive root is the MSYS2 root, as in ruby/setup-msys2-gcc's producer.
    run('7z', 'x', '-y', '-aoa', archive, '-o' + str(destination))
    env = dict(os.environ, MSYSTEM='UCRT64', MSYS2_PATH_TYPE='inherit')
    bash = destination / 'usr/bin/bash.exe'
    actual = run(bash, '-lc', 'pacman -Q', env=env, capture_output=True, text=True).stdout
    if actual.splitlines() != packages.read_text().splitlines():
        raise ValueError('Prepared MSYS2 package closure differs from its pinned distribution')
    compiler = destination / 'ucrt64/bin/gcc.exe'
    owner = run(bash, '-lc', 'pacman -Qqo /ucrt64/bin/gcc.exe', env=env, capture_output=True, text=True).stdout.strip()
    version = run(compiler, '-dumpfullversion', capture_output=True, text=True).stdout.strip()
    target = run(compiler, '-dumpmachine', capture_output=True, text=True).stdout.strip()
    if owner != 'mingw-w64-ucrt-x86_64-gcc' or version != pin['gcc'].rsplit('-', 1)[0] or target != 'x86_64-w64-mingw32':
        raise ValueError('Prepared MSYS2 GCC owner, version or architecture differs')
    print('release: pinned MSYS2 GCC ' + pin['gcc'] + ': ' + str(compiler))


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
    paths = [TOOLS / 'windows/R-4.6.1/bin/x64', TOOLS / 'windows-check/bin', TOOLS / 'zig' / zig_version(), msys2_home() / 'ucrt64/bin']
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


def prepare_swift(env, folder):
    if swift_components(env) is None:
        # fetch verifies the release-downloads.json SHA256 before the signature
        # check. Its version matches the Linux Swift closure.
        version = swift_version()
        name = f'swift-{version}-RELEASE-windows10.exe'
        installer = fetch(f'https://download.swift.org/swift-{version}-release/windows10/swift-{version}-RELEASE/{name}', folder / name)
        run('powershell', '-NoProfile', '-Command',
            'if ((Get-AuthenticodeSignature -LiteralPath $args[0]).Status -ne "Valid") {exit 1}', installer)
        run(installer, '-quiet', '-norestart')
        env = environment()
        if swift_components(env) is None:
            raise ValueError('Pinned Swift installation is missing after preparation')
    return env


def native():
    from release_windows_r import runtime
    runtime()
    pins = selections()['INPUTS']
    folder = TOOLS / 'windows-check'
    folder.mkdir(parents=True, exist_ok=True)
    binary = folder / 'bin'
    binary.mkdir(exist_ok=True)
    from duckdb_release import VERSION
    filename = 'duckdb-cli-windows-amd64.tar.gz'
    url = f'https://github.com/duckdb/duckdb/releases/download/v{VERSION}/{filename}'
    with tarfile.open(fetch(url, folder / filename)) as archive:
        archive.extractall(binary, filter='data')
    release = zig_version()
    zig = TOOLS / 'zig' / release
    def archive():
        url = f'https://ziglang.org/download/{release}/zig-x86_64-windows-{release}.zip'
        return fetch(url, folder / 'zig.zip')
    compiler = prepare_tool(zig, 'zig.exe', archive)
    if run(compiler, 'version', capture_output=True, text=True).stdout.strip() != release:
        raise ValueError('prepare the pinned Zig compiler')
    prepare_msys2()
    env = environment()
    env = prepare_swift(env, folder)
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


def prepare_cross_build_tools():
    # CMake and Ninja remain build prerequisites; execution uses native Windows.
    run('sudo', 'apt-get', 'update')
    run('sudo', 'apt-get', 'install', '-y', '--no-install-recommends', 'cmake', 'ninja-build')


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
    prepare_cross_build_tools()
    prepared('sh', '-eu', '-c', 'for tool in clang-cl lld-link llvm-readobj llvm-lib-21 llvm-rc-21; do command -v "$tool"; done')
    pins = selections()
    prepared('scripts/release-windows-prepare', 'inputs')
    from release_windows_frames import prepare as prepare_frames
    prepare_frames()
    prepared('scripts/release-windows-prepare', 'msvc', pins['REVIEWED_SDK'], pins['REVIEWED_CRT'])
    print('release: Windows cross inputs are prepared; candidate builds run offline')
