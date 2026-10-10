"""Owned offline counted loopback run for Objective-C. Runs never inherit ambient credentials."""
import datetime
import collections
import json
import os
import pathlib
import signal
import subprocess
import sys
import time
from backend import Backend, one_record
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env

root = pathlib.Path(__file__).resolve().parent
stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
logs = root / 'logs' / ('run-' + stamp)
logs.mkdir(parents=True)
barrier = logs / 'barrier'
barrier.mkdir()
home = logs / 'home'
(home / 'cache').mkdir(parents=True)
backend = Backend(barrier)
env = child_env(home=str(home),
                PATH='/usr/bin:/bin',
                XDG_CONFIG_HOME=str(home),
                XDG_CACHE_HOME=str(home),
                LD_LIBRARY_PATH=str(root / 'target'),
                THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.server_port}/generic/v1',
                THINKTHEN_API_KEY='tt-canary-295',
                THINKTHEN_CACHE=str(home / 'cache'),
                TT_BARRIER_DIR=str(barrier))
receipts = []
active = None

def stop(process):
    for sig in (signal.SIGTERM, signal.SIGKILL):
        try: os.killpg(process.pid, sig)
        except ProcessLookupError: pass
        if sig == signal.SIGTERM: time.sleep(.2)
    process.wait(timeout=5)

try:
    status = 'PASS'
    subprocess.run([str(root / 'target/nul_text')], env=env, cwd=root, check=True, timeout=30)
    assert not backend.arrivals, 'NUL result stub sent a request'
    for case, args in [('direct', []), ('basic', ['basic']), ('bulk', ['bulk']), ('reverse', ['reverse']), ('typed-json', ['typed-json']), ('boundaries', ['boundaries']), ('concurrent', ['concurrent']), ('strict', ['strict']), ('held', ['held'])]:
        with (logs / (case + '.log')).open('wb') as output:
            active = subprocess.Popen([str(root / 'target/direct' if case == 'direct' else root / 'target/main'), *args], env=env, cwd=root,
                                      stdout=output, stderr=subprocess.STDOUT, start_new_session=True)
            receipt = {'case': case, 'pid': active.pid, 'pgid': active.pid, 'timeout': 120}
            receipts.append(receipt)
            (logs / 'receipts.json').write_text(json.dumps(receipts, indent=2) + '\n')
            try: receipt['exit'] = active.wait(timeout=120)
            except subprocess.TimeoutExpired:
                stop(active)
                receipt['exit'] = 'timeout'
            active = None
            (logs / 'receipts.json').write_text(json.dumps(receipts, indent=2) + '\n')
            if receipt['exit'] != 0:
                status = 'FAIL:' + case
                break
    if status == 'PASS':
        # Pin 71f25087: records.md "Order and requests" and ADR 0055 carry each
        # distinct text once in its quoted question. These five batches are five
        # requests, not nine singleton requests and one or two held requests.
        packed_state = 'Each question quotes the text it asks about.'
        required = ['café','no','unsure','json-decide','choose','tag','score',
                    'annotate-one','Maria Chen','hold-scalar','recovery-scalar',
                    'hold-deadline','recovery-held','🧬','John Smith','failure-one',
                    'failure-two','success','a\u0000b','status-401','after-error',
                    'other-engine-error',packed_state,
                    '[{"id":"u001","evidence":"find-one"},{"id":"u002","evidence":"find-two"}]',
                    {'entities': [{'id': 'i1', 'name': 'First', 'kind': 'alert'},
                                  {'id': 'i2', 'name': 'Second', 'kind': 'alert'}]},
                    {'entities': [{'id': 'i1', 'name': 'Third', 'kind': 'alert'},
                                  {'id': 'i2', 'name': 'Fourth', 'kind': 'alert'}]}]
        key = lambda x: json.dumps(x, sort_keys=True, ensure_ascii=False)
        counts = {key(x): sum(key(y) == key(x) for y in backend.arrivals) for x in required}
        expected = {key(x): (5 if x == packed_state else 2 if x in ('Maria Chen', 'John Smith', 'failure-two') else 1) for x in required}
        packed = [tuple(q['instructions'] for q in request['questions'].values())
                  for request in backend.requests if one_record(request)['state'] == packed_state]
        expected_packed = [tuple('The text is ' + json.dumps(row) + '. Is it?' for row in rows)
                           for rows in (('filter-one','filter-two'),('rank-one','rank-two'),
                                        ('first','second','third'),('hold-reverse-1','hold-reverse-2'),
                                        ('hold-bulk-1','hold-bulk-2'))]
        if counts != expected or len(backend.arrivals) != 33 or sorted(packed) != sorted(expected_packed):
            status = 'COUNT_MISMATCH'
        else:
            golden = json.loads((root / 'expected-requests.json').read_text())
            identity = lambda items: collections.Counter(json.dumps(x, sort_keys=True, ensure_ascii=False, separators=(',', ':')) for x in items)
            if identity(backend.requests) != identity(golden):
                status = 'BODY_MISMATCH'
        if status == 'PASS' and ('OBJC_DIRECT_PASS' not in (logs / 'direct.log').read_text() or
              'STRICT_OBJC_CANCEL_PASS' not in (logs / 'strict.log').read_text() or
              'OBJC_HELD_PASS' not in (logs / 'held.log').read_text() or
              'OBJC_TYPED_JSON_PASS' not in (logs / 'typed-json.log').read_text() or
              'OBJC_CONCURRENT_PASS' not in (logs / 'concurrent.log').read_text() or
              'OBJC_BOUNDARIES_PASS' not in (logs / 'boundaries.log').read_text() or
              'OBJC_REVERSE_PASS' not in (logs / 'reverse.log').read_text()):
            status = 'STRICT_MARKER_MISSING'
    if status == 'PASS':
        overlap_barrier = logs / 'overlap-barrier'
        overlap_barrier.mkdir()
        overlap_backend = Backend(overlap_barrier)
        try:
            overlap_env = {**env, 'TT_BARRIER_DIR': str(overlap_barrier),
                           'THINKTHEN_BASE_URL': f'http://127.0.0.1:{overlap_backend.server_port}/generic/v1',
                           'THINKTHEN_CACHE': str(logs / 'overlap-cache')}
            completed = subprocess.run([str(root / 'target/main'), 'overlap'], env=overlap_env,
                                       cwd=root, capture_output=True, text=True, timeout=35)
            if (completed.returncode != 0 or 'OBJC_HELD_OVERLAP_PASS' not in completed.stdout or
                collections.Counter(overlap_backend.arrivals) != collections.Counter(['hold-overlap-a', 'hold-overlap-b']) or
                any(request['state'] != packed_state or
                    one_record(request)['questions'] != {'q1': {'instructions': 'Is it?', 'type': 'noul'}}
                    for request in overlap_backend.requests)):
                status = 'FAIL:overlap'
        finally:
            overlap_backend.close()
finally:
    if active is not None: stop(active)
    backend.close()
    (logs / 'arrivals.json').write_text(json.dumps(backend.arrivals, ensure_ascii=False, indent=2) + '\n')
    (logs / 'requests.json').write_text(json.dumps(backend.requests, ensure_ascii=False, indent=2) + '\n')
    outcome = {'status': status, 'log': str(logs), 'arrivals': len(backend.arrivals), 'receipts': receipts}
    (logs / 'outcome.json').write_text(json.dumps(outcome, indent=2) + '\n')
    print(json.dumps(outcome, indent=2))
if status != 'PASS': sys.exit(1)

import complete_parity
