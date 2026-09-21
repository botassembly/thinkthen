"""Cancel, on the wire: SIGINT during a bulk wait raises Cancelled.

The shape is conformance case 18's own (a mid-batch cancel), proven the
211 way: a child process runs a bulk call against the delayed stub, the
parent sends SIGINT about one second in, and the child must raise
``Cancelled`` (a ``KeyboardInterrupt``) with nothing served after its
return. Runs only when the stub is up; check.sh decides.
"""

import os
import signal
import subprocess
import sys
import time

CHILD = r"""
import thinkthen as tt
records = [f"record {i}" for i in range(40)]
try:
    tt.decide_many("Is this a complaint?", records)
except KeyboardInterrupt as stopped:
    print(f"raised {type(stopped).__name__}", flush=True)
    print(f"requests at return {tt.usage()['requests']}", flush=True)
    import time
    time.sleep(2)
    print(f"requests after settle {tt.usage()['requests']}", flush=True)
    raise SystemExit(0)
raise SystemExit("the batch ran deaf")
"""


def main() -> int:
    child = subprocess.Popen(
        [sys.executable, "-c", CHILD],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        text=True,
    )
    time.sleep(1.0)
    child.send_signal(signal.SIGINT)
    try:
        out, err = child.communicate(timeout=15)
    except subprocess.TimeoutExpired:
        child.kill()
        out, err = child.communicate()
        print("FAIL  the child never returned")
        return 1
    print(out.strip())
    if child.returncode != 0:
        print(err.strip())
        return 1
    served = [int(line.rsplit(" ", 1)[1]) for line in out.splitlines() if "requests" in line]
    raised = any(line.startswith("raised Cancelled") for line in out.splitlines())
    if raised and len(served) == 2 and served[0] == served[1]:
        print(f"OK  raised within the window, nothing served after ({served[0]} requests)")
        return 0
    print(f"FAIL  raised={raised} served={served}")
    return 1


if __name__ == "__main__":
    if os.environ.get("THINKTHEN_BASE_URL", "").strip() == "":
        print("skip  cancel needs the stub on the wire")
        raise SystemExit(0)
    raise SystemExit(main())
