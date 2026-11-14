"""Run the installed Zig consumer without a compiler or source checkout."""
import json
from pathlib import Path
import subprocess
import sys

consumer, fixtures = sys.argv[1:]
for case in json.loads(Path(fixtures).read_text(encoding='utf-8')):
    result = subprocess.check_output([consumer, 'run', json.dumps(case['client']),
                                      json.dumps(case['request'])], text=True, encoding='utf-8')
    actual = json.loads(result)
    expected = case['expected']
    if case['request']['question']['function'] == 'filter' and isinstance(expected, list):
        expected = dict(kept=[case['request']['records'][row['position'] - 1] for row in expected
                              if row['status'] == 'answered' and row['answer']],
                        failed=[row for row in expected if row['status'] == 'failed'])
    assert actual == expected, case['request']['question']['function']
print('candidate Zig: ten shared stub cases pass without a compiler')
