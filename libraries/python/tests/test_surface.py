"""The Python surface's own checks, offline on the null backend."""

import subprocess
import sys

import pytest

import thinkthen as tt


def band_question():
    return tt.question(decide="Does the writer ask for a refund?", threshold=(0.2, 0.8))


def cut_question():
    return tt.question(decide="Does the writer ask for a refund?", threshold=0.5)


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
    assert ranked[0] == "i want a refund now"


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


def test_usage_counts_sends_and_resets():
    tt.reset_usage()
    tt.decide(cut_question(), "i want a refund")
    counted = tt.usage()
    assert counted["requests"] == 1
    tt.reset_usage()
    assert tt.usage()["requests"] == 0


def test_cancelled_is_a_keyboard_interrupt():
    # pyo3 gives an exception one base; cancelled rides KeyboardInterrupt,
    # the host's own cancel gesture, per the 211 proof.
    assert issubclass(tt.Cancelled, KeyboardInterrupt)


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
