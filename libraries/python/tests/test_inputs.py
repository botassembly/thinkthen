"""What a call reads: refusals before any send, and the deadline rule."""

import pathlib

import thinkthen as tt
from conftest import child_env, run

PANDAS = ("thinkthen does not read pandas objects: the Python data frame is Polars. "
          "Pass a Polars Series or a list of str, such as series.tolist()")
ARROW = ("filter, rank, find, and relate read a list of str, not a column, and annotate and "
         "recognize read a column only from a Polars frame with on=. Pass column.to_list()")
DEADLINE_SENTENCE = "No deadline is spelled ``deadline=None`` or ``deadline=-1`` (ADR 0041)."
README = pathlib.Path(__file__).resolve().parents[1] / "README.md"

REFUSED = """
    import thinkthen as tt
    late = tt.question(decide="Is it late?")
    form = {"version": 1, "questions": {"late": {"decide": "Late?"}}}
    def said(call):
        try:
            call()
        except tt.ThinkThenError as error:
            print(type(error).__name__, error)
"""


def test_containers_are_refused_before_any_send(backend, tmp_path):
    """R1-6 and R2-11: pandas objects raise the pandas sentence, since pandas
    also exposes the Arrow stream. A list-only verb or ``annotate``
    without ``on=`` raises the Arrow sentence for a column. Nothing is sent."""
    printed = run(REFUSED + """
    import pandas, polars, pyarrow
    said(lambda: tt.decide_many(late, pandas.Series(["a"])))
    said(lambda: tt.filter(late, pandas.Series(["a"])))
    said(lambda: tt.annotate(form, pandas.DataFrame({"body": ["a"]})))
    said(lambda: tt.filter(late, pyarrow.array(["a"])))
    said(lambda: tt.rank("Late?", polars.Series(["a", "b"])))
    said(lambda: tt.annotate(form, polars.DataFrame({"body": ["a"]})))
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == 3 * [f"UsageError {PANDAS}"] + 3 * [f"UsageError {ARROW}"]
    assert backend.count() == 0


def test_every_record_is_read_before_the_first_send(backend, tmp_path):
    """Decision 7: a bad item names its index, and a generator is read whole,
    so a bad last item stops the call before its first record is sent."""
    printed = run(REFUSED + """
    said(lambda: tt.decide_many(late, (text for text in ["a", "b", 3])))
    said(lambda: tt.decide_many(late, ["a", "\\ud800"]))
    said(lambda: tt.decide_many(late, "one text"))
    said(lambda: tt.decide(late, b"bytes"))
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == [
        "UsageError record 2 is not a str",
        "UsageError record 1 holds a lone surrogate, which is not Unicode text",
        "UsageError the records are a list of str, not one str",
        "UsageError the evidence is a str",
    ]
    assert backend.count() == 0


def test_the_deadline_sentence_is_pinned_in_the_docstring_and_readme():
    """R5-7: both places a user reads carry the ADR 0041 spelling."""
    assert DEADLINE_SENTENCE in tt.__doc__
    assert DEADLINE_SENTENCE in README.read_text()


def test_deadlines_follow_adr_0041(backend, tmp_path):
    """R5-7, R5-8, R2-10, and R1-11: a bool, numpy's bool, a number past
    the cap, infinity, and NaN are usage errors; zero is a spent deadline;
    none of them sends. Then ``-1`` and ``None`` each run with no deadline."""
    printed = run(REFUSED + """
    import numpy
    for deadline in (True, False, numpy.bool_(True), "1", -2, 4294967296, 1e300,
                     float("inf"), float("nan"), 0):
        said(lambda: tt.decide(late, "a note", deadline=deadline))
    print(tt.decide(late, "no deadline", deadline=-1), tt.decide(late, "none", deadline=None))
    """, child_env(backend, tmp_path))
    spelled = "UsageError deadline is seconds from now, a number; no deadline is spelled None or -1"
    budget = "is not -1, 0, or a positive budget of at most 4294967295 seconds"
    assert printed.splitlines() == 4 * [spelled] + [
        f"UsageError a deadline of -2 seconds {budget}",
        f"UsageError a deadline of 4294967296 seconds {budget}",
        f"UsageError a deadline of {10**300} seconds {budget}",
        f"UsageError a deadline of inf seconds {budget}",
        f"UsageError a deadline of NaN seconds {budget}",
        "DeadlineError the deadline of 0 s passed before the call answered",
        "True True",
    ]
    assert backend.count() == 2
