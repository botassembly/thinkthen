"""Bounded reads for the local fixture's line-oriented control pipe, and its process."""
import os
import json
import math
from pathlib import Path
import queue
import subprocess
import sys
import threading
import time


def read_line(stream, *, timeout=30):
    messages = queue.Queue()

    def receive():
        try:
            messages.put((stream.readline(), None))
        except BaseException as error:  # noqa: BLE001 - the reader thread hands every failure to its caller
            messages.put((None, error))

    worker = threading.Thread(target=receive, daemon=True)
    worker.start()
    try:
        line, error = messages.get(timeout=timeout)
    except queue.Empty as error:
        raise TimeoutError('fixture control read deadline expired') from error
    worker.join(timeout=timeout)
    if error is not None:
        raise error
    return line


def request(process, text, *, timeout=30):
    process.stdin.write(text + '\n')
    process.stdin.flush()
    return read_line(process.stdout, timeout=timeout).strip()


class Fixture:
    def __init__(self, arguments):
        self.process = subprocess.Popen(
            [os.environ['THINKTHEN_LOOPBACK'], *arguments], stdin=subprocess.PIPE,
            stdout=subprocess.PIPE, text=True)
        try:
            port = self.read()
            self.url = f'http://127.0.0.1:{int(port)}/v1'
        except BaseException:
            self.process.kill()
            self.process.wait(timeout=120)
            self.process.stdin.close()
            self.process.stdout.close()
            raise

    def read(self, timeout=30):
        return read_line(self.process.stdout, timeout=timeout).strip()

    def command(self, command, timeout=30, *, progress=None):
        deadline = time.monotonic() + timeout
        self.process.stdin.write(command + '\n')
        self.process.stdin.flush()
        while True:
            response = self.read(max(0, deadline - time.monotonic()))
            if not response:
                raise RuntimeError(f'loopback stopped during {command}')
            if progress is None or response == 'arrived':
                return response
            report = json.loads(response)
            if (set(report) != {'event', 'received', 'required', 'elapsed_ms', 'budget_ms'}
                    or report['event'] != 'arrival'
                    or any(type(report[key]) is not int or report[key] < 0
                           for key in ('received', 'required', 'elapsed_ms', 'budget_ms'))
                    or report['received'] > report['required']):
                raise RuntimeError('invalid fixture arrival progress')
            progress(report)

    def close(self):
        try:
            self.process.stdin.write('shutdown\n')
            self.process.stdin.flush()
            self.process.stdin.close()
            if self.process.wait(timeout=10) != 0:
                raise RuntimeError('loopback failed')
        except (BrokenPipeError, subprocess.TimeoutExpired):
            self.process.kill()
            self.process.wait(timeout=120)
        finally:
            try:
                self.process.stdin.close()
            except BrokenPipeError:
                pass
            self.process.stdout.close()
            if self.process.poll() is None:
                self.process.kill()
                self.process.wait(timeout=120)


class Capture:
    """Drain one child pipe while retaining the bounded end of its diagnostics."""
    def __init__(self, pipe):
        self.data = bytearray()
        self.size = 0
        self.pipe = pipe
        self.thread = threading.Thread(target=self.drain, daemon=True)
        self.thread.start()

    def drain(self):
        with self.pipe:
            # read1 returns what has arrived; read(4096) would hold a short diagnostic until the pipe closes.
            while block := self.pipe.read1(4096):
                self.size += len(block)
                self.data.extend(block)
                del self.data[:-4096]

    def finish(self):
        self.thread.join(timeout=30)
        if self.thread.is_alive():
            raise RuntimeError("adapter output remained open after its deadline")
        return self.data.decode(errors='replace')


def arrived_or_exited(fixture, process, arrivals=1, *, timeout=30):
    """An adapter that fails before opening its request cannot strand arrival."""
    events = queue.Queue()
    started = time.monotonic()
    deadline = started + timeout
    received = 0
    def arrival():
        try:
            remaining = max(0, deadline - time.monotonic())
            milliseconds = max(1, math.ceil(remaining * 1000))
            events.put(('arrival', fixture.command(f'arrived {arrivals} {milliseconds}', remaining,
                        progress=lambda report: events.put(('progress', report)))))
        except Exception as error:  # noqa: BLE001 - the arrival thread hands every failure to its caller
            events.put(('error', str(error)))
    threading.Thread(target=arrival, daemon=True).start()
    threading.Thread(target=lambda: events.put(('exit', process.wait(timeout=timeout + 90))), daemon=True).start()
    while True:
        try:
            kind, value = events.get(timeout=max(0, deadline - time.monotonic()))
        except queue.Empty:
            kind, value = 'deadline', 'adapter did not open every request before its deadline'
        if kind == 'progress':
            if value['required'] != arrivals or value['received'] < received:
                kind, value = 'error', 'invalid fixture arrival progress count'
            else:
                received = value['received']
                continue
        if kind == 'arrival' and value == 'arrived':
            if time.monotonic() <= deadline:
                return
            kind, value = 'deadline', 'arrival confirmation exceeded the controller deadline'
        raise RuntimeError(f'adapter stopped before request arrival: {kind} {value}; '
                           f'last_observed_received={received} required={arrivals} '
                           f'elapsed_seconds={time.monotonic() - started:.3f} budget_seconds={timeout:g}')


def soak(surface, command, environment, **options):
    """Run a surface's 60-round soak on both backends when the extended soak names a folder."""
    folder = os.environ.get('THINKTHEN_SOAK_FOLDER')
    if not folder:
        return
    runner = Path(__file__).with_name('stress-adapters.py')
    failed = [backend for backend in ('systemone', 'openai') if subprocess.run(
        [sys.executable, str(runner), '--surface', surface, '--rounds', '60', '--backend', backend,
         '--evidence-path', str(Path(folder) / f'{surface}-{backend}.jsonl'), '--', *command],
        env=dict(environment, **{name: os.environ[name] for name in ('THINKTHEN_SOAK_ROUNDS', 'THINKTHEN_SOAK_RECORDS')
                                 if name in os.environ}), **options).returncode]
    if failed:
        raise SystemExit(f'{surface} 60-round soak failed on: {", ".join(failed)}')
