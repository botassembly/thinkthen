"""Bounded reads for the local fixture's line-oriented control pipe."""
import queue
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
