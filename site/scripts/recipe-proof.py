#!/usr/bin/env python3
"""Bounded controlled-loopback recording, replay, cache and audit proof.

No provider route or credential argument is accepted. Outputs are caller named.
"""
import argparse
import hashlib
import http.server
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import threading

RECIPE = 'rules-propose-model-confirms'
SOURCE = Path(__file__).resolve().parents[1] / 'examples/recipes' / RECIPE
spec = importlib.util.spec_from_file_location('candidates', SOURCE / 'files/candidates.py')
producer = importlib.util.module_from_spec(spec)
spec.loader.exec_module(producer)

def digest(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()

def dump(p, value):
    Path(p).write_text(json.dumps(value, indent=2) + '\n')

def jsonl(p, rows):
    Path(p).write_text(''.join(json.dumps(row, separators=(',', ':'))+'\n' for row in rows))

class Scratch:
    def __init__(self):
        self.owned = Path(tempfile.mkdtemp(prefix='thinkthen-recipe-'))
    def cleanup(self, path):
        if Path(path) != self.owned:
            raise ValueError('cleanup refuses unowned path')
        shutil.rmtree(path)

class Listener:
    def __init__(self, expected, port=0):
        self.expected, self.requests, self.errors, self.stage, self.ceiling = expected, [], [], 'prepare', 7
        owner = self
        class Handler(http.server.BaseHTTPRequestHandler):
            def log_message(self, *args):
                pass
            def do_POST(self):
                body = json.loads(self.rfile.read(int(self.headers['Content-Length'])))
                owner.requests.append((owner.stage,body))
                matches = [e for e in owner.expected if e['request'] == body]
                if len(matches) != 1 or owner.count(owner.stage) > owner.ceiling:
                    owner.errors.append('literal request/ceiling mismatch')
                    sys.stderr.write('synthetic request mismatch: '+json.dumps(body)+'\n')
                    self.send_response(400);self.end_headers();return
                e = matches[0]; criteria = e['request']['questions']['q1']['criteria']
                probabilities = {label: 0.0 for label in criteria}
                if e['id'] == 'N01': probabilities.update(c001=0.5,c002=0.5)
                elif e['id'] == 'N02': probabilities.update(c001=0.6,none=0.4)
                else: probabilities[e['choice']] = 1.0
                pick = max(probabilities,key=probabilities.get)
                payload = json.dumps({'model':'recipe-fixture-v1','answers':{'q1':{'type':'choice','choice':pick,'confidence':0.0,'probabilities':probabilities}},'usage':{'input_tokens':100,'output_tokens':10}}).encode()
                self.send_response(200);self.send_header('Content-Length',str(len(payload)));self.end_headers();self.wfile.write(payload)
        self.server = http.server.ThreadingHTTPServer(('127.0.0.1',port),Handler)
        self.thread = threading.Thread(target=self.server.serve_forever,daemon=True)
        self.thread.start()
        self.url = f'http://127.0.0.1:{self.server.server_port}'
    def count(self, stage):
        return sum(s==stage for s,_ in self.requests)
    def close(self):
        self.server.shutdown();self.thread.join();self.server.server_close()

def environment(home):
    # Ambient keys, product configuration and backend selection are excluded.
    return {'PATH':'/usr/bin:/bin','HOME':str(home),'LANG':'C.UTF-8','LC_ALL':'C.UTF-8',
            'XDG_CONFIG_HOME':str(home/'config'),'XDG_CACHE_HOME':str(home/'cache'),
            'XDG_STATE_HOME':str(home/'state'),'PYTHONDONTWRITEBYTECODE':'1'}

def run(bin, args, env, text=None, expected=0, marker=None):
    done = subprocess.run([str(bin),*args],input=text,capture_output=True,text=True,env=env,timeout=20,check=False)
    if done.returncode != expected or (marker and marker not in done.stderr):
        raise AssertionError(f'CLI receipt: expected {expected}/{marker}; got {done.returncode}: {done.stderr}')
    return done

def verify_rows(rows, expected):
    if [r['id'] for r in rows] != [e['id'] for e in expected]:
        raise AssertionError('producer coverage')
    for row,e in zip(rows,expected):
        if row != {k:e[k] for k in ['id','text','candidates']}:
            raise AssertionError('independent candidate order/value/description')

def reconcile(results, expected):
    ordinary = [e for e in expected if e['id'].startswith('R') and e['id'] != 'R01']
    if [r['input']['id'] for r in results] != [e['id'] for e in ordinary]:
        raise AssertionError('full result denominator')
    for r,e in zip(results,ordinary):
        if r['value'] != e['choice']:
            raise AssertionError('wrong retained choice')

def main():
    parser=argparse.ArgumentParser()
    parser.add_argument('--recipe',choices=[RECIPE],required=True)
    parser.add_argument('--bin',type=Path,required=True)
    parser.add_argument('--artifacts',type=Path,required=True)
    parser.add_argument('--mode',choices=['replay','prepare-fixture'],default='replay')
    a=parser.parse_args();a.bin=a.bin.resolve();a.artifacts=a.artifacts.resolve()
    if a.artifacts.exists():
        raise ValueError('artifacts must be a new owned output directory')
    scratch=Scratch();listener=None
    try:
        # Refusal plant precedes the first real cleanup, including exceptional cleanup.
        try: scratch.cleanup(Path.cwd())
        except ValueError: pass
        else: raise AssertionError('cleanup guard')
        a.artifacts.mkdir(parents=True)
        work=scratch.owned
        env=environment(work/'home')
        (work/'home').mkdir()
        expected=json.loads((SOURCE/'files/expected.json').read_text())
        before={str(p.relative_to(SOURCE)):digest(p) for p in SOURCE.rglob('*') if p.is_file()}
        rows=producer.produce([sys.executable,str(SOURCE/'files/candidates.py'),str(SOURCE/'files/cases.jsonl')])
        verify_rows(rows,expected)
        provenance=json.loads((SOURCE/'files/provenance.json').read_text()) if a.mode=='replay' else None
        listener=Listener(expected, int(provenance['address'].rsplit(':',1)[1]) if provenance else 0)
        url=listener.url
        q=SOURCE/'files/question.json'
        recording=work/'recording' if a.mode=='prepare-fixture' else SOURCE/'files/recording'
        receipts=[];results=[];controls=[];task_rows=[]
        labels=producer.read_rows((SOURCE/'files/labels.jsonl').read_text())
        assert [l['id'] for l in labels]==[e['id'] for e in expected]
        for label,e in zip(labels,expected):
            assert label['target_present']==e['target_present'] and label['target_value']==e['target_value']
        def ask(row, mode, store, cut=None, scalar=False, question=q, exit=0, marker=None, extra=None):
            adapted=producer.adapt(row)
            args=['choose','@'+str(question),'--url',url,'--max-retries','0','--timeout','5',
                  '--max-requests-total','1' if mode!='--replay' else '0',
                  '--max-estimated-input-tokens-total','4096' if mode!='--replay' else '0',mode,str(store),'--details']
            if scalar:
                for label,description in adapted['options'].items():args+=['--option',label+'='+description]
                text=row['text']
            else:
                args+=['--jsonl','--field','/text','--options','/options','--batch','1']
                text=json.dumps(adapted)+'\n'
            if cut is not None:args+=['--threshold',str(cut)]
            if extra:args+=extra
            done=run(a.bin,args,env,text,exit,marker)
            receipts.append({'case':row['id'],'stage':listener.stage,'args':[x.replace(str(work),'<owned>').replace(str(SOURCE),'site/examples/recipes/'+RECIPE) for x in args],
                             'exit':done.returncode,'stdout':done.stdout,'stderr':done.stderr})
            if exit not in [0,3]:return None
            parsed=json.loads(done.stdout)
            return parsed
        if a.mode=='prepare-fixture':
            for row in rows:
                if row['candidates']:ask(row,'--record',recording)
            assert listener.count('prepare')==7 and not listener.errors,listener.errors
            run(a.bin,['cache','convert',str(recording)],env)
            shutil.copytree(recording,a.artifacts/'recording')
        listener.stage='replay';listener.ceiling=0
        for row,e in zip(rows,expected):
            if not row['candidates']:
                task_rows.append({'id':row['id'],'status':'local_none','copied_value':None,'candidate_miss':False,'correct':not e['target_present']})
                continue
            r=ask(row,'--replay',recording)
            assert r['value']==e['choice'] and r['meta']['requests_sent']==0
            (controls if row['id'].startswith('N') else results).append(r)
            selected=r['value']
            copied=next((c['value'] for c in row['candidates'] if c['label']==selected),None)
            status='not_sure' if selected is None else 'model_none' if selected=='none' else 'selected'
            candidate_miss=e['target_present'] and e['target_value'] not in [c['value'] for c in row['candidates']]
            correct=not e['target_present'] if status=='model_none' else copied==e['target_value'] if status=='selected' else False
            task_rows.append({'id':row['id'],'status':status,'copied_value':copied,'candidate_miss':candidate_miss,'correct':correct})
            if status=='selected':assert copied==e['target_value'],'exact copied value'
        reconcile(results,expected)
        assert listener.count('replay')==0
        # Scalar reading controls establish exit 3 independently of JSONL exit 0.
        for row in rows[-2:]:
            r=ask(row,'--replay',recording,scalar=True,exit=3)
            assert r['value'] is None
        ordinary_none=ask(rows[2],'--replay',recording,scalar=True,exit=0)
        assert ordinary_none['value']=='none'
        assert listener.count('replay')==0
        jsonl(a.artifacts/'task-results.jsonl',task_rows)
        jsonl(a.artifacts/'results.jsonl',results);jsonl(a.artifacts/'reading-controls.jsonl',controls)
        audit=run(a.bin,['audit',str(a.artifacts/'results.jsonl'),str(SOURCE/'files/key.jsonl'),'--cases'],env)
        audits=producer.read_rows(audit.stdout)
        assert len(audits)==5 and all(r['outcome']=='right' for r in audits)
        (a.artifacts/'audit-cases.jsonl').write_text(audit.stdout)
        # Per-case groups retain variable candidate semantics; no pooled rate claim.
        aggregate=run(a.bin,['audit',str(a.artifacts/'results.jsonl'),str(SOURCE/'files/key.jsonl'),'--by','/id'],env)
        (a.artifacts/'audit.jsonl').write_text(aggregate.stdout)
        # Audit regressions: wrong choice, duplicate identity, missing key, malformed key/input and missing row.
        wrong=json.loads(json.dumps(results));wrong[0]['value']='none'
        jsonl(work/'wrong.jsonl',wrong)
        wrong_audit=run(a.bin,['audit',str(work/'wrong.jsonl'),str(SOURCE/'files/key.jsonl'),'--cases'],env)
        assert json.loads(wrong_audit.stdout.splitlines()[0])['outcome']=='wrong'
        jsonl(work/'duplicate.jsonl',results+[results[0]])
        duplicate=run(a.bin,['audit',str(work/'duplicate.jsonl'),str(SOURCE/'files/key.jsonl')],env,expected=2,marker='repeats a record for one question')
        jsonl(work/'missing-key.jsonl',producer.read_rows((SOURCE/'files/key.jsonl').read_text())[1:])
        missing=run(a.bin,['audit',str(a.artifacts/'results.jsonl'),str(work/'missing-key.jsonl'),'--cases'],env)
        assert json.loads(missing.stdout.splitlines()[0])['outcome']=='unlabeled'
        jsonl(work/'wrong-key.jsonl',[{'id':'R02','value':False}])
        run(a.bin,['audit',str(a.artifacts/'results.jsonl'),str(work/'wrong-key.jsonl')],env,expected=2,marker='choose value that is not text')
        run(a.bin,['audit',str(work/'missing-file'),str(SOURCE/'files/key.jsonl')],env,expected=5,marker='cannot read')
        (work/'bad.jsonl').write_text('{\n')
        run(a.bin,['audit',str(work/'bad.jsonl'),str(SOURCE/'files/key.jsonl')],env,expected=2,marker='results line')
        run(a.bin,['audit',str(a.artifacts/'results.jsonl'),str(work/'bad.jsonl')],env,expected=2,marker='key line')
        try:reconcile(results[1:],expected)
        except AssertionError:pass
        else:raise AssertionError('omitted result plant')
        # Cache fills and cache hits are distinct counted stages, independently bounded.
        cache=work/'cache';listener.stage='cache-fill';listener.ceiling=7
        for row in rows:
            if row['candidates']:ask(row,'--cache',cache)
        assert listener.count('cache-fill')==7 and not listener.errors,listener.errors
        listener.stage='cache-hit';listener.ceiling=0
        for row in rows:
            if row['candidates']:ask(row,'--cache',cache)
        cut_question=work/'cut-question.json'
        changed_cut=json.loads(q.read_text());changed_cut['threshold']=0.5
        dump(cut_question,changed_cut)
        assert digest(cut_question)!=digest(q)
        for row in rows:
            if row['candidates']:
                reread=ask(row,'--cache',cache,question=cut_question)
                assert reread['meta']['requests_sent']==0
                if row['id']=='N02':assert reread['value']=='c001'
                if row['id']=='N01':assert reread['value'] is None
        assert listener.count('cache-hit')==0
        # Replay identity mutations cannot fall through to a backend.
        listener.stage='negative';listener.ceiling=0
        for change in ['wording','model','options','description','evidence']:
            row=json.loads(json.dumps(rows[1]));question=q
            if change in ['wording','model']:
                value=json.loads(q.read_text());value['choose' if change=='wording' else 'model']+=' changed'
                question=work/'changed.json';dump(question,value)
            elif change=='options':row['candidates'][0]['label']='different'
            elif change=='description':row['candidates'][0]['description']+=' changed'
            else:row['text']+=' changed'
            ask(row,'--replay',recording,question=question,exit=5,marker='replay folder holds no answer')
        badq=work/'bad-question.json';dump(badq,{'choose':'Question','recipe_unknown':True})
        ask(rows[1],'--replay',recording,question=badq,exit=5,marker='question file')
        badq.write_text('{\n')
        ask(rows[1],'--replay',recording,question=badq,exit=5,marker='question file')
        args=['choose','@'+str(q),'--jsonl','--field','/text','--options','/options','--batch','1','--url',url,'--replay',str(recording),'--max-requests-total','0']
        run(a.bin,args,env,json.dumps({'id':'bad','text':'Text'})+'\n',expected=2,marker='options')
        run(a.bin,args,env,json.dumps({'id':'bad','text':'Text','options':['same','same']})+'\n',expected=2,marker='option')
        broken=work/'broken';broken.mkdir();(broken/'thinkthen.jsonl').write_text('{\n')
        ask(rows[1],'--replay',broken,exit=5,marker='line 1 is not a question entry')
        conflict=work/'conflict';shutil.copytree(recording,conflict);(conflict/'thinkthen.sqlite').write_bytes(b'not a database')
        ask(rows[1],'--replay',conflict,exit=5,marker='both thinkthen.jsonl and thinkthen.sqlite')
        assert listener.count('negative')==0
        # Invalid producer output never reaches the asking loop; the listener is the no-send oracle.
        invalid=[('exit','import sys;print("{}");sys.exit(1)')]
        for text in ['{', '{"id":"A","id":"B"}', '{}', '[1]']:
            invalid.append(('JSON','print('+repr(text)+')'))
        for mutation in ['missing','type','id','label','control','pool','value','description','order']:
            bad=json.loads(json.dumps(rows))
            if mutation=='missing':del bad[1]['text']
            elif mutation=='type':bad[1]['text']=False
            elif mutation=='id':bad[1]['id']=bad[0]['id']
            elif mutation=='label':bad[3]['candidates'][1]['label']='c001'
            elif mutation=='control':bad[1]['candidates'][0]['label']='bad\n'
            elif mutation=='pool':bad[3]['candidates']*=2
            elif mutation=='value':bad[1]['candidates'][0]['value']='$99.00'
            elif mutation=='description':bad[3]['candidates'][0]['description']=bad[3]['candidates'][1]['description']
            else:bad[3]['candidates'].reverse()
            invalid.append((mutation,'print('+repr('\n'.join(json.dumps(r) for r in bad))+')'))
        for name,code in invalid:
            try:producer.produce([sys.executable,'-c',code])
            except producer.CandidateError:pass
            else:raise AssertionError('producer plant '+name)
        assert listener.count('negative')==0
        after={str(p.relative_to(SOURCE)):digest(p) for p in SOURCE.rglob('*') if p.is_file()}
        assert before==after,'source fixture mutation'
        ordinary=[t for t in task_rows if t['id'].startswith('R')]
        counts={'population':len(task_rows),'ordinary':len(ordinary),'asking':len(results),
                **{name:sum(t['status']==name for t in task_rows) for name in ['local_none','selected','model_none','not_sure','failed']},
                'candidate_miss':sum(t['candidate_miss'] for t in task_rows),'ordinary_correct':sum(t['correct'] for t in ordinary),
                'preparation_requests':listener.count('prepare'),'cache_fill_requests':listener.count('cache-fill'),
                'replay_requests':listener.count('replay'),'cache_hit_requests':listener.count('cache-hit'),'negative_requests':listener.count('negative'),
                'cumulative_synthetic_requests':len(listener.requests),'producer_negative_controls':len(invalid),'audit_rows':len(audits)}
        assert {k:counts[k] for k in ['population','ordinary','asking','local_none','selected','model_none','not_sure','failed','candidate_miss','ordinary_correct','audit_rows']}=={
            'population':8,'ordinary':6,'asking':5,'local_none':1,'selected':2,'model_none':3,'not_sure':2,'failed':0,'candidate_miss':1,'ordinary_correct':5,'audit_rows':5}
        assert counts['cumulative_synthetic_requests']==(14 if a.mode=='prepare-fixture' else 7)
        dump(a.artifacts/'harness.json',counts);dump(a.artifacts/'receipts.json',receipts)
        record_path=a.artifacts/'recording/thinkthen.jsonl' if a.mode=='prepare-fixture' else recording/'thinkthen.jsonl'
        stored=producer.read_rows(record_path.read_text());keys=[r['key'] for r in stored if 'question' in r]
        provenance={'evidence_class':'controlled-loopback-fixture','owning_record':'sdlc/records/2026-10-05-reviewed-experiment-planning.md',
                    'owning_commit':'ea615c0e6a3412bb4716ad97b704a3f698be30cb','data':'Handwritten generic synthetic purchase notes',
                    'address':url,'context':None,'requested_model':'recipe-fixture-v1','returned_model':'recipe-fixture-v1',
                    'harness_source_commit':subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip(),'executable_sha256':digest(a.bin),'executable_version':run(a.bin,['--version'],env).stdout.strip(),
                    'preparation_source_files' if a.mode=='prepare-fixture' else 'replay_source_files':before,
                    'harness_sha256':digest(Path(__file__)),
                    'files':before,'recording_sha256':digest(record_path),'stored_question_keys':keys,'counts':counts,
                    'cut_file_sha256':digest(cut_question),'original_question_sha256':digest(q),
                    'reading_rules':'cut 0.75; exact tie and below-cut are not sure; completed JSONL run exits 0; scalar null exits 3',
                    'stage':a.mode,'receipt_hashes':{p.name:digest(p) for p in a.artifacts.iterdir() if p.is_file()},
                    'input_attachment':'CLI JSONL retains immutable input; no invented distribution or repaired failure'}
        dump(a.artifacts/'provenance.json',provenance)
        print(json.dumps(counts,sort_keys=True))
    finally:
        if listener:
            if a.artifacts.exists():
                dump(a.artifacts/'listener-counts.json',{'stage_counts':{stage:listener.count(stage) for stage in ['prepare','replay','cache-fill','cache-hit','negative']},'total':len(listener.requests),'errors':listener.errors})
            listener.close()
        scratch.cleanup(scratch.owned)

if __name__=='__main__':
    main()
