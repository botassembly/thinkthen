"""Refuse private path and key markers in copied distributable source."""
import pathlib
import sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance/children"))
from native_guard import normalized_bytes

root = pathlib.Path(sys.argv[1])
needles = (b"/home/", b"/Users/", b"auth.json", b"-----BEGIN PRIVATE KEY-----", b"tt-canary-")
files = [pathlib.Path(name) for name in pathlib.Path(sys.argv[2]).read_text().splitlines() if name]
assert all(path.resolve().is_relative_to(root.resolve()) for path in files), 'INSTALL_MEMBER_OUTSIDE_PREFIX'
for private in ('thinkthen.h', 'native_implementation.hpp', 'json.hpp'):
    assert (root / 'include/thinkthen' / private).resolve() not in {path.resolve() for path in files}, 'PRIVATE_HEADER_INSTALLED ' + private
assert files, "no source scanned"
for path in files:
    # release-pack remaps the builder's home; scan the rest of each path.
    data = normalized_bytes(path.read_bytes())
    if any(needle in data for needle in needles):
        raise SystemExit(f"private pattern in {path.name}")
print(f"PRIVATE_PACKAGE_PASS {len(files)} files")
