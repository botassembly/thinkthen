"""Compile literal TypeScript guide modules once, then prove named outcomes."""
import json
import os
from pathlib import Path
import subprocess

from bash_guide_examples import FAILURE, cases
from guide_examples import IDEAL, ROOT, blocks, check, scenario

MESSAGES = ['Please refund my order.', 'Where is my parcel?', 'I was charged twice for one invoice.']
VERIFIED = 'verified\n'


def prelude(examples, index):
    source = '' if 'import * as tt ' in examples[index][1] else 'import * as tt from "thinkthen";\n'
    source += 'import assert from "node:assert/strict";\n'
    if index == 2:
        source += examples[1][1].split('\nif (isRefund.answered)', 1)[0] + '\n'
        source = source.replace('import * as tt from "thinkthen";\n', '', 1)
    if index == 10:
        source += examples[9][1].split('\nfor await ', 1)[0] + '\n'
        source = source.replace('import * as tt from "thinkthen";\n', '', 1)
    if index in (17, 18, 19):
        source += examples[5][1].split('\nconst topic = await', 1)[0] + '\n'
        if index == 17:
            source = source.replace('import * as tt from "thinkthen";\n', '', 1)
    if index in (13, 17, 18):
        source += examples[12][1].split("\nconst urgency = await", 1)[0].replace('import * as tt from "thinkthen";\n', "") + "\n"
    if index == 18:
        source += examples[17][1].split('\nconst forms = ', 1)[0].replace('import * as tt from "thinkthen";\n', '') + '\n'
    source = source.replace('import * as tt from "thinkthen";\n', '')
    if 'import * as tt ' not in examples[index][1]:
        source = 'import * as tt from "thinkthen";\n' + source
    return source


def observe(index):
    if index == 19:
        return '\nassert.equal(isRefund.answer, true); assert.equal(topic.answer, "billing"); console.log("verified");'
    if index in (21, 25):
        return '\nassert.equal(isRefund.function, "decide"); console.log("verified");'
    if index == 22:
        return '\nassert.equal(topic.answer, "billing"); console.log("verified");'
    if index == 23:
        return '\nassert.equal(client.usage.attempts, 1n); console.log("verified");'
    if index == 18:
        return '\nconsole.log("verified");'
    return ''


def compiler(consumer):
    return [str(ROOT / 'sdk/typescript/node_modules/.bin/tsc'), '--strict',
            '--noUncheckedIndexedAccess', '--exactOptionalPropertyTypes', '--module', 'NodeNext',
            '--target', 'ES2022', '--lib', 'ES2022,DOM,ESNext.Disposable', '--types', 'node',
            '--typeRoots', str(ROOT / 'sdk/typescript/node_modules/@types')]


def compile_examples(examples, consumer):
    sources = consumer / 'guide_sources'
    emitted = consumer / 'guide_compiled'
    sources.mkdir(exist_ok=True)
    paths = []
    for index, (language, body, _) in enumerate(examples):
        assert language == 'ts', (index, language)
        path = sources / f'example{index}.mts'
        path.write_text(prelude(examples, index) + body + observe(index) + '\n')
        paths.append(str(path))
    compiled = subprocess.run([*compiler(consumer), '--outDir', str(emitted), *paths],
                              cwd=consumer, capture_output=True, text=True, timeout=90)
    if compiled.returncode:
        print(compiled.stdout, end='')
    check('typescript/ideal/compiler/all-literal-fences', compiled)
    return emitted


def names_for(index):
    if index in (1, 2, 3, 4, 6, 7, 8, 14):
        return ['yes', 'no', 'unsure', 'failed']
    if index == 5:
        return ['yes', 'shipping', 'other', 'unsure', 'failed']
    if index in (9, 10, 11):
        return ['yes', 'no', 'failed']
    if index == 12:
        return ['yes', 'low-score', 'failed']
    if index in (15, 16):
        return ['relations', 'empty', 'failed']
    if index in (17, 18):
        return ['yes', 'no', 'topic-unsure', 'refund-unsure', 'failed', 'refund-failed', 'topic-failed', 'urgency-failed']
    if index in (0, 13, 20):
        return ['yes', 'no', 'failed']
    return ['yes']


def expected(index, name):
    failed = name == 'failed'
    route = {'yes': 'refund queue', 'no': 'inbox', 'unsure': 'a person reads it', 'failed': 'try again later'}
    if index in (0, 1, 2):
        return ('' if failed else route[name] + '\n'), failed
    if index == 3:
        message = FAILURE.removeprefix('failed: backend.status: ')
        return ('could not judge it: ' + message if failed else 'false' if name == 'no' else 'true') + '\n', False
    if index in (18, 19, 21, 22, 23, 25):
        raises = failed or name.endswith('-failed')
        return ('' if raises else VERIFIED), raises
    if index == 4:
        return ('' if failed else {'yes': '0.95', 'no': '0.05', 'unsure': '0.5'}[name] + '\n'), failed
    if index == 5:
        return {'yes': 'billing team\n', 'shipping': 'warehouse\n', 'other': 'front desk\n', 'unsure': 'a person reads it\n', 'failed': ''}[name], failed
    if index in (6, 7):
        prefix = route[name] + (': ' if index == 6 else ' ')
        return ''.join(prefix + message + '\n' for message in MESSAGES), False
    if index == 8:
        return ('' if failed else {'yes':'true\n', 'no':'false\n', 'unsure':'unsure\n'}[name]), failed
    if index in (9, 10):
        if failed:
            return (''.join('try again later: ' + message + ' backend.status\n' for message in MESSAGES) if index == 10 else ''), index == 9
        return (''.join('refund queue: ' + message + '\n' for message in MESSAGES) if name == 'yes' else ''), False
    if index == 11:
        return ('billing team\nbilling\nshipping\n' if name == 'yes' else ''), failed
    if index == 12:
        return ('answer today\n1.00\n' if name == 'yes' else '0.00\n' if name == 'low-score' else ''), failed
    if index == 13:
        return (''.join(str(i) + ' ' + message + '\n' for i, message in enumerate(MESSAGES, 1)) if not failed else ''), failed
    if index == 14:
        return {'yes': 'Please refund my order.\n', 'no': 'no line answers\n', 'unsure': 'a person reads it\n', 'failed': ''}[name], failed
    if index in (15, 16):
        return (('person Morgan\norganization Example Group\n' if index == 15 else 'Morgan works_for Example Group\n') if name == 'relations' else ''), failed
    if index == 17:
        if failed or name.endswith('-failed'):
            return ''.join('try again later: ' + message + '\n' for message in MESSAGES), False
        if name.endswith('-unsure'):
            return ''.join('a person reads it: ' + message + '\n' for message in MESSAGES), False
        return ('refund queue: billing 1.00\n' * 3 if name == 'yes' else ''), False
    if index == 20:
        return ('' if failed else route[name] + '\n' + ''.join(('true' if name == 'yes' else 'false') + ' ' + m + '\n' for m in MESSAGES[:2])), failed
    if index == 24:
        return '1n 0n 0n\n', False
    raise AssertionError('unaccounted literal fence ' + str(index))


def execute(path, folder, env):
    return subprocess.run(['node', str(path)], cwd=folder, env=env,
                          capture_output=True, text=True, timeout=90)


def run_examples(examples, emitted, negative):
    scripted = cases()
    scripted['shipping'] = {'replies': [dict(distribution={'billing': 0., 'shipping': 1., 'other': 0.})]}
    failures = []
    for index, (_, _, line) in enumerate(examples):
        for name in names_for(index):
            label = f'typescript/ideal/{line}/{name}'
            try:
                with scenario(name if name not in scripted else 'yes', case=scripted.get(name)) as (folder, env):
                    toolbox = {'entries': {'topic': {'question': {'function': 'choose', 'question': 'Topic?',
                                                                  'choices': ['billing', 'shipping', 'other']}}}}
                    (folder / 'toolbox.json').write_text(json.dumps(toolbox))
                    result = execute(emitted / f'example{index}.mjs', folder, env)
                    output, raises = expected(index, name)
                    check(label, result, output=output, exit_code=1 if raises else 0,
                          refusal=r'FailedError' if raises else None,
                          branch='refund queue' if index == 0 and name == 'yes' else None,
                          negative=negative if index == 0 and name == 'yes' and negative != 'refusal' else None)
            except (AssertionError, RuntimeError, OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
                if negative:
                    raise
                failures.append(label + ': ' + str(error))
    # Exercise the literal general-call-error handler without a paid request.
    with scenario('yes') as (folder, env):
        env.pop('THINKTHEN_API_KEY', None)
        result = execute(emitted / 'example3.mjs', folder, env)
        setup_output = 'the call did not complete: api_key: expected a credential in THINKTHEN_API_KEY; received an unset or blank credential (contents redacted)\n'
        check('typescript/ideal/call-error/setup', result,
              output=setup_output, branch=setup_output.rstrip('\n'))
    return failures


def run_refusals(consumer, negative):
    expressions = [('loose-equality', 'topic == "billing"', 'TypeError'),
                   ('concatenate', 'topic + ""', 'TypeError'), ('number', '+topic', 'TypeError'),
                   ('order', 'topic > 0.5', 'TypeError'),
                   ('missing-await', 'tt.decide("Ready?", "body").answer', r'TypeError.*await'),
                   ('single-array', 'await tt.decide("Ready?", ["body"])', r'TypeError.*decideStream'),
                   ('filter-bars', 'await tt.filter("Ready?", ["body"], {threshold:{low:0.2,high:0.8}}).toArray()', r'UsageError.*decideStream'),
                   ('nested-filter', 'await tt.annotate({bad:{function:"filter",question:"Ready?"}}, "body")', 'UsageError')]
    path = consumer / 'refusal.mjs'
    for name, expression, diagnostic in expressions:
        source = 'import * as tt from "thinkthen";\nconst topic = await tt.choose("Topic?", "body", {choices:["billing","shipping"]});\n'
        path.write_text(source + ('console.log("accepted");' if negative == 'refusal' else expression) + ';\n')
        with scenario('yes') as (folder, env):
            checked = execute(path, folder, env)
            check('typescript/ideal/refusal/' + name, checked, output='', exit_code=1, refusal=diagnostic)


def run_compile_refusals(consumer, examples):
    declarations = examples[5][1].split('\nconst topic = await', 1)[0]
    status = examples[7][1].split('\nconst messages = ', 1)[0]
    variants = [('topic', declarations, declarations.replace('"other"] as const', '"other", "new"] as const')),
                ('status', status, status.replace('    case "failed":\n      return "try again later";', '')),
                ('filter-bars', 'import * as tt from "thinkthen";\n tt.filter("Ready?", ["body"], {threshold:0.7});',
                 'import * as tt from "thinkthen";\n tt.filter("Ready?", ["body"], {threshold:{low:0.2,high:0.8}});'),
                ('missing-await', 'import * as tt from "thinkthen";\n (await tt.decide("Ready?", "body")).answer;',
                 'import * as tt from "thinkthen";\n tt.decide("Ready?", "body").answer;'),
                ('strict-equality', 'import * as tt from "thinkthen";\n (await tt.choose("Topic?","body",{choices:["billing"]})).answer === "billing";',
                 'import * as tt from "thinkthen";\n (await tt.choose("Topic?","body",{choices:["billing"]})) === "billing";')]
    for name, valid, refused in variants:
        path = consumer / 'compile-refusal.mts'
        for label, source, code in [('valid', valid, 0), ('refused', refused, 2)]:
            path.write_text(source)
            checked = subprocess.run([*compiler(consumer), '--noEmit', str(path)], cwd=consumer,
                                     capture_output=True, text=True, timeout=90)
            check('typescript/ideal/compiler/' + name + '/' + label, checked, exit_code=code,
                  refusal=r'error TS[0-9]+' if code else None)


def run_lint_refusal(consumer):
    # Use the project's existing checker and rule, with the installed public types.
    path = consumer / 'lint-example.mts'
    config = consumer / 'eslint.config.mjs'
    plugin = (ROOT / 'sdk/typescript/node_modules/typescript-eslint/dist/index.js').as_uri()
    config.write_text(f'''import tseslint from {json.dumps(plugin)};
export default [{{files:["**/*.mts"], languageOptions:{{parser:tseslint.parser,
parserOptions:{{project:"./lint-tsconfig.json",tsconfigRootDir:{json.dumps(str(consumer))}}}}},
plugins:{{"@typescript-eslint":tseslint.plugin}},rules:{{"@typescript-eslint/no-unnecessary-condition":"error"}}}}];''')
    (consumer / 'lint-tsconfig.json').write_text(json.dumps({'compilerOptions': {
        'strict': True, 'module': 'NodeNext', 'target': 'ES2022',
        'lib': ['ES2022', 'DOM', 'ESNext.Disposable'],
        'typeRoots': [str(ROOT / 'sdk/typescript/node_modules/@types')]}, 'files': [path.name]}))
    for label, condition, code in [('valid', 'isRefund.answer', 0), ('refused', 'isRefund', 1)]:
        path.write_text('import * as tt from "thinkthen";\nconst isRefund = await tt.decide("Ready?", "body");\n'
                        + f'if ({condition}) console.log("refund queue");\n')
        compiled = subprocess.run([*compiler(consumer), '--noEmit', str(path)], cwd=consumer,
                                  capture_output=True, text=True, timeout=90)
        check('typescript/ideal/lint/compiler-valid/' + label, compiled)
        result = subprocess.run([str(ROOT / 'sdk/typescript/node_modules/.bin/eslint'), '--config', str(config), str(path)],
                                cwd=consumer, capture_output=True, text=True, timeout=90)
        check('typescript/ideal/lint/object-condition/' + label, result, exit_code=code,
              refusal=r'@typescript-eslint/no-unnecessary-condition' if code else None)


def run_checker_controls(consumer, emitted):
    for name in ('output', 'branch', 'exit'):
        with scenario('yes') as (folder, env):
            checked = execute(emitted / 'example0.mjs', folder, env)
            check('typescript/ideal/control/baseline/' + name, checked, output='refund queue\n', branch='refund queue')
            try:
                check('typescript/ideal/control/' + name, checked, output='refund queue\n',
                      branch='refund queue', negative=name)
            except AssertionError:
                print('PASS typescript/ideal/control/reject-' + name)
            else:
                raise AssertionError('runner accepted deliberately wrong ' + name)
    path = consumer / 'accepted-refusal.mjs'
    path.write_text('import * as tt from "thinkthen";\n'
                    'const topic = await tt.choose("Topic?", "body", {choices:["billing","shipping"]});\n'
                    'if (topic.answer !== "billing") throw new Error("wrong control answer");\n'
                    'console.log("accepted");\n')
    with scenario('yes') as (folder, env):
        checked = execute(path, folder, env)
        check('typescript/ideal/control/baseline/accepted-refusal', checked, output='accepted\n')
        try:
            check('typescript/ideal/control/accepted-refusal', checked, exit_code=1, refusal='TypeError')
        except AssertionError:
            print('PASS typescript/ideal/control/reject-accepted-refusal')
        else:
            raise AssertionError('runner accepted a deliberately successful refusal')


def run_typescript(command=None, negative=None):
    consumer = Path(command or os.environ['THINKTHEN_TYPESCRIPT_CONSUMER']).resolve()
    examples = blocks(IDEAL / 'typescript.md')
    assert len(examples) == 26, 'inventory changed; account for each new literal fence'
    emitted = compile_examples(examples, consumer)
    if negative == 'refusal':
        return run_refusals(consumer, negative)
    failures = run_examples(examples, emitted, negative)
    for label, call in [('refusals', lambda: run_refusals(consumer, negative)),
                        ('compiler-refusals', lambda: run_compile_refusals(consumer, examples)),
                        ('lint-refusal', lambda: run_lint_refusal(consumer))]:
        try:
            call()
        except (AssertionError, RuntimeError, OSError, ValueError, KeyError, TypeError, subprocess.SubprocessError) as error:
            failures.append(label + ': ' + str(error))
    if failures:
        raise AssertionError('\n'.join(failures))
    run_checker_controls(consumer, emitted)
    print('PASS TypeScript ideal guide: all26 literal fences, outputs, branches and refusals checked')
