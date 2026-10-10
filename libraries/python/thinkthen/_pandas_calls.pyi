from typing import Any
from ._calls import Result

class PandasResult(Result[Any]):
    source: Any
    present: tuple[int, ...]
    @property
    def positions(self) -> tuple[int | None, ...]: ...
