#!/usr/bin/env python3
"""Compare Git-archived source members with the extracted release source tree."""

import os
from pathlib import Path
import sys
import tarfile


def compare(archive_path, source_root):
    with tarfile.open(archive_path, "r:") as archive:
        archived = set()
        for member in archive:
            parts = Path(member.name).parts
            if not parts or any(part in ("", ".", "..") for part in parts):
                raise ValueError(f"unsafe source member {member.name}")
            archived.add(Path(*parts))
            extracted = source_root.joinpath(*parts)
            if member.isdir():
                if not extracted.is_dir() or extracted.is_symlink():
                    raise ValueError(f"source directory differs: {member.name}")
            elif member.issym():
                if not extracted.is_symlink() or os.readlink(extracted) != member.linkname:
                    raise ValueError(f"source link differs: {member.name}")
            elif member.isfile():
                if not extracted.is_file() or extracted.is_symlink() or extracted.stat().st_size != member.size:
                    raise ValueError(f"source file differs: {member.name}")
                packed = archive.extractfile(member)
                if packed is None:
                    raise ValueError(f"source file cannot be read: {member.name}")
                with packed, extracted.open("rb") as actual:
                    while chunk := packed.read(1024 * 1024):
                        if actual.read(len(chunk)) != chunk:
                            raise ValueError(f"source file differs: {member.name}")
            else:
                raise ValueError(f"unsupported source member: {member.name}")
        for extracted in sorted(source_root.rglob("*"), key=lambda path: (path.is_dir(), str(path))):
            relative = extracted.relative_to(source_root)
            if relative not in archived:
                raise ValueError(f"unexpected source path: {relative}")


if __name__ == "__main__":
    try:
        compare(Path(sys.argv[1]), Path(sys.argv[2]))
    except (OSError, ValueError, tarfile.TarError) as error:
        print(f"release-archive-tree: {error}", file=sys.stderr)
        sys.exit(1)
