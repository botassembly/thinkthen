#!/usr/bin/env python3
"""Every Python function's example, run as one test.

The file `examples.json` is keyed by function: the call and the answer
the null backend gives, plus any question files the call names. The
site's function pages and surface pages draw their Python tab from this
file (site.md's one source), so an example nobody runs cannot reach the
site.

One fresh process per example keeps every expected answer
deterministic and keeps the engine's usage counters out of the way.
Offline, no stub, no key. Run by `check.sh`; the module in this file's
folder also runs it as a test.
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

DRIVER = """\
import thinkthen as tt
print(repr({expression}))
"""


def run(example: dict) -> str:
    """One example in its own fresh process, in a private directory so
    its question files resolve and nothing leaks between examples."""
    with tempfile.TemporaryDirectory() as workdir:
        for name, content in (example.get("files") or {}).items():
            Path(workdir, name).write_text(content)
        program = DRIVER.format(expression=example["python"])
        result = subprocess.run(
            [sys.executable, "-c", program],
            capture_output=True,
            text=True,
            cwd=workdir,
            env={**os.environ, "ENGINE_NULL": "1"},
            check=False,
        )
        return (result.stdout + result.stderr).strip()


def main() -> int:
    data = json.loads(FILE.read_text())
    examples = data["examples"]
    failed = 0
    for name, example in examples.items():
        want = example["expected"]
        output = run(example)
        got = output.splitlines()[-1] if output else ""
        if want in got:
            print(f"ok       {name}")
        else:
            print(f"FAILED   {name}: want {want!r}, got {got!r}")
            failed += 1
    print(f"{len(examples) - failed} of {len(examples)} examples ok")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
