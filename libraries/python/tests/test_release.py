"""Change 6: a column's input batches are released attached to the
interpreter, behind the exit gate, or leaked on purpose once exit starts.

The producers are ``arrow_c.Stream``, whose ``ctypes`` releases drop an
object from a dictionary, and the probe build's ``_raw_producer``, whose
releases abort the process when the interpreter lock is not held, as a C
producer that assumes the lock may. Each child sets its core size to 0,
since an abort would dump core.
"""

import pathlib
import resource
import shutil
import subprocess
import sys
import tempfile
import textwrap
import threading
from concurrent.futures import ThreadPoolExecutor

import pytest

from conftest import Backend, child_env, run

TESTS = str(pathlib.Path(__file__).resolve().parent)
PACKAGE = pathlib.Path(__import__("thinkthen").__file__).parent
SETUP = f"""
    import os, sys, threading, time
    sys.path.insert(0, {TESTS!r})
    import thinkthen as tt
    from arrow_c import HELD, Stream
    late = tt.question(decide="Is it late?")
    class Raw:
        def __init__(self, batches, token):
            self.batches, self.token = batches, token
        def __arrow_c_stream__(self, requested_schema=None):
            return tt._thinkthen._raw_producer(self.batches, self.token)
"""


def no_core():
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))


def released_after_cancel(backend, tmp_path, calls):
    """Run each producer in one real child and inspect its released batches.

    Each call's send waits on the held arm. The parent cancels the call once
    the backend counts that send, then lets the held reply go, so every call
    ends by its token and no timer races a reply (ticket 0352)."""
    code = SETUP + """
    engine = tt.Engine(throttle=8, cache=False)
    token = object()
    base = sys.getrefcount(token)
    for kind in ("ctypes", "raw"):
        cancelled = 0
        for _ in range(int(os.environ["CALLS"])):
            stop = tt.CancelToken()
            def go(stop=stop):
                sys.stdin.readline()
                stop.cancel()
            threading.Thread(target=go, daemon=True).start()
            try:
                engine.decide(late, Stream(3) if kind == "ctypes" else Raw(3, token), token=stop).value
            except tt.Cancelled:
                cancelled += 1
            print("ended", flush=True)
        assert cancelled == int(os.environ["CALLS"]), (kind, cancelled)
        ended = time.monotonic() + 60
        while (HELD or tt._thinkthen._live_workers()) and time.monotonic() < ended:
            time.sleep(0.01)
        print(kind, len(HELD), tt._thinkthen._live_workers(), sys.getrefcount(token) == base, flush=True)
    """
    child = subprocess.Popen([sys.executable, "-c", textwrap.dedent(code)], stdin=subprocess.PIPE,
                             stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, bufsize=1,
                             preexec_fn=no_core,
                             env=child_env(backend, tmp_path, "arm/held", CALLS=str(calls)))
    # A hang guard, not a speed claim: a call that never ends stops the child.
    guard = threading.Timer(120 if calls == 1 else 600, child.kill)
    guard.start()
    try:
        sent, printed = 0, []
        for _ in range(2):
            for _ in range(calls):
                expected = sent + 1
                sent = backend.wait(expected)
                assert sent == expected, (sent, expected)
                child.stdin.write("stop\n")
                child.stdin.flush()
                assert child.stdout.readline() == "ended\n", child.stderr.read()
                backend.round()
            printed.append(child.stdout.readline().strip())
        assert child.wait() == 0, child.stderr.read()
    finally:
        guard.cancel()
    assert printed == ["ctypes 0 0 True", "raw 0 0 True"]


def released_after_timed_cancel(backend, tmp_path, calls):
    """Run each producer in one real child whose 5 ms timer races a 30 ms
    reply, so a cancel may land before or after its send. Stress only."""
    code = SETUP + """
    engine = tt.Engine(throttle=8, cache=False)
    token = object()
    base = sys.getrefcount(token)
    for kind in ("ctypes", "raw"):
        cancelled = 0
        for _ in range(int(os.environ["CALLS"])):
            stop = tt.CancelToken()
            threading.Timer(0.005, stop.cancel).start()
            try:
                engine.decide(late, Stream(3) if kind == "ctypes" else Raw(3, token), token=stop).value
            except tt.Cancelled:
                cancelled += 1
        assert cancelled == int(os.environ["CALLS"]), (kind, cancelled)
        ended = time.monotonic() + 10
        while (HELD or tt._thinkthen._live_workers()) and time.monotonic() < ended:
            time.sleep(0.01)
        print(kind, len(HELD), tt._thinkthen._live_workers(), sys.getrefcount(token) == base)
    """
    done = subprocess.run([sys.executable, "-c", textwrap.dedent(code)], capture_output=True,
                          text=True, timeout=15 if calls == 1 else 120, preexec_fn=no_core,
                          env=child_env(backend, tmp_path, "arm/delay/30", CALLS=str(calls)))
    assert done.returncode == 0, done.stderr
    assert done.stdout.splitlines() == ["ctypes 0 0 True", "raw 0 0 True"]


def test_one_cancelled_call_releases_each_producer(backend, tmp_path):
    """A real cancelled call releases both Python and raw Arrow batches."""
    released_after_cancel(backend, tmp_path, 1)


@pytest.mark.stress
def test_callers_that_leave_still_get_every_batch_released(backend, tmp_path):
    """Each of 200 calls per producer raises ``Cancelled`` while its worker
    waits on a held reply. Every worker then releases all its batches, so
    ``HELD`` empties and the token's references come back. Regression: a
    release without attaching aborts on the raw producer."""
    released_after_cancel(backend, tmp_path, 200)


FREEZE = SETUP + """
    trace = os.environ["TRACE"]
    tt._thinkthen._probe_trace(trace)
    def note(line, open=open):
        with open(trace, "a") as file:
            file.write(line + "\\n")
    import atexit
    atexit.register(note, "exit")
    class Slow:
        def __del__(self, sleep=time.sleep):
            sleep(0.6)
    engine = tt.Engine(throttle=1, cache=False)
    stop = tt.CancelToken()
    def go():
        sys.stdin.readline()
        stop.cancel()
    threading.Thread(target=go, daemon=True).start()
    source = Stream(1) if os.environ["KIND"] == "ctypes" else Raw(1, object())
    try:
        engine.decide(late, source, token=stop).value
    except tt.Cancelled:
        pass
    slow = Slow()
    with open(f"/proc/{os.environ['BACKEND']}/fd/0", "w") as backend:
        backend.write("release\\n")
    ended = time.perf_counter() + float(os.environ["DELAY"]) / 1000
    while time.perf_counter() < ended:
        pass
"""


def freeze_python():
    """The oldest Python 3.12 or later here. Spike 255 saw the freeze most
    often on the oldest Python and never on 3.14, so the test uses the one
    most able to catch it. The extension is abi3, so any of them loads it.
    With neither, it falls back to this Python, and every assert names it."""
    for name in ("python3.12", "python3.13"):
        found = shutil.which(name)
        if found:
            return found
    return sys.executable


class Staircase:
    """The child's busy wait, stepped toward the edge between a worker that
    wakes before the script ends and one that wakes after. The window sits at
    that edge, and where the edge lies depends on the machine."""

    def __init__(self):
        self.delay, self.lock = 4.0, threading.Lock()

    def next(self):
        with self.lock:
            return self.delay

    def saw(self, lines):
        early = "wake" in lines and "exit" in lines and lines.index("wake") < lines.index("exit")
        with self.lock:
            self.delay = min(8.0, max(0.0, self.delay + (-0.05 if early else 0.05)))


def one_run(number, python, folder, stairs):
    """One child: its outcome lines, and whether the run fell in the window."""
    backend = Backend()
    try:
        trace = folder / f"trace-{number}"
        env = child_env(backend, folder, "arm/held", TRACE=str(trace), BACKEND=str(backend.process.pid),
                        KIND=("ctypes", "raw")[number % 2], DELAY=str(stairs.next()),
                        PYTHONPATH=str(folder / "package"))
        child = subprocess.Popen([python, "-c", textwrap.dedent(FREEZE)], env=env, text=True,
                                 stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                 stderr=subprocess.PIPE, preexec_fn=no_core)
        assert backend.wait(1) == 1
        child.stdin.write("go\n")
        child.stdin.flush()
        _, error = child.communicate(timeout=20)
        assert child.returncode == 0, error
        lines = trace.read_text().split() if trace.exists() else []
    finally:
        backend.close()
    stairs.saw(lines)
    wake = lines.index("wake") if "wake" in lines else None
    outcome = next((n for n, line in enumerate(lines) if line in ("released", "leaked")), None)
    window = wake is not None and "exit" in lines and (
        outcome is None or wake < lines.index("exit") < outcome)
    return lines, window


def test_each_producer_exits_after_one_worker_wake(tmp_path):
    """One real child per producer reaches one release or leak and exits."""
    (tmp_path / "package").mkdir()
    (tmp_path / "package" / "thinkthen").symlink_to(PACKAGE)
    python, stairs = freeze_python(), Staircase()
    for kind in range(2):
        lines, _ = one_run(kind, python, tmp_path, stairs)
        assert lines.count("wake") == 1 and lines.count("done") == 1, (python, lines)
        assert len({"released", "leaked"} & set(lines)) == 1, (python, lines)


@pytest.mark.stress
def test_no_worker_freezes_at_exit(tmp_path):
    """Change 6's exit freeze: each child's worker wakes 0 to 8 ms around
    its script's end, stepped toward the edge where the window lies. Every ``wake`` must reach one outcome and ``done``.
    Runs go on, 8 at a time, until 100 fall in the exit window, at most
    1,000. Regression: without the gate a worker freezes after ``wake``."""
    (tmp_path / "package").mkdir()
    (tmp_path / "package" / "thinkthen").symlink_to(PACKAGE)
    python, stairs = freeze_python(), Staircase()
    windows, runs, outcomes = 0, 0, set()
    with ThreadPoolExecutor(8) as pool:
        while windows < 100 and runs < 1000:
            for lines, window in pool.map(lambda n: one_run(n, python, tmp_path, stairs),
                                          range(runs, runs + 40)):
                assert lines.count("wake") == 1 and lines.count("done") == 1, (python, lines)
                assert len({"released", "leaked"} & set(lines)) == 1, (python, lines)
                outcomes |= {"released", "leaked"} & set(lines)
                windows += window
            runs += 40
    print(f"{python}: {windows} window runs of {runs}; outcomes {sorted(outcomes)}")
    assert windows >= 100, f"{python}: only {windows} of {runs} runs fell in the exit window"
    assert outcomes == {"released", "leaked"}, (python, outcomes)


@pytest.mark.stress
def test_cancels_that_race_their_sends_still_get_every_batch_released(backend, tmp_path):
    """The timer form of ``test_callers_that_leave_still_get_every_batch_released``:
    a cancel may land before its send
    goes out. Every worker still releases all its batches (ticket 0352)."""
    released_after_timed_cancel(backend, tmp_path, 200)
