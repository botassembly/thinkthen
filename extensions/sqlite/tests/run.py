"""Exercise the installed extension through an ordinary SQLite C host."""

import json
from pathlib import Path
import subprocess
import sys
import tempfile
from contextlib import contextmanager

CONSUMER, EXTENSION, LOOPBACK, ROOT = sys.argv[1:5]
ADMISSION_ONLY = sys.argv[5:] == ['--admission-only']
assert not sys.argv[5:] or ADMISSION_ONLY, sys.argv[5:]
ROOT = Path(ROOT)
sys.path.insert(0, str(ROOT / "databases"))
from testing import child_environment
DONE, ROW, ERROR, INTERRUPT = 101, 100, 1, 9


def environment(**extra):
    return {**child_environment(), **extra}


def public_question(question):
    question=json.loads(json.dumps(question))
    if question['function']=='relate':
        for rule in question['rules']: rule['name']=rule.pop('relation')
    elif question['function']=='annotate':
        question['questions']={name:public_question(child) for name,child in question['questions'].items()}
    return question


class Host:
    def __init__(self, url=None):
        env = environment()
        if url:
            env.update(THINKTHEN_TEST_LOOPBACK_URL=url, THINKTHEN_API_KEY="loopback-test-key", OPENAI_API_KEY="loopback-test-key")
        self.process = subprocess.Popen([CONSUMER, EXTENSION], stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, text=True, env=env)

    def send(self, command):
        self.process.stdin.write(json.dumps(command) + "\n")
        self.process.stdin.flush()

    def receive(self):
        line = self.process.stdout.readline()
        assert line, "SQLite host exited before reporting its statement result"
        return json.loads(line)

    def sql(self, sql, parameters=(), **extra):
        self.send(dict(sql=sql, parameters=parameters, **extra))
        return self.receive()

    def ok(self, sql, parameters=(), **extra):
        result = self.sql(sql, parameters, **extra)
        assert result["code"] in (DONE, ROW), result
        return result["rows"]

    def configure(self, settings):
        for key,value in settings.items():
            for name,item in (value.items() if key=='retry' else [(key,value)]):
                self.ok('SELECT thinkthen_set(?,?)',[name,item])

    def call_failure(self, request, code):
        self.call(request, code)

    def call(self, request, expected_error=None, raw=False):
        self.ok('DROP TABLE IF EXISTS input')
        self.ok('CREATE TABLE input(n INTEGER PRIMARY KEY,record TEXT)')
        for index,record in enumerate(request.get('records',[]),1):
            self.ok('INSERT INTO input VALUES(?,?)',[index,json.dumps(record)])
        question=public_question(request['question'])
        constraints=['source_sql = ?','format = ?']
        values=['SELECT record FROM input ORDER BY n','json']
        for name,value in question.items():
            if name=='function': continue
            if name=='threshold' and isinstance(value,dict):
                for key,number in value.items(): constraints.append(key+' = ?');values.append(number)
            else:
                name='toolbox' if name=='questions' else name
                constraints.append(name+' = ?');values.append(json.dumps(value) if isinstance(value,(dict,list)) else value)
        if 'name' in request: constraints.append('name = ?');values.append(request['name'])
        result=self.sql('SELECT * FROM thinkthen_'+question['function']+' WHERE '+' AND '.join(constraints),values)
        if raw:
            return result
        if expected_error is not None:
            assert result['code'] == ERROR and expected_error in result['message'], result
            assert not result['rows'], result
            return
        assert result['code']==DONE,result
        rows=[]
        for native in result['rows']:
            row=dict(zip(result['columns'],native))
            answer=row.get('answer')
            if question['function'] in ('decide','filter') and answer is not None:
                assert type(answer) is int and answer in (0,1),row
                answer=bool(answer)
            elif question['function']=='find':
                answer=None if row['found_text'] is None else dict(text=row['found_text'],start=row['found_start'],end=row['found_end'])
            elif question['function']=='rank': answer=row['rank']
            elif question['function'] in ('tag','recognize','relate','annotate') and answer is not None: answer=json.loads(answer)
            error=None if row['error_code'] is None else dict(code=row['error_code'],message=row['error_message'])
            if error is not None and row['error_http_status'] is not None: error['http_status']=row['error_http_status']
            rows.append((row['position'],row['status'],json.dumps(answer) if row['status']=='answered' else None,row['probability'],
                json.dumps(error) if error else None,row['detail'],row.get('kept'),row['id'],row['source']))
        return rows

    def close(self):
        self.process.stdin.close()
        assert self.process.wait(timeout=20) == 0

    def __enter__(self):
        return self

    def __exit__(self, *failure):
        if failure[0] is not None:
            self.process.kill()
            self.process.wait()
        else:
            self.close()


sys.path.insert(0, str(ROOT / 'databases/duckdb/tests'))
from migration_cases import command, fixture, forbidden_settings, run_cases
from admission_cases import cases as admission_cases, diagnostic


with Host() as host:
    host.configure({'backend': 'stub'})
    for case, request, expected in admission_cases(ROOT, 'sqlite'):
        result = host.call(request, raw=True)
        assert result['code'] == ERROR and not result['rows'], (case['name'], result)
        assert 'secret-placeholder' not in json.dumps(result), case['name']
        code, separator, message = result['message'].partition(': ')
        assert separator, result
        diagnostic(case, code, message, expected)
if ADMISSION_ONLY:
    sys.exit(0)

with Host() as host:
    host.configure({'backend':'stub'})
    # SQLite's allocator includes native ID slots, independent of Rust's heap.
    def scan(count):
        source=f"WITH RECURSIVE rows(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM rows WHERE n<{count}) SELECT n AS id,NULL AS record FROM rows"
        rows=host.ok('SELECT count(*),sum(position),sum(id!=position),max(native_memory()) FROM thinkthen_decide(?,?)',[source,'Ready?'])
        assert rows[0][:3]==[count,count*(count+1)//2,0],rows
        return rows[0][3]
    small=scan(128)
    large=scan(65536)
    assert large-small<128*1024,(small,large)

    for key,value in [('batch',1.5),('api_key','do-not-store'),('url','https://invalid.example'),('key_source','OTHER_KEY'),('mode','proxy'),('adapter','stub')]:
        assert host.sql('SELECT thinkthen_set(?,?)',[key,value])['code']==ERROR,(key,value)
    assert host.sql("SELECT thinkthen_configure('{}')")['code']==ERROR
    for argument,value in [('format','json\0text'),('record_column','record\0other'),('id_column','id\0other')]:
        result=host.sql('SELECT * FROM thinkthen_decide(?,?) WHERE '+argument+' = ?',
            ["SELECT 7 AS id, '{}' AS record",'Ready?',value])
        assert result['code'] not in (ROW,DONE),(argument,result)
    host.ok('CREATE TABLE docs(n INTEGER PRIMARY KEY,id,record TEXT)')
    host.ok("INSERT INTO docs VALUES(1,19,'alpha'),(2,x'00ff',NULL),(3,'last','beta')")
    rows=host.ok('SELECT position,typeof(id),hex(id),status,answer,error_code FROM thinkthen_decide(?,?)',
        ['SELECT id,record FROM docs ORDER BY n','{literal?'])
    assert len(rows)==3 and rows[1]==[2,'blob','00FF','failed',None,'record.missing'],rows
    assert [row[1] for row in rows]==['integer','blob','text'],rows
    for statement in [
        "SELECT * FROM thinkthen_choose('SELECT ''x'' AS record','Which?')",
        "SELECT * FROM thinkthen_tag('SELECT ''x'' AS record','Labels?') WHERE labels = '[]'",
        "SELECT * FROM thinkthen_find('SELECT ''x'' AS record','Where?') WHERE low = 0.2 AND high = 0.8",
        "SELECT * FROM thinkthen_decide('SELECT ''x'' AS record','Ready?') WHERE threshold = 0.8 AND low = 0.2 AND high = 0.7",
        "SELECT * FROM thinkthen_decide('SELECT ''x'' AS record','Ready?') WHERE low = 0.2",
        "SELECT * FROM thinkthen_decide('SELECT ''{}'' AS record','Ready?') WHERE format = 'text' AND evidence = '/body'",
        "SELECT * FROM thinkthen_decide('SELECT ''x'' AS record','Ready?') WHERE threshold > 0.7",
        """SELECT * FROM thinkthen_annotate('SELECT ''x'' AS record') WHERE toolbox = '{"bad":{"function":"rank","question":"Rank?","levels":["low","high"]}}'""",
    ]: assert host.sql(statement)['code'] not in (ROW,DONE),statement
    source="SELECT json(record) AS record FROM (SELECT '{\"body\":\"alpha\"}' AS record) ORDER BY record"
    encoded=host.ok('SELECT answer,probability FROM thinkthen_decide(?,?)',[source,'Ready?'])
    plain=host.ok('SELECT answer,probability FROM thinkthen_decide(?,?)',["SELECT '{\"body\":\"alpha\"}' AS record",'Ready?'])
    assert encoded==plain,(encoded,plain)
    assert host.ok("SELECT status,error_code FROM thinkthen_decide('SELECT ''null'' AS record','Ready?') WHERE format='json'")==[['failed','record.missing']]
    unnamed=host.ok("SELECT source FROM thinkthen_decide('SELECT ''x'' AS record','Ready?')")[0][0]
    named=host.ok("SELECT source FROM thinkthen_decide('SELECT ''x'' AS record','Ready?') WHERE name='audit'")[0][0]
    assert unnamed!=named and json.loads(named)['name']=='audit',(unnamed,named)
    assert 'name' not in json.loads(unnamed),unnamed
    for source in ["SELECT json_object('body','alpha') AS record", "SELECT json('{\"body\":\"alpha\"}') AS record", "SELECT '{\"body\":\"alpha\"}' AS record"]:
        rows=host.ok('SELECT answer,probability FROM thinkthen_decide(?,?) WHERE evidence=?',[source,'Ready?','/body'])
        assert rows==host.ok('SELECT answer,probability FROM thinkthen_decide(?,?) WHERE evidence=?',
            ["SELECT '{\"body\":\"alpha\"}' AS record ORDER BY record",'Ready?','/body']),rows
    for record in ('NULL','42','1.5',"x'00ff'"):
        rows=host.ok('SELECT status,error_code FROM thinkthen_decide(?,?)',['SELECT '+record+' AS record','Ready?'])
        assert rows==[['failed','record.missing' if record=='NULL' else 'record.invalid']],rows
    for case in json.loads((ROOT/'construction/stub.json').read_text()):
        actual=host.call(case['request'])
        from migration_cases import decode, semantic
        actual=[decode(row,case['request']['question']['function'],'stub','stub') for row in actual]
        expected=[semantic(json.loads(json.dumps(row))) for row in case['expected']]
        assert actual==expected,(case['request'],actual,expected)
run_cases(Host, LOOPBACK)
for arguments,cause in [(['--unavailable'],'connection refused'),
                        (['--held','reset'],'connection reset'),
                        (['--held','headers'],'timeout')]:
    # Darwin keeps a bound non-listening TCP port silent instead of refusing it.
    if sys.platform == 'darwin' and cause == 'connection refused': continue
    with fixture(LOOPBACK,*arguments) as (server,url),Host(url) as host:
        host.configure({'backend':'openai','retry':{'max_retries':0,'attempt_timeout_ms':200}})
        rows=host.ok('SELECT status,error_code,error_message FROM thinkthen_decide(?,?)',
                     ["SELECT 'x' AS record",'Ready?'])
        assert rows==[['failed','backend.transport',
                      f'the openai backend could not be reached: {cause}']],rows
        assert 'loopback-test-key' not in json.dumps(rows),rows
for status in (401,402,429,500):
    with tempfile.TemporaryDirectory() as temporary:
        case=Path(temporary)/'reply.json'
        case.write_text(json.dumps({'replies':[{'status':status,'body':{'error':
            f'reason for {status} loopback-test-key sk-unknown-sentinel'}}]}))
        with fixture(LOOPBACK,case) as (server,url),Host(url) as host:
            host.configure({'backend':'openai','retry':{'max_retries':0}})
            rows=host.ok('SELECT status,error_code,error_http_status,error_message FROM thinkthen_decide(?,?)',
                         ["SELECT 'x' AS record",'Ready?'])
            assert rows==[['failed','backend.status',status,
                f'the backend answered status {status}: '+
                f'{{"error":"reason for {status} [redacted] [redacted]"}}']],rows
            assert 'loopback-test-key' not in json.dumps(rows),rows
            assert 'sk-unknown-sentinel' not in json.dumps(rows),rows
            assert command(server,'count')=='1'
for mode in ('headers','body','refusal-body'):
    with fixture(LOOPBACK,'--held',mode) as (server,url),Host(url) as host:
        host.send({'sql':'SELECT * FROM thinkthen_decide(?,?)','parameters':["SELECT 'x' AS record",'Ready?'],'interruptible':True})
        assert command(server,'arrived')=='arrived'
        host.process.stdin.write('i');host.process.stdin.flush()
        result=host.receive()
        assert result['code']==INTERRUPT,result
        assert command(server,'closed')=='closed'
        assert host.ok('SELECT 1')==[[1]]
        host.configure({'backend':'stub'})
        assert len(host.ok("SELECT * FROM thinkthen_decide('SELECT ''x'' AS record','Ready?')"))==1
print('SQLite typed arguments, native ids, both wires, settings and cancellation pass')

# Feature: repeated filters in one join reuse the cursor's paid result.
case=next(path for path in (ROOT/'conformance').rglob('*.json') if 'replies' in json.loads(path.read_text()) and json.loads(path.read_text()).get('request',{}).get('question',{}).get('function')=='decide')
with fixture(LOOPBACK,case) as (server,url),Host(url) as host:
    host.configure({'backend':'openai'})
    rows=host.ok("SELECT outer_row.n,t.status FROM (SELECT 1 n UNION ALL SELECT 2 UNION ALL SELECT 3) outer_row LEFT JOIN thinkthen_decide('SELECT 9 AS id, ''x'' AS record','Ready?') t ON 1=1")
    assert len(rows)==3,rows
    assert command(server,'count')=='1'
    rows=host.ok("SELECT t.id FROM (SELECT 1 n UNION ALL SELECT 2 UNION ALL SELECT 1) q LEFT JOIN thinkthen_decide t ON t.source_sql=CASE q.n WHEN 1 THEN 'SELECT 1 AS id, ''x'' AS record' ELSE 'SELECT 2 AS id, ''x'' AS record' END AND t.question='Ready?'")
    assert rows==[[1],[2],[1]],rows
    assert command(server,'count')=='4'
    host.ok("CREATE TABLE events(n)")
    host.ok("CREATE TRIGGER refuse AFTER INSERT ON events BEGIN SELECT * FROM thinkthen_decide('SELECT ''x'' AS record','Ready?'); END")
    assert host.sql('INSERT INTO events VALUES(1)')['code']==ERROR

# Feature: a source-side settings change invalidates the original client result.
with fixture(LOOPBACK,case) as (server,url),Host(url) as host:
    host.configure({'backend':'openai'})
    rows=host.ok("SELECT t.id FROM (SELECT 1 n UNION ALL SELECT 2) q LEFT JOIN thinkthen_decide(\"SELECT 7 AS id, CASE thinkthen_set('model','changed-model') WHEN 1 THEN 'x' END AS record\",'Ready?') t ON 1=1")
    assert rows==[[7],[7]],rows
    assert command(server,'count')=='2'
