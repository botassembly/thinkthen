"""Assemble the SwiftPM source package with its matching native bundle asset."""
from pathlib import Path
import argparse
import platform
import shutil
import tarfile

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--native', type=Path, required=True)
parser.add_argument('--out', type=Path, required=True)
args = parser.parse_args()
package = Path(__file__).resolve().parents[2]
out = args.out.resolve()
out.mkdir(parents=True, exist_ok=True)
for name in ('LICENSE', 'README.md', 'Package.swift'):
    shutil.copyfile(package / name, out / name)
for name in ('Sources', 'Examples', 'Tests/TypeCase'):
    shutil.copytree(package / name, out / name, dirs_exist_ok=True, ignore=shutil.ignore_patterns('*.so'))
shutil.copyfile(package.parent / 'c/include/thinkthen.h', out / 'Sources/CThinkThen/include/thinkthen.h')
triple = {'x86_64': 'x86_64-unknown-linux-gnu', 'aarch64': 'aarch64-unknown-linux-gnu'}[platform.machine()]
asset = out / 'Sources/ThinkThen/Native' / triple / 'libthinkthen.so'
asset.parent.mkdir(parents=True, exist_ok=True)
shutil.copyfile(args.native, asset)
archive = out.parent / 'thinkthen-swift-0.2.0.tar.gz'
with tarfile.open(archive, 'w:gz') as packed:
    for child in sorted(out.iterdir()): packed.add(child, './' + child.name)
print(archive)
