"""Run the installed Swift consumer with only its standard runtime."""
import json
from pathlib import Path
import subprocess
import sys

consumer, fixtures = sys.argv[1:]
result = subprocess.check_output([consumer, fixtures], text=True, encoding='utf-8')
actual = [json.loads(line) for line in result.splitlines()]
sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'sdk/swift/tests'))
from filter_contract import expected_filter
cases = json.loads(Path(fixtures).read_text(encoding='utf-8'))
expected = [expected_filter(case) if case['request']['question']['function'] == 'filter' else case['expected'] for case in cases]
assert actual == expected, (actual, expected)
print('candidate Swift: ten shared stub cases pass without a compiler')
