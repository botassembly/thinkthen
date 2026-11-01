#!/usr/bin/env python3
"""Fetch an immutable release input and reject bytes that differ from its pin."""
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
import tarfile
import tempfile
import zipfile

PINS = Path(__file__).with_name('release-downloads.json')


def fetch(url, path, digest=None):
    if digest is None:
        digest = json.loads(PINS.read_text())[url]
    if ':' in digest:
        algorithm, digest = digest.split(':', 1)
        if algorithm not in ('sha256', 'sha512') or len(digest) != {'sha256': 64, 'sha512': 128}[algorithm]:
            raise ValueError('release input needs a valid checksum algorithm: ' + url)
    if not re.fullmatch('(?:[0-9a-f]{64}|[0-9a-f]{128})', digest):
        raise ValueError('release input needs a pinned digest: ' + url)
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    algorithm = 'sha256' if len(digest) == 64 else 'sha512'

    def verified(source):
        value = hashlib.new(algorithm)
        with source.open('rb') as stream:
            for chunk in iter(lambda: stream.read(1024 * 1024), b''): value.update(chunk)
        return value.hexdigest() == digest

    if path.is_file() and verified(path):
        return path
    # A restored cache is only a hint. A changed pin or damaged cached archive
    # must fetch verified bytes before any caller extracts or executes them.
    with tempfile.NamedTemporaryFile(dir=path.parent, prefix=path.name + '.', suffix='.partial', delete=False) as stream:
        temporary = Path(stream.name)
    try:
        subprocess.run(['curl', '--fail', '--location', '--silent', '--show-error', '--retry', '2',
                        '--connect-timeout', '20', '--max-time', '600', url, '-o', str(temporary)], check=True)
        if not verified(temporary):
            raise ValueError('release input checksum differs: ' + path.name)
        temporary.replace(path)
    finally:
        temporary.unlink(missing_ok=True)

    return path


def prepare_tool(destination, executable, archive):
    """Publish one archive root into an absent or empty compiler directory."""
    compiler = destination / executable
    for parent in Path(executable).parents:
        path = destination / parent
        if path.is_symlink() or (path.exists() and not path.is_dir()):
            raise ValueError('compiler directory must be a real directory: ' + str(path))
    if compiler.is_symlink() or (compiler.exists() and not compiler.is_file()):
        raise ValueError('compiler must be a regular file: ' + str(compiler))
    if compiler.is_file():
        return compiler
    if destination.exists() and any(destination.iterdir()):
        raise ValueError('compiler directory contains conflicting files: ' + str(destination))
    source = archive()
    destination.parent.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='.compiler-', dir=destination.parent) as temporary:
        if zipfile.is_zipfile(source):
            with zipfile.ZipFile(source) as package:
                package.extractall(temporary)
        else:
            with tarfile.open(source) as package:
                package.extractall(temporary, filter='data')
        roots = list(Path(temporary).iterdir())
        if len(roots) != 1 or not roots[0].is_dir():
            raise ValueError('compiler archive must contain one directory')
        staged = roots[0] / executable
        relative = staged.relative_to(temporary)
        if not staged.is_file() or any((Path(temporary) / part).is_symlink()
                                       for part in (relative, *relative.parents)):
            raise ValueError('compiler archive must contain a regular compiler: ' + str(staged))
        if destination.exists():
            destination.rmdir()  # Never remove unknown contents, including concurrent writes.
        roots[0].rename(destination)
    return compiler


if __name__ == '__main__':
    if len(sys.argv) != 3: sys.exit('usage: release_downloads.py URL DESTINATION')
    fetch(sys.argv[1], Path(sys.argv[2]))
