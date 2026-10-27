"""Bound SQLite consumption through ordinary LIMIT and EXISTS statements."""
from migration_cases import command, fixture


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
