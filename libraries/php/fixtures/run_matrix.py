"""Bounded, independently counted PHP FFI/engine-C tests on numeric loopback."""
import collections
import datetime
import json
import os
from pathlib import Path
import signal
import subprocess
import sys
import threading
import time
from backend import Backend, unquoted_single

ROOT = Path(__file__).resolve().parent.parent
REPO = ROOT.parent.parent
PHP_BIN = os.environ.get('THINKTHEN_PHP_BIN', '/usr/bin/php8.3')
PYTHON_BIN = os.environ.get('THINKTHEN_PYTHON_BIN', '/usr/bin/python3')
RUN = REPO / 'target/php' / ('run-' + datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ'))
RUN.mkdir(parents=True)
(RUN / 'barrier').mkdir()
(RUN / 'home').mkdir()
(RUN / 'cache').mkdir()
server = Backend(RUN / 'barrier')
ENV = {'PATH': '/usr/bin:/bin', 'HOME': str(RUN / 'home'), 'XDG_CONFIG_HOME': str(RUN / 'home'),
       'XDG_CACHE_HOME': str(RUN / 'home'), 'THINKTHEN_CACHE': str(RUN / 'cache'),
       'THINKTHEN_BASE_URL': f'http://127.0.0.1:{server.server_port}/generic/v1',
       'THINKTHEN_API_KEY': 'tt-canary-291', 'TT_BARRIER_DIR': str(RUN / 'barrier'),
       'TT_LIBRARY': str(REPO / 'libraries/c/target/debug/libthinkthen_c.so'),
       'LD_LIBRARY_PATH': str(REPO / 'libraries/c/target/debug'), 'TT_AUTOLOAD': str(ROOT / 'autoload.php')}
receipts = []
active = None

def stop_group(proc, reason):
    signals = []
    for sig in (signal.SIGTERM, signal.SIGKILL):
        try:
            os.killpg(proc.pid, sig)
            signals.append(sig.name)
        except ProcessLookupError:
            pass
        if sig == signal.SIGTERM:
            try: proc.wait(timeout=0.5)
            except subprocess.TimeoutExpired: pass
    proc.wait(timeout=5)
    return signals + [reason]

def execute(label, command, timeout=120):
    global active
    receipt = {'case': label, 'command': command, 'timeout_seconds': timeout, 'signals': []}
    with (RUN / (label+'.log')).open('wb') as log:
        active = subprocess.Popen(command, cwd=ROOT, env=ENV, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
        receipt.update(pid=active.pid, pgid=active.pid)
        receipts.append(receipt)
        (RUN/'receipts.json').write_text(json.dumps(receipts, indent=2)+'\n')
        try: receipt['exit'] = active.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            receipt['signals'] = stop_group(active, 'timeout'); receipt['exit'] = 'timeout'
        except BaseException:
            receipt['signals'] = stop_group(active, 'interrupted'); receipt['exit'] = 'interrupted'; raise
        finally:
            active = None
            (RUN/'receipts.json').write_text(json.dumps(receipts, indent=2)+'\n')
    return receipt['exit']

status = 'interrupted'
try:
    status = 'PASS'
    commands = [('direct', [PHP_BIN, '-d', 'ffi.enable=1', 'examples/direct.php']),
                ('matrix', [PHP_BIN, '-d', 'ffi.enable=1', 'fixtures/matrix.php']),
                ('native-strict', [PYTHON_BIN, 'fixtures/native_strict.py'])]
    def release_deadline():
        marker = RUN/'barrier/arrived-hold-deadline'
        limit = time.monotonic()+90
        while not marker.exists() and time.monotonic()<limit:
            time.sleep(.01)
        if marker.exists():
            time.sleep(.2)
            (RUN/'barrier/release-hold-deadline').touch()
    releaser = threading.Thread(target=release_deadline, daemon=True)
    releaser.start()
    for label, command in commands:
        if execute(label, command) != 0:
            status = 'FAILED:' + label
            break
    if status == 'PASS':
        for label, marker in [('direct', 'DIRECT_PHP_PASS'), ('matrix', 'PHP_MATRIX_PASS'),
                              ('native-strict', 'STRICT_C_CANCEL_PASS')]:
            if marker not in (RUN/(label+'.log')).read_text(): status = 'MISSING_MARKER:' + label
finally:
    if active is not None: stop_group(active, 'interrupted')
    server.close()
    counted = {'arrivals': server.arrivals, 'attempts': server.attempts,
               'completions': server.completions, 'connections': server.connections,
               'bulk_completion': server.bulk_completion}
    (RUN/'counts.json').write_text(json.dumps(counted, indent=2, ensure_ascii=False)+'\n')
    norm = lambda state: json.dumps(state, sort_keys=True, separators=(',', ':'), ensure_ascii=False) if isinstance(state,dict) else state
    counts = collections.Counter(map(norm, server.arrivals))
    expected = collections.Counter({s: 1 for s in ('direct-php','direct-json','café','yes','no','unsure','a\0b',
         'malformed-backend','transport-close','retry-status',
         'post-failure-recovery','maximum-deadline',
         'json-decide','choose','tag','score',
         'annotate-one','annotate-on','Ada Lovelace','failure-one','failure-two','success',
         'hold-deadline','recovery-scalar','hold-native','native-recovery')})
    # The repeated first/second bulk call reads the question cache the
    # first/second/third call wrote (ADR 0111), so it sends nothing.
    expected['Each question quotes the text it asks about.'] = 5
    expected.update({'Maria Chen': 2, 'John Smith': 2})
    expected['[{"id":"u001","evidence":"find-one"},{"id":"u002","evidence":"find-two"}]'] = 1
    expected['[{"id":"u001","evidence":"find-none"},{"id":"u002","evidence":"find-another"}]'] = 1
    for first, second in [('First','Second'),('Third','Fourth')]:
        expected[norm({'entities':[{'id':'i1','name':first,'kind':'alert'},
              {'id':'i2','name':second,'kind':'alert'}],
              })] = 1
    bodies = [json.loads(line) for line in (RUN/'barrier/request-bodies.jsonl').read_text().splitlines()]
    canonical = lambda request: json.dumps(request, sort_keys=True, separators=(',', ':'), ensure_ascii=False)
    observed_bodies = collections.Counter(map(canonical, bodies))
    accepted_bodies = collections.Counter(canonical(json.loads(line)) for line in
        (ROOT/'fixtures/accepted_requests.jsonl').read_text().splitlines())
    assert sum(accepted_bodies.values()) == 39, 'accepted full-body fixture is incomplete'
    packed=[]
    for request in bodies:
        # A request quoting one record is that record's own request, not a packed batch.
        if request['state']=='Each question quotes the text it asks about.' and unquoted_single(request) is request:
            import re
            rows=[]
            for question in request['questions'].values():
                match=re.fullmatch(r'The text is ("(?:\\.|[^"\\])*")\. Is it\?',question['instructions'])
                assert match is not None,question
                rows.append(json.loads(match.group(1)))
            packed.append(rows)
    expected_packed=[['bulk-before-bad','bulk-middle-bad','bulk-after-bad'],
                     ['first','second','third'],
                     ['filter-one','filter-two'],['rank-one','rank-two'],
                     [f'hold-bulk-{i}' for i in range(1,7)]]
    (RUN/'packed-rows.json').write_text(json.dumps(packed,indent=2)+'\n')
    if status == 'PASS' and (counts != expected or observed_bodies != accepted_bodies or packed != expected_packed
                             or server.attempts != len(server.arrivals)
                             or server.bulk_completion != []):
        status = 'COUNT_MISMATCH'
    (RUN/'outcome.json').write_text(json.dumps({'status':status, 'arrivals':len(server.arrivals),
                                                'expected':dict(expected), 'observed':dict(counts),
                                                'full_body_match':observed_bodies==accepted_bodies,
                                                'full_body_extra':list((observed_bodies-accepted_bodies).items()),
                                                'full_body_missing':list((accepted_bodies-observed_bodies).items()),
                                                'receipts':receipts},indent=2,ensure_ascii=False)+'\n')
    print(RUN, status, 'arrivals', len(server.arrivals), flush=True)
if status != 'PASS': sys.exit(1)
facts_barrier = RUN / 'facts-barrier'
facts_barrier.mkdir()
facts_server = Backend(facts_barrier)
original_env = ENV
try:
    ENV = ENV | {'THINKTHEN_BASE_URL': f'http://127.0.0.1:{facts_server.server_port}/generic/v1',
                 'THINKTHEN_CACHE': str(RUN / 'facts-cache'), 'TT_BARRIER_DIR': str(facts_barrier),
                 'TT_FACTS_PROOF': '1'}
    exit_code = execute('omitted-facts', [PHP_BIN, '-d', 'ffi.enable=1', 'fixtures/matrix.php'], timeout=60)
    assert exit_code == 0 and 'PHP_OMITTED_FACTS_PASS' in (RUN / 'omitted-facts.log').read_text()
    assert facts_server.attempts == 2 and collections.Counter(facts_server.arrivals) == {
        'no-usage': 1, 'Each question quotes the text it asks about.': 1}
    print('PHP_OMITTED_FACTS_PASS arrivals=2')
finally:
    ENV = original_env
    facts_server.close()
