"""Compare the vendored drawn calls with the deck page they came from.

The deck is private, so the page is not in this repository and the gate
runs without it. When the page is on the machine, its DuckDB calls must
equal tools/drawn-calls/recognize.sql, or this fails and names the calls
that moved (review 5, R5-24: the copy drifted with only a remembered rule
to catch it). When the page is absent, this prints one counted skip.

The page is found at $THINKTHEN_DECK_PAGE, or else as the one
`*/decks/*/recognize-surfaces.md` among the repositories beside this one.
"""

import os
import pathlib
import re
import subprocess
import sys

HERE = pathlib.Path(__file__).resolve().parent
FIXTURE = HERE / "drawn-calls" / "recognize.sql"


def fold(call):
    return " ".join(call.split())


def find_page():
    named = os.environ.get("THINKTHEN_DECK_PAGE")
    if named:
        return [pathlib.Path(named)] if pathlib.Path(named).is_file() else []
    common = subprocess.run(
        ["git", "rev-parse", "--path-format=absolute", "--git-common-dir"],
        cwd=HERE, capture_output=True, text=True, stdin=subprocess.DEVNULL,
    )
    if common.returncode != 0:
        return []
    repos = pathlib.Path(common.stdout.strip()).parent.parent
    return sorted(repos.glob("*/decks/*/recognize-surfaces.md"))


def deck_calls(text):
    calls = []
    for block in re.findall(r"```sql\n(.*?)```", text, re.S):
        duck = re.search(r"-- DuckDB:.*?(?=\n-- (?:SQLite|PostgreSQL):|\Z)", block, re.S)
        if duck:
            calls += [fold(c) for c in re.findall(r"(SELECT[^;]*thinkthen_\w+[^;]*;)", duck.group(0), re.S)]
    return calls


pages = find_page()
if not pages:
    print("skip     drawn-calls-drift: the deck page is not on this machine (set THINKTHEN_DECK_PAGE to compare)")
    sys.exit(0)
if len(pages) > 1:
    print(f"FAILED   drawn-calls-drift: {len(pages)} deck pages match; set THINKTHEN_DECK_PAGE to the one to compare")
    sys.exit(1)
drawn = deck_calls(pages[0].read_text())
vendored = [fold(line) for line in FIXTURE.read_text().splitlines() if line.startswith("SELECT")]
if not drawn:
    print("FAILED   drawn-calls-drift: the deck page holds no DuckDB call; the reader no longer matches its layout")
    sys.exit(1)
if drawn != vendored:
    print("FAILED   drawn-calls-drift: the deck's DuckDB calls moved; vendor them again")
    for call in drawn:
        if call not in vendored:
            print(f"  in the deck only:   {call}")
    for call in vendored:
        if call not in drawn:
            print(f"  in the fixture only: {call}")
    sys.exit(1)
print(f"ok       drawn-calls-drift: the {len(drawn)} vendored calls equal the deck page's DuckDB calls")
