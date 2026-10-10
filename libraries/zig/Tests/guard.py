"""Reject private build roots and canary credentials in distributable bytes."""
import pathlib
import sys
import tarfile

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
        chunk = chunk.replace(b'/build/home/', b'/build/')
        for needle in needles:
            if needle in chunk:
                print('rejected private byte pattern in', path.name)
                raise SystemExit(1)
print('distribution bytes free of private path and canary')
