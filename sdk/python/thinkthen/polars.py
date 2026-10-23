"""Register the optional eager Series namespace with ``import thinkthen.polars``."""

from __future__ import annotations

from typing import Any, cast, get_type_hints

import polars as pl

from ._columns import ColumnMethods
from ._relation import Relation
from ._types import Found, Name, RowError


def _struct(value: type[Any]) -> pl.Struct:
    scalars = {str: pl.String, int: pl.Int64, float: pl.Float64, bool: pl.Boolean}
    return pl.Struct(
        {
            name: scalars[field] if field in scalars else _struct(field)
            for name, field in get_type_hints(value).items()
        }
    )


@pl.api.register_series_namespace("thinkthen")
class ThinkThenNamespace(ColumnMethods[pl.DataFrame]):
    def _missing(self, value: object) -> bool:
        return value is None

    def _value(self, value: object) -> object:
        return value.to_list() if isinstance(value, pl.Series) else value

    def _frame(self, function: str, rows: list[dict[str, Any]]) -> pl.DataFrame:
        answers = {
            "decide": pl.Boolean,
            "filter": pl.Boolean,
            "choose": pl.String,
            "tag": pl.List(pl.String),
            "score": pl.Float64,
            "rank": pl.Int64,
            "find": _struct(Found),
            "annotate": pl.Object,
            "recognize": pl.List(_struct(Name)),
            "relate": pl.List(_struct(Relation)),
        }
        fields = {
            "position": pl.Int64,
            "id": pl.Object,
            "status": pl.String,
            "answer": answers[function],
            "p": pl.Float64,
            "error": _struct(RowError),
            "detail": pl.Object,
        }
        if function == "filter":
            fields["kept"] = pl.Boolean
        return pl.DataFrame(
            [
                pl.Series(key, [row.get(key) for row in rows], dtype=cast(Any, dtype))
                for key, dtype in fields.items()
            ]
        )
