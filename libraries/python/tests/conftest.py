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


def pytest_configure(config):
    config.addinivalue_line("markers", "stress: opt-in load, timing, or repeated contention campaign")

FAKE = "sk-fake-loopback-python-0105"
REPO = pathlib.Path(__file__).resolve().parents[3]
TARGET = pathlib.Path(os.environ.get("CARGO_TARGET_DIR") or REPO / "target")
BINARY = TARGET / "debug" / ("conformance-backend.exe" if os.name == "nt" else "conformance-backend")
sys.path.insert(0, str(REPO / "conformance" / "children"))
from children import child_env as shared_clean_env  # noqa: E402  the shared helper, ticket 0127
from keys import question_keys  # noqa: E402,F401  re-exported for the tests


def clean_env(keep=(), **values):
    # Windows needs its system directory to load host DLLs. These defaults
    # point at the gate's owned scratch directories, never the user's folders.
    if os.name == "nt":
        keep = (*keep, "SystemRoot", "TEMP", "TMP", "APPDATA", "LOCALAPPDATA")
    return shared_clean_env(keep=keep, **values)


def pytest_sessionstart(session):
    """check.sh unsets the key first. A key here means that step is gone."""
    if "THINKTHEN_API_KEY" in os.environ:
        pytest.exit("THINKTHEN_API_KEY is set in the test process; check.sh unsets it first",
                    returncode=1)


class Backend:
    """The 0092 conformance backend, driven through its line protocol."""

    def __init__(self):
        self.process = subprocess.Popen([str(BINARY)], stdin=subprocess.PIPE, env=clean_env(),
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
        """The count once it reads at least ``least``, or at 30 s."""
        self.say(f"wait {least}")
        line = self.process.stdout.readline()
        assert line.startswith("wait "), line
        return int(line.split()[1])

    def release(self):
        self.say("release")

    def round(self):
        """Let go every reply held now; a later one holds again."""
        self.say("round")

    def close(self):
        if self.process.poll() is None:
            self.process.stdin.close()
            self.process.stdout.read()
            self.process.wait(timeout=60)


@pytest.fixture
def backend():
    started = Backend()
    yield started
    started.close()


def child_env(backend, folder, arm="generic", **extra):
    """Only PATH and the names set here: the fake key beside this test's
    loopback backend, its own cache folder, and the caller's extras."""
    return clean_env(THINKTHEN_API_KEY=FAKE, THINKTHEN_BASE_URL=backend.base(arm),
                     THINKTHEN_CACHE=str(pathlib.Path(folder) / "cache"), **extra)


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


def one_request(url, bodies, fixtures):
    """The portable fixture's questions, in order, as one request now that the
    content cut is gone, and each question's key."""
    import json
    first = json.loads(fixtures[0])
    questions = [question for fixture in fixtures
                 for _, question in sorted(json.loads(fixture)["questions"].items(),
                                           key=lambda item: int(item[0][1:]))]
    assert len(bodies) == 1, len(bodies)
    sent = json.loads(bodies[0])
    assert (sent["state"], sent["model"]) == (first["state"], first["model"])
    assert [sent["questions"][f"q{place}"] for place in range(1, len(sent["questions"]) + 1)] == questions
    return question_keys(url, bodies[0])
