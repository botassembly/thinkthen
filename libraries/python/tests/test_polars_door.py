"""The Polars door, proven offline: parity, zero copy, layouts, refusals.

No network: ENGINE_NULL=1 is set by the caller. The width equality proof
lives in tests/bench_width_polars.py and runs against the loopback stub.
"""

import ctypes
import json
import subprocess
import sys

import polars as pl
import pytest

import thinkthen as tt
from thinkthen._thinkthen import _arrow_probe

# The ArrowArray/ArrowArrayStream structs, walked from Python with ctypes
# so the address proof needs no pyarrow: the same Series exports its own
# buffers, and Rust must report exactly those addresses.
LIBPY = ctypes.pythonapi
LIBPY.PyCapsule_GetPointer.restype = ctypes.c_void_p
LIBPY.PyCapsule_GetPointer.argtypes = [ctypes.py_object, ctypes.c_char_p]


class _Schema(ctypes.Structure):
    _fields_ = [
        ("format", ctypes.c_char_p), ("name", ctypes.c_char_p),
        ("metadata", ctypes.c_void_p), ("flags", ctypes.c_int64),
        ("n_children", ctypes.c_int64), ("children", ctypes.c_void_p),
        ("dictionary", ctypes.c_void_p), ("release", ctypes.c_void_p),
        ("private_data", ctypes.c_void_p),
    ]


class _Array(ctypes.Structure):
    _fields_ = [
        ("length", ctypes.c_int64), ("null_count", ctypes.c_int64),
        ("offset", ctypes.c_int64), ("n_buffers", ctypes.c_int64),
        ("n_children", ctypes.c_int64), ("buffers", ctypes.POINTER(ctypes.c_void_p)),
        ("children", ctypes.c_void_p), ("dictionary", ctypes.c_void_p),
        ("release", ctypes.c_void_p), ("private_data", ctypes.c_void_p),
    ]


class _Stream(ctypes.Structure):
    _fields_ = [
        ("get_schema", ctypes.c_void_p), ("get_next", ctypes.c_void_p),
        ("get_last_error", ctypes.c_void_p), ("release", ctypes.c_void_p),
        ("private_data", ctypes.c_void_p),
    ]


_GET_SCHEMA = ctypes.CFUNCTYPE(ctypes.c_int, ctypes.POINTER(_Stream), ctypes.POINTER(_Schema))
_GET_NEXT = ctypes.CFUNCTYPE(ctypes.c_int, ctypes.POINTER(_Stream), ctypes.POINTER(_Array))
_RELEASE = ctypes.CFUNCTYPE(None, ctypes.POINTER(_Array))


def buffers_of(series):
    """The values and offsets buffer addresses polars itself hands out."""
    capsule = series.__arrow_c_stream__()
    stream = ctypes.cast(
        LIBPY.PyCapsule_GetPointer(capsule, b"arrow_array_stream"),
        ctypes.POINTER(_Stream),
    )
    schema = _Schema()
    rc = _GET_SCHEMA(stream.contents.get_schema)(stream, ctypes.byref(schema))
    assert rc == 0, rc
    array = _Array()
    rc = _GET_NEXT(stream.contents.get_next)(stream, ctypes.byref(array))
    assert rc == 0, rc
    assert array.n_buffers >= 3, "a string-view series carries views and data buffers"
    values = array.buffers[2] or array.buffers[3]
    views = array.buffers[1]
    length = array.length
    _RELEASE(array.release)(ctypes.byref(array))
    return values, views, length


def test_a_failed_question_widens_its_column_to_text():
    """0054 on the frame door: a question with a failed member comes back
    as a text column carrying the ruled marker JSON, so good answers and
    the failure ride the same column and no failed cell reads as `false`
    or a bare `null`. The stand-in's fixture fails the last file-order
    question of the set, so the widened column is derived from the list
    door's own marker instead of naming a question here."""
    row = "order 4471: charged twice, please refund"
    other = "I want a refund today"
    rows = tt.annotate("tests/fixture/form.json", [row])
    failed = [
        name
        for name, value in rows[0].items()
        if isinstance(value, dict) and "failed" in value
    ]
    assert failed == ["urgency"], f"the fixture fails the last file-order question: {rows[0]}"
    name = failed[0]
    frame = pl.DataFrame({"body": [row, other]})
    out = tt.annotate("tests/fixture/form.json", frame, on="body")
    assert out.schema[name] == pl.String
    assert out[name][0] == json.dumps(rows[0][name], separators=(",", ":"))
    neighbour = tt.annotate("tests/fixture/form.json", [other])[0][name]
    spelled = {True: "true", False: "false"}.get(neighbour, str(neighbour))
    assert out[name][1] == spelled


def test_decide_many_parity():
    q = tt.question(decide="Does the customer ask for a refund?", threshold=0.9)
    texts = ["please refund this", "nothing here", "I want a refund", "quiet"]
    series = pl.Series("body", texts)
    assert tt.decide_many(q, series) == tt.decide_many(q, texts)


def test_annotate_parity_with_the_list_door():
    text = "I renewed once, but my card shows two charges. Please refund the duplicate."
    rows = tt.annotate("tests/fixture/form.json", [text])
    frame = pl.DataFrame({"body": [text]})
    out = tt.annotate("tests/fixture/form.json", frame, on="body")
    assert isinstance(out, pl.DataFrame)
    assert out.columns[0] == "body"
    assert set(out.columns[1:]) == set(rows[0].keys())
    for name, value in rows[0].items():
        assert out[name][0] == value, (name, out[name][0], value)
    # The original column rides through whole.
    assert out["body"][0] == text


def test_annotate_multi_row_and_multi_column_frame():
    texts = [
        "Please refund order 9.",
        "The parcel arrived late again.",
        "Where is my invoice?",
    ]
    frame = pl.DataFrame({"ticket": range(3), "body": texts})
    out = tt.annotate("tests/fixture/form.json", frame, on="body")
    assert out["ticket"].to_list() == [0, 1, 2]
    assert out["body"].to_list() == texts
    assert len(out) == 3


def test_zero_copy_buffers_match():
    texts = [
        "please refund this",
        "short",
        "I want a refund for order 9 now please",
    ]
    series = pl.Series("body", texts)
    values_py, views_py, length = buffers_of(series)
    values_rust, views_rust, length_rust = _arrow_probe(series)
    assert length_rust == length == len(texts)
    assert values_rust == values_py, (hex(values_rust), hex(values_py))
    assert views_rust == views_py, (hex(views_rust), hex(views_py))


def test_text_layouts_across_the_view_rule():
    # Inline (<=12) and reference (>12) strings, all one door; a blank
    # record refuses the same way the list door refuses it.
    texts = ["a", "short", "x" * 12, "y" * 13, "please refund order 9"]
    series = pl.Series("body", texts)
    q = tt.question(decide="Does the customer ask for a refund?", threshold=0.9)
    got = tt.decide_many(q, series)
    assert got == tt.decide_many(q, texts)
    assert got[-1] is True
    with pytest.raises(tt.UsageError):
        tt.decide_many(q, pl.Series("body", ["", "x"]))
    with pytest.raises(tt.UsageError):
        tt.decide_many(q, ["", "x"])


def test_the_inline_view_bytes_are_read_as_the_string():
    """The view layout's inline arm: a keyword string of 12 bytes or
    fewer sits inside the 16-byte view, not in a data buffer, so a reader
    that skipped it would judge these records on nothing. Every answer
    must equal the list door's, and the keyword must win."""
    texts = ["refund", "refund now", "maybe a refund", "no thanks"]
    q = tt.question(decide="Does the customer ask for a refund?", threshold=0.9)
    series = pl.Series("body", texts)
    assert tt.decide_many(q, series) == tt.decide_many(q, texts) == [True, True, True, False]


def test_nulls_are_refused_with_the_reason():
    series = pl.Series("body", ["please refund", None])
    q = tt.question(decide="Does the customer ask for a refund?")
    with pytest.raises(tt.UsageError) as caught:
        tt.decide_many(q, series)
    assert "null" in str(caught.value)


def test_non_text_column_is_refused_with_its_format():
    series = pl.Series("body", [1, 2, 3])
    q = tt.question(decide="Does the customer ask for a refund?")
    with pytest.raises(tt.UsageError) as caught:
        tt.decide_many(q, series)
    assert "format" in str(caught.value)


def test_annotate_on_a_missing_column_says_so():
    frame = pl.DataFrame({"other": ["text"]})
    with pytest.raises(tt.UsageError) as caught:
        tt.annotate("tests/fixture/form.json", frame, on="body")
    assert "body" in str(caught.value)


def test_annotate_on_a_plain_list_names_the_container_rule():
    with pytest.raises(tt.UsageError) as caught:
        tt.annotate("tests/fixture/form.json", ["text"], on="body")
    assert "Polars" in str(caught.value)


def test_the_wheel_never_imports_polars():
    probe = (
        "import sys, thinkthen; "
        "assert 'polars' not in sys.modules, sorted(m for m in sys.modules if 'polar' in m); "
        "print('clean')"
    )
    out = subprocess.run(
        [sys.executable, "-c", probe],
        capture_output=True,
        text=True,
        check=True,
        env={"ENGINE_NULL": "1", "PATH": "/usr/bin:/bin"},
        cwd=".",
    )
    assert "clean" in out.stdout


def test_the_decode_columns_run_as_the_deck_draws():
    """The deck's two column lines, as drawn.

    `df.with_columns(complaint=tt.decide(ask, df["body"]))` and
    `tt.score("How urgent?", df["body"], levels)` both pass a Polars
    column and get a column back — the first 32 wide through the batch
    spine, no Python loop anywhere.
    """
    df = pl.DataFrame({"body": ["i want a refund now", "good morning", "maybe later"]})
    ask = "Is this a complaint?"

    df = df.with_columns(complaint=tt.decide(ask, df["body"]))
    levels = ["Routine.", "Soon.", "Immediate."]
    urgency = tt.score("How urgent?", df["body"], levels)
    df = df.with_columns(urgency=urgency)

    assert df["complaint"].dtype == pl.Boolean
    assert df["complaint"].to_list() == [True, False, True]
    assert df["urgency"].dtype == pl.Float64
    assert df["urgency"].to_list() == [1.7, 0.99, 1.05]


def test_a_column_answer_carries_nulls_for_not_sure():
    """A band question's column keeps `None` rows as Arrow nulls."""
    refund = tt.question(
        decide="Does the customer ask for a refund?",
        threshold=(0.2, 0.8),
    )
    df = pl.DataFrame({"body": ["i want a refund now", "maybe later", "good morning"]})
    answers = tt.decide(refund, df["body"])
    assert answers.to_list() == [True, None, False]


def test_the_column_form_matches_the_list_form():
    """A column crosses at the same answers as a slice, same order."""
    ask = "Is this a complaint?"
    records = ["i want a refund now", "good morning", "maybe later"]
    column = tt.decide(ask, pl.Series("body", records))
    listed = tt.decide_many(ask, records)
    assert column.to_list() == listed


# ---------------------------------------------------------------------------
# The review of 2026-09-22, finding 3 and group 2: a frame arriving in
# several pieces, and the C interface's release rule. Both tests fail on
# the code before the fixes: the two-piece round trip came back as
# [1, 2, 1, 2] (one answer a distinct text, written row by row) and a
# pyarrow consumer aborted the process on an uncleared schema release.
# ---------------------------------------------------------------------------


def test_a_multi_piece_table_rides_out_whole():
    """A 2 + 3 piece table comes back as five distinct rows, one per row,
    through both consumers' own doors."""
    import pyarrow as pa
    from thinkthen import _thinkthen

    first = pa.table({"body": ["please refund order 1", "short note"]})
    second = pa.table({"body": ["please refund order 2", "second line", "third line"]})
    pieces = pa.Table.from_batches([first.to_batches()[0], second.to_batches()[0]])
    bodies = [
        "please refund order 1",
        "short note",
        "please refund order 2",
        "second line",
        "third line",
    ]

    stream = _thinkthen.annotate_stream("tests/fixture/form.json", pieces, "body")
    table = pa.RecordBatchReader.from_stream(stream).read_all()
    assert table.num_rows == 5
    assert len(table.to_batches()) == 2
    assert table.column("body").to_pylist() == bodies

    stream = _thinkthen.annotate_stream("tests/fixture/form.json", pieces, "body")
    frame = pl.DataFrame(stream)
    assert frame.shape == (5, 4)
    assert frame["body"].to_list() == bodies


def test_the_arrow_capsules_clear_the_release_pointer():
    """pyarrow consumes the array capsule; its C++ helpers abort the
    process when a release callback leaves the pointer set."""
    import pyarrow as pa
    from thinkthen import _thinkthen

    ask = tt.question(decide="Does the customer ask for a refund?", threshold=0.9)
    wrapper = _thinkthen.decide(ask, pl.Series("body", ["please refund this", "short"]))
    values = pa.array(wrapper)
    assert values.to_pylist() == [True, False]


def test_annotate_keeps_categorical_struct_and_list_columns():
    """The caller's other columns come back as themselves: a dictionary
    keeps its values, a struct its fields, a list its elements, and a
    null stays a null."""
    frame = pl.DataFrame(
        {
            "body": ["please refund order 1", "short note"],
            "id": [10, 11],
            "cat": pl.Series(["a", "b"], dtype=pl.Categorical),
            "st": pl.Series([{"x": 1}, {"x": 2}], dtype=pl.Struct({"x": pl.Int64})),
            "li": pl.Series([[1, 2], [3]], dtype=pl.List(pl.Int64)),
            "flag": [True, None],
        }
    )
    out = tt.annotate("tests/fixture/form.json", frame, on="body")
    assert out.columns[:6] == ["body", "id", "cat", "st", "li", "flag"]
    assert out.shape == (2, 9)  # six originals plus the form's three questions
    assert out["id"].to_list() == [10, 11]
    assert out["cat"].dtype == pl.Categorical
    assert out["cat"].to_list() == ["a", "b"]
    assert out["st"].dtype == pl.Struct({"x": pl.Int64})
    assert out["st"].to_list() == [{"x": 1}, {"x": 2}]
    assert out["li"].to_list() == [[1, 2], [3]]
    assert out["flag"].to_list() == [True, None]
