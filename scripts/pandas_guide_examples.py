"""Execute the dedicated pandas guide against the existing scripted backend."""
import ast
import json
import subprocess
from textwrap import indent

from bash_guide_examples import FAILURE, cases
from guide_examples import IDEAL, blocks, check, scenario
from python_guide_examples import run_enum_compiler

MESSAGES = ['Please refund my order.', 'Where is my parcel?', 'I was charged twice for one invoice.']


def declarations(source):
    return '\n'.join(ast.get_source_segment(source, node) for node in ast.parse(source).body
                     if isinstance(node, (ast.Import, ast.ImportFrom, ast.ClassDef, ast.FunctionDef)))


def literal(python, source, proof, expected, folder, env):
    """Check values independently, then use native rendering for table layout only."""
    path = folder / 'example.py'
    path.write_text('import io, contextlib, warnings, sys\n'
                    '_stdout = io.StringIO()\n'
                    'with warnings.catch_warnings(record=True) as _warnings, contextlib.redirect_stdout(_stdout):\n'
                    '    warnings.simplefilter("always")\n' + indent(source, '    ') + '\n' + proof + '\n'
                    'assert _stdout.getvalue() == (' + expected + '), repr(_stdout.getvalue())\n'
                    'sys.stdout.write(_stdout.getvalue())\n')
    return subprocess.run([python, '-I', str(path)], cwd=folder, env=env,
                          capture_output=True, text=True, timeout=90)


def selected_case(name):
    if name == 'shipping':
        return {'replies': [dict(distribution={'billing': 0., 'shipping': 1., 'other': 0.})]}
    return cases().get(name)


def run_case(python, library, index, name, source, proof, expected, *, raises=False,
             negative=None):
    override = selected_case(name)
    with scenario('yes' if override else name, case=override) as (folder, env):
        (folder / 'toolbox.json').write_text(json.dumps({'entries': {'topic': {'question': {
            'function': 'choose', 'question': 'What is the message about?',
            'choices': ['billing', 'shipping', 'other'], 'field': '/text'}}}}))
        result = literal(python, source, proof, expected, folder, env)
        control = negative if index == (0 if library == 'pandas' else 1) and name == 'yes' and negative in ('output', 'branch', 'exit') else None
        check(f'{library}/guide/{index}/{name}', result, exit_code=1 if raises else 0,
              output='' if raises else None, refusal=r'Failed:' if raises else None,
              negative=control)


def table_refusals(python, library, negative=None):
    imports = ('import pandas as pd\nimport thinkthen.pandas\n' if library == 'pandas' else
               'import polars as pl\nimport thinkthen.polars\n')
    make = 'pd.Series(["evidence"])' if library == 'pandas' else 'pl.Series("text", ["evidence"])'
    checks = [('filter-band', 'values.thinkthen.filter("Ready?", threshold=(0.2, 0.8), client=client)',
               'tt.UsageError', 'usage.threshold')]
    for details in (False, True):
        for names in (['status'], ['probability'], ['item'], ['refund', 'refund_status']):
            question = 'tt.AnnotationQuestions({name: tt.DecideQuestion("Ready?") for name in ' + repr(names) + '})'
            checks.append(('collision-' + '-'.join(names) + str(details),
                           f'values.thinkthen.annotate({question}, details={details!r}, client=client)',
                           'tt.UsageError', names[-1]))
    checks.append(('whole-column-truth', 'bool(values)', 'ValueError' if library == 'pandas' else 'TypeError', 'ambiguous'))
    if library == 'pandas':
        checks.append(('missing-cell-truth', 'bool(pd.NA)', 'TypeError', 'ambiguous'))
    checks.extend([
        ('errors-option', 'values.thinkthen.decide("Ready?", errors="ignore", client=client)', 'ValueError', 'errors'),
        ('filter-details', 'values.thinkthen.filter("Ready?", details=True, client=client)', 'TypeError', 'details'),
        ('filter-coerce', 'values.thinkthen.filter("Ready?", errors="coerce", client=client)', 'TypeError', 'errors'),
    ])
    for label, call, error, diagnostic in checks:
        with scenario('yes') as (folder, env):
            if negative == 'refusal' and label == 'filter-band':
                call = call.replace('threshold=(0.2, 0.8)', 'threshold=0.7')
            diagnostic_check = f'getattr(failure, "code", None) == {diagnostic!r}' if diagnostic.startswith('usage.') else f'{diagnostic!r} in str(failure)'
            source = 'import thinkthen as tt\n' + imports + f'values = {make}\n' + f'''with tt.Client() as client:
    try:
        {call}
    except {error} as failure:
        assert {diagnostic_check}
        assert client.usage.attempts == 0
        print("refused before transport")
    else:
        raise AssertionError("promised refusal accepted")
'''
            result = literal(python, source, '', repr('refused before transport\n'), folder, env)
            check(f'{library}/refusal/{label}', result, output='refused before transport\n')



def form_policies(python, library, setup):
    """Prove the documented plain/details/coerce policy on the same form inputs."""
    pandas = library == 'pandas'
    rows = 'lambda frame, name: frame[name].tolist()' if pandas else 'lambda frame, name: frame[name].to_list()'
    missing = 'pd.Series([None], dtype=object)' if pandas else 'pl.Series("message", [None], dtype=pl.String)'
    question_missing = 'pd.isna(value)' if pandas else 'value is None'
    for name in ('yes', 'no', 'unsure', 'failed', 'refund-failed', 'topic-failed', 'urgency-failed', 'refund-unsure', 'topic-unsure', 'missing-parent'):
        is_failure = name == 'failed' or name.endswith('-failed') or name == 'missing-parent'
        source = setup + '\ncolumn = ' + rows + '\n'
        if name == 'missing-parent':
            source += 'messages = ' + missing + '\n'
        source += f'''try:
    plain = messages.thinkthen.annotate(questions)
except tt.Failed as failure:
    assert {is_failure!r}, "unexpected plain form failure"
    outcomes = failure.outcomes
    assert column(outcomes, "status") == {["failed"] if name == "missing-parent" else ["answered"] * 3!r}
else:
    assert not {is_failure!r}, "plain form hid a failed entry"
    assert list(plain.columns) == ["refund", "topic", "urgency"]
assert sum(issubclass(w.category, tt.FailedRowsWarning) for w in _warnings) == 0
coerced = messages.thinkthen.annotate(questions, errors="coerce")
failures = coerced.thinkthen.failures()
assert bool(len(failures)) is {is_failure!r}
assert sum(issubclass(w.category, tt.FailedRowsWarning) for w in _warnings) == {int(is_failure)}
if {is_failure!r}:
    assert column(failures, "position") == {([1] if name == "missing-parent" else [position for position in (1, 2, 3) for _ in range(3 if name == "failed" else 1)])!r}
    assert set(None if {question_missing} else value for value in column(failures, "question")) == {({None} if name == "missing-parent" else {"refund", "topic", "urgency"} if name == "failed" else {name.removesuffix("-failed")})!r}
assert not hasattr(coerced.thinkthen, "raise_if_failed")
print("verified form policies")
'''
        run_case(python, library, 'form-policies-' + name, 'yes' if name == 'missing-parent' else name,
                 source, '', repr('verified form policies\n'))

def pandas_prelude(examples, index):
    source = 'import pandas as pd\nimport thinkthen as tt\nimport thinkthen.pandas\n'
    source += declarations(examples[4][1]) + '\n' + declarations(examples[11][1]) + '\n'
    source += 'messages = pd.Series(' + repr(MESSAGES) + ', index=["same", "same", "last"], name="message")\n'
    source += 'message = "Morgan works for Example Group.\\nPlease refund my order."\n'
    source += examples[18][1].split('\nchecks =', 1)[0] + '\n'
    source += examples[21][1].split('\nchecks =', 1)[0] + '\n'
    if index in (2, 3):
        source += 'messages = pd.Series(["Where is my parcel?"], name="message")\n'
    source += 'def cells(column):\n    return [None if pd.isna(value) else value for value in column]\n'
    if index in (6, 7):
        source += examples[5][1].split('\nprint(', 1)[0] + '\n'
    if index == 19:
        source += examples[10][1].split('\nfor _, row', 1)[0] + '\n'
    return source


def pandas_case(index, name):
    failed = name == 'failed'
    state = 'failed' if failed else 'unsure' if name == 'unsure' else 'answered'
    value = None if state != 'answered' else name != 'no'
    route = 'try again later' if failed else 'a person reads it' if name == 'unsure' else 'inbox' if name == 'no' else 'refund queue'
    proof = ''
    output = repr('')
    raises = failed and index in (0, 1, 4, 8, 11, 17)
    if index in (0, 1):
        proof = f'assert str(is_refund.dtype) == "boolean"\nassert cells(is_refund) == [{value!r}]'
        output = repr(('refund queue' if index == 0 and name == 'unsure' else route) + '\n')
    elif index == 2:
        if failed:
            proof = '_expected = pd.DataFrame({"position": [1], "status": ["failed"], "error_code": ["backend.status"]})'
            output = repr('could not judge it: ' + FAILURE.removeprefix('failed: backend.status: ') + '\n') + ' + str(_expected) + "\\n"'
        else:
            proof = f'assert cells(is_refund) == [{name != "no"!r}]'
            output = 'str(is_refund) + "\\n"'
    elif index in (3, 5, 18, 21, 22, 23, 24):
        result = 'is_refund' if index == 3 else 'refund_checks' if index == 5 else 'checks'
        n = 1 if index == 3 else 3 if index in (5, 24) else 2
        proof = f'assert {result}["status"].tolist() == [{state!r}] * {n}\nassert {result}["position"].tolist() == list(range(1, {n + 1}))\nassert str({result}["answer"].dtype) == "boolean"'
        if index in (5, 18, 21, 22, 23):
            proof += f'\nassert {result}.index.tolist() == ' + repr(['same', 'same', 'last'] if index == 5 else ['same', 'same'])
        if index in (18, 21, 22, 23):
            proof += f'\nassert {result}["id"].tolist() == ["a1", "a2"]'
        if index == 3:
            probability = None if failed else .5 if name == 'unsure' else .05 if name == 'no' else .95
            proof += f'\nassert cells(is_refund["answer"]) == [{value!r}]\nassert cells(is_refund["probability"]) == [{probability!r}]'
            output = 'str(is_refund[["status", "answer", "probability"]]) + "\\n"'
        elif index == 5:
            proof += '\nassert refund_checks["item"].tolist() == ' + repr(MESSAGES)
            output = 'str(refund_checks[["position", "status", "answer", "item"]]) + "\\n"'
        elif index == 18:
            proof += f'\nassert cells(enriched["is_refund"]) == [{value!r}] * 2\nassert enriched["refund_status"].tolist() == [{state!r}] * 2'
            output = 'str(enriched) + "\\n"'
        elif index == 23:
            attempts = 4 if failed else 1
            proof += f'\nassert usage.attempts == {attempts}\nassert usage.responses == usage.attempts\nassert usage.input_tokens.reported == usage.output_tokens.reported == 0'
            output = repr(f'{attempts} 0 0\n')
    elif index == 4:
        choice = 'shipping' if name in ('no', 'shipping') else 'other' if name == 'other' else 'billing'
        proof = 'assert list(topic.cat.categories) == ["billing", "shipping", "other"]\nassert topic.cat.ordered\nassert cells(topic) == [' + repr(None if name == 'unsure' else choice) + ']'
        output = repr(('a person reads it' if name == 'unsure' else {'billing': 'billing team', 'shipping': 'warehouse', 'other': 'front desk'}[choice]) + '\n')
    elif index in (6, 7):
        sep = ': ' if index == 6 else ' '
        suffix = ' backend.status' if failed and index == 6 else ''
        output = repr(''.join(route + sep + message + suffix + '\n' for message in MESSAGES))
        proof = f'assert refund_checks["status"].tolist() == [{state!r}] * 3'
    elif index == 8:
        proof = 'assert refund_requests.tolist() == ' + repr(MESSAGES if name == 'yes' else []) + '\nassert refund_requests.index.tolist() == ' + repr(['same', 'same', 'last'] if name == 'yes' else [])
        output = repr(''.join('refund queue: ' + m + '\n' for m in MESSAGES) if name == 'yes' else '')
    elif index == 9:
        if not failed:
            proof = 'assert refund_requests.tolist() == ' + repr(MESSAGES if name == 'yes' else [])
        if failed:
            proof = 'assert len(outcomes) == 3\nassert outcomes["status"].tolist() == ["failed"] * 3\nassert len(kept) == 0\nassert outcomes["error_code"].tolist() == ["backend.status"] * 3'
            output = 'str(kept[["position", "item"]]) + "\\n" + str(outcomes.loc[outcomes["status"] == "failed", ["position", "error_code"]]) + "\\n"'
    elif index in (10, 19):
        labels = [] if name == 'no' else ['billing', 'shipping']
        proof = f'assert topics["status"].tolist() == [{"failed" if failed else "answered"!r}]\nassert topics["answer"].tolist() == [{None if failed else labels!r}]'
        if index == 10:
            output = repr('try again later: backend.status\n' if failed else 'billing team\nbilling\nshipping\n' if labels else '')
        else:
            proof += '\nassert len(expanded) == ' + str(1 if failed else max(1, len(labels))) + '\nassert expanded["position"].tolist() == [1] * len(expanded)'
            output = repr('1 try again later\n' if failed else '1 no labels\n' if not labels else '1 billing\n1 shipping\n')
    elif index == 11:
        number = 0.0 if name == 'low-score' else 1.0
        proof = f'assert urgency.tolist() == [{number}]\nassert str(urgency.dtype) == "Float64"'
        output = repr(('answer today\n' if number else '') + str(number) + '\n')
    elif index == 12:
        proof = 'assert ranked["position"].tolist() == [1, 2, 3]\nassert ranked.index.tolist() == ["same", "same", "last"]\nassert str(ranked["answer"].dtype) == "Int64"'
        proof += '\nassert cells(ranked["answer"]) == ' + repr([None] * 3 if failed else [1, 2, 3])
        output = repr(''.join(('try again later: ' + m if failed else str(i) + ' ' + m) + '\n' for i, m in enumerate(MESSAGES, 1)))
    elif index == 13:
        output = repr({'yes': 'Please refund my order.\n', 'no': 'no line answers\n', 'unsure': 'a person reads it\n', 'failed': 'try again later: backend.status\n'}[name])
        proof = f'assert action["status"].tolist() == [{state!r}]'
        if name == 'no':
            proof += '\nassert action["answer"].iloc[0] is None'
    elif index in (14, 15):
        result = 'names' if index == 14 else 'relations'
        proof = f'assert {result}["status"].tolist() == [{"failed" if failed else "answered"!r}]'
        if index == 14:
            output = repr('try again later: backend.status\n' if failed else 'person Morgan\norganization Example Group\n' if name == 'relations' else '')
            if name == 'relations':
                proof += '\nassert [(x.kind, x.text) for x in names["answer"].iloc[0]] == [("person", "Morgan"), ("organization", "Example Group")]'
        else:
            output = repr('try again later: backend.status\n' if failed else "Morgan works_for Example Group\n['Morgan', 'Example Group']\n" if name == 'relations' else '[]\n')
        if name == 'empty':
            proof += f'\nassert {result}["answer"].iloc[0] == []'
    elif index == 16:
        child_failed = failed or name.endswith('-failed')
        unsure = name.endswith('-unsure') or name == 'unsure'
        proof = 'assert forms["status"].tolist() == ["answered"] * 3'
        for child in ('refund', 'topic', 'urgency'):
            child_state = 'failed' if failed or name == child + '-failed' else 'unsure' if name == child + '-unsure' or name == 'unsure' and child in ('refund', 'topic') else 'answered'
            proof += f'\nassert forms["{child}_status"].tolist() == [{child_state!r}] * 3'
        output = repr(''.join(('try again later: ' if child_failed else 'a person reads it: ' if unsure else 'inbox: ') + m + '\n' for m in MESSAGES) if child_failed or unsure or name == 'no' else 'refund queue: billing 1.0\n' * 3)
    elif index == 17:
        proof = f'assert cells(is_refund) == [{name != "no"!r}] * 3\nassert len(topic) == 3'
    elif index == 20:
        proof = f'assert cells(answers) == [{value!r}] * 3\nassert len(failures) == {3 if failed else 0}'
        if failed:
            proof += '\nassert failures["position"].tolist() == [1, 2, 3]\nassert failures["error_code"].tolist() == ["backend.status"] * 3'
        output = 'str(failures) + "\\n"'
    else:
        raise AssertionError('unaccounted pandas fence ' + str(index))
    if not raises:
        proof += f'\nassert sum(issubclass(w.category, tt.FailedRowsWarning) for w in _warnings) == {int(index == 20 and failed)}'
    return proof, output, raises


def run_pandas(python, negative=None):
    examples = blocks(IDEAL / 'pandas.md')
    if negative == 'refusal':
        table_refusals(python, 'pandas', negative)
        raise AssertionError('negative refusal control was accepted')
    assert len(examples) == 25 and all(language == 'python' for language, _, _ in examples), 'account for every pandas fence'
    failures = []
    for index, (_, body, _) in enumerate(examples):
        names = ['yes', 'no', 'unsure', 'failed']
        if index in (0, 2, 8, 9, 17, 23, 24):
            names = ['yes', 'no', 'failed']
        if index == 4:
            names += ['shipping', 'other']
        if index == 11:
            names = ['yes', 'low-score', 'failed']
        if index in (14, 15):
            names = ['relations', 'empty', 'failed']
        if index == 16:
            names += ['refund-failed', 'topic-failed', 'urgency-failed', 'refund-unsure', 'topic-unsure']
        for name in names:
            try:
                proof, output, raises = pandas_case(index, name)
                run_case(python, 'pandas', index, name, pandas_prelude(examples, index) + body,
                         proof, output, raises=raises, negative=negative)
            except (AssertionError, RuntimeError, OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
                if negative:
                    raise
                failures.append(f"pandas/guide/{index}/{name}: {error}")
    for label, run in [
        ('forms', lambda: form_policies(python, 'pandas', pandas_prelude(examples, 16) + examples[16][1].split('\nforms =', 1)[0])),
        ('refusals', lambda: table_refusals(python, 'pandas', negative)),
        ('compiler', lambda: run_enum_compiler(python, examples[4][1], 'pandas')),
    ]:
        try:
            run()
        except (AssertionError, RuntimeError, OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
            if negative:
                raise
            failures.append(label + ": " + str(error))
    if failures:
        raise AssertionError("\n".join(failures))
    print('PASS pandas guide: all 25 literal fences, scenario outputs, branches and refusals')
