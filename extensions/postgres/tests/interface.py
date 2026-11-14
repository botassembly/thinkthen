"""Check the settled PostgreSQL interface: row shape, filter, ids, settings, help and upgrade."""
from contextlib import closing
import json
import re
from pathlib import Path
import shutil
import subprocess
import tempfile
import traceback

import psycopg2

HERE = Path(__file__).resolve().parent
OLD = 'thinkthen\\_%'
FUNCTIONS = {'decide': 'boolean', 'choose': 'text', 'tag': 'text[]', 'score': 'double precision',
             'filter': 'boolean', 'rank': 'bigint', 'find': 'thinkthen.span',
             'recognize': 'thinkthen.recognized_name[]', 'relate': 'thinkthen.relation[]', 'annotate': 'jsonb'}
SHARED = [('position', 'bigint'), ('id', 'jsonb'), ('record', 'text'), ('answer', None), ('probability', 'double precision'),
          ('status', 'thinkthen.status'), ('error_code', 'thinkthen.error_code'), ('error_message', 'text'),
          ('error_http_status', 'integer'), ('source', 'thinkthen.source'), ('detail', 'jsonb')]
FIELDS = {'rule': ['source', 'target', 'relation', 'reads', 'either'],
          'span': ['text', 'start_offset', 'end_offset'],
          'recognized_name': ['kind', 'text', 'start_offset', 'end_offset', 'probability'],
          'endpoint': ['kind', 'text', 'start_offset', 'end_offset'],
          'relation': ['source', 'target', 'relation', 'probability', 'either']}


def shape(database, query):
    # Feature: all ten functions share one row layout with the answer right after id.
    rows = query(database, """SELECT p.proname, array_agg(a.name ORDER BY a.n), array_agg(format_type(a.type, NULL) ORDER BY a.n)
        FROM pg_proc p, unnest(p.proargnames, p.proargmodes::text[], p.proallargtypes) WITH ORDINALITY a(name, mode, type, n)
        WHERE p.pronamespace = 'thinkthen'::regnamespace AND a.mode = 't' GROUP BY p.proname""")
    # format_type omits the schema when thinkthen is on the search path.
    bare = lambda kind: kind.removeprefix('thinkthen.')
    assert {name: [(column, bare(kind)) for column, kind in zip(columns, types)] for name, columns, types in rows} == {
        function: [(column, bare(answer if kind is None else kind)) for column, kind in SHARED]
        for function, answer in FUNCTIONS.items()}, rows
    # Feature: rules name their relation as rows do, span fields avoid the reserved word end,
    # and no type shadows the built-in name.
    for kind, fields in FIELDS.items():
        actual = query(database, 'SELECT attname FROM pg_attribute WHERE attrelid = %s::regclass AND attnum > 0 ORDER BY attnum',
                       ('thinkthen.' + kind,))
        assert [row[0] for row in actual] == fields, (kind, actual)
    assert query(database, "SELECT to_regtype('thinkthen.name')") == [(None,)]


def help_text(database, query):
    # Feature: \df+, \dT+ and pg_settings describe every function, type and setting.
    assert query(database, """SELECT count(*), count(obj_description(oid, 'pg_proc')) FROM pg_proc
        WHERE pronamespace = 'thinkthen'::regnamespace""") == [(10, 10)]
    assert query(database, """SELECT typname FROM pg_type WHERE typnamespace = 'thinkthen'::regnamespace
        AND typtype IN ('c', 'e') AND obj_description(oid, 'pg_type') IS NULL""") == []
    settings = query(database, 'SELECT name, short_desc FROM pg_settings WHERE name LIKE %s', ('thinkthen.%',))
    assert len(settings) == 7 and all(description and not description.startswith('ThinkThen ')
                                      for _, description in settings), settings


def kept_only(database, query, failure):
    # Filter failure is always a statement error; decide is the complete-row route.
    source = "SELECT * FROM (VALUES (1, 'Please refund me.'), (2, NULL), (3, 'Thanks.'), (4, 'Refund the fee.')) s(id, record)"
    error = failure(database, "SELECT * FROM thinkthen.filter(%s, 'Refund?')", '22000', 'record.missing', (source,))
    assert error.diag.message_primary.startswith('1 of 4 records failed; the first, at position 2, failed with record.missing: '), error
    assert not error.diag.message_hint, error.diag.message_hint
    rows = query(database, "SELECT position, id, answer, status, error_code FROM thinkthen.decide(%s, 'Refund?')", (source,))
    assert len(rows) == 4 and rows[1] == (2, 2, None, 'failed', 'record.missing'), rows
    failure(database, "SELECT * FROM thinkthen.filter('SELECT ''x''::text AS record', 'Refund?', failures => 'row')", '42883')
    error = failure(database, "SELECT * FROM thinkthen.filter('SELECT ''x''::text AS record', 'Refund?', low => .2, high => .8)", '22023', 'usage.threshold')
    assert error.diag.message_hint == 'Use thinkthen.decide for not-sure results', error


def ids(database, query, failure):
    # Feature: id_column NULL, the default, uses a column named id when there is one.
    assert query(database, "SELECT id FROM thinkthen.decide('SELECT 7 AS id, ''x''::text AS record', 'Ready?')") == [(7,)]
    assert query(database, "SELECT id FROM thinkthen.decide('SELECT ''x''::text AS record', 'Ready?')") == [(None,)]
    assert query(database, "SELECT id FROM thinkthen.decide('SELECT 7 AS id, ''x''::text AS record', 'Ready?', id_column => NULL)") == [(7,)]
    # Feature: a column the caller names must exist, id included.
    assert query(database, "SELECT id FROM thinkthen.decide('SELECT 7 AS key, ''x''::text AS record', 'Ready?', id_column => 'key')") == [(7,)]
    failure(database, "SELECT id FROM thinkthen.decide('SELECT ''x''::text AS record', 'Ready?', id_column => 'id')", '42703')
    failure(database, "SELECT id FROM thinkthen.decide('SELECT ''x''::text AS record', 'Ready?', id_column => 'key')", '42703')


def numeric_ids(database, query):
    # Feature: nonfinite PostgreSQL numeric ids fail only their rows; SQL NULL is absent.
    for kind in ('real', 'double precision', 'numeric'):
        query(database, f'CREATE TEMP TABLE numeric_ids(position integer, id {kind}, record text)')
        try:
            query(database, """INSERT INTO numeric_ids VALUES
                (1, 7, 'good before'), (2, 'NaN', 'bad NaN'), (3, 'Infinity', 'bad infinity'),
                (4, '-Infinity', 'bad negative infinity'), (5, NULL, 'absent id'), (6, 9, 'good after')""")
            # Read real stored PostgreSQL values before asking the extension to convert them.
            assert query(database, 'SELECT id::text FROM numeric_ids ORDER BY position') == [
                ('7',), ('NaN',), ('Infinity',), ('-Infinity',), (None,), ('9',)]
            rows = query(database, """SELECT position, id, id IS NULL, answer, status, error_code, error_message
                FROM thinkthen.decide('SELECT id, record FROM numeric_ids ORDER BY position', 'Ready?')""")
            assert len(rows) == 6, (kind, rows)
            assert [(row[0], row[1], row[2], row[4], row[5]) for row in rows] == [
                (1, 7, False, 'answered', None),
                (2, None, True, 'failed', 'record.json'),
                (3, None, True, 'failed', 'record.json'),
                (4, None, True, 'failed', 'record.json'),
                (5, None, True, 'answered', None),
                (6, 9, False, 'answered', None)], (kind, rows)
            assert all(row[3] is not None for row in rows if row[4] == 'answered'), (kind, rows)
            assert all(row[3] is None and row[6] ==
                       'the record result cannot be represented by PostgreSQL text or jsonb'
                       for row in rows if row[4] == 'failed'), (kind, rows)
        finally:
            query(database, 'DROP TABLE numeric_ids')
    # Other selected types keep their JSON meaning, including strings with special-number names.
    for literal, expected in [("'NaN'::text", 'NaN'), ('true', True),
                              ('9223372036854775807::bigint', 9223372036854775807),
                              ("'{\"key\":[1,false]}'::json", {'key': [1, False]}),
                              ("'[1,2]'::jsonb", [1, 2]),
                              ("'2026-01-02'::date", '2026-01-02')]:
        source = f"SELECT {literal} AS id, 'x'::text AS record"
        assert query(database, "SELECT id, status FROM thinkthen.decide(%s, 'Ready?')", (source,)) == [(expected, 'answered')]
    # An absent selected id leaves the record's own id intact.
    source = '''SELECT NULL::integer AS id, '{"id":42,"body":"x"}'::json AS record'''
    assert query(database, "SELECT id FROM thinkthen.decide(%s, 'Ready?')", (source,)) == [(42,)]


def answer_options(database, query, failure):
    source = "SELECT 'A customer asks for a refund.'::text AS record"
    original = query(database, "SELECT (source).question_identity FROM thinkthen.decide(%s, 'Refund?')", (source,))
    changed = query(database, "SELECT (source).question_identity FROM thinkthen.decide(%s, 'Refund?', yes_means => 'asks for money back', no_means => 'does not ask')", (source,))
    assert len(original) == len(changed) == 1 and original != changed, (original, changed)
    for function in ('decide', 'filter'):
        rows = query(database, f"SELECT source FROM thinkthen.{function}(%s, 'Refund?', yes_means => 'asks for money back', no_means => 'does not ask')", (source,))
        assert len(rows) <= 1, rows
    for function, parameter in [('choose', 'choices'), ('tag', 'labels'), ('score', 'levels'), ('rank', 'levels')]:
        statement = f"SELECT (source).question_identity FROM thinkthen.{function}(%s, 'Classify?', {parameter} => ARRAY['low','high'], descriptions => %s::jsonb)"
        original = query(database, statement, (source, '{}'))
        described = query(database, statement, (source, '{"low":"routine", "high":"urgent"}'))
        assert len(original) == len(described) == 1 and original != described, (original, described)
        for invalid in ('{"other":"unused"}', '{"low":null}', '{"low":7}', '[]'):
            mapping = json.loads(invalid)
            options = [{'name': label, 'description': mapping[label]} if isinstance(mapping, dict) and label in mapping else label
                       for label in ('low', 'high')]
            canonical = {'function': function, 'question': 'Classify?', parameter: options}
            if not isinstance(mapping, dict) or set(mapping) - {'low', 'high'}:
                canonical['descriptions'] = mapping
            expected = failure(database, 'SELECT pg_temp.question_probe(%s)', parameters=(json.dumps(canonical),))
            actual = failure(database, statement, parameters=(source, invalid))
            diagnostic = lambda error: (error.pgcode, error.diag.message_detail, error.diag.message_primary)
            assert diagnostic(actual) == diagnostic(expected), (function, invalid, diagnostic(actual), diagnostic(expected))
            print('PostgreSQL description diagnostic:', function, invalid, diagnostic(actual))
        # Duplicate names still reach the engine's domain check, with no invented unmatched key.
        canonical = {'function': function, 'question': 'Classify?',
                     parameter: [{'name': 'low', 'description': 'routine'}] * 2}
        expected = failure(database, 'SELECT pg_temp.question_probe(%s)', parameters=(json.dumps(canonical),))
        duplicate = statement.replace("ARRAY['low','high']", "ARRAY['low','low']")
        actual = failure(database, duplicate, parameters=(source, '{"low":"routine"}'))
        assert diagnostic(actual) == diagnostic(expected), (function, diagnostic(actual), diagnostic(expected))
    failure(database, "SELECT * FROM thinkthen.filter(%s, 'Refund?', failures => 'all')", '42883', parameters=(source,))



def settings(database, query, failure):
    # Feature: time settings take units, and a malformed backend policy fails when it is set.
    query(database, "SET thinkthen.attempt_timeout = '2min'")
    assert query(database, 'SHOW thinkthen.attempt_timeout') == [('2min',)]
    assert query(database, "SELECT setting, unit FROM pg_settings WHERE name = 'thinkthen.attempt_timeout'") == [('120000', 'ms')]
    query(database, 'RESET thinkthen.attempt_timeout')
    error = failure(database, "SET thinkthen.allowed_backends = 'stub'", '22023', 'usage.sql-policy')
    assert error.diag.message_hint, error
    assert query(database, 'SHOW thinkthen.allowed_backends') == [('stub:*',)]


def dump_restore(database, example, query, directory):
    # An upgraded extension still names public, which conflicts with the fixed
    # 0.2.0 schema on restore. Recreate it without CASCADE before taking the dump.
    metadata = """SELECT e.extversion, n.nspname, e.extrelocatable FROM pg_extension e
        JOIN pg_namespace n ON n.oid = e.extnamespace WHERE e.extname = 'thinkthen'"""
    assert query(example, metadata) == [('0.2.0', 'public', False)]
    dump = directory / 'upgraded.sql'

    def save():
        with dump.open('w') as output:
            subprocess.run(['pg_dump', '--dbname', example.dsn], stdout=output, check=True)

    save()
    query(database, 'CREATE DATABASE restored_example')
    try:
        with closing(psycopg2.connect(database.dsn, dbname='restored_example')) as restored:
            restored.autocommit = True
            command = ['psql', '-X', '-v', 'ON_ERROR_STOP=1', '--dbname', restored.dsn, '--file', str(dump)]
            failed = subprocess.run(command, capture_output=True, text=True)
            assert failed.returncode == 3, (failed.returncode, failed.stderr)
            assert 'extension "thinkthen" must be installed in schema "thinkthen"' in failed.stderr, failed.stderr
            print('Upgraded dump restore fails:', failed.stderr.strip())
            query(example, 'DROP EXTENSION thinkthen')
            query(example, "CREATE EXTENSION thinkthen VERSION '0.2.0'")
            expected_metadata = [('0.2.0', 'thinkthen', False)]
            assert query(example, metadata) == expected_metadata
            source_rows = [(7, 'ready'), (9, None)]
            assert query(example, 'TABLE public.upgrade_records ORDER BY id') == source_rows
            save()
            subprocess.run(command, check=True, stdout=subprocess.DEVNULL)
            assert query(restored, metadata) == expected_metadata
            assert query(restored, 'TABLE public.upgrade_records ORDER BY id') == source_rows
            answers = []
            for connection in (example, restored):
                query(connection, "SET thinkthen.allowed_backends = 'stub:*'")
                query(connection, "SET thinkthen.backend = 'stub'")
                answers.append(query(connection, """SELECT position, id, answer, status, error_code
                    FROM thinkthen.decide('SELECT id, record FROM public.upgrade_records ORDER BY id', 'Ready?')"""))
            assert len(answers[0]) == 2 and answers[0] == answers[1], answers
            assert answers[1][0][0:2] == (1, 7) and answers[1][0][3:] == ('answered', None), answers
            assert answers[1][1] == (2, 9, None, 'failed', 'record.missing'), answers
            print('Recreated extension dump restore:', expected_metadata, source_rows, answers[1])
    finally:
        query(database, 'DROP DATABASE restored_example')


def upgrade(database, query):
    # Feature: ALTER EXTENSION UPDATE replaces 0.1.0's functions with 0.2.0's.
    shared = Path(subprocess.check_output(['pg_config', '--sharedir'], text=True).strip()) / 'extension'
    query(database, 'CREATE DATABASE upgrade_example')
    try:
        with tempfile.TemporaryDirectory() as directory, \
                closing(psycopg2.connect(database.dsn, dbname='upgrade_example')) as example:
            folder = Path(directory) / 'extension'
            folder.mkdir()
            for name in ('thinkthen.control', 'thinkthen--0.2.0.sql', 'thinkthen--0.1.0--0.2.0.sql'):
                shutil.copy(shared / name, folder / name)
            shutil.copy(HERE / 'thinkthen--0.1.0.sql', folder / 'thinkthen--0.1.0.sql')
            # 0.1.0 had no fixed schema, so CREATE EXTENSION put it in public.
            (folder / 'thinkthen--0.1.0.control').write_text("schema = 'public'\n")
            example.autocommit = True
            query(example, 'SET extension_control_path = %s', (directory + ':' + str(folder),))
            query(example, "CREATE EXTENSION thinkthen VERSION '0.1.0'")
            query(example, 'CREATE TABLE public.upgrade_records(id integer PRIMARY KEY, record text)')
            query(example, "INSERT INTO public.upgrade_records VALUES (7, 'ready'), (9, NULL)")
            # 0.1.0 had 33 thinkthen_ functions plus its event trigger function.
            assert query(example, 'SELECT count(*) FROM pg_proc WHERE proname LIKE %s', (OLD,)) == [(34,)]
            query(example, "ALTER EXTENSION thinkthen UPDATE TO '0.2.0'")
            assert query(example, "SELECT extversion FROM pg_extension WHERE extname = 'thinkthen'") == [('0.2.0',)]
            assert query(example, 'SELECT count(*) FROM pg_proc WHERE proname LIKE %s', (OLD,)) == [(0,)]
            assert query(example, "SELECT count(*) FROM pg_event_trigger") == [(0,)]
            assert query(example, """SELECT count(*) FROM pg_depend d JOIN pg_proc p ON p.oid = d.objid
                JOIN pg_extension e ON e.oid = d.refobjid WHERE e.extname = 'thinkthen' AND d.deptype = 'e'""") == [(10,)]
            query(example, "SET thinkthen.allowed_backends = 'stub:*'")
            query(example, "SET thinkthen.backend = 'stub'")
            assert len(query(example, "SELECT answer FROM thinkthen.decide('SELECT ''x''::text AS record', 'Ready?')")) == 1
            dump_restore(database, example, query, Path(directory))
    finally:
        query(database, 'DROP DATABASE upgrade_example')


def verify(database, query, failure):
    query(database, "SET thinkthen.allowed_backends = 'stub:*'")
    query(database, "SET thinkthen.backend = 'stub'")
    # Run every check and report each failure in one pass.
    failed = []
    for check, arguments in [(shape, ()), (kept_only, (failure,)), (help_text, ()), (ids, (failure,)),
                             (settings, (failure,)), (answer_options, (failure,)), (upgrade, ())]:
        try:
            check(database, query, *arguments)
        except Exception:
            traceback.print_exc()
            failed.append(check.__name__)
    assert not failed, f'interface checks failed: {failed}'


if __name__ == '__main__':
    import runpy
    support = runpy.run_path(str(HERE / 'typed_api.py'))
    with support['server'](support['child_environment']()) as database:
        upgrade(database, support['query'])
