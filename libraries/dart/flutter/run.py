"""Bounded single-process-group Dart gate with counted local backend."""
import collections
import datetime
import json
import os
import pathlib
import signal
import subprocess
import sys
import urllib.request
from fixture import Backend
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent.parent / 'checks'))
from request_identity import matches

root = pathlib.Path(__file__).resolve().parent
flutter = os.environ['TT_FLUTTER']
native = pathlib.Path(os.environ['TT_NATIVE_LIBRARY'])
stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
plant = sys.argv[1] if len(sys.argv) == 2 else ''
if plant not in ('', 'extra-bulk', 'wrong-probability', 'swapped-bulk'):
    raise SystemExit('unknown planted variant')
logs = root / 'logs' / ('gate-' + stamp + ('-' + plant if plant else '-positive'))
logs.mkdir(parents=True, exist_ok=False)
barrier = logs / 'barrier'
barrier.mkdir()
home = logs / 'home'
(home / 'cache').mkdir(parents=True)
server = Backend(barrier, plant)
env = {
    'PATH': os.environ['PATH'],
    'HOME': str(home), 'XDG_CONFIG_HOME': str(home), 'XDG_CACHE_HOME': str(home / 'cache'),
    'THINKTHEN_BASE_URL': f'http://127.0.0.1:{server.server_port}/generic/v1',
    'THINKTHEN_API_KEY': 'tt-canary-300', 'THINKTHEN_CACHE': str(home / 'cache'),
    'TT_BARRIER_DIR': str(barrier), 'TT_PLANT': plant, 'PUB_CACHE': os.environ['PUB_CACHE'],
    'TT_NATIVE_LIBRARY': str(native), 'FLUTTER_SUPPRESS_ANALYTICS': 'true',
}
command = [flutter, 'test', '--no-pub', '--reporter=expanded', 'test/strict_test.dart']
receipt = {'command': command, 'pid': None, 'pgid': None, 'exit': None, 'signals': [], 'timeout_seconds': 210}
process = None
try:
    with (logs / 'consumer.log').open('wb') as output:
        process = subprocess.Popen(command, cwd=root / 'example', env=env, stdout=output, stderr=subprocess.STDOUT, start_new_session=True)
        receipt['pid'] = receipt['pgid'] = process.pid
        try:
            receipt['exit'] = process.wait(timeout=210)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGTERM)
            receipt['signals'].append('SIGTERM timeout')
            try: process.wait(timeout=3)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                receipt['signals'].append('SIGKILL timeout')
                process.wait(timeout=5)
            receipt['exit'] = 'TIMEOUT'
    # Planted extra backend requests are real numeric-loopback POSTs, not fabricated ledger rows.
    if plant == 'extra-bulk':
        body = server.held_bulk_body
        if body is None: raise RuntimeError(f'no captured request for {plant}')
        data = json.dumps(body).encode()
        request = urllib.request.Request(f'http://127.0.0.1:{server.server_port}/generic/v1/systemone', data=data,
                                         headers={'Authorization': 'Bearer tt-canary-300', 'Content-Type': 'application/json'})
        with urllib.request.urlopen(request, timeout=5) as response:
            if response.status != 200: raise RuntimeError(f'planted request status {response.status}')
            response.read()
finally:
    server.close()
    (logs / 'arrivals.json').write_text(json.dumps(server.arrivals, indent=2, ensure_ascii=False) + '\n')
    (logs / 'completions.json').write_text(json.dumps(server.completions, indent=2, ensure_ascii=False) + '\n')
    counts = collections.Counter(server.arrivals)
    required = {'flutter-facade': 1, 'yes': 1, 'no': 1, 'unsure': 1, 'described': 1, 'batch-pair': 1,
                'json-decide': 1, 'annotate-one': 1, 'John Smith': 2,
                'backend-failure': 1, 'configured': 1, 'hold-scalar': 1,
                'recovery-scalar': 1, 'hold-deadline': 1}
    # Relate's shared pair planner sends one exact entity set at this pin.
    relation_name = '{"entities": [{"id": "i1", "kind": "alert", "name": "First"}, {"id": "i2", "kind": "alert", "name": "Second"}]}'
    required[relation_name] = 1
    required['hold-bulk-first'] = 1
    relation = {k: v for k, v in counts.items() if k.startswith('{') and 'entities' in k}
    bulk = {k: v for k, v in counts.items() if k.startswith('hold-bulk-')}
    text = (logs / 'consumer.log').read_text(errors='replace')
    described_requests = [json.loads(line) for line in (barrier / 'requests.jsonl').read_text().splitlines() if json.loads(line)['name'] == 'described']
    description_preserved = len(described_requests) == 1 and any(q.get('criteria', {}).get('true') == {'what': 'Affirmative answer.', 'not_for': 'Unclear.', 'examples': ['yes']} for q in described_requests[0]['body']['questions'].values())
    bodies_match = matches(barrier / 'requests.jsonl', root / 'expected-requests.json')
    status = ('PASS' if receipt['exit'] == 0 and 'FLUTTER_STRICT_PASS' in text and 'FLUTTER_FACADE_PASS' in text and 'All tests passed!' in text
              and counts == required and bodies_match and description_preserved and relation == {relation_name: 1}
              and bulk == {'hold-bulk-first': 1}
              and counts['hold-scalar'] == 1 and 'hold-scalar' in server.completions
              else 'FAIL')
    result = {'status': status, 'plant': plant, 'receipt': receipt, 'counts': counts, 'relation': relation, 'bulk': bulk,
              'required': required, 'bodies_match': bodies_match, 'description_preserved': description_preserved, 'completions': server.completions,
              'markers': {'facade': 'FLUTTER_FACADE_PASS' in text, 'strict': 'FLUTTER_STRICT_PASS' in text, 'flutter_test': 'All tests passed!' in text}}
    (logs / 'outcome.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({'status': status, 'logs': str(logs), 'exit': receipt['exit'],
                      'arrivals': len(server.arrivals), 'bulk': bulk, 'relation': relation}, indent=2))
    if status != 'PASS': sys.exit(1)
