"""Typed questions about evidence through one shared engine."""

from collections.abc import Iterable, Sequence

from ._client import Client as Client
from ._client import default_client
from ._relation import Relation as Relation
from ._runtime import Cancel as Cancel
from ._runtime import Question as Question
from ._runtime import Stream as Stream
from ._runtime import Toolbox as Toolbox
from ._types import (
    Adapter as Adapter,
)
from ._types import (
    BackendError as BackendError,
)
from ._types import (
    Entity as Entity,
)
from ._types import (
    Found as Found,
)
from ._types import (
    JSONValue as JSONValue,
)
from ._types import (
    LocalError as LocalError,
)
from ._types import (
    Name as Name,
)
from ._types import (
    Record as Record,
)
from ._types import (
    RecordT,
)
from ._types import (
    RelationRule as RelationRule,
)
from ._types import (
    Row as Row,
)
from ._types import (
    RowError as RowError,
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


def decide(
    question: str | Question[object],
    text: Record,
    *,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[bool]:
    return default_client().decide(
        question, text, threshold=threshold, on=on, cancel=cancel
    )


def choose(
    question: str | Question[object],
    text: Record,
    *,
    options: Sequence[str] | None = None,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[str]:
    return default_client().choose(
        question, text, options=options, threshold=threshold, on=on, cancel=cancel
    )


def tag(
    question: str | Question[object],
    text: Record,
    *,
    labels: Sequence[str] | None = None,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[list[str]]:
    return default_client().tag(
        question, text, labels=labels, threshold=threshold, on=on, cancel=cancel
    )


def score(
    question: str | Question[object],
    text: Record,
    *,
    levels: Sequence[str] | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[float]:
    return default_client().score(question, text, levels=levels, on=on, cancel=cancel)


def filter(
    question: str | Question[object],
    records: Iterable[RecordT],
    *,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[RecordT]:
    return default_client().filter(
        question, records, threshold=threshold, on=on, cancel=cancel
    )


def rank(
    question: str | Question[JSONValue],
    records: Iterable[Record],
    *,
    levels: Sequence[str] | None = None,
    top: int | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[int]]:
    return default_client().rank(
        question, records, levels=levels, top=top, on=on, cancel=cancel
    )


def find(
    question: str | Question[object],
    text: Record,
    *,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[Found]:
    return default_client().find(
        question, text, threshold=threshold, on=on, cancel=cancel
    )


def annotate(
    toolbox: Toolbox | dict[str, JSONValue],
    text: Record,
    *,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[dict[str, JSONValue]]:
    return default_client().annotate(toolbox, text, on=on, cancel=cancel)


def recognize(
    text: Record,
    *,
    kinds: Sequence[str] | Question[object] = (),
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[list[Name]]:
    return default_client().recognize(
        text, kinds=kinds, threshold=threshold, on=on, cancel=cancel
    )


def relate(
    text: Record,
    *,
    rules: Sequence[RelationRule] | Question[object],
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Row[list[Relation]]:
    return default_client().relate(
        text, rules=rules, threshold=threshold, on=on, cancel=cancel
    )


def decide_stream(
    question: str | Question[object],
    records: Iterable[Record],
    *,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[bool]]:
    return default_client().decide_stream(
        question, records, threshold=threshold, on=on, cancel=cancel
    )


def choose_stream(
    question: str | Question[object],
    records: Iterable[Record],
    *,
    options: Sequence[str] | None = None,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[str]]:
    return default_client().choose_stream(
        question, records, options=options, threshold=threshold, on=on, cancel=cancel
    )


def tag_stream(
    question: str | Question[object],
    records: Iterable[Record],
    *,
    labels: Sequence[str] | None = None,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[list[str]]]:
    return default_client().tag_stream(
        question, records, labels=labels, threshold=threshold, on=on, cancel=cancel
    )


def score_stream(
    question: str | Question[object],
    records: Iterable[Record],
    *,
    levels: Sequence[str] | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[float]]:
    return default_client().score_stream(
        question, records, levels=levels, on=on, cancel=cancel
    )


def find_stream(
    question: str | Question[object],
    records: Iterable[Record],
    *,
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[Found]]:
    return default_client().find_stream(
        question, records, threshold=threshold, on=on, cancel=cancel
    )


def annotate_stream(
    toolbox: Toolbox | dict[str, JSONValue],
    records: Iterable[Record],
    *,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[dict[str, JSONValue]]]:
    return default_client().annotate_stream(toolbox, records, on=on, cancel=cancel)


def recognize_stream(
    records: Iterable[Record],
    *,
    kinds: Sequence[str] | Question[object] = (),
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[list[Name]]]:
    return default_client().recognize_stream(
        records, kinds=kinds, threshold=threshold, on=on, cancel=cancel
    )


def relate_stream(
    records: Iterable[Record],
    *,
    rules: Sequence[RelationRule] | Question[object],
    threshold: Threshold | None = None,
    on: str | None = None,
    cancel: Cancel | None = None,
) -> Stream[Row[list[Relation]]]:
    return default_client().relate_stream(
        records, rules=rules, threshold=threshold, on=on, cancel=cancel
    )
