"""The shared test helpers: one loopback backend per test, and each engine
call in a child Python whose environment holds a fake key beside that
loopback address and never the parent's key (amendment changes 5 and 14).
"""

import os
import pathlib
import subprocess
import sys
import textwrap

import pytest

FAKE = "sk-fake-loopback-python-0105"
REPO = pathlib.Path(__file__).resolve().parents[3]
TARGET = pathlib.Path(os.environ.get("CARGO_TARGET_DIR") or REPO / "target")
BINARY = TARGET / "debug" / "conformance-backend"


def pytest_sessionstart(session):
    """check.sh unsets the key first. A key here means that step is gone."""
    if "THINKTHEN_API_KEY" in os.environ:
        pytest.exit("THINKTHEN_API_KEY is set in the test process; check.sh unsets it first",
                    returncode=1)


class Backend:
    """The 0092 conformance backend, driven through its line protocol."""

    def __init__(self):
        self.process = subprocess.Popen([str(BINARY)], stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, text=True, bufsize=1)
        self.port = int(self.process.stdout.readline())

    def base(self, arm="generic"):
        return f"http://127.0.0.1:{self.port}/{arm}/v1"

    def say(self, line):
        self.process.stdin.write(line + "\n")
        self.process.stdin.flush()

    def count(self):
        self.say("count")
        return int(self.process.stdout.readline())

    def wait(self, least):
        """The count once it reads at least ``least``, or at 5 s."""
        self.say(f"wait {least}")
        line = self.process.stdout.readline()
        assert line.startswith("wait "), line
        return int(line.split()[1])

    def release(self):
        self.say("release")

    def close(self):
        if self.process.poll() is None:
            self.process.stdin.close()
            self.process.stdout.read()
            self.process.wait(timeout=10)


@pytest.fixture
def backend():
    started = Backend()
    yield started
    started.close()


def child_env(backend, folder, arm="generic", **extra):
    """The parent's environment, its key removed, then the fake key set
    beside this test's loopback backend and its own cache folder."""
    env = dict(os.environ)
    env.pop("THINKTHEN_API_KEY", None)
    env.update(THINKTHEN_API_KEY=FAKE, THINKTHEN_BASE_URL=backend.base(arm),
               THINKTHEN_CACHE=str(pathlib.Path(folder) / "cache"))
    env.update(extra)
    return env


def run(code, env, timeout=60):
    """Run ``code`` in a child Python and return what it printed."""
    done = subprocess.run([sys.executable, "-c", textwrap.dedent(code)], env=env,
                          capture_output=True, text=True, timeout=timeout)
    assert done.returncode == 0, done.stderr
    return done.stdout


def start(code, env):
    """Start ``code`` in a child Python with line pipes both ways."""
    return subprocess.Popen([sys.executable, "-c", textwrap.dedent(code)], env=env,
                            stdin=subprocess.PIPE, stdout=subprocess.PIPE,
                            stderr=subprocess.PIPE, text=True, bufsize=1)
