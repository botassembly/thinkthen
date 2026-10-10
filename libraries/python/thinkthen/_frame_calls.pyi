from typing import Any
from ._pandas_calls import PandasResult
class FrameResult(PandasResult):
    on: Any
    members: tuple[tuple[str, str], ...]
class PolarsFrameResult(FrameResult): ...
