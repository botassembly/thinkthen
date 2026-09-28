"""Shared pieces of the DuckDB suites: one loopback backend and one child
process per test, a fresh cache and XDG folders per child, and a small case
runner that exits nonzero on any failure.

Every child runs the venv's `duckdb` module with a fake key beside a
loopback `THINKTHEN_BASE_URL`, so no request can leave the machine. The
suites read the backend's own `count` line and never trust a dry run.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
import time
import traceback
from pathlib import Path
from urllib.parse import urlsplit

HERE = Path(__file__).resolve().parent
ROOT = HERE.parent
REPO_ROOT = ROOT.parent.parent
sys.path.insert(0, str(REPO_ROOT / "conformance" / "children"))
from children import child_env as clean_env  # noqa: E402  the shared helper, ticket 0127
EXTENSION = Path(os.environ.get("THINKTHEN_DUCKDB_EXTENSION", ROOT / "build" / "thinkthen.duckdb_extension"))
BACKEND = os.environ.get("THINKTHEN_BACKEND_BIN", "")
FAKE_KEY = "sk-loopback-duckdb-suite"


class Backend:
    """One loopback backend, driven from one writer."""

    def __init__(self) -> None:
        if not BACKEND:
            raise SystemExit("harness: THINKTHEN_BACKEND_BIN names no loopback backend")
        self.process = subprocess.Popen(
            [BACKEND], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, bufsize=1, env=clean_env()
        )
        self.port = int(self._line())

    def _line(self) -> str:
        line = self.process.stdout.readline()
        if not line:
            raise RuntimeError("the loopback backend closed its output")
        return line.strip()

    def base(self, arm: str = "generic") -> str:
        return f"http://127.0.0.1:{self.port}/{arm}/v1"

    def _say(self, line: str) -> None:
        self.process.stdin.write(line + "\n")
        self.process.stdin.flush()

    def count(self) -> int:
        self._say("count")
        return int(self._line())

    def capture(self) -> list[str]:
        """Bodies retained by the shared backend's bounded opt-in case arm."""
        self._say("capture")
        held = json.loads(self._line())
        if "error" in held:
            raise AssertionError(held["error"])
        return held["bodies"]

    def wait(self, least: int) -> int:
        self._say(f"wait {least}")
        while True:
            line = self._line()
            if line.startswith("wait "):
                return int(line.split()[1])

    def release(self) -> None:
        self._say("release")

    def round(self) -> None:
        self._say("round")

    def close(self) -> None:
        if self.process.poll() is None:
            self.process.stdin.close()
            self.process.stdout.read()
            self.process.wait(timeout=10)

    def __enter__(self) -> "Backend":
        return self

    def __exit__(self, *_: object) -> None:
        self.close()


def child_env(base: str, folder: Path, extra: dict[str, str] | None = None) -> dict[str, str]:
    """A child's whole environment: loopback only, fresh XDG folders."""
    folder.mkdir(parents=True, exist_ok=True)
    for name in ("cache", "config", "home"):
        (folder / name).mkdir(exist_ok=True)
    env = clean_env(**{
        "HOME": str(folder / "home"),
        "XDG_CACHE_HOME": str(folder / "cache"),
        "XDG_CONFIG_HOME": str(folder / "config"),
        "THINKTHEN_API_KEY": FAKE_KEY,
        "THINKTHEN_BASE_URL": base,
        **(extra or {}),
    })
    guard(env)
    return env


def guard(env: dict[str, str]) -> None:
    """Shared rules 3 and 6: refuse a child aimed off loopback or at the
    user's own cache folders."""
    base = env.get("THINKTHEN_BASE_URL", "")
    if urlsplit(base).scheme != "http" or urlsplit(base).hostname != "127.0.0.1":
        raise SystemExit(f"harness: refused a child whose backend is not on 127.0.0.1: {base!r}")
    home = Path.home()
    for name in ("XDG_CACHE_HOME", "XDG_CONFIG_HOME"):
        value = env.get(name)
        if not value or Path(value) in (home, home / ".cache", home / ".config"):
            raise SystemExit(f"harness: refused a child whose {name} is unset or the user's own")


CHILD = r"""
import json, sys
import duckdb
extension, statements = sys.argv[1], json.loads(sys.argv[2])
databases = {}
def database(name):
    if name not in databases:
        databases[name] = duckdb.connect(config={"allow_unsigned_extensions": "true"})
        databases[name].execute(f"LOAD '{extension}'")
    return databases[name]
database("A")
for statement in statements:
    name, statement = statement if isinstance(statement, list) else ("A", statement)
    try:
        rows = database(name).execute(statement).fetchall()
        print(json.dumps({"rows": rows}, default=str), flush=True)
    except BaseException as error:
        print(json.dumps({"error": str(error)}), flush=True)
"""


def run(
    statements: list,
    base: str,
    *,
    extra: dict[str, str] | None = None,
    extension: Path = EXTENSION,
    timeout: float = 60,
    wrap: list[str] | None = None,
) -> list[dict]:
    """Run statements in one fresh child and return one result per statement.
    A statement given as `[NAME, SQL]` runs on database NAME, a separate
    in-memory database in the same process; a plain one runs on `A`.
    `wrap` runs the child under a tracer such as `strace`."""
    with tempfile.TemporaryDirectory(prefix="thinkthen-duckdb-") as folder:
        env = child_env(base, Path(folder), extra)
        done = subprocess.run(
            [*(wrap or []), sys.executable, "-c", CHILD, str(extension), json.dumps(statements)],
            env=env,
            capture_output=True,
            text=True,
            timeout=timeout,
            check=False,
        )
    results = [json.loads(line) for line in done.stdout.splitlines() if line.startswith("{")]
    if len(results) != len(statements):
        raise AssertionError(f"the child answered {len(results)} of {len(statements)}: {done.stderr[-800:]}")
    return results


def said(result: dict) -> str:
    """The `thinkthen <kind>: ...` sentence of an error result."""
    error = result.get("error")
    if error is None:
        raise AssertionError(f"expected an error, got {result!r}")
    at = error.find("thinkthen ")
    # DuckDB appends a blank line and the query's position after a bind error.
    return (error[at:] if at >= 0 else error).split("\n")[0].strip()


def rows(result: dict) -> list:
    if "error" in result:
        raise AssertionError(f"expected rows, got {result['error']!r}")
    return result["rows"]


def expect(actual: object, wanted: object, what: str) -> None:
    if actual != wanted:
        raise AssertionError(f"{what}: wanted {wanted!r}, got {actual!r}")


CASES: list = []


def case(function):
    CASES.append(function)
    return function


def main() -> int:
    """Run every registered case, or the ones named on the command line."""
    named = set(sys.argv[1:])
    absent = named - {function.__name__ for function in CASES}
    if absent:
        print(f"FAIL unknown selected case: {sorted(absent)[0]}")
        return 2
    failed = 0
    for function in CASES:
        if named and function.__name__ not in named:
            continue
        started = time.monotonic()
        try:
            function()
        except BaseException as error:  # noqa: BLE001 - every failure is reported
            failed += 1
            print(f"FAIL {function.__name__}: {error}")
            if os.environ.get("THINKTHEN_SUITE_TRACE"):
                traceback.print_exc()
            continue
        print(f"ok   {function.__name__} ({time.monotonic() - started:.1f}s)")
    return 1 if failed else 0
