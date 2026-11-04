"""Exercise the installed extension through an ordinary SQLite C host."""

import json
from pathlib import Path
import subprocess
import sys
import time
from concurrent.futures import ThreadPoolExecutor
import tempfile

CONSUMER, EXTENSION, LOOPBACK, ROOT = sys.argv[1:5]
STREAMING_ONLY = sys.argv[5:] == ['--streaming-only']
ADMISSION_ONLY = sys.argv[5:] == ['--admission-only']
assert not sys.argv[5:] or ADMISSION_ONLY or STREAMING_ONLY, sys.argv[5:]
ROOT = Path(ROOT)
sys.path.insert(0, str(ROOT / "databases"))
from testing import child_environment, read_line
DONE, ROW, ERROR, INTERRUPT = 101, 100, 1, 9


def environment(**extra):
    return {**child_environment(), **extra}


def public_question(question):
    question=json.loads(json.dumps(question))
    if question['function']=='annotate':
        question['questions']={name:public_question(child) for name,child in question['questions'].items()}
    return question


COLUMNS = ['position', 'id', 'answer', 'probability', 'status', 'error_code', 'error_message',
           'error_http_status', 'source', 'detail']


def engine_spans(value):
    """Read SQL span fields back as the engine's start and end."""
    if isinstance(value, list):
        return [engine_spans(item) for item in value]
    if isinstance(value, dict):
        return {{'start_offset': 'start', 'end_offset': 'end'}.get(key, key): engine_spans(item)
                for key, item in value.items()}
    return value


def engine_id(value):
    """SQLite gives an integer id beyond signed 64 bits as TEXT digits."""
    if isinstance(value, str) and value.isdigit() and int(value) >= 2**63:
        return int(value)
    return value


class Host:
    surface = "sqlite"
    def __init__(self, url=None):
        self.url = url
        env = environment()
        if url:
            env.update(THINKTHEN_TEST_LOOPBACK_URL=url, THINKTHEN_API_KEY="loopback-test-key", OPENAI_API_KEY="loopback-test-key")
        self.process = subprocess.Popen([CONSUMER, EXTENSION], stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, text=True, env=env)

    def send(self, command):
        self.process.stdin.write(json.dumps(command) + "\n")
        self.process.stdin.flush()

    def receive(self):
        line = read_line(self.process.stdout)
        assert line, "SQLite host exited before reporting its statement result"
        return json.loads(line)

    def sql(self, sql, parameters=(), **extra):
        self.send(dict(sql=sql, parameters=parameters, **extra))
        return self.receive()

    def ok(self, sql, parameters=(), **extra):
        result = self.sql(sql, parameters, **extra)
        if result["code"] not in (DONE, ROW):
            raise SqlError(result["message"])
        return result["rows"]

    def configure(self, settings):
        for key,value in settings.items():
            for name,item in (value.items() if key=='retry' else [(key,value)]):
                self.ok('SELECT thinkthen_set(?,?)',[name,item])

    def configure_failure(self, settings, code):
        for key, value in settings.items():
            if key != "backend":
                result = self.sql("SELECT thinkthen_set(?,?)", [key, value])
                assert result["code"] == ERROR, result
                assert code in result["message"], result

    def call_failure(self, request, code):
        self.call(request, code)

    def native_failure(self, request):
        result = self.call(request, raw=True)
        assert result['code'] == ERROR and not result['rows'], result

    def call(self, request, expected_error=None, raw=False, interruptible=False):
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
        function=question['function']
        if function=='filter':
            # Filter keeps answered-yes rows; failed rows come back with failures = 'row'.
            constraints.append("failures = 'row'")
        result=self.sql('SELECT * FROM thinkthen_'+function+' WHERE '+' AND '.join(constraints),values, interruptible=interruptible)
        if raw:
            return result
        if expected_error is not None:
            assert result['code'] == ERROR and expected_error in result['message'], result
            assert not result['rows'], result
            return
        assert result['code']==DONE,result
        assert result['columns'][:len(COLUMNS)]==COLUMNS,result['columns']
        rows=[]
        for native in result['rows']:
            row=dict(zip(result['columns'],native))
            answer=row['answer']
            detail=None if row['detail'] is None else engine_spans(json.loads(row['detail']))
            if question['function'] in ('decide','filter') and answer is not None:
                assert type(answer) is int and answer in (0,1),row
                answer=bool(answer)
            elif question['function'] in ('tag','find','recognize','relate','annotate') and answer is not None:
                answer=engine_spans(json.loads(answer))
            if question['function']=='annotate' and detail is not None:
                # The answer column holds each question's bare answer; detail holds its outcome.
                assert answer=={name:child.get('answer') if child['status']=='answered' else None
                                for name,child in detail.items()},row
                answer,detail=detail,None
            if row['status'] != 'answered':
                assert answer is None, row
            error=None if row['error_code'] is None else dict(code=row['error_code'],message=row['error_message'])
            if error is not None and row['error_http_status'] is not None: error['http_status']=row['error_http_status']
            kept=row['status']=='answered' and answer is True
            identifier=None if row['id'] is None else json.dumps(engine_id(row['id']))
            rows.append((row['position'],row['status'],json.dumps(answer) if row['status']=='answered' else None,row['probability'],
                json.dumps(error) if error else None,None if detail is None else json.dumps(detail),kept,identifier,row['source']))
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
from migration_cases import command, fixture, run_cases, options_cases, SqlError, strict_equal
from admission_cases import cases as admission_cases, diagnostic


from consumption import streaming
if STREAMING_ONLY:
    streaming(Host, LOOPBACK)
    sys.exit(0)


with Host() as host:
    host.configure({'backend': 'stub'})
    source = "SELECT ?1 AS id,CASE WHEN ?2=42 AND ?3=1.5 AND ?4=1 AND ?5 IS NULL THEN ?6 END AS record"
    rows = host.ok('SELECT id,status FROM thinkthen_decide(?,?) WHERE params=?',
        [source, 'Ready?', json.dumps(['bound-id', 42, 1.5, True, None, 'evidence'])])
    assert rows == [['bound-id', 'answered']], rows
    for parameters in ['malformed', '{}', '[[]]', '[{}]', '[]', '[1,2]']:
        result = host.sql('SELECT * FROM thinkthen_decide(?,?) WHERE params=?',
            ['SELECT ? AS record', 'Ready?', parameters])
        assert result['code'] == ERROR and not result['rows'], (parameters, result)
        assert result['message'].startswith('usage.argument: params '), (parameters, result)
    result = host.sql('SELECT * FROM thinkthen_rank(?,?) WHERE levels=? AND top=?',
        ["SELECT 'evidence' AS record", 'Order?', '["low","high"]', -1])
    assert result['code'] == ERROR and 'usage.question' in result['message'] and not result['rows'], result
    # Feature: a relation rule names its relation with `relation`, as on every surface.
    relate = "SELECT * FROM thinkthen_relate('SELECT ''Ada works for Acme.'' AS record') WHERE rules = ?"
    assert len(host.ok(relate, ['[{"relation":"works_for","source":"person","target":"organization"}]'])) == 1
    result = host.sql(relate, ['[{"name":"works_for","source":"person","target":"organization"}]'])
    assert result['code'] == ERROR and 'name' in result['message'] and not result['rows'], result
    result = host.sql('SELECT thinkthen_set(?,?)', ['max_retries', 'secret-placeholder'])
    assert result['code'] == ERROR and 'usage.settings' in result['message'], result
    assert 'secret-placeholder' not in result['message'], result
    for case, request, expected in admission_cases(ROOT, 'sqlite'):
        result = host.call(request, raw=True)
        assert result['code'] == ERROR and not result['rows'], (case['name'], result)
        assert 'secret-placeholder' not in json.dumps(result), case['name']
        code, separator, message = result['message'].partition(': ')
        assert separator, result
        diagnostic(case, code, message, expected)
if ADMISSION_ONLY:
    sys.exit(0)
streaming(Host, LOOPBACK)

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
    # Feature: rank's top row keeps its native id while SQLite memory stays flat over a long input.
    def top(count):
        source=f"WITH RECURSIVE rows(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM rows WHERE n<{count}) SELECT n AS id,'x' AS record FROM rows"
        rows=host.ok("SELECT count(*),sum(id=position),typeof(id),max(native_memory()) FROM thinkthen_rank(?,?) WHERE levels='[\"low\",\"high\"]' AND top=1",[source,'Order?'])
        assert rows[0][:3]==[1,1,'integer'],rows
        return rows[0][3]
    small=top(128)
    large=top(65536)
    assert large-small<128*1024,(small,large)

    # Feature: independent ranked cursors keep their spooled ids across a source write.
    host.ok('CREATE TABLE ranked(id INTEGER PRIMARY KEY,record TEXT)')
    host.ok("INSERT INTO ranked VALUES(11,'alpha'),(22,'beta')")
    ranked = "SELECT position,id,answer FROM thinkthen_rank('SELECT id,record FROM ranked ORDER BY id','Order?') WHERE levels='[\"low\",\"high\"]'"
    first = host.sql(ranked, operation='open', slot=0)
    second = host.sql(ranked, operation='open', slot=1)
    assert first['code'] == second['code'] == ROW, (first, second)
    host.ok('BEGIN')
    host.ok("UPDATE ranked SET id=id+100")
    for slot, initial in [(0, first), (1, second)]:
        rows = initial['rows']
        while True:
            result = host.sql('', operation='step', slot=slot)
            rows += result['rows']
            if result['code'] == DONE:
                break
            assert result['code'] == ROW, result
        assert sorted(row[1] for row in rows) == [11,22], rows
        assert host.sql('', operation='close', slot=slot)['code'] == 0
    assert sorted(row[1] for row in host.ok(ranked)) == [111,122]
    host.ok('ROLLBACK')
    assert sorted(row[1] for row in host.ok(ranked)) == [11,22]

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
        "SELECT * FROM thinkthen_decide('SELECT ''{}'' AS record','Ready?') WHERE format = 'text' AND field = '/body'",
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
        rows=host.ok('SELECT answer,probability FROM thinkthen_decide(?,?) WHERE field=?',[source,'Ready?','/body'])
        assert rows==host.ok('SELECT answer,probability FROM thinkthen_decide(?,?) WHERE field=?',
            ["SELECT '{\"body\":\"alpha\"}' AS record ORDER BY record",'Ready?','/body']),rows
    for record in ('NULL','42','1.5',"x'00ff'"):
        rows=host.ok('SELECT status,error_code FROM thinkthen_decide(?,?)',['SELECT '+record+' AS record','Ready?'])
        assert rows==[['failed','record.missing' if record=='NULL' else 'record.invalid']],rows
    for case in json.loads((ROOT/'construction/stub.json').read_text()):
        actual=host.call(case['request'])
        from migration_cases import decode, semantic
        actual=[decode(row,case['request']['question']['function'],'stub','stub') for row in actual]
        expected=[semantic(json.loads(json.dumps(row))) for row in case['expected']
                  if row['function']!='filter' or row['status']=='failed' or row.get('answer') is True]
        assert strict_equal(actual,expected),(case['request'],actual,expected)

TEN = ('decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate')
with Host() as host:
    host.configure({'backend': 'stub'})
    # Feature: all ten functions share the SQL row shape that DuckDB and PostgreSQL use.
    for function in TEN:
        columns = host.ok('SELECT name,hidden FROM pragma_table_xinfo(?)', ['thinkthen_' + function])
        assert [name for name, hidden in columns if not hidden] == COLUMNS, (function, columns)
        hidden = {name for name, flag in columns if flag}
        assert ('failures' in hidden) == (function == 'filter'), (function, hidden)
        assert {'yes_means', 'no_means'} <= hidden or function not in ('decide', 'filter'), (function, hidden)
        assert 'descriptions' in hidden or function not in ('choose', 'tag', 'score', 'rank'), (function, hidden)
    version = next(line.split('"')[1] for line in (ROOT / 'Cargo.toml').read_text().splitlines() if line.startswith('version = '))
    assert host.ok('SELECT thinkthen_version()') == [[version]]

    # Feature: an id from a JSON record keeps its type, so it joins with its source.
    host.ok('CREATE TABLE messages(msg_id, body TEXT)')
    host.ok("INSERT INTO messages VALUES('r7','Please refund the charge.'),(5,'Where is my parcel?')")
    rows = host.ok(
        "SELECT m.body, typeof(t.id) FROM thinkthen_decide t JOIN messages m ON m.msg_id = t.id "
        "WHERE t.source_sql = ? AND t.question = ? AND t.field = ? ORDER BY t.position",
        ["SELECT json_object('id', msg_id, 'text', body) AS record FROM messages ORDER BY rowid", 'Refund?', '/text'])
    assert rows == [['Please refund the charge.', 'text'], ['Where is my parcel?', 'integer']], rows
    rows = host.ok("SELECT id, typeof(id) FROM thinkthen_decide(?,?) WHERE format = 'json'",
        ['SELECT \'{"id":18446744073709551615,"text":"x"}\' AS record', 'Refund?'])
    assert rows == [['18446744073709551615', 'text']], rows

    # Feature: JSON columns carry SQLite's JSON subtype, so they nest as JSON.
    rows = host.ok("SELECT json_object('labels', answer, 'source', source, 'detail', detail) FROM thinkthen_tag(?,?) WHERE labels = ?",
        ["SELECT 'x' AS record", 'Which?', '["a","b"]'])
    nested = json.loads(rows[0][0])
    assert isinstance(nested['labels'], list) and isinstance(nested['source'], dict) and isinstance(nested['detail'], dict), rows
    rows = host.ok("SELECT json_object('names', answer) FROM thinkthen_recognize(?) WHERE kinds = ?",
        ["SELECT 'Ada' AS record", '["person"]'])
    names = json.loads(rows[0][0], object_pairs_hook=lambda pairs: pairs)[0][1]
    assert [[key for key, _ in name] for name in names] == [['text', 'kind', 'start_offset', 'end_offset', 'probability']], rows

    # Feature: filter returns kept rows, then raises once for failed records.
    host.ok('CREATE TABLE judged(n INTEGER PRIMARY KEY, record TEXT)')
    host.ok('INSERT INTO judged VALUES(1,?),(2,?),(3,?),(4,?)', [json.dumps(v) for v in ('refund', 'parcel', 'uncertain')] + ['null'])
    band = "SELECT position,status FROM thinkthen_filter(?,?) WHERE format = 'json' AND low = 0.2 AND high = 0.8"
    source = 'SELECT record FROM judged ORDER BY n'
    result = host.sql(band, [source, 'Ready?'])
    assert result['code'] == ERROR and result['rows'] == [[2, 'answered'], [3, 'answered']], result
    assert result['message'].startswith("record.missing: 1 of the filter's records failed, the first at position 4"), result
    assert host.ok(band + " AND failures = 'row'", [source, 'Ready?']) == [[2, 'answered'], [3, 'answered'], [4, 'failed']]
    # Feature: a failed materialization rolls back its table, leaving the connection usable.
    result = host.sql('CREATE TABLE failed_filter AS ' + band, [source, 'Ready?'])
    assert result['code'] == ERROR and result['message'].startswith('record.missing:'), result
    assert host.ok("SELECT count(*) FROM sqlite_schema WHERE name='failed_filter'") == [[0]]


    # Feature: answer meanings and descriptions reach the question.
    def identity(sql, parameters):
        return json.loads(host.ok(sql, parameters)[0][0])['question_identity']
    plain = identity("SELECT source FROM thinkthen_decide('SELECT ''x'' AS record','Refund?')", [])
    meant = identity("SELECT source FROM thinkthen_decide('SELECT ''x'' AS record','Refund?') WHERE yes_means = ? AND no_means = ?",
        ['asks for money back', 'anything else'])
    assert plain != meant, (plain, meant)
    plain = identity("SELECT source FROM thinkthen_choose('SELECT ''x'' AS record','Team?') WHERE choices = ?", ['["billing","shipping"]'])
    described = identity("SELECT source FROM thinkthen_choose('SELECT ''x'' AS record','Team?') WHERE choices = ? AND descriptions = ?",
        ['["billing","shipping"]', '{"billing":"Invoices and refunds."}'])
    assert plain != described, (plain, described)

    # Feature: the extension's own argument errors name the argument with a shared code.
    for sql, parameters, message in [
        ("SELECT * FROM thinkthen_decide('SELECT ''x'' AS record','Ready?') WHERE threshold = '0.7'", [],
         'usage.argument: threshold must be a number, not text'),
        ("SELECT * FROM thinkthen_decide('SELECT ''x'' AS record')", [],
         'usage.argument: question is required for thinkthen_decide'),
        ("SELECT * FROM thinkthen_decide('SELECT ''x'' AS body','Ready?')", [],
         'usage.argument: source_sql has no column named record; name one with record_column'),
        ("SELECT * FROM thinkthen_choose('SELECT ''x'' AS record','Team?') WHERE choices = 'a,b'", [],
         'usage.argument: choices must be a JSON array of text'),
        ("SELECT * FROM thinkthen_choose('SELECT ''x'' AS record','Team?') WHERE choices = ? AND descriptions = ?",
         ['["billing","shipping"]', '{"sales":"Quotes."}'],
         "usage.argument: descriptions names 'sales', which is not one of the choices"),
        ("SELECT * FROM thinkthen_filter('SELECT ''x'' AS record','Ready?') WHERE failures = 'rows'", [],
         "usage.argument: failures must be 'raise' or 'row', not 'rows'"),
        ("SELECT * FROM thinkthen_decide('DELETE FROM messages RETURNING body AS record','Ready?')", [],
         'usage.argument: source_sql must be read-only'),
    ]:
        result = host.sql(sql, parameters)
        assert result['code'] == ERROR and result['message'].startswith(message) and not result['rows'], (sql, result)
print('SQLite shared shape, typed ids, JSON columns, kept-only filter and argument errors pass')

options_cases(Host)
run_cases(Host, LOOPBACK)

# Feature: filter leaves out answered-no and not-sure rows.
case = json.loads((ROOT / 'conformance/typed/filter-rows.json').read_text())
manifest = json.loads((ROOT / 'conformance/typed/manifest.json').read_text())
with fixture(LOOPBACK, ROOT / 'conformance/typed/filter-rows.json') as (server, url), Host(url) as host:
    host.configure({**manifest['client'], 'backend': manifest['backends'][0]})
    host.ok('CREATE TABLE judged(n INTEGER PRIMARY KEY, record TEXT)')
    for index, record in enumerate(case['request']['records'], 1):
        host.ok('INSERT INTO judged VALUES(?,?)', [index, json.dumps(record)])
    rows = host.ok("SELECT position,status,answer FROM thinkthen_filter(?,?) "
                   "WHERE format = 'json' AND low = 0.2 AND high = 0.8 AND failures = 'row'",
                   ['SELECT record FROM judged ORDER BY n', 'Ready?'])
    assert [row['status'] for row in case['expected']] == ['answered', 'answered', 'unsure', 'failed'], case['expected']
    assert rows == [[1, 'answered', 1], [4, 'failed', None]], rows
for arguments,cause in [(['--unavailable'],'connection refused'),
                        (['--held','reset'],'connection reset'),
                        (['--held','headers'],'timeout')]:
    # Darwin keeps a bound non-listening TCP port silent instead of refusing it.
    if sys.platform == 'darwin' and cause == 'connection refused': continue
    with fixture(LOOPBACK,*arguments) as (server,url),Host(url) as host:
        host.configure({'backend':'openai','retry':{'max_retries':0,'attempt_timeout_ms':200}})
        # Feature: a run the backend fails throughout raises its error and returns no rows.
        result=host.sql('SELECT status,error_code,error_message FROM thinkthen_decide(?,?)',
                        ["SELECT 'x' AS record",'Ready?'])
        assert result['code']==ERROR and not result['rows'],result
        assert result['message']==f'backend.transport: the openai backend could not be reached: {cause}',result
        assert 'loopback-test-key' not in json.dumps(result),result
for status in (401,402,429,500):
    with tempfile.TemporaryDirectory() as temporary:
        case=Path(temporary)/'reply.json'
        case.write_text(json.dumps({'replies':[{'status':status,'body':{'error':
            f'reason for {status} loopback-test-key sk-unknown-sentinel'}}]}))
        with fixture(LOOPBACK,case) as (server,url),Host(url) as host:
            host.configure({'backend':'openai','retry':{'max_retries':0}})
            result=host.sql('SELECT status,error_code,error_http_status,error_message FROM thinkthen_decide(?,?)',
                            ["SELECT 'x' AS record",'Ready?'])
            assert result['code']==ERROR and not result['rows'],result
            assert result['message']==(f'backend.status: the backend answered status {status}: '+
                f'{{"error":"reason for {status} [redacted] [redacted]"}}'),result
            assert 'loopback-test-key' not in json.dumps(result),result
            assert 'sk-unknown-sentinel' not in json.dumps(result),result
            assert command(server,'count')=='1'
# Feature: backend failures before the first answer still come back in order once one is answered.
with tempfile.TemporaryDirectory() as temporary:
    case=Path(temporary)/'replies.json'
    case.write_text(json.dumps({'replies':[{'body':{},'contains':'"bad"','status':400},{'probability':0.9}]}))
    with fixture(LOOPBACK,case) as (server,url),Host(url) as host:
        host.configure({'backend':'openai','batch':1,'retry':{'max_retries':0}})
        host.ok('CREATE TABLE mixed(n INTEGER PRIMARY KEY, record TEXT)')
        for n,record in [(10,'bad'),(20,'bad'),(30,'good'),(40,'bad')]:
            host.ok('INSERT INTO mixed VALUES(?,?)',[n,json.dumps(record)])
        rows=host.ok("SELECT position,id,status,error_code FROM thinkthen_decide(?,?) WHERE format='json'",
                     ['SELECT n AS id, record FROM mixed ORDER BY n','Ready?'])
        assert rows==[[1,10,'failed','backend.status'],[2,20,'failed','backend.status'],
                      [3,30,'answered',None],[4,40,'failed','backend.status']],rows
        for function,extra in [('decide',''),('filter'," AND failures = 'row'"),('filter','')]:
            result=host.sql(f"SELECT * FROM thinkthen_{function}(?,?) WHERE format='json'{extra}",
                            ['SELECT n AS id, record FROM mixed WHERE record=\'"bad"\' ORDER BY n','Ready?'])
            assert result['code']==ERROR and not result['rows'],(function,extra,result)
            assert result['message'].startswith('backend.status: '),(function,extra,result)
for case in json.loads((ROOT / 'construction/stub.json').read_text()):
    function = case['request']['question']['function']
    for mode in ('headers', 'body', 'refusal-body'):
        with fixture(LOOPBACK, '--held', mode) as (server, url), Host(url) as host:
            host.configure({'retry': {'attempt_timeout_ms': 120000}})
            with ThreadPoolExecutor(max_workers=1) as workers:
                future = workers.submit(host.call, case['request'], raw=True, interruptible=True)
                assert command(server, 'arrived') == 'arrived'
                paid = int(command(server, 'count'))
                assert paid > 0
                started = time.monotonic()
                host.process.stdin.write('i'); host.process.stdin.flush()
                assert command(server, 'closed', timeout=2) == 'closed'
                assert time.monotonic() - started < 2, (function, mode)
                result = future.result(timeout=2)
                assert result['code'] == INTERRUPT, result
            assert host.ok('SELECT 1') == [[1]]
            assert int(command(server, 'count')) == paid, (function, mode)
            host.configure({'backend': 'stub'})
            assert len(host.ok("SELECT * FROM thinkthen_decide('SELECT ''x'' AS record','Ready?')")) == 1
print('SQLite typed arguments, native ids, both wires, settings and cancellation pass')

# Feature: repeated filters in one join reuse the cursor's paid result.
case = ROOT / 'conformance/typed/decide-precision.json'
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
