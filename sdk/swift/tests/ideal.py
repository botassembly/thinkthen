#!/usr/bin/env python3
"""Compile each literal Swift fence once and execute its covered branches."""
from pathlib import Path
import os
import re
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'scripts'))
from guide_examples import blocks, check, scenario
from bash_guide_examples import cases as shared_cases

MESSAGES = ['Please refund my order.', 'Where is my parcel?', 'I was charged twice for one invoice.']
MESSAGE_SETUP = 'let messages = ["Please refund my order.", "Where is my parcel?", "I was charged twice for one invoice."]\n'
FAILURE = 'failed: backend.status: the backend answered status 503: {"error":"unavailable"}'
ENUMS = '''enum Topic: String, CaseIterable { case billing, shipping, other }
enum Urgency: String, CaseIterable { case routine, urgent }
'''


ARRAY_CALLS = {
    'decide': 'client.decide("Ready?", record: ["body"])',
    'choose': 'client.choose("Topic?", record: ["body"], choices: Topic.self)',
    'tag': 'client.tag("Topics?", record: ["body"], labels: Topic.self)',
    'score': 'client.score("Urgency?", record: ["body"], levels: Urgency.self)',
    'find': 'client.find("Which line?", record: ["body"])',
    'recognize': 'client.recognize(record: ["body"], kinds: ["person"])',
    'relate': 'client.relate(record: ["body"], rules: [RelationRule(source: "person", target: "organization", relation: "works_for")])',
    'annotate': 'client.annotate(AnnotationQuestions(["refund": DecideQuestion("Ready?")]), record: ["body"])',
}

def guide_sources():
    examples = blocks(ROOT / 'docs/surfaces/swift.md')
    if len(examples) != 26 or any(language != 'swift' for language, _, _ in examples):
        raise AssertionError('Swift fences changed: classify every example')
    source = ['\n'.join(line for line in body.splitlines() if not line.startswith('import ')) for _, body, _ in examples]
    return source


def runtime_cases():
    cases = {
        'annotation-json': ('_ = try AnnotationQuestions(json: "not-json")', r'UsageError', False),
        'missing-toolbox-file': ('_ = try Toolbox(contentsOf: URL(filePath: "missing.json"))', r'InternalError', False),
        'missing-annotation-file': ('_ = try AnnotationQuestions(contentsOf: URL(filePath: "missing.json"))', r'InternalError', False),
        'toolbox-write-file': ('try Toolbox(["refund": DecideQuestion("Ready?")]).write(to: URL(filePath: "missing/box.json"))', r'InternalError', False),
        'bad-threshold': ('_ = try await client.decide("Ready?", record: "body", threshold: 2)', r'UsageError', False),
        'missing-toolbox': ('_ = try Toolbox(["refund": DecideQuestion("Ready?")]).choose("missing")', r'UsageError', False),
        'wrong-toolbox': ('_ = try Toolbox(["refund": DecideQuestion("Ready?")]).choose("refund")', r'UsageError', False),
        'wrong-enum-domain': ('enum Wrong: String, CaseIterable { case wrong }; _ = try Toolbox(["topic": ChooseQuestion("Topic?", choices: Topic.self)]).choose("topic", choices: Wrong.self)', r'UsageError', False),
        'missing-entry': ('let form = try await client.annotate(AnnotationQuestions(["refund": DecideQuestion("Ready?")]), record: "body"); _ = try form.answer.choose("missing")', r'UsageError', True),
        'wrong-entry': ('let form = try await client.annotate(AnnotationQuestions(["refund": DecideQuestion("Ready?")]), record: "body"); _ = try form.answer.choose("refund")', r'UsageError', True),
        'rank-entry': ('_ = try await client.annotate(AnnotationQuestions(["bad": RankQuestion("Urgency?", levels: ["routine", "urgent"])]), record: "body")', r'UsageError', False),
        'filter-entry': ('_ = try await client.annotate(AnnotationQuestions(["bad": FilterQuestion("Ready?")]), record: "body")', r'UsageError', False),
        'saved-filter-band': ('let q = try JSONDecoder().decode(FilterQuestion.self, from: Data(#"{"function":"filter","question":"Ready?","threshold":{"low":0.2,"high":0.8}}"#.utf8)); _ = try await client.filter(q, records: ["body"]).collect()', r'UsageError.*decide\(_:\s*records:\)', False),
    }
    for name, key, levels in [('choose', 'choices', '["a", "b"]'),
                               ('tag', 'labels', '["a", "b"]'),
                               ('score', 'levels', '["a", "b"]'),
                               ('rank', 'levels', '["a", "b"]')]:
        argument = 'records: ["body"]' if name == 'rank' else 'record: "body"'
        expression = f'client.{name}("Question?", {argument}, {key}: {levels}, descriptions: ["typo": "lost?"])'
        if name == 'rank':
            expression += '.collect()'
        cases['unknown-description-' + name] = ('_ = try await ' + expression, r'UsageError', False)
        constructor = name.title() + 'Question'
        q = f'{constructor}("Question?", {key}: {levels}, descriptions: ["typo": "lost?"])'
        expression = f'client.{name}({q}, {argument})'
        if name == 'rank':
            expression += '.collect()'
        cases['saved-unknown-description-' + name] = ('_ = try await ' + expression, r'UsageError', False)
    for function, key in [('choose', 'choices'), ('tag', 'labels')]:
        prefix = 'enum UnicodeTopic: String, CaseIterable { case topic = "é" }; '
        constructor = function.title() + 'Question'
        value = f'Toolbox(["topic": {constructor}("Topic?", {key}: ["e\\u{{301}}"])]).{function}("topic", {key}: UnicodeTopic.self)'
        cases['unicode-domain-' + function] = (prefix + '_ = try ' + value, r'UsageError', False)
        positive = f'Toolbox(["topic": {constructor}("Topic?", {key}: ["é"])]).{function}("topic", {key}: UnicodeTopic.self)'
        # Validate the matching spelling before checking the byte-distinct refusal.
        cases['unicode-domain-' + function] = (prefix + '_ = try ' + positive + '; _ = try ' + value, r'UsageError', False)
    cases['unicode-toolbox-name'] = ('let box = Toolbox(["é": DecideQuestion("Ready?")]); _ = try box.decide("é"); _ = try box.decide("e\\u{301}")', r'UsageError', False)
    cases['unicode-form-name'] = ('let form = try await client.annotate(AnnotationQuestions(["é": DecideQuestion("Ready?")]), record: "body"); let entries = try form.answer; _ = try entries.decide("é"); _ = try entries.decide("e\\u{301}")', r'UsageError', True)
    cases.update({'array-' + name: ('_ = try await ' + call, r'UsageError.*records:', False) for name, call in ARRAY_CALLS.items()})
    cases['forwarded-array'] = ('func forward<T: Encodable & Sendable>(_ input: T) async throws { _ = try await client.decide("Ready?", record: input) }; try await forward(["body"])', r'UsageError.*records:', False)
    return cases



def stage_swift(consumer):
    source = guide_sources()
    runtime = runtime_cases()
    setup = {2: source[1].split('if isRefund.isAnswered')[0],
             10: MESSAGE_SETUP, 18: 'let questions =' + source[17].split('let questions =', 1)[1].split('let forms =')[0]}
    observations = {
        19: 'precondition(!isRefund.isFailed && !topic.isFailed)',
        21: 'precondition(isRefund.id == .string("a1")); precondition(isRefund.item["id"] == "a1")',
        23: 'precondition(isRefund.isAnswered)',
        22: 'precondition(topic.item["id"] == "a1")',
        25: '_ = try await stop.value',
    }
    program = 'import Foundation\nimport ThinkThen\n' + ENUMS
    program += 'do {\nswitch CommandLine.arguments[1] {\n'
    for index, literal in enumerate(source):
        client = '' if 'let client =' in literal else 'let client = try Client()\n'
        program += f'case "{index}": do {{\n' + client + setup.get(index, '') + literal + '\n' + observations.get(index, '') + '\n}\n'
    for name, (literal, _, _) in runtime.items():
        client = 'let client = try Client(); ' if 'client.' in literal else ''
        program += 'case "runtime-' + name + '": do { ' + client + literal + '\n}\n'
    program += '''case "host": try await checkIdealHostContracts(CommandLine.arguments[2])
default: fatalError("Unknown guide case")
}
} catch {
  let message = String(reflecting: type(of: error)) + ": " + String(describing: error) + "\\n"
  FileHandle.standardError.write(Data(message.utf8))
  exit(1)
}
'''
    target = consumer / 'LiteralGuide'
    target.mkdir()
    (target / 'main.swift').write_text(program)
    (target / 'IdealCases.swift').write_text((ROOT / 'sdk/swift/tests/IdealCases.swift').read_text())
    refusal = consumer / 'Refusal'
    refusal.mkdir()
    (refusal / 'main.swift').write_text('import ThinkThen\nfunc valid(_ client: Client) async throws { _ = try await client.decide("Ready?", record: "body").answer }\n')


def run_swift(consumer, negative=None):
    consumer = Path(consumer).resolve()
    source = guide_sources()
    runtime = runtime_cases()
    command = ['swift', 'build', '--package-path', str(consumer), '--disable-sandbox', '--jobs', os.environ.get('CARGO_BUILD_JOBS', '2'), '-Xswiftc', '-warnings-as-errors']
    binary = consumer / '.build/debug/LiteralGuide'
    errors = []

    def execute(index, name, output='', refusal=None, branch=None):
        if negative and (index != 0 or name != 'yes'): return
        scripted = shared_cases().get(name) if name in ('empty', 'other') else None
        with scenario(name if scripted is None else 'yes', case=scripted) as (work, env):
            (work / 'toolbox.json').write_text('{"entries":{"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"field":"/text"}}}}')
            result = subprocess.run([str(binary), str(index)], cwd=work, env=env, capture_output=True, text=True, timeout=60)
            try:
                if index == 3 and name == 'failed':
                    pattern = (r'could not judge it: Failed\(code: backend\.status, message: ".+503.+", '
                               r'httpStatus: Optional\(503\), rows: \[failed: backend\.status: .+503.+\]\)\n')
                    if not re.fullmatch(pattern, result.stdout):
                        raise AssertionError('swift/3/failed: call failure output or payload missing: ' + repr(result.stdout))
                check(f'swift/{index}/{name}', result, output=output, branch=branch,
                      exit_code=1 if refusal else 0, refusal=refusal,
                      negative=negative if index == 0 and name == 'yes' else None)
            except AssertionError as error:
                if negative: raise
                errors.append(str(error))
                print('FAIL ' + str(error), flush=True)

    def cases(index, outcomes):
        for name, expected in outcomes.items():
            refusal = expected[1] if isinstance(expected, tuple) else None
            output = expected[0] if isinstance(expected, tuple) else expected
            execute(index, name, output, refusal, output.splitlines()[0] if output else None)

    noanswer = ('', r'Failed')
    states = dict(yes='refund queue\n', no='inbox\n', unsure='a person reads it\n', failed=noanswer)
    cases(0, dict(yes='refund queue\n', no='inbox\n', failed=noanswer))
    cases(1, states)
    cases(2, states)
    cases(3, dict(yes='true\n', no='false\n', failed=None))
    cases(4, dict(yes='0.95\n', no='0.05\n', unsure='0.5\n', failed=noanswer))
    cases(5, dict(yes='billing team\n', no='warehouse\n', other='front desk\n', unsure='a person reads it\n', failed=noanswer))
    for name, prefix in [('yes', 'refund queue: '), ('no', 'inbox: '), ('unsure', 'a person reads it: '), ('failed', 'try again later: ')]:
        cases(6, {name: ''.join(prefix + text + '\n' for text in MESSAGES)})
    for name, prefix in [('yes', 'refund queue '), ('no', 'inbox '), ('unsure', 'a person reads it '), ('failed', 'try again later ')]:
        cases(7, {name: ''.join(prefix + text + '\n' for text in MESSAGES)})
    cases(8, dict(yes='true\n', no='false\n', unsure='unsure\n', failed=noanswer))
    cases(9, dict(yes=''.join('refund queue: ' + text + '\n' for text in MESSAGES), no='', failed=noanswer))
    cases(10, dict(yes=''.join('refund queue: ' + text + '\n' for text in MESSAGES), no='', failed=''.join('try again later: ' + text + ' (' + FAILURE + ')\n' for text in MESSAGES)))
    cases(11, dict(yes='billing team\nbilling\nshipping\n', no='', failed=noanswer))
    cases(12, dict(yes='answer today\n1.00\n', no='answer today\n1.00\n', **{'low-score':'0.00\n'}, failed=noanswer))
    cases(13, dict(yes=''.join(str(i) + ' ' + text + '\n' for i, text in enumerate(MESSAGES, 1)), failed=noanswer))
    cases(14, dict(yes='Please refund my order.\n', no='no line answers\n', unsure='a person reads it\n', failed=noanswer))
    cases(15, dict(relations='person Morgan\norganization Example Group\n', empty='', failed=noanswer))
    cases(16, dict(relations='Morgan works_for Example Group\n', empty='', failed=noanswer))
    for name in ('refund-failed', 'topic-failed', 'urgency-failed', 'refund-unsure', 'topic-unsure', 'failed', 'yes', 'no'):
        if name.endswith('-failed') or name == 'failed': output = ''.join('try again later: ' + text + '\n' for text in MESSAGES)
        elif name in ('topic-unsure', 'refund-unsure'): output = ''.join('a person reads it: ' + text + '\n' for text in MESSAGES)
        elif name == 'yes': output = 'refund queue: billing 1.00\n' * 3
        else: output = ''
        cases(17, {name: output})
        cases(18, {name: noanswer if name.endswith('-failed') or name == 'failed' else ''})
    cases(19, dict(yes='', no='', failed=noanswer))
    cases(20, dict(yes='refund queue\ntrue Please refund my order.\ntrue Where is my parcel?\n', no='inbox\nfalse Please refund my order.\nfalse Where is my parcel?\n', failed=noanswer))
    cases(21, dict(yes='', unsure='', failed=noanswer))
    cases(22, dict(yes='', unsure='', failed=noanswer))
    cases(23, dict(yes='', failed=noanswer))
    cases(24, dict(yes='1 0 0\n', failed=noanswer))
    cases(25, dict(yes='', failed=noanswer))
    # Compiler refusals share the already-built library. Only this small target changes.
    refusal = consumer / 'Refusal'
    invalid = {
        'result-in-if': ('let value = try await client.decide("Ready?", record: "body"); if value {}', r'cannot convert|cannot be used as a boolean'),
        'result-versus-bool': ('let value = try await client.decide("Ready?", record: "body"); _ = value == true', r'cannot be applied|conflicting arguments|cannot convert'),
        'result-versus-enum': ('let value = try await client.choose("Topic?", record: "body", choices: Topic.self); _ = value == Topic.billing', r'cannot be applied|conflicting arguments|cannot convert'),
        'filter-band': ('_ = client.filter("Ready?", records: ["body"], threshold: .band(low: 0.2, high: 0.8))', r'has no member.*band|cannot infer contextual base'),
        'missing-await': ('_ = try client.decide("Ready?", record: "body")', r'not marked with.*await|async.*await'),
        'retired-toolbox-decoder': ('func decode<T: Decodable>(_ type: T.Type) {}; decode(Toolbox.self)', r'conform.*Decodable|Decodable.*conform'),
        'retired-annotations-decoder': ('func decode<T: Decodable>(_ type: T.Type) {}; decode(AnnotationQuestions.self)', r'conform.*Decodable|Decodable.*conform'),
        'retired-annotate-decoder': ('func decode<T: Decodable>(_ type: T.Type) {}; decode(AnnotateQuestion.self)', r'conform.*Decodable|Decodable.*conform'),
        'retired-error-name': ('let _: RecordError', r'cannot find type.*RecordError'),
        'retired-get': ('_ = try await client.decide("Ready?", record: "body").get()', r'has no member.*get'),
        'retired-result': ('_ = try await client.decide("Ready?", record: "body").result()', r'has no member.*result'),
        'retired-filterResults': ('_ = client.filterResults("Ready?", records: ["body"])', r'has no member.*filterResults'),
        'retired-failures-row': ('_ = client.filter("Ready?", records: ["body"], failures: .row)', r'extra argument.*failures|cannot infer contextual base'),
        'retired-wrapper': ('let _: FilterResult', r'cannot find type.*FilterResult'),
        'retired-options': ('_ = try await client.decide("Ready?", record: "body", options: .init())', r'extra argument.*options|cannot infer contextual base'),
        'retired-toolbox-optional': ('_ = Toolbox(["refund": DecideQuestion("Ready?")])["refund"]', r'has no subscripts|cannot subscript'),
    }
    exhaustive = {
        'exhaustive-status': source[7].replace('    case .unsure:\n        return "a person reads it"\n', ''),
        'exhaustive-enum': source[5].split('let topic =')[0].replace('    case .other: "front desk"\n', ''),
    }
    for name, literal in exhaustive.items(): invalid[name] = (literal, r'switch must be exhaustive')
    for name, (literal, diagnostic) in invalid.items():
        if negative == 'refusal' and name == 'result-in-if':
            literal = 'let value = try await client.decide("Ready?", record: "body"); if try value.answer {}'
        text = 'import ThinkThen\n' + ENUMS + '\nfunc rejected(_ client: Client) async throws {\n' + literal + '\n}\n'
        # The enum-exhaustiveness fence already declares its own Topic.
        if name == 'exhaustive-enum': text = 'import ThinkThen\n' + literal
        (refusal / 'main.swift').write_text(text)
        result = subprocess.run([*command, '--target', 'Refusal'], capture_output=True, text=True, timeout=60)
        try:
            check('swift/refusal/' + name, result, exit_code=1, refusal=diagnostic)
        except AssertionError as error:
            if negative: raise
            errors.append(str(error)); print('FAIL ' + str(error), flush=True)

    for name, (_, diagnostic, paid) in runtime.items():
        observers = []
        with scenario('yes', observer=observers) as (work, env):
            result = subprocess.run([str(binary), 'runtime-' + name], cwd=work, env=env, capture_output=True, text=True, timeout=60)
            server = observers[0]; server.stdin.write('count\n'); server.stdin.flush()
            count = int(server.stdout.readline())
            try:
                check('swift/refusal/' + name, result, output='', exit_code=1, refusal=diagnostic)
                if (paid and count == 0) or (not paid and count != 0):
                    raise AssertionError(name + ': unexpected backend request count ' + str(count))
            except AssertionError as error:
                if negative: raise
                errors.append(str(error)); print('FAIL ' + str(error), flush=True)
    for name in ('yes', 'no', 'unsure', 'failed', 'refund-failed', 'topic-failed', 'urgency-failed', 'refund-unsure', 'topic-unsure', 'relations'):
        with scenario(name) as (work, env):
            result = subprocess.run([str(binary), 'host', name], cwd=work, env=env, capture_output=True, text=True, timeout=90)
            try:
                check('swift/host/' + name, result, output='Swift ideal host contracts passed: ' + name + '\n')
            except AssertionError as error:
                if negative: raise
                errors.append(str(error)); print('FAIL ' + str(error), flush=True)
    if errors:
        raise AssertionError('Swift ideal guide failures: ' + str(len(errors)) + '\n' + '\n'.join(errors))
    print('Swift ideal guide: every fence and scenario output passed')


if __name__ == '__main__':
    if sys.argv[1] == 'stage': stage_swift(Path(sys.argv[2]).resolve())
    elif sys.argv[1] == 'run': run_swift(Path(sys.argv[2]).resolve(), sys.argv[3] if len(sys.argv) > 3 else None)
    else: raise SystemExit('usage: ideal.py stage|run CONSUMER [NEGATIVE]')
