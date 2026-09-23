"""Review-5: a deadline is seconds from now, a real number (ADR 0031).

Python's bool is an int, so `deadline=True` used to run as a one-second
deadline while the Node surface refused `true`. A bool (Python's or
NumPy's) or a string is now refused as the usage kind on every door,
the single call and the bulk one.

Run: ENGINE_NULL=1 python -m pytest tests/test_review5_deadline.py -q
"""

from pathlib import Path

import numpy as np
import pytest

import thinkthen as tt

ASK = "Does the customer ask for a refund?"
TEXT = "I was charged twice. I want a refund."
REFUSAL = "deadline is seconds from now, a number; no deadline is spelled None or -1"


@pytest.mark.parametrize("spelled", [True, False, np.True_, "5"])
def test_a_deadline_that_is_not_a_number_is_refused_as_usage(spelled):
    with pytest.raises(tt.UsageError) as single:
        tt.decide(ASK, TEXT, deadline=spelled)
    assert str(single.value) == REFUSAL
    assert single.value.kind == "usage"
    with pytest.raises(tt.UsageError) as bulk:
        tt.decide_many(ASK, [TEXT], deadline=spelled)
    assert str(bulk.value) == REFUSAL


@pytest.mark.parametrize("spelled", [None, -1, -1.0, 5, 2.5])
def test_the_ruled_spellings_still_run(spelled):
    assert tt.decide(ASK, TEXT, deadline=spelled) is True
    assert tt.decide_many(ASK, [TEXT], deadline=spelled) == [True]


def test_the_docs_state_the_ruled_spelling():
    # Review-5: the module docstring and the README said "a negative is
    # refused" after ADR 0031 made -1 the no-deadline sentinel.
    readme = (Path(__file__).parents[1] / "README.md").read_text()
    assert "`None` or `-1` is no deadline, zero is a spent one," in readme
    assert "No deadline is spelled ``deadline=None`` or ``deadline=-1`` (ADR 0031)." in tt.__doc__
    for text in (readme, tt.__doc__):
        assert "a negative is refused" not in text
        assert "a negative number is refused" not in text
