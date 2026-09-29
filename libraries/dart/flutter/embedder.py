"""Run a real Linux Flutter engine under Xvfb, or report missing build prerequisites."""
import collections
import datetime
import json
import os
import pathlib
import shutil
import signal
import subprocess
import sys
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env
import time
from fixture import Backend
sys.path.insert(0, str(pathlib.Path(__file__).resolve().parent.parent / 'checks'))
from request_identity import matches

fixture_root = pathlib.Path(__file__).resolve().parent
root = pathlib.Path(os.environ.get('TT_FLUTTER_SOURCE', fixture_root)).resolve()
flutter = os.environ['TT_FLUTTER']
native = pathlib.Path(os.environ['TT_NATIVE_LIBRARY'])
logs = pathlib.Path(os.environ.get('TT_EMBEDDER_LOGS', root / 'logs')).resolve()
logs.mkdir(parents=True, exist_ok=True)
missing = []
if shutil.which('ninja') is None:
    missing.append('ninja-build')
if subprocess.run(['pkg-config', '--exists', 'gtk+-3.0'], env=child_env()).returncode != 0:
    missing.append('libgtk-3-dev')
if missing:
    status = {'status': 'GATE-BLOCKED-SYSTEM-DEPS', 'missing_packages': missing,
              'build_probe': 'logs/linux-build-probe.log',
              'build_probe_exit': 1, 'embedder_executed': False, 'arrivals': []}
    (logs / 'embedder-outcome.json').write_text(json.dumps(status, indent=2) + '\n')
    print(f'GATE-BLOCKED-SYSTEM-DEPS {", ".join(missing)}')
    sys.exit(78)

stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%S%fZ')
home = logs / ('embedder-home-' + stamp)
(home / 'cache').mkdir(parents=True, exist_ok=True)
barrier = logs / ('embedder-barrier-' + stamp)
barrier.mkdir()
server = Backend(barrier)
env = {
    'PATH': os.environ['PATH'],
    'HOME': str(home), 'XDG_CONFIG_HOME': str(home), 'XDG_CACHE_HOME': str(home / 'cache'),
    'PUB_CACHE': os.environ['PUB_CACHE'], 'FLUTTER_SUPPRESS_ANALYTICS': 'true',
    'THINKTHEN_BASE_URL': f'http://127.0.0.1:{server.server_port}/generic/v1',
    'THINKTHEN_API_KEY': 'tt-canary-300', 'THINKTHEN_CACHE': str(home / 'cache'),
    'TT_NATIVE_LIBRARY': str(native),
}
receipt = {'status': 'FAIL', 'build': {}, 'run': {}, 'arrivals': [], 'required': {'flutter-embedder': 1}}
def run_bounded(command, cwd, output_path, timeout):
    with output_path.open('wb') as output:
        process = subprocess.Popen(command, cwd=cwd, env=env, stdout=output, stderr=subprocess.STDOUT,
                                   start_new_session=True)
        result = {'command': command, 'pid': process.pid, 'pgid': process.pid, 'exit': None,
                  'timeout_seconds': timeout, 'signals': []}
        try:
            result['exit'] = process.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGTERM); result['signals'].append('SIGTERM timeout')
            try: process.wait(timeout=3)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL); result['signals'].append('SIGKILL timeout')
                process.wait(timeout=5)
            result['exit'] = 'TIMEOUT'
    return result

try:
    command = [flutter, 'build', 'linux', '--no-pub']
    receipt['build'] = run_bounded(command, root / 'example', logs / 'embedder-build.log', 240)
    if receipt['build']['exit'] != 0:
        raise RuntimeError('Linux Flutter build failed; see embedder-build.log')
    binary = root / 'example/build/linux/x64/release/bundle/thinkthen_flutter_example'
    command = ['/usr/bin/xvfb-run', '-a', str(binary)]
    with (logs / 'embedder-run.log').open('wb') as output:
        process = subprocess.Popen(command, cwd=binary.parent, env=env, stdout=output,
                                   stderr=subprocess.STDOUT, start_new_session=True)
        result = {'command': command, 'pid': process.pid, 'pgid': process.pid,
                  'timeout_seconds': 45, 'signals': [], 'exit': None}
        receipt['run'] = result
        try:
            end = time.monotonic() + 45
            while time.monotonic() < end:
                if process.poll() is not None:
                    result['exit'] = process.returncode
                    break
                if 'FLUTTER_EMBEDDER_PASS outcome=Outcome.yes probability=0.9' in (logs / 'embedder-run.log').read_text(errors='replace'):
                    result['marker_seen'] = True
                    break
                time.sleep(.1)
        finally:
            if process.poll() is None:
                os.killpg(process.pid, signal.SIGTERM)
                result['signals'].append('SIGTERM after marker' if result.get('marker_seen') else 'SIGTERM timeout')
                try: process.wait(timeout=4)
                except subprocess.TimeoutExpired:
                    os.killpg(process.pid, signal.SIGKILL)
                    result['signals'].append('SIGKILL timeout')
                    process.wait(timeout=5)
            result['exit'] = process.returncode
    receipt['arrivals'] = list(server.arrivals)
    receipt['bodies_match'] = matches(barrier / 'requests.jsonl', fixture_root / 'expected-embedder.json')
    if result.get('marker_seen') and collections.Counter(server.arrivals) == {'flutter-embedder': 1} and receipt['bodies_match']:
        receipt['status'] = 'PASS'
    else:
        raise RuntimeError('embedder did not produce marker and exact counted arrival')
except Exception as error:
    receipt['error'] = str(error)
finally:
    server.close()
    (logs / 'embedder-outcome.json').write_text(json.dumps(receipt, indent=2) + '\n')
print('FLUTTER_EMBEDDER_PASS' if receipt['status'] == 'PASS' else f'FLUTTER_EMBEDDER_FAIL {receipt.get("error")}')
sys.exit(0 if receipt['status'] == 'PASS' else 1)
