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
DO $check$ DECLARE d jsonb; r jsonb; m jsonb; c jsonb; n bigint; p double precision; k text;
BEGIN
 SELECT document::jsonb->'native' INTO d FROM complete_result;
 IF d ? 'facts' THEN
  FOREACH k IN ARRAY ARRAY['requests_sent','records','cache_answers'] LOOP
   IF jsonb_typeof(d->'facts'->k) IS DISTINCT FROM 'number' THEN RAISE EXCEPTION 'invalid native count type'; END IF;
   n := (d->'facts'->>k)::bigint;
   IF n < 0 THEN RAISE EXCEPTION 'negative native count'; END IF;
  END LOOP;
  IF jsonb_typeof(d->'facts'->'call_id') IS DISTINCT FROM 'string' OR length(d->'facts'->>'call_id') <> 64 THEN RAISE EXCEPTION 'invalid native call identity'; END IF;
  FOREACH k IN ARRAY ARRAY['seconds','input_tokens','output_tokens'] LOOP
   IF d->'facts' ? k AND jsonb_typeof(d->'facts'->k) <> 'number' THEN RAISE EXCEPTION 'invalid native numeric fact'; END IF;
  END LOOP;
  IF d->'facts' ? 'estimated_cost_usd' AND jsonb_typeof(d->'facts'->'estimated_cost_usd') <> 'string' THEN RAISE EXCEPTION 'invalid native price'; END IF;
  IF d->'facts' ? 'attempts' AND jsonb_typeof(d->'facts'->'attempts') <> 'array' THEN RAISE EXCEPTION 'invalid native attempts'; END IF;
 END IF;
 IF d ? 'error' THEN
  IF jsonb_typeof(d->'error'->'kind') IS DISTINCT FROM 'string' OR jsonb_typeof(d->'error'->'retryable') IS DISTINCT FROM 'boolean' OR jsonb_typeof(d->'error'->'stopped'->'cause') IS DISTINCT FROM 'string' THEN RAISE EXCEPTION 'missing typed native error'; END IF;
 ELSE
  n := (d->'facts'->>'requests_sent')::bigint;
  IF n IS NULL OR length(d->'facts'->>'call_id') <> 64 THEN RAISE EXCEPTION 'missing typed native facts'; END IF;
  FOR r IN SELECT value FROM jsonb_array_elements(CASE jsonb_typeof(d->'value') WHEN 'array' THEN d->'value' ELSE jsonb_build_array(d->'value') END) LOOP
   IF r->>'schema' <> 'thinkthen.result/2' OR length(r->>'answer_id') <> 64 THEN RAISE EXCEPTION 'missing native result identity'; END IF;
   IF r->'answer' ? 'probability' AND jsonb_typeof(r->'answer'->'probability') NOT IN ('number','null') THEN RAISE EXCEPTION 'invalid native probability'; END IF;
   p := (r->'answer'->>'probability')::double precision;
   IF r ? 'members' THEN
    FOR m IN SELECT value FROM jsonb_array_elements(r->'members') LOOP
     c := m->'result';
     IF jsonb_typeof(m->'name') IS DISTINCT FROM 'string' OR jsonb_typeof(c->'value') IS DISTINCT FROM 'number' OR (c->>'value')::bigint < 1 OR jsonb_typeof(c->'answer_id') IS DISTINCT FROM 'string' OR length(c->>'answer_id') <> 64 THEN RAISE EXCEPTION 'invalid native member identity or position'; END IF;
     IF jsonb_typeof(c->'question'->'name') IS DISTINCT FROM 'string' OR jsonb_typeof(c->'meta'->'model') IS DISTINCT FROM 'string' OR jsonb_typeof(c->'meta'->'context_sha256') IS DISTINCT FROM 'string' THEN RAISE EXCEPTION 'invalid native member metadata'; END IF;
     IF jsonb_typeof(c->'meta'->'usage'->'input_tokens') IS DISTINCT FROM 'number' OR jsonb_typeof(c->'meta'->'question_sources'->0->'batch_size') IS DISTINCT FROM 'number' OR jsonb_typeof(c->'answer'->'probability') IS DISTINCT FROM 'number' THEN RAISE EXCEPTION 'invalid native member usage, source or probability'; END IF;
     n := (c->'meta'->'usage'->>'input_tokens')::bigint;
     n := (c->'meta'->'question_sources'->0->>'batch_size')::bigint;
     p := (c->'answer'->>'probability')::double precision;
    END LOOP;
   END IF;
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
