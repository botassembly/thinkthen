"""Actual PostgreSQL native typed calls projected to the canonical fixture contract."""
import json,subprocess,sys,threading
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[4]/'conformance/children'))
from children import child_env, read_process_line
sys.path.insert(0,str(Path(__file__).resolve().parents[3]/'postgresql/tests'))
from native_inputs import query as native_query, AdmissionError
from typed_results import native
payload=json.loads(sys.stdin.readline())
def lit(value):return "'"+str(value).replace("'","''")+"'"
command=['psql','-X','-q','-At','-v','ON_ERROR_STOP=1','-v','VERBOSITY=verbose','-h',sys.argv[1],'-U','postgres','-d','postgres']
settings=[]
for key,value in payload['engine_settings'].items():
    if key=='cache' and value is False:value='off'
    if key in ('max_requests','max_requests_total','max_request_bytes') and value is None:value=-1
    if key=='refresh_cache':value='on' if value else 'off'
    settings.append('SET thinkthen.'+key+'='+lit(value)+';')
try:
    query=native_query(payload)
except AdmissionError as error:
    print(json.dumps({'native':{'error':error.error}}),flush=True)
    raise SystemExit(0)
script='\n'.join(settings)+'\nSELECT pg_backend_pid();\nSELECT to_jsonb(r) FROM '+query+' r;\n'
running=subprocess.Popen(command,env=child_env(keep=('HOME','LANG','LC_ALL')),stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
running.stdin.write(script);running.stdin.close();running.stdin=None
pid=read_process_line(running.stdout).strip()
if payload.get('held_cancel'):
    def cancel():
        sys.stdin.readline()
        done=subprocess.run(command+['-c','SELECT pg_cancel_backend('+str(int(pid))+')'],env=child_env(keep=('HOME','LANG','LC_ALL')),text=True,capture_output=True)
        assert done.returncode==0 and done.stdout.strip()=='t'
        print('cancel-fired',flush=True)
    threading.Thread(target=cancel,daemon=True).start()
stdout,stderr=running.communicate(timeout=120)
if running.returncode:
    print(json.dumps({'host_error':stderr,'host_cancelled':'57014:' in stderr}),flush=True)
else:
    rows=[json.loads(line) for line in stdout.splitlines()]
    print(json.dumps(native(rows,payload['verb']),ensure_ascii=False,separators=(',',':')),flush=True)
