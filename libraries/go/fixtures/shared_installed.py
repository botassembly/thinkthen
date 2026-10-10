"""Run shared cases or routine public type cases through a staged Go module and counted backend."""
from pathlib import Path
import importlib.util
import base64
import json
import os
import shutil
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
sys.path.insert(0, str(ROOT / 'conformance'))
from c_parity import Backend
spec = importlib.util.spec_from_file_location('type_checks', ROOT / 'specification/fixtures/types/check.py')
checks = importlib.util.module_from_spec(spec); spec.loader.exec_module(checks)
module = Path(sys.argv[1]).resolve()
if os.environ.get('THINKTHEN_TEST_PROFILE', 'routine') == 'full':
    sys.path.insert(0, str(ROOT / 'libraries/cpp/fixtures'))
    from session_cases import native_cases, descriptor
    with tempfile.TemporaryDirectory(prefix='thinkthen-go-full-') as folder:
        home = Path(folder)
        (home / 'go.mod').write_text('module example.org/full-consumer\n\ngo 1.22\n\nrequire github.com/botassembly/thinkthen/libraries/go v0.0.0\nreplace github.com/botassembly/thinkthen/libraries/go => ' + str(module) + '\n')
        shutil.copy2(Path(__file__).with_name('full_consumer.go'), home / 'main.go')
        env = child_env(home=home, GOPROXY='off', GOSUMDB='off', GOTOOLCHAIN='local', GOCACHE=str(ROOT / 'target/go/cache'), GOMODCACHE=str(home / 'modcache'), CGO_ENABLED='1')
        binary = home / 'consumer-bin'
        subprocess.run(['go', 'build', '-p', '1', '-buildvcs=false', '-o', str(binary), '.'], cwd=home, env=env, check=True)
        def invoke(step, settings, child, work, backend):
            fixture = work / 'consumer-input.json'
            fixture.write_text(json.dumps({**descriptor(step, work), "incremental": step.get("incremental", False)}))
            args = [str(binary), str(fixture), json.dumps(settings)]
            if step.get('held_cancel'):
                process = subprocess.Popen(args, env=child, cwd=work, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
                try:
                    assert backend.read('wait 1') == 'wait 1'
                    process.stdin.write('!'); process.stdin.flush()
                    assert process.stdout.readline() == 'cancel-fired\n'
                    backend.process.stdin.write('release\n'); backend.process.stdin.flush()
                    stdout, stderr = process.communicate(timeout=60)
                    result = subprocess.CompletedProcess(args, process.returncode, stdout, stderr)
                finally:
                    backend.process.stdin.write('release\n'); backend.process.stdin.flush()
                    if process.poll() is None: process.kill(); process.wait()
            else:
                result = subprocess.run(args, env=child, cwd=work, capture_output=True, text=True, timeout=120)
            payload = json.loads(result.stdout)
            # Go context cancellation returns no terminal facts. The shared assertion
            # uses the actual loopback count, not invented native facts.
            if payload.get('admission', {}).get('code') == 5:
                payload['admission']['requests_sent'] = int(backend.read('count'))
                result.stdout = json.dumps(payload)
            return result
        native_cases(binary, consumer='go', invoke=invoke)
    sys.exit(0)
cases = json.loads((ROOT / 'specification/fixtures/types/corpus.json').read_text())['cases']
# The shared public corpus owns expected answers; retain every successful named
# function and each scalar presence reading. Envelope grammar cases exercise the
# retired JSON door and do not belong to this named API.
cases = [case for case in cases if 'request' in case and 'response' in case and
         not case.get('schema_only') and not case.get('expected_error') and case['name'] != '02-decide-details']
with tempfile.TemporaryDirectory(prefix='thinkthen-go-shared-') as folder:
    home=Path(folder); consumer=home/'consumer'; consumer.mkdir()
    (consumer/'go.mod').write_text('module example.org/shared-consumer\n\ngo 1.22\n\nrequire github.com/botassembly/thinkthen/libraries/go v0.0.0\nreplace github.com/botassembly/thinkthen/libraries/go => '+str(module)+'\n')
    shutil.copy2(Path(__file__).with_name('shared_consumer.go'), consumer/'main.go')
    env=child_env(home=str(home),GOPROXY='off',GOSUMDB='off',GOTOOLCHAIN='local',GOCACHE=str(ROOT/'target/go/cache'),GOMODCACHE=str(home/'modcache'),CGO_ENABLED='1')
    subprocess.run(['go','build','-p','1','-buildvcs=false','-o',str(home/'consumer-bin'),'.'],cwd=consumer,env=env,check=True)
    server=Backend(ROOT/'target/debug/conformance-backend',env)
    try:
        for case in cases:
            request=dict(case['request'])
            verb=next(v for v in ('decide','choose','tag','score','filter','rank','find','annotate','recognize','relate') if v in request)
            items=request.pop('evidence',request.pop('records',request.pop('units',None)))
            options=request.pop('call',{})
            if verb=='find': options['none']=request.pop('none',False)
            if verb=='annotate': question=request['annotate']
            else:
                question=request
                if verb in ('filter','rank'):question['decide']=question.pop(verb)
            path=home/'question.json';path.write_text(json.dumps(question))
            settings={'base_url':f'http://127.0.0.1:{server.port}/'+('generic/v1' if not case.get('case_id') else 'case/'+case['case_id']+'/v1'), 'model':'jev-1.13.0','max_retries':0}
            # Convert the shared batch value through the generated Go union.
            batch=options.pop('batch',0)
            fixture={'verb':verb,'items':items,'question_file':str(path),'settings':settings,'options':options,'batch':batch}
            before=int(server.read('count'))
            run=subprocess.run([str(home/'consumer-bin')],input=json.dumps(fixture),env=env|{'THINKTHEN_API_KEY':'sk-conformance-loopback'},cwd=home,text=True,capture_output=True,timeout=5)
            assert run.returncode==0,(case['name'],run.stderr)
            call=json.loads(run.stdout);terminal=call['Terminal'];packets=call['Packets']
            assert terminal.get('failure') is None,(case['name'],terminal)
            facts=terminal['facts']; assert facts['requests_sent']==int(server.read('count'))-before,(case['name'],facts)
            rows=[p['value'] for p in packets if p['kind']=='row']
            aggregates=[p['value'] for p in packets if p['kind']=='aggregate']
            if verb in ('decide','choose','tag','score'): actual=rows[0]['value']
            elif verb=='annotate':actual=[row['value'] for row in rows]
            elif verb=='filter':actual=[row['input'] for row in rows if row['value'] is True]
            elif verb=='rank':actual=[{'index':r['index'],'record':r['input'],'probability':r['answer']['probability']} for r in aggregates[-1]]
            elif verb=='recognize':actual=next(row['value'] for chunk in aggregates for row in chunk if row['index']==0)
            elif verb=='relate':actual={'edges':aggregates[-1]['value']}
            else:
                found=aggregates[-1]
                actual=None if found['value'] is None else {'index':found['index'],'unit':found['value'],'probability':found['answer']['probabilities'][found['answer']['pick']]}
            assert checks.subset(actual,case['response']),(case['name'],actual,case['response'])
            assert facts['call_id'] and any(p['kind']=='observation' for p in packets),case['name']
            print('GO_SHARED_INSTALLED_PASS',case['name'],flush=True)
        # Cache and replay observations keep their native identity and send nothing.
        cached=next(c for c in cases if c['name']=='02-decide-no')
        path=home/'cache-question.json'; path.write_text(json.dumps({k:v for k,v in cached['request'].items() if k!='evidence'}))
        fixture={'verb':'decide','items':cached['request']['evidence'],'question_file':str(path),'settings':{'base_url':f'http://127.0.0.1:{server.port}/case/{cached['case_id']}/v1','model':'jev-1.13.0'},'cache_path':str(home/'answer-cache')}
        fixture['settings'].update(model='jev-1.13.0',base_url=f'http://127.0.0.1:{server.port}/arm/full/v1',max_retries=0)
        retained=[]
        for expected in (1,0):
            before=int(server.read('count'))
            run=subprocess.run([str(home/'consumer-bin')],input=json.dumps(fixture),env=env|{'THINKTHEN_API_KEY':'sk-conformance-loopback'},cwd=home,text=True,capture_output=True,timeout=5)
            assert run.returncode==0,run.stderr
            call=json.loads(run.stdout);assert call['Terminal']['facts']['requests_sent']==expected and int(server.read('count'))==before+expected,(expected,call['Terminal']['facts'])
            retained.append(next(p['value'] for p in call['Packets'] if p['kind']=='row'))
        assert retained[0]['answer_id']==retained[1]['answer_id'] and retained[1]['meta']['cached']
        print('GO_SHARED_INSTALLED_PASS cache zero-send retained-identity',flush=True)
        import c_images
        for verb in ('decide','choose','score'):
            recipe=c_images.project({'verb':verb,'input':{'scenarios_ref':'format'}})['steps'][0]
            path=home/'image-question.json'; path.write_text(json.dumps(recipe['question']))
            image=ROOT/recipe['image_paths'][0]; encoded=base64.b64encode(image.read_bytes()).decode()
            fixture={'verb':verb,'items':'','question_file':str(path),'settings':{'backend':'liquid','model':'d1','base_url':f'http://127.0.0.1:{server.port}/{recipe['arm']}','record':str(home/('recorded-'+verb))},'images':encoded,'image_paths':[str(image)],'image_media':recipe['media'],'image_only':True}
            before=int(server.read('count'))
            image_env=env|{'LIQUIDAI_API_KEY':'sk-conformance-loopback'}
            run=subprocess.run([str(home/'consumer-bin')],input=json.dumps(fixture),env=image_env,cwd=home,text=True,capture_output=True,timeout=5)
            assert run.returncode==0,run.stderr
            call=json.loads(run.stdout); row=next(p['value'] for p in call['Packets'] if p['kind']=='row')
            assert row['value']=={'decide':True,'choose':'red','score':.8}[verb]
            assert [i['base64'] for i in row['images']]==[encoded,encoded]
            assert call['Terminal']['facts']['requests_sent']==1 and int(server.read('count'))==before+1
            wire=json.loads(json.loads(server.read('capture'))['bodies'][-1]);expected_url='data:'+recipe['media']+';base64,'+encoded
            assert wire['images']==[expected_url,expected_url]
            fixture['settings'].pop('record');fixture['settings']['replay']=str(home/('recorded-'+verb))
            run=subprocess.run([str(home/'consumer-bin')],input=json.dumps(fixture),env=image_env,cwd=home,text=True,capture_output=True,timeout=5)
            assert run.returncode==0,run.stderr
            replay=json.loads(run.stdout); assert replay['Terminal']['facts']['requests_sent']==0 and int(server.read('count'))==before+1
            saved=next(p['value'] for p in replay['Packets'] if p['kind']=='row');assert saved['answer_id']==row['answer_id'] and saved['images']==row['images']
            print('GO_SHARED_INSTALLED_PASS',verb,'bytes/file duplicate-images replay zero-send',flush=True)
        malformed=c_images.project({'verb':'decide','input':{'scenarios_ref':'malformed'}})['steps'][0]
        fixture.update(verb='decide',images=base64.b64encode((ROOT/malformed['image_paths'][0]).read_bytes()).decode(),image_paths=[],question_file=str(home/'image-question.json'))
        (home/'image-question.json').write_text(json.dumps(malformed['question']))
        fixture['settings'].pop('replay');before=int(server.read('count'))
        run=subprocess.run([str(home/'consumer-bin')],input=json.dumps(fixture),env=image_env,cwd=home,text=True,capture_output=True,timeout=5)
        assert run.returncode==0,run.stderr
        refused=json.loads(run.stdout);assert refused.get('code')==1 and int(server.read('count'))==before,refused
        print('GO_SHARED_INSTALLED_PASS malformed-image typed-refusal zero-send',flush=True)
    finally:server.close()
