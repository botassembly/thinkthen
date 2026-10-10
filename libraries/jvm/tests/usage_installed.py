#!/usr/bin/env python3
"""Observe real installed JVM persistence under an owned operating-system lock."""
import argparse
import fcntl
import importlib.util
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time

ROOT = Path(__file__).resolve().parents[3]
sys.path.insert(0, str(ROOT / 'conformance/children'))
from children import child_env
from toolchains import JDK, KOTLIN, SCALA


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--jars', type=Path, required=True)
    parser.add_argument('--out', type=Path, required=True)
    args = parser.parse_args()
    args.out.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix='usage-', dir=args.out) as scratch:
        work = Path(scratch)
        feed, app = work / 'feed', work / 'app'
        feed.mkdir(); app.mkdir()
        for jar in args.jars.glob('*.jar'): shutil.copyfile(jar, feed / jar.name)
        cp = os.pathsep.join([*(str(jar) for jar in sorted(feed.glob('*.jar'))), str(app), str(KOTLIN / 'lib/kotlin-stdlib.jar'), str(KOTLIN / 'lib/kotlinx-coroutines-core-jvm.jar'), str(SCALA / 'lib/scala.jar')])
        env = child_env(JAVA_HOME=str(JDK), JAVACMD=str(JDK / 'bin/java'))
        for extension in ('java', 'kt', 'scala'):
            shutil.copyfile(ROOT / 'libraries/jvm/tests' / ('UsageStatus.' + extension), work / ('UsageStatus.' + extension))
        subprocess.run([str(JDK / 'bin/javac'), '--release', '22', '-cp', cp, '-d', str(app), str(work / 'UsageStatus.java')], env=env, check=True)
        subprocess.run([str(KOTLIN / 'bin/kotlinc'), '-J-Xmx1g', '-jvm-target', '22', '-classpath', cp, str(work / 'UsageStatus.kt'), '-d', str(app)], env=env, check=True)
        subprocess.run([str(SCALA / 'bin/scalac'), '-J-Xmx1g', '-classpath', cp, '-d', str(app), str(work / 'UsageStatus.scala')], env=env, check=True)
        spec = importlib.util.spec_from_file_location('usage_backend', ROOT / 'libraries/csharp/tests/backend.py')
        backend = importlib.util.module_from_spec(spec); spec.loader.exec_module(backend)
        server = backend.Backend(work)
        expected = []
        try:
            for language, mainclass in [('java', 'UsageStatus'), ('kotlin', 'UsageStatusKt'), ('scala', 'ScalaUsageStatus')]:
                for mode in ('written', 'failed', 'disabled'):
                    home = work / (language + '-' + mode)
                    home.mkdir()
                    usage = home / 'state/thinkthen'
                    usage.mkdir(parents=True, mode=0o700)
                    with (usage / '.lock').open('w') as lock:
                        os.chmod(lock.name, 0o600)
                        fcntl.flock(lock, fcntl.LOCK_EX)
                        env = child_env(home=home, THINKTHEN_BASE_URL=f'http://127.0.0.1:{server.server_port}/generic/v1', THINKTHEN_API_KEY='tt-canary-290')
                        if mode == 'disabled':
                            env.update(HOME='', XDG_STATE_HOME='', APPDATA='', LOCALAPPDATA='')
                        command = [str(JDK / 'bin/java'), '--enable-native-access=ALL-UNNAMED', '-Xmx1g', '-XX:ActiveProcessorCount=2', '-cp', cp, mainclass, mode, str(home)]
                        child = subprocess.Popen(command, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
                        try:
                            if mode != 'disabled':
                                end = time.monotonic() + 15
                                while not (home / 'pending').exists() and child.poll() is None and time.monotonic() < end: time.sleep(0.01)
                                if not (home / 'pending').exists():
                                    child.kill()
                                    raise AssertionError((language, mode, 'consumer did not observe Pending', child.communicate()))
                                if mode == 'written':
                                    fcntl.flock(lock, fcntl.LOCK_UN)
                                    (home / 'released').touch()
                            stdout, stderr = child.communicate(timeout=15)
                            assert child.returncode == 0, (language, mode, stdout, stderr)
                        finally:
                            if child.poll() is None: child.kill(); child.communicate()
                    if mode != 'disabled': expected.append('usage-' + language + '-' + mode)
                    assert server.arrivals == expected, server.arrivals
                    print(language, mode, 'PASS')
            assert server.attempts == server.connections == len(expected) == 6
            print('Installed usage states, retained facts/advice and exact requests PASS')
        finally:
            server.close()


if __name__ == '__main__':
    main()
