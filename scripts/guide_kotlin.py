"""Execute every literal Kotlin ideal-guide example with branch and refusal assertions."""
import os
from pathlib import Path
import subprocess
import tempfile

from bash_guide_examples import cases as shared_cases
from guide_examples import ROOT, blocks, check, scenario
from kotlin_compiler import classpath as compiler_classpath

IMPORTS = 'import dev.thinkthen.kotlin.*\nimport kotlinx.coroutines.*\nimport kotlinx.coroutines.flow.*\nimport java.nio.file.Path\n'
MESSAGES = 'val messages = listOf("Please refund my order.", "Where is my parcel?", "I was charged twice for one invoice.")\n'
FAILURE = 'failed: backend.status: the backend answered status 503: {"error":"unavailable"}'


def run_kotlin(classpath, negative=None, *, host_only=False):
    classpath = os.pathsep.join(str(Path(part).resolve()) for part in classpath.split(os.pathsep))
    repository = os.environ.get('THINKTHEN_KOTLIN_REPOSITORY')
    if not repository:
        raise AssertionError('Kotlin runner requires THINKTHEN_KOTLIN_REPOSITORY from the prepared JVM gate')
    java = str(Path(os.environ['JAVA_HOME']) / 'bin/java')
    compiler = compiler_classpath(repository)
    examples = blocks(ROOT / 'docs/surfaces/kotlin.md')
    if len(examples) != 31 or any(language != 'kotlin' for language, _, _ in examples):
        raise AssertionError('Kotlin guide fences changed: classify every new or changed example')
    source = [body for _, body, _ in examples]
    programs = {}
    states = {'yes': 'refund queue\n', 'no': 'inbox\n', 'unsure': 'a person reads it\n', 'failed': ('', r'FailedException')}
    failure = ('', r'FailedException')
    prefix = source[1].split('if (isRefund.isAnswered)')[0]
    enums = {6: source[5], 15: source[14], 17: source[16], 18: source[16],
             22: source[5] + '\n' + source[16], 23: source[5] + '\n' + source[16], 24: source[5]}
    for index, literal in enumerate(source):
        if index in (5, 10, 14, 16):
            continue  # These exact declarations run in their consuming fences.
        declarations = enums.get(index, '')
        setup = prefix if index == 2 else ''
        if index in (8, 9, 11, 30):
            setup = MESSAGES
        if index == 11:
            declarations += source[10]
        if index == 13:
            setup = source[12].split('refundRequests.collect')[0]
        if index == 23:
            setup = source[22].split('val forms =')[0]
        if 'suspend fun main()' in literal:
            program = literal
        else:
            client = 'ThinkThen().use { client ->\n' if not literal.startswith('ThinkThen') and ('client.' in literal or 'client.' in setup) else ''
            observations = ''
            state = 'System.getenv("THINKTHEN_GUIDE_CASE")'
            if index == 23:
                observations = 'check(form.isAnswered && form.throwIfFailed() === form)'
            elif index == 24:
                observations = 'check(!isRefund.isFailed && !topic.isFailed)'
            elif index in (26, 27):
                value = 'isRefund' if index == 26 else 'topic'
                observations = 'check(!' + value + '.isFailed); check(' + value + '.isUnsure == (' + state + ' == "unsure")); check(' + value + '.id.toString().contains("a1"))'
            # Preserve the literal launch/cancel branch on runBlocking's event loop.
            main = 'fun main() = runBlocking {\n' if index == 30 else 'suspend fun main() {\n'
            program = IMPORTS + '\n' + declarations + '\n' + main + client + setup + literal + '\n' + observations + '\n' + ('}\n' if client else '') + '}\n'
        programs[index] = 'package example' + str(index) + '\n' + program

    cases = []
    def add(index, outcomes):
        for name, expected in outcomes.items():
            output, refusal = expected if isinstance(expected, tuple) else (expected, None)
            cases.append((index, name, output, refusal))
    add(0, {'yes': 'refund queue\n', 'no': 'inbox\n', 'failed': failure})
    add(1, states)
    add(2, states)
    add(3, {'yes': 'true\n', 'no': 'false\n', 'unsure': 'true\n', 'failed': 'could not judge it: ' + FAILURE.removeprefix('failed: backend.status: ') + '\n'})
    add(4, {'yes': '0.95\n', 'no': '0.05\n', 'unsure': '0.5\n', 'failed': failure})
    add(6, {'yes': 'billing team\n', 'no': 'warehouse\n', 'other': 'front desk\n', 'unsure': 'a person reads it\n', 'failed': failure})
    messages = ('Please refund my order.', 'Where is my parcel?', 'I was charged twice for one invoice.')
    for name, text in [('yes', 'refund queue: '), ('no', 'inbox: '), ('unsure', 'a person reads it: '), ('failed', 'try again later: ')]:
        add(7, {name: ''.join(text + message + '\n' for message in messages)})
    add(8, {'yes': 'true\n'*3, 'no': 'false\n'*3, 'unsure': 'unsure\n'*3, 'failed': failure})
    add(9, {'yes': 'refund queue\n'*3, 'no': 'inbox\n'*3, 'unsure': 'a person reads it 0.5\n'*3, 'failed': 'try again later backend.status\n'*3})
    for name, text in [('yes', 'refund queue: '), ('no', 'inbox: '), ('unsure', 'a person reads it: '), ('failed', 'try again later: ')]:
        add(11, {name: ''.join(text + message + '\n' for message in messages)})
    add(12, {'yes': ''.join('refund queue: ' + message + '\n' for message in messages), 'no': '', 'failed': failure})
    add(13, {'yes': ''.join('refund queue: ' + message + '\n' for message in messages), 'no': '', 'failed': ''.join('try again later: ' + message + ' backend.status\n' for message in messages)})
    add(15, {'yes': 'billing team\nbilling\nshipping\n', 'no': '', 'failed': failure})
    add(17, {'yes': 'answer today\n1.00\n', 'no': 'answer today\n1.00\n', 'low-score': '0.00\n', 'failed': failure})
    add(18, {'yes': ''.join(str(index + 1) + ' ' + message + '\n' for index, message in enumerate(messages)), 'failed': failure})
    add(19, {'yes': 'Please refund my order.\n', 'no': 'no line answers\n', 'unsure': 'a person reads it\n', 'failed': failure})
    add(20, {'relations': 'person Morgan\norganization Example Group\n', 'empty': '', 'failed': failure})
    add(21, {'relations': 'Morgan works_for Example Group\n', 'empty': '', 'failed': failure})
    # The literal refund question has one cut; topic alone can be unsure here.
    for name in ('refund-failed', 'topic-failed', 'urgency-failed', 'refund-unsure', 'topic-unsure', 'failed', 'yes', 'no'):
        if name.endswith('-failed') or name == 'failed':
            output = ''.join('try again later: ' + message + '\n' for message in messages)
        elif name == 'topic-unsure':
            output = ''.join('a person reads it: ' + message + '\n' for message in messages)
        elif name in ('yes', 'refund-unsure'):
            output = 'refund queue: billing 1.00\n' * 3
        else:
            output = ''
        add(22, {name: output})
    add(23, {'yes': '', 'no': '', 'refund-unsure': '', 'topic-unsure': '', 'failed': failure,
             'refund-failed': failure, 'topic-failed': failure, 'urgency-failed': failure})
    add(24, {'yes': '', 'no': '', 'failed': failure})
    add(25, {'relations': 'refund queue\n2\n', 'failed': failure})
    add(26, {'yes': '', 'unsure': '', 'failed': failure})
    add(27, {'yes': '', 'unsure': '', 'failed': failure})
    add(28, {'yes': '', 'failed': failure})
    add(29, {'yes': '1 0 0\n', 'failed': failure})
    add(30, {'yes': ''})
    errors = []
    def verify(*args, **kwargs):
        try:
            check(*args, **kwargs)
        except AssertionError as error:
            if negative:
                raise
            errors.append(str(error))
            print('FAIL ' + str(error), flush=True)

    with tempfile.TemporaryDirectory(prefix='kotlin-ideal-') as temporary:
        scratch = Path(temporary)
        sources = scratch / 'sources'
        sources.mkdir()
        (sources / 'KotlinIdealCases.kt').write_text((ROOT / 'sdk/kotlin/tests/KotlinIdealCases.kt').read_text())
        for index, program in programs.items():
            (sources / ('Example' + str(index) + '.kt')).write_text(program)
        built = subprocess.run([java, '-cp', compiler, 'org.jetbrains.kotlin.cli.jvm.K2JVMCompiler', '-no-stdlib', '-no-reflect', '-jvm-target', '22', '-classpath', classpath, '-d', str(scratch / 'classes'), str(sources)], capture_output=True, text=True, timeout=120)
        check('kotlin/literal/compile', built)
        for index, name, output, refusal in ([] if negative == 'refusal' else [(index, name, output, refusal) for index, name, output, refusal in cases if index == 30] if host_only else cases):
            observers = []
            scripted = shared_cases().get(name) if name in ('empty', 'other') else None
            with scenario('yes' if scripted else name, case=scripted, observer=observers) as (folder, env):
                (folder / 'toolbox.json').write_text('{"entries":{"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"field":"/text"}}}}')
                env['THINKTHEN_GUIDE_CASE'] = name
                ran = subprocess.run([java, '-Duser.home=' + str(folder / 'home'), '--enable-native-access=ALL-UNNAMED', '-cp', str(scratch / 'classes') + os.pathsep + classpath, 'example' + str(index) + '.Example' + str(index) + 'Kt'], cwd=folder, env=env, capture_output=True, text=True, timeout=90)
                server = observers[0]
                server.stdin.write('count\n')
                server.stdin.flush()
                requests = int(server.stdout.readline())
                label = 'kotlin/' + str(index) + '/' + name
                if negative and index == 0 and name == 'yes':
                    if ran.returncode != 0 or ran.stdout != 'refund queue\n' or requests != 1:
                        raise AssertionError('Kotlin dispatch witness did not run the refund branch with one fixture request')
                    print('KOTLIN DISPATCH kotlin/0/yes requests=1 branch=refund queue', flush=True)
                verify(label, ran, output=output, branch=output.splitlines()[0] if output else None, exit_code=1 if refusal else 0, refusal=refusal, negative=negative if index == 0 and name == 'yes' else None)
                if (index == 30 and requests != 0) or (index != 30 and requests == 0):
                    errors.append(label + ': wrong backend request count ' + str(requests))
        for name in (() if negative == 'refusal' else ('yes', 'no', 'unsure', 'failed', 'refund-failed', 'topic-failed', 'urgency-failed', 'refund-unsure', 'topic-unsure', 'relations')):
            with scenario(name) as (folder, env):
                ran = subprocess.run([java, '-Duser.home=' + str(folder / 'home'), '--enable-native-access=ALL-UNNAMED', '-cp', str(scratch / 'classes') + os.pathsep + classpath, 'KotlinIdealCasesKt', name], cwd=folder, env=env, capture_output=True, text=True, timeout=120)
                verify('kotlin/host/' + name, ran, output='Kotlin ideal host contracts passed: ' + name + '\n')
        refusal_checks(java, compiler, classpath, scratch, source, negative, verify)
    if errors:
        raise AssertionError('Kotlin guide failures: ' + str(len(errors)) + '\n' + '\n'.join(errors))
    print('Kotlin ideal host/refusal continuation passed' if host_only else 'Kotlin ideal guide: all 31 fences, scenario outputs, branches and refusals passed')


def refusal_checks(java, compiler, classpath, scratch, source, negative, verify):
    """Each compiler refusal is paired with the valid literal compilation above."""
    rejected = {
        'retired-RecordException': ('val removed: RecordException? = null', r'Unresolved reference'),
        'result-if': ('val result = client.decide("Ready?", "text"); if (result) println("bad")', r'condition type mismatch|Boolean'),
        'result-enum': ('val result = client.choose("Which?", "text", Topic::class); println(result == Topic.BILLING)', r'cannot be applied|incompatible'),
        'result-string': ('val result = client.choose("Which?", "text", Topic::class); println(result == "billing")', r'cannot be applied|incompatible'),
        'result-boolean': ('val result = client.decide("Ready?", "text"); println(result == true)', r'cannot be applied|incompatible'),
        'error-nonfailed': ('println(client.decide("Ready?", "text").error)', r'Unresolved reference'),
        'filter-band': ('client.filter("Ready?", listOf("text"), threshold = Threshold.band(0.2, 0.8))', r'argument type mismatch|inapplicable|none of the following candidates is applicable'),
        'filter-entry': ('AnnotationQuestions(mapOf("x" to FilterQuestion("Ready?")))', r'argument type mismatch|inapplicable|none of the following candidates is applicable'),
        'rank-entry': ('AnnotationQuestions(mapOf("x" to RankQuestion("Ready?", listOf("low", "high"))))', r'argument type mismatch|inapplicable|none of the following candidates is applicable'),
        'answer-property': ('val result = client.decide("Ready?", "text"); if(result is Row.Answered) println(result.answer)', r'Function invocation'),
        'quiet-read': ('client.decide("Ready?", "text").getOrThrow()', r'Unresolved reference'),
        'filterRows': ('client.filterRows("Ready?", listOf("text"))', r'Unresolved reference'),
        'ask': ('client.ask(DecideQuestion("Ready?"), "text")', r'Unresolved reference'),
        'toolbox-quiet': ('Toolbox(mapOf("x" to DecideQuestion("Ready?"))).chooseOrNull("x")', r'Unresolved reference'),
        'entry-quiet': ('client.annotate(AnnotationQuestions(mapOf("x" to DecideQuestion("Ready?"))), "text").answer().chooseOrNull("x")', r'Unresolved reference'),
    }
    shapes = {'list': 'listOf("text")', 'array': 'arrayOf("text")', 'sequence': 'sequenceOf("text")', 'flow': 'flowOf("text")'}
    for name, shape in shapes.items():
        rejected['collection-' + name] = ('client.decide("Ready?", ' + shape + ')', r'decideStream')
        rejected['typed-' + name] = ('client.decide(DecideQuestion("Ready?"), ' + shape + ')', r'decideStream')
        rejected['enum-' + name] = ('client.choose("Which?", ' + shape + ', Topic::class)', r'chooseStream')
    declarations = source[5]
    rejected['exhaustive-result'] = ('println(whereItGoes(client.decide("Ready?", "text")))', r'exhaustive')
    rejected['exhaustive-enum'] = ('println(teamFor(Topic.BILLING))', r'exhaustive')
    import re
    directory = scratch / 'refusals'
    directory.mkdir()
    for name, (literal, _) in rejected.items():
        extra = declarations
        if name == 'exhaustive-result':
            extra += '\n' + source[10].replace('    is Row.Unsure -> "a person reads it"\n', '')
        elif name == 'exhaustive-enum':
            extra = extra.replace('    Topic.OTHER -> "front desk"\n', '')
        if negative == 'refusal' and name == 'result-if':
            literal = 'if (client.decide("Ready?", "text").answer()) println("good")'
        (directory / (name.replace('-', '_') + '.kt')).write_text('package reject.' + name.replace('-', '_') + '\n' + IMPORTS + extra + '\nsuspend fun test(client: ThinkThen) { ' + literal + ' }\n')
    built = subprocess.run([java, '-cp', compiler, 'org.jetbrains.kotlin.cli.jvm.K2JVMCompiler', '-no-stdlib', '-no-reflect', '-jvm-target', '22', '-classpath', classpath, '-d', str(scratch / 'rejected'), str(directory)], capture_output=True, text=True, timeout=120)
    diagnostics = built.stdout + built.stderr
    if built.returncode == 0:
        raise AssertionError('Kotlin promised compiler refusals all compiled')
    for name, (_, diagnostic) in rejected.items():
        filename = name.replace('-', '_') + '.kt'
        own = '\n'.join(line for line in diagnostics.splitlines() if filename in line)
        if not own or not re.search(diagnostic, own, re.I):
            raise AssertionError('kotlin/refusal/' + name + ': missing named compiler diagnostic ' + diagnostic + '\n' + diagnostics)
        print('PASS kotlin/refusal/' + name, flush=True)


    runtime = {
        'scalar-blank': ('client.decide("Ready?", "")', r'FailedException'),
        'scalar-parent-blank': ('client.annotate(AnnotationQuestions(mapOf("refund" to DecideQuestion("Ready?"))), "")', r'FailedException'),
        'missing-key': ('ThinkThen { backend = Backend.OPENAI; apiKey = null }.close()', r'SetupException.*key'),
        'collection-list': ('val record: Any = listOf("text"); client.decide("Ready?", record)', r'IllegalArgumentException.*decideStream'),
        'collection-array': ('val record: Any = arrayOf("text"); client.decide("Ready?", record)', r'IllegalArgumentException.*decideStream'),
        'collection-sequence': ('val record: Any = sequenceOf("text"); client.decide("Ready?", record)', r'IllegalArgumentException.*decideStream'),
        'collection-flow': ('val record: Any = flowOf("text"); client.decide("Ready?", record)', r'IllegalArgumentException.*decideStream'),
        'missing-toolbox': ('Toolbox(mapOf("refund" to DecideQuestion("Ready?"))).choose("absent")', r'UsageException.*No choose question named absent'),
        'wrong-toolbox': ('Toolbox(mapOf("refund" to DecideQuestion("Ready?"))).choose("refund")', r'UsageException.*No choose question named refund'),
        'missing-entry': ('client.annotate(AnnotationQuestions(mapOf("refund" to DecideQuestion("Ready?"))), "text").answer().choose("absent")', r'UsageException.*No choose annotation named absent'),
        'wrong-entry': ('client.annotate(AnnotationQuestions(mapOf("refund" to DecideQuestion("Ready?"))), "text").answer().choose("refund")', r'UsageException.*No choose annotation named refund'),
        'saved-band': ('Toolbox.fromDefinitions("""{"filter":{"function":"filter","question":"Ready?","threshold":{"low":0.2,"high":0.8}}}""")', r'UsageException.*decideStream'),
    }
    runtime_sources = scratch / 'runtime'
    runtime_sources.mkdir()
    for name, (literal, _) in runtime.items():
        body = literal if name == 'missing-key' else 'ThinkThen().use { client -> ' + literal + ' }'
        (runtime_sources / (name.replace('-', '_') + '.kt')).write_text('package runtime.' + name.replace('-', '_') + '\n' + IMPORTS + '\nsuspend fun main() { ' + body + ' }\n')
    built = subprocess.run([java, '-cp', compiler, 'org.jetbrains.kotlin.cli.jvm.K2JVMCompiler', '-no-stdlib', '-no-reflect', '-jvm-target', '22', '-classpath', classpath, '-d', str(scratch / 'runtime-classes'), str(runtime_sources)], capture_output=True, text=True, timeout=120)
    check('kotlin/runtime-refusals/compile', built)
    for name, (_, diagnostic) in runtime.items():
        observers = []
        with scenario('yes', observer=observers) as (folder, env):
            if name == 'missing-key':
                for key in ('THINKTHEN_API_KEY', 'OPENAI_API_KEY', 'SYSTEMONE_API_KEY'):
                    env.pop(key, None)
            classname = 'runtime.' + name.replace('-', '_') + '.' + name.replace('-', '_')[0].upper() + name.replace('-', '_')[1:] + 'Kt'
            ran = subprocess.run([java, '-Duser.home=' + str(folder / 'home'), '--enable-native-access=ALL-UNNAMED', '-cp', str(scratch / 'runtime-classes') + os.pathsep + classpath, classname], cwd=folder, env=env, capture_output=True, text=True, timeout=90)
            verify('kotlin/runtime-refusal/' + name, ran, exit_code=1, refusal=diagnostic)
            server = observers[0]
            server.stdin.write('count\n')
            server.stdin.flush()
            requests = int(server.stdout.readline())
            if requests != (1 if name.endswith('-entry') else 0):
                raise AssertionError('kotlin/runtime-refusal/' + name + ': wrong backend request count ' + str(requests))
