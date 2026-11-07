"""Module async calls with defaults scoped to the running event loop."""

from __future__ import annotations

import asyncio
from collections.abc import AsyncIterable, Iterable
from typing import Any, Literal, TypeVar, overload
from weakref import WeakKeyDictionary

from typing_extensions import TypeForm

from ._async import AsyncClient
from ._questions import (
    AnnotateQuestion,
    AnnotationInput,
    ChooseQuestion,
    DecideQuestion,
    FilterQuestion,
    FindQuestion,
    RankQuestion,
    RecognizeQuestion,
    RelateQuestion,
    ScoreQuestion,
    TagQuestion,
)
from ._runtime import Cancel
from ._streams import AsyncKeptStream, AsyncRowStream
from ._types import (
    AnnotateRow,
    ChoiceInput,
    ChooseRow,
    DecideRow,
    Descriptions,
    FilterRow,
    FindRow,
    RankRow,
    RecognizeRow,
    Record,
    RelateRow,
    RelationRule,
    ScoreRow,
    Strings,
    TagRow,
    Threshold,
    _Label,
)

T = TypeVar("T")
_defaults: WeakKeyDictionary[asyncio.AbstractEventLoop, AsyncClient] = (
    WeakKeyDictionary()
)


def _default() -> AsyncClient:
    loop = asyncio.get_running_loop()
    client = _defaults.get(loop)
    if client is None:
        client = AsyncClient()
        _defaults[loop] = client
    return client


def set_default_client(client: AsyncClient) -> None:
    """Use this client for module calls in the running event loop."""
    if not isinstance(client, AsyncClient):
        raise TypeError("client must be an AsyncClient")
    _defaults[asyncio.get_running_loop()] = client


async def aclose_default_client() -> None:
    """Close the running event loop's default client, if it has one."""
    client = _defaults.pop(asyncio.get_running_loop(), None)
    if client is not None:
        await client.aclose()


@overload
async def decide(
    question: DecideQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> DecideRow: ...
@overload
async def decide(
    question: str,
    record: Record,
    *,
    threshold: Threshold | None = None,
    field: str | None = None,
    yes_means: str | None = None,
    no_means: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> DecideRow: ...
async def decide(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return await (_default() if client is None else client).decide(*args, **kwargs)


@overload
def decide_stream(
    question: DecideQuestion,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[DecideRow]: ...
@overload
def decide_stream(
    question: str,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    threshold: Threshold | None = None,
    field: str | None = None,
    yes_means: str | None = None,
    no_means: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[DecideRow]: ...
def decide_stream(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return (_default() if client is None else client).decide_stream(*args, **kwargs)


@overload
def filter(
    question: FilterQuestion,
    records: Iterable[T] | AsyncIterable[T],
    *,
    errors: Literal["raise"] = "raise",
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncKeptStream[T]: ...
@overload
def filter(
    question: FilterQuestion,
    records: Iterable[T] | AsyncIterable[T],
    *,
    errors: Literal["coerce"],
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncKeptStream[T | FilterRow[T]]: ...
@overload
def filter(
    question: str,
    records: Iterable[T] | AsyncIterable[T],
    *,
    threshold: Threshold | None = None,
    field: str | None = None,
    yes_means: str | None = None,
    no_means: str | None = None,
    errors: Literal["raise"] = "raise",
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncKeptStream[T]: ...
@overload
def filter(
    question: str,
    records: Iterable[T] | AsyncIterable[T],
    *,
    threshold: Threshold | None = None,
    field: str | None = None,
    yes_means: str | None = None,
    no_means: str | None = None,
    errors: Literal["coerce"],
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncKeptStream[T | FilterRow[T]]: ...
def filter(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    """Stream the kept records; see ``AsyncClient.filter``."""
    return (_default() if client is None else client).filter(*args, **kwargs)


@overload
def filter_rows(
    question: FilterQuestion,
    records: Iterable[T] | AsyncIterable[T],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[FilterRow[T]]: ...
@overload
def filter_rows(
    question: str,
    records: Iterable[T] | AsyncIterable[T],
    *,
    threshold: Threshold | None = None,
    field: str | None = None,
    yes_means: str | None = None,
    no_means: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[FilterRow[T]]: ...
def filter_rows(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return (_default() if client is None else client).filter_rows(*args, **kwargs)


@overload
async def choose(
    question: ChooseQuestion[_Label],
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> ChooseRow[_Label]: ...
@overload
async def choose(
    question: str,
    record: Record,
    *,
    choices: TypeForm[_Label],
    threshold: float | None = None,
    field: str | None = None,
    descriptions: Descriptions[_Label] | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> ChooseRow[_Label]: ...
@overload
async def choose(
    question: str,
    record: Record,
    *,
    choices: list[_Label] | tuple[_Label, ...],
    threshold: float | None = None,
    field: str | None = None,
    descriptions: Descriptions[_Label] | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> ChooseRow[_Label]: ...
async def choose(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return await (_default() if client is None else client).choose(*args, **kwargs)


@overload
def choose_stream(
    question: ChooseQuestion[_Label],
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[ChooseRow[_Label]]: ...
@overload
def choose_stream(
    question: str,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    choices: TypeForm[_Label],
    threshold: float | None = None,
    field: str | None = None,
    descriptions: Descriptions[_Label] | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[ChooseRow[_Label]]: ...
@overload
def choose_stream(
    question: str,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    choices: list[_Label] | tuple[_Label, ...],
    threshold: float | None = None,
    field: str | None = None,
    descriptions: Descriptions[_Label] | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[ChooseRow[_Label]]: ...
def choose_stream(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return (_default() if client is None else client).choose_stream(*args, **kwargs)


@overload
async def tag(
    question: TagQuestion[_Label],
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> TagRow[_Label]: ...
@overload
async def tag(
    question: str,
    record: Record,
    *,
    labels: TypeForm[_Label],
    threshold: float | None = None,
    field: str | None = None,
    descriptions: Descriptions[_Label] | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> TagRow[_Label]: ...
@overload
async def tag(
    question: str,
    record: Record,
    *,
    labels: list[_Label] | tuple[_Label, ...],
    threshold: float | None = None,
    field: str | None = None,
    descriptions: Descriptions[_Label] | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> TagRow[_Label]: ...
async def tag(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return await (_default() if client is None else client).tag(*args, **kwargs)


@overload
def tag_stream(
    question: TagQuestion[_Label],
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[TagRow[_Label]]: ...
@overload
def tag_stream(
    question: str,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    labels: TypeForm[_Label],
    threshold: float | None = None,
    field: str | None = None,
    descriptions: Descriptions[_Label] | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[TagRow[_Label]]: ...
@overload
def tag_stream(
    question: str,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    labels: list[_Label] | tuple[_Label, ...],
    threshold: float | None = None,
    field: str | None = None,
    descriptions: Descriptions[_Label] | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[TagRow[_Label]]: ...
def tag_stream(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return (_default() if client is None else client).tag_stream(*args, **kwargs)


@overload
async def score(
    question: ScoreQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> ScoreRow: ...
@overload
async def score(
    question: str,
    record: Record,
    *,
    levels: ChoiceInput[_Label],
    field: str | None = None,
    descriptions: Descriptions[_Label] | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> ScoreRow: ...
async def score(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return await (_default() if client is None else client).score(*args, **kwargs)


@overload
def score_stream(
    question: ScoreQuestion,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[ScoreRow]: ...
@overload
def score_stream(
    question: str,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    levels: ChoiceInput[_Label],
    field: str | None = None,
    descriptions: Descriptions[_Label] | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[ScoreRow]: ...
def score_stream(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return (_default() if client is None else client).score_stream(*args, **kwargs)


@overload
def rank(
    question: RankQuestion,
    records: Iterable[T] | AsyncIterable[T],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[RankRow[T]]: ...
@overload
def rank(
    question: str,
    records: Iterable[T] | AsyncIterable[T],
    *,
    levels: ChoiceInput[_Label],
    top: int | None = None,
    field: str | None = None,
    descriptions: Descriptions[_Label] | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[RankRow[T]]: ...
def rank(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return (_default() if client is None else client).rank(*args, **kwargs)


@overload
async def find(
    question: FindQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> FindRow: ...
@overload
async def find(
    question: str,
    record: Record,
    *,
    threshold: float | None = None,
    field: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> FindRow: ...
async def find(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return await (_default() if client is None else client).find(*args, **kwargs)


@overload
def find_stream(
    question: FindQuestion,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[FindRow]: ...
@overload
def find_stream(
    question: str,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    threshold: float | None = None,
    field: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[FindRow]: ...
def find_stream(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return (_default() if client is None else client).find_stream(*args, **kwargs)


@overload
async def recognize(
    question: RecognizeQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> RecognizeRow: ...
@overload
async def recognize(
    record: Record,
    *,
    kinds: Strings,
    threshold: float | None = None,
    field: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> RecognizeRow: ...
async def recognize(
    *args: Any, client: AsyncClient | None = None, **kwargs: Any
) -> Any:
    return await (_default() if client is None else client).recognize(*args, **kwargs)


@overload
def recognize_stream(
    question: RecognizeQuestion,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[RecognizeRow]: ...
@overload
def recognize_stream(
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    kinds: Strings,
    threshold: float | None = None,
    field: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[RecognizeRow]: ...
def recognize_stream(
    *args: Any, client: AsyncClient | None = None, **kwargs: Any
) -> Any:
    return (_default() if client is None else client).recognize_stream(*args, **kwargs)


@overload
async def relate(
    question: RelateQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> RelateRow: ...
@overload
async def relate(
    record: Record,
    *,
    rules: list[RelationRule] | tuple[RelationRule, ...],
    threshold: float | None = None,
    field: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> RelateRow: ...
async def relate(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return await (_default() if client is None else client).relate(*args, **kwargs)


@overload
def relate_stream(
    question: RelateQuestion,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[RelateRow]: ...
@overload
def relate_stream(
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    rules: list[RelationRule] | tuple[RelationRule, ...],
    threshold: float | None = None,
    field: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[RelateRow]: ...
def relate_stream(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return (_default() if client is None else client).relate_stream(*args, **kwargs)


@overload
async def annotate(
    question: AnnotateQuestion,
    record: Record,
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AnnotateRow: ...
@overload
async def annotate(
    toolbox: AnnotationInput,
    record: Record,
    *,
    field: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AnnotateRow: ...
async def annotate(*args: Any, client: AsyncClient | None = None, **kwargs: Any) -> Any:
    return await (_default() if client is None else client).annotate(*args, **kwargs)


@overload
def annotate_stream(
    question: AnnotateQuestion,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[AnnotateRow]: ...
@overload
def annotate_stream(
    toolbox: AnnotationInput,
    records: Iterable[Record] | AsyncIterable[Record],
    *,
    field: str | None = None,
    name: str | None = None,
    cancel: Cancel | None = None,
    client: AsyncClient | None = None,
) -> AsyncRowStream[AnnotateRow]: ...
def annotate_stream(
    *args: Any, client: AsyncClient | None = None, **kwargs: Any
) -> Any:
    return (_default() if client is None else client).annotate_stream(*args, **kwargs)


__all__ = [
    "aclose_default_client",
    "annotate",
    "annotate_stream",
    "choose",
    "choose_stream",
    "decide",
    "decide_stream",
    "filter_rows",
    "find",
    "find_stream",
    "rank",
    "recognize",
    "recognize_stream",
    "relate",
    "relate_stream",
    "score",
    "score_stream",
    "set_default_client",
    "tag",
    "tag_stream",
]
