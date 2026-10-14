#!/usr/bin/env python3
"""Bound shared builds without weakening exclusive source and cache locks.

Keep the lock inode: shared flock conflicts with legacy exclusive flock. Linux
uses byte locks on that inode for admission; Darwin uses separate flock inodes.
Waiters release every lock before sleeping. Only the supervisor owns descriptors.
"""
import argparse
from contextlib import ExitStack
import ctypes
import fcntl
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time

PERMITS = 4
POLL = 0.05


def interrupted(signum, _frame):
    raise InterruptedError(signum)


def available(fd, operation):
    try:
        operation(fd)
        return True
    except BlockingIOError:
        return False


def byte_lock(fd, offset):
    fcntl.lockf(fd, fcntl.LOCK_EX | fcntl.LOCK_NB, 1, offset)


class Resources:
    def __init__(self, fd, directory, stack):
        self.fd = fd
        self.files = []
        if sys.platform == 'darwin':
            # Darwin's flock and fcntl locks interact, with different owners:
            # https://developer.apple.com/library/archive/documentation/System/Conceptual/ManPages_iPhoneOS/man3/lockf.3.html
            # Keep these inodes persistent, just like the canonical barrier.
            for name in [*(f'permit-{slot}' for slot in range(PERMITS)), 'admission']:
                self.files.append(stack.enter_context((directory / name).open('a+b')))

    def acquire(self, offset):
        if self.files:
            return available(self.files[offset].fileno(),
                             lambda fd: fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB))
        return available(self.fd, lambda fd: byte_lock(fd, offset))

    def release(self, offset):
        if self.files:
            fcntl.flock(self.files[offset].fileno(), fcntl.LOCK_UN)
        else:
            fcntl.lockf(self.fd, fcntl.LOCK_UN, 1, offset)


def high_waiters(directory):
    for path in directory.glob('wait-*'):
        try:
            with path.open('rb') as waiter:
                if not available(waiter.fileno(), lambda fd: fcntl.flock(fd, fcntl.LOCK_EX | fcntl.LOCK_NB)):
                    return True
                path.unlink(missing_ok=True)
        except FileNotFoundError:
            continue
    return False


def admit(fd, resources, priority, directory, deadline):
    while True:
        if deadline is not None and time.monotonic() >= deadline:
            raise TimeoutError('build admission timed out')
        shared = False
        admitted = False
        try:
            # Shared flock prevents overlap with an old exclusive supervisor.
            shared = available(fd, lambda f: fcntl.flock(f, fcntl.LOCK_SH | fcntl.LOCK_NB))
            if shared and resources.acquire(PERMITS):
                if os.getloadavg()[0] <= (os.cpu_count() or 1):
                    if priority == 'high' or not high_waiters(directory):
                        for slot in range(PERMITS):
                            if resources.acquire(slot):
                                # Never sleep while holding a permit.
                                if os.getloadavg()[0] <= (os.cpu_count() or 1):
                                    admitted = True
                                    return
                                break
        finally:
            if admitted:
                resources.release(PERMITS)
            else:
                for slot in range(PERMITS + 1):
                    resources.release(slot)
                if shared:
                    fcntl.flock(fd, fcntl.LOCK_UN)
        time.sleep(POLL)


def child_pids():
    return {int(pid) for task in Path('/proc/self/task').iterdir()
            for pid in (task / 'children').read_text().split()}


def reap_descendants(excluded, terminate=False):
    # The subreaper adopts this command's descendants even after setsid or
    # setpgrp. Killing a parent brings its children here on the next pass.
    while True:
        owned = child_pids() - excluded
        if not owned:
            return
        for pid in owned:
            if terminate:
                try:
                    os.kill(pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            try:
                os.waitpid(pid, os.WNOHANG)
            except ChildProcessError:
                pass
        if not terminate:
            return
        time.sleep(POLL)


def run_owned(command, env, deadline):
    if sys.platform == 'linux':
        libc = ctypes.CDLL(None, use_errno=True)
        if libc.prctl(36, 1, 0, 0, 0):  # PR_SET_CHILD_SUBREAPER
            raise OSError(ctypes.get_errno(), 'cannot own build descendants')
    existing = child_pids() if sys.platform == 'linux' else set()
    blocked = {signal.SIGTERM, signal.SIGINT, signal.SIGHUP}
    previous_mask = signal.pthread_sigmask(signal.SIG_BLOCK, blocked)
    child = None
    try:
        child = subprocess.Popen(command, env=env, start_new_session=True, close_fds=True,
                                 preexec_fn=lambda: signal.pthread_sigmask(signal.SIG_SETMASK, previous_mask))
        signal.pthread_sigmask(signal.SIG_SETMASK, previous_mask)
        while True:
            # Reap adopted descendants while their harness is alive too.
            if sys.platform == 'linux':
                reap_descendants(existing | {child.pid})
            if deadline is not None and time.monotonic() >= deadline:
                raise TimeoutError('build timed out')
            try:
                status = child.wait(timeout=POLL)
                return status if status >= 0 else 128 - status
            except subprocess.TimeoutExpired:
                continue
    finally:
        signal.pthread_sigmask(signal.SIG_BLOCK, {signal.SIGTERM, signal.SIGINT, signal.SIGHUP})
        if child is None:
            signal.pthread_sigmask(signal.SIG_SETMASK, previous_mask)
        else:
            # A successful compiler may still leave a daemon. Kill it before
            # releasing its permit, including descendants outside its group.
            if sys.platform == 'linux':
                child.kill()
            else:
                try:
                    os.killpg(child.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
            child.wait()
            if sys.platform == 'linux':
                reap_descendants(existing, terminate=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--lock', default=os.environ.get('THINKTHEN_HEAVY_LOCK', '/tmp/thinkthen-heavy.lock'))
    parser.add_argument('--priority', choices=('normal', 'high'), default='normal')
    parser.add_argument('--timeout', type=float, help='bound admission and command together')
    parser.add_argument('command', nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.command[1:] if args.command[:1] == ['--'] else args.command
    if not command:
        parser.error('a command is required')
    if args.timeout is not None and args.timeout <= 0:
        parser.error('timeout must be positive')
    for signum in (signal.SIGTERM, signal.SIGINT, signal.SIGHUP):
        signal.signal(signum, interrupted)
    deadline = time.monotonic() + args.timeout if args.timeout is not None else None
    directory = Path(args.lock + '.priority')
    directory.mkdir(mode=0o700, exist_ok=True)
    marker = None
    waiter = None
    try:
        if args.priority == 'high':
            waiter, marker = tempfile.mkstemp(prefix='.', dir=directory)
            fcntl.flock(waiter, fcntl.LOCK_EX)
            visible = directory / ('wait-' + Path(marker).name[1:])
            os.rename(marker, visible)
            marker = str(visible)
        with ExitStack() as stack:
            lock = stack.enter_context(open(args.lock, 'a+b'))
            resources = Resources(lock.fileno(), directory, stack)
            admit(lock.fileno(), resources, args.priority, directory, deadline)
            if marker:
                os.unlink(marker)
                marker = None
            env = dict(os.environ, THINKTHEN_HEAVY_LOCK=args.lock, THINKTHEN_HEAVY_LOCK_HELD=args.lock)
            return run_owned(command, env, deadline)
    except TimeoutError as error:
        print(error, file=sys.stderr)
        return 124
    except InterruptedError as error:
        return 128 + error.args[0]
    finally:
        if waiter is not None:
            os.close(waiter)
        if marker:
            os.unlink(marker)


if __name__ == '__main__':
    sys.exit(main())
