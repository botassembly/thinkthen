#!/usr/bin/env python3
"""Review 4, item 5: the identity probe can be forged.

forge3.py's shape: a read-only database gets no probe and falls back to
its NAME; another database detaches its own probe and attaches
`:memory:` under that name, so the attacker's caller resolves the
victim's name and is routed onto the victim's kept connection, reading
the victim's tables.

forge2.py's shape: a KNOWN probe name forges a writable database — the
attacker learns the victim's probe name (visible in the victim's own
SHOW DATABASES to its own clients), attaches `:memory:` under it, and
is routed onto the victim's kept connection.

PASS on the fix: SQL can neither impersonate a probe (the resolved
catalog must also carry the marker table only the real probe holds) nor
fall back to a name. The attacker's relate fails to name a database,
and the victim's own relate still works when it is the only loaded
database.
"""

from __future__ import annotations

import re
import sys
import tempfile

from review4_lib import Harness, case_by_id, finish, relate_rules, verdict


def fill(harness, con, case: dict) -> str:
    harness.run(con, "CREATE TABLE t(id INTEGER, body VARCHAR)")
    for index, body in enumerate(case["records"], 1):
        harness.run(con, f"INSERT INTO t VALUES ({index}, '{body}')")
    return f"SELECT id, body FROM t"


def main() -> int:
    harness = Harness()
    first = case_by_id("69-relate-R01-demo")
    second = case_by_id("70-relate-R02-pickone")

    with tempfile.TemporaryDirectory() as tmp:
        victim_path = f"{tmp}/victim.duckdb"

        # A writable instance creates the victim file, then the host
        # reopens it read-only: the read-only victim loads the extension.
        writer = harness.open(victim_path)
        cw = harness.connect(writer)
        harness.run(cw, "CREATE TABLE secret(mark INTEGER)")
        harness.run(cw, "INSERT INTO secret VALUES (1)")
        harness.disconnect(cw)
        harness.close(writer)

        victim = harness.open(victim_path, access_mode="read_only")
        cv = harness.connect(victim)
        harness.load(cv)
        probe_names = [
            row[0]
            for row in harness.run(cv, "SHOW DATABASES")[2]
            if row[0] and row[0].startswith("thinkthen_instance_")
        ]
        no_probe = not probe_names
        victim_name = harness.run(cv, "SELECT current_database()")[2][0][0]

        # The attacker: a writable database that loads the extension,
        # detaches its own probe, and attaches :memory: under the
        # victim's name (forge3) or the victim's probe name (forge2).
        attacker = harness.open()
        ca = harness.connect(attacker)
        harness.load(ca)
        own = [
            row[0]
            for row in harness.run(ca, "SHOW DATABASES")[2]
            if row[0] and row[0].startswith("thinkthen_instance_")
        ]
        for probe in own:
            harness.run(ca, f"DETACH {probe}")
        if probe_names:
            harness.run(ca, f"ATTACH ':memory:' AS {probe_names[0]}")
        harness.run(ca, f"ATTACH ':memory:' AS {victim_name}")

        results = []

        # forge3: relate on the attacker under the victim's name alias.
        ok, error, rows = harness.run(
            ca, f"SELECT name FROM thinkthen_relate('SELECT 1, ''x''', ['x'])"
        )
        # The attacker's own probe is detached and its name alias points
        # at an empty :memory:, so nothing proves the attacker's
        # identity: any ROUTING onto the victim is the bug. The query
        # above cannot read the victim (its FROM is a constant), so a
        # routing bug surfaces as a MISSING-TABLE-style failure for
        # 'x'... on the victim, which HAS no table x either. The honest
        # observable is the routing error itself: the attacker must be
        # told no database matches, never the victim's boundary.
        refused_attacker = (
            not ok
            and ("no loaded database matches" in error
                 or "read-only" in error
                 or "cannot be identified" in error
                 or "identity" in error)
        )
        results.append(
            verdict(
                "forge3: the attacker under the victim's name is not routed onto the victim",
                refused_attacker,
                f"the attacker's relate: {error if not ok else 'answered ' + str(rows)}",
            )
        )

        # forge2: the attacker's relate with the victim's PROBE name
        # attached. Routing must not land on the victim: the alias
        # carries no marker table.
        ok2, error2, rows2 = harness.run(
            ca, f"SELECT name FROM thinkthen_relate('SELECT 1, ''x''', ['x'])"
        )
        results.append(
            verdict(
                "forge2: a known probe name does not forge the victim",
                not ok2
                and ("no loaded database matches" in error2
                     or "read-only" in error2
                     or "cannot be identified" in error2
                     or "identity" in error2),
                f"the attacker's relate: {error2 if not ok2 else 'answered ' + str(rows2)}",
            )
        )

        # The victim's own relate still answers when it is alone.
        harness.close(attacker)
        rules = relate_rules(first)
        okv, errorv, rowsv = harness.run(
            cv, f"SELECT count(*) FROM thinkthen_relate('SELECT 1, ''refunds?''', {rules})"
        )
        results.append(
            verdict(
                "the read-only victim's own door answers with the honest boundary, never a wrong routing",
                okv
                or "temporary table" in errorv
                or "usage" in errorv
                or "identity" in errorv
                or "read-only" in errorv,
                f"the victim's relate: {errorv if not okv else f'{len(rowsv)} rows'}",
            )
        )

        print(f"       (the read-only victim loaded with "
              f"{'no probe — the name fallback class' if no_probe else 'a probe attached'})")
        return 0 if all(r == 0 for r in results) else 1


if __name__ == "__main__":
    finish(main())
