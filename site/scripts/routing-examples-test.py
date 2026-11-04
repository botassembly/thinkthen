"""Route uncertain or failed answers to a person using the copied programs."""
import json
from pathlib import Path
import shutil
import subprocess
import tempfile

EXAMPLES = Path(__file__).resolve().parents[1] / 'examples'


def run_example(name, work):
    result = subprocess.run(
        ['bash', '-eu', str(EXAMPLES / name / 'show.sh')],
        cwd=work, capture_output=True, text=True, check=True, timeout=10,
    )
    assert not result.stderr, result.stderr
    return result.stdout.splitlines()


with tempfile.TemporaryDirectory(prefix='routing-examples-') as directory:
    work = Path(directory)
    messages = []
    routes = []
    people = []
    # All admitted decide/choose child answers, including rendered failures.
    for refund in (True, False, 'unsure', None):
        for topic in ('billing', 'shipping', 'other', 'unsure', None):
            name = f'm{len(messages)}'
            messages.append(dict(id=name, text=name, triage=dict(refund=refund, topic=topic)))
            if refund in ('unsure', None) or topic in ('unsure', None):
                queue = 'person'
                people.append(name)
            else:
                queue = 'refunds' if refund is True else topic
            routes.append(f'{name} {queue}')
    messages.append(dict(id='failed', text='failed', triage=None))
    routes.append('failed person')
    people.append('failed')
    (work / 'triaged.jsonl').write_text(''.join(json.dumps(m) + '\n' for m in messages))
    shutil.copyfile(EXAMPLES / 'howto-triage-route' / 'route.jq', work / 'route.jq')
    actual = run_example('howto-triage-route', work)
    assert actual == routes, f'route: actual={actual!r}, expected={routes!r}'
    actual = run_example('howto-triage-person', work)
    assert actual == people, f'triage people: {actual!r} != {people!r}'
    assert [json.loads(line)['id'] for line in (work / 'person.jsonl').read_text().splitlines()] == people

    pairs = [dict(same=answer, pair=dict(left=f'company{i}', right=f'alias{i}'))
             for i, answer in enumerate((True, False, 'unsure', None))]
    (work / 'matched.jsonl').write_text(''.join(json.dumps(p) + '\n' for p in pairs))
    actual = [json.loads(line) for line in run_example('howto-match-person', work)]
    assert actual == [pairs[2]['pair'], pairs[3]['pair']], actual
    assert [json.loads(line) for line in (work / 'same.jsonl').read_text().splitlines()] == [pairs[0]['pair']]
    assert [json.loads(line) for line in (work / 'person.jsonl').read_text().splitlines()] == actual

print('PASS: actual jq programs route 21 triage and 4 company answers; yes/no queues preserved')
