"""Run the unchanged shared canonical cases through each real family public door."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time

ROOT=Path(__file__).resolve().parents[3]
sys.path.insert(0,str(ROOT/'conformance'))
import parity
from c_parity import document, Backend, prepare, assertions, compact
from complete_projection import project


def main():
    consumer=sys.argv[1]
    assert consumer in ('php','dart','flutter')
    rows=list(parity.required_cases(parity.inventory(),consumer).values())
    cases={r['id']:r for r in json.loads((ROOT/'conformance/cases.json').read_text())['cases']}
    named={r['id']:r for r in json.loads((ROOT/'conformance/named-inputs.json').read_text())['cases']}
    library=Path(os.environ.get('THINKTHEN_COMPLETE_LIBRARY',str(ROOT/'libraries/c/target/debug/libthinkthen_c.so')))
    dart=os.environ.get('TT_DART',str(Path.home()/'.local/opt/flutter/bin/dart'))
    flutter=os.environ.get('TT_FLUTTER',str(Path.home()/'.local/opt/flutter/bin/flutter'))
    failures=0
    with tempfile.TemporaryDirectory(prefix='thinkthen-0429-') as tmp:
        scratch=Path(tmp)
        helper=scratch/'cancel.so'
        subprocess.run(['cc','-shared','-fPIC','-pthread','-Wall','-Wextra','-Werror',str(ROOT/'libraries/php/fixtures/cancel_reader.c'),'-ldl','-o',str(helper)],check=True)
        # Counted descriptor constructors are an additional public door, with no synthetic parity cells.
        with tempfile.TemporaryDirectory(prefix='constructors-',dir=scratch) as owned:
            env={'PATH':os.environ['PATH'],'HOME':owned,'XDG_CONFIG_HOME':owned+'/config',
                 'XDG_CACHE_HOME':owned+'/cache','XDG_STATE_HOME':owned+'/state','LANG':'C.UTF-8',
                 'PUB_CACHE':os.environ.get('PUB_CACHE',str(Path.home()/'.pub-cache')),
                 'FLUTTER_SUPPRESS_ANALYTICS':'true','CI':'true'}
            backend=Backend(ROOT/'target/debug/conformance-backend',env)
            try:
                settings=compact({'base_url':f'http://127.0.0.1:{backend.port}/generic/v1','cache':False,'max_retries':0})
                env['THINKTHEN_API_KEY']='sk-conformance-loopback'
                if consumer=='php':command=['/usr/bin/php8.3','-n','-d','extension=ffi','-d','ffi.enable=1',str(ROOT/'libraries/php/fixtures/complete_constructed.php'),str(library),settings]
                elif consumer=='dart':command=[dart,str(ROOT/'libraries/dart/checks/consumers/alpha/bin/complete_constructed.dart'),str(library),settings]
                else:
                    command=[flutter,'test','--no-pub','--reporter','expanded',str(ROOT/'libraries/dart/flutter/example/test/complete_constructed_test.dart')]
                    env.update(TT_NATIVE_LIBRARY=str(library),TT_SETTINGS=settings)
                output=subprocess.run(command,env=env,cwd=ROOT/'libraries/dart/flutter/example' if consumer=='flutter' else ROOT,capture_output=True,text=True,timeout=60)
                assert output.returncode==0,(output.stdout,output.stderr)
                assert 'sk-conformance-loopback' not in output.stdout+output.stderr
                assert int(backend.read('count'))==11
                print(consumer+' counted constructors: ten functions, 11 arrivals, copied results retained after close',flush=True)
            finally:backend.close()
        for row in rows:
            error=None
            try:
                value=document(row,cases,named)
                with tempfile.TemporaryDirectory(prefix='case-',dir=scratch) as owned:
                    home=Path(owned)
                    child={'PATH':os.environ['PATH'],'HOME':str(home),'XDG_CONFIG_HOME':str(home/'config'), 'XDG_CACHE_HOME':str(home/'cache'),'XDG_STATE_HOME':str(home/'state'), 'LANG':'C.UTF-8','TT_REPO':str(ROOT),'TT_CANCEL_HELPER':str(helper),'PUB_CACHE':os.environ.get('PUB_CACHE',str(Path.home()/'.pub-cache')),'FLUTTER_SUPPRESS_ANALYTICS':'true','CI':'true'}
                    backend=Backend(ROOT/'target/debug/conformance-backend',child)
                    try:
                        child.update(THINKTHEN_BASE_URL=f'http://127.0.0.1:{backend.port}/{value["arm"]}', THINKTHEN_API_KEY='sk-conformance-loopback', LIQUIDAI_API_KEY='sk-conformance-loopback',OPENROUTER_API_KEY='sk-conformance-loopback',PERPLEXITY_API_KEY='sk-conformance-loopback')
                        prepare(home,value)
                        identities=[]
                        steps=value.get('steps',[value])
                        for step in steps:
                            if step.get('copy_store'):
                                import sqlite3
                                (home/'refreshed').mkdir()
                                with sqlite3.connect(home/'saved/thinkthen.sqlite') as a,sqlite3.connect(home/'refreshed/thinkthen.sqlite') as b:a.backup(b)
                            if step.get('damage_store'):
                                import sqlite3
                                with sqlite3.connect(home/'saved/thinkthen.sqlite') as db:db.execute("UPDATE answers SET answer='damaged fixture answer'")
                            if value.get('image_variants'):
                                backend.close();backend=Backend(ROOT/'target/debug/conformance-backend',child)
                                child['THINKTHEN_BASE_URL']=f'http://127.0.0.1:{backend.port}/{step["arm"]}'
                                prepare(home,step)
                            settings={'cache':False,'model':'jev-latest' if 'steps' in value else 'jev-1.13.0','batch':1,'max_retries':0,**step.get('settings',{})}
                            settings['base_url']=child['THINKTHEN_BASE_URL']
                            settings={k:str(home/'saved') if v=='$FOLDER' else str(home/'refreshed') if v=='$REFRESH' else str(home/'profile.json') if v=='$PROFILE' else v for k,v in settings.items()}
                            if row['kind'] in ('images','image-location'):settings['record']=str(home/'recorded')
                            def execute(settings):
                                path=home/'input.json';path.write_text(compact(step))
                                if consumer=='php':
                                    command=['/usr/bin/php8.3','-n','-d','extension=ffi','-d','ffi.enable=1','-d','memory_limit=2G',str(ROOT/'libraries/php/fixtures/complete_native.php'),str(path),str(library),compact(settings)]
                                elif consumer=='dart':
                                    command=[str(ROOT/'libraries/dart/checks/scratch/complete-native'),str(path),str(library),compact(settings)]
                                else:
                                    command=[flutter,'test','--no-pub','--reporter','expanded',str(ROOT/'libraries/dart/flutter/example/test/complete_native_test.dart')]
                                    child.update(TT_INPUT=str(path),TT_NATIVE_LIBRARY=str(library),TT_SETTINGS=compact(settings),TT_OUTPUT=str(home/'output.json'))
                                cwd=ROOT/'libraries/dart/flutter/example' if consumer=='flutter' else home
                                # Flutter calls explicitly adopt the fixture's owned current directory in its test.
                                child['TT_CASE_HOME']=str(home)
                                running=subprocess.Popen(command,env=child,cwd=cwd,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
                                try:
                                    if step.get('held_cancel'):
                                        assert backend.read('wait 1')=='wait 1'
                                        (home/'cancel').touch()
                                        limit=time.monotonic()+10
                                        while not (home/'cancel.fired').exists():
                                            if time.monotonic()>limit:raise AssertionError('public cancellation did not fire')
                                            time.sleep(.01)
                                        backend.process.stdin.write('release\n');backend.process.stdin.flush()
                                    stdout,stderr=running.communicate(timeout=120)
                                    assert running.returncode==0,(stdout,stderr)
                                    if consumer=='flutter':payload=json.loads((home/'output.json').read_text())
                                    else:
                                        assert not stderr,stderr
                                        assert 'sk-conformance-loopback' not in stdout+stderr, 'credential leaked'
                                        payload=json.loads(stdout)
                                    assert 'sk-conformance-loopback' not in compact(payload), 'credential leaked'
                                    return project(payload,step['verb'])
                                finally:
                                    if running.poll() is None:running.kill();running.wait()
                            before=int(backend.read('count'))
                            got=execute(settings)
                            if value.get('identity_steps'):identities.append(got)
                            if step.get('owned_jsonl'):
                                request=json.loads(json.loads(backend.read('capture'))['bodies'][0])
                                expected={f'q{i+1}':{'type':'noul','instructions':f'The text is {compact(item)}. {step["question"]["decide"]}'} for i,item in enumerate(step['items'][:2])}
                                assert request['questions']==expected,request
                            if step.get('stored_answers')==0:
                                import sqlite3
                                path=home/'saved/thinkthen.sqlite'
                                if path.exists():
                                    with sqlite3.connect(path) as db:assert db.execute('SELECT count(*) FROM answers').fetchone()[0]==0
                            if value.get('image_variants'):
                                from c_images import assert_images
                                assert_images(step,got,json.loads(backend.read('capture'))['bodies'])
                            assertions(row,step,got,int(backend.read('count'))-(before if step.get('count_delta') else 0))
                            if row['kind'] in ('images','image-location'):
                                before=int(backend.read('count'))
                                replay={k:v for k,v in settings.items() if k!='record'};replay['replay']=str(home/'recorded')
                                saved=execute(replay);assertions(row,step,saved,int(backend.read('count')))
                                assert saved['requests_sent']==0 and int(backend.read('count'))==before,saved
                                assert saved['rows'][0]['answer_id']==got['rows'][0]['answer_id'],saved
                        if value.get('identity_steps'):
                            ids=[v['rows'][0]['observation_ids'] for v in identities]
                            assert ids[0]==ids[1]==ids[2] and ids[3]!=ids[0] and ids[4]==ids[3] and ids[5]==ids[0],ids
                            assert len({v['call_id'] for v in identities})==6
                            assert len({identities[i]['rows'][0]['answer_id'] for i in (0,1,2,5)})==1
                            assert identities[3]['rows'][0]['answer_id']==identities[4]['rows'][0]['answer_id']!=identities[0]['rows'][0]['answer_id']
                    finally:backend.close()
            except (AssertionError,ValueError,KeyError,TypeError,subprocess.SubprocessError,OSError) as failure:
                failures+=1;error=type(failure).__name__+': '+str(failure)
                print(f'{consumer} fixture {row["id"]} failed: {error}',file=sys.stderr)
            print('parity: '+json.dumps({'consumer':consumer,'case':row['id'],'checks':row.get('checks',['named','runtime']),'status':'fail' if error else 'pass'}),flush=True)
        print(f'{consumer} shared fixture results: {len(rows)-failures} passed, {failures} failed')
    return bool(failures)

if __name__=='__main__':raise SystemExit(main())
