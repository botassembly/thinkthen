"""Refuse private path and key markers in copied distributable source."""
import pathlib
import sys
import os
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "sdlc/generators/results"))
from sources import tracked_files
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance/children"))
from native_guard import distribution_bytes

root = pathlib.Path(sys.argv[1]).resolve()
needles = (b"/home/", b"/Users/", b"auth.json", b"-----BEGIN PRIVATE KEY-----", b"tt-canary-")
files = [p for p in tracked_files(root) if p.is_file() and "fixtures" not in p.relative_to(root).parts
         and p.suffix in {".go", ".md", ".mod", ".hpp", ".cpp", ".txt", ".in"}]
assert files, "no source scanned"
for path in files:
    if any(needle in path.read_bytes() for needle in needles):
        raise SystemExit(f"private pattern in {path.name}")
for name in ('THINKTHEN_ARTIFACT', 'THINKTHEN_C_ARTIFACT'):
    if os.environ.get(name):
        path = pathlib.Path(os.environ[name])
        if any(needle in chunk for chunk in distribution_bytes(path) for needle in needles):
            raise SystemExit(f"private pattern in {path.name}")
print(f"PRIVATE_SOURCE_PASS {len(files)} files")
