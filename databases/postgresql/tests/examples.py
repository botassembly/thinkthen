#!/usr/bin/env python3
"""Run every example in `examples.json` and compare its whole output.

The site draws its SQL tab from that file, so an example nobody runs
cannot reach the site. Each example runs in its own psql session against
the local server check.sh started on the generic arm (ticket 0092).

Usage: examples.py <socket-directory>
"""

import json
import pathlib
import subprocess
import sys

sys.path.insert(0, str(pathlib.Path(__file__).resolve().parents[3] / "conformance" / "children"))
from children import child_env  # noqa: E402  the shared helper, ticket 0127

FILE = pathlib.Path(__file__).resolve().parents[1] / "examples.json"


def psql(socket: str, statements: list[str]) -> str:
    command = ["psql", "-X", "-q", "-At", "-h", socket, "-U", "postgres", "-d", "postgres"]
    for statement in statements:
        command += ["-c", statement]
    done = subprocess.run(command, capture_output=True, text=True, timeout=30, check=False,
                          env=child_env())
    return (done.stdout + done.stderr).strip()


def main() -> int:
    examples = json.loads(FILE.read_text())["examples"]
    failed = 0
    for name, example in examples.items():
        got = psql(sys.argv[1], [*example.get("setup", []), example["sql"]])
        if got == example["expected"]:
            print(f"ok       {name}")
        else:
            print(f"FAILED   {name}: want {example['expected']!r}, got {got!r}")
            failed += 1
    print(f"{len(examples) - failed} of {len(examples)} examples ok")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
