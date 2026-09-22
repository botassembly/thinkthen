"""Punch-list item 3 for Python: canonical results and ownership.

The results are the host's own:

- A frame a call returns does not depend on the input frame's lifetime and
  does not share its mutable state; dropping the input or rebuilding it
  leaves the result untouched.
- The recognize records are native typed objects, and the list that holds
  them is the host's own; mutating it must not change the engine's next
  answer.
- Offsets count code points and slice the name on repeated names, with an
  accented letter and an emoji ahead of it.
"""

import gc

import polars as pl

import thinkthen as tt

MARIA = "Le café 😀 Maria Chen arrived."
TWICE = "Chicago sent a delegation in March, and Chicago hosted the reply in June."


def test_a_returned_frame_outlives_and_ignores_its_input():
    df = pl.DataFrame({"body": ["I want a refund for order 9", "hello team"]})
    out = tt.annotate("tests/fixture/form.json", df, on="body")
    # Drop the input and collect: the result must stand on its own.
    del df
    gc.collect()
    assert out.columns[:1] == ["body"] or "body" in out.columns
    assert out.height == 2
    # Rebuilding an input of the same name changes nothing about the held
    # result.
    rebuilt = pl.DataFrame({"body": ["zzz", "zzz"]})
    assert out["body"].to_list() == ["I want a refund for order 9", "hello team"]
    assert rebuilt["body"].to_list() == ["zzz", "zzz"]


def test_recognize_records_are_host_owned_typed_objects():
    text = TWICE
    first = tt.recognize(text, kinds=["place"])
    assert len(first.entities) == 2
    name = first.entities[0]
    # Typed accessors: the fields are attributes, never a re-parsed JSON
    # dict.
    assert name.text == "Chicago"
    assert name.kind == "place"
    assert isinstance(name.start, int)
    # Mutating the host's own list must not change the engine's next
    # answer.
    first.entities.clear()
    del first
    gc.collect()
    second = tt.recognize(text, kinds=["place"])
    assert len(second.entities) == 2
    assert second.entities[0].text == "Chicago"
    assert second.entities[0].start == 0
    assert second.entities[1].start == 40
    assert text[second.entities[1].start : second.entities[1].end] == "Chicago"


def test_offsets_count_code_points_with_accent_and_emoji():
    found = tt.recognize(MARIA, kinds=["person"])
    name = found.entities[0]
    assert MARIA[name.start : name.end] == "Maria Chen"


def test_relation_endpoints_are_typed_ids():
    found = tt.recognize(
        "Maria Chen joined Northwind Freight in Chicago last spring.",
        kinds=["person", "organization", "place"],
        relations={"works_for": ("person", "organization")},
    )
    relation = found.relations[0]
    assert relation.name == "works_for"
    assert relation.source == 1
    assert relation.target == 2
    assert isinstance(relation.probability, float)
