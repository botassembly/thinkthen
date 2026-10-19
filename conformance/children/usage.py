"""Run installed persistence consumers with one owned usage lock and handshake."""
from contextlib import contextmanager
import fcntl
from pathlib import Path
import selectors
import subprocess
import time

from children import FIXTURE_HANG_TIMEOUT, fixture_run, fixture_communicate


@contextmanager
def usage_lock(state, *, held):
    """Own the same private state folder and OS lock as the native engine."""
    state = Path(state)
    state.mkdir(parents=True, mode=0o700, exist_ok=True)
    with (state / '.lock').open('w') as lock:
        Path(lock.name).chmod(0o600)
        if held:
            fcntl.flock(lock, fcntl.LOCK_EX)
        yield lock


def run_usage(command, *, state, mode, env, cwd=None,
              pending=None, execute=None, text=True):
    """Keep lock ownership through exit; release Written only after Pending.

    A consumer signals Pending either on stdout or in its owned home. The
    program retains its own status/answer assertions. Disabled environment
    choices stay explicit at each public caller, including sandbox paths.
    """
    with usage_lock(state, held=mode == 'failed' or pending is not None) as lock:
        if execute is not None:
            assert pending is None
            return execute(command, env=env)
        if pending is None:
            return fixture_run(command, cwd=cwd, env=env, capture_output=True, text=text)
        child = subprocess.Popen(command, cwd=cwd, env=env, stdin=subprocess.PIPE,
                                 stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=text)
        prefix = '' if text else b''
        try:
            if mode != 'disabled':
                if pending == 'stdout':
                    with selectors.DefaultSelector() as ready:
                        ready.register(child.stdout, selectors.EVENT_READ)
                        assert ready.select(FIXTURE_HANG_TIMEOUT), 'consumer did not observe Pending'
                    prefix = child.stdout.readline()
                    assert prefix.strip() == ('PENDING' if text else b'PENDING'), (prefix, child.poll())
                else:
                    marker = Path(pending) / 'pending'
                    end = time.monotonic() + FIXTURE_HANG_TIMEOUT
                    while not marker.exists() and child.poll() is None and time.monotonic() < end:
                        time.sleep(.01)
                    assert marker.exists(), 'consumer did not observe Pending'
                if mode == 'written':
                    fcntl.flock(lock, fcntl.LOCK_UN)
                if pending == 'stdout':
                    child.stdin.write('continue\n' if text else b'continue\n'); child.stdin.flush()
                elif mode == 'written':
                    (Path(pending) / 'released').touch()
            stdout, stderr = fixture_communicate(child)
            return subprocess.CompletedProcess(command, child.returncode, prefix + stdout, stderr)
        finally:
            if child.poll() is None:
                child.kill(); child.communicate()
