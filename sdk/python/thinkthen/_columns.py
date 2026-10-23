"""One column operation over the existing client and owned row stream."""

from __future__ import annotations

from abc import ABC, abstractmethod
from collections.abc import Iterable, Sequence
from typing import Any, Generic, TypeVar, cast

from ._client import Client, default_client
from ._runtime import Cancel, Question, Toolbox, binding, row_from_json
from ._types import JSONValue, RelationRule, Threshold

_Frame = TypeVar("_Frame")


class ColumnMethods(ABC, Generic[_Frame]):
    """Share call translation while optional modules own their native columns."""

    def __init__(self, series: Any) -> None:
        self._series = series

    @abstractmethod
    def _missing(self, value: object) -> bool: ...

    def _value(self, value: object) -> object:
        return value

    @abstractmethod
    def _frame(self, function: str, rows: list[dict[str, Any]]) -> _Frame: ...

    def _call(
        self,
        function: str,
        question: object,
        *,
        client: Client | None = None,
        cancel: Cancel | None = None,
        **options: object,
    ) -> _Frame:
        selected = default_client() if client is None else client
        positions: list[int] = []

        def records() -> Iterable[object]:
            for at, value in enumerate(self._series):
                if not self._missing(value):
                    positions.append(at)
                    yield self._value(value)

        toolbox = None
        if function == "annotate":
            toolbox = (
                question
                if isinstance(question, Toolbox)
                else Toolbox(cast(dict[str, JSONValue], question))
            )
            question = None
        native = selected._start(
            function,
            question,
            records(),
            keep_originals=False,
            cancel=cancel,
            toolbox=toolbox,
            loaded=None,
            **options,
        )
        rows: list[dict[str, Any]] = [
            {"position": at} for at in range(len(self._series))
        ]
        try:
            while True:
                raw = binding(native.receive)
                if raw is None:
                    break
                row = row_from_json(raw[0], raw[1])
                at = positions[raw[4] - 1]
                failure = None
                if row.error is not None:
                    failure = {"code": row.error.code, "message": row.error.message}
                rows[at].update(
                    id=row.id,
                    status=row.status,
                    answer=row.answer,
                    p=row.p,
                    error=failure,
                    detail=row.detail,
                )
                if function == "filter":
                    rows[at]["kept"] = raw[2]
        finally:
            native.close()
        return self._frame(function, rows)

    def decide(
        self,
        question: str | Question[object],
        *,
        threshold: Threshold | None = None,
        on: str | None = None,
        client: Client | None = None,
        cancel: Cancel | None = None,
    ) -> _Frame:
        return self._call(
            "decide", question, threshold=threshold, on=on, client=client, cancel=cancel
        )

    def filter(
        self,
        question: str | Question[object],
        *,
        threshold: Threshold | None = None,
        on: str | None = None,
        client: Client | None = None,
        cancel: Cancel | None = None,
    ) -> _Frame:
        return self._call(
            "filter", question, threshold=threshold, on=on, client=client, cancel=cancel
        )

    def choose(
        self,
        question: str | Question[object],
        *,
        options: Sequence[str] | None = None,
        threshold: Threshold | None = None,
        on: str | None = None,
        client: Client | None = None,
        cancel: Cancel | None = None,
    ) -> _Frame:
        return self._call(
            "choose",
            question,
            options=options,
            threshold=threshold,
            on=on,
            client=client,
            cancel=cancel,
        )

    def tag(
        self,
        question: str | Question[object],
        *,
        labels: Sequence[str] | None = None,
        threshold: Threshold | None = None,
        on: str | None = None,
        client: Client | None = None,
        cancel: Cancel | None = None,
    ) -> _Frame:
        return self._call(
            "tag",
            question,
            labels=labels,
            threshold=threshold,
            on=on,
            client=client,
            cancel=cancel,
        )

    def score(
        self,
        question: str | Question[object],
        *,
        levels: Sequence[str] | None = None,
        on: str | None = None,
        client: Client | None = None,
        cancel: Cancel | None = None,
    ) -> _Frame:
        return self._call(
            "score", question, levels=levels, on=on, client=client, cancel=cancel
        )

    def rank(
        self,
        question: str | Question[JSONValue],
        *,
        levels: Sequence[str] | None = None,
        top: int | None = None,
        on: str | None = None,
        client: Client | None = None,
        cancel: Cancel | None = None,
    ) -> _Frame:
        return self._call(
            "rank",
            question,
            levels=levels,
            top=top,
            on=on,
            client=client,
            cancel=cancel,
        )

    def find(
        self,
        question: str | Question[object],
        *,
        threshold: Threshold | None = None,
        on: str | None = None,
        client: Client | None = None,
        cancel: Cancel | None = None,
    ) -> _Frame:
        return self._call(
            "find", question, threshold=threshold, on=on, client=client, cancel=cancel
        )

    def annotate(
        self,
        toolbox: Toolbox | dict[str, JSONValue],
        *,
        on: str | None = None,
        client: Client | None = None,
        cancel: Cancel | None = None,
    ) -> _Frame:
        return self._call("annotate", toolbox, on=on, client=client, cancel=cancel)

    def recognize(
        self,
        *,
        kinds: Sequence[str] | Question[object] = (),
        threshold: Threshold | None = None,
        on: str | None = None,
        client: Client | None = None,
        cancel: Cancel | None = None,
    ) -> _Frame:
        return self._call(
            "recognize", kinds, threshold=threshold, on=on, client=client, cancel=cancel
        )

    def relate(
        self,
        *,
        rules: Sequence[RelationRule] | Question[object],
        threshold: Threshold | None = None,
        on: str | None = None,
        client: Client | None = None,
        cancel: Cancel | None = None,
    ) -> _Frame:
        return self._call(
            "relate", rules, threshold=threshold, on=on, client=client, cancel=cancel
        )
