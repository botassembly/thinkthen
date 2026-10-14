"""Installed native SQLite projections preserve call ownership and SQL behavior."""
from helper import Backend, child, environment, expect, main


def test_schema_keeps_answers_inputs_and_all_canonical_fields_separate():
    backend = Backend()
    got = child('''
from native_receipts import TYPES
names=tuple(name+'_details' for name in ('decide','choose','tag','score','filter','rank','find','annotate','recognize','relate'))+('rank_set','recognize','relate')
db=connect()
assert not db.execute("SELECT name FROM pragma_function_list WHERE name LIKE 'thinkthen_%_complete'").fetchall()
assert not db.execute("SELECT name FROM pragma_function_list WHERE name IN ('thinkthen_details','thinkthen_try_details','thinkthen_details_images')").fetchall()
for retired in ('thinkthen_details','thinkthen_try_details','thinkthen_details_images'):
    assert 'no such function' in run(db,'SELECT '+retired+'(1,2,3,4)')
    assert 'no such table' in run(db,'SELECT * FROM '+retired+'(1,2)')
visible={}
for name in names:
    schema=db.execute('SELECT name,hidden FROM pragma_table_xinfo(?)',('thinkthen_'+name,)).fetchall()
    columns=[row[0] for row in schema]
    assert len(columns)==len(set(columns)) and len(columns)<2000,(name,len(columns))
    assert 'request_records' in columns and 'projection' in columns
    assert columns.index('request_records')<columns.index('facts_call_id')
    for kind,(fields,_) in TYPES.items():
        assert kind+'_receipt_index' in columns,(name,kind)
        for field,shape in fields.items():
            base='question_definition_name' if (kind,field)==('question','name') else kind+'_'+field
            assert base+'_present' in columns,(name,base)
            suffix=[''] if isinstance(shape,str) else ['_index'] if shape[0]=='object' else ['_first_index','_count']
            assert all(base+part in columns for part in suffix),(name,base)
    visible[name]=[field for field,hidden in schema if hidden==0]
say(visible=visible)
''', environment(backend, THINKTHEN_MAX_REQUESTS_TOTAL='0'))
    expect(got['visible']['rank_set'], ['key','rank','probability','question_name'], 'rank set answer shape')
    expect(got['visible']['recognize'], ['text','start','end','length','kind','strength'], 'recognition answer shape')
    expect(got['visible']['relate'], ['relation','source','target','probability','either'], 'relation answer shape')
    for name in ('decide','choose','tag','score','filter','rank','find','annotate','recognize','relate'):
        expect(got['visible'][name+'_details'], ['ordinal','value','probability','result','failure','facts','observations'], 'typed details row')
    expect(backend.close(), 0, 'discovery and schema never send')


def test_default_details_fresh_calls_and_projection_scans_keep_original_facts():
    for cached in (False, True):
        backend = Backend()
        got = child('''
from native_receipts import Read
from pathlib import Path
db=connect()
cache=os.path.join(os.environ['SCRATCH'],'cache') if os.environ['CACHE']=='yes' else False
db.execute('SELECT thinkthen_configure(?)',(json.dumps({'cache':cache}),))
args=('Is it red?','red door')
answers=[];facts=[]
for _ in range(2):
    answers.append(run(db,'SELECT value FROM thinkthen_decide_details(?,?)',args))
    facts.append(run(db,"SELECT facts_call_id,facts_requests_sent,facts_cache_answers FROM thinkthen_decide_details(?,?) WHERE projection='facts'",args))
    assert run(db,"SELECT facts_call_id,facts_requests_sent,facts_cache_answers FROM thinkthen_decide_details(?,?) WHERE projection='facts'",args)==facts[-1]
reader=Read(db,'details','Is it red?',{},None)
reader.fields={'question':args[0],'text':args[1]}
events=[reader.object('event',row['event_receipt_index']) for row in reader.rows('event')]
assert events[0]['detail']['question']['text']=='Is it red?',events
assert events[0]['inputs'][0]['input']=='red door',events
other=run(db,"SELECT value FROM thinkthen_decide_details('Different?','other input')")
assert run(db,"SELECT facts_call_id,facts_requests_sent,facts_cache_answers FROM thinkthen_decide_details(?,?) WHERE projection='facts'",args)==facts[-1]
say(answers=answers,facts=facts,other=other)
''', environment(backend, CACHE='yes' if cached else 'no'))
        expect([rows[0][0] for rows in got['answers']], [1,1], 'fresh ordinary answers')
        expect([rows[0][1:] for rows in got['facts']], [[1,0],[0,1]] if cached else [[1,0],[1,0]], 'original facts retained per invocation')
        expect(got['facts'][0][0][0] != got['facts'][1][0][0], True, 'fresh calls keep distinct call identities')
        expect(backend.close(), 2 if cached else 3, 'only default invocations send')


def test_ordinary_details_keep_all_judgment_values_and_reported_metadata():
    backend=Backend()
    got=child('''
from native_receipts import Read
db=connect();db.execute('SELECT thinkthen_configure(?)',('{"cache":false}',))
questions={"decide":'{"decide":"Is it red?","true":{"what":"red","examples":["red door"]}}',
 "choose":'{"choose":"Which team?","options":["billing","shipping","other"]}',
 "tag":'{"tag":"Which labels?","labels":["billing","urgent","security"]}',
 "score":'{"score":"How severe?","levels":["low","medium","high"]}'}
answers={};receipts={}
for verb,question in questions.items():
    from native_receipts import ROOT_KIND
    answers[verb]=run(db,'SELECT value,'+ROOT_KIND[verb]+'_answer_id,facts_requests_sent FROM thinkthen_'+verb+'_details(?,?)',(question,'red door'))
    reader=Read(db,'details',question,{},None);reader.fields={'question':question,'text':'red door'}
    detail=reader.rows('question_detail')[0]
    receipts[verb]=reader.object('question_detail',detail['question_detail_receipt_index'])
    assert receipts[verb]['question']['verb']==verb,receipts[verb]
    assert receipts[verb]['inputs']==['red door'],receipts[verb]
    assert receipts[verb]['usage']['input_tokens']==1 and receipts[verb]['usage']['output_tokens']==1,receipts[verb]
    assert receipts[verb]['requests_sent']==1,receipts[verb]
    if verb=='decide': assert receipts[verb]['threshold']==.5,receipts[verb]
say(answers=answers,receipts=receipts)
''',environment(backend,'arm/full'))
    expect({name:rows[0][0] for name,rows in got['answers'].items()},
           {'decide':1,'choose':'billing','tag':'["billing","urgent","security"]','score':.15},'ordinary values retain native SQL types')
    expect(all(len(rows[0][1])==64 and rows[0][2]==1 for rows in got['answers'].values()),True,'native identities and request counts')
    expect(all(receipt['model']=='jev-1.13.0' for receipt in got['receipts'].values()),True,'reported model')
    expect(backend.close(),4,'one evaluation for each default details call and no projection sends')


def test_output_predicates_do_not_change_call_identity_or_admit_extra_options():
    backend = Backend()
    got = child('''
db=connect();db.execute('SELECT thinkthen_configure(?)',('{"cache":false}',))
args=('Fits?','red door')
first=run(db,'SELECT value,probability FROM thinkthen_decide_details(?,?)',args)
identity=run(db,"SELECT facts_call_id FROM thinkthen_decide_details(?,?) WHERE projection='facts'",args)
filtered=run(db,"SELECT decision_value FROM thinkthen_decide_details(?,?) WHERE projection='decision' AND facts_requests_sent=0",args)
kept=run(db,"SELECT decision_value FROM thinkthen_decide_details(?,?) WHERE projection='decision' AND facts_requests_sent=1",args)
again=run(db,"SELECT facts_call_id FROM thinkthen_decide_details(?,?) WHERE projection='facts' AND facts_requests_sent=1",args)
schema=db.execute("SELECT cid,name FROM pragma_table_xinfo('thinkthen_decide_details') WHERE name='facts_call_id'").fetchone()
# A positional operand in an output slot is a SQLite predicate, outside input admission.
visible=7;count=schema[0]-visible+1
parameters=[None]*count;parameters[:2]=args;parameters[-1]='different-call-id'
position=run(db,"SELECT decision_value FROM thinkthen_decide_details("+','.join('?' for _ in parameters)+") WHERE projection='decision'",parameters)
say(first=first,filtered=filtered,kept=kept,identity=identity,again=again,position=position)
''', environment(backend))
    expect(got['first'], [[1,.9]], 'compact default answer columns')
    expect((got['filtered'],got['kept'],got['position']), ([],[[1]],[]), 'SQLite filters receipt outputs')
    expect(got['identity'],got['again'],'output predicates reuse original call')
    expect(backend.close(),1,'projection and positional output filters send nothing extra')


def test_open_number_data_preserves_unsigned_original_exactly():
    backend=Backend()
    got=child('''
db=connect()
receipt=complete(db,'decide','Fits?',{'records':[{'json':18446744073709551615}]})
row=receipt['native']['value'][0]
say(original=row['input'],value=row['value'])
''',environment(backend))
    expect(got,{'original':18446744073709551615,'value':True},'wide original numbers retain exact native DATA')
    expect(backend.close(),1,'one wide-number judgment')


def test_changed_source_files_and_eight_entry_eviction_recompute_only_answers():
    backend=Backend()
    got=child('''
from pathlib import Path
db=connect();db.execute('SELECT thinkthen_configure(?)',('{"cache":false}',))
path=Path(os.environ['SCRATCH'])/'source.txt';path.write_text('Alpha.\\n')
args=('Fits?',json.dumps([str(path)]))
first=run(db,'SELECT file,records,facts_call_id FROM thinkthen_filter(?,?)',args)
before=path.stat().st_mtime_ns;path.write_text('Alpha.\\nBeta.\\n');os.utime(path,ns=(before+2000000000,before+2000000000))
second=run(db,'SELECT file,records,facts_call_id FROM thinkthen_filter(?,?)',args)
for i in range(8):
    run(db,'SELECT value FROM thinkthen_decide_details(?,?)',('Fits?',str(i)))
old=run(db,'SELECT file,records,facts_call_id FROM thinkthen_filter(?,?)',args)
say(first=first,second=second,old=old)
''',environment(backend))
    expect([row[1] for row in got['first']], [1], 'initial source rows')
    expect([row[1] for row in got['second']], [2,2], 'changed source invalidates retained receipt')
    expect([row[1] for row in got['old']], [2,2], 'evicted source recomputes its rows')
    expect(got['first'][0][2] != got['second'][0][2] != got['old'][0][2],True,'each recomputation owns new facts')
    expect(backend.close(),11,'one packed call per source evaluation and eight other calls')


def test_details_runtime_failure_keeps_later_valid_rows_and_final_facts():
    from conditional_backend import ConditionalBackend
    backend=Backend()
    with ConditionalBackend(backend.base()) as proxy:
        got=child('''
from native_receipts import Read
db=connect();db.execute('SELECT thinkthen_configure(?)',('{"cache":false,"batch":1,"max_retries":0}',))
questions={'decide':{'decide':'Fits?'},'filter':{'decide':'Fits?'},
 'choose':{'choose':'Which?','options':['billing','shipping','other']},
 'tag':{'tag':'Which?','labels':['billing','urgent','security']},
 'score':{'score':'How?','levels':['low','medium','high']},
 'annotate':{'version':1,'questions':{'fits':{'decide':'Fits?'}}}}
results={}
for verb,question in questions.items():
    records=[{'text':'first valid row'},{'text':'private evidence'},{'text':'last valid row'}]
    if verb=='annotate': records=[{'json':{'body':record['text']}} for record in records]
    question=json.dumps(question);items=json.dumps(records)
    rows=db.execute('SELECT ordinal,value,probability,result,failure,facts,error_kind FROM thinkthen_'+verb+'_details WHERE request_question=? AND request_records=?',(question,items)).fetchall()
    assert [row[0] for row in rows]==[0,1,2],rows
    assert [row[6] for row in rows]==[None,'backend',None],rows
    assert rows[0][3] is not None and rows[2][3] is not None,rows
    assert rows[1][1] is None and rows[1][3] is None and rows[1][4] is not None,rows
    reader=Read(db,verb,question,{'records':records},None)
    reader.fields={'request_question':question,'request_records':items}
    failure=reader.object('error',rows[1][4])
    assert failure['kind']=='backend' and failure['retryable'] is False,failure
    assert 'private evidence' not in json.dumps(failure),failure
    facts=reader.object('facts',rows[1][5])
    assert facts['requests_sent']==3,facts
    assert all(row[5]==rows[0][5] for row in rows),rows
    if verb in ('decide','filter'): assert rows[0][2]==.9 and rows[2][2]==.9,rows
    if verb in ('score','tag','annotate'): assert all(row[2] is None for row in rows),rows
    results[verb]=True
say(results=results)
''',environment(backend,THINKTHEN_BASE_URL=proxy.base))
        expect(proxy.count(),18,'one request for each valid row, including six failures')
    expect(all(got['results'].values()),True,'row failure preserves later answers and owning call facts')
    expect(backend.close(),12,'only the twelve successful rows reached the generic backend')


def test_empty_sql_frame_rejects_nonempty_and_malformed_claims():
    import sys
    from pathlib import Path
    sys.path.insert(0,str(Path(__file__).parent/'complete'))
    from project import project,assert_empty_sql,NATIVE
    for marker in ({'empty_sql_rows':[1],'details_rows':[]},
                   {'empty_sql_rows':[],'details_rows':[{}]},
                   {'empty_sql_rows':[],'details_rows':[],'native':{}},
                   {'empty_sql_rows':None,'details_rows':[]}):
        try: project(marker,'filter')
        except AssertionError: pass
        else: raise AssertionError(marker)
    failure={'error':{'kind':'usage','message':'unsafe answer','retryable':False,'cause':'missing_answer','stopped':{'cause':'reply','retryable':False,'at':0}}}
    from jsonschema.exceptions import ValidationError
    NATIVE.validate(failure)
    assert project({'native':failure},'decide')['message']=='unsafe answer'
    try: NATIVE.validate({'error':{**failure['error'],'cause':'invented'}})
    except ValidationError: pass
    else: raise AssertionError('an invented logical cause passed')
    frame=project({'empty_sql_rows':[],'details_rows':[]},'filter')
    valid={'verb':'filter','items':[],'expect':{'success':{'kind':'filter','answers':[],'operation':{'indexes':[]}}}}
    assert_empty_sql(valid,frame,0)
    for bad,count in ((valid,1),({**valid,'items':['refund']},0),
                      ({**valid,'expect':{'success':{'kind':'filter','answers':[True],'operation':{'indexes':[0]}}}},0)):
        try: assert_empty_sql(bad,frame,count)
        except AssertionError: pass
        else: raise AssertionError((bad,count))


if __name__=='__main__':
    raise SystemExit(main(globals()))
