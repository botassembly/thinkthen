"""Materialize native Polars Struct columns at an eager boundary."""

from __future__ import annotations

from collections.abc import Iterable
from typing import Any

import polars as pl

from ._columns import ColumnMethods, annotation_definitions
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
_ERROR = pl.Struct(
    {
        "code": pl.Enum([item.value for item in ErrorCode]),
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


def _shape(
    function: str,
    questions: dict[str, Any] | None = None,
    *,
    child: bool = False,
    item_dtype: Any = pl.Null,
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
                "names": pl.List(
                    pl.Struct({"start": pl.Int64, "end": pl.Int64, "kinds": _LABELS})
                ),
            }
        ),
    }
    answers: dict[str, Any] = {
        "decide": pl.Boolean,
        "filter": pl.Boolean,
        "choose": pl.String,
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
                name: pl.Struct(_shape(value["function"], child=True))
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
        if function == "filter":
            result.update(item=item_dtype, kept=pl.Boolean)
    return result


def _labels(values: dict[str, float]) -> list[dict[str, object]]:
    return [{"label": key, "probability": value} for key, value in values.items()]


def _project(value: dict[str, Any], *, child: bool = False) -> dict[str, Any]:
    function = value["function"]
    if not child:
        identity = value["id"]
        kind = (
            "string"
            if isinstance(identity, str)
            else "unsigned"
            if isinstance(identity, int) and identity >= 2**63
            else "signed"
        )
        value["id"] = {"kind": kind, kind: identity} if identity is not None else None
    detail = value["detail"]
    if detail is not None:
        if function in {"tag", "choose"}:
            detail["labels"] = _labels(detail["labels"])
        elif function == "recognize":
            for name in detail["names"]:
                name["kinds"] = _labels(name["kinds"])
    if function == "annotate" and value["answer"] is not None:
        value["answer"] = {
            key: _project(outcome, child=True)
            for key, outcome in value["answer"].items()
        }
    if function == "filter":
        value["kept"] = None if value["status"] == "failed" else value["answer"] is True
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
    if item_dtype is None:
        item_dtype = (
            pl.Series([plain(row.item) for row in rows], strict=True).dtype
            if function == "filter" and rows
            else pl.Null
        )
    dtype = pl.Struct(_shape(function, questions, item_dtype=item_dtype))
    return pl.Series(
        "result", [_project(plain(row)) for row in rows], dtype=dtype, strict=True
    )


@pl.api.register_series_namespace("thinkthen")
class ThinkThenNamespace(ColumnMethods[pl.Series]):
    def _records(self) -> Iterable[Any]:
        for value in self._value:
            yield value.to_list() if isinstance(value, pl.Series) else value

    def _frame(self, function: str, rows: list[Any], question: object) -> pl.Series:
        return series(rows, function, question, self._value.dtype)


@pl.api.register_dataframe_namespace("thinkthen")
class ThinkThenFrameNamespace(ColumnMethods[pl.DataFrame]):
    def _records(self) -> Iterable[Any]:
        yield from self._value.iter_rows(named=True)

    def _frame(self, function: str, rows: list[Any], question: object) -> pl.DataFrame:
        source: pl.DataFrame = self._value
        outcomes = series(rows, function, question, source.to_struct().dtype)
        if function != "annotate":
            return outcomes.to_frame()
        columns = []
        for name in annotation_definitions(question, rows):
            child = outcomes.struct.field("answer").struct.field(name)
            for field in ("answer", "status", "probability", "detail"):
                column = name if field == "answer" else name + "_" + field
                value = child.struct.field(field)
                if field == "status":
                    value = value.fill_null(outcomes.struct.field("status"))
                columns.append(value.alias(column))
        return source.hstack(columns)


__all__ = ["ThinkThenFrameNamespace", "ThinkThenNamespace"]
