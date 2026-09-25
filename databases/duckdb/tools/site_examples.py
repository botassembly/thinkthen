"""Run every drawn DuckDB example on the site through the stock CLI
(ticket 0110 decision 15, R5-24).

The examples come from a private deck, so this binding holds no copy of
their calls. Each drawn block runs from a folder holding the extension and
the question files the blocks name, on the generic loopback arm. A page on
the closed divergence list prints one line citing its issue instead.
"""

from __future__ import annotations

import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

from harness import EXTENSION, REPO_ROOT, Backend, child_env

CLI = os.environ["THINKTHEN_DUCKDB_CLI_PATH"]
EXAMPLES = REPO_ROOT / "site" / "src" / "data" / "examples"
ISSUE = "sdlc/issues/2026-09-24-duckdb-site-examples-draw-the-tag-shapes.md"
DIVERGES = {
    "recognize__duckdb.json": f"draws the tag's thinkthen_relations table shape; see {ISSUE}",
    "relate__duckdb.json": f"draws the tag's thinkthen_relate; ticket 0118 owns relate; see {ISSUE}",
}
FILES = {
    "form.json": {"version": 1, "questions": {"area": {"choose": "Which area is this about?", "options": ["export", "login", "billing"]}}},
    "refund.json": {"decide": "Does the writer ask for a refund?", "threshold": "0.2:0.8"},
    "names.json": {"version": 1, "recognize": {"kinds": {"person": "A person's name.", "organization": "An organization name."}}},
}


def main() -> int:
    failed = 0
    with Backend() as backend, tempfile.TemporaryDirectory() as folder:
        place = Path(folder)
        shutil.copy(EXTENSION, place / "thinkthen.duckdb_extension")
        for name, body in FILES.items():
            (place / name).write_text(json.dumps(body))
        for path in sorted(EXAMPLES.glob("*__duckdb.json")):
            example = json.loads(path.read_text())
            if example.get("status") != "drawn":
                continue
            if path.name in DIVERGES:
                print(f"diverges {path.name}: {DIVERGES[path.name]}")
                continue
            code = example["code"]
            if "LOAD" not in code:
                code = "LOAD './thinkthen.duckdb_extension';\n" + code
            done = subprocess.run(
                [CLI, "-unsigned", "-c", code],
                cwd=place,
                env=child_env(backend.base(), place / f"env-{path.stem}"),
                capture_output=True,
                text=True,
                timeout=120,
                check=False,
            )
            if done.returncode != 0 or "Error" in done.stderr:
                failed += 1
                print(f"FAIL {path.name}: {done.stderr.strip()[:300]}")
            else:
                print(f"ok   {path.name}")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
