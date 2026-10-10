"""Assemble a local Composer-compatible archive from package source and one native library."""
import argparse
from pathlib import Path
import shutil
import tarfile


def assemble(source, library, output):
    output.mkdir(parents=True, exist_ok=False)
    for name in ('autoload.php', 'composer.json', 'LICENSE', 'README.md'):
        shutil.copyfile(source / name, output / name)
    for name in ('src', 'examples'):
        shutil.copytree(source / name, output / name)
    native = output / 'native'
    native.mkdir()
    shutil.copyfile(library, native / 'libthinkthen.so')
    archive = output.with_suffix('.tar.gz')
    with tarfile.open(archive, 'w:gz') as packed:
        for file in sorted(output.rglob('*')):
            packed.add(file, arcname=str(file.relative_to(output)), recursive=False)
    return archive


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--library', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    print(assemble(Path(__file__).resolve().parents[1], args.library, args.out))
