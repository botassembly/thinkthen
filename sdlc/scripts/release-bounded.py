#!/usr/bin/env python3
"""Local release tool capture: file output, deadlines and owned process containment."""
import locale
import os
from pathlib import Path
import signal
import subprocess
import tempfile
import time


def capture(args, *, env, timeout, input=None, text=False):
    tool = Path(args[0]).name if not isinstance(args, str) else args.split()[0]
    deadline = time.monotonic() + timeout
    job = None
    child = None
    # Files cannot fill a pipe or wait for a descendant to close an inherited writer.
    with tempfile.TemporaryFile() as stdout, tempfile.TemporaryFile() as stderr, tempfile.TemporaryFile() as stdin:
        if input is not None:
            stdin.write(input)
        stdin.seek(0)
        try:
            if os.name == 'nt':
                import runpy
                job = runpy.run_path(str(Path(__file__).with_name('release-owned-job.py')))['Job']()
            child = subprocess.Popen(args, env=env, stdin=stdin, stdout=stdout, stderr=stderr,
                start_new_session=os.name != 'nt', creationflags=4 if job else 0)
            if job:
                # The root is suspended: no compiler/setup descendant can escape assignment.
                job.start(child)
            try:
                child.wait(timeout=max(0, deadline - time.monotonic()))
            except subprocess.TimeoutExpired as error:
                raise ValueError(f'{tool} timed out after {timeout} seconds') from error
        finally:
            try:
                if job:
                    try:
                        job.close()
                    except (OSError, ValueError) as error:
                        raise ValueError(f'{tool} owned job cleanup failed: {error}') from error
                    # KILL_ON_JOB_CLOSE applies to this invocation's job only.
                elif child:
                    try:
                        os.killpg(child.pid, signal.SIGKILL)  # Our new session, never the caller's group.
                    except ProcessLookupError:
                        pass
            finally:
                if child:
                    if child.poll() is None:
                        child.kill()  # Owned Popen handle, including failed suspended setup.
                    try:
                        child.wait(timeout=5)
                    except subprocess.TimeoutExpired as error:
                        raise ValueError(f'{tool} owned child cleanup timed out') from error
        stdout.seek(0)
        stderr.seek(0)
        outputs = [stream.read() for stream in (stdout, stderr)]
        if text:
            outputs = [value.decode(locale.getpreferredencoding(False)).replace('\r\n', '\n').replace('\r', '\n')
                       for value in outputs]
        return subprocess.CompletedProcess(args, child.returncode, *outputs)
