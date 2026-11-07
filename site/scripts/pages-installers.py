#!/usr/bin/env python3
"""Serve only approved installers: the held ones, or the published release's."""
import argparse
import hashlib
from pathlib import Path, PurePosixPath
import shutil
import sys
import tarfile
import urllib.error
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'scripts'))
from release_metadata import version

LIVE = 'https://thinkthen.dev'
RELEASES = 'https://github.com/botassembly/thinkthen/releases/download'
INSTALLERS = ('install.sh', 'install.ps1')


def verified(path, expected):
    if path.is_symlink() or not path.is_file():
        raise ValueError(f'Installer must be a regular file: {path}')
    if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
        raise ValueError(f'{path.name} differs from the approved bytes')


def published():
    """Read each installer's hash from the published release. A draft has no such URL."""
    with urllib.request.urlopen(f'{RELEASES}/v{version()}/SHA256SUMS', timeout=30) as response:
        if response.status != 200:
            raise ValueError('Published SHA256SUMS did not return HTTP 200')
        lines = response.read().decode().splitlines()
    wanted = [(name, digest) for digest, name in (line.split('  ', 1) for line in lines)
              if name in INSTALLERS]
    if sorted(name for name, _ in wanted) != sorted(INSTALLERS):
        raise ValueError('Published SHA256SUMS must list each installer once')
    return dict(wanted)


def held(expected):
    return {'install.sh': expected, 'install.ps1': None}


def snapshot_published(folder, wanted):
    """Refuse before any build unless the repo's installers are the published ones."""
    for name, digest in wanted.items():
        verified(ROOT / name, digest)
    folder.mkdir(parents=True, exist_ok=False)
    for name in wanted:
        shutil.copyfile(ROOT / name, folder / name)


def snapshot(folder, wanted):
    if wanted['install.ps1'] is not None:
        snapshot_published(folder, wanted)
        return
    expected = wanted['install.sh']
    folder.mkdir(parents=True, exist_ok=False)
    with urllib.request.urlopen(LIVE + '/install.sh', timeout=30) as response:
        if response.status != 200:
            raise ValueError('Live shell installer did not return HTTP 200')
        (folder / 'install.sh').write_bytes(response.read())
    verified(folder / 'install.sh', expected)
    try:
        with urllib.request.urlopen(LIVE + '/install.ps1', timeout=30) as response:
            raise ValueError(f'PowerShell installer must remain HTTP 404, got {response.status}')
    except urllib.error.HTTPError as error:
        if error.code != 404:
            raise
    (folder / 'powershell-status').write_text('404\n')


def restore(folder, dist, wanted):
    served = {name: digest for name, digest in wanted.items() if digest is not None}
    for name, digest in served.items():
        verified(folder / name, digest)
    if 'install.ps1' not in served and (folder / 'powershell-status').read_text() != '404\n':
        raise ValueError('Missing proof that the PowerShell installer remains unavailable')
    if dist.is_symlink() or not dist.is_dir():
        raise ValueError('Pages output must be a regular directory')
    if any(path.is_symlink() for path in dist.rglob('*')):
        raise ValueError('Pages output must contain no symbolic links')
    for name in ('install.sh', 'install.ps1', 'CNAME'):
        path = dist / name
        if path.exists() and not path.is_file():
            raise ValueError(f'Pages reserved output must be a regular file: {name}')
    for name in INSTALLERS:
        (dist / name).unlink(missing_ok=True)
    for name, digest in served.items():
        shutil.copyfile(folder / name, dist / name)
        verified(dist / name, digest)
    (dist / 'CNAME').write_text('thinkthen.dev\n')
    print('Pages installers: serving only the approved bytes of ' + ', '.join(served))


def verify(artifact, wanted):
    """Inspect the uploaded Pages tar itself, without extracting its contents."""
    with tarfile.open(artifact) as archive:
        reserved = {}
        for member in archive.getmembers():
            path = PurePosixPath(member.name)
            if path.is_absolute() or '..' in path.parts or member.issym() or member.islnk():
                raise ValueError('Pages artifact must contain no links or unsafe paths')
            name = str(path)
            if name not in ('install.sh', 'install.ps1', 'CNAME'):
                continue
            if name in reserved or not member.isfile():
                raise ValueError(f'Pages reserved output must be one regular file: {name}')
            reserved[name] = archive.extractfile(member).read()
        for name, digest in wanted.items():
            if digest is None and name in reserved:
                raise ValueError(f'Unreleased installer remains in Pages artifact: {name}')
            if digest is not None and hashlib.sha256(reserved.get(name, b'')).hexdigest() != digest:
                raise ValueError(f'Pages artifact {name} differs from the approved bytes')
        if reserved.get('CNAME') != b'thinkthen.dev\n':
            raise ValueError('Pages artifact must preserve the thinkthen.dev domain')
    print('Pages artifact: approved installers verified')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=('snapshot', 'restore', 'verify'))
    parser.add_argument('snapshot', type=Path)
    parser.add_argument('dist', nargs='?', type=Path)
    parser.add_argument('--expected-sha', required=True, help='Held shell installer hash')
    parser.add_argument('--installers', choices=('held', 'release'), required=True,
                        help='release serves the repo installers once they match the published SHA256SUMS')
    args = parser.parse_args()
    wanted = published() if args.installers == 'release' else held(args.expected_sha)
    if args.mode == 'verify':
        if args.dist is not None:
            parser.error('verify takes only the Pages artifact tar')
        verify(args.snapshot, wanted)
    elif args.mode == 'snapshot':
        if args.dist is not None:
            parser.error('snapshot takes no output directory')
        snapshot(args.snapshot, wanted)
    else:
        if args.dist is None:
            parser.error('restore requires the built Pages output directory')
        restore(args.snapshot, args.dist, wanted)


if __name__ == '__main__':
    main()
