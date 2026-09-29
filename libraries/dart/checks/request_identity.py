"""Compare the complete decoded wire bodies, including model and questions."""
import collections
import json
from pathlib import Path


def canonical(body):
    return json.dumps(body, ensure_ascii=False, sort_keys=True, separators=(',', ':'))


def matches(request_log, expected_path):
    actual = [json.loads(line)['body'] for line in Path(request_log).read_text().splitlines()]
    expected = json.loads(Path(expected_path).read_text())
    return collections.Counter(map(canonical, actual)) == collections.Counter(map(canonical, expected))


if __name__ == '__main__':
    import sys
    recorded = json.loads(Path(sys.argv[1]).read_text())
    if not recorded:
        raise SystemExit('EMPTY_REQUEST_FIXTURE')
    changed = json.loads(json.dumps(recorded))
    changed[0]['questions']['q1']['instructions'] = 'planted different question'
    if collections.Counter(map(canonical, changed)) == collections.Counter(map(canonical, recorded)):
        raise SystemExit('REQUEST_IDENTITY_PLANT_NOT_DETECTED')
    print(f'REQUEST_IDENTITY_PLANT_PASS {len(recorded)} bodies')
