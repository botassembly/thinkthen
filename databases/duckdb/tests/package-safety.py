"""Reject private fixture routing in every distributed SQL extension."""
from pathlib import Path
import sys

for name in sys.argv[1:]:
    data = Path(name).read_bytes()
    for marker in (b'THINKTHEN_TEST_LOOPBACK_URL', b'thinkthen_fixture_routes_install',
                   b'fixture routes must be loopback addresses'):
        assert marker not in data, (name, marker.decode())
