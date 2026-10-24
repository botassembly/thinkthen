"""Prepare public build inputs separately from the offline candidate."""
import email
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess
import tarfile
import tempfile
import zipfile

from native_platform import current

from release_prepare_common import ROOT, TOOLS, PINS, fetch, run, prepared
from release_host_tools import host_tools


def python_frames():
    destination = TOOLS / 'python-frames/cp310'
    wheels = destination / 'wheels'
    inventory = destination / 'inventory.json'
    destination.mkdir(parents=True, exist_ok=True)
    hashes = json.loads((ROOT / 'scripts/release-python-hashes.json').read_text())
    pinned = {filename: digest for entries in hashes.values() for filename, digest in entries.items()}
    requirements = []
    for name, version in PINS['python_frames'].items():
        selected = [digest for filename, digest in hashes[name].items()
                    if filename.startswith(name.replace('-', '_') + '-' + version + '-')]
        if not selected: raise ValueError('missing pinned Python wheels: ' + name)
        requirements.append(f'{name}=={version}' + ''.join(' --hash=sha256:' + digest for digest in sorted(set(selected))))
    (destination / 'requirements.txt').write_text('\n'.join(requirements) + '\n')
    if inventory.is_file():
        rows = json.loads(inventory.read_text())['wheels']
        if {row['name']: row['version'] for row in rows} == PINS['python_frames']:
            for row in rows:
                if (row['sha256'] != pinned.get(row['file']) or
                        hashlib.sha256((wheels / row['file']).read_bytes()).hexdigest() != row['sha256']):
                    raise ValueError('cached Python wheel checksum differs')
            return
    wheels.mkdir(parents=True, exist_ok=True)
    python = prepared('uv', 'python', 'find', '--offline', '3.10', capture_output=True, text=True).stdout.strip()
    run(python, '-m', 'pip', 'download', '--only-binary=:all:', '--no-deps', '--require-hashes',
        '--dest', wheels, '-r', destination / 'requirements.txt')
    rows = []
    for wheel in sorted(wheels.glob('*.whl')):
        if hashlib.sha256(wheel.read_bytes()).hexdigest() != pinned.get(wheel.name):
            raise ValueError('unlisted or changed Python wheel: ' + wheel.name)
        with zipfile.ZipFile(wheel) as archive:
            metadata = email.message_from_bytes(archive.read(next(name for name in archive.namelist() if name.endswith('.dist-info/METADATA'))))
            licenses = [name for name in archive.namelist() if not name.endswith('/') and re.search(r'(?i)(license|copying)', name)]
        rows.append({'file': wheel.name, 'name': metadata['Name'].lower().replace('_', '-'),
                     'version': metadata['Version'], 'requires': metadata.get_all('Requires-Dist', []),
                     'licenses': licenses, 'sha256': hashlib.sha256(wheel.read_bytes()).hexdigest()})
    inventory.write_text(json.dumps({'scope': 'CPython 3.10 only; optional pandas/Polars prerequisites',
                                    'interpreter': subprocess.check_output([python, '--version'], text=True).strip(),
                                    'wheels': rows}, indent=2) + '\n')


def r_sources():
    folder = TOOLS / ('floors/r-sources/sources' if platform.system() == 'Darwin' else f'r-frames/4.3.3-{current()["platform"]}/sources')
    for name, digest in PINS['r_sources'].items():
        try:
            fetch('https://cran.r-project.org/src/contrib/' + name, folder / name, digest)
        except subprocess.CalledProcessError:
            fetch('https://cran.r-project.org/src/contrib/Archive/' + name.split('_')[0] + '/' + name, folder / name, digest)


def language_inputs():
    for command in ('cpp-prepare', 'jvm-prepare', 'csharp-prepare', 'sqlite-prepare', 'duckdb-prepare'):
        prepared('scripts/' + command)
    wheel = (TOOLS / 'duckdb/1.4.3/python-wheel').read_text().strip()
    abi = wheel.split('-')[2][2:]
    prepared('uv', 'python', 'install', f'{abi[0]}.{abi[1:]}')
    version = re.search(r'minimum_zig_version = "([^"]+)"', (ROOT / 'sdk/zig/build.zig.zon').read_text())[1]
    zig = TOOLS / 'zig' / version
    if not (zig / 'zig').is_file():
        system = 'macos' if platform.system() == 'Darwin' else 'linux'
        arch = 'aarch64' if platform.machine() in ('arm64', 'aarch64') else 'x86_64'
        url = f'https://ziglang.org/download/{version}/zig-{arch}-{system}-{version}.tar.xz'
        archive = fetch(url, TOOLS / 'sources' / Path(url).name)
        with tempfile.TemporaryDirectory(prefix='thinkthen-zig-prepare-') as temporary:
            with tarfile.open(archive) as source:
                source.extractall(temporary, filter='data')
            zig.parent.mkdir(parents=True, exist_ok=True)
            shutil.move(next(Path(temporary).iterdir()), zig)
    ruby_platform = current()['ruby']
    gems = TOOLS / 'ruby/packages'
    gems.mkdir(parents=True, exist_ok=True)
    suffix = '-gnu' if ruby_platform.endswith('-linux') else ''
    filename = f'ffi-1.17.4-{ruby_platform}{suffix}.gem'
    fetch('https://rubygems.org/downloads/' + filename, gems / filename)
    python_frames()
    r_sources()


def floor_inputs():
    from runtime_floors import FLOORS
    if platform.system() == 'Darwin':
        prefix = TOOLS / ('floors/macos' + FLOORS['macos'])
        if not (prefix / 'Frameworks/R.framework/Resources/bin/Rscript').exists():
            prepared('scripts/floors/prepare-macos')
        if not list((prefix / 'duckdb/static').glob('*.a')):
            prepared('scripts/floors/prepare-duckdb')
        if not list((prefix / 'r-frames/packages').glob('dplyr_*')):
            prepared('scripts/floors/prepare-r')
    else:
        prepared('scripts/floors/prepare-image')
        prefix = TOOLS / ('floors/linux-glibc' + FLOORS['linux']['glibc'])
        if not (prefix / 'bin/sqlite3').is_file() or not list((prefix / 'r-frames/packages').glob('dplyr_*')):
            prepared('scripts/floors/prepare-linux')
        prepared('scripts/floors/prepare-swift')


def prepare(target):
    host = (platform.system(), platform.machine().lower())
    expected = {'linux-x86_64': ('Linux', 'x86_64'), 'linux-arm64': ('Linux', 'aarch64'),
                'macos-arm64': ('Darwin', 'arm64')}
    if target == 'windows-x86_64':
        from release_windows_host import prepare as prepare_windows
        prepare_windows()
        return
    if host != expected[target]:
        raise ValueError(f'{target} needs its matching native build host')
    if not os.environ.get('PM_JOB'):
        raise ValueError('run release preparation through pm job')
    for name, action in [('host tools', host_tools), ('language inputs', language_inputs), ('floor inputs', floor_inputs)]:
        action()
        print('release prepare: ' + name, flush=True)
    print('release: public host inputs are prepared; candidate builds run offline')
