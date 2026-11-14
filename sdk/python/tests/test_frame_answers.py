"""Frame columns produce native answers and filter their original rows."""

import json
import warnings
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
        details = (
            None
            if function == "filter"
            else getattr(source.thinkthen, function)(
                question, client=selected, details=True, **options
            )
        )
        if function == "annotate" or (function == "filter" and dataframe):
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
            if function != "filter":
                assert isinstance(details, pd.DataFrame)
            expected_dtype = {
                "decide": "boolean",
                "choose": "category",
                "score": "Float64",
                "rank": "Int64",
            }.get(function)
            if expected_dtype is not None:
                assert isinstance(actual, pd.Series)
                assert str(actual.dtype) == expected_dtype
        else:
            if function != "filter":
                assert isinstance(details, pl.DataFrame)
            expected = {
                "decide": pl.Boolean,
                "choose": pl.Enum(["a", "b"]),
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
            expected_forms = selected.annotate_stream(question, records).to_list()
            assert actual["ready"].to_list() == [
                row.answer["ready"].answer
                if row.answered and row.answer["ready"].answered
                else None
                for row in expected_forms
            ]
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
                    (row.answer if row.answered else None)
                    if library == "pandas"
                    else row.to_dict().get("answer")
                )
            values = source_answers(actual)
            for answer, expected_answer in zip(values, expected_answers):
                if expected_answer is None:
                    assert pd.isna(answer) if library == "pandas" else answer is None
                else:
                    assert answer == (
                        list(expected_answer)
                        if isinstance(expected_answer, tuple)
                        else expected_answer
                    )


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_unsure_failure_and_kept_subset(tmp_path: Path, library: str) -> None:
    source: Any = (
        pd.Series(["first", None, "third"], index=["same", "same", "last"], name="body")
        if library == "pandas"
        else pl.Series("body", ["first", None, "third"])
    )
    with backend(tmp_path, yes(0.5)) as server, client(server) as selected:
        with pytest.raises(tt.Failed):
            source.thinkthen.decide("Ready?", client=selected, threshold=(0.2, 0.8))
        answers = source.thinkthen.decide(
            "Ready?", client=selected, threshold=(0.2, 0.8), errors="coerce"
        )
        assert answers.isna().all() if library == "pandas" else answers.is_null().all()
        with pytest.raises(tt.UsageError):
            source.thinkthen.filter("Ready?", client=selected, threshold=(0.2, 0.8))
        with pytest.raises(tt.Failed):
            source.thinkthen.filter("Ready?", client=selected, threshold=0.8)
    with backend(tmp_path, yes()) as server, client(server) as selected:
        frame: Any = source.to_frame()
        if library == "pandas":
            frame["id"] = pd.array([1, True, 3], dtype=object)
            before = frame.copy(deep=True)
        else:
            frame = frame.with_columns(pl.Series("id", [1, True, 3], dtype=pl.Object))
            before = frame.clone()
        valid = frame.iloc[[0, 2]] if library == "pandas" else frame[[0, 2]]
        kept = valid.thinkthen.filter("Ready?", field="/body", client=selected)
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
        if function == "filter":
            with pytest.raises(tt.Failed):
                operation(question, client=selected, **options)
            return
        with pytest.raises(tt.Failed):
            operation(question, client=selected, **options)
        details = operation(question, client=selected, details=True, **options)
        assert details["status"].to_list().count("failed") == 1
        with pytest.warns(tt.FailedRowsWarning):
            answers = operation(question, client=selected, errors="coerce", **options)
        assert answers.thinkthen.failures()["position"].to_list() == [1]
        if function == "annotate":
            assert (
                answers["ready"].isna().iloc[0]
                if library == "pandas"
                else answers["ready"].is_null()[0]
            )
        elif library == "pandas":
            assert pd.isna(answers.iloc[0])
        else:
            assert answers.is_null()[0]


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_coerce_keeps_failures_apart_from_not_sure(
    tmp_path: Path, library: str
) -> None:
    values = ["first", None, ""]
    source: Any = pd.Series(values) if library == "pandas" else pl.Series(values)
    with (
        backend(tmp_path, yes(0.5)) as server,
        client(server) as selected,
        pytest.warns(tt.FailedRowsWarning) as caught,
    ):
        answers = source.thinkthen.decide(
            "Ready?", client=selected, threshold=(0.2, 0.8), errors="coerce"
        )
    assert len(caught) == 1 and caught[0].filename == __file__
    warning = cast(tt.FailedRowsWarning, caught[0].message)
    assert str(warning) == (
        "ThinkThen returned failed records: record.missing (1), record.empty (1). "
        ".thinkthen.failures() lists them."
    )
    assert warning.codes == (tt.ErrorCode.RECORD_MISSING, tt.ErrorCode.RECORD_EMPTY)
    assert [row.position for row in warning.rows] == [2, 3]
    # The three rows read alike as answers, and only the failures are listed.
    assert len(answers) == 3
    assert answers.isna().all() if library == "pandas" else answers.is_null().all()
    failures = answers.thinkthen.failures()
    assert failures["position"].to_list() == [2, 3]
    assert failures["error_code"].to_list() == ["record.missing", "record.empty"]
    # Polars 1.0 reads a null struct back as a struct of nulls.
    ids = failures["id"] if library == "pandas" else failures["id"].struct.field("kind")
    assert ids.to_list() == [None, None]
    questions = failures["question"]
    assert questions.isna().all() if library == "pandas" else questions.is_null().all()
    assert all(failures["error_message"].to_list())


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_a_clean_call_lists_no_failures(library: str) -> None:
    source: Any = pd.Series(["first"]) if library == "pandas" else pl.Series(["first"])
    with tt.Client(backend="stub") as selected:
        clean = source.thinkthen.filter("Ready?", client=selected)
        plain = source.thinkthen.decide("Ready?", client=selected)
    for result in (clean, plain):
        failures = result.thinkthen.failures()
        assert len(failures) == 0
        assert list(failures.columns) == [
            "position",
            "id",
            "question",
            "error_code",
            "error_message",
            "error_http_status",
        ]


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_annotation_failures_name_their_question_without_a_details_warning(
    library: str,
) -> None:
    values = {"id": ["a", "b"], "body": ["text", ""]}
    source: Any = pd.DataFrame(values) if library == "pandas" else pl.DataFrame(values)
    questions = tt.AnnotationQuestions(
        {
            "evidence": tt.DecideQuestion("Ready?", field="/body"),
            "gone": tt.DecideQuestion("Ready?", field="/gone"),
        }
    )
    with tt.Client(backend="stub") as selected:
        with pytest.warns(tt.FailedRowsWarning) as caught:
            answers = source.thinkthen.annotate(
                questions, client=selected, errors="coerce"
            )
        with warnings.catch_warnings():
            warnings.simplefilter("error")
            details = source.thinkthen.annotate(
                questions, client=selected, details=True, errors="coerce"
            )
    assert "record.missing (2), record.empty (1)" in str(caught[0].message)
    for result in (answers, details):
        failures = result.thinkthen.failures()
        assert failures["position"].to_list() == [1, 2, 2]
        assert failures["question"].to_list() == ["gone", "evidence", "gone"]
        assert failures["error_code"].to_list() == [
            "record.missing",
            "record.empty",
            "record.missing",
        ]
        ids = failures["id"].to_list()
        if library == "polars":
            ids = [identity["string"] for identity in ids]
        assert ids == ["a", "b", "b"]


def coerced_pandas(values: list[Any]) -> Any:
    source: Any = pd.Series(values)
    with tt.Client(backend="stub") as selected, pytest.warns(tt.FailedRowsWarning):
        return source.thinkthen.decide("Ready?", client=selected, errors="coerce")


def test_pandas_failures_go_where_attrs_go() -> None:
    answers = coerced_pandas(["first", None, ""])
    other = coerced_pandas([None])

    def positions(value: Any) -> list[int]:
        return list(value.thinkthen.failures()["position"].to_list())

    assert json.loads(json.dumps(answers.attrs)) == answers.attrs
    # A subset keeps the whole list, including failures outside the subset.
    assert positions(answers.iloc[[0]]) == [2, 3]
    assert positions(pd.concat([answers, answers])) == [2, 3]
    # These lost the list, so their empty tables say nothing about the call.
    assert positions(pd.concat([answers, other])) == []
    frame: Any = pd.DataFrame({"body": ["first", None, ""]})
    assert positions(frame.assign(is_ready=answers)) == []


def test_pandas_failures_survive_a_parquet_file(tmp_path: Path) -> None:
    pytest.importorskip("pyarrow")
    answers = coerced_pandas(["first", None, ""])
    path = tmp_path / "answers.parquet"
    answers.to_frame("is_ready").to_parquet(path)
    restored: Any = pd.read_parquet(path)
    assert restored.thinkthen.failures()["position"].to_list() == [2, 3]


def test_a_derived_polars_object_lists_no_failures() -> None:
    source: Any = pl.Series(["first", None, ""])
    with tt.Client(backend="stub") as selected, pytest.warns(tt.FailedRowsWarning):
        answers = source.thinkthen.decide("Ready?", client=selected, errors="coerce")
    assert len(answers.thinkthen.failures()) == 2
    assert len(answers.fill_null(False).thinkthen.failures()) == 0
    assert len(answers.to_frame().thinkthen.failures()) == 0


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
            assert ranked[details["position"][0] - 1] == 1


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
    assert len(answers) == 0
    if function == "annotate":
        assert list(answers.columns) == ["ready"]
    else:
        assert answers.name == "text"


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
    assert len(answers) == len(source)
    assert answers.schema == {"ready": pl.Boolean}
    assert answers["ready"].is_null().all()


@pytest.mark.parametrize("library", ["pandas", "polars"])
def test_raised_frame_error_keeps_completed_outcomes(library: str) -> None:
    source: Any = (
        pd.Series([None, "body"], index=[4, 4], name="body")
        if library == "pandas"
        else pl.Series("body", [None, "body"])
    )
    with (
        tt.Client(backend="stub") as selected,
        pytest.raises(tt.Failed) as raised,
    ):
        source.thinkthen.decide("Ready?", client=selected)
    outcomes = cast(Any, raised.value).outcomes
    if library == "pandas":
        assert outcomes.index.equals(source.index)
        assert outcomes["status"].tolist() == ["failed", "answered"]
        assert outcomes["answer"].iloc[1] in (True, False)
    else:
        assert outcomes["status"].to_list() == ["failed", "answered"]
        assert outcomes["answer"][1] in (True, False)


def test_pandas_float_ids_are_integers_and_null_or_fractional_ids_fail_their_row(
    tmp_path: Path,
) -> None:
    source = pd.DataFrame(
        {"id": [1.0, float("nan"), 1.5, 2.0**53, 5.0], "body": ["text"] * 5}
    )
    with backend(tmp_path, yes() * 2) as server, client(server) as selected:
        rows = source.thinkthen.decide(
            "Ready?", field="/body", client=selected, details=True, errors="coerce"
        )
        assert server.count() == 2
    assert rows["id"].tolist() == [1, None, None, None, 5]
    assert all(type(rows["id"].iloc[index]) is int for index in (0, 4))
    assert rows["status"].tolist() == [
        "answered",
        "failed",
        "failed",
        "failed",
        "answered",
    ]
    assert rows["error_code"].dropna().unique().tolist() == ["record.invalid"]
    with (
        backend(tmp_path, yes() * 2) as server,
        client(server) as selected,
        pytest.raises(tt.Failed) as raised,
    ):
        source.thinkthen.decide("Ready?", field="/body", client=selected)
    assert raised.value.code is tt.ErrorCode.RECORD_INVALID
