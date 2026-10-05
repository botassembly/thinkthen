#!/usr/bin/env python3
"""Release process lifecycle regressions; portable execution and mocked Windows source oracles."""
import contextlib
import ctypes
import importlib.util
import io
import os
from pathlib import Path
import runpy
import signal
import subprocess
import sys
import tempfile
import time
from unittest import mock

SCRIPTS = Path(__file__).resolve().parent


def load(name):
    spec = importlib.util.spec_from_file_location(name, SCRIPTS / (name + '.py'))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


BOUNDED = load('release-bounded')
C = load('release-windows-c')
MSVC = load('release-msvc')
SMOKE = load('release-windows-c-smoke')
FIXTURE = runpy.run_path(str(SCRIPTS / 'release-windows-c-fixture.py'))


def refused(action, cause):
    try:
        action()
    except ValueError as error:
        assert cause in str(error), (cause, str(error))
    else:
        raise AssertionError('plant passed: ' + cause)


def alive(pid):
    # Linux orphan zombies have exited; they cannot retain handles or run code.
    status = Path(f'/proc/{pid}/stat')
    if status.exists():
        try:
            return status.read_text().split(') ')[1].split()[0] != 'Z'
        except FileNotFoundError:
            return False
    try:
        os.kill(pid, 0)
        return True
    except ProcessLookupError:
        return False


def portable(root):
    environment = C.environment()
    with mock.patch.dict(os.environ, {'CL': 'planted', '_CL_': 'planted', 'LINK': 'planted',
                                      'FAKE_SERVICE_API_KEY': 'fake-only', 'THINKTHEN_BACKEND': 'planted'}):
        assert C.run([sys.executable, '-c',
            'import os; print(",".join(sorted(set(os.environ) & {"CL","_CL_","LINK","FAKE_SERVICE_API_KEY","THINKTHEN_BACKEND"})))']) == '\n'
    for caller in (C, MSVC):
        args = [sys.executable, '-c',
            'import os; os.write(1,b"a"*524288); os.write(2,b"b"*524288)']
        result = caller.CAPTURE(args, env=environment, timeout=10)
        assert result.returncode == 0 and result.stdout == b'a' * 524288 and result.stderr == b'b' * 524288
        result = caller.CAPTURE([sys.executable, '-c',
            'import os,sys; os.write(1,b"ok\\r\\n"); os.write(2,b"bad\\n"); sys.exit(23)'],
            env=environment, timeout=10, text=True)
        assert (result.returncode, result.stdout, result.stderr) == (23, 'ok\n', 'bad\n')
        result = caller.CAPTURE([sys.executable, '-c',
            'import sys; sys.stdout.buffer.write(sys.stdin.buffer.read())'],
            env=environment, timeout=10, input=b'\x00\xff\r\n')
        assert result.stdout == b'\x00\xff\r\n' and not result.stderr and result.returncode == 0
        invoke = (lambda command: C.run(command)) if caller is C else (lambda command: MSVC.run(command, environment))
        refused(lambda: invoke([sys.executable, '-c', 'import sys; sys.exit(23)']), 'exit 23')

    if os.name != 'posix':
        print('POSIX owned-session execution skipped; native Windows lifecycle proof remains separate')
        return
    sleeper = root / 'sleeper.py'
    sleeper.write_text('import time\ntime.sleep(90)\n')
    tool = root / 'inherited-output-tool.py'
    tool.write_text('import os, pathlib, subprocess, sys, time\n'
        f'child = subprocess.Popen([sys.executable, {str(sleeper)!r}])\n'
        'pathlib.Path(sys.argv[1]).write_text(str(os.getpid())+" "+str(child.pid))\n'
        'os.write(1,b"ready\\n"); os.write(2,b"ready-error\\n")\n'
        'time.sleep(90)\n')
    unrelated = subprocess.Popen([sys.executable, str(sleeper)], env=environment,
        stdin=subprocess.DEVNULL, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL, start_new_session=True)
    try:
        for caller in (C, MSVC, SMOKE):
            pids = root / ('c-pids' if caller is C else 'msvc-pids' if caller is MSVC else 'consumer-pids')
            # Exercise each real release wrapper, shortening only its declared deadline.
            def short(args, **options):
                assert options['timeout'] == (120 if caller is C else 60)
                if caller is not SMOKE:
                    assert options['env'] == environment
                    assert options.get('text', False) is (caller is MSVC)
                else:
                    assert 'text' not in options and options['input'].startswith(b'decide 3\n')
                    assert not any(name in options['env'] for name in ('INCLUDE', 'LIB', 'LIBPATH', 'CL', '_CL_', 'LINK'))
                return BOUNDED.capture(args, **(options | {'timeout': 1}))
            started = time.monotonic()
            try:
                if caller is SMOKE:
                    fixture_folder = root / 'archive'
                    fixture_folder.mkdir()
                    archive = FIXTURE['create'](fixture_folder, '0.2.0')
                    original = archive.read_bytes()
                    consumers = []
                    def compile_consumer(command):
                        executable = Path(next(arg[3:] for arg in command if arg.startswith('/Fe')))
                        consumers.append(executable)
                        executable.write_text(f'#!{sys.executable}\n' + tool.read_text().replace('sys.argv[1]', repr(str(pids))))
                        executable.chmod(0o755)
                    with mock.patch.object(SMOKE.C, 'run', side_effect=compile_consumer), mock.patch.object(SMOKE.C, 'inspect'):
                        with mock.patch.object(SMOKE.C, 'CAPTURE', short):
                            refused(lambda: SMOKE.consume(archive), 'consumer.exe timed out after 1 seconds')
                    assert archive.read_bytes() == original and consumers and not consumers[0].parent.exists()
                else:
                    with mock.patch.object(caller, 'CAPTURE', short):
                        invoke = (lambda: C.run([sys.executable, str(tool), str(pids)])) if caller is C else (
                            lambda: MSVC.run([sys.executable, str(tool), str(pids)], environment))
                        refused(invoke, Path(sys.executable).name + ' timed out after 1 seconds')
                assert time.monotonic() - started < 8, 'INHERITED_OUTPUT_TIMEOUT_NOT_PROMPT'
                root_pid, descendant_pid = map(int, pids.read_text().split())
                deadline = time.monotonic() + 3
                while alive(descendant_pid) and time.monotonic() < deadline:
                    time.sleep(0.02)
                assert not alive(root_pid), 'OWNED_TOOL_STILL_ALIVE'
                assert not alive(descendant_pid), 'OWNED_DESCENDANT_STILL_ALIVE'
                assert unrelated.poll() is None, 'UNRELATED_CHILD_KILLED'
            finally:
                if pids.exists():
                    # Only this test's recorded fresh session can be cleaned on a failed plant.
                    root_pid, _ = map(int, pids.read_text().split())
                    try:
                        os.killpg(root_pid, signal.SIGKILL)
                    except ProcessLookupError:
                        pass
        # A finished root with an inherited writer must also return without waiting for EOF.
        finished = root / 'finished-with-child.py'
        finished.write_text(tool.read_text().rsplit('time.sleep(90)', 1)[0] + 'sys.exit(23)\n')
        pids = root / 'finished-pids'
        started = time.monotonic()
        try:
            result = BOUNDED.capture([sys.executable, str(finished), str(pids)], env=environment, timeout=10)
            assert result.returncode == 23 and result.stdout == b'ready\n' and result.stderr == b'ready-error\n'
            assert time.monotonic() - started < 8, 'FINISHED_ROOT_WAITED_FOR_INHERITED_EOF'
            root_pid, descendant_pid = map(int, pids.read_text().split())
            deadline = time.monotonic() + 3
            while alive(descendant_pid) and time.monotonic() < deadline:
                time.sleep(0.02)
            assert not alive(descendant_pid), 'FINISHED_ROOT_LEFT_OWNED_DESCENDANT'
            assert unrelated.poll() is None, 'UNRELATED_CHILD_KILLED'
        finally:
            if pids.exists():
                try:
                    os.killpg(int(pids.read_text().split()[0]), signal.SIGKILL)
                except ProcessLookupError:
                    pass
    finally:
        unrelated.kill()
        unrelated.wait(timeout=5)


def windows_source_oracles():
    native = load('release-owned-job')
    events = []
    class Kernel:
        def CreateJobObjectW(self, *_):
            events.append('create-job')
            return 91
        def SetInformationJobObject(self, handle, kind, pointer, length):
            assert handle == 91 and kind == 9 and length == ctypes.sizeof(native.Extended)
            assert pointer._obj.BasicLimitInformation.LimitFlags == 0x2000
            events.append('kill-on-close')
            return 1
        def AssignProcessToJobObject(self, job, process):
            assert (job, process) == (91, 71)
            events.append('assign-owned-handle')
            return 1
        def CreateToolhelp32Snapshot(self, flags, pid):
            assert (flags, pid) == (4, 0)
            return 92
        def Thread32First(self, snapshot, pointer):
            assert snapshot == 92
            pointer._obj.th32OwnerProcessID, pointer._obj.th32ThreadID = 51, 61
            return 1
        def OpenThread(self, rights, inherit, thread):
            assert (rights, inherit, thread) == (0x0802, False, 61)
            return 93
        def GetProcessIdOfThread(self, thread):
            assert thread == 93
            events.append('verify-thread-owner')
            return 51
        def ResumeThread(self, thread):
            assert 'assign-owned-handle' in events, 'WINDOWS_RESUMED_BEFORE_OWNED_ASSIGNMENT'
            assert thread == 93 and events.index('assign-owned-handle') < events.index('verify-thread-owner')
            events.append('resume')
            return 1
        def TerminateJobObject(self, job, code):
            assert (job, code) == (91, 1)
            events.append('terminate-owned-job')
            return 1
        def QueryInformationJobObject(self, job, kind, pointer, length, returned):
            assert (job, kind, length, returned) == (91, 1, ctypes.sizeof(native.Accounting), None)
            events.append('observe-job-empty')
            pointer._obj.ActiveProcesses = 0
            return 1
        def CloseHandle(self, handle):
            events.append(('close', handle))
            return 1
    kernel = Kernel()
    child = mock.Mock(pid=51, _handle=71)
    with mock.patch.object(native, 'api', return_value=kernel):
        job = native.Job()
        job.start(child)
        job.close()
        assert events.index('assign-owned-handle') < events.index('resume')
        assert 'terminate-owned-job' in events, 'WINDOWS_OWNED_JOB_NOT_TERMINATED'
        assert events.index('terminate-owned-job') < events.index('observe-job-empty') < events.index(('close', 91))
        assert ('close', 92) in events and ('close', 93) in events
        events.clear()
        job = native.Job()
        with mock.patch.object(kernel, 'GetProcessIdOfThread', return_value=999):
            refused(lambda: job.start(child), 'thread identity changed')
        job.close()
        assert 'resume' not in events and ('close', 92) in events and ('close', 93) in events
        events.clear()
        job = native.Job()
        with mock.patch.object(kernel, 'QueryInformationJobObject', side_effect=lambda *args: 1):
            with mock.patch.object(native, 'Accounting', return_value=mock.Mock(ActiveProcesses=1)):
                with mock.patch.object(native.ctypes, 'byref', return_value=None), mock.patch.object(native.ctypes, 'sizeof', return_value=48):
                    with mock.patch.object(native.time, 'monotonic', side_effect=[0, 6]):
                        refused(job.close, 'owned job cleanup timed out')
        assert ('close', 91) in events
    # Run the actual capture branch against mocks, proving suspended creation and cleanup
    # on assignment failure. This executes Python control flow, not Windows APIs.
    events.clear()
    job = mock.Mock()
    job.start.side_effect = ValueError('job assignment refused')
    fake_child = mock.Mock()
    fake_child.poll.return_value = None
    with mock.patch.object(BOUNDED.os, 'name', 'nt'), mock.patch.object(BOUNDED, 'Path', type(SCRIPTS)):
        with mock.patch.object(runpy, 'run_path', return_value={'Job': lambda: job}):
            with mock.patch.object(subprocess, 'Popen', return_value=fake_child) as spawn:
                refused(lambda: BOUNDED.capture(['cl.exe'], env={}, timeout=1), 'job assignment refused')
    assert spawn.call_args.kwargs['creationflags'] == 4 and spawn.call_args.kwargs['start_new_session'] is False
    job.close.assert_called_once()
    fake_child.kill.assert_called_once()
    fake_child.wait.assert_called_once_with(timeout=5)


def main():
    with tempfile.TemporaryDirectory(prefix='thinkthen-release-timeout-') as temporary:
        portable(Path(temporary))
    windows_source_oracles()
    posix = 'POSIX inherited-output/owned cleanup passed' if os.name == 'posix' else 'POSIX cleanup execution skipped'
    print(f'Release process lifecycle: byte/success/nonzero/filled-output passed; {posix}; Windows source oracles passed, native Windows full proof pending')


if __name__ == '__main__':
    main()
