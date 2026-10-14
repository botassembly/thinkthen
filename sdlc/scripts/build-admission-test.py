#!/usr/bin/env python3
"""Real process regressions for shared compiler admission and ownership."""
import ctypes
from contextlib import ExitStack
import fcntl
import json
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

    def entry_scripts(self):
        scripts = self.root / 'scripts'
        scripts.mkdir()
        for name in ('heavy-lock', 'heavy-compile', 'allow-list'):
            (scripts / name).write_bytes(HELPER.with_name(name).read_bytes())
        # Reuse the controlled host-load boundary with the real supervisor.
        (scripts / 'build-admission.py').write_text(
            'import os,runpy,sys\n'
            'os.getloadavg=lambda:(float(open(' + repr(str(self.load)) + ').read()),0,0)\n'
            'runpy.run_path(' + repr(str(HELPER)) + ',run_name="__main__")\n')
        return scripts

    def launch(self, name, priority='normal', command=None, timeout=None, lock=None, hosted=False, context=None):
        release = self.root / name
        args = [sys.executable, '-c', LAUNCH, str(self.load), str(HELPER), '--lock', str(lock or self.lock), '--priority', priority]
        if timeout:
            args += ['--timeout', str(timeout)]
        args += ['--', *(command or [sys.executable, '-c', COMPILER, str(release)])]
        env = {'PATH': os.environ['PATH']} | (context or {})
        if hosted:
            env.update(GITHUB_ACTIONS='true', RUNNER_ENVIRONMENT='github-hosted')
        child = subprocess.Popen(args, env=env, stderr=subprocess.DEVNULL)
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

    def process_alive(self, pid):
        try:
            os.kill(pid, 0)
            return True
        except ProcessLookupError:
            return False

    def wait_entry(self, child, timeout=3):
        try:
            return child.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            # The caller owns this session. Its supervisors receive TERM and
            # finish their compiler cleanup before fixture locks disappear.
            os.killpg(child.pid, signal.SIGTERM)
            child.wait(timeout=2)
            def group_finished():
                if sys.platform == 'linux':
                    try:
                        while os.waitpid(-child.pid, os.WNOHANG)[0]:
                            pass
                    except ChildProcessError:
                        pass
                try:
                    os.killpg(child.pid, 0)
                    return False
                except ProcessLookupError:
                    return True
            self.wait_for(group_finished)
            raise

    def assert_resources_available(self):
        with ExitStack() as stack:
            legacy = stack.enter_context(self.lock.open('a+b'))
            fcntl.flock(legacy, fcntl.LOCK_EX | fcntl.LOCK_NB)
            if sys.platform == 'darwin':
                directory = Path(str(self.lock) + '.priority')
                for name in [*(f'permit-{slot}' for slot in range(4)), 'admission']:
                    resource = stack.enter_context((directory / name).open('a+b'))
                    fcntl.flock(resource, fcntl.LOCK_EX | fcntl.LOCK_NB)
            else:
                fcntl.lockf(legacy, fcntl.LOCK_EX | fcntl.LOCK_NB)

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

    def test_exclusive_checkpoint_blocks_all_permits_and_waits_for_all_compilers(self):
        with self.lock.open('a+b') as checkpoint:
            inode = self.lock.stat().st_ino
            fcntl.flock(checkpoint, fcntl.LOCK_EX)
            for index in range(4):
                self.launch(str(index))
            time.sleep(.1)
            self.assertFalse(any(self.started(str(i)) for i in range(4)))
        self.wait_for(lambda: all(self.started(str(i)) for i in range(4)))
        for index in range(4):
            with self.lock.open('a+b') as checkpoint:
                with self.assertRaises(BlockingIOError):
                    fcntl.flock(checkpoint, fcntl.LOCK_EX | fcntl.LOCK_NB)
            self.release(str(index))
            self.assertEqual(self.children[index].wait(timeout=2), 0)
        self.assert_resources_available()
        self.assertEqual(self.lock.stat().st_ino, inode)

    def test_symlink_lock_aliases_share_permits_priority_and_exclusive_barrier(self):
        self.lock.touch()
        alias = self.root / 'lock-alias'
        alias.symlink_to(self.lock)
        directory = self.root / 'directory-alias'
        directory.symlink_to(self.root, target_is_directory=True)
        locks = (self.lock, alias, directory / 'lock', directory / 'lock-alias')
        with self.lock.open('a+b') as checkpoint:
            inode = self.lock.stat().st_ino
            fcntl.flock(checkpoint, fcntl.LOCK_EX)
            for index, lock in enumerate(locks):
                self.launch(str(index), lock=lock)
            time.sleep(.1)
            self.assertFalse(any(self.started(str(i)) for i in range(4)))
        self.wait_for(lambda: all(self.started(str(i)) for i in range(4)))
        self.launch('alias-normal', lock=directory / 'lock')
        high = self.launch('alias-high', 'high', lock=alias)
        self.wait_for(lambda: bool(list(Path(str(self.lock.resolve()) + '.priority').glob('wait-*'))))
        time.sleep(.1)
        self.assertFalse(self.started('alias-normal'))
        self.assertFalse(self.started('alias-high'))
        with alias.open('a+b') as checkpoint:
            with self.assertRaises(BlockingIOError):
                fcntl.flock(checkpoint, fcntl.LOCK_EX | fcntl.LOCK_NB)
        self.release('0')
        self.wait_for(lambda: self.started('alias-high'))
        self.assertFalse(self.started('alias-normal'))
        self.release('alias-high')
        self.assertEqual(high.wait(timeout=2), 0)
        self.wait_for(lambda: self.started('alias-normal'))
        for name in ('1', '2', '3', 'alias-normal'):
            self.release(name)
        for child in self.children:
            self.assertEqual(child.wait(timeout=2), 0)
        self.assert_resources_available()
        self.assertEqual(self.lock.stat().st_ino, inode)

    def test_load_wait_releases_every_lock_and_legacy_exclusion_remains(self):
        self.load.write_text(str((os.cpu_count() or 1) + 1))
        self.launch('waiting', 'high')
        self.wait_for(lambda: bool(list(Path(str(self.lock) + '.priority').glob('wait-*'))))

        def resources_released():
            try:
                self.assert_resources_available()
                return True
            except BlockingIOError:
                # A waiter briefly probes admission before releasing to sleep.
                return False

        self.wait_for(resources_released)
        self.assertFalse(self.started('waiting'))
        with self.lock.open('a+b') as legacy:
            fcntl.flock(legacy, fcntl.LOCK_EX | fcntl.LOCK_NB)
            self.load.write_text('0')
            time.sleep(.1)
            self.assertFalse(self.started('waiting'))
        self.wait_for(lambda: self.started('waiting'))

    def test_hosted_high_load_keeps_four_permits_and_exclusive_barrier(self):
        self.load.write_text(str((os.cpu_count() or 1) + 1))
        with self.lock.open('a+b') as checkpoint:
            fcntl.flock(checkpoint, fcntl.LOCK_EX)
            for index in range(4):
                self.launch('hosted-' + str(index), hosted=True)
            time.sleep(.1)
            self.assertFalse(any(self.started('hosted-' + str(i)) for i in range(4)))
        self.wait_for(lambda: all(self.started('hosted-' + str(i)) for i in range(4)))
        self.launch('hosted-fifth', hosted=True)
        time.sleep(.1)
        self.assertFalse(self.started('hosted-fifth'))
        self.release('hosted-0')
        self.wait_for(lambda: self.started('hosted-fifth'))

    def test_incomplete_and_self_hosted_context_retains_load_cap(self):
        self.load.write_text(str((os.cpu_count() or 1) + 1))
        contexts = ({'GITHUB_ACTIONS': 'true'},
                    {'RUNNER_ENVIRONMENT': 'github-hosted'},
                    {'GITHUB_ACTIONS': 'true', 'RUNNER_ENVIRONMENT': 'self-hosted'})
        for index, context in enumerate(contexts):
            with self.subTest(context=context):
                name = 'local-' + str(index)
                child = self.launch(name, timeout=.2, context=context)
                self.assertEqual(child.wait(timeout=2), 124)
                self.assertFalse(self.started(name))
                self.assert_resources_available()

    def test_hosted_context_survives_rung_environment_isolation(self):
        self.load.write_text(str((os.cpu_count() or 1) + 1))
        scripts = self.entry_scripts()
        rung = scripts / 'rung'
        rung.write_text('set -eu\n. "$(dirname -- "$0")/heavy-lock"\n'
                        'test "$GITHUB_ACTIONS" = true\n'
                        'test "$RUNNER_ENVIRONMENT" = github-hosted\n'
                        'test "${PRIVATE_SERVICE_KEY+x}" != x\n')
        env = {'PATH': os.environ['PATH'], 'THINKTHEN_HEAVY_LOCK': str(self.lock),
               'GITHUB_ACTIONS': 'true', 'RUNNER_ENVIRONMENT': 'github-hosted',
               'PRIVATE_SERVICE_KEY': 'fake-rung-boundary'}
        child = subprocess.Popen(['/bin/sh', str(rung)], env=env, start_new_session=True)
        self.children.append(child)
        self.assertEqual(self.wait_entry(child), 0)
        self.assert_resources_available()

    def test_exit_timeout_and_termination_reap_children_and_release_permits(self):
        for mode in ('exit', 'timeout', 'terminate', 'interrupt', 'hangup'):
            pidfile = self.root / (mode + '.pid')
            code = ('import subprocess,sys,time,pathlib; p=subprocess.Popen([sys.executable,"-c","import time; time.sleep(30)"]); '
                    'pathlib.Path(sys.argv[1]).write_text(str(p.pid)); ' + ('sys.exit(23)' if mode == 'exit' else 'time.sleep(30)'))
            child = self.launch(mode, command=[sys.executable, '-c', code, str(pidfile)], timeout=.4 if mode == 'timeout' else None)
            self.wait_for(pidfile.exists)
            signum = {'terminate': signal.SIGTERM, 'interrupt': signal.SIGINT, 'hangup': signal.SIGHUP}.get(mode)
            if signum:
                child.send_signal(signum)
            self.assertEqual(child.wait(timeout=2), 128 + signum if signum else {'exit': 23, 'timeout': 124}[mode])
            self.wait_for(lambda: not self.process_alive(int(pidfile.read_text())))
            self.assert_resources_available()

    def test_command_launch_failure_releases_every_resource(self):
        child = self.launch('missing', command=[str(self.root / 'missing-command')])
        self.assertNotEqual(child.wait(timeout=2), 0)
        self.assert_resources_available()

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
                self.assert_resources_available()

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
            for signum in (None, signal.SIGTERM, signal.SIGINT, signal.SIGHUP):
                child = self.launch('waiting', 'high', timeout=.2 if signum is None else None)
                self.wait_for(lambda: bool(list(Path(str(self.lock) + '.priority').glob('wait-*'))))
                if signum:
                    child.send_signal(signum)
                self.assertEqual(child.wait(timeout=2), 128 + signum if signum else 124)
                self.assertEqual(list(Path(str(self.lock) + '.priority').glob('wait-*')), [])
        self.assert_resources_available()
        if sys.platform == 'linux':
            code = ('import os,pathlib,sys; lock=os.stat(sys.argv[1]); '
                    'fds=pathlib.Path("/proc/self/fd").iterdir(); '
                    '\nfor fd in fds:\n'
                    ' try: info=fd.stat()\n'
                    ' except FileNotFoundError: continue\n'
                    ' assert (info.st_dev,info.st_ino)!=(lock.st_dev,lock.st_ino)')
            child = self.launch('descriptor', command=[sys.executable, '-c', code, str(self.lock)])
            self.assertEqual(child.wait(timeout=2), 0)

    def test_library_wrapper_preserves_shell_arguments_and_owned_build_environment(self):
        # Cold compiler/SDK setup is fixture work, outside the shell deadline.
        # Use admission and a separate output; the timed caller still compiles.
        warmup = self.root / 'compiler-warmup'
        warmup.mkdir()
        source = warmup / 'probe.c'
        source.write_text('int main(void) { return 0; }\n')
        compiler = self.launch('compiler-warmup', command=[
            'cc', '-x', 'c', '-o', str(warmup / 'probe'), str(source)])
        self.assertEqual(compiler.wait(), 0, 'compiler fixture warmup failed')
        scripts = self.entry_scripts()
        caller = self.root / 'library-check.sh'
        output = self.root / 'output'
        caller.write_text("""#!/bin/bash
set -eu
unset THINKTHEN_API_KEY
heavy_scripts=$1
. "$heavy_scripts/heavy-compile"
mkdir -p "$CARGO_TARGET_DIR"
heavy_compile /bin/sh -c '
 test "$THINKTHEN_HEAVY_LOCK_HELD" = "$THINKTHEN_HEAVY_LOCK"
 test "${THINKTHEN_API_KEY+x}" != x
 exec cc "$@"
' compiler -x c -o "$CARGO_TARGET_DIR/probe" - <<'SOURCE'
int main(void) { return 0; }
SOURCE
"$CARGO_TARGET_DIR/probe"
python3 - "$2" "$3" <<'COMPILER'
import fcntl,json,os,pathlib,signal,sys
assert signal.pthread_sigmask(signal.SIG_BLOCK,set()) == set()
assert 'THINKTHEN_API_KEY' not in os.environ
assert 'THINKTHEN_HEAVY_LOCK_HELD' not in os.environ
with open(os.environ['THINKTHEN_HEAVY_LOCK'],'a+b') as lock:
 fcntl.flock(lock,fcntl.LOCK_EX | fcntl.LOCK_NB)
 if sys.platform != 'darwin': fcntl.lockf(lock,fcntl.LOCK_EX | fcntl.LOCK_NB)
names = ('CARGO_TARGET_DIR','THINKTHEN_TEST_PROFILE','THINKTHEN_ARTIFACT')
pathlib.Path(sys.argv[1]).write_text(json.dumps([sys.argv[2],*[os.environ[name] for name in names]]))
COMPILER
[[ -n "$BASH_VERSION" ]]
exit 23
""")
        env = {'PATH': os.environ['PATH'], 'THINKTHEN_HEAVY_LOCK': str(self.lock),
               'CARGO_TARGET_DIR': str(self.root / 'target'),
               'THINKTHEN_TEST_PROFILE': 'full', 'THINKTHEN_ARTIFACT': 'owned archive',
               'THINKTHEN_API_KEY': 'fake-library-boundary'}
        child = subprocess.Popen(['/bin/bash', str(caller), str(scripts), str(output), 'two words'], env=env, start_new_session=True)
        self.children.append(child)
        self.assertEqual(self.wait_entry(child), 23)
        self.assertTrue(os.access(self.root / 'target/probe', os.X_OK))
        self.assertEqual(json.loads(output.read_text()), ['two words', env['CARGO_TARGET_DIR'], 'full', 'owned archive'])
        # The existing standalone shell entry still passes through admission.
        self.assertEqual(subprocess.run(['/bin/sh', str(scripts / 'heavy-lock')], env=env, timeout=3).returncode, 0)
        rung = scripts / 'rung'
        rung.write_text('''set -eu
. "$(dirname -- "$0")/heavy-lock"
python3 - <<'BOUNDARY'
import os
assert 'THINKTHEN_API_KEY' not in os.environ
assert 'THINKTHEN_TEST_PROFILE' not in os.environ
assert 'THINKTHEN_ARTIFACT' not in os.environ
BOUNDARY
''')
        self.assertEqual(subprocess.run(['/bin/sh', str(rung)], env=env, timeout=3).returncode, 0)

    def test_shell_caller_timeout_cleans_owned_supervisor_and_compiler(self):
        scripts = self.entry_scripts()
        caller = self.root / 'timeout-check.sh'
        caller.write_text('set -eu\nheavy_scripts=$1\nshift\n. "$heavy_scripts/heavy-compile"\nheavy_compile "$@"\n')
        release = self.root / 'blocked'
        code = 'import os,pathlib,sys; pathlib.Path(sys.argv[1]+".pid").write_text(str(os.getpid())); ' + COMPILER
        env = {'PATH': os.environ['PATH'], 'THINKTHEN_HEAVY_LOCK': str(self.lock)}
        child = subprocess.Popen(['/bin/bash', str(caller), str(scripts), sys.executable, '-c', code, str(release)],
                                 env=env, start_new_session=True)
        self.children.append(child)
        self.wait_for(lambda: self.started('blocked'))
        pid = int((self.root / 'blocked.pid').read_text())
        self.descendants.append(pid)
        with self.assertRaises(subprocess.TimeoutExpired):
            self.wait_entry(child, timeout=.1)
        self.wait_for(lambda: not self.process_alive(pid))
        self.assert_resources_available()

    def test_rungs_ignore_inherited_library_mode_and_script_path(self):
        scripts = self.entry_scripts()
        hostile = self.root / 'hostile'
        hostile.mkdir()
        invoked = self.root / 'invoked'
        (hostile / 'allow-list').write_text('touch "' + str(invoked) + '"\n')
        (hostile / 'build-admission.py').write_text('raise AssertionError("hostile supervisor invoked")\n')
        rung = scripts / 'rung'
        rung.write_text('''set -eu
. "$(dirname -- "$0")/heavy-lock"
python3 - "$1" <<'RUNG'
import os,pathlib,sys,time
assert 'THINKTHEN_API_KEY' not in os.environ
assert 'PRIVATE_SERVICE_KEY' not in os.environ
assert 'heavy_library' not in os.environ
assert 'heavy_scripts' not in os.environ
assert os.environ['THINKTHEN_HEAVY_LOCK_HELD'] == os.environ['THINKTHEN_HEAVY_LOCK']
p = pathlib.Path(sys.argv[1])
p.with_suffix('.started').touch()
while not p.exists(): time.sleep(.01)
RUNG
''')
        env = {'PATH': os.environ['PATH'], 'THINKTHEN_HEAVY_LOCK': str(self.lock),
               'THINKTHEN_API_KEY': 'fake-rung-boundary', 'PRIVATE_SERVICE_KEY': 'fake-private-boundary',
               'heavy_library': '1', 'heavy_scripts': str(hostile)}
        for index in range(5):
            name = 'rung-' + str(index)
            child = subprocess.Popen(['/bin/sh', str(rung), str(self.root / name)], env=env)
            self.children.append(child)
            if index < 4:
                self.wait_for(lambda: self.started(name) or child.poll() is not None)
                self.assertTrue(self.started(name))
        time.sleep(.1)
        self.assertFalse(self.started('rung-4'))
        self.assertFalse(invoked.exists())
        self.release('rung-0')
        self.wait_for(lambda: self.started('rung-4'))
        for index in range(1, 5):
            self.release('rung-' + str(index))
        for child in self.children:
            self.assertEqual(child.wait(timeout=2), 0)
        self.assertFalse(invoked.exists())
        # An ordinary mode value must not select the inherited script path either.
        env['heavy_library'] = '0'
        child = subprocess.Popen(['/bin/sh', str(rung), str(self.root / 'ordinary')], env=env)
        self.children.append(child)
        self.wait_for(lambda: self.started('ordinary') or child.poll() is not None)
        self.assertTrue(self.started('ordinary'))
        self.release('ordinary')
        self.assertEqual(child.wait(timeout=2), 0)
        self.assertFalse(invoked.exists())

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
