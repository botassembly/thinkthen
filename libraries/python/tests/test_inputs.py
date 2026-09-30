"""What a call reads: refusals before any send, and the deadline rule."""

import pathlib

import thinkthen as tt
from conftest import child_env, clean_env, run

ARROW = ("filter, rank, find, and relate read a list of str, not a column, and annotate and "
         "recognize read a column only from a Polars or pandas frame with on=. "
         "Pass column.to_list()")
DEADLINE_SENTENCE = "Omit `deadline_ms` or pass -1 for no deadline."
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
    """A list-only verb or ``annotate`` without ``on=`` raises the Arrow
    sentence for a column. Nothing is sent. The pandas rows of R1-6 and
    R2-11 are the edge table's in ``test_pandas.py``."""
    printed = run(REFUSED + """
    import polars, pyarrow
    said(lambda: tt.filter(late, pyarrow.array(["a"])).value)
    said(lambda: tt.rank("Late?", polars.Series(["a", "b"])).value)
    said(lambda: tt.annotate(form, polars.DataFrame({"body": ["a"]})).value)
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == 3 * [f"UsageError {ARROW}"]
    assert backend.count() == 0


def test_eager_records_are_read_before_the_first_send(backend, tmp_path):
    """A bad item in an eager ordered input names its index before sending.
    A true iterator is lazy and has a separate stream boundary proof."""
    printed = run(REFUSED + """
    said(lambda: tt.decide(late, ["a", "b", 3]).value)
    said(lambda: tt.decide(late, ["a", "\\ud800"]).value)
    said(lambda: tt.filter(late, "one text").value)
    said(lambda: tt.decide(late, b"bytes").value)
    """, child_env(backend, tmp_path))
    assert printed.splitlines() == [
        "UsageError record 2 is not a str",
        "UsageError record 1 holds a lone surrogate, which is not Unicode text",
        "UsageError filter reads records; pass a list or iterator",
        "UsageError the input is text, an ordered collection, a column, or an iterator",
    ]
    assert backend.count() == 0


def test_missing_key_names_a_remedy_for_a_library_call():
    """A keyless library call keeps its Usage kind and names a usable remedy."""
    printed = run(REFUSED + """
    said(lambda: tt.Engine(base_url="https://127.0.0.2:9/v1", cache=False)
         .decide(late, "one").value)
    """, clean_env())
    assert printed.strip() == ("UsageError no key is set; configure an API key for the engine "
                               "(THINKTHEN_API_KEY)")


def test_the_deadline_sentence_is_pinned_in_the_docstring_and_readme():
    """Both reader entry points carry the new millisecond spelling."""
    assert DEADLINE_SENTENCE.replace("`", "``") in tt.__doc__
    assert DEADLINE_SENTENCE in README.read_text()


def test_deadlines_follow_the_millisecond_boundary(backend, tmp_path):
    """Integral milliseconds and the old name have distinct refusals. Zero
    spends no send; -1, None and omission remove the bound."""
    printed = run(REFUSED + """
    import numpy
    for value in (True, False, "1", 1.5, numpy.bool_(True), float("inf"), float("nan"),
                  -2, 4294967295001, 0):
        said(lambda: tt.decide(late, "a note", deadline_ms=value).value)
    said(lambda: tt.decide(late, "a note", deadline=0).value)
    print(tt.decide(late, "no deadline", deadline_ms=-1).value,
          tt.decide(late, "no bound", deadline_ms=None).value,
          tt.decide(late, "omitted").value)
    """, child_env(backend, tmp_path))
    whole = "UsageError `deadline_ms` is a whole number of milliseconds"
    most = "milliseconds is not -1, 0, or a positive budget of at most 4294967295 seconds"
    assert printed.splitlines() == 7 * [whole] + [
        f"UsageError a deadline of -2 {most}", f"UsageError a deadline of 4294967295001 {most}",
        "DeadlineError the deadline of 0 s passed before the call answered",
        "UsageError use deadline_ms= instead of deadline=",
        "True True True",
    ]
    assert backend.count() == 3


def test_shared_keywords_refuse_unknown_wrong_and_repeated_fields_before_send(backend, tmp_path):
    """The public methods reach the shared settings grammar before a worker.
    A built question keeps its identity and cannot be rewritten by a keyword."""
    printed = run(REFUSED + """
    cases = (
        lambda: tt.decide("Q?", "one", madeup=1),
        lambda: tt.decide("Q?", "one", options=["a", "b"]),
        lambda: tt.choose("Q?", "one", levels=["a", "b"]),
        lambda: tt.score("Q?", "one", threshold=0.7, levels=["a", "b"]),
        lambda: tt.tag("Q?", "one", labels=["a", "b"], descriptions={"a": "A"}),
        lambda: tt.decide(late, "one", true="late"),
        lambda: tt.decide("Q?", "one", batch=None),
        lambda: tt.decide(late, "one", none=True),
    )
    for case in cases:
        said(lambda: case().value)
    """, child_env(backend, tmp_path))
    lines = printed.splitlines()
    assert len(lines) == 8 and all(line.startswith("UsageError ") for line in lines)
    assert lines[0] == "UsageError the settings key `madeup` does not exist"
    assert lines[1] == "UsageError the settings key `options` does not belong to this verb"
    assert lines[5] == "UsageError settings repeats `true` from the question or named arguments"
    assert lines[6] == "UsageError `batch` is `max` or a whole number of at least 1"
    assert lines[7] == "UsageError the settings key `none` does not belong to this verb"
    assert backend.count() == 0


def test_process_cap_accepts_unsigned_range_and_refuses_invalid_settings(backend, tmp_path):
    """Python's arbitrary-width integers reach the u64 process-cap domain
    only when representable. Zero is a valid cap that denies the first send."""
    printed = run(REFUSED + """
    print(isinstance(tt.Engine(cache=False, max_requests_total=(1 << 64) - 1), tt.Engine))
    for cap in (True, -1, 1 << 64):
        said(lambda: tt.Engine(cache=False, max_requests_total=cap))
    said(lambda: tt.Engine(cache=False, max_requests_total=0).decide(late, "one").value)
    """, child_env(backend, tmp_path))
    assert printed.splitlines()[:4] == ["True"] + 3 * [
        "UsageError max_requests_total is a whole number of 0 or more"]
    assert printed.splitlines()[4].startswith("UsageError ")
    assert backend.count() == 0
