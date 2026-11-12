#!/usr/bin/env python3
"""Fetch an immutable release input and reject bytes that differ from its pin.

URLs pinned to one digest are sources of one input. A caller names its first
choice, and the others are tried in file order when it gives no verified bytes.
"""
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


def sources(url):
    """Return a URL's pin and every URL pinned to it, the named URL first."""
    pins = json.loads(PINS.read_text())
    return pins[url], [url, *(other for other, digest in pins.items() if digest == pins[url] and other != url)]


def fetch(url, path, digest=None):
    urls = [url]
    if digest is None:
        digest, urls = sources(url)
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
    refusal, differing = None, []
    try:
        for source in urls:
            try:
                for left in (2, 1, 0):
                    try:
                        subprocess.run(['curl', '--fail', '--location', '--silent', '--show-error', '--retry', '2',
                                        '--connect-timeout', '20', '--max-time', '600', source, '-o', str(temporary)], check=True)
                        break
                    except subprocess.CalledProcessError as error:
                        # curl's --retry leaves a transfer that closed early (exit 18) alone, and
                        # --retry-all-errors would also repeat a refusal that another source answers.
                        if error.returncode != 18 or not left: raise
            except subprocess.CalledProcessError as error:
                refusal = error
                print('release input source refused: ' + source, file=sys.stderr)
                continue
            if verified(temporary):
                temporary.replace(path)
                return path
            differing.append(source)
            print('release input source differs from its pin: ' + source, file=sys.stderr)
    finally:
        temporary.unlink(missing_ok=True)
    if differing:
        raise ValueError('release input checksum differs: ' + path.name + ' from ' + ' '.join(differing))
    # Callers with an unpinned location of their own catch this type to try it.
    raise subprocess.CalledProcessError(refusal.returncode, 'curl ' + ' '.join(urls))


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
