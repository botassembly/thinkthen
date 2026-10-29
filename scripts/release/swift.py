"""Run the installed Swift consumer with only its standard runtime."""
import json
from pathlib import Path
import subprocess
import sys

consumer, fixtures = sys.argv[1:]
result = subprocess.check_output([consumer, fixtures], text=True, encoding='utf-8')
actual = [json.loads(line) for line in result.splitlines()]
for rows in actual:
    for row in rows:
        if row['function'] == 'filter':
            kept = row.pop('kept')
            assert type(kept) is bool and kept == (row.get('answer') is True), row
assert actual == [case['expected'] for case in json.loads(Path(fixtures).read_text(encoding='utf-8'))]
print('candidate Swift: ten shared stub cases pass without a compiler')
