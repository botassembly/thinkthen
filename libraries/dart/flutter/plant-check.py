"""Executed planted wrong responses and extra backend POST must fail by exact cause."""
import json
import pathlib
import subprocess
import sys

root = pathlib.Path(__file__).resolve().parent
cases = {
    'wrong-probability': ("Bad state: scalar yes Outcome.no/0.1", 1, {'hold-bulk-first': 0}),
    'swapped-bulk': ('Bad state: bulk ordered answers .9 then .1', 1, {'hold-bulk-first': 0}),
    'extra-bulk': ('FLUTTER_STRICT_PASS', 0, {'hold-bulk-first': 2}),
}
for plant, (marker, expected_exit, expected_bulk) in cases.items():
    before = set((root / 'logs').glob(f'gate-*-{plant}'))
    result = subprocess.run([sys.executable, str(root / 'run.py'), plant], cwd=root,
                            capture_output=True, text=True, timeout=245)
    created = set((root / 'logs').glob(f'gate-*-{plant}')) - before
    if len(created) != 1:
        raise SystemExit(f'PLANTED_NEGATIVE_MISSING {plant}: {result.stdout} {result.stderr}')
    directory = created.pop()
    outcome = json.loads((directory / 'outcome.json').read_text())
    text = (directory / 'consumer.log').read_text()
    if result.returncode != 1 or outcome['status'] != 'FAIL' or outcome['receipt']['exit'] != expected_exit or marker not in text:
        raise SystemExit(f'PLANTED_NEGATIVE_WRONG_REASON {plant}: {result.stdout} {text[-500:]}')
    if plant == 'extra-bulk' and outcome['bulk'] != expected_bulk:
        raise SystemExit(f'PLANTED_NEGATIVE_EXTRA_POST_MISSING {outcome["bulk"]}')
    print(f'PLANTED_NEGATIVE_PASS {plant} {directory.name} flutter_exit={expected_exit} gate_exit={result.returncode}')
