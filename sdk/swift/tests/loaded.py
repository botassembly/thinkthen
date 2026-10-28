#!/usr/bin/env python3
"""Repeat the installed Swift consumer suite with independent resources under parallel load."""
from concurrent.futures import ThreadPoolExecutor
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import tempfile
import time

consumer, repository, loopback, output = (Path(value).resolve() for value in sys.argv[1:5])
repetitions = int(sys.argv[5]) if len(sys.argv) > 5 else 50
output.mkdir(parents=True, exist_ok=True)
prepared = os.environ.get('THINKTHEN_SWIFT_AUDIT_LIBRARY')
library = Path(prepared).resolve() if prepared else output / 'thread-audit.so'
if not prepared:
    subprocess.run([shutil.which('cc') or 'clang', '-shared', '-fPIC', '-Wall', '-Wextra', '-Werror', '-ldl',
                    str(repository / 'scripts/tests/thread-start-audit.c'), '-o', str(library)], check=True)
identity = {
    'revision': sys.argv[6],
    'platform': list(os.uname()), 'parallel_suites': 2,
    'allocated_cores': os.environ.get('PM_JOB_CORES'), 'repetitions': repetitions,
}


def suite(index):
    result = {'run': index, 'start_load': os.getloadavg(), 'start': time.monotonic()}
    with tempfile.TemporaryDirectory(prefix=f'consumer-{index}-', dir=output) as temporary:
        root = Path(temporary)
        installed = root / 'installed'
        shutil.copytree(consumer.parent, installed)
        (root / 'home').mkdir()
        (root / 'tmp').mkdir()
        env = dict(os.environ, HOME=str(root / 'home'), TMPDIR=str(root / 'tmp'),
                   PYTHONDONTWRITEBYTECODE='1', THINKTHEN_SWIFT_AUDIT_LIBRARY=str(library))
        with (output / f'{index:02}.log').open('w') as log:
            child = subprocess.Popen([sys.executable, str(repository / 'sdk/swift/tests/run.py'),
                                      str(installed / consumer.name), str(repository), str(loopback)],
                                     env=env, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
            try:
                status = child.wait(timeout=600)
            except subprocess.TimeoutExpired:
                os.killpg(child.pid, signal.SIGKILL)
                child.wait()
                status = 124
        result.update(exit_code=status, end_load=os.getloadavg(), end=time.monotonic())
    print(json.dumps(result, sort_keys=True), flush=True)
    return result


with ThreadPoolExecutor(max_workers=2) as workers:
    results = list(workers.map(suite, range(1, repetitions + 1)))
(output / 'results.json').write_text(json.dumps(dict(identity, results=results), indent=2) + '\n')
assert all(result['exit_code'] == 0 for result in results), results
