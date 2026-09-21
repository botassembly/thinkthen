#!/usr/bin/env python3
"""Check every surface's public names against the ruled list.

Ruling 4 of 2026-09-21: `reset_usage` was a public name no ruling admitted,
and a convention nobody enforces is how it got in. The ruled function list
is `functions.toml`; the contract's record types and the six error kinds are
admitted on every host that carries them; and each surface's documented
helpers are listed once below, each with the document that names it. A name
found beyond a surface's expected set, or an expected name missing from it,
fails the check.

A surface whose documented model carries fewer than the fourteen ruled
functions states its missing set and the document that says so — for
example, C's JSON door carries the verbs that have no C function.

Run by `scripts/check_surfaces.sh`. Add a name by editing this table in the
same change that documents it.
"""

from __future__ import annotations

import ast
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent

# The ruled function list: the one table `functions.toml` holds.
with (ROOT / "functions.toml").open("rb") as handle:
    RULED = [row["name"] for row in tomllib.load(handle)["function"]]

# The contract's record types and error kinds, spelled per surface in each
# surface's own table below. They are contract names, not new surface names.
ERROR_KINDS = ["Usage", "Backend", "Deadline", "Local", "Cancelled", "Defect"]


def python_names() -> set[str]:
    tree = ast.parse((ROOT / "libraries/python/thinkthen/__init__.py").read_text())
    for node in tree.body:
        if isinstance(node, ast.Assign):
            for target in node.targets:
                if getattr(target, "id", "") == "__all__":
                    return {element.value for element in node.value.elts}
    raise SystemExit("python: no __all__ found")


def typescript_names() -> set[str]:
    text = (ROOT / "libraries/typescript/index.d.ts").read_text()
    return set(re.findall(r"^export function ([A-Za-z_]\w*)", text, re.M)) | set(
        re.findall(r"^export class ([A-Za-z_]\w*)", text, re.M)
    )


def ruby_names() -> set[str]:
    text = (ROOT / "libraries/ruby/lib/thinkthen.rb").read_text()
    methods = set(re.findall(r"^    def ([a-z_]\w*)", text.split("\n    private", 1)[0], re.M))
    constants = set(re.findall(r"^  ([A-Z]\w*) =", text, re.M))
    errors = set(re.findall(r"([A-Z]\w*Error|Cancelled)\b", text.split(".each", 1)[0]))
    return methods | constants | errors


def r_names() -> set[str]:
    namespace = (ROOT / "libraries/r/thinkthen/NAMESPACE").read_text()
    exports = set(re.findall(r'export\("?(tt_\w+)"?\)', namespace))
    pattern = re.search(r'exportPattern\("([^"]+)"\)', namespace)
    if pattern:  # any exportPattern must resolve to ruled names only
        defined = set(re.findall(r"^(tt_\w+) <- function", "\n".join(
            path.read_text() for path in (ROOT / "libraries/r/thinkthen/R").glob("*.R")
        ), re.M))
        prefix = pattern.group(1).lstrip("^").replace("\\", "")
        exports |= {name for name in defined if name.startswith(prefix)}
    return exports


def rust_names() -> set[str]:
    text = (ROOT / "libraries/rust/src/lib.rs").read_text()
    return set(re.findall(r"    pub fn ([a-z_]\w*)", text))


def c_names() -> set[str]:
    text = (ROOT / "contract/include/thinkthen.h").read_text()
    return set(re.findall(
        r"^\s*(?:const\s+)?(?:char|int|void|thinkthen_engine|thinkthen_answer)\s*\*?\s*(thinkthen_[a-z_]+)\s*\(",
        text, re.M,
    ))


def sql_names(source: Path) -> set[str]:
    names: set[str] = set()
    for path in source.rglob("*.rs"):
        text = path.read_text()
        # The SQL-visible names: registration strings and pg_extern
        # declarations. Entry points such as the C API init are symbols,
        # not SQL functions, and are skipped.
        names |= set(re.findall(r'"(thinkthen_[a-z_]+)"', text))
        names |= set(re.findall(r"fn (thinkthen_[a-z_]+)", text))
    return {name for name in names if not name.startswith("thinkthen_init")}


# Each row: extractor, the expected names in the surface's raw spelling, and
# the documents that name whatever is not simply a ruled function.
SURFACES: list[dict] = [
    {
        "name": "python",
        "extract": python_names,
        "expected": set(RULED)
        | {f"{kind}Error" for kind in ERROR_KINDS if kind != "Cancelled"}
        | {"Cancelled", "ThinkThenError", "Entity", "Relation", "Edge", "Recognized"},
        "doc": "records and errors: libraries/python/README.md and the module docstrings",
    },
    {
        "name": "typescript",
        "extract": typescript_names,
        "expected": set(RULED) | {"ThinkThenError"},
        "doc": "ThinkThenError: libraries/typescript/index.d.ts",
    },
    {
        "name": "ruby",
        "extract": ruby_names,
        "expected": set(RULED)
        | {
            # Question constructors and helpers, documented in
            # libraries/ruby/README.md "Question helpers and the tick".
            "set", "built", "with_tick",
            # The level-carrying and probability-carrying forms the
            # conformance runner and the slide check need; same section.
            "score_with_level", "decide_many_with_probabilities",
        }
        | {f"{kind}Error" for kind in ERROR_KINDS}
        | {"Entity", "Relation", "Edge", "Recognized"},
        "doc": "helpers: libraries/ruby/README.md",
    },
    {
        "name": "r",
        "extract": r_names,
        # R's bulk form is the vectorized tt_decide (ADR 0017 pick 8), so
        # decide_many is deliberately absent; everything else is ruled.
        "expected": {"tt_" + name for name in RULED if name != "decide_many"},
        "doc": "decide_many absent: libraries/r/README.md, ADR 0017 pick 8",
    },
    {
        "name": "rust",
        "extract": rust_names,
        "expected": (set(RULED)
        | {
            # The constructor and the options-carrying forms, documented in
            # sdlc/planning/libraries/rust.md and the engine's doc comments.
            "from_env", "from_settings",
            "decide_opts", "decide_with", "decide_many_opts",
            "recognize_opts", "relate_opts",
        })
        - {
            # Rust builds a question through the contract's `Question` type
            # and `IntoQuestion`, so no `question` method exists.
            "question",
        },
        "doc": "Question builder: the contract's Question type; extras: libraries/rust docs",
    },
    {
        "name": "c",
        "extract": c_names,
        # The C door carries the verbs with SQL-like hot paths as functions
        # and the rest through `thinkthen_call`; the header documents both.
        "expected": {
            "thinkthen_engine_new", "thinkthen_engine_free",
            "thinkthen_error_message", "thinkthen_error_retryable",
            "thinkthen_decide", "thinkthen_decide_many",
            "thinkthen_recognize", "thinkthen_relate",
            "thinkthen_call", "thinkthen_free_string",
        },
        "doc": "the door set: contract/include/thinkthen.h",
    },
    {
        "name": "duckdb",
        "extract": lambda: sql_names(ROOT / "databases/duckdb/src"),
        # No filter, rank, find, or decide_many functions; the page says so.
        "expected": {
            "thinkthen_" + name
            for name in ["decide", "probability", "choose", "score", "tag", "annotate",
                         "details", "usage", "warm", "recognize", "relate", "relations"]
        },
        "doc": "the SQL set: databases/duckdb/README.md",
    },
    {
        "name": "sqlite",
        "extract": lambda: sql_names(ROOT / "databases/sqlite/src"),
        "expected": {
            "thinkthen_" + name
            for name in ["decide", "choose", "score", "tag", "annotate", "details",
                         "usage", "warm", "recognize", "relate"]
        },
        "doc": "the SQL set: databases/sqlite/README.md",
    },
    {
        "name": "postgresql",
        "extract": lambda: sql_names(ROOT / "databases/postgresql/src"),
        "expected": {
            "thinkthen_" + name
            for name in ["decide", "decide_array", "probability", "choose", "score", "tag",
                         "annotate", "details", "usage", "warm", "recognize", "relate",
                         "relations"]
        },
        "doc": "the SQL set: databases/postgresql/README.md",
    },
]


def main() -> int:
    failures = 0
    for surface in SURFACES:
        found = surface["extract"]()
        expected = surface["expected"]
        extra = sorted(found - expected)
        missing = sorted(expected - found)
        if extra or missing:
            failures += 1
            print(f"FAIL {surface['name']}: {len(found)} public names vs {len(expected)} expected")
            for name in extra:
                print(f"  extra:   {name}")
            for name in missing:
                print(f"  missing: {name}")
            if extra:
                print(f"  a new public name needs a ruling and a line here ({surface['doc']})")
        else:
            print(f"ok {surface['name']}: {len(found)} names, all ruled or documented")
    if failures:
        print(f"the public name list fails on {failures} surface(s)", file=sys.stderr)
        return 1
    print("every surface's public names are ruled or documented")
    return 0


if __name__ == "__main__":
    sys.exit(main())
