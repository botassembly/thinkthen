"""Owned offline counted loopback run for Objective-C. Runs never inherit ambient credentials."""
import datetime
import collections
import json
import os
import pathlib
import signal
import subprocess
import sys
import time
from backend import Backend, one_record

root = pathlib.Path(__file__).resolve().parent
stamp = datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ')
logs = root / 'logs' / ('run-' + stamp)
logs.mkdir(parents=True)
barrier = logs / 'barrier'
barrier.mkdir()
home = logs / 'home'
(home / 'cache').mkdir(parents=True)
backend = Backend(barrier)
env = {
    'PATH': '/usr/bin:/bin', 'HOME': str(home), 'XDG_CONFIG_HOME': str(home),
    'XDG_CACHE_HOME': str(home), 'LD_LIBRARY_PATH': str(root / 'target'),
    'THINKTHEN_BASE_URL': f'http://127.0.0.1:{backend.server_port}/generic/v1',
    'THINKTHEN_API_KEY': 'tt-canary-295', 'THINKTHEN_CACHE': str(home / 'cache'),
    'TT_BARRIER_DIR': str(barrier),
}
receipts = []
active = None

def stop(process):
    for sig in (signal.SIGTERM, signal.SIGKILL):
        try: os.killpg(process.pid, sig)
        except ProcessLookupError: pass
        if sig == signal.SIGTERM: time.sleep(.2)
    process.wait(timeout=5)

try:
    status = 'PASS'
    subprocess.run([str(root / 'target/nul_text')], env=env, cwd=root, check=True, timeout=30)
    assert not backend.arrivals, 'NUL result stub sent a request'
    for case, args in [('direct', []), ('basic', ['basic']), ('bulk', ['bulk']), ('reverse', ['reverse']), ('typed-json', ['typed-json']), ('boundaries', ['boundaries']), ('concurrent', ['concurrent']), ('strict', ['strict']), ('held', ['held'])]:
        with (logs / (case + '.log')).open('wb') as output:
            active = subprocess.Popen([str(root / 'target/direct' if case == 'direct' else root / 'target/main'), *args], env=env, cwd=root,
                                      stdout=output, stderr=subprocess.STDOUT, start_new_session=True)
            receipt = {'case': case, 'pid': active.pid, 'pgid': active.pid, 'timeout': 120}
            receipts.append(receipt)
            (logs / 'receipts.json').write_text(json.dumps(receipts, indent=2) + '\n')
            try: receipt['exit'] = active.wait(timeout=120)
            except subprocess.TimeoutExpired:
                stop(active)
                receipt['exit'] = 'timeout'
            active = None
            (logs / 'receipts.json').write_text(json.dumps(receipts, indent=2) + '\n')
            if receipt['exit'] != 0:
                status = 'FAIL:' + case
                break
    if status == 'PASS':
        # Pin 71f25087: records.md "Order and requests" and ADR 0055 carry each
        # distinct text once in its quoted question. These five batches are five
        # requests, not nine singleton requests and one or two held requests.
        packed_state = 'Each question quotes the text it asks about.'
        required = ['café','no','unsure','json-decide','choose','tag','score',
                    'annotate-one','Maria Chen','hold-scalar','recovery-scalar',
                    'hold-deadline','recovery-held','🧬','John Smith','failure-one',
                    'failure-two','success','a\u0000b','status-401','after-error',
                    'other-engine-error',packed_state,
                    '[{"id":"u001","evidence":"find-one"},{"id":"u002","evidence":"find-two"}]',
                    {'entities': [{'id': 'i1', 'name': 'First', 'kind': 'alert'},
                                  {'id': 'i2', 'name': 'Second', 'kind': 'alert'}]},
                    {'entities': [{'id': 'i1', 'name': 'Third', 'kind': 'alert'},
                                  {'id': 'i2', 'name': 'Fourth', 'kind': 'alert'}]}]
        key = lambda x: json.dumps(x, sort_keys=True, ensure_ascii=False)
        counts = {key(x): sum(key(y) == key(x) for y in backend.arrivals) for x in required}
        expected = {key(x): (5 if x == packed_state else 2 if x in ('Maria Chen', 'John Smith', 'failure-two') else 1) for x in required}
        packed = [tuple(q['instructions'] for q in request['questions'].values())
                  for request in backend.requests if one_record(request)['state'] == packed_state]
        expected_packed = [tuple('The text is ' + json.dumps(row) + '. Is it?' for row in rows)
                           for rows in (('filter-one','filter-two'),('rank-one','rank-two'),
                                        ('first','second','third'),('hold-reverse-1','hold-reverse-2'),
                                        ('hold-bulk-1','hold-bulk-2'))]
        if counts != expected or len(backend.arrivals) != 33 or sorted(packed) != sorted(expected_packed):
            status = 'COUNT_MISMATCH'
        else:
            golden = json.loads((root / 'expected-requests.json').read_text())
            identity = lambda items: collections.Counter(json.dumps(x, sort_keys=True, ensure_ascii=False, separators=(',', ':')) for x in items)
            if identity(backend.requests) != identity(golden):
                status = 'BODY_MISMATCH'
        if status == 'PASS' and ('OBJC_DIRECT_PASS' not in (logs / 'direct.log').read_text() or
              'STRICT_OBJC_CANCEL_PASS' not in (logs / 'strict.log').read_text() or
              'OBJC_HELD_PASS' not in (logs / 'held.log').read_text() or
              'OBJC_TYPED_JSON_PASS' not in (logs / 'typed-json.log').read_text() or
              'OBJC_CONCURRENT_PASS' not in (logs / 'concurrent.log').read_text() or
              'OBJC_BOUNDARIES_PASS' not in (logs / 'boundaries.log').read_text() or
              'OBJC_REVERSE_PASS' not in (logs / 'reverse.log').read_text()):
            status = 'STRICT_MARKER_MISSING'
    if status == 'PASS':
        overlap_barrier = logs / 'overlap-barrier'
        overlap_barrier.mkdir()
        overlap_backend = Backend(overlap_barrier)
        try:
            overlap_env = {**env, 'TT_BARRIER_DIR': str(overlap_barrier),
                           'THINKTHEN_BASE_URL': f'http://127.0.0.1:{overlap_backend.server_port}/generic/v1',
                           'THINKTHEN_CACHE': str(logs / 'overlap-cache')}
            completed = subprocess.run([str(root / 'target/main'), 'overlap'], env=overlap_env,
                                       cwd=root, capture_output=True, text=True, timeout=35)
            if (completed.returncode != 0 or 'OBJC_HELD_OVERLAP_PASS' not in completed.stdout or
                collections.Counter(overlap_backend.arrivals) != collections.Counter(['hold-overlap-a', 'hold-overlap-b']) or
                any(request['state'] != packed_state or
                    one_record(request)['questions'] != {'q1': {'instructions': 'Is it?', 'type': 'noul'}}
                    for request in overlap_backend.requests)):
                status = 'FAIL:overlap'
        finally:
            overlap_backend.close()
finally:
    if active is not None: stop(active)
    backend.close()
    (logs / 'arrivals.json').write_text(json.dumps(backend.arrivals, ensure_ascii=False, indent=2) + '\n')
    (logs / 'requests.json').write_text(json.dumps(backend.requests, ensure_ascii=False, indent=2) + '\n')
    outcome = {'status': status, 'log': str(logs), 'arrivals': len(backend.arrivals), 'receipts': receipts}
    (logs / 'outcome.json').write_text(json.dumps(outcome, indent=2) + '\n')
    print(json.dumps(outcome, indent=2))
if status != 'PASS': sys.exit(1)

from pathlib import Path
import subprocess, tempfile
ROOT = Path(__file__).resolve().parents[3]
CONSUMER = 'objective-c'
# Complete native public cases, using the shared input and assertion inventory.
import sqlite3
sys.path.insert(0, str(ROOT / 'conformance'))
import c_parity as shared
import parity

def native_content(v):
    return json.loads(v['data']) if v['kind'] == 2 else v['data']

def native_decision(v):
    return None if v['kind'] == 0 else bool(v['data']['boolean']) if v['kind'] == 1 else native_content(v['data']['authored'])

def native_member(v):
    k, d = v['kind'], v['data']
    return native_decision(d['decide']) if k == 1 else d['choose'] if k == 2 else d['tag'] if k == 3 else d['score']

def native_value(kind,v):
    if kind == 1: return native_decision(v)
    if kind == 5: return bool(v)
    if kind == 7: return native_content(v) if v is not None else None
    if kind == 9:
        result = {'entities':v['entities']}
        if v['relations'] is not None:
            result['relations'] = [{**{k:e[k] for k in ('relation','source','target','probability')}, **({'either':True} if e['either'] else {})} for e in v['relations']]
        return result
    if kind == 10:
        return [{**{k:e[k] for k in ('relation','source','target','probability')}, **({'either':True} if e['either'] else {})} for e in v]
    return v

def native_projection(raw):
    s=raw['summary']; facts=s['facts'] or {}; error=s['error']
    out={'code':error['code'] if error else 0}
    for field in ('requests_sent','records','cache_answers','call_id'):
        if field in facts: out[field]=facts[field]
    if error:
        out['message']=error['message']
        if error['stopped'] and error['stopped']['at'] is not None:out['stopped_at']=error['stopped']['at']
        return out
    out.update(schema=s['schema'],observations=s['observation_count'],rows=[])
    ordinals={}
    for event in raw['observations']:
        if event['kind'] == 2:
            r=event['data']['row']; v=r['data'][shared.FUNCTIONS[r['function']-1]]
            ordinals[v['common']['answer_id']]=r['index']
    for i,r in enumerate(raw['rows']):
        k=r['function']; v=r['data'][shared.FUNCTIONS[k-1]]; common=v['common']; meta=common['meta']
        value=v.get('value')
        if k == 8:
            causes=['','missing_answer','wrong_kind','missing_probability','invalid_probability','invalid_distribution','unexpected_probability']
            value={m['name']:native_member(m['data']['success']['value']) if m['state']==1 else {'failed':{'kind':'backend','cause':causes[m['data']['failure']['cause']]}} for m in v['answers']}
        else:value=native_value(k,value)
        row={'value':value,'answer_id':common['answer_id'],'index':v['index'] if k==7 else ordinals.get(common['answer_id'],0),
             'origin':meta['origin'],'answered_by':meta['answered_by'], 'observations':len(meta['observations']),'sources':len(meta['question_sources']),
             'observation_ids':[entry['data']['observation_id'] or entry['data']['failure_id'] for entry in meta['observations']],
             'input':native_content(common['input']) if common['input'] is not None else None}
        def location(obj, position):
            if position is not None:
                if position['file'] is not None:obj['file']=position['file']
                for key in ('first_line','last_line'):
                    if position[key] is not None:obj[key]=position[key]
        location(row, common['position'])
        if common['images'] is not None:
            row['images']=[bytes(image['bytes']).hex() for image in common['images']]
            row['image_properties']=[[image[x] for x in ('media','width','height')] for image in common['images']]
        if common['answer'] is not None:
            answer=common['answer']; ak=answer['kind']; data=answer['data']
            if ak==1:row['probability']=data['probability']
            else:
                values=data['tag'] if ak==3 else data[{2:'choice',4:'score',5:'find'}[ak]]['probabilities']
                row['probabilities']={p['name']:p['probability'] for p in values}
        row['detail_inputs']=[]
        for original in raw['details'][i]['inputs']:
            inp={'input':native_content(original['original']) if original['original'] is not None else None};location(inp,original['position']);row['detail_inputs'].append(inp)
        author=raw['authors'][i]
        for key in ('name','wording_version'):
            if author[key] is not None:row[key]=author[key]
        if k==8:row['member_authors']=[{key:author[key] for key in ('name','wording_version') if author[key] is not None} for author in raw['member_authors'][i]]
        # Validate copied complete fields in addition to the common value oracle.
        assert len(meta['requests']) == len(meta['question_sources']) == len(meta['observations']), meta
        assert len(common['answer_id']) == 64,common
        assert len(raw['observation_details']) == len(raw['observations']) == len(raw['observation_authors']),raw
        out['rows'].append(row)
    return out

def native_document(v):
    v=dict(v); verb=v['verb']; question=v['question']
    v['role']=shared.role(verb,question)
    v['question_json']=v.get('raw') if v.get('raw') is not None else shared.compact(question)
    v['find_none']=verb=='find' and bool(question.get('none'))
    if v['find_none']:
        v['find_text_literal']=isinstance(question['find'],str)
        v['find_text']=question['find'] if v['find_text_literal'] else shared.compact(question['find'])
    v['image_data']=[(ROOT/path).read_bytes().hex() for path in (v.get('image_paths',[]) if not v.get('paths') else [])]
    v['items_bytes']=[item if v.get('text') and isinstance(item,str) else shared.compact(item) for item in v['items']]
    v['items_kind']=[1 if v.get('text') and isinstance(item,str) else 2 for item in v['items']]
    if v.get('context_present'):v['context_bytes']=v['context'] if isinstance(v['context'],str) else shared.compact(v['context']);v['context_kind']=1 if isinstance(v['context'],str) else 2
    if v.get('shared_context') is not None:v['shared_context_bytes']=v['shared_context'] if isinstance(v['shared_context'],str) else shared.compact(v['shared_context']);v['shared_context_kind']=1 if isinstance(v['shared_context'],str) else 2
    v['media_code']=1 if v.get('media')=='image/jpeg' else 2
    if v.get('paths'):v['paths']=[path if v.get('owned_jsonl') else str(ROOT/path) for path in v['paths']]
    if (v.get('operation') or {}).get('injection')=='recording_read_failure':v.update(paths=[str(ROOT/'target/family0428-missing-input')],source_unit=3)
    return v

def native_cases(binary):
    inventory=parity.inventory(); rows=list(parity.required_cases(inventory,CONSUMER).values())
    cases={r['id']:r for r in json.loads((ROOT/'conformance/cases.json').read_text())['cases']}
    named={r['id']:r for r in json.loads((ROOT/'conformance/named-inputs.json').read_text())['cases']}
    failed=0
    for at,row in enumerate(rows):
        error=None
        try:
            value=shared.document(row,cases,named)
            with tempfile.TemporaryDirectory(prefix='thinkthen-'+CONSUMER+'-complete-') as folder:
                home=Path(folder); child={'PATH':os.environ.get('PATH','/usr/bin:/bin'),'HOME':folder,'XDG_CONFIG_HOME':str(home/'config'),'XDG_CACHE_HOME':str(home/'cache'),'XDG_STATE_HOME':str(home/'state'),'ASAN_OPTIONS':'detect_leaks=0','UBSAN_OPTIONS':'halt_on_error=1'}
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
                        input_file=home/'consumer-input.json';doc=native_document(step);doc['case_number']=10000+at*100+si if 'steps' in value else at;input_file.write_text(shared.compact(doc))
                        before=int(backend.read('count'));args=[str(binary),str(input_file),shared.compact(settings)]
                        if step.get('held_cancel'):
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
                        emitted=[native_projection(json.loads(line)) for line in output.stdout.splitlines()]
                        got=emitted[-1]
                        if step.get('incremental'):
                            got['completed']=[answer for partial in emitted[:-1] for answer in partial['rows']]
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
                        shared.assertions(row,step,got,int(backend.read('count'))-(before if step.get('count_delta') else 0))
                        if row['kind'] in ('images','image-location'):
                            before=int(backend.read('count'));replay={k:v for k,v in settings.items() if k!='record'};replay['replay']=str(home/'recorded')
                            repeated=subprocess.run([str(binary),str(input_file),shared.compact(replay)],env=child,cwd=home,capture_output=True,text=True,timeout=60)
                            assert repeated.returncode==0 and not repeated.stderr,repeated.stderr
                            saved=native_projection(json.loads(repeated.stdout));shared.assertions(row,step,saved,int(backend.read('count')))
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
        print('parity: '+json.dumps({'consumer':CONSUMER,'case':row['id'],'checks':row.get('checks',['named','runtime']),'status':'fail' if error else 'pass'}),flush=True)
    print(CONSUMER+' complete shared cases: %d/%d passed'%(len(rows)-failed,len(rows)))
    if failed:raise SystemExit(1)

def generated(at,v):
 s=shared.generated(at,v)
 s=s.replace('thinkthen_engine *e','TTClient *e').replace('thinkthen_question *q=NULL; thinkthen_source *source=NULL; thinkthen_result *result=NULL;','TTQuestion *q=nil; TTSource *source=nil; TTNativeResult *result=NULL, *failed=NULL;')
 s=s.replace('thinkthen_image *images','TTImage *images')
 # Distinct raw references are cloned by the source constructor.
 s=s.replace('char *caption_allocations', 'const thinkthen_image *raw_images[%d]={0}; char *caption_allocations'%max(1,len(v.get('image_paths',[]))))
 # balanced calls are parsed so nested counted compound literals remain intact
 def rewrite(name,body):
  nonlocal s
  pattern=name+'('
  pos=0
  while True:
   a=s.find(pattern,pos)
   if a<0:break
   i=a+len(pattern);start=i;depth=1;braces=0;args=[]
   while depth:
    c=s[i]
    if c=='(':depth+=1
    elif c==')':depth-=1
    elif c=='{':braces+=1
    elif c=='}':braces-=1
    elif c==',' and depth==1 and braces==0:args.append(s[start:i]);start=i+1
    i+=1
   args.append(s[start:i-1]);replacement=body(args);s=s[:a]+replacement+s[i:];pos=a+len(replacement)
 rewrite('thinkthen_question_parse',lambda a:'[e parseQuestion:'+a[2]+' role:'+a[1]+' output:'+a[3]+' failure:&failed]')
 rewrite('thinkthen_question_load',lambda a:'[e loadQuestion:'+a[1]+' output:'+a[2]+' failure:&failed]')
 for method,selector in [('load_named','namedQuestion'),('load_reference','referenceQuestion')]:rewrite('thinkthen_question_'+method,lambda a,sel=selector:'[e '+sel+':'+a[2]+' role:'+a[1]+' output:'+a[3]+' failure:&failed]')
 rewrite('thinkthen_question_new',lambda a:'[e question:'+a[1]+' author:NULL output:'+a[2]+' failure:&failed]')
 rewrite('thinkthen_question_new_authored',lambda a:'[e question:'+a[1]+' author:'+a[2]+' output:'+a[3]+' failure:&failed]')
 rewrite('thinkthen_image_clone',lambda a:'[e image:'+a[1]+' length:'+a[2]+' media:'+a[3]+' filename:'+a[4]+' output:'+a[5]+' failure:&failed]')
 # fill source references only after all images clone successfully
 s=s.replace('thinkthen_record_v1 records', 'for(size_t i=0;i<%d;++i) raw_images[i]=images[i]->native; thinkthen_record_v1 records'%len(v.get('image_paths',[]) if not v.get('paths') else []))
 s=s.replace('(const thinkthen_image *const *)images','raw_images')
 rewrite('thinkthen_source_records',lambda a:'[e records:'+a[1]+' count:'+a[2]+' output:'+a[3]+' failure:&failed]')
 for method,flag in [('files','0'),('image_files','1')]:rewrite('thinkthen_source_'+method,lambda a,flag=flag:'[e sourceFiles:'+a[1]+' imageReader:'+flag+' output:'+a[2]+' failure:&failed]')
 for name in shared.FUNCTIONS:
  rewrite('thinkthen_'+name+'_complete',lambda a,n=name:'[e '+n+'Complete:'+a[1]+' source:'+a[2]+' controls:'+a[3]+' output:'+a[4]+' failure:&failed]')
  rewrite('thinkthen_'+name+'_batch_start',lambda a,n=name:'[e '+n+'Batch:'+a[1]+' source:'+a[2]+' controls:'+a[3]+' output:'+a[4]+' failure:&failed]')
 s=s.replace('thinkthen_batch *batch=NULL','TTBatch *batch=nil').replace('thinkthen_batch_next(batch,&result)','[batch next:&result failure:&failed]').replace('thinkthen_batch_facts(batch,&result)','[batch facts:&result failure:NULL]').replace('thinkthen_batch_free(batch)','[batch dealloc]')
 s=s.replace('thinkthen_result_free(result)','tt_native_result_free(result)').replace('thinkthen_source_free(source)','[source dealloc]').replace('thinkthen_question_free(q)','[q dealloc]').replace('thinkthen_image_free(images[i])','[images[i] dealloc]')
 s=s.replace('code,result); tt_native_result_free(result);','code,code?failed:result); tt_native_result_free(failed); tt_native_result_free(result);')
 return s

with tempfile.TemporaryDirectory(prefix='thinkthen-objc-complete-consumer-') as folder:
    scratch=Path(folder); binary=scratch/'consumer'; source=scratch/'consumer.m'
    inventory=parity.inventory();rows=list(parity.required_cases(inventory,CONSUMER).values())
    cases={r['id']:r for r in json.loads((ROOT/'conformance/cases.json').read_text())['cases']};named={r['id']:r for r in json.loads((ROOT/'conformance/named-inputs.json').read_text())['cases']}
    numbers=[];defs=[]
    for at,row in enumerate(rows):
        v=shared.document(row,cases,named)
        for si,step in enumerate(v.get('steps',[v])):
            number=10000+at*100+si if 'steps' in v else at;numbers.append(number);defs.append(generated(number,step))
    source.write_text('#include <threads.h>\n#include "native_output.h"\nint cancel_on_input(void *token) { if(getchar()!=33) abort(); thinkthen_cancel(token); puts("cancel-fired"); fflush(stdout); return 0; }\nchar *read_caption(const char *path,thinkthen_optional_content_v1 *out) { FILE *f=fopen(path,"rb"); if(!f || fseek(f,0,SEEK_END)) abort(); long n=ftell(f); if(n<0 || n>16777216 || fseek(f,0,SEEK_SET)) abort(); char *s=malloc((size_t)n+1); if(!s || fread(s,1,(size_t)n,f)!=(size_t)n || fclose(f)) abort(); s[n]=0; *out=(thinkthen_optional_content_v1){1,{1,{s,(size_t)n}}}; return s; }\n'+'\n'.join(defs)+'\nint main(int argc,char **argv) { if(argc!=3) return 2; FILE *f=fopen(argv[1],"rb"); if(!f || fseek(f,0,SEEK_END)) abort(); long len=ftell(f); rewind(f); char *s=calloc((size_t)len+1,1); if(!s || fread(s,1,(size_t)len,f)!=(size_t)len) abort(); fclose(f); TTJSON *input=tt_json_parse(s,(size_t)len); free(s); const TTJSON *number=tt_json_get(input,"case_number"); if(!number) abort(); int case_id=atoi(number->text); tt_json_free(input); if(!strstr(argv[2],"http://127.0.0.1:")) return 3; TTFailure failure={0}; TTClient *e=[TTClient createWithSettings:argv[2] length:strlen(argv[2]) failure:&failure]; if(!e) { thinkthen_result *raw=NULL; TTNativeResult *r=NULL; if(thinkthen_error_complete(NULL,&raw) || tt_native_snapshot(raw,&r)) abort(); output(nil,1,failure.kind,r); tt_native_result_free(r); tt_failure_clear(&failure); return 0; } switch(case_id) {\n'+'\n'.join('case %d:case_%d(e);break;'%(n,n) for n in numbers)+'\ndefault:abort(); } tt_failure_clear(&failure); [e dealloc]; return 0; }')
    native=ROOT/'libraries/c/target/debug'
    (scratch/'libthinkthen.so.0').symlink_to(native/'libthinkthen_c.so')
    package=ROOT/'libraries/objective-c'
    # ASan/UBSan inspect reads of every host-owned field after native result_free.
    subprocess.run(['gcc','-std=gnu11','-g','-no-pie','-fsanitize=address,undefined','-fno-omit-frame-pointer','-x','objective-c','-I',str(ROOT/'libraries/c/include'),'-I',str(package/'Sources'),'-I',str(package/'checks'),
                    str(source),*[str(package/'Sources'/name) for name in ['ThinkThen.m','TTNativeAPI.m','TTNativeViews.c','TTJSON.c']],'-L',str(native),'-lthinkthen_c','-lobjc','-pthread','-lm','-Wl,-rpath,'+str(scratch),'-o',str(binary)],check=True)
    native_cases(binary)
