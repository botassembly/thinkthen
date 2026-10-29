"""Count paid requests when a native join repeats result rows."""
from migration_cases import command, fixture


def joins(factory, loopback):
    # Feature: an uncorrelated join reuses one paid invocation's rows.
    with fixture(loopback, '--stress', 'healthy') as (server, url), factory(url) as host:
        host.configure({'backend': 'openai', 'batch': 32, 'retry': {'max_retries': 0}})
        rows = host.query("SELECT count(*) AS count FROM range(3) outer_row CROSS JOIN "
            "thinkthen_decide((SELECT range::VARCHAR AS record FROM range(65)), 'Ready?') t")
        assert rows == [{'count': '195'}], rows
        assert int(command(server, 'count')) == 3
