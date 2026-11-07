"""Cancel a held stream through the installed Python package, as cancel.py directs."""
from concurrent.futures import ThreadPoolExecutor
import json
import os
from pathlib import Path

import thinkthen as tt

case = json.loads(Path(os.environ['THINKTHEN_STRESS_CASE']).read_text(encoding='utf-8'))
client = case['client']
retry = client['retry']
token = tt.Cancel()
with tt.Client(backend=client['backend'], url=client['url'], api_key=client['api_key'], model=client['model'],
               batch=client['batch'], retry=tt.RetryPolicy(retry['max_retries'], retry['initial_backoff_ms'] / 1000,
                                                           retry['attempt_timeout_ms'] / 1000)) as owner:
    with owner.decide_stream('Ready?', ('body' for _ in range(case['records'])), cancel=token) as stream, \
            ThreadPoolExecutor(max_workers=1) as pool:
        pending = pool.submit(next, stream)
        assert input() == 'cancel'
        token.cancel()
        try:
            pending.result()
        except tt.Cancelled as error:
            assert error.code == 'local.cancelled', error.code
        else:
            raise AssertionError('held stream returned a row')
        assert list(stream) == []
print(json.dumps({'calls': 1, 'rows': 0, 'failed': 0, 'cancelled': True}))
