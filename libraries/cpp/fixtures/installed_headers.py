"""Keep private implementation headers out of the installed C++ interface."""
import pathlib
import sys

root = pathlib.Path(sys.argv[1])
files = [pathlib.Path(name) for name in pathlib.Path(sys.argv[2]).read_text().splitlines() if name]
assert all(path.resolve().is_relative_to(root.resolve()) for path in files), 'INSTALL_MEMBER_OUTSIDE_PREFIX'
for private in ('thinkthen.h', 'native_implementation.hpp', 'json.hpp'):
    assert (root / 'include/thinkthen' / private).resolve() not in {path.resolve() for path in files}, 'PRIVATE_HEADER_INSTALLED ' + private
print('installed header inventory passed')
