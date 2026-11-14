"""Run the PostgreSQL ideal page through the installed supported server."""
import json
from pathlib import Path
import re
import sys
import traceback

import psycopg2

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'scripts'))
sys.path.insert(0, str(ROOT / 'databases/duckdb/tests'))
from guide_examples import IDEAL, blocks, scenario
from ideal_guides import expected, original, answer, status
from migration_cases import command


def statements(body):
    start, index, quoted, dollar = 0, 0, False, None
    while index < len(body):
        if dollar:
            if body.startswith(dollar, index):
                index += len(dollar)
                dollar = None
                continue
        elif body[index] == "'":
            quoted = not quoted
        elif not quoted:
            delimiter = re.match(r'\$(?:[A-Za-z_]\w*)?\$', body[index:])
            if delimiter:
                dollar = delimiter[0]
                index += len(dollar)
                continue
            if body[index] == ';':
                yield body[start:index + 1]
                start = index + 1
        index += 1
    assert not body[start:].strip(), body[start:]


def cell(value):
    if value is None:
        return None
    if isinstance(value, bool):
        return str(value).lower()
    if isinstance(value, (dict, list)):
        return json.dumps(value, sort_keys=True, ensure_ascii=False)
    return str(value)


def displayed(database, statement):
    with database.cursor() as cursor:
        cursor.execute(statement)
        count = cursor.rowcount
        if cursor.description is None:
            return [], count
        columns = [column.name for column in cursor.description]
        return [dict(zip(columns, map(cell, row))) for row in cursor.fetchall()], count


def wanted(index, part, name):
    if index == 31:
        return [{'msg_id': '1', 'is_refund': answer(name), 'status': status(name)}]
    if index == 32:
        return [{**row, 'is_refund': answer(name, True), 'status': status(name, True)} for row in original()]
    if index == 33:
        return expected(31, part, name)
    if index in (34, 35):
        return []
    return expected(index, part, name)


def examples(options, server, query, failure, name, negative=None):
    authored = blocks(IDEAL / 'postgresql.md')
    assert len(authored) == 37, 'unaccounted PostgreSQL guide block'
    case = {'replies': [{'distribution': {'billing': 0, 'shipping': 0, 'other': 1}}]} if name == 'other' else None
    with scenario(name, case=case) as (_, env):
        env['THINKTHEN_TEST_LOOPBACK_URL'] = env['THINKTHEN_BASE_URL']
        with server(env, install=False) as database:
            failures, covered = [], []
            for index, (language, body, line) in enumerate(authored):
                assert language == 'sql', (index, language)
                if index == 36:
                    # Migration runs separately against the actual published-source declarations.
                    assert body.strip() == 'DROP EXTENSION thinkthen;\nCREATE EXTENSION thinkthen;'
                    continue
                if name == 'low-score' and index not in (0, 1, 20, 21):
                    continue
                if name == 'other' and index not in (0, 1, 12, 13):
                    continue
                if name == 'relations' and index not in (0, 1, 25, 26):
                    continue
                if '-' in name and name != 'low-score' and index not in (0, 1, 27, 28):
                    continue
                if index in (25, 26) and name not in ('relations', 'unsure', 'failed'):
                    continue
                for part, statement in enumerate(statements(body)):
                    label = f'postgresql.md:{line} statement {part + 1} [{name}]'
                    if name == 'failed' and index in (10, 16):
                        error = failure(database, statement, '58000', 'backend.status')
                        assert ('1 of 1' if index == 10 else '3 of 3') + ' records failed' in error.diag.message_primary, error
                    else:
                        rows, count = displayed(database, statement)
                        expected_rows = wanted(index, part, name)
                        if expected_rows and set(expected_rows[0]) == {'Count'}:
                            assert rows == [] and count == int(expected_rows[0]['Count']), (label, rows, count)
                        else:
                            if negative == 'output' and index == 3:
                                expected_rows = [{'where_it_goes': 'wrong queue'}]
                            if negative == 'branch' and index == 6:
                                expected_rows = [{'where_it_goes': 'wrong branch'}]
                            if index in (19, 25, 26):
                                rows = sorted(rows, key=lambda row: json.dumps(row, sort_keys=True))
                                expected_rows = sorted(expected_rows, key=lambda row: json.dumps(row, sort_keys=True))
                            if rows != expected_rows:
                                failures.append(str((label, rows, expected_rows)))
                    covered.append((index, part))
                if index == 0:
                    query(database, 'SET thinkthen.max_retries = 0')
                    query(database, "SET thinkthen.allowed_backends = 'systemone:*'")
            assert not failures, '\n'.join(failures)
            print(f'PASS postgresql [{name}]: {len(covered)} literal statements, output and branches', flush=True)


def run(options, server, query, failure, negative=None):
    import os
    os.environ['THINKTHEN_LOOPBACK'] = str(options.loopback)
    failures = []
    for name in ('yes', 'no', 'unsure', 'failed', 'low-score', 'other', 'relations',
                 'refund-failed', 'topic-failed', 'urgency-failed', 'refund-unsure', 'topic-unsure'):
        try:
            examples(options, server, query, failure, name, negative)
        except Exception as error:
            traceback.print_exc()
            print(str(error), file=sys.stderr, flush=True)
            failures.append(str(error))
    for label, check in (
        ('host contracts', lambda: host_contracts(options, server, query, failure, negative)),
        ('outer LIMIT', lambda: limit_probe(options, server, query, failure)),
        ('published migration', lambda: migration(options, server, query, failure)),
    ):
        try:
            check()
        except Exception as error:
            traceback.print_exc()
            print(label + ': ' + str(error), file=sys.stderr, flush=True)
            failures.append(label + ': ' + str(error))
    assert not failures, '\n'.join(failures)
    if negative is None:
        for control in ('output','branch','refusal'):
            try:
                if control == 'refusal':
                    host_contracts(options, server, query, failure, control)
                else:
                    examples(options, server, query, failure, 'yes', control)
            except AssertionError:
                print('PASS PostgreSQL runner rejects wrong ' + control, flush=True)
            else:
                raise AssertionError('runner accepted negative control: ' + control)


def migration(options, server, query, failure):
    from contextlib import closing
    import hashlib
    import shutil
    import subprocess
    import tempfile
    here = Path(__file__).resolve().parent
    schema = here / 'thinkthen--0.1.2.sql'
    assert hashlib.sha256(schema.read_bytes()).hexdigest() == 'a080b263d57390b6d966df5812d232b115df84a42403cfab16f6db5e28a6869c'
    control = here / 'published-0.1.2.control'
    assert hashlib.sha256(control.read_bytes()).hexdigest() == '78ca14bb073167ac90521600e90523a9eed8551d41dfae682f38c218efce7be6'
    with scenario('yes') as (_, env), tempfile.TemporaryDirectory(prefix='published-pg-migration-') as scratch:
        env['THINKTHEN_TEST_LOOPBACK_URL'] = env['THINKTHEN_BASE_URL']
        shared = Path(subprocess.check_output(['pg_config', '--sharedir'], text=True).strip()) / 'extension'
        folder = Path(scratch) / 'extension'
        folder.mkdir()
        shutil.copy(schema, folder / schema.name)
        shutil.copy(control, folder / 'thinkthen.control')
        shutil.copy(shared / 'thinkthen--0.2.0.sql', folder / 'thinkthen--0.2.0.sql')
        metadata = "SELECT e.extversion,n.nspname FROM pg_extension e JOIN pg_namespace n ON n.oid=e.extnamespace WHERE extname='thinkthen'"
        with server(env, install=False) as database:
            query(database, 'SET extension_control_path = %s', (scratch,))
            # PostgreSQL's C validator ignores check_function_bodies. The old
            # PG16 binary cannot load into PG18, and old functions never run.
            # Register the exact published declarations in this disposable DB,
            # restoring normal validation atomically before any new code runs.
            validator = query(database, "SELECT lanvalidator::oid FROM pg_language WHERE lanname='c'")[0][0]
            query(database, 'BEGIN')
            try:
                query(database, "UPDATE pg_language SET lanvalidator=0 WHERE lanname='c'")
                query(database, "CREATE EXTENSION thinkthen VERSION '0.1.2'")
                query(database, "UPDATE pg_language SET lanvalidator=%s::oid WHERE lanname='c'", (validator,))
                query(database, 'COMMIT')
            except Exception:
                query(database, 'ROLLBACK')
                raise
            assert query(database, "SELECT lanvalidator::oid FROM pg_language WHERE lanname='c'") == [(validator,)]
            assert query(database, metadata) == [('0.1.2', 'public')]
            assert query(database, "SELECT count(*) FROM pg_proc WHERE proname LIKE %s", ('thinkthen\\_%',)) == [(34,)]
            query(database, 'CREATE TABLE migration_records(id integer PRIMARY KEY, record text)')
            query(database, "INSERT INTO migration_records VALUES(7,'ready'),(9,NULL)")
            query(database, "CREATE VIEW old_answers AS SELECT thinkthen_decide('Ready?'::text,'text'::text) AS answer")
            # Copying the new package's control replaces the source's old control,
            # exactly as the documented upgrade does before DROP/recreate.
            shutil.copy(shared / 'thinkthen.control', folder / 'thinkthen.control')
            error = failure(database, "ALTER EXTENSION thinkthen UPDATE TO '0.2.0'")
            assert 'no update path' in error.diag.message_primary, error
            failure(database, 'DROP EXTENSION thinkthen', '2BP01')
            assert query(database, 'SELECT count(*) FROM migration_records') == [(2,)]
            query(database, 'DROP VIEW old_answers')
            body = blocks(IDEAL / 'postgresql.md')[-1][1]
            for statement in statements(body):
                query(database, statement)
            assert query(database, metadata) == [('0.2.0', 'thinkthen')]
            assert query(database, "SELECT count(*) FROM pg_proc WHERE proname LIKE %s", ('thinkthen\\_%',)) == [(0,)]
            assert query(database, 'TABLE migration_records ORDER BY id') == [(7,'ready'),(9,None)]
            dump = Path(scratch) / 'recreated.sql'
            with dump.open('w') as output:
                subprocess.run(['pg_dump', '--dbname', database.dsn], stdout=output, check=True, env=env)
            query(database, 'CREATE DATABASE restored_ideal')
            try:
                with closing(psycopg2.connect(database.dsn, dbname='restored_ideal')) as restored:
                    restored.autocommit = True
                    subprocess.run(['psql','-X','-v','ON_ERROR_STOP=1','--dbname',restored.dsn,'--file',str(dump)],
                                   stdout=subprocess.DEVNULL, check=True, env=env)
                    assert query(restored, metadata) == [('0.2.0','thinkthen')]
                    assert query(restored, 'TABLE migration_records ORDER BY id') == [(7,'ready'),(9,None)]
                    query(restored, "SET thinkthen.allowed_backends = 'systemone:*'")
                    rows = query(restored, "SELECT position,id,record,status,error_code FROM thinkthen.decide('SELECT id,record FROM migration_records ORDER BY id','Ready?')")
                    assert rows == [(1,7,'ready','answered',None),(2,9,None,'failed','record.missing')], rows
            finally:
                query(database, 'DROP DATABASE restored_ideal')
    print('PASS actual published0.1.2 definitions, dependency refusal, DROP/recreate and dump restore', flush=True)


def host_contracts(options, server, query, failure, negative=None):
    with scenario('yes', observer=(servers := [])) as (_, env), server_for(env, server) as database:
        print('PostgreSQL actual server:', query(database, 'SHOW server_version')[0][0], flush=True)
        query(database, "SET thinkthen.allowed_backends = 'systemone:*'")
        query(database, 'SET thinkthen.max_retries = 0')
        refusals = [
            ("SELECT * FROM thinkthen.filter($$SELECT 'text' AS record$$,'Ready?',low=>.2,high=>.8)", '22023', 'usage.threshold'),
            ("SELECT * FROM thinkthen.decide($$SELECT 42 AS record$$,'Ready?')", '42804', None),
            ("SELECT * FROM thinkthen.decide($$SELECT 'text' AS body$$,'Ready?')", '42703', None),
            ("SELECT * FROM thinkthen.decide($$SELECT 'text' AS record$$,'Ready?',params=>'{}')", '22023', None),
            ("SELECT * FROM thinkthen.decide(NULL,'Ready?')", '22004', None),
            ("SELECT * FROM thinkthen.filter($$SELECT 'text' AS record$$,'Ready?',failures=>'row')", '42883', None),
            ("SELECT * FROM thinkthen.annotate($$SELECT 'text' AS record$$,toolbox=>'{}')", '42883', None),
        ]
        calls = (
            ('choose', "'Which?',choices=>ARRAY['billing','shipping']"),
            ('tag', "'Which?',labels=>ARRAY['billing','shipping']"),
            ('score', "'How?',levels=>ARRAY['routine','urgent']"),
            ('rank', "'How?',levels=>ARRAY['routine','urgent']"), ('find', "'Where?'"),
            ('recognize', "kinds=>ARRAY['person']"),
            ('relate', "rules=>ARRAY[ROW('person','organization','works_for',NULL,false)::thinkthen.rule]"),
            ('annotate', "questions=>'{\"ready\":{\"function\":\"decide\",\"question\":\"Ready?\"}}'"),
        )
        for function, arguments in calls:
            refusals.append(("SELECT * FROM thinkthen." + function + "($$SELECT 'text' AS record$$," + arguments + ') WHERE answer', '42804', None))
        for name in ('position','id','record','answer','probability','status','error_code','error_message','error_http_status','source','detail'):
            question = {name: {'function': 'decide','question':'Ready?'}}
            refusals.append(("SELECT * FROM thinkthen.annotate($$SELECT 'text' AS record$$,questions=>'" + json.dumps(question) + "')", '22023', 'usage.column'))
        for statement, state, code in refusals:
            error = failure(database, 'SELECT 1' if negative == 'refusal' else statement, state, code)
            if code == 'usage.threshold':
                assert error.diag.message_hint == 'Use thinkthen.decide for not-sure results', error
        for function, arguments in (('decide', "'Ready?'"), *calls):
            statement = 'SELECT * FROM thinkthen.' + function + '($$SELECT NULL::text AS record$$,' + arguments + ',raise_if_failed=>true)'
            error = failure(database, statement, '22000','record.missing')
            assert '1 of 1 records failed' in error.diag.message_primary, (function,error)
        assert command(servers[0], 'count') == '0', 'refused calls sent requests'
        question = {name: {'function':'decide','question':'Ready?'} for name in ('refund','refund_status')}
        rows = query(database, "SELECT answer FROM thinkthen.annotate($$SELECT 'text' AS record$$,questions=>%s::jsonb)", (json.dumps(question),))
        assert rows == [({'refund':True,'refund_status':True},)], rows
        before = command(servers[0], 'count')
        rows = query(database, "SELECT record,status,error_code FROM thinkthen.decide($$SELECT record FROM(VALUES(''),('  '),(NULL)) input(record)$$,'Ready?')")
        assert rows == [('', 'failed','record.empty'),('  ','failed','record.empty'),(None,'failed','record.missing')], rows
        assert query(database, "SELECT * FROM thinkthen.decide($$SELECT 'text' AS record WHERE false$$,'Ready?')") == []
        assert command(servers[0], 'count') == before
        for cast, expected_state, identifier in (('text','answered',None),('json','failed','kept'),('jsonb','failed','kept')):
            source = "SELECT '{\"id\":\"kept\",\"text\":\"\"}'::" + cast + ' AS record'
            rows = query(database, "SELECT id,status,error_code FROM thinkthen.decide(%s,'Ready?',field=>'/text')", (source,))
            assert rows == [(identifier,expected_state,'record.empty' if expected_state == 'failed' else None)], rows
        # All JSON classes and large numeric bytes are checked by PostgreSQL itself,
        # before Python's JSON decoder can turn a huge exponent into a float.
        for encoded in ('null','true','false','"alpha"','[1e1000,false]','{"n":1e1000}',
                        '1e1000','123456789012345678901234567890.123456789'):
            source = "SELECT '" + encoded + "'::json AS id,'text'::text AS record"
            rows = query(database, "SELECT id IS NOT DISTINCT FROM %s::jsonb,record,status FROM thinkthen.decide(%s,'Ready?')", (encoded,source))
            assert rows == [(True,'text','answered')], (encoded,rows)
        for selected, expected_null in (("NULL::json",False),("'null'::json",True)):
            source = 'SELECT ' + selected + ' AS id,\'{"id":42,"text":"text"}\'::json AS record'
            rows = query(database, "SELECT id= 'null'::jsonb,id FROM thinkthen.decide(%s,'Ready?')", (source,))
            assert rows == [(True,None)] if expected_null else rows == [(False,42)], rows
        line = 'Please refund my order.'
        body = 'é😀 lead\n' + line
        source = "SELECT '" + body + "'::text AS record"
        rows = query(database, "SELECT record,(answer).text,substr(record,(answer).start_offset+1,(answer).end_offset-(answer).start_offset),pg_typeof((answer).start_offset)::text FROM thinkthen.find(%s,'Where?')", (source,))
        assert rows == [(body,line,line,'integer')], rows
        # A host conversion failure reaches the shared accounting path too.
        source = "SELECT '\"\\u0000\"'::json AS id,'text'::text AS record"
        rows = query(database, "SELECT record,status,error_code FROM thinkthen.decide(%s,'Ready?')", (source,))
        assert rows == [('text','failed','record.json')], rows
        error = failure(database, "SELECT * FROM thinkthen.decide(%s,'Ready?',raise_if_failed=>true)", '22000','record.json',(source,))
        assert '1 of 1 records failed' in error.diag.message_primary, error
    # Remote call refusals are the engine's established boundary, distinct from
    # record outcomes. A later non-authentication 4xx remains a failed row.
    for http in (400,401,402,403,429):
        with scenario('remote-refusal', case={'replies':[{'status':http,'body':{'error':'refused'}}]}, observer=(servers := [])) as (_, env), server_for(env, server) as database:
            query(database, "SET thinkthen.allowed_backends='systemone:*'")
            query(database, 'SET thinkthen.max_retries=0')
            failure(database, "SELECT * FROM thinkthen.decide($$SELECT 'text' AS record$$,'Ready?')", '58000','backend.status')
            assert command(servers[0], 'count') == '1'
    case = {'replies':[{'contains':'"bad"','status':400,'body':{}},{'probability':.95}]}
    with scenario('later-4xx', case=case, observer=(servers := [])) as (_, env), server_for(env, server) as database:
        query(database, "SET thinkthen.allowed_backends='systemone:*'")
        query(database, 'SET thinkthen.max_retries=0')
        query(database, 'SET thinkthen.batch=1')
        source = "SELECT record FROM(VALUES(1,'good'),(2,'bad'),(3,'good again')) input(n,record) ORDER BY n"
        rows = query(database, "SELECT position,record,status,error_code FROM thinkthen.decide(%s,'Ready?')", (source,))
        assert rows == [(1,'good','answered',None),(2,'bad','failed','backend.status'),(3,'good again','answered',None)], rows
        assert command(servers[0], 'count') == '3'
    # Literal form call: saved details keep each independent failed entry and the
    # same call with raise_if_failed must catch it, counting each record once.
    body = blocks(IDEAL / 'postgresql.md')[27][1]
    literal_call = body[body.index('FROM thinkthen.annotate'):].rstrip().removesuffix(';')
    for name in ('failed','refund-failed','topic-failed','urgency-failed','refund-unsure','topic-unsure'):
        with scenario(name) as (_, env), server_for(env, server) as database:
            query(database, "SET thinkthen.allowed_backends = 'systemone:*'")
            query(database, 'SET thinkthen.max_retries=0')
            for statement in statements(blocks(IDEAL / 'postgresql.md')[1][1]):
                query(database, statement)
            rows = query(database, 'SELECT status,detail ' + literal_call)
            assert len(rows) == 3 and all(row[0] == 'answered' for row in rows), rows
            for _, detail in rows:
                fields = ('refund','topic','urgency') if name == 'failed' else (name.split('-')[0],)
                state = 'failed' if name.endswith('failed') else 'unsure'
                assert all(detail[field]['status'] == state for field in fields), (name,detail)
                if state == 'failed':
                    assert all(detail[field]['error']['code'] in ('backend.status','backend.reply') for field in fields), detail
            raising = 'SELECT * ' + literal_call[:-1] + ',raise_if_failed=>true)'
            if name.endswith('failed'):
                error = failure(database, raising, '58000','backend.status' if name == 'failed' else 'backend.reply')
                assert '3 of 3 records failed' in error.diag.message_primary and 'position 1' in error.diag.message_primary, error
            else:
                assert len(query(database, raising)) == 3
            error = failure(database, "SELECT * FROM thinkthen.annotate($$SELECT NULL::text AS record$$,questions=>'{\"ready\":{\"function\":\"decide\",\"question\":\"Ready?\"}}',raise_if_failed=>true)", '22000','record.missing')
            assert '1 of 1 records failed' in error.diag.message_primary, error
    # This private scripted fixture verifies exact server ERROR/DETAIL formatting;
    # the public guide describes its shape without presenting fixture output.
    case = {'replies': [{'contains':'"later"','status':503,'body':{'error':'unavailable'}}, {'probability':.95}]}
    with scenario('printed-error', case=case) as (_, env), server_for(env, server) as database:
        query(database, "SET thinkthen.allowed_backends='systemone:*'")
        query(database, 'SET thinkthen.max_retries=0')
        query(database, 'SET thinkthen.batch=1')
        source = "SELECT record FROM(VALUES(1,'first'),(2,'later'),(3,'last')) input(n,record) ORDER BY n"
        error = failure(database, "SELECT * FROM thinkthen.decide(%s,'Ready?',raise_if_failed=>true)", '58000','backend.status',(source,))
        rendered = 'ERROR:  ' + error.diag.message_primary + '\nDETAIL:  ' + error.diag.message_detail
        expected = ('ERROR:  1 of 3 records failed; the first, at position 2, failed with '
                    'backend.status: the backend answered status 503: {"error":"unavailable"}\n'
                    'DETAIL:  backend.status')
        assert rendered == expected, rendered
    print('PASS PostgreSQL refusals, field types, exact IDs, retained records/spans and parent/child raises', flush=True)


def server_for(env, server):
    env['THINKTHEN_TEST_LOOPBACK_URL'] = env['THINKTHEN_BASE_URL']
    return server(env)


def limit_probe(options, server, query, failure):
    for function, extra in (('filter',''),('decide',',raise_if_failed=>true')):
        for limit in ('',' LIMIT 1'):
            with scenario('yes', observer=(servers := [])) as (_, env), server_for(env, server) as database:
                query(database, "SET thinkthen.allowed_backends = 'systemone:*'")
                query(database, 'SET thinkthen.max_retries=0')
                query(database, 'CREATE TEMP SEQUENCE consumed')
                source = "SELECT CASE WHEN nextval('pg_temp.consumed')=1 THEN 'text' ELSE NULL END AS record FROM generate_series(1,2) n ORDER BY n"
                statement = 'SELECT position FROM thinkthen.' + function + '(%s,\'Ready?\'' + extra + ')' + limit
                error = failure(database, statement, '22000','record.missing',(source,))
                assert '1 of 2 records failed' in error.diag.message_primary and 'position 2' in error.diag.message_primary, error
                assert query(database, 'SELECT last_value FROM pg_temp.consumed') == [(2,)]
                assert command(servers[0], 'count') == '1'
                print(f'LIMIT PostgreSQL {function} outside={bool(limit)} consumed=2 requests=1: failure reported', flush=True)
