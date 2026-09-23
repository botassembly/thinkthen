#!/usr/bin/env python3
"""Review 4, item 4: the fast-path routes a reaped database's caller
onto the next database's kept connection.

The reviewer's shape (fastpath.py): database A loads the extension with
external access off and holds a table; A's connections close while the
host keeps the database open, so the reaper releases A's kept
connection; database B loads; the host opens a NEW connection on A and
relates. One loaded entry must not mean "that one is mine": A's caller
must be refused, never routed onto B's connection.

FAIL on the reviewed code (the single-entry shortcut hands B's guard to
A's caller, whose query then reads B's table), PASS on the fix (the
caller that proves no identity earns the reload error).
"""

from __future__ import annotations

import sys
import time

from review4_lib import Harness, case_by_id, finish, relate_rules, verdict


def main() -> int:
    harness = Harness()
    first = case_by_id("69-relate-R01-demo")
    second = case_by_id("70-relate-R02-pickone")

    # A: loads, external access off, its own table with the FIRST case.
    a = harness.open()
    ca = harness.connect(a)
    harness.load(ca)
    ok, error, _ = harness.run(ca, "SET enable_external_access=false")
    if not ok:
        return verdict("fastpath: A could not set its access off", False, error)
    ok, error, _ = harness.run(ca, "CREATE TABLE t(id INTEGER, body VARCHAR)")
    for index, body in enumerate(first["records"], 1):
        ok, error, _ = harness.run(ca, f"INSERT INTO t VALUES ({index}, '{body}')")
        if not ok:
            return verdict("fastpath: A could not fill its table", False, error)
    rules = relate_rules(first)
    query_a = f"SELECT id, body FROM t"

    # A's only caller closes; the host keeps the database open, so the
    # reaper finds A alone and releases its kept connection.
    harness.disconnect(ca)
    time.sleep(2.6)

    # B loads with its own table carrying the SECOND case.
    b = harness.open()
    cb = harness.connect(b)
    harness.load(cb)
    harness.run(cb, "CREATE TABLE t(id INTEGER, body VARCHAR)")
    for index, body in enumerate(second["records"], 1):
        harness.run(cb, f"INSERT INTO t VALUES ({index}, '{body}')")

    # The host reconnects on A and relates.
    ca2 = harness.connect(a)
    ok, error, rows = harness.run(
        ca2, f"SELECT name FROM thinkthen_relate('{query_a}', {rules})"
    )

    if not ok:
        # Teardown hygiene: close both databases before the verdict.
        harness.disconnect(ca2)
        harness.disconnect(cb)
        harness.close(a)
        harness.close(b)
        misroute = "the recording covers" in error
        return verdict(
            "fastpath: the reaped database's caller is refused, not rerouted",
            "no loaded database matches" in error
            or "released with its last caller" in error
            or "LOAD the extension again" in error,
            ("A's rules ran over B's table: the engine answered from the "
             "recording that covers B's relations — " if misroute else "")
            + f"the refusal: {error}",
        )

    # The query ran on SOME kept connection and answered SOME case's
    # edges. Which case's edges named themselves decides whose table
    # was read: the first case's relation names are A's, the second's
    # are B's.
    names = {row[0] for row in rows if row[0]}
    a_names = {rule["name"] for rule in first["question"]["relations"]}
    b_names = {rule["name"] for rule in second["question"]["relations"]}
    read_b = bool(names & b_names) and not (names & a_names)
    harness.disconnect(ca2)
    harness.disconnect(cb)
    harness.close(a)
    harness.close(b)
    return verdict(
        "fastpath: the reaped database's caller is refused, not rerouted",
        not read_b,
        f"the relate ANSWERED with rows {sorted(names)}: "
        + ("B's edges — A's query ran on B's kept connection" if read_b else "A's own edges"),
    )


if __name__ == "__main__":
    finish(main())
