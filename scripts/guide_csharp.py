"""Build the literal C# guide once and execute its scenarios from the installed package."""
from pathlib import Path
import re
import subprocess
import tempfile
from guide_examples import ROOT, blocks, check, scenario
from bash_guide_examples import cases as shared_cases
from csharp_consumers import project, config

FAILURE = 'failed: backend.status: the backend answered status 503: {"error":"unavailable"}'
MESSAGES = 'string[] messages = ["Please refund my order.", "Where is my parcel?", "I was charged twice for one invoice."];\n'
TEXTS = ('Please refund my order.', 'Where is my parcel?', 'I was charged twice for one invoice.')


def run_csharp(stage, negative=None, *, host_only=False):
    stage = Path(stage).resolve()
    examples = blocks(ROOT / 'docs/surfaces/csharp.md')
    if len(examples) != 31 or [language for language, _, _ in examples].count('xml') != 1:
        raise AssertionError('C# guide fences changed: classify every fence')
    source = [body for _, body, _ in examples]
    site = blocks(ROOT / 'site/src/content/docs/sdks/csharp.mdx')
    if [(language, body) for language, body, _ in site] != [(language, body) for language, body, _ in examples]:
        raise AssertionError('C# site guide fences differ from the public guide')
    if blocks(ROOT / 'sdk/csharp/README.md')[0][1] != source[0]:
        raise AssertionError('C# packaged README quickstart differs from the tested guide')
    if source[11].strip() != '<WarningsAsErrors>CS8509</WarningsAsErrors>\n<NoWarn>CS8524</NoWarn>':
        raise AssertionError('C# exhaustive-switch project settings changed')
    folder = Path(tempfile.mkdtemp(prefix='csharp-guide-')) / 'consumer'
    reference = project(folder, 'Guide')
    configuration = config(stage, folder)
    text = reference.read_text()
    text = text.replace('</PropertyGroup>', source[11] + '<LangVersion>14</LangVersion></PropertyGroup>')
    reference.write_text(text)
    methods = {}
    for index, body in enumerate(source):
        if index in (5, 10, 11, 16):
            continue
        body = re.sub(r'^using ThinkThen;\n', '', body, flags=re.M)
        setup = source[1].split('if (isRefund.IsAnswered)')[0] if index == 2 else MESSAGES if index in (8, 9, 12, 14) else source[22].split('var forms =')[0] if index == 23 else ''
        client = '' if 'await using var client =' in body else 'await using var client = new ThinkThenClient();\n'
        observation = ''
        if index == 24:
            observation = 'if (isRefund.Status != topic.Status) throw new Exception("both described questions must preserve their scenario");'
        elif index == 26:
            observation = 'if (isRefund.Id is not StringId { Value: "a1" }) throw new Exception("saved question must retain its id");'
        elif index == 27:
            observation = 'if (topic.Item.id != "a1") throw new Exception("toolbox result must retain its original record");'
        elif index in (28, 30):
            observation = 'if (isRefund.Position != 1) throw new Exception("the call must return its result");'
        methods[index] = client + setup + body + '\n' + observation
    methods[10] = 'await using var client = new ThinkThenClient(); await foreach(var row in client.DecideStreamAsync("Ready?", new[]{"text"}, new DecideOptions { Threshold = Threshold.Band(.2,.8) })) Console.WriteLine(WhereItGoes(row));'
    runtime = {
        100: ('collection', 'await client.DecideAsync("Ready?",new[]{"text"});', r'ArgumentException.*DecideStreamAsync'),
        101: ('missing-toolbox', 'Toolbox.Parse("""{"refund":{"function":"decide","question":"Ready?"}}""").Choose("absent");', r'UsageException'),
        102: ('wrong-toolbox', 'Toolbox.Parse("""{"refund":{"function":"decide","question":"Ready?"}}""").Choose("refund");', r'UsageException'),
        103: ('missing-entry', '(await client.AnnotateAsync(new AnnotationQuestions {{"refund",DecideQuestion.Create("Ready?")}},"text")).Answer.Choose("absent");', r'UsageException.*absent'),
        104: ('wrong-entry', '(await client.AnnotateAsync(new AnnotationQuestions {{"refund",DecideQuestion.Create("Ready?")}},"text")).Answer.Choose("refund");', r'UsageException.*refund'),
        105: ('saved-filter-band', 'Toolbox.Parse("""{"filter":{"function":"filter","question":"Ready?","threshold":{"low":0.2,"high":0.8}}}""");', r'UsageException.*DecideStreamAsync'),
        106: ('undefined-choose-member', 'await client.ChooseAsync("Topic?","text",new[]{Topic.Billing,(Topic)999});', r'ArgumentException.*named members'),
        107: ('undefined-score-member', 'await client.ScoreAsync("Urgency?","text",new[]{Topic.Billing,(Topic)999});', r'ArgumentException.*named members'),
        108: ('undefined-rank-member', 'await foreach(var row in client.RankAsync("Urgency?",new[]{"text"},new[]{Topic.Billing,(Topic)999})) {}', r'ArgumentException.*named members'),
        109: ('undefined-tag-member', 'await client.TagAsync("Topic?","text",new[]{Topic.Billing,(Topic)999});', r'ArgumentException.*named members'),
        110: ('undefined-description-member', 'await client.ChooseAsync("Topic?","text",new[]{Topic.Billing,Topic.Shipping},new ChooseOptions<Topic>{Descriptions=new Dictionary<Topic,string>{{(Topic)999,"undefined"}}});', r'ArgumentException.*named members'),
        111: ('unknown-code-family', '_=((ErrorCode)0).Family();', r'UsageException'),
        112: ('parsed-rank-form', 'AnnotationQuestions.Parse("""{"priority":{"function":"rank","question":"Urgency?","levels":["routine","urgent"]}}""");', r'UsageException'),
        113: ('parsed-filter-form', 'AnnotationQuestions.Parse("""{"filter":{"function":"filter","question":"Ready?"}}""");', r'UsageException'),
    }
    for index, (_, body, _) in runtime.items():
        methods[index] = 'await using var client = new ThinkThenClient();\n' + body + '\nawait Task.CompletedTask;'
    program = 'using ThinkThen;\nusing System.Text.Json;\ninternal static class GuideProgram {\n' + source[10] + source[5] + source[16]
    program += 'static async Task<int> Main(string[] args) { try { if(args[0]=="host") { await CSharpIdealCases.Run(args[1]); return 0; } switch(args[0]) {\n'
    for index in methods:
        program += f'case "{index}": await Case{index}(); break;\n'
    program += 'default: throw new Exception("unknown guide case"); } return 0; } catch(Exception error) { Console.Error.WriteLine(error); return 1; } }\n'
    for index, body in methods.items():
        program += f'static async Task Case{index}() {{\n{body}\n}}\n'
    program += '}\n'
    path = folder / 'Program.cs'
    path.write_text(program)
    (folder / 'IdealCases.cs').write_text((ROOT / 'sdk/csharp/tests/IdealCases.cs').read_text())
    built = subprocess.run(['dotnet', 'build', str(reference), '--configfile', str(configuration), '--nologo', '-o', str(folder / 'bin')], capture_output=True, text=True)
    if built.returncode:
        raise AssertionError('C# literal build failed:\n' + built.stdout[-8000:] + built.stderr[-2000:])
    check('csharp/literal-build', built)
    assembly = folder / 'bin/Guide.dll'
    scripted = {name: shared_cases()[name] for name in ('empty', 'other')}

    def execute(index, name, *, output='', refusal=None, label=None):
        label = label or f'csharp/{index}/{name}'
        observers = []
        with scenario(name if name not in scripted else 'yes', case=scripted.get(name), observer=observers) as (work, env):
            (work / 'toolbox.json').write_text('{"entries":{"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"field":"/text"}}}}')
            args = ['dotnet', str(assembly), str(index)]
            if index == 'host':
                args.append(name)
            ran = subprocess.run(args, cwd=work, env=env, capture_output=True, text=True, timeout=120)
            server = observers[0]
            server.stdin.write('count\n')
            server.stdin.flush()
            requests = int(server.stdout.readline())
            no_request = isinstance(index, int) and index in runtime and index not in (103, 104)
            if (no_request and requests) or (not no_request and requests == 0):
                raise AssertionError(f'{label}: expected {"zero" if no_request else "positive"} fixture requests, got {requests}:\n'+ran.stdout+ran.stderr)
            if negative and index == 0 and name == 'yes':
                if ran.returncode != 0 or ran.stdout != 'refund queue\n' or requests != 1:
                    raise AssertionError('C# dispatch did not execute the refund branch with one real request')
                print('CSHARP DISPATCH csharp/0/yes requests=1 branch=refund queue', flush=True)
            check(label, ran, output=output, branch=output.splitlines()[0] if output.strip() else None, exit_code=1 if refusal else 0,
                  refusal=refusal, negative=negative if index == 0 and name == 'yes' and negative in ('output', 'branch', 'exit') else None)

    def cases(index, outcomes):
        for name, expected in outcomes.items():
            output, refusal = expected if isinstance(expected, tuple) else (expected, None)
            execute(index, name, output=output, refusal=refusal)

    if not host_only and negative != 'refusal':
        noanswer = ('', r'FailedException')
        states = {'yes': 'refund queue\n', 'no': 'inbox\n', 'unsure': 'a person reads it\n', 'failed': noanswer}
        cases(0, {'yes': 'refund queue\n', 'no': 'inbox\n', 'failed': noanswer})
        cases(1, states)
        cases(2, states)
        cases(3, {'yes': 'true\n', 'no': 'false\n', 'unsure': 'true\n', 'failed': 'could not judge it: ' + FAILURE.split(': ',2)[2] + '\n'})
        cases(4, {'yes': '0.95\n', 'no': '0.05\n', 'unsure': '0.5\n', 'failed': noanswer})
        cases(6, {'yes': 'billing team\n', 'no': 'warehouse\n', 'other': 'front desk\n', 'unsure': 'a person reads it\n', 'failed': noanswer})
        for name, prefix in [('yes', 'refund queue: '), ('no', 'inbox: '), ('unsure', 'a person reads it: '), ('failed', 'try again later: ')]:
            output = ''.join(prefix + message + '\n' for message in TEXTS)
            cases(7, {name: output})
            cases(12, {name: output})
        cases(8, {'yes': 'true\n'*3, 'no': 'false\n'*3, 'unsure': 'unsure\n'*3, 'failed': noanswer})
        cases(9, {'yes': 'refund queue\n'*3, 'no': 'inbox\n'*3, 'unsure': 'a person reads it 0.5\n'*3, 'failed': 'try again later backend.status\n'*3})
        cases(10, dict(states, failed='try again later\n'))
        cases(13, {'yes': ''.join('refund queue: ' + message + '\n' for message in TEXTS), 'no': '', 'failed': noanswer})
        cases(14, {'yes': ''.join('refund queue: ' + message + '\n' for message in TEXTS), 'no': '', 'failed': ''.join('try again later: ' + message + ' backend.status\n' for message in TEXTS)})
        cases(15, {'yes': 'billing team\nBilling\nShipping\n', 'no': '', 'failed': noanswer})
        cases(17, {'yes': 'answer today\n1.00\n', 'low-score': '0.00\n', 'failed': noanswer})
        cases(18, {'yes': ''.join(str(index + 1) + ' ' + message + '\n' for index, message in enumerate(TEXTS)), 'failed': noanswer})
        cases(19, {'yes': 'Please refund my order.\n', 'no': 'no line answers\n', 'unsure': 'a person reads it\n', 'failed': noanswer})
        cases(20, {'relations': 'person Morgan\norganization Example Group\n', 'empty': '', 'failed': noanswer})
        cases(21, {'relations': 'Morgan works_for Example Group\n', 'empty': '', 'failed': noanswer})
        for name in ('yes', 'no', 'unsure', 'failed', 'refund-failed', 'topic-failed', 'urgency-failed', 'refund-unsure', 'topic-unsure'):
            if name == 'failed' or name.endswith('-failed'):
                output = ''.join('try again later: ' + message + '\n' for message in TEXTS)
            elif name in ('unsure', 'refund-unsure', 'topic-unsure'):
                output = ''.join('a person reads it: ' + message + '\n' for message in TEXTS)
            elif name == 'yes':
                output = 'refund queue: billing 1.00\n' * 3
            else:
                output = ''
            cases(22, {name: output})
            cases(23, {name: noanswer if name == 'failed' or name.endswith('-failed') else ''})
        cases(24, {'yes': '', 'no': '', 'failed': noanswer})
        cases(25, {'yes': 'refund queue\n', 'no': 'inbox\n', 'failed': noanswer})
        cases(26, {'yes': '', 'unsure': '', 'failed': noanswer})
        cases(27, {'yes': '', 'unsure': '', 'failed': noanswer})
        cases(28, {'yes': '', 'failed': noanswer})
        cases(29, {'yes': '1 0 0\n', 'failed': noanswer})
        cases(30, {'yes': '', 'failed': noanswer})
    for index, (name, _, refusal) in runtime.items():
        if negative == 'refusal':
            continue
        execute(index, 'yes', refusal=refusal, label='csharp/refusal/' + name)
    host_errors = []
    for name in (() if negative == 'refusal' else ('yes', 'no', 'unsure', 'failed', 'refund-failed', 'topic-failed', 'urgency-failed', 'refund-unsure', 'topic-unsure', 'relations', 'low-score')):
        try:
            execute('host', name, output='C# ideal host contracts passed: ' + name + '\n')
        except AssertionError as error:
            host_errors.append(str(error))
            print('FAIL ' + str(error), flush=True)

    refusals = {
        'retired-record-exception': ('_=typeof(RecordException);', 'CS0246'),
        'choose-if': ('if(await client.ChooseAsync("Topic?","text",Enum.GetValues<Topic>())) {}', 'CS0029'),
        'bare-enum-equality': ('var row=await client.ChooseAsync("Topic?","text",Enum.GetValues<Topic>()); _=row==Topic.Billing;', 'CS0019'),
        'bare-string-equality': ('var row=await client.ChooseAsync("Topic?","text",new[]{"billing","shipping"}); _=row=="billing";', 'CS0019'),
        'bare-bool-equality': ('var row=await client.DecideAsync("Ready?","text"); _=row==true;', 'CS0019'),
        'bare-answer-conversion': ('bool wrong=await client.DecideAsync("Ready?","text");', 'CS0029'),
        'error-on-nonfailed': ('var row=await client.DecideAsync("Ready?","text"); _=row.Error;', 'CS1061'),
        'filter-band': ('_=new FilterOptions { Threshold=Threshold.Band(.2,.8) };', 'CS0266'),
        'filter-entry': ('_=new AnnotationQuestions {{"filter",FilterQuestion.Create("Ready?")}};', 'CS1503'),
        'retired-filter-stream': ('_=client.FilterStreamAsync("Ready?",new[]{"text"});', 'CS1061'),
        'retired-try-answer': ('var row=await client.DecideAsync("Ready?","text"); row.TryGetAnswer(out var wrong);', 'CS1061'),
        'retired-cut-from': ('_=Cut.From(.5);', 'CS0117'),
    }
    for name, (body, diagnostic) in refusals.items():
        if negative == 'refusal' and name != 'choose-if':
            continue
        if negative == 'refusal' and name == 'choose-if':
            body = 'if((await client.ChooseAsync("Topic?","text",Enum.GetValues<Topic>())).Answer==Topic.Billing) {}'
        refusal_program = program.replace('switch(args[0]) {', 'await Refusal(); switch(args[0]) {')
        refusal_program = refusal_program[:-2] + 'static async Task Refusal() { await using var client=new ThinkThenClient(); ' + body + ' await Task.CompletedTask; }\n}\n'
        path.write_text(refusal_program)
        result = subprocess.run(['dotnet', 'build', str(reference), '--no-restore', '--nologo', '-o', str(folder / 'refusal')], capture_output=True, text=True)
        check('csharp/compiler/' + name, result, exit_code=1, refusal=r'error ' + diagnostic)
    for name, omitted in [('exhaustive-state', '    Status.Unsure => "a person reads it",\n'), ('exhaustive-enum', '    Topic.Other => "front desk",\n')]:
        if omitted not in program:
            raise AssertionError('C# exhaustive refusal no longer matches its literal declaration')
        path.write_text(program.replace(omitted, ''))
        result = subprocess.run(['dotnet', 'build', str(reference), '--no-restore', '--nologo', '-o', str(folder / 'refusal')], capture_output=True, text=True)
        check('csharp/compiler/' + name, result, exit_code=1, refusal=r'error CS8509')
    if host_errors:
        raise AssertionError('C# host failures:\n' + '\n'.join(host_errors))
    print('C# host and compiler proof passed' if host_only else 'C# ideal guide: every fence, scenario output, branch and promised refusal passed', flush=True)
