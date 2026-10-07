"""Actual named SQL consumer; extract known fields with SQL's typed JSON operations."""
import json,sys,sqlite3,threading
payload=json.loads(sys.stdin.readline())
db=sqlite3.connect(':memory:',isolation_level=None,check_same_thread=False)
db.enable_load_extension(True)
db.load_extension(sys.argv[1])
settings=payload['engine_settings']

if payload.get('held_cancel'):
    def cancel():
        sys.stdin.readline();db.interrupt();print('cancel-fired',flush=True)
    threading.Thread(target=cancel,daemon=True).start()
name='thinkthen_'+payload['verb']+'_complete'
try:
    db.execute('SELECT thinkthen_configure(?)',[json.dumps(settings)])
    result=db.execute('SELECT '+name+'(?,?,?)',[payload['question'],json.dumps(payload['inputs'],ensure_ascii=False,separators=(',',':')),json.dumps(payload.get('controls',{}))]).fetchone()[0]
    # Known complete facts and discriminators are SQL INTEGER/REAL/TEXT fields.
    db.execute('CREATE TEMP TABLE result(value TEXT)')
    db.execute("INSERT INTO result SELECT json_extract(?,'$.native')",[result])
    typed=db.execute("SELECT json_extract(value,'$.facts.call_id'),CAST(json_extract(value,'$.facts.requests_sent') AS INTEGER),json_extract(value,'$.value[0].schema'),json_extract(value,'$.error.kind') FROM result").fetchone()
    decoded=json.loads(result)['native']
    if db.execute("SELECT json_type(value,'$.facts') FROM result").fetchone()[0] is not None:
        for key in ('requests_sent','records','cache_answers'):
            kind,count=db.execute("SELECT json_type(value,?),CAST(json_extract(value,?) AS INTEGER) FROM result",['$.facts.'+key]*2).fetchone()
            assert kind=='integer' and count>=0
        kind,identity=db.execute("SELECT json_type(value,'$.facts.call_id'),json_extract(value,'$.facts.call_id') FROM result").fetchone()
        assert kind=='text' and len(identity)==64
        for key,types in (('seconds',('integer','real')),('input_tokens',('integer',)),('output_tokens',('integer',)),('estimated_cost_usd',('text',)),('attempts',('array',))):
            kind=db.execute("SELECT json_type(value,?) FROM result",['$.facts.'+key]).fetchone()[0]
            assert kind is None or kind in types
    if 'error' in decoded:
        kind,retry,stop=db.execute("SELECT json_type(value,'$.error.kind'),json_type(value,'$.error.retryable'),json_type(value,'$.error.stopped.cause') FROM result").fetchone()
        assert kind=='text' and retry in ('true','false') and stop=='text'
    if 'error' not in decoded:
        assert isinstance(typed[0],str) and len(typed[0])==64 and isinstance(typed[1],int)
        assert typed[2] in (None,'thinkthen.result/2')
        assert not db.execute("SELECT 1 FROM result,json_each(CASE json_type(result.value,'$.value') WHEN 'array' THEN json_extract(result.value,'$.value') ELSE json_array(json_extract(result.value,'$.value')) END) j WHERE json_type(j.value,'$.answer_id') <> 'text' OR json_extract(j.value,'$.schema') <> 'thinkthen.result/2'").fetchall()
        for row in db.execute("SELECT json_extract(j.value,'$.answer_id'),CAST(json_extract(j.value,'$.answer.probability') AS REAL),json_extract(j.value,'$.meta.origin') FROM result,json_each(CASE json_type(result.value,'$.value') WHEN 'array' THEN json_extract(result.value,'$.value') ELSE json_array(json_extract(result.value,'$.value')) END) j"):
            if row[0] is not None:assert isinstance(row[0],str) and len(row[0])==64
            if row[1] is not None:assert isinstance(row[1],float)
        for member, in db.execute("SELECT m.value FROM result,json_each(CASE json_type(result.value,'$.value') WHEN 'array' THEN json_extract(result.value,'$.value') ELSE '[]' END) r,json_each(json_extract(r.value,'$.members')) m ORDER BY r.key,m.key"):
            for path,kind in (('$.name','text'),('$.result.value','integer'),('$.result.answer_id','text'),('$.result.question.name','text'),('$.result.meta.model','text'),('$.result.meta.context_sha256','text'),('$.result.meta.usage.input_tokens','integer'),('$.result.meta.question_sources[0].batch_size','integer')):
                assert db.execute('SELECT json_type(?,?)',[member,path]).fetchone()[0]==kind
            position,identity,probability=db.execute("SELECT CAST(json_extract(?,'$.result.value') AS INTEGER),json_extract(?,'$.result.answer_id'),CAST(json_extract(?,'$.result.answer.probability') AS REAL)",[member]*3).fetchone()
            assert position>0 and len(identity)==64 and isinstance(probability,float)
    else:assert isinstance(typed[3],str)
    print(result,flush=True)
except sqlite3.Error as error:
    print(json.dumps({'host_error':str(error),'host_code':error.sqlite_errorcode}),flush=True)
