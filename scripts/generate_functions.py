#!/usr/bin/env python3
"""Generate the surface function lists from `functions.toml`.

The maintainability test of 2026-09-21 measured a ninth function at 19
files touched by hand. The repeated part is the function list: the same
names, respelled in every surface's own language. This script reads the
one table (`functions.toml`) and writes the lists that are pure name
lists:

- `libraries/python/src/generated.rs` — the pyo3 registration block
- `libraries/typescript/index.mjs` — the ESM re-export face

Everything else stays hand-written: a function's body has its own shape
per verb, and a binding's signature has its own types per host. The
suggested path for a new function is in `SURFACES.md`.

Run with `--check` to verify the generated files match the table without
writing anything; `scripts/check_surfaces.sh` runs it that way.
"""

from __future__ import annotations

import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# The Python surface's one name difference: the extension registers
# `annotate_rows`, and the Python package wraps it as `annotate`.
PYTHON_ALIASES = {"annotate": "annotate_rows"}

PYTHON_HEADER = """\
//! Generated from `functions.toml` by `scripts/generate_functions.py`.
//!
//! Do not edit by hand: edit the table and run the generator. Every name
//! here is a `#[pyfunction]` in this crate, and a missing one fails the
//! build with a clear error.
"""

TYPESCRIPT_HEADER = """\
// The ESM face of the same package: one wrapper, two doors. The CommonJS
// file loads through createRequire, because Deno's ESM loader does not
// synthesize a default export for a sibling CommonJS file the way Node
// does.
//
// Generated from `functions.toml` by `scripts/generate_functions.py`.
// Do not edit by hand: edit the table and run the generator.
"""


def functions() -> list[dict[str, str]]:
    with (ROOT / "functions.toml").open("rb") as handle:
        table = tomllib.load(handle)
    return table["function"]


def python_source(rows: list[dict[str, str]]) -> str:
    lines = [PYTHON_HEADER]
    lines.append("\nuse pyo3::prelude::*;\n")
    lines.append("\n/// Register every function of the table on the extension module.\n")
    lines.append("pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {\n")
    for row in rows:
        held = PYTHON_ALIASES.get(row["name"], row["name"])
        lines.append(
            f"    module.add_function(wrap_pyfunction!(crate::{held}, module)?)?;\n"
        )
    lines.append("    Ok(())\n")
    lines.append("}\n")
    return "".join(lines)


def typescript_source(rows: list[dict[str, str]]) -> str:
    lines = [TYPESCRIPT_HEADER]
    lines.append("\nimport { createRequire } from 'node:module';\n\n")
    lines.append("const cjs = createRequire(import.meta.url)('./index.js');\n\n")
    lines.append("export const ThinkThenError = cjs.ThinkThenError;\n")
    for row in rows:
        lines.append(f"export const {row['name']} = cjs.{row['name']};\n")
    lines.append("export default cjs;\n")
    return "".join(lines)


TARGETS = {
    ROOT / "libraries/python/src/generated.rs": python_source,
    ROOT / "libraries/typescript/index.mjs": typescript_source,
}


def main() -> int:
    check = "--check" in sys.argv[1:]
    rows = functions()
    stale: list[str] = []
    for path, build in TARGETS.items():
        wanted = build(rows)
        if check:
            have = path.read_text() if path.exists() else ""
            if have != wanted:
                stale.append(str(path.relative_to(ROOT)))
            continue
        path.write_text(wanted)
        print(f"wrote {path.relative_to(ROOT)} ({len(rows)} functions)")
    if check and stale:
        print(
            "generated files are stale; run scripts/generate_functions.py: "
            + ", ".join(stale),
            file=sys.stderr,
        )
        return 1
    if check:
        print(f"generated files match functions.toml ({len(rows)} functions)")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
