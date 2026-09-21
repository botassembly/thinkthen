"""The Python slide sample, run exactly as drawn.

The fixture around it is the Bash slides' own: the refund ticket, the
four reviews, and the same form.json. The checks print each result with
the slide's comment, and a mismatch is a finding, printed as MISMATCH,
never hidden.

Run from a folder holding form.json (check.sh copies the fixture there),
against the stub or the null backend.
"""

import pathlib
import sys

import polars

import thinkthen as tt

text = (
    "I renewed once this morning, but my card shows two charges.\n"
    "Please refund the duplicate."
)

reviews = [
    "Arrived a day early. Thank you!",
    "The zipper broke the first time I used it.",
    "Does this come in blue?",
    "Third time my order shows up late. Not okay.",
]

df = polars.DataFrame({"body": [text]})

# The slide, verbatim from
# repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/surfaces.md.

first = tt.decide("Does the customer ask for a refund?",
    text)  # True

refund = tt.question(
    decide="Does the customer ask for a refund?",
    threshold=(0.2, 0.8))
second = tt.decide(refund,
    "I was charged twice. Can you fix this?")  # None

complaints = tt.filter("Is this a complaint?", reviews)

# three new columns
df = tt.annotate("form.json", df, on="body")

# The report.

findings = []

print(f"decide          -> {first!r}   (comment: True)")
if first is not True:
    findings.append("decide: the comment did not reproduce")

print(f"band decide     -> {second!r}   (comment: None)")
if second is None:
    print("  band middle reproduced")
else:
    findings.append(
        "band decide: the comment says None, the run gave "
        f"{second!r}; against the three-bucket stub this evidence "
        "carries no keyword, so its probability falls below the band"
    )

print(f"filter          -> {complaints!r}   (comment: none drawn)")
print(f"annotate        -> columns {list(df.columns)}")
new = [name for name in df.columns if name != "body"]
if len(new) == 3:
    print(f"  three new columns: {new}")
    print(f"  first row: {df.row(0, named=True)}")
else:
    findings.append(f"annotate: expected three new columns, got {len(new)}")

if "--json" in sys.argv:
    print(df.write_json())

if findings:
    print()
    for finding in findings:
        print(f"MISMATCH  {finding}")
    print("findings are reported, not hidden; the slide changes, not the sample")

sys.exit(0)
