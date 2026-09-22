"""The second review's Python findings, pinned by tests.

From ``sdlc/issues/2026-09-22-surfaces-branch-second-review-new-defects-and-leftovers.md``:

- item 10, deadlines that silently mean none: ``-1`` was the C door's
  sentinel and Python copied it; only the explicit ``None`` means no
  deadline now, and every negative — the race-shaped ``end - now``
  included — is refused as a usage error;
- item 11, a refused frame runs the paid batch first: the offline witness
  here reads the engine's own request counter (the wire witness is
  ``test_review2_wire.py``);
- item 12, a sliced struct stream: the answers must land on the caller's
  rows, not the backing rows the root's offset names;
- item 17, refused inputs are never released: the harness records that
  the producer's batch and schema release pointers ran;
- the error-class leftovers: ``Cancelled`` is a ``ThinkThenError`` and
  the handler's own ``SystemExit`` is never turned into a cancel.

Run offline: ``ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_review2_findings.py -q``
"""

import os
import pathlib
import time

import pyarrow as pa
import pytest

os.environ.setdefault("ENGINE_NULL", "1")

import polars as pl
import thinkthen as tt
from sliced_struct_stream import SlicedStructStream
from thinkthen import _thinkthen

QUESTION = tt.question(decide="Does the customer ask for a refund?", threshold=0.9)
SET = str(pathlib.Path(__file__).with_name("fixture") / "form.json")


# ---------------------------------------------------------------------------
# Item 10: deadline semantics.
# ---------------------------------------------------------------------------


def test_the_c_doors_sentinel_is_not_a_deadline():
    # -1 secretly meant "no deadline"; it is refused now, and the message
    # names the one no-deadline spelling.
    for refused in (-1.0, -2.0):
        with pytest.raises(tt.UsageError) as caught:
            tt.decide(QUESTION, "please refund", deadline=refused)
        assert "None" in str(caught.value)
        assert "negative" in str(caught.value)


def test_a_computed_budget_that_landed_below_zero_is_refused():
    # The accident's own shape: `end - now` with the end already past.
    end = time.monotonic() - 1.0
    computed = end - time.monotonic()
    assert computed < 0
    with pytest.raises(tt.UsageError, match="negative"):
        tt.decide(QUESTION, "please refund", deadline=computed)


def test_zero_stays_a_spent_deadline():
    # The settled rule: zero is legal, spent before the call, and nothing
    # is sent.
    before = tt.usage()["requests"]
    with pytest.raises(tt.DeadlineError):
        tt.decide(QUESTION, "please refund", deadline=0.0)
    assert tt.usage()["requests"] == before


def test_no_deadline_is_the_explicit_none():
    assert tt.decide(QUESTION, "please refund", deadline=None) is True
    assert tt.decide_many(QUESTION, ["please refund"], deadline=None) == [True]


# ---------------------------------------------------------------------------
# Error classes: Cancelled rides both bases.
# ---------------------------------------------------------------------------


def test_cancelled_is_a_thinkthen_error_and_a_keyboard_interrupt():
    assert issubclass(tt.Cancelled, tt.ThinkThenError)
    assert issubclass(tt.Cancelled, KeyboardInterrupt)


# ---------------------------------------------------------------------------
# Item 12: a sliced struct stream answers the caller's rows.
# ---------------------------------------------------------------------------


def test_a_sliced_struct_stream_answers_the_callers_rows():
    stream = SlicedStructStream(offset=2, length=3)
    wanted_texts = stream.all_texts[2:5]
    wanted_ids = stream.ids[2:5]
    frame = _thinkthen.annotate_stream(SET, stream, "body")
    table = pa.RecordBatchReader.from_stream(frame).read_all()
    assert table.column("body").to_pylist() == wanted_texts
    assert table.column("id").to_pylist() == wanted_ids
    # The answers belong to the caller's texts, not the backing rows'.
    answered = table.column("urgency").to_pylist()
    expected = [row["urgency"] for row in tt.annotate(SET, wanted_texts)]
    assert answered == expected
    # The backing rows are different rows, so the assertions above can
    # fail: before the fix the answers came from these.
    assert table.column("body").to_pylist() != stream.all_texts[:3]
    assert table.column("id").to_pylist() != stream.ids[:3]


def test_a_whole_stream_still_answers_every_row():
    stream = SlicedStructStream(offset=0, length=5)
    frame = _thinkthen.annotate_stream(SET, stream, "body")
    table = pa.RecordBatchReader.from_stream(frame).read_all()
    assert table.column("body").to_pylist() == stream.all_texts
    assert table.column("id").to_pylist() == stream.ids


# ---------------------------------------------------------------------------
# Item 17: a refused input is released, not leaked.
# ---------------------------------------------------------------------------


def test_a_refused_frame_batch_is_released():
    # A null in the `on` column is refused while the batch is being read;
    # the batch's release must run as the refusal unwinds.
    stream = SlicedStructStream(
        texts=["please refund order 1", None, "short note"], record_releases=True
    )
    with pytest.raises(tt.UsageError):
        _thinkthen.annotate_stream(SET, stream, "body")
    assert stream.array_released
    assert stream.schema_released


def test_a_refused_frame_schema_is_released():
    # A missing column is refused before any batch is read; the schema the
    # producer handed over must still be released.
    stream = SlicedStructStream(record_releases=True)
    with pytest.raises(tt.UsageError):
        _thinkthen.annotate_stream(SET, stream, "nope")
    assert stream.schema_released


# ---------------------------------------------------------------------------
# Item 11, offline witness: the engine's own counter proves zero requests.
# ---------------------------------------------------------------------------


def test_a_refused_pandas_frame_makes_no_request():
    import pandas as pd

    frame = pd.DataFrame({"body": ["a", "b", "c"]})
    before = tt.usage()["requests"]
    with pytest.raises(tt.UsageError, match="Pass the column instead"):
        tt.annotate(SET, frame, on="body")
    assert tt.usage()["requests"] == before


def test_a_refused_pyarrow_table_makes_no_request():
    table = pa.table({"body": ["a", "b", "c"]})
    before = tt.usage()["requests"]
    with pytest.raises(tt.UsageError, match="Pass the column instead"):
        tt.annotate(SET, table, on="body")
    assert tt.usage()["requests"] == before


def test_a_polars_frame_still_works_after_the_refusals():
    frame = pl.DataFrame({"body": ["please refund order 1", "short note"]})
    out = tt.annotate(SET, frame, on="body")
    assert type(out) is pl.DataFrame
    assert out["body"].to_list() == ["please refund order 1", "short note"]
