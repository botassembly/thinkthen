#!/usr/bin/env python3
"""Generate or check the committed C interface with the reviewed external tool."""
import argparse
import difflib
from pathlib import Path
import shutil
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


def generate(root=ROOT, replacements=None):
    subprocess.run([sys.executable, str(root / "sdlc/generators/results/generate.py"),
                    "--target", "c", "--check"], check=True)
    check_tool()
    crate = root / 'libraries/c'
    env = tool_environment()
    with tempfile.TemporaryDirectory(prefix='thinkthen-c-header-') as scratch:
        source = crate / 'src/lib.rs'
        if replacements is not None:
            # The version updater generates its proposed header before writing
            # any product file. Only Rust sources need staging; no build runs.
            staged = Path(scratch) / 'src'
            shutil.copytree(crate / 'src', staged)
            for name, text in replacements.items():
                if name.startswith('libraries/c/src/'):
                    (staged / name.removeprefix('libraries/c/src/')).write_text(text)
            source = staged / 'lib.rs'
        # Source mode follows modules without Cargo metadata or dependency
        # traversal, so it cannot resolve dependencies or rewrite a lockfile.
        return subprocess.check_output(
            ['cbindgen', '--quiet', '--config', str(crate / 'cbindgen.toml'), str(source)], env=env)


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
