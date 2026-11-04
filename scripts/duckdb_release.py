#!/usr/bin/env python3
"""Prepare the pinned DuckDB release hosts separately from offline builds."""
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile

from native_platform import current
from release_downloads import fetch

ROOT = Path(__file__).resolve().parent.parent
TOOLS = Path.home() / '.local/share/thinkthen-build-tools'
PINS = json.loads((ROOT / 'scripts/release-tools.json').read_text())
VERSION = PINS['duckdb']


def prepare_source():
    """Share source inputs without mixing different target distributions."""
    folder = TOOLS / 'duckdb' / VERSION
    subprocess.run([sys.executable, ROOT / 'databases/duckdb/community-prepare.py', VERSION], check=True)
    source = folder / ('duckdb-' + VERSION)
    marker = folder / 'source-revision'
    revision = json.loads((ROOT / 'databases/duckdb/community-versions.json').read_text())[VERSION]['duckdb']
    if not marker.exists():
        folder.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(prefix='source-', dir=folder) as temporary:
            staged = Path(temporary) / 'tree'
            shutil.copytree(TOOLS / 'duckdb-community' / VERSION / 'duckdb', staged,
                            symlinks=True, ignore=shutil.ignore_patterns('.git'))
            if source.exists():
                shutil.rmtree(source)  # An interrupted copy has no completed revision marker.
            staged.rename(source)
            marker.write_text(revision + '\n')
    if marker.read_text().strip() != revision:
        raise ValueError('Prepared DuckDB source revision differs from its release pin')
    return folder


def prepare():
    """Use upstream's immutable source commit and checksum-pinned distributions."""
    platform = current()['platform']
    suffix = {'macos-arm64': 'osx-arm64', 'linux-x86_64': 'linux-amd64',
              'linux-arm64': 'linux-arm64', 'windows-x86_64': 'windows-amd64'}[platform]
    folder = prepare_source()
    for name, destination in [('cli', 'cli'), ('shared-libs', 'lib'), ('static-libs', 'static')]:
        filename = f'duckdb-{name}-{suffix}.tar.gz'
        archive = fetch(f'https://github.com/duckdb/duckdb/releases/download/v{VERSION}/{filename}', folder / filename)
        with tarfile.open(archive) as contents:
            contents.extractall(folder / destination, filter='data')
    if platform in PINS['duckdb_wheels']:
        url = PINS['duckdb_wheels'][platform]
        fetch(url, folder / url.rsplit('/', 1)[-1])
        (folder / 'python-wheel').write_text(url.rsplit('/', 1)[-1] + '\n')
    print(folder)


if __name__ == '__main__':
    if sys.argv[1:] == ['--version']:
        print(VERSION)
    elif not sys.argv[1:]:
        prepare()
    else:
        sys.exit('usage: duckdb_release.py [--version]')
