"""Reject private paths and planted credentials in selected distributable bytes."""
import argparse
import fnmatch
import gzip
import os
from pathlib import Path
import re
import sys
import tarfile
import zipfile

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'sdlc/generators/results'))
from sources import tracked_files

PRIVATE = re.compile(rb'/home/|/Users/|auth\.json|-----BEGIN PRIVATE KEY-----|tt-canary-|'
                     rb'PRIVATE_PATH_MARKER|PRIVATE_KEY_MARKER|sk-private-[A-Za-z0-9]+')


class PrivacyError(AssertionError):
    """A distributable contains private bytes."""


def check_bytes(label, data):
    if PRIVATE.search(data):
        raise PrivacyError(f'private pattern in {label}')


def distribution_bytes(path):
    """Read only an explicit shipping file and its compressed members."""
    path = Path(path)
    yield path.read_bytes()
    if tarfile.is_tarfile(path):
        with tarfile.open(path) as archive:
            for member in archive:
                yield member.name.encode()
                if member.issym() or member.islnk():
                    yield member.linkname.encode()
                if member.isfile():
                    yield archive.extractfile(member).read()
    elif zipfile.is_zipfile(path):
        with zipfile.ZipFile(path) as archive:
            for member in archive.infolist():
                yield member.filename.encode()
                yield archive.read(member)
    elif path.suffix == '.gz':
        yield gzip.decompress(path.read_bytes())


def check_file(path):
    for data in distribution_bytes(path):
        check_bytes(Path(path).name, data)


def source_files(root, includes, excludes=()):
    """Select indexed source files, including edits, and never working output."""
    root = Path(root).resolve()
    return [path for path in tracked_files(root) if path.is_file()
            and any(fnmatch.fnmatchcase(path.relative_to(root).as_posix(), pattern) for pattern in includes)
            and not any(fnmatch.fnmatchcase(path.relative_to(root).as_posix(), pattern) for pattern in excludes)]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('files', type=Path, nargs='*')
    parser.add_argument('--sources', type=Path)
    parser.add_argument('--include', action='append', default=[])
    parser.add_argument('--exclude', action='append', default=[])
    parser.add_argument('--artifact-env', action='store_true')
    parser.add_argument('--manifest', type=Path)
    parser.add_argument('--prefix', type=Path)
    args = parser.parse_args()
    paths = list(args.files)
    if args.sources:
        paths += source_files(args.sources, args.include or ['*'], args.exclude)
    if args.manifest:
        if not args.prefix:
            parser.error('--manifest requires --prefix')
        members = [Path(name) for name in args.manifest.read_text().splitlines() if name]
        assert all(path.resolve().is_relative_to(args.prefix.resolve()) for path in members), 'INSTALL_MEMBER_OUTSIDE_PREFIX'
        paths += members
    if args.artifact_env:
        paths += [Path(os.environ[name]) for name in
                  ('THINKTHEN_ARTIFACT', 'THINKTHEN_DART_ARTIFACT', 'THINKTHEN_C_ARTIFACT') if os.environ.get(name)]
    assert paths, 'no source scanned'
    for path in set(paths):
        check_file(path)
    print(f'PRIVATE_SOURCE_PASS {len(set(paths))} files')


if __name__ == '__main__':
    try:
        main()
    except PrivacyError as error:
        print(error, file=sys.stderr)
        sys.exit(1)
