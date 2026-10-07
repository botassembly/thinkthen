"""Actual PostgreSQL named complete calls and typed SQL JSON field checks."""
import json,subprocess,sys,threading
payload=json.loads(sys.stdin.readline())
def lit(value):return "'"+str(value).replace("'","''")+"'"
command=['psql','-X','-q','-At','-v','ON_ERROR_STOP=1','-v','VERBOSITY=verbose','-h',sys.argv[1],'-U','postgres','-d','postgres']
settings=[]
for key,value in payload['engine_settings'].items():
    if key=='cache' and value is False:value='off'
    if key in ('max_requests','max_requests_total','max_request_bytes') and value is None:value=-1
    if key=='refresh_cache':value='on' if value else 'off'
    settings.append('SET thinkthen.'+key+'='+lit(value)+';')
name='thinkthen_'+payload['verb']+'_complete'
query=name+'('+','.join(map(lit,[payload['question'],json.dumps(payload['inputs'],ensure_ascii=False,separators=(',',':')),json.dumps(payload.get('controls',{}))]))+')'
checks="""
DO $check$ DECLARE d jsonb; r jsonb; n bigint; p double precision;
BEGIN
 SELECT document::jsonb INTO d FROM complete_result;
 IF d ? 'error' THEN
  IF jsonb_typeof(d->'error'->'kind') <> 'string' THEN RAISE EXCEPTION 'missing typed native error'; END IF;
 ELSE
  n := (d->'facts'->>'requests_sent')::bigint;
  IF n IS NULL OR length(d->'facts'->>'call_id') <> 64 THEN RAISE EXCEPTION 'missing typed native facts'; END IF;
  FOR r IN SELECT value FROM jsonb_array_elements(CASE jsonb_typeof(d->'value') WHEN 'array' THEN d->'value' ELSE jsonb_build_array(d->'value') END) LOOP
   IF r->>'schema' <> 'thinkthen.result/2' OR length(r->>'answer_id') <> 64 THEN RAISE EXCEPTION 'missing native result identity'; END IF;
   p := (r->'answer'->>'probability')::double precision;
  END LOOP;
 END IF;
END $check$;
SELECT document FROM complete_result;
"""
script='\n'.join(settings)+'\nSELECT pg_backend_pid();\nCREATE TEMP TABLE complete_result AS SELECT '+query+' AS document;\n'+checks
running=subprocess.Popen(command,stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
running.stdin.write(script);running.stdin.close();running.stdin=None
pid=running.stdout.readline().strip()
if payload.get('held_cancel'):
    def cancel():
        sys.stdin.readline()
        done=subprocess.run(command+['-c','SELECT pg_cancel_backend('+str(int(pid))+')'],text=True,capture_output=True)
        assert done.returncode==0 and done.stdout.strip()=='t'
        print('cancel-fired',flush=True)
    threading.Thread(target=cancel,daemon=True).start()
stdout,stderr=running.communicate(timeout=120)
if running.returncode:
    print(json.dumps({'host_error':stderr,'host_cancelled':'57014:' in stderr}),flush=True)
else:
    decoded=json.loads(stdout);assert isinstance(decoded,dict)
    print(stdout.strip(),flush=True)
