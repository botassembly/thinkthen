"""Execute every literal ideal-guide statement through a supported native host."""
from contextlib import contextmanager
import json
from pathlib import Path
import sqlite3
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'scripts'))
sys.path.insert(0, str(ROOT / 'databases'))
from guide_examples import blocks, scenario
from testing import read_line


@contextmanager
def host(command, env):
    process = subprocess.Popen(command, stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, env=env)
    def sql(statement, parameters=()):
        process.stdin.write(json.dumps(dict(sql=statement, parameters=parameters)) + '\n')
        process.stdin.flush()
        return json.loads(read_line(process.stdout))
    try:
        yield sql
    finally:
        process.stdin.close()
        assert process.wait(timeout=20) == 0


def statements(body):
    pending = ''
    for char in body:
        pending += char
        if char == ';' and sqlite3.complete_statement(pending):
            yield pending.strip()
            pending = ''
    assert not pending.strip(), pending


def count(server):
    server.stdin.write('count\n')
    server.stdin.flush()
    return int(read_line(server.stdout))


def rows(sql, statement, expected, parameters=()):
    result = sql(statement, parameters)
    assert result['code'] == 101 and result['rows'] == expected, (statement, expected, result)
    return result


def run(command, negative=None):
    guide = blocks(ROOT / 'docs/surfaces/sqlite.md')
    assert len(guide) == 32 and all(lang == 'sql' for lang, _, _ in guide), 'unaccounted guide block'
    names = ('yes', 'no', 'unsure', 'failed', 'refund-failed', 'topic-failed',
             'urgency-failed', 'refund-unsure', 'topic-unsure', 'low-score', 'relations')
    failures = []
    for name in names:
        try:
            servers = []
            with scenario(name, observer=servers) as (_, env):
                env['THINKTHEN_TEST_LOOPBACK_URL'] = env['THINKTHEN_BASE_URL']
                with host(command, env) as sql:
                    rows(sql, "SELECT thinkthen_set('max_retries',0)", [[1]])
                    for index, (_, body, line) in enumerate(guide):
                        if index == 0:
                            load, body = body.split('\n', 1)
                            assert load == '.load ./thinkthen_sqlite sqlite3_thinkthensqlite_init'
                            # The native consumer performs that exact load entrypoint before accepting SQL.
                        for statement in statements(body):
                            result = sql(statement)
                            fails = name in ('failed', 'refund-failed') and index in (9, 15)
                            if fails:
                                assert result['code'] == 1 and 'records failed' in result['message'], (name, line, result)
                            else:
                                assert result['code'] == 101, (name, line, statement, result)
                            check_other(sql, name, index, statement, result, fails)
                            if index in (2, 3, 5, 6, 7, 12, 14, 20, 23, 27):
                                check_branch(name, index, result['rows'], negative)
                            if index == 1:
                                state = 'failed' if name in ('failed','refund-failed') else 'unsure' if name in ('unsure','refund-unsure') else 'answered'
                                answer = None if state != 'answered' else int(name != 'no')
                                expected = [[n, text, answer, state] for n, text in MESSAGES]
                                assert result['rows'] == expected, (name, line, expected, result)
                            if index in (24, 25):
                                assert result['rows'] and all(row[:2] == [1, None] for row in result['rows']), (name, line, result)
                                if name == 'relations':
                                    expected = ([[1,None,'answered','person','Morgan'], [1,None,'answered','organization','Example Group']] if index == 24
                                                else [[1,None,'answered','Morgan','works_for','Example Group']])
                                    assert result['rows'] == expected, (line, result)
                                if name == 'failed' or name == 'unsure' or name == 'no' and index == 25:
                                    expected = [[1,None,'failed' if name == 'failed' else 'answered'] + [None]*(2 if index == 24 else 3)]
                                    assert result['rows'] == expected, (line, result)
                            if index == 23:
                                expected = [['Please refund my order.','Please refund my order.']] if name not in ('failed','topic-failed','unsure','topic-unsure','no') else []
                                rows(sql,"SELECT substr(body,(answer ->> '$.start_offset')+1,(answer ->> '$.end_offset')-(answer ->> '$.start_offset')),answer ->> '$.text' FROM action WHERE status='answered' AND answer IS NOT NULL",expected)
                            if index == 27:
                                saved = sql('SELECT msg_id,detail FROM forms ORDER BY msg_id')
                                assert saved['code'] == 101 and [row[0] for row in saved['rows']] == [1,2,3], saved
                                for _,encoded in saved['rows']:
                                    for child,outcome in json.loads(encoded).items():
                                        if name == 'failed':
                                            expected_error = {'code':'backend.status','message':'the backend answered status 503: {"error":"unavailable"}','http_status':503}
                                        elif name == child+'-failed':
                                            expected_error = {'code':'backend.reply','message':'the backend gave a malformed answer to the question'}
                                        else:
                                            expected_error = None
                                        assert outcome.get('error') == expected_error, (name,child,outcome)
                                call = guide[26][1].replace('CREATE TABLE forms AS\n','',1).rstrip().removesuffix(';') + ' AND raise_if_failed=1'
                                raised = sql(call)
                                if name in ('failed','refund-failed','topic-failed','urgency-failed'):
                                    assert raised['code'] == 1 and '3 of 3 records failed' in raised['message'] and 'position 1' in raised['message'], (name,raised)
                                else:
                                    assert raised['code'] == 101 and len(raised['rows']) == 3, (name,raised)
                            if index == 18 and 'LEFT JOIN' in statement:
                                assert result['rows'] and all(row[:2] == [1, None] for row in result['rows']), result
                                if name in ('no','failed'):
                                    assert result['rows'] == [[1,None,'failed' if name == 'failed' else 'answered',None]], result
                    # Saved answers never make another paid request.
                    paid = count(servers[0])
                    rows(sql, 'SELECT count(*) FROM refund_checks', [[3]])
                    assert count(servers[0]) == paid
                    print(f'PASS sqlite/ideal/{name}: every literal statement, output and branch')
        except AssertionError as error:
            failures.append((name,str(error)))
            print(f'FAIL sqlite/ideal/{name}: {error}', flush=True)
    assert not failures, failures
    other_choice(command, guide)
    refusals(command, negative)
    limit_probe(command)


MESSAGES = [(1,'Please refund my order.'), (2,'Where is my parcel?'),
            (3,'I was charged twice for one invoice.'),
            (4,'I was charged twice and my parcel is late again.'),
            (5,'Morgan works for Example Group.\nPlease refund my order.')]


def check_other(sql, name, index, statement, result, fails):
    if fails:
        return
    actual = result['rows']
    pf = name in ('failed','refund-failed')
    pu = name in ('unsure','refund-unsure')
    p = None if pf else .05 if name == 'no' else .5 if pu else .95
    answer = None if pf else int(name != 'no')
    state = 'failed' if pf else 'unsure' if pu else 'answered'
    choice_failed = name in ('failed','topic-failed')
    choice_unsure = name in ('unsure','topic-unsure')
    choice = None if choice_failed or choice_unsure else 'shipping' if name == 'no' else 'billing'
    expected = None
    if statement.startswith(('CREATE','INSERT')):
        expected = []
    elif index == 8:
        saved = sql('SELECT status,error_code,error_message FROM is_refund')['rows'][0]
        expected = [] if not pf else [['could not judge it: failed: '+saved[1]+': '+saved[2]]]
    elif index == 9:
        expected = [[answer]]
    elif index == 10:
        expected = [[p]]
    elif index == 15:
        expected = [] if name == 'no' else [[n,text] for n,text in MESSAGES[:3]]
    elif index == 16:
        if "status = 'failed'" in statement:
            actual_ids = [row[:2] for row in actual]
            expected_ids = [[n,text] for n,text in MESSAGES[:3]] if pf else []
            assert actual_ids == expected_ids and all(row[2] in ('backend.status','backend.reply') for row in actual), (name,index,result)
            return
        expected = [] if pf or pu or name == 'no' else [[n,text] for n,text in MESSAGES[:3]]
    elif index == 18:
        if 'LEFT JOIN' in statement:
            tag_state = 'failed' if pf else 'answered'
            expected = [[1,None,tag_state,None]] if pf or name == 'no' else [[1,None,'answered','billing'],[1,None,'answered','shipping']]
        else:
            expected = [] if pf or name == 'no' else [['billing team']]
    elif index == 21:
        expected = [[None if name in ('failed','urgency-failed') else n,text] for n,text in MESSAGES[:3]]
    elif index == 28:
        expected = [[choice, 'failed' if choice_failed else 'unsure' if choice_unsure else 'answered']] if 'thinkthen_choose' in statement else [[answer,'failed' if pf else 'answered']]
    elif index == 29:
        expected = [['a1',answer,'failed' if pf else 'answered']]
    elif index == 30:
        expected = [[state,3]] if 'count(*)' in statement else [[n,text] for n,text in MESSAGES[:3]] if pu else []
    elif index == 31:
        expected = [[1]]
    if expected is not None:
        assert actual == expected, (name,index,expected,result)


def check_branch(name, index, actual, negative):
    predicate_failed = name in ('failed','refund-failed')
    predicate_unsure = name in ('unsure','refund-unsure')
    queue = 'inbox' if name == 'no' else 'refund queue'
    guarded = 'try again later' if predicate_failed else 'a person reads it' if predicate_unsure else queue
    if index == 2:
        expected = [] if predicate_failed or name == 'no' else [['refund queue']]
    elif index == 3:
        expected = [[None if predicate_failed else queue]]
    elif index == 5:
        expected = [['a person reads it' if predicate_failed or predicate_unsure else queue]]
    elif index == 6:
        p = None if predicate_failed else .05 if name == 'no' else .5 if predicate_unsure else .95
        assert len(actual) == 1 and actual[0][:2] == [guarded,p], (name,index,actual)
        assert (actual[0][2] is not None) == predicate_failed, (name,index,actual)
        return
    elif index == 7:
        expected = [[guarded]]
    elif index == 12:
        expected = [[None if name in ('failed','topic-failed') else 'a person reads it' if name in ('unsure','topic-unsure') else 'warehouse' if name == 'no' else 'billing team']]
    elif index == 14:
        expected = [[n,text,guarded] for n,text in MESSAGES[:3]]
    elif index == 20:
        expected = [[None,None]] if name in ('failed','urgency-failed') else [[0,None]] if name == 'low-score' else [[.5,None]] if name == 'unsure' else [[1,'answer today']]
    elif index == 23:
        # Finding no line is an answered null, distinct from a failed/unsure row.
        if name in ('failed','topic-failed'):
            expected = [[None]]
        elif name == 'no':
            expected = [['no line answers']]
        elif name in ('unsure','topic-unsure'):
            expected = [['a person reads it']]
        else:
            expected = [['Please refund my order.']]
    elif index == 27:
        branch = 'try again later' if name in ('failed','refund-failed','topic-failed','urgency-failed') else 'a person reads it' if name in ('unsure','refund-unsure','topic-unsure') else queue
        assert len(actual) == 3 and all(row[-1] == branch for row in actual), (name,index,actual)
        for row in actual:
            assert row[2] == 'answered', (name,index,row)
            for offset,target in enumerate(('refund','topic','urgency'),3):
                expected_state = 'failed' if name in ('failed',target+'-failed') else 'unsure' if target != 'urgency' and name in ('unsure',target+'-unsure') else 'answered'
                assert row[offset] == expected_state, (name,index,target,row)
        return
    else:
        raise AssertionError(index)
    if negative in ('output','branch') and name == 'yes' and index == 2:
        expected = [['deliberately wrong branch']]
    assert actual == expected, (name,index,expected,actual)


def other_choice(command, guide):
    # Reuse the scripted fixture's distribution override for the third literal branch.
    case = {'replies':[{'distribution':{'billing':0,'shipping':0,'other':1}}]}
    with scenario('other',case=case) as (_,env):
        env['THINKTHEN_TEST_LOOPBACK_URL'] = env['THINKTHEN_BASE_URL']
        with host(command,env) as sql:
            for index in (0,11,12):
                body = guide[index][1]
                if index == 0:
                    body = body.split('\n',1)[1]
                for statement in statements(body):
                    result = sql(statement)
                    assert result['code'] == 101, result
                    assert result['rows'] == ([['front desk']] if index == 12 else []), result
            print('PASS sqlite/ideal/other-choice: literal front-desk branch')


def refusals(command, negative):
    servers = []
    with scenario('yes', observer=servers) as (_,env):
        env['THINKTHEN_TEST_LOOPBACK_URL'] = env['THINKTHEN_BASE_URL']
        with host(command,env) as sql:
            rows(sql,'CREATE TABLE source(record)',[])
            rows(sql,"INSERT INTO source VALUES('x')",[])
            cases = [
                ("SELECT * FROM thinkthen_filter('SELECT ''x'' AS record','Ready?') WHERE low=.2 AND high=.8", (), 'usage.threshold'),
                ("SELECT * FROM thinkthen_decide('SELECT ''x'' AS record','Ready?') WHERE threshold='x'", (), 'usage.argument'),
                ("SELECT * FROM thinkthen_decide('DELETE FROM source RETURNING record','Ready?')", (), 'read-only'),
                ("SELECT thinkthen_set('api_key','not-a-key')", (), 'usage.settings'),
                ("SELECT * FROM thinkthen_annotate('SELECT ''x'' AS record') WHERE questions=?", (json.dumps({'status':{'function':'decide','question':'Ready?'}}),), 'usage.column'),
                ("SELECT * FROM thinkthen_annotate('SELECT ''x'' AS record') WHERE questions=?", (json.dumps({'record':{'function':'decide','question':'Ready?'}}),), 'usage.column'),
                ("SELECT * FROM thinkthen_filter('SELECT ''x'' AS record','Ready?') WHERE failures='row'", (), 'no such column'),
            ]
            for column in ('position','id','answer','probability','error_code','error_message','error_http_status','source','detail'):
                cases.append(("SELECT * FROM thinkthen_annotate('SELECT ''x'' AS record') WHERE questions=?",
                              (json.dumps({column:{'function':'decide','question':'Ready?'}}),), 'usage.column'))
            for statement, parameters, diagnostic in cases:
                if negative == 'refusal':
                    statement,parameters = "SELECT 1", ()
                before = count(servers[0])
                result = sql(statement,parameters)
                assert result['code'] == 1 and diagnostic in result['message'], (statement,result)
                assert count(servers[0]) == before, (statement,'paid refusal')
            # A present SQL NULL id overrides an embedded id, including ranked replay.
            record = '{"id":"embedded","text":"x"}'
            source = "SELECT NULL AS id,'"+record+"' AS record"
            for function in ('decide','filter','rank'):
                options = "format='json'" + (" AND levels='[\"routine\",\"urgent\"]'" if function == 'rank' else '')
                call = f"thinkthen_{function}(?, 'Ready?')"
                rows(sql,f'SELECT id,typeof(id),record FROM {call} WHERE {options}',[[None,'null',record]],[source])
                before = count(servers[0])
                replay = f'SELECT t.id,typeof(t.id),t.record FROM (SELECT 1 UNION ALL SELECT 2) q LEFT JOIN {call} t ON 1=1 AND {options}'
                rows(sql,replay,[[None,'null',record],[None,'null',record]],[source])
                assert count(servers[0])-before == 1, (function,'replay made another paid request')
            # A failed outer form also counts once and never sends its missing record.
            before = count(servers[0])
            question = json.dumps({'refund':{'function':'decide','question':'Ready?'}})
            call = "SELECT status,answer,error_code FROM thinkthen_annotate('SELECT NULL AS record') WHERE questions=?"
            rows(sql,call,[['failed',None,'record.missing']],[question])
            result = sql(call+' AND raise_if_failed=1',[question])
            assert result['code'] == 1 and result['rows'] == [['failed',None,'record.missing']] and '1 of 1 records failed' in result['message'], result
            assert count(servers[0]) == before
            # SQLite's OR planner cannot deliver this option as a virtual-table argument.
            rows(sql,"SELECT answer FROM thinkthen_decide('SELECT ''x'' AS record','Ready?') WHERE threshold=.99",[[0]])
            rows(sql,"SELECT answer FROM thinkthen_decide('SELECT ''x'' AS record','Ready?') WHERE threshold=.99 OR answer",[[1]])
            # Names stay nested; no physical refund_status column is generated.
            questions = {key:{'function':'decide','question':'Ready?'} for key in ('refund','refund_status')}
            result = sql("SELECT status FROM thinkthen_annotate('SELECT ''x'' AS record') WHERE questions=?", [json.dumps(questions)])
            assert result['code'] == 101 and result['rows'] == [['answered']], result
            sql('CREATE VIEW prohibited AS SELECT * FROM thinkthen_decide(\'SELECT \'\'x\'\' AS record\',\'Ready?\')')
            result = sql('SELECT * FROM prohibited')
            assert result['code'] == 1 and 'unsafe' in result['message'], result
            print('PASS sqlite/ideal/refusals: before paid requests, actual physical collisions, view safety')


def limit_probe(command):
    # One response contains a valid first answer and malformed second answer.
    # Both records are paid/judged in one batch before the first SQL row is yielded.
    case = {'replies':[{'contains':'"bad"','answer':None},{'probability':.95}]}
    for function in ('decide','filter'):
        servers = []
        with scenario('yes',case=case,observer=servers) as (_,env):
            env['THINKTHEN_TEST_LOOPBACK_URL'] = env['THINKTHEN_BASE_URL']
            with host(command,env) as sql:
                rows(sql,"SELECT thinkthen_set('batch',2)",[[1]])
                rows(sql,"SELECT thinkthen_set('max_retries',0)",[[1]])
                source = "SELECT 1 id,tick('good') record UNION ALL SELECT 2,tick('bad')"
                query = f"SELECT position,status FROM thinkthen_{function}(?, 'Ready?')"
                if function == 'decide':
                    query += ' WHERE raise_if_failed=1'
                before = count(servers[0])
                result = sql(query+' LIMIT 1',[source])
                assert result['code'] == 101 and result['rows'] == [[1,'answered']], result
                rows(sql,'SELECT tick_count()',[[2]])
                assert count(servers[0])-before == 1
                result = sql(query,[source])
                expected_rows = [[1,'answered'],[2,'failed']] if function == 'decide' else [[1,'answered']]
                assert result['rows'] == expected_rows and result['code'] == 1 and 'position 2' in result['message'] and 'backend.reply' in result['message'], result
                before = count(servers[0])
                result = sql(query,[f'SELECT * FROM ({source}) LIMIT 1'])
                assert result['code'] == 101 and result['rows'] == [[1,'answered']], result
                assert count(servers[0])-before == 1
                rows(sql,'SELECT tick_count()',[[5]])
                print(f'PASS sqlite/ideal/{function}/outer-limit: consumed=2, judged=2, paid_batches=1; full call raises; inner limit consumes/judges 1')


if __name__ == '__main__':
    run(sys.argv[1:3], None if len(sys.argv)<4 else sys.argv[3])
