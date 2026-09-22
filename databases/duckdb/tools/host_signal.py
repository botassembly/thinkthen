#!/usr/bin/env python3
"""The DuckDB surface's punch-list coexistence proof: the extension's
LOAD-time SIGINT handler beside a host that handles SIGINT itself.

Two child processes, one arm each, so one arm's signal cannot color the
other's evidence. Both run the extension's real build through the Python
host the build's own venv carries (duckdb 1.5.5, the CLI's version), on
the null backend: no stub, no key, no network.

  after   The host installs its own SIGINT handler after LOAD — the
          job-2 shape. The handler must fire, the process must survive,
          and the extension must keep answering. The host's install
          replaces the extension's handler at the OS level, and the
          extension is not broken by it.
  chain   The host's handler is already installed when LOAD runs — the
          default Python shape, and the CLI's. SIGINT must both stop a
          running query and reach the host's handler.

The chain arm ends with a `note` line, measured and not asserted: after
the cancelled statement the process-wide token stays set, so the next
call in that process answers `cancelled`. The CLI exits before that
matters, and a long-lived host restarts or reloads the extension. The
finding is recorded in NOTES.md and the punch-list report; a fix (a
re-armable current token) is the architect's call, not this lane's.
"""

import os
import signal
import subprocess
import sys
import threading
import time

HERE = os.path.dirname(os.path.abspath(__file__))
EXT = os.path.join(os.path.dirname(HERE), "build", "release", "thinkthen.duckdb_extension")


def arm_after(ext):
    """The job-2 shape: the host's handler is installed after LOAD."""
    import duckdb

    con = duckdb.connect(config={"allow_unsigned_extensions": "true"})
    con.execute(f"LOAD '{ext}'")

    fired = []

    def handler(signum, frame):
        fired.append(signum)

    signal.signal(signal.SIGINT, handler)
    os.kill(os.getpid(), signal.SIGINT)
    if fired != [signal.SIGINT]:
        print(f"FAILED   the host's handler did not fire: {fired}")
        return 1
    answer = con.execute(
        "SELECT thinkthen_decide('Is this a complaint?', 'I demand a refund today')"
    ).fetchone()[0]
    if answer is not True:
        print(f"FAILED   the extension stopped answering: {answer}")
        return 1
    print("ok       the host's handler took SIGINT after LOAD and the extension still answered true")
    return 0


def arm_chain(ext):
    """The chained shape: the host's handler was there when LOAD ran."""
    import duckdb

    fired = threading.Event()

    def handler(signum, frame):
        fired.set()

    signal.signal(signal.SIGINT, handler)
    con = duckdb.connect(config={"allow_unsigned_extensions": "true"})
    con.execute(f"LOAD '{ext}'")

    def later():
        time.sleep(1.0)
        os.kill(os.getpid(), signal.SIGINT)

    threading.Thread(target=later, daemon=True).start()
    start = time.monotonic()
    try:
        con.execute(
            "SELECT count(thinkthen_decide('Is this a complaint?', 'i want a refund now'))"
            " FROM range(3000000)"
        ).fetchall()
        outcome = "ran to completion"
    except duckdb.Error as error:
        outcome = str(error)
    elapsed = time.monotonic() - start
    print(f"         the query ended {elapsed:.2f}s after the signal with: {outcome}")
    if elapsed > 2.5:
        print(f"FAILED   the signal did not stop the query ({elapsed:.2f}s)")
        return 1
    if "cancelled" not in outcome:
        print("FAILED   the query did not end with the cancelled kind")
        return 1
    if not fired.wait(2.0):
        print("FAILED   the host's chained handler did not fire")
        return 1
    print("ok       a signal stopped the running query and reached the host's chained handler")
    # Measured, not asserted: what a long-lived host sees on its next call.
    try:
        again = con.execute(
            "SELECT thinkthen_decide('Is this a complaint?', 'I demand a refund today')"
        ).fetchone()[0]
        print(f"note     after the cancelled statement the next call answered: {again}")
    except duckdb.Error as error:
        print(f"note     after the cancelled statement the next call answered: {error}")
    return 0


ARMS = {"after": arm_after, "chain": arm_chain}


def main():
    if len(sys.argv) > 2 and sys.argv[1] in ARMS:
        return ARMS[sys.argv[1]](sys.argv[2])
    try:
        import duckdb
    except ImportError:
        print("this Python has no duckdb module; run the proof with configure/venv/bin/python")
        return 2
    print(
        f"host: Python {sys.version.split()[0]} with duckdb {duckdb.__version__};"
        " the extension is built for v1.5.5"
    )
    status = 0
    for name in ARMS:
        print(f"-- host SIGINT, the {name} arm", flush=True)
        env = dict(os.environ, ENGINE_NULL="1")
        done = subprocess.run([sys.executable, os.path.abspath(__file__), name, EXT], env=env)
        status |= done.returncode
    return status


if __name__ == "__main__":
    sys.exit(main())
