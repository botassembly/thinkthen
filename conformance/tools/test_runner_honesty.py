#!/usr/bin/env python3
"""The conformance runners may not hide a verdict (surfaces-review-5).

Three rules, checked over every surface's runner source and the shared
table:

1. Only the shared table holds a case back. A runner never composes a
   `diverge` line of its own; the one allowed spelling passes the table's
   own disposition through (listed below by file and line text).
2. A runner never names a case id in a string literal, which is how a
   mismatch used to be excused by name.
3. A table `diverge` entry cites conformance/DIVERGENCES.md, and the
   validator refuses one that does not.

Usage: python3 conformance/tools/test_runner_honesty.py
"""
import json
import pathlib
import re
import subprocess
import sys
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[2]
RUNNERS = [
    "libraries/python/tests/conformance.py",
    "libraries/typescript/tests/conformance.test.mjs",
    "libraries/ruby/tests/conformance.rb",
    "libraries/r/conformance.R",
    "libraries/rust/examples/conformance.rs",
    "libraries/c/conformance_driver.py",
    "databases/duckdb/tools/conformance.py",
    "databases/sqlite/tests/conformance_driver.py",
    "databases/postgresql/runner.py",
]
# The table's own disposition, passed through unchanged: allowed.
PASSTHROUGH = {
    ("libraries/rust/examples/conformance.rs",
     'Err(Outcome::Diverge(reason)) => println!("diverge  {id}: {reason}"),'),
}
OWN_DIVERGE = re.compile(r"""["'`]diverge\s""")
CASE_ID = re.compile(r"""["'`]\d{2}-[a-z][a-z0-9-]+["'`]""")
COMMENT = re.compile(r"^\s*(#|//)")


def runner_problems(root):
    problems = []
    for name in RUNNERS:
        path = root / name
        for number, line in enumerate(path.read_text().splitlines(), start=1):
            if COMMENT.match(line):
                continue
            if OWN_DIVERGE.search(line) and (name, line.strip()) not in PASSTHROUGH:
                problems.append(f"{name}:{number}: composes its own diverge line")
            if CASE_ID.search(line):
                problems.append(f"{name}:{number}: names a case id in a literal")
    return problems


def validator_refuses_uncited_diverge():
    table = json.loads((ROOT / "conformance" / "conformance.json").read_text())
    table["skips"].append({
        "when": {"id": "18-cancel-mid-batch"},
        "surfaces": ["rust"],
        "as": "diverge",
        "why": "a divergence with no record",
    })
    with tempfile.TemporaryDirectory() as scratch:
        copy = pathlib.Path(scratch) / "conformance.json"
        copy.write_text(json.dumps(table))
        done = subprocess.run(
            [sys.executable, str(ROOT / "conformance" / "skiptable.py"), "validate", str(copy)],
            capture_output=True, text=True, check=False)
    return done.returncode != 0 and "cites no conformance/DIVERGENCES.md" in done.stderr


def main():
    failed = 0
    problems = runner_problems(ROOT)
    for problem in problems:
        print(f"FAIL {problem}")
    failed += bool(problems)
    if not problems:
        print("ok       every runner leaves holding back to the table and names no case id")
    if validator_refuses_uncited_diverge():
        print("ok       the table refuses a diverge entry that cites no record")
    else:
        print("FAIL the table accepted a diverge entry that cites no record")
        failed += 1
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
