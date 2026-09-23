"""Review 4, item 6: a retired-but-live database skips the file-access
refusal.

The reviewer's window: the reaper marks up to 8 live databases retired
per pass, file_read_refusal skips retired entries, and an @file read
falls back to a raw std::fs read — 53 of 342,636 calls read a file with
external access off.

Deterministic forcing here: database A loads with external access OFF
and PREPARES a relate, whose bind holds a guard on A's kept connection;
A's last caller disconnects, so the reaper finds A alone-but-guarded and
leaves it retired while the prepared statement lives. Database B loads
with access ON. B's @file scalar call must still be refused by A's
setting — retired-but-live enforces access exactly.

FAIL on the reviewed code (the read succeeds past A's refusal), PASS on
the fix (the read is refused).
"""

from __future__ import annotations

import ctypes
import sys
import tempfile
import time
from pathlib import Path

from review4_lib import EXTENSION, Harness, finish, verdict


def main() -> int:
    harness = Harness()
    with tempfile.TemporaryDirectory() as tmp:
        question = Path(tmp) / "q.json"
        question.write_text(
            '{"decide": {"question": "does this text ask for a refund", '
            '"yes": "yes", "no": "no"}}',
            encoding="utf-8",
        )

        # A: access off, loads, prepares a relate (the bind holds a
        # guard on A's kept connection), then its caller disconnects.
        a = harness.open()
        ca = harness.connect(a)
        harness.load(ca)
        harness.run(ca, "SET enable_external_access=false")
        prep = ctypes_prepare(harness, ca)
        harness.disconnect(ca)
        time.sleep(2.6)  # a reaper pass retires the alone-but-guarded A

        # B: access on, loads, and reads the question file.
        b = harness.open()
        cb = harness.connect(b)
        harness.load(cb)
        ok, error, rows = harness.run(
            cb,
            f"SELECT thinkthen_decide('@/tmp/../{question}', 'please refund order 1')",
        )
        read_happened = ok or "was not read" not in error

        # Cleanup: finish and drop the prepared statement, close both.
        drop_prepare(harness, ca if False else cb, prep)
        harness.disconnect(cb)
        harness.close(b)
        harness.close(a)

        return verdict(
            "retired-but-live: the access-off database still refuses the @file read",
            not read_happened,
            ("the read SUCCEEDED past the retired database's refusal"
             if read_happened
             else f"the refusal: {error}"),
        )


def ctypes_prepare(harness, connection):
    """Prepare a relate without executing it, holding its bind guard."""
    lib = harness.lib
    lib.duckdb_prepare.argtypes = [
        ctypes.c_void_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p),
    ]
    lib.duckdb_prepare.restype = ctypes.c_int
    prepared = ctypes.c_void_p()
    sql = (
        "SELECT name FROM thinkthen_relate("
        "'SELECT 1, ''please refund order 1''', ['caused_by'])"
    )
    state = lib.duckdb_prepare(connection, sql.encode(), ctypes.byref(prepared))
    if state != 0:
        raise SystemExit("the prepare refused")
    return prepared


def drop_prepare(harness, connection, prepared) -> None:
    lib = harness.lib
    lib.duckdb_destroy_prepare.argtypes = [ctypes.POINTER(ctypes.c_void_p)]
    lib.duckdb_destroy_prepare(ctypes.byref(prepared))


if __name__ == "__main__":
    import ctypes  # noqa: PLC0415 - used by the helpers above

    sys.exit(main())
