"""Pinned public host runtimes, installed only into a check's owned scratch."""
import hashlib
import os
from pathlib import Path
import sys
import zipfile
from install_check import Failure

PGDG = (
    ('postgresql-16_16.15-1.pgdg24.04+2_amd64.deb', 'abe560fc574383f520753dfaef79ab24081cda488293d5c991a9f77ac98ef18f'),
    ('postgresql-client-16_16.15-1.pgdg24.04+2_amd64.deb', 'd7258aaa5c4ea092d98730b7d27e54dbcc5493d3b56b4f526c9ddec4b71135e6'),
)


def pinned(check, url, name, digest, algorithm='sha256'):
    archive = check.fetch(url, check.root / name)
    if hashlib.new(algorithm, archive.read_bytes()).hexdigest() != digest:
        raise Failure(f'host tool {name} differs from its pinned checksum')
    return archive


def unzip(archive, destination):
    destination.mkdir()
    with zipfile.ZipFile(archive) as packed:
        entries = packed.infolist()
        for entry in entries:
            name = Path(entry.filename)
            kind = (entry.external_attr >> 16) & 0o170000
            if (name.is_absolute() or '..' in name.parts
                    or kind not in (0, 0o040000 if entry.is_dir() else 0o100000)):
                raise Failure('host tool archive contains an unsafe entry')
        packed.extractall(destination)
        # ZipFile drops Unix modes. Restore safe bits after validating every path.
        for entry in sorted(entries, key=lambda item: len(Path(item.filename).parts), reverse=True):
            mode = (entry.external_attr >> 16) & 0o777
            if os.name != 'nt' and entry.create_system == 3 and entry.external_attr >> 16:
                (destination / entry.filename).chmod(mode)
    return destination


def prepend(check, directory):
    check.env['PATH'] = f'{directory}:{check.env["PATH"]}'


def postgres_tools(check, destination):
    destination.mkdir()
    for name, digest in PGDG:
        archive = pinned(check, 'https://apt.postgresql.org/pub/repos/apt/pool/main/p/postgresql-16/' + name, name, digest)
        check.run('dpkg-deb', '-x', archive, destination)
    binary = destination / 'usr/lib/postgresql/16/bin'
    if not check.run(binary / 'postgres', '--version').startswith('postgres (PostgreSQL) 16.15 '):
        raise Failure('PostgreSQL host differs from pinned 16.15')
    prepend(check, binary)


def prepare(check):
    if check.channel == 'go':
        archive = pinned(check, 'https://go.dev/dl/go1.27.1.linux-amd64.tar.gz', 'go.tar.gz', '63d339f0da5ab53635a56f2490a7984dfe12dfcff22ad749f63edaf590168445')
        folder = check.unpack(archive, check.root / 'go-tools')
        prepend(check, folder / 'go/bin')
        if check.run('go', 'version') != 'go version go1.27.1 linux/amd64':
            raise Failure('Go host differs from pinned 1.27.1')
    elif check.channel == 'pub':
        archive = pinned(check, 'https://storage.googleapis.com/dart-archive/channels/stable/release/3.13.4/sdk/dartsdk-linux-x64-release.zip', 'dart.zip', '6487a10df5eab890d746d14a55f4c70bec3c1c0633f51804eb504cbc0fc395bb')
        folder = unzip(archive, check.root / 'dart-tools')
        # zipfile does not retain Unix modes. SDK scripts and executables need them.
        for path in (folder / 'dart-sdk/bin').rglob('*'):
            if path.is_file():
                path.chmod(path.stat().st_mode | 0o111)
        prepend(check, folder / 'dart-sdk/bin')
    elif check.channel == 'sqlite':
        archive = pinned(check, 'https://sqlite.org/2026/sqlite-tools-linux-x64-3530400.zip', 'sqlite.zip', '6eeb57e8f2aef7687f9f016a980992cf2799c8c07a87c5e21495530f91915047', 'sha3_256')
        folder = unzip(archive, check.root / 'sqlite-tools')
        (folder / 'sqlite3').chmod(0o755)
        prepend(check, folder)
        if check.run('sqlite3', '--version').split()[0] != '3.53.4':
            raise Failure('SQLite host differs from pinned 3.53.4')
    elif check.channel == 'duckdb':
        sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'databases/duckdb/tools'))
        from inputs import selected, versions
        from source_hosts import prepare as prepare_source
        identity = selected(versions()[0], 'x86_64-unknown-linux-gnu')
        # The approved source host requires Git, CMake, Ninja and a C/C++ compiler.
        source, build = check.root / 'duckdb-source', check.root / 'duckdb-host'
        prepare_source(source, identity['source_commit'], 'https://github.com/duckdb/duckdb.git', True,
                       identity['version'])
        admission = (sys.executable, Path(__file__).with_name('build-admission.py'), '--')
        check.run(*admission, 'cmake', '-S', source, '-B', build, '-G', 'Ninja',
                  '-DCMAKE_BUILD_TYPE=Release', '-DBUILD_UNITTESTS=OFF', '-DBUILD_EXTENSIONS=core_functions;json',
                  '-DDISABLE_BUILTIN_EXTENSIONS=OFF', '-DENABLE_EXTENSION_AUTOLOADING=OFF',
                  '-DENABLE_EXTENSION_AUTOINSTALL=OFF')
        check.run(*admission, 'cmake', '--build', build, '--target', 'shell', '--parallel', '2')
        prepend(check, build)
        reported = check.run('duckdb', '-noheader', '-list', '-init', '/dev/null', '-c',
                             'SELECT library_version,source_id FROM pragma_version()').split('|')
        if (len(reported) != 2 or reported[0] != identity['version']
                or reported[1] not in (identity['source_commit'][:8], identity['source_commit'][:10])):
            raise Failure(f"DuckDB host differs from pinned source {identity['source_commit']}")
