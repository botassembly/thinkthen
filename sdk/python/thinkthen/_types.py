"""Immutable values at the Python boundary."""

from __future__ import annotations

from collections.abc import Mapping
from dataclasses import dataclass, field, fields, is_dataclass
from decimal import Decimal
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
)

from typing_extensions import Self, TypeForm, TypeVar

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


@dataclass(frozen=True)
class TokenUsage:
    """Reported tokens and finished attempts whose usage is unknown."""

    reported: int | None
    unknown_attempts: int


@dataclass(frozen=True)
class Usage:
    """An immutable cumulative snapshot for one explicit client lifetime."""

    attempts: int
    responses: int
    status_retries: int
    connection_retries: int
    request_body_bytes: int
    active_attempts: int
    active_calls: int
    input_tokens: TokenUsage
    output_tokens: TokenUsage

    @classmethod
    def _from_json(cls, value: str) -> Usage:
        import json

        raw = json.loads(value)
        raw["input_tokens"] = TokenUsage(**raw["input_tokens"])
        raw["output_tokens"] = TokenUsage(**raw["output_tokens"])
        return cls(**raw)


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
    max_requests: int | None = None


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


class NoAnswer(Exception):
    """No answer was available to read."""


class Failed(ThinkThenError, NoAnswer):
    """A scalar judgement failed, or a failed bulk result was checked or read.

    When ``filter`` raises after delivering its kept records, ``code`` is the
    first failed row's code and ``rows`` holds every failed row in input order.
    """

    outcomes: Any

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


class Unsure(NoAnswer):
    """A not-sure result was used as an answer. Check .unsure first."""

    def __init__(self, row: _Result[Any]) -> None:
        self.row = row
        self.probability = row.probability
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
    _answer: A_co | None
    probability: float | None
    error: RowError | None

    @property
    def answer(self) -> A_co:
        """Read the plain answer, raising when this result has none."""
        self._raise_own_failure()
        if self.unsure:
            raise Unsure(self)
        value = self._answer
        if isinstance(value, Mapping):
            return cast(A_co, dict(value))
        if isinstance(value, tuple):
            return cast(A_co, list(value))
        return cast(A_co, value)

    @property
    def answered(self) -> bool:
        return self.status is Status.ANSWERED

    @property
    def unsure(self) -> bool:
        return self.status is Status.UNSURE

    @property
    def failed(self) -> bool:
        return self.status is Status.FAILED

    def _raise_own_failure(self) -> None:
        if self.error is not None:
            raise Failed(
                self.error.code, self.error.message, self.error.http_status, (self,)
            )

    def raise_if_failed(self) -> Self:
        """Raise for this result or a failed form entry; uncertainty is not failure."""
        self._raise_own_failure()
        if self.function is Function.ANNOTATE and isinstance(self._answer, Mapping):
            for name in sorted(self._answer):
                child = self._answer[name]
                if child.error is not None:
                    raise Failed(
                        child.error.code,
                        child.error.message,
                        child.error.http_status,
                        (self,),
                    )
        return self

    def __bool__(self) -> bool:
        if self.function is not Function.DECIDE:
            raise TypeError(
                f"{self.function.value} is not a yes/no result; read .answer"
            )
        return bool(self.answer)

    def _value(self) -> tuple[object, ...]:
        return tuple(
            _freeze(self._answer if name == "answer" else getattr(self, name, None))
            for name in (
                "function",
                "status",
                "answer",
                "probability",
                "error",
                "position",
                "id",
                "source",
                "detail",
            )
        )

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, _Result):
            raise TypeError(
                "compare result.answer with a value; a result equals only another result"
            )
        return self._value() == other._value()

    def __hash__(self) -> int:
        return hash(self._value())

    def __str__(self) -> str:
        if self.failed:
            assert self.error is not None
            return f"failed: {_leaf_text(self.error.code.value)}: {_leaf_text(self.error.message)}"
        if self.unsure:
            return "unsure"
        return _printed(self._answer)

    def __repr__(self) -> str:
        return f"{type(self).__name__}({short_repr(str(self))}, probability={self.probability!r}, status={self.status.value!r})"

    def to_dict(self) -> dict[str, Any]:
        """Copy the wire result; unavailable answers and caller objects are omitted."""
        from ._decode import plain

        return cast(dict[str, Any], plain(self))


def _freeze(value: object) -> object:
    if isinstance(value, _Result):
        return value._value()
    if isinstance(value, Mapping):
        return frozenset((key, _freeze(item)) for key, item in value.items())
    if isinstance(value, (tuple, list)):
        return tuple(_freeze(item) for item in value)
    if is_dataclass(value) and not isinstance(value, type):
        return tuple(_freeze(getattr(value, part.name)) for part in fields(value))
    return value


def _leaf_text(value: object) -> str:
    return str(value).replace("\r", " ").replace("\n", " ").replace("\t", " ")


def _printed(value: object) -> str:
    if isinstance(value, _Result):
        return str(value)
    if isinstance(value, Enum):
        return _leaf_text(value.value)
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, float):
        text = format(Decimal(str(value)), "f")
        return text.rstrip("0").rstrip(".") if "." in text else text
    if isinstance(value, Name):
        return f"{_leaf_text(value.kind)}: {_leaf_text(value.text)} ({value.start}..{value.end})"
    if isinstance(value, Span):
        return _leaf_text(value.text)
    if isinstance(value, Relation):
        return f"{_leaf_text(value.source.text)} --{_leaf_text(value.relation)}--> {_leaf_text(value.target.text)}"
    if isinstance(value, Mapping):
        return "\t".join(
            f"{_leaf_text(name)}={_printed(item)}" for name, item in value.items()
        )
    if isinstance(value, (tuple, list)):
        return "\t".join(_printed(item) for item in value)
    return "" if value is None else _leaf_text(value)


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class _Row(_Result[A_co]):
    position: int
    id: str | int | None
    source: Source
    item: Any = field(kw_only=True, default=None)

    def __enter__(self) -> NoReturn:
        function = self.function.value
        stream = {"filter": "filter", "rank": "rank"}.get(
            function, function + "_stream"
        )
        raise TypeError(f"this is one {function} row; use {stream}(...) for a stream")

    def __exit__(self, *exception: object) -> NoReturn:
        self.__enter__()


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
        return self.status is Status.ANSWERED and self._answer is True


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
class TagRow(_Row[list[_Label_co]], Generic[_Label_co]):
    function: Literal[Function.TAG]
    detail: LabelDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class TagOutcome(_Result[list[_Label_co]], Generic[_Label_co]):
    function: Literal[Function.TAG]
    detail: LabelDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class ScoreRow(_Row[float]):
    function: Literal[Function.SCORE]
    detail: LevelDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class ScoreOutcome(_Result[float]):
    function: Literal[Function.SCORE]
    detail: LevelDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class RankRow(_Row[int], Generic[_Item_co]):
    """A rank starting at 1 and the original record it ranks."""

    function: Literal[Function.RANK]
    detail: LevelDetail | None = None
    item: _Item_co = field(kw_only=True)


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class FindRow(_Row[Span | None]):
    function: Literal[Function.FIND]
    detail: None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class FindOutcome(_Result[Span | None]):
    function: Literal[Function.FIND]
    detail: None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class RecognizeRow(_Row[list[Name]]):
    function: Literal[Function.RECOGNIZE]
    detail: RecognizeDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class RecognizeOutcome(_Result[list[Name]]):
    function: Literal[Function.RECOGNIZE]
    detail: RecognizeDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class RelateRow(_Row[list[Relation]]):
    function: Literal[Function.RELATE]
    detail: RelationDetail | None = None


@dataclass(frozen=True, slots=True, eq=False, repr=False)
class RelateOutcome(_Result[list[Relation]]):
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
class AnnotateRow(_Row[Mapping[str, Any]]):
    # Dynamic question names can mix result types; no static schema is declared.
    # Each returned entry retains its concrete runtime result and loud answer.
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
    "Descriptions",
    "ErrorCode",
    "Failed",
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
    "NoAnswer",
    "Outcome",
    "PieceDetail",
    "RankRow",
    "RecognizeDetail",
    "RecognizeOutcome",
    "RecognizeRow",
    "Record",
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
