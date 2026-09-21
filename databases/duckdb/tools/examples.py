#!/usr/bin/env python3
"""Every DuckDB function's example, run as one test.

The file `examples.json` is keyed by function: the statement and the
answer the null backend gives, plus an optional setup and any question
files the statement names. The site's function pages and surface pages
draw their SQL tab from this file (site.md's one source), so an example
nobody runs cannot reach the site.

One fresh CLI process per example keeps every expected answer
deterministic — the usage counters start at zero, and the stand-in replay
answers from the recordings alone. Offline, no stub, no key.
"""

from __future__ import annotations

import json
import os
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
FILE = ROOT / "examples.json"
EXTENSION = ROOT / "build" / "release" / "thinkthen.duckdb_extension"
DUCKDB = ROOT / "duckdb-bin" / "duckdb"


def run(example: dict) -> str:
    """One example in its own fresh process, in a private directory so
    its question files resolve and nothing leaks between examples."""
    with tempfile.TemporaryDirectory() as workdir:
        for name, content in (example.get("files") or {}).items():
            Path(workdir, name).write_text(content)
        init_lines = [f"LOAD '{EXTENSION}';"]
        init_lines.extend(example.get("setup") or [])
        init = Path(workdir, "init.sql")
        init.write_text("\n".join(init_lines) + "\n")
        result = subprocess.run(
            [
                str(DUCKDB),
                "-unsigned",
                "-noheader",
                "-list",
                "-init",
                str(init),
                "-c",
                example["sql"],
            ],
            capture_output=True,
            text=True,
            cwd=workdir,
            env={**os.environ, "ENGINE_NULL": "1"},
            check=False,
        )
        return result.stdout + result.stderr


def main() -> int:
    data = json.loads(FILE.read_text())
    examples = data["examples"]
    failed = 0
    for name, example in examples.items():
        want = example["expected"]
        got = run(example).strip()
        if want in got:
            print(f"ok       {name}")
        else:
            print(f"FAILED   {name}: want {want!r}, got {got!r}")
            failed += 1
    print(f"{len(examples) - failed} of {len(examples)} examples ok")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
