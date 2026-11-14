"""Return native answer columns or explicit outcome details at an eager boundary."""

from __future__ import annotations

from collections.abc import Iterable
from typing import Any, Generic, TypeVar

import polars as pl

from ._columns import ColumnMethods, annotation_definitions, answer_value, choice_domain
from ._decode import plain
from ._types import Backend, BoundaryTag, ErrorCode, Function, Status

_LABELS = pl.List(pl.Struct({"label": pl.String, "probability": pl.Float64}))
_SPAN = {"text": pl.String, "start": pl.Int64, "end": pl.Int64}
_NAME = {**_SPAN, "kind": pl.String}
_SCORED_NAME = pl.Struct({**_NAME, "probability": pl.Float64})
_RELATION = pl.Struct(
    {
        "source": pl.Struct(_NAME),
        "target": pl.Struct(_NAME),
        "relation": pl.String,
        "probability": pl.Float64,
        "either": pl.Boolean,
    }
)
_SOURCE = pl.Struct(
    {
        "question_identity": pl.String,
        "backend": pl.Enum([item.value for item in Backend]),
        "model": pl.String,
        "name": pl.String,
    }
)
_CODE = pl.Enum([item.value for item in ErrorCode])
_ERROR = pl.Struct(
    {
        "code": _CODE,
        "message": pl.String,
        "http_status": pl.Int64,
    }
)
_ID = pl.Struct(
    {
        "kind": pl.Enum(["string", "signed", "unsigned"]),
        "string": pl.String,
        "signed": pl.Int64,
        "unsigned": pl.UInt64,
    }
)
_FAILURE: dict[str, Any] = {
    "position": pl.Int64,
    "id": _ID,
    "question": pl.String,
    "error_code": _CODE,
    "error_message": pl.String,
    "error_http_status": pl.Int64,
}
# Polars has no metadata, so the list sits on the one object a call returns.
_FAILURES = "_thinkthen_failures"


def _shape(
    function: str,
    questions: dict[str, Any] | None = None,
    *,
    child: bool = False,
    item_dtype: Any = pl.Null,
    domain: list[str] | None = None,
) -> dict[str, Any]:
    details = {
        "choose": pl.Struct({"labels": _LABELS}),
        "tag": pl.Struct({"labels": _LABELS}),
        "score": pl.Struct({"levels": pl.List(pl.Float64)}),
        "rank": pl.Struct({"levels": pl.List(pl.Float64)}),
        "relate": pl.Struct(
            {"names": pl.List(_SCORED_NAME), "pairs": pl.List(_RELATION)}
        ),
        "recognize": pl.Struct(
            {
                "pieces": pl.List(
                    pl.Struct(
                        {
                            **_SPAN,
                            "tags": pl.Struct(
                                {tag.value: pl.Float64 for tag in BoundaryTag}
                            ),
                        }
                    )
                ),
                "names": pl.List(pl.Struct({**_SPAN, "kinds": _LABELS})),
            }
        ),
    }
    answers: dict[str, Any] = {
        "decide": pl.Boolean,
        "filter": pl.Boolean,
        "choose": pl.Enum(domain) if domain else pl.String,
        "tag": pl.List(pl.String),
        "score": pl.Float64,
        "rank": pl.Int64,
        "find": pl.Struct(_SPAN),
        "recognize": pl.List(_SCORED_NAME),
        "relate": pl.List(_RELATION),
    }
    if function == "annotate":
        answers[function] = pl.Struct(
            {
                name: pl.Struct(
                    _shape(value["function"], child=True, domain=choice_domain(value))
                )
                for name, value in (questions or {}).items()
            }
        )
    result = {
        "function": pl.Enum([item.value for item in Function]),
        "status": pl.Enum([item.value for item in Status]),
        "answer": answers[function],
        "probability": pl.Float64,
        "error": _ERROR,
        "detail": details.get(function, pl.Null),
    }
    if not child:
        result = dict(position=pl.Int64, id=_ID, source=_SOURCE, **result)

    return result


def _labels(values: dict[str, float]) -> list[dict[str, object]]:
    return [{"label": key, "probability": value} for key, value in values.items()]


def _identity(value: str | int | None) -> dict[str, Any] | None:
    kind = (
        "string"
        if isinstance(value, str)
        else "unsigned"
        if isinstance(value, int) and value >= 2**63
        else "signed"
    )
    return {"kind": kind, kind: value} if value is not None else None


def _project(value: dict[str, Any], *, child: bool = False) -> dict[str, Any]:
    function = value["function"]
    if not child:
        value["id"] = _identity(value.get("id"))
    detail = value.get("detail")
    if detail is not None:
        if function in {"tag", "choose"}:
            detail["labels"] = _labels(detail["labels"])
        elif function == "recognize":
            for name in detail["names"]:
                name["kinds"] = _labels(name["kinds"])
    if function == "annotate" and value.get("answer") is not None:
        value["answer"] = {
            key: _project(outcome, child=True)
            for key, outcome in value["answer"].items()
        }
    return value


def series(
    rows: list[Any],
    function: str | None = None,
    question: object = None,
    item_dtype: Any = None,
) -> pl.Series:
    function = function or (rows[0].function.value if rows else "decide")
    questions = (
        annotation_definitions(question, rows) if function == "annotate" else None
    )
    dtype = pl.Struct(
        _shape(
            function, questions, item_dtype=item_dtype, domain=choice_domain(question)
        )
    )
    return pl.Series(
        "result", [_project(plain(row)) for row in rows], dtype=dtype, strict=True
    )


D = TypeVar("D")
K = TypeVar("K")


class _PolarsNamespace(ColumnMethods[pl.Series, D, K, pl.DataFrame], Generic[D, K]):
    def _read_failures(self) -> list[dict[str, Any]]:
        return list(getattr(self._value, _FAILURES, []))

    def _write_failures(self, value: Any, entries: list[dict[str, Any]]) -> None:
        if entries:
            setattr(value, _FAILURES, entries)

    def _failure_frame(self, entries: list[dict[str, Any]]) -> pl.DataFrame:
        values = [{**entry, "id": _identity(entry["id"])} for entry in entries]
        return pl.DataFrame(
            [
                pl.Series(name, [entry[name] for entry in values], dtype, strict=True)
                for name, dtype in _FAILURE.items()
            ]
        )


class _ColumnResults(_PolarsNamespace[pl.DataFrame, K], Generic[K]):
    def _answers(self, function: str, rows: list[Any], question: object) -> Any:
        outcomes = series(rows, function, question)
        positions = {row.position - 1: index for index, row in enumerate(rows)}
        order = pl.Series(
            [positions.get(index) for index in range(len(self._value))], dtype=pl.Int64
        )
        if function == "annotate":
            answers = outcomes.struct.field("answer")
            return pl.DataFrame(
                [
                    answers.struct.field(name)
                    .struct.field("answer")
                    .gather(order)
                    .alias(name)
                    for name in annotation_definitions(question, rows)
                ]
            )
        name = self._value.name if isinstance(self._value, pl.Series) else "answer"
        return outcomes.struct.field("answer").gather(order).rename(name)

    def _frame(self, function: str, rows: list[Any], question: object) -> pl.DataFrame:
        outcomes = series(rows, function, question)
        result = outcomes.to_frame().unnest("result")
        errors = outcomes.struct.field("error")
        result = result.drop("error").with_columns(
            [
                errors.struct.field(field).alias("error_" + field)
                for field in ("code", "message", "http_status")
            ]
        )
        result = result.with_columns(
            pl.Series("item", [row.item for row in rows], dtype=pl.Object)
        )
        if function == "annotate":
            children = outcomes.struct.field("answer")
            result = result.drop("answer")
            columns = []
            for name in annotation_definitions(question, rows):
                child = children.struct.field(name)
                for field in ("answer", "status", "probability", "detail"):
                    value = child.struct.field(field)
                    if field == "status":
                        value = value.fill_null(outcomes.struct.field("status"))
                    columns.append(
                        value.alias(name if field == "answer" else name + "_" + field)
                    )
                error = child.struct.field("error")
                for field in ("code", "message", "http_status"):
                    columns.append(
                        error.struct.field(field)
                        .fill_null(errors.struct.field(field))
                        .alias(name + "_error_" + field)
                    )
            result = result.hstack(columns)
        return result


@pl.api.register_series_namespace("thinkthen")
class ThinkThenNamespace(_ColumnResults[pl.Series]):
    _value: pl.Series

    def _records(self) -> Iterable[Any]:
        for value in self._value:
            yield value.to_list() if isinstance(value, pl.Series) else value

    def _kept(self, rows: list[Any]) -> pl.Series:
        return self._value.gather(
            [row.position - 1 for row in rows if answer_value(row) is True]
        )


@pl.api.register_dataframe_namespace("thinkthen")
class ThinkThenFrameNamespace(_ColumnResults[pl.DataFrame]):
    _value: pl.DataFrame

    def _records(self) -> Iterable[Any]:
        yield from self._value.iter_rows(named=True)

    def _kept(self, rows: list[Any]) -> pl.DataFrame:
        kept = {row.position - 1 for row in rows if answer_value(row) is True}
        return self._value.filter(
            pl.Series(
                [index in kept for index in range(len(self._value))], dtype=pl.Boolean
            )
        )


__all__ = ["ThinkThenFrameNamespace", "ThinkThenNamespace"]
