"""Execute every literal Rust guide block and its documented refusals."""
import json
import subprocess
import sys

from guide_examples import IDEAL, blocks, check, scenario
from bash_guide_examples import FAILURE, cases
from rust_guide_support import consumer, compile_refusal, rank_fixture

MESSAGES = ['Please refund my order.', 'Where is my parcel?', 'I was charged twice for one invoice.']
ENUMS = '''
engine::choices! { enum Topic { Billing="billing", Shipping="shipping", Other="other" } }
engine::choices! { enum Urgency { Routine="routine", Urgent="urgent" } }
'''


def program(examples):
    functions = []
    for index, (_, body, _) in enumerate(examples):
        prefix = 'use thinkthen::*;\n'
        if 'json!' in body and 'use serde_json::json' not in body:
            prefix += 'use serde_json::json;\n'
        suffix = ''
        if index in (0, 5, 21):
            suffix = 'main()?;'
        else:
            prefix += 'let engine=Engine::from_env()?;\n'
            if index == 2:
                prefix += 'let asks_refund=Decide::new("Does the customer request a refund?")?.threshold(Threshold::band(0.2,0.8)?);\nlet is_refund=engine.run(&asks_refund,"Where is my parcel?")?;\n'
            elif index in (7, 8, 9):
                prefix += 'let asks_refund=Decide::new("Does the customer request a refund?")?.threshold(Threshold::band(0.2,0.8)?);\n'
            elif index == 11:
                prefix += 'let asks_refund=Filter::new("Does the customer request a refund?")?;\n'
            elif index in (24, 25, 26):
                prefix += 'let asks_refund=Decide::new("Does the customer request a refund?")?;\n'
            if index in (7, 8, 9, 11):
                prefix += 'let messages=vec![' + ','.join(json.dumps(message) for message in MESSAGES) + '];\n'
            if index == 19:
                prefix += 'let questions=Annotate::new([("refund",AnnotatableQuestion::from(Decide::new("Does the customer request a refund?")?)),("topic",Choose::<Topic>::new("What is the message about?")?.into()),("urgency",Score::<Urgency>::new("How urgent is the request?")?.into())])?;\n'
            elif index == 20:
                suffix = 'println!("{}",serde_json::to_string(&(asks_refund,asks_topic))?);'
            elif index in (22, 23, 24):
                suffix = 'println!("{}",' + ('topic' if index == 23 else 'is_refund') + ');'
            elif index == 26:
                suffix = 'stop.cancel();let error=engine.run(&asks_refund,"Please refund my order.").unwrap_err();assert_eq!(error.code(),Code::LocalCancelled);println!("{}",error.code());'
        functions.append(f'fn example_{index}()->Result<(),Box<dyn std::error::Error>>{{\n{prefix}{body}\n{suffix}\nOk(())\n}}')
    dispatch = ','.join(f'{index}=>example_{index}()' for index in range(len(examples)))
    return 'extern crate engine as thinkthen;\n' + ENUMS + '\n'.join(functions) + '''
fn main() {
    let index:usize=std::env::args().nth(1).unwrap().parse().unwrap();
    let result=match index {''' + dispatch + ''',_=>unreachable!()};
    if let Err(error)=result {
        if let Some(thinkthen::NoAnswer::Failed(record))=error.downcast_ref::<thinkthen::NoAnswer>() {
            eprintln!("{}: {}",record.code(),record);
        } else if let Some(call)=error.downcast_ref::<thinkthen::Error>() {
            eprintln!("{}: {}",call.code(),call);
        } else { eprintln!("{error}"); }
        std::process::exit(1);
    }
}
'''


def expected(index, name):
    failed = name == 'failed' or name.endswith('-failed')
    routed = {'yes': 'refund queue', 'no': 'inbox', 'unsure': 'a person reads it', 'failed': 'try again later'}
    scalar = (0, 1, 2, 4, 5, 12, 13, 15, 16, 17, 22, 23, 24)
    if failed and (index in scalar or index in (7, 10, 14, 19)):
        return '', 1
    if index in (0, 1, 2):
        return routed[name] + '\n', 0
    if index == 3:
        if failed:
            return 'the call did not complete: ' + FAILURE.removeprefix('failed: backend.status: ') + '\n', 0
        return ('false' if name == 'no' else 'true') + '\n', 0
    if index == 4:
        return {'yes': '0.95\n', 'no': '0.05\n', 'unsure': '0.5\n'}[name], 0
    if index == 5:
        return {'yes':'billing team\n', 'no':'warehouse\n', 'other':'front desk\n', 'unsure':'a person reads it\n'}[name], 0
    if index == 6:
        return ''.join(routed[name] + ': ' + message + '\n' for message in MESSAGES), 0
    if index == 7:
        return (('true' if name == 'yes' else 'false' if name == 'no' else 'unsure') + '\n') * 3, 0
    if index == 8:
        return (routed[name] + '\n') * 3, 0
    if index == 9:
        route = routed[name] + (': 0.5' if name == 'unsure' else ': backend.status' if failed else '')
        return ''.join(route + ': ' + message + '\n' for message in MESSAGES), 0
    if index in (10, 11):
        if failed:
            return ''.join('try again later: ' + message + ' (' + FAILURE + ')\n' for message in MESSAGES), 0
        return (''.join('refund queue: ' + message + '\n' for message in MESSAGES) if name == 'yes' else ''), 0
    if index == 12:
        return ('billing team\nbilling\nshipping\n' if name == 'yes' else ''), 0
    if index == 13:
        return ('answer today\n1.00\n' if name == 'yes' else '0.00\n'), 0
    if index == 14:
        if name == 'rank-example':
            _, rows = rank_fixture()
            return ''.join(str(row['answer'])+' '+MESSAGES[row['position']-1]+'\n' for row in rows), 0
        return ''.join(str(i)+' '+message+'\n' for i,message in enumerate(MESSAGES,1)), 0
    if index == 15:
        return {'yes':'Please refund my order.\n','no':'no line answers\n','unsure':'a person reads it\n'}[name], 0
    if index in (16, 17):
        return (('person Morgan\norganization Example Group\n' if index == 16 else 'Morgan works_for Example Group\n') if name == 'relations' else ''), 0
    if index == 18:
        if failed:
            return ''.join('try again later: '+message+'\n' for message in MESSAGES), 0
        if 'unsure' in name:
            return ''.join('a person reads it: '+message+'\n' for message in MESSAGES), 0
        return ('refund queue: billing 1.00\n'*3 if name=='yes' else ''), 0
    if index == 19:
        predicate = 'false' if name == 'no' else 'true'
        topic = 'shipping' if name == 'no' else 'unsure' if name == 'form-topic-unsure' else 'billing'
        return f'refund={predicate}\ttopic={topic}\turgency=1\n', 0
    if index == 20:
        return None, 0
    if index == 21:
        word = 'false' if name=='no' else FAILURE if failed else 'true'
        return ''.join(word+' '+message+'\n' for message in MESSAGES[:2]), 0
    if index in (22, 24):
        return ('true' if name=='yes' else 'unsure' if index==22 and name=='unsure' else 'false')+'\n', 0
    if index == 23:
        return ('billing' if name=='yes' else 'unsure')+'\n', 0
    if index == 25:
        return '1 Some(0) Some(0)\n', 0
    if index == 26:
        return 'local.cancelled\n', 0
    raise AssertionError('unaccounted Rust example '+str(index))


def refusal_sources():
    prelude = 'use engine::*;\n' + ENUMS
    row = 'let engine=Engine::from_env().unwrap();let row=engine.run(&Choose::<Topic>::new("Topic?").unwrap(),"body").unwrap();'
    return {
        'if-result': (row+'if row {}', r'E0308'),
        'bare-answer-equality': (row+'let _=row==Topic::Billing;', r'E0308|E0277'),
        'filter-band': ('let _=Filter::new("Keep?").unwrap().threshold(Threshold::band(0.2,0.8).unwrap());', r'E0308'),
        'one-record-list': ('let engine=Engine::from_env().unwrap();let _=engine.run(&Decide::new("Ready?").unwrap(),vec!["body"]);',r'E0277'),
        'one-record-filter': ('let engine=Engine::from_env().unwrap();let _=engine.run(&Filter::new("Keep?").unwrap(),"body");',r'E0277'),
        'many-record-filter': ('let engine=Engine::from_env().unwrap();let _=engine.rows(&Filter::new("Keep?").unwrap(),vec!["body"]);',r'E0277'),
        'many-record-rank': ('let engine=Engine::from_env().unwrap();let _=engine.rows(&Rank::<Urgency>::new("Urgent?").unwrap(),vec!["body"]);',r'E0277'),
        'form-filter': ('let _=AnnotatableQuestion::from(Filter::new("Keep?").unwrap());',r'E0277'),
        'missing-choice-arm': ('let topic=Topic::Billing;match topic { Topic::Billing=>(),Topic::Shipping=>() }',r'E0004'),
        'missing-state-arm': ('let engine=Engine::from_env().unwrap();let row=engine.run(&Decide::new("Ready?").unwrap(),"body").unwrap();match row.answer(){Ok(_)=>(),Err(NoAnswer::Unsure{..})=>()}',r'E0004'),
        'borrow-after-row-drop': ('let engine=Engine::from_env().unwrap();let row=engine.run(&Choose::with_strings("Topic?",["one","two"]).unwrap(),"body").unwrap();let text=row.answer().unwrap();drop(row);println!("{text}");',r'E0505'),
        'private-projection': ('fn forge<F:engine::results::Function>(row:engine::results::AnyRow){let _=F::project(row);}',r'E0061|E0624'),
        'no-async-one-record': ('let _=async {let engine=Engine::from_env().unwrap();let _=engine.run(&Decide::new("Ready?").unwrap(),"body").await;};',r'E0277'),
        'stream-is-not-iterator': ('let engine=Engine::from_env().unwrap();for _ in engine.rows(&Decide::new("Ready?").unwrap(),vec!["body"]).unwrap().into_stream(){}',r'E0277'),
        'result-is-not-hashable': (row+'let mut rows=std::collections::HashSet::new();rows.insert(row);',r'E0277|E0599'),
        'uncertainty-is-not-call-error': ('fn read(row:Row<Decide>)->Result<bool,Error>{Ok(row.answer()?)}',r'E0277'),
    }, prelude


def run_rust(command=None, negative=None):
    examples = blocks(IDEAL / 'rust.md')
    assert len(examples)==27 and all(language=='rust' for language,_,_ in examples), 'Rust guide inventory changed'
    scripted = cases()
    scripted["rank-example"]=rank_fixture()[0]
    with consumer(program(examples)) as (project, build_env, executable):
        for index,(_,_,line) in enumerate(examples):
            names = ['yes', 'failed']
            if index in (0, 10, 11): names = ['yes', 'no', 'failed']
            if index in (1, 2, 3, 4, 6, 7, 8, 9, 15, 21, 22): names = ['yes', 'no', 'unsure', 'failed']
            if index == 5: names = ['yes', 'no', 'unsure', 'failed', 'other']
            if index == 12: names = ['yes', 'empty', 'failed']
            if index == 13: names = ['yes', 'low-score', 'failed']
            if index == 14: names = ['yes', 'rank-example', 'failed']
            if index in (16, 17): names = ['relations', 'empty', 'failed']
            if index in (18, 19): names = ['yes', 'no', 'failed', 'form-refund-failed', 'form-topic-failed', 'form-urgency-failed', 'form-topic-unsure']
            if index == 23: names = ['yes', 'unsure', 'failed']
            if index in (20, 25, 26): names = ['yes']
            for name in names:
                with scenario(name.removeprefix('form-') if name not in scripted else 'yes',case=({'replies':[dict(probability=.05)]} if index==12 and name=='empty' else scripted.get(name))) as (folder,env):
                    (folder/'toolbox.json').write_text(json.dumps({'entries':{'topic':{'question':{'function':'choose','question':'What is the message about?','choices':['billing','shipping','other'],'field':'/text'}}}}))
                    result=subprocess.run([str(executable),str(index)],cwd=folder,env=env,capture_output=True,text=True,timeout=60)
                    output,status=expected(index,name)
                    label=f'rust.md:{line}/{name}'
                    kwargs=dict(output=output,exit_code=status)
                    if index in (0,1,2,3,5,6,8,9,18,19) and output:
                        kwargs['branch']=output.splitlines()[0]
                    if status: kwargs['refusal']=r'backend\.(status|reply)'
                    check(label,result,negative=negative if index==0 and name=='yes' and negative in ('output','branch','exit') else None,**kwargs)
                    if index==20:
                        refund,topic=json.loads(result.stdout)
                        assert 'yes_means' in refund and 'no_means' in refund
                        assert topic['choices'][0]['description']=='charges, invoices and refunds'
        with scenario('yes') as (folder,env):
            env.pop('THINKTHEN_API_KEY')
            result=subprocess.run([str(executable),'0'],cwd=folder,env=env,capture_output=True,text=True,timeout=60)
            check('rust/runtime-refusal/missing-key',result,exit_code=1,refusal=r'usage.key-missing')
        sources,prelude=refusal_sources()
        for label,(body,diagnostic) in sources.items():
            source=prelude+'\nfn main(){'+body+'}\n'
            if negative=='refusal' and label=='filter-band': source='fn main(){}'
            result=compile_refusal(project,build_env,source)
            check('rust/compiler-refusal/'+label,result,exit_code=101,refusal=diagnostic)


if __name__=='__main__':
    try: run_rust()
    except (AssertionError,RuntimeError,OSError) as error:
        print('FAIL '+str(error),file=sys.stderr);sys.exit(1)
