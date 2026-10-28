#!/usr/bin/env python3
"""Run installed public API adapters against the shared loopback stress table.

Usage: stress-adapters.py --surface LABEL [--full] [--sequential] -- ADAPTER [ARG ...]
THINKTHEN_LOOPBACK names the prepared loopback executable. Each adapter reads the
JSON file at THINKTHEN_STRESS_CASE. Fields: client (wire settings including URL and
synthetic key), scenario (shared expected status/code), records, probability,
concurrency, rounds, cancel. Generate records lazily where the public API allows.
Validate every row's ordinal position, record-N id, status, answer/probability and
error code. Emit one JSON line: {calls, rows, failed, cancelled}. For cancel cases,
start an in-flight batch, read 'cancel' from stdin and cancel through the public
API. No adapter selects faults, workload sizes or soak rounds. --sequential explicitly
reports a sequential API and replaces the concurrent pair with two serial calls.
"""
import argparse
import json
import os
import queue
import re
import shlex
import signal
from pathlib import Path
import subprocess
import sys
import tempfile
import threading

from stress_monitor import Monitor
from stress_resources import adopt_daemons


class Fixture:
    def __init__(self, arguments):
        self.process = subprocess.Popen(
            [os.environ['THINKTHEN_LOOPBACK'], *arguments], stdin=subprocess.PIPE,
            stdout=subprocess.PIPE, text=True)
        try:
            port = self.process.stdout.readline().strip()
            self.url = f'http://127.0.0.1:{int(port)}/v1'
        except BaseException:
            self.process.kill()
            self.process.wait()
            self.process.stdin.close()
            self.process.stdout.close()
            raise

    def command(self, command):
        self.process.stdin.write(command + '\n')
        self.process.stdin.flush()
        response = self.process.stdout.readline().strip()
        if not response:
            raise RuntimeError(f'loopback stopped during {command}')
        return response

    def close(self):
        try:
            self.process.stdin.write('shutdown\n')
            self.process.stdin.flush()
            self.process.stdin.close()
            if self.process.wait(timeout=10) != 0:
                raise RuntimeError('loopback failed')
        except (BrokenPipeError, subprocess.TimeoutExpired):
            self.process.kill()
            self.process.wait()
        finally:
            try:
                self.process.stdin.close()
            except BrokenPipeError:
                pass
            self.process.stdout.close()
            if self.process.poll() is None:
                self.process.kill()
                self.process.wait()


class Capture:
    """Drain one child pipe while retaining a bounded diagnostic prefix."""
    def __init__(self, pipe):
        self.data = bytearray()
        self.size = 0
        self.pipe = pipe
        self.thread = threading.Thread(target=self.drain, daemon=True)
        self.thread.start()

    def drain(self):
        with self.pipe:
            while block := self.pipe.read(4096):
                self.size += len(block)
                self.data.extend(block[:max(0, 4096 - len(self.data))])

    def finish(self):
        self.thread.join()
        return self.data.decode(errors='replace')


def arrived_or_exited(fixture, process):
    """An adapter that fails before opening its request cannot strand arrival."""
    events = queue.Queue()
    def arrival():
        try:
            events.put(('arrival', fixture.command('arrived')))
        except Exception as error:
            events.put(('error', str(error)))
    threading.Thread(target=arrival, daemon=True).start()
    threading.Thread(target=lambda: events.put(('exit', process.wait())), daemon=True).start()
    kind, value = events.get()
    if kind != 'arrival' or value != 'arrived':
        raise RuntimeError(f'adapter stopped before request arrival: {kind} {value}')


def stop_adapter(process):
    """The session belongs to this workload, including interpreter helper children."""
    try:
        os.killpg(process.pid, signal.SIGKILL)
    except ProcessLookupError:
        pass
    process.wait()


def identifier_bounds(table, count, concurrency, rounds):
    """One numeric source covers record, call and SQL-prefix namespaces."""
    basis = table['records']['identifier_base']
    batch = table['client']['batch']
    largest_count = max(count, table['records']['full'], table['records']['smoke'])
    rounded = ((largest_count + batch - 1) // batch) * batch
    calls = max(concurrency, 2) * max(rounds, table['records']['soak_rounds'], 2)
    largest = max(rounded * calls - 1, calls - 1)
    if not isinstance(basis, int) or basis <= 0 or basis + largest > 2**31 - 1:
        raise ValueError('stress identifier range must fit signed 32-bit consumers')
    if len(str(basis)) != len(str(basis + largest)):
        raise ValueError('stress identifier range changes decimal width')
    return basis


def run(command, table, wire, scenario, count, concurrency=1, rounds=1, cancel=False, resources=False, result=None, *, surface, short_gate=False):
    report = result if result is not None else {}
    report.update(surface=surface, command=shlex.join(command), backend=wire, scenario=scenario['name'],
        case=f"{scenario['name']}/concurrency={concurrency}/rounds={rounds}/cancel={cancel}",
        concurrency=concurrency, rounds=rounds, cancelled=cancel)
    basis = identifier_bounds(table, count, concurrency, rounds)
    fixture = Fixture(['--held', 'body'] if cancel else ['--stress', scenario['name']])
    process = None
    monitor = None
    try:
        with tempfile.TemporaryDirectory(prefix='thinkthen-stress-') as directory:
            path = Path(directory) / 'case.json'
            client = dict(table['client'], backend=wire, url=fixture.url, api_key='loopback-only')
            path.write_text(json.dumps(dict(client=client, scenario=scenario, records=count,
                probability=table['replies'][0]['probability'], concurrency=concurrency,
                rounds=rounds, cancel=cancel, identifier_base=basis)))
            env = dict(os.environ, THINKTHEN_STRESS_CASE=str(path),
                THINKTHEN_STRESS_CONTEXT=json.dumps({key: report[key] for key in ('surface', 'backend', 'case')}))
            if resources:
                adopt_daemons()
                monitor = Monitor(lambda: [os.getpid()], int(fixture.url.split(':')[2].split('/')[0]))
                env.update(THINKTHEN_STRESS_CHECKPOINT=monitor.path, THINKTHEN_STRESS_CONCURRENCY=str(concurrency),
                    THINKTHEN_STRESS_CHECKPOINT_COMMAND='exec ' + shlex.join([sys.executable, str(Path(__file__).with_name('stress-checkpoint'))]))
            process = subprocess.Popen(command, env=env, stdin=subprocess.PIPE,
                stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
            output, errors = Capture(process.stdout), Capture(process.stderr)
            try:
                if cancel:
                    arrived_or_exited(fixture, process)
                    process.stdin.write(b'cancel\n')
                    process.stdin.flush()
                process.stdin.close()
                status = process.wait()
                report['exit_code'] = status
                stop_adapter(process)
                result, diagnostic = output.finish(), errors.finish()
                if status or output.size > 4096:
                    raise RuntimeError(f'adapter failed ({status}): {diagnostic} {result}')
                report.update(json.loads(result))
                report.update(concurrency=concurrency, rounds=rounds)
                expected_calls = concurrency * rounds
                expected_rows = 0 if cancel else count * expected_calls
                if report['calls'] != expected_calls or report['rows'] != expected_rows or report['cancelled'] != cancel:
                    raise RuntimeError(f'adapter lost calls, rows or cancellation: {report}')
                if not cancel and (report['failed'] > 0) != (scenario['status'] == 'failed'):
                    raise RuntimeError(f'adapter lost expected failure rows: {report}')
                if cancel and fixture.command('closed') != 'closed':
                    raise RuntimeError('cancelled request remained open')
                mask = report.pop('identifier_width_mask')
                widths = [width for width in range(mask.bit_length()) if mask & (1 << width)]
                if not cancel and len(widths) != 1:
                    raise RuntimeError(f'actual stress identifiers changed width: {widths}')
                report.update(concurrency=concurrency, rounds=rounds,
                    identifiers=dict(observed_widths=widths, min_width=min(widths, default=None),
                        max_width=max(widths, default=None), width_changed=len(widths) > 1))
                if monitor is not None:
                    report['resources'] = monitor.finish(rounds,
                        verified_records={'workload': [report['rows'] // rounds] * rounds},
                        expected_context={key: report[key] for key in ('surface', 'backend', 'case')},
                        short_gate=short_gate)
                    monitor = None
                    if not report['resources']['growth']['passed']:
                        raise RuntimeError('resource use is unverifiable' if
                            report['resources']['growth'].get('verdict') == 'unverifiable' else
                            'retained resources fail growth criterion')
                return report
            except BaseException as error:
                stop_adapter(process)
                fixture.process.kill()
                fixture.process.wait()
                raise RuntimeError(f'{error}; adapter diagnostics: {errors.finish()}') from error
    finally:
        if process is not None:
            stop_adapter(process)
        if monitor is not None:
            report['resources'] = monitor.finish()
        fixture.close()


def surface_label(value):
    if not re.fullmatch('[a-z][a-z0-9-]{0,31}', value):
        raise argparse.ArgumentTypeError('surface must be 1 to 32 lowercase letters, digits or hyphens, starting with a letter')
    return value


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--surface', type=surface_label, required=True)
    parser.add_argument('--full', action='store_true', default=bool(os.environ.get('THINKTHEN_STRESS_FULL')))
    parser.add_argument('--sequential', action='store_true')
    parser.add_argument('--sanitizer', choices=('address', 'thread'))
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.command[1:] if args.command[:1] == ['--'] else args.command
    if not command:
        parser.error('an installed adapter command is required')
    if args.full and not Path('/proc/self/status').exists():
        parser.error('full resource accounting requires Linux /proc; run inside the Linux container')
    root = Path(__file__).resolve().parent.parent
    table = json.loads((root / 'conformance/stress.json').read_text())
    backends = json.loads((root / 'conformance/typed/manifest.json').read_text())['backends']
    count = table['records']['full' if args.full else 'smoke']
    healthy = next(s for s in table['scenarios'] if s['name'] == 'healthy')
    calls = rows = failed = 0
    reports = []
    for wire in backends:
        cases = [(s, 1, 1, False) for s in table['scenarios']]
        pair = (healthy, 1, 2, False) if args.sequential else (healthy, 2, 1, False)
        cases += [pair, (healthy, 1, 1, True)]
        if args.full:
            cases += [(healthy, 1, table['records']['soak_rounds'], False)]
        for scenario, concurrency, rounds, cancel in cases:
            result = {}
            try:
                run(command, table, wire, scenario, count, concurrency, rounds, cancel,
                    resources=args.full and not args.sanitizer and rounds == table['records']['soak_rounds'],
                    result=result, surface=args.surface)
                totals = calls + result['calls'], rows + result['rows'], failed + result['failed']
            except Exception as error:
                result['error'] = str(error)
            reports.append(result)
            print(json.dumps(result), file=sys.stderr, flush=True)
            if 'error' in result:
                break
            calls, rows, failed = totals
        if 'error' in reports[-1]:
            break
    print(json.dumps(dict(adapter=command[0], full=args.full, execution="sequential" if args.sequential else "concurrent", calls=calls, rows=rows, failed=failed, cases=reports, sanitizer=args.sanitizer,
        resource_growth='not measured under sanitizer instrumentation' if args.sanitizer else
            ('completed-round verdicts' if args.full else 'not measured in smoke workload'))), flush=True)
    return int('error' in reports[-1])


if __name__ == '__main__':
    sys.exit(main())
