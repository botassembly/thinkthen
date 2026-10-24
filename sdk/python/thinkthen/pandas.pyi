from typing import Any

import pandas as pd

from ._columns import ColumnMethods

class ThinkThenAccessor(ColumnMethods[pd.DataFrame]):
    def __init__(self, series: pd.Series[Any]) -> None: ...
    def _missing(self, value: object) -> bool: ...
    def _frame(self, function: str, rows: list[dict[str, Any]]) -> pd.DataFrame: ...
