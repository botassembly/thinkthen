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
    db.execute('INSERT INTO result VALUES (?)',[result])
    typed=db.execute("SELECT json_extract(value,'$.facts.call_id'),CAST(json_extract(value,'$.facts.requests_sent') AS INTEGER),json_extract(value,'$.value[0].schema'),json_extract(value,'$.error.kind') FROM result").fetchone()
    decoded=json.loads(result)
    if 'error' not in decoded:
        assert isinstance(typed[0],str) and len(typed[0])==64 and isinstance(typed[1],int)
        assert typed[2] in (None,'thinkthen.result/2')
        for row in db.execute("SELECT json_extract(j.value,'$.answer_id'),CAST(json_extract(j.value,'$.answer.probability') AS REAL),json_extract(j.value,'$.meta.origin') FROM result,json_each(CASE json_type(result.value,'$.value') WHEN 'array' THEN json_extract(result.value,'$.value') ELSE json_array(json_extract(result.value,'$.value')) END) j"):
            if row[0] is not None:assert isinstance(row[0],str) and len(row[0])==64
            if row[1] is not None:assert isinstance(row[1],float)
    else:assert isinstance(typed[3],str)
    print(result,flush=True)
except sqlite3.Error as error:
    print(json.dumps({'host_error':str(error),'host_code':error.sqlite_errorcode}),flush=True)
