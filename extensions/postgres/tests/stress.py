"""Run shared stress cases through PostgreSQL statements and native cancellation."""
from concurrent.futures import ThreadPoolExecutor
from contextlib import ExitStack, closing
from pathlib import Path
import os
import subprocess
import runpy
import sys
from threading import Barrier

import psycopg2

ROOT = Path(__file__).resolve().parents[3]
# The existing isolated server and public SQL settings remain the one source.
sys.path.insert(0, str(ROOT / 'databases/duckdb/tests'))
api = runpy.run_path(str(Path(__file__).with_name('typed_api.py')))
sys.path.insert(0, str(ROOT / 'databases'))
from testing import read_line
from stress import check_round, environment, report, settings, workload


def scan(database, count, prefix='', basis=0):
    source = f"SELECT '{prefix}record-' || (n-1+{basis})::text AS id,'{prefix}' || n::text AS record FROM generate_series(1,{count}) n ORDER BY n"
    return api['query'](database, "SELECT position,id,status,answer,probability,error_code FROM thinkthen.decide(%s,'Ready?',id_column=>'id')", (source,))


def configure(database, case):
    api['query'](database, 'SET thinkthen.allowed_backends = %s', (case['client']['backend'] + ':*',))
    api['set_settings'](database, settings(case))


def main():
    case = workload()
    calls = failed = 0
    with api['server'](environment(case)) as database:
        configure(database, case)
        if case['cancel']:
            with ThreadPoolExecutor(max_workers=1) as workers:
                future = workers.submit(scan, database, case['records'])
                try:
                    assert read_line(sys.stdin).strip() == 'cancel'
                    database.cancel()
                    try:
                        future.result(timeout=30)
                    except psycopg2.Error as error:
                        assert error.pgcode == '57014', error
                    else:
                        raise AssertionError('cancel returned completed materialized rows')
                finally:
                    if not future.done():
                        database.cancel()
            assert api['query'](database, 'SELECT count(*) FROM pg_cursors') == [(0,)]
            api['query'](database, "SET thinkthen.allowed_backends = 'stub:*'")
            api['set_settings'](database, {'backend': 'stub'})
            assert len(scan(database, 1)) == 1
            report(case, 1, cancelled=True)
            return
        # A psycopg connection serializes queries; independent sessions provide overlap.
        with ExitStack() as stack:
            connections = [database]
            for _ in range(1, case['concurrency']):
                connection = stack.enter_context(closing(psycopg2.connect(database.dsn)))
                connection.autocommit = True
                configure(connection, case)
                connections.append(connection)
            with ThreadPoolExecutor(max_workers=len(connections)) as workers:
                for round_index in range(case['rounds']):
                    barrier = Barrier(len(connections))
                    def call(item):
                        index, connection = item
                        prefix = f'round-{case["identifier_base"] + round_index}-call-{case["identifier_base"] + index}-'
                        barrier.wait()
                        return check_round(case, [(prefix, scan(connection, case['records'], prefix, case['identifier_base']))])
                    for round_calls, round_failed in workers.map(call, enumerate(connections)):
                        calls += round_calls
                        failed += round_failed
                    if hook := os.environ.get('THINKTHEN_STRESS_CHECKPOINT_COMMAND'):
                        subprocess.run(hook, shell=True, check=True)
            for connection in connections:
                assert api['query'](connection, 'SELECT count(*) FROM pg_cursors') == [(0,)]
    report(case, calls, failed)


if __name__ == '__main__':
    main()
