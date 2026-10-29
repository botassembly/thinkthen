"""Bound SQL consumption and rank source retention through the installed host."""
import json
from pathlib import Path
import subprocess
import sys
import time

from migration_cases import child_environment, command, fixture


def rank_memory(package):
    # Feature: rank retains native source columns within a bounded resident heap.
    if sys.platform != 'win32':
        result = subprocess.run([str(package / 'sql'), str(package / 'thinkthen.duckdb_extension'), '--rank-memory'],
                                env=child_environment(), capture_output=True, text=True, timeout=120)
        assert result.returncode == 0, (result.stdout, result.stderr)
        observation = json.loads(result.stdout)
        assert observation['rows'] == 100000 and observation['misplaced'] == 0, observation
        print('DuckDB rank source retention:', observation, flush=True)
        assert observation['growth_bytes'] < 128 * 1024 * 1024, observation


def first_response(host_type, loopback):
    # Opt-in diagnostic: measure the pinned host's streaming LIMIT behavior.
    marker = 'first-visible-source'
    with fixture(loopback, '--first-response', marker) as (server, url), host_type(url) as host:
        settings = {'backend': 'openai', 'batch': 32, 'retry': {'attempt_timeout_ms': 2000}}
        host.configure(settings)
        # Reduce the native caller buffer; the host pipeline can still need
        # another chunk before Fetch returns.
        host.query("SET streaming_buffer_size='1B'")
        rows = host.query({'first': "SELECT document_id,position,status FROM thinkthen_decide((SELECT range AS document_id, CASE WHEN range=0 THEN '" + marker + "' ELSE range::VARCHAR END AS record FROM range(8192)), 'Ready?') LIMIT 10"})
        # The runner fetches one chunk before closing the stream. Pending
        # upstream work can yield a short chunk before the SQL LIMIT is reached.
        count = len(rows)
        assert 0 < count <= 10 and all(row['status'] == 'answered' for row in rows), rows
        assert [int(row['document_id']) for row in rows] == list(range(count)), rows
        assert [int(row['position']) for row in rows] == list(range(1, count + 1)), rows
        stats = json.loads(command(server, 'stats'))
        print('DuckDB first-chunk LIMIT diagnostic (lifetime totals; sampled request sizes):',
              dict(stats, first_chunk_rows=count,
                   request_sizes_truncated=stats['recorded_requests'] < stats['requests']), flush=True)
        assert 0 < stats['recorded_requests'] <= stats['requests'], stats
        assert len(stats['request_sizes']) == stats['recorded_requests'], stats
        assert all(1 <= size <= settings['batch'] for size in stats['request_sizes']), stats
        # Held attempts can time out and workers can send later requests before
        # DuckDB returns its first chunk. Lifetime totals are diagnostic metrics;
        # the fixture retains a bounded sample of request sizes. Exact paid-work,
        # read-ahead and post-cancel bounds run in separate installed-host tests.


def streaming(host_type, package, loopback):
    rank_memory(package)
    native_read_ahead(package, loopback)


def native_read_ahead(package, loopback):
    # Feature: native source buffering remains bounded while the engine is held.
    with fixture(loopback, '--held', 'headers') as (server, url):
        environment = dict(child_environment(), THINKTHEN_TEST_LOOPBACK_URL=url,
                           THINKTHEN_API_KEY='loopback-test-key')
        statement = ("SET thinkthen_batch=32; SET thinkthen_attempt_timeout_ms=120000; "
            "SELECT count(*) FROM thinkthen_decide((SELECT record_read(range)::VARCHAR AS record "
            "FROM range(8192)), 'Ready?')")
        with subprocess.Popen([str(package / 'sql'), str(package / 'thinkthen.duckdb_extension'),
                               '--read-ahead', statement], stdin=subprocess.PIPE,
                              stdout=subprocess.PIPE, text=True, env=environment) as process:
            try:
                assert command(server, 'arrived') == 'arrived'
                started = time.monotonic()
                process.stdin.write('c'); process.stdin.flush()
                assert command(server, 'closed', timeout=2) == 'closed'
                from migration_cases import read_line
                assert 'interrupt' in json.loads(read_line(process.stdout, timeout=2))['error'].lower()
                observed = json.loads(read_line(process.stdout, timeout=2))
                assert 0 < observed['source_rows'] <= observed['native_vector_size'], observed
                observed['paid_requests'] = int(command(server, 'count'))
                observed['cancel_seconds'] = time.monotonic() - started
                print('DuckDB held native source buffering (engine iterator is a separate boundary):', observed, flush=True)
                assert process.wait(timeout=2) == 0
            finally:
                if process.poll() is None:
                    process.kill(); process.wait()
