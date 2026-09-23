#!/usr/bin/env python3
"""Two databases in one process, each loading this extension.

The relate scan must run on a connection belonging to the CALLING
database; before the fix one process-global connection served whichever
database loaded last, so the first database's query ran against the
second database's tables. Both databases here hold a table named `t`
with recorded-but-different contents, so the wrong connection answers
with the other case's edges instead of erroring. The venv carries duckdb
1.5.5, the CLI's version.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
import time
from pathlib import Path
from review4_lib import relate_rules  # noqa: E402 - the one either-aware rule list

ROOT = Path(__file__).resolve().parent.parent
EXTENSION = ROOT / "build" / "release" / "thinkthen.duckdb_extension"
CASES = ROOT.parent.parent / "conformance" / "conformance.json"


def case_by_id(cases: list[dict], name: str) -> dict:
    for case in cases:
        if case["id"] == name:
            return case
    raise SystemExit(f"no conformance case {name}")


def main() -> int:
    os.environ["ENGINE_NULL"] = "1"
    import duckdb  # noqa: PLC0415 - imported after the environment is set

    cases = json.loads(CASES.read_text())["cases"]
    first_case = case_by_id(cases, "69-relate-alerts")
    second_case = case_by_id(cases, "70-relate-founders")

    with tempfile.TemporaryDirectory() as tmp:

        def connect(name: str):
            con = duckdb.connect(
                os.path.join(tmp, name),
                config={"allow_unsigned_extensions": "true"},
            )
            con.execute(f"LOAD '{EXTENSION}'")
            return con

        def fill(con, case: dict) -> None:
            con.execute("CREATE TABLE t(id INTEGER, body VARCHAR)")
            for index, body in enumerate(case["records"], 1):
                con.execute("INSERT INTO t VALUES (?, ?)", [index, body])

        def ask(con, case: dict) -> int:
            rules = relate_rules(case)
            rows = con.execute(
                f"SELECT * FROM thinkthen_relate('SELECT id, body FROM t', {rules})"
            ).fetchall()
            return len(rows)

        first = connect("a.db")
        second = connect("b.db")
        fill(first, first_case)
        fill(second, second_case)

        got_first = ask(first, first_case)
        got_second = ask(second, second_case)
        want_first = len(first_case["expect"]["edges"])
        want_second = len(second_case["expect"]["edges"])
        print(
            f"a.db: {got_first} edges (want {want_first}), "
            f"b.db: {got_second} edges (want {want_second})"
        )

        if got_first != want_first:
            print("FAILED   a.db answered from the wrong database")
            return 1
        if got_second != want_second:
            print("FAILED   b.db answered from the wrong database")
            return 1
        print("ok       each database's relate ran on its own connection")

        # Two in-memory databases both report the name `memory`; before the
        # identity token, relate could not tell them apart and failed both.
        memory_first = duckdb.connect(":memory:", config={"allow_unsigned_extensions": "true"})
        memory_second = duckdb.connect(":memory:", config={"allow_unsigned_extensions": "true"})
        for con, case in ((memory_first, first_case), (memory_second, second_case)):
            con.execute(f"LOAD '{EXTENSION}'")
            fill(con, case)
        memory_got_first = ask(memory_first, first_case)
        memory_got_second = ask(memory_second, second_case)
        if memory_got_first != want_first or memory_got_second != want_second:
            print(
                "FAILED   two in-memory databases: "
                f"got {memory_got_first}/{memory_got_second}, want {want_first}/{want_second}"
            )
            return 1
        print("ok       two in-memory databases each relate on their own connection")
        memory_first.close()
        memory_second.close()

        # ATTACH ... USE other: the relate query must run under the calling
        # session's own search path, so `FROM tt` reads the attached
        # database's table, not the main one's. The attached table carries
        # a NULL row, whose refusal proves which table was read.
        attached = connect("attached-main.db")
        attached.execute("CREATE TABLE tt(id INTEGER, body VARCHAR)")
        attached.execute("INSERT INTO tt VALUES (1, 'main table body')")
        # The main catalog's own name, read before USE moves the default.
        main_name = attached.execute("SELECT current_database()").fetchone()[0]
        other_path = os.path.join(tmp, "attached-other.db")
        attached.execute(f"ATTACH '{other_path}' AS other")
        attached.execute("CREATE TABLE other.tt(id INTEGER, body VARCHAR)")
        attached.execute("INSERT INTO other.tt VALUES (1, 'the payment failed'), (2, NULL)")
        attached.execute("USE other")
        try:
            attached.execute(
                "SELECT * FROM thinkthen_relate('SELECT id, body FROM tt', ['caused_by'])"
            ).fetchall()
            print("FAILED   USE other did not reach the relate query: no NULL refusal")
            return 1
        except Exception as error:  # noqa: BLE001 - the refusal is the proof
            if "record 2 carries a NULL id or text" not in str(error):
                print(f"FAILED   USE other read the wrong table: {str(error)[:120]}")
                return 1
        print("ok       USE other reached the relate query: it read the attached table")
        # Back to the main catalog, by the name read before USE moved it:
        # `USE main` would name a schema inside the catalog still in force.
        attached.execute(f'USE "{main_name}"')
        try:
            attached.execute(
                "SELECT * FROM thinkthen_relate('SELECT id, body FROM tt', ['caused_by'])"
            ).fetchall()
            print("FAILED   the main table's relate answered, so no table was read")
            return 1
        except Exception as error:  # noqa: BLE001 - main's text is unrecorded
            if "main table body" not in str(error):
                print(f"FAILED   the main catalog's relate read another table: {str(error)[:120]}")
                return 1
        print("ok       leaving USE behind reads the main table again")
        attached.close()

        # The kept connection must not keep a closed database's file
        # locked: the reaper releases it when the last caller connection
        # closes, so a reopen — in this process and in another — works.
        locked_path = os.path.join(tmp, "locked.db")
        holder = duckdb.connect(locked_path, config={"allow_unsigned_extensions": "true"})
        holder.execute(f"LOAD '{EXTENSION}'")
        holder.execute("CREATE TABLE t AS SELECT 1 AS id, 'the payment failed' AS body")
        holder.close()
        released = False
        for _ in range(50):
            time.sleep(0.1)
            try:
                again = duckdb.connect(locked_path, config={"allow_unsigned_extensions": "true"})
                again.close()
                released = True
                break
            except Exception:  # noqa: BLE001 - the lock is what we wait out
                continue
        if not released:
            print("FAILED   the closed database's file stayed locked in-process")
            return 1
        print("ok       the closed database's file reopened in-process")
        other_process = subprocess.run(
            [
                sys.executable,
                "-c",
                f"import duckdb; duckdb.connect({locked_path!r}).close(); print('opened')",
            ],
            capture_output=True,
            text=True,
            timeout=60,
        )
        if other_process.returncode != 0:
            print(
                "FAILED   another process could not open the closed database: "
                f"{other_process.stderr.strip()[:120]}"
            )
            return 1
        print("ok       another process opened the closed database's file")
        return 0


if __name__ == "__main__":
    sys.exit(main())
