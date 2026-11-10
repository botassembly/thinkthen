"""Count paid requests through PostgreSQL joins and its materializing LIMIT."""
import json
from pathlib import Path
import re
import tempfile
import uuid

from migration_cases import command, fixture

# 200 records arrive in 128-record source chunks. A pause while fetching the second
# chunk leaves 28 of a 50-record pack pending past the engine's idle window.
PAUSED = ("SELECT count(*) FROM thinkthen.decide('SELECT concat(n, CASE WHEN n = {pause} "
          "THEN pg_sleep(0.5)::text END) AS record FROM generate_series(1,200) n','Ready?') t")


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
    # Run the guide's literal save-and-join example, including its typed id cast.
    guide = Path(__file__).resolve().parents[3] / 'docs/surfaces/postgresql.md'
    examples = [block for block in re.findall(r'```sql\n(.*?)\n```', guide.read_text(), re.S)
                if block.startswith('CREATE TABLE refund_answers AS')]
    assert len(examples) == 1, examples
    save, join, *_ = examples[0].split(';')
    with fixture(options.loopback, '--stress', 'healthy') as (loopback, url):
        environment = dict(child_environment(), THINKTHEN_TEST_LOOPBACK_URL=url,
                           THINKTHEN_API_KEY='loopback-test-key', OPENAI_API_KEY='loopback-test-key')
        with server(environment) as database:
            query(database, "SET thinkthen.allowed_backends='openai:*'")
            set_settings(database, {'backend': 'openai', 'batch': 32, 'retry': {'max_retries': 0}})
            query(database, 'CREATE TABLE messages(msg_id integer PRIMARY KEY, body text)')
            query(database, "INSERT INTO messages VALUES (1,'Refund please'),(2,'Parcel?'),(3,'Thanks'),(4,'Outside range')")
            query(database, save)
            requests = int(command(loopback, 'count'))
            assert requests == 1, requests
            assert query(database, 'SELECT pg_typeof(msg_id)::text FROM refund_answers LIMIT 1') == [('integer',)]
            for _ in range(2):
                rows = query(database, join)
                assert [row[0] for row in rows] == [1, 2, 3], rows
                assert int(command(loopback, 'count')) == requests, rows
    # A host pause splits one pack; the backend's engine reports that split as idle.
    for pause, requests, idle in ((0, 4, 0), (129, 5, 1)):
        statement = PAUSED.format(pause=pause)
        with fixture(options.loopback, '--stress', 'healthy') as (loopback, url), \
                tempfile.TemporaryDirectory(prefix='thinkthen-pg-packing-') as scratch:
            packing, run = Path(scratch) / 'packing.jsonl', uuid.uuid4().hex
            packing.touch()
            environment = dict(child_environment(), THINKTHEN_TEST_LOOPBACK_URL=url,
                               THINKTHEN_API_KEY='loopback-test-key', OPENAI_API_KEY='loopback-test-key',
                               THINKTHEN_INPUT_REPORT=str(packing), THINKTHEN_INPUT_REPORT_RUN=run)
            with server(environment) as database:
                query(database, "SET thinkthen.allowed_backends='openai:*'")
                set_settings(database, {'backend': 'openai', 'batch': 50, 'retry': {'max_retries': 0}})
                assert query(database, statement) == [(200,)], statement
                assert int(command(loopback, 'count')) == requests, statement
            reports = [json.loads(line) for line in packing.read_text().splitlines()]
            observed = [(item['run'], item['complete'], item['records'], item['packed_requests'],
                         item['idle_added_requests']) for item in reports]
            assert observed == [(run, True, 200, requests, idle)], (statement, reports)
