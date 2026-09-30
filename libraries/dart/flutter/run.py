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
if plant not in ('', 'extra-post', 'wrong-probability'):
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
    'PUB_CACHE': os.environ['PUB_CACHE'],
    'TT_NATIVE_LIBRARY': str(native), 'FLUTTER_SUPPRESS_ANALYTICS': 'true',
}
command = [flutter, 'test', '--no-pub', '--reporter=expanded', 'test/facade_test.dart']
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
    # The planted extra request is a real numeric-loopback POST, not a fabricated ledger row.
    if plant == 'extra-post':
        body = server.last_body
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
    counts = collections.Counter(server.arrivals)
    text = (logs / 'consumer.log').read_text(errors='replace')
    bodies_match = matches(barrier / 'requests.jsonl', root / 'expected-requests.json')
    status = ('PASS' if receipt['exit'] == 0 and 'FLUTTER_FACADE_PASS' in text and 'All tests passed!' in text
              and counts == {'flutter-facade': 1} and bodies_match else 'FAIL')
    result = {'status': status, 'plant': plant, 'receipt': receipt, 'counts': counts, 'bodies_match': bodies_match,
              'markers': {'facade': 'FLUTTER_FACADE_PASS' in text, 'flutter_test': 'All tests passed!' in text}}
    (logs / 'outcome.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({'status': status, 'logs': str(logs), 'exit': receipt['exit'], 'counts': counts}, indent=2))
    if status != 'PASS': sys.exit(1)
