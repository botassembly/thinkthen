"""Shared cases through installed owned C++ calls; full inventory is release-only."""
import base64
import json
import os
from pathlib import Path
import sqlite3
import subprocess
import sys
import tempfile
ROOT=Path(__file__).resolve().parents[3]
sys.path[:0]=[str(ROOT/'conformance/children'),str(ROOT/'conformance')]
from children import child_env
import parity
import c_parity as shared
from session_projection import project_session
CONSUMER='cpp'

def descriptor(v,home):
    definition=dict(v['question'])
    if 'questions' not in definition:definition.update(v.get('metadata',{}))
    loader=v.get('loader')
    if loader:
        kind={'file':'file','load':'file','named':'name','load_named':'name','reference':'reference','load_reference':'reference'}[loader]
        question={'kind':kind,{'file':'path','name':'name','reference':'reference'}[kind]:v['reference']}
    elif v.get('question_form')=='file':question={'kind':'file','path':'fixture-question.json'}
    elif v.get('raw') is not None:
        (home/'raw-question.json').write_text(v['raw']);question={'kind':'file','path':'raw-question.json'}
    else:question={'kind':'definition','value':definition}
    injection=(v.get('operation') or {}).get('injection')
    if v.get('paths') or injection=='recording_read_failure' or v.get('question_form')=='file':
        source={'paths':[path if v.get('owned_jsonl') else str(ROOT/path) for path in (v.get('paths') or ['target/missing-input'])],
                'reading':{'unit':{1:'line',2:'window',5:'line'}.get(v.get('source_unit',3),'file')}}
        if v.get('window'):source['reading']['window']=v['window']
        if v.get('owned_jsonl') or v.get('source_unit')==5:source['framing']='jsonl'
        if v.get('image_reader') or v.get('source_unit')==4:source['media']='image'
        input={'kind':'source','source':source}
    else:
        images=[{'kind':'bytes','bytes':base64.b64encode((ROOT/path).read_bytes()).decode(),'media':v.get('media','image/png')} for path in v.get('image_paths',[])]
        items=[]
        for i,value in enumerate(v['items']):
            item={}
            if not v.get('image_only'):
                if v.get('caption_files'):value=(home/('caption-%d.txt'%i)).read_text()
                text=v.get('text') and isinstance(value,str) or v.get('caption_files')
                item['original']={'kind':'text' if text else 'json','text' if text else 'value':value}
            if images:item['images']=images
            if v.get('contexts') is not None:item['context']=v['contexts'][i]
            elif v.get('context_present'):item['context']=v['context']
            if v.get('candidate_orders') is not None:item['options']=[{'name':name} for name in v['candidate_orders'][i]]
            items.append(item)
        input={'kind':{'find':'units','relate':'entities'}.get(v['verb'],'records'),'items':items}
    options={'attempts':True}
    if v['verb']=='find':
        options['none']=definition.pop('none',False)
    if injection=='expired_deadline':options['deadline_ms']=0
    if v.get('shared_context') is not None:options['context']=v['shared_context']
    # A null/non-context value is original JSON data, not a valid ContextSchema
    # descriptor. Select it through the shared native reading rule to test the
    # declaration's exact refusal, preserving the canonical assertion.
    if input['kind'] != 'source' and any('context' in item and not isinstance(item['context'], (str,dict)) for item in input['items']):
        for item in input['items']:
            original=item['original'].get('value',item['original'].get('text'))
            item['original']={'kind':'json','value':{'item':original,'context':item.pop('context')}}
        options.update(field=['/item'],context_field='/context')
    return {'verb':v['verb'],'question':question,'input':input,'options':options,'cancel':injection=='cancel_token','held_cancel':v.get('held_cancel',False)}

def native_cases(binary, *, consumer=CONSUMER, invoke=None):
    CONSUMER = consumer
    inventory=parity.inventory(); rows=list(parity.required_cases(inventory,CONSUMER).values())
    if os.environ.get('THINKTHEN_' + CONSUMER.upper() + '_CASES'):
        selected=set(os.environ['THINKTHEN_' + CONSUMER.upper() + '_CASES'].split(','))
        assert selected <= {row['id'] for row in rows}, 'unknown requested C++ case'
        rows=[row for row in rows if row['id'] in selected]
    elif os.environ.get('THINKTHEN_TEST_PROFILE', 'routine') != 'full':
        selected = set((ROOT/'conformance/routine-ids.txt').read_text().splitlines())
        selected.update(('complete-decide','images-decide','image-file-decide','images-choose','images-score','cache-hit-cannot-bypass-declaration','settings-replay-answers-from-the-folder-alone','settings-cache-off-sends-again','recognize-context-cache-replay'))
        rows = [row for row in rows if row['id'] in selected]
    cases={r['id']:r for r in json.loads((ROOT/'conformance/cases.json').read_text())['cases']}
    named={r['id']:r for r in json.loads((ROOT/'conformance/named-inputs.json').read_text())['cases']}
    required = {row['id'] for row in rows}
    rows += [{**row,'id':'native-filter-first-excluded'} for row in rows if row['id']=='15-rank-records']
    rows += [{**row,'id':'native-duplicate-row-indices'} for row in rows if row['id']=='complete-decide']
    failed=0
    for at,row in enumerate(rows):
        error=None
        try:
            original={'native-filter-first-excluded':'15-rank-records','native-duplicate-row-indices':'complete-decide'}.get(row['id'],row['id'])
            value=shared.document({**row,'id':original},cases,named)
            # Go's sealed generated definition types cannot carry arbitrary authored
            # JSON. Its public file selector preserves those bytes and reports Local
            # (question-file.md); keep the diagnostic and zero-send assertions.
            if CONSUMER == 'go' and original in (
                '29-usage-json-text', '31-usage-rank-blank-question',
                'declaration-shorthand', 'declaration-null', 'declaration-empty',
                'declaration-nested', 'declaration-unknown-keyword',
                'declaration-required-unknown', 'declaration-duplicate-required',
                'wording-version-zero', 'wording-version-overflow', 'wording-version-string',
                'wording-version-boolean', 'wording-version-null',
                'author-name-blank', 'author-name-uppercase', 'author-name-leading-digit',
                'author-name-control', 'author-name-nonascii'):
                value['expect']={**value['expect'],'error':'local','requests_sent':0}
            if row['id']=='native-filter-first-excluded':
                value.update(verb='filter',question={**value['question'],'threshold':0.5},expect={'success':{'operation':{'indexes':[1,2]}}})
            if row['id']=='native-duplicate-row-indices':value['items'] *= 2
            with tempfile.TemporaryDirectory(prefix='thinkthen-'+CONSUMER+'-complete-') as folder:
                home=Path(folder); child=child_env(home=folder,
                          PATH=os.environ.get('PATH','/usr/bin:/bin'),
                          ASAN_OPTIONS='detect_leaks=1',
                          UBSAN_OPTIONS='halt_on_error=1')
                backend=shared.Backend(ROOT/'target/debug/conformance-backend',child)
                try:
                    child.update(THINKTHEN_BASE_URL='http://127.0.0.1:%d/%s'%(backend.port,value['arm']),THINKTHEN_API_KEY='sk-conformance-loopback',LIQUIDAI_API_KEY='sk-conformance-loopback',OPENROUTER_API_KEY='sk-conformance-loopback')
                    shared.prepare(home,value);identities=[]
                    for si,step in enumerate(value.get('steps',[value])):
                        if step.get('copy_store'):
                            (home/'refreshed').mkdir()
                            with sqlite3.connect(home/'saved/thinkthen.sqlite') as old,sqlite3.connect(home/'refreshed/thinkthen.sqlite') as new:old.backup(new)
                        if step.get('damage_store'):
                            with sqlite3.connect(home/'saved/thinkthen.sqlite') as db:db.execute("UPDATE answers SET answer='damaged fixture answer'")
                        if value.get('image_variants'):
                            backend.close();backend=shared.Backend(ROOT/'target/debug/conformance-backend',child)
                            child['THINKTHEN_BASE_URL']='http://127.0.0.1:%d/%s'%(backend.port,step['arm']);child['PERPLEXITY_API_KEY']='sk-conformance-loopback';shared.prepare(home,step)
                        settings={'cache':False,'model':'jev-latest' if 'steps' in value else 'jev-1.13.0','batch':1,'max_retries':0,**step.get('settings',{}),'base_url':child['THINKTHEN_BASE_URL']}
                        replacements={'$FOLDER':str(home/'saved'),'$REFRESH':str(home/'refreshed'),'$PROFILE':str(home/'profile.json')}
                        settings={k:replacements.get(v,v) if isinstance(v,str) else v for k,v in settings.items()}
                        if row['kind'] in ('images','image-location'):settings['record']=str(home/'recorded')
                        request=descriptor(step,home)
                        fixture=dict(request)
                        if CONSUMER in ('cpp','zig') and step.get('incremental') and request['input']['kind']!='source':
                            fixture={**request,'input':{'kind':'feed','name':'records'},'feed_items':request['input']['items']}
                        input_file=home/'consumer-input.json';input_file.write_text(shared.compact(fixture))
                        before=int(backend.read('count'));args=[str(binary),str(input_file),shared.compact(settings)]
                        if invoke is not None:
                            output=invoke(step,settings,child,home,backend)
                        elif step.get('held_cancel'):
                            process=subprocess.Popen(args,env=child,cwd=home,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
                            try:
                                assert backend.read('wait 1')=='wait 1';process.stdin.write('!');process.stdin.flush();assert process.stdout.readline()=='cancel-fired\n'
                                backend.process.stdin.write('release\n');backend.process.stdin.flush();stdout,stderr=process.communicate(timeout=60)
                                output=subprocess.CompletedProcess(args,process.returncode,stdout,stderr)
                            finally:
                                backend.process.stdin.write('release\n');backend.process.stdin.flush()
                                if process.poll() is None:process.kill();process.wait()
                        else:output=subprocess.run(args,env=child,cwd=home,capture_output=True,text=True,timeout=120)
                        assert output.returncode==0 and not output.stderr,(output.returncode,output.stderr)
                        assert 'sk-conformance-loopback' not in output.stdout+output.stderr, 'key leaked through public result or failure'
                        emitted=[project_session(json.loads(output.stdout),step)]
                        got=emitted[-1]
                        if step.get('incremental'):
                            got.setdefault('completed', [answer for partial in emitted[:-1] for answer in partial['rows']])
                            if step.get('owned_jsonl'):
                                bodies=json.loads(backend.read('capture'))['bodies']; request=json.loads(bodies[0]); expected={'q%d'%(i+1):{'type':'noul','instructions':'The text is %s. %s'%(shared.compact(item),step['question']['decide'])} for i,item in enumerate(step['items'][:2])};assert request['questions']==expected,request
                        if value.get('identity_steps'):identities.append(got)
                        if step.get('stored_answers')==0:
                            path=home/'saved/thinkthen.sqlite'
                            if path.exists():
                                with sqlite3.connect(path) as db:assert db.execute('SELECT count(*) FROM answers').fetchone()[0]==0
                        if value.get('image_variants'):
                            from c_images import assert_images
                            assert_images(step,got,json.loads(backend.read('capture'))['bodies'])
                        expected=step
                        if step.get('raw') is not None and request['question']['kind']=='file' and step['expect'].get('error')=='usage':
                            # Authored bytes use the public file selector: question-file.md
                            # assigns Local here, retaining the message and request count.
                            expected={**step,'expect':{**step['expect'],'error':'local'}}
                        shared.assertions(row,expected,got,int(backend.read('count'))-(before if step.get('count_delta') else 0))
                        if row['id']=='native-filter-first-excluded':
                            assert [r['index'] for r in got['rows']]==[1,2] and [r['value'] for r in got['rows']]==[True,True],got
                            assert int(backend.read('count'))==3,got
                        if row['id']=='native-duplicate-row-indices':
                            assert [r['index'] for r in got['rows']]==[0,1] and got['rows'][0]['input']==got['rows'][1]['input'],got
                            assert got['records']==2 and int(backend.read('count'))==2,got
                        if row['kind'] in ('images','image-location'):
                            before=int(backend.read('count'));replay={k:v for k,v in settings.items() if k!='record'};replay['replay']=str(home/'recorded')
                            repeated=invoke(step,replay,child,home,backend) if invoke is not None else subprocess.run([str(binary),str(input_file),shared.compact(replay)],env=child,cwd=home,capture_output=True,text=True,timeout=60)
                            assert repeated.returncode==0 and not repeated.stderr,repeated.stderr
                            saved=project_session(json.loads(repeated.stdout),step);shared.assertions(row,step,saved,int(backend.read('count')))
                            assert saved['requests_sent']==0 and int(backend.read('count'))==before and saved['rows'][0]['answer_id']==got['rows'][0]['answer_id'],saved
                    if value.get('identity_steps'):
                        ids=[x['rows'][0]['observation_ids'] for x in identities];assert ids[0]==ids[1]==ids[2] and ids[3]!=ids[0] and ids[4]==ids[3] and ids[5]==ids[0],ids
                        assert len({x['call_id'] for x in identities})==6
                        assert len({identities[i]['rows'][0]['answer_id'] for i in (0,1,2,5)})==1
                        assert identities[3]['rows'][0]['answer_id']==identities[4]['rows'][0]['answer_id']!=identities[0]['rows'][0]['answer_id']
                finally:backend.close()
        except (AssertionError,ValueError,KeyError,TypeError,subprocess.SubprocessError,OSError) as f:error=type(f).__name__+': '+str(f)
        if error:
            failed+=1;print(CONSUMER+' complete fixture '+row['id']+' failed: '+error,file=sys.stderr)
        print(('parity: ' if row['id'] in required else 'regression: ')+json.dumps({'consumer':CONSUMER,'case':row['id'],'checks':row.get('checks',['named','runtime']),'status':'fail' if error else 'pass'}),flush=True)
    print(CONSUMER+' complete shared cases: %d/%d passed'%(len(rows)-failed,len(rows)))
    if failed:raise SystemExit(1)

if __name__ == "__main__":
    native_cases(Path(sys.argv[1]))
