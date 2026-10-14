"""Actual DuckDB complete functions with typed SQL checks on native JSON fields."""
import json,sys,threading,os,subprocess,signal
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'duckdb/tools'))
from receipt_adapter import carrier
payload=json.loads(sys.stdin.readline())
def literal(value):return "'"+str(value).replace("'","''")+"'"
if os.environ.get('THINKTHEN_DUCKDB_NATIVE_CLI'):
    statements=['LOAD '+literal(sys.argv[1])]
    for key,value in payload['engine_settings'].items():
        if key=='cache' and value is False:value='off'
        if key=='refresh_cache':value=int(value)
        statements.append('SET thinkthen_'+key+'='+('NULL' if value is None else literal(value)))
    statements.append('CREATE TEMP TABLE native_receipt AS SELECT thinkthen_'+payload['verb']+'_complete('+','.join(literal(value) for value in (payload['question'],json.dumps(payload['inputs'],ensure_ascii=False,separators=(',',':')),json.dumps(payload.get('controls',{}))))+') AS receipt')
    statements.append('SELECT receipt,typeof(receipt) AS native_type,receipt.facts.call_id AS call_id,receipt.facts.requests_sent::BIGINT AS sends,receipt.failure.kind AS kind FROM native_receipt')
    process=subprocess.Popen([os.environ['THINKTHEN_DUCKDB_NATIVE_CLI'],'-unsigned','-json','-c',';'.join(statements)],stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
    if payload.get('held_cancel'):
        def cancel_cli():
            sys.stdin.readline();process.send_signal(signal.SIGINT);print('cancel-fired',flush=True)
        threading.Thread(target=cancel_cli,daemon=True).start()
    stdout,stderr=process.communicate(timeout=120)
    if process.returncode:
        print(json.dumps({'host_error':stderr,'host_cancelled':'Interrupted' in stderr or 'interrupt' in stderr.lower()}),flush=True)
    else:
        row,=json.loads(stdout)
        assert row['native_type'].startswith('STRUCT(')
        assert row['call_id'] is None or isinstance(row['call_id'],str) and len(row['call_id'])==64
        assert row['sends'] is None or isinstance(row['sends'],int) and row['sends']>=0
        assert row['kind'] is None or isinstance(row['kind'],str)
        print(json.dumps(carrier(row['receipt'],payload['verb']),ensure_ascii=False,separators=(',',':')),flush=True)
    sys.exit(0)
import duckdb
db=duckdb.connect(config={'allow_unsigned_extensions':True,'autoinstall_known_extensions':False})
db.execute('LOAD '+literal(sys.argv[1]))
if payload.get('held_cancel'):
    def cancel():
        sys.stdin.readline();db.interrupt();print('cancel-fired',flush=True)
    threading.Thread(target=cancel,daemon=True).start()
try:
    for key,value in payload['engine_settings'].items():
        if key=='cache' and value is False:value='off'
        if key=='refresh_cache':value=int(value)
        db.execute('SET thinkthen_'+key+'='+('NULL' if value is None else literal(value)))
    db.execute('CREATE TEMP TABLE native_receipt AS SELECT thinkthen_'+payload['verb']+'_complete(?,?,?) AS receipt',[payload['question'],json.dumps(payload['inputs'],ensure_ascii=False,separators=(',',':')),json.dumps(payload.get('controls',{}))])
    receipt=db.execute('SELECT receipt FROM native_receipt').fetchone()[0]
    assert isinstance(receipt,dict)
    assert 'STRUCT' in db.execute('DESCRIBE native_receipt').fetchone()[1]
    call_id,sends,kind=db.execute('SELECT receipt.facts.call_id,receipt.facts.requests_sent,receipt.failure.kind FROM native_receipt').fetchone()
    assert call_id is None or isinstance(call_id,str) and len(call_id)==64
    assert sends is None or isinstance(sends,int) and sends>=0
    assert kind is None or isinstance(kind,str)
    result=json.dumps(carrier(receipt,payload['verb'],json_text=True),ensure_ascii=False,separators=(',',':'))
    db.execute('CREATE TEMP TABLE result(value JSON)')
    db.execute("INSERT INTO result SELECT json_extract(?,'$.native')",[result])
    typed=db.execute("SELECT json_extract_string(value,'$.facts.call_id'),CAST(json_extract(value,'$.facts.requests_sent') AS BIGINT),json_extract_string(value,'$.error.kind') FROM result").fetchone()
    decoded=json.loads(result)['native']
    if db.execute("SELECT json_type(value,'$.facts') FROM result").fetchone()[0] is not None:
        for key in ('requests_sent','records','cache_answers'):
            kind,count=db.execute("SELECT json_type(value,?),CAST(json_extract(value,?) AS UBIGINT) FROM result",['$.facts.'+key]*2).fetchone()
            assert kind in ('UBIGINT','BIGINT') and count>=0
        kind,identity=db.execute("SELECT json_type(value,'$.facts.call_id'),json_extract_string(value,'$.facts.call_id') FROM result").fetchone()
        assert kind=='VARCHAR' and len(identity)==64
        for key,types in (('seconds',('DOUBLE','UBIGINT','BIGINT')),('input_tokens',('UBIGINT','BIGINT')),('output_tokens',('UBIGINT','BIGINT')),('estimated_cost_usd',('VARCHAR',)),('attempts',('ARRAY',))):
            kind=db.execute("SELECT json_type(value,?) FROM result",['$.facts.'+key]).fetchone()[0]
            assert kind is None or kind in types
    if 'error' in decoded:
        kind,retry,stop=db.execute("SELECT json_type(value,'$.error.kind'),json_type(value,'$.error.retryable'),json_type(value,'$.error.stopped.cause') FROM result").fetchone()
        assert kind=='VARCHAR' and retry=='BOOLEAN' and stop=='VARCHAR'
    if 'error' not in decoded:
        assert isinstance(typed[0],str) and len(typed[0])==64 and isinstance(typed[1],int)
        for row in db.execute("SELECT json_extract_string(j.value,'$.answer_id'),CAST(json_extract(j.value,'$.answer.probability') AS DOUBLE),json_extract_string(j.value,'$.schema') FROM result,json_each(CASE json_type(result.value,'$.value') WHEN 'ARRAY' THEN json_extract(result.value,'$.value') ELSE json_array(json_extract(result.value,'$.value')) END) j").fetchall():
            assert isinstance(row[0],str) and len(row[0])==64 and row[2]=='thinkthen.result/2'
            if row[1] is not None:assert isinstance(row[1],float)
        for member, in db.execute("SELECT m.value FROM result,json_each(CASE json_type(result.value,'$.value') WHEN 'ARRAY' THEN json_extract(result.value,'$.value') ELSE '[]' END) r,json_each(json_extract(r.value,'$.members')) m ORDER BY r.key::BIGINT,m.key::BIGINT").fetchall():
            for path,types in (('$.name',('VARCHAR',)),('$.result.value',('UBIGINT','BIGINT')),('$.result.answer_id',('VARCHAR',)),('$.result.question.name',('VARCHAR',)),('$.result.meta.model',('VARCHAR',)),('$.result.meta.context_sha256',('VARCHAR',)),('$.result.meta.usage.input_tokens',('UBIGINT','BIGINT')),('$.result.meta.question_sources[0].batch_size',('UBIGINT','BIGINT'))):
                assert db.execute('SELECT json_type(?::JSON,?)',[member,path]).fetchone()[0] in types
            position,identity,probability=db.execute("SELECT CAST(json_extract(?::JSON,'$.result.value') AS UBIGINT),json_extract_string(?::JSON,'$.result.answer_id'),CAST(json_extract(?::JSON,'$.result.answer.probability') AS DOUBLE)",[member]*3).fetchone()
            assert position>0 and len(identity)==64 and isinstance(probability,float)
    else:assert isinstance(typed[2],str)
    print(result,flush=True)
except duckdb.Error as error:
    print(json.dumps({'host_error':str(error),'host_cancelled':isinstance(error,duckdb.InterruptException)}),flush=True)
