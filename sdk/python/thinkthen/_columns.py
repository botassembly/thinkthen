"""One eager operation supplies native records to the shared client."""

from __future__ import annotations

import json
from abc import ABC, abstractmethod
from collections.abc import Iterable
from typing import Any, Generic, TypeVar, overload

from ._client import Client, default_client
from ._questions import (
    AnnotateQuestion,
    AnnotationQuestions,
    ChooseQuestion,
    DecideQuestion,
    FilterQuestion,
    FindQuestion,
    RankQuestion,
    RecognizeQuestion,
    RelateQuestion,
    ScoreQuestion,
    TagQuestion,
    Toolbox,
)
from ._runtime import Cancel, normalize
from ._types import RelationRule, Strings, Threshold


def annotation_definitions(question: object, rows: list[Any]) -> dict[str, Any]:
    if isinstance(question, (Toolbox, AnnotationQuestions)):
        return question._definitions()
    if isinstance(question, AnnotateQuestion):
        return dict(json.loads(question._native.definition)["questions"])
    return {
        name: {"function": value.function.value}
        for row in rows
        if row.answer is not None
        for name, value in row.answer.items()
    }


F = TypeVar("F")


class ColumnMethods(ABC, Generic[F]):
    def __init__(self, value: Any) -> None:
        self._value = value

    @abstractmethod
    def _records(self) -> Iterable[Any]: ...
    @abstractmethod
    def _frame(self, function: str, rows: list[Any], question: object) -> F: ...
    def _missing(self, value: object) -> bool:
        return False

    def _call(
        self,
        function: str,
        question: object,
        *,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        **options: object,
    ) -> F:
        selected = default_client() if client is None else client
        with selected._rows(
            function,
            question,
            list(self._records()),
            lambda value: normalize(value, self._missing),
            name=name,
            cancel=cancel,
            **options,
        ) as stream:
            rows = stream.to_list()
        return self._frame(function, rows, question)

    @overload
    def decide(
        self,
        question: DecideQuestion,
        *,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    @overload
    def decide(
        self,
        question: str,
        *,
        threshold: Threshold | None = None,
        evidence: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    def decide(self, question: Any, **options: Any) -> F:
        return self._call("decide", question, **options)

    @overload
    def filter(
        self,
        question: FilterQuestion,
        *,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    @overload
    def filter(
        self,
        question: str,
        *,
        threshold: Threshold | None = None,
        evidence: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    def filter(self, question: Any, **options: Any) -> F:
        return self._call("filter", question, **options)

    @overload
    def choose(
        self,
        question: ChooseQuestion,
        *,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    @overload
    def choose(
        self,
        question: str,
        *,
        choices: Strings,
        threshold: Threshold | None = None,
        evidence: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    def choose(self, question: Any, **options: Any) -> F:
        return self._call("choose", question, **options)

    @overload
    def tag(
        self,
        question: TagQuestion,
        *,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    @overload
    def tag(
        self,
        question: str,
        *,
        labels: Strings,
        threshold: float | None = None,
        evidence: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    def tag(self, question: Any, **options: Any) -> F:
        return self._call("tag", question, **options)

    @overload
    def score(
        self,
        question: ScoreQuestion,
        *,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    @overload
    def score(
        self,
        question: str,
        *,
        levels: Strings,
        evidence: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    def score(self, question: Any, **options: Any) -> F:
        return self._call("score", question, **options)

    @overload
    def rank(
        self,
        question: RankQuestion,
        *,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    @overload
    def rank(
        self,
        question: str,
        *,
        levels: Strings,
        top: int | None = None,
        evidence: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    def rank(self, question: Any, **options: Any) -> F:
        return self._call("rank", question, **options)

    @overload
    def find(
        self,
        question: FindQuestion,
        *,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    @overload
    def find(
        self,
        question: str,
        *,
        threshold: float | None = None,
        evidence: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    def find(self, question: Any, **options: Any) -> F:
        return self._call("find", question, **options)

    @overload
    def recognize(
        self,
        question: RecognizeQuestion,
        *,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    @overload
    def recognize(
        self,
        *,
        kinds: Strings,
        threshold: float | None = None,
        evidence: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    def recognize(self, question: Any = None, **options: Any) -> F:
        return self._call("recognize", question, **options)

    @overload
    def relate(
        self,
        question: RelateQuestion,
        *,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    @overload
    def relate(
        self,
        *,
        rules: list[RelationRule] | tuple[RelationRule, ...],
        threshold: float | None = None,
        evidence: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    def relate(self, question: Any = None, **options: Any) -> F:
        return self._call("relate", question, **options)

    @overload
    def annotate(
        self,
        question: AnnotateQuestion,
        *,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    @overload
    def annotate(
        self,
        toolbox: AnnotationQuestions,
        *,
        evidence: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
    ) -> F: ...
    def annotate(self, *args: Any, **options: Any) -> F:
        question = (
            args[0] if args else options.pop("question", options.pop("toolbox", None))
        )
        return self._call("annotate", question, **options)


__all__ = []
