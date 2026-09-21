#!/usr/bin/env python3
"""Every SQLite function's example, run as one test.

The file `examples.json` is keyed by function: the statement and the
answer the null backend gives, plus an optional setup. The site's
function pages and surface pages draw their SQL tab from this file
(site.md's one source), so an example nobody runs cannot reach the site.

Each example runs in its own fresh Python process, because the engine's
usage counters are process-wide: a fresh process makes every expected
answer deterministic. Offline, no stub, no key.
"""

from __future__ import annotations

import json
import os
import pathlib
import sqlite3
import subprocess
import sys

os.environ.setdefault("ENGINE_NULL", "1")
os.environ.setdefault("ENGINE_WIDTH", "32")

HERE = pathlib.Path(__file__).resolve().parent
FILE = HERE.parent / "examples.json"
LIB = HERE.parent / "target" / "release" / "libthinkthen0.so"


def run_one(name: str) -> str:
    """One example in this process, cells joined with `|` per row."""
    example = json.loads(FILE.read_text())["examples"][name]
    connection = sqlite3.connect(":memory:")
    connection.enable_load_extension(True)
    connection.load_extension(str(LIB))
    try:
        for statement in example.get("setup") or []:
            connection.execute(statement)
        rows = connection.execute(example["sql"]).fetchall()
    except sqlite3.Error as failure:
        return f"error: {failure}"
    return "\n".join("|".join("" if cell is None else str(cell) for cell in row) for row in rows)


def main() -> int:
    if len(sys.argv) == 3 and sys.argv[1] == "--one":
        print(run_one(sys.argv[2]))
        return 0
    examples = json.loads(FILE.read_text())["examples"]
    failed = 0
    for name, example in examples.items():
        result = subprocess.run(
            [sys.executable, __file__, "--one", name],
            capture_output=True,
            text=True,
            cwd=HERE.parent,
            env={**os.environ, "ENGINE_NULL": "1"},
            check=False,
        )
        got = (result.stdout + result.stderr).strip()
        want = example["expected"]
        if want in got:
            print(f"ok       {name}")
        else:
            print(f"FAILED   {name}: want {want!r}, got {got!r}")
            failed += 1
    print(f"{len(examples) - failed} of {len(examples)} examples ok")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
