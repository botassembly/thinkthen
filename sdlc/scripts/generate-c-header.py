#!/usr/bin/env python3
"""Generate or check the committed C interface with the reviewed external tool."""
import argparse
import difflib
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import CARGO, child_env

VERSION = 'cbindgen 0.29.4'


def tool_environment():
    return child_env(keep=(*CARGO, 'LANG', 'LC_ALL', 'TMPDIR'), CARGO_NET_OFFLINE='true')


def check_tool():
    try:
        found = subprocess.check_output(['cbindgen', '--version'], text=True,
                                        env=tool_environment()).strip()
    except FileNotFoundError as error:
        raise RuntimeError(f'install the external {VERSION} CLI; it is not a product dependency') from error
    if found != VERSION:
        raise RuntimeError(f'C header generation requires {VERSION}; found {found}')


def generate():
    check_tool()
    crate = ROOT / 'libraries/c'
    env = tool_environment()
    with tempfile.TemporaryDirectory(prefix='thinkthen-c-header-') as scratch:
        metadata = Path(scratch) / 'metadata.json'
        # Supplying locked metadata prevents cbindgen's default metadata command
        # from updating a product lockfile. Neither command can use the network.
        metadata.write_bytes(subprocess.check_output(
            ['cargo', 'metadata', '--locked', '--offline', '--all-features',
             '--format-version', '1', '--manifest-path', str(crate / 'Cargo.toml')], env=env))
        return subprocess.check_output(
            ['cbindgen', '--quiet', '--metadata', str(metadata), '--config',
             str(crate / 'cbindgen.toml'), '--crate', 'thinkthen-c', str(crate)], env=env)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--check', action='store_true', help='refuse committed header drift')
    args = parser.parse_args()
    header = ROOT / 'libraries/c/include/thinkthen.h'
    generated = generate()
    if args.check:
        if header.read_bytes() != generated:
            print('C header drift: run sdlc/scripts/generate-c-header.py', file=sys.stderr)
            sys.stderr.writelines(difflib.unified_diff(
                header.read_text().splitlines(True), generated.decode().splitlines(True),
                fromfile=str(header.relative_to(ROOT)), tofile='generated from Rust'))
            return 1
        print('C header: committed declarations match Rust')
    else:
        header.write_bytes(generated)
        print('C header: generated from Rust')
    return 0


if __name__ == '__main__':
    sys.exit(main())
