"""Execute every literal ideal Python fence with observable scenario assertions."""
import ast
import json
import subprocess

from bash_guide_examples import cases
from guide_examples import IDEAL, blocks, check, scenario

FAILURE = 'failed: backend.status: the backend answered status 503: {"error":"unavailable"}'
MESSAGES = ['Please refund my order.', 'Where is my parcel?', 'I was charged twice for one invoice.']
VERIFIED = 'verified\n'


def definitions(source):
    """Reuse the guide's literal declarations, without running its example calls."""
    tree = ast.parse(source)
    return '\n\n'.join(ast.get_source_segment(source, node) for node in tree.body
                       if isinstance(node, (ast.Import, ast.ImportFrom, ast.ClassDef, ast.FunctionDef)))


def prelude(examples, index):
    source = 'import thinkthen as tt\n'
    if index not in (5, 13):
        source += definitions(examples[5][1]) + '\n'
    if index != 14:
        source += definitions(examples[14][1]) + '\n'
    source += examples[6][1].split('\nwith tt.decide_stream', 1)[0] + '\n'
    if index == 2:
        source += examples[1][1].split('\nif is_refund.answered:', 1)[0] + '\n'
    if index in (8, 9):
        source += 'is_refund = tt.decide_stream("Does the customer request a refund?", messages[:1], threshold=(0.2, 0.8)).to_list()[0]\n'
    if index == 10:
        source += examples[9][1] + '\n'
    if index == 20:
        source += examples[19][1].split('\nwith tt.annotate_stream', 1)[0] + '\n'
    return source


def execute(python, source, folder, env):
    path = folder / 'example.py'
    path.write_text(source + '\n', encoding='utf-8')
    return subprocess.run([python, '-I', str(path)], cwd=folder, env=env,
                          capture_output=True, text=True, timeout=90)


def prepare(folder):
    (folder / 'messages.csv').write_text('id,text\n' + ''.join(
        f'a{i},{message}\n' for i, message in enumerate(MESSAGES, 1)))
    toolbox = {'entries': {'topic': {'question': {'function': 'choose',
        'question': 'What is the message about?', 'choices': ['billing', 'shipping', 'other']}}}}
    (folder / 'toolbox.json').write_text(json.dumps(toolbox))


def names_for(index):
    if index in (1, 2, 3, 4, 6, 7, 8, 9, 10, 16):
        return ['yes', 'no', 'unsure', 'failed']
    if index == 5:
        return ['yes', 'shipping', 'other', 'unsure', 'failed']
    if index == 14:
        return ['yes', 'low-score', 'failed']
    if index in (17, 18):
        return ['relations', 'empty', 'failed']
    if index in (19, 20):
        return ['yes', 'no', 'topic-unsure', 'failed', 'refund-failed', 'topic-failed', 'urgency-failed']
    if index in (21, 23, 24, 25, 26, 27):
        return ['yes']
    return ['yes', 'no', 'failed']


def expectation(index, name):
    failed = name == 'failed'
    route = {'yes': 'refund queue', 'no': 'inbox', 'unsure': 'a person reads it', 'failed': 'try again later'}
    scalar = (0, 1, 2, 4, 5, 13, 14, 16, 17, 18, 22)
    if failed and index in scalar:
        return '', True
    if index in (0, 1, 2):
        return route[name] + '\n', False
    if index == 3:
        value = 'could not judge it: ' + FAILURE.removeprefix('failed: backend.status: ') if failed else {'yes': 'true', 'no': 'false', 'unsure': 'true'}[name]
        return value + '\n', False
    if index == 4:
        return {'yes': '0.95\n', 'no': '0.05\n', 'unsure': '0.5\n'}[name], False
    if index == 5:
        return {'yes': 'billing team\n', 'shipping': 'warehouse\n', 'other': 'front desk\n', 'unsure': 'a person reads it\n'}[name], False
    if index == 6:
        return ''.join(route[name] + ': ' + message + '\n' for message in MESSAGES), False
    if index == 7:
        return ('' if failed else {'yes': 'true\n', 'no': 'false\n', 'unsure': 'unsure\n'}[name] * 3), failed
    if index == 8:
        return {'yes': 'refund queue', 'no': 'inbox', 'unsure': 'a person reads it 0.5', 'failed': 'try again later backend.status'}[name] + '\n', False
    if index == 9:
        return route[name] + '\n', False
    if index == 10:
        return ''.join(route[name] + ' ' + message + '\n' for message in MESSAGES), False
    if index in (11, 12):
        if failed:
            return (''.join('try again later: ' + message + ' backend.status\n' for message in MESSAGES) if index == 12 else ''), index == 11
        return (''.join('refund queue: ' + message + '\n' for message in MESSAGES) if name == 'yes' else ''), False
    if index == 13:
        return ('billing team\nbilling\nshipping\n' if name == 'yes' else ''), False
    if index == 14:
        return ('answer today\n1.0\n' if name == 'yes' else '0.0\n'), False
    if index == 15:
        return (''.join(str(i) + ' ' + message + '\n' for i, message in enumerate(MESSAGES, 1)) if not failed else ''), failed
    if index == 16:
        return {'yes': 'Please refund my order.\n', 'no': 'no line answers\n', 'unsure': 'a person reads it\n'}[name], False
    if index in (17, 18):
        return (('person Morgan\norganization Example Group\n' if index == 17 else 'Morgan works_for Example Group\n') if name == 'relations' else ''), False
    if index == 19:
        if failed or name.endswith('-failed'):
            return ''.join('try again later: ' + message + '\n' for message in MESSAGES), False
        if name == 'topic-unsure':
            return ''.join('a person reads it: ' + message + '\n' for message in MESSAGES), False
        return ('refund queue: billing 1.0\n' * 3 if name == 'yes' else ''), False
    if index == 20:
        raises = failed or name.endswith('-failed')
        return '' if raises else VERIFIED, raises
    if index == 22:
        return route[name] + '\n' + ''.join(('true' if name == 'yes' else 'false') + ' ' + m + '\n' for m in MESSAGES[:2]), False
    if index == 26:
        return '1 0 0\n', False
    if index in (21, 23, 24, 25, 27):
        return VERIFIED, False
    raise AssertionError('unaccounted Python fence ' + str(index))


def observed(index, name):
    if index == 9:
        return 'print(where_it_goes(is_refund))'
    if index in (20, 21, 23, 25, 27):
        if index == 20:
            return 'assert form.answered\nassert form.raise_if_failed() is form\nprint("verified")'
        return f'assert is_refund.answered and is_refund.answer is {name == "yes"!r}\nprint("verified")'
    if index == 24:
        return 'assert topic.answered and topic.answer == "billing"\nprint("verified")'
    if index == 19:
        return 'assert form.position == len(messages)'
    return ''


def run_examples(python, examples, negative):
    scripted = cases()
    scripted['shipping'] = {'replies': [dict(distribution={'billing': 0., 'shipping': 1., 'other': 0.})]}
    failures = []
    for index, (language, body, line) in enumerate(examples):
        assert language == 'python', (line, language)
        for name in names_for(index):
            label = f'python/ideal/{line}/{name}'
            try:
                with scenario(name if name not in scripted else 'yes', case=scripted.get(name)) as (folder, env):
                    prepare(folder)
                    result = execute(python, prelude(examples, index) + body + '\n' + observed(index, name), folder, env)
                    output, raises = expectation(index, name)
                    check(label, result, output=output, exit_code=1 if raises else 0,
                          refusal=r'Failed' if raises else None,
                          branch='refund queue' if index == 0 and name == 'yes' else None,
                          negative=negative if index == 0 and name == 'yes' and negative != 'refusal' else None)
            except (AssertionError, RuntimeError, OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
                if negative:
                    raise
                failures.append(label + ': ' + str(error))
    return failures


def run_forms(python, examples):
    setup = prelude(examples, 19) + examples[19][1].split('\nwith tt.annotate_stream', 1)[0]
    for name in ('yes', 'topic-unsure', 'failed', 'refund-failed', 'topic-failed', 'urgency-failed'):
        source = setup + '\nform = tt.annotate(questions, messages[0])\n'
        raises = name == 'failed' or name.endswith('-failed')
        if raises:
            source += 'form.raise_if_failed()\n'
        else:
            source += 'assert form.raise_if_failed() is form\nprint("verified")\n'
        with scenario(name) as (folder, env):
            result = execute(python, source, folder, env)
            check('python/ideal/form-raise/' + name, result, output='' if raises else VERIFIED,
                  exit_code=1 if raises else 0, refusal=r'Failed' if raises else None)


def refusal_cases():
    return [
        ('choose-bool', 'bool(topic)', 'TypeError'),
        ('choose-equality', 'topic == Topic.BILLING', 'TypeError'),
        ('tag-bool', 'bool(topics)', 'TypeError'),
        ('find-bool', 'bool(action)', 'TypeError'),
        ('recognize-bool', 'bool(names)', 'TypeError'),
        ('relate-bool', 'bool(relations)', 'TypeError'),
        ('iterate', 'iter(names)', 'TypeError'), ('length', 'len(names)', 'TypeError'),
        ('index', 'names[0]', 'TypeError'), ('slice', 'names[:1]', 'TypeError'),
        ('float', 'float(urgency)', 'TypeError'), ('int', 'int(urgency)', 'TypeError'),
        ('integer-index', '__import__("operator").index(urgency)', 'TypeError'),
        ('ordering', 'urgency > 0.5', 'TypeError'),
        ('find-start', 'action.start', 'AttributeError'), ('find-end', 'action.end', 'AttributeError'),
        ('form-index', 'form["refund"]', 'TypeError'),
        ('single-list', 'tt.decide("Ready?", ["body"])', 'TypeError'),
        ('two-bars', 'list(tt.filter("Ready?", messages, threshold=(0.2, 0.8)))', r'UsageError.*decide_stream'),
        ('filter-coerce', 'list(tt.filter("Ready?", messages, errors="coerce"))', 'TypeError'),
        ('filter-rows', 'tt.filter_rows', 'AttributeError'),
        ('bad-threshold', 'tt.decide("Ready?", "body", threshold=(0.8, 0.2))', 'UsageError'),
        ('bad-choices', 'tt.choose("Which?", "body", choices="not-a-list")', '(TypeError|UsageError)'),
    ]


def run_refusals(python, examples, negative):
    setup = prelude(examples, 0)
    for index, split in ((5, '\nif topic.unsure:'), (13, '\nif Topic.BILLING'),
                         (14, '\nif urgency.answer'), (16, '\nif action.unsure:'),
                         (17, '\nfor name in'), (18, '\nfor relation in')):
        setup += examples[index][1].split(split, 1)[0] + '\n'
    setup += examples[19][1].split('\nwith tt.annotate_stream', 1)[0] + '\nform = tt.annotate(questions, messages[0])\n'
    for name, expression, diagnostic in refusal_cases():
        with scenario('yes') as (folder, env):
            statement = 'print("incorrectly accepted")' if negative == 'refusal' else expression
            result = execute(python, setup + statement + '\n', folder, env)
            check('python/ideal/refusal/' + name, result, output='', exit_code=1, refusal=diagnostic)



def run_admission(python):
    checks = []
    for blank in ('', '   ', {}):
        checks.append(('blank-' + repr(blank), f'''try:
    tt.decide("Ready?", {blank!r})
except tt.Failed as failure:
    assert failure.code == "record.empty"
    print("verified")
else:
    raise AssertionError("blank call succeeded")'''))
    checks.append(('cancelled', '''stop = tt.Cancel()
stop.cancel()
try:
    tt.decide("Ready?", "body", cancel=stop)
except tt.Cancelled:
    print("verified")
else:
    raise AssertionError("cancelled call was accepted")'''))
    checks.append(('no-key', '''import os
os.environ.pop("THINKTHEN_API_KEY", None)
try:
    tt.Client().decide("Ready?", "body")
except tt.SetupError:
    print("verified")
else:
    raise AssertionError("missing key was accepted")'''))
    for library in ('pandas', 'polars'):
        frame = 'pd.DataFrame' if library == 'pandas' else 'pl.DataFrame'
        imports = 'import pandas as pd' if library == 'pandas' else 'import polars as pl'
        for details in (False, True):
            for clash in ('status', 'refund_status'):
                names = ['status'] if clash == 'status' else ['refund', 'refund_status']
                source = f'''{imports}
import thinkthen.{library}
frame = {frame}({{"text": ["evidence"]}})
questions = tt.AnnotationQuestions({{name: tt.DecideQuestion("Ready?") for name in {names!r}}})
try:
    frame.thinkthen.annotate(questions, field="/text", details={details!r})
except tt.UsageError as error:
    assert {clash!r} in str(error)
    print("verified")
else:
    raise AssertionError("clashing annotation name was accepted")'''
                checks.append((f'{library}/{clash}/{details}', source))
            source = f'''{imports}
import thinkthen.{library}
frame = {frame}({{"text": ["evidence"]}})
try:
    frame.thinkthen.filter("Ready?", field="/text", threshold=(0.2, 0.8))
except tt.UsageError as error:
    assert "decide_stream" in str(error)
    print("verified")
else:
    raise AssertionError("two filter bars were accepted")'''
            checks.append((f'{library}/filter-bars/{details}', source))
    for label, source in checks:
        observer = []
        with scenario('yes', observer=observer) as (folder, env):
            result = execute(python, 'import thinkthen as tt\n' + source, folder, env)
            check('python/ideal/admission/' + label, result, output=VERIFIED)
            observer[0].stdin.write('count\n')
            observer[0].stdin.flush()
            assert observer[0].stdout.readline().strip() == '0', label + ': refused call reached backend'


def run_compiler(python, examples):
    topic = definitions(examples[5][1])
    status = 'import thinkthen as tt\n' + examples[9][1]
    variants = [('topic', topic, topic.replace('    OTHER = "other"', '    OTHER = "other"\n    NEW = "new"')),
                ('status', status, status.replace('        case tt.Status.FAILED:\n            return "try again later"', ''))]
    compiler_variants(python, variants)


def run_enum_compiler(python, source, surface):
    topic = definitions(source)
    missing = topic.replace('    OTHER = "other"', '    OTHER = "other"\n    NEW = "new"')
    assert missing != topic, 'the literal enum needs its missing-case witness'
    compiler_variants(python, [('topic', topic, missing)], surface)


def compiler_variants(python, variants, surface='python'):
    with scenario('yes') as (folder, env):
        for name, valid, missing in variants:
            path = folder / (name + '.py')
            for label, source, code in [('valid', valid, 0), ('missing-case', missing, 1)]:
                path.write_text(source)
                args = ['mypy', '--strict', '--no-incremental', '--enable-incomplete-feature=TypeForm',
                        '--python-executable', python, str(path)]
                result = subprocess.run(args, cwd=folder, env=env, capture_output=True, text=True, timeout=90)
                check(surface + '/ideal/compiler/' + name + '/' + label, result, exit_code=code,
                      refusal=r'Missing return statement|missing return' if code else None)


def run_python_ideal(python, negative=None):
    examples = blocks(IDEAL / 'python.md')
    assert len(examples) == 28, 'inventory changed; account for every new fence'
    if negative == 'refusal':
        run_refusals(python, examples, negative)
        raise AssertionError('negative refusal control was accepted')
    failures = run_examples(python, examples, negative)
    for label, call in [('form-raise', lambda: run_forms(python, examples)),
                        ('refusals', lambda: run_refusals(python, examples, negative)),
                        ('compiler', lambda: run_compiler(python, examples)),
                        ('admission', lambda: run_admission(python))]:
        try:
            call()
        except (AssertionError, RuntimeError, OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
            failures.append(label + ': ' + str(error))
    if failures:
        raise AssertionError('\n'.join(failures))
    print('PASS Python ideal guide: all 28 fences, outputs, branches and refusals checked')
