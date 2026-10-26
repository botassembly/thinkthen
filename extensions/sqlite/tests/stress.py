"""Run shared stress cases through SQLite's installed extension and cursors."""
import json
from pathlib import Path
import os
import signal
import subprocess
import sys

sys.path.insert(0, str(Path(__file__).resolve().parents[3] / 'databases'))
from stress import check_round, environment, report, settings, workload


class Host:
    def __init__(self, case):
        self.process = subprocess.Popen(sys.argv[1:], stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                        text=True, env=environment(case))

    def send(self, command):
        try:
            self.process.stdin.write(json.dumps(command) + '\n')
            self.process.stdin.flush()
        except BrokenPipeError:
            self.exited()

    def exited(self):
        code = self.process.wait(timeout=30)
        name = f' ({signal.Signals(-code).name})' if code < 0 else ''
        raise RuntimeError(f'SQLite consumer exited during a statement: returncode={code}{name}') from None

    def receive(self):
        line = self.process.stdout.readline()
        if not line:
            self.exited()
        result = json.loads(line)
        assert result['code'] in (0, 100, 101), result
        return result['rows']

    def sql(self, sql, parameters=(), **options):
        self.send(dict(sql=sql, parameters=parameters, **options))
        return self.receive()

    def close(self, check=True):
        try:
            try:
                self.process.stdin.close()
            except BrokenPipeError:
                pass
            code = self.process.wait(timeout=30)
            if check and code != 0:
                self.exited()
        finally:
            if self.process.poll() is None:
                self.process.kill(); self.process.wait()


def statement(count, prefix='', basis=0):
    source = (f'WITH RECURSIVE rows(n) AS (VALUES(1) UNION ALL SELECT n+1 FROM rows WHERE n<{count}) '
              f"SELECT '{prefix}record-' || (n-1+{basis}) AS id,'{prefix}' || CAST(n AS TEXT) AS record FROM rows ORDER BY n")
    return ('SELECT position,id,status,answer,probability,error_code FROM thinkthen_decide(?,?)',
            [source, 'Ready?'])


def decode(rows):
    return [[*row[:3], None if row[3] is None else bool(row[3]), *row[4:]] for row in rows]


def round_rows(host, case, round_index):
    prefixes = [f'round-{case["identifier_base"] + round_index}-call-{case["identifier_base"] + index}-' for index in range(case['concurrency'])]
    statements = [statement(case['records'], prefix, case['identifier_base']) for prefix in prefixes]
    if case['concurrency'] == 1:
        results = [host.sql(*statements[0])]
    else:
        # One serialized SQLite connection permits interleaved live statements.
        assert case['concurrency'] == 2, 'SQLite consumer owns two cursor slots'
        results = [host.sql(*statements[slot], operation='open', slot=slot) for slot in range(2)]
        for slot in range(2):
            while len(results[slot]) < case['records']:
                for index in range(2):
                    if len(results[index]) < case['records']:
                        host.send(dict(operation='step', slot=index))
                        row = host.receive()
                        assert row, (index, len(results[index]))
                        results[index].extend(row)
            host.send(dict(operation='close', slot=slot))
            host.receive()
    for prefix, rows in zip(prefixes, results):
        yield prefix, decode(rows)


def main():
    case = workload()
    host = Host(case)
    calls = failed = 0
    try:
        for key, value in settings(case).items():
            for name, item in (value.items() if key == 'retry' else [(key, value)]):
                host.sql('SELECT thinkthen_set(?,?)', [name, item])
        sql, parameters = statement(case['records'])
        if case['cancel']:
            host.send(dict(sql=sql, parameters=parameters, interruptible=True))
            assert sys.stdin.readline().strip() == 'cancel'
            host.process.stdin.write('i'); host.process.stdin.flush()
            result = json.loads(host.process.stdout.readline())
            assert result['code'] == 9, result
            assert host.sql('SELECT 1') == [[1]]
            host.sql("SELECT thinkthen_set('backend','stub')")
            assert len(host.sql(*statement(1))) == 1
            report(case, 1, cancelled=True)
            return
        for round_index in range(case['rounds']):
            round_calls, round_failed = check_round(case, round_rows(host, case, round_index))
            calls += round_calls
            failed += round_failed
            if hook := os.environ.get('THINKTHEN_STRESS_CHECKPOINT_COMMAND'):
                subprocess.run(hook, shell=True, check=True)
        assert host.sql('SELECT 1') == [[1]]
    finally:
        host.close(check=sys.exc_info()[0] is None)
    report(case, calls, failed)


if __name__ == '__main__':
    main()
