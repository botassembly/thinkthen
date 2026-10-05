"""Validate selected source and exact static archive membership before compilation."""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import re
import sys
import zipfile
from pathlib import Path

HERE = Path(__file__).resolve().parent
spec = importlib.util.spec_from_file_location('release_archive_tree', HERE.parents[2] / 'sdlc/scripts/release-archive-tree.py')
archive_tree = importlib.util.module_from_spec(spec)
spec.loader.exec_module(archive_tree)


def inventory(path: Path) -> dict[str, str]:
    if path.stat().st_size > 16384:
        raise ValueError('DuckDB archive manifest is too large')
    result = {}
    for line in path.read_text().splitlines():
        match = re.fullmatch(r'([0-9a-f]{64}) (lib[A-Za-z0-9_.-]+\.a)', line)
        if not match or match[2] in result:
            raise ValueError('malformed or duplicate DuckDB archive manifest line')
        result[match[2]] = match[1]
    if not result:
        raise ValueError('DuckDB archive manifest is empty')
    return result


def static_archives(folder: Path, manifest: Path, zip_path: Path | None = None) -> None:
    expected = inventory(manifest)
    actual = {p.name for p in folder.iterdir() if p.name != 'duckdb.h'}
    if actual != set(expected):
        raise ValueError('DuckDB static archive set differs from its manifest')
    for name, digest in expected.items():
        path = folder / name
        if path.is_symlink() or not path.is_file() or hashlib.sha256(path.read_bytes()).hexdigest() != digest:
            raise ValueError(f'DuckDB archive {name} differs from the pinned release')
    if zip_path is not None:
        with zipfile.ZipFile(zip_path) as archive:
            names = archive.namelist()
            if len(names) != len(set(names)) or set(names) != set(expected) | {'duckdb.h'}:
                raise ValueError('DuckDB static ZIP membership differs from its manifest')
            for name in names:
                path = folder / name
                if path.is_symlink() or not path.is_file() or path.read_bytes() != archive.read(name):
                    raise ValueError(f'DuckDB extracted ZIP member differs: {name}')


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument('--source', type=Path)
    parser.add_argument('--commit')
    parser.add_argument('--static', required=True, type=Path)
    parser.add_argument('--manifest', required=True, type=Path)
    parser.add_argument('--zip', type=Path)
    args = parser.parse_args()
    if args.source:
        archive_tree.raw_git(args.source, args.commit)
    static_archives(args.static, args.manifest, args.zip)


if __name__ == '__main__':
    try:
        main()
    except (OSError, ValueError, zipfile.BadZipFile) as error:
        raise SystemExit(f'inputs: {error}') from error
