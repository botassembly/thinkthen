"""Executed wrong-fixture gates must fail for the intended reason, not just exit nonzero."""
import json
import pathlib
import subprocess
import sys

root = pathlib.Path(__file__).resolve().parent
logs = root.parents[2] / 'target/cpp/logs'
cases = {
    'extra-bulk': (0, 'CPP_PRODUCT_PASS'),
    'wrong-relation': (0, 'CPP_PRODUCT_PASS'),
    'wrong-probability': (255, 'CHECK_FAILED scalar yes 0/0.9'),
    'swapped-bulk': (255, 'CHECK_FAILED bulk ordered answers .9 then .1'),
    'wrong-json': (255, 'CHECK_FAILED JSON door exact decision and facts identity'),
    'wrong-annotate': (255, 'CHECK_FAILED annotate JSON exact field identity'),
    'wrong-recognize': (255, 'CHECK_FAILED recognize JSON exact entity identity'),
    'wrong-relate': (255, 'CHECK_FAILED relate JSON exact edges and order'),
    'fail-mid-case': (255, 'CHECK_FAILED PLANTED_MID_CASE_FAILURE'),
}
for plant, (expected_exit, marker) in cases.items():
    existing = set(logs.glob(f'gate-*-{plant}'))
    process = subprocess.run([sys.executable, str(root / 'run.py'), plant], cwd=root, capture_output=True, text=True, timeout=165)
    created = set(logs.glob(f'gate-*-{plant}')) - existing
    if len(created) != 1:
        raise SystemExit(f'PLANTED_NEGATIVE_MISSING_RECEIPT {plant} {created} {process.stdout} {process.stderr}')
    directory = created.pop()
    outcome = json.loads((directory / 'outcome.json').read_text())
    log = (directory / 'consumer.log').read_text()
    if process.returncode != 1 or outcome['status'] != 'FAIL' or outcome['receipt']['exit'] != expected_exit or marker not in log:
        raise SystemExit(f'PLANTED_NEGATIVE_WRONG_REASON {plant}: {directory} {process.stdout} {log[-500:]}')
    if plant == 'extra-bulk' and outcome['bulk'] != {'hold-bulk-first': 2}:
        raise SystemExit('EXTRA_BULK_NOT_PLANTED')
    if plant == 'wrong-relation' and not any('"wrong"' in name for name in outcome['relation']):
        raise SystemExit('WRONG_RELATION_NOT_PLANTED')
    if plant == 'fail-mid-case':
        order = ['THREADS_JOINED worker canceller(5)', 'TOKEN_FREED_AFTER_JOIN', 'CLEANUP_ORDER_PASS hold-scalar', 'ENGINE_FREED_AFTER_JOIN']
        offsets = [log.find(item) for item in order]
        if -1 in offsets or offsets != sorted(offsets):
            raise SystemExit(f'FAILURE_CLEANUP_ORDER_NOT_PROVED {directory}')
    print(f'PLANTED_NEGATIVE_PASS {plant} {directory.name} exit={expected_exit}')
