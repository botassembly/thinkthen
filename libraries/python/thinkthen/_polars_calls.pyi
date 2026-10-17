from polars import Series
from ._native_results import _NativeResult
from ._pandas_calls import PandasResult
class PolarsResult(PandasResult):
    @property
    def value(self) -> Series: ...
    @property
    def probability(self) -> Series | None: ...
