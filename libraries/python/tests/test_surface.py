"""The Python surface's own checks, offline on the null backend."""

import json
import os
import pathlib
import subprocess
import sys
import tempfile

import pytest

import thinkthen as tt


def band_question():
    return tt.question(decide="Does the writer ask for a refund?", threshold=(0.2, 0.8))


def cut_question():
    return tt.question(decide="Does the writer ask for a refund?", threshold=0.5)


def _partial_set():
    """The conformance case 74's set, built from parts so the test needs
    no file: the stand-in fails the last file-order question for exactly
    one record (SYNTHETIC_PARTIAL_RECORD)."""
    path = pathlib.Path(tempfile.mkdtemp()) / "partial.json"
    path.write_text(json.dumps({
        "version": 1,
        "questions": {
            "refund": {"decide": "Is this a refund request?", "threshold": 0.5},
            "topic": {"decide": "Is this a billing problem?", "threshold": 0.5},
        },
    }))
    return str(path)


def test_unsure_is_none():
    asked = band_question()
    assert tt.decide(asked, "Maybe I will ask for my money back, maybe not.") is None


def test_yes_and_no_are_booleans():
    asked = cut_question()
    assert tt.decide(asked, "i want a refund now") is True
    assert tt.decide(asked, "good morning") is False


def test_text_question_takes_the_default_cut():
    assert tt.decide("Does the writer ask for a refund?", "i want a refund now") is True


def test_failure_never_reads_as_false():
    with pytest.raises(tt.ThinkThenError) as seen:
        tt.decide(cut_question(), "this malformed line is not a request")
    assert seen.value.kind == "backend"
    assert isinstance(seen.value.retryable, bool)


def test_blank_question_is_usage():
    with pytest.raises(tt.UsageError):
        tt.question(decide="   ")


def test_bad_threshold_is_usage():
    with pytest.raises(tt.UsageError):
        tt.question(decide="Refund?", threshold=90)


def test_missing_file_is_local():
    with pytest.raises(tt.LocalError):
        tt.question(file="no-such-file.json")


def test_deadline_names_the_limit():
    with pytest.raises(tt.DeadlineError) as seen:
        tt.decide(cut_question(), "i want a refund now", deadline=0.0)
    assert "deadline of 0 s" in str(seen.value)


def test_a_past_deadline_is_spent_not_refused():
    """A zero budget is legal: the call sends nothing and returns the
    deadline kind, per the settled rule (the contract's own conversion)."""
    with pytest.raises(tt.DeadlineError) as seen:
        tt.decide(cut_question(), "i want a refund now", deadline=0.0)
    assert "deadline of" in str(seen.value)


def test_the_sentinel_is_refused_like_every_other_negative():
    """Minus one is the C door's sentinel, where a number has to stand for
    "none"; Python's one no-deadline spelling is ``None``, so the sentinel
    is refused with the rest of the negatives (second review, item 10)."""
    with pytest.raises(tt.UsageError) as seen:
        tt.decide(cut_question(), "i want a refund now", deadline=-1.0)
    assert "None" in str(seen.value)
    assert tt.decide(cut_question(), "i want a refund now", deadline=None) is True


def test_deadlines_that_cannot_be_budgets_are_usage_errors():
    """NaN, an infinity, and every negative — the sentinel included — are
    usage errors at this door. Before the review's fix the same values
    reached `Duration::from_secs_f64` and raised an error that `except
    ThinkThenError` did not catch; before the second review's fix, -1
    silently meant "no deadline" and a computed budget could land there."""
    for budget in (float("nan"), float("inf"), -0.001, -1.0, -2.0, 1e300):
        with pytest.raises(tt.UsageError):
            tt.decide(cut_question(), "i want a refund now", deadline=budget)


def test_a_built_question_plus_members_refuses_as_ambiguous():
    """The question carries its members once; naming them beside it is
    a usage error naming both."""
    asked = tt.question(choose="Which desk owns this?", options=["the refund desk", "anywhere else"])
    with pytest.raises(tt.UsageError) as seen:
        tt.choose(asked, "please route this ticket", options=["the refund desk", "anywhere else"])
    assert "options" in str(seen.value)
    with pytest.raises(tt.UsageError) as seen:
        tt.score(asked, "maybe later", levels=["low", "high"])
    assert "levels" in str(seen.value)


def test_filter_returns_the_records():
    records = ["i want a refund now", "good morning", "refund, please"]
    kept = tt.filter(cut_question(), records)
    assert kept == ["i want a refund now", "refund, please"]


def test_filter_refuses_a_band():
    with pytest.raises(tt.UsageError):
        tt.filter(band_question(), ["anything"])


def test_decide_many_keeps_order_and_answers():
    answers = tt.decide_many(
        cut_question(),
        ["i want a refund now", "good morning", "refund, please", "maybe later"],
    )
    assert answers == [True, False, True, True]


def test_rank_orders_most_likely_first():
    ranked = tt.rank(
        "Does the writer ask for a refund?",
        ["good morning", "i want a refund now", "maybe later"],
    )
    # The settled pair shape: the place in the input and the probability.
    assert ranked[0] == {
        "index": 1,
        "record": "i want a refund now",
        "probability": 0.97,
    }
    assert [one["index"] for one in ranked] == [1, 2, 0]


def test_find_returns_the_pair_or_none():
    found = tt.find(
        "Which unit asks for a refund?",
        ["good morning", "i want a refund now"],
    )
    assert found == {"index": 1, "unit": "i want a refund now", "probability": 0.97}
    # The stand-in's find judges each unit alone and the best wins — it has
    # no none arm, so `None` never comes back from it; the none case is
    # real-engine data (case 25's recorded divergence).


def test_choose_and_score_and_tag():
    picked = tt.choose(
        "Which desk owns this?",
        "please route this ticket",
        options=["the refund desk", "the maybe desk", "anywhere else"],
    )
    assert isinstance(picked, (str, type(None)))
    value = tt.score(
        "How strong is the claim?",
        "maybe later",
        levels=["low", "mid", "high"],
    )
    assert isinstance(value, float)
    held = tt.tag(
        "Which words appear?",
        "maybe later",
        labels=["the refund word", "the maybe word", "nothing at all"],
    )
    assert isinstance(held, list)


def test_details_carries_the_trail():
    held = tt.details(cut_question(), "i want a refund")
    assert held["probability"] == pytest.approx(0.97)
    assert held["model"] == "jev-latest"
    assert len(held["digest"]) == 64
    assert held["sends"] >= 1


def test_details_carries_the_requests_list_and_the_failure_count():
    """0053 and 0054 on the details shape: the ordered requests list
    (one 64-figure digest a logical request, a retry adds no element) and
    `failed_questions`, always present, zero for one good question."""
    held = tt.details(cut_question(), "i want a refund")
    assert isinstance(held["requests"], list)
    assert len(held["requests"]) == 1
    assert all(len(digest) == 64 for digest in held["requests"])
    assert held["failed_questions"] == 0


def test_annotate_preserves_the_good_answers_and_marks_the_failed_one():
    """The stand-in's one synthesized partial failure (0054): the reply
    answers one question and omits the last in file order, so its field
    carries the ruled marker in this host's spelling (a dict), never
    `None`, while the good fields answer as usual."""
    rows = tt.annotate(
        _partial_set(), ["order 4471: charged twice, please refund"]
    )
    assert rows[0] == {
        "refund": True,
        "topic": {
            "failed": {"kind": "backend", "cause": "missing_answer"}
        },
    }


def test_annotate_answers_everything_when_nothing_failed():
    rows = tt.annotate(
        _partial_set(), ["I want a refund for order 4471"]
    )
    assert rows[0] == {"refund": True, "topic": True}
    assert all(
        not (isinstance(value, dict) and "failed" in value)
        for value in rows[0].values()
    )


def test_usage_counts_sends():
    # No reset exists (ruling 4): a caller who wants fresh counters builds
    # a new engine; this test takes the difference across one send.
    before = tt.usage()["requests"]
    tt.decide(cut_question(), "i want a refund")
    assert tt.usage()["requests"] - before == 1


def test_cancelled_is_a_keyboard_interrupt_and_a_thinkthen_error():
    # Two bases now: KeyboardInterrupt, the host's own cancel gesture, and
    # ThinkThenError, this package's base, so either `except` catches a
    # cancel.
    assert issubclass(tt.Cancelled, KeyboardInterrupt)
    assert issubclass(tt.Cancelled, tt.ThinkThenError)


def test_question_parts_match_the_file_digest():
    """Parts and files give the same question, so the same digest."""
    from_parts = tt.question(
        decide="Does the customer ask for a refund?",
        threshold=(0.2, 0.8),
        true_="The customer asks for money back.",
        false_="Anything else, such as a question or a complaint.",
    )
    from_file = tt.question(file="tests/fixture/refund.json")
    assert from_parts.digest() == from_file.digest()


def test_fork_child_answers():
    """The fork check: a child forked after a call answers too."""
    code = (
        "import thinkthen as tt\n"
        "assert tt.decide('Does the writer ask for a refund?', 'i want a refund') is True\n"
        "import os\n"
        "pid = os.fork()\n"
        "if pid == 0:\n"
        "    assert tt.decide('Does the writer ask for a refund?', 'i want a refund') is True\n"
        "    os._exit(0)\n"
        "_, status = os.waitpid(pid, 0)\n"
        "assert status == 0\n"
    )
    done = subprocess.run(
        [sys.executable, "-c", code],
        capture_output=True,
        text=True,
        timeout=30,
    )
    assert done.returncode == 0, done.stderr


def test_the_fixture_fires_from_the_build_not_the_environment():
    """Finding 7, as the compile-time door left it: the gate builds the
    wheel with the stand-in's `synthetic-partial` feature, so the marker
    appears for exactly the named record with the old environment variable
    unset. A shipped wheel is a default build with no fixture code at all,
    and the stand-in's own default-build test
    (`the_env_variable_arms_nothing`) proves no environment variable can
    arm it."""
    os.environ.pop("ENGINE_SYNTHETIC_PARTIAL", None)
    rows = tt.annotate(
        _partial_set(), ["order 4471: charged twice, please refund"]
    )
    assert rows[0]["topic"] == {
        "failed": {"kind": "backend", "cause": "missing_answer"}
    }
