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
prefix = "LOAD '" + str(package/'thinkthen.duckdb_extension').replace("'", "''") + "';"
if stub:
    prefix += "SET thinkthen_backend='stub';"

def query(statement):
    output = subprocess.check_output(['duckdb', '-unsigned', '-json', '-init', '/dev/null', ':memory:', prefix + statement],
                                     env=env, text=True)
    return json.loads(output)

for statement, count in zip(statements, expected_rows, strict=True):
    result = query(statement)
    if 'thinkthen_filter(' in statement:
        assert len(result) <= count, result
        assert all(row['status'] == 'answered' and row['needs_refund'] is True for row in result), result
    else:
        assert len(result) == count, result
    assert all(row['status'] != 'failed' for row in result), result
    print(json.dumps(result), flush=True)

if stub:
    # Use the first example's single record, whose stub probability is 0.28.
    probe = statements[0].replace('thinkthen_decide(', 'thinkthen_filter(').rstrip().removesuffix(';').removesuffix(')')
    for threshold, expected in ((0.1, 1), (1, 0)):
        result = query(probe + f', threshold := {threshold});')
        assert len(result) == expected, result
        assert all(row['status'] == 'answered' and row['needs_refund'] is True for row in result), result
