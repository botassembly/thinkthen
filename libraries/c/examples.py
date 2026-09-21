#!/usr/bin/env python3
"""Every C function's example, run as one test.

The file `examples.json` is keyed by function: the call and the answer the
null backend gives. The site's function pages and surface pages draw their
C tab from this file (site.md's one source), so an example nobody runs
cannot reach the site.

The example program `examples/functions.c` runs all ten; this runner
checks that every snippet appears in it verbatim, compiles it with a plain
cc against the built door, runs it offline, and compares each expected
line. Run through `./check.sh`.
"""

from __future__ import annotations

import json
import os
import pathlib
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
FILE = HERE / "examples.json"
SOURCE = HERE / "examples" / "functions.c"
BINARY = HERE / "build" / "functions"
INCLUDE = HERE.parent.parent / "contract" / "include"


def main() -> int:
    data = json.loads(FILE.read_text())
    examples = data["examples"]
    source = SOURCE.read_text()

    failures = []
    for name, example in examples.items():
        snippet = example["c"]
        if snippet not in source:
            failures.append(f"{name}: the snippet is not in examples/functions.c")

    subprocess.run(
        [
            "cc", "-std=c11", "-Wall", "-Wextra", "-I", str(INCLUDE),
            str(SOURCE), "-o", str(BINARY), "-L", str(HERE / "target" / "release"),
            "-lthinkthen", "-Wl,-rpath," + str(HERE / "target" / "release"),
        ],
        check=True,
    )
    run = subprocess.run(
        [str(BINARY)],
        capture_output=True,
        text=True,
        env={**os.environ, "ENGINE_NULL": "1"},
        check=False,
    )
    output = run.stdout + run.stderr
    if run.returncode != 0:
        failures.append(f"the example program exited {run.returncode}: {output}")

    for name, example in examples.items():
        if example["expected"] not in output:
            failures.append(
                f"{name}: expected {example['expected']!r} in the output"
            )

    for failure in failures:
        print(f"FAILED   {failure}")
    if failures:
        return 1
    print(f"{len(examples)} of {len(examples)} examples ok")
    return 0


if __name__ == "__main__":
    sys.exit(main())
