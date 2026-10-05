"""Validate shipped repository members, their footers and referenced legal files."""
from __future__ import annotations

import re
import sys
import tarfile
from pathlib import Path, PurePosixPath

sys.path.insert(0, str(Path(__file__).resolve().parent.parent / 'tools'))
from inputs import selected, versions  # noqa: E402
from verify_footer import FOOTER_BYTES, verify_common  # noqa: E402


def verify_repository(source: Path, target: str, release: str) -> None:
    platform = selected(versions()[0], target)['platform']
    wanted = {f'{version}/{platform}/thinkthen.duckdb_extension': version for version in versions()}
    files: dict[str, bytes] = {}
    if source.is_dir():
        for path in source.rglob('*'):
            if path.is_symlink():
                raise ValueError('DuckDB repository contains a link')
            if path.is_file():
                with path.open('rb') as stream:
                    if path.suffix == '.duckdb_extension':
                        stream.seek(-FOOTER_BYTES, 2)
                    files[path.relative_to(source).as_posix()] = stream.read()
    else:
        with tarfile.open(source, 'r:gz') as archive:
            seen = set()
            for member in archive:
                name = str(PurePosixPath(member.name))
                if name.startswith('/') or '..' in PurePosixPath(name).parts:
                    raise ValueError('DuckDB repository contains an escaping path')
                if name in seen:
                    raise ValueError('DuckDB repository contains a duplicate member')
                seen.add(name)
                if member.isdir():
                    continue
                if not member.isfile():
                    raise ValueError('DuckDB repository contains a link or special member')
                stream = archive.extractfile(member)
                if stream is None:
                    raise ValueError('DuckDB repository member cannot be read')
                if name.endswith('.duckdb_extension'):
                    stream.seek(max(0, member.size - FOOTER_BYTES))
                files[name] = stream.read()
    for name in wanted:
        if name not in files:
            raise ValueError(f'DuckDB repository is missing {name}')
    for name in files:
        if name not in wanted and name not in ('LICENSE.thinkthen', 'LICENSE.duckdb', 'NOTICE', 'DEPENDENCIES.txt') and not name.startswith('LICENSES/'):
            raise ValueError(f'DuckDB repository has unexpected member {name}')
    for name, version in wanted.items():
        fields = verify_common(files[name], platform)
        for index, expected, label in ((4, release, 'extension version'), (5, version, 'DuckDB version')):
            if fields[index] != expected.encode():
                raise ValueError(f'DuckDB footer {label} is {fields[index]!r}, expected {expected!r}')
    for name in ('LICENSE.thinkthen', 'LICENSE.duckdb', 'NOTICE', 'DEPENDENCIES.txt'):
        if not files.get(name):
            raise ValueError(f'DuckDB repository is missing legal file {name}')
    notice, inventory = (files[name].decode('utf-8') for name in ('NOTICE', 'DEPENDENCIES.txt'))
    for version in versions():
        if f'DuckDB {version} source and static archives:' not in notice or f'Pinned DuckDB {version} static archives:' not in inventory or f'Pinned DuckDB {version} source license and notice files:' not in inventory:
            raise ValueError(f'DuckDB repository lacks legal inventory for {version}')
    references = re.findall(r'\((LICENSE[^)]+)\)', notice) + re.findall(r' -> (LICENSE[^\n]+)', inventory)
    if not references or not any(name.startswith('LICENSES/rust/') for name in files):
        raise ValueError('DuckDB repository lacks third-party notices')
    for name in references:
        if not files.get(name):
            raise ValueError(f'DuckDB repository is missing referenced legal file {name}')


if __name__ == '__main__':
    if len(sys.argv) != 4:
        raise SystemExit('usage: verify_repository.py FOLDER_OR_ARCHIVE TARGET THINKTHEN_VERSION')
    try:
        verify_repository(Path(sys.argv[1]), sys.argv[2], sys.argv[3])
    except (OSError, ValueError, tarfile.TarError) as error:
        raise SystemExit(f'release-pack: {error}') from error
