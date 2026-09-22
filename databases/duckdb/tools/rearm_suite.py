#!/usr/bin/env python3
"""One interrupt stops one query, and the next query still answers.

Before the fix the surface carried one process-wide cancel token, and a
one-shot token is spent forever: after one Ctrl-C every later call in
that process returned `cancelled` without asking anything. Here a long
null-backend query is interrupted with SIGINT, and a fresh query in the
same process must answer. The venv carries duckdb 1.5.5, the CLI's
version.
"""

from __future__ import annotations

import os
import signal
import sys
import threading
import time
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
EXTENSION = ROOT / "build" / "release" / "thinkthen.duckdb_extension"
SLOW = (
    "SELECT count(thinkthen_decide('Is this a complaint?', 'i want a refund now'))"
    " FROM range(3000000)"
)
FRESH = "SELECT thinkthen_decide('Is this a complaint?', 'I demand a refund today')"


def interrupt_after(seconds: float) -> threading.Timer:
    timer = threading.Timer(seconds, os.kill, args=(os.getpid(), signal.SIGINT))
    timer.daemon = True
    timer.start()
    return timer


def main() -> int:
    os.environ["ENGINE_NULL"] = "1"
    import duckdb  # noqa: PLC0415 - imported after the environment is set

    con = duckdb.connect(config={"allow_unsigned_extensions": "true"})
    con.execute(f"LOAD '{EXTENSION}'")

    interrupted = False
    timer = interrupt_after(1.0)
    try:
        con.execute(SLOW).fetchall()
        print("FAILED   the long query ran to completion despite the interrupt")
        timer.cancel()
        return 1
    except BaseException as error:  # noqa: BLE001 - the interrupt's own kind
        interrupted = True
        try:
            print(f"the interrupted call ended with {type(error).__name__}: {str(error)[:90]}")
        except BaseException:  # noqa: BLE001 - a pending interrupt may land in the print
            pass
    finally:
        timer.cancel()
    # A KeyboardInterrupt can still be pending after the C call returns.
    try:
        time.sleep(0.2)
    except KeyboardInterrupt:
        pass
    if not interrupted:
        print("FAILED   the long query was not interrupted")
        return 1

    try:
        fresh = con.execute(FRESH).fetchone()
    except BaseException as error:  # noqa: BLE001 - the poisoned token's own error
        print(
            "FAILED   the query after the interrupt errored, so the interrupt poisoned "
            f"the process: {type(error).__name__}: {str(error)[:90]}"
        )
        return 1
    if fresh != (True,):
        print(f"FAILED   the query after the interrupt must answer, got {fresh!r}")
        return 1
    print("ok       the interrupt stopped one query and the next query answered")
    return 0


if __name__ == "__main__":
    sys.exit(main())
