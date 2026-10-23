"""Register the optional eager Series accessor with ``import thinkthen.pandas``."""

from __future__ import annotations

from typing import Any, cast

import pandas as pd

from ._columns import ColumnMethods


@pd.api.extensions.register_series_accessor("thinkthen")
class ThinkThenAccessor(ColumnMethods[pd.DataFrame]):
    def _missing(self, value: object) -> bool:
        return (
            bool(pd.isna(cast(Any, value))) if pd.api.types.is_scalar(value) else False
        )

    def _frame(self, function: str, rows: list[dict[str, Any]]) -> pd.DataFrame:
        answer = {
            "decide": "boolean",
            "filter": "boolean",
            "choose": "string",
            "score": "Float64",
            "rank": "Int64",
        }.get(function, "object")
        fields = {
            "position": "int64",
            "id": "object",
            "status": "string",
            "answer": answer,
            "p": "Float64",
            "error": "object",
            "detail": "object",
        }
        if function == "filter":
            fields["kept"] = "boolean"
        return pd.DataFrame(
            {
                key: pd.Series(
                    [pd.NA if row.get(key) is None else row[key] for row in rows],
                    index=self._series.index,
                    dtype=dtype,
                )
                for key, dtype in fields.items()
            },
            index=self._series.index,
        )
