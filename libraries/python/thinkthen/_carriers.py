"""Shared immutable result primitives retain absence, offsets and safe identities."""
from dataclasses import dataclass
import re
import math
from collections.abc import Mapping
from types import MappingProxyType

class Absent:
    def __repr__(self): return "ABSENT"
ABSENT = Absent()

class Carrier:
    def __repr__(self): return f"<{type(self).__name__}: content withheld>"

class Identity(str):
    def __new__(cls, value):
        if not isinstance(value, str) or re.fullmatch(r"[0-9a-f]{64}", value) is None:
            raise ValueError("invalid identity")
        return super().__new__(cls, value)
    def __repr__(self): return f"<{type(self).__name__}>"

@dataclass(frozen=True, repr=False, kw_only=True)
class Span(Carrier):
    start: int
    end: int



def _json(value):
    if value is None or type(value) in (str, bool, int):
        return value
    if type(value) is float and math.isfinite(value):
        return value
    if isinstance(value, (list, tuple)):
        return tuple(_json(x) for x in value)
    if isinstance(value, Mapping) and all(type(k) is str for k in value):
        return MappingProxyType({k: _json(v) for k, v in value.items()})
    raise ValueError("invalid complete result")

