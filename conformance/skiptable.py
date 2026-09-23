#!/usr/bin/env python3
"""The one skip-table reader every conformance runner calls.

The table's entries may select by any mix of five facets: `id` (one case),
`verb` (a string or list of case verbs), `kind` (an error kind), `form`
(a request form), and `record` (a row shape). An entry matches a case when
every facet it names matches that case's facets and the surface is listed
in the entry. This one implementation replaces the nine private readers
the third review found disagreeing on these fields (surfaces-review-3).

Exit codes and output, stable for script callers:
  lookup: prints one line - `RUN`, `SKIP<TAB>why`, or `DIVERGE<TAB>why` -
  exits 0 on a decision, 2 on a usage error.
  validate: checks every entry names known cases, verbs, and surfaces and
  carries a reason; exits nonzero with the reasons on stderr.

Usage:
  skiptable.py lookup SURFACE CASE_ID [--verb V] [--kind K] [--form F] [--record R]
  skiptable.py validate [CONFORMANCE_JSON]
"""
import json
import sys
from pathlib import Path

SURFACES = ("python", "typescript", "ruby", "r", "rust", "c", "duckdb", "sqlite", "postgresql")
FACETS = ("verb", "kind", "form", "record")


def load(path):
    here = Path(__file__).resolve().parent
    table = json.loads((here / (path or "conformance.json")).read_text())
    return table


def case_facets(table):
    """Map every case id to the facets its own record names."""
    facets = {}
    for case in table.get("cases", []):
        held = {key: case[key] for key in FACETS if key in case}
        facets[case["id"]] = held
    return facets


def as_list(value):
    if isinstance(value, list):
        return value
    return [value] if value is not None else []


def entry_facets(entry):
    when = entry.get("when", {})
    return {facet: as_list(when[facet]) for facet in FACETS if facet in when}


def entry_matches(entry, case_id, facets):
    """True when the entry selects exactly this case on every facet it names."""
    when = entry.get("when", {})
    if "id" in when:
        return when["id"] == case_id
    for facet, wanted in entry_facets(entry).items():
        held = facets.get(facet)
        if held is None or not any(one in as_list(held) for one in wanted):
            return False
    return bool(entry_facets(entry))


def validate(path):
    table = load(path)
    skips = table.get("skips", [])
    facets = case_facets(table)
    case_ids = set(facets)
    known = {facet: set() for facet in FACETS}
    for held in facets.values():
        for facet, value in held.items():
            known[facet].update(as_list(value))
    problems = []
    seen = set()
    for entry in skips:
        when = entry.get("when", {})
        selectors = [key for key in ("id", *FACETS) if key in when]
        if not selectors:
            problems.append(f"entry with no selector: {entry}")
            continue
        if "id" in when and when["id"] not in case_ids:
            problems.append(f"id {when['id']} names no case")
        # `verb` selectors name case fields and are checked against them;
        # `kind`, `form`, and `record` select run context the table cannot
        # cross-check, so only their presence is validated.
        for value in as_list(when.get("verb")):
            if value not in known["verb"]:
                problems.append(f"verb {value} names no case verb")
        # An entry that lists no surface applies to every surface.
        surfaces = entry.get("surfaces", [])
        for surface in surfaces:
            if surface not in SURFACES:
                problems.append(f"surface {surface} is not one of the nine")
            key = (surface, when.get("id"), tuple(sorted((k, tuple(v)) for k, v in entry_facets(entry).items())))
            if key in seen:
                problems.append(f"duplicate entry for {key}")
            seen.add(key)
        if not entry.get("why", "").strip():
            problems.append(f"entry {when} carries no reason")
    if problems:
        for problem in problems:
            print(f"skiptable: {problem}", file=sys.stderr)
        return 1
    print(f"skiptable: {len(skips)} entries valid")
    return 0


def lookup(surface, case_id, asked):
    if surface not in SURFACES:
        print(f"skiptable: surface {surface} is not one of the nine", file=sys.stderr)
        return 2
    table = load(None)
    facets = dict(case_facets(table).get(case_id, {}))
    facets.update({k: v for k, v in asked.items() if v is not None})
    for entry in table.get("skips", []):
        surfaces = entry.get("surfaces", [])
        if surfaces and surface not in surfaces:
            continue
        if not entry_matches(entry, case_id, facets):
            continue
        status = "DIVERGE" if entry.get("as") == "diverge" else "SKIP"
        why = entry.get("why", "").strip() or "no reason recorded"
        print(f"{status}\t{why}")
        return 0
    print("RUN")
    return 0


def main(argv):
    if len(argv) < 2:
        print(__doc__, file=sys.stderr)
        return 2
    if argv[1] == "validate":
        return validate(argv[2] if len(argv) > 2 else None)
    if argv[1] == "lookup" and len(argv) >= 4:
        surface, case_id, asked = argv[2], argv[3], {}
        rest = argv[4:]
        for index in range(0, len(rest) - 1, 2):
            flag = rest[index].lstrip("-")
            if flag in FACETS:
                asked[flag] = rest[index + 1]
        return lookup(surface, case_id, asked)
    print(__doc__, file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))
