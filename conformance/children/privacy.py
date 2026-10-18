"""Reject private paths and planted credentials in selected distributable bytes."""
import argparse
import fnmatch
import gzip
import io
import json
import os
from pathlib import Path
import re
import sys
import struct
import tarfile
import zipfile

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'sdlc/generators/results'))
from sources import classification, generated_paths, source_owners, tracked_files

PRIVATE = re.compile(rb'(?P<home_path>/home/)|(?P<users_path>/Users/)|(?P<auth_file>auth\.json)|'
                     rb'(?P<private_key>-----BEGIN PRIVATE KEY-----)|(?P<canary>tt-canary-)|'
                     rb'(?P<path_marker>PRIVATE_PATH_MARKER)|(?P<key_marker>PRIVATE_KEY_MARKER)|'
                     rb'(?P<secret_key>sk-private-[A-Za-z0-9]+)')

# Exact crate member and rule pairs; every other match still fails.
ALLOWANCES = {
    ('src/config/tests.rs', 'home_path'): rb'/home/person(?=[/"\\])',  # Synthetic config home used by path tests.
    ('src/config/tests.rs', 'users_path'): rb'/Users/person(?=[/"\\])',  # Synthetic macOS home used by path tests.
    ('tests/library/ca_bundle.rs', 'private_key'):
        rb'-----BEGIN PRIVATE KEY-----\\nAA==\\n-----END PRIVATE KEY-----\\n',  # Malformed PEM rejection case.
    ('tests/library/tls/localhost.key', 'private_key'): None,  # Public loopback TLS fixture.
    ('tests/library/tls/other-host.key', 'private_key'): None,  # Public wrong-host TLS fixture.
}


class PrivacyError(AssertionError):
    """A distributable contains private bytes."""


def check_bytes(label, data, *, crate_member=None):
    for match in PRIVATE.finditer(data):
        pair = (crate_member, match.lastgroup)
        if pair in ALLOWANCES:
            allowed = ALLOWANCES[pair]
            if allowed is None or re.match(allowed, data[match.start():]):
                continue
        raise PrivacyError(f'private pattern in {label}')


def crate_member(name):
    """Recognize exact crate paths in the checkout or Cargo's versioned root."""
    path = Path(name)
    if path.as_posix() != str(name) or '..' in path.parts:
        return None
    repo = Path(__file__).resolve().parents[2]
    if path.is_absolute():
        try:
            return path.relative_to(repo / 'crates/thinkthen').as_posix()
        except ValueError:
            # Cargo unpack directories retain the archive's exact versioned root.
            path = Path(*path.parts[1:])
            for parent in path.parents:
                if re.fullmatch(r'thinkthen-[0-9]+\.[0-9]+\.[0-9]+(?:[-+][A-Za-z0-9.-]+)?', parent.name):
                    path = Path(parent.name) / path.relative_to(parent)
                    break
    parts = path.parts
    if len(parts) > 1 and re.fullmatch(r'thinkthen-[0-9]+\.[0-9]+\.[0-9]+(?:[-+][A-Za-z0-9.-]+)?', parts[0]):
        return Path(*parts[1:]).as_posix()
    return None


def distribution_bytes(path):
    """Scan member content separately; keep all container metadata under every rule."""
    path = Path(path)
    raw = bytearray(path.read_bytes())
    yield None, path.name.encode()
    if tarfile.is_tarfile(path):
        try:
            archive = tarfile.open(path, 'r:')
            stored = True
        except tarfile.ReadError:
            archive = tarfile.open(path)
            stored = False
        with archive:
            # Keep the entire decoded stream, including ignored headers and trailers.
            archive.fileobj.seek(0)
            decoded = bytearray(archive.fileobj.read())
        with tarfile.open(fileobj=io.BytesIO(decoded), mode='r:') as archive:
            for member in archive:
                yield None, member.name.encode() + member.linkname.encode() + repr(member.pax_headers).encode()
                if member.isfile():
                    data = archive.extractfile(member).read()
                    yield crate_member(member.name), data
                    start = member.offset_data
                    # Sparse or inconsistent layouts must retain their raw coverage.
                    if decoded[start:start + len(data)] == data:
                        decoded[start:start + len(data)] = b'\0' * len(data)
        yield None, decoded
        if stored:
            return
    elif zipfile.is_zipfile(path):
        with zipfile.ZipFile(path) as archive:
            for member in archive.infolist():
                yield None, member.filename.encode() + member.comment + member.extra
                yield crate_member(member.filename), archive.read(member)
                if member.compress_type == zipfile.ZIP_STORED:
                    name_size, extra_size = struct.unpack_from('<HH', raw, member.header_offset + 26)
                    start = member.header_offset + 30 + name_size + extra_size
                    data = archive.read(member)
                    if raw[start:start + len(data)] == data:
                        raw[start:start + len(data)] = b'\0' * len(data)
    elif path.suffix == '.gz':
        yield None, gzip.decompress(raw)
    else:
        yield crate_member(path.resolve()), raw
        return
    # Stored payloads were checked above; trailers, headers and compressed bytes still have no allowance.
    yield None, raw


def check_file(path):
    for member, data in distribution_bytes(path):
        check_bytes(Path(path).name, data, crate_member=member)


def source_files(root, includes, excludes=()):
    """Select indexed source files, including edits, and never working output."""
    root = Path(root).resolve()
    return [path for path in tracked_files(root) if path.is_file()
            and any(fnmatch.fnmatchcase(path.relative_to(root).as_posix(), pattern) for pattern in includes)
            and not any(fnmatch.fnmatchcase(path.relative_to(root).as_posix(), pattern) for pattern in excludes)]


def public_files(root):
    """Use the existing surface ownership to omit test fixtures from public source."""
    root = Path(root).resolve()
    repo = Path(__file__).resolve().parents[2]
    manifest = json.loads((repo / 'conformance/library-surface.json').read_text())
    relative_root = root.relative_to(repo).as_posix()
    name = next((name for name, config in source_owners(manifest).items() if config['root'] == relative_root), None)
    generated = generated_paths()
    result = []
    for path in tracked_files(root):
        relative = path.relative_to(root)
        public = relative.as_posix() in {'README.md', 'README-PUBLIC.md', 'LICENSE', 'Cargo.toml', 'package.json',
                                   'pyproject.toml', 'source-package.json'} or relative.parts[0] in {'examples', 'Examples'}
        if name is not None:
            public |= classification(path.relative_to(repo).as_posix(), name, manifest, generated) in {'handwritten', 'generated'}
        else:
            public |= relative.parts[0] == 'src' or relative.as_posix() == 'client.py'
        if path.is_file() and public:
            result.append(path)
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('files', type=Path, nargs='*')
    parser.add_argument('--sources', type=Path)
    parser.add_argument('--public', action='store_true')
    parser.add_argument('--include', action='append', default=[])
    parser.add_argument('--exclude', action='append', default=[])
    parser.add_argument('--artifact-env', action='store_true')
    parser.add_argument('--manifest', type=Path)
    parser.add_argument('--prefix', type=Path)
    args = parser.parse_args()
    paths = list(args.files)
    if args.sources:
        paths += public_files(args.sources) if args.public else source_files(args.sources, args.include or ['*'], args.exclude)
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
