"""Results are safe values whose answers have one explicit read."""

from __future__ import annotations

from operator import eq, ge, gt, le, lt, ne
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
        with pytest.raises(TypeError, match="answer"):
            assert is_refund == negative
        assert str(is_refund) == "false"
        assert isinstance(hash(is_refund), int)


def test_unsure_reads_and_scalar_failure_have_distinct_boundaries(
    tmp_path: Path,
) -> None:
    with backend(tmp_path, yes(0.5)) as server, client(server) as selected:
        is_refund = selected.decide("Refund?", "Maybe", threshold=(0.2, 0.8))
        assert is_refund.unsure and not is_refund.failed
        with pytest.raises(tt.Unsure) as unsure:
            _ = is_refund.answer
        assert unsure.value.probability == 0.5
        assert isinstance(unsure.value, tt.NoAnswer)
        assert not isinstance(unsure.value, (ValueError, tt.ThinkThenError))
        assert str(is_refund) == "unsure"
        with pytest.raises(tt.Unsure):
            bool(is_refund)
        with pytest.raises(tt.Failed) as failure:
            selected.decide("Refund?", None)
        invalid = failure.value.rows[0]
        assert invalid.failed and not invalid.unsure
        assert isinstance(failure.value, (tt.NoAnswer, tt.ThinkThenError))
        assert str(invalid).startswith("failed: record.missing:")
        assert invalid.error is not None and failure.value.code == invalid.error.code


@pytest.mark.parametrize("container", [list, tuple])
def test_single_calls_refuse_record_collections(
    container: type[list[str] | tuple[str, ...]],
) -> None:
    with tt.Client(backend="stub") as selected:
        with pytest.raises(TypeError, match="decide_stream"):
            selected.decide("Refund?", cast(tt.Record, container(["yes", "no"])))
        assert selected.decide("Refund?", {"items": ["yes", "no"]}).answered


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
                "from thinkthen.aio import *; assert filter is builtins.filter; "
                "from thinkthen import aio; "
                "assert set_default_client is aio.set_default_client"
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
    with pytest.raises(tt.SetupError, match=variable):
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
        assert team.answer == "billing" and isinstance(hash(team), int)
        assert str(team) == "billing" and len(repr(team)) < 120
        with pytest.raises(TypeError, match="answer"):
            bool(team)
        assert team.to_dict()["answer"] == "billing"
        assert str(team) == "billing"
    with tt.Client(backend="stub") as selected:
        urgency = selected.score("How urgent?", "body", levels=["low", "high"])
        assert 0 <= float(urgency.answer) <= 1
        assert urgency.answer >= 0 and urgency.answer <= 1
        with pytest.raises(TypeError):
            bool(urgency)
        ranks = selected.rank("How urgent?", ["body"], levels=["low", "high"]).to_list()
        assert (
            int(ranks[0].answer) == 1 and ["zero", "first"][ranks[0].answer] == "first"
        )
        assert ranks[0].answer > 0 and ranks[0].answer < 2
        with pytest.raises(TypeError):
            bool(ranks[0])
        fitting = selected.tag("Which?", "body", labels=["billing", "shipping"])
        assert fitting.answer == list(fitting.answer)
        assert len(fitting.answer) == len(fitting.answer[:])
        if fitting.answer:
            assert fitting.answer[0] in fitting.answer
        found = selected.find("Which line?", "first line\nsecond line")
        assert found.answer is not None and found.answer.text == "second line"
        assert found.answer.start == 11 and found.answer.end == 22
        assert str(found) == "second line" and isinstance(hash(found), int)
        names = selected.recognize("Alice", kinds=["person"])
        assert names.answer == list(names.answer) and len(names.answer) == len(
            names.answer[:]
        )
        relations = selected.relate(
            "Alice knows Bob", rules=[tt.RelationRule("person", "person", "knows")]
        )
        assert relations.answer == list(relations.answer) and len(
            relations.answer
        ) == len(relations.answer[:])


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
            assert list(triage.answer) == ["ready"] and len(triage.answer) == 1
            assert list(triage.answer.keys()) == ["ready"]
            assert list(triage.answer.values()) == [triage.answer["ready"]]
            assert dict(triage.answer.items()) == {"ready": triage.answer["ready"]}
            assert triage.answer["ready"].answered
            assert bool(triage.answer["ready"]) is triage.answer["ready"].answer
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
        assert team.answer is Team.BILLING
        assert isinstance(hash(team), int)
        assert team.to_dict()["answer"] == "billing"
        assert (
            selected.choose(
                "Which?", "body", choices=Literal["billing", "shipping"]
            ).answer
            == "billing"
        )
        assert (
            selected.choose_stream("Which?", ["body"], choices=Team).to_list()[0].answer
            is Team.BILLING
        )
        box = tt.Toolbox({"team": question})
        assert selected.choose(box.choose("team"), "body").answer is Team.BILLING
        triage = selected.annotate(MappingProxyType({"team": question}), "body")
        assert triage.answer["team"].answer is Team.BILLING
        assert triage.to_dict()["answer"]["team"]["answer"] == "billing"
        assert (
            selected.annotate(tt.AnnotateQuestion(box), "body").answer["team"].answer
            is Team.BILLING
        )
    with backend(tmp_path, yes(0.9)) as server, client(server) as selected:
        fitting = selected.tag("Which?", "body", labels=Team)
        assert fitting.answer == [Team.BILLING, Team.SHIPPING]
        assert list(fitting.answer) == [Team.BILLING, Team.SHIPPING]
    with tt.Client(backend="stub") as selected:
        assert selected.score(
            "How much?", "body", levels=Literal["low", "high"]
        ).answered
        assert (
            selected.rank("How much?", ["body"], levels=Team).to_list()[0].answer == 1
        )


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
    _assert_comparisons_refuse_no_answer(tmp_path, function, reply, options, failed)


def test_filter_failed_rows_refuse_comparisons(tmp_path: Path) -> None:
    _assert_comparisons_refuse_no_answer(
        tmp_path, "filter", {"probability": 0.5}, {"threshold": 0.8}, True
    )


def _assert_comparisons_refuse_no_answer(
    tmp_path: Path,
    function: str,
    reply: dict[str, Any],
    options: dict[str, Any],
    failed: bool,
) -> None:
    with backend(tmp_path, [reply]) as server, client(server) as selected:
        record = None if failed else "first\nsecond"
        if function == "filter":
            with pytest.raises(tt.Failed) as failure:
                list(selected.filter("Ready?", [record], **options))
            row = failure.value.rows[0]
        else:
            row = getattr(selected, function + "_stream")(
                "Ready?", [record], **options
            ).to_list()[0]
        assert row.status is (tt.Status.FAILED if failed else tt.Status.UNSURE)
        expected_error = tt.Failed if failed else tt.Unsure
        with pytest.raises(expected_error):
            _ = row.answer
        assert eq(row, row) and isinstance(hash(row), int)
        assert str(row).startswith("failed:") if failed else str(row) == "unsure"
        for comparison in (eq, ne):
            for value in (True, False, None, "first"):
                for left, right in ((row, value), (value, row)):
                    with pytest.raises(TypeError, match="answer"):
                        comparison(left, right)
        with tt.Client(backend="stub") as answered_client:
            answered = answered_client.decide("Ready?", "evidence")
        assert answered != row and row != answered


def test_answered_equality_keeps_plain_values_and_host_comparison_behavior() -> None:
    class Uncomparable:
        def __eq__(self, other: object) -> bool:
            raise RuntimeError("unrelated object")

    with tt.Client(backend="stub") as selected:
        row = selected.decide("Ready?", "evidence")
        assert eq(row, row)
        for other in (row.answer, Uncomparable()):
            with pytest.raises(TypeError, match="answer"):
                _ = row == other
            with pytest.raises(TypeError, match="answer"):
                _ = row != other
        assert isinstance(hash(row), int)


def test_result_hash_uses_owned_values_and_never_the_mutable_item(
    tmp_path: Path,
) -> None:
    first_record: dict[str, Any] = {"id": "a", "text": "body", "caller_note": "first"}
    second_record: dict[str, Any] = {"id": "a", "text": "body", "caller_note": "second"}
    with backend(tmp_path, yes(0.9)) as server, client(server) as selected:
        first = selected.decide_stream(
            "Ready?", [first_record], field="/text"
        ).to_list()[0]
        second = selected.decide_stream(
            "Ready?", [second_record], field="/text"
        ).to_list()[0]
        assert first.item is first_record and second.item is second_record
        assert first == second
        stored = {first: "kept"}
        original_hash = hash(first)
        first_record.clear()
        second_record["text"] = "changed after judgment"
        assert first == second and hash(first) == original_hash
        assert stored[second] == "kept" and first in {second}
        assert "item" not in first.to_dict()
        with tt.Client(backend="stub") as tags_client:
            tags = tags_client.tag("Which?", "body", labels=["a", "b"])
        original_tags = tags.answer
        assert type(original_tags) is list
        snapshot = list(original_tags)
        tag_hash = hash(tags)
        original_tags.append("external")
        assert tags.answer == snapshot and hash(tags) == tag_hash and tags in {tags}
        questions = tt.AnnotationQuestions(
            {
                "refund": tt.DecideQuestion("Ready?"),
                "ready": tt.DecideQuestion("Ready?"),
            }
        )
        form = selected.annotate_stream(questions, [{"text": "body"}]).to_list()[0]
        assert type(form.answer) is dict
        form_set = {form}
        entries = form.answer
        entries.clear()
        form.item.clear()
        assert set(form.answer) == {"ready", "refund"}
        assert form in form_set
        assert all(isinstance(hash(child), int) for child in form.answer.values())
        assert form.raise_if_failed() is form


def test_removed_result_shortcuts_require_the_explicit_answer() -> None:
    from operator import index

    with tt.Client(backend="stub") as selected:
        names = selected.recognize("Alice", kinds=["person"])
        urgency = selected.score("How urgent?", "body", levels=["low", "high"])
        ranked = selected.rank(
            "How urgent?", ["body"], levels=["low", "high"]
        ).to_list()[0]
        form = selected.annotate({"ready": tt.DecideQuestion("Ready?")}, "body")
        span = selected.find("Which?", "first\nsecond")
        for value in (names, urgency, ranked, form, span):
            target: Any = value
            for operation in (iter, len, float, int, index):
                with pytest.raises(TypeError):
                    operation(target)
            for key in (0, slice(None), "ready"):
                with pytest.raises(TypeError):
                    _ = target[key]
            for comparison in (lt, le, gt, ge):
                with pytest.raises(TypeError):
                    comparison(target, 1)
            with pytest.raises(TypeError, match="answer"):
                bool(value)
        with pytest.raises(AttributeError):
            _ = cast(Any, span).start
        with pytest.raises(AttributeError):
            _ = cast(Any, span).end


@pytest.mark.parametrize(
    "probability,printed", [(0.0, "0"), (1.0, "1"), (0.0000001, "0.0000001")]
)
def test_score_result_prints_the_commands_numeric_words(
    tmp_path: Path, probability: float, printed: str
) -> None:
    reply = [
        {
            "answer": {
                "type": "score",
                "probabilities": {"0": 1 - probability, "1": probability},
            }
        }
    ]
    with backend(tmp_path, reply) as server, client(server) as selected:
        urgency = selected.score("How much?", "body", levels=["low", "high"])
        assert urgency.answer == probability
        assert str(urgency) == printed and f"{urgency}" == printed


def text_leaf_replies() -> list[dict[str, Any]]:
    kind = "person\tkind"
    company = "company"
    replies: list[dict[str, Any]] = [
        {
            "contains": "FAIL WITNESS",
            "status": 503,
            "body": {"error": "service\tunavailable\ntry\rlater"},
        }
    ]
    for word, tag in (
        ("Morgan", "BEGIN"),
        ("Lee", "INSIDE"),
        ("Jr", "END"),
        ("Example", "BEGIN"),
        ("Group", "END"),
    ):
        replies.append(
            {
                "contains": "[[" + word + "]]",
                "criterion": "BEGIN",
                "distribution": {
                    name: float(name == tag)
                    for name in ("BEGIN", "END", "INSIDE", "OUT", "SINGLE")
                },
            }
        )
    replies.extend(
        [
            {
                "criterion": "BEGIN",
                "distribution": {
                    "BEGIN": 0.0,
                    "END": 0.0,
                    "INSIDE": 0.0,
                    "OUT": 1.0,
                    "SINGLE": 0.0,
                },
            },
            {
                "contains": "[[Morgan",
                "criterion": company,
                "distribution": {"person": 1.0, company: 0.0, "none of these": 0.0},
            },
            {
                "criterion": company,
                "distribution": {"person": 0.0, company: 1.0, "none of these": 0.0},
            },
            {"criterion": kind, "distribution": {kind: 1.0, "none of these": 0.0}},
            {
                "criterion": "left\tchoice",
                "distribution": {"left\tchoice": 1.0, "right\nchoice": 0.0},
            },
            {"criterion": "u1", "distribution": {"u1": 1.0, "none": 0.0}},
            {"probability": 1.0},
        ]
    )
    return replies


def test_result_text_folds_string_leaves_and_preserves_wire_values(
    tmp_path: Path,
) -> None:
    import json
    import os
    import subprocess
    from enum import Enum

    from conformance import child_environment

    class Choice(Enum):
        LEFT = "left\tchoice"
        RIGHT = "right\nchoice"

    kind = "person\tkind"
    name = "Morgan\tLee\nJr"
    evidence = "🦀 prefix " + name + " after"
    start = evidence.index(name)
    with backend(tmp_path, text_leaf_replies()) as server, client(server) as selected:
        names = selected.recognize(evidence, kinds=[kind])
        saved = names.to_dict()
        owned_hash = hash(names)
        assert (
            str(names) == f"person kind: Morgan Lee Jr ({start}..{start + len(name)})"
        )
        assert len(names.answer) == 1 and names.answer[0].text == name
        assert names.answer[0].start == start and names.answer[0].end == start + len(
            name
        )
        assert names.answer[0].kind == kind
        assert names.to_dict() == saved and hash(names) == owned_hash
        assert saved["answer"][0]["text"] == name
        env = child_environment()
        env.update(
            THINKTHEN_BACKEND="systemone",
            THINKTHEN_BASE_URL=server.url,
            THINKTHEN_API_KEY="synthetic-loopback-key",
        )
        command = str(Path(os.environ["THINKTHEN_LOOPBACK"]).with_name("thinkthen"))
        printed = subprocess.run(
            [command, "recognize", "--text", evidence, "--kind", kind],
            env=env,
            capture_output=True,
            text=True,
            check=True,
        )
        assert printed.stdout == str(names) + "\n"
        choice = selected.choose("Which?", "body", choices=Choice)
        assert str(choice) == "left choice" and choice.answer is Choice.LEFT
        labels = ["first\tlabel", "last\r\nlabel"]
        tags = selected.tag("Which?", "body", labels=labels)
        assert str(tags) == "first label\tlast  label" and tags.answer == labels
        found = selected.find("Which line?", "Morgan\tLee\rJr")
        assert str(found) == "Morgan Lee Jr"
        assert found.answer is not None and found.answer.text == "Morgan\tLee\rJr"
        rules = [tt.RelationRule("person", "company", "works_for")]
        relations = selected.relate(evidence + " Example\rGroup", rules=rules)
        assert str(relations) == "Morgan Lee Jr --works_for--> Example Group"
        assert len(relations.answer) == 1
        assert relations.answer[0].source.text == name
        assert relations.answer[0].target.text == "Example\rGroup"
        assert relations.answer[0].relation == "works_for"
        form = selected.annotate(
            {
                "a\tname": tt.DecideQuestion("Ready?"),
                "b\r\nname": tt.DecideQuestion("Ready?"),
            },
            "body",
        )
        assert str(form) == "a name=true\tb  name=true"
        assert set(form.answer) == {"a\tname", "b\r\nname"}
        failed = selected.decide_stream("FAIL WITNESS", ["body"]).to_list()[0]
        assert failed.error is not None
        assert (
            "\\t" in failed.error.message
            and "\\n" in failed.error.message
            and "\\r" in failed.error.message
        )
        assert (
            str(failed) == f"failed: {failed.error.code.value}: {failed.error.message}"
        )
        assert (
            json.loads(json.dumps(failed.to_dict()))["error"]["message"]
            == failed.error.message
        )
        for value in (choice, tags, found, relations, form):
            before = value.to_dict()
            str(value)
            assert value.to_dict() == before
