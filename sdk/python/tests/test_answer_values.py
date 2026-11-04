"""Feature: Python results support the operations of their answers."""

from __future__ import annotations

from operator import eq, ne
from pathlib import Path
from typing import Any, cast

import pytest
from api_support import backend, client, isolated_environment, yes  # noqa: F401

import thinkthen as tt

pytestmark = pytest.mark.usefixtures("isolated_environment")


def test_no_decision_cannot_pass_an_assert(tmp_path: Path) -> None:
    with backend(tmp_path, yes(0.1)) as server, client(server) as selected:
        is_refund = selected.decide("Refund?", "Keep the order")
        assert is_refund.answer is False
        with pytest.raises(AssertionError):
            assert is_refund
        negative = False
        assert is_refund == negative
        assert str(is_refund) == "False"
        assert hash(is_refund) == hash(False)


def test_unsure_and_failed_answers_raise_only_when_used(tmp_path: Path) -> None:
    with backend(tmp_path, yes(0.5)) as server, client(server) as selected:
        is_refund = selected.decide("Refund?", "Maybe", threshold=(0.2, 0.8))
        assert is_refund.unsure and not is_refund.failed
        assert is_refund.answer is None
        with pytest.raises(tt.Unsure):
            bool(is_refund)
        invalid = selected.decide("Refund?", None)
        assert invalid.failed and not invalid.unsure
        assert invalid.answer is None
        with pytest.raises(tt.RecordError) as failure:
            str(invalid)
        assert invalid.error is not None and failure.value.code == invalid.error.code


@pytest.mark.parametrize("container", [list, tuple])
def test_single_calls_refuse_record_collections(
    container: type[list[str] | tuple[str, ...]],
) -> None:
    with tt.Client(backend="stub") as selected:
        with pytest.raises(TypeError, match="decide_stream"):
            selected.decide("Refund?", cast(tt.Record, container(["yes", "no"])))
        assert selected.decide("Refund?", {"items": ["yes", "no"]}).ok


def test_star_import_preserves_builtin_filter_and_default_client_is_configurable() -> (
    None
):
    import subprocess
    import sys

    subprocess.run(
        [
            sys.executable,
            "-c",
            (
                "from thinkthen import *; import builtins; assert filter is builtins.filter; "
                "from thinkthen.aio import *; assert filter is builtins.filter"
            ),
        ],
        check=True,
    )
    assert not hasattr(tt.Client, "from_env")
    assert not hasattr(tt.AsyncClient, "from_env")
    with tt.Client(backend="stub") as selected:
        tt.set_default_client(selected)
        assert tt.decide("Refund?", "body").source.backend is tt.Backend.STUB


@pytest.mark.parametrize(
    "selected,variable",
    [("systemone", "THINKTHEN_API_KEY"), ("openai", "OPENAI_API_KEY")],
)
def test_missing_key_names_the_variable(
    selected: tt.BackendInput, variable: str
) -> None:
    with pytest.raises(tt.UsageError, match=variable):
        tt.Client(backend=selected)


def test_scalar_and_collection_answers_keep_their_python_operations(
    tmp_path: Path,
) -> None:
    replies = [
        {
            "answer": {
                "type": "choice",
                "probabilities": {"billing": 0.9, "shipping": 0.1},
            }
        }
    ]
    with backend(tmp_path, replies) as server, client(server) as selected:
        team = selected.choose("Which team?", "body", choices=["billing", "shipping"])
        assert team == "billing" and hash(team) == hash("billing")
        assert str(team) == "billing" and len(repr(team)) < 120
        with pytest.raises(TypeError, match="compare"):
            bool(team)
        assert team.to_dict()["answer"] == "billing"
        assert str(team) == "billing"
    with tt.Client(backend="stub") as selected:
        urgency = selected.score("How urgent?", "body", levels=["low", "high"])
        assert 0 <= float(urgency) <= 1
        assert urgency >= 0 and urgency <= 1
        with pytest.raises(TypeError):
            bool(urgency)
        ranks = selected.rank("How urgent?", ["body"], levels=["low", "high"]).to_list()
        assert int(ranks[0]) == 1 and ["zero", "first"][ranks[0]] == "first"
        assert ranks[0] > 0 and ranks[0] < 2
        with pytest.raises(TypeError):
            bool(ranks[0])
        fitting = selected.tag("Which?", "body", labels=["billing", "shipping"])
        assert fitting == tuple(fitting)
        assert len(fitting) == len(fitting[:])
        if fitting:
            assert fitting[0] in fitting
        found = selected.find("Which line?", "first line\nsecond line")
        assert found and found == "second line"
        assert found.start == 11 and found.end == 22
        assert str(found) == "second line" and hash(found) == hash("second line")
        names = selected.recognize("Alice", kinds=["person"])
        assert names == tuple(names) and len(names) == len(names[:])
        relations = selected.relate(
            "Alice knows Bob", rules=[tt.RelationRule("person", "person", "knows")]
        )
        assert relations == tuple(relations) and len(relations) == len(relations[:])


def test_annotations_accept_mapping_and_toolbox_and_expose_child_answers(
    tmp_path: Path,
) -> None:
    questions = {"ready": tt.DecideQuestion("Ready?")}
    box = tt.Toolbox(questions)
    saved = tmp_path / "questions.json"
    saved.write_text(box.to_json(), encoding="utf-8")
    with tt.Client(backend="stub") as selected:
        for form in (
            questions,
            box,
            tt.Toolbox.load(saved),
            tt.AnnotateQuestion(questions),
        ):
            triage = selected.annotate(form, "body")
            assert list(triage) == ["ready"] and len(triage) == 1
            assert list(triage.keys()) == ["ready"]
            assert list(triage.values()) == [triage["ready"]]
            assert dict(triage.items()) == {"ready": triage["ready"]}
            assert triage == {"ready": triage["ready"].answer}
            assert bool(triage["ready"]) is triage["ready"].answer
            with pytest.raises(TypeError):
                bool(triage)


def test_host_frame_subclasses_and_async_calls_refuse_collection_inputs() -> None:
    import asyncio
    from collections.abc import AsyncIterator
    from typing import Any, cast

    import numpy as np
    import pandas as pd
    import polars as pl

    class Messages(pd.Series):
        pass

    async def messages() -> AsyncIterator[str]:
        yield "a"

    inputs = [
        ["a", "b"],
        ("a", "b"),
        pd.Series(["a"]),
        pd.DataFrame({"body": ["a"]}),
        pl.Series(["a"]),
        pl.DataFrame({"body": ["a"]}),
        Messages(["a"]),
        iter(["a"]),
        np.array(["a"]),
        messages(),
    ]
    with tt.Client(backend="stub") as selected:
        for record in inputs[2:]:
            with pytest.raises(TypeError, match=".thinkthen"):
                selected.decide("Refund?", cast(Any, record))

    async def check() -> None:
        async with tt.AsyncClient(backend="stub") as selected:
            for record in inputs:
                with pytest.raises(TypeError, match="decide_stream"):
                    await selected.decide("Refund?", cast(Any, record))

    asyncio.run(check())


def test_enum_and_literal_choice_sets_preserve_native_labels_and_enum_members(
    tmp_path: Path,
) -> None:
    from enum import Enum
    from types import MappingProxyType
    from typing import Literal

    class Team(Enum):
        BILLING = "billing"
        SHIPPING = "shipping"

    replies = [
        {
            "answer": {
                "type": "choice",
                "probabilities": {"billing": 0.9, "shipping": 0.1},
            }
        }
    ]
    with backend(tmp_path, replies) as server, client(server) as selected:
        question = tt.ChooseQuestion("Which?", choices=Team)
        team = selected.choose(question, "body")
        assert team.answer is Team.BILLING and team == Team.BILLING
        assert hash(team) == hash(Team.BILLING)
        assert team.to_dict()["answer"] == "billing"
        assert (
            selected.choose("Which?", "body", choices=Literal["billing", "shipping"])
            == "billing"
        )
        assert (
            selected.choose_stream("Which?", ["body"], choices=Team).to_list()[0].answer
            is Team.BILLING
        )
        box = tt.Toolbox({"team": question})
        assert selected.choose(box.choose("team"), "body").answer is Team.BILLING
        triage = selected.annotate(MappingProxyType({"team": question}), "body")
        assert triage["team"].answer is Team.BILLING
        assert triage.to_dict()["answer"]["team"]["answer"] == "billing"
        assert (
            selected.annotate(tt.AnnotateQuestion(box), "body")["team"].answer
            is Team.BILLING
        )
    with backend(tmp_path, yes(0.9)) as server, client(server) as selected:
        fitting = selected.tag("Which?", "body", labels=Team)
        assert fitting == (Team.BILLING, Team.SHIPPING)
        assert list(fitting) == [Team.BILLING, Team.SHIPPING]
    with tt.Client(backend="stub") as selected:
        assert selected.score("How much?", "body", levels=Literal["low", "high"]).ok
        assert selected.rank("How much?", ["body"], levels=Team).to_list()[0] == 1


def test_typed_question_metadata_does_not_change_repr_or_hash() -> None:
    questions = (
        tt.DecideQuestion("Ready?"),
        tt.ChooseQuestion("Which?", choices=["a", "b"]),
        tt.AnnotateQuestion({"ready": tt.DecideQuestion("Ready?")}),
    )
    for question in questions:
        assert type(question).__name__ in repr(question)
        assert isinstance(hash(question), int)


@pytest.mark.parametrize(
    "function,reply,options",
    [
        ("decide", {"probability": 0.5}, {"threshold": (0.2, 0.8)}),
        ("filter", {"probability": 0.5}, {"threshold": (0.2, 0.8)}),
        (
            "choose",
            {"distribution": {"first": 0.5, "second": 0.5}},
            {"choices": ["first", "second"], "threshold": 0.8},
        ),
        (
            "find",
            {"distribution": {"none": 0.2, "u1": 0.4, "u2": 0.4}},
            {"threshold": 0.8},
        ),
    ],
)
@pytest.mark.parametrize("failed", [False, True])
def test_comparisons_refuse_unsure_and_failed_answers(
    tmp_path: Path,
    function: str,
    reply: dict[str, Any],
    options: dict[str, Any],
    failed: bool,
) -> None:
    with backend(tmp_path, [reply]) as server, client(server) as selected:
        record = None if failed else "first\nsecond"
        if function == "filter":
            row = selected.filter_rows("Ready?", [record], **options).to_list()[0]
        else:
            row = getattr(selected, function)("Ready?", record, **options)
        assert row.status is (tt.Status.FAILED if failed else tt.Status.UNSURE)
        assert row.answer is None
        assert hash(row) == hash(None)
        expected_error = tt.RecordError if failed else tt.Unsure
        for comparison in (eq, ne):
            for value in (True, False, None, "first", row):
                for left, right in ((row, value), (value, row)):
                    with pytest.raises(expected_error) as raised:
                        comparison(left, right)
                    if failed:
                        assert isinstance(raised.value, tt.RecordError)
                        assert raised.value.code is tt.ErrorCode.RECORD_MISSING
        if not failed:
            answer_options = {**options, "threshold": 0.1}
            if function == "filter":
                answered_row = selected.filter_rows(
                    "Ready?", [record], **answer_options
                ).to_list()[0]
            else:
                answered_row = getattr(selected, function)(
                    "Ready?", record, **answer_options
                )
            value = True if function in {"decide", "filter"} else "first"
            assert answered_row.status is tt.Status.ANSWERED
            assert eq(answered_row, value) and eq(value, answered_row)
            assert ne(answered_row, value) is False
            assert ne(value, answered_row) is False
        # An answered result on the left must still validate the other result.
        with tt.Client(backend="stub") as answered_client:
            answered = answered_client.decide("Ready?", "evidence")
        for comparison in (eq, ne):
            with pytest.raises(expected_error):
                comparison(answered, row)


def test_answered_equality_keeps_plain_values_and_host_comparison_behavior() -> None:
    class Uncomparable:
        def __eq__(self, other: object) -> bool:
            raise RuntimeError("unrelated object")

    with tt.Client(backend="stub") as selected:
        row = selected.decide("Ready?", "evidence")
        assert row == row.answer and row.answer == row
        assert ne(row, row.answer) is False and ne(row.answer, row) is False
        assert row != Uncomparable()
        assert hash(row) == hash(row.answer)
