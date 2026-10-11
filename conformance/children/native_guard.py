"""Reject private build roots and canary credentials in distributable bytes."""
import pathlib
import sys
import tarfile

def normalized_bytes(data):
    """Release remaps use /build/home; it is not a private /home origin."""
    return data.replace(b'/build/home/', b'/build/')


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
            # release-pack remaps the builder's home; keep scanning the rest of each path.
            chunk = normalized_bytes(chunk)
            for needle in needles:
                if needle in chunk:
                    print('rejected private byte pattern in', path.name)
                    raise SystemExit(1)
    print('distribution bytes free of private path and canary')

if __name__ == '__main__':
    main()
