"""The third review's offline findings for the Python surface.

From ``sdlc/issues/2026-09-22-surfaces-branch-third-review-the-unheld-fixes.md``:

- item 21, one deadline spelling everywhere: -1 is the only no-deadline
  sentinel, every other negative refuses, zero is a spent deadline;
- item 22, the question builder silently dropped arguments that did not
  belong to the chosen verb;
- item 22, the pandas advice in the docstrings, run exactly as written.

Run offline: ``ENGINE_NULL=1 .venv/bin/python -m pytest tests/test_review3_offline.py -q``
"""

import os

import pytest

os.environ.setdefault("ENGINE_NULL", "1")

import pandas as pd  # noqa: E402
import polars as pl  # noqa: E402

import thinkthen as tt  # noqa: E402

ASK = "Does the customer ask for a refund?"
TEXT = "I want a refund now"


# --- item 21: one deadline spelling --------------------------------------


def test_minus_one_is_the_no_deadline_sentinel():
    assert tt.decide(ASK, TEXT, deadline=-1.0) is True


def test_every_other_negative_refuses():
    with pytest.raises(tt.UsageError, match="negative"):
        tt.decide(ASK, TEXT, deadline=-2.0)
    with pytest.raises(tt.UsageError, match="negative"):
        tt.decide(ASK, TEXT, deadline=-0.5)


def test_zero_is_a_spent_deadline_that_sends_nothing():
    with pytest.raises(tt.DeadlineError):
        tt.decide(ASK, TEXT, deadline=0.0)


def test_not_a_number_and_too_large_refuse():
    with pytest.raises(tt.UsageError):
        tt.decide(ASK, TEXT, deadline=float("nan"))
    with pytest.raises(tt.UsageError):
        tt.decide(ASK, TEXT, deadline=1e12)


# --- item 22: the builder names a dropped argument ------------------------


def test_the_builder_refuses_a_mismatched_argument():
    with pytest.raises(tt.UsageError, match="options"):
        tt.question(decide=ASK, options=["a", "b"])
    with pytest.raises(tt.UsageError, match="levels"):
        tt.question(choose="Which desk?", options=["a", "b"], levels=["x", "y"])
    with pytest.raises(tt.UsageError, match="labels"):
        tt.question(score="How urgent?", levels=["low", "high"], labels=["x"])
    with pytest.raises(tt.UsageError, match="true"):
        tt.question(tag="Which words?", labels=["a", "b"], true_="yes")
    with pytest.raises(tt.UsageError, match="false"):
        tt.question(score="How urgent?", levels=["low", "high"], false_="no")


def test_the_builder_refuses_file_beside_anything():
    set_path = "tests/fixture/form.json"
    with pytest.raises(tt.UsageError, match="file"):
        tt.question(file=set_path, decide=ASK)
    with pytest.raises(tt.UsageError, match="file"):
        tt.question(file=set_path, model="m")


def test_the_builder_still_builds_each_verb():
    assert tt.decide(tt.question(decide=ASK, threshold=0.5), TEXT) is True
    picked = tt.choose(
        tt.question(choose="Which desk?", options=["a", "b"]), TEXT
    )
    assert picked in {"a", "b", None}


# --- item 22: the docstrings' pandas advice, run as written ---------------


def test_the_decide_docstring_advice_round_trips():
    series = pd.Series([TEXT, "good morning"], dtype=object)
    answers = tt.decide(ASK, series)
    back = pd.Series(answers)
    assert back.tolist() == [True, False]


def test_the_annotate_docstring_advice_round_trips():
    frame = pd.DataFrame({"body": [TEXT, "good morning"], "id": [1, 2]})
    out = tt.annotate(
        "tests/fixture/form.json", pl.from_pandas(frame), on="body"
    ).to_pandas()
    assert type(out) is pd.DataFrame
    assert out["wants_refund"].tolist() == [True, False]
    # The engine leaves the tag question unanswered under the null
    # backend, and pandas spells a null answer NaN: the round trip is
    # honest, not silent — pinned here so a real drop cannot hide.
    assert out["team"].isna().all()
    assert out["urgency"].notna().all()
