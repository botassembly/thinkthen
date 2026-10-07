"""Actual DuckDB complete functions with typed SQL checks on native JSON fields."""
import json,sys,threading
import duckdb
payload=json.loads(sys.stdin.readline())
db=duckdb.connect(config={'allow_unsigned_extensions':True,'autoinstall_known_extensions':False})
def literal(value):return "'"+str(value).replace("'","''")+"'"
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
    result=db.execute('SELECT thinkthen_'+payload['verb']+'_complete(?,?,?)',[payload['question'],json.dumps(payload['inputs'],ensure_ascii=False,separators=(',',':')),json.dumps(payload.get('controls',{}))]).fetchone()[0]
    db.execute('CREATE TEMP TABLE result(value JSON)')
    db.execute('INSERT INTO result VALUES (?)',[result])
    typed=db.execute("SELECT json_extract_string(value,'$.facts.call_id'),CAST(json_extract(value,'$.facts.requests_sent') AS BIGINT),json_extract_string(value,'$.error.kind') FROM result").fetchone()
    decoded=json.loads(result)
    if 'error' not in decoded:
        assert isinstance(typed[0],str) and len(typed[0])==64 and isinstance(typed[1],int)
        for row in db.execute("SELECT json_extract_string(j.value,'$.answer_id'),CAST(json_extract(j.value,'$.answer.probability') AS DOUBLE),json_extract_string(j.value,'$.schema') FROM result,json_each(CASE json_type(result.value,'$.value') WHEN 'ARRAY' THEN json_extract(result.value,'$.value') ELSE json_array(json_extract(result.value,'$.value')) END) j").fetchall():
            assert isinstance(row[0],str) and len(row[0])==64 and row[2]=='thinkthen.result/2'
            if row[1] is not None:assert isinstance(row[1],float)
    else:assert isinstance(typed[2],str)
    print(result,flush=True)
except duckdb.Error as error:
    print(json.dumps({'host_error':str(error),'host_cancelled':isinstance(error,duckdb.InterruptException)}),flush=True)
