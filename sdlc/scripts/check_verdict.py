#!/usr/bin/env python3
"""Apply the declared platform support to every check result."""
import argparse
import json
import os
from pathlib import Path
import platform

ROOT = Path(__file__).resolve().parents[2]


def unsupported(surface):
    manifest = json.loads((ROOT / 'sdlc/tools.json').read_text())
    family, _, name = surface.partition('/')
    if family == 'libraries':
        entry = manifest['libraries'].get(name)
        platforms = entry['check_platforms'] if entry else None
    elif family == 'databases':
        entry = manifest['databases'].get(name)
        platforms = entry
    else:
        return None
    arch = {'aarch64': 'arm64'}.get(platform.machine(), platform.machine())
    host = platform.system().lower() + '-' + arch
    if platforms is not None and host not in platforms:
        return f'sdlc/tools.json {family}.{name} = {json.dumps(entry, sort_keys=True)}; host={host}'
    return None


def verdict(surface, code, prefix='surfaces'):
    if code == 0:
        print(f'{prefix}: pass {surface}')
        return True
    entry = unsupported(surface) if code == 77 else None
    if entry:
        print(f'{prefix}: unsupported {surface} ({entry})')
        return True
    reason = 'not run' if code == 77 else f'exit {code}'
    print(f'{prefix}: FAIL {surface} ({reason})')
    return False


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('operation', choices=('platform', 'verdict'))
    parser.add_argument('surface')
    parser.add_argument('code', nargs='?', type=int)
    parser.add_argument('--prefix', default='surfaces')
    args = parser.parse_args()
    if args.operation == 'verdict':
        if args.code is None:
            parser.error('verdict requires an exit code')
        return 0 if verdict(args.surface, args.code, args.prefix) else 1
    profile = os.environ.get('THINKTHEN_TEST_PROFILE', 'routine')
    if profile not in ('routine', 'full', 'smoke', 'stress'):
        print(f'{args.surface}: unknown test profile: {profile}')
        return 2
    for name in ('THINKTHEN_ARTIFACT', 'THINKTHEN_C_ARTIFACT', 'THINKTHEN_COMMAND'):
        value = os.environ.get(name)
        if value and not Path(value).is_file():
            print(f'{args.surface}: missing input {name}: {value}')
            return 1
    entry = unsupported(args.surface)
    if entry:
        print(f'{args.surface}: not run ({entry})')
        return 77
    return 0


if __name__ == '__main__':
    raise SystemExit(main())
