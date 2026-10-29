"""Exercise the typed PostgreSQL extension in an isolated installed server."""
import argparse
from concurrent.futures import ThreadPoolExecutor
from contextlib import closing, contextmanager
import json
from pathlib import Path
import subprocess
import sys
import time
import tempfile

import psycopg2
from psycopg2 import sql

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'databases/duckdb/tests'))
from typed_api import fixture, contract_row
from migration_cases import command
from migration_cases import semantic, expected_row, strict_equal
from admission_cases import cases as admission_cases, diagnostic
sys.path.insert(0, str(ROOT / 'databases'))
from testing import child_environment


@contextmanager
def server(environment):
    with tempfile.TemporaryDirectory(prefix='thinkthen-pg-typed-') as scratch:
        path = Path(scratch)
        data = path / 'data'
        subprocess.run(['initdb', '--no-sync', '-D', str(data), '-A', 'trust', '-U', 'thinkthen', '--no-locale', '-E', 'UTF8'],
                       env=environment, check=True, stdout=subprocess.DEVNULL)
        subprocess.run(['pg_ctl', '-D', str(data), '-l', str(path / 'server.log'), '-o',
                        f"-k {path} -c listen_addresses='' -c fsync=off", '-w', 'start'],
                       env=environment, check=True, stdout=subprocess.DEVNULL)
        database = None
        try:
            database = psycopg2.connect(host=str(path), user='thinkthen', dbname='postgres')
            database.autocommit = True
            with database.cursor() as cursor:
                cursor.execute('CREATE SCHEMA thinkthen')
                cursor.execute('CREATE EXTENSION thinkthen SCHEMA thinkthen')
            yield database
        finally:
            if database is not None:
                database.close()
            subprocess.run(['pg_ctl', '-D', str(data), '-m', 'fast', '-w', 'stop'],
                           env=environment, check=True, stdout=subprocess.DEVNULL)


def query(database, statement, parameters=()):
    with database.cursor() as cursor:
        cursor.execute(statement, parameters)
        return cursor.fetchall() if cursor.description else []


def guide(database):
    query(database, 'CREATE DATABASE guide_example')
    try:
        with closing(psycopg2.connect(database.dsn, dbname='guide_example')) as example:
            with tempfile.TemporaryDirectory() as temporary:
                helper = ROOT / 'scripts/surface-example'
                subprocess.run([sys.executable, str(helper), 'prepare', 'postgresql', temporary], check=True)
                subprocess.run([sys.executable, str(helper), 'check', 'postgresql', 'psql', '-XAt',
                                '-v', 'ON_ERROR_STOP=1', '-d', example.dsn, '-f', 'example.sql'],
                               env=child_environment(), cwd=temporary, check=True)
    finally:
        query(database, 'DROP DATABASE guide_example')


def set_settings(database, settings):
    for key, value in settings.items():
        for name, item in (value.items() if key == 'retry' else [(key, value)]):
            if item is None:
                continue
            query(database, sql.SQL('SET {} = %s').format(sql.Identifier('thinkthen.' + name)), (str(item),))


def invoke(database, request):
    question = request['question']
    arguments = [sql.SQL('%s')]
    values = ['SELECT value AS record FROM json_array_elements($1::json) WITH ORDINALITY s(value,n) ORDER BY n']
    for key, value in question.items():
        if key == 'function':
            continue
        if key == 'threshold' and isinstance(value, dict):
            for bound, number in value.items():
                arguments.append(sql.SQL('{} => %s').format(sql.Identifier(bound)))
                values.append(number)
            continue
        key = 'toolbox' if key == 'questions' else key
        if key == 'rules':
            rules = []
            for rule in value:
                rules.append(sql.SQL('ROW(%s,%s,%s,%s,%s)::thinkthen.rule'))
                values.extend((rule['relation'], rule['source'], rule['target'], rule.get('reads'), rule.get('either', False)))
            arguments.append(sql.SQL('rules => ARRAY[{}]::thinkthen.rule[]').format(sql.SQL(',').join(rules)))
        else:
            arguments.append(sql.SQL('{} => %s' + ('::jsonb' if key == 'toolbox' else '')).format(sql.Identifier(key)))
            if key == 'toolbox':
                value = json.loads(json.dumps(value))
                for child in value.values():
                    for rule in child.get('rules', []):
                        rule['name'] = rule.pop('relation')
            values.append(json.dumps(value) if key == 'toolbox' else value)
    if 'name' in request:
        arguments.append(sql.SQL('name => %s'))
        values.append(request['name'])
    arguments.append(sql.SQL('params => %s::jsonb'))
    values.append(json.dumps([json.dumps(request.get('records', []))]))
    statement = sql.SQL('SELECT to_jsonb(t) FROM thinkthen.{}({}) t').format(
        sql.Identifier(question['function']), sql.SQL(',').join(arguments))
    rows = []
    for (row,) in query(database, statement, values):
        if question['function'] == 'filter':
            assert row['kept'] is (row['status'] == 'answered' and row['answer'] is True), row
        if row['source'].get('name') is None:
            row['source'].pop('name', None)
        if question['function'] == 'find':
            row['found'] = (None if row['found_text'] is None else
                {'text': row['found_text'], 'start': row['found_start'], 'end': row['found_end']})
        if question['function'] == 'relate' and row['answer'] is not None:
            for relation in row['answer']:
                assert type(relation['either']) is bool, relation
                if not relation['either']:
                    relation.pop('either')
        rows.append(contract_row(row, question['function']))
    return rows


def failure(database, statement, state=None, code=None, parameters=()):
    try:
        query(database, statement, parameters)
    except psycopg2.Error as error:
        if state is not None:
            assert error.pgcode == state, (error.pgcode, error.diag.message_primary)
        if code is not None:
            assert error.diag.message_detail == code, (error.diag.message_detail, error.diag.message_primary)
        return error
    raise AssertionError(('query unexpectedly succeeded', statement))


def admission():
    with server(child_environment()) as database:
        query(database, "SET thinkthen.allowed_backends = 'stub:*'")
        set_settings(database, {'backend': 'stub'})
        # Native integer GUC conversion precedes engine JSON admission.
        failure(database, "SET thinkthen.max_retries='secret-placeholder'", '22023')
        settings = query(database, 'SELECT name FROM pg_settings WHERE name LIKE %s ORDER BY name', ('thinkthen.%',))
        assert [row[0] for row in settings] == [
            'thinkthen.allowed_backends', 'thinkthen.attempt_timeout_ms', 'thinkthen.backend',
            'thinkthen.batch', 'thinkthen.initial_backoff_ms', 'thinkthen.max_retries', 'thinkthen.model'], settings
        for case, request, expected in admission_cases(ROOT, 'postgresql'):
            try:
                invoke(database, request)
            except psycopg2.Error as error:
                assert error.pgcode == '22023', (case['name'], error.pgcode)
                assert 'secret-placeholder' not in str(error), case['name']
                diagnostic(case, error.diag.message_detail, error.diag.message_primary, expected)
            else:
                raise AssertionError(('admission succeeded', case['name']))


def native():
    # Feature: schema-qualified typed SQL keeps native diagnostics and source rights.
    with server(child_environment()) as database:
        guide(database)
        query(database, "SET thinkthen.allowed_backends = 'stub:*'")
        set_settings(database, {'backend': 'stub'})
        failure(database, "SELECT * FROM thinkthen.decide('SELECT ''x'' AS record',NULL)", '22023', 'usage.question')
        failure(database, "SELECT * FROM thinkthen.choose('SELECT ''x'' AS record','Which?',ARRAY['one',NULL,'two'])", '22023', 'usage.question')
        assert query(database, "SELECT status,error_code,kept FROM thinkthen.filter('SELECT NULL::text AS record','Ready?')") == [('failed', 'record.missing', False)]
        query(database, 'CREATE SCHEMA hostile')
        query(database, "SET search_path = hostile,pg_catalog")
        rows = query(database, """SELECT position,id,status,answer,error_code,
            pg_typeof(status)::text,pg_typeof(answer)::text,pg_typeof(detail)::text
            FROM thinkthen.decide('SELECT 42 AS id,NULL::text AS record','{literal?', id_column => 'id')""")
        assert rows == [(1, 42, 'failed', None, 'record.missing', 'thinkthen.status', 'boolean', 'jsonb')], rows
        assert query(database, "SELECT nspname,extrelocatable FROM pg_extension e JOIN pg_namespace n ON n.oid=e.extnamespace WHERE extname='thinkthen'") == [('thinkthen', False)]
        assert query(database, "SELECT to_regprocedure('public.thinkthen_decide(text,text,json,json)')") == [(None,)]
        registry = json.loads((ROOT / 'schemas/contract.json').read_text())
        for native_type, key in [('status', 'statuses'), ('error_code', 'codes')]:
            actual = query(database, "SELECT enumlabel FROM pg_enum e JOIN pg_type t ON t.oid=e.enumtypid JOIN pg_namespace n ON n.oid=t.typnamespace WHERE n.nspname='thinkthen' AND t.typname=%s ORDER BY enumsortorder", (native_type,))
            assert [item[0] for item in actual] == [item['name'] for item in registry[key]], actual
        # Scalar bindings preserve numeric precision and require explicit casts for nullable values.
        source = "SELECT ($1::numeric = 123456789012345678901234567890.123456789 AND $2::boolean AND $3::text IS NULL) AS id, 'evidence'::text AS record"
        assert query(database, "SELECT id FROM thinkthen.decide(%s,%s,params => %s::jsonb,id_column => 'id')",
                     (source, 'Ready?', '[123456789012345678901234567890.123456789,true,null]')) == [(True,)]
        failure(database, 'SELECT * FROM thinkthen.decide(%s,%s,params => %s::jsonb)', '22023',
                parameters=('SELECT $1::text AS record', 'Ready?', '[[1,2]]'))
        failure(database, "SELECT * FROM thinkthen.choose('SELECT ''x'' AS record','Which?',choices => ARRAY['one'])", '22023', 'usage.question')
        failure(database, "SELECT * FROM thinkthen.decide('SELECT ''x'' AS record','Ready?',low => 0.2)", '22023', 'usage.threshold')
        failure(database, "SELECT * FROM thinkthen.rank('SELECT ''x'' AS record','Urgent?',ARRAY['low','high'],top=>-1)", '22023', 'usage.question')
        query(database, 'CREATE TABLE hostile.docs(record text)')
        query(database, "INSERT INTO hostile.docs VALUES('alpha')")
        for source in ["DELETE FROM hostile.docs RETURNING record",
                       "WITH d AS (DELETE FROM hostile.docs RETURNING record) SELECT record FROM d",
                       "SELECT record FROM hostile.docs; SELECT record FROM hostile.docs"]:
            failure(database, 'SELECT * FROM thinkthen.decide(%s,%s)', parameters=(source, 'Ready?'))
        assert query(database, 'SELECT count(*) FROM hostile.docs') == [(1,)]
        assert query(database, 'SELECT count(*) FROM pg_cursors') == [(0,)]
        query(database, 'CREATE ROLE caller LOGIN')
        query(database, 'GRANT USAGE ON SCHEMA thinkthen TO caller')
        query(database, 'SET ROLE caller')
        failure(database, "SELECT * FROM thinkthen.decide('SELECT ''x'' AS record','Ready?')", '42501')
        failure(database, "SET thinkthen.allowed_backends = 'stub:*'", '42501')
        query(database, 'RESET ROLE')
        query(database, 'GRANT EXECUTE ON ALL FUNCTIONS IN SCHEMA thinkthen TO caller')
        with closing(psycopg2.connect(database.dsn, user='caller')) as unprivileged:
            unprivileged.autocommit = True
            query(unprivileged, "SET thinkthen.allowed_backends = 'stub:*'")
            query(unprivileged, "SET thinkthen.backend = 'stub'")
            failure(unprivileged, "SELECT * FROM thinkthen.decide('SELECT ''x'' AS record','Ready?')", '42501', 'usage.sql-selection')
        query(database, 'SET ROLE caller')
        assert len(query(database, "SELECT * FROM thinkthen.decide('SELECT ''x'' AS record','Ready?')")) == 1
        failure(database, "SELECT * FROM thinkthen.decide('SELECT record FROM hostile.docs','Ready?')", '42501')
        query(database, 'RESET ROLE')
        # Tightening policy invalidates an already usable cached client.
        query(database, "SET thinkthen.allowed_backends = ''")
        failure(database, "SELECT * FROM thinkthen.decide('SELECT ''x'' AS record','Ready?')", '42501', 'usage.sql-selection')
        query(database, "SET thinkthen.allowed_backends = 'stub:*'")
        query(database, 'BEGIN')
        query(database, "SET LOCAL thinkthen.backend = 'systemone'")
        query(database, 'ROLLBACK')
        assert len(query(database, "SELECT * FROM thinkthen.decide('SELECT ''x'' AS record','Ready?')")) == 1
        # PostgreSQL accepts unknown dotted GUC placeholders before extension recognition.
        # A key/address placeholder must never affect client selection or reach output.
        for key in ('api_key', 'url', 'key_source', 'mode', 'adapter'):
            try:
                query(database, sql.SQL('SET {} = %s').format(sql.Identifier('thinkthen.' + key)), ('do-not-store',))
            except psycopg2.Error:
                continue
            rows = query(database, "SELECT (source).backend FROM thinkthen.decide('SELECT ''x'' AS record','Ready?')")
            assert rows == [('stub',)], (key, rows)
        from sources import verify
        verify(database, query, failure)
        # Missing process credentials remain an engine usage error.
        query(database, "SET thinkthen.allowed_backends = 'systemone:*'")
        set_settings(database, {'backend': 'systemone'})
        error = failure(database, "SELECT * FROM thinkthen.decide('SELECT ''x'' AS record','Ready?')", code='usage.key-missing')
        assert error.pgcode == '22023', error.pgcode
        assert query(database, 'SELECT 1') == [(1,)]


def abi_mismatch(options):
    # Feature: an incompatible module is rejected before any client can be opened.
    with server(child_environment()) as database:
        error = failure(database, 'LOAD %s', parameters=(str(options.abi_mismatch_extension),), code='usage.abi')
        assert error.pgcode == '22023', error.pgcode


def error_families(options):
    # Contract consumer: the adapter source maps every registered engine code.
    with server(child_environment()) as database:
        query(database, "CREATE FUNCTION pg_temp.family_probe(integer) RETURNS void "
              "AS %s,'thinkthen_family_probe' LANGUAGE C STRICT",
              (str(options.abi_mismatch_extension.parent / 'family-probe.so'),))
        registry = json.loads((ROOT / 'schemas/contract.json').read_text())
        for code in registry['codes']:
            expected = '22023' if code['family'] == 'usage' else '58000'
            failure(database, 'SELECT pg_temp.family_probe(%s)', expected, parameters=(code['value'],))


def conformance(options):
    # Feature: PostgreSQL composites, arrays and jsonb preserve both wire contracts.
    covered = set()
    with server(child_environment()) as database:
        query(database, "SET thinkthen.allowed_backends = 'stub:*'")
        set_settings(database, {'backend': 'stub'})
        for case in json.loads((ROOT / 'construction/stub.json').read_text()):
            assert strict_equal(invoke(database, case['request']), case['expected']), case['request']
        for case in json.loads((ROOT / 'construction/options.json').read_text()):
            if 'expected_error' in case:
                try:
                    invoke(database, case['request'])
                except psycopg2.Error as error:
                    reason = case.get('not_applicable', {}).get('postgres')
                    if reason is not None:
                        assert isinstance(reason, str) and reason.strip(), case['name']
                        print(f"postgres {case['name']}: canonical diagnostic not applicable: {reason}")
                        assert error.pgcode == '42883', (case['name'], error)
                    else:
                        assert error.pgcode == '22023', (case['name'], error)
                        assert error.diag.message_detail == case['expected_error'], (case['name'], error)
                else:
                    raise AssertionError(('option admission succeeded', case['name']))
            else:
                request = case.get('equivalent_request', case['request'])
                actual = invoke(database, request)
                assert strict_equal(actual, case['expected']), (case['name'], actual, case['expected'])
    directory = ROOT / 'conformance/typed'
    manifest = json.loads((directory / 'manifest.json').read_text())
    for filename in manifest['cases']:
        path = directory / filename
        case = json.loads(path.read_text())
        question = case['request']['question']
        if case.get('cancelled'):
            assert case.get('not_applicable', {}).get('postgres'), filename
            print(f'postgres {filename}: not applicable: ' + case['not_applicable']['postgres'])
            continue
        for backend in manifest['backends']:
            settings = {**manifest['client'], **case.get('client', {}), 'backend': backend}
            with fixture(options, path) as (loopback, environment), server(environment) as database:
                query(database, 'SET thinkthen.allowed_backends = %s', (backend + ':*',))
                if isinstance(case['expected'], dict):
                    reason = case.get('not_applicable', {}).get('postgres')
                    if reason is not None:
                        assert isinstance(reason, str) and reason.strip(), filename
                        zero_byte = r'\u0000' in json.dumps(case['request'])
                        retry_bound = (case['expected']['error'] == 'usage.settings' and
                            'max_retries' in case.get('client', {}).get('retry', {}))
                        assert zero_byte or retry_bound, filename
                        try:
                            set_settings(database, settings)
                            invoke(database, case['request'])
                        except ValueError as error:
                            assert zero_byte, error
                            assert 'NUL' in str(error) or '0x00' in str(error), error
                        except psycopg2.Error as error:
                            assert error.pgcode in (('22021', '22P05') if zero_byte else ('22023',)), error
                        else:
                            raise AssertionError(('host accepted a zero byte', filename))
                        assert int(command(loopback, 'count')) == 0, filename
                        print(f'postgres {filename}: canonical diagnostic not applicable: {reason}')
                        continue
                    try:
                        set_settings(database, settings)
                        invoke(database, case['request'])
                    except psycopg2.Error as error:
                        assert error.diag.message_detail == case['expected']['error'], error
                    else:
                        raise AssertionError(('admission succeeded', path))
                    assert int(command(loopback, 'count')) == case['requests'], path
                    continue
                set_settings(database, settings)
                actual = invoke(database, case['request'])
                for row in actual:
                    source = row['source']
                    assert source['backend'] == backend and source['model'] == settings['model'], source
                    assert len(source['question_identity']) == 64, source
                    assert source.get('name') == case['request'].get('name'), source
                assert len(actual) == len(case['expected']), (path, backend, len(actual), len(case['expected']))
                assert strict_equal([semantic(row, expected) for row, expected in zip(actual, case['expected'])], [expected_row(row) for row in case['expected']]), (path, backend, actual, case['expected'])
                requests = int(command(loopback, 'count'))
                assert requests == case['requests'], (path, backend, requests, case['requests'])
                missing = query(database, "SELECT position,status,answer,error_code FROM thinkthen.decide('SELECT NULL::text AS record','Ready?')")
                assert missing == [(1, 'failed', None, 'record.missing')], missing
                assert int(command(loopback, 'count')) == requests, (path, backend)
            covered.add((question['function'], backend))
    functions = {'decide', 'choose', 'tag', 'score', 'filter', 'rank', 'find', 'annotate', 'recognize', 'relate'}
    assert covered == {(function, backend) for function in functions for backend in ('systemone', 'openai')}, covered


def cancellation(options):
    # Feature: every SQL function cancels a paid held request and releases its cursor.
    for case in json.loads((ROOT / 'construction/stub.json').read_text()):
        with fixture(options, held=True) as (loopback, environment), server(environment) as database:
            query(database, "SET thinkthen.allowed_backends = 'systemone:*'")
            set_settings(database, {'retry': {'attempt_timeout_ms': 120000}})
            with ThreadPoolExecutor(max_workers=1) as workers:
                future = workers.submit(invoke, database, case['request'])
                try:
                    assert command(loopback, 'arrived') == 'arrived'
                    paid = int(command(loopback, 'count'))
                    assert paid > 0
                    started = time.monotonic()
                    database.cancel()
                    assert command(loopback, 'closed', timeout=2) == 'closed'
                    assert time.monotonic() - started < 2, case['request']['question']['function']
                    try:
                        future.result(timeout=2)
                    except psycopg2.Error as error:
                        assert error.pgcode == '57014', error
                    else:
                        raise AssertionError('cancel returned a normal result')
                finally:
                    if not future.done():
                        database.cancel()
            assert query(database, 'SELECT 1') == [(1,)]
            assert query(database, 'SELECT count(*) FROM pg_cursors') == [(0,)]
            assert int(command(loopback, 'count')) == paid
            query(database, "SET thinkthen.allowed_backends = 'stub:*'")
            set_settings(database, {'backend': 'stub'})
            assert len(query(database, "SELECT * FROM thinkthen.decide('SELECT ''x'' AS record','Ready?')")) == 1


def statement_timeout(options):
    # Feature: PostgreSQL's statement timeout cancels an active engine request.
    with fixture(options, held=True) as (loopback, environment), server(environment) as database:
        query(database, "SET thinkthen.allowed_backends='systemone:*'")
        query(database, "SET statement_timeout='1s'")
        failure(database, "SELECT * FROM thinkthen.decide('SELECT ''x'' AS record','Ready?')", '57014')
        assert command(loopback, 'closed') == 'closed'
        query(database, 'SET statement_timeout=0')
        assert query(database, 'SELECT 1') == [(1,)]


def adapter_contracts():
    # Feature: planner contracts keep remote calls volatile and serial.
    with server(child_environment()) as database:
        assert query(database, "SELECT DISTINCT provolatile, proparallel FROM pg_proc WHERE pronamespace='thinkthen'::regnamespace AND prolang=(SELECT oid FROM pg_language WHERE lanname='c')") == [('v','u')]
        assert query(database, "SHOW thinkthen.batch") == [('0',)]
        query(database, 'SET thinkthen.batch=7')
        assert query(database, "SHOW thinkthen.batch") == [('7',)]
        query(database, 'RESET thinkthen.batch')
        assert query(database, "SHOW thinkthen.batch") == [('0',)]
        for setting, value in [('batch', -1), ('batch', 0), ('max_retries', -1), ('max_retries', 4), ('initial_backoff_ms', -1), ('attempt_timeout_ms', 0)]:
            failure(database, 'SET thinkthen.' + setting + '=' + str(value), '22023')
        for setting, value in [('batch', 7), ('max_retries', 0), ('initial_backoff_ms', 0), ('attempt_timeout_ms', 1)]:
            query(database, 'SET thinkthen.' + setting + '=' + str(value))
            query(database, 'RESET thinkthen.' + setting)
        shared = subprocess.check_output(['pg_config', '--sharedir'], text=True).strip()
        direct = subprocess.run(['psql', '-X', '-v', 'ON_ERROR_STOP=1', '-d', database.dsn,
                                 '-f', str(Path(shared)/'extension/thinkthen--0.2.0.sql')],
                                text=True, capture_output=True, check=True, env=child_environment())
        assert 'CREATE EXTENSION thinkthen' in direct.stdout and 'ERROR' not in direct.stderr, direct
        query(database, "SET thinkthen.backend='stub'")
        query(database, "SET thinkthen.allowed_backends='stub:*'")
        query(database, "CREATE TEMP TABLE memory_sample(bytes bigint, used bigint, contexts bigint)")
        query(database, """CREATE FUNCTION pg_temp.sample(i integer) RETURNS text LANGUAGE plpgsql VOLATILE AS $$
            BEGIN
                IF i=20000 THEN
                    INSERT INTO memory_sample SELECT coalesce(sum(total_bytes),0), coalesce(sum(used_bytes),0), count(*)
                    FROM pg_backend_memory_contexts WHERE name IN ('ThinkThen invocation','ThinkThen selected id');
                END IF;
                RETURN 'x';
            END $$""")
        assert query(database, "SELECT count(*) FROM thinkthen.rank('SELECT i::text AS id, pg_temp.sample(i) AS record FROM generate_series(1,20000) i','Order?',ARRAY['low','high'],id_column=>'id')") == [(20000,)]
        sample = query(database, "SELECT bytes,used,contexts FROM memory_sample")
        assert len(sample)==1 and sample[0][0] < 16*1024*1024, sample
        print('PostgreSQL id memory (allocated bytes, used bytes, contexts):', sample)


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('loopback', type=Path)
    parser.add_argument('--group', choices=['all', 'native', 'ownership', 'conformance', 'admission', 'adapter'], default='all')
    parser.add_argument('--abi-mismatch-extension', type=Path, required=True)
    parser.add_argument('--systemone-url-env', required=True)
    parser.add_argument('--openai-url-env', required=True)
    options = parser.parse_args()
    if options.group == 'adapter':
        adapter_contracts()
        statement_timeout(options)
        from read_ahead import verify as read_ahead
        read_ahead(options, server, query, set_settings, fixture, child_environment)
        print('PostgreSQL adapter contracts pass')
        sys.exit(0)
    admission()
    if options.group == 'admission':
        sys.exit(0)
    if options.group != 'conformance':
        abi_mismatch(options)
        error_families(options)
        native()
        adapter_contracts()
        from request_counts import verify as request_counts
        request_counts(options, server, query, set_settings, child_environment)
        from read_ahead import verify as read_ahead
        read_ahead(options, server, query, set_settings, fixture, child_environment)
    if options.group in ('all', 'conformance'):
        conformance(options)
    if options.group in ('all', 'native'):
        cancellation(options)
        statement_timeout(options)
    if options.group != 'conformance':
        subprocess.run(['/work/bundle/run-live', '--stub'], check=True, env=child_environment())
    print(f'PostgreSQL {options.group} checks pass')
