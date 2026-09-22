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
import sys
import tempfile
from pathlib import Path

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
    first_case = case_by_id(cases, "69-relate-R01-demo")
    second_case = case_by_id(cases, "70-relate-R02-pickone")

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
            rules = "[" + ",".join(
                f"'{rule['name']}'" for rule in case["question"]["relations"]
            ) + "]"
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
        return 0


if __name__ == "__main__":
    sys.exit(main())
