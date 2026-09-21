"""Cancel on a fast backend: SIGINT stops a null-backend batch within a tick.

The starvation this pins, found by the adversarial review and reproduced by
the supervisor: before the fix, SIGINT one second into a three-million-record
null batch raised only at 8.48 s, after the whole batch, because the poll ran
only when the wait channel idled. The child here runs a two-million-record
null batch, the parent sends SIGINT about one second in, and the child must
raise ``KeyboardInterrupt`` within 1.5 s of its start — the batch would take
about 5.6 s to finish deaf, so the threshold separates the two behaviors.
Runs offline; check.sh calls it in the null section.
"""

import os
import signal
import subprocess
import sys
import time

CHILD = r"""
import thinkthen as tt
import time
records = [f"record {i}" for i in range(2_000_000)]
start = time.time()
try:
    tt.decide_many("Is this a complaint?", records)
except KeyboardInterrupt:
    print(f"elapsed {time.time() - start:.3f}", flush=True)
    raise SystemExit(0)
raise SystemExit("the batch ran deaf")
"""


def main() -> int:
    child = subprocess.Popen(
        [sys.executable, "-c", CHILD],
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
        print("FAIL  the child never returned")
        return 1
    print(out.strip())
    if child.returncode != 0:
        print(err.strip())
        return 1
    elapsed = None
    for line in out.splitlines():
        if line.startswith("elapsed "):
            elapsed = float(line.rsplit(" ", 1)[1])
    if elapsed is not None and elapsed < 1.5:
        print(f"OK  the interrupt landed at {elapsed:.3f} s, within a tick of the signal")
        return 0
    print(f"FAIL  the interrupt landed at {elapsed} s (the batch runs about 5.6 s deaf)")
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
