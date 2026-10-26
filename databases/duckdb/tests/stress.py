"""Run the shared stress workload through native DuckDB connections."""
import json
from pathlib import Path
import os
import subprocess
import sys

from typed_api import Host, literal

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from stress import check_round, environment, report, settings, workload


def statement(count, prefix='', basis=0):
    source = (f"SELECT {literal(prefix + 'record-')} || (range+{basis})::VARCHAR AS document_id, "
              f"{literal(prefix)} || range::VARCHAR AS record FROM range({count})")
    return ('SELECT to_json(t) AS result FROM (SELECT position,document_id,status,answer,probability,error_code '
            f"FROM thinkthen_decide(({source}), 'Ready?')) t")


def decode(result):
    assert 'error' not in result, result
    rows = (json.loads(item['result']) for item in result['rows'])
    return [[row[key] for key in ('position', 'document_id', 'status', 'answer', 'probability', 'error_code')]
            for row in rows]


def configure(host, case, prefix=''):
    for key, value in settings(case).items():
        for name, item in (value.items() if key == 'retry' else [(key, value)]):
            host.ok(prefix + 'SET thinkthen_' + name + '=' + (literal(item) if isinstance(item, str) else str(item)))


def round_rows(host, case, round_index):
    prefixes = [f'round-{case["identifier_base"] + round_index}-call-{case["identifier_base"] + index}-' for index in range(case['concurrency'])]
    statements = [statement(case['records'], prefix, case['identifier_base']) for prefix in prefixes]
    if case['concurrency'] == 1:
        results = [host.query(statements[0])]
    else:
        assert case['concurrency'] == 2, 'DuckDB host owns two connections'
        host.process.stdin.write(json.dumps({'concurrent': statements}) + '\n')
        host.process.stdin.flush()
        results = [json.loads(host.process.stdout.readline()) for _ in range(2)]
    for prefix, result in zip(prefixes, results):
        yield prefix, decode(result)


def main():
    case = workload()
    package = Path(sys.argv[1])
    if case['cancel']:
        # The runner sends cancellation only after the held request arrives.
        commands = []
        for key, value in settings(case).items():
            for name, item in (value.items() if key == 'retry' else [(key, value)]):
                commands.append('SET thinkthen_' + name + '=' + (literal(item) if isinstance(item, str) else str(item)))
        sql = ';'.join([*commands, statement(case['records'])])
        process = subprocess.Popen([str(package / 'sql'), str(package / 'thinkthen.duckdb_extension'), 'cancel', sql],
                                   stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, env=environment(case))
        try:
            assert sys.stdin.readline().strip() == 'cancel'
            process.stdin.write('c'); process.stdin.flush()
            result = json.loads(process.stdout.readline())
            assert 'interrupt' in result.get('error', '').lower(), result
            assert process.wait(timeout=30) == 0
        finally:
            if process.poll() is None:
                process.kill(); process.wait()
        report(case, 1, cancelled=True)
        return
    calls = failed = 0
    with Host(package, environment(case)) as host:
        configure(host, case)
        if case['concurrency'] > 1:
            configure(host, case, '2:')
        for round_index in range(case['rounds']):
            round_calls, round_failed = check_round(case, round_rows(host, case, round_index))
            calls += round_calls
            failed += round_failed
            if hook := os.environ.get('THINKTHEN_STRESS_CHECKPOINT_COMMAND'):
                subprocess.run(hook, shell=True, check=True)
        assert host.ok('SELECT 1 AS alive') == [{'alive': '1'}]
    report(case, calls, failed)


if __name__ == '__main__':
    main()
