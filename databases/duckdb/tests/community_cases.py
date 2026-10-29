"""Run the community SQL corpus through the installed native host."""
from pathlib import Path
import sys

from typed_api import Host, child_environment

package = Path(sys.argv[1])
path = Path(__file__).resolve().parents[1] / 'test/sql/community.test'
blocks = path.read_text().split('\n\n')
with Host(package, child_environment()) as host:
    for block in blocks:
        lines = [line for line in block.splitlines() if line and not line.startswith('#')]
        if not lines or lines[0].startswith('require '):
            continue
        kind, *body = lines
        if kind == 'statement ok':
            host.ok('\n'.join(body))
        else:
            assert kind.startswith('query '), kind
            divider = body.index('----')
            rows = host.ok('\n'.join(body[:divider]))
            actual = ['\t'.join(str(value) for value in row.values()) for row in rows]
            assert actual == body[divider + 1:], (kind, actual, body[divider + 1:])
print('Community SQL corpus passes through the installed adapter')
