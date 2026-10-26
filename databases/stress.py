"""SQL row checks for the shared offline stress workload."""
import json
import os
from pathlib import Path

from testing import child_environment


def workload():
    return json.loads(Path(os.environ['THINKTHEN_STRESS_CASE']).read_text())


def environment(case):
    client = case['client']
    return dict(child_environment(), THINKTHEN_TEST_LOOPBACK_URL=client['url'],
                THINKTHEN_API_KEY=client['api_key'], OPENAI_API_KEY=client['api_key'])


def settings(case):
    return {name: value for name, value in case['client'].items()
            if name not in ('url', 'api_key')}


def check_rows(case, rows, prefix):
    """Rows carry position, native source ID, status, answer, p and error code."""
    assert len(rows) == case['records'], (len(rows), case['records'])
    assert sorted(row[0] for row in rows) == list(range(1, case['records'] + 1))
    failed = 0
    widths = case.setdefault("identifier_widths", set())
    for position, identifier, status, answer, probability, code in rows:
        widths.add(len(identifier))
        assert identifier == f'{prefix}record-{case["identifier_base"] + position - 1}', (position, identifier)
        if status == 'answered':
            assert answer is True and probability == case['probability'], (position, answer, probability)
            assert code is None, (position, code)
            assert case['scenario']['name'] != 'refused'
        else:
            assert status == 'failed' and code == case['scenario']['code'], (position, status, code)
            assert answer is None and probability is None, (position, answer, probability)
            failed += 1
    return failed


def check_round(case, results):
    """Release completed result ownership before returning round counts."""
    calls = failed = 0
    for prefix, rows in results:
        failed += check_rows(case, rows, prefix)
        calls += 1
    return calls, failed


def report(case, calls, failed=0, cancelled=False):
    if not cancelled and case['scenario']['status'] == 'failed':
        assert failed > 0, case['scenario']
    print(json.dumps(dict(identifier_width_mask=sum(1 << width for width in case.get("identifier_widths", ())), calls=calls, rows=0 if cancelled else calls * case['records'],
                          failed=failed, cancelled=cancelled)), flush=True)
