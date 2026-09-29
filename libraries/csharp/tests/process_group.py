"""Run one recorded, owned process group; stop and reap it on timeout or interruption."""
import os
import signal
import subprocess
import time
from dataclasses import dataclass


@dataclass
class Result:
    exit: int | str
    actual_exit: int
    stdout: bytes
    stderr: bytes
    pid: int
    pgid: int
    signals: list[str]


def stop(process, signals):
    # start_new_session gives this child its own process group. Never signal by name.
    for kind in (signal.SIGTERM, signal.SIGKILL):
        try:
            os.killpg(process.pid, kind)
            signals.append(kind.name)
        except ProcessLookupError:
            pass
        if kind == signal.SIGTERM:
            time.sleep(0.2)
    # Do not reap the group leader before both signals: its PID cannot be reused yet.
    try:
        return process.communicate(timeout=5)
    except subprocess.TimeoutExpired:
        # This is an engine/OS cleanup failure, not permission to kill another group.
        raise RuntimeError(f"owned process group {process.pid} did not close its pipes")


def run(command, *, timeout, cwd=None, env=None, on_start=None, on_finish=None):
    process = subprocess.Popen(command, cwd=cwd, env=env, stdout=subprocess.PIPE,
                               stderr=subprocess.PIPE, start_new_session=True)
    signals = []
    try:
        if on_start:
            on_start(process.pid)
        stdout, stderr = process.communicate(timeout=timeout)
        result = Result(process.returncode, process.returncode, stdout, stderr,
                        process.pid, process.pid, signals)
    except subprocess.TimeoutExpired:
        stdout, stderr = stop(process, signals)
        result = Result('timeout', process.returncode, stdout, stderr,
                        process.pid, process.pid, signals)
    except BaseException:
        stdout, stderr = stop(process, signals)
        if on_finish:
            on_finish(Result('interrupted', process.returncode, stdout, stderr,
                             process.pid, process.pid, signals))
        raise
    if on_finish:
        on_finish(result)
    return result
