from typing import Any

import polars as pl

from ._columns import ColumnMethods

class ThinkThenNamespace(ColumnMethods[pl.DataFrame]):
    def __init__(self, series: pl.Series) -> None: ...
    def _missing(self, value: object) -> bool: ...
    def _frame(self, function: str, rows: list[dict[str, Any]]) -> pl.DataFrame: ...
