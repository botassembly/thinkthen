"""Repeat stage-one complete Swift matrix with stage-two installed native bits."""
import collections,datetime,json,os,pathlib,signal,subprocess,sys,time
from backend import Backend, one_record
from process_group import run
R=pathlib.Path(__file__).resolve().parents[2]
L=R/'target/logs'/('run-'+datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ'))
L.mkdir(parents=True);(L/'barrier').mkdir();(L/'home').mkdir();(L/'cache').mkdir()
server=Backend(L/'barrier');receipts=[];status='FAILED'
env={'PATH':'/usr/bin:/bin','HOME':str(L/'home'),'XDG_CONFIG_HOME':str(L/'home'),'XDG_CACHE_HOME':str(L/'home'),
 'LD_LIBRARY_PATH':str(R/'target/native/lib'),'THINKTHEN_BASE_URL':f'http://127.0.0.1:{server.server_port}/generic/v1',
 'THINKTHEN_API_KEY':'tt-canary-294','THINKTHEN_CACHE':str(L/'cache'),'TT_BARRIER_DIR':str(L/'barrier')}
try:
    if len(sys.argv)==2 and sys.argv[1]=='facts':
        result=run([str(R/'target/scratch/swift-matrix'),'facts'],cwd=R,env=env,timeout=60)
        (L/'facts.log').write_bytes(result.stdout+result.stderr)
        assert result.exit==0 and b'SWIFT_FACTS_LIFETIME_PASS' in result.stdout,(result.exit,(result.stdout+result.stderr)[-1500:])
        assert collections.Counter(server.arrivals)==collections.Counter(['hold-facts-one','hold-facts-no-usage','status-401','recovery-scalar']) and server.attempts==server.connections==4,server.arrivals
        bodies=[json.loads(line) for line in (L/'barrier/requests.jsonl').read_text().splitlines()]
        assert collections.Counter(one_record(row)['state'] for row in bodies)==collections.Counter(server.arrivals) and all(row['questions']['q1']['type']=='noul' for row in bodies),bodies
        status='PASS'
        print('SWIFT_FACTS_EXACT_4_PASS',flush=True)
        sys.exit(0)
    for mode in ('direct','matrix'):
        result=run([str(R/'target/scratch/swift-matrix'),mode],cwd=R,env=env,timeout=120)
        (L/(mode+'.log')).write_bytes(result.stdout+result.stderr)
        receipts.append({'case':mode,'pid':result.pid,'pgid':result.pgid,'exit':result.exit,'signals':result.signals})
        assert result.exit==0 and (b'DIRECT_SWIFT_C_PASS' if mode=='direct' else b'STRICT_CANCELLED_SCALAR_PASS') in result.stdout,(mode,result.exit,(result.stdout+result.stderr)[-1500:])
    counts=collections.Counter(str(s) for s in server.arrivals)
    # Main 71f25087: records.md "Order and requests" and ADR 0055 1+3
    # pack distinct decide/filter/rank rows under one shared evidence state.
    # The repeated first/second bulk call reads the question cache (ADR 0111).
    packed_state='Each question quotes the text it asks about.'
    expected={'direct':1,'café':1,'yes':1,'no':1,'unsure':1,'a\x00b':1,packed_state:4,'json-decide':1,
     'choose':1,'tag':1,'score':1,
     '[{"id":"u001","evidence":"find-one"},{"id":"u002","evidence":"find-two"}]':1,
     "{'entities': [{'id': 'i1', 'name': 'First', 'kind': 'alert'}, {'id': 'i2', 'name': 'Second', 'kind': 'alert'}]}":1,
     "{'entities': [{'id': 'i1', 'name': 'Third', 'kind': 'alert'}, {'id': 'i2', 'name': 'Fourth', 'kind': 'alert'}]}":1,
     'annotate-one':1,'Maria Chen':2,'John Smith':2,'status-401':1,'failure-one':1,'failure-two':1,
     'success':1,'hold-deadline':1,'hold-scalar':1,'recovery-scalar':1}
    from backend import packed_rows
    requests=[json.loads(line) for line in (L/'barrier/requests.jsonl').read_text().splitlines()]
    expected_requests=json.loads((R/'Tests/fixtures/expected_requests.json').read_text())
    normalize=lambda rows: sorted((json.dumps(row,sort_keys=True,ensure_ascii=False,separators=(',',':')) for row in rows))
    assert normalize(requests)==normalize(expected_requests), 'complete normalized Swift request bodies differ'
    layouts=collections.Counter(tuple(packed_rows(request).values()) for request in requests if one_record(request)['state']==packed_state)
    wanted_layouts=collections.Counter({
      ('first','second','third'):1,
      ('filter-one','filter-two'):1, ('rank-one','rank-two'):1,
      tuple(f'hold-bulk-{i}' for i in range(1,7)):1})
    assert (counts==expected and layouts==wanted_layouts and
     server.attempts==server.connections==len(server.arrivals)==29 and
     server.bulk_completion==['packed:first,second,third'] and 'hold-scalar' in server.completions),(counts,layouts,server.bulk_completion)
    (L/'packed-layouts.json').write_text(json.dumps([list(k) for k in layouts.elements()],indent=2)+'\n')
    print('REPIN_ADAPTED_PACKING_AND_RESULT exact=29 packed=4 layouts=4',flush=True)
    status='PASS'
finally:
    server.close()
    (L/'arrivals.json').write_text(json.dumps(server.arrivals,ensure_ascii=False,indent=2)+'\n')
    (L/'outcome.json').write_text(json.dumps({'status':status,'arrivals':len(server.arrivals),'attempts':server.attempts,'receipts':receipts,'bulk_completion':server.bulk_completion},indent=2)+'\n')
    print(L,status,'arrivals',len(server.arrivals),flush=True)
if status!='PASS':sys.exit(1)

from pathlib import Path
import subprocess, tempfile
ROOT = Path(__file__).resolve().parents[4]
CONSUMER = 'swift'
# Complete native public cases, using the shared input and assertion inventory.
import sqlite3
sys.path.insert(0, str(ROOT / 'conformance'))
import c_parity as shared
from children import child_env
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
    for i,r in enumerate(raw['rows']):
        k=r['function']; v=r['data'][shared.FUNCTIONS[k-1]]; common=v['common']; meta=common['meta']
        value=v.get('value')
        if k == 8:
            causes=['','missing_answer','wrong_kind','missing_probability','invalid_probability','invalid_distribution','unexpected_probability']
            value={m['name']:native_member(m['data']['success']['value']) if m['state']==1 else {'failed':{'kind':'backend','cause':causes[m['data']['failure']['cause']]}} for m in v['answers']}
        else:value=native_value(k,value)
        row={'value':value,'answer_id':common['answer_id'],'index':v['index'] if k==7 else r['index'],
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
    rows += [{**row,'id':'native-filter-first-excluded'} for row in rows if row['id']=='15-rank-records']
    rows += [{**row,'id':'native-duplicate-row-indices'} for row in rows if row['id']=='complete-decide']
    failed=0
    for at,row in enumerate(rows):
        error=None
        try:
            original={'native-filter-first-excluded':'15-rank-records','native-duplicate-row-indices':'complete-decide'}.get(row['id'],row['id'])
            value=shared.document({**row,'id':original},cases,named)
            if row['id']=='native-filter-first-excluded':
                value.update(verb='filter',question={**value['question'],'threshold':0.5},expect={'success':{'operation':{'indexes':[1,2]}}})
            if row['id']=='native-duplicate-row-indices':value['items'] *= 2
            with tempfile.TemporaryDirectory(prefix='thinkthen-'+CONSUMER+'-complete-') as folder:
                home=Path(folder); child={'PATH':os.environ.get('PATH','/usr/bin:/bin'),'HOME':folder,'XDG_CONFIG_HOME':str(home/'config'),'XDG_CACHE_HOME':str(home/'cache'),'XDG_STATE_HOME':str(home/'state'),'ASAN_OPTIONS':'detect_leaks=1','UBSAN_OPTIONS':'halt_on_error=1'}
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
                        if row['id']=='native-filter-first-excluded':
                            assert [r['index'] for r in got['rows']]==[0,1,2] and [r['value'] for r in got['rows']]==[False,True,True],got
                            assert int(backend.read('count'))==3,got
                        if row['id']=='native-duplicate-row-indices':
                            assert [r['index'] for r in got['rows']]==[0,1] and got['rows'][0]['input']==got['rows'][1]['input'],got
                            assert got['records']==2 and int(backend.read('count'))==1,got
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

with tempfile.TemporaryDirectory(prefix='thinkthen-swift-complete-consumer-') as folder:
    binary=Path(folder)/'consumer'; package=ROOT/'libraries/swift'
    subprocess.run([os.environ.get('THINKTHEN_SWIFTC','swiftc'),'-swift-version','6','-warnings-as-errors','-j',os.environ.get('CARGO_BUILD_JOBS','2'),'-I',str(package/'Sources/CThinkThen'),
                    *map(str,sorted((package/'Sources/ThinkThen').glob('*.swift'))),str(package/'Tests/fixtures/native_views.swift'),str(package/'Tests/fixtures/native_consumer.swift'),
                    '-L',str(package/'target/native/lib'),'-lthinkthen','-Xlinker','-rpath','-Xlinker',str(package/'target/native/lib'),'-o',str(binary)],env=child_env(keep=('XDG_CACHE_HOME','SWIFTPM_MODULECACHE_OVERRIDE'),HOME=folder,LANG='C.UTF-8'),check=True)
    native_cases(binary)
