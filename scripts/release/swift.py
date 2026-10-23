"""Run the installed Swift consumer with only its standard runtime."""
import json
from pathlib import Path
import subprocess
import sys

consumer, fixtures = sys.argv[1:]
result = subprocess.check_output([consumer, fixtures], text=True)
actual = [json.loads(line) for line in result.splitlines()]
assert actual == [case['expected'] for case in json.loads(Path(fixtures).read_text())]
print('candidate Swift: ten shared stub cases pass without a compiler')
