"""Eager pandas operations that preserve each input's index."""

from __future__ import annotations

from collections.abc import Iterable
from typing import Any, cast

import pandas as pd

from ._columns import ColumnMethods, annotation_definitions
from ._types import ErrorCode, Status


def _answer_dtype(function: str) -> str:
    return {
        "decide": "boolean",
        "filter": "boolean",
        "choose": "string",
        "score": "Float64",
        "rank": "Int64",
    }.get(function, "object")


def frame(
    rows: list[Any], index: Any = None, function: str | None = None
) -> pd.DataFrame:
    if function is None:
        function = rows[0].function.value if rows else "decide"
    answer = _answer_dtype(function)
    values: dict[str, list[Any]] = {
        name: [getattr(row, name) for row in rows]
        for name in (
            "position",
            "id",
            "function",
            "status",
            "answer",
            "probability",
            "source",
            "detail",
        )
    }
    values["answer"] = [pd.NA if value is None else value for value in values["answer"]]
    for name in ("code", "message", "http_status"):
        values["error_" + name] = [
            getattr(row.error, name) if row.error is not None else None for row in rows
        ]
    dtypes: dict[str, Any] = {
        "position": "Int64",
        "id": "object",
        "function": "string",
        "status": pd.CategoricalDtype([item.value for item in Status]),
        "answer": answer,
        "probability": "Float64",
        "source": "object",
        "detail": "object",
        "error_code": pd.CategoricalDtype([item.value for item in ErrorCode]),
        "error_message": "string",
        "error_http_status": "Int64",
    }
    if function == "filter":
        values.update(
            item=[row.item for row in rows],
            kept=[row.kept for row in rows],
        )
        dtypes.update(item="object", kept="boolean")
    return pd.DataFrame(
        {
            key: pd.Series(value, index=index, dtype=dtypes[key])
            for key, value in values.items()
        },
        index=index,
    )


def _missing(value: object) -> bool:
    return bool(pd.isna(cast(Any, value))) if pd.api.types.is_scalar(value) else False


@pd.api.extensions.register_series_accessor("thinkthen")
class ThinkThenAccessor(ColumnMethods[pd.DataFrame]):
    def _missing(self, value: object) -> bool:
        return _missing(value)

    def _records(self) -> Iterable[Any]:
        yield from self._value

    def _frame(self, function: str, rows: list[Any], question: object) -> pd.DataFrame:
        index = self._value.index.take([row.position - 1 for row in rows])
        return frame(rows, index, function)


@pd.api.extensions.register_dataframe_accessor("thinkthen")
class ThinkThenFrameAccessor(ThinkThenAccessor):
    def _records(self) -> Iterable[Any]:
        yield from self._value.to_dict(orient="records")

    def _frame(self, function: str, rows: list[Any], question: object) -> pd.DataFrame:
        if function != "annotate":
            return super()._frame(function, rows, question)
        source: pd.DataFrame = self._value
        result = source.iloc[[row.position - 1 for row in rows]].copy()
        for name, definition in annotation_definitions(question, rows).items():
            children = [
                row.answer.get(name) if row.answer is not None else None for row in rows
            ]
            for field, dtype in (
                ("answer", _answer_dtype(definition["function"])),
                ("status", pd.CategoricalDtype([item.value for item in Status])),
                ("probability", "Float64"),
                ("detail", "object"),
            ):
                values = [
                    getattr(child, field)
                    if child is not None
                    else row.status
                    if field == "status"
                    else None
                    for child, row in zip(children, rows)
                ]
                result.insert(
                    len(result.columns),
                    name if field == "answer" else name + "_" + field,
                    pd.Series(values, index=result.index, dtype=dtype),
                )
        return result


__all__ = ["ThinkThenAccessor", "ThinkThenFrameAccessor"]
