"""Reject private build roots and canary credentials in distributable bytes."""
import pathlib
import sys
import tarfile
import gzip
import zipfile

def normalized_bytes(data):
    """Preserve every raw path marker for package privacy checks."""
    return data


def distribution_bytes(path):
    """Inspect an explicitly selected shipping file and its compressed members."""
    path = pathlib.Path(path)
    yield path.read_bytes()
    if tarfile.is_tarfile(path):
        with tarfile.open(path) as archive:
            for member in archive:
                yield member.name.encode()
                if member.isfile():
                    yield archive.extractfile(member).read()
    elif zipfile.is_zipfile(path):
        with zipfile.ZipFile(path) as archive:
            for name in archive.namelist():
                yield name.encode()
                yield archive.read(name)
    elif path.suffix == '.gz':
        yield gzip.decompress(path.read_bytes())


def main():
    needles = (b'/home/', b'/Users/', b'tt-canary-273')
    for path in map(pathlib.Path, sys.argv[1:]):
        for chunk in distribution_bytes(path):
            for needle in needles:
                if needle in chunk:
                    print('rejected private byte pattern in', path.name)
                    raise SystemExit(1)
    print('distribution bytes free of private path and canary')

if __name__ == '__main__':
    main()
