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
import sys
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
        prepared('scripts/' + command, *(['--refresh'] if command == 'sqlite-prepare' else []))
    wheel = (TOOLS / 'duckdb/1.4.3/python-wheel').read_text().strip()
    abi = wheel.split('-')[2][2:]
    prepared('uv', 'python', 'install', f'{abi[0]}.{abi[1:]}')
    sdk_tools()
    python_frames()
    r_sources()


def sdk_tools():
    """Prepare the same Zig compiler and Ruby runtime gem for either gate."""
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


def floor_inputs():
    import fcntl
    from release_native import complete
    from release_images import image
    from runtime_floors import FLOORS
    macos = platform.system() == 'Darwin'
    prefix = TOOLS / ('floors/macos' + FLOORS['macos'] if macos else 'floors/linux-glibc' + FLOORS['linux']['glibc'])
    identity = {'floor': FLOORS['macos'] if macos else image('floor')}

    def produce(command, folder, roots, required, inputs):
        folder.parent.mkdir(parents=True, exist_ok=True)
        with (folder.parent / ('.' + folder.name + '.prepare.lock')).open('w') as lock:
            fcntl.flock(lock, fcntl.LOCK_EX)
            # Each existing producer owns these roots. Reuse its exact output
            # inventory, never another group's success marker or representative file.
            inputs = dict(identity, **inputs)
            inputs['recipe'] = hashlib.sha256((ROOT / command).read_bytes()).hexdigest()
            inputs['pins'] = hashlib.sha256((ROOT / 'scripts/release-downloads.json').read_bytes()).hexdigest()

            def outputs():
                return sorted({str(path.relative_to(folder)) for name in roots
                               for path in ([folder / name] if (folder / name).is_file() else (folder / name).rglob('*'))
                               if path.is_file()})

            if all(any(path.is_file() for path in folder.glob(name)) for name in required) and complete(folder, inputs, outputs()):
                return
            staging = Path(tempfile.mkdtemp(prefix='.' + folder.name + '.prepare-', dir=folder.parent))
            staged_floor = staging / 'floor'
            staged_floor.mkdir()
            staged = staged_floor if folder == prefix else staged_floor / folder.name
            if folder == prefix and (prefix / 'sources').is_dir():
                shutil.copytree(prefix / 'sources', staged_floor / 'sources')
            if command.endswith('prepare-r'):
                for name in ('sources', 'bin', 'Frameworks'):
                    if (prefix / name).exists():
                        (staged_floor / name).symlink_to(prefix / name, target_is_directory=True)
            environment = dict(os.environ, THINKTHEN_RUNTIME_PREFIX=str(staged_floor))
            try:
                if not macos and command.endswith('prepare-r'):
                    prepared('scripts/runtime-floor-build', command, env=environment)
                else:
                    prepared(command, env=environment)
                if not all(any(path.is_file() for path in staged.glob(name)) for name in required):
                    raise ValueError('floor producer left an incomplete output group: ' + command)
                if macos and folder == prefix:
                    # CRAN's compiled install names stay intact. These are the
                    # same launcher/configuration files the producer relocates.
                    text_paths = ['bin/R', 'bin/Rscript', 'Frameworks/R.framework/Resources/bin/R',
                                  'Frameworks/R.framework/Resources/etc/Makeconf',
                                  'Frameworks/R.framework/Resources/etc/Renviron']
                    text_paths += [str(p.relative_to(staged)) for p in (staged / 'jansson').rglob('*')
                                   if p.suffix in ('.pc', '.cmake')]
                    for name in text_paths:
                        path = staged / name
                        path.write_text(path.read_text().replace(str(staged_floor), str(prefix)))
                previous_folder = folder
                folder = staged
                artifacts = {name: hashlib.sha256((folder / name).read_bytes()).hexdigest() for name in outputs()}
                (folder / 'identity.json').write_text(json.dumps({'inputs': inputs, 'artifacts': artifacts}, sort_keys=True) + '\n')
                folder = previous_folder
                names = [*roots, 'identity.json'] if folder == prefix else ['.']
                published = []
                try:
                    for name in names:
                        target = folder / name if name != '.' else folder
                        replacement = staged / name if name != '.' else staged
                        backup = staging / 'previous' / name
                        target.parent.mkdir(parents=True, exist_ok=True)
                        backup.parent.mkdir(parents=True, exist_ok=True)
                        existed = target.exists() or target.is_symlink()
                        if existed:
                            target.rename(backup)
                        published.append((target, backup, existed))
                        replacement.rename(target)
                except BaseException:
                    for target, backup, existed in reversed(published):
                        if target.is_dir() and not target.is_symlink():
                            shutil.rmtree(target)
                        else:
                            target.unlink(missing_ok=True)
                        if existed:
                            backup.rename(target)
                    raise
            except BaseException:
                print('floor preparation failed; previous group retained; staging: ' + str(staging), file=sys.stderr)
                raise
            shutil.rmtree(staging)

    base = 'scripts/floors/prepare-macos' if macos else 'scripts/floors/prepare-linux'
    roots = (['Frameworks/R.framework', 'bin/R', 'bin/Rscript', 'include/libintl.h'] if macos else ['bin/sqlite3'])
    roots += ['jansson', 'sources']
    required = ['jansson/lib/libjansson.a', 'jansson/include/jansson.h', 'sources/jsonlite_2.0.0.tar.gz']
    required += ['Frameworks/R.framework/Resources/bin/Rscript', 'bin/R', 'bin/Rscript', 'include/libintl.h', 'sources/R.pkg',
                 'Frameworks/R.framework/Resources/bin/R',
                 'Frameworks/R.framework/Resources/etc/Makeconf',
                 'Frameworks/R.framework/Resources/etc/Renviron'] if macos else ['bin/sqlite3', 'sources/sqlite.zip']
    produce(base, prefix, roots, required, {})
    if macos:
        wheel = (TOOLS / 'duckdb/1.4.3/python-wheel').read_text().strip()
        produce('scripts/floors/prepare-duckdb', prefix / 'duckdb',
                ['static', 'cli', 'duckdb-1.4.3', wheel],
                ['static/libduckdb_static.a', 'static/libduckdb_yyjson.a', 'cli/duckdb',
                 'duckdb-1.4.3/src/include/duckdb.hpp', wheel],
                {'source_recipe': hashlib.sha256((ROOT / 'scripts/duckdb-prepare').read_bytes()).hexdigest()})
    required = ['library/jsonlite/DESCRIPTION']
    for archive in PINS['r_sources']:
        package, version = archive.removesuffix('.tar.gz').split('_')
        required += ['library/' + package + '/DESCRIPTION', 'packages/' + package + '_' + version + '[._]*']
    produce('scripts/floors/prepare-r', prefix / 'r-frames', ['library', 'packages'], required,
            {'sources': PINS['r_sources'], 'runtime_recipe': hashlib.sha256((ROOT / base).read_bytes()).hexdigest()})


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
    actions = [('host tools', host_tools)]
    if platform.system() == 'Linux':
        actions.append(('floor image', lambda: prepared('scripts/floors/prepare-image')))
    actions.extend([('language inputs', language_inputs), ('floor inputs', floor_inputs)])
    if platform.system() == 'Linux':
        actions.append(('Swift inputs', lambda: prepared('scripts/floors/prepare-swift')))
    for name, action in actions:
        action()
        print('release prepare: ' + name, flush=True)
    print('release: public host inputs are prepared; candidate builds run offline')
