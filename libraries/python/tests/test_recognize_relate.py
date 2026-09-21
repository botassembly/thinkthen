"""`recognize` and `relate` through the Python surface, offline.

The acceptance test is the deck's own section in
`repos/mktg/decks/2026-09-21-thinkthen-semantic-commands/recognize-surfaces.md`.
Every call below is that page's Python section, run as written, and the
comments are asserted where a recording backs them. One call cannot run
as written — the deck asks the rule `located_in` on the Maria Chen
sentence, and the recording covers `works_for` and `based_in` — so that
gap is pinned in its own test as the known deck gap, filed with the
deck's owner. The stand-in answers only from the recordings; nothing
here is invented.
"""

import json

import pytest

import thinkthen as tt

TEXT = "Maria Chen joined Northwind Freight in Chicago last spring."
ALERTS = [
    "Checkout returns 500 at the payment step.",
    "Card charges are failing for every customer.",
    "The nightly export ran two hours late.",
    "The payments database ran out of disk space.",
]
SYNTH = "Le café 😀 Maria Chen arrived."


def test_the_deck_call_with_the_recorded_rules():
    """The deck's call and comments, with the recording's own rule names."""
    found = tt.recognize(
        TEXT,
        kinds=["person", "organization", "place"],
        relations={"works_for": ("person", "organization"),
                   "based_in": ("organization", "place")},
    )
    assert found.entities[0].text == "Maria Chen"
    assert found.entities[0].kind == "person"
    assert TEXT[found.entities[0].start:found.entities[0].end] == "Maria Chen"
    assert found.relations[0].name == "works_for"
    assert found.relations[0].source == 1
    assert found.relations[0].target == 2


def test_the_deck_call_as_written_and_the_located_in_gap():
    """The deck's own rule set cannot run yet; the refusal names why.

    The deck asks `located_in` on the Maria Chen sentence. The recording
    for that sentence covers `works_for` and `based_in`, and the stand-in
    never invents an answer, so the call refuses and names the covered
    rules. This is the known deck gap, filed with the deck's owner; the
    library is not bent around it.
    """
    with pytest.raises(tt.UsageError) as caught:
        tt.recognize(
            TEXT,
            kinds=["person", "organization", "place"],
            relations={"works_for": ("person", "organization"),
                       "located_in": ("*", "place")},
        )
    message = str(caught.value)
    assert "located_in" in message
    assert "works_for" in message and "based_in" in message


def test_offsets_slice_in_python_indexing():
    """An accented letter and an emoji before the name; Python counts
    code points, so text[start:end] is the name."""
    found = tt.recognize(SYNTH, kinds=["person"])
    entity = found.entities[0]
    assert (entity.start, entity.end) == (10, 20)
    assert SYNTH[entity.start:entity.end] == "Maria Chen"


def test_the_any_kind_end_is_the_star_string():
    found = tt.recognize(
        "The road from Hull to Leeds was closed.",
        kinds=["place"],
        relations={"located_in": ("*", "place")},
    )
    assert [entity.text for entity in found.entities] == ["Hull", "Leeds"]
    # The recording offers located_in on this sentence and it must refuse.
    assert found.relations == []


def test_a_named_end_outside_the_kinds_is_a_usage_error():
    with pytest.raises(tt.UsageError) as caught:
        tt.recognize(TEXT, kinds=["person"],
                     relations={"works_for": ("person", "organization")})
    assert "organization" in str(caught.value)


def test_the_deck_relate_call():
    edges = tt.relate(ALERTS, relations=["caused_by"], either=["same_as"],
                      threshold=0.9)
    first = edges[0]
    assert (first.name, first.source, first.target, first.probability) == (
        "caused_by", 1, 4, 0.94)
    assert [(edge.source, edge.target) for edge in edges] == [(1, 4), (2, 4)]
    # No kinds ride these recordings, so the kind fields stay absent.
    assert first.source_kind is None and first.target_kind is None


def test_relate_refuses_more_than_255_records():
    with pytest.raises(tt.UsageError) as caught:
        tt.relate(["x"] * 256, relations=["caused_by"])
    assert "255" in str(caught.value)


def test_recognize_question_file_goes_where_the_kinds_go(tmp_path):
    """The design page's question file, through the file door.

    The ruled spelling, landed by the core lane: a relation's ends in the
    question file are `source` and `target`, and `from` and `to` are
    refused with the ruled spelling named.
    """
    path = tmp_path / "recognize.json"
    path.write_text(json.dumps({
        "version": 1,
        "recognize": {
            "kinds": {"person": "A human being, by name.",
                      "organization": "A company.", "place": "A city."},
            "relations": [
                {"name": "works_for", "source": "person",
                 "target": "organization", "reads": "works for"},
                {"name": "based_in", "source": "organization",
                 "target": "place"},
            ],
        },
        "threshold": 0.4,
        "relation_threshold": 0.6,
    }))
    found = tt.recognize(TEXT, kinds=str(path))
    assert [entity.text for entity in found.entities] == [
        "Maria Chen", "Northwind Freight", "Chicago"]
    assert [relation.name for relation in found.relations] == ["works_for"]


def test_relate_question_file(tmp_path):
    path = tmp_path / "relate.json"
    path.write_text(json.dumps({
        "version": 1,
        "relate": {
            "relations": ["caused_by"],
            "either": ["same_as"],
        },
        "threshold": 0.9,
    }))
    edges = tt.relate(ALERTS, relations=str(path))
    assert [(edge.name, edge.source, edge.target) for edge in edges] == [
        ("caused_by", 1, 4), ("caused_by", 2, 4)]


def test_recognize_over_a_polars_frame_is_a_long_frame():
    pl = pytest.importorskip("polars")
    frame = pl.DataFrame({"body": [TEXT, "Amara Okafor founded Kestrel Labs in 2015."]})
    long = tt.recognize(frame, on="body")
    assert isinstance(long, pl.DataFrame)
    assert long.columns == ["row", "text", "kind", "start", "end", "strength"]
    first = long.filter(pl.col("row") == 1)
    assert first["text"].to_list() == ["Maria Chen", "Northwind Freight", "Chicago"]
    # Every offset slices its own row's text in Python indexing.
    for row in long.iter_rows(named=True):
        body = frame["body"][row["row"] - 1]
        assert body[row["start"]:row["end"]] == row["text"]
    assert long["row"].to_list() == [1, 1, 1, 2, 2]


def test_recognize_frame_form_refuses_relation_rules():
    pl = pytest.importorskip("polars")
    frame = pl.DataFrame({"body": [TEXT]})
    with pytest.raises(tt.UsageError) as caught:
        tt.recognize(frame, on="body",
                     relations={"works_for": ("person", "organization")})
    assert "text" in str(caught.value)


def test_relate_over_a_polars_frame_is_a_frame_of_edges():
    pl = pytest.importorskip("polars")
    frame = pl.DataFrame({"body": ALERTS})
    edges = tt.relate(frame, on="body", relations=["caused_by"], threshold=0.9)
    assert isinstance(edges, pl.DataFrame)
    assert edges.columns == ["name", "source", "target", "probability"]
    assert edges.row(0) == ("caused_by", 1, 4, 0.94)
    assert edges["source"].to_list() == [1, 2]


def test_the_frame_forms_name_the_list_form_for_a_plain_list():
    with pytest.raises(tt.UsageError) as caught:
        tt.recognize(TEXT, on="body")
    assert "text" in str(caught.value)
    with pytest.raises(tt.UsageError) as caught:
        tt.relate(ALERTS, on="body")
    assert "list" in str(caught.value)


def test_a_real_string_result_column_rides_the_frame_door_whole():
    """The frame door's string buffers, proven with a value that lands.

    `annotate`'s choose results are unresolved against the offline
    backend, so the frame path's text buffers were never exercised with a
    real value. A tag question answers offline, so this pins the fixed
    offsets convention: one stray leading byte here means the buffer
    slicing is off by one.
    """
    pl = pytest.importorskip("polars")
    import pathlib
    import tempfile
    path = pathlib.Path(tempfile.mkdtemp()) / "tagset.json"
    path.write_text(json.dumps({"version": 1, "questions": {
        "kinds": {"tag": "Which labels fit this ticket?",
                  "labels": ["urgent", "late"]}}}))
    out = tt.annotate(str(path), pl.DataFrame({"body": ["Please refund order 9."]}),
                      on="body")
    assert out["kinds"][0] == "[]"
