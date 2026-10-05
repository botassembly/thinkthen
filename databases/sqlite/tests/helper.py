"""The shared test helper: one loopback backend per test, and one child
process per test with its own cache, configuration, and address.

A child never sees the caller's `THINKTHEN_API_KEY`. It gets the fixed
loopback key beside its own loopback address (ticket 0109 decision 16).
The engine refuses to send with no key, so a loopback key is what lets a
child count sends at all. Only the secrecy test sets another key.
"""

from __future__ import annotations

import json
import os
import pathlib
import queue
import subprocess
import sys
import tempfile
import threading
import traceback

HERE = pathlib.Path(__file__).resolve().parent
ROOT = HERE.parent
sys.path.insert(0, str(ROOT.parents[1] / "conformance" / "children"))
from children import child_env  # noqa: E402  the shared helper, ticket 0127
# One build-folder rule, as check.sh reads it: CARGO_TARGET_DIR, or each workspace's own target.
BUILT = pathlib.Path(os.environ.get("CARGO_TARGET_DIR", ROOT / "target"))
SUFFIX = "dylib" if sys.platform == "darwin" else "so"
LIB = pathlib.Path(os.environ.get("THINKTHEN_SQLITE_EXTENSION", BUILT / "release" / f"libthinkthen0.{SUFFIX}"))
BACKEND = os.environ.get("THINKTHEN_BACKEND", str(pathlib.Path(os.environ.get("CARGO_TARGET_DIR", ROOT.parents[1] / "target")) / "debug" / "conformance-backend"))
CLI = os.environ.get("THINKTHEN_SQLITE_CLI", str(pathlib.Path.home() / ".cache/thinkthen-toolchains/sqlite-3500000-host/sqlite3"))
LOOPBACK_KEY = "sk-sqlite-loopback"
LIVE: list[subprocess.Popen] = []

# What every child runs first: a connection with the extension loaded, and
# helpers that turn a statement into rows or its error text.
PRELUDE = f"""
import json, os, sqlite3, sys, threading, time
LIB = {str(LIB)!r}
def connect(path=":memory:"):
    connection = sqlite3.connect(path, isolation_level=None, check_same_thread=False)
    connection.enable_load_extension(True)
    connection.load_extension(LIB)
    return connection
def run(connection, sql, parameters=()):
    try:
        return connection.execute(sql, parameters).fetchall()
    except sqlite3.Error as failure:
        return str(failure)
def say(**fields):
    print(json.dumps(fields), flush=True)
"""


class Backend:
    """One conformance backend process, driven over its standard input."""

    def __init__(self, markers=None) -> None:
        self.process = subprocess.Popen(
            [BACKEND], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.DEVNULL, text=True,
            env=child_env(**({"THINKTHEN_TEST_MARKERS":json.dumps(markers)} if markers else {})),
        )
        LIVE.append(self.process)
        self.port = int(self._line())
        self.counts: queue.Queue[str] = queue.Queue()
        self.waits: queue.Queue[str] = queue.Queue()
        threading.Thread(target=self._read, daemon=True).start()

    def _line(self) -> str:
        assert self.process.stdout is not None
        return self.process.stdout.readline().strip()

    def _read(self) -> None:
        while line := self._line():
            (self.waits if line.startswith("wait ") else self.counts).put(line)

    def _say(self, line: str) -> None:
        assert self.process.stdin is not None
        self.process.stdin.write(line + "\n")
        self.process.stdin.flush()

    def base(self, arm: str = "generic") -> str:
        return f"http://127.0.0.1:{self.port}/{arm}/v1"

    def count(self) -> int:
        self._say("count")
        return int(self.counts.get(timeout=30))

    def snapshot(self, command):
        self._say(command)
        return json.loads(self.counts.get(timeout=30))

    def capture(self) -> list[str]:
        """Exact bodies from the bounded, opted-in recognize case arm."""
        self._say("capture")
        held = json.loads(self.counts.get(timeout=30))
        if "error" in held:
            raise AssertionError(held["error"])
        return held["bodies"]

    def wait(self, least: int) -> int:
        """The count once it reads at least `least`, or at 30 s."""
        self._say(f"wait {least}")
        return int(self.waits.get(timeout=60).split()[1])

    def release(self) -> None:
        self._say("release")

    def round(self) -> None:
        self._say("round")

    def close(self) -> int:
        assert self.process.stdin is not None
        self.process.stdin.close()
        self.process.wait(timeout=10)
        return int(self.counts.get(timeout=30))


def environment(backend: Backend | None, arm: str = "generic", **extra: str) -> dict[str, str]:
    """A child's whole environment: PATH, the pinned host's library path, and
    fresh folders. The loopback key rides only beside a loopback address."""
    scratch = pathlib.Path(tempfile.mkdtemp(prefix="thinkthen-sqlite-"))
    held = child_env(
        keep=("LD_LIBRARY_PATH",),
        THINKTHEN_CACHE=str(scratch / "cache"),
        XDG_CACHE_HOME=str(scratch / "xdg-cache"),
        XDG_STATE_HOME=str(scratch / "xdg-state"),
        XDG_CONFIG_HOME=str(scratch / "xdg-config"),
        SCRATCH=str(scratch),
    )
    if backend is not None:
        held.update(THINKTHEN_BASE_URL=backend.base(arm), THINKTHEN_API_KEY=LOOPBACK_KEY)
    held.update(extra)
    return held


class Child:
    """One test child: the prelude plus `code`, fed lines on standard input."""

    def __init__(self, code: str, env: dict[str, str], program: list[str] | None = None) -> None:
        self.process = subprocess.Popen(
            program or [sys.executable, "-c", PRELUDE + code],
            env=env,
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
        )
        LIVE.append(self.process)
        self.lines: queue.Queue[str] = queue.Queue()
        self.said: list[str] = []
        self.reader = threading.Thread(target=self._read, daemon=True)
        self.reader.start()

    def _read(self) -> None:
        assert self.process.stdout is not None
        for line in self.process.stdout:
            self.said.append(line)
            self.lines.put(line)

    def send(self, line: str = "go") -> None:
        assert self.process.stdin is not None
        self.process.stdin.write(line + "\n")
        self.process.stdin.flush()

    def read(self, timeout: float = 30) -> dict:
        """The child's next JSON line, while it runs on."""
        while True:
            try:
                line = self.lines.get(timeout=timeout)
            except queue.Empty:
                raise AssertionError(f"the child said nothing in {timeout} s") from None
            if line.startswith("{"):
                return json.loads(line)

    def result(self, timeout: float = 60) -> dict:
        """End the child's input, wait for it, and return its last JSON line."""
        assert self.process.stdin is not None and self.process.stderr is not None
        try:
            self.process.stdin.close()
            self.process.wait(timeout=timeout)
        except subprocess.TimeoutExpired:
            self.process.kill()
            raise AssertionError(f"the child ran past {timeout} s") from None
        self.reader.join(timeout=5)
        lines = [line for line in self.said if line.startswith("{")]
        if not lines:
            err = self.process.stderr.read().strip()
            raise AssertionError(f"the child printed no result (exit {self.process.returncode}): {err}")
        return json.loads(lines[-1])


def child(code: str, env: dict[str, str], timeout: float = 60) -> dict:
    """Run one child to its end and return its result."""
    return Child(code, env).result(timeout)


class NotRun(Exception):
    """A test that could not run here; `main` reports it apart from a pass."""


def expect(held: object, wanted: object, what: str) -> None:
    """Fail with both values when they differ."""
    if held != wanted:
        raise AssertionError(f"{what}: held {held!r}, wanted {wanted!r}")


def main(tests: dict) -> int:
    """Run every `test_` function of a module and report each by name."""
    failed = 0
    for name, test in list(tests.items()):
        if not name.startswith("test_") or not callable(test):
            continue
        try:
            test()
            print(f"ok       {name}")
        except NotRun as reason:
            print(f"not run  {name}: {reason}")
        except Exception:  # noqa: BLE001 (a test's failure of any kind is reported)
            failed += 1
            print(f"FAILED   {name}\n{traceback.format_exc()}")
        while LIVE:
            LIVE.pop().kill()
    return 1 if failed else 0
