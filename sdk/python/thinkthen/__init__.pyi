from collections.abc import Iterable, Sequence
from typing import TypeVar

from ._client import Client as Client
from ._relation import Relation as Relation
from ._runtime import Cancel as Cancel
from ._runtime import Question as Question
from ._runtime import Stream as Stream
from ._runtime import Toolbox as Toolbox
from ._types import Adapter as Adapter
from ._types import BackendError as BackendError
from ._types import Entity as Entity
from ._types import Found as Found
from ._types import JSONValue as JSONValue
from ._types import LocalError as LocalError
from ._types import Name as Name
from ._types import Record as Record
from ._types import RelationRule as RelationRule
from ._types import Row as Row
from ._types import RowError as RowError
from ._types import ThinkThenError as ThinkThenError
from ._types import Threshold as Threshold
from ._types import UsageError as UsageError

_RecordT = TypeVar("_RecordT")

def decide(
    question: str | Question[object],
    text: Record,
    *,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[bool]: ...
def choose(
    question: str | Question[object],
    text: Record,
    *,
    options: Sequence[str] | None = None,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[str]: ...
def tag(
    question: str | Question[object],
    text: Record,
    *,
    labels: Sequence[str] | None = None,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[list[str]]: ...
def score(
    question: str | Question[object],
    text: Record,
    *,
    levels: Sequence[str] | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[float]: ...
def filter(
    question: str | Question[object],
    records: Iterable[_RecordT],
    *,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[_RecordT]: ...
def rank(
    question: str | Question[JSONValue],
    records: Iterable[Record],
    *,
    levels: Sequence[str] | None = None,
    top: int | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[int]]: ...
def find(
    question: str | Question[object],
    text: Record,
    *,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[Found]: ...
def annotate(
    toolbox: Toolbox | dict[str, JSONValue],
    text: Record,
    *,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[dict[str, JSONValue]]: ...
def recognize(
    text: Record,
    *,
    kinds: Sequence[str] | Question[object] = (),
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[list[Name]]: ...
def relate(
    text: Record,
    *,
    rules: Sequence[RelationRule] | Question[object],
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[list[Relation]]: ...
def decide_stream(
    question: str | Question[object],
    records: Iterable[Record],
    *,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[bool]]: ...
def choose_stream(
    question: str | Question[object],
    records: Iterable[Record],
    *,
    options: Sequence[str] | None = None,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[str]]: ...
def tag_stream(
    question: str | Question[object],
    records: Iterable[Record],
    *,
    labels: Sequence[str] | None = None,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[list[str]]]: ...
def score_stream(
    question: str | Question[object],
    records: Iterable[Record],
    *,
    levels: Sequence[str] | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[float]]: ...
def find_stream(
    question: str | Question[object],
    records: Iterable[Record],
    *,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[Found]]: ...
def annotate_stream(
    toolbox: Toolbox | dict[str, JSONValue],
    records: Iterable[Record],
    *,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[dict[str, JSONValue]]]: ...
def recognize_stream(
    records: Iterable[Record],
    *,
    kinds: Sequence[str] | Question[object] = (),
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[list[Name]]]: ...
def relate_stream(
    records: Iterable[Record],
    *,
    rules: Sequence[RelationRule] | Question[object],
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[list[Relation]]]: ...
