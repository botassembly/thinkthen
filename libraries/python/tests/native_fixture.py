"""Family fixture framing and projection into the retained shared assertions."""
import base64
import json
from pathlib import Path

VERBS=('decide','choose','tag','score','filter','rank','find','annotate','recognize','relate')
ERRORS={'usage':1,'backend':2,'deadline':3,'local':4,'cancelled':5,'defect':6}

def request(value, root, home, settings):
    verb=value['verb'];question=value.get('question',{})
    role='set' if 'questions' in question or verb=='annotate' else 'rank' if verb=='rank' else 'find' if verb=='find' else 'recognize' if verb=='recognize' else 'relate' if verb=='relate' else 'dynamic' if verb=='choose' and 'options' not in question else 'atomic'
    q={'role':role,'raw':value['raw']} if value.get('raw') else {'role':role,'body':question}
    if verb=='find' and question.get('none'):
        q['body']={k:v for k,v in question.items() if k!='none'};q['none']=True
    if value.get('question_form')=='file':q={'role':role,'path':str(home/'fixture-question.json')}
    if value.get('loader'):
        method={'load_named':'name','named':'name','load_reference':'reference','reference':'reference','load':'path','file':'path'}[value['loader']]
        q={'role':role,method:value['reference']}
    operation=value.get('operation') or {}
    if value.get('paths') or operation.get('injection')=='recording_read_failure':
        paths=[str(home/'missing-input')] if operation.get('injection')=='recording_read_failure' else [str(home/p) if value.get('owned_jsonl') else str(root/p) for p in value['paths']]
        unit=value.get('source_unit',1)
        reading={'unit':{1:'line',2:'window',3:'file',4:'file',5:'line'}[unit]}
        if unit==2:reading['window']=value['window']
        source={'kind':'files','paths':paths,'options':{'reading':reading,'media':'image' if unit==4 or value.get('image_reader') else 'text'},'jsonl':unit==5}
    else:
        images=[{'media':value.get('media','image/png'),'bytes':list((root/p).read_bytes())} for p in value.get('image_paths',[])]
        records=[]
        for at,item in enumerate(value['items']):
            if isinstance(value.get('caption_files'),list): item=(home/value['caption_files'][at]).read_text()
            row={'content':{'kind':'images'} if value.get('image_only') else {'kind':'text' if value.get('text') and isinstance(item,str) else 'json','value':item},'images':images}
            if value.get('context_present') or value.get('contexts') is not None:
                context=value['contexts'][at] if value.get('contexts') is not None else value['context']
                row['context']={'kind':'text' if isinstance(context,str) else 'json','value':context}
            if value.get('record_options') is not None:
                row['options']=[{'name':name,'description':description} for name,description in value['record_options'].items()]
            records.append(row)
        source={'kind':'records','records':records}
    if value.get('candidate_orders'):
        for at,row in enumerate(source['records']):row['options']=[{'name':n} for n in value['candidate_orders'][at]]
    return {'verb':verb,'question':q,'input':source,'settings':settings,'cancel':operation.get('injection')=='cancel_token', 'deadline_ms':0 if operation.get('injection')=='expired_deadline' else None, 'shared_context':value.get('shared_context'), 'held_cancel':bool(value.get('held_cancel')), 'incremental':bool(value.get('incremental')), 'batch_probe':bool(value.get('batch_probe'))}

def native_request(document):
    """Translate the existing caller fixture into canonical Request selectors."""
    q=document['question']
    selector=next(({'kind':kind,key:q[key]} for key,kind in
        [('path','file'),('name','name'),('reference','reference')] if key in q),None)
    if selector is None:
        selector={'kind':'definition','value':json.loads(q['raw']) if 'raw' in q else q['body']}
    source=document['input']
    if source['kind']=='files':
        if source.get('jsonl'):raise ValueError('native source JSONL requires its owning feed consumer')
        source={'kind':'source','source':{'paths':source['paths'],**source['options']}}
    else:
        items=[]
        for record in source['records']:
            content=record['content'];item={}
            if content['kind']!='images':
                item['original']={'kind':content['kind'],
                    'text' if content['kind']=='text' else 'value':content['value']}
            for key in ('context','options'):
                if key in record:
                    item[key]=record[key]['value'] if key=='context' else record[key]
            item['images']=[{'kind':'bytes','media':image['media'],
                'bytes':base64.b64encode(bytes(image['bytes'])).decode()} for image in record['images']]
            items.append(item)
        source={'kind':'records','items':items}
    options={'attempts':True}
    if q.get('none'):options['none']=True
    if document.get('shared_context') is not None:options['context']=document['shared_context']
    if document.get('deadline_ms') is not None:options['deadline_ms']=document['deadline_ms']
    return {**document,'question':selector,'input':source,'options':options}


def native_inputs(result, verb):
    """Read admitted originals and locations from actual complete documents."""
    if verb=='find':
        return [{'original':candidate['input'],'location':candidate.get('source',{}),'images':[]}
                for candidate in result['candidates'] if candidate['index'] is not None]
    if verb=='relate':
        sources={entry['index']:entry['source'] for entry in result.get('input_sources',[])}
        return [{'original':original,'location':sources.get(index,{}),'images':[]}
                for index,original in enumerate(result['input'])]
    return [{'original':result.get('input'),'location':result.get('source',{}),
             'images':result.get('images',[])}]

def project(packet,verb):
    if 'error' in packet:
        e=packet['error'];out={'code':ERRORS[e['kind']],'message':e['message']}
        if packet.get('facts'):out.update({k:packet['facts'][k] for k in ['requests_sent','records']})
        if packet.get('completed'):
            out['completed']=project(packet['completed'],verb)['rows']
        else:out['completed']=[]
        if e.get('stopped',{}).get('at') is not None:out['stopped_at']=e['stopped']['at']
        return out
    facts=packet['facts'];out={'code':0,'schema':'thinkthen.result/2','call_id':facts['call_id'],'requests_sent':facts['requests_sent'],'cache_answers':facts['cache_answers'],'rows':[]}
    out.update({k:facts[k] for k in ('records','input_tokens','output_tokens') if k in facts})
    results=packet['results'] if isinstance(packet['results'],list) else [packet['results']]
    for at,r in enumerate(results):
        meta=r['meta'];v=r['value'];answer=r.get('answer',{})
        row={'value':v,'index':r.get('index') if packet.get('native') else packet['ordinals'][at],'answer_id':r['answer_id'], 'origin':{'live':1,'cache':2,'replay':3,None:None}[meta['origin']], 'answered_by':meta.get('answered_by'), 'observations':len(meta['observations']),'sources':len(meta['question_sources']), 'observation_ids':[o.get('observation_id',o.get('failure_id')) for o in meta['observations']], 'input':r.get('input'), 'question_digest':meta.get('question_sha256'), 'cache_keys':meta['requests']}
        if verb=='decide' and isinstance(v,bool):row['value']=r['question'].get('true' if v else 'false',v)
        row.update(r.get('source',{}))
        if not packet.get('native') and row['index'] is not None:row.update(packet['inputs'][row['index']].get('location',{}))
        if 'probability' in answer:row['probability']=answer['probability']
        if 'probabilities' in answer:row['probabilities']=answer['probabilities']
        for key in ['file','first_line','last_line']: 
            if key in r:row[key]=r[key]
        row.update({k:r.get('question',{}).get(k) for k in ['name','wording_version'] if k in r.get('question',{})})
        row['member_authors']=[{k:a['question'][k] for k in ['name','wording_version'] if k in a['question']} for a in r.get('answers',{}).values()]
        inputs=native_inputs(r,verb) if packet.get('native') else packet['inputs'] if verb in ('find','relate') else [packet['inputs'][row['index']]]
        row['detail_inputs']=[{'input':i['original'],**i.get('location',{})} for i in inputs]
        images=[im for i in inputs for im in i['images']]
        if images:
            row['images']=[base64.b64decode(im['base64']).hex() for im in images]
            row['image_properties']=[[1 if im['media']=='image/jpeg' else 2,im['width'],im['height']] for im in images]
        if 'members' in r:
            row.update(question_name=r['question_name'],usage=meta['usage'],model=meta['model'],context_digest=meta.get('context_sha256'),source_batch_sizes=[s['batch_size'] for s in meta['question_sources']])
            row['members']=[]
            for member in r['members']:
                child=member['result'];cm=child['meta'];cq=child['question']
                projected={'name':member['name'],'value':child['value'],'answer_id':child['answer_id'],
                    'author':cq['name'],'probability':child['answer']['probability'],'usage':cm['usage'],
                    'model':cm['model'],'context_digest':cm.get('context_sha256'),
                    'source_batch_sizes':[s['batch_size'] for s in cm['question_sources']],
                    'observations':len(cm['observations']),'sources':len(cm['question_sources'])}
                if 'wording_version' in cq:projected['wording_version']=cq['wording_version']
                if 'source' in child:projected['source']=child['source']
                row['members'].append(projected)
        out['rows'].append(row)
    return out

def assert_required(packet,row,value,bodies,root):
    """Compare required known fields and independent selected-content expectations."""
    if 'error' in packet:
        if 'prefix_indexes' in value['expect']:
            completed=project(packet,value['verb'])
            assert [r['index'] for r in completed['completed']]==value['expect']['prefix_indexes'],completed
            assert completed['records']==len(completed['completed']),completed
            assert all(len(r['answer_id'])==64 and r['observations']==r['sources']>0 for r in completed['completed']),completed
        return
    results=packet['results'] if isinstance(packet['results'],list) else [packet['results']]
    expect=value['expect']
    for result in results:
        assert result['schema']=='thinkthen.result/2'
        paths=value.get('image_paths') or (value.get('paths') if value.get('source_unit')==4 else [])
        if paths:
            assert 'images' in result,'native canonical image propagation prerequisite is missing'
            assert [base64.b64decode(im['base64']) for im in result['images']]==[(root/path).read_bytes() for path in paths]
            dimensions=value.get('image_scenario',{}).get('construction',{}).get('dimensions',[1,1])
            assert [[im['width'],im['height']] for im in result['images']]==[dimensions]*len(paths)
            assert [im['media'] for im in result['images']]==[value.get('media','image/png')]*len(paths)
        assert len(result['answer_id'])==64 and len(packet['facts']['call_id'])==64
        questions=[a['question'] for a in result.get('answers',{}).values() if 'question' in a] or [result['question']]
        for question in questions:
            if value.get('unadorned'):assert 'name' not in question and 'wording_version' not in question
            for key,want in expect.get('resolved_metadata',{}).items():assert question[key]==want,(key,question,want)
            if 'ordered_properties' in expect:assert list(question['item_schema']['properties'])==expect['ordered_properties']
            if 'required_properties' in expect:assert question['item_schema']['required']==expect['required_properties']
        # Complete records retain caller originals; selected evidence is pinned below.
        if 'selected_item' in expect:
            original=value['items'][result['index']] if packet.get('native') else packet['inputs'][0]['original']
            assert result['input']==original,(result,original)
    if packet.get('native') and value['verb']=='find':
        for result in results:
            originals=native_inputs(result,'find')
            assert [item['original'] for item in originals]==value['items'],(originals,value['items'])
            if 'index' in expect:assert result['index']==expect['index'],result
    if 'capture_request' in expect:
        assert [json.loads(body) for body in bodies]==[expect['capture_request']],(bodies,expect)
    if 'per_item_context' in expect:
        want=expect['per_item_context']
        if want=='':want='Each question quotes the text it asks about.'
        assert bodies and all(json.loads(body)['state']==want for body in bodies),(bodies,expect)
    if 'selected_item' in expect:
        selected=json.dumps(expect['selected_item'],ensure_ascii=False,separators=(',',':'))
        if value['verb']=='annotate':selected=json.dumps(selected,ensure_ascii=False)
        # Native quoted instructions preserve the selected value, including false/null/Unicode.
        assert bodies and any(selected in q['instructions'] or json.loads(body)['state']==expect['selected_item'] for body in bodies for q in json.loads(body)['questions'].values()),(bodies,selected)
    if row['kind']=='located':
        if packet.get('native') and value['verb']=='relate':
            paths=[file for path in value['paths']
                for file in (sorted((root/path).rglob('*')) if (root/path).is_dir() else [root/path])
                if file.is_file()]
            for result in results:
                assert result['input']==[path.read_text() for path in paths],result
                assert [entry['index'] for entry in result['input_sources']]==list(range(len(paths))),result
                assert [entry['source']['file'] for entry in result['input_sources']]==[str(path) for path in paths],result
        for original in ([item for result in results for item in native_inputs(result,value['verb'])] if packet.get('native') else packet['inputs']):
            location=original['location'];path=Path(location['file'])
            assert original['original']==path.read_text()
            assert location['first_line']==1 and location['last_line']==len(path.read_text().splitlines())

def run(consumer, command, root, extra_env=None, settings_names=None, rust_manifest=None, typescript_compiler=None):
    import os, subprocess, sys, tempfile, sqlite3
    sys.path.insert(0,str(root/'conformance'))
    import parity,c_parity,c_images
    from children import child_env
    cases={v['id']:v for v in json.loads((root/'conformance/cases.json').read_text())['cases']}
    named={v['id']:v for v in json.loads((root/'conformance/named-inputs.json').read_text())['cases']}
    rows=list(parity.required_cases(parity.inventory(),consumer).values())
    selected=os.environ.get('THINKTHEN_CONFORMANCE_IDS')
    if selected:
        ids=Path(selected).read_text().splitlines() if Path(selected).is_file() else selected.split(',')
        rows=[r for r in rows if r['id'] in ids]
    # The static consumers compile their actual public accessors before executing cells.
    source=Path(command[-1])
    compiler=None
    if consumer in ('python','pandas','python-polars'):
        compiler=[sys.executable,'-m','mypy','--strict','--python-executable',command[0],str(source.with_name('native_types.py' if consumer=='python' else 'native_frame_types.py'))]
    elif consumer=='typescript':
        if typescript_compiler is None:raise ValueError('the TypeScript consumer needs its selected compiler')
        compiler=[command[0],str(typescript_compiler),'--strict','--module','NodeNext','--moduleResolution','NodeNext','--target','ES2022','--rootDir',str(source.parent),'--outDir',str(source.parent),str(source.with_suffix('.ts'))]
    elif consumer=='rust':
        compiler=['cargo','build','--locked','--offline','--manifest-path',str(rust_manifest or root/'libraries/python/Cargo.toml')]
        if rust_manifest is None:compiler+=['--example','native_case']
        else:compiler+=['--target-dir',str(root/'libraries/python/target')]
    if compiler:
        with tempfile.TemporaryDirectory(prefix='thinkthen-0431-types-') as tmp:
            env=child_env(keep=('CARGO_HOME','RUSTUP_HOME','CARGO_NET_OFFLINE','CARGO_BUILD_JOBS'), home=tmp, LANG='C.UTF-8', LC_ALL='C.UTF-8')
            env.setdefault('CARGO_HOME',str(Path.home()/'.cargo'))
            env.setdefault('RUSTUP_HOME',str(Path.home()/'.rustup'))
            env['CARGO_NET_OFFLINE']='true'
            env.update(extra_env or {})
            subprocess.run(compiler,cwd=source.parent,env=env,check=True)
    failures=[]
    for row in rows:
        try:
            value=c_parity.document(row,cases,named)
            if row['kind'] in ('typed-result','result2'):
                value['arm']='case/'+row['input']['case_ref']+'/v1'
                value['metadata_only']=False
            with tempfile.TemporaryDirectory(prefix='thinkthen-0431-') as tmp:
                home=Path(tmp)
                env=child_env(home=home, LANG='C.UTF-8', LC_ALL='C.UTF-8', **(extra_env or {}))
                backend=c_parity.Backend(root/'target/debug/conformance-backend',env)
                try:
                    env.update(THINKTHEN_API_KEY='sk-conformance-loopback', LIQUIDAI_API_KEY='sk-conformance-loopback',OPENROUTER_API_KEY='sk-conformance-loopback',PERPLEXITY_API_KEY='sk-conformance-loopback')
                    c_parity.prepare(home,value)
                    identities=[]
                    unadorned=None
                    steps=value.get('steps',[value])
                    batch_cases={'01-decide-yes-captured','06-choose-billing','09-tag-two','12-score-upper','13-filter-records','17-annotate-mixed'}
                    if row['id'] in batch_cases:
                        steps=[*steps,{**value,'incremental':True,'batch_probe':True,'count_delta':True},
                            {**value,'items':value['items'][:1],'incremental':True,'held_cancel':True,'arm':'arm/held/v1','override_arm':'arm/held/v1','settings':{**value.get('settings',{}),'timeout':1},'count_delta':True,'expect':{'error':'cancelled','requests_sent':1}}]
                    if consumer=='r' and row['id']=='01-decide-yes-captured':
                        bad=home/'invalid-utf8.txt';bad.write_bytes(b'\xff')
                        for incremental in (False,True):
                            steps.append({**value,'paths':[str(root/'specification/fixtures/files/documents/01-policy.txt'),
                                str(root/'specification/fixtures/files/documents/02-contract.txt'),str(bad)],
                                'source_unit':3,'incremental':incremental,'count_delta':True,'override_arm':'arm/full/capture/v1',
                                'expect':{'error':'usage','requests_sent':2,'prefix_indexes':[0,1]}})
                    if consumer=='r' and row['id']=='19-find-none':
                        steps.append({**value,'items':[None,'other'],'text':False,'count_delta':True,
                            'override_arm':'arm/full/capture/v1','expect':{'value':None,'index':0,'requests_sent':1}})
                    if consumer=='r' and row['id']=='files-relate':
                        for count in (1,2):
                            steps.append({**value,'paths':['specification/fixtures/files/documents/01-policy.txt']*count,
                                'zero_observations':True,'count_delta':True,'expect':{}})
                    if row['id']=='06-choose-billing':
                        steps.append({**value,'question':{k:v for k,v in value['question'].items() if k!='options'},
                            'record_options':dict.fromkeys(value['question']['options']),
                            'incremental':True,'batch_probe':True,'count_delta':True})
                    if value.get('metadata'):
                        def bare(question):
                            return {k:({n:bare(q) for n,q in v.items()} if k=='questions' else v) for k,v in question.items() if k not in ('name','wording_version')}
                        baseline={**value,'question':bare(value['question']),'unadorned':True,'count_delta':True,'settings':{**value.get('settings',{}),'record':'$FOLDER'},'arm':'arm/full/capture/v1','override_arm':'arm/full/capture/v1','metadata_only':True,'expect':{}}
                        baseline.pop('metadata',None)
                        steps=[*[{**step,'count_delta':True} for step in steps],baseline,{**value,'count_delta':True,'settings':{**value.get('settings',{}),'cache':'$FOLDER'},'arm':'arm/full/capture/v1','override_arm':'arm/full/capture/v1','metadata_only':True,'expect':{'requests_sent':0}}]
                    for original in steps:
                        step=dict(original)
                        if step.get('copy_store'):
                            (home/'refreshed').mkdir()
                            with sqlite3.connect(home/'saved/thinkthen.sqlite') as source,sqlite3.connect(home/'refreshed/thinkthen.sqlite') as target:source.backup(target)
                        if step.get('damage_store'):
                            with sqlite3.connect(home/'saved/thinkthen.sqlite') as db:db.execute("UPDATE answers SET answer='damaged fixture answer'")
                        if value.get('image_variants'):
                            backend.close();backend=c_parity.Backend(root/'target/debug/conformance-backend',env);c_parity.prepare(home,step)
                        settings={'cache':False,'batch':1,'max_retries':0,'model':'jev-latest' if 'steps' in value else 'jev-1.13.0',**step.get('settings',{})}
                        settings['base_url']=f'http://127.0.0.1:{backend.port}/{step.get("override_arm",step["arm"] if value.get("image_variants") else value["arm"])}'
                        for key,v in settings.items():
                            if isinstance(v,str) and v in ('$FOLDER','$REFRESH','$PROFILE'):settings[key]=str(home/{'$FOLDER':'saved','$REFRESH':'refreshed','$PROFILE':'profile.json'}[v])
                        if row['kind'] in ('images','image-location'):settings['record']=str(home/'recorded')
                        before=int(backend.read('count'))
                        def invoke(given):
                            invocation_count=int(backend.read('count'))
                            framed=request(step,root,home,given)
                            if consumer=='r':framed=native_request(framed)
                            if settings_names:framed['settings']={settings_names.get(k,k):v for k,v in given.items()}
                            if framed['batch_probe'] or (consumer=='r' and framed['held_cancel']):
                                child=subprocess.Popen(command,cwd=home,env=env,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
                                try:
                                    child.stdin.write(c_parity.compact(framed)+'\n');child.stdin.flush()
                                    if framed['batch_probe']:
                                        assert child.stdout.readline()=='ready\n'
                                        assert int(backend.read('count'))==before,'batch sent before its first pull'
                                        child.stdin.write('continue\n');child.stdin.close();child.stdin=None
                                        stdout,stderr=child.communicate(timeout=120)
                                    else:
                                        admission=f'wait {invocation_count+1}'
                                        assert backend.read(admission)==admission,'held cancellation requires an admitted request'
                                        stdout,stderr=child.communicate(input='continue\n',timeout=120)
                                    child=subprocess.CompletedProcess(command,child.returncode,stdout,stderr)
                                except BaseException:
                                    child.kill();child.wait();raise
                            else:
                                child=subprocess.run(command,input=c_parity.compact(framed)+'\n',cwd=home,env=env,capture_output=True,text=True,timeout=120)
                            assert child.returncode==0,(child.returncode,child.stdout[-1000:],child.stderr[-1600:])
                            assert not child.stderr,child.stderr
                            assert 'sk-conformance-loopback' not in child.stdout
                            packet=json.loads(child.stdout)
                            if packet.get('facts') is not None:assert packet['facts']['requests_sent']==int(backend.read('count'))-invocation_count,(packet,invocation_count)
                            needs_body=any(key in step['expect'] for key in ('selected_item','per_item_context','capture_request'))
                            bodies=json.loads(backend.read('capture'))['bodies'] if needs_body else []
                            assert_required(packet,row,step,bodies,root)
                            return project(packet,step['verb'])
                        got=invoke(settings)
                        count=int(backend.read('count'))
                        c_parity.assertions(row,step,got,count-before if step.get('count_delta') else count)
                        if step.get('unadorned'):unadorned=got
                        elif value.get('metadata') and unadorned is not None:
                            for before_row,after_row in zip(unadorned['rows'],got['rows'],strict=True):
                                for key in ('question_digest','cache_keys','observation_ids','answer_id'):assert before_row[key]==after_row[key],(key,before_row,after_row)
                        if value.get('identity_steps'):identities.append(got)
                        if step.get('stored_answers')==0:
                            path=home/'saved/thinkthen.sqlite'
                            if path.exists():
                                with sqlite3.connect(path) as db:assert db.execute('SELECT count(*) FROM answers').fetchone()[0]==0
                        if row['kind'] in ('images','image-location') and got['code']==0:
                            repeated=invoke({**{k:v for k,v in settings.items() if k!='record'},'cache':str(home/'recorded')})
                            assert repeated['code']==0,repeated
                            assert int(backend.read('count'))==count and repeated['rows'][0]['answer_id']==got['rows'][0]['answer_id']
                        if value.get('image_variants'):c_images.assert_images(step,got,json.loads(backend.read('capture'))['bodies'])
                        elif step.get('shared_context') is not None or 'request_items' in step['expect']:
                            bodies=json.loads(backend.read('capture'))['bodies']
                            if step.get('shared_context') is not None and step['verb']!='recognize':
                                want=step['context'] if step.get('context_present') else step['shared_context']
                                if want=='':want='Each question quotes the text it asks about.'
                                assert all(json.loads(b).get('state')==want for b in bodies),bodies
                            if 'request_items' in step['expect']:assert [json.loads(b)['records'] for b in bodies]==[step['expect']['request_items']],bodies
                    if identities:
                        first=identities[0]['rows'][0]
                        assert identities[1]['rows'][0]['answer_id']==identities[2]['rows'][0]['answer_id']==first['answer_id']
                        assert identities[4]['rows'][0]['answer_id']==identities[3]['rows'][0]['answer_id']
                        assert identities[5]['rows'][0]['answer_id']==first['answer_id']
                finally:backend.close()
            print('parity: '+json.dumps({'consumer':consumer,'case':row['id'],'checks':row.get('checks',['named','runtime']),'status':'pass'}),flush=True)
        except Exception as e:
            failures.append(row['id']);print('parity: '+json.dumps({'consumer':consumer,'case':row['id'],'checks':row.get('checks',['named','runtime']),'status':'fail'}),flush=True)
            print(row['id']+': '+repr(e)[:2200],file=sys.stderr,flush=True)
    return len(failures)

if __name__=='__main__':
    import os,sys
    root=Path(__file__).resolve().parents[3]
    library=os.environ.get('THINKTHEN_FRAME_LIBRARY')
    if library is None:
        failures=run('python',[sys.executable,str(root/'libraries/python/tests/native_case.py')],root,{'PYTHONPATH':str(root/'libraries/python')})
    else:
        consumer={'pandas':'pandas','polars':'python-polars'}[library]
        failures=run(consumer,[sys.executable,str(root/'libraries/python/tests/native_frame_case.py')],root,{'PYTHONPATH':str(root/'libraries/python'),'THINKTHEN_FRAME_LIBRARY':library})
    sys.exit(bool(failures))
