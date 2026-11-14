"""Public table accessors preserve missing states, form failures and column names."""

from __future__ import annotations

import asyncio
import warnings
from pathlib import Path
from typing import Any, cast

import pandas as pd
import polars as pl
import pytest
from api_support import (
    async_client,
    backend,
    client,
    isolated_environment,  # noqa: F401 - shared pytest fixture.
    yes,
)

import thinkthen as tt
import thinkthen.pandas
import thinkthen.polars

pytestmark = pytest.mark.usefixtures("isolated_environment")


def source(library: str, records: list[Any]) -> Any:
    return (
        pd.Series(records, index=["same"] * len(records), dtype=object)
        if library == "pandas"
        else pl.Series("text", records)
    )


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_details_keep_missing_states_without_reading_unavailable_answers(
    tmp_path: Path, library: str
) -> None:
    values = source(library, ["first", None, "last"])
    with backend(tmp_path, yes(0.5)) as server, client(server) as selected:
        details = values.thinkthen.decide(
            "Ready?", threshold=(0.2, 0.8), details=True, client=selected
        )
        assert isinstance(
            details, pd.DataFrame if library == "pandas" else pl.DataFrame
        )
        assert details["status"].to_list() == ["unsure", "failed", "unsure"]
        assert details["position"].to_list() == [1, 2, 3]
        assert (
            cast(Any, details["answer"]).isna().all()
            if library == "pandas"
            else details["answer"].is_null().all()
        )
        assert details["item"].to_list() == ["first", None, "last"]
        assert server.count() == 2
        with pytest.raises(tt.Failed) as failure:
            values.thinkthen.decide("Ready?", threshold=(0.2, 0.8), client=selected)
        assert failure.value.outcomes["status"].to_list() == [
            "unsure",
            "failed",
            "unsure",
        ]
        with pytest.warns(tt.FailedRowsWarning) as caught:
            coerced = values.thinkthen.decide(
                "Ready?", threshold=(0.2, 0.8), errors="coerce", client=selected
            )
        assert len(caught) == 1
        assert coerced.thinkthen.failures()["position"].to_list() == [2]


@pytest.mark.parametrize("library", ["pandas", "polars"])
@pytest.mark.parametrize("failed", ["refund", "topic", "urgency"])
def test_each_failed_form_child_is_visible_and_plain_forms_raise(
    tmp_path: Path, library: str, failed: str
) -> None:
    questions = {
        name: tt.DecideQuestion(
            "Ready?", field="/absent" if name == failed else "/text"
        )
        for name in ("refund", "topic", "urgency")
    }
    values = source(library, [{"text": "record"}])
    with backend(tmp_path, yes()) as server, client(server) as selected:
        details = values.thinkthen.annotate(questions, details=True, client=selected)
        assert details["status"].to_list() == ["answered"]
        assert details[failed + "_status"].to_list() == ["failed"]
        assert details[failed + "_error_code"].to_list() == ["record.missing"]
        for name in questions:
            if name != failed:
                assert details[name].to_list() == [True]
        with pytest.raises(tt.Failed) as failure:
            values.thinkthen.annotate(questions, client=selected)
        assert failure.value.outcomes[failed + "_status"].to_list() == ["failed"]
        with pytest.warns(tt.FailedRowsWarning) as caught:
            answers = values.thinkthen.annotate(
                questions, errors="coerce", client=selected
            )
        assert len(caught) == 1
        assert list(answers.columns) == sorted(questions)
        assert answers.thinkthen.failures()["question"].to_list() == [failed]


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_unsure_children_are_missing_without_becoming_failures(
    tmp_path: Path, library: str
) -> None:
    questions = {"refund": tt.DecideQuestion("Ready?", threshold=(0.2, 0.8))}
    with backend(tmp_path, yes(0.5)) as server, client(server) as selected:
        with warnings.catch_warnings(record=True) as caught:
            answers = source(library, ["body"]).thinkthen.annotate(
                questions, client=selected
            )
        assert not caught
        assert (
            answers["refund"].isna().all()
            if library == "pandas"
            else answers["refund"].is_null().all()
        )
        assert len(answers.thinkthen.failures()) == 0


@pytest.mark.parametrize("library", ["pandas", "polars"])
@pytest.mark.parametrize("details", [False, True])
@pytest.mark.parametrize("saved", [False, True])
@pytest.mark.parametrize("names", [("status",), ("refund", "refund_status")])
def test_colliding_annotation_names_are_refused_before_requests(
    tmp_path: Path, library: str, details: bool, saved: bool, names: tuple[str, ...]
) -> None:
    question: Any = {name: tt.DecideQuestion("Ready?") for name in names}
    if saved:
        question = tt.AnnotateQuestion(question)
    with backend(tmp_path, yes()) as server, client(server) as selected:
        with pytest.raises(tt.UsageError) as failure:
            source(library, ["body"]).thinkthen.annotate(
                question, details=details, client=selected
            )
        assert any(name in str(failure.value) for name in names)
        assert server.count() == 0


@pytest.mark.parametrize("library", ["pandas", "polars"])
@pytest.mark.parametrize("details", [False, True])
def test_annotation_answer_name_is_not_reserved_without_an_outer_answer_column(
    tmp_path: Path, library: str, details: bool
) -> None:
    with backend(tmp_path, yes()) as server, client(server) as selected:
        result = source(library, ["body"]).thinkthen.annotate(
            {"answer": tt.DecideQuestion("Ready?")},
            details=details,
            client=selected,
        )
        assert result["answer"].to_list() == [True]
        assert server.count() == 1


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_filter_refuses_details_and_coercion_and_reports_failed_rows(
    tmp_path: Path, library: str
) -> None:
    values = source(library, [None, "body"])
    with backend(tmp_path, yes()) as server, client(server) as selected:
        for option in ({"details": True}, {"errors": "coerce"}):
            with pytest.raises(TypeError):
                values.thinkthen.filter("Ready?", client=selected, **option)
        assert server.count() == 0
        with pytest.raises(tt.Failed) as failure:
            values.thinkthen.filter("Ready?", client=selected)
        assert failure.value.outcomes["status"].to_list() == ["failed", "answered"]
        kept = source(library, ["first", "last"]).thinkthen.filter(
            "Ready?", client=selected
        )
        assert kept.to_list() == ["first", "last"]


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_choice_columns_keep_the_full_ordered_domain_and_missing_states(
    tmp_path: Path, library: str
) -> None:
    from enum import Enum

    class Topic(Enum):
        SHIPPING = "shipping"
        BILLING = "billing"
        OTHER = "other"

    values = source(library, ["first", None])
    replies = [
        {
            "answer": {
                "type": "choice",
                "probabilities": {"shipping": 0.03, "billing": 0.95, "other": 0.02},
            }
        }
    ]
    with backend(tmp_path, replies) as server, client(server) as selected:
        with pytest.warns(tt.FailedRowsWarning):
            answers = values.thinkthen.choose(
                "Which?", choices=Topic, errors="coerce", client=selected
            )
        details = values.thinkthen.choose(
            "Which?", choices=Topic, details=True, client=selected
        )
        assert details["status"].to_list() == ["answered", "failed"]
        if library == "pandas":
            assert isinstance(answers.dtype, pd.CategoricalDtype)
            assert answers.cat.categories.to_list() == ["shipping", "billing", "other"]
            assert answers.iloc[0] == "billing" and pd.isna(answers.iloc[1])
        else:
            assert answers.dtype == pl.Enum(["shipping", "billing", "other"])
            assert answers.to_list() == ["billing", None]
        assert server.count() == 2


@pytest.mark.parametrize("library", ["pandas", "polars"])
@pytest.mark.parametrize("asynchronous", [False, True])
def test_stream_table_conversion_preserves_nested_form_states(
    tmp_path: Path, library: str, asynchronous: bool
) -> None:
    record: dict[str, Any] = {"id": "original", "text": "body"}
    questions = tt.AnnotationQuestions(
        {
            "ready": tt.DecideQuestion("Ready?", field="/text"),
            "missing": tt.DecideQuestion("Ready?", field="/absent"),
        }
    )
    method = "to_" + library
    with backend(tmp_path, yes()) as server:
        if asynchronous:

            async def convert() -> Any:
                async with async_client(server) as selected:
                    stream = selected.annotate_stream(questions, [record])
                    return await getattr(stream, method)()

            result = asyncio.run(convert())
        else:
            with client(server) as selected:
                stream = selected.annotate_stream(questions, [record])
                result = getattr(stream, method)()
        if library == "pandas":
            entries = result["answer"].iloc[0]
            assert entries["ready"].answer is True
            assert entries["missing"].failed
            assert entries["missing"].error.code == "record.missing"
            assert result["position"].to_list() == [1]
            assert result["item"].to_list()[0] is record
        else:
            outer = result["result"].to_list()[0]
            assert outer["answer"]["ready"]["answer"] is True
            assert outer["answer"]["missing"]["status"] == "failed"
            assert outer["answer"]["missing"]["error"]["code"] == "record.missing"
            assert outer["position"] == 1
        assert server.count() == 1


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_async_stream_choice_conversion_keeps_the_admitted_enum_domain(
    tmp_path: Path, library: str
) -> None:
    from enum import Enum

    class Topic(Enum):
        SHIPPING = "shipping"
        BILLING = "billing"
        OTHER = "other"

    reply = {
        "answer": {
            "type": "choice",
            "probabilities": {
                "shipping": 0.03,
                "billing": 0.95,
                "other": 0.02,
            },
        }
    }
    with backend(tmp_path, [reply]) as server:

        async def convert() -> Any:
            async with async_client(server) as selected:
                stream = selected.choose_stream("Which?", ["body"], choices=Topic)
                return await getattr(stream, "to_" + library)()

        result = asyncio.run(convert())
        if library == "pandas":
            assert result["answer"].to_list() == ["billing"]
            assert result["answer"].cat.categories.to_list() == [
                "shipping",
                "billing",
                "other",
            ]
        else:
            answers = result["result"].struct.field("answer")
            assert answers.to_list() == ["billing"]
            assert answers.dtype == pl.Enum(["shipping", "billing", "other"])
        assert server.count() == 1


@pytest.mark.parametrize("library", ["pandas", "polars"])
@pytest.mark.parametrize("asynchronous", [False, True])
def test_stream_table_conversion_keeps_question_names_inside_nested_answers(
    tmp_path: Path, library: str, asynchronous: bool
) -> None:
    questions = tt.AnnotationQuestions({"status": tt.DecideQuestion("Ready?")})
    with backend(tmp_path, yes()) as server:
        if asynchronous:

            async def convert() -> Any:
                async with async_client(server) as selected:
                    stream = selected.annotate_stream(questions, ["body"])
                    return await getattr(stream, "to_" + library)()

            result = asyncio.run(convert())
        else:
            with client(server) as selected:
                stream = selected.annotate_stream(questions, ["body"])
                result = getattr(stream, "to_" + library)()
        if library == "pandas":
            assert result["status"].to_list() == ["answered"]
            assert result["answer"].iloc[0]["status"].answer is True
        else:
            outer = result["result"].to_list()[0]
            assert outer["status"] == "answered"
            assert outer["answer"]["status"]["answer"] is True
        assert server.count() == 1


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_rank_details_keep_input_order_with_reversed_scores_and_a_failed_row(
    tmp_path: Path, library: str
) -> None:
    records: list[Any] = ["low", None, "high"]
    values: Any = (
        pd.Series(records, index=["first", "failed", "last"], dtype=object)
        if library == "pandas"
        else pl.Series("text", records)
    )
    replies = [
        {"contains": '"low"', "distribution": {"low": 1.0, "high": 0.0}},
        {"contains": '"high"', "distribution": {"low": 0.0, "high": 1.0}},
    ]
    with backend(tmp_path, replies) as server, client(server) as selected:
        ranked = selected.rank("How urgent?", records, levels=["low", "high"]).to_list()
        assert [row.position for row in ranked] != [1, 2, 3]
        details = values.thinkthen.rank(
            "How urgent?", levels=["low", "high"], details=True, client=selected
        )
        assert details["position"].to_list() == [1, 2, 3]
        assert details["status"].to_list() == ["answered", "failed", "answered"]
        if library == "pandas":
            assert details.index.equals(values.index)
            assert details["answer"].iloc[0] == 2
            assert pd.isna(details["answer"].iloc[1])
            assert details["answer"].iloc[2] == 1
        else:
            assert details["answer"].to_list() == [2, None, 1]
