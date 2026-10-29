"""Count paid requests through PostgreSQL joins and its materializing LIMIT."""
from migration_cases import command, fixture


def verify(options, server, query, set_settings, child_environment):
    cases = [
        ("SELECT count(*) FROM generate_series(1,3) outer_row CROSS JOIN "
         "thinkthen.decide('SELECT n::text AS record FROM generate_series(1,65) n','Ready?') t", 195, 3),
        ("SELECT count(*) FROM generate_series(1,3) outer_row(n) CROSS JOIN LATERAL "
         "thinkthen.decide('SELECT $1::text AS record','Ready?',params=>jsonb_build_array(outer_row.n)) t", 3, 3),
        ("SELECT position FROM thinkthen.decide("
         "'SELECT n::text AS record FROM generate_series(1,65) n','Ready?') LIMIT 1", 1, 3),
    ]
    for statement, value, requests in cases:
        with fixture(options.loopback, '--stress', 'healthy') as (loopback, url):
            environment = dict(child_environment(), THINKTHEN_TEST_LOOPBACK_URL=url,
                               THINKTHEN_API_KEY='loopback-test-key', OPENAI_API_KEY='loopback-test-key')
            with server(environment) as database:
                query(database, "SET thinkthen.allowed_backends='openai:*'")
                set_settings(database, {'backend': 'openai', 'batch': 32, 'retry': {'max_retries': 0}})
                assert query(database, statement) == [(value,)], statement
                assert int(command(loopback, 'count')) == requests, statement
