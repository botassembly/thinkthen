"""Literal Zig ideal-guide examples and their observable scenario assertions."""
from pathlib import Path
import os
import subprocess
import sys
from guide_examples import blocks, IDEAL, check, scenario

PRELUDE = '''const std = @import("std");
const tt = @import("thinkthen");
const Topic = enum { billing, shipping, other };
const Urgency = enum { routine, urgent };
const Ticket = struct { id: []const u8, text: []const u8 };
'''
SETUP = '''const arena = init.arena.allocator();
var stdout = std.Io.File.stdout().writerStreaming(init.io, &.{});
var diag: tt.Diagnostic = .{};
errdefer std.debug.print("{f}\\n", .{diag});
var client = try tt.Client.init(.{}, &diag);
defer client.deinit();
'''
ASK = 'const asks_refund: tt.Decide = .{ .question = "Does the customer request a refund?" };\n'


def sources():
    fences = {i: body for i, (lang, body, _) in enumerate(blocks(IDEAL / 'zig.md')) if lang == 'zig'}
    if set(fences) != set(range(28)):
        raise AssertionError('Zig guide fences changed: classify every new or changed example')
    result = {}
    for i, body in fences.items():
        if i == 0:
            continue  # build.zig is exercised separately by the installed-package gate.
        if 'pub fn main(' in body:
            result[i] = body
            continue
        prefix = PRELUDE
        setup = SETUP
        if i == 3:
            setup += fences[2][:fences[2].index('\nif (is_refund.status')]
        if i in (25, 26, 27):
            setup += ASK
        if i in (8, 9, 10, 11):
            setup += fences[7].split('\nvar refund_checks', 1)[0] + '\n'
        if i == 13:
            setup += fences[12].split('\nvar call', 1)[0] + '\n'
        if i == 10:
            prefix += body + '\n'
            body = fences[11]
        if i == 11:
            prefix += fences[10] + '\n'
        if i == 21:
            body += '\ntry stdout.interface.print("form checked\\n", .{});'
            setup += 'const asks_refund:' + fences[20].split('const asks_refund:', 1)[1].split('\nvar forms', 1)[0] + '\n'
        if i in (25, 27):
            setup = setup.replace('var client = try tt.Client.init(.{}, &diag);\ndefer client.deinit();\n', '')
        for name in ('Topic', 'Urgency', 'Ticket'):
            if f'const {name} =' in body:
                prefix = '\n'.join(line for line in prefix.splitlines() if not line.startswith(f'const {name} =')) + '\n'
        # Zig rejects unused locals. The wrapper observes shared setup values; the literal stays intact.
        suffix = '\n_ = &arena; _ = &stdout; _ = &client; _ = &diag;\n'
        result[i] = prefix + 'pub fn main(init: std.process.Init) !void {\n' + setup + body + suffix + '\n}\n'
    return result


def compile_examples(stage, package=None):
    stage = Path(stage)
    stage.mkdir(parents=True, exist_ok=True)
    root = Path(__file__).resolve().parent.parent
    sdk = Path(package).resolve() if package else root / 'sdk/zig'
    header = sdk / 'include/thinkthen.h' if package else root / 'crates/thinkthen-ffi/include/thinkthen.h'
    subprocess.run(['zig', 'translate-c', '-lc', str(header)], check=True, stdout=(stage / 'c.zig').open('w'))
    from zig_codes import generate
    generate(root, stage / 'codes.zig')
    errors = {}
    def compile(path, output, *, tests=False):
        command = ['zig', 'test' if tests else 'build-exe', *(['--test-no-exec'] if tests else []), '-lc', '-fllvm', *(['-flld'] if sys.platform != 'darwin' else []), '--cache-dir', str(stage / 'cache'), '--dep', 'thinkthen', '-Mroot=' + str(path), '--dep', 'c', '--dep', 'codes', '-Mthinkthen=' + str(sdk / 'src/thinkthen.zig'), '-Mc=' + str(stage / 'c.zig'), '-Mcodes=' + str(stage / 'codes.zig'), '-femit-bin=' + str(output)]
        return subprocess.run(command, text=True, capture_output=True)
    for i, source in sources().items():
        path = stage / f'example-{i}.zig'
        path.write_text(source)
        ran = compile(path, stage / f'example-{i}')
        if ran.returncode:
            errors.setdefault(ran.stdout + ran.stderr, []).append(str(i))
        else:
            print(f'Zig literal {i} compiles', flush=True)
    for file, tests in [('consumer', False), ('public_api', True), ('minor_error', False), ('host_contract', False)]:
        ran = compile(root / 'sdk/zig/tests' / (file + '.zig'), stage / file, tests=tests)
        if ran.returncode:
            errors.setdefault(ran.stdout + ran.stderr, []).append(file)
        else:
            print(f'Zig existing consumer {file} compiles', flush=True)
    if errors:
        raise RuntimeError('\n'.join(f'Zig sources {indices}:\n{error}' for error, indices in errors.items()))
    (stage / 'revision').write_text(subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip())
    return stage

def installed_readme(stage, package):
    stage = Path(stage).resolve()
    root = Path(__file__).resolve().parent.parent
    application = stage / 'application'
    (application / 'src').mkdir(parents=True, exist_ok=True)
    readme = next(body for language, body, _ in blocks(root / 'sdk/zig/README.md') if language == 'zig' and 'pub fn main(' in body)
    build = next(body for language, body, _ in blocks(IDEAL / 'zig.md') if language == 'zig' and 'pub fn build(' in body)
    (application / 'src/main.zig').write_text(readme)
    (application / 'build.zig').write_text(build)
    template = (root / 'sdk/zig/tests/build.zig.zon').read_text()
    (application / 'build.zig.zon').write_text(template.replace('"../package"', '"' + os.path.relpath(package, application) + '"'))
    subprocess.run(['zig','build','-j'+os.environ.get('CARGO_BUILD_JOBS','2'),'--cache-dir',str(stage/'cache')],cwd=application,check=True)
    with scenario('yes') as (folder, env):
        ran = subprocess.run([str(application/'zig-out/bin/example')],cwd=folder,env=env,capture_output=True,text=True)
        check('zig/installed/guide-build-and-readme',ran,output='refund queue\n',branch='refund queue')

def compile_refusals(stage, package=None, negative=None):
    stage = Path(stage).resolve()
    root = Path(__file__).resolve().parent.parent
    sdk = Path(package).resolve() if package else root / 'sdk/zig'
    intro = PRELUDE + 'pub fn main(init: std.process.Init) !void {\n' + SETUP + ASK
    decide = 'const row = try client.decide(arena, asks_refund, "text", &diag);\n'
    choice = 'const topic = try client.choose(arena, tt.Choose(Topic){ .question = "Which?" }, "text", &diag);\n'
    form = 'const questions = tt.annotate(.{ .refund = asks_refund }); const form = try client.annotate(arena, questions, "text", &diag); const entries = try form.answer(&diag);\n'
    witnesses = {
        'result-in-if': (decide + 'if (row) {}', r"expected type 'bool'"),
        'pointer-in-if': (decide + 'if (&row) {}', r"expected type 'bool'"),
        'optional-pointer-in-if': (decide + 'const ptr: ?*@TypeOf(row) = @constCast(&row); if (ptr) {}', r"expected type 'bool'"),
        'choice-equals-answer': (choice + 'if (topic == .billing) {}', r'(incompatible types|operator == not allowed|enum literal)'),
        'incomplete-choice-switch': (choice + 'switch (try topic.answer(&diag)) { .billing => {}, .shipping => {} }', r'switch must handle all possibilities'),
        'incomplete-no-answer-switch': (decide + 'if (row.answer(&diag)) |_| {} else |err| switch (err) { error.Unsure => {} }', r'switch must handle all possibilities'),
        'filter-band': ('const question: tt.Filter = .{ .question = "Keep?", .threshold = .band(0.2,0.8) }; _ = question;', r"(no member named 'band'|has no member named 'band')"),
        'slice-in-single': ('const records = [_][]const u8{ "first", "second" }; _ = try client.decide(arena, asks_refund, records[0..], &diag);', r'one-record calls refuse.*All'),
        'array-in-single': ('const records = [_][]const u8{ "first", "second" }; _ = try client.decide(arena, asks_refund, records, &diag);', r'one-record calls refuse.*All'),
        'missing-form-name': (form + '_ = entries.nope;', r"no field named 'nope'"),
        'wrong-entry-answer-type': (form + 'const text: []const u8 = try entries.refund.answer(&diag); _ = text;', r'expected type.*found.*bool'),
        'filter-in-form': ('_ = tt.annotate(.{ .refund = tt.Filter{ .question = "Keep?" } });', r'annotate cannot hold filter, rank or annotate'),
        'rank-in-form': ('_ = tt.annotate(.{ .urgency = tt.Rank(Urgency){ .question = "Urgency?" } });', r'annotate cannot hold filter, rank or annotate'),
        'unknown-description': ('const question: tt.Choose(Topic) = .{ .question = "Which?", .descriptions = .{ .missing = "wrong" } }; _ = question;', r"no field named 'missing'"),
        'ignored-answer': (decide + 'row.answer(&diag);', r'error union.*is ignored'),
        'unsure-in-error-return': ('const R = struct { fn read(row: tt.Row(tt.Decide, []const u8)) tt.Error!void { _ = try row.answer(null); } }; ' + decide + 'try R.read(row);', r'(Unsure|not a member)'),
        'discarded-answer': (decide + '_ = row.answer(&diag);', r'error union is discarded'),
    }
    errors = []
    for name, (body, diagnostic) in witnesses.items():
        path = stage / ('refuse-' + name + '.zig')
        if negative == 'refusal' and name == 'result-in-if':
            body = decide + 'if (try row.answer(&diag)) {}'
        path.write_text(intro + body + '\n_ = &arena; _ = &stdout; _ = &client; _ = &diag; _ = &asks_refund;\n}\n')
        command = ['zig', 'build-exe', '-lc', '-fllvm', *(['-flld'] if sys.platform != 'darwin' else []), '--cache-dir', str(stage / 'cache'), '--dep', 'thinkthen', '-Mroot=' + str(path), '--dep', 'c', '--dep', 'codes', '-Mthinkthen=' + str(sdk / 'src/thinkthen.zig'), '-Mc=' + str(stage / 'c.zig'), '-Mcodes=' + str(stage / 'codes.zig'), '-fno-emit-bin']
        ran = subprocess.run(command, text=True, capture_output=True)
        try:
            check('zig/refusal/' + name, ran, exit_code=1, refusal=diagnostic)
        except AssertionError as error:
            if negative: raise
            errors.append(str(error) + '\n' + ran.stderr[:800])
    if errors:
        raise AssertionError('\n'.join(errors))

MESSAGES = ["Please refund my order.", "Where is my parcel?", "I was charged twice for one invoice."]
FAILURE = 'failed: backend.status: the backend answered status 503: {"error":"unavailable"}'


def run_examples(stage, negative=None):
    stage = Path(stage).resolve()
    matrix = {
        1: {'yes': 'refund queue\n', 'no': 'inbox\n', 'failed': None},
        2: {'yes': 'refund queue\n', 'no': 'inbox\n', 'unsure': 'a person reads it\n', 'failed': None},
        3: {'yes': 'refund queue\n', 'no': 'inbox\n', 'unsure': 'a person reads it\n', 'failed': None},
        4: {'yes': 'true\n', 'no': 'false\n', 'failed': 'could not judge it: ' + FAILURE.removeprefix('failed: ') + '\n'},
        5: {'yes': '0.95\n', 'no': '0.05\n', 'unsure': '0.5\n', 'failed': None},
        6: {'yes': 'billing team\n', 'no': 'warehouse\n', 'other': 'front desk\n', 'unsure': 'a person reads it\n', 'failed': None},
        7: {name: ''.join(prefix + message + '\n' for message in MESSAGES) for name, prefix in [('yes','refund queue: '),('no','inbox: '),('unsure','a person reads it: '),('failed','try again later: ')]},
        8: {'yes': 'true\n' * 3, 'no': 'false\n' * 3, 'unsure': 'unsure\n' * 3, 'failed': None},
        9: {'yes': 'refund queue\n' * 3, 'no': 'inbox\n' * 3, 'unsure': 'a person reads it\n' * 3, 'failed': 'try again later\n' * 3},
        **{i: {name: ''.join(prefix + message + '\n' for message in MESSAGES) for name, prefix in [('yes','refund queue: '),('no','inbox: '),('unsure','a person reads it: 0.5: '),('failed','try again later: backend.status: ')]} for i in (10,11)},
        12: {'yes': ''.join('refund queue: ' + message + '\n' for message in MESSAGES), 'no': '', 'failed': None},
        13: {'yes': ''.join('refund queue: ' + message + '\n' for message in MESSAGES), 'no': '', 'failed': ''.join('try again later: ' + message + ' (' + FAILURE + ')\n' for message in MESSAGES)},
        14: {'yes': 'billing team\nbilling\nshipping\n', 'no': '', 'failed': None},
        15: {'yes': 'answer today\n1.00\n', 'low-score': '0.00\n', 'failed': None},
        16: {'yes': ''.join(str(i) + ' ' + message + '\n' for i, message in enumerate(MESSAGES,1)), 'failed': None},
        17: {'yes': 'Please refund my order.\n', 'no': 'no line answers\n', 'unsure': 'a person reads it\n', 'failed': None},
        18: {'relations': 'person Morgan\norganization Example Group\n', 'unsure': '', 'failed': None},
        19: {'relations': 'Morgan works_for Example Group\n', 'unsure': '', 'failed': None},
        20: {'yes': 'refund queue: billing 1.00\n' * 3, 'no': '', **{name: ''.join('a person reads it: ' + message + '\n' for message in MESSAGES) for name in ('unsure','refund-unsure','topic-unsure')}, **{name: ''.join('try again later: ' + message + '\n' for message in MESSAGES) for name in ('failed','refund-failed','topic-failed','urgency-failed')}},
        21: {'yes': 'form checked\n', 'refund-unsure': 'form checked\n', 'topic-unsure': 'form checked\n', **{name: None for name in ('failed','refund-failed','topic-failed','urgency-failed')}},
        22: {'yes': 'true billing\n', 'failed': None},
        23: {'yes': 'true\n', 'unsure': 'unsure\n', 'failed': None},
        24: {'yes': 'billing\n', 'unsure': 'unsure\n', 'failed': None},
        25: {'yes': 'true\n', 'failed': None},
        26: {'yes': 'true\n1 0 0\n'},
        27: {'yes': 'true\n'},
    }
    errors = []
    special = {'other': {'replies': [{'distribution': {'billing': 0., 'shipping': 0., 'other': 1.}}]}}
    for i, cases in matrix.items():
        for name, output in cases.items():
            with scenario('yes' if name in special else name, case=special.get(name)) as (folder, env):
                (folder / 'toolbox.json').write_text('{"entries":{"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"field":"/text"}}}}')
                ran = subprocess.run([str(stage / f'example-{i}')], cwd=folder, env=env, capture_output=True, text=True)
                try:
                    check(f'zig/{i}/{name}', ran, output='' if output is None else output, branch=output.splitlines()[0] if output else None, exit_code=1 if output is None else 0, refusal=r'Failed' if output is None else None, negative=negative if i == 1 and name == 'yes' else None)
                except AssertionError as error:
                    if negative: raise
                    errors.append(str(error) + '\nfirst diagnostic: ' + ran.stderr[:300])
    if errors:
        raise AssertionError('\n'.join(errors))


def run_host_contract(stage):
    stage = Path(stage).resolve()
    cases = [('forms', name) for name in ('yes','low-score','no','unsure','failed','refund-failed','topic-failed','urgency-failed','refund-unsure','topic-unsure')]
    cases.append(('bounded-filter','no'))
    cases += [(name,'yes') for name in ('outer-form','unicode','order','print','refusals','scalars')]
    cases += [('scalar-outcomes',name) for name in ('yes','no','unsure','failed')]
    for mode, name in cases:
        observers = []
        with scenario(name, observer=observers) as (folder, env):
            ran = subprocess.run([str(stage / 'host_contract'),mode,name], cwd=folder,env=env,capture_output=True,text=True)
            expected = ('forms okay ' + name) if mode == 'forms' else {'outer-form':'outer failure okay','unicode':'unicode ownership okay','order':'UTF-8 form order okay','print':'printed leaves okay','refusals':'runtime refusals okay','bounded-filter':'bounded declined source okay','scalars':'scalar boundaries okay','scalar-outcomes':'scalar outcomes okay'}[mode]
            check('zig/host/' + mode + '/' + name,ran,output=expected+'\n',branch=expected)
            if mode in ('refusals','outer-form','order','scalars'):
                observers[0].stdin.write('count\n'); observers[0].stdin.flush()
                requests = int(observers[0].stdout.readline())
                if requests != 0: raise AssertionError(f'zig/host/{mode}: expected zero backend requests, got {requests}')


def dispatch(stage):
    root = Path(__file__).resolve().parent.parent
    arguments = [sys.executable,str(root/'scripts/guide-examples'),'--surface','zig','--zig-stage',str(stage)]
    subprocess.run(arguments,check=True)
    for control, reason in (('output','output:'),('branch','branch:'),('exit','exit:'),('refusal','promised refusal')):
        ran = subprocess.run([*arguments,'--negative-control',control],text=True,capture_output=True)
        if ran.returncode == 0 or 'FAIL zig/' not in ran.stderr or reason not in ran.stderr:
            raise AssertionError(f'Zig dispatcher {control} control was not rejected: {ran.stdout} {ran.stderr}')
        print('PASS zig/dispatcher/reject-' + control,flush=True)


def run_zig(stage, negative=None):
    """Run already-built literal programs through the shared guide dispatcher."""
    if not stage:
        raise RuntimeError('THINKTHEN_ZIG_GUIDE_STAGE or --zig-stage must name a prepared Zig guide stage')
    stage = Path(stage).resolve()
    root = Path(__file__).resolve().parent.parent
    revision = subprocess.check_output(['git','rev-parse','HEAD'],cwd=root,text=True).strip()
    if (stage / 'revision').read_text().strip() != revision:
        raise RuntimeError('the Zig guide stage belongs to another source revision')
    package = stage / 'archive/thinkthen'
    if negative == 'refusal':
        return compile_refusals(stage, package, negative)
    run_examples(stage,negative)
    if negative:
        raise AssertionError('the selected Zig negative checker control did not fail')
    run_host_contract(stage)
    compile_refusals(stage,package)


if __name__ == '__main__':
    if len(sys.argv) > 2 and sys.argv[2] == '--readme-only':
        installed_readme(sys.argv[1],sys.argv[3])
        raise SystemExit(0)
    compile_examples(sys.argv[1], sys.argv[3] if len(sys.argv) > 3 else None)
    if len(sys.argv) > 2 and sys.argv[2] in ('--run', '--consumer-only'):
        errors = []
        phases = (lambda: installed_readme(sys.argv[1],sys.argv[3]),)
        phases += ((lambda: run_host_contract(sys.argv[1])),) if sys.argv[2] == '--consumer-only' else ((lambda: dispatch(sys.argv[1])),)
        for phase in phases:
            try:
                phase()
            except (AssertionError,subprocess.CalledProcessError) as error:
                errors.append(str(error))
        public = subprocess.run([str(Path(sys.argv[1]) / 'public_api')], text=True, capture_output=True)
        if public.returncode:
            errors.append('existing Zig public consumers:\n' + public.stdout + public.stderr)
        else:
            print(public.stdout + public.stderr, flush=True)
        if errors:
            raise AssertionError('\n'.join(errors))
