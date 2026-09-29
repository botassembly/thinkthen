"""Refuse private path and key markers in copied distributable source."""
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
needles = (b"/home/", b"/Users/", b"auth.json", b"-----BEGIN PRIVATE KEY-----", b"tt-canary-")
files = [p for p in root.rglob("*") if p.is_file() and "fixtures" not in p.parts
         and p.suffix in {".go", ".md", ".mod", ".hpp", ".cpp", ".txt", ".in"}]
assert files, "no source scanned"
for path in files:
    if any(needle in path.read_bytes() for needle in needles):
        raise SystemExit(f"private pattern in {path.name}")
print(f"PRIVATE_SOURCE_PASS {len(files)} files")
