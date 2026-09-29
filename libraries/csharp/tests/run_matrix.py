"""Local, allow-listed offline .NET fixture. Own every child process group."""
import collections
import datetime
import json
import os
import pathlib
import signal
import subprocess
import sys
import time
from backend import Backend
from toolchains import dotnet as resolve_dotnet

root = pathlib.Path(__file__).resolve().parent.parent
dotnet = str(resolve_dotnet())
stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
logs = root / 'target/logs' / ('run-' + stamp)
logs.mkdir(parents=True)
barrier = logs / 'barrier'
barrier.mkdir()
home = logs / 'home'
(home / 'cache').mkdir(parents=True)
for name in ('dotnet-home', 'nuget', 'dotnet-cache'):
    (root / 'target/scratch' / name).mkdir(parents=True, exist_ok=True)
server = Backend(barrier)
env = {
    'PATH': '/usr/bin:/bin', 'HOME': str(home), 'XDG_CONFIG_HOME': str(home),
    'XDG_CACHE_HOME': str(root / 'target/scratch/dotnet-cache'),
    'DOTNET_CLI_HOME': str(root / 'target/scratch/dotnet-home'),
    'NUGET_PACKAGES': str(root / 'target/scratch/nuget'),
    'DOTNET_CLI_TELEMETRY_OPTOUT': '1', 'DOTNET_SKIP_FIRST_TIME_EXPERIENCE': '1',
    'DOTNET_NOLOGO': '1', 'DOTNET_MULTILEVEL_LOOKUP': '0',
    'LD_LIBRARY_PATH': str(root / 'target/scratch/lib'),
    'THINKTHEN_BASE_URL': f'http://127.0.0.1:{server.server_port}/generic/v1',
    'THINKTHEN_API_KEY': 'tt-canary-290', 'THINKTHEN_CACHE': str(home / 'cache'),
    'TT_BARRIER_DIR': str(barrier),
}
receipts = []
active = None

def stop_group(process, reason):
    signals = []
    for kind in (signal.SIGTERM, signal.SIGKILL):
        try:
            os.killpg(process.pid, kind)
            signals.append(kind.name)
        except ProcessLookupError:
            pass
        if kind == signal.SIGTERM:
            time.sleep(.2)
    process.wait(timeout=5)
    return signals + [reason]

def interrupted(*_):
    raise KeyboardInterrupt

signal.signal(signal.SIGTERM, interrupted)

def execute(label, command, timeout):
    global active
    with (logs / (label + '.log')).open('wb') as output:
        active = subprocess.Popen(command, cwd=root, env=env, stdout=output,
                                  stderr=subprocess.STDOUT, start_new_session=True)
        receipt = {'case': label, 'pid': active.pid, 'pgid': active.pid,
                   'command': command, 'timeout_seconds': timeout, 'signals': []}
        receipts.append(receipt)
        (logs / 'receipts.json').write_text(json.dumps(receipts, indent=2) + '\n')
        try:
            receipt['exit'] = active.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            receipt['signals'] = stop_group(active, 'timeout')
            receipt['exit'] = 'timeout'
        except BaseException:
            receipt['signals'] = stop_group(active, 'interrupted')
            receipt['exit'] = 'interrupted'
            raise
        finally:
            active = None
            (logs / 'receipts.json').write_text(json.dumps(receipts, indent=2) + '\n')
    return receipt['exit']

status = 'interrupted'
try:
    dll = root / 'tests/bin/Release/net8.0/Consumer.dll'
    status = 0
    if status == 0:
        status = execute('build', [dotnet, 'build', str(root / 'tests/Consumer.csproj'),
                                   '--configuration', 'Release', '--source', str(root / 'target/scratch/nuget'), '-v', 'quiet'], 120)
    if status == 0:
        status = execute('direct', [dotnet, str(dll), 'direct'], 60)
    if status == 0:
        status = execute('matrix', [dotnet, str(dll), 'matrix'], 120)
    if status == 0:
        states = collections.Counter(str(s) for s in server.arrivals)
        required = {'direct': 1, 'café': 1, 'yes': 1, 'no': 1, 'unsure': 1, 'a\x00b': 1,
                    'packed:first,second,third': 1, 'packed:first,second': 1, 'json-decide': 1,
                    'choose': 1, 'tag': 1, 'score': 1,
                    'packed:filter-one,filter-two': 1, 'packed:rank-one,rank-two': 1,
                    '[{"id":"u001","evidence":"find-one"},{"id":"u002","evidence":"find-two"}]': 1,
                    "{'entities': [{'id': 'i1', 'name': 'First', 'kind': 'alert'}, {'id': 'i2', 'name': 'Second', 'kind': 'alert'}]}": 1,
                    "{'entities': [{'id': 'i1', 'name': 'Third', 'kind': 'alert'}, {'id': 'i2', 'name': 'Fourth', 'kind': 'alert'}]}": 1,
                    'annotate-one': 1, 'Maria Chen': 2, 'John Smith': 2,
                    'status-401': 1, 'failure-one': 1, 'failure-two': 1,
                    'success': 1, 'hold-deadline': 1, 'hold-scalar': 1,
                    'recovery-scalar': 1,
                    'packed:hold-bulk-1,hold-bulk-2,hold-bulk-3,hold-bulk-4,hold-bulk-5,hold-bulk-6': 1}
        # New literal request multiset: specification/records.md "Order and requests"
        # and ADR 0048 item 1 put six held rows into one counted POST.
        status = 'PASS' if (states == required
                            and server.attempts == server.connections == len(server.arrivals)
                            and server.bulk_completion == ['packed:first,second,third']
                            and 'hold-scalar' in server.completions) else 'COUNT_MISMATCH'
finally:
    if active is not None:
        stop_group(active, 'interrupted')
    server.close()
    (logs / 'arrivals.json').write_text(json.dumps(server.arrivals, ensure_ascii=False, indent=2) + '\n')
    (logs / 'completions.json').write_text(json.dumps(server.completions, ensure_ascii=False, indent=2) + '\n')
    outcome = {'logs': str(logs), 'status': status, 'arrivals': len(server.arrivals),
               'attempts': server.attempts, 'connections': server.connections,
               'completions': len(server.completions), 'bulk_completion': server.bulk_completion,
               'receipts': receipts}
    (logs / 'outcome.json').write_text(json.dumps(outcome, indent=2) + '\n')
    print(json.dumps(outcome, indent=2))
if status != 'PASS':
    sys.exit(1)
