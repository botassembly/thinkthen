"""Execute the literal Java ideal guide against its installed package."""
import os
from pathlib import Path
import subprocess
import sys
from guide_examples import ROOT, blocks, check, scenario
from bash_guide_examples import cases as shared_cases

IMPORTS = 'import dev.thinkthen.*;\nimport java.util.*;\nimport java.nio.file.*;\n'
MESSAGES = 'List<String> messages = List.of("Please refund my order.", "Where is my parcel?", "I was charged twice for one invoice.");\n'
FAILURE = 'failed: backend.status: the backend answered status 503: {"error":"unavailable"}'


def run_java(classpath, negative=None, *, host_only=False):
    classpath = os.pathsep.join(str(Path(part).resolve()) for part in classpath.split(os.pathsep))
    guide = ROOT / 'docs/surfaces/java.md'
    examples = blocks(guide)
    if len(examples) != 30 or any(language != 'java' for language, _, _ in examples):
        raise AssertionError('Java guide fences changed: classify every new or changed example')
    source = [body for _, body, _ in examples]
    java = str(Path(os.environ['JAVA_HOME']) / 'bin/java')
    javac = str(Path(os.environ['JAVA_HOME']) / 'bin/javac')

    scripted = {name: shared_cases()[name] for name in ('empty','other')}
    errors = []
    def verify(*args, **kwargs):
        try:
            check(*args, **kwargs)
            sys.stdout.flush()
        except AssertionError as error:
            if negative:
                raise
            errors.append(str(error))
            print('FAIL ' + str(error), flush=True)

    def execute(label, literal, name, *, setup='', declarations='', output='', branch=None, refusal=None, compile_refusal=None, observations=''):
        observers=[]
        with scenario(name if name not in scripted else 'yes',case=scripted.get(name),observer=observers) as (folder, env):
            full = 'public class ' in literal
            classname = 'AsyncExample' if 'public class AsyncExample' in literal else 'Example'
            if full:
                program = literal
            else:
                client = 'try (var client = ThinkThen.builder().build()) {' if not literal.startswith('try (var client =') and ('client.' in literal or 'client.' in setup) else ''
                end = '}' if client else ''
                program = IMPORTS + 'public class Example {\n' + declarations + '\npublic static void main(String[] args) throws Exception {\n' + client + setup + literal + '\n' + observations + '\n' + end + '\n} }\n'
            path = folder / (classname + '.java')
            path.write_text(program)
            # Setup belongs to the guide's named Toolbox; it is not a scripted public output.
            (folder / 'toolbox.json').write_text('{"entries":{"topic":{"question":{"function":"choose","question":"What is the message about?","choices":["billing","shipping","other"],"field":"/text"}}}}')
            built = subprocess.run([javac, '-J-Duser.home='+str(folder / 'home'), '--release', '22', '-Xlint:all', '-Werror', '-cp', classpath, str(path)], cwd=folder, env=env, capture_output=True, text=True, timeout=60)
            if compile_refusal:
                verify(label, built, exit_code=1, refusal=compile_refusal)
                return
            verify(label + '/compile', built)
            if built.returncode: return
            ran = subprocess.run([java, '-Duser.home='+str(folder / 'home'), '--enable-native-access=ALL-UNNAMED', '-cp', str(folder) + os.pathsep + classpath, classname], cwd=folder, env=env, capture_output=True, text=True, timeout=90)
            server=observers[0]; server.stdin.write('count\n'); server.stdin.flush()
            requests=int(server.stdout.readline())
            if negative and label=='java/0/yes':
                if ran.returncode!=0 or ran.stdout!='refund queue\n' or requests!=1:
                    raise AssertionError('java/0/yes: dispatch witness did not execute the refund branch with one real fixture request')
                print('JAVA DISPATCH java/0/yes requests=1 branch=refund queue',flush=True)
            verify(label, ran, output=output, branch=branch, exit_code=1 if refusal else 0, refusal=refusal,
                  negative=negative if label == 'java/0/yes' and negative in ('output', 'branch', 'exit') else None)
            no_request=label.startswith('java/refusal/') and label.rsplit('/',1)[-1] not in ('missing-entry','wrong-entry')
            if (no_request and requests != 0) or (not no_request and requests == 0):
                message=label+': expected '+('zero' if no_request else 'positive')+' backend requests, got '+str(requests)
                errors.append(message); print('FAIL '+message,flush=True)


    def cases(index, outcomes, *, setup='', declarations=''):
        for name, output in outcomes.items():
            refusal = output[1] if isinstance(output, tuple) else None
            expected = output[0] if isinstance(output, tuple) else output
            observations=''
            if index==23 and name not in ('failed','refund-failed','topic-failed','urgency-failed'):
                observations='if(!form.isAnswered() || form.throwIfFailed()!=form) throw new AssertionError("explicit form check returns the answered parent even with unsure children");'
            elif index==24 and name!='failed':
                observations='if(!isRefund.isAnswered() || !topic.isAnswered()) throw new AssertionError("described questions must return answered results");'
            elif index==26 and name!='failed':
                state='isUnsure' if name=='unsure' else 'isAnswered'
                observations=f'if(!isRefund.{state}() || !isRefund.id().orElseThrow().equals(new JsonString("a1"))) throw new AssertionError("saved question keeps state and id");'
            elif index==27 and name!='failed':
                state='isUnsure' if name=='unsure' else 'isAnswered'
                observations=f'if(!topic.{state}() || !topic.item().get("id").equals("a1")) throw new AssertionError("toolbox call keeps state and original record");'
            execute(f'java/{index}/{name}', source[index], name, setup=setup, declarations=declarations,
                    output=expected, branch=expected.splitlines()[0] if expected else None, refusal=refusal, observations=observations)

    if negative == 'refusal':
        execute('java/refusal/result-in-if', 'var topic=client.choose("Which?","text",Topic.class); if(topic.answer()==Topic.BILLING) {}',
                'yes', declarations=source[5], compile_refusal=r'incompatible types')
        raise AssertionError('Java negative refusal control was accepted')

    if not host_only:
        noanswer = ('', r'FailedException')
        all_states = {'yes':'refund queue\n', 'no':'inbox\n', 'unsure':'a person reads it\n', 'failed':noanswer}
        cases(0, {'yes':'refund queue\n', 'no':'inbox\n', 'failed':noanswer})
        cases(1, all_states)
        cases(2, all_states, setup=source[1].split('if (isRefund.isAnswered())')[0])
        cases(3, {'yes':'true\n','no':'false\n','unsure':'true\n','failed':'could not judge it: '+FAILURE.removeprefix('failed: backend.status: ')+'\n'})
        cases(4, {'yes':'0.95\n','no':'0.05\n','unsure':'0.5\n','failed':noanswer})
        cases(6, {'yes':'billing team\n','no':'warehouse\n','other':'front desk\n','unsure':'a person reads it\n','failed':noanswer}, declarations=source[5])
        messages=('Please refund my order.','Where is my parcel?','I was charged twice for one invoice.')
        for name,prefix in [('yes','refund queue: '),('no','inbox: '),('unsure','a person reads it: '),('failed','try again later: ')]:
            cases(7, {name:''.join(prefix+message+'\n' for message in messages)})
        cases(8, {'yes':'true\n'*3,'no':'false\n'*3,'unsure':'unsure\n'*3,'failed':noanswer},setup=MESSAGES)
        for name,route in [('yes','refund queue'),('no','inbox'),('unsure','a person reads it 0.5'),('failed','try again later backend.status')]:
            cases(9, {name:(route+'\n')*3},setup=MESSAGES)
        for name,route in [('yes','refund queue'),('no','inbox'),('unsure','a person reads it'),('failed','try again later')]:
            cases(11, {name:''.join(route+': '+message+'\n' for message in messages)},setup=MESSAGES,declarations=source[10])
        cases(12, {'yes':''.join('refund queue: '+message+'\n' for message in messages),'no':'','failed':noanswer})
        cases(13, {'yes':''.join('refund queue: '+message+'\n' for message in messages),'no':'','failed':''.join('try again later: '+message+' backend.status\n' for message in messages)},setup=MESSAGES)
        cases(15, {'yes':'billing team\nbilling\nshipping\n','no':'','failed':noanswer}, declarations=source[14])
        cases(17, {'yes':'answer today\n1.00\n','no':'answer today\n1.00\n','low-score':'0.00\n','failed':noanswer},declarations=source[16])
        cases(18, {'yes':'1 Please refund my order.\n2 Where is my parcel?\n3 I was charged twice for one invoice.\n','failed':noanswer},declarations=source[16])
        cases(19, {'yes':'Please refund my order.\n','no':'no line answers\n','unsure':'a person reads it\n','failed':noanswer})
        cases(20, {'relations':'person Morgan\norganization Example Group\n','empty':'','failed':noanswer})
        cases(21, {'relations':'Morgan works_for Example Group\n','empty':'','failed':noanswer})
        enums=source[5].split('static String teamFor')[0]+source[16]
        for name in ('refund-failed','topic-failed','urgency-failed','refund-unsure','topic-unsure','failed','yes','no'):
            if name.endswith('-failed') or name=='failed': output=''.join('try again later: '+message+'\n' for message in messages)
            elif name=='topic-unsure': output=''.join('a person reads it: '+message+'\n' for message in messages)
            elif name in ('yes','refund-unsure'): output='refund queue: billing 1.00\n'*3
            else: output=''
            cases(22,{name:output},declarations=enums)
        form_setup=source[22].split('try (var forms')[0]
        cases(23, {'yes':'','no':'','refund-unsure':'','topic-unsure':'','failed':noanswer,'refund-failed':noanswer,'topic-failed':noanswer,'urgency-failed':noanswer}, setup=form_setup,declarations=enums)
        cases(24, {'yes':'','no':'','failed':noanswer},declarations=source[5])
        cases(25, {'yes':'refund queue\n','no':'inbox\n','failed':noanswer})
        cases(26, {'yes':'','unsure':'','failed':noanswer})
        cases(27, {'yes':'','unsure':'','failed':noanswer})
        cases(28, {'yes':'','failed':noanswer})
        cases(29, {'yes':'1 0 0\n','failed':noanswer})
        # Every Java-specific refusal named by the guide has its own compiler or
        # runtime case; a valid program is compiled by all preceding executions.
        enum = source[5]
        refusals = {
            'result-in-if': ('var topic=client.choose("Which?","text",Topic.class); if(topic) {}',r'incompatible types'),
            'error-on-nonfailed': ('client.decide("Ready?","text").error();',r'cannot find symbol'),
            'result-versus-enum': ('var topic=client.choose("Which?","text",Topic.class); boolean wrong=topic==Topic.BILLING;',r'incomparable types'),
            'filter-band': ('FilterOptions.builder().threshold(Threshold.band(0.2,0.8));',r'incompatible types'),
            'filter-entry': ('AnnotationQuestions.builder().put("filter",FilterQuestion.of("Ready?"));',r'incompatible types'),
            'retired-RecordException': ('Class<?> removed=RecordException.class;',r'cannot find symbol'),
            'retired-filterRows': ('client.filterRows("Ready?",List.of("text"));',r'cannot find symbol'),
            'retired-choose-numeric-threshold': ('ChooseOptions.builder().threshold(0.7);',r'incompatible types'),
            'retired-tag-numeric-threshold': ('TagOptions.builder().threshold(0.7);',r'incompatible types'),
            'retired-find-numeric-threshold': ('FindOptions.builder().threshold(0.7);',r'incompatible types'),
            'retired-recognize-numeric-threshold': ('RecognizeOptions.builder().threshold(0.7);',r'incompatible types'),
            'retired-relate-numeric-threshold': ('RelateOptions.builder().threshold(0.7);',r'incompatible types'),
        }
        for name,(literal,diagnostic) in refusals.items():
            if negative == 'refusal' and name == 'result-in-if':
                literal='var topic=client.choose("Which?","text",Topic.class); if(topic.answer()==Topic.BILLING) {}'
            execute('java/refusal/'+name,literal,'yes',declarations=enum,compile_refusal=diagnostic)
        execute('java/refusal/exhaustive-result', 'System.out.println(whereItGoes(client.decide("Ready?","text")));','yes',declarations=source[10].replace('        case Unsure<Boolean, String> unsure -> "a person reads it";\n',''),compile_refusal=r'does not cover all possible')
        execute('java/refusal/exhaustive-enum', 'System.out.println(teamFor(Topic.BILLING));','yes',declarations=enum.replace('        case OTHER -> "front desk";\n',''),compile_refusal=r'does not cover all possible')
        runtime = {
            'scalar-blank': ('client.decide("Ready?","");',r'FailedException'),
            'scalar-parent-blank': ('client.annotate(AnnotationQuestions.builder().put("refund",DecideQuestion.of("Ready?")).build(),"");',r'FailedException'),
            'collection': ('client.decide("Ready?",List.of("text"));',r'IllegalArgumentException.*decideStream'),
            'missing-toolbox': ('Toolbox.builder().put("refund",DecideQuestion.of("Ready?")).build().choose("absent");',r'UsageException.*No choose question named absent'),
            'wrong-toolbox': ('Toolbox.builder().put("refund",DecideQuestion.of("Ready?")).build().choose("refund");',r'UsageException.*No choose question named refund'),
            'missing-entry': ('client.annotate(AnnotationQuestions.builder().put("refund",DecideQuestion.of("Ready?")).build(),"text").answer().choose("absent");',r'UsageException.*No choose annotation named absent'),
            'wrong-entry': ('client.annotate(AnnotationQuestions.builder().put("refund",DecideQuestion.of("Ready?")).build(),"text").answer().choose("refund");',r'UsageException.*No choose annotation named refund'),
            'saved-filter-band': ('Toolbox.fromDefinitions("{\\"filter\\":{\\"function\\":\\"filter\\",\\"question\\":\\"Ready?\\",\\"threshold\\":{\\"low\\":0.2,\\"high\\":0.8}}}");',r'UsageException.*decideStream'),
        }
        for name,(literal,diagnostic) in runtime.items():
            execute('java/refusal/'+name,literal,'yes',refusal=diagnostic)
    # These additional public cases cover host behavior not expressed by a guide fence.
    host_source=(ROOT / 'sdk/java/consumer/src/main/java/JavaIdealCases.java').read_text()
    for name in ('yes','no','unsure','failed','refund-failed','topic-failed','urgency-failed','refund-unsure','topic-unsure','relations'):
        with scenario(name) as (folder,env):
            path=folder / 'JavaIdealCases.java'; path.write_text(host_source)
            built=subprocess.run([javac,'-J-Duser.home='+str(folder / 'home'),'--release','22','-Xlint:all','-Werror','-cp',classpath,str(path)],cwd=folder,env=env,capture_output=True,text=True,timeout=60)
            verify('java/host/'+name+'/compile',built)
            if built.returncode: continue
            ran=subprocess.run([java,'-Duser.home='+str(folder / 'home'),'--enable-native-access=ALL-UNNAMED','-cp',str(folder)+os.pathsep+classpath,'JavaIdealCases',name],cwd=folder,env=env,capture_output=True,text=True,timeout=90)
            verify('java/host/'+name,ran,output='Java ideal host contracts passed: '+name+'\n')
    if errors:
        raise AssertionError('Java ideal guide failures: ' + str(len(errors)) + '\n' + '\n'.join(errors))
    print('Java ideal host proof: toolbox types, formatting, ownership and equality passed' if host_only else 'Java ideal guide: every fence accounted for; scenario output, branches and refusals passed')
