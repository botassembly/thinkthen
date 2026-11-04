"""One eager operation supplies native records to the shared client."""

from __future__ import annotations

import json
from abc import ABC, abstractmethod
from collections.abc import Iterable, Mapping
from typing import Any, Generic, Literal, TypeVar, cast, overload

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
from ._types import RecordError, RelationRule, Strings, Threshold


def annotation_definitions(question: object, rows: list[Any]) -> dict[str, Any]:
    if isinstance(question, (Toolbox, AnnotationQuestions)):
        return question._definitions()
    if isinstance(question, AnnotateQuestion):
        return dict(json.loads(question._native.definition)["questions"])
    if isinstance(question, Mapping):
        return AnnotationQuestions(question)._definitions()
    return {
        name: {"function": value.function.value}
        for row in rows
        if row.answer is not None
        for name, value in row.answer.items()
    }


A = TypeVar("A")
D = TypeVar("D")
K = TypeVar("K")


class ColumnMethods(ABC, Generic[A, D, K]):
    def __init__(self, value: Any) -> None:
        self._value = value

    @abstractmethod
    def _records(self) -> Iterable[Any]: ...
    @abstractmethod
    def _frame(self, function: str, rows: list[Any], question: object) -> D: ...
    @abstractmethod
    def _answers(self, function: str, rows: list[Any], question: object) -> A: ...
    @abstractmethod
    def _kept(self, rows: list[Any]) -> K: ...

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
        details: bool = False,
        errors: Literal["raise", "coerce"] = "raise",
        **options: object,
    ) -> A | D | K:
        if errors not in {"raise", "coerce"}:
            raise ValueError('errors must be "raise" or "coerce"')
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
        if errors == "raise":
            try:
                for row in rows:
                    row.raise_for_error()
                    if function == "annotate" and row.answer is not None:
                        for child in row.answer.values():
                            child.raise_for_error()
            except RecordError as error:
                cast(Any, error).outcomes = self._frame(function, rows, question)
                raise
        if details:
            return self._frame(function, rows, question)
        if function == "filter":
            return self._kept(rows)
        return self._answers(function, rows, question)

    @overload
    def decide(
        self,
        question: DecideQuestion | str,
        *,
        threshold: Threshold | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[False] = False,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A: ...
    @overload
    def decide(
        self,
        question: DecideQuestion | str,
        *,
        threshold: Threshold | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[True],
        errors: Literal["raise", "coerce"] = "raise",
    ) -> D: ...
    @overload
    def decide(
        self,
        question: DecideQuestion | str,
        *,
        threshold: Threshold | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: bool,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A | D: ...
    def decide(self, question: Any, **options: Any) -> A | D | K:
        """Return an answer column; use details=True for outcomes.

        Failed rows raise by default; errors="coerce" replaces failed answers with null.
        """
        return self._call("decide", question, **options)

    @overload
    def filter(
        self,
        question: FilterQuestion | str,
        *,
        threshold: Threshold | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[False] = False,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> K: ...
    @overload
    def filter(
        self,
        question: FilterQuestion | str,
        *,
        threshold: Threshold | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[True],
        errors: Literal["raise", "coerce"] = "raise",
    ) -> D: ...
    @overload
    def filter(
        self,
        question: FilterQuestion | str,
        *,
        threshold: Threshold | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: bool,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> K | D: ...
    def filter(self, question: Any, **options: Any) -> A | D | K:
        """Return the kept input rows; use details=True for outcomes.

        Failed rows raise by default. With errors="coerce", the default subset
        omits failed rows.
        """
        return self._call("filter", question, **options)

    @overload
    def choose(
        self,
        question: ChooseQuestion | str,
        *,
        choices: Strings | None = None,
        threshold: Threshold | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[False] = False,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A: ...
    @overload
    def choose(
        self,
        question: ChooseQuestion | str,
        *,
        choices: Strings | None = None,
        threshold: Threshold | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[True],
        errors: Literal["raise", "coerce"] = "raise",
    ) -> D: ...
    @overload
    def choose(
        self,
        question: ChooseQuestion | str,
        *,
        choices: Strings | None = None,
        threshold: Threshold | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: bool,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A | D: ...
    def choose(self, question: Any, **options: Any) -> A | D | K:
        """Return an answer column; use details=True for outcomes.

        Failed rows raise by default; errors="coerce" replaces failed answers with null.
        """
        return self._call("choose", question, **options)

    @overload
    def tag(
        self,
        question: TagQuestion | str,
        *,
        labels: Strings | None = None,
        threshold: float | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[False] = False,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A: ...
    @overload
    def tag(
        self,
        question: TagQuestion | str,
        *,
        labels: Strings | None = None,
        threshold: float | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[True],
        errors: Literal["raise", "coerce"] = "raise",
    ) -> D: ...
    @overload
    def tag(
        self,
        question: TagQuestion | str,
        *,
        labels: Strings | None = None,
        threshold: float | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: bool,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A | D: ...
    def tag(self, question: Any, **options: Any) -> A | D | K:
        """Return an answer column; use details=True for outcomes.

        Failed rows raise by default; errors="coerce" replaces failed answers with null.
        """
        return self._call("tag", question, **options)

    @overload
    def score(
        self,
        question: ScoreQuestion | str,
        *,
        levels: Strings | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[False] = False,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A: ...
    @overload
    def score(
        self,
        question: ScoreQuestion | str,
        *,
        levels: Strings | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[True],
        errors: Literal["raise", "coerce"] = "raise",
    ) -> D: ...
    @overload
    def score(
        self,
        question: ScoreQuestion | str,
        *,
        levels: Strings | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: bool,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A | D: ...
    def score(self, question: Any, **options: Any) -> A | D | K:
        """Return an answer column; use details=True for outcomes.

        Failed rows raise by default; errors="coerce" replaces failed answers with null.
        """
        return self._call("score", question, **options)

    @overload
    def rank(
        self,
        question: RankQuestion | str,
        *,
        levels: Strings | None = None,
        top: int | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[False] = False,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A: ...
    @overload
    def rank(
        self,
        question: RankQuestion | str,
        *,
        levels: Strings | None = None,
        top: int | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[True],
        errors: Literal["raise", "coerce"] = "raise",
    ) -> D: ...
    @overload
    def rank(
        self,
        question: RankQuestion | str,
        *,
        levels: Strings | None = None,
        top: int | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: bool,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A | D: ...
    def rank(self, question: Any, **options: Any) -> A | D | K:
        """Return an answer column; use details=True for outcomes.

        Failed rows raise by default; errors="coerce" replaces failed answers with null.
        """
        return self._call("rank", question, **options)

    @overload
    def find(
        self,
        question: FindQuestion | str,
        *,
        threshold: float | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[False] = False,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A: ...
    @overload
    def find(
        self,
        question: FindQuestion | str,
        *,
        threshold: float | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[True],
        errors: Literal["raise", "coerce"] = "raise",
    ) -> D: ...
    @overload
    def find(
        self,
        question: FindQuestion | str,
        *,
        threshold: float | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: bool,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A | D: ...
    def find(self, question: Any, **options: Any) -> A | D | K:
        """Return an answer column; use details=True for outcomes.

        Failed rows raise by default; errors="coerce" replaces failed answers with null.
        """
        return self._call("find", question, **options)

    @overload
    def recognize(
        self,
        question: RecognizeQuestion | None = None,
        *,
        kinds: Strings | None = None,
        threshold: float | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[False] = False,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A: ...
    @overload
    def recognize(
        self,
        question: RecognizeQuestion | None = None,
        *,
        kinds: Strings | None = None,
        threshold: float | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[True],
        errors: Literal["raise", "coerce"] = "raise",
    ) -> D: ...
    @overload
    def recognize(
        self,
        question: RecognizeQuestion | None = None,
        *,
        kinds: Strings | None = None,
        threshold: float | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: bool,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A | D: ...
    def recognize(self, question: Any = None, **options: Any) -> A | D | K:
        """Return an answer column; use details=True for outcomes.

        Failed rows raise by default; errors="coerce" replaces failed answers with null.
        """
        return self._call("recognize", question, **options)

    @overload
    def relate(
        self,
        question: RelateQuestion | None = None,
        *,
        rules: list[RelationRule] | tuple[RelationRule, ...] | None = None,
        threshold: float | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[False] = False,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A: ...
    @overload
    def relate(
        self,
        question: RelateQuestion | None = None,
        *,
        rules: list[RelationRule] | tuple[RelationRule, ...] | None = None,
        threshold: float | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[True],
        errors: Literal["raise", "coerce"] = "raise",
    ) -> D: ...
    @overload
    def relate(
        self,
        question: RelateQuestion | None = None,
        *,
        rules: list[RelationRule] | tuple[RelationRule, ...] | None = None,
        threshold: float | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: bool,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A | D: ...
    def relate(self, question: Any = None, **options: Any) -> A | D | K:
        """Return an answer column; use details=True for outcomes.

        Failed rows raise by default; errors="coerce" replaces failed answers with null.
        """
        return self._call("relate", question, **options)

    @overload
    def annotate(
        self,
        question: AnnotateQuestion | AnnotationQuestions | None = None,
        *,
        toolbox: AnnotationQuestions | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[False] = False,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A: ...
    @overload
    def annotate(
        self,
        question: AnnotateQuestion | AnnotationQuestions | None = None,
        *,
        toolbox: AnnotationQuestions | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: Literal[True],
        errors: Literal["raise", "coerce"] = "raise",
    ) -> D: ...
    @overload
    def annotate(
        self,
        question: AnnotateQuestion | AnnotationQuestions | None = None,
        *,
        toolbox: AnnotationQuestions | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: bool,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A | D: ...
    def annotate(self, question: Any = None, **options: Any) -> A | D | K:
        """Return an answer column; use details=True for outcomes.

        Failed rows raise by default; errors="coerce" replaces failed answers with null.
        """
        toolbox = options.pop("toolbox", None)
        return self._call(
            "annotate", question if question is not None else toolbox, **options
        )


__all__ = []
