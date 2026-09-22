"""The second review's signal findings, in child processes.

From ``sdlc/issues/2026-09-22-surfaces-branch-second-review-new-defects-and-leftovers.md``:

- a ``SystemExit`` raised by a signal handler was turned into a cancel;
  it must be raised as itself when the call stops;
- ``Cancelled`` must be catchable as this package's own error, not only
  as a ``KeyboardInterrupt``.

Each child runs a two-million-record null batch — about 5.6 s deaf — and
the parent sends SIGINT about one second in; the child must stop within
1.5 s of its start, so the threshold separates a heard signal from a deaf
batch. Runs offline; check.sh calls it in the null section.
"""

import os
import signal
import subprocess
import sys
import time

CHILD_SYSTEM_EXIT = r"""
import signal
import time
import thinkthen as tt

def handler(signum, frame):
    raise SystemExit(3)

signal.signal(signal.SIGINT, handler)
records = [f"record {i}" for i in range(2_000_000)]
start = time.time()
try:
    tt.decide_many("Is this a complaint?", records)
except SystemExit as stopped:
    print(f"raised SystemExit {stopped.code} at {time.time() - start:.3f}", flush=True)
    raise SystemExit(0)
except BaseException as other:
    print(f"raised {type(other).__name__} at {time.time() - start:.3f}", flush=True)
    raise SystemExit(1)
raise SystemExit("the batch ran deaf")
"""

CHILD_THINKTHEN_ERROR = r"""
import signal
import time
import thinkthen as tt

records = [f"record {i}" for i in range(2_000_000)]
start = time.time()
try:
    tt.decide_many("Is this a complaint?", records)
except tt.ThinkThenError as stopped:
    print(f"caught {type(stopped).__name__} as ThinkThenError at {time.time() - start:.3f}", flush=True)
    raise SystemExit(0)
except BaseException as other:
    print(f"raised {type(other).__name__} at {time.time() - start:.3f}", flush=True)
    raise SystemExit(1)
raise SystemExit("the batch ran deaf")
"""


def _run_child(source: str) -> tuple[str, str, int]:
    child = subprocess.Popen(
        [sys.executable, "-c", source],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
        env={**os.environ, "ENGINE_NULL": "1"},
    )
    time.sleep(1.0)
    child.send_signal(signal.SIGINT)
    try:
        out, err = child.communicate(timeout=30)
    except subprocess.TimeoutExpired:
        child.kill()
        out, err = child.communicate()
    return out, err, child.returncode


def _elapsed(out: str) -> float:
    for line in out.splitlines():
        if " at " in line:
            return float(line.rsplit(" ", 1)[1])
    raise AssertionError(f"the child printed no timing: {out!r}")


def test_a_signal_handlers_system_exit_is_raised_as_itself():
    out, err, code = _run_child(CHILD_SYSTEM_EXIT)
    assert code == 0, f"{out}\n{err}"
    assert "raised SystemExit 3" in out, out
    assert _elapsed(out) < 1.5, out


def test_a_cancelled_batch_is_catchable_as_a_thinkthen_error():
    out, err, code = _run_child(CHILD_THINKTHEN_ERROR)
    assert code == 0, f"{out}\n{err}"
    assert "caught Cancelled as ThinkThenError" in out, out
    assert _elapsed(out) < 1.5, out
