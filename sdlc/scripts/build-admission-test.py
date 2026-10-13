#!/usr/bin/env python3
"""Real process regressions for shared compiler admission and ownership."""
import ctypes
import fcntl
import os
from pathlib import Path
import signal
import subprocess
import sys
import tempfile
import time
import unittest

HELPER = Path(__file__).with_name('build-admission.py')
# The launch boundary controls host load without an extra production setting.
LAUNCH = 'import os,runpy,sys; load=sys.argv[1]; os.getloadavg=lambda:(float(open(load).read()),0,0); sys.argv=sys.argv[2:]; runpy.run_path(sys.argv[0],run_name="__main__")'
COMPILER = 'import pathlib,sys,time; p=pathlib.Path(sys.argv[1]); p.with_suffix(".started").write_text("yes");\nwhile not p.exists(): time.sleep(.01)'


class Admission(unittest.TestCase):
    def setUp(self):
        if sys.platform == 'linux':
            self.assertEqual(ctypes.CDLL(None).prctl(36, 1, 0, 0, 0), 0)
        self.temporary = tempfile.TemporaryDirectory()
        self.root = Path(self.temporary.name)
        self.lock = self.root / 'lock'
        self.load = self.root / 'load'
        self.load.write_text('0')
        self.children = []
        self.descendants = []

    def tearDown(self):
        for child in self.children:
            if child.poll() is None:
                child.terminate()
            child.wait(timeout=2)
        for pid in self.descendants:
            try:
                os.kill(pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            try:
                os.waitpid(pid, 0)
            except ChildProcessError:
                pass
        self.temporary.cleanup()

    def launch(self, name, priority='normal', command=None, timeout=None):
        release = self.root / name
        args = [sys.executable, '-c', LAUNCH, str(self.load), str(HELPER), '--lock', str(self.lock), '--priority', priority]
        if timeout:
            args += ['--timeout', str(timeout)]
        args += ['--', *(command or [sys.executable, '-c', COMPILER, str(release)])]
        child = subprocess.Popen(args, env={'PATH': os.environ['PATH']}, stderr=subprocess.DEVNULL)
        self.children.append(child)
        return child

    def wait_for(self, predicate):
        deadline = time.monotonic() + 2
        while not predicate():
            if time.monotonic() >= deadline:
                self.fail('process did not reach expected state')
            time.sleep(.01)

    def started(self, name):
        return (self.root / (name + '.started')).exists()

    def release(self, name):
        (self.root / name).touch()

    def test_four_compilers_fifth_waits_and_priority_takes_next_permit(self):
        for index in range(4):
            self.launch(str(index))
        self.wait_for(lambda: all(self.started(str(i)) for i in range(4)))
        self.launch('normal')
        high = self.launch('high', 'high')
        self.wait_for(lambda: bool(list(Path(str(self.lock) + '.priority').glob('wait-*'))))
        time.sleep(.1)
        self.assertFalse(self.started('normal'))
        self.assertFalse(self.started('high'))
        self.release('0')
        self.wait_for(lambda: self.started('high'))
        self.assertFalse(self.started('normal'))
        self.release('high')
        self.assertEqual(high.wait(timeout=2), 0)
        self.wait_for(lambda: self.started('normal'))

    def test_load_wait_releases_every_lock_and_legacy_exclusion_remains(self):
        self.load.write_text(str((os.cpu_count() or 1) + 1))
        self.launch('waiting', 'high')
        self.wait_for(lambda: bool(list(Path(str(self.lock) + '.priority').glob('wait-*'))))
        time.sleep(.1)
        with self.lock.open('a+b') as legacy:
            fcntl.flock(legacy, fcntl.LOCK_EX | fcntl.LOCK_NB)
            self.load.write_text('0')
            time.sleep(.1)
            self.assertFalse(self.started('waiting'))
        self.wait_for(lambda: self.started('waiting'))

    def test_exit_timeout_and_termination_reap_children_and_release_permits(self):
        for mode in ('exit', 'timeout', 'terminate'):
            pidfile = self.root / (mode + '.pid')
            code = ('import subprocess,sys,time,pathlib; p=subprocess.Popen([sys.executable,"-c","import time; time.sleep(30)"]); '
                    'pathlib.Path(sys.argv[1]).write_text(str(p.pid)); ' + ('sys.exit(23)' if mode == 'exit' else 'time.sleep(30)'))
            child = self.launch(mode, command=[sys.executable, '-c', code, str(pidfile)], timeout=.4 if mode == 'timeout' else None)
            self.wait_for(pidfile.exists)
            if mode == 'terminate':
                child.send_signal(signal.SIGTERM)
            self.assertEqual(child.wait(timeout=2), {'exit': 23, 'timeout': 124, 'terminate': 143}[mode])
            with self.assertRaises(ProcessLookupError):
                os.kill(int(pidfile.read_text()), 0)
            with self.lock.open('a+b') as legacy:
                fcntl.flock(legacy, fcntl.LOCK_EX | fcntl.LOCK_NB)
                fcntl.lockf(legacy, fcntl.LOCK_EX | fcntl.LOCK_NB)

    @unittest.skipUnless(sys.platform == 'linux', 'Linux descendant ownership')
    def test_detached_descendants_are_reaped_before_permits_release(self):
        unrelated = subprocess.Popen([sys.executable, '-c', 'import time; time.sleep(30)'])
        self.children.append(unrelated)
        for mode in ('exit', 'timeout', 'terminate', 'interrupt', 'hangup'):
            with self.subTest(mode=mode):
                pidfile = self.root / (mode + '.detached')
                code = ('import subprocess,sys,time,pathlib; '
                        'p=subprocess.Popen([sys.executable,"-c","import time; time.sleep(30)"],start_new_session=True); '
                        'pathlib.Path(sys.argv[1]).write_text(str(p.pid)); ' +
                        ('sys.exit(23)' if mode == 'exit' else 'time.sleep(30)'))
                child = self.launch(mode, command=[sys.executable, '-c', code, str(pidfile)], timeout=.3 if mode == 'timeout' else None)
                self.wait_for(pidfile.exists)
                pid = int(pidfile.read_text())
                self.descendants.append(pid)
                signum = {'terminate': signal.SIGTERM, 'interrupt': signal.SIGINT, 'hangup': signal.SIGHUP}.get(mode)
                if signum:
                    child.send_signal(signum)
                self.assertEqual(child.wait(timeout=2), 128 + signum if signum else {'exit': 23, 'timeout': 124}[mode])
                with self.assertRaises(ProcessLookupError):
                    os.kill(pid, 0)
                self.assertIsNone(unrelated.poll())
                with self.lock.open('a+b') as legacy:
                    fcntl.flock(legacy, fcntl.LOCK_EX | fcntl.LOCK_NB)
                    fcntl.lockf(legacy, fcntl.LOCK_EX | fcntl.LOCK_NB)

    def test_child_inherits_original_signal_mask(self):
        code = 'import signal; assert signal.pthread_sigmask(signal.SIG_BLOCK,set()) == {signal.SIGUSR1}'
        previous = signal.pthread_sigmask(signal.SIG_BLOCK, {signal.SIGUSR1})
        try:
            child = self.launch('mask', command=[sys.executable, '-c', code])
        finally:
            signal.pthread_sigmask(signal.SIG_SETMASK, previous)
        self.assertEqual(child.wait(timeout=2), 0)

    def test_wait_timeout_removes_priority_and_child_inherits_no_lock_descriptor(self):
        with self.lock.open('a+b') as legacy:
            fcntl.flock(legacy, fcntl.LOCK_EX)
            child = self.launch('waiting', 'high', timeout=.2)
            self.assertEqual(child.wait(timeout=2), 124)
            self.assertEqual(list(Path(str(self.lock) + '.priority').glob('wait-*')), [])
        if sys.platform == 'linux':
            code = ('import os,pathlib,sys; lock=os.stat(sys.argv[1]); '
                    'fds=pathlib.Path("/proc/self/fd").iterdir(); '
                    '\nfor fd in fds:\n'
                    ' try: info=fd.stat()\n'
                    ' except FileNotFoundError: continue\n'
                    ' assert (info.st_dev,info.st_ino)!=(lock.st_dev,lock.st_ino)')
            child = self.launch('descriptor', command=[sys.executable, '-c', code, str(self.lock)])
            self.assertEqual(child.wait(timeout=2), 0)

    def test_compilers_keep_cache_mutation_lock_exclusive(self):
        cache = self.root / 'cache'
        code = ('import fcntl,pathlib,sys,time; f=open(sys.argv[1],"a+"); fcntl.flock(f,fcntl.LOCK_EX); '
                'p=pathlib.Path(sys.argv[2]); p.with_suffix(".started").touch(); '
                '\nwhile not p.exists(): time.sleep(.01)')
        for name in ('first', 'second'):
            self.launch(name, command=[sys.executable, '-c', code, str(cache), str(self.root / name)])
            if name == 'first':
                self.wait_for(lambda: self.started('first'))
        time.sleep(.1)
        self.assertFalse(self.started('second'))
        self.release('first')
        self.wait_for(lambda: self.started('second'))


if __name__ == '__main__':
    unittest.main()
