"""Immutable values at the Python boundary."""

from __future__ import annotations

from collections.abc import ItemsView, Iterator, KeysView, Mapping, ValuesView
from dataclasses import dataclass, field
from enum import Enum
from reprlib import repr as short_repr
from typing import (
    Any,
    Generic,
    Literal,
    NamedTuple,
    NoReturn,
    TypeAlias,
    cast,
    overload,
)

from typing_extensions import TypeForm, TypeVar

from ._codes import ErrorCode
from ._enums import StringEnum

Record: TypeAlias = (
    None | bool | int | float | str | list["Record"] | dict[str, "Record"]
)
JSONValue = Record
Strings: TypeAlias = list[str] | tuple[str, ...]
_Label = TypeVar("_Label", bound=str | Enum, default=str)
_Label_co = TypeVar("_Label_co", bound=str | Enum, default=str, covariant=True)
ChoiceInput: TypeAlias = list[_Label] | tuple[_Label, ...] | TypeForm[_Label]
# Each choice, label or level, or its Enum member, maps to what it means.
Descriptions: TypeAlias = Mapping[_Label, str] | Mapping[str, str]
T_co = TypeVar("T_co", covariant=True)
_Item_co = TypeVar("_Item_co", covariant=True, default=Any)
A_co = TypeVar("A_co", covariant=True)


class Backend(StringEnum):
    SYSTEMONE = "systemone"
    OPENAI = "openai"
    STUB = "stub"


BackendInput: TypeAlias = Backend | Literal["systemone", "openai", "stub"]


class Status(StringEnum):
    ANSWERED = "answered"
    UNSURE = "unsure"
    FAILED = "failed"


class Function(StringEnum):
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


class BoundaryTag(StringEnum):
    BEGIN = "BEGIN"
    INSIDE = "INSIDE"
    END = "END"
    SINGLE = "SINGLE"
    OUT = "OUT"


# A ranked record the engine could not keep, because it had no JSON form.
UNAVAILABLE: Any = object()


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

    def __reduce__(
        self,
    ) -> tuple[object, tuple[ErrorCode, str, int | None], dict[str, Any]]:
        return type(self), (self.code, str(self), self.http_status), self.__dict__


class UsageError(ThinkThenError, ValueError):
    """The engine refused the call arguments."""


class SetupError(ThinkThenError):
    """The client cannot start an operation."""


class Cancelled(ThinkThenError):
    """The operation was cancelled."""


class InternalError(ThinkThenError):
    """The native operation could not complete."""


class RecordError(ThinkThenError):
    """A failed result was raised explicitly or used as an answer.

    When ``filter`` raises after delivering its kept records, ``code`` is the
    first failed row's code and ``rows`` holds every failed row in input order.
    """

    def __init__(
        self,
        code: ErrorCode,
        message: str,
        http_status: int | None = None,
        rows: tuple[Any, ...] = (),
    ) -> None:
        super().__init__(code, message, http_status)
        self.rows = rows


class FailedRowsWarning(UserWarning):
    """A frame call under ``errors="coerce"`` kept going past failed records.

    ``codes`` holds each distinct failure code in order of first appearance, and
    ``rows`` holds every row with a failure in input order. An annotate row is
    listed when any of its questions failed.
    """

    def __init__(
        self,
        message: str,
        codes: tuple[ErrorCode, ...] = (),
        rows: tuple[Any, ...] = (),
    ) -> None:
        super().__init__(message)
        self.codes = codes
        self.rows = rows


class Unsure(ValueError):
    """A not-sure result was used as an answer. Check .unsure first."""

    def __init__(self, row: _Result[Any]) -> None:
        self.row = row
        super().__init__("not sure; check .unsure or inspect .answer")


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
    text: str
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


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class _Result(Generic[A_co]):
    """An answer with its result fields.

    Probability is of yes for decide/filter, or of the chosen label for choose.
    """

    function: Function
    status: Status
    answer: A_co | None
    probability: float | None
    error: RowError | None

    @property
    def unsure(self) -> bool:
        return self.status is Status.UNSURE

    @property
    def failed(self) -> bool:
        return self.status is Status.FAILED

    def _answer_value(self) -> A_co | None:
        self.raise_for_error()
        if self.unsure:
            raise Unsure(self)
        return self.answer

    def _comparison_value(self) -> object:
        return self.answer.text if isinstance(self.answer, Span) else self.answer

    def __bool__(self) -> bool:
        answer = self._answer_value()
        if self.function in {
            Function.CHOOSE,
            Function.SCORE,
            Function.RANK,
            Function.ANNOTATE,
        }:
            raise TypeError(
                f"{self.function.value} is not a yes/no answer; compare it or inspect .answer"
            )
        return bool(answer)

    def __eq__(self, other: object) -> bool:
        self._answer_value()
        if isinstance(other, _Result):
            other._answer_value()
            other = other._comparison_value()
        try:
            return bool(self._comparison_value() == other)
        except Exception:  # noqa: BLE001 - equality must never raise on host objects.
            return False

    def __hash__(self) -> int:
        return hash(self._comparison_value())

    def __str__(self) -> str:
        self._answer_value()
        value = self._comparison_value()
        return str(value.value if isinstance(value, Enum) else value)

    def __repr__(self) -> str:
        return f"{type(self).__name__}({short_repr(self._comparison_value())}, probability={self.probability!r}, status={self.status.value!r})"

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


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class _Row(_Result[A_co]):
    position: int
    id: str | int | None
    source: Source

    def __enter__(self) -> NoReturn:
        function = self.function.value
        stream = {"filter": "filter_rows", "rank": "rank"}.get(
            function, function + "_stream"
        )
        raise TypeError(f"this is one {function} row; use {stream}(...) for a stream")

    def __exit__(self, *exception: object) -> NoReturn:
        self.__enter__()


class _SequenceAnswer(Generic[T_co]):
    __slots__ = ()

    def _items(self) -> tuple[T_co, ...]:
        return cast(
            tuple[T_co, ...], cast(_Result[tuple[T_co, ...]], self)._answer_value()
        )

    def __iter__(self) -> Iterator[T_co]:
        return iter(self._items())

    def __len__(self) -> int:
        return len(self._items())

    @overload
    def __getitem__(self, index: int) -> T_co: ...

    @overload
    def __getitem__(self, index: slice) -> tuple[T_co, ...]: ...

    def __getitem__(self, index: int | slice) -> T_co | tuple[T_co, ...]:
        return self._items()[index]


class _NumberAnswer:
    __slots__ = ()

    def _number(self) -> float:
        return cast(float, cast(_Result[float], self)._answer_value())

    def __float__(self) -> float:
        return float(self._number())

    def __lt__(self, other: float) -> bool:
        return self._number() < other

    def __le__(self, other: float) -> bool:
        return self._number() <= other

    def __gt__(self, other: float) -> bool:
        return self._number() > other

    def __ge__(self, other: float) -> bool:
        return self._number() >= other


class _FindAnswer:
    __slots__ = ()

    @property
    def start(self) -> int | None:
        span = cast(_Result[Span], self)._answer_value()
        return None if span is None else span.start

    @property
    def end(self) -> int | None:
        span = cast(_Result[Span], self)._answer_value()
        return None if span is None else span.end


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class DecideRow(_Row[bool]):
    """A yes/no answer; probability is the probability of yes, even on a no."""

    function: Literal[Function.DECIDE]
    detail: None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class DecideOutcome(_Result[bool]):
    """An annotation's yes/no answer; probability has DecideRow semantics."""

    function: Literal[Function.DECIDE]
    detail: None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class FilterRow(_Row[bool], Generic[T_co]):
    """A filter answer and original item; probability has DecideRow semantics."""

    function: Literal[Function.FILTER]
    detail: None = None
    item: T_co = field(kw_only=True)

    @property
    def kept(self) -> bool:
        return self.status is Status.ANSWERED and self.answer is True


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class FilterOutcome(_Result[bool]):
    """An annotation's filter answer; probability has DecideRow semantics."""

    function: Literal[Function.FILTER]
    detail: None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class ChooseRow(_Row[_Label_co], Generic[_Label_co]):
    """A selected label; probability is the probability of that label."""

    function: Literal[Function.CHOOSE]
    detail: LabelDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class ChooseOutcome(_Result[_Label_co], Generic[_Label_co]):
    """An annotation's selected label; probability has ChooseRow semantics."""

    function: Literal[Function.CHOOSE]
    detail: LabelDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class TagRow(
    _Row[tuple[_Label_co, ...]], _SequenceAnswer[_Label_co], Generic[_Label_co]
):
    function: Literal[Function.TAG]
    detail: LabelDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class TagOutcome(
    _Result[tuple[_Label_co, ...]], _SequenceAnswer[_Label_co], Generic[_Label_co]
):
    function: Literal[Function.TAG]
    detail: LabelDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class ScoreRow(_Row[float], _NumberAnswer):
    function: Literal[Function.SCORE]
    detail: LevelDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class ScoreOutcome(_Result[float], _NumberAnswer):
    function: Literal[Function.SCORE]
    detail: LevelDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class RankRow(_Row[int], _NumberAnswer, Generic[_Item_co]):
    """A rank starting at 1 and a copy of the record it ranks."""

    function: Literal[Function.RANK]
    detail: LevelDetail | None = None
    _item: Any = field(kw_only=True, default=None)

    @property
    def item(self) -> _Item_co:
        """An equal copy of the ranked record, kept by the engine.

        A record with no JSON form failed as ``record.invalid``; reading its
        item raises that row's ``RecordError``.
        """
        if self._item is UNAVAILABLE:
            self.raise_for_error()
            raise SetupError(ErrorCode.USAGE_ABI, "the ranked record is unavailable")
        return cast(_Item_co, self._item)

    def __int__(self) -> int:
        return cast(int, self._answer_value())

    def __index__(self) -> int:
        return int(self)


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class FindRow(_Row[Span], _FindAnswer):
    function: Literal[Function.FIND]
    detail: None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class FindOutcome(_Result[Span], _FindAnswer):
    function: Literal[Function.FIND]
    detail: None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class RecognizeRow(_Row[tuple[Name, ...]], _SequenceAnswer[Name]):
    function: Literal[Function.RECOGNIZE]
    detail: RecognizeDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class RecognizeOutcome(_Result[tuple[Name, ...]], _SequenceAnswer[Name]):
    function: Literal[Function.RECOGNIZE]
    detail: RecognizeDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class RelateRow(_Row[tuple[Relation, ...]], _SequenceAnswer[Relation]):
    function: Literal[Function.RELATE]
    detail: RelationDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class RelateOutcome(_Result[tuple[Relation, ...]], _SequenceAnswer[Relation]):
    function: Literal[Function.RELATE]
    detail: RelationDetail | None = None


Outcome: TypeAlias = (
    DecideOutcome
    | FilterOutcome
    | ChooseOutcome[Any]
    | TagOutcome[Any]
    | ScoreOutcome
    | FindOutcome
    | RecognizeOutcome
    | RelateOutcome
)


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class AnnotateRow(_Row[Mapping[str, Outcome]]):
    function: Literal[Function.ANNOTATE]
    detail: None = None

    def __getitem__(self, key: str) -> Outcome:
        return cast(Mapping[str, Outcome], self._answer_value())[key]

    def __iter__(self) -> Iterator[str]:
        return iter(cast(Mapping[str, Outcome], self._answer_value()))

    def __len__(self) -> int:
        return len(cast(Mapping[str, Outcome], self._answer_value()))

    def keys(self) -> KeysView[str]:
        return cast(Mapping[str, Outcome], self._answer_value()).keys()

    def values(self) -> ValuesView[Outcome]:
        return cast(Mapping[str, Outcome], self._answer_value()).values()

    def items(self) -> ItemsView[str, Outcome]:
        return cast(Mapping[str, Outcome], self._answer_value()).items()


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
    "Descriptions",
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
    "Unsure",
    "UsageError",
]
