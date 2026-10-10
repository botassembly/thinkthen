"""Refuse private path and key markers in copied distributable source."""
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
needles = (b"/home/", b"/Users/", b"auth.json", b"-----BEGIN PRIVATE KEY-----", b"tt-canary-")
files = [p for p in root.rglob("*") if p.is_file() and "fixtures" not in p.parts
         and (p.suffix in {".go", ".md", ".mod", ".hpp", ".h", ".cpp", ".txt", ".in"}
              or p.name.startswith("libthinkthen"))]
assert files, "no source scanned"
for path in files:
    # release-pack remaps the builder's home; scan the rest of each path.
    data = path.read_bytes().replace(b"/build/home/", b"/build/")
    if any(needle in data for needle in needles):
        raise SystemExit(f"private pattern in {path.name}")
print(f"PRIVATE_PACKAGE_PASS {len(files)} files")
