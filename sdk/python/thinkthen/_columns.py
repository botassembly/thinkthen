"""One eager operation supplies native records to the shared client."""

from __future__ import annotations

import json
import warnings
from abc import ABC, abstractmethod
from collections.abc import Iterable, Mapping
from typing import Any, Generic, Literal, TypeVar, cast, overload

from ._client import Client, default_client
from ._codes import ErrorCode
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
from ._types import (
    FailedRowsWarning,
    RecordError,
    RelationRule,
    Strings,
    Threshold,
)


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


def failure_entries(function: str, rows: list[Any]) -> list[dict[str, Any]]:
    """List each failure as plain JSON values, in input order."""
    entries: list[dict[str, Any]] = []
    for row in sorted(rows, key=lambda row: row.position):
        outcomes: list[tuple[str | None, Any]] = [(None, row)]
        if function == "annotate" and row.answer is not None:
            outcomes.extend(row.answer.items())
        entries.extend(
            {
                "position": row.position,
                "id": row.id,
                "question": question,
                "error_code": outcome.error.code.value,
                "error_message": outcome.error.message,
                "error_http_status": outcome.error.http_status,
            }
            for question, outcome in outcomes
            if outcome.error is not None
        )
    return entries


def failed_rows_warning(
    rows: list[Any], entries: list[dict[str, Any]]
) -> FailedRowsWarning:
    records: dict[str, set[int]] = {}
    for entry in entries:
        records.setdefault(entry["error_code"], set()).add(entry["position"])
    counts = ", ".join(f"{code} ({len(found)})" for code, found in records.items())
    positions = {entry["position"] for entry in entries}
    return FailedRowsWarning(
        f"ThinkThen returned failed records: {counts}. "
        ".thinkthen.failures() lists them.",
        tuple(ErrorCode(code) for code in records),
        tuple(
            row
            for row in sorted(rows, key=lambda row: row.position)
            if row.position in positions
        ),
    )


A = TypeVar("A")
D = TypeVar("D")
K = TypeVar("K")
F = TypeVar("F")


class ColumnMethods(ABC, Generic[A, D, K, F]):
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
    @abstractmethod
    def _read_failures(self) -> list[dict[str, Any]]: ...
    @abstractmethod
    def _write_failures(self, value: Any, entries: list[dict[str, Any]]) -> None: ...
    @abstractmethod
    def _failure_frame(self, entries: list[dict[str, Any]]) -> F: ...

    def failures(self) -> F:
        """List the failures of an ``errors="coerce"`` result, one row each.

        Columns are ``position`` in the call's input, ``id``, ``question`` (the
        annotate question, otherwise missing), ``error_code``, ``error_message``
        and ``error_http_status``. A result with no failure gives zero rows, and so
        does an object that lost the list, such as a new Polars object, so call this
        on the object the call returned.
        """
        return self._failure_frame(self._read_failures())

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
        result: A | D | K
        if details:
            result = self._frame(function, rows, question)
        elif function == "filter":
            result = self._kept(rows)
        else:
            result = self._answers(function, rows, question)
        entries = failure_entries(function, rows)
        self._write_failures(result, entries)
        # Outcome details already show each status, so only plain results warn.
        if entries and not details:
            # The caller sits above this method and the public function method.
            warnings.warn(failed_rows_warning(rows, entries), stacklevel=3)
        return result

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

        Failed rows raise by default; errors="coerce" replaces failed answers with
        null, warns with every failed code, and failures() lists them.
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
        omits failed rows, warns with every failed code, and failures() lists them.
        """
        return self._call("filter", question, **options)

    @overload
    def choose(
        self,
        question: ChooseQuestion | str,
        *,
        choices: Strings | None = None,
        threshold: float | None = None,
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
        threshold: float | None = None,
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
        threshold: float | None = None,
        field: str | None = None,
        client: Client | None = None,
        name: str | None = None,
        cancel: Cancel | None = None,
        details: bool,
        errors: Literal["raise", "coerce"] = "raise",
    ) -> A | D: ...
    def choose(self, question: Any, **options: Any) -> A | D | K:
        """Return an answer column; use details=True for outcomes.

        Failed rows raise by default; errors="coerce" replaces failed answers with
        null, warns with every failed code, and failures() lists them.
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

        Failed rows raise by default; errors="coerce" replaces failed answers with
        null, warns with every failed code, and failures() lists them.
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

        Failed rows raise by default; errors="coerce" replaces failed answers with
        null, warns with every failed code, and failures() lists them.
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

        Failed rows raise by default; errors="coerce" replaces failed answers with
        null, warns with every failed code, and failures() lists them.
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

        Failed rows raise by default; errors="coerce" replaces failed answers with
        null, warns with every failed code, and failures() lists them.
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

        Failed rows raise by default; errors="coerce" replaces failed answers with
        null, warns with every failed code, and failures() lists them.
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

        Failed rows raise by default; errors="coerce" replaces failed answers with
        null, warns with every failed code, and failures() lists them.
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

        Failed rows raise by default; errors="coerce" replaces failed answers with
        null, warns with every failed code, and failures() lists them.
        """
        toolbox = options.pop("toolbox", None)
        return self._call(
            "annotate", question if question is not None else toolbox, **options
        )


__all__ = []
