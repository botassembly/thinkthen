"""Check source translation, per-row ownership and catalog isolation through SQL."""
from concurrent.futures import ThreadPoolExecutor
from contextlib import closing
from threading import Barrier

from psycopg2 import sql


def records(host, case):
    examples = dict(zip(case['request']['records'], case['expected']))
    source = "SELECT CASE WHEN n % 7 = 0 THEN 'evidence 5' ELSE 'alpha' END::text AS record FROM generate_series(0,1024) source(n) ORDER BY n"
    actual = host.call(source=source)
    for position, status, answer, probability, error, detail, kept in actual:
        wanted = examples['evidence 5' if (position - 1) % 7 == 0 else 'alpha']
        assert (status, answer, probability, error, detail) == (
            wanted['status'], wanted['answer'], wanted['p'], None, wanted['detail']), position
    assert [row[0] for row in actual] == list(range(1, 1026))
    barrier = Barrier(2)

    def concurrent(descending):
        with closing(host.connect()) as connection, connection.cursor() as cursor:
            cursor.execute("SET thinkthen.allowed_backends = '{\"stub\":[\"stub\"]}'")
            cursor.execute("SET thinkthen.settings = '{\"adapter\":\"stub\"}'")
            barrier.wait()
            cursor.execute("SELECT * FROM thinkthen_decide(%s, 'Ready?')", (source + (' DESC' if descending else ''),))
            result = cursor.fetchall()
            assert [row[0] for row in result] == list(range(1, 1026))
            for row in result:
                index = 1025 - row[0] if descending else row[0] - 1
                wanted = examples['evidence 5' if index % 7 == 0 else 'alpha']
                assert (row[1], row[2], row[3], row[4], row[5]) == (
                    wanted['status'], wanted['answer'], wanted['p'], None, wanted['detail']), row[0]
            cursor.execute('SELECT count(*) FROM pg_cursors')
            assert cursor.fetchone() == (0,)

    with ThreadPoolExecutor(max_workers=2) as workers:
        list(workers.map(concurrent, (False, True)))


def admission(host, expect_error):
    for column_type in ('text', 'json'):
        for null_at in (0, 256):
            source = f"SELECT CASE WHEN n={null_at} THEN NULL ELSE 'null' END::{column_type} AS record FROM generate_series(0,256) source(n) ORDER BY n"
            expect_error(host, lambda: host.call(source=source), '22004', 'SQL NULL')
    assert len(host.call(source="SELECT 'null'::json AS record")) == 1
    for source, parameters, text in (
        ("SELECT $1::text AS record", ["café' $5"], "café' $5"),
        ("SELECT $1::numeric::text AS record", [18446744073709551615], '18446744073709551615'),
        ("SELECT $1::text || $2::numeric::text || $3::boolean::text || coalesce($4::text,'null') || $1 AS record",
         ["café' $5", 123.25, True, None], "café' $5123.25truenullcafé' $5"),
    ):
        literal = sql.SQL('SELECT {}::text AS record').format(sql.Literal(text)).as_string(host.connection)
        assert host.call(source=source, parameters=parameters) == host.call(source=literal)


def catalog(host):
    expected = host.call(source="SELECT 'alpha'::text AS record")
    host.query('CREATE SCHEMA shadow')
    host.query('SET search_path = shadow, public, pg_catalog')
    failures = []
    for name, args, result, after in (
        ('json_array_elements', 'pg_catalog.json', 'SETOF pg_catalog.json', ''),
        ('json_typeof', 'pg_catalog.json', 'pg_catalog.text', ''),
        ('extract_json', 'pg_catalog.json, pg_catalog.text[]', 'pg_catalog.text',
         'CREATE OPERATOR shadow.#>> (LEFTARG = pg_catalog.json, RIGHTARG = pg_catalog.text[], FUNCTION = shadow.extract_json)'),
    ):
        host.query(f"CREATE FUNCTION shadow.{name}({args}) RETURNS {result} LANGUAGE plpgsql AS $$ BEGIN RAISE EXCEPTION 'shadow decoder called'; END $$")
        if after:
            host.query(after)
        try:
            assert host.call(source='SELECT $1::text AS record', parameters=['alpha']) == expected
        except Exception as error:
            failures.append((name, str(error)))
        if after:
            host.query('DROP OPERATOR shadow.#>> (pg_catalog.json, pg_catalog.text[])')
        host.query(f'DROP FUNCTION shadow.{name}({args})')
    host.query('RESET search_path')
    with closing(host.connect()) as connection, connection.cursor() as cursor:
        cursor.execute("SET thinkthen.misspelled = 'unused'")
        cursor.execute("SET thinkthen.allowed_backends = '{\"stub\":[\"stub\"]}'")
        cursor.execute("SET thinkthen.settings = '{\"adapter\":\"stub\"}'")
        cursor.execute("SELECT * FROM thinkthen_decide('SELECT ''alpha''::text AS record', 'Ready?')")
        assert cursor.fetchall() == expected
        if not any('thinkthen.misspelled' in notice for notice in connection.notices):
            failures.append(('prefix', 'pre-load placeholder was not removed'))
        try:
            cursor.execute("SET thinkthen.misspelled = 'unused'")
        except Exception as error:
            assert error.pgcode == '42602', error
        else:
            failures.append(('prefix', 'unknown setting remained accepted after load'))
    assert not failures, failures
