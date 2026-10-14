"""Reject private build roots and canary credentials in distributable bytes."""
import pathlib
import sys
import tarfile

def normalized_bytes(data):
    """Preserve every raw path marker for package privacy checks."""
    return data


def main():
    needles = (b'/home/', b'/Users/', b'tt-canary-273')
    for path in map(pathlib.Path, sys.argv[1:]):
        data = [path.read_bytes()]
        if tarfile.is_tarfile(path):
            with tarfile.open(path) as archive:
                for member in archive:
                    data.append(member.name.encode())
                    if member.isfile():
                        data.append(archive.extractfile(member).read())
        for chunk in data:
            for needle in needles:
                if needle in chunk:
                    print('rejected private byte pattern in', path.name)
                    raise SystemExit(1)
    print('distribution bytes free of private path and canary')

if __name__ == '__main__':
    main()
