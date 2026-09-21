"""The pandas checks: what actually works, pinned in tests.

The five checks come from
``sdlc/issues/2026-09-21-pandas-is-supported-only-when-the-library-team-proves-it.md``:
run on the stand-in, no pandas door built, no pandas import in the wheel.
The measured record, including the check-5 finding and the export-stability
table, lives in ``NOTES.md`` under "pandas checks". These tests pin the
verified behavior so a regression is a red test.

Run offline: ``ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_pandas_checks.py -q``
"""

import subprocess
import sys

import pandas as pd
import polars as pl
import pytest

import thinkthen as tt
from test_polars_door import buffers_of
from thinkthen._thinkthen import _arrow_probe

TEXTS = [
    "please refund this",
    "short",
    "I want a refund for order 9",
]
QUESTION = tt.question(decide="Does the customer ask for a refund?", threshold=0.9)


# ---------------------------------------------------------------------------
# Check 5 first, as the issue rules: a whole pandas frame into annotate.
# The finding: it raises, so it never half works, but the sentence names no
# fix — the issue's bar ("refuses with a sentence that names the fix") is not
# met. Do not bend the library: this test pins the truth until the product
# side rules on a fix-naming refusal.
# ---------------------------------------------------------------------------


def test_check_5_pandas_frame_refuses_and_never_half_works():
    frame = pd.DataFrame({"body": TEXTS, "id": [1, 2, 3]})
    with pytest.raises(ValueError) as caught:
        tt.annotate("tests/fixture/form.json", frame, on="body")
    # The message is pandas' own constructor error; it names no fix.
    assert "constructor" in str(caught.value)
    # No half work: the frame is untouched, no columns appeared, and the
    # judged text would not have been the column names.
    assert frame.columns.tolist() == ["body", "id"]


def test_check_5_single_column_frame_refuses_the_same_way():
    frame = pd.DataFrame({"body": TEXTS})
    with pytest.raises(ValueError, match="constructor"):
        tt.annotate("tests/fixture/form.json", frame, on="body")


# ---------------------------------------------------------------------------
# Check 1: a default pandas text column, object dtype, gives the list answers.
# On pandas 3 every Series carries the capsule, object dtype included, so the
# column takes the Arrow door; the answers must equal the list door's.
# ---------------------------------------------------------------------------


def test_check_1_object_dtype_answers_equal_the_list():
    series = pd.Series(TEXTS, dtype=object)
    assert hasattr(series, "__arrow_c_stream__")  # pandas 3: every Series exports
    assert tt.decide_many(QUESTION, series) == tt.decide_many(QUESTION, TEXTS)


def test_check_1_the_pure_python_container_is_unchanged():
    assert tt.decide_many(QUESTION, list(TEXTS)) == tt.decide_many(QUESTION, TEXTS)
    assert tt.decide(QUESTION, TEXTS[0]) is True


# ---------------------------------------------------------------------------
# Check 2: an Arrow-backed pandas column. The issue's spelling `str[pyarrow]`
# does not exist in pandas 3 (TypeError at Series construction); the working
# spellings are the default `str` and `string[pyarrow]`, and both export the
# same buffers twice, so the 205 address form holds for them. An object column
# re-converts on every export (measured in the notes), so no address form can
# hold there; its answers are check 1's.
# ---------------------------------------------------------------------------


def test_check_2_the_issue_spelling_is_rejected_by_pandas_3():
    with pytest.raises(TypeError):
        pd.Series(TEXTS, dtype="str[pyarrow]")


@pytest.mark.parametrize("dtype", [None, "string[pyarrow]"])
def test_check_2_arrow_backed_answers_and_the_205_address_form(dtype):
    series = pd.Series(TEXTS, dtype=dtype) if dtype else pd.Series(TEXTS)
    assert str(series.dtype) in ("str", "string")  # Arrow-backed under pandas 3
    assert tt.decide_many(QUESTION, series) == tt.decide_many(QUESTION, TEXTS)
    values_py, views_py, length = buffers_of(series)
    values_rust, views_rust, length_rust = _arrow_probe(series)
    assert length_rust == length == len(TEXTS)
    assert values_rust == values_py, (hex(values_rust), hex(values_py))
    assert views_rust == views_py, (hex(views_rust), hex(views_py))


# ---------------------------------------------------------------------------
# Check 4: what comes back. Every container returns a plain list of bare
# answers, because the bulk door returns judgments, not a re-created column;
# the one line to a pandas Series is pd.Series(answers). The test runs it.
# ---------------------------------------------------------------------------


def test_check_4_returned_type_and_the_conversion_line():
    object_column = pd.Series(TEXTS, dtype=object)
    arrow_column = pd.Series(TEXTS, dtype="string[pyarrow]")
    polars_column = pl.Series("body", TEXTS)
    for container in (TEXTS, object_column, arrow_column, polars_column):
        out = tt.decide_many(QUESTION, container)
        assert type(out) is list, type(out)
        assert all(isinstance(one, bool) for one in out)
        back = pd.Series(out)
        assert back.tolist() == tt.decide_many(QUESTION, TEXTS)
    assert pd.Series(tt.decide_many(QUESTION, arrow_column)).dtype == bool


# ---------------------------------------------------------------------------
# The import pin: importing the library must not import pandas (nor polars).
# The stronger absent-environment run is recorded in NOTES.md.
# ---------------------------------------------------------------------------


def test_import_does_not_pull_pandas_or_polars():
    code = (
        "import thinkthen, sys;"
        "assert 'pandas' not in sys.modules, 'pandas imported';"
        "assert 'polars' not in sys.modules, 'polars imported'"
    )
    done = subprocess.run(
        [sys.executable, "-c", code], capture_output=True, text=True, check=False
    )
    assert done.returncode == 0, done.stderr
