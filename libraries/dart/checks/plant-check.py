"""Both consumers must fail executed wrong-fixture gates for the intended reason."""
import json
import pathlib
import subprocess
import sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env

root = pathlib.Path(__file__).resolve().parent
cases = {
    'extra-bulk': (0, 'DART_STAGE_TWO_PASS'),
    'wrong-relation': (0, 'DART_STAGE_TWO_PASS'),
    'wrong-probability': (255, 'Bad state: scalar yes Outcome.no/0.1'),
    'swapped-bulk': (255, 'Bad state: bulk ordered answers .9 then .1'),
    'wrong-json': (255, 'Bad state: JSON door exact decision and facts identity'),
    'wrong-annotate': (255, 'Bad state: annotate JSON exact field identity'),
    'wrong-recognize': (255, 'Bad state: recognize JSON exact entity identity'),
    'wrong-relate': (255, 'Bad state: relate JSON exact edges and order'),
    'fail-mid-case': (255, 'Bad state: PLANTED_MID_CASE_FAILURE'),
}
for consumer in ('alpha', 'bravo'):
    for plant, (expected_exit, marker) in cases.items():
        existing = set((root / 'logs').glob(f'gate-*-{consumer}-{plant}'))
        process = subprocess.run([sys.executable, str(root / 'run.py'), consumer, plant], cwd=root,
                                 env=child_env(keep=('TT_DART', 'TT_NATIVE_LIBRARY', 'PUB_CACHE')),
                                 capture_output=True, text=True, timeout=165)
        created = set((root / 'logs').glob(f'gate-*-{consumer}-{plant}')) - existing
        if len(created) != 1:
            raise SystemExit(f'PLANTED_NEGATIVE_MISSING_RECEIPT {consumer} {plant} {created} {process.stdout} {process.stderr}')
        directory = created.pop()
        outcome = json.loads((directory / 'outcome.json').read_text())
        log = (directory / 'consumer.log').read_text()
        expected_marker = f'{marker} {consumer}' if expected_exit == 0 else marker
        if (process.returncode != 1 or outcome['status'] != 'FAIL' or
            outcome['receipt']['exit'] != expected_exit or expected_marker not in log):
            raise SystemExit(f'PLANTED_NEGATIVE_WRONG_REASON {consumer} {plant}: {directory} {process.stdout} {log[-500:]}')
        if plant == 'extra-bulk' and outcome['bulk'] != {'hold-bulk-first': 2}:
            raise SystemExit('EXTRA_BULK_NOT_PLANTED')
        if plant == 'wrong-relation' and not any('"wrong"' in name for name in outcome['relation']):
            raise SystemExit('WRONG_RELATION_NOT_PLANTED')
        if plant == 'fail-mid-case':
            order = ['ISOLATES_JOINED worker canceller(5)', 'TOKEN_FREED_AFTER_JOIN', 'CLEANUP_ORDER_PASS hold-scalar', 'ENGINE_FREED_AFTER_JOIN']
            offsets = [log.find(item) for item in order]
            if -1 in offsets or offsets != sorted(offsets):
                raise SystemExit(f'FAILURE_CLEANUP_ORDER_NOT_PROVED {directory}')
        print(f'PLANTED_NEGATIVE_PASS {consumer} {plant} {directory.name} exit={expected_exit}')
