"""Shared cases execute named SQL functions against counted owned loopback."""
import json,os,sys,subprocess,tempfile,sqlite3
from pathlib import Path
ROOT=Path(__file__).resolve().parents[4]
sys.path.insert(0,str(ROOT/'conformance'))
import parity as inventory
from c_parity import document,prepare,assertions,compact,Backend
from project import project
from jsonschema.exceptions import ValidationError

sys.path.insert(0,str(ROOT/'databases/duckdb/tools'))
import inputs as duckdb_inputs
DUCKDB_VERSION=duckdb_inputs.selected()['version']
DUCKDB_PYTHON=str(Path(os.environ.get('THINKTHEN_TOOLCHAINS',Path.home()/'.cache/thinkthen-toolchains'))/'duckdb'/DUCKDB_VERSION/'venv/bin/python')

FUNCTIONS=('decide','choose','tag','score','filter','rank','find','annotate','recognize','relate')

def framing(step):
    question=step.get('raw') or compact(step['question'])
    if step.get('loader'):
        reference=step['reference']
        question='@@'+reference if step['loader']=='load_named' else reference if step['loader']=='load_reference' else '@./'+reference
    elif step.get('question_form')=='file':question='@fixture-question.json'
    records=[]
    images=[{'media':step.get('media','image/png' if path.endswith('.png') else 'image/jpeg'),'bytes':list((ROOT/path).read_bytes())} for path in step.get('image_paths',[])]
    for item in step.get('items',[]):
        record={} if step.get('image_only') else {'document' if step['verb']=='annotate' and step.get('text') and isinstance(item,str) else 'text' if step.get('text') and isinstance(item,str) else 'json':item}
        if images:record['images']=images
        if step.get('candidate_orders'):record['options']=step['candidate_orders'][len(records)]
        if step.get('context_present'):record['context']=step['context']
        records.append(record)
    controls={key:step['settings'][key] for key in ('context','deadline_ms','batch','threshold','options','levels','labels','none','proxy') if key in step.get('settings',{})}
    if step.get('shared_context') is not None:controls['context']=step['shared_context']
    if step['verb']=='find' and 'none' in step['question']:
        question=compact({k:v for k,v in step['question'].items() if k!='none'});controls['none']=step['question']['none']
    if (step.get('operation') or {}).get('injection')=='expired_deadline':controls['deadline_ms']=0
    inputs={'records':records,'attempts':True,'incremental':step.get('incremental',False)}
    if (step.get('operation') or {}).get('injection')=='cancel_token':inputs['cancelled']=True
    if step.get('paths'):
        unit=step.get('source_unit',3)
        options={'reading':{'unit':{1:'line',2:'window',3:'file',4:'file',5:'line'}[unit]},'media':'image' if unit==4 or step.get('image_reader') else 'text'}
        if unit==2:options['reading']['window']=step.get('window',2)
        inputs.pop('records')
        inputs['files']={'paths':[str(ROOT/path) if not step.get('owned_jsonl') else path for path in step['paths']],'options':options}
        if unit==5:inputs['files']['format']='jsonl'
    if (step.get('operation') or {}).get('injection')=='recording_read_failure':
        inputs.pop('records',None);inputs['files']={'paths':['missing-input']}
    return {'verb':step['verb'],'question':question,'inputs':inputs,'controls':controls,'held_cancel':step.get('held_cancel',False)}

def execute(consumer,frame,env,home,backend):
    if consumer=='sqlite':
        library=os.environ.get('THINKTHEN_SQLITE_EXTENSION',str(ROOT/'databases/sqlite/target/debug/libthinkthen0.so'))
        command=[sys.executable,str(ROOT/'databases/sqlite/tests/complete/sqlite_child.py'),library]
    elif consumer=='duckdb':
        command=[os.environ.get('THINKTHEN_DUCKDB_PYTHON',DUCKDB_PYTHON),str(ROOT/'databases/sqlite/tests/complete/duckdb_child.py'),os.environ.get('THINKTHEN_DUCKDB_EXTENSION',str(duckdb_inputs.canonical()))]
    elif consumer=='postgresql':
        command=[sys.executable,str(ROOT/'databases/sqlite/tests/complete/postgresql_child.py'),os.environ['THINKTHEN_POSTGRESQL_SOCKET']]
        if 'files' in frame['inputs']:
            reader=subprocess.run([str(ROOT/'databases/postgresql/target/debug/thinkthen_read_inputs')],input=compact(frame['inputs']),env=env,cwd=home,text=True,capture_output=True)
            if reader.returncode:raise AssertionError(reader.stderr)
            frame={**frame,'inputs':json.loads(reader.stdout)}
    else:raise ValueError('unknown SQL consumer: '+consumer)
    running=subprocess.Popen(command,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True,env=env,cwd=home)
    try:
        if frame['held_cancel']:
            running.stdin.write(compact(frame)+'\n');running.stdin.flush()
            assert backend.read('wait 1')=='wait 1'
            running.stdin.write('!\n');running.stdin.flush()
            assert running.stdout.readline()=='cancel-fired\n'
            backend.process.stdin.write('release\n');backend.process.stdin.flush()
            stdout,stderr=running.communicate(timeout=120)
        else:stdout,stderr=running.communicate(compact(frame)+'\n',timeout=120)
        assert running.returncode==0 and not stderr,(running.returncode,stderr[:2000])
        assert 'sk-conformance-loopback' not in stdout+stderr
        return project(json.loads(stdout),frame['verb'])
    finally:
        if running.poll() is None:running.kill();running.wait()

def main():
    consumer=sys.argv[1]
    host=None
    if consumer=="postgresql":
        from postgresql_host import PostgresqlHost
        host=PostgresqlHost()
    rows=list(inventory.required_cases(inventory.inventory(),consumer).values())
    if len(sys.argv)>2:rows=[r for r in rows if r['id']==sys.argv[2]]
    cases={r['id']:r for r in json.loads((ROOT/'conformance/cases.json').read_text())['cases']}
    named={r['id']:r for r in json.loads((ROOT/'conformance/named-inputs.json').read_text())['cases']}
    failed=0
    for row in rows:
        failure=None
        try:
            value=document(row,cases,named)
            with tempfile.TemporaryDirectory(prefix='thinkthen-sql-complete-') as tmp:
                home=Path(tmp)
                env={'PATH':os.environ['PATH'],'HOME':tmp,'XDG_CONFIG_HOME':tmp+'/config','XDG_CACHE_HOME':tmp+'/cache','XDG_STATE_HOME':tmp+'/state','LANG':'C.UTF-8','LD_LIBRARY_PATH':os.environ.get('LD_LIBRARY_PATH','')}
                if consumer=='postgresql':env['THINKTHEN_POSTGRESQL_SOCKET']=os.environ['THINKTHEN_POSTGRESQL_SOCKET']
                backend=Backend(ROOT/'target/debug/conformance-backend',env)
                try:
                    env.update(THINKTHEN_API_KEY='sk-conformance-loopback',LIQUIDAI_API_KEY='sk-conformance-loopback',OPENROUTER_API_KEY='sk-conformance-loopback',PERPLEXITY_API_KEY='sk-conformance-loopback')
                    prepare(home,value)
                    if host:
                        server_env={**env,"THINKTHEN_BASE_URL":f"http://127.0.0.1:{backend.port}/{value['arm']}"}
                        host.start(server_env)
                    identities=[]
                    for step in value.get('steps',[value]):
                        if step.get('copy_store'):
                            (home/'refreshed').mkdir()
                            with sqlite3.connect(home/'saved/thinkthen.sqlite') as a,sqlite3.connect(home/'refreshed/thinkthen.sqlite') as b:a.backup(b)
                        if step.get('damage_store'):
                            with sqlite3.connect(home/'saved/thinkthen.sqlite') as db:db.execute("UPDATE answers SET answer='damaged fixture answer'")
                        if value.get('image_variants'):
                            backend.close();backend=Backend(ROOT/'target/debug/conformance-backend',env);prepare(home,step)
                        settings={'cache':False,'model':'jev-latest' if 'steps' in value else 'jev-1.13.0','batch':1,'max_retries':0,**step.get('settings',{})}
                        env['THINKTHEN_BASE_URL']=f'http://127.0.0.1:{backend.port}/{step["arm"] if value.get("image_variants") else value["arm"]}'
                        settings['base_url']=env['THINKTHEN_BASE_URL']
                        settings={k:str(home/'saved') if v=='$FOLDER' else str(home/'refreshed') if v=='$REFRESH' else str(home/'profile.json') if v=='$PROFILE' else v for k,v in settings.items()}
                        if row['kind'] in ('images','image-location'):settings['record']=str(home/'recorded')
                        frame=framing(step)
                        if consumer=='postgresql' and (step.get('loader')=='load' or step.get('question_form')=='file'):
                            # PostgreSQL's server CWD is PGDATA; a client names its owned file absolutely.
                            frame['question']='@'+str(home/frame['question'][1:])
                        # Reading fields belong to the complete door; engine fields retain configure.
                        engine={k:v for k,v in settings.items() if k not in frame['controls']}
                        if isinstance(engine.get('profile'),str) and Path(engine['profile']).is_file():engine['profile']=Path(engine['profile']).read_text()
                        frame['engine_settings']=engine
                        before=int(backend.read('count'))
                        got=execute(consumer,frame,env,home,backend)
                        if value.get('identity_steps'):identities.append(got)
                        if value.get('image_variants'):
                            from c_images import assert_images
                            assert_images(step,got,json.loads(backend.read('capture'))['bodies'])
                        if step.get('owned_jsonl'):
                            request=json.loads(json.loads(backend.read('capture'))['bodies'][0])
                            expected={f'q{i+1}':{'type':'noul','instructions':f'The text is {compact(item)}. {step["question"]["decide"]}'} for i,item in enumerate(step['items'][:2])}
                            assert request['questions']==expected,request
                        if step.get('stored_answers')==0:
                            path=home/'saved/thinkthen.sqlite'
                            if path.exists():
                                with sqlite3.connect(path) as db:assert db.execute('SELECT count(*) FROM answers').fetchone()[0]==0
                        assertions(row,step,got,int(backend.read('count'))-(before if step.get('count_delta') else 0))
                        if row['kind'] in ('images','image-location'):
                            replay={k:v for k,v in engine.items() if k!='record'};replay['replay']=str(home/'recorded')
                            saved=execute(consumer,{**frame,'engine_settings':replay},env,home,backend)
                            assert saved['requests_sent']==0 and int(backend.read('count'))==before+got['requests_sent']
                            assert saved['rows'][0]['answer_id']==got['rows'][0]['answer_id']
                    if value.get('identity_steps'):
                        ids=[v['rows'][0]['observation_ids'] for v in identities]
                        assert ids[0]==ids[1]==ids[2] and ids[3]!=ids[0] and ids[4]==ids[3] and ids[5]==ids[0],ids
                        assert len({v['call_id'] for v in identities})==6
                        assert len({identities[i]['rows'][0]['answer_id'] for i in (0,1,2,5)})==1
                        assert identities[3]['rows'][0]['answer_id']==identities[4]['rows'][0]['answer_id']!=identities[0]['rows'][0]['answer_id']
                finally:
                    if host:host.stop()
                    backend.close()
        except (ValidationError,AssertionError,ValueError,KeyError,TypeError,AttributeError,StopIteration,IndexError,OSError,subprocess.SubprocessError) as error:
            failed+=1;failure=f'{type(error).__name__}: {str(error)[:2000]}'
            print(f'{consumer} {row["id"]} failed: {failure}',file=sys.stderr,flush=True)
        print('parity: '+compact({'consumer':consumer,'case':row['id'],'checks':row.get('checks',['named','runtime']),'status':'fail' if failure else 'pass'}),flush=True)
    print(f'{consumer} complete SQL: {len(rows)-failed} passed, {failed} failed',flush=True)
    return bool(failed)
if __name__=='__main__':raise SystemExit(main())
