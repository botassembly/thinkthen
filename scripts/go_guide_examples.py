"""Run every literal Go guide block and its promised branches and refusals."""
from pathlib import Path
import json
import os
import subprocess
import tempfile

from guide_examples import ROOT, IDEAL, blocks, scenario, check, environment

MESSAGES = ['Please refund my order.', 'Where is my parcel?',
            'I was charged twice for one invoice.']
FAILURE = 'failed: backend.status: the backend answered status 503: {"error":"unavailable"}'
TOOLBOX = {'entries': {'topic': {'question': {
    'function': 'choose', 'question': 'What is the message about?',
    'choices': ['billing', 'shipping', 'other']}}}}
IMPORTS = '''package main
import("context";"encoding/json";"errors";"fmt";"log";"os";"slices";tt "github.com/botassembly/thinkthen/sdk/go/thinkthen")
var _=json.Marshal;var _=errors.Is;var _=fmt.Println;var _=os.ReadFile;var _=slices.Values[[]string]
'''
MAIN = 'func main(){if err:=run(context.Background());err!=nil{log.Fatal(err)}}\n'
OPEN = '''func run(ctx context.Context) error {
client,err:=tt.New(tt.Config{});if err!=nil{return err};defer client.Close()
'''
REFUND = 'asksRefund:=tt.DecideQuestion{Question:"Does the customer request a refund?",Threshold:tt.Band(0.2,0.8)}\n'


def program(fences, index):
    body = fences[index][1]
    if index == 0:
        return body
    topic = fences[13 if index == 14 else 5][1]
    prelude = IMPORTS + topic + '\n' + fences[15][1] + '\n'
    setup = REFUND if index in (2, 27, 28) else ''
    messages = 'messages:=[]string{' + ','.join(json.dumps(m) for m in MESSAGES) + '}\n'
    if index in (8, 10):
        setup = messages + REFUND
    if index == 10:
        prelude += fences[9][1] + '\n'
    if index == 12:
        setup = messages + 'asksRefund:=tt.FilterQuestion{Question:"Does the customer request a refund?"}\n'
    if index == 22:
        setup = 'asksRefund :=' + fences[21][1].split('asksRefund :=', 1)[1].split('\nforms :=', 1)[0] + '\n'
    if index == 23:
        body += '''
yes,err:=tt.Answer(client.Decide(ctx,asksRefund,"Please refund my order."));if err!=nil{return err}
choice,err:=tt.Answer(client.Choose(ctx,asksTopic,"Please refund my order."));if err!=nil{return err}
fmt.Println(yes,choice)
'''
    if index == 24:
        body += '''
if err!=nil{return err};id,ok:=isRefund.ID();if !ok||id.String()!="a1"{return fmt.Errorf("record ID lost")}
yes,err:=isRefund.Answer();if err!=nil{return err};fmt.Println(yes,isRefund.Item()["id"])
'''
    if index == 25:
        body += '\nif err!=nil{return err};choice,err:=topic.Answer();if err!=nil{return err};fmt.Println(choice)\n'
    if index == 26:
        body += '\nif err!=nil{return err};yes,err:=isRefund.Answer();if err!=nil{return err};fmt.Println(yes)\n'
        return prelude + MAIN + 'func run(ctx context.Context)error{\n' + REFUND + body + '\nreturn nil}'
    if index == 28:
        body += '''
if err!=nil{return err};if !isRefund.IsAnswered(){return fmt.Errorf("first call did not answer")}
stop();_,err=client.Decide(ctx,asksRefund,"Please refund my order.")
if !errors.Is(err,context.Canceled){return fmt.Errorf("cancel was lost: %v",err)};fmt.Println("cancelled")
'''
    return prelude + MAIN + OPEN + setup + body + '\nreturn nil\n}\n'


def examples():
    lines = lambda prefix: ''.join(prefix + m + '\n' for m in MESSAGES)
    cases = {
        0: [('yes', 'refund queue\n'), ('no', 'inbox\n'), ('failed', '')],
        1: [('yes', 'refund queue\n'), ('no', 'inbox\n'),
            ('unsure', 'a person reads it\n'), ('failed', '')],
        2: [('yes', 'refund queue\n'), ('no', 'inbox\n'),
            ('unsure', 'a person reads it\n'), ('failed', '')],
        3: [('yes', 'true\n'), ('no', 'false\n'),
            ('failed', 'the call did not complete: ' + FAILURE.removeprefix('failed: backend.status: ') + '\n')],
        4: [('yes', '0.95\n'), ('no', '0.05\n'), ('unsure', '0.5\n'), ('failed', '')],
        6: [('yes', 'billing team\n'), ('no', 'warehouse\n'), ('other', 'front desk\n'),
            ('unsure', 'a person reads it\n'), ('failed', '')],
        7: [('yes', lines('refund queue: ')), ('no', lines('inbox: ')),
            ('unsure', lines('a person reads it: ')), ('failed', lines('try again later: '))],
        8: [('yes', 'true\n' * 3), ('no', 'false\n' * 3), ('unsure', 'unsure\n' * 3), ('failed', '')],
        10: [('yes', lines('refund queue ')), ('no', lines('inbox ')),
             ('unsure', lines('a person reads it ')), ('failed', lines('try again later '))],
        11: [('yes', lines('refund queue: ')), ('no', ''), ('failed', '')],
        12: [('yes', lines('refund queue: ')), ('no', ''),
             ('failed', ''.join('try again later: ' + m + ' ' + FAILURE + '\n' for m in MESSAGES))],
        14: [('yes', 'billing team\nbilling\nshipping\n'), ('no', ''), ('failed', '')],
        16: [('yes', 'answer today\n1.00\n'), ('low-score', '0.00\n'), ('failed', '')],
        17: [('yes', ''.join(str(i) + ' ' + m + '\n' for i, m in enumerate(MESSAGES, 1))), ('failed', '')],
        18: [('yes', 'Please refund my order.\n'), ('no', 'no line answers\n'),
             ('unsure', 'a person reads it\n'), ('failed', '')],
        19: [('relations', 'person Morgan\norganization Example Group\n'), ('empty', ''), ('failed', '')],
        20: [('relations', 'Morgan works_for Example Group\n'), ('empty', ''), ('failed', '')],
        21: [('yes', 'refund queue: billing 1.00\n' * 3), ('no', ''),
             ('refund-unsure', lines('a person reads it: ')), ('topic-unsure', lines('a person reads it: ')),
             ('failed', lines('try again later: ')), ('refund-failed', lines('try again later: ')),
             ('topic-failed', lines('try again later: ')), ('urgency-failed', lines('try again later: '))],
        22: [('yes', 'refund=true\ttopic=billing\turgency=1\n'),
             ('refund-unsure', 'refund=unsure\ttopic=billing\turgency=1\n'),
             ('topic-unsure', 'refund=true\ttopic=unsure\turgency=1\n'),
             ('failed', ''), ('refund-failed', ''), ('topic-failed', ''), ('urgency-failed', '')],
        23: [('yes', 'true billing\n')], 24: [('yes', 'true a1\n')],
        25: [('yes', 'billing\n')], 26: [('yes', 'true\n')],
        27: [('yes', None)], 28: [('yes', 'cancelled\n')],
    }
    return cases


def run_go(command=None, negative=None, start=0):
    fences = blocks(IDEAL / 'go.md')
    assert len(fences) == 29 and all(language == 'go' for language, _, _ in fences)
    module = os.environ['THINKTHEN_GO_GUIDE_MODULE']
    go = str(ROOT / 'scripts/go-build')
    special = {
        'empty': {'replies': [{'criterion': 'BEGIN', 'distribution': {
            key: float(key == 'OUT') for key in ('BEGIN', 'INSIDE', 'END', 'SINGLE', 'OUT')}}]},
        'other': {'replies': [{'distribution': {'billing': 0., 'shipping': 0., 'other': 1.}}]},
    }
    with tempfile.TemporaryDirectory(prefix='go-guide-build-') as temporary:
        build = Path(temporary)
        env = isolated_environment(build)
        if negative == 'refusal':
            refusals(go, module, negative, build, env)
            return
        for index, runs in examples().items():
            if index < start:
                continue
            folder = build / str(index)
            folder.mkdir()
            module_file(folder, module)
            (folder / 'main.go').write_text(program(fences, index))
            executable = str(folder / 'example')
            compiled = subprocess.run([go, 'build', '-o', executable, '.'],
                                      cwd=folder, env=env, text=True, capture_output=True)
            check(f'go compile block {index} line {fences[index][2]}', compiled, output='')
            for name, output in runs:
                with scenario('yes' if name in special else name, case=special.get(name)) as (runtime, run_env):
                    (runtime / 'toolbox.json').write_text(json.dumps(TOOLBOX))
                    result = subprocess.run([executable], cwd=runtime, env=run_env, text=True, capture_output=True)
                    if index == 27:
                        attempts, responses, sent = map(int, result.stdout.split())
                        if (attempts, responses) != (1, 1) or sent <= 0:
                            raise AssertionError('Go usage did not account for the successful request')
                    failed_exit = name == 'failed' and index in (0, 1, 2, 4, 6, 8, 11, 14, 16, 17, 18, 19, 20, 22)
                    failed_exit |= index == 22 and name.endswith('-failed')
                    check(f'go block {index} line {fences[index][2]} {name}', result,
                          output=output, branch=output.splitlines()[0] if output else None,
                          exit_code=int(failed_exit), negative=negative)
        refusals(go, module, negative, build, env)
        runtime_contract(command, env, negative)


def isolated_environment(folder):
    env = environment()
    env['THINKTHEN_GO_HOME'] = os.environ['THINKTHEN_GO_HOME']
    env['THINKTHEN_LOOPBACK'] = os.environ['THINKTHEN_LOOPBACK']
    for key, child in (('HOME', 'home'), ('XDG_CONFIG_HOME', 'config'), ('XDG_CACHE_HOME', 'cache')):
        target = folder / child
        target.mkdir()
        env[key] = str(target)
    return env


def runtime_contract(command, env, negative=None):
    executable = command or os.environ.get('THINKTHEN_GO_GUIDE_TEST_COMMAND')
    if not executable:
        raise RuntimeError('Go guide needs its prepared runtime-contract consumer')
    result = subprocess.run([executable, '-test.run=^TestGuide', '-test.v'],
                            env=env, text=True, capture_output=True)
    check('go runtime refusals, form failures, record identity, scale order and canonical offsets',
          result, branch='PASS', negative=negative)


def run_runtime():
    with tempfile.TemporaryDirectory(prefix='go-guide-runtime-') as directory:
        runtime_contract(None, isolated_environment(Path(directory)))


def module_file(folder, module):
    (folder / 'go.mod').write_text('module guide\n\ngo 1.27\nrequire github.com/botassembly/thinkthen/sdk/go/thinkthen v0.0.0\nreplace github.com/botassembly/thinkthen/sdk/go/thinkthen => ' + module + '\n')


def refusals(go, module, negative, build, env):
    expressions = {
        'result in if': ('func f(r tt.DecideRow[string]) {if r {}}', r'non-boolean'),
        'choice result in if': ('func f(r tt.ChooseRow[string,string]) {if r {}}', r'non-boolean'),
        'choice equals answer': ('func f(r tt.ChooseRow[string,string]) bool {return r=="billing"}', r'mismatched types'),
        'result equals result': ('func f(a,b tt.DecideRow[string])bool{return a==b}', r'cannot be compared'),
        'result equals bool': ('func f(r tt.DecideRow[string])bool{return r==true}', r'mismatched types'),
        'result as map key': ('var _ map[tt.DecideRow[string]]bool', r'invalid map key type'),
        'filter band': ('var _ = tt.FilterQuestion{Threshold:tt.Band(.2,.8)}', r'cannot use.*Band.*Cut'),
        'filter annotation': ('var _ tt.AnnotationQuestion=tt.FilterQuestion{}', r'does not implement.*AnnotationQuestion'),
        'rank annotation': ('var _ tt.AnnotationQuestion=tt.RankQuestion[string]{}', r'does not implement.*AnnotationQuestion'),
        'nested annotation': ('var _ tt.AnnotationQuestion=tt.AnnotateQuestion{}', r'does not implement.*AnnotationQuestion'),
        'general annotation map': ('var _=tt.AnnotateQuestion{Questions:map[string]tt.Question{}}', r'cannot use.*map.*AnnotationQuestion'),
    }
    for number, (label, (expression, diagnostic)) in enumerate(expressions.items()):
        folder = build / ('refusal-' + str(number))
        folder.mkdir()
        module_file(folder, module)
        (folder / 'main.go').write_text('package main\nimport tt "github.com/botassembly/thinkthen/sdk/go/thinkthen"\n' + expression + '\nfunc main(){}\n')
        result = subprocess.run([go, 'build', '.'], cwd=folder, env=env, text=True, capture_output=True)
        if negative == 'refusal':
            result = subprocess.CompletedProcess(result.args, 0, '', '')
        check('go refusal ' + label, result, exit_code=1,
              refusal=diagnostic, negative=negative)

    folder = build / 'refusal-unhandled-topic'
    folder.mkdir()
    module_file(folder, module)
    declaration = blocks(IDEAL / 'go.md')[5][1]
    source = 'package main\nimport("context";"fmt";"log")\n' + declaration
    source += '\nconst TopicNew Topic = "new"\n' + MAIN
    source += 'func run(ctx context.Context)error{_,err:=teamFor(TopicNew);return err}\n'
    (folder / 'main.go').write_text(source)
    executable = str(folder / 'example')
    compiled = subprocess.run([go, 'build', '-o', executable, '.'], cwd=folder,
                              env=env, text=True, capture_output=True)
    check('go added choice compiles without an exhaustive-switch checker', compiled, output='')
    result = subprocess.run([executable], cwd=folder, env=env, text=True, capture_output=True)
    check('go added choice is refused by the literal guide switch', result, output='',
          exit_code=1, refusal=r'no team for topic "new"', negative=negative)
