#!/usr/bin/env python3
"""ADR 0113: the extension adds its sends and cache answers to the command's
usage totals, and the process's exit flushes them while another writer holds
the usage lock."""

from __future__ import annotations

import fcntl
import json
import os
import pathlib
import subprocess
import sys
import time

from helper import ROOT, Backend, Child, child, child_env, environment, expect, main

COMMAND = os.environ.get("THINKTHEN_COMMAND", str(pathlib.Path(os.environ.get("CARGO_TARGET_DIR", ROOT.parents[1] / "target")) / "debug" / "thinkthen"))
HOLD = 0.3
STRESS = os.environ.get("THINKTHEN_TEST_PROFILE") == "stress"


def usage_environment(backend: Backend) -> dict[str, str]:
    """A child whose usage folder is its own scratch one on either platform."""
    env = environment(backend)
    env["HOME"] = env["SCRATCH"]
    return env


def usage_folder(env: dict[str, str]) -> pathlib.Path:
    if sys.platform == "darwin":
        return pathlib.Path(env["HOME"]) / "Library/Application Support/thinkthen/usage"
    return pathlib.Path(env["XDG_STATE_HOME"]) / "thinkthen"


def held_lock(env: dict[str, str]) -> int:
    """Make the usage folder and take its lock, as another writer would."""
    folder = usage_folder(env)
    folder.mkdir(mode=0o700, parents=True)
    descriptor = os.open(folder / ".lock", os.O_RDWR | os.O_CREAT | os.O_EXCL, 0o600)
    fcntl.flock(descriptor, fcntl.LOCK_EX)
    return descriptor


def totals(env: dict[str, str]) -> dict:
    status = subprocess.run([COMMAND, "status", "--json"], env=child_env(home=env["HOME"], PATH=env["PATH"], XDG_STATE_HOME=env["XDG_STATE_HOME"]),
                            capture_output=True, text=True, timeout=30, check=True)
    return json.loads(status.stdout)["usage"]["total"]


def exits_while_held(code: str, env: dict[str, str]) -> tuple[dict, float]:
    """Run `code` to its last line, then hold the usage lock for HOLD seconds
    while the child exits. Return its result and how long the exit took."""
    lock = held_lock(env)
    child = Child(code, env)
    held = child.read()
    started = time.monotonic()
    time.sleep(HOLD)
    os.close(lock)
    child.process.wait(timeout=30)
    expect(child.process.returncode, 0, "the child exits cleanly")
    return held, time.monotonic() - started


def test_the_exit_flushes_a_call_while_the_usage_lock_is_held() -> None:
    backend = Backend()
    env = usage_environment(backend)
    held, _ = exits_while_held("""
say(answer=run(connect(), "SELECT thinkthen_decide('Is this a complaint?', 'i want a refund')"))
""", env)
    expect(held["answer"], [[1]], "the answer")
    expect(backend.count(), 1, "one send")
    expect(totals(env)["requests_sent"], 1, "the exit wrote the send")


def test_an_unloaded_and_reloaded_extension_counts_each_call() -> None:
    backend = Backend()
    env = usage_environment(backend)
    held, _ = exits_while_held("""
first = connect()
one = run(first, "SELECT thinkthen_decide('Is this a complaint?', 'i want a refund')")
first.close()
second = connect()
two = run(second, "SELECT thinkthen_decide('Is this a complaint?', 'where is my parcel')")
say(answers=[one, two])
""", env)
    expect(held["answers"], [[[1]], [[1]]], "both answers")
    expect(backend.count(), 2, "two sends")
    expect(totals(env)["requests_sent"], 2, "each call counted once")


def test_a_cached_rerun_sends_nothing_and_adds_a_cache_answer() -> None:
    """A second process asks the same question from the named cache folder."""
    backend = Backend()
    env = usage_environment(backend)
    code = """
say(answer=run(connect(), "SELECT thinkthen_decide('Is this a complaint?', 'i want a refund')"))
"""
    expect([child(code, env)["answer"] for _ in range(2)], [[[1]], [[1]]], "both answers")
    expect(backend.count(), 1, "one send")
    held = totals(env)
    expect((held["requests_sent"], held["cache_answers"]), (1, 1), "one send and one cache answer")


def test_a_forked_child_exits_promptly_and_counts_nothing_twice() -> None:
    backend = Backend()
    env = usage_environment(backend)
    held, _ = exits_while_held("""
import warnings
warnings.simplefilter("ignore", DeprecationWarning)
answer = run(connect(), "SELECT thinkthen_decide('Is this a complaint?', 'i want a refund')")
started = time.monotonic()
pid = os.fork()
if pid == 0:
    sys.exit(0)
os.waitpid(pid, 0)
say(answer=answer, child_exit=time.monotonic() - started)
""", env)
    expect(held["answer"], [[1]], "the parent's answer")
    # The parent holds the lock until this child prints, so a fork that waited for
    # the lock would hang the read. The stress profile also times it (ticket 0352).
    expect(not STRESS or held["child_exit"] < 0.25, True, f"the child exited in {held['child_exit']:.3f} s, inside the held lock")
    expect(backend.count(), 1, "one send")
    expect(totals(env)["requests_sent"], 1, "the parent's send counted once")


if __name__ == "__main__":
    os.chdir(pathlib.Path(__file__).resolve().parent)
    sys.exit(main(globals()))
