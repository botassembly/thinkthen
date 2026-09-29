"""Bounded C++ consumer with one counted local backend."""
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

root = pathlib.Path(__file__).resolve().parent
repo = root.parents[2]
stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
plant = sys.argv[1] if len(sys.argv) == 2 else ''
if plant not in ('', 'owned-facts', 'extra-bulk', 'wrong-relation', 'wrong-probability', 'swapped-bulk', 'wrong-json', 'wrong-annotate', 'wrong-recognize', 'wrong-relate', 'fail-mid-case'):
    raise SystemExit('unknown planted variant')
logs = repo / 'target/cpp/logs' / ('gate-' + stamp + ('-' + plant if plant else ''))
logs.mkdir(parents=True, exist_ok=False)
barrier = logs / 'barrier'
barrier.mkdir()
home = logs / 'home'
(home / 'cache').mkdir(parents=True)
server = Backend(barrier, plant)
env = {
    'PATH': '/usr/bin:/bin',
    'HOME': str(home), 'XDG_CONFIG_HOME': str(home), 'XDG_CACHE_HOME': str(home / 'cache'),
    'THINKTHEN_BASE_URL': f'http://127.0.0.1:{server.server_port}/generic/v1',
    'THINKTHEN_API_KEY': 'tt-canary-301', 'THINKTHEN_CACHE': str(home / 'cache'),
    'TT_BARRIER_DIR': str(barrier), 'TT_PLANT': plant,
    'TT_SOURCE': os.environ.get('CPP_CORPUS_ROOT', str(repo)),
    'ASAN_OPTIONS': 'detect_leaks=1:abort_on_error=1', 'UBSAN_OPTIONS': 'halt_on_error=1',
}
if plant == 'owned-facts': env['TT_FACTS_ONLY'] = '1'
command = [os.environ.get('CPP_CONSUMER', str(repo / 'target/cpp/shared-build/consumer'))]
receipt = {'command': command, 'pid': None, 'pgid': None, 'exit': None, 'signals': [], 'timeout_seconds': 140}
process = None
try:
    with (logs / 'consumer.log').open('wb') as output:
        process = subprocess.Popen(command, cwd=root, env=env, stdout=output, stderr=subprocess.STDOUT, start_new_session=True)
        receipt['pid'] = receipt['pgid'] = process.pid
        try:
            receipt['exit'] = process.wait(timeout=140)
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
    if plant in ('extra-bulk', 'wrong-relation'):
        body = server.held_bulk_body if plant == 'extra-bulk' else server.relation_body
        if body is None: raise RuntimeError(f'no captured request for {plant}')
        if plant == 'wrong-relation':
            body = dict(body)
            body['state'] = {'entities': [{'id': 'wrong', 'kind': 'alert', 'name': 'Other'}]}
        data = json.dumps(body).encode()
        request = urllib.request.Request(f'http://127.0.0.1:{server.server_port}/generic/v1/systemone', data=data,
                                         headers={'Authorization': 'Bearer tt-canary-301', 'Content-Type': 'application/json'})
        with urllib.request.urlopen(request, timeout=5) as response:
            if response.status != 200: raise RuntimeError(f'planted request status {response.status}')
            response.read()
finally:
    server.close()
    (logs / 'arrivals.json').write_text(json.dumps(server.arrivals, indent=2, ensure_ascii=False) + '\n')
    (logs / 'requests.jsonl').write_text(''.join(json.dumps(row, ensure_ascii=False) + '\n' for row in server.requests))
    (logs / 'completions.json').write_text(json.dumps(server.completions, indent=2, ensure_ascii=False) + '\n')
    counts = collections.Counter(server.arrivals)
    normalize = lambda rows: collections.Counter(json.dumps(row, sort_keys=True, ensure_ascii=False, separators=(',', ':')) for row in rows)
    accepted = [json.loads(row) for row in (root / 'accepted_requests.jsonl').read_text().splitlines()]
    full_body_match = normalize(server.requests) == normalize(accepted)
    required = {'yes': 1, 'no': 1, 'unsure': 1, 'described': 1, 'batch-pair': 1,
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
    owned_required = {'without-usage': 1, 'hold-owned-scalar': 1, 'hold-owned-bulk-first': 1}
    if plant == 'owned-facts':
        status = 'PASS' if receipt['exit'] == 0 and 'CPP_OWNED_FACTS_PASS' in text and counts == owned_required and \
            len(server.completions) == 3 else 'FAIL'
    else:
        status = ('PASS' if receipt['exit'] == 0 and 'CPP_PRODUCT_PASS' in text
              and counts == required and (plant or full_body_match) and relation == {relation_name: 1}
              and bulk == {'hold-bulk-first': 1}
              and counts['hold-scalar'] == 1 and 'hold-scalar' in server.completions
              else 'FAIL')
    result = {'status': status, 'plant': plant, 'receipt': receipt, 'counts': counts, 'relation': relation, 'bulk': bulk,
              'full_body_match': full_body_match, 'full_body_extra': list((normalize(server.requests)-normalize(accepted)).elements()),
              'full_body_missing': list((normalize(accepted)-normalize(server.requests)).elements()),
              'required': required, 'completions': server.completions}
    (logs / 'outcome.json').write_text(json.dumps(result, indent=2) + '\n')
    print(json.dumps({'status': status, 'logs': str(logs), 'exit': receipt['exit'],
                      'arrivals': len(server.arrivals), 'bulk': bulk, 'relation': relation}, indent=2))
    if status != 'PASS': sys.exit(1)
