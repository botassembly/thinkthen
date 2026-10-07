"""Fresh isolated JVM consumers use only packaged JARs and native archive."""
import collections
import json
import os
import pathlib
import re
import shutil
import signal
import sys
from backend import Backend
from process_group import run

WORK=pathlib.Path('/work')
for hidden in ('/usr/bin/cargo','/usr/bin/rustc','/home','/Users','/work/inputs/source','/work/scratch/native-target'):
    assert not pathlib.Path(hidden).exists(),hidden
assert shutil.which('cargo') is None and shutil.which('rustc') is None
assert pathlib.Path('/opt/jdk/bin/javac').is_file()
HOME=WORK/'home';HOME.mkdir()
BARRIER=HOME/'barrier';BARRIER.mkdir()
(HOME/'cache').mkdir()
CLASSES=WORK/'project/classes';CLASSES.mkdir()
server=Backend(BARRIER)
receipts=[]
def interrupt(*_):raise KeyboardInterrupt('consumer interrupted')
signal.signal(signal.SIGTERM,interrupt)
BASE={'PATH':'/usr/bin:/opt/jdk/bin','HOME':str(HOME),'XDG_CACHE_HOME':str(HOME),'XDG_CONFIG_HOME':str(HOME),
      'JAVA_HOME':'/opt/jdk','TMPDIR':'/tmp','THINKTHEN_API_KEY':'tt-canary-275',
      'THINKTHEN_BASE_URL':f'http://127.0.0.1:{server.server_port}/generic/v1',
      'THINKTHEN_CACHE':str(HOME/'cache'),'TT_BARRIER_DIR':str(BARRIER)}
CP='/work/jars/thinkthen-door.jar:/work/jars/thinkthen-kotlin.jar:/work/jars/thinkthen-scala.jar:/work/project/classes'  # packaged examples first, never test-only shadow classes
KOTLIN='/opt/kotlin/lib/kotlin-stdlib.jar'
SCALA='/opt/scala/lib/scala.jar'
JAVA=['/opt/jdk/bin/java','--enable-preview','--enable-native-access=ALL-UNNAMED','-XX:ActiveProcessorCount=2',
      '-Dthinkthen.library=/work/native/lib/libthinkthen.so']
def execute(label,cmd,expected=0,timeout=180,extra=None):
    owner=HOME/(label+'.owner.json')
    def started(pid):owner.write_text(json.dumps({'pid':pid,'pgid':pid,'state':'running'})+'\n')
    def finished(r):owner.write_text(json.dumps({'pid':r.pid,'pgid':r.pgid,'exit':r.exit,'actual_exit':r.actual_exit,'signals':r.signals})+'\n')
    result=run(cmd,cwd=WORK/'project',env=BASE|(extra or {}),timeout=timeout,on_start=started,on_finish=finished)
    text=(result.stdout+result.stderr).decode(errors='replace')
    (HOME/(label+'.log')).write_text(text)
    receipts.append({'case':label,'command':cmd,'exit':result.exit,'expected':expected,'pid':result.pid,'pgid':result.pgid,'signals':result.signals})
    (HOME/'child-receipts.json').write_text(json.dumps(receipts,indent=2)+'\n')
    print(label,'exit',result.exit,text[-250:].replace('\n',' '),flush=True)
    assert result.exit==expected and not result.signals,(label,result.exit,text[-1200:])
    return text

def check_count():
    def normalized(state):return json.dumps(state,sort_keys=True,separators=(',',':'),ensure_ascii=False) if isinstance(state,dict) else state
    arrivals=collections.Counter(normalized(state) for state in server.arrivals)
    (HOME/'actual-arrivals.json').write_text(json.dumps({'counts':arrivals,'arrivals':server.arrivals,'completions':server.completions,'bulk_completion':server.bulk_completion},indent=2,ensure_ascii=False)+'\n')
    required=['direct-java','café','yes','no','unsure','a\0b',
      'malformed-backend','transport-close',
      'retry-status','post-failure-recovery','maximum-deadline',
      'json-decide','choose','tag','score',
      'annotate-one','annotate-on','failure-one','failure-two',
      'success','hold-deadline','hold-scalar','recovery-scalar','parallel-0',
      'parallel-1','parallel-2','parallel-failure-one','parallel-failure-two',
      'Ada Lovelace','kotlin-direct','kotlin-future','kotlin-json',
      'scala-direct','scala-future','scala-json',
      'hold-kotlin','kotlin-recovery','hold-scala','scala-recovery',
      'parallel-independent-0','parallel-independent-1','parallel-independent-2']
    expected=collections.Counter({s:1 for s in required})
    # The repeated first/second bulk call reads the question cache (ADR 0111).
    expected['Each question quotes the text it asks about.']=5
    expected.update({'Maria Chen':2,'John Smith':2,'hold-contract':1,'recovery-contract':1,
      'hold-contract-plant':1,'recovery-contract-plant':1})
    expected.update({'[{"id":"u001","evidence":"find-one"},{"id":"u002","evidence":"find-two"}]':1,
      '[{"id":"u001","evidence":"find-none"},{"id":"u002","evidence":"find-another"}]':1})
    for first,second in (('First','Second'),('Third','Fourth')):
        expected[normalized({'entities':[{'id':'i1','name':first,'kind':'alert'},
          {'id':'i2','name':second,'kind':'alert'}]})]=1
    assert arrivals==expected,('exact wire-state counts',arrivals-expected,expected-arrivals)
    # records.md §Batching and backends.md §Request-size: five logical record
    # groups now travel in five full requests. Assert each actual packed wire
    # body, not only the identical shared-evidence state string.
    def signature(request):return normalized(request)
    packed_groups=[('bulk-before-bad','bulk-middle-bad','bulk-after-bad'),
      ('first','second','third'),('filter-one','filter-two'),
      ('rank-one','rank-two'),tuple('hold-bulk-'+str(i) for i in range(1,7))]
    packed_expected=collections.Counter(signature({'state':'Each question quotes the text it asks about.',
      'model':'jev-1.13.0', 'questions':{'q'+str(i+1):{'type':'noul','instructions':'The text is '+json.dumps(name)+'. Is it?'} for i,name in enumerate(group)}}) for group in packed_groups)
    captured=[json.loads(path.read_text()) for path in BARRIER.glob('wire-body-*.json')]
    packed_actual=collections.Counter(signature(req) for req in captured if req['state']=='Each question quotes the text it asks about.')
    assert packed_actual==packed_expected,('exact packed bodies',packed_actual-packed_expected,packed_expected-packed_actual)
    relation_expected=collections.Counter(signature({'state':{'entities':[{'id':'i1','name':first,'kind':'alert'},
      {'id':'i2','name':second,'kind':'alert'}]},'model':'jev-1.13.0',
      'questions':{'q1':{'type':'noul','instructions':'Is it true that i1 caused by i2?'},
                   'q2':{'type':'noul','instructions':'Is it true that i2 caused by i1?'}}})
      for first,second in (('First','Second'),('Third','Fourth')))
    relation_actual=collections.Counter(signature(req) for req in captured if isinstance(req['state'],dict) and 'entities' in req['state'])
    assert relation_actual==relation_expected,('exact relation pair bodies',relation_actual-relation_expected,relation_expected-relation_actual)
    assert len(server.arrivals)==sum(expected.values()) and server.attempts==len(server.arrivals)
    assert server.bulk_completion==['first+second+third (one packed request)'],server.bulk_completion
    return {'arrivals':server.arrivals,'attempts':server.attempts,'connections':server.connections,
      'completions':server.completions,'bulk_completion':server.bulk_completion}

if len(sys.argv) == 2:
    selected=sys.argv[1]
    portable=selected.startswith('portable-')
    lang=selected.removeprefix('portable-')
    assert lang in ('java','kotlin','scala'),lang
    try:
        if lang=='java':
            execute('release-javac',['/opt/jdk/bin/javac','--enable-preview','--release','21',
              '-cp',CP,'-d',str(CLASSES),'InstalledJava.java'])
            run_class='InstalledJava'
            runtime_cp=CP
        elif lang=='kotlin':
            execute('release-kotlinc',['/opt/kotlin/bin/kotlinc','-J-XX:ActiveProcessorCount=2',
              '-jvm-target','21','-classpath',CP,'InstalledKotlin.kt','-d',str(CLASSES)],timeout=240)
            run_class='InstalledKotlinKt'
            runtime_cp=CP+':'+KOTLIN
        else:
            execute('release-scalac',['/opt/scala/bin/scalac','-J-XX:ActiveProcessorCount=2',
              '-classpath',CP,'-d',str(CLASSES),'InstalledScala.scala'],timeout=240)
            run_class='installedScala'
            runtime_cp=CP+':'+SCALA
        output=execute(selected,JAVA+['-cp',runtime_cp,run_class],
          extra={'TT_PORTABLE_BATCH':'1'} if portable else None)
        marker=('PORTABLE_BATCH_' if portable else 'INSTALLED_')+lang.upper()+'_PASS'
        assert marker in output,output
        summary={'arrivals':server.arrivals,'attempts':server.attempts,'connections':server.connections,
          'classpath':runtime_cp,'native':'/work/native/lib/libthinkthen.so',
          'url':f'http://127.0.0.1:{server.server_port}/generic/v1/systemone'}
        if portable:
            assert summary['attempts']==summary['connections']==1,summary
        else:
            expected={'model':'jev-1.13.0','questions':{'q1':{'type':'noul','instructions':'The text is "release-'+lang+'". Is it?'}},'state':'Each question quotes the text it asks about.'}
            captured=[json.loads(path.read_text()) for path in BARRIER.glob('wire-body-*.json')]
            assert captured==[expected],(captured,expected)
            summary['body']=captured[0]
            assert summary['arrivals']==['release-'+lang] and summary['attempts']==summary['connections']==1,summary
        (HOME/'summary.json').write_text(json.dumps(summary,indent=2)+'\n')
        print('isolated',selected,'counted bodies PASS',flush=True)
    finally: server.close()
    sys.exit(0)

try:
    execute('consumer-javac',['/opt/jdk/bin/javac','--enable-preview','--release','21','-cp',CP,'-d',str(CLASSES),
      'Direct.java','Matrix.java','StrictScalar.java','Concurrent.java','BoundedString.java','JsonTest.java','thinkthen/ProbeDoor.java'])
    execute('consumer-kotlinc',['/opt/kotlin/bin/kotlinc','-J-XX:ActiveProcessorCount=2','-jvm-target','21',
      '-classpath',CP,'KotlinCaller.kt','-d',str(CLASSES)],timeout=240)
    execute('consumer-scalac',['/opt/scala/bin/scalac','-J-XX:ActiveProcessorCount=2',
      '-classpath',CP,'-d',str(CLASSES),'ScalaCaller.scala'],timeout=240)
    execute('example-java',JAVA+['-cp',CP,'Direct'])
    reader=execute('json-reader',JAVA+['-cp',CP,'JsonTest'])
    assert 'JSON_READER_AND_FIELD_PASS' in reader
    matrix=execute('matrix-java',JAVA+['-cp',CP,'Matrix'],timeout=180)
    assert 'JAVA_HELD_SCALAR_CANCELLED_PASS' in matrix and 'KNOWN NATIVE CONTRACT FINDING' not in matrix
    kotlin=execute('kotlin',JAVA+['-cp',CP+':'+KOTLIN,'KotlinCallerKt'])
    assert 'KOTLIN_HELD_SCALAR_CANCELLED_PASS' in kotlin and 'KOTLIN_PASS' in kotlin and 'KNOWN NATIVE CONTRACT FINDING' not in kotlin
    scala=execute('scala',JAVA+['-cp',CP+':'+SCALA,'scalaCaller'])
    assert 'SCALA_HELD_SCALAR_CANCELLED_PASS' in scala and 'SCALA_PASS' in scala and 'KNOWN NATIVE CONTRACT FINDING' not in scala
    execute('bounded-json-refusal',JAVA+['-cp',CP,'thinkthen.BoundedString'],extra={'THINKTHEN_API_KEY':'tt-canary-275'})
    execute('independent-concurrent',JAVA+['-cp',CP,'Concurrent'])
    plant=execute('held-plant',JAVA+['-cp',CP,'StrictScalar'],expected=1,extra={'TT_GATE_STRICT_PLANT':'1'})
    assert plant.count('PLANTED DIFFERENT STRICT FAILURE AFTER RECOVERY')==1 and plant.count('FRESH_TOKEN_RECOVERY_PASS')==1
    assert 'STRICT_SCALAR_CONTRACT_PASS' not in plant
    strict=execute('held-cancellation',JAVA+['-cp',CP,'StrictScalar'])
    assert strict.count('STRICT_SCALAR_CONTRACT_PASS')==1 and strict.count('FRESH_TOKEN_RECOVERY_PASS')==1
    assert 'PLANTED DIFFERENT STRICT FAILURE' not in strict and 'Exception in thread' not in strict
    summary=check_count()
    (HOME/'summary.json').write_text(json.dumps(summary,indent=2,ensure_ascii=False)+'\n')
    print('isolated JVM consumer PASS with strict native scalar cancellation; counted arrivals',len(server.arrivals),flush=True)
finally:server.close()
