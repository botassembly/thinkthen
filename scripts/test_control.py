"""Bounded reads for the local fixture's line-oriented control pipe, and its process."""
import os
import queue
import subprocess
import threading


def read_line(stream, *, timeout=30):
    messages = queue.Queue()

    def receive():
        try:
            messages.put((stream.readline(), None))
        except BaseException as error:
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

    def command(self, command, timeout=30):
        self.process.stdin.write(command + '\n')
        self.process.stdin.flush()
        response = self.read(timeout)
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
            while block := self.pipe.read(4096):
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
    def arrival():
        try:
            events.put(('arrival', fixture.command('arrived' if arrivals == 1 else f'arrived {arrivals}', timeout)))
        except Exception as error:
            events.put(('error', str(error)))
    threading.Thread(target=arrival, daemon=True).start()
    threading.Thread(target=lambda: events.put(('exit', process.wait(timeout=timeout + 90))), daemon=True).start()
    try:
        kind, value = events.get(timeout=timeout)
    except queue.Empty as error:
        raise RuntimeError("adapter did not open a request before its deadline") from error
    if kind != 'arrival' or value != 'arrived':
        raise RuntimeError(f'adapter stopped before request arrival: {kind} {value}')
