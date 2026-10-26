"""Run the installed Zig consumer without a compiler or source checkout."""
import json
from pathlib import Path
import subprocess
import sys

consumer, fixtures = sys.argv[1:]
for case in json.loads(Path(fixtures).read_text()):
    result = subprocess.check_output([consumer, 'run', json.dumps(case['client']),
                                      json.dumps(case['request'])], text=True)
    assert json.loads(result) == case['expected'], case['request']['question']['function']
print('candidate Zig: ten shared stub cases pass without a compiler')
