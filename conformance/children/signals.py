"""Own Darwin signal observation outside the installed command artifact."""
from contextlib import contextmanager
import os
from pathlib import Path
import select
import subprocess
import sys
import time


class SignalObservation:
    def __init__(self, home, env):
        library = Path(home) / 'signal-observer.dylib'
        source = Path(__file__).with_name('signal_observer.c')
        subprocess.run(['cc', '-std=c11', '-Wall', '-Wextra', '-Werror', '-dynamiclib',
                        str(source), '-o', str(library)], env=env, check=True,
                       text=True)
        self.reader, self.writer = os.pipe()
        os.set_blocking(self.writer, False)
        self.env = {**env, 'DYLD_INSERT_LIBRARIES': str(library),
                    'CLI_SIGNAL_OBSERVER_FD': str(self.writer)}
        self.pass_fds = (self.writer,)

    def wait(self, child, signum, deadline):
        while True:
            assert child.poll() is None, 'signal ended the child before the held reply drained'
            remaining = deadline - time.monotonic()
            assert remaining > 0, 'native signal handler did not return'
            ready, _, _ = select.select([self.reader], [], [], min(remaining, 0.1))
            if ready:
                assert os.read(self.reader, 1) == bytes([signum]), 'unexpected signal acknowledgement'
                return

    def close(self):
        os.close(self.writer)
        os.close(self.reader)


@contextmanager
def observe_signals(home, env):
    observation = SignalObservation(home, env) if sys.platform == 'darwin' else None
    try:
        yield observation
    finally:
        if observation:
            observation.close()
