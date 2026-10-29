#!/usr/bin/env python3
"""Fetch an immutable release input and reject bytes that differ from its pin."""
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
import tempfile

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


if __name__ == '__main__':
    if len(sys.argv) != 3: sys.exit('usage: release_downloads.py URL DESTINATION')
    fetch(sys.argv[1], Path(sys.argv[2]))
