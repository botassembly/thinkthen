"""Run the literal ideal Bash guide with shared scenario/assertion support."""
import csv
import json
import os
from pathlib import Path
import subprocess

from guide_examples import IDEAL, blocks, check, scenario

FAILURE = 'failed: backend.status: the backend answered status 503: {"error":"unavailable"}'


def execute(source, folder, env):
    return subprocess.run(['bash', '-euo', 'pipefail', '-c', source], cwd=folder,
                          env=env, capture_output=True, text=True, timeout=90)


def prepare(examples, folder, env, command, audit=False):
    env['PATH'] = str(Path(command).resolve().parent) + os.pathsep + env['PATH']
    # The literal guide supplies its own records and JSON setup documents.
    setup = examples[8][1].split('\nthinkthen ', 1)[0]
    check('bash/setup/messages', execute(setup, folder, env))
    messages = (folder / 'messages.txt').read_text().splitlines()
    for index, filename in ((18, 'questions.json'), (24, 'toolbox.json')):
        (folder / filename).write_text(examples[index][1])
    records = [dict(id='a' + str(i), text=message) for i, message in enumerate(messages, 1)]
    (folder / 'messages.jsonl').write_text(''.join(json.dumps(row) + '\n' for row in records))
    with (folder / 'messages.csv').open('w', newline='') as file:
        writer = csv.DictWriter(file, fieldnames=['id', 'text'])
        writer.writeheader()
        writer.writerows(records)
    if audit:
        prefix = examples[28][1].split('\nthinkthen audit', 1)[0]
        check('bash/setup/saved-answers', execute(prefix, folder, env))
        rows = [json.loads(line) for line in (folder / 'outcomes.jsonl').read_text().splitlines()]
        source = rows[0]['source']
        manifest = dict(version=1, function='decide',
                        question=dict(function='decide', question='Does the customer request a refund?', field='/text'),
                        name=None, question_identity=source['question_identity'], backend=source['backend'],
                        model=source['model'], input_representation='jsonl/v1',
                        offset_unit='code_points', expected_cases=[row['id'] for row in records])
        (folder / 'run.json').write_text(json.dumps(manifest))
        (folder / 'labels.jsonl').write_text(''.join(json.dumps(dict(case_id=row['id'],
            question_identity=source['question_identity'], gold=True)) + '\n' for row in records))
        for name in ('left', 'right'):
            (folder / (name + '.jsonl')).write_text((folder / 'outcomes.jsonl').read_text())
            (folder / (name + '-run.json')).write_text(json.dumps(manifest))
    return messages


def expected(index, name, messages):
    failed = name == 'failed' or name.endswith('-failed')
    if index == 0:
        return ('refund queue\n' if name == 'yes' else ''), 0
    if index == 1:
        return ({'yes': 'refund queue\n', 'no': 'inbox\n', 'unsure': 'refund queue\n', 'failed': ''}[name], 6 if failed else 0)
    if index == 2:
        return ({'yes': 'refund queue\n', 'no': 'inbox\n', 'unsure': 'a person reads it\n', 'failed': 'a person reads it\n'}[name], 0)
    if index == 3:
        return ({'yes': 'refund queue\n', 'no': 'inbox\n', 'unsure': 'a person reads it 0.5\n', 'failed': 'try again later backend.status\n'}[name], 0)
    if index == 4:
        return ('could not judge it: ' + FAILURE + '\n' if failed else ''), 0
    if index == 5:
        return '', 6 if failed else 0
    if index == 6:
        return ('0.95\n' if name == 'yes' else '0.05\n' if name == 'no' else '0.5\n' if name == 'unsure' else 'null\n'), 6 if failed else 0
    if index == 7:
        return ({'yes': 'billing team\n', 'no': 'warehouse\n', 'unsure': 'a person reads it\n', 'failed': '', 'other': 'front desk\n'}[name], 6 if failed else 0)
    if index == 8:
        prefix = {'yes': 'refund queue: ', 'no': 'inbox: ', 'unsure': 'a person reads it: ', 'failed': 'try again later: '}[name]
        return ''.join(prefix + message + '\n' for message in messages), 6 if failed else 0
    if index == 10:
        return (''.join('refund queue: ' + message + '\n' for message in messages) if name == 'yes' else ''), 6 if failed else 0
    if index == 11:
        return ('try again later: some messages could not be judged\n' if failed else ''), 0
    if index == 12:
        return ('billing team\nbilling\nshipping\n' if name == 'yes' else ''), 6 if failed else 0
    if index == 13:
        return ('answer today\n1.00\n' if name == 'yes' else '0.00\n' if name == 'low-score' else ''), 6 if failed else 0
    if index == 14:
        return (''.join(str(i) + ' ' + message + '\n' for i, message in enumerate(messages, 1)) if not failed else ''), 6 if failed else 0
    if index == 15:
        return ({'yes': 'Please refund my order.\n', 'no': 'no line answers\n', 'unsure': 'a person reads it\n', 'failed': ''}[name], 6 if failed else 0)
    if index in (16, 17):
        return (('person Morgan\norganization Example Group\n' if index == 16 else 'Morgan works_for Example Group\n') if name == 'relations' else FAILURE + '\n' if failed else ''), 6 if failed else 0
    if index == 19:
        if failed:
            return ''.join('try again later: ' + message + '\n' for message in messages), 6
        if 'unsure' in name:
            return ''.join('a person reads it: ' + message + '\n' for message in messages), 0
        return 'refund queue: billing 1\n' * 3 if name == 'yes' else '', 0
    if index == 20:
        return 'true\nbilling\n', 0
    if index == 21:
        return None, 0
    if index in (22, 23):
        return 'messages.thinkthen.csv\n', 6 if failed else 0
    if index == 25:
        return None, 6 if failed else 0
    if index in (26, 27):
        return 'true\n', 0
    if index >= 28:
        return None, 0
    raise AssertionError('unaccounted Bash example ' + str(index))


def cases():
    boundary = lambda label: {value: float(value == label) for value in ('BEGIN', 'INSIDE', 'END', 'SINGLE', 'OUT')}
    return {'empty': {'replies': [dict(criterion='BEGIN', distribution=boundary('OUT'))]},
            'other': {'replies': [dict(distribution={'billing': 0., 'shipping': 0., 'other': 1.})]}}


def validate(index, name, result, folder, messages):
    if index in (21, 25):
        row = json.loads(result.stdout)
        assert row['id'] == 'a1' and row['item'] == dict(id='a1', text=messages[0]), row
        assert row['status'] == ('failed' if name == 'failed' else 'answered'), row
        if name != 'failed':
            assert row['answer'] is True and row['probability'] == .95, row
    if index in (22, 23):
        with (folder / 'messages.thinkthen.csv').open() as file:
            rows = list(csv.DictReader(file))
        assert len(rows) == 3
        assert [row['is_refund'].lower() for row in rows] == (['true'] * 3 if name == 'yes' else ['false'] * 3 if name == 'no' else [''] * 3), rows
        if index == 23:
            assert [row['is_refund_status'] for row in rows] == [{'yes': 'answered', 'no': 'answered', 'unsure': 'unsure', 'failed': 'failed'}[name]] * 3, rows
    if index == 11:
        assert (folder / 'refunds.txt').read_text() == ('\n'.join(messages) + '\n' if name == 'yes' else ''), result
    if index == 27:
        usage = json.loads(result.stderr)['usage']
        assert usage['attempts'] == 1 and usage['active_attempts'] == 0, usage
    if index == 28:
        report = json.loads(result.stdout)
        assert report['counts']['correct'] == 3 and report['counts']['missing_results'] == 0, report
        assert len(report['bins']) == 10, report
    if index in (29, 30):
        lines = result.stdout.splitlines()
        assert len(lines) == (2 if index == 29 else 1), result
        for line in lines:
            report = json.loads(line)
            assert report['counts']['changed_answer'] == 0, report
            assert report['counts']['matched'] == 3 and report['counts']['unchanged'] == 3, report
            assert len(report['cases']) == 3, report
            assert report['left']['counts']['correct'] == report['right']['counts']['correct'] == 3, report
    if name == 'failed' or name.endswith('-failed'):
        assert 'thinkthen:' in result.stderr and 'backend.' in result.stderr, result
    elif index in (22, 23) and name == 'unsure':
        assert result.stderr == 'thinkthen: 3 rows not sure, 0 rows failed\n', result.stderr
    elif index != 27:
        assert not result.stderr, result.stderr


def run_bash_ideal(command, negative=None):
    authored = blocks(IDEAL / 'bash.md')
    assert len(authored) == 30, 'inventory changed; account for new source fences'
    # Preserve stable scenario addresses after moving the illustrative output
    # out of the public guide. Every remaining public fence still runs below.
    examples = {index + (index >= 9): example for index, example in enumerate(authored)}
    scripted = cases()
    failures = []
    for index, (language, source, line) in examples.items():
        if language != 'bash':
            assert index in (18, 24), (index, language)
            continue
        names = ['yes', 'no', 'unsure', 'failed'] if index in (0, 1, 2, 3, 6, 7, 8, 15, 22, 23) else ['yes', 'failed'] if index in (4, 5, 10, 11, 12, 14, 25) else ['yes']
        if index in (0, 1):
            names = ['yes', 'no', 'failed']
        if index == 7:
            names.append('other')
        if index == 10:
            names = ['yes', 'no', 'failed']
        if index == 13:
            names = ['yes', 'low-score', 'failed']
        if index in (16, 17):
            names = ['relations', 'empty', 'failed']
        if index == 19:
            names = ['yes', 'no', 'topic-unsure', 'failed', 'refund-failed', 'topic-failed', 'urgency-failed']
        for name in names:
            label = f'bash/ideal/{line}/{name}'
            try:
                with scenario(name if name not in scripted else 'yes', case=scripted.get(name)) as (folder, env):
                    messages = prepare(examples, folder, env, command, audit=index >= 28)
                    result = execute(source, folder, env)
                    output, code = expected(index, name, messages)
                    control = negative if index == 0 and name == 'yes' else None
                    check(label, result, output=output, exit_code=code,
                          branch='refund queue' if index == 0 and name == 'yes' else None, negative=control)
                    validate(index, name, result, folder, messages)
            except (AssertionError, RuntimeError, OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
                if negative:
                    raise
                failures.append(label + ': ' + str(error))
    for label, run in [('form-text', lambda: run_form_text(examples, command)),
                       ('printed-listing', lambda: run_listing(examples, command)),
                       ('refusals', lambda: run_refusals(examples, command, negative))]:
        try:
            run()
        except (AssertionError, RuntimeError, OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
            failures.append('bash/ideal/' + label + ': ' + str(error))
    if failures:
        raise AssertionError('\n'.join(failures))
    print('PASS bash ideal guide: every source fence accounted for; output, branches, exits and refusals checked')


def run_form_text(examples, command):
    with scenario('refund-failed') as (folder, env):
        prepare(examples, folder, env, command)
        source = examples[19][1].split('|', 1)[0].replace('--json', '')
        result = execute(source, folder, env)
        value = 'refund=failed: backend.reply: the backend gave a malformed answer to the question\ttopic=billing\turgency=1'
        check('bash/ideal/form-text/failed-child', result,
              output=''.join(str(i) + '\t' + value + '\n' for i in range(1, 4)), exit_code=6)
        assert 'backend.reply' in result.stderr


def run_listing(examples, command):
    case = {'replies': [dict(contains='Please refund my order.', probability=.95),
                        dict(contains='Where is my parcel?', probability=.5),
                        dict(status=503, contains='I was charged twice for one invoice.', body={'error': 'unavailable'})]}
    with scenario('yes', case=case) as (folder, env):
        prepare(examples, folder, env, command)
        source = 'thinkthen ' + examples[8][1].split('\nthinkthen ', 1)[1].split('|', 1)[0].replace('--json', '') + ' --batch 1'
        result = execute(source, folder, env)
        check('bash/ideal/printed-stream-listing', result, output='1\ttrue\n2\tunsure\n3\t' + FAILURE + '\n', exit_code=6)
        assert 'backend.status' in result.stderr


def run_refusals(examples, command, negative):
    base = examples[5][1].split("is_refund=$(", 1)[1].rsplit(")", 1)[0]
    checks = [
        ('blank', base.replace('Please refund my order.', '')),
        ('filter-json', base.replace('decide ', 'filter ', 1) + ' --json'),
        ('filter-low', base.replace('decide ', 'filter ', 1) + ' --low 0.2'),
        ('filter-high', base.replace('decide ', 'filter ', 1) + ' --high 0.8'),
        ('old-yes', base + ' --true yes'),
        ('old-no', base + ' --false no'),
        ('bad-bar', base + ' --threshold 2'),
        ('stream-check', "thinkthen decide 'Does the customer request a refund?' messages.txt --from lines --check"),
        ('no-key', "unset THINKTHEN_API_KEY; " + base),
        ('team-default', examples[7][1].split('\nstatus=0', 1)[0] + '\nteam_for unknown'),
        ('column-clash', 'thinkthen annotate questions.json messages.csv --column text --details'),
    ]
    for label, source in checks:
        observer = []
        with scenario('yes', observer=observer) as (folder, env):
            prepare(examples, folder, env, command)
            if label == 'column-clash':
                path = folder / 'questions.json'
                body = json.loads(path.read_text())
                body['questions']['refund_status'] = body['questions'].pop('topic')
                path.write_text(json.dumps(body))
            result = execute(base if negative == 'refusal' and label == 'blank' else source, folder, env)
            check('bash/ideal/refusal/' + label, result, exit_code=2,
                  refusal=r'(usage\.|record.empty|no team for)')
            observer[0].stdin.write('count\n')
            observer[0].stdin.flush()
            assert observer[0].stdout.readline().strip() == '0', label + ': refused call reached backend'


if __name__ == '__main__':
    import sys
    run_bash_ideal(sys.argv[1])
