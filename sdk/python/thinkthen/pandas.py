"""Eager pandas operations that preserve each input's index."""

from __future__ import annotations

from collections.abc import Iterable
from enum import Enum
from typing import Any, Generic, TypeVar, cast

import pandas as pd

from ._columns import (
    ANNOTATION_COLUMNS,
    ColumnMethods,
    annotation_definitions,
    answer_value,
    choice_domain,
)
from ._types import ErrorCode, Status

_FAILURES = "thinkthen_failures"
_ERROR_CODE = pd.CategoricalDtype([item.value for item in ErrorCode])
_STATUS = pd.CategoricalDtype([item.value for item in Status])


def _answer_dtype(function: str, question: object = None) -> Any:
    domain = choice_domain(question)
    if function == "choose" and domain:
        return pd.CategoricalDtype(domain, ordered=True)
    return {
        "decide": "boolean",
        "filter": "boolean",
        "choose": "string",
        "score": "Float64",
        "rank": "Int64",
    }.get(function, "object")


def _answer(row: Any) -> Any:
    value = answer_value(row) if row is not None else None
    if isinstance(value, Enum):
        return value.value
    if isinstance(value, tuple):
        return [item.value if isinstance(item, Enum) else item for item in value]
    return value


def frame(
    rows: list[Any],
    index: Any = None,
    function: str | None = None,
    question: object = None,
) -> pd.DataFrame:
    function = function or (rows[0].function.value if rows else "decide")
    values = {
        name: [getattr(row, name) for row in rows]
        for name in (
            "position",
            "id",
            "function",
            "status",
            "probability",
            "source",
            "detail",
            "item",
        )
    }
    values["answer"] = [_answer(row) for row in rows]
    for name in ("code", "message", "http_status"):
        values["error_" + name] = [
            getattr(row.error, name) if row.error is not None else None for row in rows
        ]
    dtypes: dict[str, Any] = {
        "position": "Int64",
        "id": "object",
        "function": "string",
        "status": _STATUS,
        "answer": _answer_dtype(function, question),
        "probability": "Float64",
        "source": "object",
        "detail": "object",
        "item": "object",
        "error_code": _ERROR_CODE,
        "error_message": "string",
        "error_http_status": "Int64",
    }
    result = pd.DataFrame(
        {
            key: pd.Series(value, index=index, dtype=dtypes[key])
            for key, value in values.items()
        },
        index=index,
    )
    return result


def _missing(value: object) -> bool:
    return bool(pd.isna(cast(Any, value))) if pd.api.types.is_scalar(value) else False


P = TypeVar("P", "pd.Series[Any]", pd.DataFrame)


class _PandasAccessor(
    ColumnMethods["pd.Series[Any]", pd.DataFrame, P, pd.DataFrame], Generic[P]
):
    def __init__(self, value: P) -> None:
        super().__init__(value)

    def _missing(self, value: object) -> bool:
        return _missing(value)

    def _records(self) -> Iterable[Any]:
        yield from self._value

    def _columns(self) -> list[str]:
        return list(ANNOTATION_COLUMNS)

    def _answers(self, function: str, rows: list[Any], question: object) -> Any:
        if function == "annotate":
            columns = {}
            for name, definition in annotation_definitions(question, rows).items():
                values: list[Any] = [None] * len(self._value)
                for row in rows:
                    child = (answer_value(row) or {}).get(name)
                    values[row.position - 1] = _answer(child)
                columns[name] = pd.Series(
                    values,
                    index=self._value.index,
                    dtype=_answer_dtype(definition["function"], definition),
                )
            return pd.DataFrame(columns, index=self._value.index)
        values = [None] * len(self._value)
        for row in rows:
            values[row.position - 1] = _answer(row)
        return pd.Series(
            values,
            index=self._value.index,
            name=self._value.name if isinstance(self._value, pd.Series) else "answer",
            dtype=_answer_dtype(function, question),
        )

    def _kept(self, rows: list[Any]) -> P:
        return cast(
            P,
            self._value.iloc[
                [row.position - 1 for row in rows if answer_value(row) is True]
            ].copy(),
        )

    def _read_failures(self) -> list[dict[str, Any]]:
        return list(self._value.attrs.get(_FAILURES, []))

    def _write_failures(self, value: Any, entries: list[dict[str, Any]]) -> None:
        value.attrs.pop(_FAILURES, None)
        if entries:
            value.attrs[_FAILURES] = entries

    def _failure_frame(self, entries: list[dict[str, Any]]) -> pd.DataFrame:
        dtypes: dict[str, Any] = {
            "position": "Int64",
            "id": "object",
            "question": "string",
            "error_code": _ERROR_CODE,
            "error_message": "string",
            "error_http_status": "Int64",
        }
        return pd.DataFrame(
            {
                name: pd.Series([entry[name] for entry in entries], dtype=dtype)
                for name, dtype in dtypes.items()
            }
        )

    def _frame(self, function: str, rows: list[Any], question: object) -> pd.DataFrame:
        index = self._value.index.take([row.position - 1 for row in rows])
        result = frame(rows, index, function, question)
        if function == "annotate":
            result = result.drop(columns="answer")
            for name, definition in annotation_definitions(question, rows).items():
                children = [(answer_value(row) or {}).get(name) for row in rows]
                result[name] = pd.Series(
                    [_answer(child) for child in children],
                    index=index,
                    dtype=_answer_dtype(definition["function"], definition),
                )
                for field, dtype in (
                    ("status", _STATUS),
                    ("probability", "Float64"),
                    ("detail", "object"),
                ):
                    cells = [
                        getattr(child, field)
                        if child is not None
                        else row.status
                        if field == "status"
                        else None
                        for child, row in zip(children, rows)
                    ]
                    result[name + "_" + field] = pd.Series(
                        cells, index=index, dtype=dtype
                    )
                for field, dtype in (
                    ("code", _ERROR_CODE),
                    ("message", "string"),
                    ("http_status", "Int64"),
                ):
                    errors = [
                        child.error if child is not None else row.error
                        for child, row in zip(children, rows)
                    ]
                    result[name + "_error_" + field] = pd.Series(
                        [
                            getattr(error, field) if error is not None else None
                            for error in errors
                        ],
                        index=index,
                        dtype=dtype,
                    )
        return result


@pd.api.extensions.register_series_accessor("thinkthen")
class ThinkThenAccessor(_PandasAccessor["pd.Series[Any]"]):
    pass


@pd.api.extensions.register_dataframe_accessor("thinkthen")
class ThinkThenFrameAccessor(_PandasAccessor[pd.DataFrame]):
    def _records(self) -> Iterable[Any]:
        yield from self._value.to_dict(orient="records")


__all__ = ["ThinkThenAccessor", "ThinkThenFrameAccessor"]
