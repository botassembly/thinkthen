"""Measure native SPI buffering separately from bounded engine iterator pulls."""
from concurrent.futures import ThreadPoolExecutor
import json
import subprocess
import time

import psycopg2

from migration_cases import command


def verify(options, server, query, set_settings, fixture, child_environment):
    limits = json.loads(subprocess.check_output([str(options.loopback), '--limits'],
        env=child_environment(), text=True, timeout=30))
    batch = 32
    with fixture(options, held=True) as (loopback, environment), server(environment) as database:
        query(database, "SET thinkthen.allowed_backends='systemone:*'")
        set_settings(database, {'batch': batch, 'retry': {'attempt_timeout_ms': 120000}})
        query(database, "SET statement_timeout='10s'")
        query(database, 'CREATE TEMP SEQUENCE source_reads')
        query(database, "CREATE FUNCTION pg_temp.source_chunk_limit() RETURNS integer "
              "AS %s,'thinkthen_source_chunk_limit' LANGUAGE C",
              (str(options.abi_mismatch_extension.parent / 'family-probe.so'),))
        chunk = query(database, 'SELECT pg_temp.source_chunk_limit()')[0][0]
        source = "SELECT nextval('source_reads')::text AS record FROM generate_series(1,8192)"
        with ThreadPoolExecutor(max_workers=1) as workers:
            future = workers.submit(query, database,
                "SELECT count(*) FROM thinkthen.decide(%s,'Ready?')", (source,))
            try:
                assert command(loopback, 'arrived') == 'arrived'
                started = time.monotonic()
                database.cancel()
                assert command(loopback, 'closed', timeout=2) == 'closed'
                try:
                    future.result(timeout=2)
                except psycopg2.Error as error:
                    assert error.pgcode == '57014', error
                else:
                    raise AssertionError('held source returned normally')
            finally:
                if not future.done():
                    database.cancel()
        paid = int(command(loopback, 'count'))
        count = query(database, 'SELECT last_value FROM source_reads')[0][0]
        engine_bound = batch * limits['read_ahead_window'] + 1
        host_bound = ((engine_bound + chunk - 1) // chunk) * chunk
        assert 0 < count <= host_bound, (count, chunk, engine_bound, host_bound)
        assert int(command(loopback, 'count')) == paid
        print('PostgreSQL held native source buffering:', dict(source_rows=count,
            native_chunk_size=chunk, engine_iterator_bound=engine_bound, native_bound=host_bound,
            paid_requests=paid, cancel_seconds=time.monotonic() - started), flush=True)
