import concurrent.futures, datetime, json, pathlib, subprocess, time, os, sys
D=pathlib.Path(sys.argv[2]).resolve()
H=sys.argv[1]
R='repos/botassembly/thinkthen'
B='ticket/0543-final'
def save(name,obj):
 p=D/(name+'.tmp'); p.write_text(json.dumps(obj,indent=2)+'\n'); os.replace(p,D/name)
def gh(*args):
 p=subprocess.run(['gh',*args],capture_output=True,text=True)
 if p.returncode: raise RuntimeError(str(args)+': '+p.stderr)
 return p.stdout
def api(path): return json.loads(gh('api',path))
work=[('windows','windows.yml',{'candidate_sha':H,'lane':'windows'}),('swift','swift-lane.yml',{'language':'swift'}),('objective-c','objective-c.yml',{'website_examples':False})]
start=datetime.datetime.now(datetime.timezone.utc).isoformat()
save('result.json',{'sha':H,'branch':B,'state':'dispatching','all_watches_reaped':False})
def dispatch(w):
 name,workflow,inputs=w
 payload={'ref':B,'inputs':inputs}; save(name+'.dispatch.json',payload)
 p=subprocess.run(['gh','api','--method','POST',R+'/actions/workflows/'+workflow+'/dispatches','--input','-'],input=json.dumps(payload),capture_output=True,text=True)
 save(name+'.dispatch-result.json',{'exit':p.returncode,'stdout':p.stdout,'stderr':p.stderr,'payload':payload,'time':start})
 if p.returncode: raise RuntimeError(name+' dispatch failed: '+p.stderr)
 for attempt in range(60):
  runs=api(R+'/actions/workflows/'+workflow+'/runs?event=workflow_dispatch&per_page=30')['workflow_runs']
  matches=[r for r in runs if r['head_branch']==B and r['head_sha']==H and r['created_at']>=start[:19]+'Z']
  if matches:
   run=max(matches,key=lambda r:r['id']); save(name+'.accepted.json',run)
   jobs=api(R+'/actions/runs/'+str(run['id'])+'/jobs'); save(name+'.jobs-initial.json',jobs)
   print(json.dumps({'accepted':name,'run_id':run['id'],'head_sha':run['head_sha'],'runner_labels':[j['labels'] for j in jobs['jobs']],'url':run['html_url']}),flush=True)
   return name,run
  time.sleep(2)
 raise RuntimeError(name+' no matching exact-head run found')
accepted={}
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as ex:
 futures=[ex.submit(dispatch,w) for w in work]
 for f in concurrent.futures.as_completed(futures):
  try:
   name,run=f.result(); accepted[name]=run
  except Exception as e:
   print('DISPATCH_ERROR '+str(e),flush=True)
save('runs.json',{name:r['id'] for name,r in accepted.items()})
save('result.json',{'sha':H,'branch':B,'state':'waiting_for_completion','all_watches_reaped':False,'runs':{name:r['id'] for name,r in accepted.items()}})
def wait_finished(name,initial):
 runid=initial['id']; deadline=time.monotonic()+10800
 while time.monotonic()<deadline:
  try: run=api(R+'/actions/runs/'+str(runid))
  except Exception as e:
   with (D/(name+'.wait-errors.log')).open('a') as log: log.write(str(e)+'\n')
   time.sleep(60); continue
  if run['status']=='completed': break
  time.sleep(60)
 else: raise RuntimeError(name+' completion wait expired')
 save(name+'.completed.json',run)
 jobs=api(R+'/actions/runs/'+str(runid)+'/jobs?per_page=100'); save(name+'.jobs-completed.json',jobs)
 with (D/(name+'.finished.log')).open('w') as log:
  p=subprocess.run(['gh','run','view',str(runid),'--repo','botassembly/thinkthen','--log'],stdout=log,stderr=subprocess.STDOUT,text=True)
 result={'run_id':runid,'url':run['html_url'],'head_sha':run['head_sha'],'conclusion':run['conclusion'],'exit':0 if run['head_sha']==H and run['conclusion']=='success' else 77 if run['head_sha']!=H or run['conclusion']=='cancelled' else 1,'completed_log_exit':p.returncode,'jobs':[{'name':j['name'],'labels':j['labels'],'runner_name':j['runner_name'],'runner_group_name':j['runner_group_name'],'conclusion':j['conclusion'],'failed_steps':[s['name'] for s in j['steps'] if s['conclusion']=='failure']} for j in jobs['jobs']]}
 save(name+'.watch-result.json',result)
 print('COMPLETED '+json.dumps(result),flush=True)
 return name,result
results={};errors=[]
with concurrent.futures.ThreadPoolExecutor(max_workers=3) as ex:
 futures=[ex.submit(wait_finished,name,run) for name,run in accepted.items()]
 for f in concurrent.futures.as_completed(futures):
  try:
   name,result=f.result();results[name]=result
  except Exception as e: errors.append(str(e));print('WAIT_ERROR '+str(e),flush=True)
save('result.json',{'sha':H,'branch':B,'state':'completed','all_watches_reaped':True,'scope':'Windows own-platform checks and ARM macOS installed Swift/Foundation with cross-built Intel Swift framework; not full Intel macOS qualification or release rehearsal','runs':results,'errors':errors,'exit':0 if len(results)==3 and all(r['exit']==0 for r in results.values()) else 77 if len(results)!=3 or errors or any(r['exit']==77 for r in results.values()) else 1})
print('STOP all completion waits reaped',flush=True)
sys.exit(json.loads((D/'result.json').read_text())['exit'])
