"""Cancel one installed SDK call on a held answer and prove the call ended.

Usage: cancel.py --surface NAME [--native] [--interrupt] -- ADAPTER [ARG ...]
ADAPTER is an SDK's stress adapter (scripts/stress-adapters.py states the contract).
--native passes the case as arguments, as sdk/c/tests/stress.py does. --interrupt
sends the console interrupt for an SDK whose cancel is its host's interrupt.
THINKTHEN_LOOPBACK names the prepared loopback executable.
"""
import argparse
import ctypes
from ctypes import wintypes
import json
import math
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time

from test_control import Capture, Fixture, arrived_or_exited

STARTUP_SECONDS = 120  # A cold JVM on a loaded machine starts before any deadline begins.
LIMIT_SECONDS = 15  # The shortest real-time limit a test may expect to pass.
CREATE_SUSPENDED, CREATE_NEW_PROCESS_GROUP = 0x4, 0x200
JOB_ACCOUNTING = 1


class Accounting(ctypes.Structure):
    _fields_ = [('times', ctypes.c_longlong * 4), ('faults', ctypes.c_ulong),
                ('total', ctypes.c_ulong), ('active', ctypes.c_ulong), ('ended', ctypes.c_ulong)]


class WindowsTree:
    """A job object holds the adapter from its first instruction, with every child it starts."""

    def __init__(self):
        self.kernel = ctypes.WinDLL('kernel32', use_last_error=True)
        self.kernel.CreateJobObjectW.restype = wintypes.HANDLE
        self.kernel.CreateJobObjectW.argtypes = [ctypes.c_void_p, wintypes.LPCWSTR]
        self.kernel.AssignProcessToJobObject.argtypes = [wintypes.HANDLE, wintypes.HANDLE]
        self.kernel.QueryInformationJobObject.argtypes = [wintypes.HANDLE, ctypes.c_int, ctypes.c_void_p,
                                                          wintypes.DWORD, ctypes.c_void_p]
        self.kernel.TerminateJobObject.argtypes = [wintypes.HANDLE, wintypes.UINT]
        self.kernel.CloseHandle.argtypes = [wintypes.HANDLE]
        self.job = self.checked(self.kernel.CreateJobObjectW(None, None))
        # An ssh session has no console for the interrupt to travel through.
        if not self.kernel.GetConsoleCP():
            self.checked(self.kernel.AllocConsole())

    @staticmethod
    def checked(result):
        if not result:
            raise ctypes.WinError(ctypes.get_last_error())
        return result

    def start(self, command, env):
        process = subprocess.Popen(command, env=env, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE,
                                   creationflags=CREATE_SUSPENDED | CREATE_NEW_PROCESS_GROUP)
        self.checked(self.kernel.AssignProcessToJobObject(self.job, int(process._handle)))
        ntdll = ctypes.WinDLL('ntdll')
        ntdll.NtResumeProcess.argtypes = [wintypes.HANDLE]
        if ntdll.NtResumeProcess(int(process._handle)):
            raise RuntimeError('adapter did not resume')
        return process

    def interrupt(self, process):
        process.send_signal(signal.CTRL_BREAK_EVENT)

    def left(self, process):
        info = Accounting()
        self.checked(self.kernel.QueryInformationJobObject(self.job, JOB_ACCOUNTING, ctypes.byref(info),
                                                           ctypes.sizeof(info), None))
        return info.active

    def close(self, process):
        self.kernel.TerminateJobObject(self.job, 1)
        self.kernel.CloseHandle(self.job)


class PosixTree:
    """A session holds the adapter and every child it starts."""

    def start(self, command, env):
        return subprocess.Popen(command, env=env, stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                                stderr=subprocess.PIPE, start_new_session=True)

    def interrupt(self, process):
        process.send_signal(signal.SIGINT)

    def left(self, process):
        try:
            os.killpg(process.pid, 0)
        except ProcessLookupError:
            return 0
        return 1

    def close(self, process):
        try:
            os.killpg(process.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass


def remaining(started):
    return max(0.001, LIMIT_SECONDS - (time.monotonic() - started))


def prove(surface, command, *, native, interrupt):
    table = json.loads(Path(__file__).with_name('stress.json').read_text(encoding='utf-8'))
    limits = json.loads(subprocess.check_output([os.environ['THINKTHEN_LOOPBACK'], '--limits'],
                                                text=True, timeout=30))
    scenario = next(case for case in table['scenarios'] if case['name'] == 'healthy')
    count, batch = table['records']['smoke'], table['client']['batch']
    fixture = Fixture(['--held', 'body'])
    tree = WindowsTree() if os.name == 'nt' else PosixTree()
    process = None
    try:
        with tempfile.TemporaryDirectory(prefix='thinkthen-cancel-') as directory:
            client = dict(table['client'], backend='systemone', url=fixture.url, api_key='loopback-only')
            client['retry'] = dict(client['retry'], attempt_timeout_ms=60000)
            case = dict(client=client, scenario=scenario, records=count, cancel=True, concurrency=1, rounds=1,
                        probability=table['replies'][0]['probability'], read_ahead_limit=None,
                        identifier_base=table['records']['identifier_base'])
            path = Path(directory) / 'case.json'
            path.write_text(json.dumps(case), encoding='utf-8')
            if native:
                command = [*command, 'stress-cancel', json.dumps(client), str(count), scenario['status'], '',
                           str(case['probability']), '1', '1', str(case['identifier_base'])]
            process = tree.start(command, dict(os.environ, THINKTHEN_STRESS_CASE=str(path)))
            output, errors = Capture(process.stdout), Capture(process.stderr)
            try:
                arrived_or_exited(fixture, process, min(math.ceil(count / batch), limits['workers']),
                                  timeout=STARTUP_SECONDS)
                before = int(fixture.command('count'))
                started = time.monotonic()
                if interrupt:
                    tree.interrupt(process)
                else:
                    process.stdin.write(b'cancel\n')
                    process.stdin.flush()
                status = process.wait(timeout=LIMIT_SECONDS)
                ended = time.monotonic() - started
                report = output.finish()
                if status:
                    raise RuntimeError(f'adapter exited {status}: {errors.finish()} {report}')
                summary = json.loads(report)
                if (summary['calls'], summary['rows'], summary['cancelled']) != (1, 0, True):
                    raise RuntimeError(f'adapter lost its cancellation: {summary}')
                if fixture.command('closed', timeout=remaining(started)) != 'closed':
                    raise RuntimeError('cancelled request remained open')
                requests = int(fixture.command('count'))
                if requests != before:
                    raise RuntimeError(f'requests continued after cancel: {before} -> {requests}')
                while tree.left(process):
                    if time.monotonic() - started > LIMIT_SECONDS:
                        raise RuntimeError('cancelled call left a process running')
                    time.sleep(0.05)
            except BaseException as error:
                raise RuntimeError(f'{surface} cancel failed: {error}\n{errors.data.decode(errors="replace")}') from error
            finally:
                tree.close(process)
                process.wait(timeout=120)
    finally:
        fixture.close()
    print(f'candidate Windows {surface}: cancelled held call ended in {ended:.3f}s; no process left', flush=True)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--surface', required=True)
    parser.add_argument('--native', action='store_true')
    parser.add_argument('--interrupt', action='store_true')
    parser.add_argument('adapter', nargs='+')
    arguments = parser.parse_args()
    prove(arguments.surface, arguments.adapter, native=arguments.native, interrupt=arguments.interrupt)


if __name__ == '__main__':
    sys.exit(main())
