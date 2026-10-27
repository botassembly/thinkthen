"""String enum behavior shared by every supported Python version."""

from enum import Enum


class StringEnum(str, Enum):
    __str__ = str.__str__

    def __format__(self, format_spec: str) -> str:
        return str.__format__(self, format_spec)
