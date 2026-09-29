"""Owned offline counted loopback run for GNAT. Runs never inherit ambient credentials."""
import datetime
import collections
import json
import os
import pathlib
import re
import signal
import subprocess
import sys
import time
from backend import Backend

root = pathlib.Path(__file__).resolve().parent
stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
logs = root / 'logs' / ('run-' + stamp)
logs.mkdir(parents=True)
barrier = logs / 'barrier'
barrier.mkdir()
home = logs / 'home'
(home / 'cache').mkdir(parents=True)
backend = Backend(barrier)
env = {
    'PATH': '/usr/bin:/bin', 'HOME': str(home), 'XDG_CONFIG_HOME': str(home),
    'XDG_CACHE_HOME': str(home), 'LD_LIBRARY_PATH': str(root / 'target'),
    'THINKTHEN_BASE_URL': f'http://127.0.0.1:{backend.server_port}/generic/v1',
    'THINKTHEN_API_KEY': 'tt-canary-293', 'THINKTHEN_CACHE': str(home / 'cache'),
    'TT_BARRIER_DIR': str(barrier),
}
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
    for case, args in [('direct', []), ('basic', []), ('bulk', ['bulk']), ('reverse', ['reverse']), ('typed-json', ['typed-json']), ('boundaries', ['boundaries']), ('concurrent', ['concurrent']), ('strict', ['strict']), ('held', ['held'])]:
        with (logs / (case + '.log')).open('wb') as output:
            active = subprocess.Popen([str(root / 'target/legacy/direct' if case == 'direct' else root / 'target/legacy/main'), *args], env=env, cwd=root,
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
        required = ['café','no','unsure','json-decide','choose','tag','score','filter-one',
                    'filter-two','rank-one','rank-two','annotate-one','Maria Chen',
                    'hold-scalar','recovery-scalar','first','second','third',
                    'hold-deadline','recovery-held','hold-reverse-1','hold-reverse-2','John Smith','failure-one','failure-two','success','a\u0000b','status-401','after-error','other-engine-error',
                    '[{"id":"u001","evidence":"find-one"},{"id":"u002","evidence":"find-two"}]',
                    {'entities': [{'id': 'i1', 'name': 'First', 'kind': 'alert'},
                                  {'id': 'i2', 'name': 'Second', 'kind': 'alert'}]}]
        required.append({'entities': [{'id': 'i1', 'name': 'Third', 'kind': 'alert'},
                                      {'id': 'i2', 'name': 'Fourth', 'kind': 'alert'}]})
        key = lambda x: json.dumps(x, sort_keys=True, ensure_ascii=False)
        shared = 'Each question quotes the text it asks about.'
        packed_rows = [('filter-one','filter-two'),('rank-one','rank-two'),
                       ('first','second','third'),('hold-reverse-1','hold-reverse-2'),
                       ('hold-bulk-1','hold-bulk-2')]
        old_rows = {row for group in packed_rows for row in group}
        expected = {key(x): (2 if x in ('Maria Chen', 'John Smith', 'failure-two') else 1)
                    for x in required if not (isinstance(x,str) and x in old_rows)}
        expected[key(shared)] = 5
        counts = {key(x): sum(key(y) == key(x) for y in backend.arrivals)
                  for x in required if not (isinstance(x,str) and x in old_rows)}
        counts[key(shared)] = sum(key(y) == key(shared) for y in backend.arrivals)
        bodies = [json.loads(line) for line in (barrier/'request-bodies.jsonl').read_text().splitlines()]
        packed = [body for body in bodies if body['state'] == shared]
        actual_rows = []
        for body in packed:
            rows = []
            for name, question in sorted(body['questions'].items()):
                if set(question) != {'type','instructions'} or question['type'] != 'noul':
                    status = 'PACKED_QUESTION_SHAPE_MISMATCH'; break
                match = re.fullmatch(r'The text is (.+)\. Is it\?',question['instructions'],re.DOTALL)
                if not match: status = 'PACKED_QUESTION_SHAPE_MISMATCH'; break
                rows.append(json.loads(match.group(1)))
            actual_rows.append(tuple(rows))
        relations=[body for body in bodies if isinstance(body['state'],dict)]
        expected_pairs={'q1':{'instructions':'Is it true that i1 caused by i2?','type':'noul'},
                        'q2':{'instructions':'Is it true that i2 caused by i1?','type':'noul'}}
        golden = json.loads((root / 'expected-requests.json').read_text())
        identity = lambda items: collections.Counter(json.dumps(x, sort_keys=True, ensure_ascii=False, separators=(',', ':')) for x in items)
        if counts != expected or len(backend.arrivals) != sum(expected.values()) or actual_rows != packed_rows or len(relations)!=2 or any(body['questions']!=expected_pairs for body in relations):
            status = 'COUNT_MISMATCH' if status == 'PASS' else status
        elif identity(bodies) != identity(golden):
            status = 'BODY_MISMATCH'
        elif ('ADA_DIRECT_PASS' not in (logs / 'direct.log').read_text() or
              'STRICT_ADA_CANCEL_PASS' not in (logs / 'strict.log').read_text() or
              'ADA_HELD_BULK_PASS' not in (logs / 'held.log').read_text() or
              'ADA_TYPED_JSON_PASS' not in (logs / 'typed-json.log').read_text() or
              'ADA_CONCURRENT_ERRORS_PASS' not in (logs / 'concurrent.log').read_text() or
              'ADA_BOUNDARIES_PASS' not in (logs / 'boundaries.log').read_text() or
              'ADA_REVERSE_BULK_PASS' not in (logs / 'reverse.log').read_text()):
            status = 'STRICT_MARKER_MISSING'
except BaseException:
    status = 'FAILED_EXCEPTION'
    raise
finally:
    if active is not None: stop(active)
    backend.close()
    (logs / 'arrivals.json').write_text(json.dumps(backend.arrivals, ensure_ascii=False, indent=2) + '\n')
    outcome = {'status': status, 'log': str(logs), 'arrivals': len(backend.arrivals), 'receipts': receipts}
    (logs / 'outcome.json').write_text(json.dumps(outcome, indent=2) + '\n')
    print(json.dumps(outcome, indent=2))
if status != 'PASS': sys.exit(1)
