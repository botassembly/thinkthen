"""Run every JSON service-guide block against the public engine and row codec."""
import json
import subprocess
import sys

from guide_examples import IDEAL, ROOT, blocks, check, scenario
from bash_guide_examples import cases
from rust_guide_support import consumer, rank_fixture

def classify(value):
    """Account for every published JSON example by its native wire shape."""
    if not isinstance(value, dict):
        raise AssertionError('service example must be an object')
    if 'question' in value and 'records' in value:
        return 'request'
    if 'function' in value:
        return 'row' if 'status' in value else 'question'
    if set(value) == {'code', 'message'}:
        return 'error'
    if set(value) == {'entries'}:
        return 'toolbox'
    if 'backend' in value and set(value) <= {'backend', 'model', 'batch', 'retry'}:
        return 'settings'
    raise AssertionError('unclassified public service example: ' + repr(value))


def values(source):
    decoder=json.JSONDecoder()
    rest=source.strip()
    result=[]
    while rest:
        value,end=decoder.raw_decode(rest)
        result.append(value)
        rest=rest[end:].lstrip()
    return result


def invoke(executable, mode, value, folder, env):
    return subprocess.run([str(executable),mode],input=json.dumps(value),cwd=folder,env=env,
                          capture_output=True,text=True,timeout=60)


def fields(row):
    status=row['status']
    assert ('answer' in row)==(status=='answered'),row
    assert ('error' in row)==(status=='failed'),row
    if status=='failed':
        assert 'probability' not in row and 'detail' not in row,row
        assert row['error']['code'].startswith(('record.','backend.')),row
    elif status=='unsure':
        assert row['function'] in ('decide','choose','find') and 'probability' in row,row
    else:
        assert ('probability' in row)==(row['function'] in ('decide','filter','choose','find')),row
        if row['function']!='find': assert row['answer'] is not None,row
        if row['function']=='annotate':
            for child in row['answer'].values():
                fields(child)
                assert not set(child)&{'position','id','source'},child
    assert all(value is not None for key,value in row.items() if key!='answer'),row


def check_request(request, name, rows):
    assert len(rows)==len(request['records']),rows
    assert sorted(row['position'] for row in rows)==list(range(1,len(rows)+1)),rows
    function=request['question']['function']
    failed=name=='failed'
    for row in rows:
        assert row['function']==function and 'source' in row,row
        fields(row)
        if failed:
            if function=='annotate':
                assert row['status']=='answered' and set(row['answer'])=={'refund','topic','urgency'},row
                for child in row['answer'].values():
                    assert child['status']=='failed' and child['error']['code']=='backend.status',row
            else:
                assert row['status']=='failed' and row['error']['code']=='backend.status',row
            continue
        if name.startswith('form-'):
            target=name.removeprefix('form-').rsplit('-',1)[0]
            expected='failed' if name.endswith('-failed') else 'unsure'
            assert row['status']=='answered',row
            for child_name,child in row['answer'].items():
                assert child['status']==(expected if child_name==target else 'answered'),row
            continue
        if name=='unsure':
            assert row['status']=='unsure',row
            continue
        assert row['status']=='answered',row
        answer=row['answer']
        if function in ('decide','filter'): assert answer is (name=='yes'),row
        elif function=='choose': assert answer==('billing' if name=='yes' else 'shipping'),row
        elif function=='tag': assert answer==(['billing','shipping'] if name=='yes' else []),row
        elif function=='score': assert answer==(0.0 if name=='low-score' else 1.0),row
        elif function=='rank':
            expected={entry['position']:entry['answer'] for entry in rank_fixture()[1]} if name=='rank-example' else {}
            assert answer==expected.get(row['position'],row['position']),row
        elif function=='find':
            assert answer==({'text':'Please refund my order.','start':32,'end':55} if name=='yes' else None),row
        elif function=='recognize': assert [value['text'] for value in answer]==(['Morgan','Example Group'] if name=='relations' else []),row
        elif function=='relate': assert [value['relation'] for value in answer]==(['works_for'] if name=='relations' else []),row
        elif function=='annotate': assert set(answer)=={'refund','topic','urgency'},row
    for row in rows:
        record=request['records'][row['position']-1]
        if isinstance(record,dict) and 'id' in record:
            assert row['id']==record['id'],rows
    assert all('item' not in row for row in rows),rows


def run_service(command=None, negative=None):
    examples=blocks(IDEAL/'service.md')
    assert examples and all(language=='json' for language,_,_ in examples),'service guide JSON inventory changed'
    requests=[value for _,body,_ in examples for value in values(body) if classify(value)=='request']
    assert len(requests)==13,'account for each public service request'
    codec=json.loads((ROOT/'construction/service-guide-codec.json').read_text())
    answered=next(entry['value'] for entry in codec if entry['mode']=='row' and entry['value'].get('status')=='answered')
    source=(ROOT/'scripts/service-guide-consumer.rs').read_text()
    scripted=cases()
    scripted["rank-example"]=rank_fixture()[0]
    base_source=answered['source']
    with consumer(source) as (_,_,executable):
        for index,(_,body,line) in enumerate(examples):
            entries=values(body)
            mode=classify(entries[0])
            assert all(classify(entry)==mode for entry in entries),'mixed JSON example roles'
            if mode=='request':
                assert len(entries)==1,'one request per public example'
                request=entries[0]
                function=request['question']['function']
                names=['yes','no','failed']
                if function in ('choose','find') or isinstance(request['question'].get('threshold'),dict): names.append('unsure')
                if function=='score': names=['yes','low-score','failed']
                if function=='rank': names=['yes','rank-example','failed']
                if function in ('recognize','relate'): names=['relations','empty','failed']
                if function=='annotate': names=['yes','failed','form-refund-failed','form-topic-failed','form-urgency-failed','form-topic-unsure']
                for name in names:
                    with scenario(name.removeprefix('form-') if name not in scripted else 'yes',case=scripted.get(name)) as (folder,env):
                        result=invoke(executable,'request',request,folder,env)
                        check(f'service.md:{line}/{name}',result,negative=negative if index==0 and name=='yes' and negative in ('output','branch','exit') else None)
                        rows=json.loads(result.stdout)
                        check_request(request,name,rows)
                        if name=='rank-example':
                            assert [{key:value for key,value in row.items() if key!='source'} for row in rows]==rank_fixture()[1],rows
            else:
                for number,value in enumerate(entries):
                    with scenario('yes') as (folder,env):
                        result=invoke(executable,mode,value,folder,env)
                        output=json.loads(result.stdout) if result.returncode==0 else None
                        if mode in ('row','error'):
                            check(f'service.md:{line}/saved-{number}',result,output=None)
                            assert output==value,(value,output)
                            if mode=='row': fields(output)
                        else:
                            check(f'service.md:{line}/{mode}-{number}',result)
                            assert isinstance(output,dict),output
        for number,entry in enumerate(codec):
            value=entry['value']
            if entry['mode']=='row': value={**value,'source':value.get('source',base_source)}
            with scenario('yes') as (folder,env):
                result=invoke(executable,entry['mode'],value,folder,env)
                check('service/private-codec/'+str(number),result)
                assert json.loads(result.stdout)==value
        request=requests[0]
        refusal_requests=[
            ('filter-band',dict(question=dict(function='filter',question='Ready?',threshold=dict(low=.2,high=.8)),records=['body']),'usage.threshold'),
            ('form-filter',dict(question=dict(function='annotate',questions=dict(refund=dict(function='filter',question='Ready?'))),records=['body']),'usage.question'),
            ('unknown-request-key',{**request,'extra':True},'usage.question'),
            ('zero-threshold',dict(question=dict(function='decide',question='Ready?',threshold=0),records=['body']),'usage.threshold'),
            ('reversed-band',dict(question=dict(function='decide',question='Ready?',threshold=dict(low=.8,high=.2)),records=['body']),'usage.threshold'),
            ('old-meaning-key',dict(question=dict(function='decide',question='Ready?',true='yes'),records=['body']),'usage.question'),
        ]
        for label,value,code in refusal_requests:
            observer=[]
            with scenario('yes',observer=observer) as (folder,env):
                result=invoke(executable,'request',request if negative=='refusal' and label=='filter-band' else value,folder,env)
                check('service/request-refusal/'+label,result,exit_code=1,refusal=(code+r'.*decide' if label=='filter-band' else code))
                observer[0].stdin.write('count\n'); observer[0].stdin.flush()
                assert observer[0].stdout.readline().strip()=='0',label
        # Every failure in the record table remains a row, with nothing sent for it.
        record_cases=[('', 'record.empty'),('  ','record.empty'),({},'record.empty'),([], 'record.empty'),({'id':'only'},'record.empty'),(None,'record.missing'),({'id':None},'record.invalid'),({'id':1.5},'record.invalid'),({'id':True},'record.invalid'),({'id':{}},'record.invalid'),({'id':[]},'record.invalid')]
        for record,code in record_cases:
            observer=[]
            with scenario('yes',observer=observer) as (folder,env):
                result=invoke(executable,'request',{**request,'records':[record]},folder,env)
                check('service/record/'+code,result)
                rows=json.loads(result.stdout)
                assert len(rows)==1 and rows[0]['status']=='failed' and rows[0]['error']['code']==code,rows
                observer[0].stdin.write('count\n'); observer[0].stdin.flush()
                assert observer[0].stdout.readline().strip()=='0',code
        with scenario('yes') as (folder,env):
            value={**request,'records':[{'id':18446744073709551615,'text':'body'}]}
            result=invoke(executable,'request',value,folder,env)
            check('service/exact-unsigned-id',result)
            assert json.loads(result.stdout)[0]['id']==18446744073709551615
        with scenario('yes') as (folder,env):
            value={**request,'question':{**request['question'],'field':'/text'},'records':[{'other':'body'},{'text':None}]}
            result=invoke(executable,'request',value,folder,env)
            check('service/missing-selected-field',result)
            assert [row['error']['code'] for row in json.loads(result.stdout)]==['record.missing','record.missing']
        with scenario('yes') as (folder,env):
            value=next(value for value in requests if value['question']['function']=='rank')
            value=json.loads(json.dumps(value))
            value['question']['top']=1
            value['records']=['first',None,'last']
            result=invoke(executable,'request',value,folder,env)
            check('service/rank-top-preserves-failures',result)
            rows=json.loads(result.stdout)
            assert [(row['position'],row['status']) for row in rows]==[(1,'answered'),(2,'failed')],rows
        observer=[]
        with scenario('yes',observer=observer) as (folder,env):
            result=invoke(executable,'request',{**request,'records':[]},folder,env)
            check('service/empty-records',result,output='[]\n')
            observer[0].stdin.write('count\n'); observer[0].stdin.flush()
            assert observer[0].stdout.readline().strip()=='0'
        with scenario('yes') as (folder,env):
            result=invoke(executable,'request',{**request,'name':'saved-run'},folder,env)
            check('service/request-name',result)
            assert json.loads(result.stdout)[0]['source']['name']=='saved-run'
            result=invoke(executable,'settings',{'api_key':'synthetic-loopback-key'},folder,env)
            check('service/settings-never-save-key',result)
            assert 'api_key' not in json.loads(result.stdout) and 'synthetic-loopback-key' not in result.stdout
        # Absent answers, nulls, impossible uncertainty and row/call codes are enforced by the same codec.
        invalid=[{key:value for key,value in answered.items() if key!='answer'},
                 {**answered,'answer':None},{**answered,'id':None},
                 {**answered,'probability':None},
                 {**answered,'status':'unsure','answer':False},
                 {'function':'score','position':1,'source':base_source,'status':'unsure','probability':.5},
                 {'function':'decide','position':1,'source':base_source,'status':'failed','error':dict(code='usage.question',message='bad call')}]
        for number,value in enumerate(invalid):
            with scenario('yes') as (folder,env):
                result=invoke(executable,'row',value,folder,env)
                check('service/row-refusal/'+str(number),result,exit_code=1,refusal=r'.+')


if __name__=='__main__':
    try: run_service()
    except (AssertionError,RuntimeError,OSError) as error:
        print('FAIL '+str(error),file=sys.stderr);sys.exit(1)
