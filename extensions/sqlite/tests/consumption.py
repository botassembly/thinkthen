"""Bound SQLite consumption through ordinary LIMIT and EXISTS statements."""
import json
import subprocess
import time

from migration_cases import child_environment, command, fixture, read_line


def streaming(host_type, loopback):
    # Feature: LIMIT and EXISTS stop without consuming the remaining source.
    for statement, expected in (
        ('SELECT position,id,status FROM thinkthen_filter(?,?) LIMIT 1', [[1,1,'answered']]),
        ('SELECT EXISTS(SELECT 1 FROM thinkthen_filter(?,?))', [[1]]),
    ):
        with fixture(loopback, '--stress', 'healthy') as (server,url),host_type(url) as host:
            host.configure({'backend':'openai','batch':32})
            source="WITH RECURSIVE r(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM r WHERE n<4096) SELECT n AS id,CAST(n AS TEXT) AS record FROM r"
            assert host.ok(statement,[source,'Ready?'])==expected
            requests=int(command(server,'count'))
            assert 1<=requests<=2,requests
    native_read_ahead(host_type, loopback)


def native_read_ahead(host_type, loopback):
    # Feature: native chunks add a bounded source buffer to engine iterator demand.
    limits = json.loads(subprocess.check_output([str(loopback), '--limits'],
        env=child_environment(), text=True, timeout=30))
    batch = 32
    with fixture(loopback, '--held', 'headers') as (server, url), host_type(url) as host:
        host.configure({'batch': batch, 'retry': {'attempt_timeout_ms': 120000}})
        source = ("WITH RECURSIVE r(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM r WHERE n<8192) "
                  "SELECT CAST(tick(n) AS TEXT) AS record FROM r")
        host.send(dict(sql='SELECT count(*) FROM thinkthen_decide(?,?)',
                       parameters=[source, 'Ready?'], interruptible=True))
        assert command(server, 'arrived') == 'arrived'
        started = time.monotonic()
        host.process.stdin.write('i'); host.process.stdin.flush()
        assert command(server, 'closed', timeout=2) == 'closed'
        result = json.loads(read_line(host.process.stdout, timeout=2))
        assert result['code'] == 9, result
        paid = int(command(server, 'count'))
        count, chunk = host.ok('SELECT tick_count(),source_chunk_limit()')[0]
        engine_bound = batch * limits['read_ahead_window'] + 1
        host_bound = ((engine_bound + chunk - 1) // chunk) * chunk
        assert 0 < count <= host_bound, (count, chunk, engine_bound, host_bound)
        assert int(command(server, 'count')) == paid
        print('SQLite held native source buffering:', dict(source_rows=count,
            native_chunk_size=chunk, engine_iterator_bound=engine_bound, native_bound=host_bound,
            paid_requests=paid, cancel_seconds=time.monotonic() - started), flush=True)
