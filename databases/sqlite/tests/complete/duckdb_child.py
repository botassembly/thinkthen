"""Ordinary DuckDB functions execute native SQL records and typed receipt rows."""
import json,sys,threading
from pathlib import Path
sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'duckdb/tools'))
from receipt_adapter import carrier
payload=json.loads(sys.stdin.readline())
def literal(value):return "'"+str(value).replace("'","''")+"'"
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
    bind = [payload['question']]
    inputs = payload['inputs']
    def argument(value, cast=''):
        bind.append(value)
        return '?' + cast
    def record_expression(record):
        if 'read_error' in record:
            return "{'read_error':" + argument(record['read_error']) + "}"
        members = []
        for key in ('text','document','context','options','examples','seed_spans','source'):
            if key in record:
                members.append(key + ' := ' + (argument(json.dumps(record[key]),'::JSON')
                               if key in ('context','options','examples','seed_spans') else argument(record[key])))
        if 'json' in record or 'json_text' in record:
            original = record.get('json_text') if 'json_text' in record else json.dumps(record['json'],ensure_ascii=False,separators=(',',':'))
            members.append('json := ' + argument(original, '::JSON'))
        if 'images' in record:
            images = []
            for image in record['images']:
                images.append('thinkthen_image(' + argument(bytes(image['bytes']), '::BLOB') + ',' + argument(image['media']) + ')')
            members.append('images := [' + ','.join(images) + ']')
        return 'thinkthen_record(' + ','.join(members) + ')'
    if 'files' in inputs:
        files = inputs['files']
        options = files.get('options', {})
        physical = options.get('reading', {})
        collection = 'thinkthen_source(' + argument(files['paths']) + ',unit := ' + argument(physical.get('unit','line'))
        if 'window' in physical:
            collection += ',"window" := ' + argument(physical['window'])
        if 'format' in files:
            collection += ',format := ' + argument(files['format'])
        collection += ',media := ' + argument(options.get('media','text')) + ')'
    else:
        collection = '[' + ','.join(record_expression(record) for record in inputs['records']) + ']'
    reading = dict(inputs.get('reading', {}))
    for original, canonical in [('fields','field'),('context','context_field'),('options','options_field'),('examples','examples_field'),('seed_spans','seed_spans_field')]:
        if original in reading:
            reading[canonical] = reading.pop(original)
    reading['attempts'] = inputs.get('attempts',False)
    reading_sql = 'thinkthen_reading(' + ','.join('"'+key+'" := '+argument(value) for key,value in reading.items()) + ')'
    settings = argument(json.dumps(payload.get('controls',{})))
    incremental = argument(inputs.get('incremental',False))
    cancelled = argument(inputs.get('cancelled',False))
    db.execute('CREATE TEMP TABLE native_rows AS SELECT detail.* FROM (SELECT UNNEST(thinkthen_'+payload['verb']+
               '_details(question := ?,input := '+collection+',reading := '+reading_sql+
               ',settings := '+settings+',incremental := '+incremental+',cancelled := '+cancelled+')) AS detail)', bind)
    columns = [row[0] for row in db.execute('DESCRIBE native_rows').fetchall()]
    cursor = db.execute('SELECT * FROM native_rows ORDER BY ordinal NULLS LAST')
    actual = []
    while (row := cursor.fetchone()) is not None:
        actual.append(dict(zip(columns,row)))
    if not actual:
        print(json.dumps({"empty_sql_rows":[],"details_rows":[]}),flush=True)
        sys.exit(0)
    if payload['verb'] == 'rank' and actual[0]['failure'] is None and actual[0]['result'] is not None:
        actual.sort(key=lambda row: row['result']['value'])
    first = next((row for row in actual if row['failure'] is not None), actual[0])
    if payload['verb'] in ('rank','relate') and all(row['failure'] is None for row in actual):
        observation_row = next(row for row in actual if row['ordinal'] == 0)
        assert all(row['observations'] == [] for row in actual if row['ordinal'] != 0)
        events = observation_row['observations']
    else:
        unique = {json.dumps(event,sort_keys=True):event for row in actual for event in (row['observations'] or [])}
        events = list(unique.values())
    assert isinstance(first['facts'],dict) or first['facts'] is None
    assert isinstance(first['failure'],dict) or first['failure'] is None
    receipt = {'results':[row['result'] for row in actual if row['result'] is not None],
               'facts':first['facts'],'failure':first['failure'],'observations':events,
               'ordinals':[row['ordinal'] for row in actual if row['ordinal'] is not None and row['result'] is not None],
               'selection': first['ordinal'] if payload['verb']=='find' and first['result'] is not None else None}
    call_id = first['facts']['call_id'] if first['facts'] else None
    sends = first['facts']['requests_sent'] if first['facts'] else None
    kind = first['failure']['kind'] if first['failure'] else None
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
