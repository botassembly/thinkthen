#!/usr/bin/env python3
"""Build the pinned Windows R binary natively and verify its cross-job handoff."""
import hashlib
import json
import os
import re
import struct
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import tomllib
import zipfile

from release_prepare_common import ROOT, TOOLS, fetch, run


def revision():
    return subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=ROOT, text=True).strip()


def pins():
    from release_windows_host import selections
    from release_r_dependencies import fields, names
    selected = {name: value for name, value in selections()['INPUTS'].items()
                if name.startswith('R-') or re.fullmatch(r'[A-Za-z][A-Za-z0-9.]*_[0-9].*\.zip', name)}
    available = {name.split('_')[0] for name in selected if '_' in name}
    missing = names(fields((ROOT / 'sdk/r/DESCRIPTION').read_bytes()), ('Imports',)) - available
    if missing:
        raise ValueError('Missing pinned Windows R binary inputs: ' + ', '.join(sorted(missing)))
    return selected


def dependencies(folder, r=None):
    from release_r_dependencies import bundle, fields
    selected = pins()
    installer = next(name for name in selected if name.startswith('R-'))
    r = r or TOOLS / 'windows' / installer.removesuffix('-win.exe')
    standard = []
    for path in (r / 'library').glob('*/DESCRIPTION'):
        info = fields(path.read_bytes())
        if info.get('Priority'):
            standard.append(info['Package'])
    if not standard:
        raise ValueError('Prepare the pinned R runtime before resolving Windows R dependencies')
    archives = []
    for name, (_, digest) in selected.items():
        if not name.endswith('.zip'):
            continue
        path = TOOLS / 'windows' / name
        if hashlib.sha256(path.read_bytes()).hexdigest() != digest:
            raise ValueError('Pinned Windows R binary checksum differs: ' + name)
        archives.append(path)
    result = bundle(ROOT / 'sdk/r/DESCRIPTION', archives, folder,
                    installer.removeprefix('R-').removesuffix('-win.exe'), standard)
    shutil.copytree(r / 'share/licenses', Path(folder) / 'licenses', dirs_exist_ok=True)
    (Path(folder) / 'runtime.json').write_text(json.dumps({'version': result['r'],
        'standard_packages': sorted(standard)}, indent=2) + '\n')
    return result


def version():
    return tomllib.loads((ROOT / 'Cargo.toml').read_text())['workspace']['package']['version']


def files():
    return (f'thinkthen_{version()}.zip', 'artifacts.json', 'prerequisites.zip')


def verified(folder=None):
    folder = Path(folder or os.environ.get('THINKTHEN_WINDOWS_R_PACKAGE', TOOLS / 'windows-r' / revision()))
    manifest = json.loads((folder / 'manifest.json').read_text())
    if (manifest.get('revision') != revision() or manifest.get('target') != 'x86_64-pc-windows-msvc'
            or manifest.get('inputs') != {name: digest for name, (_, digest) in pins().items()}
            or set(manifest.get('files', {})) != set(files())):
        raise ValueError('Windows R package provenance differs from this source and pinned inputs')
    for name in files():
        path = folder / name
        if path.is_symlink() or hashlib.sha256(path.read_bytes()).hexdigest() != manifest['files'][name]:
            raise ValueError('Windows R package checksum differs: ' + name)
    return folder


def bundle_dependencies(source, folder):
    """Consume only prerequisites covered by the native producer's manifest."""
    source = verified(source)
    with zipfile.ZipFile(source / 'prerequisites.zip') as archive:
        archive.extractall(folder)


def runtime():
    if os.name != 'nt':
        raise ValueError('Windows R preparation requires native Windows')
    selected = pins()
    installer = next(name for name in selected if name.startswith('R-'))
    r = TOOLS / 'windows' / installer.removesuffix('-win.exe')
    for name, (url, digest) in selected.items():
        fetch(url, TOOLS / 'windows' / name, digest)
    if not (r / 'bin/x64/R.dll').is_file():
        run(TOOLS / 'windows' / installer, '/VERYSILENT', '/SUPPRESSMSGBOXES', '/NORESTART', '/SP-', '/DIR=' + str(r))
    with tempfile.TemporaryDirectory(prefix='thinkthen-r-dependencies-') as temporary:
        inventory = dependencies(temporary, r)
        for dependency in inventory['install_order']:
            with zipfile.ZipFile(Path(temporary) / dependency) as archive:
                archive.extractall(r / 'library')
    return r


def prepare():
    r = runtime()
    from release_rust import prepare as rust
    rust('x86_64-pc-windows-msvc')
    env = environment(r)
    run('cargo', 'fetch', '--locked', '--target', 'x86_64-pc-windows-msvc', env=dict(env, CARGO_NET_OFFLINE='false'))
    return r


def environment(r):
    from release_windows_host import environment as msvc
    env = msvc()
    channel = tomllib.loads((ROOT / 'rust-toolchain.toml').read_text())['toolchain']['channel']
    env.update(R_HOME=str(r), R_INCLUDE_DIR=str(r / 'include'), RUSTUP_TOOLCHAIN=channel,
               CARGO_NET_OFFLINE='true', RUSTFLAGS='-C target-feature=+crt-static',
               R_LIBS_USER='', R_LIBS_SITE='')
    env.pop('CARGO_ENCODED_RUSTFLAGS', None)
    env['PATH'] = str(r / 'bin/x64') + os.pathsep + env['PATH']
    return env



def import_definition(binary, report):
    """Preserve exported data identity when creating the MSVC import library."""
    data = Path(binary).read_bytes()
    pe = struct.unpack_from('<I', data, 0x3c)[0]
    if data[:2] != b'MZ' or data[pe:pe + 4] != b'PE\0\0':
        raise ValueError('R import source is not a PE image')
    count = struct.unpack_from('<H', data, pe + 6)[0]
    section_table = pe + 24 + struct.unpack_from('<H', data, pe + 20)[0]
    sections = []
    for index in range(count):
        section = section_table + 40 * index
        size, address, raw_size = struct.unpack_from('<III', data, section + 8)
        flags = struct.unpack_from('<I', data, section + 36)[0]
        sections.append((address, address + max(size, raw_size), bool(flags & 0x20000000)))
    exports = []
    for block in re.findall(r'Export \{(.*?)\}', report, flags=re.S):
        name = re.search(r'^\s*Name: (\S+)\s*$', block, flags=re.M)
        address = re.search(r'^\s*RVA: (0x[0-9A-Fa-f]+)\s*$', block, flags=re.M)
        if not name or not address:
            raise ValueError('R export lacks a name or RVA')
        rva = int(address[1], 16)
        executable, = [code for start, end, code in sections if start <= rva < end]
        exports.append(name[1] + ('' if executable else ' DATA'))
    if not exports:
        raise ValueError('pinned Windows R DLL has no exported symbols')
    return 'LIBRARY R.dll\nEXPORTS\n' + '\n'.join(exports) + '\n'


def build(folder):
    if os.name != 'nt' or not os.environ.get('PM_JOB'):
        raise ValueError('build Windows R through a native Windows release job')
    selected = pins()
    installer = next(name for name in selected if name.startswith('R-'))
    r = TOOLS / 'windows' / installer.removesuffix('-win.exe')
    for name, (_, digest) in selected.items():
        if hashlib.sha256((TOOLS / 'windows' / name).read_bytes()).hexdigest() != digest:
            raise ValueError('prepared Windows R input differs: ' + name)
    env = environment(r)
    folder = Path(folder).resolve()
    folder.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='thinkthen-r-dependencies-') as temporary:
        dependencies(temporary, r)
        with zipfile.ZipFile(folder / 'prerequisites.zip', 'w', zipfile.ZIP_DEFLATED) as archive:
            for path in sorted(Path(temporary).rglob('*')):
                if path.is_file(): archive.write(path, path.relative_to(temporary).as_posix())
    # Generate the MSVC COFF import library from the exact pinned R DLL.
    report = subprocess.check_output(['llvm-readobj', '--coff-exports', str(r / 'bin/x64/R.dll')], text=True, env=env)
    (r / 'lib').mkdir(exist_ok=True)
    definition = r / 'lib/R.def'
    definition.write_text(import_definition(r / 'bin/x64/R.dll', report))
    run('lld-link', '/lib', '/machine:x64', '/def:' + str(definition), '/out:' + str(r / 'lib/R.lib'), env=env)
    shutil.copy2(r / 'lib/R.lib', r / 'bin/x64/R.lib')
    result = run('cargo', 'build', '--offline', '--locked', '--release', '--target', 'x86_64-pc-windows-msvc',
                 '--message-format=json-render-diagnostics', '-p', 'thinkthen-r', env=env, capture_output=True, text=True)
    messages = []
    for line in result.stdout.splitlines():
        item = json.loads(line)
        if item.get('reason') == 'compiler-artifact':
            item['filenames'] = [name.replace('\\', '/') for name in item['filenames']]
        messages.append(json.dumps(item))
    (folder / 'artifacts.json').write_text('\n'.join(messages) + '\n')
    target = Path(env.get('CARGO_TARGET_DIR', ROOT / 'target')).resolve() / 'x86_64-pc-windows-msvc/release'
    with tempfile.TemporaryDirectory(prefix='thinkthen-windows-r-') as temporary:
        stage = Path(temporary)
        source = stage / 'source'
        shutil.copytree(ROOT / 'sdk/r', source, ignore=shutil.ignore_patterns('native', 'src', 'tests'))
        library = stage / 'library'
        library.mkdir()
        run(r / 'bin/x64/R.exe', 'CMD', 'INSTALL', '--no-libs', '--no-test-load', '--library=' + str(library), source, env=env)
        installed = library / 'thinkthen'
        dll = installed / 'libs/x64/thinkthen.dll'
        dll.parent.mkdir(parents=True, exist_ok=True)
        from release_windows_native import system_libraries
        run('clang-cl', '/nologo', '/O2', '/MT', '/TC', '/std:c11', '/W4', '/WX', '/Brepro', '/LD',
            '/I' + str(r / 'include'), ROOT / 'sdk/r/src/entrypoint.c', '/Fe:' + str(dll),
            '/link', '/Brepro', target / 'thinkthen_r.lib', r / 'lib/R.lib', *system_libraries(),
            '/export:R_init_thinkthen', env=env, cwd=stage)
        # Prove that native R can load the delivered library before transport.
        run(r / 'bin/x64/Rscript.exe', '--vanilla', '-e',
            'a <- commandArgs(TRUE); library(thinkthen, lib.loc=a[[1]]); stopifnot(packageVersion("thinkthen") == a[[2]])',
            library, version(), env=env)
        # Linker side products are construction artifacts, never package members.
        for suffix in ('.lib', '.exp'):
            dll.with_suffix(suffix).unlink(missing_ok=True)
        with zipfile.ZipFile(folder / files()[0], 'w', zipfile.ZIP_DEFLATED) as archive:
            for path in sorted(library.rglob('*')):
                if path.is_file(): archive.write(path, path.relative_to(library).as_posix())
    manifest = {'revision': revision(), 'target': 'x86_64-pc-windows-msvc',
                'inputs': {name: digest for name, (_, digest) in selected.items()},
                'rust': subprocess.check_output(['rustc', '--version'], text=True, env=env).strip(),
                'compiler': subprocess.check_output(['clang-cl', '--version'], text=True, env=env).splitlines()[0],
                'r': subprocess.check_output([str(r / 'bin/x64/Rscript.exe'), '--version'], text=True, env=env).strip(),
                'files': {name: hashlib.sha256((folder / name).read_bytes()).hexdigest() for name in files()}}
    (folder / 'manifest.json').write_text(json.dumps(manifest, sort_keys=True, indent=2) + '\n')
    verified(folder)
    print('Native Windows R binary and exact-source inventory are ready')


if __name__ == '__main__':
    if sys.argv[1:] == ['prepare']:
        prepare()
    elif len(sys.argv) == 3 and sys.argv[1] == 'build':
        build(sys.argv[2])
    else:
        sys.exit('usage: release_windows_r.py prepare | build DESTINATION')
