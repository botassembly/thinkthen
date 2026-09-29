"""Repeat stage-one complete Swift matrix with stage-two installed native bits."""
import collections,datetime,json,os,pathlib,signal,subprocess,sys,time
from backend import Backend
from process_group import run
R=pathlib.Path(__file__).resolve().parents[2]
L=R/'target/logs'/('run-'+datetime.datetime.now(datetime.timezone.utc).strftime('%Y%m%dT%H%M%SZ'))
L.mkdir(parents=True);(L/'barrier').mkdir();(L/'home').mkdir();(L/'cache').mkdir()
server=Backend(L/'barrier');receipts=[];status='FAILED'
env={'PATH':'/usr/bin:/bin','HOME':str(L/'home'),'XDG_CONFIG_HOME':str(L/'home'),'XDG_CACHE_HOME':str(L/'home'),
 'LD_LIBRARY_PATH':str(R/'target/native/lib'),'THINKTHEN_BASE_URL':f'http://127.0.0.1:{server.server_port}/generic/v1',
 'THINKTHEN_API_KEY':'tt-canary-294','THINKTHEN_CACHE':str(L/'cache'),'TT_BARRIER_DIR':str(L/'barrier')}
try:
    if len(sys.argv)==2 and sys.argv[1]=='facts':
        result=run([str(R/'target/scratch/swift-matrix'),'facts'],cwd=R,env=env,timeout=30)
        (L/'facts.log').write_bytes(result.stdout+result.stderr)
        assert result.exit==0 and b'SWIFT_FACTS_LIFETIME_PASS' in result.stdout,(result.exit,(result.stdout+result.stderr)[-1500:])
        assert collections.Counter(server.arrivals)==collections.Counter(['hold-facts-one','hold-facts-no-usage','status-401','recovery-scalar']) and server.attempts==server.connections==4,server.arrivals
        bodies=[json.loads(line) for line in (L/'barrier/requests.jsonl').read_text().splitlines()]
        assert collections.Counter(row['state'] for row in bodies)==collections.Counter(server.arrivals) and all(row['questions']['q1']['type']=='noul' for row in bodies),bodies
        status='PASS'
        print('SWIFT_FACTS_EXACT_4_PASS',flush=True)
        sys.exit(0)
    for mode in ('direct','matrix'):
        result=run([str(R/'target/scratch/swift-matrix'),mode],cwd=R,env=env,timeout=120)
        (L/(mode+'.log')).write_bytes(result.stdout+result.stderr)
        receipts.append({'case':mode,'pid':result.pid,'pgid':result.pgid,'exit':result.exit,'signals':result.signals})
        assert result.exit==0 and (b'DIRECT_SWIFT_C_PASS' if mode=='direct' else b'STRICT_CANCELLED_SCALAR_PASS') in result.stdout,(mode,result.exit,(result.stdout+result.stderr)[-1500:])
    counts=collections.Counter(str(s) for s in server.arrivals)
    # Main 71f25087: records.md "Order and requests" and ADR 0055 1+3
    # pack distinct decide/filter/rank rows under one shared evidence state.
    packed_state='Each question quotes the text it asks about.'
    expected={'direct':1,'café':1,'yes':1,'no':1,'unsure':1,'a\x00b':1,packed_state:5,'json-decide':1,
     'choose':1,'tag':1,'score':1,
     '[{"id":"u001","evidence":"find-one"},{"id":"u002","evidence":"find-two"}]':1,
     "{'entities': [{'id': 'i1', 'name': 'First', 'kind': 'alert'}, {'id': 'i2', 'name': 'Second', 'kind': 'alert'}]}":1,
     "{'entities': [{'id': 'i1', 'name': 'Third', 'kind': 'alert'}, {'id': 'i2', 'name': 'Fourth', 'kind': 'alert'}]}":1,
     'annotate-one':1,'Maria Chen':2,'John Smith':2,'status-401':1,'failure-one':1,'failure-two':1,
     'success':1,'hold-deadline':1,'hold-scalar':1,'recovery-scalar':1}
    from backend import packed_rows
    requests=[json.loads(line) for line in (L/'barrier/requests.jsonl').read_text().splitlines()]
    expected_requests=json.loads((R/'Tests/fixtures/expected_requests.json').read_text())
    normalize=lambda rows: sorted((json.dumps(row,sort_keys=True,ensure_ascii=False,separators=(',',':')) for row in rows))
    assert normalize(requests)==normalize(expected_requests), 'complete normalized Swift request bodies differ'
    layouts=collections.Counter(tuple(packed_rows(request).values()) for request in requests if request['state']==packed_state)
    wanted_layouts=collections.Counter({
      ('first','second','third'):1, ('first','second'):1,
      ('filter-one','filter-two'):1, ('rank-one','rank-two'):1,
      tuple(f'hold-bulk-{i}' for i in range(1,7)):1})
    assert (counts==expected and layouts==wanted_layouts and
     server.attempts==server.connections==len(server.arrivals)==30 and
     server.bulk_completion==['packed:first,second,third'] and 'hold-scalar' in server.completions),(counts,layouts,server.bulk_completion)
    (L/'packed-layouts.json').write_text(json.dumps([list(k) for k in layouts.elements()],indent=2)+'\n')
    print('REPIN_ADAPTED_PACKING_AND_RESULT exact=30 packed=5 layouts=5',flush=True)
    status='PASS'
finally:
    server.close()
    (L/'arrivals.json').write_text(json.dumps(server.arrivals,ensure_ascii=False,indent=2)+'\n')
    (L/'outcome.json').write_text(json.dumps({'status':status,'arrivals':len(server.arrivals),'attempts':server.attempts,'receipts':receipts,'bulk_completion':server.bulk_completion},indent=2)+'\n')
    print(L,status,'arrivals',len(server.arrivals),flush=True)
if status!='PASS':sys.exit(1)
