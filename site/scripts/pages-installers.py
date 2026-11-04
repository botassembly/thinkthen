#!/usr/bin/env python3
"""Keep the live installers unchanged when deploying documentation before release."""
import argparse
import hashlib
from pathlib import Path
import shutil
import urllib.error
import urllib.request


LIVE = 'https://thinkthen.dev'


def verified(path, expected):
    if path.is_symlink() or not path.is_file():
        raise ValueError(f'Installer must be a regular file: {path}')
    if hashlib.sha256(path.read_bytes()).hexdigest() != expected:
        raise ValueError('Live installer differs from the approved pre-release bytes')


def snapshot(folder, expected):
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


def restore(folder, dist, expected):
    verified(folder / 'install.sh', expected)
    if (folder / 'powershell-status').read_text() != '404\n':
        raise ValueError('Missing proof that the PowerShell installer remains unavailable')
    if dist.is_symlink() or not dist.is_dir():
        raise ValueError('Pages output must be a regular directory')
    if any(path.is_symlink() for path in dist.rglob('*')):
        raise ValueError('Pages output must contain no symbolic links')
    for name in ('install.sh', 'install.ps1', 'CNAME'):
        path = dist / name
        if path.exists() and not path.is_file():
            raise ValueError(f'Pages reserved output must be a regular file: {name}')
    shutil.copyfile(folder / 'install.sh', dist / 'install.sh')
    (dist / 'install.ps1').unlink(missing_ok=True)
    (dist / 'CNAME').write_text('thinkthen.dev\n')
    verified(dist / 'install.sh', expected)
    if (dist / 'install.ps1').exists():
        raise ValueError('Unreleased PowerShell installer remains in Pages output')
    print('Pages installers: approved shell bytes preserved; PowerShell installer absent')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('mode', choices=('snapshot', 'restore'))
    parser.add_argument('snapshot', type=Path)
    parser.add_argument('dist', nargs='?', type=Path)
    parser.add_argument('--expected-sha', required=True)
    args = parser.parse_args()
    if args.mode == 'snapshot':
        if args.dist is not None:
            parser.error('snapshot takes no output directory')
        snapshot(args.snapshot, args.expected_sha)
    else:
        if args.dist is None:
            parser.error('restore requires the built Pages output directory')
        restore(args.snapshot, args.dist, args.expected_sha)


if __name__ == '__main__':
    main()
