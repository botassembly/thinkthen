from typing import Any
from polars import DataFrame, Series
from ._native_results import _NativeResult
from ._pandas_calls import PandasResult
class FrameResult(PandasResult):
    on: Any
    members: tuple[tuple[str, str], ...]
class PolarsFrameResult(FrameResult):
    @property
    def value(self) -> DataFrame | Series | list[_NativeResult]: ...
