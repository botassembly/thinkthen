"""Python values at the public boundary."""

from __future__ import annotations

import json
from dataclasses import dataclass
from typing import Generic, Literal, TypeAlias, TypedDict, TypeVar, cast

JSONValue: TypeAlias = (
    None | bool | int | float | str | list["JSONValue"] | dict[str, "JSONValue"]
)
Record: TypeAlias = object
Adapter: TypeAlias = str
Threshold: TypeAlias = float | str
A_co = TypeVar("A_co", covariant=True)
T_co = TypeVar("T_co", covariant=True)
RecordT = TypeVar("RecordT")


class ThinkThenError(Exception):
    """A library failure with its stable code and original message."""

    def __init__(self, code: str, message: str) -> None:
        self._code = code
        super().__init__(message)

    @property
    def code(self) -> str:
        return self._code


class UsageError(ThinkThenError):
    """The library refused the arguments."""


class BackendError(ThinkThenError):
    """The backend could not answer."""


class LocalError(ThinkThenError):
    """A local resource could not be read or written."""


@dataclass(frozen=True)
class RowError:
    code: str
    message: str


class _Detail:
    def __get__(self, instance: object, owner: type[object] | None = None) -> JSONValue:
        if instance is None:
            return None
        values = vars(instance)
        raw = values.get("_detail_json")
        if raw is not None:
            values["_detail_value"] = json.loads(raw)
            values["_detail_json"] = None
        return cast(JSONValue, values.get("_detail_value"))

    def __set__(self, instance: object, value: JSONValue) -> None:
        vars(instance)["_detail_value"] = value


_DETAIL = cast(JSONValue, _Detail())


@dataclass(frozen=True)
class Row(Generic[A_co]):
    id: JSONValue
    status: Literal["answered", "unsure", "failed"]
    answer: A_co | None = None
    p: float | None = None
    error: RowError | None = None
    detail: JSONValue = _DETAIL


class Found(TypedDict):
    text: str
    start: int
    end: int


class Entity(Found):
    kind: str


class Name(Entity):
    p: float


class _OptionalRule(TypedDict, total=False):
    reads: str
    either: bool


class RelationRule(_OptionalRule):
    name: str
    source: str
    target: str


# These names are shared by the implementation and declaration file.
__all__ = [
    "Adapter",
    "BackendError",
    "Entity",
    "Found",
    "JSONValue",
    "LocalError",
    "Name",
    "Record",
    "RelationRule",
    "Row",
    "RowError",
    "ThinkThenError",
    "Threshold",
    "UsageError",
]
