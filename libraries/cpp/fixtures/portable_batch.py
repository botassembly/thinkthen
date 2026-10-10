"""Count exact shared Max bodies from the public C++ bulk call."""

import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / "conformance/children"))
from children import child_env
from backend_cases import ROWS, alias, configuration, paths
from portable import one_portable_request
import tempfile

ROOT = Path(__file__).resolve().parents[3]
FIXTURES = ROOT / "specification/fixtures/batching"
NATIVE = Path(os.environ["THINKTHEN_PORTABLE_NATIVE"])
CPP_INCLUDE = Path(os.environ.get("THINKTHEN_PORTABLE_CPP_INCLUDE", ROOT / "libraries/cpp/include"))
target = Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target"))
if not target.is_absolute():
    target = ROOT / target
BACKEND = Path(os.environ.get("THINKTHEN_BACKEND_BIN", target / "debug/conformance-backend"))
CORPUS = FIXTURES / "portable-records.json"
corpus = json.loads(CORPUS.read_text())
assert corpus["schema"] == "thinkthen.portable-batch-records/1" and len(corpus["texts"]) == 5

with tempfile.TemporaryDirectory(prefix="thinkthen-cpp-portable-") as scratch:
    folder = Path(scratch)
    include = folder / "include/thinkthen"
    include.mkdir(parents=True)
    header = NATIVE / "include/thinkthen.h"
    if not header.exists():
        header = NATIVE / "include/thinkthen/thinkthen.h"
    shutil.copyfile(header, include / "thinkthen.h")
    consumer = folder / "portable-consumer"
    library = NATIVE / "lib/libthinkthen.so"
    if not library.exists():
        library = NATIVE / "lib/libthinkthen.so.0"
    subprocess.run([os.environ.get("CXX", "c++"), "-std=c++17", "-I", str(CPP_INCLUDE),
                    "-I", str(folder / "include"), str(ROOT / "libraries/cpp/fixtures/portable_consumer.cpp"),
                    "-L", str(NATIVE / "lib"), "-Wl,-rpath," + str(NATIVE / "lib"), "-l:" + library.name,
                    "-o", str(consumer)], env=child_env(), check=True, timeout=60)
    for named in (False, True):
        server = subprocess.Popen([BACKEND], env=child_env(THINKTHEN_TEST_MARKERS=json.dumps({"local":"tt-named-loopback"})), stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True)
        try:
            port = int(server.stdout.readline())
            base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
            env = child_env(home=scratch, XDG_CONFIG_HOME=scratch if named else str(Path(scratch) / "config"))
            env.update(THINKTHEN_API_KEY="sk-loopback-cpp-portable", THINKTHEN_BASE_URL=base,
                       TT_PORTABLE_SETTINGS=json.dumps({"base_url": base, "model": corpus["model"],
                                                        "batch": "max", "cache": False, "max_retries": 0,
                                                        "throttle": 1}),
                       TT_PORTABLE_CORPUS=str(CORPUS), THINKTHEN_CACHE=str(folder / "cache"),
                       LD_LIBRARY_PATH=str(NATIVE / "lib"))
            if named:
                row = next(row for row in ROWS if row["name"] == "typesafe")
                base = f"http://127.0.0.1:{port}/arm/full/capture/v1"
                configuration(env, {"local": alias(row, base)})
                env[row["key"]] = "tt-named-loopback"
                env["THINKTHEN_BASE_URL"] = f"http://127.0.0.1:{port}/generic/v1"
                env["TT_PORTABLE_SETTINGS"] = json.dumps({"backend":"local", "batch":"max", "cache":False, "max_retries":0, "throttle":1})
            run = subprocess.run([consumer], env=env, capture_output=True, text=True, timeout=60)
            assert run.returncode == 0 and "CPP_PORTABLE_BATCH_PASS" in run.stdout, (run.stdout, run.stderr)
            server.stdin.write("count\n")
            server.stdin.flush()
            count = int(server.stdout.readline())
            server.stdin.write("capture\n")
            server.stdin.flush()
            captured = json.loads(server.stdout.readline())
            assert count == 1, (count, captured)
            one_portable_request(captured["bodies"])
            if named:
                for command, expected in (("paths", paths(row["path"])),
                                          ("bearers", {"markers":{"local":1},"absent":0,"unknown":0,"overflow":False})):
                    server.stdin.write(command + "\n"); server.stdin.flush()
                    assert json.loads(server.stdout.readline()) == expected, command
                assert "tt-named-loopback" not in run.stdout + run.stderr
                assert "tt-named-loopback" not in json.dumps(captured["bodies"])
                print("cpp named backend: selected path, bearer, result and secrecy PASS")
            print("cpp portable: five typed rows, one request with the fixture questions")
        finally:
            server.stdin.close()
            server.wait(timeout=10)

from pathlib import Path
import subprocess, tempfile
ROOT = Path(__file__).resolve().parents[3]
CONSUMER = 'cpp'
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
    for field in ('requests_sent','records','cache_answers','call_id','input_tokens','output_tokens'):
        if facts.get(field) is not None: out[field]=facts[field]
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
        if k==6 and raw['rank_members'][i]:
            def rank_facts(target, common, details):
                meta=common['meta']
                target['model']=meta['model']; target['context_digest']=meta['context_sha256']
                target['usage']={key:details['usage'][key] for key in ('input_tokens','output_tokens') if details['usage'][key] is not None}
                target['source_batch_sizes']=[source['batch_size'] for source in details['question_sources']]
            rank_facts(row,common,raw['details'][i]); row['question_name']=v['question_name']; row['members']=[]
            for j,member in enumerate(raw['rank_members'][i]):
                member=member['data']['rank'] if 'function' in member else member
                mc=member['common']; mm=mc['meta']; author=raw['member_authors'][i][j]
                child={'name':member['question_name'],'value':member['value'],'probability':mc['answer']['data']['probability'],
                       'answer_id':mc['answer_id'],'author':author['name'],'observations':len(mm['observations']),'sources':len(mm['question_sources'])}
                if author['wording_version'] is not None:child['wording_version']=author['wording_version']
                rank_facts(child,mc,raw['rank_member_details'][i][j]); row['members'].append(child)
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
    required = {row['id'] for row in rows}
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
        print(('parity: ' if row['id'] in required else 'regression: ')+json.dumps({'consumer':CONSUMER,'case':row['id'],'checks':row.get('checks',['named','runtime']),'status':'fail' if error else 'pass'}),flush=True)
    print(CONSUMER+' complete shared cases: %d/%d passed'%(len(rows)-failed,len(rows)))
    if failed:raise SystemExit(1)

with tempfile.TemporaryDirectory(prefix='thinkthen-cpp-complete-consumer-') as folder:
    scratch = Path(folder); include = scratch/'include/thinkthen'; include.mkdir(parents=True)
    header = NATIVE/'include/thinkthen.h'
    if not header.is_file(): header = NATIVE/'include/thinkthen/thinkthen.h'
    shutil.copyfile(header,include/'thinkthen.h')
    binary=scratch/'consumer'; library=NATIVE/'lib/libthinkthen.so'
    if not library.is_file(): library=NATIVE/'lib/libthinkthen.so.0'
    subprocess.run([os.environ.get('CXX','c++'),'-std=c++17','-Wall','-Wextra','-Werror','-I',str(CPP_INCLUDE),'-I',str(scratch/'include'),
                    str(ROOT/'libraries/cpp/fixtures/native_consumer.cpp'),'-L',str(NATIVE/'lib'),'-l:'+library.name,'-Wl,-rpath,'+str(NATIVE/'lib'),'-o',str(binary)],env=child_env(HOME=str(scratch),LANG='C.UTF-8'),check=True)
    native_cases(binary)
