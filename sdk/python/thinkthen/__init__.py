"""Typed questions about evidence through one shared engine."""

from __future__ import annotations

from collections.abc import Iterable
from typing import Any, TypeVar, overload

from . import _loader as _loader
from ._async import AsyncClient as AsyncClient
from ._client import (
    Client as Client,
)
from ._client import (
    close_default_client as close_default_client,
)
from ._client import (
    default_client,
)
from ._questions import (
    AnnotateQuestion as AnnotateQuestion,
)
from ._questions import (
    AnnotationQuestion as AnnotationQuestion,
)
from ._questions import (
    AnnotationQuestions as AnnotationQuestions,
)
from ._questions import (
    ChooseQuestion as ChooseQuestion,
)
from ._questions import (
    DecideQuestion as DecideQuestion,
)
from ._questions import (
    FilterQuestion as FilterQuestion,
)
from ._questions import (
    FindQuestion as FindQuestion,
)
from ._questions import (
    NonAnnotateQuestion as NonAnnotateQuestion,
)
from ._questions import (
    RankQuestion as RankQuestion,
)
from ._questions import (
    RecognizeQuestion as RecognizeQuestion,
)
from ._questions import (
    RelateQuestion as RelateQuestion,
)
from ._questions import (
    ScoreQuestion as ScoreQuestion,
)
from ._questions import (
    TagQuestion as TagQuestion,
)
from ._questions import (
    Toolbox as Toolbox,
)
from ._runtime import Cancel as Cancel
from ._streams import AsyncRowStream as AsyncRowStream
from ._streams import RowStream as RowStream
from ._types import (
    AnnotateRow as AnnotateRow,
)
from ._types import (
    Backend as Backend,
)
from ._types import (
    BackendInput as BackendInput,
)
from ._types import (
    Band as Band,
)
from ._types import (
    BoundaryTag as BoundaryTag,
)
from ._types import (
    Cancelled as Cancelled,
)
from ._types import (
    ChooseOutcome as ChooseOutcome,
)
from ._types import (
    ChooseRow as ChooseRow,
)
from ._types import (
    ClientSettings as ClientSettings,
)
from ._types import (
    DecideOutcome as DecideOutcome,
)
from ._types import (
    DecideRow as DecideRow,
)
from ._types import (
    ErrorCode as ErrorCode,
)
from ._types import (
    FilterOutcome as FilterOutcome,
)
from ._types import (
    FilterRow as FilterRow,
)
from ._types import (
    FindOutcome as FindOutcome,
)
from ._types import (
    FindRow as FindRow,
)
from ._types import (
    Function as Function,
)
from ._types import (
    InternalError as InternalError,
)
from ._types import (
    JSONValue as JSONValue,
)
from ._types import (
    KindDetail as KindDetail,
)
from ._types import (
    LabelDetail as LabelDetail,
)
from ._types import (
    LevelDetail as LevelDetail,
)
from ._types import (
    Name as Name,
)
from ._types import (
    Outcome as Outcome,
)
from ._types import (
    PieceDetail as PieceDetail,
)
from ._types import (
    RankRow as RankRow,
)
from ._types import (
    RecognizeDetail as RecognizeDetail,
)
from ._types import (
    RecognizeOutcome as RecognizeOutcome,
)
from ._types import (
    RecognizeRow as RecognizeRow,
)
from ._types import (
    Record as Record,
)
from ._types import (
    RecordError as RecordError,
)
from ._types import (
    RelateOutcome as RelateOutcome,
)
from ._types import (
    RelateRow as RelateRow,
)
from ._types import (
    Relation as Relation,
)
from ._types import (
    RelationDetail as RelationDetail,
)
from ._types import (
    RelationName as RelationName,
)
from ._types import (
    RelationRule as RelationRule,
)
from ._types import (
    RetryPolicy as RetryPolicy,
)
from ._types import (
    RowError as RowError,
)
from ._types import (
    ScoreOutcome as ScoreOutcome,
)
from ._types import (
    ScoreRow as ScoreRow,
)
from ._types import (
    SetupError as SetupError,
)
from ._types import (
    Source as Source,
)
from ._types import (
    Span as Span,
)
from ._types import (
    Status as Status,
)
from ._types import (
    Strings as Strings,
)
from ._types import (
    TagOutcome as TagOutcome,
)
from ._types import (
    TagRow as TagRow,
)
from ._types import (
    ThinkThenError as ThinkThenError,
)
from ._types import (
    Threshold as Threshold,
)
from ._types import (
    UsageError as UsageError,
)

T = TypeVar("T")


@overload
def decide(
    question: DecideQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> DecideRow: ...
@overload
def decide(
    question: str,
    record: Record,
    *,
    threshold: Threshold | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> DecideRow: ...
def decide(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).decide(*args, **kwargs)


@overload
def decide_stream(
    question: DecideQuestion,
    records: Iterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[DecideRow]: ...
@overload
def decide_stream(
    question: str,
    records: Iterable[Record],
    *,
    threshold: Threshold | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[DecideRow]: ...
def decide_stream(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).decide_stream(
        *args, **kwargs
    )


@overload
def filter(
    question: FilterQuestion,
    records: Iterable[T],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[FilterRow[T]]: ...
@overload
def filter(
    question: str,
    records: Iterable[T],
    *,
    threshold: Threshold | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[FilterRow[T]]: ...
def filter(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).filter(*args, **kwargs)


@overload
def choose(
    question: ChooseQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> ChooseRow: ...
@overload
def choose(
    question: str,
    record: Record,
    *,
    choices: Strings,
    threshold: Threshold | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> ChooseRow: ...
def choose(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).choose(*args, **kwargs)


@overload
def choose_stream(
    question: ChooseQuestion,
    records: Iterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[ChooseRow]: ...
@overload
def choose_stream(
    question: str,
    records: Iterable[Record],
    *,
    choices: Strings,
    threshold: Threshold | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[ChooseRow]: ...
def choose_stream(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).choose_stream(
        *args, **kwargs
    )


@overload
def tag(
    question: TagQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> TagRow: ...
@overload
def tag(
    question: str,
    record: Record,
    *,
    labels: Strings,
    threshold: float | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> TagRow: ...
def tag(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).tag(*args, **kwargs)


@overload
def tag_stream(
    question: TagQuestion,
    records: Iterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[TagRow]: ...
@overload
def tag_stream(
    question: str,
    records: Iterable[Record],
    *,
    labels: Strings,
    threshold: float | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[TagRow]: ...
def tag_stream(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).tag_stream(*args, **kwargs)


@overload
def score(
    question: ScoreQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> ScoreRow: ...
@overload
def score(
    question: str,
    record: Record,
    *,
    levels: Strings,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> ScoreRow: ...
def score(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).score(*args, **kwargs)


@overload
def score_stream(
    question: ScoreQuestion,
    records: Iterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[ScoreRow]: ...
@overload
def score_stream(
    question: str,
    records: Iterable[Record],
    *,
    levels: Strings,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[ScoreRow]: ...
def score_stream(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).score_stream(
        *args, **kwargs
    )


@overload
def rank(
    question: RankQuestion,
    records: Iterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[RankRow]: ...
@overload
def rank(
    question: str,
    records: Iterable[Record],
    *,
    levels: Strings,
    top: int | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[RankRow]: ...
def rank(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).rank(*args, **kwargs)


@overload
def find(
    question: FindQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> FindRow: ...
@overload
def find(
    question: str,
    record: Record,
    *,
    threshold: float | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> FindRow: ...
def find(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).find(*args, **kwargs)


@overload
def find_stream(
    question: FindQuestion,
    records: Iterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[FindRow]: ...
@overload
def find_stream(
    question: str,
    records: Iterable[Record],
    *,
    threshold: float | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[FindRow]: ...
def find_stream(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).find_stream(*args, **kwargs)


@overload
def recognize(
    question: RecognizeQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RecognizeRow: ...
@overload
def recognize(
    record: Record,
    *,
    kinds: Strings,
    threshold: float | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RecognizeRow: ...
def recognize(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).recognize(*args, **kwargs)


@overload
def recognize_stream(
    question: RecognizeQuestion,
    records: Iterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[RecognizeRow]: ...
@overload
def recognize_stream(
    records: Iterable[Record],
    *,
    kinds: Strings,
    threshold: float | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[RecognizeRow]: ...
def recognize_stream(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).recognize_stream(
        *args, **kwargs
    )


@overload
def relate(
    question: RelateQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RelateRow: ...
@overload
def relate(
    record: Record,
    *,
    rules: list[RelationRule] | tuple[RelationRule, ...],
    threshold: float | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RelateRow: ...
def relate(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).relate(*args, **kwargs)


@overload
def relate_stream(
    question: RelateQuestion,
    records: Iterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[RelateRow]: ...
@overload
def relate_stream(
    records: Iterable[Record],
    *,
    rules: list[RelationRule] | tuple[RelationRule, ...],
    threshold: float | None = None,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[RelateRow]: ...
def relate_stream(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).relate_stream(
        *args, **kwargs
    )


@overload
def annotate(
    question: AnnotateQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> AnnotateRow: ...
@overload
def annotate(
    toolbox: AnnotationQuestions,
    record: Record,
    *,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> AnnotateRow: ...
def annotate(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).annotate(*args, **kwargs)


@overload
def annotate_stream(
    question: AnnotateQuestion,
    records: Iterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[AnnotateRow]: ...
@overload
def annotate_stream(
    toolbox: AnnotationQuestions,
    records: Iterable[Record],
    *,
    evidence: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: Client | None = None,
) -> RowStream[AnnotateRow]: ...
def annotate_stream(*args: Any, client: Client | None = None, **kwargs: Any) -> Any:
    return (default_client() if client is None else client).annotate_stream(
        *args, **kwargs
    )


__all__ = [
    "AnnotateQuestion",
    "AnnotateRow",
    "AnnotationQuestion",
    "AnnotationQuestions",
    "AsyncClient",
    "AsyncRowStream",
    "Backend",
    "BackendInput",
    "Band",
    "BoundaryTag",
    "Cancel",
    "Cancelled",
    "ChooseOutcome",
    "ChooseQuestion",
    "ChooseRow",
    "Client",
    "ClientSettings",
    "DecideOutcome",
    "DecideQuestion",
    "DecideRow",
    "ErrorCode",
    "FilterOutcome",
    "FilterQuestion",
    "FilterRow",
    "FindOutcome",
    "FindQuestion",
    "FindRow",
    "Function",
    "InternalError",
    "JSONValue",
    "KindDetail",
    "LabelDetail",
    "LevelDetail",
    "Name",
    "NonAnnotateQuestion",
    "Outcome",
    "PieceDetail",
    "RankQuestion",
    "RankRow",
    "RecognizeDetail",
    "RecognizeOutcome",
    "RecognizeQuestion",
    "RecognizeRow",
    "Record",
    "RecordError",
    "RelateOutcome",
    "RelateQuestion",
    "RelateRow",
    "Relation",
    "RelationDetail",
    "RelationName",
    "RelationRule",
    "RetryPolicy",
    "RowError",
    "RowStream",
    "ScoreOutcome",
    "ScoreQuestion",
    "ScoreRow",
    "SetupError",
    "Source",
    "Span",
    "Status",
    "Strings",
    "TagOutcome",
    "TagQuestion",
    "TagRow",
    "ThinkThenError",
    "Threshold",
    "Toolbox",
    "UsageError",
    "annotate",
    "annotate_stream",
    "choose",
    "choose_stream",
    "close_default_client",
    "decide",
    "decide_stream",
    "filter",
    "find",
    "find_stream",
    "rank",
    "recognize",
    "recognize_stream",
    "relate",
    "relate_stream",
    "score",
    "score_stream",
    "tag",
    "tag_stream",
]
