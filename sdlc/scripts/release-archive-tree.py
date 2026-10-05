#!/usr/bin/env python3
"""Compare Git-archived source members with the extracted release source tree."""

import os
import stat
import subprocess
from pathlib import Path
import sys
import tarfile


def raw_git(source_root, commit, extra_paths=()):
    """Compare actual bytes/modes/links to raw Git blobs, bypassing index and filters."""
    def git(*args):
        return subprocess.check_output(["git", "-C", str(source_root), *args])
    if git("rev-parse", "HEAD").decode().strip() != commit:
        raise ValueError("DuckDB C++ source differs from the pinned commit")
    rows = [row.split(b"\t", 1) for row in git("ls-tree", "-rz", commit).split(b"\0") if row]
    batch = subprocess.Popen(["git", "-C", str(source_root), "cat-file", "--batch"],
                             stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    tracked = set()
    try:
        for head, name in rows:
            mode, kind, oid = head.split()
            if kind != b"blob":
                raise ValueError("source tree contains a non-blob input")
            relative = Path(os.fsdecode(name))
            tracked.add(relative)
            path = source_root / relative
            batch.stdin.write(oid + b"\n")
            batch.stdin.flush()
            info = batch.stdout.readline().split()
            raw = batch.stdout.read(int(info[2]))
            batch.stdout.read(1)
            meta = path.lstat()
            if mode == b"120000":
                same = stat.S_ISLNK(meta.st_mode) and os.fsencode(os.readlink(path)) == raw
            else:
                same = stat.S_ISREG(meta.st_mode) and bool(meta.st_mode & 0o111) == (mode == b"100755")
                same = same and path.read_bytes() == raw
            if not same:
                raise ValueError(f"raw Git source file differs: {relative}")
    finally:
        batch.stdin.close()
        batch.stdout.close()
        batch.wait()
    allowed = set(map(Path, extra_paths))
    for directory, folders, files in os.walk(source_root):
        if Path(directory) == source_root:
            folders[:] = [name for name in folders if name != ".git"]
        for name in folders:
            path = Path(directory) / name
            relative = path.relative_to(source_root)
            if path.is_symlink() and relative not in tracked and relative not in allowed:
                raise ValueError(f"unexpected source input: {relative}")
        for name in files:
            relative = (Path(directory) / name).relative_to(source_root)
            if relative not in tracked and relative not in allowed and relative != Path(".git"):
                raise ValueError(f"unexpected source input: {relative}")
    return len(tracked)


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
