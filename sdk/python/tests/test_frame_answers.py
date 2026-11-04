"""Frame columns produce native answers and filter their original rows."""

from pathlib import Path
from typing import Any, cast

import pandas as pd
import polars as pl
import pytest
from api_support import backend, client, isolated_environment, yes  # noqa: F401

import thinkthen as tt
import thinkthen.pandas
import thinkthen.polars

pytestmark = pytest.mark.usefixtures("isolated_environment")


def source_answers(value: Any) -> list[Any]:
    return list(value.to_list())


FUNCTIONS: list[tuple[str, dict[str, Any]]] = [
    ("decide", {}),
    ("filter", {}),
    ("choose", {"choices": ["a", "b"]}),
    ("tag", {"labels": ["a", "b"]}),
    ("score", {"levels": ["low", "high"]}),
    ("rank", {"levels": ["low", "high"]}),
    ("find", {}),
    ("recognize", {"kinds": ["person"]}),
    ("relate", {"rules": [tt.RelationRule("person", "person", "knows")]}),
    ("annotate", {}),
]


@pytest.mark.parametrize("function, options", FUNCTIONS)
@pytest.mark.parametrize("library", ["pandas", "polars"])
@pytest.mark.parametrize("dataframe", [False, True])
def test_native_answer_column(
    function: str, options: dict[str, Any], library: str, dataframe: bool
) -> None:
    source: Any = (
        pd.Series(["first", "second"], index=["same", "same"], name="body")
        if library == "pandas"
        else pl.Series("body", ["first", "second"])
    )
    question: Any = (
        tt.AnnotationQuestions({"ready": tt.DecideQuestion("Ready?")})
        if function == "annotate"
        else None
        if function in {"recognize", "relate"}
        else "Ready?"
    )
    records: list[Any] = ["first", "second"]
    if dataframe:
        source = source.to_frame()
        if library == "pandas":
            source["name"] = ["one", "two"]
        else:
            source = source.with_columns(pl.Series("name", ["one", "two"]))
        records = [{"body": "first", "name": "one"}, {"body": "second", "name": "two"}]
    with tt.Client(backend="stub") as selected:
        actual = getattr(source.thinkthen, function)(
            question, client=selected, **options
        )
        details = getattr(source.thinkthen, function)(
            question, client=selected, details=True, **options
        )
        if function == "filter" and dataframe:
            assert isinstance(
                actual, pd.DataFrame if library == "pandas" else pl.DataFrame
            )
        else:
            assert isinstance(actual, pd.Series if library == "pandas" else pl.Series)
            assert actual.name == ("answer" if dataframe else "body")
        actual = cast(Any, actual)
        if library == "pandas":
            assert isinstance(actual, (pd.Series, pd.DataFrame))
            assert actual.index.isin(source.index).all()
            if function != "filter":
                assert actual.index.equals(source.index)
            assert isinstance(details, pd.DataFrame)
            expected_dtype = {
                "decide": "boolean",
                "choose": "string",
                "score": "Float64",
                "rank": "Int64",
            }.get(function)
            if expected_dtype is not None:
                assert isinstance(actual, pd.Series)
                assert str(actual.dtype) == expected_dtype
        else:
            if dataframe:
                assert isinstance(details, pl.DataFrame)
            else:
                assert isinstance(details.dtype, pl.Struct)
            expected = {
                "decide": pl.Boolean,
                "choose": pl.String,
                "score": pl.Float64,
                "rank": pl.Int64,
            }.get(function)
            if expected is not None:
                assert isinstance(actual, pl.Series)
                assert actual.dtype == expected
        actual = cast(Any, actual)
        if function == "filter":
            expected_values = selected.filter(question, records).to_list()
            actual_values = (
                (
                    actual.to_dict(orient="records")
                    if library == "pandas"
                    else actual.to_dicts()
                )
                if dataframe
                else list(actual)
            )
            assert actual_values == expected_values
        elif function == "annotate":
            values = source_answers(actual)
            assert all(isinstance(value["ready"], bool) for value in values)
        else:
            assert len(actual) == 2
            stream_name = function if function == "rank" else function + "_stream"
            stream_args = (
                (records,)
                if function in {"recognize", "relate"}
                else (question, records)
            )
            with getattr(selected, stream_name)(*stream_args, **options) as rows:
                canonical = rows.to_list()
            expected_answers = [None, None]
            for row in canonical:
                expected_answers[row.position - 1] = (
                    row.answer if library == "pandas" else row.to_dict()["answer"]
                )
            values = source_answers(actual)
            for answer, expected_answer in zip(values, expected_answers):
                if expected_answer is None:
                    assert answer is pd.NA if library == "pandas" else answer is None
                else:
                    assert answer == expected_answer


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_unsure_failure_and_kept_subset(tmp_path: Path, library: str) -> None:
    source: Any = (
        pd.Series(["first", None, "third"], index=["same", "same", "last"], name="body")
        if library == "pandas"
        else pl.Series("body", ["first", None, "third"])
    )
    with backend(tmp_path, yes(0.5)) as server, client(server) as selected:
        with pytest.raises(tt.RecordError):
            source.thinkthen.decide("Ready?", client=selected, threshold=(0.2, 0.8))
        answers = source.thinkthen.decide(
            "Ready?", client=selected, threshold=(0.2, 0.8), errors="coerce"
        )
        assert answers.isna().all() if library == "pandas" else answers.is_null().all()
        with pytest.raises(tt.RecordError):
            source.thinkthen.filter("Ready?", client=selected, threshold=(0.2, 0.8))
        subset = source.thinkthen.filter(
            "Ready?", client=selected, threshold=(0.2, 0.8), errors="coerce"
        )
        assert len(subset) == 0
    with backend(tmp_path, yes()) as server, client(server) as selected:
        frame: Any = source.to_frame()
        if library == "pandas":
            frame["id"] = pd.array([1, True, 3], dtype=object)
            before = frame.copy(deep=True)
        else:
            frame = frame.with_columns(pl.Series("id", [1, True, 3], dtype=pl.Object))
            before = frame.clone()
        kept = frame.thinkthen.filter(
            "Ready?", field="/body", client=selected, errors="coerce"
        )
        expected = frame.iloc[[0, 2]] if library == "pandas" else frame[[0, 2]]
        if library == "pandas":
            assert kept.equals(expected)
            assert frame.equals(before)
        else:
            # Polars Object columns preserve Python values but do not implement equals.
            assert kept.schema == expected.schema
            assert kept.to_dicts() == expected.to_dicts()
            assert frame.schema == before.schema
            assert frame.to_dicts() == before.to_dicts()


@pytest.mark.parametrize("function, options", FUNCTIONS)
@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_every_function_raises_or_coerces_failed_rows(
    function: str, options: dict[str, Any], library: str
) -> None:
    source: Any = (
        pd.Series([None, "body"], index=[7, 7], name="text")
        if library == "pandas"
        else pl.Series("text", [None, "body"])
    )
    question: Any = (
        tt.AnnotationQuestions({"ready": tt.DecideQuestion("Ready?")})
        if function == "annotate"
        else None
        if function in {"recognize", "relate"}
        else "Ready?"
    )
    with tt.Client(backend="stub") as selected:
        operation = getattr(source.thinkthen, function)
        for details in (False, True):
            with pytest.raises(tt.RecordError):
                operation(question, client=selected, details=details, **options)
        answers = operation(question, client=selected, errors="coerce", **options)
        if function == "filter":
            assert None not in list(answers)
        elif library == "pandas":
            assert answers.iloc[0] is pd.NA
        else:
            assert answers[0] is None


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_rank_top_is_aligned_to_the_whole_input(library: str) -> None:
    source: Any = (
        pd.Series(["first", "second", "third"], index=["a", "b", "a"], name="body")
        if library == "pandas"
        else pl.Series("body", ["first", "second", "third"])
    )
    with tt.Client(backend="stub") as selected:
        ranked = source.thinkthen.rank(
            "How much?", levels=["low", "high"], top=1, client=selected
        )
        details = source.thinkthen.rank(
            "How much?", levels=["low", "high"], top=1, details=True, client=selected
        )
        assert len(ranked) == 3 and ranked.name == "body"
        if library == "pandas":
            assert ranked.index.equals(source.index)
            assert ranked.notna().sum() == 1
            assert ranked.iloc[int(details["position"].iloc[0]) - 1] == 1
        else:
            assert ranked.is_not_null().sum() == 1
            assert ranked[details.struct.field("position")[0] - 1] == 1


@pytest.mark.parametrize("function, options", FUNCTIONS)
@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_empty_answer_columns_keep_their_name_and_type(
    function: str, options: dict[str, Any], library: str
) -> None:
    source: Any = (
        pd.Series([], name="text", dtype="string")
        if library == "pandas"
        else pl.Series("text", [], dtype=pl.String)
    )
    question: Any = (
        tt.AnnotationQuestions({"ready": tt.DecideQuestion("Ready?")})
        if function == "annotate"
        else None
        if function in {"recognize", "relate"}
        else "Ready?"
    )
    with tt.Client(backend="stub") as selected:
        answers = getattr(source.thinkthen, function)(
            question, client=selected, **options
        )
    assert len(answers) == 0 and answers.name == "text"


@pytest.mark.parametrize("values", [[], [None]])
def test_polars_annotation_mapping_keeps_schema_without_answered_rows(
    values: list[None],
) -> None:
    source: Any = pl.Series("body", values, dtype=pl.String)
    with tt.Client(backend="stub") as selected:
        answers = source.thinkthen.annotate(
            {"ready": tt.DecideQuestion("Ready?")},
            client=selected,
            errors="coerce",
        )
    assert answers.name == "body" and len(answers) == len(source)
    assert answers.dtype == pl.Struct({"ready": pl.Boolean})
    assert answers.to_list() == [None] * len(source)


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_raised_frame_error_keeps_completed_outcomes(library: str) -> None:
    source: Any = (
        pd.Series([None, "body"], index=[4, 4], name="body")
        if library == "pandas"
        else pl.Series("body", [None, "body"])
    )
    with (
        tt.Client(backend="stub") as selected,
        pytest.raises(tt.RecordError) as raised,
    ):
        source.thinkthen.decide("Ready?", client=selected)
    outcomes = cast(Any, raised.value).outcomes
    if library == "pandas":
        assert outcomes.index.equals(source.index)
        assert outcomes["status"].tolist() == ["failed", "answered"]
        assert outcomes["answer"].iloc[1] in (True, False)
    else:
        assert outcomes.struct.field("status").to_list() == ["failed", "answered"]
        assert outcomes.struct.field("answer")[1] in (True, False)
