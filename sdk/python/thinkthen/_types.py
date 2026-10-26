"""Immutable values at the Python boundary."""

from __future__ import annotations

from collections.abc import Mapping
from dataclasses import dataclass, field
from enum import Enum
from typing import Any, Generic, Literal, NamedTuple, NoReturn, TypeAlias, TypeVar, cast

from ._codes import ErrorCode

Record: TypeAlias = (
    None | bool | int | float | str | list["Record"] | dict[str, "Record"]
)
JSONValue = Record
Strings: TypeAlias = list[str] | tuple[str, ...]
T_co = TypeVar("T_co", covariant=True)
A_co = TypeVar("A_co", covariant=True)


class Backend(str, Enum):
    SYSTEMONE = "systemone"
    OPENAI = "openai"
    STUB = "stub"


BackendInput: TypeAlias = Backend | Literal["systemone", "openai", "stub"]


class Status(str, Enum):
    ANSWERED = "answered"
    UNSURE = "unsure"
    FAILED = "failed"


class Function(str, Enum):
    DECIDE = "decide"
    FILTER = "filter"
    CHOOSE = "choose"
    TAG = "tag"
    SCORE = "score"
    RANK = "rank"
    FIND = "find"
    RECOGNIZE = "recognize"
    RELATE = "relate"
    ANNOTATE = "annotate"


class BoundaryTag(str, Enum):
    BEGIN = "BEGIN"
    INSIDE = "INSIDE"
    END = "END"
    SINGLE = "SINGLE"
    OUT = "OUT"


class Band(NamedTuple):
    low: float
    high: float


Threshold: TypeAlias = float | Band | tuple[float, float]


@dataclass(frozen=True, slots=True)
class RetryPolicy:
    max_retries: int | None = None
    initial_backoff: float | None = None
    attempt_timeout: float | None = None


@dataclass(frozen=True, slots=True)
class ClientSettings:
    backend: Backend
    model: str
    url: str | None
    batch: int
    retry: RetryPolicy


class ThinkThenError(Exception):
    def __init__(
        self, code: ErrorCode, message: str, http_status: int | None = None
    ) -> None:
        self.code = code
        self.http_status = http_status
        super().__init__(message)


class UsageError(ThinkThenError, ValueError):
    """The engine refused the call arguments."""


class SetupError(ThinkThenError):
    """The client cannot start an operation."""


class Cancelled(ThinkThenError):
    """The operation was cancelled."""


class InternalError(ThinkThenError):
    """The native operation could not complete."""


class RecordError(ThinkThenError):
    """A_co caller explicitly raised a failed row."""


@dataclass(frozen=True, slots=True)
class RowError:
    code: ErrorCode
    message: str
    http_status: int | None = None


@dataclass(frozen=True, slots=True)
class Source:
    question_identity: str
    backend: Backend
    model: str
    name: str | None = None


@dataclass(frozen=True, slots=True)
class Span:
    text: str
    start: int
    end: int


@dataclass(frozen=True, slots=True)
class RelationName(Span):
    kind: str


@dataclass(frozen=True, slots=True)
class Name(RelationName):
    probability: float


@dataclass(frozen=True, slots=True)
class Relation:
    source: RelationName
    target: RelationName
    relation: str
    probability: float
    either: bool


@dataclass(frozen=True, slots=True)
class RelationRule:
    source: str
    target: str
    relation: str
    reads: str | None = None
    either: bool = False


@dataclass(frozen=True, slots=True)
class LabelDetail:
    labels: Mapping[str, float]


@dataclass(frozen=True, slots=True)
class LevelDetail:
    levels: tuple[float, ...]


@dataclass(frozen=True, slots=True)
class PieceDetail:
    text: str
    start: int
    end: int
    tags: Mapping[BoundaryTag, float]


@dataclass(frozen=True, slots=True)
class KindDetail:
    start: int
    end: int
    kinds: Mapping[str, float]


@dataclass(frozen=True, slots=True)
class RecognizeDetail:
    pieces: tuple[PieceDetail, ...]
    names: tuple[KindDetail, ...]


@dataclass(frozen=True, slots=True)
class RelationDetail:
    names: tuple[Name, ...]
    pairs: tuple[Relation, ...]


@dataclass(frozen=True, slots=True)
class _Result(Generic[A_co]):
    function: Function
    status: Status
    answer: A_co | None
    probability: float | None
    error: RowError | None

    def to_dict(self) -> dict[str, Any]:
        """Copy this result and its nested values into plain Python containers."""
        from ._decode import plain

        return cast(dict[str, Any], plain(self))

    @property
    def ok(self) -> bool:
        return self.status is Status.ANSWERED

    def raise_for_error(self) -> None:
        if self.error is not None:
            raise RecordError(
                self.error.code, self.error.message, self.error.http_status
            )


@dataclass(frozen=True, slots=True)
class _Row(_Result[A_co]):
    position: int
    id: str | int | None
    source: Source

    def __enter__(self) -> NoReturn:
        function = self.function.value
        stream = function if function in {"filter", "rank"} else function + "_stream"
        raise TypeError(f"this is one {function} row; use {stream}(...) for a stream")

    def __exit__(self, *exception: object) -> NoReturn:
        self.__enter__()


@dataclass(frozen=True, slots=True)
class DecideRow(_Row[bool]):
    function: Literal[Function.DECIDE]
    detail: None = None


@dataclass(frozen=True, slots=True)
class DecideOutcome(_Result[bool]):
    function: Literal[Function.DECIDE]
    detail: None = None


@dataclass(frozen=True, slots=True)
class FilterRow(_Row[bool], Generic[T_co]):
    function: Literal[Function.FILTER]
    detail: None = None
    item: T_co = field(kw_only=True)

    @property
    def kept(self) -> bool:
        return self.status is Status.ANSWERED and self.answer is True


@dataclass(frozen=True, slots=True)
class FilterOutcome(_Result[bool]):
    function: Literal[Function.FILTER]
    detail: None = None


@dataclass(frozen=True, slots=True)
class ChooseRow(_Row[str]):
    function: Literal[Function.CHOOSE]
    detail: LabelDetail | None = None


@dataclass(frozen=True, slots=True)
class ChooseOutcome(_Result[str]):
    function: Literal[Function.CHOOSE]
    detail: LabelDetail | None = None


@dataclass(frozen=True, slots=True)
class TagRow(_Row[tuple[str, ...]]):
    function: Literal[Function.TAG]
    detail: LabelDetail | None = None


@dataclass(frozen=True, slots=True)
class TagOutcome(_Result[tuple[str, ...]]):
    function: Literal[Function.TAG]
    detail: LabelDetail | None = None


@dataclass(frozen=True, slots=True)
class ScoreRow(_Row[float]):
    function: Literal[Function.SCORE]
    detail: LevelDetail | None = None


@dataclass(frozen=True, slots=True)
class ScoreOutcome(_Result[float]):
    function: Literal[Function.SCORE]
    detail: LevelDetail | None = None


@dataclass(frozen=True, slots=True)
class RankRow(_Row[int]):
    function: Literal[Function.RANK]
    detail: LevelDetail | None = None


@dataclass(frozen=True, slots=True)
class FindRow(_Row[Span]):
    function: Literal[Function.FIND]
    detail: None = None


@dataclass(frozen=True, slots=True)
class FindOutcome(_Result[Span]):
    function: Literal[Function.FIND]
    detail: None = None


@dataclass(frozen=True, slots=True)
class RecognizeRow(_Row[tuple[Name, ...]]):
    function: Literal[Function.RECOGNIZE]
    detail: RecognizeDetail | None = None


@dataclass(frozen=True, slots=True)
class RecognizeOutcome(_Result[tuple[Name, ...]]):
    function: Literal[Function.RECOGNIZE]
    detail: RecognizeDetail | None = None


@dataclass(frozen=True, slots=True)
class RelateRow(_Row[tuple[Relation, ...]]):
    function: Literal[Function.RELATE]
    detail: RelationDetail | None = None


@dataclass(frozen=True, slots=True)
class RelateOutcome(_Result[tuple[Relation, ...]]):
    function: Literal[Function.RELATE]
    detail: RelationDetail | None = None


Outcome: TypeAlias = (
    DecideOutcome
    | FilterOutcome
    | ChooseOutcome
    | TagOutcome
    | ScoreOutcome
    | FindOutcome
    | RecognizeOutcome
    | RelateOutcome
)


@dataclass(frozen=True, slots=True)
class AnnotateRow(_Row[Mapping[str, Outcome]]):
    function: Literal[Function.ANNOTATE]
    detail: None = None


__all__ = [
    "AnnotateRow",
    "Backend",
    "BackendInput",
    "Band",
    "BoundaryTag",
    "Cancelled",
    "ChooseOutcome",
    "ChooseRow",
    "ClientSettings",
    "DecideOutcome",
    "DecideRow",
    "ErrorCode",
    "FilterOutcome",
    "FilterRow",
    "FindOutcome",
    "FindRow",
    "Function",
    "InternalError",
    "JSONValue",
    "KindDetail",
    "LabelDetail",
    "LevelDetail",
    "Name",
    "Outcome",
    "PieceDetail",
    "RankRow",
    "RecognizeDetail",
    "RecognizeOutcome",
    "RecognizeRow",
    "Record",
    "RecordError",
    "RelateOutcome",
    "RelateRow",
    "Relation",
    "RelationDetail",
    "RelationName",
    "RelationRule",
    "RetryPolicy",
    "RowError",
    "ScoreOutcome",
    "ScoreRow",
    "SetupError",
    "Source",
    "Span",
    "Status",
    "Strings",
    "TagOutcome",
    "TagRow",
    "ThinkThenError",
    "Threshold",
    "UsageError",
]
