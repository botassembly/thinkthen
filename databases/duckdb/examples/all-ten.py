#!/usr/bin/env python3
"""Run by hand: all-ten.py /path/to/installed/package [--stub]."""
import json
import os
from pathlib import Path
import subprocess
import sys

package = Path(sys.argv[1]).resolve()
stub = sys.argv[2:] == ['--stub']
if sys.argv[2:] and not stub:
    raise SystemExit('usage: all-ten.py PACKAGE [--stub]')
# Default-backend checks never inherit an address override. Keys stay unread.
env = dict(os.environ)
env.pop('THINKTHEN_BASE_URL', None)
if stub:
    env = {name: env[name] for name in ('PATH', 'HOME', 'SYSTEMROOT', 'TMPDIR', 'TEMP', 'TMP') if name in env}
# The authored SQL file holds one complete statement per line.
statements = Path(__file__).with_suffix('.sql').read_text().splitlines()
expected_rows = (1, 1, 1, 1, 2, 2, 1, 1, 1, 1)

# Use the user's installed official DuckDB command, never a bundled runtime.
for statement, count in zip(statements, expected_rows, strict=True):
    prefix = "LOAD '" + str(package/'thinkthen.duckdb_extension').replace("'", "''") + "';"
    if stub:
        prefix += "SET thinkthen_backend='stub';"
    output = subprocess.check_output(['duckdb', '-unsigned', '-json', '-init', '/dev/null', ':memory:', prefix + statement],
                                     env=env, text=True)
    result = json.loads(output)
    assert len(result) == count, result
    assert all(row['status'] != 'failed' for row in result), result
    print(json.dumps(result), flush=True)
