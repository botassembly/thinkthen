#!/usr/bin/env python3
"""Every PostgreSQL function's example, run as one test.

The file `examples.json` is keyed by function: the statement and the
answer the null backend gives, plus an optional setup. The site's
function pages and surface pages draw their SQL tab from this file
(site.md's one source), so an example nobody runs cannot reach the site.

Each example runs through its own psql invocation — one fresh backend
process — so the usage counters start at zero and every expected answer
is deterministic. The container already has the extension installed, the
null backend on, and the `@names.json` fixture in its data directory.

Usage: examples.py <container-name>
"""

from __future__ import annotations

import json
import pathlib
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
FILE = HERE.parent / "examples.json"
CONTAINER = sys.argv[1] if len(sys.argv) > 1 else "laneb-pg"


def psql(*statements: str) -> str:
    command = [
        "docker",
        "exec",
        "-e",
        "PGHOST=/run/postgresql",
        CONTAINER,
        "psql",
        "-U",
        "postgres",
        "-Atq",
    ]
    for statement in statements:
        command += ["-c", statement]
    done = subprocess.run(command, capture_output=True, text=True, check=False)
    out = done.stdout.strip()
    if done.stderr.strip():
        out += "\n" + done.stderr.strip()
    return out


def main() -> int:
    examples = json.loads(FILE.read_text())["examples"]
    failed = 0
    for name, example in examples.items():
        got = psql(*(example.get("setup") or []), example["sql"])
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
