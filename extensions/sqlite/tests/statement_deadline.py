"""A statement reply slower than 30 s arrives; a reply that never comes still times out."""
from pathlib import Path
import runpy
import sys
import time

stress = runpy.run_path(str(Path(__file__).with_name('stress.py')))
CASE = {'client': {'url': 'http://127.0.0.1:9/v1', 'api_key': 'loopback-only'}}
# The stand-in consumer answers one statement after a delay, or never.
CONSUMER = '''import sys, time
sys.stdin.readline()
if sys.argv[1] == "never":
    sys.stdin.readline()
    sys.exit(0)
time.sleep(float(sys.argv[1]))
print('{"code": 0, "rows": [[1]]}', flush=True)
sys.stdin.readline()
'''


def host(delay):
    return stress['Host'](CASE, [sys.executable, '-c', CONSUMER, delay])


assert stress['STATEMENT_SECONDS'] > 30
slow = host('35')
started = time.monotonic()
try:
    assert slow.sql('SELECT 1') == [[1]]
finally:
    slow.close()
print(f'slow statement reply arrived after {time.monotonic() - started:.1f} s')

missing = host('never')
started = time.monotonic()
try:
    missing.sql('SELECT 1')
except TimeoutError:
    waited = time.monotonic() - started
    assert waited >= stress['STATEMENT_SECONDS'], waited
    print(f'missing statement reply timed out after {waited:.1f} s')
else:
    raise AssertionError('a missing statement reply did not time out')
finally:
    missing.close(check=False)
