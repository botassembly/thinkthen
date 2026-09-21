"""The Polars door, proven offline: parity, zero copy, layouts, refusals.

No network: ENGINE_NULL=1 is set by the caller. The width equality proof
lives in tests/bench_width_polars.py and runs against the loopback stub.
"""

import ctypes
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
