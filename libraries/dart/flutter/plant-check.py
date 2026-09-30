"""Executed planted wrong responses and extra backend POST must fail by exact cause."""
import json
import pathlib
import subprocess
import sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env

root = pathlib.Path(__file__).resolve().parent
# Dart consumer bravo's plant check covers the strict behaviors, including swapped bulk answers.
cases = {
    'wrong-probability': ('Expected: Outcome:<Outcome.yes>', 1, 1),
    'extra-post': ('FLUTTER_FACADE_PASS', 0, 2),
}
for plant, (marker, expected_exit, expected_arrivals) in cases.items():
    before = set((root / 'logs').glob(f'gate-*-{plant}'))
    result = subprocess.run([sys.executable, str(root / 'run.py'), plant], cwd=root,
                            env=child_env(keep=('TT_FLUTTER', 'TT_NATIVE_LIBRARY', 'PUB_CACHE')),
                            capture_output=True, text=True, timeout=245)
    created = set((root / 'logs').glob(f'gate-*-{plant}')) - before
    if len(created) != 1:
        raise SystemExit(f'PLANTED_NEGATIVE_MISSING {plant}: {result.stdout} {result.stderr}')
    directory = created.pop()
    outcome = json.loads((directory / 'outcome.json').read_text())
    text = (directory / 'consumer.log').read_text()
    if result.returncode != 1 or outcome['status'] != 'FAIL' or outcome['receipt']['exit'] != expected_exit or marker not in text:
        raise SystemExit(f'PLANTED_NEGATIVE_WRONG_REASON {plant}: {result.stdout} {text[-500:]}')
    if outcome['counts'] != {'flutter-facade': expected_arrivals}:
        raise SystemExit(f'PLANTED_NEGATIVE_WRONG_ARRIVALS {plant}: {outcome["counts"]}')
    print(f'PLANTED_NEGATIVE_PASS {plant} {directory.name} flutter_exit={expected_exit} gate_exit={result.returncode}')
